use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::LazyLock;
use tap::Pipe;

// Lazy-initialized regex patterns for validation
//
// SAFETY (BEAD-006): All regex patterns are hardcoded string literals verified to be valid.
// The `.expect()` calls will never panic - this is guaranteed by:
// 1. Patterns are compile-time constants (no user input)
// 2. All patterns are tested in tests/bead_006_regex_initialization_tests.rs
// 3. If a pattern were invalid, tests would fail immediately
//
// Using `.expect()` here is acceptable per BEAD-006 Option A: "Keep LazyLock + Add Compile-Time Test"
static H1_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^# [^#]").expect("valid H1 regex"));

static TAGS_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"tags:\s*\[[^\]]{10,}\]").expect("valid tags regex"));

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub files_checked: usize,
    pub files_passed: usize,
    pub total_errors: usize,
    pub total_warnings: usize,
}

/// Validate all files using functional composition with fold
pub fn validate_all(output_dir: &Path) -> Result<ValidationResult> {
    let docs_dir = output_dir.join("docs");

    if !docs_dir.exists() {
        return Ok(ValidationResult {
            files_checked: 0,
            files_passed: 0,
            total_errors: 0,
            total_warnings: 0,
        });
    }

    fs::read_dir(docs_dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
        .filter_map(|path| fs::read_to_string(&path).ok())
        .map(|content| validate_file(&content))
        .fold(
            (0usize, 0usize, 0usize, 0usize),
            |(checked, passed, errors, warnings), (e, w)| {
                (
                    checked.saturating_add(1),
                    passed.saturating_add(if e == 0 { 1 } else { 0 }),
                    errors.saturating_add(e),
                    warnings.saturating_add(w),
                )
            },
        )
        .pipe(
            |(files_checked, files_passed, total_errors, total_warnings)| {
                Ok(ValidationResult {
                    files_checked,
                    files_passed,
                    total_errors,
                    total_warnings,
                })
            },
        )
}

fn validate_file(content: &str) -> (usize, usize) {
    let mut errors: usize = 0;
    let mut warnings: usize = 0;

    // V001: single_h1
    let h1_count = H1_REGEX.find_iter(content).count();
    if h1_count != 1 {
        errors = errors.saturating_add(1);
    }

    // V002: frontmatter_exists
    if !content.starts_with("---") {
        errors = errors.saturating_add(1);
    }

    // V003: required_fields
    let required = ["id:", "title:", "category:", "tags:"];
    let search_chars = std::cmp::min(500, content.chars().count());
    let search_portion: String = content.chars().take(search_chars).collect();
    for field in &required {
        if !search_portion.contains(field) {
            errors = errors.saturating_add(1);
        }
    }

    // V006: min_tags
    if !TAGS_REGEX.is_match(content) {
        warnings = warnings.saturating_add(1);
    }

    // V007: has_context
    if !content.contains("> **Context**:") {
        warnings = warnings.saturating_add(1);
    }

    // V008: has_see_also
    if !content.contains("## See Also") {
        warnings = warnings.saturating_add(1);
    }

    (errors, warnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_h1_at_start() {
        let content = "---\nid: test\ntitle: Test\ncategory: ref\ntags: [\"test\", \"example\"]\n---\n\n# Title\n\nContent here.";
        let (errors, _warnings) = validate_file(content);
        // Should pass - has frontmatter and single H1
        assert_eq!(
            errors, 0,
            "Document with H1 at start should have 0 errors for H1 check"
        );
    }

    #[test]
    fn test_validate_h1_in_middle() {
        let content = "---\nid: test\ntitle: Test\ncategory: ref\ntags: [\"test\", \"example\"]\n---\n\nSome intro text.\n\n# Title\n\nBody text.";
        let (errors, _warnings) = validate_file(content);
        // Should pass - H1 exists even though not at very start
        assert_eq!(
            errors, 0,
            "Document with H1 in middle should have 0 errors for H1 check"
        );
    }

    #[test]
    fn test_validate_multiple_h1() {
        let content = "---\nid: test\ntitle: Test\ncategory: ref\ntags: [\"test\", \"example\"]\n---\n\n# One\n\n# Two\n\nContent.";
        let (errors, _warnings) = validate_file(content);
        // Should fail - has 2 H1s
        assert!(errors >= 1, "Document with multiple H1s should have errors");
    }

    #[test]
    fn test_validate_no_h1() {
        let content = "---\nid: test\ntitle: Test\ncategory: ref\ntags: [\"test\", \"example\"]\n---\n\n## Only H2\n\nContent.";
        let (errors, _warnings) = validate_file(content);
        // Should fail - no H1
        assert!(errors >= 1, "Document with no H1 should have errors");
    }
}
