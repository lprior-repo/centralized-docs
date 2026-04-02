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

    Ok(FileDiff {
        unchanged,
        changed,
        new: new_files,
        deleted,
    })
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
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::field_reassign_with_default
)]
mod tests {
    use super::*;
    use std::fs;

    // === Scenario 3.1: compute_config_hash returns empty hash for None ===
    #[test]
    fn compute_config_hash_returns_empty_hash_when_none() {
        let result = compute_config_hash(None);
        assert_eq!(
            result,
            content_hash(b""),
            "None input must yield empty hash"
        );
    }

    // === Scenario 3.2: compute_config_hash returns SHA-256 of file bytes ===
    #[test]
    fn compute_config_hash_returns_sha256_when_file_readable() {
        let dir = tempfile::TempDir::new().expect("tempdir");
        let file_path = dir.path().join("config.yaml");
        fs::write(&file_path, b"hello world").expect("write");

        let result = compute_config_hash(Some(&file_path));
        assert_eq!(
            result,
            content_hash(b"hello world"),
            "must match SHA-256 of file bytes"
        );
    }

    // === Scenario 3.3: compute_config_hash returns empty hash for missing file ===
    #[test]
    fn compute_config_hash_returns_empty_hash_when_file_missing() {
        let dir = tempfile::TempDir::new().expect("tempdir");
        let missing = dir.path().join("nonexistent.yaml");

        let result = compute_config_hash(Some(&missing));
        assert_eq!(
            result,
            content_hash(b""),
            "missing file must yield empty hash"
        );
    }

    // === Scenario 3.4: compute_config_hash returns empty hash for unreadable file ===
    #[test]
    fn compute_config_hash_returns_empty_hash_when_file_unreadable() {
        let dir = tempfile::TempDir::new().expect("tempdir");
        let file_path = dir.path().join("secret.yaml");
        fs::write(&file_path, b"secret").expect("write");

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&file_path, PermissionsExt::from_mode(0o000)).expect("chmod");
        }

        let result = compute_config_hash(Some(&file_path));
        assert_eq!(
            result,
            content_hash(b""),
            "unreadable file must yield empty hash"
        );

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&file_path, PermissionsExt::from_mode(0o644));
        }
    }

    // === Scenario 3.5: compute_config_hash is deterministic ===
    #[test]
    fn compute_config_hash_returns_identical_hash_across_calls() {
        let dir = tempfile::TempDir::new().expect("tempdir");
        let file_path = dir.path().join("deterministic.yaml");
        fs::write(&file_path, b"deterministic test content").expect("write");

        let hash1 = compute_config_hash(Some(&file_path));
        let hash2 = compute_config_hash(Some(&file_path));

        assert_eq!(hash1, hash2, "same input must yield identical hash");
        assert_eq!(hash1, content_hash(b"deterministic test content"));
    }

    // === Scenario 3.6: compute_config_hash returns distinct hashes ===
    #[test]
    fn compute_config_hash_returns_distinct_concrete_hashes_for_different_contents() {
        let dir = tempfile::TempDir::new().expect("tempdir");
        let file_a = dir.path().join("a.yaml");
        let file_b = dir.path().join("b.yaml");
        fs::write(&file_a, b"aaa").expect("write a");
        fs::write(&file_b, b"bbb").expect("write b");

        let hash_a = compute_config_hash(Some(&file_a));
        let hash_b = compute_config_hash(Some(&file_b));

        assert_eq!(hash_a, content_hash(b"aaa"));
        assert_eq!(hash_b, content_hash(b"bbb"));
        assert_ne!(
            hash_a, hash_b,
            "different content must yield different hashes"
        );
    }

    // === Scenario 3.7: compute_config_hash returns empty hash for 0-byte file ===
    #[test]
    fn compute_config_hash_returns_empty_hash_when_file_is_zero_bytes() {
        let dir = tempfile::TempDir::new().expect("tempdir");
        let file_path = dir.path().join("empty.yaml");
        fs::write(&file_path, b"").expect("write empty");

        let result = compute_config_hash(Some(&file_path));
        assert_eq!(
            result,
            content_hash(b""),
            "0-byte file must yield empty hash"
        );
    }

    // === Scenario 3.8: compute_config_hash handles large file ===
    #[test]
    fn compute_config_hash_returns_exact_sha256_when_file_is_large() {
        let dir = tempfile::TempDir::new().expect("tempdir");
        let file_path = dir.path().join("large.yaml");
        let large_content = vec![b'X'; 1_048_576]; // 1MB
        fs::write(&file_path, &large_content).expect("write large");

        let result = compute_config_hash(Some(&file_path));
        assert_eq!(
            result,
            content_hash(&large_content),
            "large file hash must match"
        );
    }
}
