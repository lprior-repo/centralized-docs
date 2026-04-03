# Contract Specification: Validated StateChanges and Atomic commit_changes

## Bead Metadata

- **bead_id**: cdocs-pxx
- **bead_title**: data: implement validated StateChanges and atomic commit_changes
- **phase**: data (types + invariants + method signature)
- **source_file**: `src/cache/mod.rs` (extends existing module)
- **architecture_ref**: `architecture-spec.md` sections 5, 6

## Context

- **Feature**: Introduce a `StateChanges` batch-commit type and a `StateDb::commit_changes` method that writes all state mutations inside a single redb write transaction. Replace the current per-key `DocCache::put` pattern (which opens a new write transaction per call) with a bulk, pre-validated, atomic commit.
- **Domain terms**:
  - `StateChanges`: A batch of state mutations to commit atomically (updated/deleted files, new analysis/transform/chunk outputs, updated/deleted URLs, new/deleted snapshots)
  - `StateDb`: A newtype wrapper over `redb::Database` providing the two-transaction architecture (one read, one write per command run)
  - `FileStateRaw`: Fixed 200-byte Pod type (`#[repr(C)]`) for file state; read/written via bytemuck memcpy
  - `UrlStateRaw`: Fixed 120-byte Pod type for URL state
  - `OwnedArchive<T>`: Wrapper holding `Box<[u8]>` rkyv-serialized bytes with zero-copy archived access
  - `ContentHash`: Existing SHA-256 newtype (`[u8; 32]`)
  - Reference hash: A `[u8; 32]` key that MUST resolve to an existing row in a payload table (e.g. `analysis_hash` must point to a row in `analysis_outputs`)
  - Payload table: redb tables keyed by `[u8; 32]` that store rkyv-serialized outputs (`analysis_outputs`, `transform_outputs`, `chunk_outputs`, `scrape_outputs`, `snapshots`)
  - State table: redb tables keyed by string (`file_state` keyed by source_path, `url_state` keyed by URL)
  - Write transaction: A single `redb::WriteTransaction` opened, all writes applied, then committed exactly once
- **Assumptions**:
  - `bytemuck` and `rkyv` dependencies will be added to `Cargo.toml` (covered by a separate bead)
  - `FileStateRaw` and `UrlStateRaw` Pod types are defined elsewhere (covered by a separate bead)
  - The caller is responsible for ensuring the `StateReadSession` is dropped before calling `commit_changes`
  - `StateDb` replaces `DocCache` for state operations; `DocCache` is deprecated but not removed yet
  - redb version is `2.x` with MVCC support
  - All `[u8; 32]` hash keys are SHA-256 digests (32 bytes)
- **Open questions**:
  - Should `commit_changes` validate that deleted file/URL keys actually exist in the database (fail-if-missing), or silently skip non-existent deletes? **Assumption: silently skip** (idempotent delete semantics).
  - Should `StateChanges` enforce a maximum batch size to prevent memory blowout in a single transaction? **Assumption: no hard limit**, the caller already bounds the batch to the number of files in the repo.

---

## Type Specifications

### StateChanges

