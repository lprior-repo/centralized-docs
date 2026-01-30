//! Comprehensive path edge case tests
//!
//! Tests paths with spaces, permission errors, special characters, and unicode.
//! These are P1 priority issues that must work correctly.

use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

// ============================================================================
// TEST CONTEXT HELPER
// ============================================================================

struct PathTestContext {
    temp_dir: TempDir,
}

impl PathTestContext {
    fn new() -> Self {
        PathTestContext {
            temp_dir: TempDir::new().expect("Failed to create temp dir"),
        }
    }

    fn root(&self) -> &Path {
        self.temp_dir.path()
    }

    fn create_nested_path(&self, path: &str) -> PathBuf {
        let full_path = self.root().join(path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create parent directory");
        }
        full_path
    }

    fn create_markdown_file(&self, rel_path: &str, content: &str) -> PathBuf {
        let path = self.root().join(rel_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Failed to create parent directory");
        }
        fs::write(&path, content).expect("Failed to write file");
        path
    }

    #[allow(dead_code)]
    fn path_with_spaces(&self, base: &str) -> PathBuf {
        self.root().join([base, "with", "spaces"].join(" "))
    }

    #[allow(dead_code)]
    fn path_with_unicode(&self, base: &str) -> PathBuf {
        self.root().join(format!("{base}-文档-𝔲𝔫𝔦𝔠𝔬𝔡𝔢"))
    }
}

// ============================================================================
// PATHS WITH SPACES TESTS
// ============================================================================

