use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::LazyLock;
use thiserror::Error;

// Lazy-initialized regex patterns for validation
//
// These patterns are hardcoded and verified by tests, but we use Option
// to maintain the zero-panic guarantee across all code.
static H1_REGEX: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"(?m)^# [^#]").ok());

static TAGS_REGEX: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"tags:\s*\[[^\]]{10,}\]").ok());

/// Get H1 regex or return error if compilation failed
fn h1_regex() -> Result<&'static Regex, anyhow::Error> {
    H1_REGEX
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("H1 regex failed to compile"))
}

/// Get tags regex or return error if compilation failed
fn tags_regex() -> Result<&'static Regex, anyhow::Error> {
    TAGS_REGEX
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Tags regex failed to compile"))
}

/// Query validation errors
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ValidationError {
    #[error("Query cannot be empty")]
    EmptyQuery,

    #[error("Query too long ({length} bytes, max {max})")]
    QueryTooLong { length: usize, max: usize },

    #[error("Regex queries not allowed (potential ReDoS attack)")]
    RegexNotAllowed,

    #[error("Limit must be positive (cannot return negative results), got {0}")]
    InvalidLimitNegative(i64),

    #[error("limit must be at least 1 (use --limit 1 or higher)")]
    InvalidLimitZero,

    #[error("limit must be at most 1000 results, got {0}")]
    InvalidLimitTooLarge(i64),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileValidationResult {
    pub file_path: String,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub files_checked: usize,
    pub files_passed: usize,
    pub total_errors: usize,
    pub total_warnings: usize,
    pub failed_files: Vec<FileValidationResult>,
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
            failed_files: Vec::new(),
        });
    }

    let file_results = collect_validation_results(&docs_dir)?;
    let summary = summarize_results(&file_results);
    let failed_files = file_results
        .into_iter()
        .filter(|(_, e, w)| !e.is_empty() || !w.is_empty())
        .map(|(file_path, errors, warnings)| FileValidationResult {
            file_path,
            errors,
            warnings,
        })
        .collect();

    Ok(ValidationResult {
        files_checked: summary.files_checked,
        files_passed: summary.files_passed,
        total_errors: summary.total_errors,
        total_warnings: summary.total_warnings,
        failed_files,
    })
}

type ValidationEntry = (String, Vec<String>, Vec<String>);

