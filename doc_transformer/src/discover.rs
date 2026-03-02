use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use walkdir::WalkDir;

const MAX_SOURCE_FILE_BYTES: u64 = 50 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryFile {
    pub source_path: String,
    pub size_bytes: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoverManifest {
    pub source_dir: String,
    pub discovered_at: String,
    pub total_files: usize,
    pub files: Vec<DiscoveryFile>,
}

/// Discover all markdown and text files in a directory tree
///
/// Walks the directory tree recursively, finding all supported file types
/// and building a manifest with metadata.
///
/// # Errors
///
/// Returns an error if:
/// - The source directory does not exist
/// - The canonical path cannot be resolved
#[allow(clippy::too_many_lines)]
pub fn discover_files(source_dir: &Path) -> Result<(Vec<DiscoveryFile>, DiscoverManifest)> {
    if !source_dir.exists() {
        anyhow::bail!("Source not found: {}", source_dir.display());
    }

    let canonical_path = source_dir.canonicalize().context(format!(
        "Failed to resolve canonical path for {}",
        source_dir.display()
    ))?;

    // Ensure source is a file or directory (not device, socket, pipe, etc.)
    if !canonical_path.is_file() && !canonical_path.is_dir() {
        anyhow::bail!(
            "Source must be a file or directory, not a special file: {}",
            source_dir.display()
        );
    }

    let mut files = Vec::new();
    let mut skipped_large = 0usize;
    let mut skipped_empty = 0usize;
    let mut skipped_broken_symlink = 0usize;
    let mut skipped_io_error = 0usize;
    // Track files skipped specifically due to permission denied (chmod 000, etc.)
    // These must cause a non-zero exit so the user knows indexing is incomplete.
    let mut permission_denied_files: Vec<String> = Vec::new();
    // Markdown extensions (primary)
    let markdown_exts = [".md", ".mdx", ".markdown", ".mdown", ".mkd"];
    // Text extensions (processed but may not be actual markdown - warn)
    let text_exts = [".txt"];
    // RestructuredText
    let rst_exts = [".rst"];
    // All supported extensions combined
    let all_exts: Vec<&str> = markdown_exts
        .iter()
        .copied()
        .chain(text_exts.iter().copied())
        .chain(rst_exts.iter().copied())
        .collect();
    let exclude_dirs = ["node_modules", ".git", "_build", "dist", "vendor"];

    // Handle single file case directly
    if canonical_path.is_file() {
        return discover_single_file(&canonical_path, &all_exts);
    }

    // Directory case: walk the directory tree
    for entry in WalkDir::new(&canonical_path) {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                // Check if the underlying IO error is a permission denied
                let is_permission_denied = e
                    .io_error()
                    .map(|io_err| io_err.kind() == std::io::ErrorKind::PermissionDenied)
                    .unwrap_or(false);
                if is_permission_denied {
                    let path_str = e
                        .path()
                        .map(|p| p.display().to_string())
                        .unwrap_or_else(|| "<unknown>".to_string());
                    eprintln!("Error: Cannot read file '{path_str}': permission denied");
                    permission_denied_files.push(path_str);
                } else {
                    skipped_io_error = skipped_io_error.saturating_add(1);
                    eprintln!("Warning: Skipping path due to I/O error: {e}");
                }
                continue;
            }
        };

        let path = entry.path();

        // Skip excluded directories (exact match on directory name)
        if path.components().any(|c| {
            exclude_dirs
                .iter()
                .any(|excl| c.as_os_str().to_string_lossy() == *excl)
        }) {
            continue;
        }

        // Check for broken symlinks before file type check
        // Symlinks with no valid target should be warned about and skipped
        if entry.file_type().is_symlink() {
            // Try to read the metadata following the symlink
            // If this fails, the symlink target doesn't exist (broken symlink)
            if std::fs::metadata(path).is_err() {
                // Symlink is broken (target does not exist)
                skipped_broken_symlink = skipped_broken_symlink.saturating_add(1);
                let symlink_name = path.file_name().map_or_else(
                    || "unknown".to_string(),
                    |n| n.to_string_lossy().to_string(),
                );
                eprintln!(
                    "Warning: Skipping broken symlink '{symlink_name}' (target does not exist)"
                );
                continue;
            }
        }

        if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext_str = format!(".{}", ext.to_string_lossy());
                if all_exts.contains(&ext_str.as_str()) {
                    // Warn for non-markdown text files being processed as markdown
                    if text_exts.contains(&ext_str.as_str()) {
                        let filename = path.file_name().map_or_else(
                            || "unknown".to_string(),
                            |n| n.to_string_lossy().to_string(),
                        );
                        eprintln!(
                            "Warning: Processing non-markdown file '{filename}' with extension '{ext_str}'. \
                            This file may not be valid markdown."
                        );
                    }
                    // Get relative path, skip if it fails (e.g., prefix mismatch)
                    let rel_path = match path.strip_prefix(&canonical_path) {
                        Ok(p) => p.to_string_lossy().to_string(),
                        Err(e) => {
                            eprintln!(
                                "Warning: Failed to get relative path for {}: {e}",
                                path.display()
                            );
                            continue;
                        }
                    };

                    // Get file size, skip if metadata fails (e.g., permission denied)
                    let size = match path.metadata() {
                        Ok(meta) => meta.len(),
                        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                            let path_str = path.display().to_string();
                            eprintln!("Error: Cannot read file '{path_str}': permission denied");
                            permission_denied_files.push(path_str);
                            continue;
                        }
                        Err(e) => {
                            skipped_io_error = skipped_io_error.saturating_add(1);
                            eprintln!(
                                "Warning: Failed to read metadata for {}: {e}, skipping file",
                                path.display()
                            );
                            continue;
                        }
                    };

                    if size == 0 {
                        skipped_empty = skipped_empty.saturating_add(1);
                        eprintln!("Error: Skipping empty file {}", path.display());
                        continue;
                    }

                    if size > MAX_SOURCE_FILE_BYTES {
                        skipped_large = skipped_large.saturating_add(1);
                        eprintln!(
                            "Warning: Skipping oversized file {} ({} bytes exceeds {} byte limit)",
                            path.display(),
                            size,
                            MAX_SOURCE_FILE_BYTES
                        );
                        continue;
                    }

                    files.push(DiscoveryFile {
                        source_path: rel_path,
                        size_bytes: size,
                    });
                }
            }
        }
    }

    // Error if no files found but there were I/O errors (e.g., all files have permission issues)
    // This ensures we don't silently skip unreadable files without user feedback
    if files.is_empty() && skipped_io_error > 0 {
        anyhow::bail!(
            "No indexable files found after filtering (skipped {skipped_io_error} file(s) due to I/O errors). \
             Check file permissions: ensure source files are readable."
        );
    }

    if files.is_empty() && skipped_large > 0 {
        anyhow::bail!(
            "No indexable files found after filtering (skipped {skipped_large} oversized, {skipped_empty} empty, and {skipped_broken_symlink} broken symlink files). \
             Maximum supported file size is {MAX_SOURCE_FILE_BYTES} bytes."
        );
    }

    // Broken symlinks cause non-zero exit code as they indicate problems with the source
    if skipped_broken_symlink > 0 {
        anyhow::bail!(
            "Found {} broken symlink(s) in source directory. \
             Please fix or remove broken symlinks before indexing.",
            skipped_broken_symlink
        );
    }

    // Emit warning summary for I/O errors (permission denied, etc.) if any were found
    if skipped_io_error > 0 {
        eprintln!(
            "Warning: Skipped {skipped_io_error} path(s) due to I/O errors (e.g., permission denied). \
            Some files may not have been processed."
        );
    }

    // Handle permission denied files:
    // - If ALL files are unreadable (files is empty): fail with error
    // - If SOME files are unreadable but readable files exist: warn but continue
    if !permission_denied_files.is_empty() {
        let file_list = permission_denied_files.join(", ");
        if files.is_empty() {
            // All files are unreadable - hard fail
            anyhow::bail!(
                "Error: Cannot read {} file(s) due to permission denied: {}. \
                 Please check file permissions with 'chmod +r' or remove unreadable files.",
                permission_denied_files.len(),
                file_list
            );
        }
        // Some files are readable, some are not - warn but continue
        eprintln!(
            "Warning: Cannot read {} file(s) due to permission denied: {}. \
             These files will be skipped. To include them, run 'chmod +r' on these files.",
            permission_denied_files.len(),
            file_list
        );
    }

    let manifest = DiscoverManifest {
        source_dir: canonical_path.to_string_lossy().to_string(),
        discovered_at: chrono::Utc::now().to_rfc3339(),
        total_files: files.len(),
        files: files.clone(),
    };

    Ok((files, manifest))
}

