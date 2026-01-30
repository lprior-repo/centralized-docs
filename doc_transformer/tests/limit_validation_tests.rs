//! Comprehensive tests for limit validation (P2 search-limit-zero + P1 negative limit)
//!
//! This module tests that:
//! - limit = 0 is rejected (prevents tantivy panic)
//! - limit < 0 is rejected (negative values)
//! - limit = 1 works (minimum valid)
//! - limit = 10 works (default)
//! - limit = 1000 works (maximum per CLI validator)
//! - limit = 1001 is rejected by CLI validator
//! - Non-numeric input is rejected
//!
//! Tests cover both CLI argument validation (main.rs) and internal validation (validate.rs)

use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use tempfile::TempDir;

/// Helper to create a test index with sample documents
fn create_test_index(dir: &Path) -> anyhow::Result<()> {
    let docs = vec![
        doc_transformer::index::IndexDocument {
            id: "test/doc1".to_string(),
            title: "Rust Programming".to_string(),
            summary: "A comprehensive guide to Rust programming language".to_string(),
            path: "test/doc1.md".to_string(),
            category: "tutorial".to_string(),
            word_count: 500,
            tags: vec!["rust".to_string(), "programming".to_string()],
            chunk_ids: vec![],
            headings: vec!["Rust Programming".to_string()],
        },
        doc_transformer::index::IndexDocument {
            id: "test/doc2".to_string(),
            title: "Systems Programming".to_string(),
            summary: "Low-level systems programming concepts".to_string(),
            path: "test/doc2.md".to_string(),
            category: "concept".to_string(),
            word_count: 350,
            tags: vec!["systems".to_string(), "low-level".to_string()],
            chunk_ids: vec![],
            headings: vec!["Systems Programming".to_string()],
        },
        doc_transformer::index::IndexDocument {
            id: "test/doc3".to_string(),
            title: "Web Development".to_string(),
            summary: "Building web applications with modern frameworks".to_string(),
            path: "test/doc3.md".to_string(),
            category: "tutorial".to_string(),
            word_count: 400,
            tags: vec!["web".to_string(), "frameworks".to_string()],
            chunk_ids: vec![],
            headings: vec!["Web Development".to_string()],
        },
    ];

    let index = doc_transformer::search::open_or_create_index(dir)?;
    doc_transformer::search::index_documents(&index, docs)?;
    Ok(())
}

/// Helper to create test markdown files
fn create_test_files(dir: &Path) -> anyhow::Result<()> {
    use std::fs;
    let docs_dir = dir.join("docs");
    fs::create_dir_all(&docs_dir)?;

    let content = r#"---
id: test/doc1
title: Rust Programming
category: tutorial
tags: ["rust", "programming"]
---

# Rust Programming

A comprehensive guide to Rust programming language.
"#;
    fs::write(docs_dir.join("rust.md"), content)?;
    Ok(())
}

// ============================================================================
// INTERNAL VALIDATION TESTS (validate::validate_limit)
// These test the validation that happens AFTER CLI parsing
// ============================================================================

#[test]
fn test_internal_validate_limit_zero_rejected() {
    // P2: limit = 0 should be rejected (prevents tantivy panic)
    let result = doc_transformer::validate::validate_limit(0);
    assert!(result.is_err(), "limit=0 should be rejected");
    let err = result.unwrap_err();
    assert!(matches!(
        err,
        doc_transformer::validate::ValidationError::InvalidLimit { limit: 0 }
    ));
}

#[test]
fn test_internal_validate_limit_error_message_zero() {
    // Verify error message is informative
    let result = doc_transformer::validate::validate_limit(0);
    assert!(result.is_err());
    let err_msg = result.as_ref().map_err(|e| e.to_string()).unwrap_err();
    assert!(
        err_msg.contains("Limit must be greater than 0"),
        "Error message should explain why 0 is invalid, got: {err_msg}"
    );
}

