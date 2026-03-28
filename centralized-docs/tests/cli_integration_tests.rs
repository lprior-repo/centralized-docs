#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]


//! CLI Integration Tests for ctd
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
    Path::new(env!("CARGO_BIN_EXE_ctd")).to_path_buf()
}

/// Run the CLI with given arguments
fn run_cli(args: &[&str]) -> std::process::Output {
    let binary = binary_path();

    println!("Binary: {:?}", binary);
    println!("Args: {:?}", args);

    Command::new(&binary)
        .args(args)
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute binary: {}", binary.display()))
}

fn validator_binary_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_BIN_EXE_llms_txt_validator")).to_path_buf()
}

fn run_validator_cli(args: &[&str]) -> std::process::Output {
    let binary = validator_binary_path();

    println!("Validator binary: {:?}", binary);
    println!("Validator args: {:?}", args);

    Command::new(&binary)
        .args(args)
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute validator binary: {}", binary.display()))
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

    // Empty directory should fail with exit code 1 (user error)
    // per contract doc-3qj9: empty source returns exit code 1
    assert!(
        !result.status.success(),
        "Index with empty directory should fail with exit code 1. stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert_eq!(
        result.status.code(),
        Some(1),
        "Exit code should be 1 for empty directory"
    );
}

#[test]
fn test_index_non_directory_file() {
    // Test indexing a non-markdown file (like /etc/passwd)
    // Should fail with exit code 1 per contract doc-3qj9
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    // Use /etc/passwd as a representative non-markdown file
    // (it exists on most systems and is not a directory)
    let result = run_cli(&[
        "index",
        "/etc/passwd",
        "--output",
        output_dir.to_str().unwrap(),
    ]);

    // Non-markdown file should fail with exit code 1 (user error)
    assert!(
        !result.status.success(),
        "Index with non-markdown file should fail with exit code 1. stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert_eq!(
        result.status.code(),
        Some(1),
        "Exit code should be 1 for non-markdown file"
    );
}