/// Discover a single file (alternative to directory-based discovery)
///
/// Design by Contract:
/// - **Preconditions:**
///   - `file_path` exists and is a file
///   - extensions is a non-empty slice of supported extensions
/// - **Postconditions:**
///   - Returns Ok with (files, manifest)
///   - If file has supported extension: files contains one `DiscoveryFile`
///   - If file has unsupported extension: files is empty
///   - `manifest.source_dir` is the parent directory
/// - **Errors:**
///   - Returns error if metadata cannot be read
fn discover_single_file(
    file_path: &Path,
    extensions: &[&str],
) -> Result<(Vec<DiscoveryFile>, DiscoverManifest)> {
    let filename = match file_path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => anyhow::bail!("Invalid file path: {}", file_path.display()),
    };

    // Check if the file has a supported extension
    let has_supported_ext = file_path.extension().is_some_and(|ext| {
        let ext_str = format!(".{}", ext.to_string_lossy());
        extensions.contains(&ext_str.as_str())
    });

    // Text extensions that should trigger a warning (not true markdown)
    let text_extensions = [".txt"];

    let files = if has_supported_ext {
        let ext_str = file_path
            .extension()
            .map(|e| format!(".{}", e.to_string_lossy()))
            .unwrap_or_default();

        // Warn for non-markdown text files being processed as markdown
        if text_extensions.contains(&ext_str.as_str()) {
            eprintln!(
                "Warning: Processing non-markdown file '{filename}' with extension '{ext_str}'. \
                This file may not be valid markdown."
            );
        }

        let size = file_path
            .metadata()
            .context(format!(
                "Failed to read metadata for {}",
                file_path.display()
            ))?
            .len();

        if size == 0 {
            anyhow::bail!("Cannot index empty file: {}", file_path.display());
        }

        if size > MAX_SOURCE_FILE_BYTES {
            anyhow::bail!(
                "File too large to index safely: {} ({} bytes exceeds {} byte limit)",
                file_path.display(),
                size,
                MAX_SOURCE_FILE_BYTES
            );
        }

        vec![DiscoveryFile {
            source_path: filename,
            size_bytes: size,
        }]
    } else {
        Vec::new()
    };

    // Use parent directory as source_dir for manifest
    let source_dir = file_path
        .parent()
        .map_or_else(|| ".".to_string(), |p| p.to_string_lossy().to_string());

    let manifest = DiscoverManifest {
        source_dir,
        discovered_at: chrono::Utc::now().to_rfc3339(),
        total_files: files.len(),
        files: files.clone(),
    };

    Ok((files, manifest))
}

