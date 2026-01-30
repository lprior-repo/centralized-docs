//! CLI validation tests for P0 CRITICAL fixes: required CLI arguments
//!
//! This module tests that required CLI arguments are properly enforced
//! for INDEX, SCRAPE, and SEARCH commands.
//!
//! Test coverage:
//! - INDEX command: --output is required
//! - SCRAPE command: --output is required
//! - SEARCH command: --index-dir is required

use std::io::Read;
use std::process::{Command, Stdio};

/// Helper function to run the doc_transformer binary with arguments
fn run_command(args: &[&str]) -> (bool, String, String) {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet", "--bin", "doc_transformer", "--"])
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

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

    let status = child.wait().expect("Failed to wait for command");
    let success = status.success();

    (success, stdout, stderr)
}

// ============================================================================
// INDEX COMMAND TESTS (--output required)
// ============================================================================

#[test]
fn test_index_command_missing_output_fails() {
    // INDEX command without --output should fail
    let (success, stdout, stderr) = run_command(&["index", "/some/source/path"]);

    let output = format!("{stdout}\n{stderr}");

    // Should fail
    assert!(
        !success,
        "INDEX command without --output should fail.\nOutput: {output}"
    );

    // Error message should mention the missing required argument
    assert!(
        output.contains("output") || output.contains("required") || output.contains("OUTPUT"),
        "Error message should mention 'output' or 'required'.\nOutput: {output}"
    );
}

#[test]
fn test_index_command_with_output_succeeds_parsing() {
    // INDEX command with --output should succeed at parsing level
    // (will fail at execution because paths don't exist, but parsing should work)
    let (success, stdout, stderr) = run_command(&[
        "index",
        "/nonexistent/source",
        "--output",
        "/nonexistent/output",
    ]);

    let output = format!("{stdout}\n{stderr}");

    // If parsing failed, output should NOT complain about missing 'output'
    // If parsing succeeded, execution will fail for other reasons (paths don't exist)
    // We just want to ensure clap doesn't complain about missing required argument
    assert!(
        !output.contains("required") || !output.contains("output"),
        "When --output is provided, should not complain about missing required 'output'.\nOutput: {output}"
    );

    // The error should be about path/execution, not missing argument
    if !success {
        assert!(
            output.contains("No such file")
                || output.contains("does not exist")
                || output.contains("not found")
                || output.contains("directory"),
            "Error should be about path issues, not missing arguments.\nOutput: {output}"
        );
    }
}

#[test]
fn test_index_command_invalid_output_path() {
    // INDEX command with invalid output path (non-existent parent)
    let (success, stdout, stderr) = run_command(&[
        "index",
        "/nonexistent/source",
        "--output",
        "/nonexistent/parent/that/does/not/exist/output",
    ]);

    let output = format!("{stdout}\n{stderr}");

    // Should fail (either parsing or execution)
    assert!(
        !success,
        "INDEX with invalid output path should fail.\nOutput: {output}"
    );

    // Error should mention the path issue, not missing argument
    assert!(
        !output.contains("required") || !output.contains("output"),
        "Should not complain about missing 'output' when it was provided.\nOutput: {output}"
    );
}

#[test]
fn test_index_command_shorthand_output_flag() {
    // INDEX command with -o shorthand
    let (_success, stdout, stderr) =
        run_command(&["index", "/nonexistent/source", "-o", "/nonexistent/output"]);

    let output = format!("{stdout}\n{stderr}");

    // -o is shorthand for --output, should work the same
    assert!(
        !output.contains("required") || !output.contains("output"),
        "Shorthand -o should be recognized.\nOutput: {output}"
    );
}

// ============================================================================
// SCRAPE COMMAND TESTS (--output required)
// ============================================================================

#[test]
fn test_scrape_command_missing_output_fails() {
    // SCRAPE command without --output should fail
    let (success, stdout, stderr) = run_command(&["scrape", "https://example.com"]);

    let output = format!("{stdout}\n{stderr}");

    // Should fail
    assert!(
        !success,
        "SCRAPE command without --output should fail.\nOutput: {output}"
    );

    // Error message should mention the missing required argument
    assert!(
        output.contains("output")
            || output.contains("required")
            || output.contains("OUTPUT")
            || output.contains("DIR"),
        "Error message should mention 'output', 'required', 'OUTPUT', or 'DIR'.\nOutput: {output}"
    );
}