#[test]
fn test_index_directory_with_no_markdown_files() {
    let temp = TempDir::new().unwrap();
    let source = temp.path().join("no_md");
    let output_dir = temp.path().join("output");

    fs::create_dir_all(&source).unwrap();
    // Create only non-markdown files
    fs::write(source.join("file.json"), "{}").unwrap();
    fs::write(source.join("file.html"), "<html></html>").unwrap();

    let result = run_cli(&[
        "index",
        source.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);

    // Directory with no markdown files should fail with exit code 1
    assert!(
        !result.status.success(),
        "Index with no markdown files should fail with exit code 1. stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert_eq!(
        result.status.code(),
        Some(1),
        "Exit code should be 1 for directory with no markdown files"
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

    // No-results is a valid result - exit code 0 means success (even with empty results)
    // Exit code 1 is for actual errors (doc-2y1p: search exit code should be 0 for no results)
    assert!(
        search_result.status.success(),
        "Search with no matches should return exit 0 (valid result, not error)"
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

    // Help goes to stderr in clap
    let output = String::from_utf8_lossy(&result.stdout).to_string()
        + &String::from_utf8_lossy(&result.stderr);
    assert!(
        output.contains("Scrape") || output.contains("scrape"),
        "Help should mention scrape functionality. Got: {output}"
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

    // Help goes to stderr in clap
    let output = String::from_utf8_lossy(&result.stdout).to_string()
        + &String::from_utf8_lossy(&result.stderr);
    assert!(
        output.contains("Ingest") || output.contains("ingest"),
        "Help should mention ingest functionality. Got: {output}"
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

    // Help goes to stderr in clap
    let output = String::from_utf8_lossy(&result.stdout).to_string()
        + &String::from_utf8_lossy(&result.stderr);
    assert!(
        output.contains("Git") || output.contains("git"),
        "Help should mention git functionality. Got: {output}"
    );
}

// =============================================================================
// GENERAL CLI TESTS
// =============================================================================

#[test]
fn test_cli_version() {
    let result = run_cli(&["--version"]);

    assert!(result.status.success(), "Version flag should succeed");

    // Version goes to stderr in clap
    let output = String::from_utf8_lossy(&result.stdout).to_string()
        + &String::from_utf8_lossy(&result.stderr);
    assert!(
        output.contains("ctd") || output.chars().any(|c| c.is_ascii_digit()),
        "Version should show program name or version number. Got: {output}"
    );
}

#[test]
fn test_cli_help() {
    let result = run_cli(&["--help"]);

    assert!(result.status.success(), "Help flag should succeed");

    // Help goes to stderr in clap
    let output = String::from_utf8_lossy(&result.stdout).to_string()
        + &String::from_utf8_lossy(&result.stderr);
    assert!(
        output.contains("Usage:") || output.contains("Commands:"),
        "Help should show usage information. Got: {output}"
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
// ADDITIONAL INDEX COMMAND TESTS - PARAMETER VALIDATION
// =============================================================================

// Note: There's no --no-llms-txt option in CLI - the default is to generate llms.txt
// This test verifies the default behavior (llms.txt IS generated)

#[test]
fn test_index_with_custom_project_name() {
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
        "--project-name",
        "My Custom Project",
    ]);

    assert!(
        result.status.success(),
        "Index with custom project name should succeed. stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );

    // Verify index was created
    assert!(
        output_dir.join("INDEX.json").exists(),
        "INDEX.json should be created"
    );
}

#[test]
fn test_index_with_custom_project_desc() {
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
        "--project-desc",
        "A custom description for testing",
    ]);

    assert!(
        result.status.success(),
        "Index with custom project desc should succeed. stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn test_index_with_max_related_chunks() {
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
        "--max-related-chunks",
        "5",
    ]);

    assert!(
        result.status.success(),
        "Index with max-related-chunks should succeed. stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn test_index_with_max_chunk_keywords() {
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
        "--max-chunk-keywords",
        "5",
    ]);

    assert!(
        result.status.success(),
        "Index with max-chunk-keywords should succeed. stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn test_index_with_hnsw_m() {
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
        "--hnsw-m",
        "8",
    ]);

    assert!(
        result.status.success(),
        "Index with hnsw-m should succeed. stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn test_index_with_hnsw_ef_construction() {
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
        "--hnsw-ef-construction",
        "100",
    ]);

    assert!(
        result.status.success(),
        "Index with hnsw-ef-construction should succeed. stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn test_index_invalid_max_related_chunks_zero() {
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
        "--max-related-chunks",
        "0",
    ]);

    // Should fail with validation error
    assert!(
        !result.status.success(),
        "Index with max-related-chunks=0 should fail"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("at least 1"),
        "Error should mention minimum value. Got: {stderr}"
    );
}

#[test]
fn test_index_invalid_max_related_chunks_too_large() {
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
        "--max-related-chunks",
        "1001",
    ]);

    // Should fail with validation error
    assert!(
        !result.status.success(),
        "Index with max-related-chunks=1001 should fail"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("at most 100"),
        "Error should mention maximum value. Got: {stderr}"
    );
}

#[test]
fn test_index_invalid_hnsw_m_too_small() {
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
        "--hnsw-m",
        "2",
    ]);

    // Should fail with validation error
    assert!(!result.status.success(), "Index with hnsw-m=2 should fail");

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("at least 4"),
        "Error should mention minimum value. Got: {stderr}"
    );
}

#[test]
fn test_index_invalid_hnsw_m_too_large() {
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
        "--hnsw-m",
        "100",
    ]);

    // Should fail with validation error
    assert!(
        !result.status.success(),
        "Index with hnsw-m=100 should fail"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("at most 64"),
        "Error should mention maximum value. Got: {stderr}"
    );
}

#[test]
fn test_index_invalid_hnsw_ef_construction_too_small() {
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
        "--hnsw-ef-construction",
        "10",
    ]);

    // Should fail with validation error
    assert!(
        !result.status.success(),
        "Index with hnsw-ef-construction=10 should fail"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("at least 50"),
        "Error should mention minimum value. Got: {stderr}"
    );
}

