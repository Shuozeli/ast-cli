use std::path::Path;

use anyhow::{Result, bail};
use tree_sitter::Node;

use crate::languages::{self, Lang, is_body_container, node_text};

pub fn run(path: &Path, address: &str) -> Result<String> {
    let (tree, source, lang) = languages::parse_file(path)?;
    let root = tree.root_node();

    // Check if address is a line range (e.g. "150:200")
    if let Some((start, end)) = parse_line_range(address) {
        return extract_by_lines(&source, start, end);
    }

    // Otherwise treat as a symbol address (e.g. "Parser::parse_field")
    let parts: Vec<&str> = address.split("::").collect();
    let node = find_node_by_address(root, &source, lang, &parts)?;
    let text = node.utf8_text(source.as_bytes())?;
    Ok(text.to_string())
}

fn parse_line_range(address: &str) -> Option<(usize, usize)> {
    let parts: Vec<&str> = address.splitn(2, ':').collect();
    if parts.len() == 2
        && let (Ok(start), Ok(end)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>())
    {
        return Some((start, end));
    }
    None
}

fn extract_by_lines(source: &str, start: usize, end: usize) -> Result<String> {
    let lines: Vec<&str> = source.lines().collect();
    if start == 0 || start > lines.len() || end < start {
        bail!(
            "invalid line range {start}:{end} (file has {} lines)",
            lines.len()
        );
    }
    let end = end.min(lines.len());
    let slice = lines
        .get(start - 1..end)
        .ok_or_else(|| anyhow::anyhow!("invalid line range {start}:{end}"))?;
    Ok(slice.join("\n"))
}

fn find_node_by_address<'a>(
    root: Node<'a>,
    source: &str,
    lang: Lang,
    parts: &[&str],
) -> Result<Node<'a>> {
    if parts.is_empty() {
        bail!("empty address");
    }

    if parts.len() == 1 {
        return find_named_child(root, source, lang, parts[0])
            .ok_or_else(|| anyhow::anyhow!("could not find '{}'", parts[0]));
    }

    let type_name = parts[0];
    let member_name = parts[1];

    let containers = find_all_containers(root, source, lang, type_name);
    for container in containers {
        if let Some(found) = find_named_child(container, source, lang, member_name) {
            return Ok(found);
        }
    }

    bail!("could not find '{}' in '{}'", member_name, parts.join("::"))
}

/// Recursively find all container nodes (impl, class, trait) matching a type name.
fn find_all_containers<'a>(
    node: Node<'a>,
    source: &str,
    lang: Lang,
    type_name: &str,
) -> Vec<Node<'a>> {
    let mut results = Vec::new();
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if let Some(name) = get_container_type_name(child, source, lang)
            && name == type_name
        {
            results.push(child);
        }
        results.extend(find_all_containers(child, source, lang, type_name));
    }
    results
}

/// Get the type name that a container (impl, class, trait) defines.
fn get_container_type_name(node: Node, source: &str, lang: Lang) -> Option<String> {
    match lang {
        Lang::Rust => match node.kind() {
            "impl_item" | "trait_item" | "struct_item" | "enum_item" | "mod_item"
            | "union_item" => node
                .child_by_field_name("type")
                .or_else(|| node.child_by_field_name("name"))
                .and_then(|n| node_text(n, source).ok())
                .map(|s| strip_generics(&s)),
            _ => None,
        },
        Lang::Cpp => match node.kind() {
            "class_specifier" | "struct_specifier" | "namespace_definition" | "union_specifier" => {
                node.child_by_field_name("name")
                    .and_then(|n| node_text(n, source).ok())
            }
            "template_declaration" => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if let Some(name) = get_container_type_name(child, source, lang) {
                        return Some(name);
                    }
                }
                None
            }
            _ => None,
        },
        Lang::Typescript | Lang::Tsx => match node.kind() {
            "class_declaration"
            | "abstract_class_declaration"
            | "interface_declaration"
            | "internal_module"
            | "module" => node
                .child_by_field_name("name")
                .and_then(|n| node_text(n, source).ok()),
            "export_statement" => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if let Some(name) = get_container_type_name(child, source, lang) {
                        return Some(name);
                    }
                }
                None
            }
            _ => None,
        },
        Lang::Python => match node.kind() {
            "class_definition" => node
                .child_by_field_name("name")
                .and_then(|n| node_text(n, source).ok()),
            "decorated_definition" => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if let Some(name) = get_container_type_name(child, source, lang) {
                        return Some(name);
                    }
                }
                None
            }
            _ => None,
        },
        Lang::Protobuf => match node.kind() {
            "message" | "service" | "oneof" => node
                .child_by_field_name("name")
                .or_else(|| {
                    let mut cursor = node.walk();
                    node.children(&mut cursor).find(|c| {
                        matches!(c.kind(), "message_name" | "service_name" | "oneof_name")
                    })
                })
                .and_then(|n| node_text(n, source).ok()),
            _ => None,
        },
    }
}

