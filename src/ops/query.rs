use std::path::Path;

use anyhow::Result;
use serde::Serialize;
use tree_sitter::StreamingIterator;

use crate::languages;

#[derive(Debug, Serialize)]
pub struct QueryMatch {
    pub capture_name: String,
    pub text: String,
    pub start_line: usize,
    pub end_line: usize,
    pub start_col: usize,
    pub end_col: usize,
}

pub fn run(path: &Path, query_str: &str) -> Result<Vec<QueryMatch>> {
    let (tree, source, lang) = languages::parse_file(path)?;
    let ts_lang = lang.tree_sitter_language();
    let query = tree_sitter::Query::new(&ts_lang, query_str)?;
    let mut cursor = tree_sitter::QueryCursor::new();
    let root = tree.root_node();
    let mut matches = cursor.matches(&query, root, source.as_bytes());

    let capture_names = query.capture_names();
    let mut results = Vec::new();

    while let Some(m) = matches.next() {
        for capture in m.captures {
            let node = capture.node;
            let name = capture_names
                .get(capture.index as usize)
                .ok_or_else(|| anyhow::anyhow!("invalid capture index: {}", capture.index))?;
            let text = node.utf8_text(source.as_bytes())?.to_string();
            results.push(QueryMatch {
                capture_name: name.to_string(),
                text,
                start_line: node.start_position().row + 1,
                end_line: node.end_position().row + 1,
                start_col: node.start_position().column,
                end_col: node.end_position().column,
            });
        }
    }

    Ok(results)
}

pub fn print_text(results: &[QueryMatch]) {
    if results.is_empty() {
        println!("No matches.");
        return;
    }
    for r in results {
        let loc = if r.start_line == r.end_line {
            format!("{}:{}", r.start_line, r.start_col)
        } else {
            format!(
                "{}:{}-{}:{}",
                r.start_line, r.start_col, r.end_line, r.end_col
            )
        };
        // Truncate long text for display
        let display_text = if r.text.len() > 80 {
            let boundary = r.text.floor_char_boundary(77);
            format!("{}...", &r.text[..boundary])
        } else {
            r.text.clone()
        };
        println!("@{} [{}] {}", r.capture_name, loc, display_text);
    }
}
