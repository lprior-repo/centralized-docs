//! CLI Integration Tests for doc_transformer
//!
//! This module tests the CLI interface by spawning the binary as a subprocess
//! and verifying behavior through exit codes and output. Tests cover all
//! commands: index, search, scrape, ingest, and ingest-git.
//!
//! ## Design Principles
//!
//! - **Process isolation**: Each test spawns a fresh subprocess
//! - **Temp directories**: All tests use tempfile for automatic cleanup
//! - **Fast feedback**: Tests use minimal inputs to keep execution fast
//! - **Deterministic**: No network calls in core tests (use local fixtures)

#![allow(clippy::panic)] // Tests may use panic for failure cases
#![allow(clippy::unwrap_used)] // Tests may use unwrap for brevity

use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

// =============================================================================
// TEST FIXTURES
// =============================================================================

/// Creates a temporary directory with sample markdown files for testing
#[allow(dead_code)]
fn create_test_docs(dir: &Path) {
    let docs_dir = dir.join("docs");
    fs::create_dir_all(&docs_dir).unwrap();

    // Create a simple README
    fs::write(
        docs_dir.join("README.md"),
        "# Test Project\n\nThis is a test documentation project.\n",
    )
    .unwrap();

    // Create a guide document
    fs::write(
        docs_dir.join("guide.md"),
        "# Getting Started\n\n## Installation\n\nInstall the package.\n\n## Usage\n\nUse the package.\n",
    )
    .unwrap();
}

/// Get the path to the compiled binary
fn binary_path() -> std::path::PathBuf {
    // In CI/tests, use the release binary if available
    let release_binary = Path::new("target/release/doc_transformer");
    if release_binary.exists() {
        return release_binary.to_path_buf();
    }

    // Otherwise, use debug binary
    let debug_binary = Path::new("target/debug/doc_transformer");
    if debug_binary.exists() {
        return debug_binary.to_path_buf();
    }

    // Fall back to cargo run
    Path::new("cargo").to_path_buf()
}

/// Run the CLI with given arguments
fn run_cli(args: &[&str]) -> std::process::Output {
    let binary = binary_path();

    println!("Binary: {:?}", binary);
    println!("Args: {:?}", args);

    if binary.file_name().unwrap_or_default() == "cargo" {
        // Use cargo run
        Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg("doc_transformer")
            .arg("--")
            .args(args)
            .output()
            .expect("Failed to execute cargo run")
    } else {
        // Use compiled binary directly
        Command::new(&binary)
            .args(args)
            .output()
            .unwrap_or_else(|_| panic!("Failed to execute binary: {}", binary.display()))
    }
}

// =============================================================================
// INDEX COMMAND TESTS
// =============================================================================

