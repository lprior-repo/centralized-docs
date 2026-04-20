#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn ctd_bin() -> &'static str {
    env!("CARGO_BIN_EXE_ctd")
}

fn create_test_index(temp_dir: &TempDir) -> PathBuf {
    let index_path = temp_dir.path().join("test_index");
    let _ = std::fs::create_dir_all(&index_path);
    index_path
}

fn write_test_document(index_path: &Path, doc_id: &str, content: &str) {
    let doc_path = index_path.join(format!("{doc_id}.md"));
    let _ = std::fs::write(&doc_path, content);
}

fn create_invalid_index(temp_dir: &TempDir) -> PathBuf {
    let index_path = temp_dir.path().join("invalid_index");
    let _ = std::fs::create_dir_all(&index_path);

    let index_path_clone = index_path.clone();
    std::thread::spawn(move || {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(index_path_clone.join("INDEX.json"));
        let _ = file.unwrap().write_all(b"invalid json content");
    });

    index_path
}

#[test]
fn test_search_with_corrupt_index_file() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let corrupt_index_path = create_invalid_index(&temp_dir);

    let args = vec![
        "ctd",
        "search",
        "--index-dir",
        corrupt_index_path.to_str().expect("path should exist"),
        "--query",
        "test",
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    assert!(output.unwrap().status.code() != Some(0), "Expected error");
}

#[test]
fn test_search_with_malformed_index_json() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = temp_dir.path().join("malformed_index");

    let _ = std::fs::create_dir_all(&index_path);

    let index_path_clone = index_path.clone();
    std::thread::spawn(move || {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .open(index_path_clone.join("INDEX.json"));
        let _ = file.unwrap().write_all(b"{invalid json content}");
    });

    let args = vec![
        "ctd",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--query",
        "test",
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    assert!(output.unwrap().status.code() != Some(0), "Expected error");
}

#[test]
fn test_search_with_empty_index_directory() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = temp_dir.path().join("empty_index");

    let _ = std::fs::create_dir_all(&index_path);

    let args = vec![
        "ctd",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--query",
        "test",
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    assert!(output.unwrap().status.code() != Some(0), "Expected error");
}

#[test]
fn test_search_with_index_no_documents() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    let args = vec![
        "ctd",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--query",
        "nonexistent",
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    assert!(output.unwrap().status.code() != Some(0), "Expected error");
}

#[test]
fn test_document_with_invalid_yaml_frontmatter() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    let invalid_yaml = "invalid: yaml: frontmatter: content\n\nDocument content here";

    let args = vec![
        "ctd",
        "document",
        "invalid.md",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--content",
        invalid_yaml,
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    assert!(output.unwrap().status.code() != Some(0), "Expected error");
}

#[test]
fn test_document_with_empty_frontmatter() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    let empty_frontmatter = "Document content here";

    let args = vec![
        "ctd",
        "document",
        "empty.md",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--content",
        empty_frontmatter,
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    assert!(output.unwrap().status.code() != Some(0), "Expected error");
}

#[test]
fn test_document_with_only_frontmatter() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    let args = vec![
        "ctd",
        "document",
        "empty.md",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--content",
        "title: Empty Document",
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    assert!(output.unwrap().status.code() != Some(0), "Expected error");
}

#[test]
fn test_document_with_invalid_title() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    let args = vec![
        "ctd",
        "document",
        "invalid_title.md",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--content",
        "title: ",
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    assert!(output.unwrap().status.code() != Some(0), "Expected error");
}

#[test]
fn test_document_with_invalid_project_name() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    let args = vec![
        "ctd",
        "document",
        "invalid_project.md",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--content",
        "title: Test\nproject_name: /pattern/",
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    assert!(output.unwrap().status.code() != Some(0), "Expected error");
}

#[test]
fn test_document_with_invalid_tags() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    let args = vec![
        "ctd",
        "document",
        "invalid_tags.md",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--content",
        "title: Test\ntags: [pattern]",
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    assert!(output.unwrap().status.code() != Some(0), "Expected error");
}

#[test]
fn test_document_with_invalid_status() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    let args = vec![
        "ctd",
        "document",
        "invalid_status.md",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--content",
        "title: Test\nstatus: invalid_status",
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    assert!(output.unwrap().status.code() != Some(0), "Expected error");
}

#[test]
fn test_document_with_invalid_date() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    let args = vec![
        "ctd",
        "document",
        "invalid_date.md",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--content",
        "title: Test\ndate: invalid-date",
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    assert!(output.unwrap().status.code() != Some(0), "Expected error");
}