fn collect_validation_results(docs_dir: &Path) -> Result<Vec<ValidationEntry>> {
    fs::read_dir(docs_dir)?
        .filter_map(std::result::Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
        .map(validate_path)
        .collect()
}

fn validate_path(path: std::path::PathBuf) -> Result<ValidationEntry> {
    let path_str = path.display().to_string();
    match fs::read_to_string(&path) {
        Ok(content) => {
            let (errors, warnings) = validate_file(&content);
            Ok((path_str, errors, warnings))
        }
        Err(err) => Ok((
            path_str,
            vec![format!("Failed to read file: {err}")],
            Vec::new(),
        )),
    }
}

struct ValidationSummary {
    files_checked: usize,
    files_passed: usize,
    total_errors: usize,
    total_warnings: usize,
}

fn summarize_results(file_results: &[ValidationEntry]) -> ValidationSummary {
    let files_checked = file_results.len();
    let total_errors = file_results.iter().map(|(_, e, _)| e.len()).sum();
    let total_warnings = file_results.iter().map(|(_, _, w)| w.len()).sum();
    let files_passed = file_results.iter().filter(|(_, e, _)| e.is_empty()).count();

    ValidationSummary {
        files_checked,
        files_passed,
        total_errors,
        total_warnings,
    }
}

fn validate_file(content: &str) -> (Vec<String>, Vec<String>) {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // V001: single_h1
    let h1_count = h1_regex()
        .map(|regex| regex.find_iter(content).count())
        .unwrap_or(0);
    if h1_count == 0 {
        errors.push("Missing H1 heading".to_string());
    } else if h1_count > 1 {
        errors.push(format!(
            "Multiple H1 headings found ({h1_count}), should have exactly one"
        ));
    }

    // V002: frontmatter_exists
    if !content.starts_with("---") {
        errors.push("Missing frontmatter (should start with ---)".to_string());
    }

    // V003: required_fields
    let required = ["id:", "title:", "category:", "tags:"];
    let search_chars = std::cmp::min(500, content.chars().count());
    let search_portion: String = content.chars().take(search_chars).collect();
    for field in &required {
        if !search_portion.contains(field) {
            errors.push(format!("Missing required field: {field}"));
        }
    }

    // V006: min_tags
    let has_sufficient_tags = tags_regex()
        .map(|regex| regex.is_match(content))
        .unwrap_or(false);
    if !has_sufficient_tags {
        warnings.push("Insufficient tags (should have at least 10 characters of tags)".to_string());
    }

    // V007: has_context
    if !content.contains("> **Context**:") {
        warnings.push("Missing context section (> **Context**:)".to_string());
    }

    // V008: has_see_also
    if !content.contains("## See Also") {
        warnings.push("Missing 'See Also' section".to_string());
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
/// - Queries < 1 char (trimmed) rejected with `EmptyQuery`
/// - Queries > 1000 bytes rejected with `QueryTooLong`
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

    if contains_regex_pattern(trimmed) {
        return Err(ValidationError::RegexNotAllowed);
    }

    Ok(trimmed)
}

/// Validate a limit value (must be greater than 0)
/// Used by internal library functions to prevent Tantivy panics
pub fn validate_limit(s: &str) -> Result<usize, ValidationError> {
    let value = s
        .parse::<i64>()
        .map_err(|_| ValidationError::InvalidLimitNegative(0))?;

    if value < 0 {
        return Err(ValidationError::InvalidLimitNegative(value));
    }

    if value == 0 {
        return Err(ValidationError::InvalidLimitZero);
    }

    if value > 1000 {
        return Err(ValidationError::InvalidLimitTooLarge(value));
    }

    value
        .try_into()
        .map_err(|_| ValidationError::InvalidLimitTooLarge(value))
}

fn contains_regex_pattern(query: &str) -> bool {
    let chars: Vec<char> = query.chars().collect();
    let len = chars.len();

    for i in 0..len {
        if chars[i] == '/' {
            let next_idx = i.saturating_add(1);
            if next_idx < len && chars[next_idx] == '/' {
                continue;
            }
            let start_idx = i.saturating_add(1);
            for j in start_idx..len {
                if chars[j] == '/' {
                    let next_j = j.saturating_add(1);
                    if next_j >= len || chars[next_j] != '/' {
                        return true;
                    }
                    break;
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    use tempfile::TempDir;

    #[test]
    fn test_validate_all_reports_read_error() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let docs_dir = temp_dir.path().join("docs");
        fs::create_dir_all(&docs_dir)?;

        let file_path = docs_dir.join("bad.md");
        fs::write(&file_path, "---\n")?;

        let mut perms = fs::metadata(&file_path)?.permissions();
        perms.set_mode(0o000);
        fs::set_permissions(&file_path, perms)?;

        let result = validate_all(temp_dir.path())?;

        let mut restore_perms = fs::metadata(&file_path)?.permissions();
        restore_perms.set_mode(0o644);
        fs::set_permissions(&file_path, restore_perms)?;

        assert_eq!(result.files_checked, 1);
        assert_eq!(result.total_errors, 1);
        assert!(result.failed_files[0]
            .errors
            .first()
            .is_some_and(|msg| msg.contains("Failed to read")));

        Ok(())
    }

    #[test]
    fn test_validate_h1_at_start() {
        let content = "---\nid: test\ntitle: Test\ncategory: ref\ntags: [\"test\", \"example\"]\n---\n\n# Title\n\nContent here.";
        let (errors, _warnings) = validate_file(content);
        // Should pass - has frontmatter and single H1
        assert_eq!(
            errors.len(),
            0,
            "Document with H1 at start should have 0 errors for H1 check"
        );
    }

    #[test]
    fn test_validate_h1_in_middle() {
        let content = "---\nid: test\ntitle: Test\ncategory: ref\ntags: [\"test\", \"example\"]\n---\n\nSome intro text.\n\n# Title\n\nBody text.";
        let (errors, _warnings) = validate_file(content);
        // Should pass - H1 exists even though not at very start
        assert_eq!(
            errors.len(),
            0,
            "Document with H1 in middle should have 0 errors for H1 check"
        );
    }

    #[test]
    fn test_validate_multiple_h1() {
        let content = "---\nid: test\ntitle: Test\ncategory: ref\ntags: [\"test\", \"example\"]\n---\n\n# One\n\n# Two\n\nContent.";
        let (errors, _warnings) = validate_file(content);
        // Should fail - has 2 H1s
        assert!(
            !errors.is_empty(),
            "Document with multiple H1s should have errors"
        );
        assert!(
            errors.iter().any(|e| e.contains("Multiple H1")),
            "Should report multiple H1s"
        );
    }

    #[test]
    fn test_validate_no_h1() {
        let content = "---\nid: test\ntitle: Test\ncategory: ref\ntags: [\"test\", \"example\"]\n---\n\n## Only H2\n\nContent.";
        let (errors, _warnings) = validate_file(content);
        // Should fail - no H1
        assert!(!errors.is_empty(), "Document with no H1 should have errors");
        assert!(
            errors.iter().any(|e| e.contains("Missing H1")),
            "Should report missing H1"
        );
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
        assert_eq!(validate_query("a"), Ok("a"));
    }

    #[test]
    fn test_validate_query_normal() {
        assert_eq!(validate_query("rust programming"), Ok("rust programming"));
    }

    #[test]
    fn test_validate_query_trimmed() {
        assert_eq!(
            validate_query("  rust programming  "),
            Ok("rust programming")
        );
    }

    #[test]
    fn test_validate_query_at_limit() {
        let query = "a".repeat(1000);
        let result = validate_query(&query);
        assert!(result.is_ok());
        assert_eq!(result.map(|s| s.len()), Ok(1000));
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
        assert_eq!(validate_query("café rust"), Ok("café rust"));
    }

    #[test]
    fn test_validate_query_unicode_at_limit() {
        // Euro sign "€" is 3 bytes, so 333 reps = 999 bytes + "a" = 1000 bytes
        let query = format!("{}a", "€".repeat(333));
        assert_eq!(query.len(), 1000);
        assert!(validate_query(&query).is_ok());
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
        assert_eq!(
            validate_query("rust-lang & systems *2025*"),
            Ok("rust-lang & systems *2025*")
        );
    }

    #[test]
    fn test_validate_query_error_message_empty() {
        let result = validate_query("");
        assert!(result.is_err());
        // Convert error to string for message validation
        let err_str = result.as_ref().map_err(|e| e.to_string());
        assert!(matches!(err_str, Err(ref msg) if msg == "Query cannot be empty"));
    }

    #[test]
    fn test_validate_query_error_message_too_long() {
        let query = "a".repeat(1001);
        let result = validate_query(&query);
        assert!(result.is_err());
        // Convert error to string for message validation
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("1001"));
            assert!(msg.contains("1000"));
            assert!(msg.contains("too long"));
        }
    }

    #[test]
    fn test_validate_query_rejects_simple_regex() {
        assert!(matches!(
            validate_query("/rust/"),
            Err(ValidationError::RegexNotAllowed)
        ));
    }

    #[test]
    fn test_validate_query_rejects_nested_regex() {
        assert!(matches!(
            validate_query("/((a+)+)b/"),
            Err(ValidationError::RegexNotAllowed)
        ));
    }

    #[test]
    fn test_validate_query_rejects_regex_in_middle() {
        assert!(matches!(
            validate_query("search /rust/ programming"),
            Err(ValidationError::RegexNotAllowed)
        ));
    }

    #[test]
    fn test_validate_query_rejects_multiple_regex() {
        assert!(matches!(
            validate_query("/rust/ OR /python/"),
            Err(ValidationError::RegexNotAllowed)
        ));
    }

    #[test]
    fn test_validate_query_accepts_slash_without_regex() {
        assert_eq!(validate_query("rust/python"), Ok("rust/python"));
    }

    #[test]
    fn test_validate_query_accepts_double_slash() {
        assert_eq!(validate_query("// comment"), Ok("// comment"));
    }

    #[test]
    fn test_validate_query_accepts_double_slash_at_end() {
        assert_eq!(validate_query("rust //"), Ok("rust //"));
    }

    #[test]
    fn test_validate_query_rejects_regex_error_message() {
        let result = validate_query("/((a+)+)b/");
        assert!(result.is_err());
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        assert!(matches!(err_msg, Err(ref msg) if msg.contains("Regex")));
    }

    // ============================================================================
    // Limit validation tests
    // ============================================================================

    #[test]
    fn test_validate_limit_zero() {
        let result = validate_limit("0");
        assert!(result.is_err());
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        assert!(matches!(err_msg, Err(ref msg) if msg.contains("at least 1")));
    }

    #[test]
    fn test_validate_limit_one() {
        assert_eq!(validate_limit("1"), Ok(1));
    }

    #[test]
    fn test_validate_limit_normal() {
        assert_eq!(validate_limit("10"), Ok(10));
        assert_eq!(validate_limit("100"), Ok(100));
        assert_eq!(validate_limit("1000"), Ok(1000));
    }

    #[test]
    fn test_validate_limit_negative() {
        let result = validate_limit("-1");
        assert!(result.is_err());
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        assert!(matches!(err_msg, Err(ref msg) if msg.contains("positive")));
    }

    #[test]
    fn test_validate_limit_too_large() {
        let result = validate_limit("1001");
        assert!(result.is_err());
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        assert!(matches!(err_msg, Err(ref msg) if msg.contains("at most 1000")));
    }

    #[test]
    fn test_validate_limit_error_message() {
        let result = validate_limit("0");
        assert!(result.is_err());
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        assert!(matches!(err_msg, Err(ref msg) if msg.contains("at least 1")));
    }
}
