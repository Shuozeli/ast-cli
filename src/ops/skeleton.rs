use std::path::Path;

use anyhow::Result;
use tree_sitter::Node;

use crate::languages::{self, Lang};

pub fn run(path: &Path) -> Result<String> {
    let (tree, source, lang) = languages::parse_file(path)?;
    let root = tree.root_node();
    let mut output = String::with_capacity(source.len());
    build_skeleton(root, &source, lang, &mut output);
    Ok(output)
}

fn build_skeleton(node: Node, source: &str, lang: Lang, output: &mut String) {
    let mut cursor = node.walk();
    let mut last_end = node.start_byte();

    for child in node.children(&mut cursor) {
        if is_function_body(child, lang) {
            output.push_str(&source[last_end..child.start_byte()]);
            output.push_str(lang.body_placeholder());
            last_end = child.end_byte();
        } else if has_nested_functions(child, lang) {
            output.push_str(&source[last_end..child.start_byte()]);
            build_skeleton(child, source, lang, output);
            last_end = child.end_byte();
        }
    }

    if last_end < node.end_byte() {
        output.push_str(&source[last_end..node.end_byte()]);
    }
}

fn is_function_body(node: Node, lang: Lang) -> bool {
    let parent_kind = node.parent().map(|p| p.kind()).unwrap_or("");
    match lang {
        Lang::Rust => {
            node.kind() == "block" && matches!(parent_kind, "function_item" | "closure_expression")
        }
        Lang::Cpp => {
            node.kind() == "compound_statement" && matches!(parent_kind, "function_definition")
        }
        Lang::Typescript | Lang::Tsx => {
            node.kind() == "statement_block"
                && matches!(
                    parent_kind,
                    "function_declaration" | "method_definition" | "arrow_function"
                )
        }
        Lang::Python => {
            // For Python, we replace the block inside function_definition
            node.kind() == "block" && parent_kind == "function_definition"
        }
        Lang::Protobuf => false, // protobuf has no function bodies
    }
}

fn has_nested_functions(node: Node, lang: Lang) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if is_function_body(child, lang) {
            return true;
        }
        if has_nested_functions(child, lang) {
            return true;
        }
    }
    false
}