#[cfg(test)]
#[allow(clippy::panic)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;
    use tempfile::TempDir;

    /// Test that an unreadable directory causes discovery to FAIL with exit code 1.
    /// This is the P0 bug fix: unreadable files should not be silently skipped.
    #[test]
    fn test_unreadable_directory_returns_nonzero_exit() {
        // Create temp directory with ONLY an unreadable subdirectory (no readable files)
        let temp_dir = match TempDir::new() {
            Ok(d) => d,
            Err(e) => panic!("Failed to create temp dir: {e}"),
        };
        let dir_path = temp_dir.path();

        // Create a subdirectory with a file inside - but NO readable files at root
        let unreadable_dir = dir_path.join("restricted");
        match fs::create_dir(&unreadable_dir) {
            Ok(_) => (),
            Err(e) => panic!("Failed to create restricted dir: {e}"),
        };

        let file_in_dir = unreadable_dir.join("inside.md");
        match fs::write(&file_in_dir, "# Inside Restricted\nContent") {
            Ok(_) => (),
            Err(e) => panic!("Failed to create file in restricted dir: {e}"),
        };

        // Remove read+execute permissions from subdirectory (making it unreadable)
        // This prevents WalkDir from even entering the directory
        match fs::set_permissions(&unreadable_dir, PermissionsExt::from_mode(0o000)) {
            Ok(_) => (),
            Err(e) => panic!("Failed to set permissions: {e}"),
        };

        // Discover files - should FAIL because no readable files exist
        let result = discover_files(dir_path);

        // Clean up: restore permissions so temp dir can be removed
        let _ = fs::set_permissions(&unreadable_dir, PermissionsExt::from_mode(0o755));

        // Result should be Err (permission denied causes failure when no readable files)
        assert!(
            result.is_err(),
            "discover_files should FAIL when no readable files exist due to permission errors"
        );

        // Check error message mentions permission denied
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("permission denied"),
            "Error should mention 'permission denied', got: {}",
            err_msg
        );
    }

    /// Test that readable files work normally (happy path)
    #[test]
    fn test_discover_files_with_readable_files() {
        let temp_dir = match TempDir::new() {
            Ok(d) => d,
            Err(e) => panic!("Failed to create temp dir: {e}"),
        };
        let dir_path = temp_dir.path();

        // Create readable markdown files
        let file1 = dir_path.join("readable1.md");
        let file2 = dir_path.join("readable2.md");

        let mut f1 = match File::create(&file1) {
            Ok(f) => f,
            Err(e) => panic!("Failed to create file1: {e}"),
        };
        match f1.write_all(b"# Readable Document 1\nContent here") {
            Ok(_) => (),
            Err(e) => panic!("Failed to write file1: {e}"),
        }

        let mut f2 = match File::create(&file2) {
            Ok(f) => f,
            Err(e) => panic!("Failed to create file2: {e}"),
        };
        match f2.write_all(b"# Readable Document 2\nMore content") {
            Ok(_) => (),
            Err(e) => panic!("Failed to write file2: {e}"),
        }

        // Discover files - should succeed with readable files
        let result = discover_files(dir_path);

        assert!(
            result.is_ok(),
            "discover_files should succeed with readable files"
        );

        let (discovered_files, _manifest) = result.unwrap();

        assert_eq!(
            discovered_files.len(),
            2,
            "Expected 2 readable files to be discovered, got {}",
            discovered_files.len()
        );

        let file_names: Vec<_> = discovered_files
            .iter()
            .map(|f| f.source_path.clone())
            .collect();

        assert!(
            file_names.iter().any(|n| n.contains("readable1.md")),
            "readable1.md should be in discovered files"
        );
        assert!(
            file_names.iter().any(|n| n.contains("readable2.md")),
            "readable2.md should be in discovered files"
        );
    }

    /// Test basic file discovery functionality
    #[test]
    fn test_discover_files_basic() {
        let temp_dir = match TempDir::new() {
            Ok(d) => d,
            Err(e) => panic!("Failed to create temp dir: {e}"),
        };
        let dir_path = temp_dir.path();

        // Create test files
        let md_file = dir_path.join("test.md");
        let txt_file = dir_path.join("test.txt");
        let rst_file = dir_path.join("test.rst");
        let mdx_file_test = dir_path.join("test.mdx");
        let other_file = dir_path.join("test.html");

        match fs::write(&md_file, "# Markdown\n\ncontent\n") {
            Ok(_) => (),
            Err(e) => panic!("Failed to create md file: {e}"),
        }
        match fs::write(&txt_file, "plain text") {
            Ok(_) => (),
            Err(e) => panic!("Failed to create txt file: {e}"),
        }
        match fs::write(&rst_file, "Heading\n=======\n") {
            Ok(_) => (),
            Err(e) => panic!("Failed to create rst file: {e}"),
        }
        match fs::write(&mdx_file_test, "# MDX\n\n<Component />") {
            Ok(_) => (),
            Err(e) => panic!("Failed to create mdx file: {e}"),
        }
        match File::create(&other_file) {
            Ok(_) => (),
            Err(e) => panic!("Failed to create html file: {e}"),
        }

        let result = discover_files(dir_path);
        assert!(result.is_ok());

        let (files, _manifest) = match result {
            Ok(v) => v,
            Err(e) => panic!("discover_files failed: {e}"),
        };
        assert_eq!(files.len(), 4, "Should discover 4 supported files");
    }

    /// Test that empty directory returns empty file list (not error)
    #[test]
    fn test_discover_files_empty_directory() {
        let temp_dir = match TempDir::new() {
            Ok(d) => d,
            Err(e) => panic!("Failed to create temp dir: {e}"),
        };
        let dir_path = temp_dir.path();

        let result = discover_files(dir_path);
        assert!(result.is_ok());

        let (files, manifest) = match result {
            Ok(v) => v,
            Err(e) => panic!("discover_files failed: {e}"),
        };
        assert_eq!(files.len(), 0, "Empty directory should have 0 files");
        assert_eq!(manifest.total_files, 0, "Manifest should show 0 files");
    }

    /// Test discovery in nested directories
    #[test]
    fn test_discover_files_nested_directories() {
        let temp_dir = match TempDir::new() {
            Ok(d) => d,
            Err(e) => panic!("Failed to create temp dir: {e}"),
        };
        let dir_path = temp_dir.path();

        // Create nested structure
        let subdir = dir_path.join("subdir");
        match fs::create_dir(&subdir) {
            Ok(_) => (),
            Err(e) => panic!("Failed to create subdir: {e}"),
        }

        let root_file = dir_path.join("root.md");
        let sub_file = subdir.join("sub.md");

        match fs::write(&root_file, "# Root\n") {
            Ok(_) => (),
            Err(e) => panic!("Failed to create root file: {e}"),
        }
        match fs::write(&sub_file, "# Sub\n") {
            Ok(_) => (),
            Err(e) => panic!("Failed to create sub file: {e}"),
        }

        let result = discover_files(dir_path);
        assert!(result.is_ok());

        let (files, _manifest) = match result {
            Ok(v) => v,
            Err(e) => panic!("discover_files failed: {e}"),
        };
        assert_eq!(
            files.len(),
            2,
            "Should discover files in nested directories"
        );
    }

    /// Test that a single markdown file can be indexed directly
    /// This was P1 bug doc-tx-xpm: discover_files rejected single files
    #[test]
    fn test_discover_single_file() {
        let temp_dir = match TempDir::new() {
            Ok(d) => d,
            Err(e) => panic!("Failed to create temp dir: {e}"),
        };
        let dir_path = temp_dir.path();

        // Create a single markdown file
        let single_file = dir_path.join("single.md");
        let mut f = match File::create(&single_file) {
            Ok(f) => f,
            Err(e) => panic!("Failed to create single file: {e}"),
        };
        match f.write_all(b"# Single Document\n\nThis is a single file to index.") {
            Ok(_) => (),
            Err(e) => panic!("Failed to write single file: {e}"),
        }

        // Test: discover_files should accept a single file path
        let result = discover_files(&single_file);
        assert!(
            result.is_ok(),
            "discover_files should accept single file, got: {:?}",
            result.as_ref().map_err(|e| e.to_string())
        );

        let (files, manifest) = match result {
            Ok(v) => v,
            Err(e) => panic!("discover_files failed for single file: {e}"),
        };

        // Should discover exactly one file
        assert_eq!(files.len(), 1, "Should discover exactly 1 file");
        assert_eq!(manifest.total_files, 1, "Manifest should show 1 file");

        // The discovered file should be the single file itself
        let expected_name = single_file
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        assert_eq!(files[0].source_path, expected_name);
    }

    /// Test that single file discovery rejects unsupported file types
    #[test]
    fn test_discover_single_file_unsupported_type() {
        let temp_dir = match TempDir::new() {
            Ok(d) => d,
            Err(e) => panic!("Failed to create temp dir: {e}"),
        };
        let dir_path = temp_dir.path();

        // Create a single file with unsupported extension
        let unsupported_file = dir_path.join("data.json");
        match File::create(&unsupported_file) {
            Ok(_) => (),
            Err(e) => panic!("Failed to create unsupported file: {e}"),
        }

        // Should succeed but find no files (unsupported type)
        let result = discover_files(&unsupported_file);
        assert!(
            result.is_ok(),
            "discover_files should succeed even with unsupported file type"
        );

        let (files, _manifest) = match result {
            Ok(v) => v,
            Err(e) => panic!("discover_files failed: {e}"),
        };

        assert_eq!(
            files.len(),
            0,
            "Should discover 0 files for unsupported type"
        );
    }

    /// Test that single file discovery handles non-existent file
    #[test]
    fn test_discover_single_file_not_found() {
        let nonexistent = PathBuf::from("/nonexistent/path/file.md");
        let result = discover_files(&nonexistent);

        assert!(
            result.is_err(),
            "discover_files should error for non-existent file"
        );
        let err_msg = result.map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(
                msg.contains("not found"),
                "Error should mention 'not found'"
            );
        }
    }

    /// Test that excluded directories are skipped
    #[test]
    fn test_discover_files_excludes_directories() {
        let temp_dir = match TempDir::new() {
            Ok(d) => d,
            Err(e) => panic!("Failed to create temp dir: {e}"),
        };
        let dir_path = temp_dir.path();

        // Create directories that should be excluded
        let node_modules = dir_path.join("node_modules");
        let git_dir = dir_path.join(".git");
        let _build = dir_path.join("_build");
        let dist_dir = dir_path.join("dist");
        let vendor_dir = dir_path.join("vendor");

        match fs::create_dir(&node_modules) {
            Ok(_) => (),
            Err(e) => panic!("Failed to create node_modules: {e}"),
        }
        match fs::create_dir(&git_dir) {
            Ok(_) => (),
            Err(e) => panic!("Failed to create .git: {e}"),
        }
        match fs::create_dir(&_build) {
            Ok(_) => (),
            Err(e) => panic!("Failed to create _build: {e}"),
        }
        match fs::create_dir(&dist_dir) {
            Ok(_) => (),
            Err(e) => panic!("Failed to create dist: {e}"),
        }
        match fs::create_dir(&vendor_dir) {
            Ok(_) => (),
            Err(e) => panic!("Failed to create vendor: {e}"),
        }

        // Create files inside excluded directories
        let nm_file = node_modules.join("package.md");
        let git_file = git_dir.join("config.md");
        let build_file = _build.join("output.md");
        let dist_file = dist_dir.join("bundle.md");
        let vendor_file = vendor_dir.join("lib.md");

        match File::create(&nm_file) {
            Ok(_) => (),
            Err(e) => panic!("Failed to create nm file: {e}"),
        }
        match File::create(&git_file) {
            Ok(_) => (),
            Err(e) => panic!("Failed to create git file: {e}"),
        }
        match File::create(&build_file) {
            Ok(_) => (),
            Err(e) => panic!("Failed to create build file: {e}"),
        }
        match File::create(&dist_file) {
            Ok(_) => (),
            Err(e) => panic!("Failed to create dist file: {e}"),
        }
        match File::create(&vendor_file) {
            Ok(_) => (),
            Err(e) => panic!("Failed to create vendor file: {e}"),
        }

        // Create a file in root that should be found
        let root_file = dir_path.join("root.md");
        match fs::write(&root_file, "# Root\n") {
            Ok(_) => (),
            Err(e) => panic!("Failed to create root file: {e}"),
        }

        let result = discover_files(dir_path);
        assert!(result.is_ok());

        let (files, _manifest) = match result {
            Ok(v) => v,
            Err(e) => panic!("discover_files failed: {e}"),
        };
        assert_eq!(
            files.len(),
            1,
            "Should only find root file, not files in excluded directories"
        );
        assert!(
            files[0].source_path.contains("root.md"),
            "Found file should be root.md"
        );
    }

    #[test]
    fn test_discover_single_file_rejects_empty_file() {
        let temp_dir = match TempDir::new() {
            Ok(d) => d,
            Err(e) => panic!("Failed to create temp dir: {e}"),
        };
        let file = temp_dir.path().join("empty.md");
        match File::create(&file) {
            Ok(_) => (),
            Err(e) => panic!("Failed to create empty file: {e}"),
        }

        let result = discover_files(&file);
        assert!(result.is_err(), "Empty single file should be rejected");
    }

    #[test]
    fn test_discover_single_file_rejects_oversized_file() {
        let temp_dir = match TempDir::new() {
            Ok(d) => d,
            Err(e) => panic!("Failed to create temp dir: {e}"),
        };
        let file = temp_dir.path().join("huge.md");
        let f = match File::create(&file) {
            Ok(f) => f,
            Err(e) => panic!("Failed to create huge file: {e}"),
        };
        let new_len = super::MAX_SOURCE_FILE_BYTES.saturating_add(1);
        match f.set_len(new_len) {
            Ok(_) => (),
            Err(e) => panic!("Failed to set file length: {e}"),
        }

        let result = discover_files(&file);
        assert!(result.is_err(), "Oversized single file should be rejected");
    }

    /// Test that broken symlinks cause discovery to FAIL with non-zero exit code
    #[test]
    fn test_discover_files_fails_on_broken_symlinks() {
        let temp_dir = match TempDir::new() {
            Ok(d) => d,
            Err(e) => panic!("Failed to create temp dir: {e}"),
        };
        let dir_path = temp_dir.path();

        // Create a valid markdown file
        let valid_file = dir_path.join("valid.md");
        match fs::write(&valid_file, "# Valid Document\nContent here") {
            Ok(_) => (),
            Err(e) => panic!("Failed to create valid file: {e}"),
        }

        // Create a broken symlink (points to non-existent file)
        let broken_link = dir_path.join("broken-link.md");
        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            if let Err(e) = symlink("/nonexistent/target/file.md", &broken_link) {
                panic!("Failed to create broken symlink: {e}");
            }
        }

        // Discover files - should FAIL because broken symlinks cause non-zero exit
        let result = discover_files(dir_path);
        assert!(
            result.is_err(),
            "discover_files should FAIL when broken symlinks are found"
        );

        // Check error message mentions broken symlink
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("broken symlink"),
            "Error should mention 'broken symlink', got: {}",
            err_msg
        );
    }

    /// Test that valid symlinks (pointing to real files) are processed correctly
    #[test]
    fn test_discover_files_with_valid_symlink() {
        let temp_dir = match TempDir::new() {
            Ok(d) => d,
            Err(e) => panic!("Failed to create temp dir: {e}"),
        };
        let dir_path = temp_dir.path();

        let real_file = dir_path.join("real.md");
        match fs::write(&real_file, "# Real Document\nContent here") {
            Ok(_) => (),
            Err(e) => panic!("Failed to create real file: {e}"),
        }

        let valid_link = dir_path.join("link.md");
        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            if let Err(e) = symlink(&real_file, &valid_link) {
                panic!("Failed to create valid symlink: {e}");
            }
        }

        let result = discover_files(dir_path);
        assert!(
            result.is_ok(),
            "discover_files should succeed with valid symlinks, got: {:?}",
            result.as_ref().err()
        );

        let (discovered_files, _manifest) = match result {
            Ok(v) => v,
            Err(e) => panic!("discover_files failed: {e}"),
        };

        assert!(
            discovered_files.len() >= 1,
            "Should discover at least 1 file, got {}",
            discovered_files.len()
        );

        let file_names: Vec<_> = discovered_files
            .iter()
            .map(|f| f.source_path.clone())
            .collect();
        assert!(
            file_names.iter().any(|n| n.contains("real.md")),
            "Should find real.md, found: {:?}",
            file_names
        );
    }

    /// Test that symlink pointing to directory is processed correctly
    #[test]
    fn test_discover_files_with_symlink_to_directory() {
        let temp_dir = match TempDir::new() {
            Ok(d) => d,
            Err(e) => panic!("Failed to create temp dir: {e}"),
        };
        let dir_path = temp_dir.path();

        let real_dir = dir_path.join("realdir");
        let real_file_in_dir = real_dir.join("nested.md");
        match fs::create_dir(&real_dir) {
            Ok(_) => (),
            Err(e) => panic!("Failed to create dir: {e}"),
        };
        match fs::write(&real_file_in_dir, "# Nested\nContent") {
            Ok(_) => (),
            Err(e) => panic!("Failed to create nested file: {e}"),
        }

        let dir_link = dir_path.join("linkdir");
        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            if let Err(e) = symlink(&real_dir, &dir_link) {
                panic!("Failed to create dir symlink: {e}");
            }
        }

        let result = discover_files(dir_path);
        assert!(
            result.is_ok(),
            "discover_files should succeed with symlink to directory, got: {:?}",
            result.as_ref().err()
        );

        let (discovered_files, _manifest) = match result {
            Ok(v) => v,
            Err(e) => panic!("discover_files failed: {e}"),
        };

        let file_names: Vec<_> = discovered_files
            .iter()
            .map(|f| f.source_path.clone())
            .collect();
        assert!(
            file_names.iter().any(|n| n.contains("nested.md")),
            "Should find nested.md inside symlinked dir, found: {:?}",
            file_names
        );
    }

    /// Test that multiple broken symlinks are counted correctly in error
    #[test]
    fn test_discover_files_multiple_broken_symlinks() {
        let temp_dir = match TempDir::new() {
            Ok(d) => d,
            Err(e) => panic!("Failed to create temp dir: {e}"),
        };
        let dir_path = temp_dir.path();

        match fs::write(dir_path.join("good.md"), "# Good\nContent") {
            Ok(_) => (),
            Err(e) => panic!("Failed to create good file: {e}"),
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            let _ = symlink("/nonexistent1", dir_path.join("broken1.md"));
            let _ = symlink("/nonexistent2", dir_path.join("broken2.md"));
            let _ = symlink("/nonexistent3", dir_path.join("broken3.md"));
        }

        let result = discover_files(dir_path);
        assert!(
            result.is_err(),
            "Should fail with multiple broken symlinks"
        );

        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("3 broken symlink"),
            "Error should mention 3 broken symlinks, got: {}",
            err_msg
        );
    }

    /// Test that self-referential symlink (circular) is treated as broken
    #[test]
    fn test_discover_files_circular_symlink() {
        let temp_dir = match TempDir::new() {
            Ok(d) => d,
            Err(e) => panic!("Failed to create temp dir: {e}"),
        };
        let dir_path = temp_dir.path();

        match fs::write(dir_path.join("file.md"), "# Content\nText") {
            Ok(_) => (),
            Err(e) => panic!("Failed to create file: {e}"),
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            let link_path = dir_path.join("self.md");
            if let Err(e) = symlink(&link_path, &link_path) {
                panic!("Failed to create self-referential symlink: {e}");
            }
        }

        let result = discover_files(dir_path);
        assert!(
            result.is_err(),
            "Self-referential (circular) symlink should be treated as broken"
        );

        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("broken symlink"),
            "Error should mention broken symlink, got: {}",
            err_msg
        );
    }
}
