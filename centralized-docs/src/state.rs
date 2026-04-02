//! `StateDb` — in-memory accumulator for index-state mutations.
//!
//! Holds a `StateBatch` that is populated progressively as pipeline
//! stages succeed. Only flushes to durable storage on explicit
//! `commit_changes` invocation.

#![allow(unexpected_cfgs)]

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Unique identifier for a single `run_index` invocation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RunId(pub String);

impl std::fmt::Display for RunId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Accumulated state mutations from a single pipeline run.
///
/// Populated progressively by each pipeline stage. Committed atomically
/// via `StateDb::commit_changes` only on successful completion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateBatch {
    pub run_id: RunId,
    pub source_path: String,
    pub output_path: String,
    pub document_count: usize,
    pub chunk_count: usize,
    pub file_hashes: Vec<FileHashRecord>,
    pub created_at_unix_secs: u64,
}

/// A single file's content hash, used for incremental rebuild detection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileHashRecord {
    pub relative_path: String,
    pub content_hash: String,
}

/// Errors specific to state database operations.
#[non_exhaustive]
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum StateError {
    /// Attempted to commit a batch that has already been committed.
    #[error("state batch already committed for run {run_id}")]
    AlreadyCommitted { run_id: RunId },

    /// Attempted to mutate state after the batch has been committed.
    #[error("cannot mutate committed state for run {run_id}")]
    MutationAfterCommit { run_id: RunId },

    /// The batch is empty when commit is called (no documents processed).
    #[error("cannot commit empty state batch for run {run_id}")]
    EmptyBatch { run_id: RunId },

    /// Duplicate file path detected in the batch.
    #[error("duplicate file path in state batch: {path}")]
    DuplicateFilePath { path: String },

    /// Failed to write state to durable storage.
    #[error("failed to persist state batch for run {run_id}: {reason}")]
    PersistenceFailed { run_id: RunId, reason: String },

    /// The output directory is not writable or does not exist.
    #[error("output directory not accessible: {path}")]
    OutputNotAccessible { path: String },

    /// Serialization of the state batch failed.
    #[error("failed to serialize state batch: {reason}")]
    SerializationFailed { reason: String },

    /// A precondition was violated (e.g., no `OutputLock` held).
    #[error("precondition violated: {detail}")]
    PreconditionViolation { detail: String },
}

/// Monotonic counter guaranteeing unique [`RunId`] values within a process.
static RUN_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Generate a unique [`RunId`] combining wall-clock seconds with a monotonic counter.
fn generate_run_id() -> RunId {
    let counter = RUN_COUNTER.fetch_add(1, Ordering::Relaxed);
    let secs = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(_) => 0,
    };
    RunId(format!("run-{secs}-{counter}"))
}

/// The state database handle. Holds an in-memory batch that is NOT
/// written until `commit_changes` is called.
#[derive(Debug)]
pub struct StateDb {
    batch: Option<StateBatch>,
    output_dir: PathBuf,
    committed: bool,
}

impl StateDb {
    /// Create a new `StateDb` bound to the given output directory.
    ///
    /// # Preconditions
    /// - P-01: `output_dir` exists and is writable
    ///
    /// # Postconditions
    /// - POST-02 (initial): batch is initialised empty, committed is false
    #[allow(clippy::new_ret_no_self)]
    pub fn new(output_dir: &Path) -> Result<Self, StateError> {
        let path_str = output_dir.to_string_lossy().to_string();

        // P-01 validation: reject empty path
        if path_str.is_empty() {
            return Err(StateError::OutputNotAccessible { path: path_str });
        }

        // P-01 validation: must be an existing, accessible directory
        // (handles: nonexistent, regular file, dangling symlink)
        if !output_dir.is_dir() {
            return Err(StateError::OutputNotAccessible { path: path_str });
        }

        // P-01 validation: directory must be writable (probe write)
        let probe = output_dir.join(".ctd-state-probe");
        match std::fs::File::create(&probe) {
            Ok(_) => {
                let _ = std::fs::remove_file(&probe);
            }
            Err(_) => {
                return Err(StateError::OutputNotAccessible { path: path_str });
            }
        }

        let run_id = generate_run_id();
        let created_at = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(d) => d.as_secs(),
            Err(_) => 0,
        };

