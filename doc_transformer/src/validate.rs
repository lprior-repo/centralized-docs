use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::LazyLock;
use tap::Pipe;
use thiserror::Error;

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

/// Query validation errors
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ValidationError {
    #[error("Query cannot be empty")]
    EmptyQuery,

    #[error("Query too long ({length} bytes, max {max})")]
    QueryTooLong { length: usize, max: usize },
}

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

/// Validate query length for search operations
///
/// ## Design by Contract
///
/// **Preconditions:**
/// - Query may be any length (including 0)
/// - Validation happens before expensive operations
///
/// **Postconditions:**
/// - Queries < 1 char (trimmed) rejected with EmptyQuery
/// - Queries > 1000 bytes rejected with QueryTooLong
/// - Valid queries (1-1000 bytes) return Ok with trimmed query
///
/// **Invariants:**
/// - No expensive operations on invalid input
/// - Error messages are user-friendly
/// - Validation is consistent across all entry points
///
/// ## Error Handling
///
/// Returns `ValidationError` for invalid queries:
/// - `EmptyQuery`: Query is empty or whitespace-only after trimming
/// - `QueryTooLong`: Query exceeds 1000 byte limit
///
/// ## Example
///
/// ```
/// use doc_transformer::validate::{validate_query, ValidationError};
///
/// // Valid query
/// assert!(validate_query("rust programming").is_ok());
///
/// // Empty query
/// assert!(matches!(validate_query(""), Err(ValidationError::EmptyQuery)));
/// assert!(matches!(validate_query("   "), Err(ValidationError::EmptyQuery)));
///
/// // Too long query
/// let long = "a".repeat(1001);
/// assert!(matches!(validate_query(&long), Err(ValidationError::QueryTooLong{..})));
/// ```
pub fn validate_query(query: &str) -> Result<&str, ValidationError> {
    const MAX_QUERY_LENGTH: usize = 1000;

    let trimmed = query.trim();

    if trimmed.is_empty() {
        return Err(ValidationError::EmptyQuery);
    }

    if trimmed.len() > MAX_QUERY_LENGTH {
        return Err(ValidationError::QueryTooLong {
            length: trimmed.len(),
            max: MAX_QUERY_LENGTH,
        });
    }

    Ok(trimmed)
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

    // ============================================================================
    // Query validation tests
    // ============================================================================

    #[test]
    fn test_validate_query_empty() {
        let result = validate_query("");
        assert!(matches!(result, Err(ValidationError::EmptyQuery)));
    }

    #[test]
    fn test_validate_query_whitespace_only() {
        let result = validate_query("   ");
        assert!(matches!(result, Err(ValidationError::EmptyQuery)));
    }

    #[test]
    fn test_validate_query_tabs_and_newlines() {
        let result = validate_query("\t\n  \r\n");
        assert!(matches!(result, Err(ValidationError::EmptyQuery)));
    }

    #[test]
    fn test_validate_query_single_char() {
        let result = validate_query("a");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "a");
    }

    #[test]
    fn test_validate_query_normal() {
        let result = validate_query("rust programming");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "rust programming");
    }

    #[test]
    fn test_validate_query_trimmed() {
        let result = validate_query("  rust programming  ");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "rust programming");
    }

    #[test]
    fn test_validate_query_at_limit() {
        let query = "a".repeat(1000);
        let result = validate_query(&query);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1000);
    }

    #[test]
    fn test_validate_query_exceeds_limit() {
        let query = "a".repeat(1001);
        let result = validate_query(&query);
        assert!(matches!(
            result,
            Err(ValidationError::QueryTooLong {
                length: 1001,
                max: 1000
            })
        ));
    }

    #[test]
    fn test_validate_query_far_exceeds_limit() {
        let query = "a".repeat(5000);
        let result = validate_query(&query);
        assert!(matches!(
            result,
            Err(ValidationError::QueryTooLong {
                length: 5000,
                max: 1000
            })
        ));
    }

    #[test]
    fn test_validate_query_unicode() {
        let result = validate_query("café rust");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "café rust");
    }

    #[test]
    fn test_validate_query_unicode_at_limit() {
        // Euro sign "€" is 3 bytes, so 333 reps = 999 bytes + "a" = 1000 bytes
        let query = format!("{}a", "€".repeat(333));
        assert_eq!(query.len(), 1000);
        let result = validate_query(&query);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_query_unicode_exceeds_limit() {
        // Euro sign "€" is 3 bytes, 334 reps = 1002 bytes
        let query = "€".repeat(334);
        assert_eq!(query.len(), 1002);
        let result = validate_query(&query);
        assert!(matches!(
            result,
            Err(ValidationError::QueryTooLong {
                length: 1002,
                max: 1000
            })
        ));
    }

    #[test]
    fn test_validate_query_special_chars() {
        let result = validate_query("rust-lang & systems *2025*");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "rust-lang & systems *2025*");
    }

    #[test]
    fn test_validate_query_error_message_empty() {
        let result = validate_query("");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Query cannot be empty");
    }

    #[test]
    fn test_validate_query_error_message_too_long() {
        let query = "a".repeat(1001);
        let result = validate_query(&query);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("1001"));
        assert!(err.to_string().contains("1000"));
        assert!(err.to_string().contains("too long"));
    }
}
