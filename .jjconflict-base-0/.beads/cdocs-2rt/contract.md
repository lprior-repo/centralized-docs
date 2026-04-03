# Contract Specification

## Bead Metadata

- **bead_id**: cdocs-2rt
- **bead_title**: calc: expose deterministic config hashing and add `compute_file_diff`
- **phase**: contract
- **created**: 2026-04-02
- **status**: draft

---

## Context

### Feature

Promote the private `compute_config_hash` helper from `analyze.rs` to a reusable
public API, and implement a new pure-function `compute_file_diff` that classifies
discovered files into four mutually-exclusive diff buckets by comparing their
on-disk content hashes and config hashes against previously-stored hashes.

### Domain Terms

| Term | Definition |
|------|-----------|
| `ContentHash` | SHA-256 digest newtype (`[u8; 32]`) defined in `cache::ContentHash`. Represents a deterministic fingerprint of byte content. |
| `config_hash` | `ContentHash` of the category-config file bytes (or `content_hash(b"")` when no config is provided). |
| `content_hash(file_bytes)` | `ContentHash` of raw file bytes on disk. |
| `composite_hash` | `ContentHash` derived from ordered concatenation of `[source_path, file_bytes, config_hash]`. Used as cache key. |
| `StoredHashes` | Record of previously-known `content_hash` and `config_hash` for a given path, loaded from cache or prior run. |
| `FileDiff` | Partition of all input paths into exactly four buckets: `unchanged`, `changed`, `new`, `deleted`. |
| `DiscoveryFile` | `{ source_path: String, size_bytes: u64 }` -- lightweight file descriptor from the discover phase. |
| `source_dir` | Root directory of documentation source tree. Files are resolved as `source_dir.join(&file.source_path)`. |

### Assumptions

1. `compute_config_hash` already exists as a private `fn` in `analyze.rs` (lines 263-271). Promotion means making it `pub` and relocating (or re-exporting) from the `cache` or `analyze` module.
2. `ContentHash`, `content_hash`, and `composite_hash` are already public in `crate::cache`.
3. The diff function is a **pure calculation** -- it reads file bytes from disk but performs **zero state writes** (no cache mutations, no file writes).
4. File bytes are read in parallel via `rayon`.
5. The caller supplies `stored_hashes: HashMap<String, StoredHashes>` representing previously-known state. If the caller has no prior state, they pass an empty map (all files become `new`).
6. Paths in `deleted` are those present in `stored_hashes` but absent from `discovered_files`.

### Open Questions

1. **Module placement**: Should `compute_file_diff` live in `analyze.rs` alongside `compute_config_hash`, or in a new `diff.rs` module? This contract assumes `analyze.rs` (co-located with existing config-hash usage) but the implementation may extract to a dedicated module if file-length limits require it.
2. **Parallelism granularity**: Should the rayon parallelism be configurable, or is `par_iter` on the file list sufficient? Contract assumes `par_iter` with no tunable.

---

## Type Definitions

```rust
use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::cache::ContentHash;
use crate::discover::DiscoveryFile;

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
    /// File exists on disk and both content_hash and config_hash match stored values.
    Unchanged,
    /// File exists on disk but content_hash or config_hash differs from stored values.
    Changed,
    /// File exists on disk but has no entry in stored_hashes.
    New,
    /// File exists in stored_hashes but was not present in discovered_files.
    Deleted,
}

/// The four diff buckets. Every input path appears in exactly one bucket.
/// Union of all buckets == union of discovered paths and stored-hashes paths.
/// Intersection of any two distinct buckets == empty set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileDiff {
    pub unchanged: HashSet<String>,
    pub changed: HashSet<String>,
    pub new: HashSet<String>,
    pub deleted: HashSet<String>,
}

/// Error type for diff computation.
#[derive(Debug, thiserror::Error)]
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
```

---

## Contract Signatures

### 1. `pub fn compute_config_hash`

Promoted from `analyze::compute_config_hash` (currently `fn`, becomes `pub fn`).

```rust
/// Compute a deterministic SHA-256 hash of the category config file contents.
///
/// Returns `content_hash(b"")` when `category_config_path` is `None`
/// or when the file cannot be read (e.g., deleted between runs).
///
/// This function is infallible: it never returns Err. A missing/unreadable
/// config is treated as "no config" (empty-byte hash).
#[must_use]
pub fn compute_config_hash(category_config_path: Option<&Path>) -> ContentHash;
```

### 2. `pub fn compute_file_diff`

```rust
/// Classify discovered files into unchanged, changed, new, and deleted buckets
/// by comparing on-disk content hashes and config hashes against stored state.
///
/// This function performs **no state writes**. It reads file bytes in parallel
/// (rayon) and returns a pure `FileDiff` partition.
///
/// # Arguments
///
/// * `discovered_files` -- Files found by the discover phase.
/// * `source_dir` -- Root directory; files resolved as `source_dir.join(source_path)`.
/// * `category_config_path` -- Optional category config; used to compute config_hash.
/// * `stored_hashes` -- Previously-known hashes per path. Empty map => all files are `new`.
///
/// # Errors
///
/// Returns `DiffError::SourceDirNotFound` if `source_dir` does not exist.
/// Returns `DiffError::FileRead` if any discovered file cannot be read.
/// Returns `DiffError::PathTraversal` if any source_path escapes source_dir.
pub fn compute_file_diff(
    discovered_files: &[DiscoveryFile],
    source_dir: &Path,
    category_config_path: Option<&Path>,
    stored_hashes: &HashMap<String, StoredHashes>,
) -> Result<FileDiff, DiffError>;
```

---

## Preconditions

