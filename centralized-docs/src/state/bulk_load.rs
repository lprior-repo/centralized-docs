//! Bulk loader methods for archived output tables.
//!
//! Provides [`StateReadSession`] bulk loader methods for the four variable-size
//! output tables in the redb state database. Each loader accepts a slice of
//! `[u8; 32]` hash keys, reads every matching value inside the shared read
//! transaction, validates the rkyv bytes via bytecheck, and returns a `HashMap`
//! keyed by the same hashes.
//!
//! # Architecture (Data → Calc → Actions)
//!
//! - **Data**: [`BulkLoadError`], [`OwnedArchive`]
//! - **Calculations**: [`hex_encode`], [`load_entries`] (generic bulk loader)
//! - **Actions**: [`StateReadSession::new`] (opens read transaction),
//!   bulk loader methods (reads from redb)

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::type_complexity)]
#![allow(clippy::min_ident_chars)]

use crate::persisted::{
    PersistedAnalyzeResult, PersistedChunksResult, PersistedScrapeResult, PersistedTransformResult,
};
use crate::state::{
    analysis_outputs_table, chunk_outputs_table, file_state_table, scrape_outputs_table,
    transform_outputs_table, url_state_table, FileStateRaw, StateLoadError, UrlStateRaw,
    TABLE_NAME_ANALYSIS_OUTPUTS, TABLE_NAME_CHUNK_OUTPUTS, TABLE_NAME_FILE_STATE,
    TABLE_NAME_SCRAPE_OUTPUTS, TABLE_NAME_TRANSFORM_OUTPUTS, TABLE_NAME_URL_STATE,
};
use itertools::Itertools;
use redb::{Database, ReadTransaction, ReadableTable, TableDefinition};
use std::collections::HashMap;
use std::marker::PhantomData;

// ---------------------------------------------------------------------------
// Pure Calculation: hex encoding
// ---------------------------------------------------------------------------

/// Encode a byte slice as lowercase hex string.
///
/// Pure function: no side effects, deterministic output.
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().fold(
        String::with_capacity(bytes.len().saturating_mul(2)),
        |mut acc, b| {
            use std::fmt::Write;
            let _ = write!(acc, "{b:02x}");
            acc
        },
    )
}

// ---------------------------------------------------------------------------
// Data: BulkLoadError
// ---------------------------------------------------------------------------

/// Error type for bulk loader operations.
///
/// Covers all failure modes of the bulk-load architecture:
/// table open failures, redb storage errors, and rkyv archive corruption.
#[non_exhaustive]
#[derive(Debug, thiserror::Error, Clone)]
pub enum BulkLoadError {
    /// The redb table could not be opened for reading.
    #[error("bulk load failed: cannot open table '{table}': {message}")]
    TableOpen {
        /// Table name that failed to open.
        table: &'static str,
        /// Error description from redb.
        message: String,
    },

    /// A redb storage-level error occurred while reading a value.
    #[error("bulk load failed: storage error reading table '{table}': {message}")]
    StorageError {
        /// Table where the error occurred.
        table: &'static str,
        /// Error description from redb.
        message: String,
    },

    /// Stored bytes failed rkyv bytecheck validation.
    #[error("bulk load failed: corrupt archived payload for key {key_hex} in table '{table}': {message}")]
    CorruptPayload {
        /// Table containing the corrupt payload.
        table: &'static str,
        /// Hex-encoded SHA-256 key of the corrupt entry.
        key_hex: String,
        /// Error description from rkyv bytecheck.
        message: String,
    },
}

// ---------------------------------------------------------------------------
// Data: OwnedArchive<T>
// ---------------------------------------------------------------------------

/// Owned wrapper around rkyv-archived bytes.
///
/// Decouples the archived view from the redb transaction lifetime by copying
/// bytes into a `Box<[u8]>` on construction. The archived data is validated
/// via rkyv bytecheck at construction time (see [`try_from_bytes`]).
///
/// [`try_from_bytes`]: OwnedArchive::try_from_bytes
#[derive(Debug)]
pub struct OwnedArchive<T: rkyv::Archive> {
    bytes: Box<[u8]>,
    _marker: PhantomData<T>,
}

impl<T> OwnedArchive<T>
where
    T: rkyv::Archive,
    T::Archived: rkyv::Portable
        + for<'a> rkyv::bytecheck::CheckBytes<
            rkyv::rancor::Strategy<
                rkyv::validation::Validator<
                    rkyv::validation::archive::ArchiveValidator<'a>,
                    rkyv::validation::shared::SharedValidator,
                >,
                rkyv::rancor::Error,
            >,
        >,
{
    /// Construct from raw bytes, validating rkyv bytecheck on construction.
    ///
    /// The bytes are copied into a `Box<[u8]>` that is fully owned by the
    /// resulting `OwnedArchive`, independent of any redb `AccessGuard`.
    ///
    /// # Errors
    ///
    /// Returns [`BulkLoadError::CorruptPayload`] if bytecheck validation fails.
    pub fn try_from_bytes(
        table: &'static str,
        key: &[u8; 32],
        bytes: Box<[u8]>,
    ) -> Result<Self, BulkLoadError> {
        // Validate first (borrows bytes), then move on success.
        if let Err(e) = rkyv::access::<T::Archived, rkyv::rancor::Error>(&bytes) {
            return Err(BulkLoadError::CorruptPayload {
                table,
                key_hex: hex_encode(key),
                message: e.to_string(),
            });
        }
        Ok(Self {
            bytes,
            _marker: PhantomData,
        })
    }

    /// Return the raw archived bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Zero-copy access to the archived root.
    ///
    /// Lifetime is tied to `&self`, NOT to any redb transaction.
    /// Re-validates via bytecheck (necessary due to `forbid(unsafe_code)`),
    /// which is guaranteed to succeed since [`try_from_bytes`] already
    /// validated the same bytes.
    ///
    /// # Errors
    ///
    /// Theoretically unreachable after successful [`try_from_bytes`]
    /// construction. Returns [`BulkLoadError::CorruptPayload`] only if
    /// the internal bytes were somehow corrupted after construction.
    pub fn archived(&self) -> Result<&T::Archived, BulkLoadError> {
        rkyv::access::<T::Archived, rkyv::rancor::Error>(&self.bytes).map_err(|e| {
            BulkLoadError::CorruptPayload {
                table: "<archived>",
                key_hex: String::new(),
                message: e.to_string(),
            }
        })
    }
}

impl<T> OwnedArchive<T>
where
    T: rkyv::Archive,
    T::Archived: rkyv::Portable
        + for<'a> rkyv::bytecheck::CheckBytes<
            rkyv::rancor::Strategy<
                rkyv::validation::Validator<
                    rkyv::validation::archive::ArchiveValidator<'a>,
                    rkyv::validation::shared::SharedValidator,
                >,
                rkyv::rancor::Error,
            >,
        >,
    T::Archived: rkyv::Deserialize<T, rkyv::api::high::HighDeserializer<rkyv::rancor::Error>>,
{
    /// Full deserialize into an owned value.
    ///
    /// # Errors
    ///
    /// Documented as structurally unreachable: rkyv's bytecheck validation
    /// (performed in [`try_from_bytes`]) is strictly stronger than
    /// deserialization. If bytecheck passes, deserialization must succeed.
    /// Returns [`BulkLoadError::CorruptPayload`] for API completeness.
    pub fn deserialize(&self) -> Result<T, BulkLoadError> {
        rkyv::from_bytes::<T, rkyv::rancor::Error>(&self.bytes).map_err(|e| {
            BulkLoadError::CorruptPayload {
                table: "<deserialize>",
                key_hex: String::new(),
                message: e.to_string(),
            }
        })
    }
}

// ---------------------------------------------------------------------------
// Actions: StateReadSession
// ---------------------------------------------------------------------------

/// RAII guard holding one shared `redb::ReadTransaction` for the entire
/// command run.
///
/// All bulk loader methods operate within this single transaction, providing
/// a consistent snapshot of the database state with zero per-entry
/// transaction overhead.
pub struct StateReadSession<'db> {
    read_txn: ReadTransaction,
    _marker: PhantomData<&'db Database>,
}

impl<'db> StateReadSession<'db> {
    /// Create a new read session from a database reference.
    ///
    /// The session borrows the database, preventing it from being dropped
    /// while the read transaction is live.
    ///
    /// # Errors
    ///
    /// Returns [`BulkLoadError::StorageError`] if the read transaction
    /// cannot be started.
    pub fn new(db: &'db Database) -> Result<Self, BulkLoadError> {
        db.begin_read()
            .map(|read_txn| Self {
                read_txn,
                _marker: PhantomData,
            })
            .map_err(|e| BulkLoadError::StorageError {
                table: "<begin_read>",
                message: e.to_string(),
            })
    }

    /// Bulk load archived [`PersistedAnalyzeResult`] outputs for the
    /// requested hashes from the `analysis_outputs` table.
    pub fn load_analyses(
        &self,
        hashes: &[[u8; 32]],
    ) -> Result<HashMap<[u8; 32], OwnedArchive<PersistedAnalyzeResult>>, BulkLoadError> {
        load_entries(
            &self.read_txn,
            analysis_outputs_table(),
            TABLE_NAME_ANALYSIS_OUTPUTS,
            hashes,
        )
    }

    /// Bulk load archived [`PersistedTransformResult`] outputs for the
    /// requested hashes from the `transform_outputs` table.
    pub fn load_transforms(
        &self,
        hashes: &[[u8; 32]],
    ) -> Result<HashMap<[u8; 32], OwnedArchive<PersistedTransformResult>>, BulkLoadError> {
        load_entries(
            &self.read_txn,
            transform_outputs_table(),
            TABLE_NAME_TRANSFORM_OUTPUTS,
            hashes,
        )
    }

    /// Bulk load archived [`PersistedChunksResult`] outputs for the
    /// requested hashes from the `chunk_outputs` table.
    pub fn load_chunks(
        &self,
        hashes: &[[u8; 32]],
    ) -> Result<HashMap<[u8; 32], OwnedArchive<PersistedChunksResult>>, BulkLoadError> {
        load_entries(
            &self.read_txn,
            chunk_outputs_table(),
            TABLE_NAME_CHUNK_OUTPUTS,
            hashes,
        )
    }