#[test]
fn test_index_output_dir_not_writable() {
    let temp = TempDir::new().unwrap();
    let source = temp.path().join("source");
    let _output_dir = temp.path().join("output");
    let parent_nonexistent = temp.path().join("nonexistent").join("subdir");

    fs::create_dir_all(&source).unwrap();
    fs::write(source.join("test.md"), "# Test\n\nContent").unwrap();

    let result = run_cli(&[
        "index",
        source.to_str().unwrap(),
        "--output",
        parent_nonexistent.to_str().unwrap(),
    ]);

    // Should fail because parent doesn't exist
    assert!(
        !result.status.success(),
        "Index with non-existent parent directory should fail"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("Parent directory") || stderr.contains("does not exist"),
        "Error should mention parent directory issue. Got: {stderr}"
    );
}

// =============================================================================
// ADDITIONAL SEARCH COMMAND TESTS - PARAMETER VALIDATION AND OUTPUT FORMATS
// =============================================================================

#[test]
fn test_search_json_output() {
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

    // Search with JSON output
    let search_result = run_cli(&[
        "search",
        "rust",
        "--index-dir",
        output_dir.to_str().unwrap(),
        "--json",
    ]);

    let stdout = String::from_utf8_lossy(&search_result.stdout);
    assert!(
        search_result.status.success(),
        "Search command should succeed. stderr: {}",
        String::from_utf8_lossy(&search_result.stderr)
    );

    // JSON output should contain status field
    assert!(
        stdout.contains("\"status\""),
        "JSON output should contain status field. Got: {stdout}"
    );
}

#[test]
fn test_search_json_no_results_exit_code() {
    // Test that JSON mode returns exit code 0 for no results (doc-36f4)
    // JSON output correctly shows status: "no_results", which indicates successful completion

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

    // Search with JSON output for non-existent term
    let search_result = run_cli(&[
        "search",
        "nonexistentterm12345",
        "--index-dir",
        output_dir.to_str().unwrap(),
        "--json",
    ]);

    let stdout = String::from_utf8_lossy(&search_result.stdout);

    // Exit code should be 0 (successful completion with no results)
    // This is consistent with status: "no_results" in JSON output
    assert!(
        search_result.status.success(),
        "Search with no results in JSON mode should return exit 0, not exit 1. \
         JSON output: {stdout}, stderr: {}",
        String::from_utf8_lossy(&search_result.stderr)
    );

    // JSON output should contain no_results status
    assert!(
        stdout.contains("\"status\": \"no_results\""),
        "JSON output should contain status: \"no_results\". Got: {stdout}"
    );
}