```rust
/// Batch of state mutations to commit atomically in a single redb write transaction.
///
/// All fields are `Vec`-based batches. The caller populates these during the
/// in-memory diff + pipeline phase, then passes the whole struct to
/// `StateDb::commit_changes` for a single atomic write.
///
/// # Ownership
/// - Moved into `commit_changes` (consumed).
/// - Not `Clone` by design: one batch per command run, one commit, then dropped.
///
/// # Invariants (enforced by commit_changes)
/// - No duplicate source_path keys in `updated_files`
/// - No duplicate URL keys in `updated_urls`
/// - No duplicate hash keys within any payload vec (`new_analyses`, etc.)
/// - Every hash referenced in `updated_files[i].1.{analysis_hash, transform_hash, chunk_hash}`
///   has a corresponding entry in the appropriate `new_*` payload vec (reference integrity)
/// - Every hash referenced in `updated_urls[i].1.url_hash` has a corresponding
///   entry in `new_scrapes` (reference integrity)
pub struct StateChanges {
    /// Files to upsert: (source_path, FileStateRaw).
    /// Duplicate source_paths are an error. Keys not already in the DB are inserts;
    /// keys already present are updates.
    pub updated_files: Vec<(String, FileStateRaw)>,

    /// File source_paths to delete. Non-existent keys are silently skipped.
    pub deleted_files: Vec<String>,

    /// New/updated analysis outputs: (hash_key, rkyv_bytes).
    /// Duplicate hash keys are deduplicated (last-write-wins within the vec).
    pub new_analyses: Vec<([u8; 32], Vec<u8>)>,

    /// New/updated transform outputs: (hash_key, rkyv_bytes).
    /// Duplicate hash keys are deduplicated.
    pub new_transforms: Vec<([u8; 32], Vec<u8>)>,

    /// New/updated chunk outputs: (hash_key, rkyv_bytes).
    /// Duplicate hash keys are deduplicated.
    pub new_chunks: Vec<([u8; 32], Vec<u8>)>,

    /// URLs to upsert: (url, UrlStateRaw).
    /// Duplicate URLs are an error.
    pub updated_urls: Vec<(String, UrlStateRaw)>,

    /// URLs to delete. Non-existent keys are silently skipped.
    pub deleted_urls: Vec<String>,

    /// New/updated scrape outputs: (hash_key, rkyv_bytes).
    /// Duplicate hash keys are deduplicated.
    pub new_scrapes: Vec<([u8; 32], Vec<u8>)>,

    /// New/updated snapshot outputs: (hash_key, rkyv_bytes).
    /// Duplicate hash keys are deduplicated.
    pub new_snapshots: Vec<([u8; 32], Vec<u8>)>,

    /// Snapshot hash keys to delete. Non-existent keys are silently skipped.
    pub deleted_snapshots: Vec<[u8; 32]>,
}
```

### StateDb (new, alongside existing DocCache)

```rust
/// State database providing the two-transaction architecture:
/// - Transaction 1 (read): bulk load all state into memory
/// - Transaction 2 (write): commit all changes atomically via `commit_changes`
///
/// Wraps a `redb::Database`. Does NOT support in-memory LRU mode.
pub struct StateDb {
    db: redb::Database,
}
```

### StateReadSession

```rust
/// A scoped read transaction. One per command run.
/// Must be dropped before calling `StateDb::commit_changes`.
pub struct StateReadSession<'db> {
    read_txn: redb::ReadTransaction<'db>,
}
```

---

## Method Signatures

### StateDb::open

```rust
impl StateDb {
    /// Open the state database at the given path.
    /// Creates the database and all required tables if they do not exist.
    ///
    /// # Preconditions
    /// - Parent directory exists (or is created by this method).
    ///
    /// # Postconditions
    /// - All tables (`file_state`, `url_state`, `analysis_outputs`,
    ///   `transform_outputs`, `chunk_outputs`, `scrape_outputs`,
    ///   `snapshots`, `metadata`) exist and are writable.
    /// - Exactly one redb write transaction is used for initialization.
    ///
    /// # Errors
    /// - `CommitError::DatabaseOpen` if redb cannot create/open the file.
    /// - `CommitError::TableInit` if any table cannot be created.
    pub fn open(path: &Path) -> Result<Self, CommitError>;
}
```

### StateDb::begin_read

```rust
impl StateDb {
    /// Open a single shared read transaction for the command's lifetime.
    ///
    /// # Preconditions
    /// - Database is open and tables are initialized.
    ///
    /// # Postconditions
    /// - Returned `StateReadSession` holds an open read transaction.
    /// - Caller MUST drop the session before calling `commit_changes`.
    ///
    /// # Errors
    /// - `CommitError::ReadTransaction` if redb cannot begin a read.
    pub fn begin_read(&self) -> Result<StateReadSession<'_>, CommitError>;
}
```

