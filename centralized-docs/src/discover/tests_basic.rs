#[allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]
use super::*;
use std::fs;
use tempfile::TempDir;

/// Test basic file discovery functionality
#[test]
fn test_discover_files_basic() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    let md_file = dir_path.join("test.md");
    let txt_file = dir_path.join("test.txt");
    let rst_file = dir_path.join("test.rst");
    let mdx_file_test = dir_path.join("test.mdx");
    let other_file = dir_path.join("test.html");

    fs::write(&md_file, "# Markdown\n\ncontent\n").unwrap();
    fs::write(&txt_file, "plain text").unwrap();
    fs::write(&rst_file, "Heading\n=======\n").unwrap();
    fs::write(&mdx_file_test, "# MDX\n\n<Component />").unwrap();
    fs::write(&other_file, "").unwrap();

    let result = discover_files(dir_path, None);
    assert!(result.is_ok());

    let (files, _manifest) = result.unwrap();
    assert_eq!(files.len(), 4, "Should discover 4 supported files");
}

/// Test that empty directory returns empty file list (not error)
#[test]
fn test_discover_files_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    let result = discover_files(dir_path, None);
    assert!(result.is_ok());

    let (files, manifest) = result.unwrap();
    assert_eq!(files.len(), 0, "Empty directory should have 0 files");
    assert_eq!(manifest.total_files, 0, "Manifest should show 0 files");
}

/// Test discovery in nested directories
#[test]
fn test_discover_files_nested_directories() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    let subdir = dir_path.join("subdir");
    fs::create_dir(&subdir).unwrap();

    let root_file = dir_path.join("root.md");
    let sub_file = subdir.join("sub.md");

    fs::write(&root_file, "# Root\n").unwrap();
    fs::write(&sub_file, "# Sub\n").unwrap();

    let result = discover_files(dir_path, None);
    assert!(result.is_ok());

    let (files, _manifest) = result.unwrap();
    assert_eq!(
        files.len(),
        2,
        "Should discover files in nested directories"
    );
}

/// Test that a single markdown file can be indexed directly
#[test]
fn test_discover_single_file() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    let single_file = dir_path.join("single.md");
    fs::write(
        &single_file,
        "# Single Document\n\nThis is a single file to index.",
    )
    .unwrap();

    let result = discover_files(&single_file, None);
    assert!(result.is_ok(), "discover_files should accept single file");

    let (files, manifest) = result.unwrap();
    assert_eq!(files.len(), 1, "Should discover exactly 1 file");
    assert_eq!(manifest.total_files, 1, "Manifest should show 1 file");

    let expected_name = single_file.file_name().map_or_else(
        || "unknown".to_string(),
        |n| n.to_string_lossy().to_string(),
    );
    assert_eq!(files[0].source_path, expected_name);
}

/// Test that single file discovery rejects unsupported file types
#[test]
fn test_discover_single_file_unsupported_type() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    let unsupported_file = dir_path.join("data.json");
    fs::write(&unsupported_file, "{}").unwrap();

    let result = discover_files(&unsupported_file, None);
    assert!(
        result.is_ok(),
        "discover_files should succeed even with unsupported file type"
    );

    let (files, _manifest) = result.unwrap();
    assert_eq!(
        files.len(),
        0,
        "Should discover 0 files for unsupported type"
    );
}

/// Test that single file discovery handles non-existent file
#[test]
fn test_discover_single_file_not_found() {
    use std::path::PathBuf;

    let nonexistent = PathBuf::from("/nonexistent/path/file.md");
    let result = discover_files(&nonexistent, None);

    assert!(
        result.is_err(),
        "discover_files should error for non-existent file"
    );
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("not found"),
        "Error should mention 'not found'"
    );
}

#[test]
fn test_discover_single_file_rejects_empty_file() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("empty.md");
    fs::write(&file, "").unwrap();

    let result = discover_files(&file, None);
    assert!(result.is_err(), "Empty single file should be rejected");
}

#[test]
fn test_discover_single_file_rejects_oversized_file() {
    use std::fs::File;

    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("huge.md");
    let f = File::create(&file).unwrap();
    let config = DiscoverConfig::default();
    let new_len = config.max_file_bytes.saturating_add(1);
    f.set_len(new_len).unwrap();

    let result = discover_files(&file, None);
    assert!(result.is_err(), "Oversized single file should be rejected");
}