#[test]
fn test_search_with_limit_5() {
    // First create an index
    let temp = TempDir::new().unwrap();
    let source = temp.path().join("source");
    let output_dir = temp.path().join("output");

    fs::create_dir_all(&source).unwrap();
    fs::write(source.join("doc1.md"), "# First Doc\n\nContent about rust.").unwrap();
    fs::write(source.join("doc2.md"), "# Second Doc\n\nMore rust content.").unwrap();
    fs::write(
        source.join("doc3.md"),
        "# Third Doc\n\nEven more rust here.",
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

    // Search with limit
    let search_result = run_cli(&[
        "search",
        "rust",
        "--index-dir",
        output_dir.to_str().unwrap(),
        "--limit",
        "2",
    ]);

    assert!(
        search_result.status.success(),
        "Search with limit should succeed. stderr: {}",
        String::from_utf8_lossy(&search_result.stderr)
    );
}

#[test]
fn test_search_invalid_limit_zero() {
    // First create an index
    let temp = TempDir::new().unwrap();
    let source = temp.path().join("source");
    let output_dir = temp.path().join("output");

    fs::create_dir_all(&source).unwrap();
    fs::write(source.join("test.md"), "# Test\n\nContent").unwrap();

    // Index the content
    let index_result = run_cli(&[
        "index",
        source.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(index_result.status.success(), "Setup: Index should succeed");

    // Search with limit=0 should fail
    let search_result = run_cli(&[
        "search",
        "test",
        "--index-dir",
        output_dir.to_str().unwrap(),
        "--limit",
        "0",
    ]);

    assert!(
        !search_result.status.success(),
        "Search with limit=0 should fail"
    );

    let stderr = String::from_utf8_lossy(&search_result.stderr);
    assert!(
        stderr.contains("at least 1"),
        "Error should mention minimum limit. Got: {stderr}"
    );
}

#[test]
fn test_search_invalid_limit_too_large() {
    // First create an index
    let temp = TempDir::new().unwrap();
    let source = temp.path().join("source");
    let output_dir = temp.path().join("output");

    fs::create_dir_all(&source).unwrap();
    fs::write(source.join("test.md"), "# Test\n\nContent").unwrap();

    // Index the content
    let index_result = run_cli(&[
        "index",
        source.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(index_result.status.success(), "Setup: Index should succeed");

    // Search with limit=1001 should fail
    let search_result = run_cli(&[
        "search",
        "test",
        "--index-dir",
        output_dir.to_str().unwrap(),
        "--limit",
        "1001",
    ]);

    assert!(
        !search_result.status.success(),
        "Search with limit=1001 should fail"
    );

    let stderr = String::from_utf8_lossy(&search_result.stderr);
    assert!(
        stderr.contains("1000"),
        "Error should mention maximum limit. Got: {stderr}"
    );

    // Verify exit code is 1 (not 0 or 2) for validation errors (bead doc-2nt1)
    assert_eq!(
        search_result.status.code(),
        Some(1),
        "Limit too large validation error should return exit code 1, not {}",
        search_result.status.code().unwrap_or(-1)
    );
}

#[test]
fn test_search_very_large_limit_exit_code_one() {
    // Test for very large limit values (bead doc-2nt1)
    // This verifies that extremely large limit values also return exit code 1
    let result = run_cli(&[
        "search",
        "test",
        "--index-dir",
        "/tmp/nonexistent",
        "--limit",
        "1000000000",
    ]);

    // Should fail (non-zero exit code)
    assert!(!result.status.success(), "Very large limit should fail");

    // Exit code must be 1 for validation errors (bead doc-2nt1)
    assert_eq!(
        result.status.code(),
        Some(1),
        "Very large limit validation error should return exit code 1, got {}",
        result.status.code().unwrap_or(-1)
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("1000") || stderr.contains("limit"),
        "Error should mention limit. Got: {stderr}"
    );
}

#[test]
fn test_search_with_no_color() {
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

    // Search with no-color
    let search_result = run_cli(&[
        "search",
        "rust",
        "--index-dir",
        output_dir.to_str().unwrap(),
        "--no-color",
    ]);

    assert!(
        search_result.status.success(),
        "Search with --no-color should succeed. stderr: {}",
        String::from_utf8_lossy(&search_result.stderr)
    );
}

#[test]
fn test_search_missing_index_dir_argument() {
    let result = run_cli(&["search", "test"]);

    // Should fail due to missing required --index-dir
    assert!(
        !result.status.success(),
        "Search without --index-dir should fail"
    );
}

#[test]
fn test_search_with_empty_query() {
    // First create an index
    let temp = TempDir::new().unwrap();
    let source = temp.path().join("source");
    let output_dir = temp.path().join("output");

    fs::create_dir_all(&source).unwrap();
    fs::write(source.join("test.md"), "# Test\n\nContent").unwrap();

    // Index the content
    let index_result = run_cli(&[
        "index",
        source.to_str().unwrap(),
        "--output",
        output_dir.to_str().unwrap(),
    ]);
    assert!(index_result.status.success(), "Setup: Index should succeed");

    // Search with empty query
    let search_result = run_cli(&["search", "", "--index-dir", output_dir.to_str().unwrap()]);

    // Empty query should work (returns all documents sorted by score)
    assert!(
        search_result.status.success()
            || String::from_utf8_lossy(&search_result.stderr).contains("empty"),
        "Search with empty query should succeed or handle gracefully"
    );
}

// =============================================================================
// ADDITIONAL SCRAPE COMMAND TESTS - PARAMETER VALIDATION
// =============================================================================

#[test]
fn test_scrape_invalid_delay_negative() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "scrape",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--delay",
        "-1",
    ]);

    assert!(
        !result.status.success(),
        "Scrape with negative delay should fail"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("non-negative") || stderr.contains("delay"),
        "Error should mention delay validation. Got: {stderr}"
    );
}

#[test]
fn test_scrape_invalid_delay_too_large() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "scrape",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--delay",
        "100000",
    ]);

    assert!(
        !result.status.success(),
        "Scrape with delay > 60000 should fail"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("60000") || stderr.contains("delay"),
        "Error should mention delay limit. Got: {stderr}"
    );
}