### StateDb::commit_changes

```rust
impl StateDb {
    /// Commit all state changes in exactly one redb write transaction.
    ///
    /// # Preconditions
    /// - `StateReadSession` has been dropped (no concurrent read transaction
    ///   held by this caller).
    /// - All hash keys in `new_analyses`, `new_transforms`, `new_chunks`,
    ///   `new_scrapes`, `new_snapshots` are non-zero (not `[0u8; 32]`).
    /// - All `source_path` strings in `updated_files` are non-empty.
    /// - All URL strings in `updated_urls` are non-empty.
    /// - No duplicate source_path keys in `updated_files`.
    /// - No duplicate URL keys in `updated_urls`.
    /// - Reference integrity: every hash stored in `FileStateRaw.analysis_hash`,
    ///   `FileStateRaw.transform_hash`, `FileStateRaw.chunk_hash` for entries
    ///   in `updated_files` appears as a key in the corresponding `new_*` vec.
    /// - Reference integrity: every hash stored in `UrlStateRaw.url_hash` for
    ///   entries in `updated_urls` appears as a key in `new_scrapes`.
    ///
    /// # Postconditions (on Ok)
    /// - Exactly one write transaction was opened and committed.
    /// - All entries in `updated_files` are persisted to `file_state` table.
    /// - All entries in `deleted_files` are removed from `file_state` table
    ///   (non-existent keys silently skipped).
    /// - All entries in `new_analyses` are persisted to `analysis_outputs` table
    ///   (duplicate hashes deduplicated, last-write-wins).
    /// - All entries in `new_transforms` are persisted to `transform_outputs`.
    ///   (duplicate hashes deduplicated).
    /// - All entries in `new_chunks` are persisted to `chunk_outputs`.
    ///   (duplicate hashes deduplicated).
    /// - All entries in `updated_urls` are persisted to `url_state` table.
    /// - All entries in `deleted_urls` are removed from `url_state` table.
    /// - All entries in `new_scrapes` are persisted to `scrape_outputs`.
    ///   (duplicate hashes deduplicated).
    /// - All entries in `new_snapshots` are persisted to `snapshots`.
    ///   (duplicate hashes deduplicated).
    /// - All entries in `deleted_snapshots` are removed from `snapshots`.
    /// - Rows whose state did not change (same bytes already in table) are
    ///   NOT rewritten (skip unchanged optimization).
    /// - Database is in a consistent state (ACID guarantee from redb).
    /// - The write transaction is no longer open.
    ///
    /// # Postconditions (on Err)
    /// - NO writes were applied (transaction was aborted / never committed).
    /// - Database is in the same state as before the call.
    /// - The write transaction is no longer open.
    ///
    /// # Errors
    /// - `CommitError::PreconditionViolation(variant)` if any precondition fails.
    /// - `CommitError::WriteTransaction` if redb cannot begin a write transaction.
    /// - `CommitError::WriteFailed { table, reason }` if any individual write fails.
    /// - `CommitError::CommitFailed` if redb transaction commit fails.
    pub fn commit_changes(&self, changes: StateChanges) -> Result<(), CommitError>;
}
```

### StateReadSession methods

