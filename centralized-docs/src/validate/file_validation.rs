use anyhow::Result;
use regex::Regex;
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

use super::types::{FileValidationResult, ValidationResult};

// Pre-compiled regex patterns to avoid compilation on every check.
//
// These patterns are hardcoded and verified by tests, but we use Option
// to maintain the zero-panic guarantee across all code.
static TAGS_REGEX: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"tags:\s*\[[^\]]{10,}\]").ok());

/// Get tags regex or return error if compilation failed
fn tags_regex() -> Result<&'static Regex, anyhow::Error> {
    TAGS_REGEX
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Tags regex failed to compile"))
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
    let results = fs::read_dir(docs_dir)?
        .filter_map(std::result::Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
        .map(|path| validate_path(&path))
        .collect();
    Ok(results)
}

fn validate_path(path: &std::path::Path) -> ValidationEntry {
    let path_str = path.display().to_string();
    match fs::read_to_string(path) {
        Ok(content) => {
            let (errors, warnings) = validate_file(&content);
            (path_str, errors, warnings)
        }
        Err(err) => (
            path_str,
            vec![format!("Failed to read file: {err}")],
            Vec::new(),
        ),
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
    let errors: Vec<String> = check_h1_errors(content)
        .into_iter()
        .chain(check_frontmatter_error(content))
        .chain(check_required_field_errors(content))
        .collect();

    let warnings = [
        check_tags_sufficient(content),
        check_context_section(content),
        check_see_also_section(content),
    ]
    .into_iter()
    .flatten()
    .collect();

    (errors, warnings)
}

fn check_h1_errors(content: &str) -> Option<String> {
    let h1_count = pulldown_cmark::Parser::new(content)
        .filter(|event| {
            matches!(
                event,
                pulldown_cmark::Event::Start(pulldown_cmark::Tag::Heading {
                    level: pulldown_cmark::HeadingLevel::H1,
                    ..
                })
            )
        })
        .count();

    match h1_count {
        0 => Some("Missing H1 heading".to_string()),
        n if n > 1 => Some(format!(
            "Multiple H1 headings found ({n}), should have exactly one"
        )),
        _ => None,
    }
}

fn check_frontmatter_error(content: &str) -> Option<String> {
    (!content.starts_with("---")).then(|| "Missing frontmatter (should start with ---)".to_string())
}

fn check_required_field_errors(content: &str) -> Vec<String> {
    let required = ["id:", "title:", "category:", "tags:"];
    let search_chars = std::cmp::min(500, content.chars().count());
    let search_portion: String = content.chars().take(search_chars).collect();
    required
        .iter()
        .filter(|field| !search_portion.contains(*field))
        .map(|field| format!("Missing required field: {field}"))
        .collect()
}

fn check_tags_sufficient(content: &str) -> Option<String> {
    let has_sufficient_tags = tags_regex().is_ok_and(|regex| regex.is_match(content));
    (!has_sufficient_tags)
        .then(|| "Insufficient tags (should have at least 10 characters of tags)".to_string())
}

fn check_context_section(content: &str) -> Option<String> {
    (!content.contains("> **Context**:"))
        .then(|| "Missing context section (> **Context**:)".to_string())
}

fn check_see_also_section(content: &str) -> Option<String> {
    (!content.contains("## See Also")).then(|| "Missing 'See Also' section".to_string())
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
        assert!(!errors.is_empty(), "Document with no H1 should have errors");
        assert!(
            errors.iter().any(|e| e.contains("Missing H1")),
            "Should report missing H1"
        );
    }
}
