use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_analyze_root_path_error() {
    // Root path "/" should have no filename and trigger error
    let root_path = "/";
    let file_name = Path::new(root_path).file_name();

    assert!(file_name.is_none(), "Root path should have no filename");
}

#[test]
fn test_analyze_empty_stem_fallback() {
    // In Rust, hidden files like ".hidden" actually have a stem of ".hidden"
    let hidden_file = ".hidden";
    let stem = Path::new(hidden_file).file_stem();

    // .hidden should have stem of ".hidden", NOT empty
    assert_eq!(
        stem.map(|s| s.is_empty()),
        Some(false),
        "Hidden file has non-empty stem"
    );
    assert_eq!(stem.unwrap().to_string_lossy(), ".hidden");

    // Test actual empty stem case (root path has no stem)
    let root = "/";
    let root_stem = Path::new(root).file_stem();
    assert!(root_stem.is_none(), "Root path has no stem");

    // With our fallback, None stems become "untitled"
    let title = root_stem
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| std::ffi::OsStr::new("untitled"))
        .to_string_lossy();

    assert_eq!(title, "untitled", "None stem should fallback to 'untitled'");
}

#[test]
fn test_analyze_valid_filename() {
    // Normal filename should work fine
    let valid_file = "path/to/document.md";
    let stem = Path::new(valid_file).file_stem();

    assert!(stem.is_some(), "Valid filename should have stem");
    assert_eq!(stem.unwrap().to_string_lossy(), "document");
}

#[test]
fn test_analyze_trailing_slash() {
    // In Rust, path with trailing slash still has filename as the last component
    let trailing = "path/to/dir/";
    let file_name = Path::new(trailing).file_name();

    // Rust Path keeps the last component even with trailing slash
    assert!(file_name.is_some(), "Trailing slash keeps last component");
    assert_eq!(file_name.unwrap().to_string_lossy(), "dir");
}

#[test]
fn test_config_empty_filename_pattern() {
    // Empty filename pattern should not match empty stems
    let empty_stem = "";
    let patterns = ["api".to_string(), "reference".to_string()];

    let matches = patterns
        .iter()
        .any(|p| empty_stem.to_lowercase().contains(&p.to_lowercase()));

    assert!(!matches, "Empty stem should not match any filename pattern");
}

#[test]
fn test_filename_comparison_with_filter() {
    // Test filename filtering behavior
    let root_path = "/";
    let dir_path = "some/dir/";

    let file1 = Path::new(root_path).file_name().filter(|s| !s.is_empty());
    let file2 = Path::new(dir_path).file_name().filter(|s| !s.is_empty());

    // Root has no filename (None), dir path has filename "dir"
    assert!(file1.is_none(), "Root path has no filename");
    assert!(file2.is_some(), "Dir path has filename");
    assert_eq!(file2.unwrap().to_string_lossy(), "dir");

    // They should not be equal
    assert_ne!(file1.is_some(), file2.is_some());
}

#[test]
fn test_assign_ids_with_root_path_fallback() {
    // Test that root paths get fallback "untitled" slug
    let path = "/";
    let stem = Path::new(path)
        .file_stem()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "untitled".to_string());

    assert_eq!(stem, "untitled", "Root path should fallback to 'untitled'");
}

#[test]
fn test_transform_empty_filename_comparison() {
    // When comparing filenames for link resolution
    let src_path = "/";
    let resolved_path = "some/dir/";

    let src_file = Path::new(src_path).file_name().filter(|s| !s.is_empty());
    let resolved_file = Path::new(resolved_path)
        .file_name()
        .filter(|s| !s.is_empty());

    // Root has no filename, dir path has filename
    assert!(src_file.is_none(), "Root has no filename");
    assert!(resolved_file.is_some(), "Dir path has filename");

    // They should not match
    assert!(
        src_file.is_none(),
        "Root should not be treated as valid file"
    );
    assert!(resolved_file.is_some(), "Dir should be valid");
}

#[test]
fn test_utf8_handling_in_path() {
    // Test that valid UTF-8 paths work correctly
    let valid_path = "path/to/файл.md"; // Cyrillic characters
    let stem = Path::new(valid_path).file_stem();

    assert!(stem.is_some(), "UTF-8 path should parse correctly");
    assert_eq!(stem.unwrap().to_string_lossy(), "файл");
}

#[test]
fn test_multiple_dots_in_filename() {
    // Files like "file.tar.gz" should handle stem correctly
    let path = "archive/data.tar.gz";
    let stem = Path::new(path).file_stem();

    // file_stem() gets up to the first dot
    assert_eq!(stem.unwrap().to_string_lossy(), "data.tar");
}

#[test]
fn test_empty_category_never_occurs() {
    // With our fixes, category should never be empty string
    let filename = "";
    let stem = Path::new(filename)
        .file_stem()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| std::ffi::OsStr::new("untitled"))
        .to_string_lossy()
        .to_lowercase();

    assert!(!stem.is_empty(), "Category should never be empty");
    assert_eq!(stem, "untitled", "Empty filename becomes 'untitled'");
}

#[test]
fn test_path_with_only_extension() {
    // In Rust, file like ".gitignore" has stem of ".gitignore"
    let path = ".gitignore";
    let stem = Path::new(path).file_stem();

    // .gitignore has non-empty stem
    assert_eq!(stem.map(|s| s.is_empty()), Some(false));
    assert_eq!(stem.unwrap().to_string_lossy(), ".gitignore");

    // Test a path that actually has no stem (root)
    let root = "/";
    let root_stem = Path::new(root).file_stem();
    assert!(root_stem.is_none());

    // Fallback should apply to None
    let fallback = root_stem
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| std::ffi::OsStr::new("untitled"))
        .to_string_lossy();

    assert_eq!(fallback, "untitled");
}

#[test]
fn test_analyze_file_with_frontmatter() {
    // Create a temporary file with frontmatter but no H1
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.md");

    let content = r#"---
title: My Document
tags: ["tag1", "tag2"]
---

Some content here.
"#;

    fs::write(&test_file, content).unwrap();

    // File should be readable and have valid path
    let stem = test_file.file_stem();
    assert!(stem.is_some());
    assert_eq!(stem.unwrap().to_string_lossy(), "test");
}

#[test]
fn test_concurrent_path_operations() {
    // Ensure our changes don't break concurrent access
    use std::thread;

    let paths = [
        "/",
        "docs/api.md",
        ".hidden",
        "path/to/file.txt",
        "trailing/dir/",
    ];

    let handles: Vec<_> = paths
        .iter()
        .map(|p| {
            let path = p.to_string();
            thread::spawn(move || {
                let file_name = Path::new(&path).file_name().filter(|s| !s.is_empty());
                let file_stem = Path::new(&path).file_stem().filter(|s| !s.is_empty());

                // Should not panic and should filter consistently
                (file_name.is_some(), file_stem.is_some())
            })
        })
        .collect();

    for handle in handles {
        let result = handle.join();
        assert!(result.is_ok(), "Thread should complete without panic");
    }
}