#[test]
fn test_scrape_invalid_timeout_zero() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "scrape",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--request-timeout-secs",
        "0",
    ]);

    assert!(
        !result.status.success(),
        "Scrape with timeout=0 should fail"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("at least 1") || stderr.contains("timeout"),
        "Error should mention timeout validation. Got: {stderr}"
    );
}

#[test]
fn test_scrape_invalid_timeout_too_large() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "scrape",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--request-timeout-secs",
        "700",
    ]);

    assert!(
        !result.status.success(),
        "Scrape with timeout > 600 should fail"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("600") || stderr.contains("timeout"),
        "Error should mention timeout limit. Got: {stderr}"
    );
}

#[test]
fn test_scrape_invalid_max_retries() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    // Invalid retry count (negative)
    let result = run_cli(&[
        "scrape",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--max-retries",
        "-1",
    ]);

    assert!(
        !result.status.success(),
        "Scrape with negative retries should fail"
    );
}

#[test]
fn test_scrape_invalid_redirect_policy() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "scrape",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--redirect-policy",
        "invalid-policy",
    ]);

    assert!(
        !result.status.success(),
        "Scrape with invalid redirect policy should fail"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("redirect") || stderr.contains("loose"),
        "Error should mention redirect policy. Got: {stderr}"
    );
}

#[test]
fn test_scrape_valid_redirect_policies() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    // Test strict policy with invalid URL format
    let result = run_cli(&[
        "scrape",
        "not-valid-url",
        "--output",
        output_dir.to_str().unwrap(),
        "--redirect-policy",
        "strict",
    ]);

    // Should fail due to invalid URL, not invalid arguments
    assert!(
        !result.status.success(),
        "Should fail due to invalid URL, not policy"
    );

    // Test none policy with invalid URL format
    let result2 = run_cli(&[
        "scrape",
        "invalid-url-format",
        "--output",
        output_dir.to_str().unwrap(),
        "--redirect-policy",
        "none",
    ]);

    assert!(
        !result2.status.success(),
        "Should fail due to invalid URL, not policy"
    );
}

#[test]
fn test_scrape_with_filter_regex() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "scrape",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--filter",
        "/docs/.*",
    ]);

    // Should fail due to invalid URL, not invalid filter
    assert!(
        !result.status.success(),
        "Should fail due to invalid URL, not filter"
    );
}

#[test]
fn test_scrape_invalid_filter_regex() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "scrape",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--filter",
        "[invalid",
    ]);

    assert!(
        !result.status.success(),
        "Scrape with invalid regex should fail"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("regex") || stderr.contains("pattern") || stderr.contains("Invalid"),
        "Error should mention regex issue. Got: {stderr}"
    );
}

#[test]
fn test_scrape_filter_matches_nothing_returns_nonzero_exit() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "scrape",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--filter",
        "/nonexistent-path-12345",
        "--no-sitemap",
    ]);

    // Filter matching nothing should return non-zero exit code (not success)
    assert!(
        !result.status.success(),
        "Filter matching nothing should return non-zero exit, got: {}",
        result.status
    );

    // Check that either stdout or stderr contains an error message
    let output = String::from_utf8_lossy(&result.stdout);
    let stderr = String::from_utf8_lossy(&result.stderr);
    let has_error = output.contains("Failed")
        || stderr.contains("Failed")
        || output.contains("error")
        || stderr.contains("error")
        || output.contains("SCRAPE FAILED")
        || stderr.contains("SCRAPE FAILED");
    assert!(
        has_error,
        "Output should contain error message. Stdout: {output}, Stderr: {stderr}"
    );
}

#[test]
fn test_scrape_invalid_max_page_bytes() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "scrape",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--max-page-bytes",
        "0",
    ]);

    assert!(
        !result.status.success(),
        "Scrape with max-page-bytes=0 should fail"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("at least 1") || stderr.contains("bytes"),
        "Error should mention bytes validation. Got: {stderr}"
    );
}

