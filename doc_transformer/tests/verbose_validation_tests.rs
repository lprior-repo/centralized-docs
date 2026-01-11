/// Tests for detailed validation reporting (verbose flag feature)

use std::fs;
use tempfile::TempDir;

fn create_test_file(temp_dir: &TempDir, filename: &str, content: &str) {
    let docs_dir = temp_dir.path().join("docs");
    fs::create_dir_all(&docs_dir).unwrap();
    fs::write(docs_dir.join(filename), content).unwrap();
}

#[test]
fn test_detailed_validation_captures_rule_id() {
    let temp_dir = TempDir::new().unwrap();
    let content = "No frontmatter here\n# Title\n";
    create_test_file(&temp_dir, "test.md", content);

    let result = doc_transformer::validate::validate_all(temp_dir.path()).unwrap();

    // Should have issues
    assert!(!result.issues.is_empty());

    // Should capture V002 (missing frontmatter)
    assert!(result.issues.iter().any(|i| i.rule_id == "V002"));
}

#[test]
fn test_multiple_issues_same_file() {
    let temp_dir = TempDir::new().unwrap();
    // Missing frontmatter, has H1 but missing required fields
    let content = "---\n---\n# Title\n";
    create_test_file(&temp_dir, "test.md", content);

    let result = doc_transformer::validate::validate_all(temp_dir.path()).unwrap();

    // Should have multiple errors (missing id, title, category, tags)
    assert!(result.total_errors >= 2);

    // Issues should be from the same file
    let file_issues: Vec<_> = result.issues.iter()
        .filter(|i| i.file == "test.md")
        .collect();
    assert!(file_issues.len() >= 2);
}

#[test]
fn test_validation_result_includes_issues_list() {
    let temp_dir = TempDir::new().unwrap();
    let valid_doc = r#"---
id: test/doc
title: Test Document
category: tutorial
tags: [example, testing, markdown]
---

# Test Document

> **Context**: This is a test document.

Content here with enough words to pass validation.
More content to ensure we meet word count requirements.
Even more content.

## See Also

- Related docs
"#;
    create_test_file(&temp_dir, "valid.md", valid_doc);

    let result = doc_transformer::validate::validate_all(temp_dir.path()).unwrap();

    // Should have issues field
    assert_eq!(result.files_checked, 1);

    // If no errors, file should pass
    if result.total_errors == 0 {
        assert_eq!(result.files_passed, 1);
    }

    // Valid document might still have warnings
    let warnings = result.issues.iter()
        .filter(|i| i.severity == doc_transformer::validate::Severity::Warning)
        .count();
    assert_eq!(warnings, result.total_warnings);
}

#[test]
fn test_validation_issue_structure() {
    let temp_dir = TempDir::new().unwrap();
    let content = "# Title without frontmatter\n";
    create_test_file(&temp_dir, "broken.md", content);

    let result = doc_transformer::validate::validate_all(temp_dir.path()).unwrap();

    assert!(!result.issues.is_empty());

    let issue = &result.issues[0];
    // Check that issue has required fields
    assert!(!issue.file.is_empty());
    assert!(!issue.rule_id.is_empty());
    assert!(!issue.message.is_empty());

    // Severity should be either Error or Warning
    use doc_transformer::validate::Severity;
    match issue.severity {
        Severity::Error | Severity::Warning => {},
    }
}

#[test]
fn test_v001_single_h1_validation() {
    let temp_dir = TempDir::new().unwrap();
    let content = r#"---
id: test
title: Test
category: ref
tags: [test, example]
---

## No H1
"#;
    create_test_file(&temp_dir, "no-h1.md", content);

    let result = doc_transformer::validate::validate_all(temp_dir.path()).unwrap();

    // Should have V001 error
    let v001_errors: Vec<_> = result.issues.iter()
        .filter(|i| i.rule_id == "V001")
        .collect();
    assert_eq!(v001_errors.len(), 1);
    assert_eq!(v001_errors[0].severity, doc_transformer::validate::Severity::Error);
}

#[test]
fn test_v003_missing_required_fields() {
    let temp_dir = TempDir::new().unwrap();
    let content = "---\ntitle: Test\n---\n# Test\n";
    create_test_file(&temp_dir, "missing-fields.md", content);

    let result = doc_transformer::validate::validate_all(temp_dir.path()).unwrap();

    // Should have errors for missing id, category, tags
    assert!(result.total_errors >= 3);

    // Should have V003 errors
    let v003_errors: Vec<_> = result.issues.iter()
        .filter(|i| i.rule_id == "V003")
        .collect();
    assert!(!v003_errors.is_empty());
}

#[test]
fn test_warnings_dont_fail_file() {
    let temp_dir = TempDir::new().unwrap();
    let content = r#"---
id: test
title: Test
category: ref
tags: [test-tags]
---

# Test

No context block
No See Also
"#;
    create_test_file(&temp_dir, "warnings-only.md", content);

    let result = doc_transformer::validate::validate_all(temp_dir.path()).unwrap();

    // Should have 1 file checked
    assert_eq!(result.files_checked, 1);

    // If there are no errors, file should pass
    if result.total_errors == 0 {
        assert_eq!(result.files_passed, 1);
    }

    // But should have warnings (V006 for short tags, V007 for missing context, V008 for missing see also)
    assert!(result.total_warnings > 0);

    let warnings: Vec<_> = result.issues.iter()
        .filter(|i| i.severity == doc_transformer::validate::Severity::Warning)
        .collect();
    assert!(!warnings.is_empty());
}

#[test]
fn test_severity_enum_values() {
    // Test that Severity enum works correctly
    use doc_transformer::validate::Severity;

    let error = Severity::Error;
    let warning = Severity::Warning;

    assert_eq!(error, Severity::Error);
    assert_eq!(warning, Severity::Warning);
    assert_ne!(error, warning);
}

#[test]
fn test_empty_directory_returns_empty_issues() {
    let temp_dir = TempDir::new().unwrap();

    let result = doc_transformer::validate::validate_all(temp_dir.path()).unwrap();

    assert_eq!(result.files_checked, 0);
    assert_eq!(result.files_passed, 0);
    assert_eq!(result.total_errors, 0);
    assert_eq!(result.total_warnings, 0);
    assert!(result.issues.is_empty());
}
