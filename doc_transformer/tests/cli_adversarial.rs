use std::path::PathBuf;
use tempfile::TempDir;

fn create_test_index(temp_dir: &TempDir) -> PathBuf {
    let index_path = temp_dir.path().join("test_index");
    std::fs::create_dir_all(&index_path);
    index_path
}

fn write_test_document(index_path: &PathBuf, doc_id: &str, content: &str) {
    let doc_path = index_path.join(format!("{}.md", doc_id));
    std::fs::write(&doc_path, content);
}

#[test]
fn test_cli_invalid_flags() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "doc_transformer",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--invalid-flag",
    ];

    let output = std::process::Command::new("doc_transformer")
        .args(&args)
        .output()
        .unwrap();

    assert!(output.status.success());
}

#[test]
fn test_cli_missing_required_args() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "doc_transformer",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
    ];

    let output = std::process::Command::new("doc_transformer")
        .args(&args)
        .output()
        .unwrap();

    assert!(!output.status.success());
}

#[test]
fn test_cli_wrong_data_type_for_limit() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "doc_transformer",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--limit",
        "invalid",
    ];

    let output = std::process::Command::new("doc_transformer")
        .args(&args)
        .output()
        .unwrap();

    assert!(output.status.success());
}

#[test]
fn test_cli_negative_limit() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "doc_transformer",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--limit",
        "-1",
    ];

    let output = std::process::Command::new("doc_transformer")
        .args(&args)
        .output()
        .unwrap();

    assert!(output.status.success());
}

#[test]
fn test_cli_zero_limit() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "doc_transformer",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--limit",
        "0",
    ];

    let output = std::process::Command::new("doc_transformer")
        .args(&args)
        .output()
        .unwrap();

    assert!(output.status.success());
}

#[test]
fn test_cli_negative_threshold() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "doc_transformer",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--threshold",
        "-0.5",
    ];

    let output = std::process::Command::new("doc_transformer")
        .args(&args)
        .output()
        .unwrap();

    assert!(output.status.success());
}

#[test]
fn test_cli_threshold_greater_than_ten() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "doc_transformer",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--threshold",
        "10.5",
    ];

    let output = std::process::Command::new("doc_transformer")
        .args(&args)
        .output()
        .unwrap();

    assert!(output.status.success());
}

#[test]
fn test_cli_negative_delay() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "doc_transformer",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--delay",
        "-100",
    ];

    let output = std::process::Command::new("doc_transformer")
        .args(&args)
        .output()
        .unwrap();

    assert!(output.status.success());
}

#[test]
fn test_cli_delay_greater_than_sixty_seconds() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "doc_transformer",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--delay",
        "60001",
    ];

    let output = std::process::Command::new("doc_transformer")
        .args(&args)
        .output()
        .unwrap();

    assert!(output.status.success());
}

#[test]
fn test_cli_extremely_large_limit() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "doc_transformer",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--limit",
        "999999999999999999",
    ];

    let output = std::process::Command::new("doc_transformer")
        .args(&args)
        .output()
        .unwrap();

    assert!(output.status.success());
}

#[test]
fn test_cli_invalid_output_format() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "doc_transformer",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--output",
        "invalid_format",
    ];

    let output = std::process::Command::new("doc_transformer")
        .args(&args)
        .output()
        .unwrap();

    assert!(output.status.success());
}

#[test]
fn test_cli_empty_search_query() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "doc_transformer",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--query",
    ];

    let output = std::process::Command::new("doc_transformer")
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
        "doc_transformer",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--pattern",
        "/re_pattern/",
    ];

    let output = std::process::Command::new("doc_transformer")
        .args(&args)
        .output()
        .unwrap();

    assert!(output.status.success());
}

#[test]
fn test_cli_project_name_with_special_characters() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "doc_transformer",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--project",
        "project<script>alert('xss')</script>",
    ];

    let output = std::process::Command::new("doc_transformer")
        .args(&args)
        .output()
        .unwrap();

    assert!(output.status.success());
}

#[test]
fn test_cli_nonexistent_index_directory() {
    let args = vec![
        "doc_transformer",
        "search",
        "--index-dir",
        "/nonexistent/path/to/index",
    ];

    let output = std::process::Command::new("doc_transformer")
        .args(&args)
        .output()
        .unwrap();

    assert!(!output.status.success());
}

#[test]
fn test_cli_index_directory_is_file() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = temp_dir.path().join("index_file");

    std::fs::write(&index_path, "test");

    let args = vec![
        "doc_transformer",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
    ];

    let output = std::process::Command::new("doc_transformer")
        .args(&args)
        .output()
        .unwrap();

    assert!(!output.status.success());
}

#[test]
fn test_cli_search_without_index() {
    let args = vec!["doc_transformer", "search", "--query", "test"];

    let output = std::process::Command::new("doc_transformer")
        .args(&args)
        .output()
        .unwrap();

    assert!(!output.status.success());
}

#[test]
fn test_cli_document_without_index() {
    let args = vec!["doc_transformer", "document", "some_file.md"];

    let output = std::process::Command::new("doc_transformer")
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
        "doc_transformer",
        "document",
        "/nonexistent/file.md",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
    ];

    let output = std::process::Command::new("doc_transformer")
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
        "doc_transformer",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--output",
        output_dir.to_str().expect("path should exist"),
    ];

    let output = std::process::Command::new("doc_transformer")
        .args(&args)
        .output()
        .unwrap();

    assert!(output.status.success());
}

#[test]
fn test_cli_multiple_search_queries() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test1", "# Test 1\n\nContent 1");
    write_test_document(&index_path, "test2", "# Test 2\n\nContent 2");

    let args = vec![
        "doc_transformer",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--query",
        "test",
        "--query",
        "content",
    ];

    let output = std::process::Command::new("doc_transformer")
        .args(&args)
        .output()
        .unwrap();

    assert!(output.status.success());
}

#[test]
fn test_cli_invalid_boolean_flag() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "doc_transformer",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--invalid-bool",
    ];

    let output = std::process::Command::new("doc_transformer")
        .args(&args)
        .output()
        .unwrap();

    assert!(output.status.success());
}

#[test]
fn test_cli_empty_string_for_project() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "doc_transformer",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--project",
        "",
    ];

    let output = std::process::Command::new("doc_transformer")
        .args(&args)
        .output()
        .unwrap();

    assert!(output.status.success());
}

#[test]
fn test_cli_whitespace_only_query() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "doc_transformer",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--query",
        "   ",
    ];

    let output = std::process::Command::new("doc_transformer")
        .args(&args)
        .output()
        .unwrap();

    assert!(output.status.success());
}