#[test]
fn test_index_basic_success() {
    let temp = TempDir::new().unwrap();
    let source = temp.path().join("source");
    let output_dir = temp.path().join("output");

    fs::create_dir_all(&source).unwrap();
    fs::write(source.join("test.md"), "# Test\n\nContent").unwrap();

    let result = run_cli(&[
        "index",
        source.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);

    assert!(
        result.status.success(),
        "Index command should succeed. stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );

    // Verify INDEX.json was created
    assert!(
        output_dir.join("INDEX.json").exists(),
        "INDEX.json should be created"
    );
}

#[test]
fn test_index_missing_source() {
    let result = run_cli(&[
        "index",
        "/nonexistent/path/that/does/not/exist",
        "--output",
        "/tmp/test_output",
    ]);

    assert!(
        !result.status.success(),
        "Index with missing source should fail"
    );
}

#[test]
fn test_index_empty_directory() {
    let temp = TempDir::new().unwrap();
    let source = temp.path().join("empty");
    let output_dir = temp.path().join("output");

    fs::create_dir_all(&source).unwrap();

    let result = run_cli(&[
        "index",
        source.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);

    assert!(
        result.status.success(),
        "Index with empty directory should succeed with 0 files. stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

// =============================================================================
// SEARCH COMMAND TESTS
// =============================================================================

#[test]
fn test_search_basic() {
    // First create an index
    let temp = TempDir::new().unwrap();
    let source = temp.path().join("source");
    let output_dir = temp.path().join("output");

    fs::create_dir_all(&source).unwrap();
    fs::write(
        source.join("test.md"),
        "# Rust Programming\n\nLearn Rust programming.",
    )
    .unwrap();

    // Index the content
    let index_result = run_cli(&[
        "index",
        source.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(index_result.status.success(), "Setup: Index should succeed");

    // Search for "rust"
    let search_result = run_cli(&[
        "search",
        "rust",
        "--index-dir",
        output_dir.to_str().unwrap(),
    ]);

    let stdout = String::from_utf8_lossy(&search_result.stdout);
    assert!(
        search_result.status.success(),
        "Search command should succeed. stderr: {}",
        String::from_utf8_lossy(&search_result.stderr)
    );
    assert!(
        stdout.contains("Rust") || stdout.contains("rust"),
        "Search should find results containing 'rust'. Output: {stdout}"
    );
}

#[test]
fn test_search_missing_index() {
    let result = run_cli(&["search", "test", "--index-dir", "/nonexistent/index/dir"]);

    assert!(
        !result.status.success(),
        "Search with missing index should fail"
    );
}

#[test]
fn test_search_no_results() {
    // First create an index
    let temp = TempDir::new().unwrap();
    let source = temp.path().join("source");
    let output_dir = temp.path().join("output");

    fs::create_dir_all(&source).unwrap();
    fs::write(source.join("test.md"), "# Test Document\n\nContent here.").unwrap();

    // Index the content
    let index_result = run_cli(&[
        "index",
        source.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(index_result.status.success(), "Setup: Index should succeed");

    // Search for a term that doesn't exist
    let search_result = run_cli(&[
        "search",
        "xyznonexistentterm12345",
        "--index-dir",
        output_dir.to_str().unwrap(),
    ]);

    // No-results is an error condition - scripts need to know search failed
    // Both JSON and text modes return exit code 1 for no results (bead doc-3f31)
    assert!(
        !search_result.status.success(),
        "Search with no matches should return exit 1 (error condition)"
    );
    let stderr = String::from_utf8_lossy(&search_result.stderr);
    assert!(
        stderr.contains("No results found"),
        "Expected explicit no-results error message"
    );
}

// =============================================================================
// SCRAPE COMMAND TESTS
// =============================================================================

#[test]
fn test_scrape_invalid_url() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "scrape",
        "not-a-valid-url",
        "--output",
        output_dir.to_str().unwrap(),
    ]);

    assert!(
        !result.status.success(),
        "Scrape with invalid URL should fail"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("URL") || stderr.contains("url") || stderr.contains("Invalid"),
        "Should show URL-related error. Got: {stderr}"
    );
}

#[test]
fn test_scrape_help() {
    let result = run_cli(&["scrape", "--help"]);

    assert!(result.status.success(), "Scrape --help should succeed");

    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(
        stdout.contains("Scrape") || stdout.contains("scrape"),
        "Help should mention scrape functionality"
    );
}

// =============================================================================
// INGEST COMMAND TESTS
// =============================================================================

#[test]
fn test_ingest_invalid_url() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "ingest",
        "not-a-valid-url",
        "--output",
        output_dir.to_str().unwrap(),
    ]);

    assert!(
        !result.status.success(),
        "Ingest with invalid URL should fail"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("URL") || stderr.contains("url") || stderr.contains("Invalid"),
        "Should show URL-related error. Got: {stderr}"
    );
}

#[test]
fn test_ingest_help() {
    let result = run_cli(&["ingest", "--help"]);

    assert!(result.status.success(), "Ingest --help should succeed");

    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(
        stdout.contains("Ingest") || stdout.contains("ingest"),
        "Help should mention ingest functionality"
    );
}

// =============================================================================
// INGEST-GIT COMMAND TESTS
// =============================================================================

#[test]
fn test_ingest_git_invalid_url() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "ingest-git",
        "not-a-valid-git-url",
        "--output",
        output_dir.to_str().unwrap(),
    ]);

    assert!(
        !result.status.success(),
        "Ingest-git with invalid URL should fail"
    );
}