```rust
impl<'db> StateReadSession<'db> {
    /// Bulk load all file states. Returns HashMap<source_path, FileStateRaw>.
    pub fn load_file_states(&self) -> Result<HashMap<String, FileStateRaw>, CommitError>;

    /// Bulk load all URL states. Returns HashMap<url, UrlStateRaw>.
    pub fn load_url_states(&self) -> Result<HashMap<String, UrlStateRaw>, CommitError>;

    /// Bulk load archived analysis outputs for the given hashes.
    pub fn load_analyses(
        &self,
        hashes: &[[u8; 32]],
    ) -> Result<HashMap<[u8; 32], OwnedArchive<Analysis>>, CommitError>;

    /// Bulk load archived transform outputs for the given hashes.
    pub fn load_transforms(
        &self,
        hashes: &[[u8; 32]],
    ) -> Result<HashMap<[u8; 32], OwnedArchive<String>>, CommitError>;

    /// Bulk load archived chunk outputs for the given hashes.
    pub fn load_chunks(
        &self,
        hashes: &[[u8; 32]],
    ) -> Result<HashMap<[u8; 32], OwnedArchive<Vec<Chunk>>>, CommitError>;

    /// Bulk load archived scrape outputs for the given hashes.
    pub fn load_scrapes(
        &self,
        hashes: &[[u8; 32]],
    ) -> Result<HashMap<[u8; 32], OwnedArchive<ScrapedPage>>, CommitError>;

    /// Bulk load archived snapshots for the given hashes.
    pub fn load_snapshots(
        &self,
        hashes: &[[u8; 32]],
    ) -> Result<HashMap<[u8; 32], OwnedArchive<Snapshot>>, CommitError>;
}
```

---

## Preconditions

### P1: Zero hash keys
- All `[u8; 32]` hash keys in `new_analyses`, `new_transforms`, `new_chunks`, `new_scrapes`, `new_snapshots` MUST NOT be the zero hash `[0u8; 32]`.
- Rationale: the zero hash is indistinguishable from uninitialized memory and would cause silent data corruption.

### P2: Non-empty string keys
- All `source_path` strings in `updated_files` MUST be non-empty after trimming.
- All URL strings in `updated_urls` MUST be non-empty after trimming.

### P3: No duplicate state-table keys
- `updated_files` MUST NOT contain duplicate `source_path` entries.
- `updated_urls` MUST NOT contain duplicate URL entries.

### P4: Reference integrity (hashes resolve)
- For every `(path, state)` in `updated_files`:
  - `state.analysis_hash` MUST appear as a key in `new_analyses`, UNLESS it is the zero hash (indicating "no analysis yet").
  - `state.transform_hash` MUST appear as a key in `new_transforms`, UNLESS it is the zero hash.
  - `state.chunk_hash` MUST appear as a key in `new_chunks`, UNLESS it is the zero hash.
- For every `(url, state)` in `updated_urls`:
  - `state.url_hash` MUST appear as a key in `new_scrapes`, UNLESS it is the zero hash.

### P5: Read session dropped
- No `StateReadSession` derived from this `StateDb` may be alive when `commit_changes` is called.
- Rationale: redb MVCC allows concurrent reads and writes, but the architecture spec mandates sequential read-then-write (drop read, then write). Holding a read session would not cause a deadlock but violates the two-transaction contract.

### P6: Payload value size
- Each `Vec<u8>` in `new_analyses`, `new_transforms`, `new_chunks`, `new_scrapes`, `new_snapshots` MUST NOT exceed `MAX_VALUE_SIZE` (50 MiB).
- Rationale: inherited from existing `DocCache` size constraints.

---

## Postconditions

### On Success (Ok)

**PS1: Exactly one write transaction**
- `commit_changes` opens exactly one `redb::WriteTransaction`, performs all writes, and commits it. The transaction is closed after the method returns.

**PS2: All upserts applied**
- Every entry in `updated_files` is present in the `file_state` table after commit.
- Every entry in `updated_urls` is present in the `url_state` table after commit.
- Every entry in `new_analyses` is present in `analysis_outputs`.
- Every entry in `new_transforms` is present in `transform_outputs`.
- Every entry in `new_chunks` is present in `chunk_outputs`.
- Every entry in `new_scrapes` is present in `scrape_outputs`.
- Every entry in `new_snapshots` is present in `snapshots`.

**PS3: All deletes applied**
- Every entry in `deleted_files` that existed in `file_state` is removed.
- Every entry in `deleted_urls` that existed in `url_state` is removed.
- Every entry in `deleted_snapshots` that existed in `snapshots` is removed.

**PS4: Deduplication**
- Within each payload vec, duplicate hash keys result in a single stored value (last-write-wins within the batch). The database contains exactly one entry per unique hash after commit.