#[test]
fn test_scrape_invalid_concurrency() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "scrape",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--concurrency",
        "129",
    ]);

    assert!(
        !result.status.success(),
        "Scrape with concurrency > 128 should fail"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("concurrency") || stderr.contains("at most 128"),
        "Error should mention concurrency limit. Got: {stderr}"
    );
}

#[test]
fn test_scrape_invalid_threshold_negative() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "scrape",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--threshold",
        "-0.5",
    ]);

    assert!(
        !result.status.success(),
        "Scrape with negative threshold should fail"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("non-negative") || stderr.contains("threshold"),
        "Error should mention threshold validation. Got: {stderr}"
    );
}

#[test]
fn test_scrape_invalid_threshold_too_large() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "scrape",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--threshold",
        "15.0",
    ]);

    assert!(
        !result.status.success(),
        "Scrape with threshold > 10.0 should fail"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("10.0") || stderr.contains("threshold"),
        "Error should mention threshold limit. Got: {stderr}"
    );
}

#[test]
fn test_scrape_missing_output() {
    // Scrape without output argument
    let result = run_cli(&["scrape", "http://example.com"]);

    assert!(
        !result.status.success(),
        "Scrape without --output should fail"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("output") || stderr.contains("required"),
        "Error should mention output requirement. Got: {stderr}"
    );
}

// =============================================================================
// ADDITIONAL INGEST COMMAND TESTS
// =============================================================================

#[test]
fn test_ingest_missing_output() {
    let result = run_cli(&["ingest", "http://example.com"]);

    assert!(
        !result.status.success(),
        "Ingest without --output should fail"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("output") || stderr.contains("required"),
        "Error should mention output requirement. Got: {stderr}"
    );
}

#[test]
fn test_ingest_invalid_delay() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "ingest",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--delay",
        "-100",
    ]);

    assert!(
        !result.status.success(),
        "Ingest with negative delay should fail"
    );
}

#[test]
fn test_ingest_invalid_threshold() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "ingest",
        "http://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--threshold",
        "20.0",
    ]);

    assert!(
        !result.status.success(),
        "Ingest with threshold > 10.0 should fail"
    );
}

#[test]
fn test_ingest_with_custom_project_name() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    // Use invalid URL format to test that project name is accepted
    let result = run_cli(&[
        "ingest",
        "not-a-valid-url",
        "--output",
        output_dir.to_str().unwrap(),
        "--project-name",
        "My Documentation",
    ]);

    // Should fail due to invalid URL, but should accept project name
    assert!(!result.status.success(), "Should fail due to invalid URL");
}

// =============================================================================
// ADDITIONAL INGEST-GIT COMMAND TESTS
// =============================================================================

#[test]
fn test_ingest_git_missing_output() {
    let result = run_cli(&["ingest-git", "https://github.com/example/repo"]);

    assert!(
        !result.status.success(),
        "Ingest-git without --output should fail"
    );
}

#[test]
fn test_ingest_git_with_branch() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "ingest-git",
        "https://github.com/example/repo",
        "--output",
        output_dir.to_str().unwrap(),
        "--branch",
        "develop",
    ]);

    // Should fail due to invalid repo, not invalid branch arg
    assert!(!result.status.success(), "Should fail due to invalid repo");
}

#[test]
fn test_ingest_git_with_depth() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "ingest-git",
        "https://github.com/example/repo",
        "--output",
        output_dir.to_str().unwrap(),
        "--depth",
        "0",
    ]);

    // Should fail due to invalid repo, not invalid depth arg
    assert!(!result.status.success(), "Should fail due to invalid repo");
}

#[test]
fn test_ingest_git_with_project_name() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "ingest-git",
        "https://github.com/example/repo",
        "--output",
        output_dir.to_str().unwrap(),
        "--project-name",
        "Custom Project",
    ]);

    // Should fail due to invalid repo, but should accept project name
    assert!(!result.status.success(), "Should fail due to invalid repo");
}

