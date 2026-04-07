//! RAII read session and generic table loaders.

use std::collections::HashMap;
use std::marker::PhantomData;

use itertools::Itertools;
use redb::{Database, ReadTransaction, ReadableTable, TableDefinition};

use crate::persisted::{
    PersistedAnalyzeResult, PersistedChunksResult, PersistedScrapeResult, PersistedTransformResult,
};
use crate::state::{
    analysis_outputs_table, chunk_outputs_table, file_state_table, scrape_outputs_table,
    transform_outputs_table, url_state_table, FileStateRaw, StateLoadError, UrlStateRaw,
    TABLE_NAME_ANALYSIS_OUTPUTS, TABLE_NAME_CHUNK_OUTPUTS, TABLE_NAME_FILE_STATE,
    TABLE_NAME_SCRAPE_OUTPUTS, TABLE_NAME_TRANSFORM_OUTPUTS, TABLE_NAME_URL_STATE,
};

use super::error::BulkLoadError;
use super::owned_archive::OwnedArchive;

// ---------------------------------------------------------------------------
// StateReadSession
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

    /// Bulk load archived [`PersistedAnalyzeResult`] outputs.
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

    /// Bulk load archived [`PersistedTransformResult`] outputs.
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

    /// Bulk load archived [`PersistedChunksResult`] outputs.
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

    /// Bulk load archived [`PersistedScrapeResult`] outputs.
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
    /// # Errors
    ///
    /// - [`StateLoadError::MalformedRow`] if any row's value length != 200 bytes.
    /// - [`StateLoadError::BackendError`] if the redb table cannot be opened.
    /// - [`StateLoadError::Utf8KeyError`] if a key is not valid UTF-8.
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
    /// # Errors
    ///
    /// - [`StateLoadError::MalformedRow`] if any row's value length != 120 bytes.
    /// - [`StateLoadError::BackendError`] if the redb table cannot be opened.
    /// - [`StateLoadError::Utf8KeyError`] if a key is not valid UTF-8.
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
// Generic bulk loader
// ---------------------------------------------------------------------------

/// Generic bulk loader: read N entries from a redb table inside a shared
/// read transaction, validate bytes, and return a `HashMap`.
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
                    let bytes: Box<[u8]> = access_guard.value().to_vec().into_boxed_slice();
                    let archive = OwnedArchive::<T>::try_from_bytes(table_name, hash, bytes)?;
                    acc.insert(*hash, archive);
                    Ok(acc)
                }
                None => Ok(acc), // Q-06: missing key silently omitted
            }
        },
    )
}

// ---------------------------------------------------------------------------
// Generic Pod table scanner
// ---------------------------------------------------------------------------

/// Generic full-table scanner for Pod state tables (`&str` → `&[u8]`).
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