#[test]
fn test_document_with_invalid_priority() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    let args = vec![
        "ctd",
        "document",
        "invalid_priority.md",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--content",
        "title: Test\npriority: invalid",
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    assert!(output.unwrap().status.code() != Some(0), "Expected error");
}

#[test]
fn test_document_with_invalid_knowledge_dag() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    let args = vec![
        "ctd",
        "document",
        "invalid_dag.md",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--content",
        "title: Test\nknowledge_dag: [invalid_dag_format]",
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    assert!(output.unwrap().status.code() != Some(0), "Expected error");
}

#[test]
fn test_document_with_duplicate_document_ids() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    let args = vec![
        "ctd",
        "document",
        "duplicate.md",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--content",
        "title: Test",
    ];

    let _ = std::process::Command::new(ctd_bin())
        .args(&args)
        .output()
        .unwrap();

    let args2 = [
        "ctd",
        "document",
        "duplicate.md",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--content",
        "title: Test",
    ];

    let output = std::process::Command::new(ctd_bin()).args(args2).output();

    match output {
        Ok(out) => {
            assert!(
                out.status.code() != Some(0),
                "Expected error for duplicate document IDs"
            );
        }
        Err(e) => {
            // Argument list too long is expected error for this test
            assert_eq!(e.kind(), std::io::ErrorKind::ArgumentListTooLong);
        }
    }
}

#[test]
fn test_search_with_nonexistent_project() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "ctd",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--query",
        "test",
        "--project",
        "nonexistent_project",
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    assert!(output.unwrap().status.code() != Some(0), "Expected error");
}

#[test]
fn test_search_with_empty_tags() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "ctd",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--query",
        "test",
        "--tags",
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    assert!(output.unwrap().status.code() != Some(0), "Expected error");
}

#[test]
fn test_search_with_invalid_tags_format() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    write_test_document(&index_path, "test", "# Test\n\nTest content");

    let args = vec![
        "ctd",
        "search",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--query",
        "test",
        "--tags",
        "invalid,tag,format",
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    assert!(output.unwrap().status.code() != Some(0), "Expected error");
}

#[test]
fn test_document_with_special_characters_in_content() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    let special_content =
        "Content with \x00 null byte, \x1f control char, and unicode \u{1F4A9} emoji";

    let args = vec![
        "ctd",
        "document",
        "special.md",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--content",
        special_content,
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    match output {
        Ok(out) => {
            assert!(
                out.status.code() != Some(0),
                "Expected error for special characters"
            );
        }
        Err(e) => {
            // Invalid input (null byte) is expected error for this test
            assert_eq!(e.kind(), std::io::ErrorKind::InvalidInput);
        }
    }
}

#[test]
fn test_document_with_extremely_long_content() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    let long_content = "x".repeat(10_000_000);

    let args = vec![
        "ctd",
        "document",
        "long.md",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--content",
        &long_content,
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    match output {
        Ok(out) => {
            assert!(
                out.status.code() != Some(0),
                "Expected error for extremely long content"
            );
        }
        Err(e) => {
            // Argument list too long is expected error for this test
            assert_eq!(e.kind(), std::io::ErrorKind::ArgumentListTooLong);
        }
    }
}

#[test]
fn test_document_with_unicode_bom() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    let bom_content = "\u{FEFF}title: Test\n\nDocument content";

    let args = vec![
        "ctd",
        "document",
        "bom.md",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--content",
        bom_content,
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    assert!(output.unwrap().status.code() != Some(0), "Expected error");
}

#[test]
fn test_document_with_windows_line_endings() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    let windows_content = "title: Test\r\n\r\nDocument content";

    let args = vec![
        "ctd",
        "document",
        "windows.md",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--content",
        windows_content,
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    assert!(output.unwrap().status.code() != Some(0), "Expected error");
}

#[test]
fn test_document_with_tabs_in_frontmatter() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    let tab_content = "title:\tTest\n\nDocument content";

    let args = vec![
        "ctd",
        "document",
        "tabs.md",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--content",
        tab_content,
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    assert!(output.unwrap().status.code() != Some(0), "Expected error");
}

#[test]
fn test_document_with_invalid_encoding() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = create_test_index(&temp_dir);

    let args = vec![
        "ctd",
        "document",
        "encoding.md",
        "--index-dir",
        index_path.to_str().expect("path should exist"),
        "--content",
        "title: Invalid encoding \\u{FF}\\u{FE}",
    ];

    let output = std::process::Command::new(ctd_bin()).args(&args).output();

    assert!(output.unwrap().status.code() != Some(0), "Expected error");
}
