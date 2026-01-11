use doc_transformer::discover::{discover_files, DiscoveryFile, DiscoverManifest};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper function to create test directory structure
fn setup_test_dir() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path();

    // Create markdown files with various extensions
    fs::write(base_path.join("readme.md"), "# README\nTest content").expect("Failed to write readme.md");
    fs::write(base_path.join("guide.mdx"), "# Guide\nMDX content").expect("Failed to write guide.mdx");
    fs::write(base_path.join("spec.rst"), "Spec\n====\nRST content").expect("Failed to write spec.rst");
    fs::write(base_path.join("notes.txt"), "Plain text notes").expect("Failed to write notes.txt");

    // Create files that should be ignored
    fs::write(base_path.join("ignore.pdf"), "PDF content").expect("Failed to write ignore.pdf");
    fs::write(base_path.join("ignore.doc"), "DOC content").expect("Failed to write ignore.doc");

    // Create subdirectory with files
    fs::create_dir(base_path.join("subdirs")).expect("Failed to create subdirs");
    fs::write(
        base_path.join("subdirs").join("nested.md"),
        "# Nested\nNested content",
    )
    .expect("Failed to write nested.md");

    // Create node_modules (should be excluded)
    fs::create_dir(base_path.join("node_modules")).expect("Failed to create node_modules");
    fs::write(
        base_path.join("node_modules").join("package.md"),
        "# Package\nShould be ignored",
    )
    .expect("Failed to write package.md");

    // Create .git (should be excluded)
    fs::create_dir(base_path.join(".git")).expect("Failed to create .git");
    fs::write(base_path.join(".git").join("config"), "git config").expect("Failed to write config");

    // Create deeply nested structure
    fs::create_dir_all(base_path.join("docs").join("api").join("v1"))
        .expect("Failed to create nested dirs");
    fs::write(
        base_path.join("docs").join("api").join("v1").join("endpoints.md"),
        "# API Endpoints\nContent here",
    )
    .expect("Failed to write endpoints.md");

    temp_dir
}

#[test]
fn test_discover_files_basic() {
    let temp_dir = setup_test_dir();
    let source_path = temp_dir.path();

    let result = discover_files(source_path);
    assert!(result.is_ok(), "discover_files should succeed");

    let (files, manifest) = result.unwrap();

    // Should find 5 valid files (readme.md, guide.mdx, spec.rst, notes.txt, nested.md, endpoints.md)
    assert_eq!(files.len(), 6, "Should discover 6 documentation files");

    // Verify manifest
    assert_eq!(manifest.total_files, 6, "Manifest should show 6 files");
    assert_eq!(manifest.files.len(), 6, "Manifest files list should have 6 entries");
    assert!(!manifest.source_dir.is_empty(), "Source dir should be set");
    assert!(!manifest.discovered_at.is_empty(), "Discovered_at should be set");
}

#[test]
fn test_discover_files_excludes_extensions() {
    let temp_dir = setup_test_dir();
    let source_path = temp_dir.path();

    let (files, _) = discover_files(source_path).expect("discover_files should succeed");

    // Should NOT find .pdf, .doc files
    let filenames: Vec<String> = files.iter().map(|f| f.source_path.clone()).collect();
    assert!(!filenames.iter().any(|f| f.ends_with(".pdf")), "PDF files should be excluded");
    assert!(!filenames.iter().any(|f| f.ends_with(".doc")), "DOC files should be excluded");
}

#[test]
fn test_discover_files_excludes_directories() {
    let temp_dir = setup_test_dir();
    let source_path = temp_dir.path();

    let (files, _) = discover_files(source_path).expect("discover_files should succeed");

    let filenames: Vec<String> = files.iter().map(|f| f.source_path.clone()).collect();

    // Should NOT find files in node_modules or .git
    assert!(
        !filenames.iter().any(|f| f.contains("node_modules")),
        "Files in node_modules should be excluded"
    );
    assert!(
        !filenames.iter().any(|f| f.contains(".git")),
        "Files in .git should be excluded"
    );
}

#[test]
fn test_discover_files_calculates_sizes() {
    let temp_dir = setup_test_dir();
    let source_path = temp_dir.path();

    let (files, _) = discover_files(source_path).expect("discover_files should succeed");

    // All files should have size_bytes > 0
    for file in &files {
        assert!(
            file.size_bytes > 0,
            "File {} should have non-zero size",
            file.source_path
        );
    }
}

