use super::*;
use std::fs;
use tempfile::TempDir;

/// Test that broken symlinks cause discovery to FAIL with non-zero exit code
#[test]
fn test_discover_files_fails_on_broken_symlinks() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    // Create a valid markdown file
    let valid_file = dir_path.join("valid.md");
    fs::write(&valid_file, "# Valid Document\nContent here").unwrap();

    // Create a broken symlink (points to non-existent file)
    let broken_link = dir_path.join("broken-link.md");
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        symlink("/nonexistent/target/file.md", &broken_link).unwrap();
    }

    // Discover files - should FAIL because broken symlinks cause non-zero exit
    let result = discover_files(dir_path, None);
    assert!(
        result.is_err(),
        "discover_files should FAIL when broken symlinks are found"
    );

    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("broken symlink"),
        "Error should mention 'broken symlink', got: {err_msg}"
    );
}

/// Test that valid symlinks (pointing to real files) are processed correctly
#[test]
fn test_discover_files_with_valid_symlink() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    let real_file = dir_path.join("real.md");
    fs::write(&real_file, "# Real Document\nContent here").unwrap();

    let valid_link = dir_path.join("link.md");
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        symlink(&real_file, &valid_link).unwrap();
    }

    let result = discover_files(dir_path, None);
    assert!(
        result.is_ok(),
        "discover_files should succeed with valid symlinks"
    );

    let (discovered_files, _manifest) = result.unwrap();

    assert!(
        !discovered_files.is_empty(),
        "Should discover at least 1 file"
    );

    let file_names: Vec<_> = discovered_files
        .iter()
        .map(|f| f.source_path.clone())
        .collect();
    assert!(
        file_names.iter().any(|n| n.contains("real.md")),
        "Should find real.md, found: {file_names:?}"
    );
}

/// Test that symlink pointing to directory is processed correctly
#[test]
fn test_discover_files_with_symlink_to_directory() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    let real_dir = dir_path.join("realdir");
    let real_file_in_dir = real_dir.join("nested.md");
    fs::create_dir(&real_dir).unwrap();
    fs::write(&real_file_in_dir, "# Nested\nContent").unwrap();

    let dir_link = dir_path.join("linkdir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        symlink(&real_dir, &dir_link).unwrap();
    }

    let result = discover_files(dir_path, None);
    assert!(
        result.is_ok(),
        "discover_files should succeed with symlink to directory"
    );

    let (discovered_files, _manifest) = result.unwrap();

    let file_names: Vec<_> = discovered_files
        .iter()
        .map(|f| f.source_path.clone())
        .collect();
    assert!(
        file_names.iter().any(|n| n.contains("nested.md")),
        "Should find nested.md inside symlinked dir, found: {file_names:?}"
    );
}

/// Test that multiple broken symlinks are counted correctly in error
#[test]
fn test_discover_files_multiple_broken_symlinks() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    fs::write(dir_path.join("good.md"), "# Good\nContent").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let _ = symlink("/nonexistent1", dir_path.join("broken1.md"));
        let _ = symlink("/nonexistent2", dir_path.join("broken2.md"));
        let _ = symlink("/nonexistent3", dir_path.join("broken3.md"));
    }

    let result = discover_files(dir_path, None);
    assert!(result.is_err(), "Should fail with multiple broken symlinks");

    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("3 broken symlink"),
        "Error should mention 3 broken symlinks, got: {err_msg}"
    );
}

/// Test that self-referential symlink (circular) is treated as broken
#[test]
fn test_discover_files_circular_symlink() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    fs::write(dir_path.join("file.md"), "# Content\nText").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let link_path = dir_path.join("self.md");
        symlink(&link_path, &link_path).unwrap();
    }

    let result = discover_files(dir_path, None);
    assert!(
        result.is_err(),
        "Self-referential (circular) symlink should be treated as broken"
    );

    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("broken symlink"),
        "Error should mention broken symlink, got: {err_msg}"
    );
}
