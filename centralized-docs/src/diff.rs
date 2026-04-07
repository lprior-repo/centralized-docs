//! Deterministic file diff computation.
//!
//! Provides pure functions for computing configuration hashes and classifying
//! discovered files into unchanged/changed/new/deleted buckets.
//!
//! # Contract: cdocs-2rt
//!
//! - `compute_config_hash` — promoted from private `analyze::compute_config_hash`.
//! - `compute_file_diff` — classifies files via rayon-parallel hashing.
//! - Zero writes to filesystem, cache, or mutable state.
//! - All fallible operations return `Result<T, DiffError>`.

use std::collections::{HashMap, HashSet};
use std::path::{Component, Path};

use rayon::prelude::*;
use thiserror::Error;

use crate::cache::{content_hash, ContentHash};
use crate::discover::DiscoveryFile;

// ---------------------------------------------------------------------------
// Type Definitions
// ---------------------------------------------------------------------------

/// Previously-stored hashes for a single file path.
///
/// Invariant: `content_hash` is the SHA-256 of the file bytes at the time
/// of last indexing. `config_hash` is the SHA-256 of the config file at
/// that same time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredHashes {
    pub content_hash: ContentHash,
    pub config_hash: ContentHash,
}

/// Mutually-exclusive classification of a file's diff status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffStatus {
    /// File exists on disk and both `content_hash` and `config_hash` match stored values.
    Unchanged,
    /// File exists on disk but `content_hash` or `config_hash` differs from stored values.
    Changed,
    /// File exists on disk but has no entry in `stored_hashes`.
    New,
    /// File exists in `stored_hashes` but was not present in `discovered_files`.
    Deleted,
}

/// The four diff buckets. Every input path appears in exactly one bucket.
/// Union of all buckets == union of discovered paths and stored-hashes paths.
/// Intersection of any two distinct buckets == empty set.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FileDiff {
    pub unchanged: HashSet<String>,
    pub changed: HashSet<String>,
    pub new: HashSet<String>,
    pub deleted: HashSet<String>,
}

impl FileDiff {
    /// Validate the 4-Set mutual exclusivity invariant.
    ///
    /// Asserts that every path appears in exactly one bucket. Returns an error
    /// if any path is found in two or more buckets simultaneously.
    pub fn validate(&self) -> Result<(), DiffError> {
        let all_paths: Vec<&String> = self
            .unchanged
            .iter()
            .chain(self.changed.iter())
            .chain(self.new.iter())
            .chain(self.deleted.iter())
            .collect();

        let mut seen: HashSet<&str> = HashSet::new();
        for path in &all_paths {
            if !seen.insert(path.as_str()) {
                return Err(DiffError::PathTraversal {
                    path: format!(
                        "FileDiff invariant violated: '{path}' appears in multiple buckets"
                    ),
                });
            }
        }
        Ok(())
    }
}

/// Error type for diff computation.
#[derive(Debug, Error)]
pub enum DiffError {
    /// A discovered file could not be read from disk.
    #[error("failed to read file '{path}': {source}")]
    FileRead {
        path: String,
        source: std::io::Error,
    },

    /// The source directory does not exist or is not a directory.
    #[error("source directory does not exist: {0}")]
    SourceDirNotFound(String),

    /// A discovered path resolved to a location outside the source directory.
    #[error("path traversal detected: '{path}' escapes source directory")]
    PathTraversal { path: String },
}

// ---------------------------------------------------------------------------
// Pure Functions (Calculations Layer)
// ---------------------------------------------------------------------------

/// Compute a deterministic SHA-256 hash of the category config file contents.
///
/// **I/O boundary function**: reads file bytes from disk.
///
/// Returns `content_hash(b"")` when `category_config_path` is `None`
/// or when the file cannot be read (e.g., deleted between runs).
///
/// This function is infallible: it never returns Err. A missing/unreadable
/// config is treated as "no config" (empty-byte hash).
#[must_use]
pub fn compute_config_hash(category_config_path: Option<&Path>) -> ContentHash {
    category_config_path
        .and_then(|path| std::fs::read(path).ok())
        .map_or_else(|| content_hash(b""), |bytes| content_hash(&bytes))
}

