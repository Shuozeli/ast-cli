use std::path::Path;

use anyhow::Result;
use serde::Serialize;
use tree_sitter::Node;

use crate::languages::{self, Lang, node_text};

#[derive(Debug, Serialize)]
pub struct OutlineItem {
    pub kind: String,
    pub name: String,
    pub start_line: usize,
    pub end_line: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<OutlineItem>,
}

pub fn run(path: &Path) -> Result<Vec<OutlineItem>> {
    let (tree, source, lang) = languages::parse_file(path)?;
    let root = tree.root_node();
    let items = collect_items(root, &source, lang);
    Ok(items)
}

fn collect_items(node: Node, source: &str, lang: Lang) -> Vec<OutlineItem> {
    let mut items = Vec::new();
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        if let Some(item) = node_to_item(child, source, lang) {
            items.push(item);
        } else if lang == Lang::Cpp && should_recurse_cpp(child) {
            items.extend(collect_items(child, source, lang));
        }
    }

    items
}

/// C/C++ headers often wrap definitions in preprocessor guards, type_definition,
/// linkage_specification ("extern C { }"), or macro-expanded blocks. Recurse into these.
fn should_recurse_cpp(node: Node) -> bool {
    matches!(
        node.kind(),
        "type_definition"
            | "linkage_specification"
            | "declaration"
            | "expression_statement"
            | "preproc_ifdef"
            | "preproc_if"
            | "preproc_elif"
            | "preproc_else"
            | "ERROR"
    )
}

fn node_to_item(node: Node, source: &str, lang: Lang) -> Option<OutlineItem> {
    match lang {
        Lang::Rust => rust_item(node, source),
        Lang::Cpp => cpp_item(node, source),
        Lang::Typescript | Lang::Tsx => ts_item(node, source),
        Lang::Python => python_item(node, source),
        Lang::Protobuf => proto_item(node, source),
    }
}

fn rust_item(node: Node, source: &str) -> Option<OutlineItem> {
    let kind_str = node.kind();
    let kind = match kind_str {
        "function_item" => "function",
        "struct_item" => "struct",
        "enum_item" => "enum",
        "trait_item" => "trait",
        "impl_item" => "impl",
        "type_item" => "type_alias",
        "const_item" => "const",
        "static_item" => "static",
        "mod_item" => "module",
        "use_declaration" => "use",
        "macro_definition" => "macro",
        _ => return None,
    };

    let name = extract_name(node, source, kind_str);
    let visibility = extract_child_text(node, "visibility_modifier", source);
    let signature = (kind == "function").then(|| extract_fn_signature(node, source, "block"));

    let children = if matches!(kind, "impl" | "trait" | "module") {
        collect_body_items(node, source, Lang::Rust)
    } else {
        Vec::new()
    };

    Some(OutlineItem {
        kind: kind.to_string(),
        name,
        start_line: node.start_position().row + 1,
        end_line: node.end_position().row + 1,
        visibility,
        signature,
        children,
    })
}

/// Extract a function signature by collecting all children before the body node.
/// `body_kind` is the tree-sitter node kind that marks the start of the body
/// (e.g. "block" for Rust, "compound_statement" for C++, "statement_block" for TS).
fn extract_fn_signature(node: Node, source: &str, body_kind: &str) -> String {
    let mut sig = String::new();
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == body_kind {
            break;
        }
        if !sig.is_empty() {
            sig.push(' ');
        }
        sig.push_str(child.utf8_text(source.as_bytes()).unwrap_or_default());
    }
    sig.trim().to_string()
}

fn cpp_item(node: Node, source: &str) -> Option<OutlineItem> {
    let kind_str = node.kind();
    let kind = match kind_str {
        "function_definition" => "function",
        "declaration" if has_function_declarator(node) => "function_decl",
        "class_specifier" => "class",
        "struct_specifier" => "struct",
        "enum_specifier" => "enum",
        "namespace_definition" => "namespace",
        "template_declaration" => return cpp_template_item(node, source),
        "type_definition" => "typedef",
        _ => return None,
    };

    let name = extract_name_cpp(node, source, kind_str);
    let signature = if kind == "function" || kind == "function_decl" {
        Some(extract_fn_signature(node, source, "compound_statement"))
    } else {
        None
    };

    let children = if matches!(kind, "class" | "struct" | "namespace") {
        collect_body_items(node, source, Lang::Cpp)
    } else {
        Vec::new()
    };

    Some(OutlineItem {
        kind: kind.to_string(),
        name,
        start_line: node.start_position().row + 1,
        end_line: node.end_position().row + 1,
        visibility: None,
        signature,
        children,
    })
}