#[test]
fn test_discover_files_relative_paths() {
    let temp_dir = setup_test_dir();
    let source_path = temp_dir.path();

    let (files, _) = discover_files(source_path).expect("discover_files should succeed");

    // Paths should be relative (not absolute)
    for file in &files {
        assert!(
            !file.source_path.starts_with("/"),
            "Paths should be relative: {}",
            file.source_path
        );
        assert!(
            !file.source_path.contains(".."),
            "Paths should not contain ..: {}",
            file.source_path
        );
    }
}

#[test]
fn test_discover_files_nested_structure() {
    let temp_dir = setup_test_dir();
    let source_path = temp_dir.path();

    let (files, _) = discover_files(source_path).expect("discover_files should succeed");

    let filenames: Vec<String> = files.iter().map(|f| f.source_path.clone()).collect();

    // Should find deeply nested files
    assert!(
        filenames.iter().any(|f| f.contains("docs") && f.contains("endpoints.md")),
        "Should find deeply nested endpoints.md"
    );
}

#[test]
fn test_discover_files_nonexistent_directory() {
    let nonexistent = PathBuf::from("/tmp/this_directory_should_not_exist_xyz");
    let result = discover_files(&nonexistent);

    assert!(
        result.is_err(),
        "Should return error for nonexistent directory"
    );
    let error_msg = format!("{}", result.unwrap_err());
    assert!(
        error_msg.contains("not found"),
        "Error should mention 'not found': {}",
        error_msg
    );
}

#[test]
fn test_discover_files_empty_directory() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let result = discover_files(temp_dir.path()).expect("discover_files should succeed");

    let (files, manifest) = result;
    assert_eq!(files.len(), 0, "Empty directory should have no files");
    assert_eq!(manifest.total_files, 0, "Manifest should show 0 files");
}

#[test]
fn test_discovery_file_structure() {
    let temp_dir = setup_test_dir();
    let source_path = temp_dir.path();

    let (files, _) = discover_files(source_path).expect("discover_files should succeed");

    for file in &files {
        // DiscoveryFile should have source_path and size_bytes
        assert!(
            !file.source_path.is_empty(),
            "source_path should not be empty"
        );
        assert!(file.size_bytes > 0, "size_bytes should be greater than 0");
    }
}

#[test]
fn test_discover_manifest_structure() {
    let temp_dir = setup_test_dir();
    let source_path = temp_dir.path();

    let (_, manifest) = discover_files(source_path).expect("discover_files should succeed");

    // Manifest should have required fields
    assert!(!manifest.source_dir.is_empty(), "source_dir should be set");
    assert!(!manifest.discovered_at.is_empty(), "discovered_at should be set");
    assert!(manifest.total_files > 0, "total_files should match actual files");
    assert_eq!(
        manifest.total_files,
        manifest.files.len(),
        "total_files should match files.len()"
    );
}

#[test]
fn test_discover_supported_extensions() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path();

    // Create files with each supported extension
    fs::write(base_path.join("file1.md"), "markdown").expect("Failed to write");
    fs::write(base_path.join("file2.mdx"), "mdx").expect("Failed to write");
    fs::write(base_path.join("file3.rst"), "restructured text").expect("Failed to write");
    fs::write(base_path.join("file4.txt"), "plain text").expect("Failed to write");

    let (files, _) = discover_files(base_path).expect("discover_files should succeed");

    assert_eq!(files.len(), 4, "Should find all 4 supported extensions");

    let paths: Vec<&str> = files.iter().map(|f| f.source_path.as_str()).collect();
    assert!(paths.iter().any(|p| p.contains(".md")), "Should find .md files");
    assert!(paths.iter().any(|p| p.contains(".mdx")), "Should find .mdx files");
    assert!(paths.iter().any(|p| p.contains(".rst")), "Should find .rst files");
    assert!(paths.iter().any(|p| p.contains(".txt")), "Should find .txt files");
}

#[test]
fn test_discover_excludes_build_dirs() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let base_path = temp_dir.path();

    fs::create_dir(base_path.join("_build")).expect("Failed to create _build");
    fs::write(
        base_path.join("_build").join("doc.md"),
        "in build dir",
    )
    .expect("Failed to write");

    fs::create_dir(base_path.join("dist")).expect("Failed to create dist");
    fs::write(base_path.join("dist").join("doc.md"), "in dist").expect("Failed to write");

    fs::create_dir(base_path.join("vendor")).expect("Failed to create vendor");
    fs::write(
        base_path.join("vendor").join("doc.md"),
        "in vendor",
    )
    .expect("Failed to write");

    fs::write(base_path.join("normal.md"), "normal file").expect("Failed to write");

    let (files, _) = discover_files(base_path).expect("discover_files should succeed");

    assert_eq!(files.len(), 1, "Should only find the normal file");
    assert_eq!(files[0].source_path, "normal.md", "Only normal.md should be found");
}
