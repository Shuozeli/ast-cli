use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::Serialize;

use crate::languages::Lang;
use crate::ops::outline;
use crate::ops::walk;

#[derive(Debug, Serialize)]
pub struct ProjectSummary {
    pub files: Vec<FileSummary>,
    pub total_files: usize,
    pub total_lines: usize,
}

#[derive(Debug, Serialize)]
pub struct FileSummary {
    pub path: PathBuf,
    pub language: String,
    pub lines: usize,
    pub functions: usize,
    pub types: usize,
    pub tests: usize,
}

pub fn run(dir: &Path, exclude: &[String]) -> Result<ProjectSummary> {
    let mut files = Vec::new();
    walk::walk_source_files(dir, exclude, &mut |path| {
        if let Ok(lang) = Lang::detect(path) {
            match outline::run(path) {
                Ok(items) => {
                    let source = std::fs::read_to_string(path)?;
                    let lines = source.lines().count();
                    let mut functions = 0;
                    let mut types = 0;
                    let mut tests = 0;
                    count_items(&items, &mut functions, &mut types, &mut tests);
                    files.push(FileSummary {
                        path: path.to_path_buf(),
                        language: lang.name().to_string(),
                        lines,
                        functions,
                        types,
                        tests,
                    });
                }
                Err(e) => {
                    eprintln!("warning: skipping {}: {e}", path.display());
                }
            }
        }
        Ok(())
    })?;

    let total_files = files.len();
    let total_lines: usize = files.iter().map(|f| f.lines).sum();

    files.sort_by(|a, b| a.path.cmp(&b.path));

    Ok(ProjectSummary {
        files,
        total_files,
        total_lines,
    })
}

fn count_items(
    items: &[outline::OutlineItem],
    functions: &mut usize,
    types: &mut usize,
    tests: &mut usize,
) {
    for item in items {
        match item.kind.as_str() {
            "function" | "method" | "rpc" => {
                *functions += 1;
                if item.name.starts_with("test_") || item.name.starts_with("test ") {
                    *tests += 1;
                }
            }
            "struct" | "class" | "enum" | "trait" | "interface" | "message" | "type_alias" => {
                *types += 1;
            }
            _ => {}
        }
        count_items(&item.children, functions, types, tests);
    }
}

pub fn print_text(summary: &ProjectSummary) {
    println!(
        "{} files, {} lines\n",
        summary.total_files, summary.total_lines
    );
    for f in &summary.files {
        println!(
            "  {:50} {:>10}  {:>4} lines  {:>3} fn  {:>3} types  {:>2} tests",
            f.path.display(),
            f.language,
            f.lines,
            f.functions,
            f.types,
            f.tests,
        );
    }
}