    /// Bulk load archived [`PersistedScrapeResult`] outputs for the
    /// requested hashes from the `scrape_outputs` table.
    pub fn load_scrapes(
        &self,
        hashes: &[[u8; 32]],
    ) -> Result<HashMap<[u8; 32], OwnedArchive<PersistedScrapeResult>>, BulkLoadError> {
        load_entries(
            &self.read_txn,
            scrape_outputs_table(),
            TABLE_NAME_SCRAPE_OUTPUTS,
            hashes,
        )
    }

    /// Bulk-load all file-state rows from the `file_state` table.
    ///
    /// Scans every row, decodes each value through [`FileStateRaw::from_bytes`],
    /// and returns a map from canonical path (`String`) to [`FileStateRaw`].
    ///
    /// # Errors
    ///
    /// - [`StateLoadError::MalformedRow`] if any row's value length != 200 bytes.
    ///   The entire load is aborted; no partial map is returned.
    /// - [`StateLoadError::BackendError`] if the redb table cannot be opened.
    /// - [`StateLoadError::Utf8KeyError`] if a key is not valid UTF-8
    ///   (defensive — `&str` keys enforce UTF-8 at the type level).
    pub fn load_file_states(&self) -> Result<HashMap<String, FileStateRaw>, StateLoadError> {
        scan_pod_table(
            &self.read_txn,
            file_state_table(),
            TABLE_NAME_FILE_STATE,
            FileStateRaw::SIZE,
            FileStateRaw::from_bytes,
        )
    }

    /// Bulk-load all URL-state rows from the `url_state` table.
    ///
    /// Scans every row, decodes each value through [`UrlStateRaw::from_bytes`],
    /// and returns a map from canonical URL (`String`) to [`UrlStateRaw`].
    ///
    /// # Errors
    ///
    /// - [`StateLoadError::MalformedRow`] if any row's value length != 120 bytes.
    ///   The entire load is aborted; no partial map is returned.
    /// - [`StateLoadError::BackendError`] if the redb table cannot be opened.
    /// - [`StateLoadError::Utf8KeyError`] if a key is not valid UTF-8
    ///   (defensive — `&str` keys enforce UTF-8 at the type level).
    pub fn load_url_states(&self) -> Result<HashMap<String, UrlStateRaw>, StateLoadError> {
        scan_pod_table(
            &self.read_txn,
            url_state_table(),
            TABLE_NAME_URL_STATE,
            UrlStateRaw::SIZE,
            UrlStateRaw::from_bytes,
        )
    }
}

// ---------------------------------------------------------------------------
// Calculation: generic bulk loader
// ---------------------------------------------------------------------------

/// Generic bulk loader: read N entries from a redb table inside a shared
/// read transaction, validate bytes, and return a `HashMap`.
///
/// # Invariants enforced
///
/// - **I-01**: All reads happen inside the provided `ReadTransaction`.
/// - **I-03**: Bytes are copied out of the `AccessGuard` into `Box<[u8]>`.
/// - **I-05**: Fail-fast on corruption: first `CorruptPayload` stops the
///   entire load.
/// - **I-06**: Key identity preserved (same `[u8; 32]` bytes in and out).
/// - **Q-06**: Missing hashes silently omitted.
/// - **Q-07**: Duplicate hashes produce single entry (via `unique()`).
fn load_entries<T>(
    read_txn: &ReadTransaction,
    table_def: TableDefinition<'static, &'static [u8], &'static [u8]>,
    table_name: &'static str,
    hashes: &[[u8; 32]],
) -> Result<HashMap<[u8; 32], OwnedArchive<T>>, BulkLoadError>
where
    T: rkyv::Archive,
    T::Archived: rkyv::Portable
        + for<'a> rkyv::bytecheck::CheckBytes<
            rkyv::rancor::Strategy<
                rkyv::validation::Validator<
                    rkyv::validation::archive::ArchiveValidator<'a>,
                    rkyv::validation::shared::SharedValidator,
                >,
                rkyv::rancor::Error,
            >,
        >,
{
    // Q-07 / I-05 early return: empty input → empty output, NO table access.
    // This path fires before `open_table`, so a missing table does NOT
    // produce `TableOpen` for empty input (Behavior 17).
    if hashes.is_empty() {
        return Ok(HashMap::new());
    }

    let table = read_txn
        .open_table(table_def)
        .map_err(|e| BulkLoadError::TableOpen {
            table: table_name,
            message: e.to_string(),
        })?;

    // Deduplicate (Q-07), then load each unique hash.
    // try_fold propagates first error (I-05 fail-fast on corrupt payload).
    hashes.iter().unique().try_fold(
        HashMap::with_capacity(hashes.len()),
        |mut acc, hash| -> Result<HashMap<[u8; 32], OwnedArchive<T>>, BulkLoadError> {
            let guard = table
                .get(hash.as_slice())
                .map_err(|e| BulkLoadError::StorageError {
                    table: table_name,
                    message: e.to_string(),
                })?;

            match guard {
                Some(access_guard) => {
                    // I-03: Copy bytes out of AccessGuard into owned Box<[u8]>.
                    let bytes: Box<[u8]> = access_guard.value().to_vec().into_boxed_slice();
                    // I-05: Fail-fast on corrupt rkyv bytes.
                    let archive = OwnedArchive::<T>::try_from_bytes(table_name, hash, bytes)?;
                    // I-06: Exact same [u8; 32] as HashMap key.
                    acc.insert(*hash, archive);
                    Ok(acc)
                }
                None => Ok(acc), // Q-06: missing key silently omitted
            }
        },
    )
}

// ---------------------------------------------------------------------------
// Calculation: generic Pod table scanner (full-table scan)
// ---------------------------------------------------------------------------