#[test]
fn test_scrape_command_with_output_succeeds_parsing() {
    // SCRAPE command with --output should succeed at parsing level
    let (_success, stdout, stderr) = run_command(&[
        "scrape",
        "https://example.com",
        "--output",
        "/tmp/test-output",
    ]);

    let output = format!("{stdout}\n{stderr}");

    // Clap should accept the arguments (execution may fail for other reasons like network)
    // We just want to ensure clap doesn't complain about missing required argument
    assert!(
        !output.contains("required") || !output.contains("output") && !output.contains("missing"),
        "When --output is provided, should not complain about missing required 'output'.\nOutput: {output}"
    );
}

#[test]
fn test_scrape_command_invalid_output_path() {
    // SCRAPE command with output path that has non-existent parent
    let (success, stdout, stderr) = run_command(&[
        "scrape",
        "https://example.com",
        "--output",
        "/nonexistent/parent/that/does/not/exist/output",
    ]);

    let output = format!("{stdout}\n{stderr}");

    // Should fail
    assert!(
        !success,
        "SCRAPE with invalid output path should fail.\nOutput: {output}"
    );

    // Error should be about path, not missing argument
    assert!(
        !output.contains("required") || !output.contains("output"),
        "Should not complain about missing 'output' when it was provided.\nOutput: {output}"
    );
}

#[test]
fn test_scrape_command_shorthand_output_flag() {
    // SCRAPE command with -o shorthand
    let (_success, stdout, stderr) =
        run_command(&["scrape", "https://example.com", "-o", "/tmp/test-output"]);

    let output = format!("{stdout}\n{stderr}");

    // -o is shorthand for --output, should work the same
    assert!(
        !output.contains("required") || !output.contains("output"),
        "Shorthand -o should be recognized.\nOutput: {output}"
    );
}

#[test]
fn test_scrape_command_with_url_and_output() {
    // Full scrape command with URL and output
    let (success, stdout, stderr) = run_command(&[
        "scrape",
        "https://docs.example.com",
        "--output",
        "/tmp/scrape-test",
    ]);

    let output = format!("{stdout}\n{stderr}");

    // Parsing should succeed (execution may fail for network/path reasons)
    // The important thing is no complaint about missing 'output'
    assert!(
        !output.contains("required") || !output.contains("output"),
        "SCRAPE with URL and --output should parse correctly.\nOutput: {output}"
    );

    // If execution failed, it should be for a non-parsing reason
    if !success {
        assert!(
            output.contains("No such file")
                || output.contains("does not exist")
                || output.contains("error")
                || !output.is_empty(),
            "Execution errors should have descriptive messages.\nOutput: {output}"
        );
    }
}

// ============================================================================
// SEARCH COMMAND TESTS (--index-dir required)
// ============================================================================

#[test]
fn test_search_command_missing_index_dir_fails() {
    // SEARCH command without --index-dir should fail
    let (success, stdout, stderr) = run_command(&["search", "test query"]);

    let output = format!("{stdout}\n{stderr}");

    // Should fail
    assert!(
        !success,
        "SEARCH command without --index-dir should fail.\nOutput: {output}"
    );

    // Error message should mention the missing required argument
    assert!(
        output.contains("index-dir") || output.contains("required") || output.contains("index_dir") || output.contains("DIR"),
        "Error message should mention 'index-dir', 'required', 'index_dir', or 'DIR'.\nOutput: {output}"
    );
}

#[test]
fn test_search_command_with_index_dir_succeeds_parsing() {
    // SEARCH command with --index-dir should succeed at parsing level
    let (success, stdout, stderr) =
        run_command(&["search", "test query", "--index-dir", "/nonexistent/index"]);

    let output = format!("{stdout}\n{stderr}");

    // Clap should accept the arguments (execution will fail because index doesn't exist)
    // We just want to ensure clap doesn't complain about missing required argument
    assert!(
        !output.contains("required") || !output.contains("index-dir"),
        "When --index-dir is provided, should not complain about missing required 'index-dir'.\nOutput: {output}"
    );

    // Should fail at execution level (index doesn't exist)
    assert!(
        !success,
        "Should fail at execution because index doesn't exist"
    );

    // Error should be about INDEX.json missing
    assert!(
        output.contains("INDEX.json")
            || output.contains("not found")
            || output.contains("does not exist"),
        "Error should be about missing INDEX.json.\nOutput: {output}"
    );
}

