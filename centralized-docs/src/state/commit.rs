//! Batch-commit pipeline for atomic state mutations.
//!
//! Implements the two-transaction architecture:
//! 1. **Read transaction**: bulk load all state into memory via [`StateReadSession`]
//! 2. **Write transaction**: commit all changes atomically via [`StateDb::commit_changes`]
//!
//! # Design
//!
//! - Precondition validation is pure (runs before opening write transaction)
//! - All writes happen in a single redb write transaction (ACID)
//! - Unchanged rows are skipped via [`should_skip_write`]
//! - Payload duplicates use last-write-wins semantics

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

use super::{
    analysis_outputs_table, chunk_outputs_table, file_state_table, scrape_outputs_table,
    snapshots_table, transform_outputs_table, url_state_table, FileStateRaw, UrlStateRaw,
};
use redb::{Database, ReadableTable, TableDefinition, WriteTransaction};
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::path::Path;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Maximum payload value size (50 `MiB`).
pub const MAX_VALUE_SIZE: usize = 50 * 1024 * 1024;

/// The zero hash `[0u8; 32]`, representing "no output yet".
const ZERO_HASH: [u8; 32] = [0u8; 32];

// ---------------------------------------------------------------------------
// StateChanges — batch of mutations (consumed by commit_changes)
// ---------------------------------------------------------------------------

/// Batch of state mutations to commit atomically in a single redb write transaction.
///
/// All fields are `Vec`-based batches. The caller populates these during the
/// in-memory diff + pipeline phase, then passes the whole struct to
/// [`StateDb::commit_changes`] for a single atomic write.
///
/// # Ownership
///
/// Moved into `commit_changes` (consumed). Not `Clone` by design: one batch
/// per command run, one commit, then dropped.
///
/// # Invariants (enforced by `commit_changes`)
///
/// - No duplicate `source_path` keys in `updated_files`
/// - No duplicate URL keys in `updated_urls`
/// - No duplicate hash keys within any payload vec (deduplicated on write)
/// - Every non-zero hash in `FileStateRaw`/`UrlStateRaw` has a corresponding
///   entry in the appropriate `new_*` payload vec (reference integrity)
pub struct StateChanges {
    /// Files to upsert: `(source_path, FileStateRaw)`.
    pub updated_files: Vec<(String, FileStateRaw)>,
    /// File `source_path`s to delete. Non-existent keys silently skipped.
    pub deleted_files: Vec<String>,
    /// New/updated analysis outputs: `(hash_key, rkyv_bytes)`. Deduplicated on write.
    pub new_analyses: Vec<([u8; 32], Vec<u8>)>,
    /// New/updated transform outputs: `(hash_key, rkyv_bytes)`. Deduplicated on write.
    pub new_transforms: Vec<([u8; 32], Vec<u8>)>,
    /// New/updated chunk outputs: `(hash_key, rkyv_bytes)`. Deduplicated on write.
    pub new_chunks: Vec<([u8; 32], Vec<u8>)>,
    /// URLs to upsert: `(url, UrlStateRaw)`.
    pub updated_urls: Vec<(String, UrlStateRaw)>,
    /// URLs to delete. Non-existent keys silently skipped.
    pub deleted_urls: Vec<String>,
    /// New/updated scrape outputs: `(hash_key, rkyv_bytes)`. Deduplicated on write.
    pub new_scrapes: Vec<([u8; 32], Vec<u8>)>,
    /// New/updated snapshot outputs: `(hash_key, rkyv_bytes)`. Deduplicated on write.
    pub new_snapshots: Vec<([u8; 32], Vec<u8>)>,
    /// Snapshot hash keys to delete. Non-existent keys silently skipped.
    pub deleted_snapshots: Vec<[u8; 32]>,
}

impl StateChanges {
    /// Create an empty `StateChanges` with all vecs empty (minimal valid state).
    #[must_use]
    pub fn empty() -> Self {
        Self {
            updated_files: Vec::new(),
            deleted_files: Vec::new(),
            new_analyses: Vec::new(),
            new_transforms: Vec::new(),
            new_chunks: Vec::new(),
            updated_urls: Vec::new(),
            deleted_urls: Vec::new(),
            new_scrapes: Vec::new(),
            new_snapshots: Vec::new(),
            deleted_snapshots: Vec::new(),
        }
    }
}

impl std::fmt::Debug for StateChanges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StateChanges")
            .field("updated_files", &self.updated_files.len())
            .field("deleted_files", &self.deleted_files.len())
            .field("new_analyses", &self.new_analyses.len())
            .field("new_transforms", &self.new_transforms.len())
            .field("new_chunks", &self.new_chunks.len())
            .field("updated_urls", &self.updated_urls.len())
            .field("deleted_urls", &self.deleted_urls.len())
            .field("new_scrapes", &self.new_scrapes.len())
            .field("new_snapshots", &self.new_snapshots.len())
            .field("deleted_snapshots", &self.deleted_snapshots.len())
            .finish()
    }
}

impl Default for StateChanges {
    fn default() -> Self {
        Self::empty()
    }
}

// ---------------------------------------------------------------------------
// CommitError — error taxonomy for the commit pipeline
// ---------------------------------------------------------------------------