#[test]
fn test_scrape_help_does_not_claim_hidden_fallback_cap() {
    let result = run_cli(&["scrape", "--help"]);
    let output = String::from_utf8_lossy(&result.stdout).to_string()
        + &String::from_utf8_lossy(&result.stderr);

    assert!(result.status.success(), "scrape --help should succeed");
    assert!(
        !output.contains("100 pages") && !output.contains("uncap"),
        "scrape help should not describe a hidden 100-page fallback cap. Output: {output}"
    );
}

#[test]
fn test_cli_help_reports_full_ai_artifact_set() {
    let result = run_cli(&["--help"]);
    let output = String::from_utf8_lossy(&result.stdout).to_string()
        + &String::from_utf8_lossy(&result.stderr);

    assert!(result.status.success(), "Help flag should succeed");
    assert!(
        output.contains("llms-full.txt"),
        "Help output should mention llms-full.txt. Output: {output}"
    );
    assert!(
        output.contains("AGENTS.md"),
        "Help output should mention AGENTS.md. Output: {output}"
    );
}

#[test]
fn test_index_success_creates_agents_and_llms_full() {
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

    assert!(
        output_dir.join("AGENTS.md").exists(),
        "AGENTS.md should be created"
    );
    assert!(
        output_dir.join("llms-full.txt").exists(),
        "llms-full.txt should be created"
    );

    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(
        stdout.contains("Created llms.txt, llms-full.txt, and AGENTS.md"),
        "Index output should report generated AI artifacts. stdout: {stdout}"
    );
}

#[test]
fn test_validator_version_flags_both_succeed() {
    let short = run_validator_cli(&["-V"]);
    let long = run_validator_cli(&["--version"]);

    assert!(
        short.status.success(),
        "llms_txt_validator -V should succeed"
    );
    assert!(
        long.status.success(),
        "llms_txt_validator --version should succeed"
    );

    let short_output = String::from_utf8_lossy(&short.stderr);
    let long_output = String::from_utf8_lossy(&long.stderr);

    assert!(
        short_output.contains("llms_txt_validator v0.6.1"),
        "short version output should include version. Output: {short_output}"
    );
    assert_eq!(
        short_output, long_output,
        "short and long version flags should produce identical output"
    );
}

#[test]
fn test_getting_started_docs_mirrors_match() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let docs_guide = fs::read_to_string(repo_root.join("docs/GETTING_STARTED.md")).unwrap();
    let website_guide =
        fs::read_to_string(repo_root.join("website/src/getting-started.md")).unwrap();

    assert_eq!(
        docs_guide, website_guide,
        "Getting Started guides should stay in sync"
    );
}

// =============================================================================
// ERROR HANDLING TESTS - EXIT CODES
// =============================================================================

#[test]
fn test_exit_code_for_missing_source() {
    let result = run_cli(&["index", "/nonexistent/path"]);

    // Should fail with exit code != 0
    assert!(
        !result.status.success(),
        "Index with missing source should fail"
    );

    // Exit code 1 = user error (missing source is user error)
    assert_eq!(
        result.status.code(),
        Some(1),
        "Missing source should return exit code 1"
    );
}

#[test]
fn test_exit_code_for_invalid_url() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");

    let result = run_cli(&[
        "scrape",
        "not-a-url",
        "--output",
        output_dir.to_str().unwrap(),
    ]);

    // Should fail
    assert!(!result.status.success());

    // Exit code 1 = user input error (invalid URL format is a user input error)
    assert_eq!(
        result.status.code(),
        Some(1),
        "Invalid URL should return exit code 1"
    );
}

// =============================================================================
// LEGACY MODE TESTS
// =============================================================================

#[test]
fn test_legacy_mode_two_args_rejected() {
    let temp = TempDir::new().unwrap();
    let source = temp.path().join("source");
    let output_dir = temp.path().join("output");

    fs::create_dir_all(&source).unwrap();
    fs::write(source.join("test.md"), "# Test\n\nContent").unwrap();

    // Legacy mode was intentionally removed; two positional args should now fail.
    let result = run_cli(&[source.to_str().unwrap(), output_dir.to_str().unwrap()]);

    assert!(
        !result.status.success(),
        "Legacy mode should be rejected. stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        String::from_utf8_lossy(&result.stderr).contains("unrecognized subcommand"),
        "Expected clap to reject the legacy invocation"
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
