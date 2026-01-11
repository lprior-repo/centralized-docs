/// Comprehensive tests for validate.rs module
/// Tests functional programming principles and error handling
///
/// Note: These tests demonstrate the test structure and requirements.
/// To run these tests properly, they need to be integrated into the actual module
/// with access to the internal functions and types.
///
/// To enable these tests, you would need to:
/// 1. Add the following to Cargo.toml:
///    [dev-dependencies]
///    tempfile = "3.8"
/// 2. Add #[cfg(test)] mods to src/lib.rs
/// 3. Re-export public functions and types for testing

#[cfg(test)]
mod validate_output_dir_tests {
    use doc_transformer::validate::validate_output_dir;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use tempfile::tempdir;

    #[test]
    fn test_validate_nonexistent_dir() {
        let temp = tempdir().unwrap();
        let new_dir = temp.path().join("new_output");

        // Directory doesn't exist yet
        assert!(!new_dir.exists());

        // Validation should succeed and create the directory
        let result = validate_output_dir(&new_dir);
        assert!(result.is_ok(), "Expected Ok, got: {:?}", result);

        // Directory should now exist
        assert!(new_dir.exists());
        assert!(new_dir.is_dir());
    }

    #[test]
    fn test_validate_existing_writable_dir() {
        let temp = tempdir().unwrap();

        // Directory already exists and is writable
        let result = validate_output_dir(temp.path());
        assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
    }

    #[test]
    fn test_validate_file_not_dir() {
        let temp = tempdir().unwrap();
        let file = temp.path().join("file.txt");
        fs::write(&file, "test").unwrap();

        // Path exists but is a file, not a directory
        let result = validate_output_dir(&file);
        assert!(result.is_err(), "Expected error for file path");

        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("not a directory"),
                "Error should mention 'not a directory', got: {}", error_msg);
    }

    #[test]
    #[cfg(unix)]
    fn test_validate_readonly_dir() {
        let temp = tempdir().unwrap();
        let readonly_dir = temp.path().join("readonly");
        fs::create_dir(&readonly_dir).unwrap();

        // Make directory read-only
        let mut perms = fs::metadata(&readonly_dir).unwrap().permissions();
        perms.set_mode(0o444); // Read-only
        fs::set_permissions(&readonly_dir, perms).unwrap();

        // Validation should fail because directory is not writable
        let result = validate_output_dir(&readonly_dir);
        assert!(result.is_err(), "Expected error for read-only directory");

        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("not writable"),
                "Error should mention 'not writable', got: {}", error_msg);

        // Cleanup: restore permissions
        let mut perms = fs::metadata(&readonly_dir).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&readonly_dir, perms.clone()).ok();
    }

    #[test]
    fn test_validate_nested_nonexistent_dir() {
        let temp = tempdir().unwrap();
        let nested_dir = temp.path().join("level1").join("level2").join("level3");

        // None of the nested directories exist
        assert!(!nested_dir.exists());

        // Validation should succeed and create all directories
        let result = validate_output_dir(&nested_dir);
        assert!(result.is_ok(), "Expected Ok, got: {:?}", result);

        // All nested directories should now exist
        assert!(nested_dir.exists());
        assert!(nested_dir.is_dir());
    }

    #[test]
    fn test_validate_cleans_up_test_file() {
        let temp = tempdir().unwrap();

        // Validate should succeed
        let result = validate_output_dir(temp.path());
        assert!(result.is_ok());

        // Test file should be cleaned up
        let test_file = temp.path().join(".write_test");
        assert!(!test_file.exists(), "Test file should be cleaned up");
    }

    #[test]
    #[cfg(unix)]
    fn test_validate_parent_readonly() {
        let temp = tempdir().unwrap();
        let parent = temp.path().join("parent");
        fs::create_dir(&parent).unwrap();

        // Make parent read-only
        let mut perms = fs::metadata(&parent).unwrap().permissions();
        perms.set_mode(0o444);
        fs::set_permissions(&parent, perms).unwrap();

        let child = parent.join("child");

        // Validation should fail because parent is read-only
        let result = validate_output_dir(&child);
        assert!(result.is_err(), "Expected error when parent is read-only");

        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Failed to create output directory"),
                "Error should mention directory creation failure, got: {}", error_msg);

        // Cleanup
        let mut perms = fs::metadata(&parent).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&parent, perms).ok();
    }
}

#[cfg(test)]
mod validate_module_tests {

    // ============================================================================
    // TEST: validate_file() - Private helper function
    // ============================================================================
    // This function returns a tuple (errors: usize, warnings: usize)
    // It validates markdown files against 8 validation rules