/// Errors from the [`StateDb`] two-transaction state commit pipeline.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum CommitError {
    // -- Precondition violations (detected BEFORE opening write transaction) --
    /// A hash key in a payload vec is the zero hash (all zeros).
    #[error("zero hash key not allowed in {table}: entry index {index}")]
    ZeroHashKey { table: &'static str, index: usize },
    /// A string key (`source_path` or URL) is empty after trimming.
    #[error("empty string key in {table}: entry index {index}")]
    EmptyStringKey { table: &'static str, index: usize },
    /// Duplicate string keys found in a state-table batch.
    #[error("duplicate key in {table}: '{key}'")]
    DuplicateStateKey { table: &'static str, key: String },
    /// A hash referenced in `FileStateRaw`/`UrlStateRaw` has no matching payload entry.
    #[error("reference integrity violation: {field} hash {hash_hex} in {table} has no matching entry in {payload_table}")]
    MissingReference {
        table: &'static str,
        field: &'static str,
        hash_hex: String,
        payload_table: &'static str,
    },
    /// A payload value exceeds [`MAX_VALUE_SIZE`].
    #[error("payload too large in {table}: {size} bytes (max {max})")]
    PayloadTooLarge {
        table: &'static str,
        size: usize,
        max: usize,
    },
    // -- Transaction errors --
    /// Failed to open the redb database.
    #[error("failed to open state database at {path}: {reason}")]
    DatabaseOpen { path: String, reason: String },
    /// Failed to initialize redb tables.
    #[error("failed to initialize tables: {reason}")]
    TableInit { reason: String },
    /// Failed to begin a read transaction.
    #[error("failed to begin read transaction: {reason}")]
    ReadTransaction { reason: String },
    /// Failed to begin a write transaction.
    #[error("failed to begin write transaction: {reason}")]
    WriteTransaction { reason: String },
    /// An individual write to a redb table failed.
    #[error("write failed for table '{table}': {reason}")]
    WriteFailed { table: &'static str, reason: String },
    /// Failed to commit the write transaction.
    #[error("failed to commit write transaction: {reason}")]
    CommitFailed { reason: String },
    /// A read from a redb table failed.
    #[error("read failed for table '{table}': {reason}")]
    ReadFailed { table: &'static str, reason: String },
}

// ---------------------------------------------------------------------------
// Pure Calculation: should_skip_write
// ---------------------------------------------------------------------------

/// Returns `true` when `existing` and `new` are byte-identical, indicating
/// the write can be safely skipped without changing observable state.
///
/// # Pure function
///
/// No side effects. Deterministic for all inputs. Used by [`StateDb::commit_changes`]
/// to avoid rewriting unchanged rows.
#[must_use]
pub fn should_skip_write(existing: &[u8], new: &[u8]) -> bool {
    existing == new
}

// ---------------------------------------------------------------------------
// Pure Calculations: Precondition validation
// ---------------------------------------------------------------------------

/// Run all precondition checks. Returns the first error found.
/// Order: zero hashes → empty strings → duplicates → payload sizes → references.
fn validate_all(changes: &StateChanges) -> Result<(), CommitError> {
    validate_no_zero_hashes(changes)?;
    validate_no_empty_string_keys(changes)?;
    validate_no_duplicate_keys(changes)?;
    validate_payload_sizes(changes)?;
    validate_reference_integrity(changes)?;
    Ok(())
}

/// P1: Reject any zero hash key in payload vecs.
fn validate_no_zero_hashes(changes: &StateChanges) -> Result<(), CommitError> {
    check_zero_hash(&changes.new_analyses, "analysis_outputs")?;
    check_zero_hash(&changes.new_transforms, "transform_outputs")?;
    check_zero_hash(&changes.new_chunks, "chunk_outputs")?;
    check_zero_hash(&changes.new_scrapes, "scrape_outputs")?;
    check_zero_hash(&changes.new_snapshots, "snapshots")?;
    Ok(())
}

fn check_zero_hash(
    entries: &[([u8; 32], Vec<u8>)],
    table: &'static str,
) -> Result<(), CommitError> {
    entries
        .iter()
        .enumerate()
        .find_map(|(idx, (hash, _))| {
            (*hash == ZERO_HASH).then_some(CommitError::ZeroHashKey { table, index: idx })
        })
        .map_or(Ok(()), Err)
}

/// P2: Reject empty/whitespace-only string keys.
fn validate_no_empty_string_keys(changes: &StateChanges) -> Result<(), CommitError> {
    check_empty_string_keys(&changes.updated_files, "file_state")?;
    check_empty_string_keys(&changes.updated_urls, "url_state")?;
    Ok(())
}

fn check_empty_string_keys<S>(
    entries: &[(String, S)],
    table: &'static str,
) -> Result<(), CommitError> {
    entries
        .iter()
        .enumerate()
        .find_map(|(idx, (key, _))| {
            key.trim()
                .is_empty()
                .then_some(CommitError::EmptyStringKey { table, index: idx })
        })
        .map_or(Ok(()), Err)
}

/// P3: Reject duplicate string keys in state-table vecs.
fn validate_no_duplicate_keys(changes: &StateChanges) -> Result<(), CommitError> {
    check_duplicate_keys(&changes.updated_files, "file_state")?;
    check_duplicate_keys(&changes.updated_urls, "url_state")?;
    Ok(())
}

fn check_duplicate_keys<S>(
    entries: &[(String, S)],
    table: &'static str,
) -> Result<(), CommitError> {
    let mut seen = HashSet::new();
    entries
        .iter()
        .find_map(|(key, _)| {
            if seen.contains(key) {
                Some(CommitError::DuplicateStateKey {
                    table,
                    key: key.clone(),
                })
            } else {
                seen.insert(key.clone());
                None
            }
        })
        .map_or(Ok(()), Err)
}

/// P6: Reject payloads exceeding [`MAX_VALUE_SIZE`].
fn validate_payload_sizes(changes: &StateChanges) -> Result<(), CommitError> {
    check_payload_size(&changes.new_analyses, "analysis_outputs")?;
    check_payload_size(&changes.new_transforms, "transform_outputs")?;
    check_payload_size(&changes.new_chunks, "chunk_outputs")?;
    check_payload_size(&changes.new_scrapes, "scrape_outputs")?;
    check_payload_size(&changes.new_snapshots, "snapshots")?;
    Ok(())
}

fn check_payload_size(
    entries: &[([u8; 32], Vec<u8>)],
    table: &'static str,
) -> Result<(), CommitError> {
    entries
        .iter()
        .find_map(|(_, value)| {
            (value.len() > MAX_VALUE_SIZE).then_some(CommitError::PayloadTooLarge {
                table,
                size: value.len(),
                max: MAX_VALUE_SIZE,
            })
        })
        .map_or(Ok(()), Err)
}

/// P4: Reference integrity — every non-zero hash in state rows resolves to a payload entry.
fn validate_reference_integrity(changes: &StateChanges) -> Result<(), CommitError> {
    let analysis_set: HashSet<[u8; 32]> = changes.new_analyses.iter().map(|(h, _)| *h).collect();
    let transform_set: HashSet<[u8; 32]> = changes.new_transforms.iter().map(|(h, _)| *h).collect();
    let chunk_set: HashSet<[u8; 32]> = changes.new_chunks.iter().map(|(h, _)| *h).collect();
    let scrape_set: HashSet<[u8; 32]> = changes.new_scrapes.iter().map(|(h, _)| *h).collect();

    for (_, state) in &changes.updated_files {
        check_ref(
            &state.analysis_hash,
            &analysis_set,
            "file_state",
            "analysis_hash",
            "analysis_outputs",
        )?;
        check_ref(
            &state.transform_hash,
            &transform_set,
            "file_state",
            "transform_hash",
            "transform_outputs",
        )?;
        check_ref(
            &state.chunk_hash,
            &chunk_set,
            "file_state",
            "chunk_hash",
            "chunk_outputs",
        )?;
    }

    for (_, state) in &changes.updated_urls {
        check_ref(
            &state.url_hash,
            &scrape_set,
            "url_state",
            "url_hash",
            "scrape_outputs",
        )?;
    }

    Ok(())
}

fn check_ref(
    hash: &[u8; 32],
    known: &HashSet<[u8; 32]>,
    table: &'static str,
    field: &'static str,
    payload_table: &'static str,
) -> Result<(), CommitError> {
    if *hash == ZERO_HASH {
        return Ok(());
    }
    known
        .contains(hash)
        .then_some(())
        .ok_or_else(|| CommitError::MissingReference {
            table,
            field,
            hash_hex: hash_to_hex(hash),
            payload_table,
        })
}

fn hash_to_hex(hash: &[u8; 32]) -> String {
    hash.iter().fold(String::with_capacity(64), |mut acc, b| {
        let _ = write!(acc, "{b:02x}");
        acc
    })
}

// ---------------------------------------------------------------------------
// Actions: Write helpers (operate within a redb WriteTransaction)
// ---------------------------------------------------------------------------

/// Apply all mutations within the given write transaction.
fn apply_all_writes(
    write_tx: &WriteTransaction,
    changes: &StateChanges,
) -> Result<(), CommitError> {
    // Payload writes first (they're referenced by state entries)
    write_payload_entries(
        write_tx,
        &changes.new_analyses,
        analysis_outputs_table(),
        "analysis_outputs",
    )?;
    write_payload_entries(
        write_tx,
        &changes.new_transforms,
        transform_outputs_table(),
        "transform_outputs",
    )?;
    write_payload_entries(
        write_tx,
        &changes.new_chunks,
        chunk_outputs_table(),
        "chunk_outputs",
    )?;
    write_payload_entries(
        write_tx,
        &changes.new_scrapes,
        scrape_outputs_table(),
        "scrape_outputs",
    )?;
    write_payload_entries(
        write_tx,
        &changes.new_snapshots,
        snapshots_table(),
        "snapshots",
    )?;

    // State upserts
    write_file_states(write_tx, &changes.updated_files)?;
    write_url_states(write_tx, &changes.updated_urls)?;

    // Deletes
    delete_entries(
        write_tx,
        &changes.deleted_files,
        file_state_table(),
        "file_state",
    )?;
    delete_entries(
        write_tx,
        &changes.deleted_urls,
        url_state_table(),
        "url_state",
    )?;
    delete_snapshot_entries(write_tx, &changes.deleted_snapshots)?;

    Ok(())
}

/// Write deduplicated payload entries to a hash-keyed table.
/// Last-write-wins semantics for duplicate keys within the batch.
fn write_payload_entries(
    write_tx: &WriteTransaction,
    entries: &[([u8; 32], Vec<u8>)],
    table_def: TableDefinition<&[u8], &[u8]>,
    table_name: &'static str,
) -> Result<(), CommitError> {
    if entries.is_empty() {
        return Ok(());
    }

    // Dedup: last-write-wins (HashMap::insert overwrites)
    let deduped: HashMap<[u8; 32], &[u8]> = entries
        .iter()
        .map(|(hash, value)| (*hash, value.as_slice()))
        .collect();

    let mut table = open_table_for_write(write_tx, table_def, table_name)?;

    for (hash, new_value) in &deduped {
        let skip = read_and_compare(&table, hash.as_slice(), new_value, table_name)?;
        if !skip {
            table
                .insert(hash.as_slice(), *new_value)
                .map_err(|e| CommitError::WriteFailed {
                    table: table_name,
                    reason: e.to_string(),
                })?;
        }
    }

    Ok(())
}

/// Write file state entries to the string-keyed `file_state` table.
fn write_file_states(
    write_tx: &WriteTransaction,
    entries: &[(String, FileStateRaw)],
) -> Result<(), CommitError> {
    if entries.is_empty() {
        return Ok(());
    }

    let mut table = open_table_for_write(write_tx, file_state_table(), "file_state")?;

    for (path, state) in entries {
        let new_bytes = state.to_bytes();
        let skip = read_and_compare(&table, path.as_str(), &new_bytes, "file_state")?;
        if !skip {
            table
                .insert(path.as_str(), new_bytes.as_slice())
                .map_err(|e| CommitError::WriteFailed {
                    table: "file_state",
                    reason: e.to_string(),
                })?;
        }
    }

    Ok(())
}

/// Write URL state entries to the string-keyed `url_state` table.
fn write_url_states(
    write_tx: &WriteTransaction,
    entries: &[(String, UrlStateRaw)],
) -> Result<(), CommitError> {
    if entries.is_empty() {
        return Ok(());
    }

    let mut table = open_table_for_write(write_tx, url_state_table(), "url_state")?;

    for (url, state) in entries {
        let new_bytes = state.to_bytes();
        let skip = read_and_compare(&table, url.as_str(), &new_bytes, "url_state")?;
        if !skip {
            table
                .insert(url.as_str(), new_bytes.as_slice())
                .map_err(|e| CommitError::WriteFailed {
                    table: "url_state",
                    reason: e.to_string(),
                })?;
        }
    }

    Ok(())
}

/// Delete string-keyed entries from a state table. Silently skips non-existent keys.
fn delete_entries(
    write_tx: &WriteTransaction,
    keys: &[String],
    table_def: TableDefinition<&str, &[u8]>,
    table_name: &'static str,
) -> Result<(), CommitError> {
    if keys.is_empty() {
        return Ok(());
    }

    let mut table = open_table_for_write(write_tx, table_def, table_name)?;

    for key in keys {
        let _ = table
            .remove(key.as_str())
            .map_err(|e: redb::StorageError| CommitError::WriteFailed {
                table: table_name,
                reason: e.to_string(),
            })?;
    }

    Ok(())
}

/// Delete hash-keyed snapshot entries. Silently skips non-existent keys.
fn delete_snapshot_entries(
    write_tx: &WriteTransaction,
    hashes: &[[u8; 32]],
) -> Result<(), CommitError> {
    if hashes.is_empty() {
        return Ok(());
    }

    let mut table = open_table_for_write(write_tx, snapshots_table(), "snapshots")?;

    for hash in hashes {
        let _ = table
            .remove(hash.as_slice())
            .map_err(|e: redb::StorageError| CommitError::WriteFailed {
                table: "snapshots",
                reason: e.to_string(),
            })?;
    }

    Ok(())
}

/// Open a table within a write transaction.
fn open_table_for_write<'a, K: redb::Key + 'static, V: redb::Value + 'static>(
    write_tx: &'a WriteTransaction,
    table_def: TableDefinition<'a, K, V>,
    table_name: &'static str,
) -> Result<redb::Table<'a, K, V>, CommitError> {
    write_tx
        .open_table(table_def)
        .map_err(|e: redb::TableError| CommitError::WriteFailed {
            table: table_name,
            reason: e.to_string(),
        })
}

/// Read existing value and compare with new value. Returns `true` if write should be skipped.
fn read_and_compare<K: redb::Key>(
    table: &redb::Table<'_, K, &'static [u8]>,
    key: K::SelfType<'_>,
    new_value: &[u8],
    table_name: &'static str,
) -> Result<bool, CommitError> {
    let existing = table
        .get(key)
        .map_err(|e: redb::StorageError| CommitError::WriteFailed {
            table: table_name,
            reason: e.to_string(),
        })?;

    Ok(existing.is_some_and(|guard| should_skip_write(guard.value(), new_value)))
}

// ---------------------------------------------------------------------------
// ArchivedRaw — owned wrapper for raw archived bytes (snapshot API stub)
// ---------------------------------------------------------------------------

/// Owned wrapper around raw archived bytes.
///
/// Stub type for the snapshot load API. Will be replaced with a proper
/// rkyv-based implementation in a future bead.
#[derive(Debug)]
pub struct ArchivedRaw {
    #[allow(dead_code)]
    bytes: Vec<u8>,
}

impl ArchivedRaw {
    /// Construct from raw bytes.
    #[must_use]
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    /// Deserialize the archived bytes into type `T`.
    ///
    /// # Errors
    ///
    /// Returns [`super::StateError`] if deserialization fails.
    ///
    /// # TODO
    ///
    /// This is a stub — implementation deferred to snapshot API bead.
    pub fn deserialize<T>(&self) -> Result<T, super::StateError> {
        let _ = &self.bytes;
        todo!()
    }
}

// ---------------------------------------------------------------------------
// StateDb — newtype wrapper over redb::Database
// ---------------------------------------------------------------------------

/// State database providing the two-transaction architecture:
/// - Transaction 1 (read): bulk load all state into memory
/// - Transaction 2 (write): commit all changes atomically via [`commit_changes`](StateDb::commit_changes)
///
/// Wraps a `redb::Database`. Does NOT support in-memory LRU mode.
#[derive(Debug)]
pub struct StateDb {
    db: Database,
}

impl StateDb {
    /// Open the state database at the given path.
    ///
    /// Creates the database and all required tables if they do not exist.
    /// Parent directories are created automatically.
    ///
    /// # Errors
    ///
    /// - [`CommitError::DatabaseOpen`] if redb cannot create/open the file.
    /// - [`CommitError::TableInit`] if any table cannot be created.
    pub fn open(path: &Path) -> Result<Self, CommitError> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(|e| CommitError::DatabaseOpen {
                    path: path.display().to_string(),
                    reason: e.to_string(),
                })?;
            }
        }

        let db = Database::create(path).map_err(|e| CommitError::DatabaseOpen {
            path: path.display().to_string(),
            reason: e.to_string(),
        })?;

        super::initialize_tables(&db).map_err(|e| CommitError::TableInit {
            reason: e.to_string(),
        })?;

        Ok(Self { db })
    }

    /// Open a single shared read transaction for the command's lifetime.
    ///
    /// The caller MUST drop the session before calling [`commit_changes`](Self::commit_changes).
    ///
    /// # Errors
    ///
    /// - [`CommitError::ReadTransaction`] if redb cannot begin a read.
    pub fn begin_read(&self) -> Result<StateReadSession<'_>, CommitError> {
        let read_txn = self
            .db
            .begin_read()
            .map_err(|e| CommitError::ReadTransaction {
                reason: e.to_string(),
            })?;
        Ok(StateReadSession {
            read_txn,
            _phantom: std::marker::PhantomData,
        })
    }

    /// Commit all state changes in exactly one redb write transaction.
    ///
    /// # Preconditions (validated before opening write transaction)
    ///
    /// - No zero hash keys in payload vecs
    /// - No empty/whitespace string keys
    /// - No duplicate string keys in state vecs
    /// - No payloads exceeding 50 `MiB`
    /// - Reference integrity: every non-zero hash resolves
    ///
    /// # Postconditions (on `Ok`)
    ///
    /// - Exactly one write transaction opened and committed
    /// - All upserts applied, all deletes applied
    /// - Duplicates deduplicated (last-write-wins)
    /// - Unchanged rows not rewritten
    ///
    /// # Postconditions (on `Err`)
    ///
    /// - NO writes applied (transaction aborted)
    ///
    /// # Errors
    ///
    /// See [`CommitError`] variants.
    #[allow(clippy::needless_pass_by_value)]
    pub fn commit_changes(&self, changes: StateChanges) -> Result<(), CommitError> {
        // Phase 1: Pure precondition validation (before write transaction)
        validate_all(&changes)?;

        // Phase 2: Open write transaction
        let write_tx = self
            .db
            .begin_write()
            .map_err(|e| CommitError::WriteTransaction {
                reason: e.to_string(),
            })?;

        // Phase 3: Apply all writes within transaction
        apply_all_writes(&write_tx, &changes)?;

        // Phase 4: Commit (transaction is dropped/aborted on any earlier error)
        write_tx.commit().map_err(|e| CommitError::CommitFailed {
            reason: e.to_string(),
        })?;

        Ok(())
    }

    /// Get a reference to the underlying redb database.
    /// Useful for testing and advanced operations.
    #[must_use]
    pub fn database(&self) -> &Database {
        &self.db
    }

    /// Drop the snapshots table.
    ///
    /// # Errors
    ///
    /// Returns [`super::StateError`] if the table cannot be dropped.
    ///
    /// # TODO
    ///
    /// This is a stub — implementation deferred to snapshot API bead.
    pub fn drop_snapshots_table(&self) -> Result<(), super::StateError> {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// StateReadSession — scoped read transaction (bulk-load methods deferred)
// ---------------------------------------------------------------------------

/// A scoped read transaction. One per command run.
/// Must be dropped before calling [`StateDb::commit_changes`].
///
/// Bulk-load methods (`load_file_states`, etc.) are deferred to a separate bead.
pub struct StateReadSession<'db> {
    /// Underlying redb read transaction.
    #[allow(dead_code)]
    read_txn: redb::ReadTransaction,
    _phantom: std::marker::PhantomData<&'db ()>,
}

