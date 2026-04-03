#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use std::path::{Path, PathBuf};
use tempfile::TempDir;

const DOC_TRANSFORMER_BIN: &str = env!("CARGO_BIN_EXE_ctd");

fn create_test_index(temp_dir: &TempDir) -> PathBuf {
    let index_path = temp_dir.path().join("test_index");
    let _ = std::fs::create_dir_all(&index_path);
    index_path
}

fn write_test_document(index_path: &Path, doc_id: &str, content: &str) {
    let doc_path = index_path.join(format!("{doc_id}.md"));
    let _ = std::fs::write(&doc_path, content);
}

#[allow(dead_code)]
fn run_cli_test<F>(test_fn: F) -> std::io::Result<()>
where
    F: FnOnce() -> std::io::Result<()>,
{
    let temp_dir = TempDir::new()?;
    let index_path = create_test_index(&temp_dir);
    write_test_document(&index_path, "test", "# Test\n\nTest content");
    test_fn()
}

#[test]
fn test_cli_invalid_flags() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--invalid-flag",
        "test",
    ];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .unwrap();

    assert!(!output.status.success(), "CLI should reject invalid flag");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unexpected") || stderr.contains("argument") || stderr.contains("flag"),
        "Error message should mention unexpected argument"
    );
}

#[test]
fn test_cli_missing_required_args() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "test",
    ];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .expect("failed to execute command");

    assert!(!output.status.success());
}

#[test]
fn test_cli_wrong_data_type_for_limit() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--limit",
        "invalid",
        "test",
    ];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .expect("failed to execute command");

    assert!(!output.status.success());
}

#[test]
fn test_cli_negative_limit() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--limit",
        "-1",
    ];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .unwrap();

    assert!(!output.status.success(), "CLI should reject negative limit");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("positive"),
        "Error message should mention 'positive'"
    );
}

#[test]
fn test_cli_zero_limit() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--limit",
        "0",
        "test",
    ];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .unwrap();

    assert!(!output.status.success(), "CLI should reject zero limit");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("at least 1")
            || stderr.contains("positive")
            || stderr.contains("or higher"),
        "Error message should mention limit must be positive or at least 1"
    );
}

#[test]
fn test_cli_negative_threshold() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "scrape",
        "https://example.com",
        "--output",
        index_path.to_str().expect("path should exist"),
        "--threshold",
        "-0.5",
    ];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "CLI should reject negative threshold"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("threshold") || stderr.contains("range"),
        "Error message should mention threshold range"
    );
}

#[test]
fn test_cli_threshold_greater_than_ten() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "scrape",
        "https://example.com",
        "--output",
        index_path.to_str().expect("path should exist"),
        "--threshold",
        "10.5",
    ];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .unwrap();

    assert!(!output.status.success(), "CLI should reject threshold > 10");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("threshold") || stderr.contains("range"),
        "Error message should mention threshold range"
    );
}

#[test]
fn test_cli_negative_delay() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "scrape",
        "https://example.com",
        "--output",
        index_path.to_str().expect("path should exist"),
        "--delay",
        "-100",
    ];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .unwrap();

    assert!(!output.status.success(), "CLI should reject negative delay");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("delay")
            || stderr.contains("negative")
            || stderr.contains("invalid")
            || stderr.contains("Scraping is not available"),
        "Error message should mention delay issue or scraping unavailable: {stderr}"
    );
}

#[test]
fn test_cli_delay_greater_than_sixty_seconds() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "scrape",
        "https://example.com",
        "--output",
        index_path.to_str().expect("path should exist"),
        "--delay",
        "60001",
    ];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .unwrap();

    assert!(!output.status.success(), "CLI should reject delay > 60000");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("delay") || stderr.contains("range"),
        "Error message should mention delay range"
    );
}