| ID | Precondition | Enforced by |
|----|-------------|-------------|
| PRE-1 | `source_dir` must exist on the filesystem. | `compute_file_diff` checks at entry, returns `DiffError::SourceDirNotFound`. |
| PRE-2 | Every `DiscoveryFile::source_path` in `discovered_files` must resolve to a file that exists under `source_dir`. | `compute_file_diff` reads each file; returns `DiffError::FileRead` on failure. |
| PRE-3 | No `source_path` may traverse above `source_dir` (path traversal protection). | `compute_file_diff` canonicalizes and checks; returns `DiffError::PathTraversal`. |
| PRE-4 | `discovered_files` contains no duplicate `source_path` entries. | Caller responsibility. If violated, the last-read hash wins (no panic). |
| PRE-5 | `stored_hashes` keys are relative paths consistent with `DiscoveryFile::source_path`. | Caller responsibility. Mismatched key formats cause false `deleted`/`new` classifications but never panic. |
| PRE-6 | `compute_config_hash` accepts `None` or a valid `Path`. | Always satisfied by signature. |

---

## Postconditions

| ID | Postcondition | Verified by |
|----|-------------|-------------|
| POST-1 | `FileDiff` buckets are **mutually exclusive**: the intersection of any two buckets is the empty set. | Invariant test: pairwise intersection == empty. |
| POST-2 | `FileDiff` buckets are **collectively exhaustive**: `unchanged U changed U new` == set of all `discovered_files` source_paths; `deleted` == set of `stored_hashes` keys minus the discovered set. | Invariant test: union matches input sets. |
| POST-3 | A file is `Unchanged` if and only if: (a) it exists in `stored_hashes`, AND (b) the on-disk content hash matches `stored_hashes[path].content_hash`, AND (c) the current config hash matches `stored_hashes[path].config_hash`. | Property-based test. |
| POST-4 | A file is `Changed` if and only if: (a) it exists in `stored_hashes`, AND (b) either the content hash or config hash differs from stored values (but not both matching). | Property-based test. |
| POST-5 | A file is `New` if and only if: it does NOT exist in `stored_hashes` and IS in `discovered_files`. | Property-based test. |
| POST-6 | A file is `Deleted` if and only if: it exists in `stored_hashes` but is NOT in `discovered_files`. | Property-based test. |
| POST-7 | `compute_file_diff` performs **zero writes** to the filesystem, cache, or any mutable state. | Review + test (no side effects observable). |
| POST-8 | `compute_config_hash` returns the same `ContentHash` for the same input across calls (deterministic). | Property-based test. |
| POST-9 | `compute_config_hash(None)` always returns `content_hash(b"")`. | Unit test. |
| POST-10 | When `stored_hashes` is empty, all discovered files are classified as `New` and `deleted` is empty. | Unit test. |
| POST-11 | When `discovered_files` is empty, all `stored_hashes` keys are classified as `Deleted` and the other three buckets are empty. | Unit test. |

---

## Invariants

| ID | Invariant | Scope |
|----|----------|-------|
| INV-1 | **Partition invariant**: Every path in the union of `discovered_files` source_paths and `stored_hashes` keys appears in exactly one `FileDiff` bucket. | `compute_file_diff` |
| INV-2 | **Determinism invariant**: Given identical inputs (same file bytes, same config bytes, same stored_hashes), `compute_file_diff` always produces identical `FileDiff` output. | `compute_file_diff` |
| INV-3 | **Hash determinism invariant**: `compute_config_hash` is a pure function of its input path's bytes. Same bytes => same `ContentHash`, always. | `compute_config_hash` |
| INV-4 | **No-mutation invariant**: `compute_file_diff` never mutates its inputs (`discovered_files`, `stored_hashes`) nor writes to disk or cache. | `compute_file_diff` |
| INV-5 | **Empty-input-total invariant**: Both empty `discovered_files` and empty `stored_hashes` are valid inputs that produce well-defined (possibly all-empty) `FileDiff` results. | `compute_file_diff` |
| INV-6 | **Single-bucket membership**: No path string appears in more than one `HashSet` within `FileDiff`. | `FileDiff` |

---

## Error Taxonomy

| Variant | Trigger | Recovery |
|---------|---------|----------|
| `DiffError::SourceDirNotFound` | `source_dir` does not exist on the filesystem. | Caller should validate path or create directory before retry. |
| `DiffError::FileRead { path, source }` | `fs::read(source_dir.join(path))` fails for any discovered file. Possible causes: permission denied, file deleted between discovery and diff, I/O error. | Caller may retry, skip the file, or report to user. The entire diff fails (no partial result). |
| `DiffError::PathTraversal { path }` | A `source_path` component resolves to a location outside `source_dir` after canonicalization (e.g., `../../etc/passwd`). | Caller should sanitize input paths. Indicates a bug in discovery or malicious input. |

**Design note**: `compute_config_hash` is intentionally infallible. A missing or unreadable config file is treated as "no config" (empty hash). This matches the existing behavior in `analyze_files_cached` where a read failure silently falls back to `content_hash(b"")`.

---

## Non-goals

1. **Partial diff results**: This contract does NOT support returning a partial diff when some files fail to read. A single read failure fails the entire operation. Future iterations may add a best-effort mode with `FailedFile` reporting.
2. **Config file validation**: `compute_config_hash` does not validate the config contents -- it hashes raw bytes only. Validation remains the responsibility of `CategoryConfig::load_from_file`.
3. **File metadata diffing**: This contract only considers byte-level content hash changes. Metadata changes (e.g., file permissions, timestamps) are NOT detected.
4. **Directory-level diffing**: Only files are diffed, not directory structure changes.
5. **Concurrency control**: No locking or synchronization is provided. The caller must ensure `source_dir` is not mutated during diff computation.