impl<'db> StateReadSession<'db> {
    /// Bulk-load archived snapshots for the requested hash keys.
    ///
    /// Returns a `HashMap` keyed by the same `[u8; 32]` hashes, with
    /// [`ArchivedRaw`] values that own their bytes independently of the
    /// redb transaction lifetime.
    ///
    /// # Errors
    ///
    /// - [`super::StateError::TableOpenFailed`] if the snapshots table cannot be opened.
    /// - [`super::StateError::StorageError`] if a redb read fails.
    /// - [`super::StateError::ArchiveValidationFailed`] if stored bytes fail rkyv validation.
    ///
    /// # TODO
    ///
    /// This is a stub — implementation deferred to snapshot API bead.
    pub fn load_snapshots(
        &self,
        _keys: &[[u8; 32]],
    ) -> Result<HashMap<[u8; 32], ArchivedRaw>, super::StateError> {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::expect_used)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::unreadable_literal)]
#[allow(clippy::redundant_closure_for_method_calls)]
mod tests {
    use super::*;
    use crate::state::{
        analysis_outputs_table, chunk_outputs_table, file_state_table, metadata_table,
        scrape_outputs_table, snapshots_table, transform_outputs_table, url_state_table,
        FileStateRaw, UrlStateRaw,
    };
    use redb::ReadableTableMetadata;
    use tempfile::TempDir;

    // =======================================================================
    // Test helpers
    // =======================================================================

    fn create_temp_state_db() -> (StateDb, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("state.redb");
        let state_db = StateDb::open(&db_path).unwrap();
        (state_db, temp_dir)
    }

    fn make_file_state_raw(
        analysis: [u8; 32],
        transform: [u8; 32],
        chunk: [u8; 32],
    ) -> FileStateRaw {
        FileStateRaw {
            content_hash: [0xAA; 32],
            config_hash: [0xBB; 32],
            analysis_hash: analysis,
            transform_hash: transform,
            chunk_hash: chunk,
            last_processed_secs: 12345,
            reserved: [0u8; 32],
        }
    }

    fn make_url_state_raw(url_hash: [u8; 32]) -> UrlStateRaw {
        UrlStateRaw {
            content_hash: [0xCC; 32],
            url_hash,
            last_fetched_secs: 67890,
            status_code: 200,
            reserved: [0u8; 46],
        }
    }

    fn make_minimal_valid_state_changes() -> StateChanges {
        StateChanges::empty()
    }

    /// Helper: read a value from a hash-keyed table.
    fn read_hash_table(
        db: &Database,
        table_def: TableDefinition<&[u8], &[u8]>,
        key: &[u8; 32],
    ) -> Option<Vec<u8>> {
        let read_tx = db.begin_read().unwrap();
        let table = read_tx.open_table(table_def).unwrap();
        table
            .get(key.as_slice())
            .unwrap()
            .map(|g| g.value().to_vec())
    }

    /// Helper: read a value from a string-keyed state table.
    fn read_string_table(
        db: &Database,
        table_def: TableDefinition<&str, &[u8]>,
        key: &str,
    ) -> Option<Vec<u8>> {
        let read_tx = db.begin_read().unwrap();
        let table = read_tx.open_table(table_def).unwrap();
        table.get(key).unwrap().map(|g| g.value().to_vec())
    }

    /// Helper: count entries in a table.
    fn count_table_entries(db: &Database, table_name: &str) -> u64 {
        let read_tx = db.begin_read().unwrap();
        match table_name {
            "file_state" => read_tx
                .open_table(file_state_table())
                .unwrap()
                .len()
                .unwrap(),
            "url_state" => read_tx
                .open_table(url_state_table())
                .unwrap()
                .len()
                .unwrap(),
            "analysis_outputs" => read_tx
                .open_table(analysis_outputs_table())
                .unwrap()
                .len()
                .unwrap(),
            "transform_outputs" => read_tx
                .open_table(transform_outputs_table())
                .unwrap()
                .len()
                .unwrap(),
            "chunk_outputs" => read_tx
                .open_table(chunk_outputs_table())
                .unwrap()
                .len()
                .unwrap(),
            "scrape_outputs" => read_tx
                .open_table(scrape_outputs_table())
                .unwrap()
                .len()
                .unwrap(),
            "snapshots" => read_tx
                .open_table(snapshots_table())
                .unwrap()
                .len()
                .unwrap(),
            _ => 0,
        }
    }

    // =======================================================================
    // Behavior 1: StateDb::open succeeds with valid path
    // =======================================================================

    #[test]
    fn state_db_open_returns_ok_when_path_valid() {
        let (state_db, _temp_dir) = create_temp_state_db();
        // Verify begin_read works (proves tables exist)
        let session = state_db.begin_read();
        assert!(
            session.is_ok(),
            "begin_read should succeed on valid StateDb"
        );
    }

    // =======================================================================
    // Behavior 2: StateDb::open returns DatabaseOpen for invalid path
    // =======================================================================

    #[test]
    fn state_db_open_returns_database_open_error_when_path_invalid() {
        let path = std::path::Path::new("/nonexistent_root_xyz_cdocs/deeply/nested/state.redb");
        let result = StateDb::open(path);
        let err = result.expect_err("should fail for nonexistent root");
        let msg = format!("{err}");
        assert!(
            msg.contains("nonexistent_root_xyz_cdocs"),
            "error should reference path: {msg}"
        );
    }

    // =======================================================================
    // Behavior 3: StateDb::open returns TableInit when tables fail
    // =======================================================================
    // NOTE: Hard to trigger deterministically with redb 2.x.
    // Table creation failure requires corrupted database state.
    // The error variant is exercised by the code path; this test documents the difficulty.

    // =======================================================================
    // Behavior 4: StateDb::open returns DatabaseOpen for empty path
    // =======================================================================

    #[test]
    fn state_db_open_returns_database_open_error_when_path_is_empty() {
        let path = std::path::Path::new("");
        let result = StateDb::open(path);
        let err = result.expect_err("should fail for empty path");
        let msg = format!("{err}");
        assert!(
            msg.contains("failed to open"),
            "error should mention open failure: {msg}"
        );
    }

    // =======================================================================
    // Behavior 5: StateDb::begin_read returns session
    // =======================================================================