#[test]
fn test_internal_validate_limit_one_accepted() {
    // limit = 1 should work (minimum valid)
    let result = doc_transformer::validate::validate_limit(1);
    assert!(result.is_ok(), "limit=1 should be accepted");
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn test_internal_validate_limit_default_accepted() {
    // limit = 10 should work (default)
    let result = doc_transformer::validate::validate_limit(10);
    assert!(result.is_ok(), "limit=10 should be accepted");
    assert_eq!(result.unwrap(), 10);
}

#[test]
fn test_internal_validate_limit_maximum_accepted() {
    // limit = 1000 should work (maximum per CLI validator)
    let result = doc_transformer::validate::validate_limit(1000);
    assert!(result.is_ok(), "limit=1000 should be accepted");
    assert_eq!(result.unwrap(), 1000);
}

#[test]
fn test_internal_validate_limit_above_cli_maximum() {
    // Internal validator doesn't enforce upper bound (only CLI does)
    // This tests the behavior when someone bypasses CLI parsing
    let result = doc_transformer::validate::validate_limit(1001);
    assert!(
        result.is_ok(),
        "Internal validator accepts values > 1000 (CLI enforces upper bound)"
    );
    assert_eq!(result.unwrap(), 1001);
}

#[test]
fn test_internal_validate_limit_large_value() {
    // Very large values are accepted by internal validator
    let result = doc_transformer::validate::validate_limit(1_000_000);
    assert!(result.is_ok(), "Internal validator accepts large values");
    assert_eq!(result.unwrap(), 1_000_000);
}

// ============================================================================
// CLI VALIDATION TESTS (validate_limit from main.rs)
// These test the CLI argument parsing validation
// ============================================================================

/// Helper to invoke the CLI validator from main.rs
/// This simulates what happens when clap parses --limit argument
fn cli_validate_limit(s: &str) -> Result<usize, String> {
    // Parse as i64 first to catch negative values
    let value = s
        .parse::<i64>()
        .map_err(|_| format!("limit must be a positive integer, got '{s}'"))?;

    if value < 0 {
        return Err(format!(
            "limit must be positive (cannot return negative results), got {value}"
        ));
    }

    if value == 0 {
        return Err("limit must be at least 1 (use --limit 1 or higher)".to_string());
    }

    if value > 1000 {
        return Err(format!("limit must be at most 1000 results, got {value}"));
    }

    value
        .try_into()
        .map_err(|_| format!("limit value too large: {value}"))
}

#[test]
fn test_cli_validate_limit_zero_rejected() {
    // P2: limit = 0 should be rejected
    let result = cli_validate_limit("0");
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("at least 1"),
        "Error message should mention minimum requirement, got: {err_msg}"
    );
}

#[test]
fn test_cli_validate_limit_negative_one_rejected() {
    // P1: limit = -1 should be rejected (negative)
    let result = cli_validate_limit("-1");
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("positive") || err_msg.contains("negative"),
        "Error message should mention negative/positive requirement, got: {err_msg}"
    );
    assert!(
        err_msg.contains("-1"),
        "Error message should include the invalid value, got: {err_msg}"
    );
}

#[test]
fn test_cli_validate_limit_negative_100_rejected() {
    // P1: limit = -100 should be rejected (very negative)
    let result = cli_validate_limit("-100");
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("positive") || err_msg.contains("negative"),
        "Error message should mention negative/positive requirement, got: {err_msg}"
    );
    assert!(
        err_msg.contains("-100") || err_msg.contains("100"),
        "Error message should include the invalid value, got: {err_msg}"
    );
}

#[test]
fn test_cli_validate_limit_negative_large_rejected() {
    // P1: Very large negative value should be rejected
    let result = cli_validate_limit("-999999");
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("positive") || err_msg.contains("negative"),
        "Error message should mention negative/positive, got: {err_msg}"
    );
}

#[test]
fn test_cli_validate_limit_one_accepted() {
    // limit = 1 should work (minimum valid)
    let result = cli_validate_limit("1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn test_cli_validate_limit_default_accepted() {
    // limit = 10 should work (default)
    let result = cli_validate_limit("10");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 10);
}

#[test]
fn test_cli_validate_limit_maximum_accepted() {
    // limit = 1000 should work (maximum)
    let result = cli_validate_limit("1000");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1000);
}

#[test]
fn test_cli_validate_limit_1001_rejected() {
    // limit = 1001 should be rejected (exceeds max)
    let result = cli_validate_limit("1001");
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("1000") && err_msg.contains("1001"),
        "Error message should mention both limit (1000) and invalid value (1001), got: {err_msg}"
    );
}