#[test]
fn test_search_command_invalid_index_dir_path() {
    // SEARCH command with invalid index-dir path
    let (success, stdout, stderr) = run_command(&[
        "search",
        "test query",
        "--index-dir",
        "/nonexistent/index/that/does/not/exist",
    ]);

    let output = format!("{stdout}\n{stderr}");

    // Should fail
    assert!(
        !success,
        "SEARCH with invalid index-dir should fail.\nOutput: {output}"
    );

    // Error should be about INDEX.json missing, not missing argument
    assert!(
        !output.contains("required") || !output.contains("index-dir"),
        "Should not complain about missing 'index-dir' when it was provided.\nOutput: {output}"
    );

    assert!(
        output.contains("INDEX.json")
            || output.contains("not found")
            || output.contains("does not exist"),
        "Error should be about missing INDEX.json.\nOutput: {output}"
    );
}

#[test]
fn test_search_command_shorthand_index_dir_flag() {
    // SEARCH command with -i shorthand
    let (_success, stdout, stderr) =
        run_command(&["search", "test query", "-i", "/nonexistent/index"]);

    let output = format!("{stdout}\n{stderr}");

    // -i is shorthand for --index-dir, should work the same
    assert!(
        !output.contains("required") || !output.contains("index-dir"),
        "Shorthand -i should be recognized.\nOutput: {output}"
    );
}