fn find_named_child<'a>(
    parent: Node<'a>,
    source: &str,
    lang: Lang,
    name: &str,
) -> Option<Node<'a>> {
    let mut cursor = parent.walk();
    for child in parent.children(&mut cursor) {
        if let Some(child_name) = get_definition_name(child, source, lang)
            && child_name == name
        {
            return Some(child);
        }
        if (is_body_container(child.kind()) || is_wrapper_node(child, lang))
            && let Some(found) = find_named_child(child, source, lang, name)
        {
            return Some(found);
        }
    }
    None
}

fn is_wrapper_node(node: Node, lang: Lang) -> bool {
    match lang {
        Lang::Typescript | Lang::Tsx => node.kind() == "export_statement",
        Lang::Python => node.kind() == "decorated_definition",
        Lang::Cpp => matches!(
            node.kind(),
            "template_declaration"
                | "linkage_specification"
                | "preproc_ifdef"
                | "preproc_if"
                | "declaration"
                | "field_declaration"
                | "ERROR"
        ),
        Lang::Rust => node.kind() == "foreign_mod_item",
        _ => false,
    }
}

fn get_definition_name(node: Node, source: &str, lang: Lang) -> Option<String> {
    match lang {
        Lang::Rust => match node.kind() {
            "function_item" | "struct_item" | "enum_item" | "trait_item" | "type_item"
            | "const_item" | "static_item" | "mod_item" | "macro_definition" | "union_item" => node
                .child_by_field_name("name")
                .and_then(|n| node_text(n, source).ok()),
            "impl_item" => node
                .child_by_field_name("type")
                .and_then(|n| node_text(n, source).ok()),
            "use_declaration" => node
                .child_by_field_name("argument")
                .and_then(|n| node_text(n, source).ok()),
            "field_declaration" => node
                .child_by_field_name("name")
                .or_else(|| node.child_by_field_name("type"))
                .and_then(|n| node_text(n, source).ok()),
            _ => None,
        },
        Lang::Cpp => match node.kind() {
            "function_definition"
            | "class_specifier"
            | "struct_specifier"
            | "enum_specifier"
            | "union_specifier"
            | "concept_definition"
            | "namespace_definition" => node
                .child_by_field_name("name")
                .or_else(|| node.child_by_field_name("declarator"))
                .map(|n| languages::extract_innermost_name(n, source)),
            "field_declaration" => {
                if node.child_by_field_name("declarator").is_some() {
                    node.child_by_field_name("declarator")
                        .map(|n| languages::extract_innermost_name(n, source))
                } else {
                    None
                }
            }
            "using_declaration" | "friend_declaration" => {
                let mut cursor = node.walk();
                node.children(&mut cursor)
                    .find(|c| c.is_named())
                    .and_then(|n| node_text(n, source).ok())
            }
            "preproc_include" => node
                .child_by_field_name("path")
                .and_then(|n| node_text(n, source).ok()),
            _ => None,
        },
        Lang::Typescript | Lang::Tsx => match node.kind() {
            "function_declaration"
            | "generator_function_declaration"
            | "class_declaration"
            | "abstract_class_declaration"
            | "interface_declaration"
            | "enum_declaration"
            | "type_alias_declaration"
            | "method_definition"
            | "internal_module"
            | "module" => node
                .child_by_field_name("name")
                .and_then(|n| node_text(n, source).ok()),
            "public_field_definition" => node
                .child_by_field_name("name")
                .or_else(|| node.child_by_field_name("property"))
                .and_then(|n| node_text(n, source).ok()),
            "import_statement" | "import_require_clause" => node
                .child_by_field_name("source")
                .and_then(|n| node_text(n, source).ok()),
            _ => None,
        },
        Lang::Python => match node.kind() {
            "function_definition" | "class_definition" | "type_alias_statement" => node
                .child_by_field_name("name")
                .and_then(|n| node_text(n, source).ok()),
            "import_statement" | "import_from_statement" | "future_import_statement" => node
                .child_by_field_name("module_name")
                .or_else(|| {
                    let mut cursor = node.walk();
                    node.children(&mut cursor)
                        .find(|c| c.kind() == "dotted_name" || c.kind() == "aliased_import")
                })
                .and_then(|n| node_text(n, source).ok()),
            "decorated_definition" => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if let Some(name) = get_definition_name(child, source, lang) {
                        return Some(name);
                    }
                }
                None
            }
            _ => None,
        },
        Lang::Protobuf => match node.kind() {
            "message" | "enum" | "service" | "rpc" | "oneof" | "field" => node
                .child_by_field_name("name")
                .or_else(|| {
                    let mut cursor = node.walk();
                    node.children(&mut cursor).find(|c| {
                        matches!(
                            c.kind(),
                            "message_name"
                                | "enum_name"
                                | "service_name"
                                | "rpc_name"
                                | "oneof_name"
                                | "field_name"
                        )
                    })
                })
                .and_then(|n| node_text(n, source).ok()),
            _ => None,
        },
    }
}

/// Strip generic parameters from a type name: `BinaryWalker<'a>` -> `BinaryWalker`
fn strip_generics(name: &str) -> String {
    name.split('<').next().unwrap_or(name).to_string()
}