        Ok(Self {
            batch: Some(StateBatch {
                run_id,
                source_path: String::new(),
                output_path: path_str,
                document_count: 0,
                chunk_count: 0,
                file_hashes: Vec::new(),
                created_at_unix_secs: created_at,
            }),
            output_dir: output_dir.to_path_buf(),
            committed: false,
        })
    }

    /// Record a file's content hash in the pending batch.
    ///
    /// # Errors
    /// - `StateError::MutationAfterCommit` if already committed (INV-02)
    /// - `StateError::DuplicateFilePath` if `relative_path` already recorded (INV-04)
    pub fn record_file_hash(
        &mut self,
        relative_path: &str,
        content_hash: &str,
    ) -> Result<(), StateError> {
        self.ensure_uncommitted()?;

        let batch = match self.batch.as_mut() {
            Some(b) => b,
            None => {
                return Err(StateError::PreconditionViolation {
                    detail: String::from("batch is None"),
                })
            }
        };

        // INV-04: no duplicate relative_path entries
        if batch
            .file_hashes
            .iter()
            .any(|h| h.relative_path == relative_path)
        {
            return Err(StateError::DuplicateFilePath {
                path: relative_path.to_string(),
            });
        }

        batch.file_hashes.push(FileHashRecord {
            relative_path: relative_path.to_string(),
            content_hash: content_hash.to_string(),
        });

        Ok(())
    }

    /// Set the document count in the pending batch.
    ///
    /// # Errors
    /// - `StateError::MutationAfterCommit` if already committed (INV-02)
    pub fn set_document_count(&mut self, count: usize) -> Result<(), StateError> {
        self.ensure_uncommitted()?;

        let batch = match self.batch.as_mut() {
            Some(b) => b,
            None => {
                return Err(StateError::PreconditionViolation {
                    detail: String::from("batch is None"),
                })
            }
        };

        batch.document_count = count;
        Ok(())
    }

    /// Set the chunk count in the pending batch.
    ///
    /// # Errors
    /// - `StateError::MutationAfterCommit` if already committed (INV-02)
    pub fn set_chunk_count(&mut self, count: usize) -> Result<(), StateError> {
        self.ensure_uncommitted()?;

        let batch = match self.batch.as_mut() {
            Some(b) => b,
            None => {
                return Err(StateError::PreconditionViolation {
                    detail: String::from("batch is None"),
                })
            }
        };

        batch.chunk_count = count;
        Ok(())
    }

    /// Commit the accumulated batch to durable storage exactly once.
    ///
    /// # Postconditions
    /// - POST-01: batch is durably persisted
    /// - POST-03: write is atomic (all-or-nothing via temp-file + rename)
    ///
    /// # Errors
    /// - `StateError::AlreadyCommitted` if called twice (INV-01)
    /// - `StateError::EmptyBatch` if no documents were processed
    /// - `StateError::PersistenceFailed` if I/O fails
    /// - `StateError::SerializationFailed` if batch cannot be serialized
    pub fn commit_changes(&mut self) -> Result<(), StateError> {
        // INV-01: at most one commit per StateDb lifetime
        if self.committed {
            return Err(StateError::AlreadyCommitted {
                run_id: self.effective_run_id(),
            });
        }

        let batch = match self.batch.as_ref() {
            Some(b) => b,
            None => {
                return Err(StateError::PreconditionViolation {
                    detail: String::from("batch is None"),
                })
            }
        };

        // B19: reject empty batch (document_count == 0)
        if batch.document_count == 0 {
            return Err(StateError::EmptyBatch {
                run_id: batch.run_id.clone(),
            });
        }

        // Calculation layer: serialize (pure transform)
        let serialized =
            serde_json::to_string_pretty(batch).map_err(|e| StateError::SerializationFailed {
                reason: e.to_string(),
            })?;

        // Action layer: atomic write via temp-file + rename
        let tmp_path = self.output_dir.join("state-batch.json.tmp");
        let final_path = self.output_dir.join("state-batch.json");

        std::fs::write(&tmp_path, &serialized).map_err(|e| StateError::PersistenceFailed {
            run_id: batch.run_id.clone(),
            reason: e.to_string(),
        })?;

        std::fs::rename(&tmp_path, &final_path).map_err(|e| StateError::PersistenceFailed {
            run_id: batch.run_id.clone(),
            reason: e.to_string(),
        })?;

        // INV-01: state machine transitions {Uncommitted} → {Committed}
        self.committed = true;
        Ok(())
    }

    /// Query whether the batch has been committed.
    #[must_use]
    pub fn is_committed(&self) -> bool {
        self.committed
    }

    /// Access the internal batch (test-only helper for verifying state).
    #[cfg(test)]
    pub fn batch(&self) -> &Option<StateBatch> {
        &self.batch
    }

    // -- Private helpers --

    /// Guard: returns `Err(MutationAfterCommit)` if already committed (INV-02).
    fn ensure_uncommitted(&self) -> Result<(), StateError> {
        if self.committed {
            return Err(StateError::MutationAfterCommit {
                run_id: self.effective_run_id(),
            });
        }
        Ok(())
    }

    /// Extract the current [`RunId`] from the batch, or a sentinel fallback.
    fn effective_run_id(&self) -> RunId {
        match &self.batch {
            Some(b) => b.run_id.clone(),
            None => RunId(String::from("unknown")),
        }
    }
}

