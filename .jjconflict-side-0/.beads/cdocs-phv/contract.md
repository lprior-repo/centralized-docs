---
bead_id: cdocs-phv
bead_title: "action: commit index state once at shutdown and assert transaction invariants"
phase: state-1-contract
updated_at: 2026-04-02T14:00:00Z
---

# Contract Specification

## Context

- **Feature:** Introduce a `StateDb` abstraction that accumulates index-state mutations
  in-memory throughout the `run_index` pipeline and commits the entire batch exactly
  once at shutdown (successful completion). If any pipeline stage fails, zero state is
  written to disk. This replaces the current pattern where each step independently
  writes artifacts to the output directory without a coordinated transaction boundary.
- **Domain terms:**
  - **StateDb** -- An in-memory accumulator for index-state mutations. Holds a
    `StateBatch` that is populated progressively as pipeline stages succeed. Only
    flushes to durable storage on explicit `commit_changes` invocation.
  - **StateBatch** -- A serializable collection of all mutations produced by one
    `run_index` invocation (document records, chunk records, graph edges, index
    metadata, file hashes).
  - **Commit** -- The single atomic write of the accumulated `StateBatch` to the
    output directory's state database (redb-backed or equivalent).
  - **Pipeline stage** -- Each numbered step in `run_index` (DISCOVER through STEP 8).
- **Assumptions:**
  - The `StateDb` is created fresh per `run_index` invocation (not reused across runs).
  - The `StateDb` lifecycle is bounded: open at pipeline start, either commit on success
    or drop on failure. `Drop` does NOT commit.
  - The existing `OutputLock` mechanism continues to guard against concurrent runs.
  - The output directory exists and is writable (enforced by `validate_output_path` and
    `acquire_output_lock` preconditions).
- **Open questions:**
  - Exact schema of `StateBatch` fields (to be refined in implementation bead).
  - Whether `StateDb` wraps redb directly or uses the existing `DocCache` as a backend.
    Contract is backend-agnostic; the commit-once invariant is independent of storage.

---

## Domain Types

```rust
/// Unique identifier for a single run_index invocation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RunId(String);

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

/// The state database handle. Holds an in-memory batch that is NOT
/// written until `commit_changes` is called.
pub struct StateDb {
    batch: Option<StateBatch>,
    output_dir: PathBuf,
    committed: bool,
}
```

---

## Preconditions

### P-01: StateDb is created before any pipeline stage executes
- `StateDb::new(output_dir)` MUST be called before STEP 1 (DISCOVER) in `run_index`.
- The `output_dir` MUST be a valid, writable directory (enforced by existing
  `validate_output_path` + `acquire_output_lock`).

### P-02: StateBatch is initialised as empty
- Upon construction, `StateDb.batch` is `Some(StateBatch { ... })` with zero
  document_count, zero chunk_count, empty file_hashes.
- `StateDb.committed` is `false`.

### P-03: Output directory is locked
- The `OutputLock` MUST be held for the entire lifetime of the `StateDb`.
  No state operations may proceed without an active lock.

### P-04: commit_changes is called at most once
- `commit_changes` MUST be called zero or one times per `StateDb` instance.
- Calling `commit_changes` on an already-committed `StateDb` is a contract
  violation (returns `StateError::AlreadyCommitted`).

---

## Postconditions

### POST-01: Successful pipeline -- state is committed exactly once
- When `run_index` completes all stages without error, `StateDb::commit_changes`
  MUST have been called exactly once.
- After `commit_changes` returns `Ok(())`, `StateDb.committed` is `true`.
- The `StateBatch` is durably persisted to the output directory.

### POST-02: Failed pipeline -- zero state written
- If any pipeline stage returns `Err`, `commit_changes` MUST NOT have been called.
- The `StateDb` is dropped without committing. No partial state appears in the
  output directory from this run.

### POST-03: commit_changes is atomic
- Either the entire `StateBatch` is written, or nothing is written. There is no
  intermediate state observable by a concurrent reader.

### POST-04: committed batch reflects all successful stages
- After commit, `StateBatch.document_count` equals the number of documents that
  passed through STEP 2 (ANALYZE).
- `StateBatch.chunk_count` equals `chunks_result.total_chunks` from STEP 5.
- `StateBatch.file_hashes` contains exactly one entry per discovered file (STEP 1).

---

## Invariants

### INV-01: At most one commit per StateDb lifetime
- `StateDb.committed` transitions from `false` to `true` at most once.
- This is a state machine: `{Uncommitted} -> {Committed}`. No reverse transition.

### INV-02: No mutation after commit
- Once `commit_changes` returns `Ok(())`, calling any mutation method on `StateDb`
  (e.g., `record_file_hash`, `set_document_count`) returns `StateError::AlreadyCommitted`.

