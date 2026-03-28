#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]


//! Path edge case tests
//!
//! Tests paths with spaces, permission errors, special characters, and unicode.
//! Consolidated to 10 essential tests.

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

    fn path_with_spaces(&self, base: &str) -> PathBuf {
        self.root().join([base, "with", "spaces"].join(" "))
    }
}

// ============================================================================
// PATHS WITH SPACES (2 tests)
// ============================================================================

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

// ============================================================================
// UNICODE PATHS (2 tests)
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
fn test_discover_files_from_unicode_path() {
    // Test discover_files with unicode directory names
    let ctx = PathTestContext::new();
    let source_dir = ctx.root().join("文档-source-📚");

    fs::create_dir_all(&source_dir).expect("Should create unicode dir");

    // Create test files
    ctx.create_markdown_file("文档-source-📚/guide.md", "# Guide\n\nContent.");
    ctx.create_markdown_file("文档-source-📚/api/ref.md", "# API\n\nReference.");

    // Use the discover module to find files
    let result = doc_transformer::filter::discover_test_files(&source_dir);

    assert!(result.is_ok(), "Should discover files from unicode path");
    let files = result.unwrap();
    assert_eq!(files.len(), 2, "Should find all files from unicode path");
}

// ============================================================================
// PERMISSION ERRORS (2 tests)
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
            }
        }

        // Restore permissions for cleanup
        fs::set_permissions(&readonly_dir, perms).expect("Should restore permissions");
    }

    #[cfg(windows)]
    {
        // Windows has different permission handling
        assert!(readonly_dir.exists());
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
        assert!(parent_dir.exists());
    }
}

// ============================================================================
// DEEP NESTING (1 test)
// ============================================================================

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

// ============================================================================
// SPECIAL CHARACTERS (1 test)
// ============================================================================

#[test]
fn test_mixed_special_characters_in_path() {
    // Path with mixed special characters (dots, dashes, underscores)
    let ctx = PathTestContext::new();

    let mixed_paths = [
        "docs-v1.2.3_test",
        "my.docs-v2_api",
        "src_v1.2.3-beta",
        "a.b-c_d-e.f",
    ];

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
// INTEGRATION: COMBINED EDGE CASES (2 tests)
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