#[test]
fn test_cli_extremely_large_limit() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--limit",
        "999999999999999999",
    ];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "CLI should reject extremely large limit"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("limit") || stderr.contains("1000") || stderr.contains("too large"),
        "Error message should mention limit limit"
    );
}

#[test]
fn test_cli_invalid_output_format() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "index",
        "some_file.md",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--output",
        "invalid_format",
    ];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "CLI should reject invalid output path"
    );
}

#[test]
fn test_cli_empty_search_query() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--query",
    ];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .unwrap();

    assert!(!output.status.success());
}

#[test]
fn test_cli_pattern_rejection() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "scrape",
        "https://example.com",
        "--output",
        index_path.to_str().expect("path should exist"),
        "--filter",
        "[unclosed",
    ];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "CLI should reject invalid regex pattern"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("regex") || stderr.contains("pattern"),
        "Error message should mention regex/pattern"
    );
}

#[test]
fn test_cli_project_name_with_special_characters() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "scrape",
        "https://example.com",
        "--output",
        index_path.to_str().expect("path should exist"),
        "--project-name",
        "project<script>alert('xss')</script>",
    ];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "CLI should reject project name with HTML/JS"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("project") || stderr.contains("name"),
        "Error message should mention project name"
    );
}

#[test]
fn test_cli_nonexistent_index_directory() {
    let args = vec!["search", "--index-dir", "/nonexistent/path/to/index"];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .unwrap();

    assert!(!output.status.success());
}

#[test]
fn test_cli_index_directory_is_file() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = temp_dir.path().join("index_file");

    let _ = std::fs::write(&index_path, "test");

    let args = vec![
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "test",
    ];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .expect("failed to execute command");

    assert!(!output.status.success());
}

#[test]
fn test_cli_search_without_index() {
    let args = vec!["ctd", "search", "--query", "test"];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .unwrap();

    assert!(!output.status.success());
}

#[test]
fn test_cli_document_without_index() {
    let args = vec!["ctd", "document", "some_file.md"];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .unwrap();

    assert!(!output.status.success());
}

#[test]
fn test_cli_document_invalid_file() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "document",
        "/nonexistent/file.md",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
    ];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .unwrap();

    assert!(!output.status.success());
}

#[test]
fn test_cli_output_to_nonexistent_directory() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let output_dir = temp_dir.path().join("nonexistent_output");
    let args = vec![
        "index",
        "some_nonexistent_dir",
        "--output",
        output_dir.to_str().expect("path should exist"),
    ];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "CLI should reject nonexistent source directory"
    );
}

#[test]
fn test_cli_multiple_search_queries() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test1", "# Test 1\n\nContent 1");
    write_test_document(&index_path, "test2", "# Test 2\n\nContent 2");

    let args = vec![
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--query",
        "test",
        "--query",
        "content",
    ];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "CLI should reject multiple --query arguments"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unexpected") || stderr.contains("argument") || stderr.contains("query"),
        "Error message should mention unexpected argument"
    );
}

#[test]
fn test_cli_invalid_boolean_flag() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--invalid-bool",
    ];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .unwrap();

    assert!(!output.status.success(), "CLI should reject invalid flag");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unexpected") || stderr.contains("argument") || stderr.contains("flag"),
        "Error message should mention unexpected argument"
    );
}

#[test]
fn test_cli_empty_string_for_project() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "scrape",
        "https://example.com",
        "--output",
        index_path.to_str().expect("path should exist"),
        "--project-name",
        "",
    ];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "CLI should reject empty project name"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("project") || stderr.contains("name") || stderr.contains("empty"),
        "Error message should mention project name"
    );
}

#[test]
fn test_cli_whitespace_only_query() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--query",
        "   ",
    ];

    let output = std::process::Command::new(DOC_TRANSFORMER_BIN)
        .args(&args)
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "CLI should reject empty/whitespace-only query"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("empty") || stderr.contains("query") || stderr.contains("trim"),
        "Error message should mention empty query"
    );
}