// Drop does NOT commit — only cleans up.
impl Drop for StateDb {
    fn drop(&mut self) {
        // Intentionally does NOT call commit_changes.
        // Logging an uncommitted-state warning is acceptable.
    }
}

// ==========================================================================
// LAYER 1: UNIT TESTS (inline #[cfg(test)] module)
// ==========================================================================

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod unit_tests {
    use super::*;
    use std::path::PathBuf;

    // Helper: create a temp dir for StateDb::new
    fn temp_output_dir() -> tempfile::TempDir {
        tempfile::TempDir::new().expect("Failed to create temp dir")
    }

    // Helper: create a StateDb via the real constructor (will todo!() in red phase)
    // For unit tests that don't need filesystem, we'll create via internal construction
    // when the types are implemented. For now, they'll hit todo!().

    // ---- B04: StateDb::new initializes empty batch and uncommitted ----

    #[test]
    fn state_db_new_initializes_empty_batch_and_uncommitted() {
        // Given: a writable temporary directory
        let dir = temp_output_dir();

        // When: StateDb::new is called
        let state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

        // Then: is_committed() == false
        assert!(
            !state_db.is_committed(),
            "newly created StateDb should not be committed"
        );

        // And: batch has document_count == 0, chunk_count == 0, file_hashes is empty
        let batch = state_db
            .batch()
            .as_ref()
            .expect("batch should be Some after construction");
        assert_eq!(
            batch.document_count, 0,
            "document_count should be 0 initially"
        );
        assert_eq!(batch.chunk_count, 0, "chunk_count should be 0 initially");
        assert!(
            batch.file_hashes.is_empty(),
            "file_hashes should be empty initially"
        );
    }

    // ---- B09: record_file_hash appends entry when uncommitted ----

    #[test]
    fn record_file_hash_appends_entry_when_uncommitted() {
        // Given: a fresh StateDb in uncommitted state
        let dir = temp_output_dir();
        let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

        // When: record_file_hash is called with valid data
        let result = state_db.record_file_hash("src/guide.md", "sha256:abc123");

        // Then: Ok(()) is returned
        assert_eq!(result, Ok(()));

        // And: the batch's file_hashes contains the entry
        let batch = state_db.batch().as_ref().expect("batch should be Some");
        assert_eq!(batch.file_hashes.len(), 1);
        assert_eq!(batch.file_hashes[0].relative_path, "src/guide.md");
        assert_eq!(batch.file_hashes[0].content_hash, "sha256:abc123");
    }

    // ---- B10: record_file_hash returns MutationAfterCommit when committed ----

    #[test]
    fn record_file_hash_returns_mutation_after_commit_when_committed() {
        // Given: a committed StateDb
        let dir = temp_output_dir();
        let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");
        state_db
            .set_document_count(1)
            .expect("set_document_count should succeed");
        state_db
            .record_file_hash("a.md", "hash")
            .expect("record_file_hash should succeed");
        state_db
            .commit_changes()
            .expect("commit_changes should succeed");

        // When: record_file_hash is called after commit
        let result = state_db.record_file_hash("any.md", "hash");

        // Then: Err(MutationAfterCommit { run_id })
        let batch = state_db.batch().as_ref().expect("batch should be Some");
        let expected_run_id = batch.run_id.clone();
        assert_eq!(
            result,
            Err(StateError::MutationAfterCommit {
                run_id: expected_run_id,
            })
        );
    }

    // ---- B11: record_file_hash returns DuplicateFilePath when same path twice ----

    #[test]
    fn record_file_hash_returns_duplicate_file_path_when_same_path_twice() {
        // Given: a fresh StateDb where record_file_hash was called once
        let dir = temp_output_dir();
        let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");
        state_db
            .record_file_hash("src/guide.md", "hash1")
            .expect("first call should succeed");

        // When: record_file_hash is called with the same path
        let result = state_db.record_file_hash("src/guide.md", "hash2");

        // Then: Err(DuplicateFilePath { path: "src/guide.md" })
        assert_eq!(
            result,
            Err(StateError::DuplicateFilePath {
                path: "src/guide.md".to_string(),
            })
        );
    }

    // ---- B12: set_document_count updates batch when uncommitted ----

    #[test]
    fn set_document_count_updates_batch_when_uncommitted() {
        // Given: a fresh StateDb in uncommitted state
        let dir = temp_output_dir();
        let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

        // When: set_document_count(42) is called
        let result = state_db.set_document_count(42);

        // Then: Ok(())
        assert_eq!(result, Ok(()));

        // And: batch document_count == 42
        let batch = state_db.batch().as_ref().expect("batch should be Some");
        assert_eq!(batch.document_count, 42);
    }

    // ---- B13: set_document_count returns MutationAfterCommit when committed ----

    #[test]
    fn set_document_count_returns_mutation_after_commit_when_committed() {
        // Given: a committed StateDb
        let dir = temp_output_dir();
        let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");
        state_db
            .set_document_count(1)
            .expect("set_document_count should succeed");
        state_db
            .record_file_hash("a.md", "h")
            .expect("record_file_hash should succeed");
        state_db
            .commit_changes()
            .expect("commit_changes should succeed");

        // When: set_document_count(10) is called after commit
        let result = state_db.set_document_count(10);

        // Then: Err(MutationAfterCommit { run_id })
        let batch = state_db.batch().as_ref().expect("batch should be Some");
        let expected_run_id = batch.run_id.clone();
        assert_eq!(
            result,
            Err(StateError::MutationAfterCommit {
                run_id: expected_run_id,
            })
        );
    }

    // ---- B14: set_chunk_count updates batch when uncommitted ----

    #[test]
    fn set_chunk_count_updates_batch_when_uncommitted() {
        // Given: a fresh StateDb in uncommitted state
        let dir = temp_output_dir();
        let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

        // When: set_chunk_count(128) is called
        let result = state_db.set_chunk_count(128);

        // Then: Ok(())
        assert_eq!(result, Ok(()));

        // And: batch chunk_count == 128
        let batch = state_db.batch().as_ref().expect("batch should be Some");
        assert_eq!(batch.chunk_count, 128);
    }

    // ---- B15: set_chunk_count returns MutationAfterCommit when committed ----

    #[test]
    fn set_chunk_count_returns_mutation_after_commit_when_committed() {
        // Given: a committed StateDb
        let dir = temp_output_dir();
        let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");
        state_db
            .set_document_count(1)
            .expect("set_document_count should succeed");
        state_db
            .record_file_hash("a.md", "h")
            .expect("record_file_hash should succeed");
        state_db
            .commit_changes()
            .expect("commit_changes should succeed");

        // When: set_chunk_count(50) is called after commit
        let result = state_db.set_chunk_count(50);

        // Then: Err(MutationAfterCommit { run_id })
        let batch = state_db.batch().as_ref().expect("batch should be Some");
        let expected_run_id = batch.run_id.clone();
        assert_eq!(
            result,
            Err(StateError::MutationAfterCommit {
                run_id: expected_run_id,
            })
        );
    }

    // ---- B16: set_chunk_count with zero succeeds and commit succeeds when documents exist ----

    #[test]
    fn set_chunk_count_zero_succeeds_and_commit_succeeds_when_documents_exist() {
        // Given: a fresh StateDb in uncommitted state
        let dir = temp_output_dir();
        let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

        // When: set_chunk_count(0) is called
        let result = state_db.set_chunk_count(0);

        // Then: Ok(())
        assert_eq!(result, Ok(()));
        assert_eq!(state_db.batch().as_ref().expect("batch").chunk_count, 0);

        // And: commit succeeds when document_count > 0
        state_db
            .set_document_count(1)
            .expect("set_document_count should succeed");
        state_db
            .record_file_hash("a.md", "h")
            .expect("record_file_hash should succeed");
        let commit_result = state_db.commit_changes();
        assert_eq!(commit_result, Ok(()));
    }

    // ---- B18: commit_changes returns AlreadyCommitted when called twice ----

    #[test]
    fn commit_changes_returns_already_committed_when_called_twice() {
        // Given: a committed StateDb
        let dir = temp_output_dir();
        let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");
        state_db
            .set_document_count(1)
            .expect("set_document_count should succeed");
        state_db
            .record_file_hash("a.md", "h")
            .expect("record_file_hash should succeed");
        state_db
            .commit_changes()
            .expect("first commit should succeed");

        // When: commit_changes is called a second time
        let result = state_db.commit_changes();

        // Then: Err(AlreadyCommitted { run_id })
        let batch = state_db.batch().as_ref().expect("batch should be Some");
        let expected_run_id = batch.run_id.clone();
        assert_eq!(
            result,
            Err(StateError::AlreadyCommitted {
                run_id: expected_run_id,
            })
        );
    }

    // ---- B19: commit_changes returns EmptyBatch when no documents ----

    #[test]
    fn commit_changes_returns_empty_batch_when_no_documents() {
        // Given: a fresh StateDb with document_count == 0 and no file_hashes
        let dir = temp_output_dir();
        let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

        // When: commit_changes is called
        let result = state_db.commit_changes();

        // Then: Err(EmptyBatch { run_id })
        let batch = state_db.batch().as_ref().expect("batch should be Some");
        let expected_run_id = batch.run_id.clone();
        assert_eq!(
            result,
            Err(StateError::EmptyBatch {
                run_id: expected_run_id,
            })
        );
    }

    // ---- B22: is_committed returns false when newly created ----

    #[test]
    fn is_committed_returns_false_when_newly_created() {
        // Given: a freshly created StateDb
        let dir = temp_output_dir();
        let state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

        // Then: is_committed() == false
        assert!(
            !state_db.is_committed(),
            "newly created StateDb should report uncommitted"
        );
    }

    // ---- B23: is_committed returns true after commit ----

    #[test]
    fn is_committed_returns_true_after_commit() {
        // Given: a committed StateDb
        let dir = temp_output_dir();
        let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");
        state_db
            .set_document_count(1)
            .expect("set_document_count should succeed");
        state_db
            .record_file_hash("a.md", "h")
            .expect("record_file_hash should succeed");
        state_db.commit_changes().expect("commit should succeed");

        // Then: is_committed() == true
        assert!(
            state_db.is_committed(),
            "StateDb should report committed after commit_changes"
        );
    }

    // ---- B25: Distinct file paths accumulate correctly ----

    #[test]
    fn batch_accepts_distinct_paths_and_accumulates_correctly() {
        // Given: a fresh StateDb
        let dir = temp_output_dir();
        let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

        // When: three distinct paths are recorded
        assert_eq!(state_db.record_file_hash("a.md", "h1"), Ok(()));
        assert_eq!(state_db.record_file_hash("b.md", "h2"), Ok(()));
        assert_eq!(state_db.record_file_hash("c.md", "h3"), Ok(()));

        // Then: after commit, the batch file_hashes has exactly 3 entries
        state_db
            .set_document_count(3)
            .expect("set_document_count should succeed");
        state_db.commit_changes().expect("commit should succeed");

        let batch = state_db.batch().as_ref().expect("batch should be Some");
        assert_eq!(batch.file_hashes.len(), 3);
        assert_eq!(
            batch.file_hashes[0],
            FileHashRecord {
                relative_path: "a.md".to_string(),
                content_hash: "h1".to_string(),
            }
        );
        assert_eq!(
            batch.file_hashes[1],
            FileHashRecord {
                relative_path: "b.md".to_string(),
                content_hash: "h2".to_string(),
            }
        );
        assert_eq!(
            batch.file_hashes[2],
            FileHashRecord {
                relative_path: "c.md".to_string(),
                content_hash: "h3".to_string(),
            }
        );
    }

    // ---- Additional unit tests for combinatorial coverage ----

    #[test]
    fn record_file_hash_accepts_empty_relative_path_when_uncommitted() {
        // Given: a fresh StateDb
        let dir = temp_output_dir();
        let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

        // When: record_file_hash with empty path
        let result = state_db.record_file_hash("", "hash");

        // Then: Ok(()) — empty paths are accepted
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn record_file_hash_accepts_path_traversal_string_when_uncommitted() {
        // Given: a fresh StateDb
        let dir = temp_output_dir();
        let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

        // When: record_file_hash with traversal path
        let result = state_db.record_file_hash("../etc/passwd", "hash");

        // Then: Ok(()) — no path sanitization in contract
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn set_document_count_accepts_usize_max_when_uncommitted() {
        let dir = temp_output_dir();
        let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

        let result = state_db.set_document_count(usize::MAX);

        assert_eq!(result, Ok(()));
        assert_eq!(
            state_db.batch().as_ref().expect("batch").document_count,
            usize::MAX
        );
    }

    #[test]
    fn set_document_count_accepts_one_when_uncommitted() {
        let dir = temp_output_dir();
        let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

        let result = state_db.set_document_count(1);

        assert_eq!(result, Ok(()));
        assert_eq!(state_db.batch().as_ref().expect("batch").document_count, 1);
    }

    #[test]
    fn set_chunk_count_accepts_usize_max_when_uncommitted() {
        let dir = temp_output_dir();
        let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

        let result = state_db.set_chunk_count(usize::MAX);

        assert_eq!(result, Ok(()));
        assert_eq!(
            state_db.batch().as_ref().expect("batch").chunk_count,
            usize::MAX
        );
    }

    #[test]
    fn set_document_count_zero_succeeds_but_commit_returns_empty_batch() {
        let dir = temp_output_dir();
        let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

        state_db
            .set_document_count(0)
            .expect("set_document_count(0) should succeed");
        let result = state_db.commit_changes();

        let batch = state_db.batch().as_ref().expect("batch should be Some");
        let expected_run_id = batch.run_id.clone();
        assert_eq!(
            result,
            Err(StateError::EmptyBatch {
                run_id: expected_run_id
            })
        );
    }

    #[test]
    fn two_state_db_instances_have_different_run_ids() {
        let dir1 = temp_output_dir();
        let dir2 = temp_output_dir();
        let db1 = StateDb::new(dir1.path()).expect("StateDb::new should succeed");
        let db2 = StateDb::new(dir2.path()).expect("StateDb::new should succeed");

        let id1 = db1.batch().as_ref().expect("batch1").run_id.clone();
        let id2 = db2.batch().as_ref().expect("batch2").run_id.clone();

        assert_ne!(
            id1, id2,
            "Two StateDb instances should have different run IDs"
        );
    }

    #[test]
    fn set_document_count_last_write_wins() {
        let dir = temp_output_dir();
        let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

        state_db
            .set_document_count(5)
            .expect("first set should succeed");
        state_db
            .set_document_count(10)
            .expect("second set should succeed");
        state_db
            .set_document_count(42)
            .expect("third set should succeed");

        assert_eq!(state_db.batch().as_ref().expect("batch").document_count, 42);
    }

    #[test]
    fn set_chunk_count_last_write_wins() {
        let dir = temp_output_dir();
        let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

        state_db
            .set_chunk_count(100)
            .expect("first set should succeed");
        state_db
            .set_chunk_count(200)
            .expect("second set should succeed");
        state_db
            .set_chunk_count(7)
            .expect("third set should succeed");

        assert_eq!(state_db.batch().as_ref().expect("batch").chunk_count, 7);
    }

    #[test]
    fn record_file_hash_empty_content_hash_accepted() {
        let dir = temp_output_dir();
        let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

        let result = state_db.record_file_hash("path.md", "");

        assert_eq!(result, Ok(()));
        let batch = state_db.batch().as_ref().expect("batch");
        assert_eq!(batch.file_hashes[0].content_hash, "");
    }
}

// ==========================================================================
// LAYER 3: PROPERTY TESTS (proptest)
// ==========================================================================

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod proptests {
    use super::*;
    use proptest::prop_assert;
    use proptest::prop_assert_eq;
    use proptest::prop_assume;
    use std::collections::HashSet;

    fn temp_output_dir() -> tempfile::TempDir {
        tempfile::TempDir::new().expect("Failed to create temp dir")
    }

    // Helper to create a fully populated StateDb ready for commit
    fn populated_state_db() -> (tempfile::TempDir, StateDb) {
        let dir = temp_output_dir();
        let mut db = StateDb::new(dir.path()).expect("StateDb::new should succeed");
        db.set_document_count(1)
            .expect("set_document_count should succeed");
        db.record_file_hash("a.md", "hash")
            .expect("record_file_hash should succeed");
        (dir, db)
    }

    // ---- PROP-01: record_file_hash idempotent rejection ----

    proptest::proptest! {
        #[test]
        fn prop_01_record_file_hash_duplicate_rejection(
            relative_path in "[a-zA-Z0-9_/]+\\.md",
            hash_a in "[a-f0-9]{16}",
            hash_b in "[a-f0-9]{16}",
        ) {
            let dir = temp_output_dir();
            let mut db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

            // First call succeeds
            let first = db.record_file_hash(&relative_path, &hash_a);
            prop_assert_eq!(first, Ok(()));

            // Second call with same path always returns DuplicateFilePath
            let second = db.record_file_hash(&relative_path, &hash_b);
            prop_assert_eq!(
                second,
                Err(StateError::DuplicateFilePath { path: relative_path.clone() }),
                "second call with same path must be rejected"
            );
        }

        #[test]
        fn prop_01_record_file_hash_distinct_paths_succeed(
            path_a in "[a-zA-Z0-9_/]+\\.md",
            path_b in "[a-zA-Z0-9_/]+\\.md",
            hash_a in "[a-f0-9]{16}",
            hash_b in "[a-f0-9]{16}",
        ) {
            // Skip if paths are the same (covered by the duplicate test)
            prop_assume!(path_a != path_b);

            let dir = temp_output_dir();
            let mut db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

            prop_assert_eq!(db.record_file_hash(&path_a, &hash_a), Ok(()));
            prop_assert_eq!(db.record_file_hash(&path_b, &hash_b), Ok(()));

            let batch = db.batch().as_ref().expect("batch should be Some");
            prop_assert_eq!(batch.file_hashes.len(), 2);
        }

        // ---- PROP-02: set_document_count / set_chunk_count last-write-wins ----

        #[test]
        fn prop_02_document_count_last_write_wins(
            counts in proptest::collection::vec(1_usize..=1000, 1..10)
        ) {
            let dir = temp_output_dir();
            let mut db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

            for count in &counts {
                db.set_document_count(*count).expect("set_document_count should succeed");
            }

            let expected = *counts.last().expect("non-empty vec");
            let actual = db.batch().as_ref().expect("batch").document_count;
            prop_assert_eq!(actual, expected);
        }

        #[test]
        fn prop_02_chunk_count_last_write_wins(
            counts in proptest::collection::vec(0_usize..=10000, 1..10)
        ) {
            let dir = temp_output_dir();
            let mut db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

            for count in &counts {
                db.set_chunk_count(*count).expect("set_chunk_count should succeed");
            }

            let expected = *counts.last().expect("non-empty vec");
            let actual = db.batch().as_ref().expect("batch").chunk_count;
            prop_assert_eq!(actual, expected);
        }

        // ---- PROP-03: Round-trip serialization ----

        #[test]
        fn prop_03_state_batch_roundtrips(
            run_id_str in "[a-zA-Z0-9]{8,32}",
            source_path in "[a-zA-Z0-9_/]{4,64}",
            output_path in "[a-zA-Z0-9_/]{4,64}",
            document_count in 1_usize..=1000,
            chunk_count in 0_usize..=10000,
            created_at in 0_u64..=1_700_000_000_u64,
            num_hashes in 0_usize..=20,
        ) {
            let mut file_hashes = Vec::with_capacity(num_hashes);
            for i in 0..num_hashes {
                file_hashes.push(FileHashRecord {
                    relative_path: format!("file_{i}.md"),
                    content_hash: format!("sha256:{i:016x}"),
                });
            }

            let batch = StateBatch {
                run_id: RunId(run_id_str),
                source_path,
                output_path,
                document_count,
                chunk_count,
                file_hashes,
                created_at_unix_secs: created_at,
            };

            let serialized = serde_json::to_vec(&batch).expect("serialization should succeed");
            let recovered: StateBatch = serde_json::from_slice(&serialized)
                .expect("deserialization should succeed");

            prop_assert_eq!(recovered.run_id, batch.run_id);
            prop_assert_eq!(recovered.source_path, batch.source_path);
            prop_assert_eq!(recovered.output_path, batch.output_path);
            prop_assert_eq!(recovered.document_count, batch.document_count);
            prop_assert_eq!(recovered.chunk_count, batch.chunk_count);
            prop_assert_eq!(recovered.file_hashes, batch.file_hashes);
            prop_assert_eq!(recovered.created_at_unix_secs, batch.created_at_unix_secs);
        }

        // ---- PROP-04: State machine exhaustiveness ----

        #[test]
        fn prop_04_state_machine_no_revert(
            operations in proptest::collection::vec(
                proptest::sample::select(&[
                    "record_hash",
                    "set_doc_count",
                    "set_chunk_count",
                ]),
                0..20,
            )
        ) {
            let dir = temp_output_dir();
            let mut db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

            // Before commit: is_committed must be false
            prop_assert!(!db.is_committed());

            // Apply operations
            for (i, op) in operations.iter().enumerate() {
                match *op {
                    "record_hash" => {
                        let _ = db.record_file_hash(&format!("file_{i}.md"), "hash");
                    }
                    "set_doc_count" => {
                        let _ = db.set_document_count(1);
                    }
                    "set_chunk_count" => {
                        let _ = db.set_chunk_count(1);
                    }
                    _ => {}
                }
                // Still must be false before commit
                prop_assert!(!db.is_committed());
            }

            // Commit
            db.set_document_count(1).expect("need at least 1 doc");
            if db.batch().as_ref().expect("batch").file_hashes.is_empty() {
                db.record_file_hash("a.md", "h").expect("record hash");
            }
            db.commit_changes().expect("commit should succeed");

            // After commit: is_committed must be true
            prop_assert!(db.is_committed());

            // No mutation after commit should succeed
            let doc_result = db.set_document_count(999);
            prop_assert!(doc_result.is_err());
            prop_assert!(!db.is_committed() == false, "committed must remain true");
        }

        // ---- PROP-05: Batch non-negativity ----

        #[test]
        fn prop_05_counts_always_non_negative(
            doc_count in proptest::num::usize::ANY,
            chunk_count in proptest::num::usize::ANY,
        ) {
            let dir = temp_output_dir();
            let mut db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

            db.set_document_count(doc_count).expect("set_document_count should succeed");
            db.set_chunk_count(chunk_count).expect("set_chunk_count should succeed");

            let batch = db.batch().as_ref().expect("batch should be Some");
            prop_assert_eq!(batch.document_count, doc_count);
            prop_assert_eq!(batch.chunk_count, chunk_count);
            // usize is always >= 0 by Rust's type system, but we verify the API contract
        }

        // ---- PROP-06: File hash uniqueness ----

        #[test]
        fn prop_06_file_hash_uniqueness_after_distinct_calls(
            paths in proptest::collection::hash_set("[a-zA-Z0-9_]+\\.md", 0..50)
        ) {
            let dir = temp_output_dir();
            let mut db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

            for path in &paths {
                db.record_file_hash(path, "hash").expect("distinct paths should succeed");
            }

            let batch = db.batch().as_ref().expect("batch should be Some");
            let recorded_paths: HashSet<&str> = batch.file_hashes
                .iter()
                .map(|h| h.relative_path.as_str())
                .collect();

            prop_assert_eq!(recorded_paths.len(), paths.len());
            prop_assert_eq!(batch.file_hashes.len(), paths.len());
        }
    }
}

// ==========================================================================
// LAYER 5: KANI HARNESSSES
// ==========================================================================

#[cfg(kani)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod verification {
    use super::*;

    // ---- KANI-01: State machine transition completeness ----

    #[kani::proof]
    fn verify_committed_never_reverts() {
        let output_dir = std::path::PathBuf::from("/tmp/kani-state-db");
        let _ = std::fs::create_dir_all(&output_dir);

        if let Ok(mut db) = StateDb::new(&output_dir) {
            // Pre-condition: not committed
            assert!(!db.is_committed());

            // Setup minimal state for commit
            let _ = db.set_document_count(1);
            let _ = db.record_file_hash("kani.md", "hash");

            if db.commit_changes().is_ok() {
                // Post-condition: committed
                assert!(db.is_committed());

                // Try all mutation operations — all must fail
                let r1 = db.record_file_hash("x.md", "h");
                assert!(r1.is_err());

                let r2 = db.set_document_count(42);
                assert!(r2.is_err());

                let r3 = db.set_chunk_count(42);
                assert!(r3.is_err());

                let r4 = db.commit_changes();
                assert!(r4.is_err());

                // Still committed after all failed attempts
                assert!(db.is_committed());
            }
        }

        let _ = std::fs::remove_dir_all(&output_dir);
    }

    // ---- KANI-02: document_count / chunk_count arithmetic safety ----

    #[kani::proof]
    fn verify_count_assignment_exact() {
        let output_dir = std::path::PathBuf::from("/tmp/kani-counts");
        let _ = std::fs::create_dir_all(&output_dir);

        let count: usize = kani::any();

        if let Ok(mut db) = StateDb::new(&output_dir) {
            let _ = db.set_document_count(count);
            let batch = db.batch().as_ref().expect("batch");
            assert_eq!(batch.document_count, count);

            let _ = db.set_chunk_count(count);
            let batch = db.batch().as_ref().expect("batch");
            assert_eq!(batch.chunk_count, count);
        }

        let _ = std::fs::remove_dir_all(&output_dir);
    }

    // ---- KANI-03: file_hashes vector capacity bounds ----

    #[kani::proof]
    fn verify_file_hashes_len_after_distinct_inserts() {
        let output_dir = std::path::PathBuf::from("/tmp/kani-hashes");
        let _ = std::fs::create_dir_all(&output_dir);

        if let Ok(mut db) = StateDb::new(&output_dir) {
            // Kani: bounded model checking with small N
            let n: usize = kani::any();
            kani::assume(n <= 10);

            let mut inserted = 0_usize;
            for i in 0..n {
                let path = format!("file_{i}.md");
                if db.record_file_hash(&path, "hash").is_ok() {
                    inserted = inserted.saturating_add(1);
                }
            }

            let batch = db.batch().as_ref().expect("batch");
            assert_eq!(batch.file_hashes.len(), inserted);
            assert!(batch.file_hashes.len() <= n);
        }

        let _ = std::fs::remove_dir_all(&output_dir);
    }
}
