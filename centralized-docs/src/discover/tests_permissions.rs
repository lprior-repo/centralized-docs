use super::*;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use tempfile::TempDir;

/// Test that an unreadable directory causes discovery to FAIL with exit code 1.
#[test]
fn test_unreadable_directory_returns_nonzero_exit() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    // Create a subdirectory with a file inside - but NO readable files at root
    let unreadable_dir = dir_path.join("restricted");
    fs::create_dir(&unreadable_dir).unwrap();

    let file_in_dir = unreadable_dir.join("inside.md");
    fs::write(&file_in_dir, "# Inside Restricted\nContent").unwrap();

    // Remove read+execute permissions from subdirectory
    fs::set_permissions(&unreadable_dir, PermissionsExt::from_mode(0o000)).unwrap();

    // Discover files - should FAIL because no readable files exist
    let result = discover_files(dir_path, None);

    // Clean up: restore permissions so temp dir can be removed
    let _ = fs::set_permissions(&unreadable_dir, PermissionsExt::from_mode(0o755));

    assert!(
        result.is_err(),
        "discover_files should FAIL when no readable files exist due to permission errors"
    );

    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("permission denied"),
        "Error should mention 'permission denied', got: {err_msg}"
    );
}

/// Test that readable files work normally (happy path)
#[test]
fn test_discover_files_with_readable_files() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    let file1 = dir_path.join("readable1.md");
    let file2 = dir_path.join("readable2.md");

    fs::write(&file1, "# Readable Document 1\nContent here").unwrap();
    fs::write(&file2, "# Readable Document 2\nMore content").unwrap();

    let result = discover_files(dir_path, None);

    assert!(
        result.is_ok(),
        "discover_files should succeed with readable files"
    );

    let (discovered_files, _manifest) = result.unwrap();

    assert_eq!(
        discovered_files.len(),
        2,
        "Expected 2 readable files to be discovered"
    );

    let file_names: Vec<_> = discovered_files
        .iter()
        .map(|f| f.source_path.clone())
        .collect();

    assert!(file_names.iter().any(|n| n.contains("readable1.md")));
    assert!(file_names.iter().any(|n| n.contains("readable2.md")));
}

/// Test that excluded directories are skipped
#[test]
fn test_discover_files_excludes_directories() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    // Create directories that should be excluded
    let node_modules = dir_path.join("node_modules");
    let git_dir = dir_path.join(".git");
    let _build = dir_path.join("_build");
    let dist_dir = dir_path.join("dist");
    let vendor_dir = dir_path.join("vendor");

    fs::create_dir(&node_modules).unwrap();
    fs::create_dir(&git_dir).unwrap();
    fs::create_dir(&_build).unwrap();
    fs::create_dir(&dist_dir).unwrap();
    fs::create_dir(&vendor_dir).unwrap();

    // Create files inside excluded directories
    let nm_file = node_modules.join("package.md");
    let git_file = git_dir.join("config.md");
    let build_file = _build.join("output.md");
    let dist_file = dist_dir.join("bundle.md");
    let vendor_file = vendor_dir.join("lib.md");

    fs::write(&nm_file, "").unwrap();
    fs::write(&git_file, "").unwrap();
    fs::write(&build_file, "").unwrap();
    fs::write(&dist_file, "").unwrap();
    fs::write(&vendor_file, "").unwrap();

    // Create a file in root that should be found
    let root_file = dir_path.join("root.md");
    fs::write(&root_file, "# Root\n").unwrap();

    let result = discover_files(dir_path, None);
    assert!(result.is_ok());

    let (files, _manifest) = result.unwrap();
    assert_eq!(
        files.len(),
        1,
        "Should only find root file, not files in excluded directories"
    );
    assert!(files[0].source_path.contains("root.md"));
}