fn has_function_declarator(node: Node) -> bool {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .any(|c| c.kind() == "function_declarator")
}

fn cpp_template_item(node: Node, source: &str) -> Option<OutlineItem> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if let Some(mut item) = cpp_item(child, source) {
            item.kind = format!("template_{}", item.kind);
            item.start_line = node.start_position().row + 1;
            return Some(item);
        }
    }
    None
}

fn extract_name_cpp(node: Node, source: &str, kind_str: &str) -> String {
    match kind_str {
        "namespace_definition" => extract_child_text(node, "name", source)
            .or_else(|| extract_child_text(node, "namespace_identifier", source))
            .unwrap_or_else(|| "<anonymous>".to_string()),
        "type_definition" => node
            .child_by_field_name("declarator")
            .and_then(|n| node_text(n, source).ok())
            .unwrap_or_else(|| {
                let mut cursor = node.walk();
                node.children(&mut cursor)
                    .filter(|c| c.kind() == "type_identifier" || c.kind() == "identifier")
                    .last()
                    .and_then(|n| node_text(n, source).ok())
                    .unwrap_or_else(|| "<unknown>".to_string())
            }),
        _ => node
            .child_by_field_name("name")
            .or_else(|| node.child_by_field_name("declarator"))
            .map(|n| {
                let mut n = n;
                while n.kind() == "function_declarator"
                    || n.kind() == "qualified_identifier"
                    || n.kind() == "pointer_declarator"
                    || n.kind() == "reference_declarator"
                {
                    if let Some(inner) = n
                        .child_by_field_name("name")
                        .or_else(|| n.child_by_field_name("declarator"))
                    {
                        n = inner;
                    } else {
                        break;
                    }
                }
                node_text(n, source).unwrap_or_else(|_| "<unknown>".to_string())
            })
            .unwrap_or_else(|| "<unknown>".to_string()),
    }
}

fn ts_item(node: Node, source: &str) -> Option<OutlineItem> {
    let kind_str = node.kind();
    let kind = match kind_str {
        "function_declaration" => "function",
        "class_declaration" => "class",
        "interface_declaration" => "interface",
        "enum_declaration" => "enum",
        "type_alias_declaration" => "type_alias",
        "lexical_declaration" => return ts_lexical_item(node, source),
        "export_statement" => return ts_export_item(node, source),
        "method_definition" => "method",
        _ => return None,
    };

    let name = extract_child_text(node, "name", source).unwrap_or_else(|| "<unknown>".to_string());
    let signature = if kind == "function" || kind == "method" {
        Some(extract_fn_signature(node, source, "statement_block"))
    } else {
        None
    };

    let children = if matches!(kind, "class" | "interface") {
        collect_body_items(node, source, Lang::Typescript)
    } else {
        Vec::new()
    };

    Some(OutlineItem {
        kind: kind.to_string(),
        name,
        start_line: node.start_position().row + 1,
        end_line: node.end_position().row + 1,
        visibility: None,
        signature,
        children,
    })
}

fn ts_lexical_item(node: Node, source: &str) -> Option<OutlineItem> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "variable_declarator" {
            let name = extract_child_text(child, "name", source)
                .unwrap_or_else(|| "<unknown>".to_string());
            let has_arrow = child
                .child_by_field_name("value")
                .is_some_and(|v| v.kind() == "arrow_function");
            let kind = if has_arrow { "function" } else { "const" };
            return Some(OutlineItem {
                kind: kind.to_string(),
                name,
                start_line: node.start_position().row + 1,
                end_line: node.end_position().row + 1,
                visibility: None,
                signature: None,
                children: Vec::new(),
            });
        }
    }
    None
}

fn ts_export_item(node: Node, source: &str) -> Option<OutlineItem> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if let Some(mut item) = ts_item(child, source) {
            item.visibility = Some("export".to_string());
            item.start_line = node.start_position().row + 1;
            return Some(item);
        }
    }
    None
}

fn python_item(node: Node, source: &str) -> Option<OutlineItem> {
    let kind_str = node.kind();
    let kind = match kind_str {
        "function_definition" => "function",
        "class_definition" => "class",
        "decorated_definition" => return python_decorated_item(node, source),
        _ => return None,
    };

    let name = extract_child_text(node, "name", source).unwrap_or_else(|| "<unknown>".to_string());
    let signature = if kind == "function" {
        Some(extract_fn_signature_python(node, source))
    } else {
        None
    };

    let children = if kind == "class" {
        collect_body_items_python(node, source)
    } else {
        Vec::new()
    };

    Some(OutlineItem {
        kind: kind.to_string(),
        name,
        start_line: node.start_position().row + 1,
        end_line: node.end_position().row + 1,
        visibility: None,
        signature,
        children,
    })
}