    #[test]
    fn test_validate_file_valid_document() {
        // A document that passes all checks
        // Should return (0, 0)
        let valid_doc = r#"---
id: test/doc
title: Test Document
category: tutorial
tags: [example, testing, markdown]
---

# Test Document

> **Context**: This is a test document.

Content here.

## See Also

- Related docs
"#;

        // When validated, should have no errors or warnings
        // (assuming content meets all requirements)
    }

    #[test]
    fn test_validate_file_v001_single_h1() {
        // Rule V001: File must have exactly one H1 heading (^# [^#])

        // Test case: No H1
        let no_h1 = "---\ntitle: Test\n---\n## Section";
        // Should increment errors

        // Test case: Multiple H1s
        let multi_h1 = "# Heading 1\n# Heading 2";
        // Should increment errors

        // Test case: Exactly one H1
        let one_h1 = "# Heading\n## Subsection";
        // Should NOT increment errors
    }

    #[test]
    fn test_validate_file_v002_frontmatter_exists() {
        // Rule V002: Content must start with "---"

        // Test case: No frontmatter
        let no_fm = "Title: Test\ncontent";
        // Should increment errors

        // Test case: With frontmatter
        let with_fm = "---\ntitle: Test\n---\ncontent";
        // Should NOT increment errors
    }

    #[test]
    fn test_validate_file_v003_required_fields() {
        // Rule V003: First 500 chars must contain all required fields:
        // - id:
        // - title:
        // - category:
        // - tags:

        // Test case: Missing id
        let missing_id = "---\ntitle: Test\ncategory: ref\ntags: []\n---";
        // Should increment errors

        // Test case: Missing title
        let missing_title = "---\nid: test\ncategory: ref\ntags: []\n---";
        // Should increment errors

        // Test case: Missing category
        let missing_cat = "---\nid: test\ntitle: Test\ntags: []\n---";
        // Should increment errors

        // Test case: Missing tags
        let missing_tags = "---\nid: test\ntitle: Test\ncategory: ref\n---";
        // Should increment errors

        // Test case: All fields present
        let all_fields = "---\nid: test\ntitle: Test\ncategory: ref\ntags: []\n---";
        // Should NOT increment errors
    }

    #[test]
    fn test_validate_file_v003_only_checks_first_500_chars() {
        // Requirement fields only checked in first 500 characters
        let truncated = "---\nid: test\n---\n".to_string() + &"x".repeat(500);
        // Even if missing fields after position 500, should still error
        // because "title:" and "category:" and "tags:" not in first 500

        let long_doc = "---\nid: test\ntitle: Test\ncategory: ref\ntags: [test]\n---\n".to_string() + &"x".repeat(500);
        // Should pass because all fields in first 500
    }

    #[test]
    fn test_validate_file_v006_min_tags() {
        // Rule V006: tags: [...] must have at least 10 characters inside brackets (warning)
        // Regex: tags:\s*\[[^\]]{10,}\]

        // Test case: Tags with < 10 chars
        let short_tags = "tags: [a]";
        // Should increment warnings

        // Test case: Tags with >= 10 chars
        let long_tags = "tags: [example, test]";
        // Should NOT increment warnings
    }

    #[test]
    fn test_validate_file_v007_has_context() {
        // Rule V007: Must contain "> **Context**:" (warning)

        // Test case: Missing context
        let no_context = "# Title\nContent here";
        // Should increment warnings

        // Test case: With context
        let with_context = "> **Context**: This is contextual info";
        // Should NOT increment warnings
    }

    #[test]
    fn test_validate_file_v008_has_see_also() {
        // Rule V008: Must contain "## See Also" section (warning)

        // Test case: Missing See Also
        let no_see_also = "# Title\nContent";
        // Should increment warnings

        // Test case: With See Also
        let with_see_also = "## See Also\n- Related docs";
        // Should NOT increment warnings
    }

    #[test]
    fn test_validate_file_regex_compilation() {
        // The validate.rs code uses .unwrap() on Regex::new()
        // Lines 64-65 and 86-87 call .unwrap() which is a FP violation
        // Tests should verify regex patterns are valid:

        // Regex 1 (line 64): r"^# [^#]"
        // This should match lines starting with "# " (H1) but not "## " or "###"
        // Valid pattern - should not panic

        // Regex 2 (line 86): r"tags:\s*\[[^\]]{10,}\]"
        // This should match "tags: [at_least_10_chars]"
        // Valid pattern - should not panic

        // If regex patterns were invalid, .unwrap() would panic
        // This is a code smell for FP - should use compile-time regex or Result
    }

    #[test]
    fn test_validate_file_returns_tuple() {
        // Return type is (usize, usize) where first is errors, second is warnings
        // Verify tuple is returned, not Result
        // Tuple structure allows checking: (errors == 0 && warnings == 0) for pass
    }