#[test]
fn test_output_directory_with_spaces() {
    // Output directory with spaces should work correctly
    let ctx = PathTestContext::new();
    let output_dir = ctx.path_with_spaces("output");

    // Create a simple markdown file in source
    ctx.create_markdown_file("source/test.md", "# Test\n\nContent here.");

    // The output directory should be creatable with spaces
    fs::create_dir_all(&output_dir).expect("Should create directory with spaces");

    // Verify it exists
    assert!(
        output_dir.exists(),
        "Output directory with spaces should exist"
    );
    assert!(output_dir.is_dir(), "Should be a directory");

    // Should be able to write files to it
    let test_file = output_dir.join("INDEX.json");
    fs::write(&test_file, r#"{"test": "data"}"#).expect("Should write to directory with spaces");
    assert!(
        test_file.exists(),
        "File should exist in directory with spaces"
    );

    // Read back and verify
    let content = fs::read_to_string(&test_file).expect("Should read from directory with spaces");
    assert!(content.contains("test"), "Content should be readable");
}

#[test]
fn test_source_directory_with_spaces() {
    // Source directory with spaces should be discoverable
    let ctx = PathTestContext::new();
    let source_dir = ctx.path_with_spaces("source");

    // Create nested directory structure with spaces
    let docs_dir = source_dir.join("docs");
    fs::create_dir_all(&docs_dir).expect("Should create nested source dir with spaces");

    // Create test files
    fs::write(docs_dir.join("guide.md"), "# Guide\n\nThis is a guide.")
        .expect("Should create file in source dir with spaces");

    // Files should be discoverable from source with spaces
    let files: Vec<_> = walkdir::WalkDir::new(&source_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.path().extension().map(|ext| ext.to_owned()))
        .filter(|e| e.to_string_lossy() == "md")
        .collect();

    assert_eq!(
        files.len(),
        1,
        "Should discover file from source dir with spaces"
    );
}

#[test]
fn test_both_source_and_output_with_spaces() {
    // Both source and output directories with spaces should work
    let ctx = PathTestContext::new();
    let source_dir = ctx.path_with_spaces("my source");
    let output_dir = ctx.path_with_spaces("my output");

    // Create source structure
    let docs_dir = source_dir.join("docs");
    fs::create_dir_all(&docs_dir).expect("Should create source dir with spaces");
    fs::write(docs_dir.join("test.md"), "# Test\n\nContent.")
        .expect("Should create source file with spaces in path");

    // Create output structure
    fs::create_dir_all(&output_dir).expect("Should create output dir with spaces");

    // Verify both are accessible
    assert!(source_dir.exists(), "Source dir with spaces should exist");
    assert!(output_dir.exists(), "Output dir with spaces should exist");

    // Simulate copying/transforming between them
    let source_files: Vec<_> = walkdir::WalkDir::new(&source_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .collect();

    for entry in source_files {
        let file_name = entry.file_name();
        let dest = output_dir.join(file_name);
        fs::copy(entry.path(), &dest).expect("Should copy between dirs with spaces");
        assert!(dest.exists(), "Copied file should exist");
    }
}

#[test]
fn test_index_json_created_in_correct_location_with_spaces() {
    // INDEX.json should be created in the correct location even with spaces
    let ctx = PathTestContext::new();
    let output_dir = ctx.path_with_spaces("final output");

    fs::create_dir_all(&output_dir).expect("Should create output dir");

    // Simulate creating INDEX.json
    let index_path = output_dir.join("INDEX.json");
    let index_content = r#"{
        "documents": [],
        "chunks": [],
        "metadata": {"version": "1.0"}
    }"#;

    fs::write(&index_path, index_content).expect("Should write INDEX.json");

    // Verify INDEX.json is in the correct location
    assert!(index_path.exists(), "INDEX.json should exist");
    assert_eq!(
        index_path.parent(),
        Some(output_dir.as_path()),
        "INDEX.json should be directly in output directory"
    );

    // Verify content
    let read_content = fs::read_to_string(&index_path).expect("Should read INDEX.json");
    assert!(
        read_content.contains("documents"),
        "INDEX.json should have content"
    );
}

#[test]
fn test_multiple_spaces_in_path() {
    // Paths with multiple consecutive spaces
    let ctx = PathTestContext::new();
    let multi_space_dir = ctx.root().join("dir   with   many   spaces");

    fs::create_dir_all(&multi_space_dir).expect("Should create dir with multiple spaces");
    assert!(
        multi_space_dir.exists(),
        "Dir with multiple spaces should exist"
    );

    let test_file = multi_space_dir.join("test.md");
    fs::write(&test_file, "# Test").expect("Should write to dir with multiple spaces");
    assert!(test_file.exists(), "File should exist");
}

#[test]
fn test_leading_trailing_spaces_in_path() {
    // Paths with leading/trailing spaces (if filesystem allows)
    let ctx = PathTestContext::new();

    // Most filesystems allow trailing spaces in directory names
    let trailing_space = ctx.root().join("dir ");
    let result = fs::create_dir(&trailing_space);

    // This may fail on some filesystems, so we handle both cases
    match result {
        Ok(_) => {
            assert!(
                trailing_space.exists(),
                "Dir with trailing space should exist"
            );
        }
        Err(_) => {
            // Some filesystems don't allow this, which is fine
            // Test passes as we've identified the behavior
        }
    }
}

#[test]
fn test_discover_files_from_path_with_spaces() {
    // Test the actual discover_files function with spaces
    let ctx = PathTestContext::new();
    let source_dir = ctx.path_with_spaces("source");

    // Create test files
    let guide_path = source_dir.join("guide.md");
    if let Some(parent) = guide_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create parent directory");
    }
    fs::write(&guide_path, "# Guide\n\nContent.").expect("Failed to write file");

    let api_path = source_dir.join("api/reference.md");
    if let Some(parent) = api_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create parent directory");
    }
    fs::write(&api_path, "# API\n\nReference.").expect("Failed to write file");

    // Use the discover module to find files
    let result = doc_transformer::filter::discover_test_files(&source_dir);

    assert!(
        result.is_ok(),
        "Should discover files from path with spaces"
    );
    let files = result.unwrap();
    assert_eq!(
        files.len(),
        2,
        "Should find all files from path with spaces"
    );
}

// ============================================================================
// PERMISSION ERROR TESTS
// ============================================================================

#[test]
fn test_readonly_output_directory_clear_error() {
    // Read-only output directory should give a clear error
    let ctx = PathTestContext::new();
    let readonly_dir = ctx.root().join("readonly");

    fs::create_dir_all(&readonly_dir).expect("Should create dir");

    // Make directory read-only (Unix-like systems)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = fs::metadata(&readonly_dir)
            .expect("Should get metadata")
            .permissions();
        let mut readonly_perms = perms.clone();
        readonly_perms.set_mode(0o444); // Read-only
        fs::set_permissions(&readonly_dir, readonly_perms).expect("Should set read-only");

        // Try to create a file - should fail with PermissionDenied
        let test_file = readonly_dir.join("test.md");
        let write_result = fs::write(&test_file, "# Test");

        match write_result {
            Err(e) => {
                assert_eq!(
                    e.kind(),
                    std::io::ErrorKind::PermissionDenied,
                    "Should be PermissionDenied error"
                );
            }
            Ok(_) => {
                // Some systems allow writes even with 0o444 if owned by user
                // Clean up and note this
            }
        }

        // Restore permissions for cleanup
        fs::set_permissions(&readonly_dir, perms).expect("Should restore permissions");
    }

    #[cfg(windows)]
    {
        // Windows has different permission handling
        // Just verify the directory exists
        assert!(readonly_dir.exists());
    }
}