    #[test]
    fn state_db_begin_read_returns_session_when_db_open() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let _session = state_db.begin_read().expect("begin_read should succeed");
    }

    // =======================================================================
    // Behavior 6: begin_read returns ReadTransaction error (hard to trigger)
    // =======================================================================
    // NOTE: redb 2.x makes it difficult to force a read transaction failure
    // on a healthy database. This variant is tested by redb's own test suite.

    // =======================================================================
    // Behavior 7: commit_changes rejects ZeroHashKey in analysis_outputs
    // =======================================================================

    #[test]
    fn commit_changes_rejects_zero_hash_key_in_analysis_outputs() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = make_minimal_valid_state_changes();
        changes.new_analyses = vec![([0u8; 32], vec![1, 2, 3])];

        let err = state_db
            .commit_changes(changes)
            .expect_err("should reject zero hash");
        assert!(
            matches!(
                err,
                CommitError::ZeroHashKey {
                    table: "analysis_outputs",
                    index: 0
                }
            ),
            "expected ZeroHashKey(analysis_outputs, 0), got: {err}"
        );
    }

    // =======================================================================
    // Behavior 8: ZeroHashKey at index 2 in new_analyses
    // =======================================================================

    #[test]
    fn commit_changes_reports_index_2_for_zero_hash_in_analyses() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = make_minimal_valid_state_changes();
        changes.new_analyses = vec![
            ([1u8; 32], vec![10]),
            ([2u8; 32], vec![20]),
            ([0u8; 32], vec![30]),
        ];

        let err = state_db
            .commit_changes(changes)
            .expect_err("should reject zero hash");
        assert!(
            matches!(
                err,
                CommitError::ZeroHashKey {
                    table: "analysis_outputs",
                    index: 2
                }
            ),
            "expected ZeroHashKey(analysis_outputs, 2), got: {err}"
        );
    }

    // =======================================================================
    // Behavior 9: ZeroHashKey in new_transforms
    // =======================================================================

    #[test]
    fn commit_changes_rejects_zero_hash_key_in_transform_outputs() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = make_minimal_valid_state_changes();
        changes.new_transforms = vec![([0u8; 32], vec![1])];

        let err = state_db
            .commit_changes(changes)
            .expect_err("should reject zero hash");
        assert!(
            matches!(
                err,
                CommitError::ZeroHashKey {
                    table: "transform_outputs",
                    index: 0
                }
            ),
            "expected ZeroHashKey(transform_outputs, 0), got: {err}"
        );
    }

    // =======================================================================
    // Behavior 10: ZeroHashKey in new_chunks
    // =======================================================================

    #[test]
    fn commit_changes_rejects_zero_hash_key_in_chunk_outputs() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = make_minimal_valid_state_changes();
        changes.new_chunks = vec![([0u8; 32], vec![1])];

        let err = state_db
            .commit_changes(changes)
            .expect_err("should reject zero hash");
        assert!(
            matches!(
                err,
                CommitError::ZeroHashKey {
                    table: "chunk_outputs",
                    index: 0
                }
            ),
            "expected ZeroHashKey(chunk_outputs, 0), got: {err}"
        );
    }

    // =======================================================================
    // Behavior 11: ZeroHashKey in new_scrapes
    // =======================================================================

    #[test]
    fn commit_changes_rejects_zero_hash_key_in_scrape_outputs() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = make_minimal_valid_state_changes();
        changes.new_scrapes = vec![([0u8; 32], vec![1])];

        let err = state_db
            .commit_changes(changes)
            .expect_err("should reject zero hash");
        assert!(
            matches!(
                err,
                CommitError::ZeroHashKey {
                    table: "scrape_outputs",
                    index: 0
                }
            ),
            "expected ZeroHashKey(scrape_outputs, 0), got: {err}"
        );
    }

    // =======================================================================
    // Behavior 12: ZeroHashKey in new_snapshots
    // =======================================================================

    #[test]
    fn commit_changes_rejects_zero_hash_key_in_snapshots() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = make_minimal_valid_state_changes();
        changes.new_snapshots = vec![([0u8; 32], vec![1])];

        let err = state_db
            .commit_changes(changes)
            .expect_err("should reject zero hash");
        assert!(
            matches!(
                err,
                CommitError::ZeroHashKey {
                    table: "snapshots",
                    index: 0
                }
            ),
            "expected ZeroHashKey(snapshots, 0), got: {err}"
        );
    }

    // =======================================================================
    // Behavior 13: EmptyStringKey for empty source_path
    // =======================================================================

    #[test]
    fn commit_changes_rejects_empty_source_path_in_updated_files() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = make_minimal_valid_state_changes();
        changes.updated_files = vec![(String::new(), FileStateRaw::zeroed())];

        let err = state_db
            .commit_changes(changes)
            .expect_err("should reject empty key");
        assert!(
            matches!(
                err,
                CommitError::EmptyStringKey {
                    table: "file_state",
                    index: 0
                }
            ),
            "expected EmptyStringKey(file_state, 0), got: {err}"
        );
    }

    // =======================================================================
    // Behavior 14: EmptyStringKey for empty URL
    // =======================================================================

    #[test]
    fn commit_changes_rejects_empty_url_in_updated_urls() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = make_minimal_valid_state_changes();
        changes.updated_urls = vec![(String::new(), UrlStateRaw::zeroed())];

        let err = state_db
            .commit_changes(changes)
            .expect_err("should reject empty key");
        assert!(
            matches!(
                err,
                CommitError::EmptyStringKey {
                    table: "url_state",
                    index: 0
                }
            ),
            "expected EmptyStringKey(url_state, 0), got: {err}"
        );
    }

    // =======================================================================
    // Behavior 15: EmptyStringKey for whitespace-only source_path
    // =======================================================================

    #[test]
    fn commit_changes_rejects_whitespace_only_source_path() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = make_minimal_valid_state_changes();
        changes.updated_files = vec![("   ".to_string(), FileStateRaw::zeroed())];

        let err = state_db
            .commit_changes(changes)
            .expect_err("should reject whitespace key");
        assert!(
            matches!(
                err,
                CommitError::EmptyStringKey {
                    table: "file_state",
                    index: 0
                }
            ),
            "expected EmptyStringKey(file_state, 0), got: {err}"
        );
    }

    // =======================================================================
    // Behavior 16: EmptyStringKey for whitespace-only URL
    // =======================================================================

    #[test]
    fn commit_changes_rejects_whitespace_only_url() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = make_minimal_valid_state_changes();
        changes.updated_urls = vec![("\t\n".to_string(), UrlStateRaw::zeroed())];

        let err = state_db
            .commit_changes(changes)
            .expect_err("should reject whitespace key");
        assert!(
            matches!(
                err,
                CommitError::EmptyStringKey {
                    table: "url_state",
                    index: 0
                }
            ),
            "expected EmptyStringKey(url_state, 0), got: {err}"
        );
    }

    // =======================================================================
    // Behavior 17: DuplicateStateKey in updated_files
    // =======================================================================

    #[test]
    fn commit_changes_rejects_duplicate_source_path_in_updated_files() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = make_minimal_valid_state_changes();
        changes.updated_files = vec![
            ("src/main.rs".to_string(), FileStateRaw::zeroed()),
            ("src/main.rs".to_string(), FileStateRaw::zeroed()),
        ];

        let err = state_db
            .commit_changes(changes)
            .expect_err("should reject duplicate");
        assert!(
            matches!(
                err,
                CommitError::DuplicateStateKey { table: "file_state", ref key }
                if key == "src/main.rs"
            ),
            "expected DuplicateStateKey(file_state, src/main.rs), got: {err}"
        );
    }

    // =======================================================================
    // Behavior 18: DuplicateStateKey in updated_urls
    // =======================================================================

    #[test]
    fn commit_changes_rejects_duplicate_url_in_updated_urls() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = make_minimal_valid_state_changes();
        changes.updated_urls = vec![
            ("https://example.com".to_string(), UrlStateRaw::zeroed()),
            ("https://example.com".to_string(), UrlStateRaw::zeroed()),
        ];

        let err = state_db
            .commit_changes(changes)
            .expect_err("should reject duplicate");
        assert!(
            matches!(
                err,
                CommitError::DuplicateStateKey { table: "url_state", ref key }
                if key == "https://example.com"
            ),
            "expected DuplicateStateKey(url_state, https://example.com), got: {err}"
        );
    }

    // =======================================================================
    // Behavior 19: MissingReference for analysis_hash
    // =======================================================================

    #[test]
    fn commit_changes_rejects_missing_analysis_hash_reference() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = make_minimal_valid_state_changes();
        changes.updated_files = vec![(
            "src/main.rs".to_string(),
            make_file_state_raw([1u8; 32], [0u8; 32], [0u8; 32]),
        )];
        // new_analyses is empty — [1u8; 32] not found

        let err = state_db
            .commit_changes(changes)
            .expect_err("should reject missing ref");
        assert!(
            matches!(
                err,
                CommitError::MissingReference {
                    table: "file_state",
                    field: "analysis_hash",
                    payload_table: "analysis_outputs",
                    ..
                }
            ),
            "expected MissingReference(analysis_hash), got: {err}"
        );
        let hex = match err {
            CommitError::MissingReference { hash_hex, .. } => hash_hex,
            _ => String::new(),
        };
        assert_eq!(
            hex,
            "01".repeat(32),
            "hash_hex should be 64-char hex of [1u8; 32]"
        );
    }

    // =======================================================================
    // Behavior 20: MissingReference for transform_hash
    // =======================================================================

    #[test]
    fn commit_changes_rejects_missing_transform_hash_reference() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = make_minimal_valid_state_changes();
        changes.updated_files = vec![(
            "src/main.rs".to_string(),
            make_file_state_raw([0u8; 32], [2u8; 32], [0u8; 32]),
        )];

        let err = state_db
            .commit_changes(changes)
            .expect_err("should reject missing ref");
        assert!(
            matches!(
                err,
                CommitError::MissingReference {
                    table: "file_state",
                    field: "transform_hash",
                    payload_table: "transform_outputs",
                    ..
                }
            ),
            "expected MissingReference(transform_hash), got: {err}"
        );
    }

    // =======================================================================
    // Behavior 21: MissingReference for chunk_hash
    // =======================================================================

    #[test]
    fn commit_changes_rejects_missing_chunk_hash_reference() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = make_minimal_valid_state_changes();
        changes.updated_files = vec![(
            "src/main.rs".to_string(),
            make_file_state_raw([0u8; 32], [0u8; 32], [3u8; 32]),
        )];

        let err = state_db
            .commit_changes(changes)
            .expect_err("should reject missing ref");
        assert!(
            matches!(
                err,
                CommitError::MissingReference {
                    table: "file_state",
                    field: "chunk_hash",
                    payload_table: "chunk_outputs",
                    ..
                }
            ),
            "expected MissingReference(chunk_hash), got: {err}"
        );
    }

    // =======================================================================
    // Behavior 22: MissingReference for url_hash
    // =======================================================================

    #[test]
    fn commit_changes_rejects_missing_url_hash_reference() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = make_minimal_valid_state_changes();
        changes.updated_urls = vec![(
            "https://example.com".to_string(),
            make_url_state_raw([4u8; 32]),
        )];

        let err = state_db
            .commit_changes(changes)
            .expect_err("should reject missing ref");
        assert!(
            matches!(
                err,
                CommitError::MissingReference {
                    table: "url_state",
                    field: "url_hash",
                    payload_table: "scrape_outputs",
                    ..
                }
            ),
            "expected MissingReference(url_hash), got: {err}"
        );
    }

    // =======================================================================
    // Behavior 23: Zero hashes accepted (no-analysis-yet semantics)
    // =======================================================================

    #[test]
    fn commit_changes_accepts_zero_hashes_as_no_output() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = make_minimal_valid_state_changes();
        changes.updated_files = vec![("src/main.rs".to_string(), FileStateRaw::zeroed())];
        // new_analyses, new_transforms, new_chunks all empty — should succeed

        let result = state_db.commit_changes(changes);
        assert!(result.is_ok(), "zero hashes should be accepted: {result:?}");
    }

    // =======================================================================
    // Behavior 24: PayloadTooLarge in new_analyses
    // =======================================================================

    #[test]
    fn commit_changes_rejects_payload_exceeding_max_value_size_in_analysis_outputs() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = make_minimal_valid_state_changes();
        changes.new_analyses = vec![([1u8; 32], vec![0u8; MAX_VALUE_SIZE + 1])];

        let err = state_db
            .commit_changes(changes)
            .expect_err("should reject oversized payload");
        assert!(
            matches!(
                err,
                CommitError::PayloadTooLarge {
                    table: "analysis_outputs",
                    size: 52428801,
                    max: 52428800,
                }
            ),
            "expected PayloadTooLarge(analysis_outputs), got: {err}"
        );
    }

    // =======================================================================
    // Behavior 25: PayloadTooLarge in new_transforms
    // =======================================================================

    #[test]
    fn commit_changes_rejects_payload_exceeding_max_value_size_in_transform_outputs() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = make_minimal_valid_state_changes();
        changes.new_transforms = vec![([1u8; 32], vec![0u8; MAX_VALUE_SIZE + 1])];

        let err = state_db
            .commit_changes(changes)
            .expect_err("should reject oversized payload");
        assert!(
            matches!(
                err,
                CommitError::PayloadTooLarge {
                    table: "transform_outputs",
                    size: 52428801,
                    max: 52428800,
                }
            ),
            "expected PayloadTooLarge(transform_outputs), got: {err}"
        );
    }

    // =======================================================================
    // Behavior 26: PayloadTooLarge in new_chunks
    // =======================================================================

    #[test]
    fn commit_changes_rejects_payload_exceeding_max_value_size_in_chunk_outputs() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = make_minimal_valid_state_changes();
        changes.new_chunks = vec![([1u8; 32], vec![0u8; MAX_VALUE_SIZE + 1])];

        let err = state_db
            .commit_changes(changes)
            .expect_err("should reject oversized payload");
        assert!(
            matches!(
                err,
                CommitError::PayloadTooLarge {
                    table: "chunk_outputs",
                    size: 52428801,
                    max: 52428800,
                }
            ),
            "expected PayloadTooLarge(chunk_outputs), got: {err}"
        );
    }

    // =======================================================================
    // Behavior 27: PayloadTooLarge in new_scrapes
    // =======================================================================

    #[test]
    fn commit_changes_rejects_payload_exceeding_max_value_size_in_scrape_outputs() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = make_minimal_valid_state_changes();
        changes.new_scrapes = vec![([1u8; 32], vec![0u8; MAX_VALUE_SIZE + 1])];

        let err = state_db
            .commit_changes(changes)
            .expect_err("should reject oversized payload");
        assert!(
            matches!(
                err,
                CommitError::PayloadTooLarge {
                    table: "scrape_outputs",
                    size: 52428801,
                    max: 52428800,
                }
            ),
            "expected PayloadTooLarge(scrape_outputs), got: {err}"
        );
    }

    // =======================================================================
    // Behavior 28: PayloadTooLarge in new_snapshots
    // =======================================================================

    #[test]
    fn commit_changes_rejects_payload_exceeding_max_value_size_in_snapshots() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = make_minimal_valid_state_changes();
        changes.new_snapshots = vec![([1u8; 32], vec![0u8; MAX_VALUE_SIZE + 1])];

        let err = state_db
            .commit_changes(changes)
            .expect_err("should reject oversized payload");
        assert!(
            matches!(
                err,
                CommitError::PayloadTooLarge {
                    table: "snapshots",
                    size: 52428801,
                    max: 52428800,
                }
            ),
            "expected PayloadTooLarge(snapshots), got: {err}"
        );
    }

    // =======================================================================
    // Behavior 29: Persists updated_files to file_state table
    // =======================================================================

    #[test]
    fn commit_changes_persists_updated_files_to_file_state_table() {
        let (state_db, _temp_dir) = create_temp_state_db();

        let file_a = make_file_state_raw([0u8; 32], [0u8; 32], [0u8; 32]);
        let file_b = make_file_state_raw([0u8; 32], [0u8; 32], [0u8; 32]);
        let mut changes = make_minimal_valid_state_changes();
        changes.updated_files = vec![
            ("src/main.rs".to_string(), file_a),
            ("docs/README.md".to_string(), file_b),
        ];

        state_db
            .commit_changes(changes)
            .expect("commit should succeed");

        let db = state_db.database();
        let bytes_a = read_string_table(db, file_state_table(), "src/main.rs");
        let bytes_b = read_string_table(db, file_state_table(), "docs/README.md");

        assert_eq!(
            bytes_a.as_ref().map(|v| v.as_slice()),
            Some(file_a.to_bytes().as_slice())
        );
        assert_eq!(
            bytes_b.as_ref().map(|v| v.as_slice()),
            Some(file_b.to_bytes().as_slice())
        );
    }

    // =======================================================================
    // Behavior 30: Deletes files and skips nonexistent
    // =======================================================================

    #[test]
    fn commit_changes_deletes_files_and_skips_nonexistent() {
        let (state_db, _temp_dir) = create_temp_state_db();

        // Pre-populate
        let state = FileStateRaw::zeroed();
        let mut setup = make_minimal_valid_state_changes();
        setup.updated_files = vec![("old_file.rs".to_string(), state)];
        state_db
            .commit_changes(setup)
            .expect("setup commit should succeed");

        // Delete
        let mut changes = make_minimal_valid_state_changes();
        changes.deleted_files = vec!["old_file.rs".to_string(), "nonexistent.rs".to_string()];

        state_db
            .commit_changes(changes)
            .expect("delete commit should succeed");

        let db = state_db.database();
        assert!(
            read_string_table(db, file_state_table(), "old_file.rs").is_none(),
            "old_file.rs should be deleted"
        );
    }

    // =======================================================================
    // Behavior 31: Persists new_analyses to analysis_outputs
    // =======================================================================

    #[test]
    fn commit_changes_persists_new_analyses_to_analysis_outputs() {
        let (state_db, _temp_dir) = create_temp_state_db();

        let hash_a = [1u8; 32];
        let hash_b = [2u8; 32];
        let bytes_a = vec![10, 20, 30];
        let bytes_b = vec![40, 50, 60];

        let mut changes = make_minimal_valid_state_changes();
        changes.new_analyses = vec![(hash_a, bytes_a.clone()), (hash_b, bytes_b.clone())];

        state_db
            .commit_changes(changes)
            .expect("commit should succeed");

        let db = state_db.database();
        assert_eq!(
            read_hash_table(db, analysis_outputs_table(), &hash_a),
            Some(bytes_a)
        );
        assert_eq!(
            read_hash_table(db, analysis_outputs_table(), &hash_b),
            Some(bytes_b)
        );
    }

    // =======================================================================
    // Behavior 32: Persists new_transforms
    // =======================================================================

    #[test]
    fn commit_changes_persists_new_transforms_to_transform_outputs() {
        let (state_db, _temp_dir) = create_temp_state_db();

        let hash_a = [1u8; 32];
        let bytes_a = vec![100, 200];

        let mut changes = make_minimal_valid_state_changes();
        changes.new_transforms = vec![(hash_a, bytes_a.clone())];

        state_db
            .commit_changes(changes)
            .expect("commit should succeed");

        let db = state_db.database();
        assert_eq!(
            read_hash_table(db, transform_outputs_table(), &hash_a),
            Some(bytes_a)
        );
    }

    // =======================================================================
    // Behavior 33: Persists new_chunks
    // =======================================================================

    #[test]
    fn commit_changes_persists_new_chunks_to_chunk_outputs() {
        let (state_db, _temp_dir) = create_temp_state_db();

        let hash_a = [1u8; 32];
        let bytes_a = vec![5, 6, 7, 8];

        let mut changes = make_minimal_valid_state_changes();
        changes.new_chunks = vec![(hash_a, bytes_a.clone())];

        state_db
            .commit_changes(changes)
            .expect("commit should succeed");

        let db = state_db.database();
        assert_eq!(
            read_hash_table(db, chunk_outputs_table(), &hash_a),
            Some(bytes_a)
        );
    }

    // =======================================================================
    // Behavior 34: Persists updated_urls to url_state
    // =======================================================================

    #[test]
    fn commit_changes_persists_updated_urls_to_url_state() {
        let (state_db, _temp_dir) = create_temp_state_db();

        let url_state = make_url_state_raw([0u8; 32]);
        let mut changes = make_minimal_valid_state_changes();
        changes.updated_urls = vec![("https://example.com".to_string(), url_state)];

        state_db
            .commit_changes(changes)
            .expect("commit should succeed");

        let db = state_db.database();
        let stored = read_string_table(db, url_state_table(), "https://example.com");
        assert_eq!(
            stored.as_ref().map(|v| v.as_slice()),
            Some(url_state.to_bytes().as_slice())
        );
    }

    // =======================================================================
    // Behavior 35: Deletes URLs and skips nonexistent
    // =======================================================================

    #[test]
    fn commit_changes_deletes_urls_and_skips_nonexistent() {
        let (state_db, _temp_dir) = create_temp_state_db();

        // Pre-populate
        let url_state = UrlStateRaw::zeroed();
        let mut setup = make_minimal_valid_state_changes();
        setup.updated_urls = vec![("https://old.com".to_string(), url_state)];
        state_db
            .commit_changes(setup)
            .expect("setup commit should succeed");

        // Delete
        let mut changes = make_minimal_valid_state_changes();
        changes.deleted_urls = vec![
            "https://old.com".to_string(),
            "https://nonexistent.com".to_string(),
        ];

        state_db
            .commit_changes(changes)
            .expect("delete commit should succeed");

        let db = state_db.database();
        assert!(
            read_string_table(db, url_state_table(), "https://old.com").is_none(),
            "old URL should be deleted"
        );
    }

    // =======================================================================
    // Behavior 36: Persists new_scrapes
    // =======================================================================

    #[test]
    fn commit_changes_persists_new_scrapes_to_scrape_outputs() {
        let (state_db, _temp_dir) = create_temp_state_db();

        let hash_a = [1u8; 32];
        let bytes_a = vec![99, 88, 77];

        let mut changes = make_minimal_valid_state_changes();
        changes.new_scrapes = vec![(hash_a, bytes_a.clone())];

        state_db
            .commit_changes(changes)
            .expect("commit should succeed");

        let db = state_db.database();
        assert_eq!(
            read_hash_table(db, scrape_outputs_table(), &hash_a),
            Some(bytes_a)
        );
    }

    // =======================================================================
    // Behavior 37: Persists new_snapshots
    // =======================================================================

    #[test]
    fn commit_changes_persists_new_snapshots_to_snapshots_table() {
        let (state_db, _temp_dir) = create_temp_state_db();

        let hash_a = [1u8; 32];
        let bytes_a = vec![55, 66];

        let mut changes = make_minimal_valid_state_changes();
        changes.new_snapshots = vec![(hash_a, bytes_a.clone())];

        state_db
            .commit_changes(changes)
            .expect("commit should succeed");

        let db = state_db.database();
        assert_eq!(
            read_hash_table(db, snapshots_table(), &hash_a),
            Some(bytes_a)
        );
    }

    // =======================================================================
    // Behavior 38: Deletes snapshots and skips nonexistent
    // =======================================================================

    #[test]
    fn commit_changes_deletes_snapshots_and_skips_nonexistent() {
        let (state_db, _temp_dir) = create_temp_state_db();

        let hash_old = [0xABu8; 32];
        let hash_missing = [0xCDu8; 32];

        // Pre-populate
        let mut setup = make_minimal_valid_state_changes();
        setup.new_snapshots = vec![(hash_old, vec![1, 2, 3])];
        state_db
            .commit_changes(setup)
            .expect("setup commit should succeed");

        // Delete
        let mut changes = make_minimal_valid_state_changes();
        changes.deleted_snapshots = vec![hash_old, hash_missing];

        state_db
            .commit_changes(changes)
            .expect("delete commit should succeed");

        let db = state_db.database();
        assert!(
            read_hash_table(db, snapshots_table(), &hash_old).is_none(),
            "old snapshot should be deleted"
        );
    }

    // =======================================================================
    // Behavior 39: Deduplication (last-write-wins)
    // =======================================================================

    #[test]
    fn commit_changes_deduplicates_payload_entries_last_write_wins() {
        let (state_db, _temp_dir) = create_temp_state_db();

        let hash_a = [1u8; 32];
        let hash_b = [2u8; 32];
        let bytes_v1 = vec![10, 20];
        let bytes_v2 = vec![30, 40];
        let bytes_v3 = vec![50, 60];

        let mut changes = make_minimal_valid_state_changes();
        changes.new_analyses = vec![
            (hash_a, bytes_v1.clone()),
            (hash_b, bytes_v2.clone()),
            (hash_a, bytes_v3.clone()), // duplicate key
        ];

        state_db
            .commit_changes(changes)
            .expect("commit should succeed");

        let db = state_db.database();
        assert_eq!(
            read_hash_table(db, analysis_outputs_table(), &hash_a),
            Some(bytes_v3),
            "hash_a should have last-write-wins value (v3)"
        );
        assert_eq!(
            read_hash_table(db, analysis_outputs_table(), &hash_b),
            Some(bytes_v2),
            "hash_b should have v2"
        );
        assert_eq!(
            count_table_entries(db, "analysis_outputs"),
            2,
            "exactly 2 unique keys should exist"
        );
    }

    // =======================================================================
    // Behavior 40: should_skip_write returns true for identical bytes
    // =======================================================================

    #[test]
    fn should_skip_write_returns_true_when_bytes_identical() {
        assert!(should_skip_write(&[1, 2, 3, 4], &[1, 2, 3, 4]));
        assert!(should_skip_write(&[], &[]));
        assert!(should_skip_write(&[0xFF; 100], &[0xFF; 100]));
    }

    // =======================================================================
    // Behavior 41: should_skip_write returns false for different bytes
    // =======================================================================

    #[test]
    fn should_skip_write_returns_false_when_bytes_differ() {
        assert!(!should_skip_write(&[1, 2, 3, 4], &[1, 2, 3, 5]));
        assert!(!should_skip_write(&[], &[1]));
        assert!(!should_skip_write(&[1, 2], &[1]));
        assert!(!should_skip_write(&[1], &[1, 2]));
    }

    // =======================================================================
    // Behavior 42: Skips unchanged rows (integration verification)
    // =======================================================================

    #[test]
    fn commit_changes_skips_unchanged_rows_without_rewriting() {
        let (state_db, _temp_dir) = create_temp_state_db();

        let state = make_file_state_raw([0u8; 32], [0u8; 32], [0u8; 32]);

        // Write initial
        let mut setup = make_minimal_valid_state_changes();
        setup.updated_files = vec![("src/main.rs".to_string(), state)];
        state_db
            .commit_changes(setup)
            .expect("initial commit should succeed");

        // Re-commit identical data
        let mut changes = make_minimal_valid_state_changes();
        changes.updated_files = vec![("src/main.rs".to_string(), state)];

        state_db
            .commit_changes(changes)
            .expect("re-commit should succeed");

        // Verify the value is still correct
        let db = state_db.database();
        let stored = read_string_table(db, file_state_table(), "src/main.rs");
        assert_eq!(
            stored.as_ref().map(|v| v.as_slice()),
            Some(state.to_bytes().as_slice())
        );
    }

    // =======================================================================
    // Behavior 43: Rolls back ALL writes on validation failure
    // =======================================================================

    #[test]
    fn commit_changes_rolls_back_all_writes_when_validation_fails() {
        let (state_db, _temp_dir) = create_temp_state_db();

        let valid_state = FileStateRaw::zeroed();

        let mut changes = make_minimal_valid_state_changes();
        changes.updated_files = vec![("valid.rs".to_string(), valid_state)];
        // This triggers a validation failure (zero hash in payload)
        changes.new_analyses = vec![([0u8; 32], vec![1, 2, 3])];

        let result = state_db.commit_changes(changes);
        assert!(
            matches!(result, Err(CommitError::ZeroHashKey { .. })),
            "should fail with ZeroHashKey: {result:?}"
        );

        // Verify valid.rs was NOT written (rolled back)
        let db = state_db.database();
        assert!(
            read_string_table(db, file_state_table(), "valid.rs").is_none(),
            "no writes should be visible after validation failure"
        );
    }

    // =======================================================================
    // Behavior 44: No-op batch succeeds
    // =======================================================================

    #[test]
    fn commit_changes_succeeds_with_noop_empty_batch() {
        let (state_db, _temp_dir) = create_temp_state_db();

        let changes = make_minimal_valid_state_changes();
        let result = state_db.commit_changes(changes);
        assert!(result.is_ok(), "empty batch should succeed: {result:?}");
    }

    // =======================================================================
    // Behavior 45: Mixed mutations atomically
    // =======================================================================

    #[test]
    fn commit_changes_applies_mixed_mutations_atomically_in_single_transaction() {
        let (state_db, _temp_dir) = create_temp_state_db();

        // Pre-populate
        let old_state = FileStateRaw::zeroed();
        let old_url = UrlStateRaw::zeroed();
        let hash_old = [0x99u8; 32];
        let mut setup = make_minimal_valid_state_changes();
        setup.updated_files = vec![("old.rs".to_string(), old_state)];
        setup.updated_urls = vec![("https://old.com".to_string(), old_url)];
        setup.new_analyses = vec![(hash_old, vec![0])];
        state_db
            .commit_changes(setup)
            .expect("setup should succeed");

        // Mixed batch
        let hash_new = [0xA1u8; 32];
        let hash_t = [0xA2u8; 32];
        let hash_c = [0xA3u8; 32];
        let hash_s = [0xA4u8; 32];
        let hash_snap = [0xA5u8; 32];

        let new_file_state = make_file_state_raw(hash_new, hash_t, hash_c);
        let new_url_state = make_url_state_raw(hash_s);

        let mut changes = make_minimal_valid_state_changes();
        changes.updated_files = vec![("new.rs".to_string(), new_file_state)];
        changes.deleted_files = vec!["old.rs".to_string()];
        changes.new_analyses = vec![(hash_new, vec![10, 20])];
        changes.new_transforms = vec![(hash_t, vec![30])];
        changes.new_chunks = vec![(hash_c, vec![40])];
        changes.updated_urls = vec![("https://new.com".to_string(), new_url_state)];
        changes.deleted_urls = vec!["https://old.com".to_string()];
        changes.new_scrapes = vec![(hash_s, vec![50])];
        changes.new_snapshots = vec![(hash_snap, vec![60])];
        changes.deleted_snapshots = vec![hash_old];

        state_db
            .commit_changes(changes)
            .expect("mixed commit should succeed");

        let db = state_db.database();

        // File state: new.rs present, old.rs absent
        assert!(
            read_string_table(db, file_state_table(), "new.rs").is_some(),
            "new.rs should exist"
        );
        assert!(
            read_string_table(db, file_state_table(), "old.rs").is_none(),
            "old.rs should be deleted"
        );

        // URL state: new.com present, old.com absent
        assert!(
            read_string_table(db, url_state_table(), "https://new.com").is_some(),
            "new.com should exist"
        );
        assert!(
            read_string_table(db, url_state_table(), "https://old.com").is_none(),
            "old.com should be deleted"
        );

        // Payloads present
        assert_eq!(
            read_hash_table(db, analysis_outputs_table(), &hash_new),
            Some(vec![10, 20])
        );
        assert_eq!(
            read_hash_table(db, transform_outputs_table(), &hash_t),
            Some(vec![30])
        );
        assert_eq!(
            read_hash_table(db, chunk_outputs_table(), &hash_c),
            Some(vec![40])
        );
        assert_eq!(
            read_hash_table(db, scrape_outputs_table(), &hash_s),
            Some(vec![50])
        );
        assert_eq!(
            read_hash_table(db, snapshots_table(), &hash_snap),
            Some(vec![60])
        );

        // Old snapshot deleted
        assert!(
            read_hash_table(db, snapshots_table(), &hash_old).is_none(),
            "old snapshot should not exist (was in deleted_snapshots)"
        );
    }

    // =======================================================================
    // Behavior 49: Long source_path boundary
    // =======================================================================

    #[test]
    fn commit_changes_handles_long_source_path_approaching_redb_key_limit() {
        let (state_db, _temp_dir) = create_temp_state_db();

        // 4096-char path
        let long_path: String = "a".repeat(4096);
        let mut changes = make_minimal_valid_state_changes();
        changes.updated_files = vec![(long_path.clone(), FileStateRaw::zeroed())];

        let result = state_db.commit_changes(changes);
        // Either succeeds or returns WriteFailed — both are acceptable
        match result {
            Ok(()) => {
                let db = state_db.database();
                let stored = read_string_table(db, file_state_table(), &long_path);
                assert!(stored.is_some(), "long path should be stored if accepted");
            }
            Err(CommitError::WriteFailed {
                table: "file_state",
                ..
            }) => {
                // redb rejected the oversized key — acceptable
            }
            Err(e) => panic!("unexpected error: {e}"),
        }
    }

    // =======================================================================
    // Proptest 1: Zero-hash scan is exhaustive
    // =======================================================================

    #[test]
    fn proptest_zero_hash_scan_exhaustive() {
        use proptest::prelude::*;

        proptest!(|(
            hash_a in proptest::array::uniform32(1u8..=255u8),
            hash_b in proptest::array::uniform32(1u8..=255u8),
            inject_zero in 0u8..5, // which vec to inject into
        )| {
            let mut changes = StateChanges::empty();
            let entries = vec![(hash_a, vec![1]), (hash_b, vec![2])];

            match inject_zero {
                0 => changes.new_analyses = vec![([0u8; 32], vec![0])],
                1 => changes.new_transforms = vec![([0u8; 32], vec![0])],
                2 => changes.new_chunks = vec![([0u8; 32], vec![0])],
                3 => changes.new_scrapes = vec![([0u8; 32], vec![0])],
                4 => changes.new_snapshots = vec![([0u8; 32], vec![0])],
                _ => {}
            }

            // Add valid entries too
            changes.new_analyses = [changes.new_analyses, entries.clone()].concat();
            changes.new_transforms = [changes.new_transforms, entries.clone()].concat();
            changes.new_chunks = [changes.new_chunks, entries.clone()].concat();
            changes.new_scrapes = [changes.new_scrapes, entries.clone()].concat();
            changes.new_snapshots = [changes.new_snapshots, entries.clone()].concat();

            let result = validate_no_zero_hashes(&changes);
            prop_assert!(
                matches!(result, Err(CommitError::ZeroHashKey { .. })),
                "must detect zero hash in vec {inject_zero}"
            );
        });
    }

    // =======================================================================
    // Proptest 2: Duplicate detection is order-independent
    // =======================================================================

    #[test]
    fn proptest_duplicate_detection_order_independent() {
        use proptest::prelude::*;

        proptest!(|(
            keys in proptest::collection::vec(".*", 1..10),
        )| {
            let mut changes = StateChanges::empty();
            changes.updated_files = keys.iter().enumerate().map(|(i, k)| {
                (k.clone(), FileStateRaw { content_hash: [i as u8; 32], ..FileStateRaw::zeroed() })
            }).collect();

            let has_dupes = keys.len() != keys.iter().collect::<HashSet<_>>().len();
            let result = validate_no_duplicate_keys(&changes);

            if has_dupes {
                assert!(matches!(result, Err(CommitError::DuplicateStateKey { .. })));
            } else {
                prop_assert!(result.is_ok());
            }
        });
    }

    // =======================================================================
    // Proptest 3: Reference integrity is complete
    // =======================================================================

    #[test]
    fn proptest_reference_integrity_complete() {
        use proptest::prelude::*;

        proptest!(|(
            analysis_hash in proptest::array::uniform32(1u8..=255u8),
            transform_hash in proptest::array::uniform32(1u8..=255u8),
            chunk_hash in proptest::array::uniform32(1u8..=255u8),
            omit_analysis in proptest::bool::ANY,
        )| {
            let mut changes = StateChanges::empty();

            if !omit_analysis {
                changes.new_analyses = vec![(analysis_hash, vec![1])];
            }
            changes.new_transforms = vec![(transform_hash, vec![2])];
            changes.new_chunks = vec![(chunk_hash, vec![3])];

            changes.updated_files = vec![(
                "test.rs".to_string(),
                make_file_state_raw(analysis_hash, transform_hash, chunk_hash),
            )];

            let result = validate_reference_integrity(&changes);
            if omit_analysis {
                assert!(matches!(
                    result,
                    Err(CommitError::MissingReference { field: "analysis_hash", .. })
                ));
            } else {
                prop_assert!(result.is_ok());
            }
        });
    }

    // =======================================================================
    // Proptest 5: Atomicity under mixed valid/invalid batches
    // =======================================================================

    #[test]
    fn proptest_atomicity_mixed_batches() {
        use proptest::prelude::*;

        proptest!(|(
            valid_hash in proptest::array::uniform32(1u8..=255u8),
            valid_bytes in proptest::collection::vec(0u8..=255u8, 0..100),
        )| {
            let temp_dir = TempDir::new().unwrap();
            let db_path = temp_dir.path().join("proptest_atomic.redb");
            let state_db = StateDb::open(&db_path).unwrap();

            // Pre-populate
            let mut setup = StateChanges::empty();
            setup.new_analyses = vec![(valid_hash, valid_bytes.clone())];
            state_db.commit_changes(setup).unwrap();

            // Attempt invalid commit (zero hash)
            let mut invalid = StateChanges::empty();
            invalid.new_analyses = vec![([0u8; 32], vec![99])];
            let err = state_db.commit_changes(invalid);
            assert!(
                matches!(err, Err(CommitError::ZeroHashKey { .. })),
                "invalid commit should fail with ZeroHashKey: {err:?}"
            );

            // Verify original data is intact
            let db = state_db.database();
            let stored = read_hash_table(db, analysis_outputs_table(), &valid_hash);
            prop_assert_eq!(stored, Some(valid_bytes));
        });
    }

    // =======================================================================
    // Proptest 6: should_skip_write correctness
    // =======================================================================

    #[test]
    fn proptest_should_skip_write_correctness() {
        use proptest::prelude::*;

        proptest!(|(
            a in proptest::collection::vec(0u8..=255u8, 0..256),
            b in proptest::collection::vec(0u8..=255u8, 0..256),
        )| {
            let expected = a == b;
            prop_assert_eq!(should_skip_write(&a, &b), expected);
        });
    }

    // =======================================================================
    // Error variant completeness: TableInit
    // =======================================================================

    #[test]
    fn commit_error_table_init_display_contains_reason() {
        let err = CommitError::TableInit {
            reason: "corrupt tables".to_string(),
        };
        let msg = format!("{err}");
        assert!(
            msg.contains("corrupt tables"),
            "TableInit display should contain reason: {msg}"
        );
        assert!(
            msg.contains("initialize tables"),
            "TableInit display should mention table init: {msg}"
        );
    }

    // =======================================================================
    // Error variant completeness: ReadTransaction
    // =======================================================================

    #[test]
    fn commit_error_read_transaction_display_contains_reason() {
        let err = CommitError::ReadTransaction {
            reason: "read tx failed".to_string(),
        };
        let msg = format!("{err}");
        assert!(
            msg.contains("read tx failed"),
            "ReadTransaction display should contain reason: {msg}"
        );
        assert!(
            msg.contains("read transaction"),
            "ReadTransaction display should mention read transaction: {msg}"
        );
    }

    // =======================================================================
    // Error variant completeness: WriteTransaction
    // =======================================================================

    #[test]
    fn commit_error_write_transaction_display_contains_reason() {
        let err = CommitError::WriteTransaction {
            reason: "write tx failed".to_string(),
        };
        let msg = format!("{err}");
        assert!(
            msg.contains("write tx failed"),
            "WriteTransaction display should contain reason: {msg}"
        );
        assert!(
            msg.contains("write transaction"),
            "WriteTransaction display should mention write transaction: {msg}"
        );
    }

    // =======================================================================
    // Error variant completeness: CommitFailed
    // =======================================================================

    #[test]
    fn commit_error_commit_failed_display_contains_reason() {
        let err = CommitError::CommitFailed {
            reason: "commit aborted".to_string(),
        };
        let msg = format!("{err}");
        assert!(
            msg.contains("commit aborted"),
            "CommitFailed display should contain reason: {msg}"
        );
        assert!(
            msg.contains("commit write transaction"),
            "CommitFailed display should mention commit: {msg}"
        );
    }

    // =======================================================================
    // Error variant completeness: ReadFailed
    // =======================================================================

    #[test]
    fn commit_error_read_failed_display_contains_table_and_reason() {
        let err = CommitError::ReadFailed {
            table: "analysis_outputs",
            reason: "disk error".to_string(),
        };
        let msg = format!("{err}");
        assert!(
            msg.contains("analysis_outputs"),
            "ReadFailed display should contain table name: {msg}"
        );
        assert!(
            msg.contains("disk error"),
            "ReadFailed display should contain reason: {msg}"
        );
        assert!(
            msg.contains("read failed"),
            "ReadFailed display should mention read failure: {msg}"
        );
    }

    // =======================================================================
    // Boundary: payload at exactly MAX_VALUE_SIZE should be accepted
    // =======================================================================

    #[test]
    fn commit_changes_accepts_payload_exactly_at_max_value_size_boundary() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = StateChanges::empty();
        changes.new_analyses = vec![([1u8; 32], vec![0u8; MAX_VALUE_SIZE])];
        let result = state_db.commit_changes(changes);
        assert!(
            result.is_ok(),
            "payload at exactly MAX_VALUE_SIZE should be accepted: {result:?}"
        );
    }

    // =======================================================================
    // G01 (B02): StateDb::open creates parent directories when missing
    // =======================================================================

    #[test]
    fn state_db_open_creates_parent_directories_when_missing() {
        let temp_dir = TempDir::new().unwrap();
        let nested_path = temp_dir.path().join("deeply/nested/dir/state.redb");

        let state_db = StateDb::open(&nested_path).expect("open should succeed");

        // Verify all parent directories were created
        assert!(
            temp_dir.path().join("deeply").is_dir(),
            "deeply/ should be a directory"
        );
        assert!(
            temp_dir.path().join("deeply/nested").is_dir(),
            "deeply/nested/ should be a directory"
        );
        assert!(
            temp_dir.path().join("deeply/nested/dir").is_dir(),
            "deeply/nested/dir/ should be a directory"
        );
        assert!(nested_path.exists(), "state.redb file should exist");

        // Verify the returned StateDb is usable
        let _session = state_db.begin_read().expect("begin_read should succeed");
    }

    // =======================================================================
    // G02 (B07): StateDb::open handles filename-only path
    // =======================================================================

    #[test]
    fn state_db_open_handles_filename_only_path_without_create_dir() {
        let temp_dir = TempDir::new().unwrap();
        // Use just a filename — parent is empty string, no create_dir_all needed
        let db_path = temp_dir.path().join("state.redb");

        let state_db = StateDb::open(&db_path).expect("open with filename path should succeed");
        let _session = state_db.begin_read().expect("begin_read should succeed");
    }

    // =======================================================================
    // G03 (B09): StateDb::open succeeds with unicode and spaces in path
    // =======================================================================

    #[test]
    fn state_db_open_succeeds_with_unicode_and_spaces_in_path() {
        let temp_dir = TempDir::new().unwrap();
        let unicode_path = temp_dir.path().join("path with spaces/数据库/state.redb");

        let state_db = StateDb::open(&unicode_path).expect("open with unicode path should succeed");

        assert!(
            temp_dir.path().join("path with spaces").is_dir(),
            "directory with spaces should exist"
        );
        assert!(
            temp_dir.path().join("path with spaces/数据库").is_dir(),
            "unicode directory should exist"
        );

        let _session = state_db.begin_read().expect("begin_read should succeed");
    }

    // =======================================================================
    // G04 (B10): StateDb::open returns DatabaseOpen on read-only parent (Unix)
    // =======================================================================

    #[test]
    #[cfg(unix)]
    fn state_db_open_returns_database_open_error_on_read_only_parent() {
        let temp_dir = TempDir::new().unwrap();
        let readonly_dir = temp_dir.path().join("readonly");
        std::fs::create_dir(&readonly_dir).unwrap();

        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&readonly_dir, std::fs::Permissions::from_mode(0o444)).unwrap();

        let db_path = readonly_dir.join("state.redb");
        let result = StateDb::open(&db_path);

        let err = result.expect_err("should fail on read-only parent");
        let msg = format!("{err}");
        assert!(
            msg.contains("readonly") || msg.contains("failed to open"),
            "error should reference path or failure: {msg}"
        );

        // Restore for cleanup
        let _ = std::fs::set_permissions(&readonly_dir, std::fs::Permissions::from_mode(0o755));
    }

    // =======================================================================
    // G05 (B11): StateDb::open creates deeply nested parent directories
    // =======================================================================

    #[test]
    fn state_db_open_creates_deeply_nested_parent_directories() {
        let temp_dir = TempDir::new().unwrap();
        let deep_path = temp_dir.path().join("a/b/c/d/e/f/g/h/i/j/state.redb");

        let state_db = StateDb::open(&deep_path).expect("open should succeed");

        assert!(
            temp_dir.path().join("a/b/c/d/e/f/g/h/i/j").is_dir(),
            "all 10 nested directories should exist"
        );

        let _session = state_db.begin_read().expect("begin_read should succeed");
    }

    // =======================================================================
    // G06 (B38): commit_changes accepts 0-byte payload in analyses
    // =======================================================================

    #[test]
    fn commit_changes_accepts_zero_byte_payload_in_analyses() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let hash_key = [1u8; 32];
        let mut changes = StateChanges::empty();
        changes.new_analyses = vec![(hash_key, vec![])];

        state_db
            .commit_changes(changes)
            .expect("zero-byte payload should succeed");

        let db = state_db.database();
        let stored = read_hash_table(db, analysis_outputs_table(), &hash_key);
        assert_eq!(
            stored,
            Some(vec![]),
            "zero-byte payload should be stored as empty vec"
        );
    }

    // =======================================================================
    // G07 (B39): commit_changes succeeds with partial vec population
    // =======================================================================

    #[test]
    fn commit_changes_succeeds_with_only_analyses_populated() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let hash_a = [1u8; 32];
        let mut changes = StateChanges::empty();
        changes.new_analyses = vec![(hash_a, vec![10])];

        state_db
            .commit_changes(changes)
            .expect("partial population should succeed");

        let db = state_db.database();
        assert_eq!(
            read_hash_table(db, analysis_outputs_table(), &hash_a),
            Some(vec![10]),
            "analysis should be persisted"
        );
        assert_eq!(
            count_table_entries(db, "transform_outputs"),
            0,
            "transform_outputs should be empty"
        );
        assert_eq!(
            count_table_entries(db, "chunk_outputs"),
            0,
            "chunk_outputs should be empty"
        );
        assert_eq!(
            count_table_entries(db, "scrape_outputs"),
            0,
            "scrape_outputs should be empty"
        );
    }

    // =======================================================================
    // G08 (B56): commit_changes persists large batch (100 entries per vec)
    // =======================================================================

    #[test]
    fn commit_changes_persists_batch_with_100_entries_per_vec() {
        let (state_db, _temp_dir) = create_temp_state_db();
        let mut changes = StateChanges::empty();

        // 100 file states
        let mut file_states = Vec::with_capacity(100);
        for i in 0..100u8 {
            let path = format!("file_{i}.rs");
            let state = FileStateRaw {
                content_hash: [i; 32],
                config_hash: [i.saturating_add(1); 32],
                analysis_hash: [0u8; 32],
                transform_hash: [0u8; 32],
                chunk_hash: [0u8; 32],
                last_processed_secs: u64::from(i),
                reserved: [0u8; 32],
            };
            file_states.push((path, state));
        }
        changes.updated_files = file_states;

        // 100 analysis outputs (start at 1 to avoid zero hash)
        let mut analyses = Vec::with_capacity(100);
        for i in 0..100u8 {
            let mut hash = [0u8; 32];
            hash[0] = i.wrapping_add(1);
            analyses.push((hash, vec![i]));
        }
        changes.new_analyses = analyses;

        state_db
            .commit_changes(changes)
            .expect("large batch should succeed");

        let db = state_db.database();
        assert_eq!(
            count_table_entries(db, "file_state"),
            100,
            "file_state should have 100 entries"
        );
        assert_eq!(
            count_table_entries(db, "analysis_outputs"),
            100,
            "analysis_outputs should have 100 entries"
        );

        // Spot-check a few entries
        let stored = read_string_table(db, file_state_table(), "file_0.rs");
        assert!(stored.is_some(), "file_0.rs should exist");
        let stored = read_string_table(db, file_state_table(), "file_99.rs");
        assert!(stored.is_some(), "file_99.rs should exist");

        let mut check_hash = [0u8; 32];
        check_hash[0] = 42;
        let stored_analysis = read_hash_table(db, analysis_outputs_table(), &check_hash);
        assert_eq!(
            stored_analysis,
            Some(vec![42]),
            "analysis for i=42 should match"
        );
    }

    // =======================================================================
    // G09 (B58): CommitError::WriteFailed variant construction test
    // =======================================================================

    #[test]
    fn commit_error_write_failed_display_contains_table_and_reason() {
        let err = CommitError::WriteFailed {
            table: "file_state",
            reason: "disk full".to_string(),
        };
        let msg = format!("{err}");
        assert!(
            matches!(
                err,
                CommitError::WriteFailed {
                    table: "file_state",
                    reason: _,
                }
            ),
            "WriteFailed must match with exact table name"
        );
        assert!(
            msg.contains("file_state"),
            "Display should contain table name: {msg}"
        );
        assert!(
            msg.contains("disk full"),
            "Display should contain reason: {msg}"
        );
        assert!(
            msg.contains("write failed"),
            "Display should mention write failure: {msg}"
        );
    }

    // =======================================================================
    // G10 (B61): StateDb::database() returns reference to underlying redb Database
    // =======================================================================

    #[test]
    fn database_returns_reference_to_underlying_redb_database() {
        let (state_db, _temp_dir) = create_temp_state_db();

        let db_ref = state_db.database();
        // Verify the reference is valid by using it
        let read_txn = db_ref
            .begin_read()
            .expect("begin_read on returned &Database should succeed");
        // All 8 tables should be accessible
        read_txn.open_table(file_state_table()).unwrap();
        read_txn.open_table(url_state_table()).unwrap();
        read_txn.open_table(analysis_outputs_table()).unwrap();
        read_txn.open_table(transform_outputs_table()).unwrap();
        read_txn.open_table(chunk_outputs_table()).unwrap();
        read_txn.open_table(scrape_outputs_table()).unwrap();
        read_txn.open_table(snapshots_table()).unwrap();
        read_txn.open_table(metadata_table()).unwrap();
    }

    // =======================================================================
    // G11 (B62): StateChanges::empty() creates valid batch with all empty vecs
    // =======================================================================

    #[test]
    fn state_changes_empty_creates_batch_with_all_empty_vecs() {
        let changes = StateChanges::empty();
        assert_eq!(changes.updated_files.len(), 0);
        assert_eq!(changes.deleted_files.len(), 0);
        assert_eq!(changes.new_analyses.len(), 0);
        assert_eq!(changes.new_transforms.len(), 0);
        assert_eq!(changes.new_chunks.len(), 0);
        assert_eq!(changes.updated_urls.len(), 0);
        assert_eq!(changes.deleted_urls.len(), 0);
        assert_eq!(changes.new_scrapes.len(), 0);
        assert_eq!(changes.new_snapshots.len(), 0);
        assert_eq!(changes.deleted_snapshots.len(), 0);
    }

    // =======================================================================
    // G12 (B63): StateChanges::default() equals empty()
    // =======================================================================

    #[test]
    fn state_changes_default_equals_empty() {
        let default = StateChanges::default();
        let empty = StateChanges::empty();
        assert_eq!(default.updated_files.len(), empty.updated_files.len());
        assert_eq!(default.deleted_files.len(), empty.deleted_files.len());
        assert_eq!(default.new_analyses.len(), empty.new_analyses.len());
        assert_eq!(default.new_transforms.len(), empty.new_transforms.len());
        assert_eq!(default.new_chunks.len(), empty.new_chunks.len());
        assert_eq!(default.updated_urls.len(), empty.updated_urls.len());
        assert_eq!(default.deleted_urls.len(), empty.deleted_urls.len());
        assert_eq!(default.new_scrapes.len(), empty.new_scrapes.len());
        assert_eq!(default.new_snapshots.len(), empty.new_snapshots.len());
        assert_eq!(
            default.deleted_snapshots.len(),
            empty.deleted_snapshots.len()
        );
    }

    // =======================================================================
    // G13 (B69): should_skip_write with large 1 MiB differing inputs
    // =======================================================================

    #[test]
    fn should_skip_write_returns_false_for_large_differing_inputs() {
        let large_a = vec![0xFFu8; 1_048_576];
        let large_b = vec![0xFEu8; 1_048_576];
        assert!(!should_skip_write(&large_a, &large_b));
        assert!(should_skip_write(&large_a, &large_a.clone()));
    }

    // =======================================================================
    // G15 (4.12): Proptest — EmptyStringKey boundary detection
    // =======================================================================

    #[test]
    fn proptest_empty_string_key_boundary_detection() {
        use proptest::prelude::*;

        proptest!(|(whitespace in "([ \t\n\r]{0,20})")| {
            let mut changes = StateChanges::empty();
            changes.updated_files = vec![(whitespace.clone(), FileStateRaw::zeroed())];
            let result = validate_no_empty_string_keys(&changes);
            prop_assert!(
                matches!(result, Err(CommitError::EmptyStringKey { table: "file_state", index: 0 })),
                "whitespace-only key '{}' should be rejected as EmptyStringKey",
                whitespace.escape_unicode()
            );
        });
    }

    #[test]
    fn proptest_non_empty_string_key_always_accepted() {
        use proptest::prelude::*;

        proptest!(|(key in "[^\t\n\r\x00-\\\x1F\x7F-\\\x7F]{1,10}")| {
            let mut changes = StateChanges::empty();
            changes.updated_files = vec![(key, FileStateRaw::zeroed())];
            let result = validate_no_empty_string_keys(&changes);            prop_assert!(
                result.is_ok(),
                "non-whitespace key should be accepted"
            );
        });
    }

    // =======================================================================
    // G16 (4.13): Proptest — validate_hash_key classifies by length
    // =======================================================================

    #[test]
    fn proptest_validate_hash_key_classifies_by_length() {
        use crate::state::validate_hash_key;
        use proptest::prelude::*;

        proptest!(|(bytes in proptest::collection::vec(any::<u8>(), 0..64))| {
            let result = validate_hash_key(&bytes);
            if bytes.len() == 32 {
                prop_assert!(result.is_ok(), "32-byte key should be valid");
            } else {
                prop_assert!(
                    matches!(result, Err(crate::state::StateError::InvalidHashKeyLength { actual }) if actual == bytes.len()),
                    "non-32-byte key (len={}) should return InvalidHashKeyLength",
                    bytes.len()
                );
            }
        });
    }

    // =======================================================================
    // G17 (4.14): Proptest — validate_source_path rejects invalid patterns
    // =======================================================================

    #[test]
    fn proptest_validate_source_path_rejects_invalid_patterns() {
        use crate::state::validate_source_path;
        use proptest::prelude::*;

        proptest!(|(s in ".*{0,50}")| {
            let result = validate_source_path(&s);
            let is_invalid = s.is_empty()
                || s.as_bytes().first() == Some(&b'/')
                || s.split('/').any(|c| c == "..");
            if is_invalid {
                prop_assert!(
                    result.is_err(),
                    "path '{}' should be rejected",
                    s.escape_unicode()
                );
            } else {
                prop_assert!(
                    result.is_ok(),
                    "valid relative path '{}' should be accepted",
                    s.escape_unicode()
                );
            }
        });
    }

    // =======================================================================
    // G18 (4.15): Proptest — validate_url_key rejects invalid patterns
    // =======================================================================

    #[test]
    fn proptest_validate_url_key_rejects_invalid_patterns() {
        use crate::state::validate_url_key;
        use proptest::prelude::*;

        proptest!(|(s in ".*{0,100}")| {
            let result = validate_url_key(&s);
            let is_invalid = s.is_empty() || !s.contains("://");
            if is_invalid {
                prop_assert!(
                    result.is_err(),
                    "URL '{}' should be rejected",
                    s.escape_unicode()
                );
            } else {
                prop_assert!(
                    result.is_ok(),
                    "URL with scheme '{}' should be accepted",
                    s.escape_unicode()
                );
            }
        });
    }

    // =======================================================================
    // G19 (4.16): Proptest — payload size boundary
    // =======================================================================

    #[test]
    fn proptest_payload_size_boundary() {
        use proptest::prelude::*;

        proptest!(|(
            sizes in proptest::collection::vec(0usize..MAX_VALUE_SIZE + 2, 0..5),
        )| {
            let mut entries: Vec<([u8; 32], Vec<u8>)> = Vec::new();
            for (i, size) in sizes.iter().enumerate() {
                let mut hash = [0u8; 32];
                hash[0] = u8::try_from(i).unwrap_or(u8::MAX);
                entries.push((hash, vec![0u8; *size]));
            }
            let result = check_payload_size(&entries, "analysis_outputs");
            let has_oversized = entries.iter().any(|(_, v)| v.len() > MAX_VALUE_SIZE);
            if has_oversized {
                prop_assert!(
                    matches!(result, Err(CommitError::PayloadTooLarge { table: "analysis_outputs", .. })),
                    "oversized payload should be rejected"
                );
            } else {
                prop_assert!(result.is_ok(), "all valid sizes should be accepted");
            }
        });
    }

    // =======================================================================
    // G20 (5.5): Fuzz target seed test for OwnedArchive::try_from_bytes
    // =======================================================================
    // Note: actual fuzz target is in fuzz/ directory. This test verifies the
    // key property: OwnedArchive::try_from_bytes must not panic on any input.

    #[test]
    fn owned_archive_try_from_bytes_never_panics_on_arbitrary_bytes() {
        use crate::persisted::PersistedAnalyzeResult;
        use crate::state::bulk_load::{BulkLoadError, OwnedArchive};

        let seeds: &[&[u8]] = &[
            &[0xFF, 0xFF, 0xFF, 0xFF],
            &[],
            &[0u8; 64],
            &[0xFFu8; 256],
            &[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        ];

        for seed in seeds {
            let bytes: Box<[u8]> = seed.to_vec().into_boxed_slice();
            let key: [u8; 32] = [0x42; 32];
            let result = OwnedArchive::<PersistedAnalyzeResult>::try_from_bytes(
                "analysis_outputs",
                &key,
                bytes,
            );
            // Must not panic — either Ok or CorruptPayload
            match result {
                Ok(_) | Err(BulkLoadError::CorruptPayload { .. }) => {}
                Err(other) => panic!("unexpected error variant: {other:?}"),
            }
        }
    }
}
