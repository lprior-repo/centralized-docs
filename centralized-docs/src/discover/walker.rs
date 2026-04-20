use anyhow::{Context, Result};
use std::path::Path;
use walkdir::WalkDir;

use super::validation::validate_discovery_results;
use super::{DiscoverConfig, DiscoverManifest, DiscoveryFile};

/// Discover all markdown and text files with custom configuration.
///
/// Internal implementation in walker submodule to keep mod.rs under 300 lines.
#[allow(clippy::too_many_lines)]
pub(super) fn discover_files_with_config(
    source_dir: &Path,
    path_filter: Option<&str>,
    config: &DiscoverConfig,
) -> Result<(Vec<DiscoveryFile>, DiscoverManifest)> {
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
    // Use config exclude_dirs (or default)
    let exclude_dirs: Vec<&str> = config.exclude_dirs.iter().map(String::as_str).collect();

    // Handle single file case directly
    if canonical_path.is_file() {
        return super::single_file::discover_single_file(&canonical_path, &all_exts, config);
    }

    // Compile regex once before the walk — avoids recompiling per file
    let compiled_regex: Option<Result<regex::Regex, regex::Error>> =
        path_filter.map(regex::Regex::new);

    enum DiscoveryEvent {
        File(DiscoveryFile),
        SkippedLarge,
        SkippedEmpty,
        SkippedBrokenSymlink,
        SkippedIoError,
        PermissionDenied(String),
        None,
    }

    let events: Vec<DiscoveryEvent> = WalkDir::new(&canonical_path)
        .into_iter()
        .map(|entry_res| {
            let entry = match entry_res {
                Ok(e) => e,
                Err(e) => {
                    let is_permission_denied = e
                        .io_error()
                        .is_some_and(|io_err| io_err.kind() == std::io::ErrorKind::PermissionDenied);
                    if is_permission_denied {
                        let path_str = e
                            .path()
                            .map_or_else(|| "<unknown>".to_string(), |p| p.display().to_string());
                        eprintln!("Error: Cannot read file '{path_str}': permission denied");
                        return DiscoveryEvent::PermissionDenied(path_str);
                    }
                    eprintln!("Warning: Skipping path due to I/O error: {e}");
                    return DiscoveryEvent::SkippedIoError;
                }
            };

            let path = entry.path();

            // Skip excluded directories (exact match on directory name)
            if path.components().any(|c| {
                exclude_dirs
                    .iter()
                    .any(|excl| c.as_os_str().to_string_lossy() == *excl)
            }) {
                return DiscoveryEvent::None;
            }

            // Check for broken symlinks before file type check
            if entry.file_type().is_symlink() && std::fs::metadata(path).is_err() {
                let symlink_name = path.file_name().map_or_else(
                    || "unknown".to_string(),
                    |n| n.to_string_lossy().to_string(),
                );
                eprintln!(
                    "Warning: Skipping broken symlink '{symlink_name}' (target does not exist)"
                );
                return DiscoveryEvent::SkippedBrokenSymlink;
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
                        let rel_path = match path.strip_prefix(&canonical_path) {
                            Ok(p) => p.to_string_lossy().to_string(),
                            Err(e) => {
                                eprintln!(
                                    "Warning: Failed to get relative path for {}: {e}",
                                    path.display()
                                );
                                return DiscoveryEvent::None;
                            }
                        };

                        if let Some(regex_result) = compiled_regex.as_ref() {
                            match regex_result {
                                Ok(regex) => {
                                    if !regex.is_match(&rel_path) {
                                        return DiscoveryEvent::None;
                                    }
                                }
                                Err(e) => {
                                    eprintln!(
                                        "Warning: Invalid regex pattern '{}': {}. Path filter ignored.",
                                        path_filter.map_or("", |s| s),
                                        e
                                    );
                                }
                            }
                        }

                        let size = match path.metadata() {
                            Ok(meta) => meta.len(),
                            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                                let path_str = path.display().to_string();
                                eprintln!("Error: Cannot read file '{path_str}': permission denied");
                                return DiscoveryEvent::PermissionDenied(path_str);
                            }
                            Err(e) => {
                                eprintln!(
                                    "Warning: Failed to read metadata for {}: {e}, skipping file",
                                    path.display()
                                );
                                return DiscoveryEvent::SkippedIoError;
                            }
                        };

                        if size == 0 {
                            eprintln!("Error: Skipping empty file {}", path.display());
                            return DiscoveryEvent::SkippedEmpty;
                        }

                        if size > config.max_file_bytes {
                            eprintln!(
                                "Warning: Skipping oversized file {} ({} bytes exceeds {} byte limit)",
                                path.display(),
                                size,
                                config.max_file_bytes
                            );
                            return DiscoveryEvent::SkippedLarge;
                        }

                        return DiscoveryEvent::File(DiscoveryFile {
                            source_path: rel_path,
                            size_bytes: size,
                        });
                    }
                }
            }
            DiscoveryEvent::None
        })
        .collect();

    let files: Vec<DiscoveryFile> = events
        .iter()
        .filter_map(|e| {
            if let DiscoveryEvent::File(f) = e {
                Some(f.clone())
            } else {
                None
            }
        })
        .collect();

    let skipped_large = events
        .iter()
        .filter(|e| matches!(e, DiscoveryEvent::SkippedLarge))
        .count();
    let skipped_empty = events
        .iter()
        .filter(|e| matches!(e, DiscoveryEvent::SkippedEmpty))
        .count();
    let skipped_broken_symlink = events
        .iter()
        .filter(|e| matches!(e, DiscoveryEvent::SkippedBrokenSymlink))
        .count();
    let skipped_io_error = events
        .iter()
        .filter(|e| matches!(e, DiscoveryEvent::SkippedIoError))
        .count();
    let permission_denied_files: Vec<String> = events
        .iter()
        .filter_map(|e| {
            if let DiscoveryEvent::PermissionDenied(p) = e {
                Some(p.clone())
            } else {
                None
            }
        })
        .collect();

    validate_discovery_results(
        &files,
        skipped_large,
        skipped_empty,
        skipped_broken_symlink,
        skipped_io_error,
        &permission_denied_files,
        config.max_file_bytes,
    )?;

    let manifest = DiscoverManifest {
        source_dir: canonical_path.to_string_lossy().to_string(),
        discovered_at: chrono::Utc::now().to_rfc3339(),
        total_files: files.len(),
        files: files.clone(),
    };

    Ok((files, manifest))
}