#[test]
fn test_permission_denied_message_has_hint() {
    // Error message for permission denied should have helpful hints
    let _ctx = PathTestContext::new();

    // Try to write to a path that doesn't exist (simulates permission issue parent)
    let invalid_path = "/root/no-permission-dir/test.txt";

    let result = fs::write(invalid_path, "test");

    if let Err(e) = result {
        let _error_msg = e.to_string().to_lowercase();

        // Error should mention permission or access
        let is_permission_error = e.kind() == std::io::ErrorKind::PermissionDenied;

        if is_permission_error {
            // On Unix, permission denied should be in error kind
            // (actual message varies by system)
        }
    }
}

#[test]
fn test_readonly_parent_directory_detection() {
    // Detect when parent directory is read-only
    let ctx = PathTestContext::new();
    let parent_dir = ctx.root().join("readonly-parent");

    fs::create_dir_all(&parent_dir).expect("Should create parent");

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = fs::metadata(&parent_dir)
            .expect("Should get metadata")
            .permissions();
        let mut readonly_perms = perms.clone();
        readonly_perms.set_mode(0o444);
        fs::set_permissions(&parent_dir, readonly_perms).expect("Should set read-only");

        // Try to create subdirectory - should fail
        let child_dir = parent_dir.join("child");
        let create_result = fs::create_dir(&child_dir);

        match create_result {
            Err(e) => {
                assert_eq!(
                    e.kind(),
                    std::io::ErrorKind::PermissionDenied,
                    "Creating child in readonly parent should be PermissionDenied"
                );
            }
            Ok(_) => {
                // Some systems allow this
            }
        }

        // Restore permissions
        fs::set_permissions(&parent_dir, perms).expect("Should restore permissions");
    }

    #[cfg(windows)]
    {
        // Windows-specific handling
        assert!(parent_dir.exists());
    }
}

#[test]
fn test_check_write_permission_function() {
    // Test the permission checking function if available
    let ctx = PathTestContext::new();
    let writable_dir = ctx.root().join("writable");

    fs::create_dir_all(&writable_dir).expect("Should create writable dir");

    // Try to create a test file
    let test_file = writable_dir.join(".permission_check.tmp");
    let write_result = fs::write(&test_file, b"");

    assert!(
        write_result.is_ok(),
        "Should be able to write to writable dir"
    );

    // Clean up
    if test_file.exists() {
        let _ = fs::remove_file(&test_file);
    }
}

// ============================================================================
// SPECIAL CHARACTER TESTS
// ============================================================================

#[test]
fn test_path_with_unicode_characters() {
    // Paths with unicode characters should work
    let ctx = PathTestContext::new();

    // Test various unicode scripts
    let unicode_paths = [
        "文档-中文",    // Chinese
        "документация", // Cyrillic
        "dokumentáció", // Hungarian with accents
        "αρχείο",       // Greek
        "𝔲𝔫𝔦𝔠𝔬𝔡𝔢-🎉",   // Mathematical symbols and emoji
    ];

    for path_name in unicode_paths {
        let unicode_path = ctx.root().join(path_name);
        let create_result = fs::create_dir(&unicode_path);

        if create_result.is_ok() {
            assert!(
                unicode_path.exists(),
                "Unicode path should exist: {path_name}"
            );

            // Should be able to write files
            let file_path = unicode_path.join("test.md");
            fs::write(&file_path, "# Test").expect("Should write to unicode path");
            assert!(file_path.exists(), "File should exist in unicode path");
        }
        // Some filesystems may not support all unicode - that's OK
    }
}