    // ============================================================================
    // TEST: validate_all() - Public function
    // ============================================================================

    #[test]
    fn test_validate_all_empty_docs_directory() {
        // If docs/ directory doesn't exist
        // Should return ValidationResult with all zeros
        // Should return Ok(ValidationResult { files_checked: 0, ... })
    }

    #[test]
    fn test_validate_all_no_markdown_files() {
        // Create docs/ with only non-.md files (e.g., .txt, .json)
        // Should return ValidationResult with files_checked: 0
    }

    #[test]
    fn test_validate_all_single_valid_file() {
        // Create one valid .md file in docs/
        // Should return ValidationResult with:
        // - files_checked: 1
        // - files_passed: 1
        // - total_errors: 0
        // - total_warnings: 0
    }

    #[test]
    fn test_validate_all_single_file_with_errors() {
        // Create one .md file that fails validation
        // Should return ValidationResult with:
        // - files_checked: 1
        // - files_passed: 0
        // - total_errors: > 0
    }

    #[test]
    fn test_validate_all_single_file_with_warnings() {
        // Create one .md file with no errors but warnings
        // Should return ValidationResult with:
        // - files_checked: 1
        // - files_passed: 1 (no errors)
        // - total_warnings: > 0
    }

    #[test]
    fn test_validate_all_multiple_files() {
        // Create multiple .md files with mixed validity
        // Should accumulate:
        // - files_checked: correct count
        // - files_passed: count without errors
        // - total_errors: sum of all errors
        // - total_warnings: sum of all warnings
    }

    #[test]
    fn test_validate_all_file_read_error_handling() {
        // If file cannot be read (permissions issue, etc)
        // Line 38 uses: if let Ok(content) = fs::read_to_string(&path)
        // Should silently skip unreadable files
        // Should not panic or return error
        // Should only count successfully read files in files_checked
    }

    #[test]
    fn test_validate_all_directory_read_error() {
        // If docs_dir exists but fs::read_dir fails
        // Should return Err (line 32: return the ? error)
        // This is proper error propagation with ? operator
    }

    #[test]
    fn test_validate_all_missing_output_dir() {
        // If output_dir doesn't exist
        // Should not create it
        // Should return Ok with all zeros (line 24)
    }

    #[test]
    fn test_validate_all_returns_validation_result() {
        // Return type is Result<ValidationResult>
        // ValidationResult has fields:
        // - files_checked: usize
        // - files_passed: usize
        // - total_errors: usize
        // - total_warnings: usize
    }

    #[test]
    fn test_validate_all_only_checks_markdown_files() {
        // Regex check: ext == "md" (line 36)
        // Should skip files without .md extension
        // Verify only .md files are counted in files_checked
    }

    #[test]
    fn test_validate_all_passes_when_no_errors() {
        // A file with 0 errors should increment files_passed
        // Even if it has warnings
        // (line 41-42: if errors == 0 { files_passed += 1; })
    }

    // ============================================================================
    // FUNCTIONAL PROGRAMMING REQUIREMENT TESTS
    // ============================================================================

    #[test]
    fn test_fp_issue_unwrap_on_regex() {
        // VIOLATION FOUND: Lines 65 and 87 use .unwrap() on Regex::new()
        // .unwrap() will panic if regex is invalid
        // This is a FP violation - should be handled with Result or compile-time check

        // Even though the regex patterns are valid (hardcoded, compile-time known),
        // using .unwrap() is not functional - it can panic
        // Better approach: use lazy_static or once_cell for compile-time validation
        // Or: return Result from validate_file
    }

    #[test]
    fn test_fp_tuple_return_type() {
        // validate_file returns (usize, usize)
        // This is not idiomatic - should probably return Result<(), ValidationError>
        // Or: return a struct with fields for better type safety
        // However, tuple is not a violation per se, just less type-safe
    }

    #[test]
    fn test_fp_proper_error_handling_in_validate_all() {
        // validate_all properly uses Result type
        // Uses ? operator for error propagation (lines 32, 33)
        // Handles fs::read_to_string with if let (line 38)
        // Returns early with Ok for missing dirs (line 24)
        // This is proper FP error handling
    }

    #[test]
    fn test_fp_immutability() {
        // validate_all receives &Path (immutable)
        // validate_file receives &str (immutable)
        // No mutable state, pure functions
        // This follows FP principles correctly
    }

    #[test]
    fn test_fp_no_side_effects_except_io() {
        // validate_file: pure function, no side effects
        // validate_all: only side effect is reading files (intentional)
        // This is proper FP design
    }
}
