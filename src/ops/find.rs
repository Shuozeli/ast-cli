use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::Serialize;

use crate::languages::Lang;
use crate::ops::outline::{self, OutlineItem};
use crate::ops::walk;

#[derive(Debug, Serialize)]
pub struct FindResult {
    pub file: PathBuf,
    pub kind: String,
    pub name: String,
    pub line: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

pub fn run(dir: &Path, name: &str, kind_filter: Option<&str>) -> Result<Vec<FindResult>> {
    let mut results = Vec::new();
    walk::walk_source_files(dir, &[], &mut |path| {
        if Lang::detect(path).is_err() {
            return Ok(());
        }
        let items = outline::run(path)?;
        collect_matches(&items, path, name, kind_filter, &mut results);
        Ok(())
    })?;
    Ok(results)
}

fn collect_matches(
    items: &[OutlineItem],
    path: &Path,
    name: &str,
    kind_filter: Option<&str>,
    results: &mut Vec<FindResult>,
) {
    for item in items {
        if item.name == name {
            if let Some(kf) = kind_filter
                && item.kind != kf
            {
                continue;
            }
            results.push(FindResult {
                file: path.to_path_buf(),
                kind: item.kind.clone(),
                name: item.name.clone(),
                line: item.start_line,
                visibility: item.visibility.clone(),
                signature: item.signature.clone(),
            });
        }
        // Search children too
        collect_matches(&item.children, path, name, kind_filter, results);
    }
}

pub fn print_text(results: &[FindResult]) {
    if results.is_empty() {
        println!("No matches found.");
        return;
    }
    for r in results {
        let vis = r
            .visibility
            .as_deref()
            .map(|v| format!(" ({v})"))
            .unwrap_or_default();
        let sig = r
            .signature
            .as_deref()
            .map(|s| format!("  -- {s}"))
            .unwrap_or_default();
        println!(
            "{}:{} {} {}{vis}{sig}",
            r.file.display(),
            r.line,
            r.kind,
            r.name,
        );
    }
}