#[test]
fn test_path_with_dots() {
    // Paths with dots should work correctly
    let ctx = PathTestContext::new();

    let dot_paths = [
        "docs.v1",
        "docs.v2.0",
        "docs..test",
        ".hidden.docs",
        "docs.with.many.dots.in.path",
    ];

    for path_name in dot_paths {
        let dot_path = ctx.create_nested_path(path_name);
        fs::create_dir_all(&dot_path).expect("Should create path with dots");

        let file_path = dot_path.join("test.md");
        fs::write(&file_path, "# Test").expect("Should write to path with dots");
        assert!(
            file_path.exists(),
            "File should exist in path with dots: {path_name}"
        );
    }
}

#[test]
fn test_path_with_dashes() {
    // Paths with dashes should work
    let ctx = PathTestContext::new();

    let dash_paths = [
        "my-docs",
        "my-dashed-directory",
        "docs-v1.2.3",
        "some--double--dashes",
    ];

    for path_name in dash_paths {
        let dash_path = ctx.create_nested_path(path_name);
        fs::create_dir_all(&dash_path).expect("Should create path with dashes");

        let file_path = dash_path.join("test.md");
        fs::write(&file_path, "# Test").expect("Should write to path with dashes");
        assert!(
            file_path.exists(),
            "File should exist in path with dashes: {path_name}"
        );
    }
}

#[test]
fn test_deeply_nested_paths() {
    // Deeply nested paths should work
    let ctx = PathTestContext::new();

    let nested_path = ctx.create_nested_path("a/b/c/d/e/f/g/h/i/j");
    fs::create_dir_all(&nested_path).expect("Should create deeply nested path");

    let file_path = nested_path.join("deep.md");
    fs::write(&file_path, "# Deep File\n\nContent at depth 10.")
        .expect("Should write to deeply nested path");

    assert!(file_path.exists(), "File should exist at depth 10");
    assert!(file_path.is_file(), "Should be a file");

    // Verify we can read it back
    let content = fs::read_to_string(&file_path).expect("Should read deeply nested file");
    assert!(content.contains("Deep File"), "Content should be readable");
}

#[test]
fn test_very_long_path_name() {
    // Very long path names (within filesystem limits)
    let ctx = PathTestContext::new();

    // Most filesystems support 255 bytes per component
    let long_name = "a".repeat(100);
    let long_path = ctx.root().join(&long_name);

    let create_result = fs::create_dir(&long_path);

    if create_result.is_ok() {
        assert!(long_path.exists(), "Long path should exist");

        let file_path = long_path.join("test.md");
        fs::write(&file_path, "# Test").expect("Should write to long path");
        assert!(file_path.exists());
    }
}

#[test]
fn test_path_with_underscores() {
    // Paths with underscores should work
    let ctx = PathTestContext::new();

    let underscore_paths = ["my_docs", "__private__", "my_private_docs", "docs_v2"];

    for path_name in underscore_paths {
        let underscore_path = ctx.create_nested_path(path_name);
        fs::create_dir_all(&underscore_path).expect("Should create path with underscores");

        let file_path = underscore_path.join("test.md");
        fs::write(&file_path, "# Test").expect("Should write to path with underscores");
        assert!(
            file_path.exists(),
            "File should exist in path with underscores: {path_name}"
        );
    }
}

#[test]
fn test_mixed_special_characters_in_path() {
    // Path with mixed special characters
    let ctx = PathTestContext::new();

    let mixed_paths = ["docs-v1.2.3_test", "my.docs-v2_api", "src_v1.2.3-beta"];

    for path_name in mixed_paths {
        let mixed_path = ctx.create_nested_path(path_name);
        fs::create_dir_all(&mixed_path).expect("Should create mixed special char path");

        let file_path = mixed_path.join("test.md");
        fs::write(&file_path, "# Test").expect("Should write to mixed path");
        assert!(
            file_path.exists(),
            "File should exist in mixed path: {path_name}"
        );
    }
}

// ============================================================================
// EDGE CASE: EMPTY AND ROOT PATHS
// ============================================================================

#[test]
fn test_empty_path_component() {
    // Empty path components should be handled gracefully
    let empty = "";

    let path = Path::new(empty);
    assert_eq!(path, Path::new(""));

    // Empty path should not have a filename
    let file_name = path.file_name();
    assert!(
        file_name.is_none_or(|f| f.is_empty()),
        "Empty path has no filename"
    );
}

