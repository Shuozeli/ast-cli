use std::path::Path;

use anyhow::Result;

const SKIP_DIRS: &[&str] = &[
    "target",
    "node_modules",
    ".git",
    "__pycache__",
    "build",
    "dist",
    ".next",
];

/// Walk a directory recursively, calling `cb` for each non-directory entry.
/// Skips hidden directories and common non-source directories.
/// Additional patterns can be excluded via `exclude`.
pub fn walk_source_files(
    dir: &Path,
    exclude: &[String],
    cb: &mut dyn FnMut(&Path) -> Result<()>,
) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // Skip hidden directories
        if name.starts_with('.') {
            continue;
        }
        // Skip common non-source directories
        if SKIP_DIRS.contains(&name) {
            continue;
        }
        // Check exclude patterns
        if !exclude.is_empty() {
            let path_str = path.to_string_lossy();
            if exclude.iter().any(|pat| matches_exclude(&path_str, pat)) {
                continue;
            }
        }

        if path.is_dir() {
            walk_source_files(&path, exclude, cb)?;
        } else {
            cb(&path)?;
        }
    }
    Ok(())
}

fn matches_exclude(path: &str, pattern: &str) -> bool {
    if let Some(stripped) = pattern.strip_suffix('/') {
        path.contains(pattern) || path.contains(stripped)
    } else if let Some(stripped) = pattern.strip_prefix('*') {
        path.ends_with(stripped)
    } else {
        path.contains(pattern)
    }
}