fn python_decorated_item(node: Node, source: &str) -> Option<OutlineItem> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if let Some(mut item) = python_item(child, source) {
            item.start_line = node.start_position().row + 1;
            return Some(item);
        }
    }
    None
}

fn extract_fn_signature_python(node: Node, source: &str) -> String {
    let name = extract_child_text(node, "name", source).unwrap_or_default();
    let params = node
        .child_by_field_name("parameters")
        .and_then(|n| node_text(n, source).ok())
        .unwrap_or_else(|| "()".to_string());
    let ret = node
        .child_by_field_name("return_type")
        .and_then(|n| node_text(n, source).ok())
        .map(|t| format!(" -> {t}"))
        .unwrap_or_default();
    format!("def {name}{params}{ret}")
}

fn collect_body_items_python(node: Node, source: &str) -> Vec<OutlineItem> {
    let mut items = Vec::new();
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "block" {
            let mut block_cursor = child.walk();
            for stmt in child.children(&mut block_cursor) {
                if let Some(item) = python_item(stmt, source) {
                    items.push(item);
                }
            }
        }
    }
    items
}

fn proto_item(node: Node, source: &str) -> Option<OutlineItem> {
    let kind_str = node.kind();
    let kind = match kind_str {
        "message" => "message",
        "enum" => "enum",
        "service" => "service",
        "rpc" => "rpc",
        _ => return None,
    };

    let name = extract_child_text(node, "name", source)
        .or_else(|| {
            let mut cursor = node.walk();
            node.children(&mut cursor)
                .find(|c| {
                    c.kind() == "message_name"
                        || c.kind() == "enum_name"
                        || c.kind() == "service_name"
                        || c.kind() == "rpc_name"
                })
                .and_then(|n| node_text(n, source).ok())
        })
        .unwrap_or_else(|| "<unknown>".to_string());

    let children = if matches!(kind, "message" | "service") {
        collect_body_items(node, source, Lang::Protobuf)
    } else {
        Vec::new()
    };

    Some(OutlineItem {
        kind: kind.to_string(),
        name,
        start_line: node.start_position().row + 1,
        end_line: node.end_position().row + 1,
        visibility: None,
        signature: None,
        children,
    })
}

fn extract_name(node: Node, source: &str, kind_str: &str) -> String {
    match kind_str {
        "impl_item" => {
            let mut parts = Vec::new();
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                match child.kind() {
                    "declaration_list" => break,
                    "impl" | "where_clause" => {}
                    _ => {
                        if let Ok(text) = node_text(child, source) {
                            parts.push(text);
                        }
                    }
                }
            }
            parts.join(" ")
        }
        _ => node
            .child_by_field_name("name")
            .and_then(|n| node_text(n, source).ok())
            .unwrap_or_else(|| "<unknown>".to_string()),
    }
}

fn extract_child_text(node: Node, field: &str, source: &str) -> Option<String> {
    node.child_by_field_name(field)
        .and_then(|n| node_text(n, source).ok())
}

fn collect_body_items(node: Node, source: &str, lang: Lang) -> Vec<OutlineItem> {
    let mut items = Vec::new();
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if languages::is_body_container(child.kind()) {
            let mut inner_cursor = child.walk();
            for inner in child.children(&mut inner_cursor) {
                if let Some(item) = node_to_item(inner, source, lang) {
                    items.push(item);
                }
            }
        }
    }
    items
}

pub fn print_text(items: &[OutlineItem]) {
    for item in items {
        print_item(item, 0);
    }
}

fn print_item(item: &OutlineItem, indent: usize) {
    let pad = "  ".repeat(indent);
    let vis = item
        .visibility
        .as_deref()
        .map(|v| format!("{v} "))
        .unwrap_or_default();
    let sig = item
        .signature
        .as_deref()
        .map(|s| format!("  -- {s}"))
        .unwrap_or_default();
    println!(
        "{pad}{vis}{} {} [{}:{}]{sig}",
        item.kind, item.name, item.start_line, item.end_line,
    );
    for child in &item.children {
        print_item(child, indent + 1);
    }
}
