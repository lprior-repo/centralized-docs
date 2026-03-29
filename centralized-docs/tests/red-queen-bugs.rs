#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use std::fs::{self, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use tempfile::TempDir;

fn ctd_bin() -> &'static str {
    env!("CARGO_BIN_EXE_ctd")
}

#[test]
fn test_permission_denied_returns_nonzero() {
    let temp_dir = std::env::temp_dir().join("ctd_test_protected");
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");

    let protected_dir = temp_dir.join("protected");
    fs::create_dir_all(&protected_dir).expect("Failed to create protected dir");

    let output_dir = protected_dir.join("output");
    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    let test_file = output_dir.join("test.md");
    fs::write(&test_file, "# Test").expect("Failed to write test file");

    let mut perms = fs::metadata(&output_dir).unwrap().permissions();
    perms.set_mode(0o000);
    fs::set_permissions(&output_dir, perms).expect("Failed to set permissions");

    let result = Command::new(ctd_bin())
        .arg("index")
        .arg(temp_dir.join("test-input"))
        .arg("--output")
        .arg(&output_dir)
        .output();

    let exit_code = result.expect("Command failed").status.code().unwrap_or(0);
    assert_ne!(
        exit_code, 0,
        "Permission denied should return non-zero exit code"
    );

    fs::set_permissions(&output_dir, Permissions::from_mode(0o755))
        .expect("Failed to restore permissions");

    fs::remove_dir_all(temp_dir).expect("Failed to clean up");
}

#[test]
fn test_corrupted_index_returns_error() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let root = temp_dir.path();

    let input_dir = root.join("input");
    fs::create_dir_all(&input_dir).expect("Failed to create input dir");

    let test_file = input_dir.join("test.md");
    fs::write(&test_file, "# Test\n\nThis is a test document.").expect("Failed to write test file");

    let index_dir = root.join("corrupt-index");
    fs::create_dir_all(&index_dir).expect("Failed to create index dir");

    let corrupted_file = index_dir.join("INDEX.json");
    fs::write(&corrupted_file, "{invalid json").expect("Failed to write corrupted file");

    let result = Command::new(ctd_bin())
        .arg("search")
        .arg("test")
        .arg("--index-dir")
        .arg(&index_dir)
        .output();

    let output = result.expect("Command failed");
    let stderr = String::from_utf8_lossy(&output.stderr);
    let _stdout = String::from_utf8_lossy(&output.stdout);

    let exit_code = output.status.code().unwrap_or(0);
    assert_ne!(
        exit_code, 0,
        "Corrupted INDEX.json should return non-zero exit code"
    );
    let stderr_lower = stderr.to_lowercase();
    assert!(
        stderr_lower.contains("invalid")
            || stderr_lower.contains("parse")
            || stderr_lower.contains("error"),
        "Should report JSON parse error, got: {stderr}",
    );
}

#[test]
fn test_empty_parent_path_shows_full_path() {
    let temp_dir = std::env::temp_dir().join("ctd_test_parent");

    let input_dir = temp_dir.join("input");
    fs::create_dir_all(&input_dir).expect("Failed to create input dir");

    let test_file = input_dir.join("test.md");
    fs::write(&test_file, "# Test\n\nThis is a test document.").expect("Failed to write test file");

    let nonexistent_output = temp_dir.join("nonexistent").join("output");

    let result = Command::new(ctd_bin())
        .arg("index")
        .arg(&input_dir)
        .arg("--output")
        .arg(&nonexistent_output)
        .output();

    let output = result.expect("Command failed");
    let stderr = String::from_utf8_lossy(&output.stderr);

    let exit_code = output.status.code().unwrap_or(0);
    assert_ne!(
        exit_code, 0,
        "Nonexistent parent directory should return non-zero exit code"
    );
    assert!(
        stderr.contains("nonexistent"),
        "Error message should show full parent path 'nonexistent', got: {stderr}",
    );

    fs::remove_dir_all(temp_dir).expect("Failed to clean up");
}