#[test]
fn test_search_command_with_all_arguments() {
    // Full search command with all arguments
    let (success, stdout, stderr) = run_command(&[
        "search",
        "rust programming",
        "--index-dir",
        "/nonexistent/index",
        "--limit",
        "5",
    ]);

    let output = format!("{stdout}\n{stderr}");

    // Parsing should succeed (execution will fail because index doesn't exist)
    // No complaint about missing arguments
    assert!(
        !output.contains("required") || !output.contains("missing"),
        "SEARCH with all arguments should parse correctly.\nOutput: {output}"
    );

    // Should fail at execution level
    assert!(
        !success,
        "Should fail at execution because index doesn't exist"
    );
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_index_command_only_source_no_output() {
    // INDEX with only source, missing output
    let (success, stdout, stderr) = run_command(&["index", "/some/path"]);

    let output = format!("{stdout}\n{stderr}");

    // Should fail
    assert!(
        !success,
        "INDEX with only source should fail.\nOutput: {output}"
    );

    // Error should mention output
    assert!(
        output.contains("output") || output.contains("OUTPUT") || output.contains("required"),
        "Error should mention output or required.\nOutput: {output}"
    );
}

#[test]
fn test_scrape_command_only_url_no_output() {
    // SCRAPE with only URL, missing output
    let (success, stdout, stderr) = run_command(&["scrape", "https://example.com"]);

    let output = format!("{stdout}\n{stderr}");

    // Should fail
    assert!(
        !success,
        "SCRAPE with only URL should fail.\nOutput: {output}"
    );

    // Error should mention output
    assert!(
        output.contains("output")
            || output.contains("OUTPUT")
            || output.contains("required")
            || output.contains("DIR"),
        "Error should mention output or required.\nOutput: {output}"
    );
}

#[test]
fn test_search_command_only_query_no_index_dir() {
    // SEARCH with only query, missing index-dir
    let (success, stdout, stderr) = run_command(&["search", "test"]);

    let output = format!("{stdout}\n{stderr}");

    // Should fail
    assert!(
        !success,
        "SEARCH with only query should fail.\nOutput: {output}"
    );

    // Error should mention index-dir
    assert!(
        output.contains("index-dir")
            || output.contains("index_dir")
            || output.contains("required")
            || output.contains("DIR"),
        "Error should mention index-dir or required.\nOutput: {output}"
    );
}

#[test]
fn test_empty_command_shows_help() {
    // No arguments should show help or error
    let (success, stdout, stderr) = run_command(&[]);

    let output = format!("{stdout}\n{stderr}");

    // Should fail (no subcommand provided)
    assert!(!success, "Empty command should fail.\nOutput: {output}");

    // Should show help or mention subcommand is required
    assert!(
        output.contains("Usage:") || output.contains("required") || output.contains("subcommand"),
        "Should show usage or mention required subcommand.\nOutput: {output}"
    );
}

#[test]
fn test_help_command_succeeds() {
    // Help command should always succeed
    let (success, stdout, stderr) = run_command(&["--help"]);

    let output = format!("{stdout}\n{stderr}");

    // Should succeed
    assert!(success, "Help command should succeed.\nOutput: {output}");

    // Should contain help text
    assert!(
        output.contains("USAGE") || output.contains("Usage") || output.contains("help"),
        "Should contain help information.\nOutput: {output}"
    );
}

#[test]
fn test_index_help_shows_output_required() {
    // INDEX help should show --output is required
    let (success, stdout, stderr) = run_command(&["index", "--help"]);

    let output = format!("{stdout}\n{stderr}");

    // Should succeed
    assert!(success, "INDEX help should succeed.\nOutput: {output}");

    // Help should mention output
    assert!(
        output.contains("output") || output.contains("OUTPUT") || output.contains("DIR"),
        "Help should mention output argument.\nOutput: {output}"
    );
}

#[test]
fn test_scrape_help_shows_output_required() {
    // SCRAPE help should show --output is required
    let (success, stdout, stderr) = run_command(&["scrape", "--help"]);

    let output = format!("{stdout}\n{stderr}");

    // Should succeed
    assert!(success, "SCRAPE help should succeed.\nOutput: {output}");

    // Help should mention output
    assert!(
        output.contains("output") || output.contains("OUTPUT") || output.contains("DIR"),
        "Help should mention output argument.\nOutput: {output}"
    );
}

#[test]
fn test_search_help_shows_index_dir_required() {
    // SEARCH help should show --index-dir is required
    let (success, stdout, stderr) = run_command(&["search", "--help"]);

    let output = format!("{stdout}\n{stderr}");

    // Should succeed
    assert!(success, "SEARCH help should succeed.\nOutput: {output}");

    // Help should mention index-dir
    assert!(
        output.contains("index-dir") || output.contains("index_dir") || output.contains("DIR"),
        "Help should mention index-dir argument.\nOutput: {output}"
    );
}

// ============================================================================
// INGEST COMMAND TESTS (--output required)
// ============================================================================

#[test]
fn test_ingest_command_missing_output_fails() {
    // INGEST command without --output should fail
    let (success, stdout, stderr) = run_command(&["ingest", "https://example.com"]);

    let output = format!("{stdout}\n{stderr}");

    // Should fail
    assert!(
        !success,
        "INGEST command without --output should fail.\nOutput: {output}"
    );

    // Error message should mention the missing required argument
    assert!(
        output.contains("output")
            || output.contains("required")
            || output.contains("OUTPUT")
            || output.contains("DIR"),
        "Error message should mention 'output', 'required', 'OUTPUT', or 'DIR'.\nOutput: {output}"
    );
}

#[test]
fn test_ingest_command_with_output_succeeds_parsing() {
    // INGEST command with --output should succeed at parsing level
    let (_success, stdout, stderr) = run_command(&[
        "ingest",
        "https://example.com",
        "--output",
        "/tmp/test-output",
    ]);

    let output = format!("{stdout}\n{stderr}");

    // Clap should accept the arguments
    assert!(
        !output.contains("required") || !output.contains("output"),
        "When --output is provided, should not complain about missing required 'output'.\nOutput: {output}"
    );
}

// ============================================================================
// INGEST GIT COMMAND TESTS (--output required)
// ============================================================================

#[test]
fn test_ingest_git_command_missing_output_fails() {
    // INGEST-GIT command without --output should fail
    let (success, stdout, stderr) =
        run_command(&["ingest-git", "https://github.com/example/repo.git"]);

    let output = format!("{stdout}\n{stderr}");

    // Should fail
    assert!(
        !success,
        "INGEST-GIT command without --output should fail.\nOutput: {output}"
    );

    // Error message should mention the missing required argument
    assert!(
        output.contains("output")
            || output.contains("required")
            || output.contains("OUTPUT")
            || output.contains("DIR"),
        "Error message should mention 'output', 'required', 'OUTPUT', or 'DIR'.\nOutput: {output}"
    );
}

#[test]
fn test_ingest_git_command_with_output_succeeds_parsing() {
    // INGEST-GIT command with --output should succeed at parsing level
    let (_success, stdout, stderr) = run_command(&[
        "ingest-git",
        "https://github.com/example/repo.git",
        "--output",
        "/tmp/test-output",
    ]);

    let output = format!("{stdout}\n{stderr}");

    // Clap should accept the arguments
    assert!(
        !output.contains("required") || !output.contains("output"),
        "When --output is provided, should not complain about missing required 'output'.\nOutput: {output}"
    );
}