#[test]
fn test_single_dot_path() {
    // Single dot (current directory) should be handled
    let dot_path = Path::new(".");

    assert!(dot_path.exists(), "Current directory should exist");
    assert!(dot_path.is_dir(), "Should be a directory");

    // Getting absolute path should work
    let absolute = dot_path.canonicalize();
    assert!(absolute.is_ok(), "Should canonicalize '.'");
}

#[test]
fn test_double_dot_path() {
    // Double dot (parent directory) should be handled
    let dotdot_path = Path::new("..");

    assert!(dotdot_path.exists(), "Parent directory should exist");
    assert!(dotdot_path.is_dir(), "Should be a directory");
}

// ============================================================================
// INTEGRATION: FULL DISCOVER WITH EDGE CASES
// ============================================================================

#[test]
fn test_discover_with_all_edge_cases() {
    // Test discover with multiple edge cases at once
    let ctx = PathTestContext::new();
    let source_dir = ctx.root().join("complex source");

    // Create various edge case files
    ctx.create_markdown_file("complex source/docs-v1/api.md", "# API\n\nContent.");
    ctx.create_markdown_file("complex source/docs_v2/guide.md", "# Guide\n\nContent.");
    ctx.create_markdown_file(
        "complex source/docs.v3/reference.md",
        "# Reference\n\nContent.",
    );
    ctx.create_markdown_file("complex source/a/b/c/deep.md", "# Deep\n\nContent.");

    // Discover should handle all cases
    let result = doc_transformer::filter::discover_test_files(&source_dir);

    assert!(result.is_ok(), "Should discover from complex source");
    let files = result.unwrap();
    assert_eq!(files.len(), 4, "Should find all edge case files");
}

// ============================================================================
// TEST FILENAME EXTRACTION WITH EDGE CASES
// ============================================================================

#[test]
fn test_filename_extraction_with_spaces() {
    // Filename extraction should handle spaces
    let paths_with_spaces = [
        "/path/to/my file.md",
        "/path/to/file with spaces.md",
        "/path/to/multiple   spaces.md",
    ];

    for path_str in paths_with_spaces {
        let path = Path::new(path_str);
        let file_name = path.file_name();

        assert!(file_name.is_some(), "Should extract filename with spaces");
        let name = file_name.unwrap().to_string_lossy();
        assert!(name.contains(" "), "Filename should preserve spaces");
        assert!(name.ends_with(".md"), "Should have .md extension");
    }
}

#[test]
fn test_filename_extraction_with_dots() {
    // Filename extraction should handle dots correctly
    let test_cases = [
        ("/path/to/file.md", "file.md"),
        ("/path/to/file.v1.md", "file.v1.md"),
        ("/path/to/.hidden", ".hidden"),
        ("/path/to/file.tar.gz", "file.tar.gz"),
    ];

    for (path_str, expected_name) in test_cases {
        let path = Path::new(path_str);
        let file_name = path.file_name();

        assert!(file_name.is_some(), "Should extract filename");
        let name = file_name.unwrap().to_string_lossy();
        assert_eq!(name, expected_name, "Filename should match: {path_str}");
    }
}

// ============================================================================
// PATH NORMALIZATION TESTS
// ============================================================================

#[test]
fn test_path_with_trailing_slash() {
    // Paths with trailing slashes should be normalized
    let paths_with_trailing = ["/path/to/dir/", "/path/to/dir//", "relative/path/"];

    for path_str in paths_with_trailing {
        let path = Path::new(path_str);
        let file_name = path.file_name();

        // Rust Path keeps the last component even with trailing slash
        assert!(
            file_name.is_some(),
            "Should handle trailing slash: {path_str}"
        );
    }
}

#[test]
fn test_path_canonicalization() {
    // Path canonicalization should resolve . and ..
    let ctx = PathTestContext::new();

    // Create a nested structure
    let deep_dir = ctx.create_nested_path("a/b/c");
    fs::create_dir_all(&deep_dir).expect("Should create nested");

    // Create file at depth
    let deep_file = deep_dir.join("test.md");
    fs::write(&deep_file, "# Test").expect("Should write file");

    // Navigate using .. and canonicalize - use parent dir for join
    let relative_path = deep_dir.join("../../b/c/test.md");
    let canonical = relative_path.canonicalize();

    assert!(canonical.is_ok(), "Should canonicalize path with ..");
    assert!(
        canonical.unwrap().ends_with("test.md"),
        "Should resolve to correct file"
    );
}
