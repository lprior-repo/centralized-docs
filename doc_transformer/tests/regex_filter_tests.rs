//! Tests for --filter argument regex validation (P1 filter-invalid-regex)
//!
//! Tests verify that invalid regex patterns are rejected with clear error messages.

use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Helper function to get the path to the doc_transformer binary
fn binary_path() -> PathBuf {
    let paths = vec![
        "../target/release/doc_transformer",
        "../target/debug/doc_transformer",
        "target/release/doc_transformer",
        "target/debug/doc_transformer",
    ];

    for path in paths {
        if PathBuf::from(path).exists() {
            return PathBuf::from(path);
        }
    }

    PathBuf::from("../target/release/doc_transformer")
}

/// Test helper: Run scrape command with filter and check for expected output
fn run_scrape_with_filter(url: &str, filter: &str, output_dir: &TempDir) -> (bool, String, String) {
    let bin = binary_path();

    let output = Command::new(&bin)
        .arg("scrape")
        .arg(url)
        .arg("--output")
        .arg(output_dir.path())
        .arg("--filter")
        .arg(filter)
        .arg("--no-sitemap")
        .output();

    match output {
        Ok(result) => {
            let stdout = String::from_utf8_lossy(&result.stdout).to_string();
            let stderr = String::from_utf8_lossy(&result.stderr).to_string();
            (result.status.success(), stdout, stderr)
        }
        Err(e) => (false, String::new(), format!("Failed to execute: {e}")),
    }
}

// ============================================================================
// INVALID INPUT TESTS (boundary conditions)
// ============================================================================

#[test]
fn test_invalid_regex_unmatched_bracket() {
    // Invalid regex "[" should fail with clear error
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let (_success, stdout, stderr) =
        run_scrape_with_filter("https://example.com/docs", "[", &temp_dir);

    let combined = format!("{stdout}{stderr}");
    assert!(
        combined.contains("Invalid regex")
            || combined.contains("regex pattern")
            || combined.contains("regex"),
        "Invalid regex '[' should produce clear error. Output: {combined}"
    );
}

#[test]
fn test_invalid_regex_unmatched_parenthesis() {
    // Invalid regex "(unclosed" should fail
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let (_success, stdout, stderr) =
        run_scrape_with_filter("https://example.com/docs", "(unclosed", &temp_dir);

    let combined = format!("{stdout}{stderr}");
    assert!(
        combined.contains("Invalid regex")
            || combined.contains("regex pattern")
            || combined.contains("regex"),
        "Invalid regex '(unclosed' should produce clear error. Output: {combined}"
    );
}

#[test]
fn test_invalid_regex_invalid_escape() {
    // Invalid escape sequence at end
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let (_success, stdout, stderr) =
        run_scrape_with_filter("https://example.com/docs", "test\\", &temp_dir);

    let combined = format!("{stdout}{stderr}");
    assert!(
        combined.contains("Invalid regex")
            || combined.contains("regex pattern")
            || combined.contains("regex")
            || combined.contains("escape"),
        "Invalid regex 'test\\' should produce clear error. Output: {combined}"
    );
}

// ============================================================================
// VALID INPUT TESTS (happy path)
// ============================================================================

#[test]
fn test_valid_regex_wildcard_dot_star() {
    // Valid regex ".*" should work
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let (_success, stdout, stderr) =
        run_scrape_with_filter("https://example.com/docs", ".*", &temp_dir);

    let combined = format!("{stdout}{stderr}");
    assert!(
        !combined.contains("Invalid regex") && !combined.contains("regex pattern"),
        "Valid regex '.*' should not produce regex error. Output: {combined}"
    );
}

#[test]
fn test_valid_regex_character_class() {
    // Valid regex "[a-z]+" should work
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let (_success, stdout, stderr) =
        run_scrape_with_filter("https://example.com/docs", "[a-z]+", &temp_dir);

    let combined = format!("{stdout}{stderr}");
    assert!(
        !combined.contains("Invalid regex") && !combined.contains("regex pattern"),
        "Valid regex '[a-z]+' should not produce regex error. Output: {combined}"
    );
}

#[test]
fn test_valid_regex_path_filter() {
    // Common path filter pattern
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let (_success, stdout, stderr) =
        run_scrape_with_filter("https://example.com", "^/docs/", &temp_dir);

    let combined = format!("{stdout}{stderr}");
    assert!(
        !combined.contains("Invalid regex") && !combined.contains("regex pattern"),
        "Valid regex '^/docs/' should not produce regex error. Output: {combined}"
    );
}
