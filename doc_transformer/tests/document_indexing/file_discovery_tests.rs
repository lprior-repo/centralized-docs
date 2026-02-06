//! Tests for file-discovery bug (bead doc-tx-qc7)
//!
//! Bug: When files exist but fail during analysis (e.g., invalid UTF-8),
//! the index command proceeds with 0 documents and prints "Documents: 0"
//! instead of reporting an error.
//!
//! This test verifies the fix: the command should fail with a clear
//! error message when files cannot be analyzed.

use std::fs;
use tempfile::TempDir;

/// Test that a single invalid UTF-8 file (no valid files) causes error.
/// This is the core bug: when files exist but ALL fail analysis,
/// we should error instead of proceeding with 0 documents.
#[test]
fn test_invalid_utf8_all_files_cause_error() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let source = temp_dir.path();

    // Create ONLY invalid UTF-8 files (no valid files)
    let bad_md = source.join("bad.md");
    // Invalid UTF-8 sequence
    let invalid_utf8 = vec![0xFF, 0xFE, 0x20, 0x20]; // Invalid bytes
    fs::write(&bad_md, invalid_utf8).expect("Failed to write bad file");

    // Discover should find the file
    let (files, _manifest) =
        doc_transformer::discover::discover_files(source).expect("Discovery should succeed");
    assert_eq!(files.len(), 1, "Should discover the file");

    // Analysis should error because the file cannot be read
    let result = doc_transformer::analyze::analyze_files(&files, source, None);

    // After the fix: this should be an error
    // Before the fix: this would return Ok(vec![]) leading to "Documents: 0"
    assert!(
        result.is_err(),
        "analyze_files should return error when no files can be analyzed"
    );

    if let Err(e) = result {
        let error_msg = e.to_string();
        // The error should mention the failure
        assert!(
            error_msg.contains("Failed to analyze") || error_msg.contains("1"),
            "Error message should mention the analysis failure: {error_msg}"
        );
    }
}

/// Test that when all discovered files fail analysis, we get a clear error
#[test]
fn test_all_files_unreadable_causes_error() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let source = temp_dir.path();

    // Create multiple files with invalid UTF-8
    for i in 1..=3 {
        let bad_md = source.join(format!("bad{i}.md"));
        let invalid_utf8 = vec![0xFF, 0xFE, 0x00]; // Invalid UTF-8
        fs::write(&bad_md, invalid_utf8).expect("Failed to write bad file");
    }

    // Discover finds the files
    let (files, _manifest) =
        doc_transformer::discover::discover_files(source).expect("Discovery should succeed");
    assert_eq!(files.len(), 3, "Should discover all 3 files");

    // Analysis should fail with a clear error
    let result = doc_transformer::analyze::analyze_files(&files, source, None);
    assert!(result.is_err(), "Should error when all files fail analysis");
}

/// Test that partial failures are OK - if at least one file succeeds,
/// we continue (with warnings for failures)
#[test]
fn test_partial_file_analysis_failure_is_ok() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let source = temp_dir.path();

    // Create one valid file
    let valid_md = source.join("valid.md");
    fs::write(&valid_md, "# Valid\n\nContent here").expect("Failed to write valid file");

    // And one invalid file
    let bad_md = source.join("bad.md");
    let invalid_utf8 = vec![0xFF, 0xFE, 0x00];
    fs::write(&bad_md, invalid_utf8).expect("Failed to write bad file");

    let (files, _manifest) =
        doc_transformer::discover::discover_files(source).expect("Discovery should succeed");
    assert_eq!(files.len(), 2, "Should discover both files");

    // Analysis should succeed with one valid file
    let result = doc_transformer::analyze::analyze_files(&files, source, None);
    assert!(
        result.is_ok(),
        "Should succeed when at least one file can be analyzed"
    );

    let analyses = result.unwrap();
    assert_eq!(analyses.len(), 1, "Should have one analysis result");
}