**PS5: Unchanged rows not rewritten**
- If a `FileStateRaw` in `updated_files` is byte-identical to the existing row in `file_state`, the write is skipped (no redb `insert` call for that key).
- Same for `UrlStateRaw` vs `url_state`.
- Same for payload entries: if the `Vec<u8>` is byte-identical to the existing value, the write is skipped.

**PS6: Atomicity**
- All changes are committed or none are. If the redb `commit()` call fails, no partial writes are visible to subsequent reads.

### On Failure (Err)

**PF1: Zero partial writes**
- If `commit_changes` returns `Err`, the database state is identical to the state before the call. No partial mutations are visible.

**PF2: Transaction cleaned up**
- The write transaction (if opened) is aborted/dropped. No lingering transaction holds locks.

---

## Invariants

### I1: Single-writer guarantee
- At most one `commit_changes` call is in progress at any time. This is a redb guarantee (MVCC single-writer).

### I2: Idempotent deletes
- Deleting a non-existent key is a no-op, not an error. `commit_changes` succeeds even if all `deleted_*` entries reference keys that do not exist.

### I3: Hash-only equality for deduplication
- Payload tables (`analysis_outputs`, `transform_outputs`, `chunk_outputs`, `scrape_outputs`, `snapshots`) use hash-key equality only. Two entries with the same `[u8; 32]` key are considered duplicates regardless of value content. Last-write-wins.

### I4: State tables use string-key equality
- `file_state` and `url_state` tables use exact string match. Two entries with the same `source_path` or URL are considered the same row (upsert semantics).

### I5: No zero hash stored as a payload key
- The zero hash `[0u8; 32]` is never stored as a key in any payload table. It represents "no output" in `FileStateRaw`/`UrlStateRaw`.

### I6: Pod size invariants
- `FileStateRaw` is exactly 200 bytes (`std::mem::size_of::<FileStateRaw>() == 200`).
- `UrlStateRaw` is exactly 120 bytes (`std::mem::size_of::<UrlStateRaw>() == 120`).
- Both types are `#[repr(C)]` and `bytemuck::Pod`/`bytemuck::NoUninit`.

### I7: OwnedArchive ownership
- `OwnedArchive<T>` owns its bytes (`Box<[u8]>`). The archived reference (`archived()`) borrows from the owned bytes, not from any redb transaction. Safe to hold across the command's lifetime.

### I8: Byte-identical skip preserves correctness
- Skipping a write for byte-identical data does NOT change the observable database state. A subsequent read returns the same bytes whether the write was skipped or applied.

---

## Error Taxonomy

### CommitError

```rust
/// Errors from the StateDb two-transaction state commit pipeline.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum CommitError {
    // -- Precondition violations (detected BEFORE opening write transaction) --

    /// A hash key in a payload vec is the zero hash (all zeros).
    #[error("zero hash key not allowed in {table}: entry index {index}")]
    ZeroHashKey {
        table: &'static str,
        index: usize,
    },

    /// A string key (source_path or URL) is empty.
    #[error("empty string key in {table}: entry index {index}")]
    EmptyStringKey {
        table: &'static str,
        index: usize,
    },

    /// Duplicate string keys found in a state-table batch.
    #[error("duplicate key in {table}: '{key}'")]
    DuplicateStateKey {
        table: &'static str,
        key: String,
    },

    /// A hash referenced in FileStateRaw/UrlStateRaw has no matching payload entry.
    #[error("reference integrity violation: {field} hash {hash_hex} in {table} has no matching entry in {payload_table}")]
    MissingReference {
        table: &'static str,
        field: &'static str,
        hash_hex: String,
        payload_table: &'static str,
    },

    /// A payload value exceeds MAX_VALUE_SIZE.
    #[error("payload too large in {table}: {size} bytes (max {max})")]
    PayloadTooLarge {
        table: &'static str,
        size: usize,
        max: usize,
    },

    // -- Transaction errors --

    /// Failed to open the redb database.
    #[error("failed to open state database at {path}: {reason}")]
    DatabaseOpen {
        path: String,
        reason: String,
    },

    /// Failed to initialize redb tables.
    #[error("failed to initialize tables: {reason}")]
    TableInit {
        reason: String,
    },

    /// Failed to begin a read transaction.
    #[error("failed to begin read transaction: {reason}")]
    ReadTransaction {
        reason: String,
    },

    /// Failed to begin a write transaction.
    #[error("failed to begin write transaction: {reason}")]
    WriteTransaction {
        reason: String,
    },

    /// An individual write to a redb table failed.
    #[error("write failed for table '{table}': {reason}")]
    WriteFailed {
        table: &'static str,
        reason: String,
    },

    /// Failed to commit the write transaction.
    #[error("failed to commit write transaction: {reason}")]
    CommitFailed {
        reason: String,
    },

    /// A read from a redb table failed.
    #[error("read failed for table '{table}': {reason}")]
    ReadFailed {
        table: &'static str,
        reason: String,
    },
}
```