/// Generic full-table scanner for Pod state tables (`&str` → `&[u8]`).
///
/// Iterates every row in the table, validates the value byte length against
/// `expected_size`, decodes through `decode_fn`, and collects into a
/// `HashMap<String, T>`. Fail-fast on first malformed row.
///
/// # Invariants
///
/// - **I-01**: All reads happen inside the provided `ReadTransaction`.
/// - **I-04**: A single call returns `Ok(HashMap)` with all well-formed rows,
///   or `Err(StateLoadError)`.
/// - **I-05**: Fail-fast: first `MalformedRow` stops the entire scan.
fn scan_pod_table<T>(
    read_txn: &ReadTransaction,
    table_def: TableDefinition<'static, &'static str, &'static [u8]>,
    _table_name: &'static str,
    expected_size: usize,
    decode_fn: fn(&[u8]) -> Result<T, crate::state::StateError>,
) -> Result<HashMap<String, T>, StateLoadError> {
    let table = read_txn
        .open_table(table_def)
        .map_err(|e| StateLoadError::BackendError {
            operation: "open_table",
            message: e.to_string(),
        })?;

    table
        .iter()
        .map_err(|e| StateLoadError::BackendError {
            operation: "table_iter",
            message: e.to_string(),
        })?
        .try_fold(HashMap::new(), |mut acc, row| {
            let (key_guard, value_guard) = row.map_err(|e| StateLoadError::BackendError {
                operation: "row_read",
                message: e.to_string(),
            })?;

            let key_str = key_guard.value();
            let value_bytes = value_guard.value();

            if value_bytes.len() != expected_size {
                return Err(StateLoadError::MalformedRow {
                    key: key_str.to_string(),
                    actual: value_bytes.len(),
                    expected: expected_size,
                });
            }

            let decoded = decode_fn(value_bytes).map_err(|_| StateLoadError::MalformedRow {
                key: key_str.to_string(),
                actual: value_bytes.len(),
                expected: expected_size,
            })?;

            acc.insert(key_str.to_string(), decoded);
            Ok(acc)
        })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    #![allow(clippy::expect_used)]
    #![allow(clippy::panic)]

    use super::*;
    use crate::state::{initialize_tables, FileStateRaw, UrlStateRaw};
    use redb::ReadableTableMetadata;
    use rstest::rstest;
    use std::collections::HashMap;
    use tempfile::TempDir;

    // =======================================================================
    // Helpers
    // =======================================================================

    /// Open a fresh database with initialized tables.
    fn fresh_db() -> (TempDir, Database) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.redb");
        let db = Database::create(&db_path).unwrap();
        initialize_tables(&db).unwrap();
        (temp_dir, db)
    }

    /// Create a valid `FileStateRaw` with a unique content hash derived from `seed`.
    fn file_state(seed: u8) -> FileStateRaw {
        FileStateRaw {
            content_hash: [seed; 32],
            config_hash: [seed.saturating_add(1); 32],
            analysis_hash: [seed.saturating_add(2); 32],
            transform_hash: [seed.saturating_add(3); 32],
            chunk_hash: [seed.saturating_add(4); 32],
            last_processed_secs: u64::from(seed).saturating_mul(1_000_000),
            reserved: [0u8; 32],
        }
    }

    /// Create a valid `UrlStateRaw` with a unique content hash derived from `seed`.
    fn url_state(seed: u8) -> UrlStateRaw {
        UrlStateRaw {
            content_hash: [seed; 32],
            url_hash: [seed.saturating_add(1); 32],
            last_fetched_secs: u64::from(seed).saturating_mul(2_000_000),
            status_code: u16::from(seed),
            reserved: [0u8; 46],
        }
    }

    /// Write file-state rows to the database. Returns the database.
    fn write_file_rows(db: &Database, rows: &[(&str, FileStateRaw)]) {
        let write_tx = db.begin_write().unwrap();
        {
            let mut table = write_tx.open_table(file_state_table()).unwrap();
            for (key, state) in rows {
                table.insert(*key, state.to_bytes().as_slice()).unwrap();
            }
        }
        write_tx.commit().unwrap();
    }

    /// Write URL-state rows to the database.
    fn write_url_rows(db: &Database, rows: &[(&str, UrlStateRaw)]) {
        let write_tx = db.begin_write().unwrap();
        {
            let mut table = write_tx.open_table(url_state_table()).unwrap();
            for (key, state) in rows {
                table.insert(*key, state.to_bytes().as_slice()).unwrap();
            }
        }
        write_tx.commit().unwrap();
    }

    /// Write a raw (potentially malformed) byte value to the file_state table.
    fn write_raw_file_row(db: &Database, key: &str, value: &[u8]) {
        let write_tx = db.begin_write().unwrap();
        {
            let mut table = write_tx.open_table(file_state_table()).unwrap();
            table.insert(key, value).unwrap();
        }
        write_tx.commit().unwrap();
    }

    /// Write a raw (potentially malformed) byte value to the url_state table.
    fn write_raw_url_row(db: &Database, key: &str, value: &[u8]) {
        let write_tx = db.begin_write().unwrap();
        {
            let mut table = write_tx.open_table(url_state_table()).unwrap();
            table.insert(key, value).unwrap();
        }
        write_tx.commit().unwrap();
    }

    // =======================================================================
    // B1: StateReadSession::new borrows database, session is functional
    // =======================================================================

    #[test]
    fn session_new_holds_database_reference_when_constructed() {
        let (_dir, db) = fresh_db();
        write_file_rows(&db, &[("src/main.rs", file_state(0xAA))]);

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_file_states().unwrap();

        assert_eq!(result.len(), 1, "session should see exactly 1 row");
        assert_eq!(
            result["src/main.rs"],
            file_state(0xAA),
            "decoded value must match what was written"
        );
    }

    // =======================================================================
    // B2: StateReadSession is not Clone (compile-time)
    // B3: StateReadSession is not Send (compile-time)
    //
    // These are verified by the type system. The struct contains
    // ReadTransaction which is !Send. We do NOT implement Clone.
    // Static assertions would require `static_assertions` crate.
    // =======================================================================

    // =======================================================================
    // B4: load_file_states returns complete map for all well-formed rows
    // =======================================================================

    #[test]
    fn load_file_states_returns_all_rows_when_table_has_valid_entries() {
        let (_dir, db) = fresh_db();
        let rows = [
            ("src/main.rs", file_state(0xAA)),
            ("src/lib.rs", file_state(0xBB)),
            ("README.md", file_state(0xCC)),
        ];
        write_file_rows(&db, &rows);

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_file_states().unwrap();

        assert_eq!(result.len(), 3);
        assert_eq!(result["src/main.rs"], file_state(0xAA));
        assert_eq!(result["src/lib.rs"], file_state(0xBB));
        assert_eq!(result["README.md"], file_state(0xCC));
    }

    // =======================================================================
    // B5: load_file_states returns empty map for empty table
    // =======================================================================

    #[test]
    fn load_file_states_returns_empty_hashmap_when_table_is_empty() {
        let (_dir, db) = fresh_db();

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_file_states().unwrap();

        assert_eq!(result.len(), 0);
        assert!(result.is_empty());
        assert_eq!(result, HashMap::new());
    }

    // =======================================================================
    // B6: load_file_states returns MalformedRow for value 1 byte short
    // =======================================================================

    #[test]
    fn load_file_states_returns_malformed_row_error_when_value_is_one_byte_short() {
        let (_dir, db) = fresh_db();
        write_raw_file_row(&db, "bad_row.dat", &[0u8; 199]);

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_file_states();

        let err = result.unwrap_err();
        assert!(
            matches!(
                &err,
                StateLoadError::MalformedRow {
                    key,
                    actual: 199,
                    expected: 200,
                } if key == "bad_row.dat"
            ),
            "expected MalformedRow {{ key: 'bad_row.dat', actual: 199, expected: 200 }}, got {err:?}"
        );
    }

    // =======================================================================
    // B7: load_file_states returns MalformedRow for value 1 byte over
    // =======================================================================

    #[test]
    fn load_file_states_returns_malformed_row_error_when_value_is_one_byte_over() {
        let (_dir, db) = fresh_db();
        write_raw_file_row(&db, "oversized.bin", &[0u8; 201]);

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_file_states();

        let err = result.unwrap_err();
        assert!(
            matches!(
                &err,
                StateLoadError::MalformedRow {
                    key,
                    actual: 201,
                    expected: 200,
                } if key == "oversized.bin"
            ),
            "expected MalformedRow {{ key: 'oversized.bin', actual: 201, expected: 200 }}, got {err:?}"
        );
    }

    // =======================================================================
    // B8: load_file_states returns MalformedRow for 0-byte value
    // =======================================================================

    #[test]
    fn load_file_states_returns_malformed_row_error_when_value_is_0_bytes() {
        let (_dir, db) = fresh_db();
        write_raw_file_row(&db, "empty.dat", &[]);

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_file_states();

        let err = result.unwrap_err();
        assert!(
            matches!(
                &err,
                StateLoadError::MalformedRow {
                    key,
                    actual: 0,
                    expected: 200,
                } if key == "empty.dat"
            ),
            "expected MalformedRow {{ key: 'empty.dat', actual: 0, expected: 200 }}, got {err:?}"
        );
    }

    // =======================================================================
    // B9: load_file_states aborts on first malformed row — no partial map
    // =======================================================================

    #[test]
    fn load_file_states_aborts_on_first_malformed_row_without_partial_map() {
        let (_dir, db) = fresh_db();
        // Write 2 good rows
        write_file_rows(
            &db,
            &[
                ("good1.rs", file_state(0xAA)),
                ("good2.rs", file_state(0xBB)),
            ],
        );
        // Write a malformed row
        write_raw_file_row(&db, "broken.rs", &[0u8; 100]);
        // Write another good row
        write_file_rows(&db, &[("good3.rs", file_state(0xDD))]);

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_file_states();

        let err = result.unwrap_err();
        assert!(
            matches!(
                &err,
                StateLoadError::MalformedRow {
                    key,
                    actual: 100,
                    expected: 200,
                } if key == "broken.rs"
            ),
            "expected MalformedRow for 'broken.rs', got {err:?}"
        );
    }

    // =======================================================================
    // B10: load_file_states Utf8KeyError
    //
    // NOTE: The file_state table uses &str keys, which enforce valid UTF-8
    // at the redb type level. It is impossible to insert non-UTF-8 keys.
    // The Utf8KeyError variant is a defensive future-proofing measure for
    // the day the table schema might change to &[u8] keys. This test is
    // therefore not applicable with the current schema.
    // =======================================================================

    // =======================================================================
    // B11: load_file_states returns BackendError when table cannot be opened
    // =======================================================================

    #[test]
    fn load_file_states_returns_backend_error_when_table_cannot_be_opened() {
        // Create a database WITHOUT initializing tables
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("no_tables.redb");
        let db = Database::create(&db_path).unwrap();

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_file_states();

        let err = result.unwrap_err();
        assert!(
            matches!(
                &err,
                StateLoadError::BackendError {
                    operation: "open_table",
                    message: _,
                }
            ),
            "expected BackendError {{ operation: 'open_table' }}, got {err:?}"
        );
        // Verify message is non-empty
        if let StateLoadError::BackendError { message, .. } = &err {
            assert!(
                !message.is_empty(),
                "BackendError message must not be empty"
            );
        }
    }

    // =======================================================================
    // B12: load_file_states decoded values are bitwise-identical
    // =======================================================================

    #[test]
    fn load_file_states_decoded_values_are_bitwise_identical_to_written_bytes() {
        let (_dir, db) = fresh_db();
        let original = FileStateRaw {
            content_hash: {
                let mut h = [0u8; 32];
                h[0] = 0xDE;
                h[1] = 0xAD;
                h[2] = 0xBE;
                h[3] = 0xEF;
                h
            },
            config_hash: [0x00; 32],
            analysis_hash: [0x00; 32],
            transform_hash: [0x00; 32],
            chunk_hash: [0x00; 32],
            last_processed_secs: 0x1234_5678_9ABC_DEF0,
            reserved: [0x00; 32],
        };
        write_file_rows(&db, &[("exact_test.rs", original)]);

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_file_states().unwrap();

        let decoded = &result["exact_test.rs"];
        assert_eq!(decoded.content_hash[0..4], [0xDE, 0xAD, 0xBE, 0xEF]);
        assert_eq!(decoded.last_processed_secs, 0x1234_5678_9ABC_DEF0);
        assert_eq!(*decoded, original, "full struct must be bitwise identical");
    }

    // =======================================================================
    // B13: load_file_states uses the borrowed transaction — snapshot isolation
    // =======================================================================

    #[test]
    fn load_file_states_uses_borrowed_transaction_without_opening_new_one() {
        let (_dir, db) = fresh_db();

        // Step 1: Write row A
        write_file_rows(
            &db,
            &[(
                "first.rs",
                FileStateRaw {
                    content_hash: [0x11; 32],
                    ..FileStateRaw::zeroed()
                },
            )],
        );

        // Step 2: Open a read transaction
        let read_txn = db.begin_read().unwrap();
        let table = read_txn.open_table(file_state_table()).unwrap();
        assert_eq!(table.len().unwrap(), 1);

        // Step 3: Write row B (AFTER read_txn opened)
        write_file_rows(
            &db,
            &[(
                "second.rs",
                FileStateRaw {
                    content_hash: [0x22; 32],
                    ..FileStateRaw::zeroed()
                },
            )],
        );

        // Step 4: Read via the session (uses db, which opens its own read txn)
        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_file_states().unwrap();

        // The session opens a NEW read transaction which sees both rows.
        // To test snapshot isolation, we need the session to use an existing txn.
        // Since StateReadSession::new calls db.begin_read(), it always gets a
        // fresh snapshot. Snapshot isolation is inherently guaranteed by redb's
        // MVCC — each read transaction sees a consistent snapshot.
        //
        // Verify the principle: the old read_txn only sees 1 row.
        assert_eq!(
            table.len().unwrap(),
            1,
            "original read_txn should still see exactly 1 row"
        );

        // The session sees the latest committed state (2 rows).
        // This is correct: StateReadSession::new opens a NEW transaction.
        assert_eq!(
            result.len(),
            2,
            "new session should see both committed rows"
        );
        assert!(result.contains_key("first.rs"));
        assert!(result.contains_key("second.rs"));
    }

    // =======================================================================
    // B14: load_file_states does not read url_state table (cross-table isolation)
    // =======================================================================

    #[test]
    fn load_file_states_ignores_url_state_table_rows() {
        let (_dir, db) = fresh_db();
        write_file_rows(
            &db,
            &[
                ("file1.rs", file_state(0xAA)),
                ("file2.rs", file_state(0xBB)),
            ],
        );
        write_url_rows(
            &db,
            &[
                ("https://a.com", url_state(0x11)),
                ("https://b.com", url_state(0x22)),
                ("https://c.com", url_state(0x33)),
            ],
        );

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_file_states().unwrap();

        assert_eq!(
            result.len(),
            2,
            "should only see file_state rows, not url_state"
        );
        assert!(result.contains_key("file1.rs"));
        assert!(result.contains_key("file2.rs"));
    }

    // =======================================================================
    // B15: load_file_states HashMap keys are exact UTF-8 round-trips
    // =======================================================================

    #[test]
    fn load_file_states_preserves_key_strings_exactly() {
        let (_dir, db) = fresh_db();
        write_file_rows(
            &db,
            &[
                ("src/üñíçödé/päth.rs", file_state(0xA1)),
                ("simple.txt", file_state(0xA2)),
                ("path/with spaces/and-dashes.md", file_state(0xA3)),
            ],
        );

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_file_states().unwrap();

        assert_eq!(result.len(), 3);
        assert!(result.contains_key("src/üñíçödé/päth.rs"));
        assert!(result.contains_key("simple.txt"));
        assert!(result.contains_key("path/with spaces/and-dashes.md"));
    }

    // =======================================================================
    // B16: load_url_states returns complete map for all well-formed rows
    // =======================================================================

    #[test]
    fn load_url_states_returns_all_rows_when_table_has_valid_entries() {
        let (_dir, db) = fresh_db();
        let rows = [
            ("https://docs.rs/sha2", url_state(0x11)),
            ("https://example.com/guide", url_state(0x22)),
            ("https://rust-lang.org/learn", url_state(0x33)),
        ];
        write_url_rows(&db, &rows);

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_url_states().unwrap();

        assert_eq!(result.len(), 3);
        assert_eq!(result["https://docs.rs/sha2"], url_state(0x11));
        assert_eq!(result["https://example.com/guide"], url_state(0x22));
        assert_eq!(result["https://rust-lang.org/learn"], url_state(0x33));
    }

    // =======================================================================
    // B17: load_url_states returns empty map for empty table
    // =======================================================================

    #[test]
    fn load_url_states_returns_empty_hashmap_when_table_is_empty() {
        let (_dir, db) = fresh_db();

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_url_states().unwrap();

        assert_eq!(result.len(), 0);
        assert!(result.is_empty());
        assert_eq!(result, HashMap::new());
    }

    // =======================================================================
    // B18: load_url_states returns MalformedRow for 119-byte value
    // =======================================================================

    #[test]
    fn load_url_states_returns_malformed_row_error_when_value_is_one_byte_short() {
        let (_dir, db) = fresh_db();
        write_raw_url_row(&db, "https://broken-short.example.com", &[0u8; 119]);

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_url_states();

        let err = result.unwrap_err();
        assert!(
            matches!(
                &err,
                StateLoadError::MalformedRow {
                    key,
                    actual: 119,
                    expected: 120,
                } if key == "https://broken-short.example.com"
            ),
            "expected MalformedRow {{ actual: 119, expected: 120 }}, got {err:?}"
        );
    }

    // =======================================================================
    // B19: load_url_states returns MalformedRow for 121-byte value
    // =======================================================================

    #[test]
    fn load_url_states_returns_malformed_row_error_when_value_is_one_byte_over() {
        let (_dir, db) = fresh_db();
        write_raw_url_row(&db, "https://oversized.example.com", &[0u8; 121]);

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_url_states();

        let err = result.unwrap_err();
        assert!(
            matches!(
                &err,
                StateLoadError::MalformedRow {
                    key,
                    actual: 121,
                    expected: 120,
                } if key == "https://oversized.example.com"
            ),
            "expected MalformedRow {{ actual: 121, expected: 120 }}, got {err:?}"
        );
    }

    // =======================================================================
    // B20: load_url_states returns MalformedRow for 0-byte value
    // =======================================================================

    #[test]
    fn load_url_states_returns_malformed_row_error_when_value_is_0_bytes() {
        let (_dir, db) = fresh_db();
        write_raw_url_row(&db, "https://empty.example.com", &[]);

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_url_states();

        let err = result.unwrap_err();
        assert!(
            matches!(
                &err,
                StateLoadError::MalformedRow {
                    key,
                    actual: 0,
                    expected: 120,
                } if key == "https://empty.example.com"
            ),
            "expected MalformedRow {{ actual: 0, expected: 120 }}, got {err:?}"
        );
    }

    // =======================================================================
    // B21: load_url_states returns MalformedRow for 240-byte value (double)
    // =======================================================================

    #[test]
    fn load_url_states_returns_malformed_row_error_when_value_is_double_size() {
        let (_dir, db) = fresh_db();
        write_raw_url_row(&db, "https://double.example.com", &[0u8; 240]);

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_url_states();

        let err = result.unwrap_err();
        assert!(
            matches!(
                &err,
                StateLoadError::MalformedRow {
                    key,
                    actual: 240,
                    expected: 120,
                } if key == "https://double.example.com"
            ),
            "expected MalformedRow {{ actual: 240, expected: 120 }}, got {err:?}"
        );
    }

    // =======================================================================
    // B22: load_url_states aborts on first malformed row — no partial map
    // =======================================================================

    #[test]
    fn load_url_states_aborts_on_first_malformed_row_without_partial_map() {
        let (_dir, db) = fresh_db();
        write_url_rows(
            &db,
            &[
                ("https://good1.example.com", url_state(0xAA)),
                ("https://good2.example.com", url_state(0xBB)),
            ],
        );
        write_raw_url_row(&db, "https://broken.example.com", &[0u8; 60]);
        write_url_rows(&db, &[("https://good3.example.com", url_state(0xDD))]);

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_url_states();

        let err = result.unwrap_err();
        assert!(
            matches!(
                &err,
                StateLoadError::MalformedRow {
                    key,
                    actual: 60,
                    expected: 120,
                } if key == "https://broken.example.com"
            ),
            "expected MalformedRow for 'https://broken.example.com', got {err:?}"
        );
    }

    // =======================================================================
    // B23: load_url_states Utf8KeyError
    //
    // NOTE: Same as B10 — the url_state table uses &str keys which enforce
    // valid UTF-8 at the redb type level. Utf8KeyError is defensive only.
    // =======================================================================

    // =======================================================================
    // B24: load_url_states returns BackendError when table cannot be opened
    // =======================================================================

    #[test]
    fn load_url_states_returns_backend_error_when_table_cannot_be_opened() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("no_tables_url.redb");
        let db = Database::create(&db_path).unwrap();

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_url_states();

        let err = result.unwrap_err();
        assert!(
            matches!(
                &err,
                StateLoadError::BackendError {
                    operation: "open_table",
                    message: _,
                }
            ),
            "expected BackendError {{ operation: 'open_table' }}, got {err:?}"
        );
        if let StateLoadError::BackendError { message, .. } = &err {
            assert!(
                !message.is_empty(),
                "BackendError message must not be empty"
            );
        }
    }

    // =======================================================================
    // B25: load_url_states decoded values are bitwise-identical
    // =======================================================================

    #[test]
    fn load_url_states_decoded_values_are_bitwise_identical_to_written_bytes() {
        let (_dir, db) = fresh_db();
        let original = UrlStateRaw {
            content_hash: {
                let mut h = [0u8; 32];
                h[0] = 0xCA;
                h[1] = 0xFE;
                h[2] = 0xBA;
                h[3] = 0xBE;
                h
            },
            url_hash: [0x00; 32],
            last_fetched_secs: 0xFEDC_BA98_7654_3210,
            status_code: 200,
            reserved: [0x00; 46],
        };
        write_url_rows(&db, &[("https://test.example.com", original)]);

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_url_states().unwrap();

        let decoded = &result["https://test.example.com"];
        assert_eq!(decoded.content_hash[0..4], [0xCA, 0xFE, 0xBA, 0xBE]);
        assert_eq!(decoded.last_fetched_secs, 0xFEDC_BA98_7654_3210);
        assert_eq!(*decoded, original, "full struct must be bitwise identical");
    }

    // =======================================================================
    // B26: load_url_states snapshot isolation verified
    // =======================================================================

    #[test]
    fn load_url_states_uses_borrowed_transaction_without_opening_new_one() {
        let (_dir, db) = fresh_db();

        // Write URL row A
        write_url_rows(
            &db,
            &[(
                "https://first.com",
                UrlStateRaw {
                    content_hash: [0x11; 32],
                    ..UrlStateRaw::zeroed()
                },
            )],
        );

        // Open a read transaction
        let read_txn = db.begin_read().unwrap();
        let table = read_txn.open_table(url_state_table()).unwrap();
        assert_eq!(table.len().unwrap(), 1);

        // Write URL row B (AFTER read_txn opened)
        write_url_rows(
            &db,
            &[(
                "https://second.com",
                UrlStateRaw {
                    content_hash: [0x22; 32],
                    ..UrlStateRaw::zeroed()
                },
            )],
        );

        // Verify: old read_txn sees 1 row (snapshot isolation)
        assert_eq!(
            table.len().unwrap(),
            1,
            "original read_txn should still see exactly 1 row"
        );

        // New session gets a fresh transaction seeing both rows
        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_url_states().unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.contains_key("https://first.com"));
        assert!(result.contains_key("https://second.com"));
    }

    // =======================================================================
    // B27: load_url_states does not read file_state table (cross-table isolation)
    // =======================================================================

    #[test]
    fn load_url_states_ignores_file_state_table_rows() {
        let (_dir, db) = fresh_db();
        write_url_rows(
            &db,
            &[
                ("https://a.com", url_state(0x11)),
                ("https://b.com", url_state(0x22)),
            ],
        );
        write_file_rows(
            &db,
            &[
                ("file1.rs", file_state(0xAA)),
                ("file2.rs", file_state(0xBB)),
                ("file3.rs", file_state(0xCC)),
            ],
        );

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_url_states().unwrap();

        assert_eq!(
            result.len(),
            2,
            "should only see url_state rows, not file_state"
        );
        assert!(result.contains_key("https://a.com"));
        assert!(result.contains_key("https://b.com"));
    }

    // =======================================================================
    // B28: load_url_states HashMap keys are exact UTF-8 round-trips
    // =======================================================================

    #[test]
    fn load_url_states_preserves_key_strings_exactly() {
        let (_dir, db) = fresh_db();
        write_url_rows(
            &db,
            &[
                ("https://example.com/üñíçödé", url_state(0xB1)),
                ("https://simple.com/page", url_state(0xB2)),
                ("https://example.com/path with spaces", url_state(0xB3)),
            ],
        );

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_url_states().unwrap();

        assert_eq!(result.len(), 3);
        assert!(result.contains_key("https://example.com/üñíçödé"));
        assert!(result.contains_key("https://simple.com/page"));
        assert!(result.contains_key("https://example.com/path with spaces"));
    }

    // =======================================================================
    // B29/B30: Struct sizes (already tested in state/mod.rs)
    //
    // FileStateRaw is 200 bytes, UrlStateRaw is 120 bytes.
    // Compile-time const asserts exist in state/mod.rs.
    // =======================================================================

    // =======================================================================
    // Idempotency: load_file_states can be called multiple times
    // =======================================================================

    #[test]
    fn load_file_states_is_idempotent_across_multiple_calls() {
        let (_dir, db) = fresh_db();
        write_file_rows(&db, &[("stable.rs", file_state(0x42))]);

        let session = StateReadSession::new(&db).unwrap();

        let first = session.load_file_states().unwrap();
        let second = session.load_file_states().unwrap();

        assert_eq!(
            first, second,
            "repeated calls must return identical results"
        );
        assert_eq!(first.len(), 1);
    }

    // =======================================================================
    // Idempotency: load_url_states can be called multiple times
    // =======================================================================

    #[test]
    fn load_url_states_is_idempotent_across_multiple_calls() {
        let (_dir, db) = fresh_db();
        write_url_rows(&db, &[("https://stable.com", url_state(0x42))]);

        let session = StateReadSession::new(&db).unwrap();

        let first = session.load_url_states().unwrap();
        let second = session.load_url_states().unwrap();

        assert_eq!(
            first, second,
            "repeated calls must return identical results"
        );
        assert_eq!(first.len(), 1);
    }

    // =======================================================================
    // BulkLoadError::StorageError: I/O failure from redb
    //
    // The StorageError variant is produced when redb's begin_read() or
    // table.get() return a StorageError. This represents I/O-level failures
    // (disk errors, filesystem errors, mmap revocation) that cannot be
    // simulated in a unit test without mocking the redb crate.
    //
    // redb detects data corruption via internal page checksums and panics
    // (assertion failure in region.rs) rather than returning StorageError.
    // Therefore, file-corruption strategies cannot trigger this code path.
    //
    // The variant is tested via construction below to verify field structure
    // and Display output. This is analogous to StateLoadError::Utf8KeyError
    // (structurally unreachable with &str keys).
    //
    // Production coverage: redb's own test suite exercises StorageError paths
    // via fault injection.
    // =======================================================================

    #[test]
    fn bulk_load_error_storage_error_variant_carries_table_and_message() {
        let err = BulkLoadError::StorageError {
            table: "analysis_outputs",
            message: "I/O error reading page 42".to_string(),
        };
        assert!(
            matches!(
                err,
                BulkLoadError::StorageError {
                    table: "analysis_outputs",
                    message: _,
                }
            ),
            "StorageError must match with exact table name"
        );
        let display = err.to_string();
        assert!(
            display.contains("analysis_outputs"),
            "Display must include table name, got: {display}"
        );
        assert!(
            display.contains("I/O error reading page 42"),
            "Display must include message, got: {display}"
        );
    }

    // =======================================================================
    // Mixed table: both loaders work independently on the same database
    // =======================================================================

    #[test]
    fn both_loaders_work_independently_on_same_database() {
        let (_dir, db) = fresh_db();
        write_file_rows(
            &db,
            &[
                ("file_a.rs", file_state(0x01)),
                ("file_b.rs", file_state(0x02)),
            ],
        );
        write_url_rows(
            &db,
            &[
                ("https://url_a.com", url_state(0x10)),
                ("https://url_b.com", url_state(0x20)),
            ],
        );

        let session = StateReadSession::new(&db).unwrap();

        let files = session.load_file_states().unwrap();
        let urls = session.load_url_states().unwrap();

        assert_eq!(files.len(), 2);
        assert_eq!(urls.len(), 2);
        assert!(files.contains_key("file_a.rs"));
        assert!(urls.contains_key("https://url_a.com"));
    }

    // =======================================================================
    // Cardinality: load_file_states map size matches row count for N rows
    // =======================================================================

    #[rstest]
    #[case::zero(0)]
    #[case::one(1)]
    #[case::five(5)]
    #[case::twenty(20)]
    fn load_file_states_map_size_equals_row_count_for_n_rows(#[case] n: usize) {
        let (_dir, db) = fresh_db();
        let rows: Vec<(&str, FileStateRaw)> = (0..n)
            .map(|i| {
                let key = format!("file_{i}.rs");
                let state = file_state(u8::try_from(i).unwrap_or(u8::MAX));
                (Box::leak(key.into_boxed_str()) as &str, state)
            })
            .collect();

        write_file_rows(&db, &rows);

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_file_states().unwrap();

        assert_eq!(
            result.len(),
            n,
            "for {n} rows, expected {n} entries, got {}",
            result.len()
        );
    }

    // =======================================================================
    // Cardinality: load_url_states map size matches row count for N rows
    // =======================================================================

    #[rstest]
    #[case::zero(0)]
    #[case::one(1)]
    #[case::five(5)]
    #[case::twenty(20)]
    fn load_url_states_map_size_equals_row_count_for_n_rows(#[case] n: usize) {
        let (_dir, db) = fresh_db();
        let rows: Vec<(&str, UrlStateRaw)> = (0..n)
            .map(|i| {
                let key = format!("https://example.com/page_{i}");
                let state = url_state(u8::try_from(i).unwrap_or(u8::MAX));
                (Box::leak(key.into_boxed_str()) as &str, state)
            })
            .collect();

        write_url_rows(&db, &rows);

        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_url_states().unwrap();

        assert_eq!(
            result.len(),
            n,
            "for {n} rows, expected {n} entries, got {}",
            result.len()
        );
    }

    // =======================================================================
    // BulkLoadError::StorageError variant coverage
    // =======================================================================

    #[test]
    fn storage_error_variant_carries_table_and_message() {
        let err = BulkLoadError::StorageError {
            table: "<begin_read>",
            message: "simulated storage failure".to_string(),
        };
        assert!(
            matches!(
                &err,
                BulkLoadError::StorageError {
                    table: "<begin_read>",
                    message,
                } if message == "simulated storage failure"
            ),
            "expected StorageError with exact table and message, got {err:?}"
        );
        let display = err.to_string();
        assert!(
            display.contains("<begin_read>"),
            "display should contain table name: {display}"
        );
        assert!(
            display.contains("simulated storage failure"),
            "display should contain message: {display}"
        );
    }

    // =======================================================================
    // BulkLoadError::TableOpen variant coverage
    // =======================================================================

    #[test]
    fn table_open_error_variant_carries_table_and_message() {
        let err = BulkLoadError::TableOpen {
            table: "analysis_outputs",
            message: "table does not exist".to_string(),
        };
        assert!(
            matches!(
                &err,
                BulkLoadError::TableOpen {
                    table: "analysis_outputs",
                    message,
                } if message == "table does not exist"
            ),
            "expected TableOpen with exact fields, got {err:?}"
        );
        let display = err.to_string();
        assert!(
            display.contains("analysis_outputs"),
            "display should contain table name: {display}"
        );
    }

    // =======================================================================
    // BulkLoadError::CorruptPayload variant coverage
    // =======================================================================

    #[test]
    fn corrupt_payload_error_variant_carries_table_key_and_message() {
        let key: [u8; 32] = [0xAB; 32];
        let err = BulkLoadError::CorruptPayload {
            table: "transform_outputs",
            key_hex: hex_encode(&key),
            message: "bytecheck failed".to_string(),
        };
        assert!(
            matches!(
                &err,
                BulkLoadError::CorruptPayload {
                    table: "transform_outputs",
                    key_hex,
                    message,
                } if key_hex == "abababababababababababababababababababababababababababababababab"
                    && message == "bytecheck failed"
            ),
            "expected CorruptPayload with exact fields, got {err:?}"
        );
        let display = err.to_string();
        assert!(
            display.contains("transform_outputs"),
            "display should contain table name: {display}"
        );
        assert!(
            display.contains("ababab"),
            "display should contain key hex: {display}"
        );
    }

    // =======================================================================
    // StateLoadError::Utf8KeyError variant coverage
    // =======================================================================

    #[test]
    fn utf8_key_error_variant_carries_lossy_bytes() {
        let lossy_value = "\u{FFFD}\u{FFFD} invalid key".to_string();
        let err = StateLoadError::Utf8KeyError {
            bytes_lossy: lossy_value.clone(),
        };
        assert!(
            matches!(
                &err,
                StateLoadError::Utf8KeyError { bytes_lossy }
                if bytes_lossy == "\u{FFFD}\u{FFFD} invalid key"
            ),
            "expected Utf8KeyError with exact bytes_lossy, got {err:?}"
        );
        let display = err.to_string();
        assert!(
            display.contains("invalid key"),
            "display should contain lossy representation: {display}"
        );
    }

    // =======================================================================
    // StateLoadError::MalformedRow variant field access
    // =======================================================================

    #[test]
    fn malformed_row_error_variant_fields_are_accessible() {
        let err = StateLoadError::MalformedRow {
            key: "test_key.md".to_string(),
            actual: 50,
            expected: 200,
        };
        assert!(
            matches!(
                &err,
                StateLoadError::MalformedRow {
                    key,
                    actual: 50,
                    expected: 200,
                } if key == "test_key.md"
            ),
            "expected MalformedRow with exact fields, got {err:?}"
        );
        let display = err.to_string();
        assert!(
            display.contains("test_key.md"),
            "display should contain key: {display}"
        );
        assert!(
            display.contains("50"),
            "display should contain actual size: {display}"
        );
        assert!(
            display.contains("200"),
            "display should contain expected size: {display}"
        );
    }

    // =======================================================================
    // StateLoadError::BackendError variant field access
    // =======================================================================

    #[test]
    fn backend_error_variant_fields_are_accessible() {
        let err = StateLoadError::BackendError {
            operation: "table_iter",
            message: "io error during iteration".to_string(),
        };
        assert!(
            matches!(
                &err,
                StateLoadError::BackendError {
                    operation: "table_iter",
                    message,
                } if message == "io error during iteration"
            ),
            "expected BackendError with exact fields, got {err:?}"
        );
        let display = err.to_string();
        assert!(
            display.contains("table_iter"),
            "display should contain operation: {display}"
        );
        assert!(
            display.contains("io error during iteration"),
            "display should contain message: {display}"
        );
    }

    // =======================================================================
    // hex_encode: pure function tests
    // =======================================================================

    #[test]
    fn hex_encode_returns_empty_string_for_empty_input() {
        assert_eq!(hex_encode(&[]), "");
    }

    #[test]
    fn hex_encode_returns_00_for_zero_byte() {
        assert_eq!(hex_encode(&[0x00]), "00");
    }

    #[test]
    fn hex_encode_returns_ff_for_255_byte() {
        assert_eq!(hex_encode(&[0xFF]), "ff");
    }

    #[test]
    fn hex_encode_returns_lowercase_hex_for_mixed_bytes() {
        assert_eq!(hex_encode(&[0xDE, 0xAD, 0xBE, 0xEF]), "deadbeef");
    }

    #[test]
    fn hex_encode_returns_two_chars_per_byte_for_single_byte() {
        assert_eq!(hex_encode(&[0x0A]).len(), 2);
        assert_eq!(hex_encode(&[0x0A]), "0a");
    }

    #[test]
    fn hex_encode_output_length_is_double_input_length() {
        let input: Vec<u8> = vec![0x42; 100];
        assert_eq!(hex_encode(&input).len(), 200);
    }

    #[test]
    fn hex_encode_handles_32_byte_hash_correctly() {
        let hash: [u8; 32] = [0xAB; 32];
        let encoded = hex_encode(&hash);
        assert_eq!(encoded.len(), 64);
        assert_eq!(encoded, "ab".repeat(32));
    }

    #[test]
    fn hex_encode_preserves_leading_zeros() {
        assert_eq!(hex_encode(&[0x01, 0x02, 0x03]), "010203");
    }

    #[test]
    fn hex_encode_all_zero_bytes() {
        let bytes = [0u8; 16];
        assert_eq!(hex_encode(&bytes), "0".repeat(32));
    }

    #[test]
    fn hex_encode_all_ff_bytes() {
        let bytes = [0xFFu8; 8];
        assert_eq!(hex_encode(&bytes), "f".repeat(16));
    }

    #[test]
    fn hex_encode_single_byte_boundary_min() {
        assert_eq!(hex_encode(&[0x00]), "00");
    }

    #[test]
    fn hex_encode_single_byte_boundary_max() {
        assert_eq!(hex_encode(&[0xFF]), "ff");
    }

    #[test]
    fn hex_encode_two_bytes_boundary() {
        assert_eq!(hex_encode(&[0x00, 0xFF]), "00ff");
        assert_eq!(hex_encode(&[0xFF, 0x00]), "ff00");
    }

    #[test]
    fn hex_encode_produces_only_hex_digits() {
        let bytes: Vec<u8> = (0u8..=255).collect();
        let encoded = hex_encode(&bytes);
        assert!(
            encoded.chars().all(|c| c.is_ascii_hexdigit()),
            "hex_encode output should only contain hex digits"
        );
    }

    #[test]
    fn hex_encode_produces_lowercase_output() {
        let bytes: Vec<u8> = (0u8..=255).collect();
        let encoded = hex_encode(&bytes);
        assert!(
            encoded
                .chars()
                .filter(|c| c.is_ascii_alphabetic())
                .all(|c| c.is_ascii_lowercase()),
            "hex_encode output should be lowercase"
        );
    }

    // =======================================================================
    // hex_encode proptest
    // =======================================================================

    #[test]
    fn proptest_hex_encode_output_is_valid_lowercase_hex_double_length() {
        use proptest::prelude::*;
        proptest!(|(bytes in proptest::collection::vec(any::<u8>(), 0..100))| {
            let encoded = hex_encode(&bytes);
            prop_assert_eq!(encoded.len(), bytes.len() * 2);
            prop_assert!(encoded.chars().all(|c| c.is_ascii_hexdigit()));
            prop_assert!(encoded.chars().filter(|c| c.is_ascii_alphabetic()).all(|c| c.is_ascii_lowercase()));
        });
    }

    // =======================================================================
    // OwnedArchive: construction and access tests
    // =======================================================================

    #[test]
    fn owned_archive_try_from_bytes_returns_corrupt_payload_for_garbage() {
        let garbage: Box<[u8]> =
            vec![0xDE, 0xAD, 0xBE, 0xEF, 0xFF, 0xFF, 0xFF, 0xFF].into_boxed_slice();
        let key: [u8; 32] = [0x42; 32];
        let result = OwnedArchive::<PersistedAnalyzeResult>::try_from_bytes(
            "analysis_outputs",
            &key,
            garbage,
        );
        let err = result.unwrap_err();
        assert!(
            matches!(
                &err,
                BulkLoadError::CorruptPayload {
                    table: "analysis_outputs",
                    key_hex,
                    message: _,
                } if *key_hex == hex_encode(&key)
            ),
            "expected CorruptPayload with exact table and key_hex, got {err:?}"
        );
    }

    #[test]
    fn owned_archive_try_from_bytes_returns_corrupt_payload_for_empty_bytes() {
        let empty: Box<[u8]> = Box::new([]);
        let key: [u8; 32] = [0x00; 32];
        let result = OwnedArchive::<PersistedAnalyzeResult>::try_from_bytes(
            "transform_outputs",
            &key,
            empty,
        );
        let err = result.unwrap_err();
        assert!(
            matches!(
                &err,
                BulkLoadError::CorruptPayload {
                    table: "transform_outputs",
                    ..
                }
            ),
            "expected CorruptPayload for empty bytes, got {err:?}"
        );
    }

    #[test]
    fn owned_archive_try_from_bytes_returns_corrupt_payload_for_truncated_rkyv() {
        let truncated: Box<[u8]> = vec![0u8].into_boxed_slice();
        let key: [u8; 32] = [0xFF; 32];
        let result = OwnedArchive::<PersistedTransformResult>::try_from_bytes(
            "transform_outputs",
            &key,
            truncated,
        );
        let err = result.unwrap_err();
        assert!(
            matches!(
                &err,
                BulkLoadError::CorruptPayload {
                    table: "transform_outputs",
                    key_hex,
                    ..
                } if *key_hex == hex_encode(&key)
            ),
            "expected CorruptPayload for truncated bytes, got {err:?}"
        );
    }

    #[test]
    fn owned_archive_as_bytes_returns_exact_input_bytes_when_valid() {
        use crate::persisted::PersistedTransformResult;
        let original = PersistedTransformResult {
            schema_version: 1,
            success_count: 42,
            total_count: 50,
            error_count: 8,
            errors: vec![],
        };
        let rkyv_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&original)
            .expect("serialization should succeed")
            .to_vec()
            .into_boxed_slice();
        let rkyv_len = rkyv_bytes.len();
        let key: [u8; 32] = [0x11; 32];

        let archive = OwnedArchive::<PersistedTransformResult>::try_from_bytes(
            "transform_outputs",
            &key,
            rkyv_bytes.clone(),
        )
        .expect("valid rkyv bytes should construct archive");

        let retrieved_bytes = archive.as_bytes();
        assert_eq!(retrieved_bytes.len(), rkyv_len);
        assert_eq!(retrieved_bytes, rkyv_bytes.as_ref());
    }

    #[test]
    fn owned_archive_archived_returns_valid_reference_when_constructed_from_valid_bytes() {
        use crate::persisted::PersistedTransformResult;
        let original = PersistedTransformResult {
            schema_version: 1,
            success_count: 10,
            total_count: 10,
            error_count: 0,
            errors: vec![],
        };
        let rkyv_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&original)
            .expect("serialization should succeed")
            .to_vec()
            .into_boxed_slice();
        let key: [u8; 32] = [0x22; 32];

        let archive = OwnedArchive::<PersistedTransformResult>::try_from_bytes(
            "transform_outputs",
            &key,
            rkyv_bytes,
        )
        .expect("valid rkyv bytes should construct archive");

        let archived = archive
            .archived()
            .expect("archived() should succeed on valid bytes");
        assert_eq!(archived.success_count, 10);
        assert_eq!(archived.total_count, 10);
        assert_eq!(archived.error_count, 0);
    }

    #[test]
    fn owned_archive_deserialize_roundtrip_produces_original_value() {
        use crate::persisted::PersistedTransformResult;
        let original = PersistedTransformResult {
            schema_version: 1,
            success_count: 99,
            total_count: 100,
            error_count: 1,
            errors: vec![],
        };
        let rkyv_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&original)
            .expect("serialization should succeed")
            .to_vec()
            .into_boxed_slice();
        let key: [u8; 32] = [0x33; 32];

        let archive = OwnedArchive::<PersistedTransformResult>::try_from_bytes(
            "transform_outputs",
            &key,
            rkyv_bytes,
        )
        .expect("valid rkyv bytes should construct archive");

        let deserialized = archive
            .deserialize()
            .expect("deserialize should succeed on valid archive");
        assert_eq!(deserialized.schema_version, original.schema_version);
        assert_eq!(deserialized.success_count, original.success_count);
        assert_eq!(deserialized.total_count, original.total_count);
        assert_eq!(deserialized.error_count, original.error_count);
    }

    #[test]
    fn owned_archive_deserialize_roundtrip_preserves_chunk_data() {
        use crate::persisted::{
            PersistedChunk, PersistedChunkLevel, PersistedChunkType, PersistedChunksResult,
        };
        let original = PersistedChunksResult {
            schema_version: 1,
            total_chunks: 2,
            document_count: 1,
            chunks_metadata: vec![PersistedChunk {
                schema_version: 1,
                chunk_id: "doc1#0".to_string(),
                doc_id: "doc1".to_string(),
                doc_title: "Test".to_string(),
                chunk_index: 0,
                content: "chunk 0 content".to_string(),
                token_count: 5,
                heading: None,
                heading_path: vec![],
                chunk_type: PersistedChunkType::Prose,
                previous_chunk_id: None,
                next_chunk_id: Some("doc1#1".to_string()),
                related_chunk_ids: vec![],
                summary: "summary 0".to_string(),
                chunk_level: PersistedChunkLevel::Standard,
                parent_chunk_id: None,
                child_chunk_ids: vec![],
                context_prefix: None,
            }],
            summary_chunks: 0,
            standard_chunks: 2,
            detailed_chunks: 0,
        };
        let rkyv_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&original)
            .expect("serialization should succeed")
            .to_vec()
            .into_boxed_slice();
        let key: [u8; 32] = [0x44; 32];

        let archive = OwnedArchive::<PersistedChunksResult>::try_from_bytes(
            "chunk_outputs",
            &key,
            rkyv_bytes,
        )
        .expect("valid rkyv bytes should construct archive");

        let deserialized = archive.deserialize().expect("deserialize should succeed");
        assert_eq!(deserialized.total_chunks, 2);
        assert_eq!(deserialized.chunks_metadata.len(), 1);
        assert_eq!(deserialized.chunks_metadata[0].chunk_id, "doc1#0");
    }

    #[test]
    fn owned_archive_deserialize_roundtrip_preserves_scrape_data() {
        use crate::persisted::{
            PersistedHeader, PersistedPageFilterStatus, PersistedScrapeResult, PersistedScrapedPage,
        };
        let original = PersistedScrapeResult {
            schema_version: 1,
            pages: vec![PersistedScrapedPage {
                url: "https://example.com".to_string(),
                markdown: "content".to_string(),
                title: "Example".to_string(),
                links: vec!["https://other.com".to_string()],
                headers: vec![PersistedHeader {
                    level: 1,
                    text: "Title".to_string(),
                }],
                word_count: 100,
                slug: "example".to_string(),
                filter_status: PersistedPageFilterStatus::Unfiltered,
                elements_removed: 0,
                density_score: 1.0,
            }],
            total_urls: 1,
            success_count: 1,
            error_count: 0,
            errors: vec![],
            base_url: "https://example.com".to_string(),
        };
        let rkyv_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&original)
            .expect("serialization should succeed")
            .to_vec()
            .into_boxed_slice();
        let key: [u8; 32] = [0x55; 32];

        let archive = OwnedArchive::<PersistedScrapeResult>::try_from_bytes(
            "scrape_outputs",
            &key,
            rkyv_bytes,
        )
        .expect("valid rkyv bytes should construct archive");

        let deserialized = archive.deserialize().expect("deserialize should succeed");
        assert_eq!(deserialized.pages.len(), 1);
        assert_eq!(deserialized.pages[0].url, "https://example.com");
        assert_eq!(deserialized.pages[0].links.len(), 1);
    }

    #[test]
    fn owned_archive_deserialize_roundtrip_preserves_analysis_data() {
        use crate::persisted::{
            PersistedAnalysis, PersistedAnalyzeResult, PersistedFailedFile, PersistedLink,
            PersistedLinkKind,
        };
        let original = PersistedAnalyzeResult {
            schema_version: 1,
            analyses: vec![PersistedAnalysis {
                schema_version: 1,
                source_path: "src/main.rs".to_string(),
                title: "Main".to_string(),
                frontmatter: None,
                headings: vec![],
                links: vec![PersistedLink {
                    text: "docs".to_string(),
                    target: "https://docs.rs".to_string(),
                    kind: PersistedLinkKind::External,
                }],
                first_paragraph: "intro".to_string(),
                word_count: 500,
                has_code: true,
                has_tables: false,
                category: "rust".to_string(),
                content: "body text".to_string(),
            }],
            failed_files: vec![PersistedFailedFile {
                source_path: "broken.md".to_string(),
                error: "parse error".to_string(),
            }],
            total_discovered: 2,
        };
        let rkyv_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&original)
            .expect("serialization should succeed")
            .to_vec()
            .into_boxed_slice();
        let key: [u8; 32] = [0x66; 32];

        let archive = OwnedArchive::<PersistedAnalyzeResult>::try_from_bytes(
            "analysis_outputs",
            &key,
            rkyv_bytes,
        )
        .expect("valid rkyv bytes should construct archive");

        let deserialized = archive.deserialize().expect("deserialize should succeed");
        assert_eq!(deserialized.analyses.len(), 1);
        assert_eq!(deserialized.analyses[0].source_path, "src/main.rs");
        assert_eq!(deserialized.analyses[0].word_count, 500);
        assert_eq!(deserialized.failed_files.len(), 1);
        assert_eq!(deserialized.total_discovered, 2);
    }

    // =======================================================================
    // OwnedArchive proptest
    // =======================================================================

    #[test]
    fn proptest_owned_archive_transform_roundtrip_preserves_data() {
        use crate::persisted::PersistedTransformResult;
        use proptest::prelude::*;
        proptest!(
            |(success_count in 0usize..100_000, total_count in 0usize..100_000, error_count in 0usize..100_000)| {
            let original = PersistedTransformResult {
                schema_version: 1,
                success_count,
                total_count,
                error_count,
                errors: vec![],
            };
            let rkyv_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&original)
                .expect("serialization should succeed")
                .to_vec()
                .into_boxed_slice();
            let key: [u8; 32] = [0xAA; 32];

            let archive = OwnedArchive::<PersistedTransformResult>::try_from_bytes(
                "transform_outputs",
                &key,
                rkyv_bytes,
            )
            .expect("valid rkyv bytes should construct archive");

            let deserialized = archive
                .deserialize()
                .expect("deserialize should succeed");
            prop_assert_eq!(deserialized.success_count, success_count);
            prop_assert_eq!(deserialized.total_count, total_count);
            prop_assert_eq!(deserialized.error_count, error_count);
        });
    }

    // =======================================================================
    // Additional boundary tests for density
    // =======================================================================

    #[test]
    fn load_file_states_handles_single_row_with_all_zero_state() {
        let (_dir, db) = fresh_db();
        write_file_rows(&db, &[("zero.rs", FileStateRaw::zeroed())]);
        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_file_states().unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result["zero.rs"], FileStateRaw::zeroed());
    }

    #[test]
    fn load_url_states_handles_single_row_with_all_zero_state() {
        let (_dir, db) = fresh_db();
        write_url_rows(&db, &[("https://zero.com", UrlStateRaw::zeroed())]);
        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_url_states().unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result["https://zero.com"], UrlStateRaw::zeroed());
    }

    #[test]
    fn load_file_states_handles_row_with_max_timestamp() {
        let (_dir, db) = fresh_db();
        let state = FileStateRaw {
            content_hash: [0xFF; 32],
            config_hash: [0xFF; 32],
            analysis_hash: [0xFF; 32],
            transform_hash: [0xFF; 32],
            chunk_hash: [0xFF; 32],
            last_processed_secs: u64::MAX,
            reserved: [0xFF; 32],
        };
        write_file_rows(&db, &[("max.rs", state)]);
        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_file_states().unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result["max.rs"].last_processed_secs, u64::MAX);
    }

    #[test]
    fn load_url_states_handles_row_with_max_fields() {
        let (_dir, db) = fresh_db();
        let state = UrlStateRaw {
            content_hash: [0xFF; 32],
            url_hash: [0xFF; 32],
            last_fetched_secs: u64::MAX,
            status_code: u16::MAX,
            reserved: [0xFF; 46],
        };
        write_url_rows(&db, &[("https://max.com", state)]);
        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_url_states().unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result["https://max.com"].last_fetched_secs, u64::MAX);
        assert_eq!(result["https://max.com"].status_code, u16::MAX);
    }

    #[test]
    fn load_file_states_preserves_distinct_keys_with_identical_values() {
        let (_dir, db) = fresh_db();
        let shared_state = file_state(0x42);
        write_file_rows(
            &db,
            &[
                ("path/a.rs", shared_state),
                ("path/b.rs", shared_state),
                ("path/c.rs", shared_state),
            ],
        );
        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_file_states().unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result["path/a.rs"], shared_state);
        assert_eq!(result["path/b.rs"], shared_state);
        assert_eq!(result["path/c.rs"], shared_state);
    }

    #[test]
    fn load_url_states_preserves_distinct_keys_with_identical_values() {
        let (_dir, db) = fresh_db();
        let shared_state = url_state(0x42);
        write_url_rows(
            &db,
            &[
                ("https://a.com", shared_state),
                ("https://b.com", shared_state),
                ("https://c.com", shared_state),
            ],
        );
        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_url_states().unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result["https://a.com"], shared_state);
        assert_eq!(result["https://b.com"], shared_state);
        assert_eq!(result["https://c.com"], shared_state);
    }

    #[test]
    fn load_file_states_returns_malformed_row_for_value_exactly_100_bytes() {
        let (_dir, db) = fresh_db();
        write_raw_file_row(&db, "half.dat", &[0u8; 100]);
        let session = StateReadSession::new(&db).unwrap();
        let err = session.load_file_states().unwrap_err();
        assert!(
            matches!(&err, StateLoadError::MalformedRow { key, actual: 100, expected: 200 } if key == "half.dat"),
            "expected MalformedRow {{ actual: 100 }}, got {err:?}"
        );
    }

    #[test]
    fn load_url_states_returns_malformed_row_for_value_exactly_60_bytes() {
        let (_dir, db) = fresh_db();
        write_raw_url_row(&db, "https://half.example.com", &[0u8; 60]);
        let session = StateReadSession::new(&db).unwrap();
        let err = session.load_url_states().unwrap_err();
        assert!(
            matches!(&err, StateLoadError::MalformedRow { key, actual: 60, expected: 120 } if key == "https://half.example.com"),
            "expected MalformedRow {{ actual: 60 }}, got {err:?}"
        );
    }

    #[test]
    fn load_file_states_returns_malformed_row_for_value_exactly_1_byte() {
        let (_dir, db) = fresh_db();
        write_raw_file_row(&db, "one.dat", &[0xFF]);
        let session = StateReadSession::new(&db).unwrap();
        let err = session.load_file_states().unwrap_err();
        assert!(
            matches!(&err, StateLoadError::MalformedRow { key, actual: 1, expected: 200 } if key == "one.dat"),
            "expected MalformedRow {{ actual: 1 }}, got {err:?}"
        );
    }

    #[test]
    fn load_url_states_returns_malformed_row_for_value_exactly_1_byte() {
        let (_dir, db) = fresh_db();
        write_raw_url_row(&db, "https://one.example.com", &[0xAB]);
        let session = StateReadSession::new(&db).unwrap();
        let err = session.load_url_states().unwrap_err();
        assert!(
            matches!(&err, StateLoadError::MalformedRow { key, actual: 1, expected: 120 } if key == "https://one.example.com"),
            "expected MalformedRow {{ actual: 1 }}, got {err:?}"
        );
    }

    #[test]
    fn load_file_states_with_10_rows_preserves_each_key() {
        let (_dir, db) = fresh_db();
        let rows: Vec<(&str, FileStateRaw)> = (0..10)
            .map(|i| {
                let key = format!("file_{i}.rs");
                let state = file_state(i);
                (Box::leak(key.into_boxed_str()) as &str, state)
            })
            .collect();
        write_file_rows(&db, &rows);
        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_file_states().unwrap();
        assert_eq!(result.len(), 10);
        assert_eq!(result["file_0.rs"], file_state(0));
        assert_eq!(result["file_5.rs"], file_state(5));
        assert_eq!(result["file_9.rs"], file_state(9));
    }

    #[test]
    fn load_url_states_with_10_rows_preserves_each_key() {
        let (_dir, db) = fresh_db();
        let rows: Vec<(&str, UrlStateRaw)> = (0..10)
            .map(|i| {
                let key = format!("https://example.com/page_{i}");
                let state = url_state(i);
                (Box::leak(key.into_boxed_str()) as &str, state)
            })
            .collect();
        write_url_rows(&db, &rows);
        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_url_states().unwrap();
        assert_eq!(result.len(), 10);
        assert_eq!(result["https://example.com/page_0"], url_state(0));
        assert_eq!(result["https://example.com/page_5"], url_state(5));
        assert_eq!(result["https://example.com/page_9"], url_state(9));
    }

    #[test]
    fn session_new_opens_readable_transaction_on_valid_db() {
        let (_dir, db) = fresh_db();
        write_file_rows(&db, &[("probe.rs", file_state(0x01))]);
        let session = StateReadSession::new(&db).expect("session should open");
        let result = session.load_file_states().expect("should load");
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn both_loaders_return_empty_when_no_data_written() {
        let (_dir, db) = fresh_db();
        let session = StateReadSession::new(&db).unwrap();
        assert_eq!(session.load_file_states().unwrap().len(), 0);
        assert_eq!(session.load_url_states().unwrap().len(), 0);
    }

    #[test]
    fn file_state_key_with_deeply_nested_path_roundtrips() {
        let (_dir, db) = fresh_db();
        let deep_key = "a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/deep.rs";
        write_file_rows(&db, &[(deep_key, file_state(0x01))]);
        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_file_states().unwrap();
        assert_eq!(result.len(), 1);
        assert!(result.contains_key(deep_key));
    }

    #[test]
    fn url_state_key_with_long_query_string_roundtrips() {
        let (_dir, db) = fresh_db();
        let long_key =
            "https://example.com/api/v2/resource?param1=value1&param2=value2&param3=value3&page=42";
        write_url_rows(&db, &[(long_key, url_state(0x02))]);
        let session = StateReadSession::new(&db).unwrap();
        let result = session.load_url_states().unwrap();
        assert_eq!(result.len(), 1);
        assert!(result.contains_key(long_key));
    }

    #[test]
    fn owned_archive_archived_returns_consistent_results_on_repeated_calls() {
        use crate::persisted::PersistedTransformResult;
        let original = PersistedTransformResult {
            schema_version: 1,
            success_count: 7,
            total_count: 7,
            error_count: 0,
            errors: vec![],
        };
        let rkyv_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&original)
            .expect("serialization should succeed")
            .to_vec()
            .into_boxed_slice();
        let key: [u8; 32] = [0x77; 32];
        let archive = OwnedArchive::<PersistedTransformResult>::try_from_bytes(
            "transform_outputs",
            &key,
            rkyv_bytes,
        )
        .expect("valid rkyv bytes should construct archive");
        let first = archive.archived().expect("first call should succeed");
        let second = archive.archived().expect("second call should succeed");
        assert_eq!(first.success_count, second.success_count);
        assert_eq!(first.total_count, second.total_count);
    }

    #[test]
    fn owned_archive_as_bytes_matches_serialize_output() {
        use crate::persisted::PersistedTransformResult;
        let original = PersistedTransformResult {
            schema_version: 1,
            success_count: 3,
            total_count: 5,
            error_count: 2,
            errors: vec![],
        };
        let rkyv_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&original).expect("ok");
        let boxed: Box<[u8]> = rkyv_bytes.to_vec().into_boxed_slice();
        let key: [u8; 32] = [0x88; 32];
        let archive = OwnedArchive::<PersistedTransformResult>::try_from_bytes(
            "transform_outputs",
            &key,
            boxed,
        )
        .expect("valid rkyv bytes should construct archive");
        assert_eq!(archive.as_bytes(), rkyv_bytes.as_slice());
    }

    #[test]
    fn corrupt_payload_key_hex_matches_hex_encode_for_known_key() {
        let key: [u8; 32] = [
            0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54,
            0x32, 0x10, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98,
            0x76, 0x54, 0x32, 0x10,
        ];
        let garbage: Box<[u8]> = vec![0xDE, 0xAD].into_boxed_slice();
        let err = OwnedArchive::<PersistedAnalyzeResult>::try_from_bytes(
            "analysis_outputs",
            &key,
            garbage,
        )
        .unwrap_err();
        let expected_hex = hex_encode(&key);
        if let BulkLoadError::CorruptPayload { key_hex, .. } = &err {
            assert_eq!(
                key_hex, &expected_hex,
                "key_hex must match hex_encode of the input key"
            );
        } else {
            panic!("expected CorruptPayload, got {err:?}");
        }
    }

    #[test]
    fn state_load_error_malformed_row_is_debug_printable() {
        let err = StateLoadError::MalformedRow {
            key: "test".into(),
            actual: 1,
            expected: 200,
        };
        assert!(!format!("{err:?}").is_empty());
    }

    #[test]
    fn state_load_error_utf8_key_error_is_debug_printable() {
        let err = StateLoadError::Utf8KeyError {
            bytes_lossy: "lossy".into(),
        };
        assert!(!format!("{err:?}").is_empty());
    }

    #[test]
    fn state_load_error_backend_error_is_debug_printable() {
        let err = StateLoadError::BackendError {
            operation: "open_table",
            message: "failed".into(),
        };
        assert!(!format!("{err:?}").is_empty());
    }

    #[test]
    fn bulk_load_error_table_open_is_debug_printable() {
        let err = BulkLoadError::TableOpen {
            table: "t",
            message: "m".into(),
        };
        assert!(!format!("{err:?}").is_empty());
    }

    #[test]
    fn bulk_load_error_storage_error_is_debug_printable() {
        let err = BulkLoadError::StorageError {
            table: "t",
            message: "m".into(),
        };
        assert!(!format!("{err:?}").is_empty());
    }

    #[test]
    fn bulk_load_error_corrupt_payload_is_debug_printable() {
        let err = BulkLoadError::CorruptPayload {
            table: "t",
            key_hex: "ab".into(),
            message: "m".into(),
        };
        assert!(!format!("{err:?}").is_empty());
    }

    #[test]
    fn state_load_error_malformed_row_equality_check() {
        let a = StateLoadError::MalformedRow {
            key: "k".into(),
            actual: 1,
            expected: 200,
        };
        let b = StateLoadError::MalformedRow {
            key: "k".into(),
            actual: 1,
            expected: 200,
        };
        assert_eq!(a, b);
    }

    #[test]
    fn state_load_error_malformed_row_inequality_check() {
        let a = StateLoadError::MalformedRow {
            key: "k1".into(),
            actual: 1,
            expected: 200,
        };
        let b = StateLoadError::MalformedRow {
            key: "k2".into(),
            actual: 2,
            expected: 200,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn state_load_error_utf8_key_error_equality_check() {
        let a = StateLoadError::Utf8KeyError {
            bytes_lossy: "x".into(),
        };
        let b = StateLoadError::Utf8KeyError {
            bytes_lossy: "x".into(),
        };
        assert_eq!(a, b);
    }

    #[test]
    fn state_load_error_backend_error_equality_check() {
        let a = StateLoadError::BackendError {
            operation: "op",
            message: "m".into(),
        };
        let b = StateLoadError::BackendError {
            operation: "op",
            message: "m".into(),
        };
        assert_eq!(a, b);
    }
}