/// Classify discovered files into unchanged, changed, new, and deleted buckets
/// by comparing on-disk content hashes and config hashes against stored state.
///
/// This function performs **no state writes**. It reads file bytes in parallel
/// (rayon) and returns a pure `FileDiff` partition.
///
/// # Errors
///
/// Returns `DiffError::SourceDirNotFound` if `source_dir` does not exist.
/// Returns `DiffError::FileRead` if any discovered file cannot be read.
/// Returns `DiffError::PathTraversal` if any `source_path` escapes `source_dir`.
pub fn compute_file_diff(
    discovered_files: &[DiscoveryFile],
    source_dir: &Path,
    category_config_path: Option<&Path>,
    stored_hashes: &HashMap<String, StoredHashes>,
) -> Result<FileDiff, DiffError> {
    // PRE-1: source_dir must exist
    let canonical_source = source_dir
        .canonicalize()
        .map_err(|_| DiffError::SourceDirNotFound(source_dir.to_string_lossy().to_string()))?;

    let config_hash = compute_config_hash(category_config_path);

    // Build set of discovered paths for deleted calculation
    let discovered_paths: HashSet<String> = discovered_files
        .iter()
        .map(|f| f.source_path.clone())
        .collect();

    // Parallel file hashing and classification
    let results: Vec<(String, DiffStatus)> = discovered_files
        .par_iter()
        .map(|file| {
            classify_file(file, &canonical_source, &config_hash, stored_hashes)
                .map(|status| (file.source_path.clone(), status))
        })
        .collect::<Result<Vec<_>, DiffError>>()?;

    // Partition into buckets via fold (single pass, no intermediate allocations)
    let (unchanged, changed, new_files) = results.into_iter().fold(
        (HashSet::new(), HashSet::new(), HashSet::new()),
        |(mut u, mut c, mut n), (path, status)| {
            match status {
                DiffStatus::Unchanged => {
                    u.insert(path);
                }
                DiffStatus::Changed => {
                    c.insert(path);
                }
                DiffStatus::New => {
                    n.insert(path);
                }
                DiffStatus::Deleted => {}
            }
            (u, c, n)
        },
    );

    // Deleted: in stored_hashes but not in discovered
    let deleted: HashSet<String> = stored_hashes
        .keys()
        .filter(|k| !discovered_paths.contains(*k))
        .cloned()
        .collect();

    let diff = FileDiff {
        unchanged,
        changed,
        new: new_files,
        deleted,
    };

    diff.validate()?;

    Ok(diff)
}

// ---------------------------------------------------------------------------
// Internal Helpers
// ---------------------------------------------------------------------------

/// Validate path safety and classify a single discovered file.
fn classify_file(
    file: &DiscoveryFile,
    canonical_source: &Path,
    config_hash: &ContentHash,
    stored_hashes: &HashMap<String, StoredHashes>,
) -> Result<DiffStatus, DiffError> {
    // PRE-3: Path traversal check
    validate_path_safety(&file.source_path, canonical_source)?;

    // Read file bytes
    let full_path = canonical_source.join(&file.source_path);
    let file_bytes = std::fs::read(&full_path).map_err(|e| DiffError::FileRead {
        path: file.source_path.clone(),
        source: e,
    })?;

    let current_content_hash = content_hash(&file_bytes);

    Ok(match stored_hashes.get(&file.source_path) {
        None => DiffStatus::New,
        Some(stored) => {
            if stored.content_hash == current_content_hash && stored.config_hash == *config_hash {
                DiffStatus::Unchanged
            } else {
                DiffStatus::Changed
            }
        }
    })
}

/// Check if a `source_path` would resolve to a location outside the source directory.
///
/// Strategy:
/// 1. Reject absolute paths immediately.
/// 2. Try canonicalize (works for existing files + symlinks) and check `starts_with`.
/// 3. For non-existent paths, check if `..` components would escape the root.
fn validate_path_safety(source_path: &str, canonical_source: &Path) -> Result<(), DiffError> {
    let path = Path::new(source_path);

    // Reject absolute paths
    if path.is_absolute() {
        return Err(DiffError::PathTraversal {
            path: source_path.to_string(),
        });
    }

    // Try canonicalize for existing files (catches symlink traversal)
    let full_path = canonical_source.join(source_path);
    if let Ok(canonical) = full_path.canonicalize() {
        if !canonical.starts_with(canonical_source) {
            return Err(DiffError::PathTraversal {
                path: source_path.to_string(),
            });
        }
        return Ok(());
    }

    // For non-existent paths, check component depth to detect `..` escaping
    let mut depth: u32 = 0;
    for component in path.components() {
        match component {
            Component::ParentDir => {
                depth = depth
                    .checked_sub(1)
                    .ok_or_else(|| DiffError::PathTraversal {
                        path: source_path.to_string(),
                    })?;
            }
            Component::Normal(_) => {
                depth = depth.saturating_add(1);
            }
            Component::CurDir => {}
            Component::RootDir | Component::Prefix(_) => {
                return Err(DiffError::PathTraversal {
                    path: source_path.to_string(),
                });
            }
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests (cfg(test))
// ---------------------------------------------------------------------------

#[cfg(test)]
#[path = "diff_tests.rs"]
mod tests;