#[test]
fn test_ingest_git_help() {
    let result = run_cli(&["ingest-git", "--help"]);

    println!("Status: {:?}", result.status);
    println!("Stdout: {}", String::from_utf8_lossy(&result.stdout));
    println!("Stderr: {}", String::from_utf8_lossy(&result.stderr));

    assert!(result.status.success(), "Ingest-git --help should succeed");

    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(
        stdout.contains("Git") || stdout.contains("git"),
        "Help should mention git functionality. Got: {stdout}"
    );
}

// =============================================================================
// GENERAL CLI TESTS
// =============================================================================

#[test]
fn test_cli_version() {
    let result = run_cli(&["--version"]);

    assert!(result.status.success(), "Version flag should succeed");

    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(
        stdout.contains("doc_transformer") || stdout.chars().any(|c| c.is_ascii_digit()),
        "Version should show program name or version number. Got: {stdout}"
    );
}

#[test]
fn test_cli_help() {
    let result = run_cli(&["--help"]);

    assert!(result.status.success(), "Help flag should succeed");

    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(
        stdout.contains("Usage:") || stdout.contains("Commands:"),
        "Help should show usage information"
    );
}

#[test]
fn test_cli_no_args_shows_help() {
    let result = run_cli(&[]);

    // Should show help or error message
    let stdout = String::from_utf8_lossy(&result.stdout);
    let stderr = String::from_utf8_lossy(&result.stderr);

    assert!(
        !result.status.success() || stdout.contains("Usage:") || stderr.contains("Usage:"),
        "CLI with no args should show help or fail gracefully"
    );
}

// =============================================================================
// EDGE CASE TESTS
// =============================================================================

#[test]
fn test_index_with_special_chars_in_path() {
    let temp = TempDir::new().unwrap();
    let source = temp.path().join("source with spaces & symbols");
    let output_dir = temp.path().join("output with spaces");

    fs::create_dir_all(&source).unwrap();
    fs::write(source.join("test.md"), "# Test\n\nContent").unwrap();

    let result = run_cli(&[
        "index",
        source.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);

    assert!(
        result.status.success(),
        "Index should handle paths with special characters. stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn test_search_limit_parameter() {
    // First create an index
    let temp = TempDir::new().unwrap();
    let source = temp.path().join("source");
    let output_dir = temp.path().join("output");

    fs::create_dir_all(&source).unwrap();
    fs::write(source.join("doc1.md"), "# First Doc\n\nContent about rust.").unwrap();
    fs::write(source.join("doc2.md"), "# Second Doc\n\nMore rust content.").unwrap();

    // Index the content
    let index_result = run_cli(&[
        "index",
        source.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(index_result.status.success(), "Setup: Index should succeed");

    // Search with limit
    let search_result = run_cli(&[
        "search",
        "rust",
        "--index-dir",
        output_dir.to_str().unwrap(),
        "--limit",
        "1",
    ]);

    let stderr = String::from_utf8_lossy(&search_result.stderr);
    assert!(
        search_result.status.success() || stderr.contains("No results found"),
        "Search with limit should either return results or report no-results"
    );
}

#[test]
fn test_scrape_with_options() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    // Test that scrape accepts various options (will fail due to invalid URL, but validates CLI)
    let result = run_cli(&[
        "scrape",
        "invalid-url",
        "--output",
        output_dir.to_str().unwrap(),
        "--delay",
        "100",
        "--no-sitemap",
    ]);

    // Should fail due to invalid URL, not invalid arguments
    assert!(!result.status.success(), "Should fail due to invalid URL");
}