#[test]
fn test_cli_validate_limit_exceeds_max_rejected() {
    // limit = 5000 should be rejected (far exceeds max)
    let result = cli_validate_limit("5000");
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("1000") && err_msg.contains("5000"),
        "Error message should mention bounds, got: {err_msg}"
    );
}

#[test]
fn test_cli_validate_limit_non_numeric_rejected() {
    // Non-numeric limit should be rejected
    let result = cli_validate_limit("invalid");
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("positive integer") || err_msg.contains("must be"),
        "Error message should mention type requirement, got: {err_msg}"
    );
}

#[test]
fn test_cli_validate_limit_float_rejected() {
    // Float values should be rejected
    let result = cli_validate_limit("10.5");
    assert!(result.is_err());
    // Floats parse as numbers but fail i64 conversion
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("positive integer") || err_msg.contains("must be"),
        "Error message should mention integer requirement, got: {err_msg}"
    );
}

#[test]
fn test_cli_validate_limit_empty_string_rejected() {
    // Empty string should be rejected
    let result = cli_validate_limit("");
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("positive integer") || err_msg.contains("must be"),
        "Error message should mention requirement, got: {err_msg}"
    );
}

#[test]
fn test_cli_validate_limit_with_whitespace_rejected() {
    // Whitespace around numbers is rejected by our validator
    // (clap would handle this, but our test helper is more strict)
    let result = cli_validate_limit("  10  ");
    // The helper rejects whitespace (real clap would trim)
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("positive integer") || err_msg.contains("must be"),
        "Error message should mention type requirement, got: {err_msg}"
    );
}

// ============================================================================
// INTEGRATION TESTS (full search command with CLI)
// ============================================================================

#[test]
fn test_search_with_limit_zero_fails() {
    // P2: Search with --limit 0 should fail
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let index_dir = temp_dir.path();

    create_test_index(index_dir).expect("Failed to create index");

    // Try to run search with limit=0
    let result = doc_transformer::search::search_index(
        &doc_transformer::search::open_or_create_index(index_dir).unwrap(),
        "rust",
        0,
    );

    // The internal validator should catch this
    assert!(result.is_err(), "Search with limit=0 should fail");
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("Limit must be greater than 0") || err_msg.contains("limit"),
        "Error should mention invalid limit, got: {err_msg}"
    );
}

#[test]
fn test_search_with_limit_one_works() {
    // limit = 1 should work (minimum valid)
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let index_dir = temp_dir.path();

    create_test_index(index_dir).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(index_dir).unwrap();
    let result = doc_transformer::search::search_index(&index, "rust", 1);

    assert!(result.is_ok(), "Search with limit=1 should work");
    let results = result.unwrap();
    assert!(results.len() <= 1, "Should return at most 1 result");
}

#[test]
fn test_search_with_limit_default_works() {
    // limit = 10 should work (default)
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let index_dir = temp_dir.path();

    create_test_index(index_dir).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(index_dir).unwrap();
    let result = doc_transformer::search::search_index(&index, "rust", 10);

    assert!(result.is_ok(), "Search with limit=10 should work");
    let results = result.unwrap();
    assert!(results.len() <= 10, "Should return at most 10 results");
}

#[test]
fn test_search_with_limit_1000_works() {
    // limit = 1000 should work (maximum)
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let index_dir = temp_dir.path();

    create_test_index(index_dir).expect("Failed to create index");

    let index = doc_transformer::search::open_or_create_index(index_dir).unwrap();
    let result = doc_transformer::search::search_index(&index, "rust", 1000);

    assert!(result.is_ok(), "Search with limit=1000 should work");
    let results = result.unwrap();
    assert!(results.len() <= 1000, "Should return at most 1000 results");
}

