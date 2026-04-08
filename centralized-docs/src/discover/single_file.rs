use anyhow::{Context, Result};
use std::path::Path;

use super::{DiscoverConfig, DiscoverManifest, DiscoveryFile};

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
pub(super) fn discover_single_file(
    file_path: &Path,
    extensions: &[&str],
    config: &DiscoverConfig,
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
            .map_or_else(String::new, |e| format!(".{}", e.to_string_lossy()));

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

        if size > config.max_file_bytes {
            anyhow::bail!(
                "File too large to index safely: {} ({} bytes exceeds {} byte limit)",
                file_path.display(),
                size,
                config.max_file_bytes
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