### Error-to-Phase Mapping

| Phase | Error Variant | When |
|-------|---------------|------|
| Validation (pre-write) | `ZeroHashKey` | Scanning payload vecs for `[0u8; 32]` keys |
| Validation (pre-write) | `EmptyStringKey` | Scanning state vecs for empty strings |
| Validation (pre-write) | `DuplicateStateKey` | Checking for duplicate string keys |
| Validation (pre-write) | `MissingReference` | Cross-referencing hashes in state vs. payload vecs |
| Validation (pre-write) | `PayloadTooLarge` | Checking value sizes against MAX_VALUE_SIZE |
| Database open | `DatabaseOpen` | `StateDb::open` fails |
| Database open | `TableInit` | Table creation fails |
| Read phase | `ReadTransaction` | `begin_read` fails |
| Read phase | `ReadFailed` | Bulk load or hash lookup fails |
| Write phase | `WriteTransaction` | `begin_write` fails |
| Write phase | `WriteFailed` | Individual `insert`/`delete` fails |
| Write phase | `CommitFailed` | `commit()` fails |

### Design Rules for Errors

1. **Precondition errors fire BEFORE the write transaction is opened.** No wasted I/O.
2. **Every error variant is distinguishable.** No two variants share the same `Display` output for the same inputs.
3. **No `unwrap` or `expect` in the commit path.** All redb errors are captured into `CommitError` variants.
4. **`CommitError` is `Send + Sync + 'static`.** Required for `anyhow::Error` compatibility.

---

## redb Table Definitions

```
file_state        key: &str              -> &[u8]  (200 bytes, FileStateRaw via bytemuck)
url_state         key: &str              -> &[u8]  (120 bytes, UrlStateRaw via bytemuck)
analysis_outputs  key: &[u8; 32]         -> &[u8]  (rkyv bytes)
transform_outputs key: &[u8; 32]         -> &[u8]  (rkyv bytes)
chunk_outputs     key: &[u8; 32]         -> &[u8]  (rkyv bytes)
scrape_outputs    key: &[u8; 32]         -> &[u8]  (rkyv bytes)
snapshots         key: &[u8; 32]         -> &[u8]  (rkyv bytes)
metadata          key: &str              -> &str
```

---

## Non-goals

- Do NOT implement `StateDb::commit_changes` body (this is a contract, not implementation).
- Do NOT define `FileStateRaw`, `UrlStateRaw`, or `OwnedArchive<T>` types (separate bead).
- Do NOT add `bytemuck` or `rkyv` to `Cargo.toml` (separate bead).
- Do NOT modify the existing `DocCache` type or its tests.
- Do NOT handle schema migration from the old `documents`/`scrape`/etc. tables to the new `file_state`/`analysis_outputs` schema (separate bead).
- Do NOT implement the `StateReadSession` bulk-load methods (separate bead).
- Do NOT add `commit_changes` to `DocCache` (it goes on `StateDb` only).
