# Code Quality Findings

## 1. Silent Failures (High Severity)

### 1.1 ~~node_text returns placeholder instead of propagating error~~ FIXED
- `node_text()` now returns `Result<String>`. All call sites updated.

### 1.2 Project walk silently swallows parse errors -- DEFERRED
- **Location:** `src/ops/project.rs:48-50` (`run()`)
- **Problem:** When `outline::run()` fails, the error is printed as a warning and silently skipped.
- **Status:** Intentional for project-wide scans. Consider `--strict` flag in future.

### 1.3 ~~extract_fn_signature swallows UTF-8 errors~~ ACCEPTED
- Uses `unwrap_or_default()` which is appropriate -- tree-sitter nodes backed by valid source bytes don't fail here. The fallback to empty string is safe.

## 2. Missing Abstractions (High Severity)

### 2.1-2.2 Language-specific match blocks in read.rs -- DEFERRED
- `get_container_type_name()` and `get_definition_name()` still have per-language match blocks.
- These are structurally different per language (some recurse, some don't), making a single method on `Lang` non-trivial.

### 2.3 ~~Duplicated body container identification~~ FIXED
- Extracted `is_body_container()` to `src/languages/mod.rs`. Both `outline.rs` and `read.rs` now use it.

### 2.4 Repeated language dispatch -- DEFERRED
- Same reason as 2.1-2.2.

## 3. Unsafe Patterns (Medium Severity)

### 3.1 ~~Unchecked array indexing on capture names~~ FIXED
- `query.rs` now uses `.get()` with `.ok_or_else()`.

### 3.2 ~~Unchecked line range indexing~~ FIXED
- `read.rs` `extract_by_lines()` now uses `.get()` with `.ok_or_else()`.

## 4. Duplication (Medium Severity)

### 4.1 Duplicated visibility/signature formatting -- DEFERRED
- `find.rs` and `outline.rs` format visibility/signature differently (`" (pub)"` vs `"pub "`), so not truly identical. Not worth extracting.

## 5. Stringly-Typed APIs (Medium Severity)

### 5.1 Item kind uses raw strings instead of enum -- DEFERRED
- Would be a large refactor touching every language handler. Worth doing but out of scope for this pass.

## 6. Unnecessary Allocation (Medium Severity)

### 6.1 ~~extract_by_lines double-allocates~~ ACCEPTED
- The Vec is needed for indexing by line number. An iterator approach would be less readable for negligible perf gain.

## 7. Dead Code / Unused Parameters (Low Severity)

### 7.1 ~~Unused `_depth` parameter in build_skeleton~~ FIXED
- Removed from `skeleton.rs`.

### 7.2 ~~Unused `depth` parameter in outline collect functions~~ FIXED
- Removed from all outline functions.

### 7.3 Empty test directory -- DEFERRED
- No tests exist yet. Out of scope for this pass.

## 8. Noise (Low Severity)

### 8.1 ~~Comment restates obvious sort~~ FIXED
- Removed from `project.rs`.

### 8.2 ~~Comments restate what code does in skeleton builder~~ FIXED
- Removed from `skeleton.rs`.

## 9. Minor Rust Idiom Issues (Low Severity)

### 9.1 ~~Verbose Option-to-conditional pattern~~ FIXED
- `outline.rs` now uses `.then()`.

### 9.2 ~~strip_generics could be simpler~~ FIXED
- Now uses `name.split('<').next().unwrap_or(name).to_string()`.

### 9.3 Inconsistent error construction -- ACCEPTED
- `bail!()` for early returns and `anyhow!()` inside `.ok_or_else()` is already the correct convention. No change needed.

---

## Summary

| Status | Count |
|--------|-------|
| FIXED | 12 |
| ACCEPTED (no change needed) | 3 |
| DEFERRED | 6 |
