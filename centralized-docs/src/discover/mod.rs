use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

mod single_file;
mod validation;
mod walker;

#[cfg(test)]
mod tests_basic;
#[cfg(test)]
mod tests_permissions;
#[cfg(test)]
mod tests_symlinks;

/// Default maximum file size in bytes (50MB)
const DEFAULT_MAX_FILE_BYTES: u64 = 50 * 1024 * 1024;

/// Configuration for file discovery
#[derive(Debug, Clone)]
pub struct DiscoverConfig {
    /// Maximum file size in bytes (files larger than this are skipped)
    pub max_file_bytes: u64,
    /// Directories to exclude from discovery
    pub exclude_dirs: Vec<String>,
}

impl Default for DiscoverConfig {
    fn default() -> Self {
        Self {
            max_file_bytes: DEFAULT_MAX_FILE_BYTES,
            exclude_dirs: vec![
                "node_modules".to_string(),
                ".git".to_string(),
                "_build".to_string(),
                "dist".to_string(),
                "vendor".to_string(),
            ],
        }
    }
}

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
pub fn discover_files(
    source_dir: &Path,
    path_filter: Option<&str>,
) -> Result<(Vec<DiscoveryFile>, DiscoverManifest)> {
    discover_files_with_config(source_dir, path_filter, &DiscoverConfig::default())
}

/// Discover all markdown and text files with custom configuration.
///
/// This version allows runtime configuration of max file size and excluded directories.
#[allow(clippy::too_many_lines)]
pub fn discover_files_with_config(
    source_dir: &Path,
    path_filter: Option<&str>,
    config: &DiscoverConfig,
) -> Result<(Vec<DiscoveryFile>, DiscoverManifest)> {
    walker::discover_files_with_config(source_dir, path_filter, config)
}