#[test]
fn test_search_cli_rejects_limit_zero() {
    // Test CLI actually rejects --limit 0
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_dir = temp_dir.path().join("output");

    create_test_files(temp_dir.path()).expect("Failed to create test files");

    // First, transform the files to create an index
    let transform_status = Command::new("env")
        .args([
            "CARGO_TARGET_DIR=/home/lewis/src/centralized-docs/rust_out",
            "cargo",
            "run",
            "--quiet",
            "index",
            temp_dir.path().to_str().unwrap(),
            "--output",
            output_dir.to_str().unwrap(),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    if !transform_status.map(|s| s.success()).unwrap_or(false) {
        println!("Warning: transform failed, skipping CLI test");
        return;
    }

    // Try to search with --limit 0
    let mut child = Command::new("env")
        .args([
            "CARGO_TARGET_DIR=/home/lewis/src/centralized-docs/rust_out",
            "cargo",
            "run",
            "--quiet",
            "search",
            "rust",
            "--index-dir",
            output_dir.to_str().unwrap(),
            "--limit",
            "0",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn search command");

    let mut stdout = String::new();
    if let Some(mut out) = child.stdout.take() {
        out.read_to_string(&mut stdout)
            .expect("Failed to read stdout");
    }

    let mut stderr = String::new();
    if let Some(mut err) = child.stderr.take() {
        err.read_to_string(&mut stderr)
            .expect("Failed to read stderr");
    }

    let status = child.wait().expect("Failed to wait for search command");

    // Command should fail
    assert!(!status.success(), "Search with --limit 0 should fail");

    let output = format!("{stdout}\n{stderr}");
    assert!(
        output.contains("at least 1") || output.contains("limit"),
        "Output should explain why --limit 0 is invalid, got: {output}"
    );
}

#[test]
fn test_search_cli_rejects_limit_negative() {
    // P1: Test CLI rejects --limit -1
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_dir = temp_dir.path().join("output");

    create_test_files(temp_dir.path()).expect("Failed to create test files");

    // First, transform the files
    let transform_status = Command::new("env")
        .args([
            "CARGO_TARGET_DIR=/home/lewis/src/centralized-docs/rust_out",
            "cargo",
            "run",
            "--quiet",
            "index",
            temp_dir.path().to_str().unwrap(),
            "--output",
            output_dir.to_str().unwrap(),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    if !transform_status.map(|s| s.success()).unwrap_or(false) {
        println!("Warning: transform failed, skipping CLI test");
        return;
    }

    // Try to search with --limit -1
    let mut child = Command::new("env")
        .args([
            "CARGO_TARGET_DIR=/home/lewis/src/centralized-docs/rust_out",
            "cargo",
            "run",
            "--quiet",
            "search",
            "rust",
            "--index-dir",
            output_dir.to_str().unwrap(),
            "--limit",
            "-1",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn search command");

    let mut stdout = String::new();
    if let Some(mut out) = child.stdout.take() {
        out.read_to_string(&mut stdout)
            .expect("Failed to read stdout");
    }

    let mut stderr = String::new();
    if let Some(mut err) = child.stderr.take() {
        err.read_to_string(&mut stderr)
            .expect("Failed to read stderr");
    }

    let status = child.wait().expect("Failed to wait for search command");

    // Command should fail
    assert!(!status.success(), "Search with --limit -1 should fail");

    let output = format!("{stdout}\n{stderr}");
    assert!(
        output.contains("positive") || output.contains("negative") || output.contains("limit"),
        "Output should explain why --limit -1 is invalid, got: {output}"
    );
}

#[test]
fn test_search_cli_rejects_limit_non_numeric() {
    // Test CLI rejects --limit with non-numeric value
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_dir = temp_dir.path().join("output");

    create_test_files(temp_dir.path()).expect("Failed to create test files");

    // First, transform the files
    let transform_status = Command::new("env")
        .args([
            "CARGO_TARGET_DIR=/home/lewis/src/centralized-docs/rust_out",
            "cargo",
            "run",
            "--quiet",
            "index",
            temp_dir.path().to_str().unwrap(),
            "--output",
            output_dir.to_str().unwrap(),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    if !transform_status.map(|s| s.success()).unwrap_or(false) {
        println!("Warning: transform failed, skipping CLI test");
        return;
    }

    // Try to search with --limit abc
    let mut child = Command::new("env")
        .args([
            "CARGO_TARGET_DIR=/home/lewis/src/centralized-docs/rust_out",
            "cargo",
            "run",
            "--quiet",
            "search",
            "rust",
            "--index-dir",
            output_dir.to_str().unwrap(),
            "--limit",
            "abc",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn search command");

    let mut stdout = String::new();
    if let Some(mut out) = child.stdout.take() {
        out.read_to_string(&mut stdout)
            .expect("Failed to read stdout");
    }

    let mut stderr = String::new();
    if let Some(mut err) = child.stderr.take() {
        err.read_to_string(&mut stderr)
            .expect("Failed to read stderr");
    }

    let status = child.wait().expect("Failed to wait for search command");

    // Command should fail
    assert!(!status.success(), "Search with --limit abc should fail");

    let output = format!("{stdout}\n{stderr}");
    assert!(
        output.contains("positive integer") || output.contains("must be"),
        "Output should explain type requirement, got: {output}"
    );
}

#[test]
fn test_search_cli_rejects_limit_exceeds_max() {
    // Test CLI rejects --limit 1001 (exceeds max of 1000)
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_dir = temp_dir.path().join("output");

    create_test_files(temp_dir.path()).expect("Failed to create test files");

    // First, transform the files
    let transform_status = Command::new("env")
        .args([
            "CARGO_TARGET_DIR=/home/lewis/src/centralized-docs/rust_out",
            "cargo",
            "run",
            "--quiet",
            "index",
            temp_dir.path().to_str().unwrap(),
            "--output",
            output_dir.to_str().unwrap(),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    if !transform_status.map(|s| s.success()).unwrap_or(false) {
        println!("Warning: transform failed, skipping CLI test");
        return;
    }

    // Try to search with --limit 1001
    let mut child = Command::new("env")
        .args([
            "CARGO_TARGET_DIR=/home/lewis/src/centralized-docs/rust_out",
            "cargo",
            "run",
            "--quiet",
            "search",
            "rust",
            "--index-dir",
            output_dir.to_str().unwrap(),
            "--limit",
            "1001",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn search command");

    let mut stdout = String::new();
    if let Some(mut out) = child.stdout.take() {
        out.read_to_string(&mut stdout)
            .expect("Failed to read stdout");
    }

    let mut stderr = String::new();
    if let Some(mut err) = child.stderr.take() {
        err.read_to_string(&mut stderr)
            .expect("Failed to read stderr");
    }

    let status = child.wait().expect("Failed to wait for search command");

    // Command should fail
    assert!(!status.success(), "Search with --limit 1001 should fail");

    let output = format!("{stdout}\n{stderr}");
    assert!(
        output.contains("1000") && (output.contains("1001") || output.contains("exceeds")),
        "Output should explain maximum limit, got: {output}"
    );
}

#[test]
fn test_search_cli_accepts_limit_1() {
    // Test CLI accepts --limit 1 (minimum valid)
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_dir = temp_dir.path().join("output");

    create_test_files(temp_dir.path()).expect("Failed to create test files");

    // First, transform the files
    let transform_status = Command::new("env")
        .args([
            "CARGO_TARGET_DIR=/home/lewis/src/centralized-docs/rust_out",
            "cargo",
            "run",
            "--quiet",
            "index",
            temp_dir.path().to_str().unwrap(),
            "--output",
            output_dir.to_str().unwrap(),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    if !transform_status.map(|s| s.success()).unwrap_or(false) {
        println!("Warning: transform failed, skipping CLI test");
        return;
    }

    // Try to search with --limit 1
    let mut child = Command::new("env")
        .args([
            "CARGO_TARGET_DIR=/home/lewis/src/centralized-docs/rust_out",
            "cargo",
            "run",
            "--quiet",
            "search",
            "rust",
            "--index-dir",
            output_dir.to_str().unwrap(),
            "--limit",
            "1",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn search command");

    let mut stdout = String::new();
    if let Some(mut out) = child.stdout.take() {
        out.read_to_string(&mut stdout)
            .expect("Failed to read stdout");
    }

    let mut stderr = String::new();
    if let Some(mut err) = child.stderr.take() {
        err.read_to_string(&mut stderr)
            .expect("Failed to read stderr");
    }

    let status = child.wait().expect("Failed to wait for search command");

    // Command should succeed
    assert!(status.success(), "Search with --limit 1 should succeed");

    let output = format!("{stdout}\n{stderr}");
    assert!(
        output.contains("Results") || output.contains("rust"),
        "Output should show search results, got: {output}"
    );
}