### INV-03: Drop does not commit
- The `Drop` impl for `StateDb` MUST NOT call `commit_changes`. Drop is for cleanup
  only (logging an uncommitted-state warning is acceptable).

### INV-04: Batch consistency
- At all times before commit, `StateBatch.document_count >= 0` and
  `StateBatch.chunk_count >= 0`.
- `StateBatch.file_hashes` contains no duplicate `relative_path` entries.

### INV-05: Single-writer via OutputLock
- Only one `StateDb` can be active for a given output directory at a time,
  guaranteed by the existing `OutputLock` mechanism.

---

## Error Taxonomy

```rust
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

    /// A precondition was violated (e.g., no OutputLock held).
    #[error("precondition violated: {detail}")]
    PreconditionViolation { detail: String },
}
```

---

## Contract Signatures

```rust
impl StateDb {
    /// Create a new StateDb bound to the given output directory.
    ///
    /// # Preconditions
    /// - P-01: output_dir exists and is writable
    /// - P-03: OutputLock is held for output_dir
    ///
    /// # Postconditions
    /// - POST-02 (initial): batch is initialised empty, committed is false
    pub fn new(output_dir: &Path) -> Result<Self, StateError>;

    /// Record a file's content hash in the pending batch.
    ///
    /// # Preconditions
    /// - INV-02: state is not yet committed
    ///
    /// # Errors
    /// - StateError::MutationAfterCommit if already committed
    /// - StateError::DuplicateFilePath if relative_path already recorded
    pub fn record_file_hash(
        &mut self,
        relative_path: &str,
        content_hash: &str,
    ) -> Result<(), StateError>;

    /// Set the document count in the pending batch.
    ///
    /// # Preconditions
    /// - INV-02: state is not yet committed
    ///
    /// # Errors
    /// - StateError::MutationAfterCommit if already committed
    pub fn set_document_count(&mut self, count: usize) -> Result<(), StateError>;

    /// Set the chunk count in the pending batch.
    ///
    /// # Preconditions
    /// - INV-02: state is not yet committed
    ///
    /// # Errors
    /// - StateError::MutationAfterCommit if already committed
    pub fn set_chunk_count(&mut self, count: usize) -> Result<(), StateError>;

    /// Commit the accumulated batch to durable storage exactly once.
    ///
    /// THE SYSTEM SHALL call this method exactly once after successful
    /// processing. If any earlier pipeline stage has failed, this method
    /// MUST NOT be called.
    ///
    /// # Preconditions
    /// - P-04: commit_changes has not been called before
    /// - INV-01: state machine is in Uncommitted state
    ///
    /// # Postconditions
    /// - POST-01: batch is durably persisted
    /// - POST-03: write is atomic (all-or-nothing)
    /// - POST-04: batch contains correct counts from all stages
    ///
    /// # Errors
    /// - StateError::AlreadyCommitted if called twice
    /// - StateError::EmptyBatch if no documents were processed
    /// - StateError::PersistenceFailed if I/O fails
    /// - StateError::SerializationFailed if batch cannot be serialized
    pub fn commit_changes(&mut self) -> Result<(), StateError>;

    /// Query whether the batch has been committed.
    ///
    /// This is a pure query with no side effects.
    #[must_use]
    pub fn is_committed(&self) -> bool;
}
```

---

## Integration Contract: run_index

The modified `run_index` function SHALL satisfy this pseudocode contract:

```rust
pub fn run_index(source: &Path, output: &Path, config: &IndexConfig) -> Result<()> {
    // Pre-pipeline (existing)
    validate_output_path(output)?;
    let _output_lock = acquire_output_lock(output)?;

    // NEW: Create state accumulator
    let mut state_db = StateDb::new(output)?;

    // STEP 1: DISCOVER
    let (files, manifest) = discover::discover_files(source, ...)?;
    state_db.record_file_hash_for_each(&files)?;  // accumulate hashes

    // STEP 2: ANALYZE
    let analyze_result = analyze::analyze_files(...)?;
    state_db.set_document_count(analyze_result.analyses.len())?;

    // ... STEPS 3-8 (unchanged, no early returns bypass commit) ...

    // FINAL: Commit state exactly once on success
    // THE SYSTEM SHALL reach this line ONLY if all prior stages succeeded.
    state_db.commit_changes()?;

    Ok(())
}
```

**Key guarantee:** If the `?` operator propagates an error from ANY step (1-8),
`state_db.commit_changes()` is NEVER called. The `StateDb` is dropped silently,
writing nothing to disk.

---

## Non-goals

- This contract does NOT define the on-disk schema for `StateBatch` persistence
  (implementation detail).
- This contract does NOT define incremental rebuild logic (future bead).
- This contract does NOT modify the existing `DocCache` / `CacheBackend` types.
- This contract does NOT handle concurrent readers of committed state (single-writer
  guarantee via `OutputLock` is sufficient).
