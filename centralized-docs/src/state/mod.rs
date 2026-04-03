//! State database table definitions and Pod types for raw state storage.
//!
//! Defines all 8 redb table definitions for the state database schema:
//! - **Pod state tables**: `file_state`, `url_state` (fixed-size values, `&str` keys)
//! - **rkyv output tables**: `analysis_outputs`, `transform_outputs`, `chunk_outputs`,
//!   `scrape_outputs`, `snapshots` (variable-size values, `&[u8]` hash keys)
//! - **Metadata table**: `metadata` (`&str` keys, `&str` values)
//!
//! # Pod Structs
//!
//! [`FileStateRaw`] (200 bytes) and [`UrlStateRaw`] (120 bytes) are `#[repr(C)]` fixed-size
//! structs designed for zero-copy reads from redb. They use explicit `reserved` padding to
//! eliminate undefined padding bytes. Safe conversion is provided via `from_bytes`/`to_bytes`
//! methods instead of `bytemuck` (which requires `unsafe impl` blocked by `forbid(unsafe_code)`).
//!
//! # Table Definitions
//!
//! All 8 tables are declared as `const` [`redb::TableDefinition`] values. Accessor functions
//! (`file_state_table()`, etc.) return these constants. [`initialize_tables`] creates all
//! tables in a single write transaction.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

pub mod bulk_load;

use redb::{Database, TableDefinition};
use std::mem::size_of;

// ---------------------------------------------------------------------------
// Static size assertions (compile-time)
// ---------------------------------------------------------------------------

/// Compile-time assertion: `FileStateRaw` is exactly 200 bytes.
const _FILE_STATE_RAW_SIZE_ASSERT: () = assert!(size_of::<FileStateRaw>() == 200);

/// Compile-time assertion: `UrlStateRaw` is exactly 120 bytes.
const _URL_STATE_RAW_SIZE_ASSERT: () = assert!(size_of::<UrlStateRaw>() == 120);

// ---------------------------------------------------------------------------
// Table name constants (single source of truth)
// ---------------------------------------------------------------------------

/// Table name: `"file_state"`.
pub const TABLE_NAME_FILE_STATE: &str = "file_state";
/// Table name: `"url_state"`.
pub const TABLE_NAME_URL_STATE: &str = "url_state";
/// Table name: `"analysis_outputs"`.
pub const TABLE_NAME_ANALYSIS_OUTPUTS: &str = "analysis_outputs";
/// Table name: `"transform_outputs"`.
pub const TABLE_NAME_TRANSFORM_OUTPUTS: &str = "transform_outputs";
/// Table name: `"chunk_outputs"`.
pub const TABLE_NAME_CHUNK_OUTPUTS: &str = "chunk_outputs";
/// Table name: `"scrape_outputs"`.
pub const TABLE_NAME_SCRAPE_OUTPUTS: &str = "scrape_outputs";
/// Table name: `"snapshots"`.
pub const TABLE_NAME_SNAPSHOTS: &str = "snapshots";
/// Table name: `"metadata"`.
pub const TABLE_NAME_METADATA: &str = "metadata";

// ---------------------------------------------------------------------------
// Pod type: FileStateRaw (200 bytes)
// ---------------------------------------------------------------------------

/// Fixed-size file state. 200 bytes. Zero-copy read from redb.
///
/// # Layout
///
/// ```text
/// offset   size  field
/// 0        32    content_hash: [u8; 32]    // SHA-256 of file bytes
/// 32       32    config_hash: [u8; 32]     // SHA-256 of category config (or zeroed)
/// 64       32    analysis_hash: [u8; 32]   // FK -> analysis_outputs key
/// 96       32    transform_hash: [u8; 32]  // FK -> transform_outputs key
/// 128      32    chunk_hash: [u8; 32]      // FK -> chunk_outputs key
/// 160       8    last_processed_secs: u64   // unix timestamp
/// 168      32    reserved: [u8; 32]       // future-proof padding
/// Total: 200 bytes
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct FileStateRaw {
    /// SHA-256 of file bytes.
    pub content_hash: [u8; 32],
    /// SHA-256 of category config (or zeroed).
    pub config_hash: [u8; 32],
    /// FK -> `analysis_outputs` key.
    pub analysis_hash: [u8; 32],
    /// FK -> `transform_outputs` key.
    pub transform_hash: [u8; 32],
    /// FK -> `chunk_outputs` key.
    pub chunk_hash: [u8; 32],
    /// Unix timestamp of last processing.
    pub last_processed_secs: u64,
    /// Future-proof padding. All zeros until needed.
    pub reserved: [u8; 32],
}

impl FileStateRaw {
    /// Fixed byte size of `FileStateRaw`.
    pub const SIZE: usize = 200;

    /// Create an all-zero `FileStateRaw`.
    #[must_use]
    pub const fn zeroed() -> Self {
        Self {
            content_hash: [0u8; 32],
            config_hash: [0u8; 32],
            analysis_hash: [0u8; 32],
            transform_hash: [0u8; 32],
            chunk_hash: [0u8; 32],
            last_processed_secs: 0,
            reserved: [0u8; 32],
        }
    }

    /// Deserialize from exactly 200 raw bytes.
    ///
    /// # Errors
    ///
    /// Returns [`StateError::PodSizeMismatch`] if `bytes.len() != 200`.
    /// Returns [`StateError::PodCastFailed`] if field extraction fails.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, StateError> {
        if bytes.len() != Self::SIZE {
            return Err(StateError::PodSizeMismatch {
                table: TABLE_NAME_FILE_STATE,
                expected: Self::SIZE,
                actual: bytes.len(),
            });
        }

        Ok(Self {
            content_hash: read_array::<32>(bytes, 0)?,
            config_hash: read_array::<32>(bytes, 32)?,
            analysis_hash: read_array::<32>(bytes, 64)?,
            transform_hash: read_array::<32>(bytes, 96)?,
            chunk_hash: read_array::<32>(bytes, 128)?,
            last_processed_secs: u64::from_le_bytes(read_array::<8>(bytes, 160)?),
            reserved: read_array::<32>(bytes, 168)?,
        })
    }

    /// Serialize to exactly 200 raw bytes.
    #[must_use]
    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut result = [0u8; Self::SIZE];
        copy_into(&mut result, 0, &self.content_hash);
        copy_into(&mut result, 32, &self.config_hash);
        copy_into(&mut result, 64, &self.analysis_hash);
        copy_into(&mut result, 96, &self.transform_hash);
        copy_into(&mut result, 128, &self.chunk_hash);
        copy_into(&mut result, 160, &self.last_processed_secs.to_le_bytes());
        copy_into(&mut result, 168, &self.reserved);
        result
    }
}

// ---------------------------------------------------------------------------
// Pod type: UrlStateRaw (120 bytes)
// ---------------------------------------------------------------------------

/// Fixed-size URL state. 120 bytes. Zero-copy read from redb.
///
/// # Layout
///
/// ```text
/// offset   size  field
/// 0        32    content_hash: [u8; 32]    // SHA-256 of scraped markdown content
/// 32       32    url_hash: [u8; 32]        // FK -> scrape_outputs key
/// 64        8    last_fetched_secs: u64    // unix timestamp
/// 72        2    status_code: u16          // last HTTP status
/// 74       46    reserved: [u8; 46]       // future ETag/Last-Modified slot
/// Total: 120 bytes
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct UrlStateRaw {
    /// SHA-256 of scraped markdown content.
    pub content_hash: [u8; 32],
    /// FK -> `scrape_outputs` key.
    pub url_hash: [u8; 32],
    /// Unix timestamp of last fetch.
    pub last_fetched_secs: u64,
    /// Last HTTP status code.
    pub status_code: u16,
    /// Future-proof padding (ETag/Last-Modified slot).
    pub reserved: [u8; 46],
}

impl UrlStateRaw {
    /// Fixed byte size of `UrlStateRaw`.
    pub const SIZE: usize = 120;

    /// Create an all-zero `UrlStateRaw`.
    #[must_use]
    pub const fn zeroed() -> Self {
        Self {
            content_hash: [0u8; 32],
            url_hash: [0u8; 32],
            last_fetched_secs: 0,
            status_code: 0,
            reserved: [0u8; 46],
        }
    }

    /// Deserialize from exactly 120 raw bytes.
    ///
    /// # Errors
    ///
    /// Returns [`StateError::PodSizeMismatch`] if `bytes.len() != 120`.
    /// Returns [`StateError::PodCastFailed`] if field extraction fails.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, StateError> {
        if bytes.len() != Self::SIZE {
            return Err(StateError::PodSizeMismatch {
                table: TABLE_NAME_URL_STATE,
                expected: Self::SIZE,
                actual: bytes.len(),
            });
        }

        Ok(Self {
            content_hash: read_array::<32>(bytes, 0)?,
            url_hash: read_array::<32>(bytes, 32)?,
            last_fetched_secs: u64::from_le_bytes(read_array::<8>(bytes, 64)?),
            status_code: u16::from_le_bytes(read_array::<2>(bytes, 72)?),
            reserved: read_array::<46>(bytes, 74)?,
        })
    }

    /// Serialize to exactly 120 raw bytes.
    #[must_use]
    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut result = [0u8; Self::SIZE];
        copy_into(&mut result, 0, &self.content_hash);
        copy_into(&mut result, 32, &self.url_hash);
        copy_into(&mut result, 64, &self.last_fetched_secs.to_le_bytes());
        copy_into(&mut result, 72, &self.status_code.to_le_bytes());
        copy_into(&mut result, 74, &self.reserved);
        result
    }
}

// ---------------------------------------------------------------------------
// Internal byte helpers (safe, no bytemuck)
// ---------------------------------------------------------------------------

/// Copy `src` into `dest` at the given offset. Silently no-ops if out of bounds.
fn copy_into(dest: &mut [u8], offset: usize, src: &[u8]) {
    let end = offset.saturating_add(src.len());
    if let Some(slice) = dest.get_mut(offset..end) {
        slice.copy_from_slice(src);
    }
}

/// Read a fixed-size array from `src` at the given offset.
fn read_array<const N: usize>(src: &[u8], offset: usize) -> Result<[u8; N], StateError> {
    let end = offset
        .checked_add(N)
        .ok_or_else(|| StateError::PodCastFailed {
            type_name: "slice",
            message: format!("offset overflow at offset {offset}"),
        })?;
    let slice = src
        .get(offset..end)
        .ok_or_else(|| StateError::PodCastFailed {
            type_name: "slice",
            message: format!("slice [{offset}..{end}) out of bounds (len={})", src.len()),
        })?;
    slice.try_into().map_err(|_| StateError::PodCastFailed {
        type_name: "slice",
        message: format!("failed to convert {N}-byte slice to array"),
    })
}

// ---------------------------------------------------------------------------
// StateError -- error taxonomy for state database operations
// ---------------------------------------------------------------------------

/// Error type for state database operations.
///
/// Covers all failure modes of the two-transaction bulk-load architecture:
/// database lifecycle, Pod read/write, rkyv archive validation, table operations,
/// and constraint violations.
#[non_exhaustive]
#[derive(Debug, thiserror::Error, Clone)]
pub enum StateError {
    // -- Database lifecycle errors --
    /// The state database could not be opened.
    #[error("failed to open state database at {path}: {detail}")]
    OpenFailed {
        /// Path to the database file.
        path: std::path::PathBuf,
        /// Error description.
        detail: String,
    },

    /// A read transaction could not be started.
    #[error("failed to begin read transaction: {message}")]
    ReadTransactionFailed {
        /// Error description.
        message: String,
    },

    /// A write transaction could not be started.
    #[error("failed to begin write transaction: {message}")]
    WriteTransactionFailed {
        /// Error description.
        message: String,
    },

    // -- Pod read/write errors --
    /// A value read from a Pod table has the wrong byte count.
    #[error("pod value size mismatch for table {table}: expected {expected} bytes, got {actual}")]
    PodSizeMismatch {
        /// Table name where the mismatch occurred.
        table: &'static str,
        /// Expected byte count.
        expected: usize,
        /// Actual byte count.
        actual: usize,
    },

    /// A Pod cast failed (alignment or size violation).
    #[error("pod cast failed for type {type_name}: {message}")]
    PodCastFailed {
        /// Type that failed to cast.
        type_name: &'static str,
        /// Error description.
        message: String,
    },

    // -- rkyv archive errors --
    /// Bytes read from an rkyv table are not a valid archive.
    #[error("invalid rkyv archive for type {type_name}: {message}")]
    InvalidArchive {
        /// Expected type name.
        type_name: &'static str,
        /// Error description.
        message: String,
    },

    /// An rkyv deserialization failed.
    #[error("rkyv deserialization failed for type {type_name}: {message}")]
    DeserializationFailed {
        /// Type that failed to deserialize.
        type_name: &'static str,
        /// Error description.
        message: String,
    },

    /// Archive validation failed for a specific key in an rkyv output table.
    #[error("archive validation failed for key {key_hex}: {message}")]
    ArchiveValidationFailed {
        /// Hex-encoded key of the failed archive.
        key_hex: String,
        /// Error description.
        message: String,
    },

    /// An rkyv serialization failed.
    #[error("rkyv serialization failed for type {type_name}: {message}")]
    SerializationFailed {
        /// Type that failed to serialize.
        type_name: &'static str,
        /// Error description.
        message: String,
    },

    // -- Table operation errors --
    /// A table could not be opened within a transaction.
    #[error("failed to open table {table}: {message}")]
    TableOpenFailed {
        /// Table that failed to open.
        table: &'static str,
        /// Error description.
        message: String,
    },

    /// A key was not found in the expected table.
    #[error("key not found in {table}")]
    KeyNotFound {
        /// Table where the key was expected.
        table: &'static str,
    },

    /// A redb storage error occurred.
    #[error("redb storage error during {operation}: {message}")]
    StorageError {
        /// Operation that triggered the error.
        operation: &'static str,
        /// Error description.
        message: String,
    },

    /// A write transaction commit failed.
    #[error("failed to commit state changes: {message}")]
    CommitFailed {
        /// Error description.
        message: String,
    },

    // -- Constraint violations --
    /// A hash key has the wrong length (not 32 bytes).
    #[error("hash key has wrong length: expected 32 bytes, got {actual}")]
    InvalidHashKeyLength {
        /// Actual key length.
        actual: usize,
    },

    /// A source path key violates the key format invariant.
    #[error("invalid source path key: {reason}")]
    InvalidSourcePath {
        /// Why the path is invalid.
        reason: String,
    },

    /// A URL key violates the URL format invariant.
    #[error("invalid URL key: {reason}")]
    InvalidUrlKey {
        /// Why the URL is invalid.
        reason: String,
    },
}

// ---------------------------------------------------------------------------
// StateLoadError -- error taxonomy for bulk state loading
// ---------------------------------------------------------------------------

/// Errors that can occur during bulk state loading.
///
/// Covers all failure modes of `StateReadSession::load_file_states` and
/// `StateReadSession::load_url_states`: corrupt rows, non-UTF-8 keys,
/// and backend failures.
#[non_exhaustive]
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum StateLoadError {
    /// A raw value in the table has an unexpected byte length.
    ///
    /// The entire load is aborted; no partial map is returned.
    #[error("malformed raw state row for key {key:?}: got {actual} bytes, expected {expected}")]
    MalformedRow {
        /// Key of the malformed row.
        key: String,
        /// Actual byte count of the value.
        actual: usize,
        /// Expected byte count (200 for `FileStateRaw`, 120 for `UrlStateRaw`).
        expected: usize,
    },

    /// A table key is not valid UTF-8.
    ///
    /// Stored for future-proofing; current table definitions use `&str` keys
    /// which enforce UTF-8 at the type level.
    #[error("non-UTF-8 key in state table: {bytes_lossy:?}")]
    Utf8KeyError {
        /// Lossy UTF-8 representation of the invalid key bytes.
        bytes_lossy: String,
    },

    /// The underlying redb backend failed during a table operation.
    #[error("cache backend error during {operation}: {message}")]
    BackendError {
        /// Operation that triggered the error.
        operation: &'static str,
        /// Error description from redb.
        message: String,
    },
}

// ---------------------------------------------------------------------------
// Table definitions (compile-time constants)
// ---------------------------------------------------------------------------

/// Pod state table: file source path -> raw file state bytes (200 bytes).
const FILE_STATE_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new(TABLE_NAME_FILE_STATE);

/// Pod state table: canonical URL -> raw URL state bytes (120 bytes).
const URL_STATE_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new(TABLE_NAME_URL_STATE);

/// rkyv output table: 32-byte analysis hash -> rkyv-archived Analysis bytes.
const ANALYSIS_OUTPUTS_TABLE: TableDefinition<&[u8], &[u8]> =
    TableDefinition::new(TABLE_NAME_ANALYSIS_OUTPUTS);

/// rkyv output table: 32-byte transform hash -> rkyv-archived String bytes.
const TRANSFORM_OUTPUTS_TABLE: TableDefinition<&[u8], &[u8]> =
    TableDefinition::new(TABLE_NAME_TRANSFORM_OUTPUTS);

/// rkyv output table: 32-byte chunk hash -> rkyv-archived Vec<Chunk> bytes.
const CHUNK_OUTPUTS_TABLE: TableDefinition<&[u8], &[u8]> =
    TableDefinition::new(TABLE_NAME_CHUNK_OUTPUTS);

/// rkyv output table: 32-byte URL hash -> rkyv-archived `ScrapedPage` bytes.
const SCRAPE_OUTPUTS_TABLE: TableDefinition<&[u8], &[u8]> =
    TableDefinition::new(TABLE_NAME_SCRAPE_OUTPUTS);

/// rkyv output table: 32-byte snapshot hash -> rkyv-archived Snapshot bytes.
const SNAPSHOTS_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new(TABLE_NAME_SNAPSHOTS);

/// Metadata table: well-known string key -> string value.
const METADATA_TABLE: TableDefinition<&str, &str> = TableDefinition::new(TABLE_NAME_METADATA);

// ---------------------------------------------------------------------------
// Table accessor functions
// ---------------------------------------------------------------------------

/// Returns the `FILE_STATE_TABLE` definition.
///
/// Used by `StateReadSession::load_file_states` and `StateDb::commit_changes`.
#[must_use]
pub const fn file_state_table() -> TableDefinition<'static, &'static str, &'static [u8]> {
    FILE_STATE_TABLE
}

/// Returns the `URL_STATE_TABLE` definition.
#[must_use]
pub const fn url_state_table() -> TableDefinition<'static, &'static str, &'static [u8]> {
    URL_STATE_TABLE
}

/// Returns the `ANALYSIS_OUTPUTS_TABLE` definition.
#[must_use]
pub const fn analysis_outputs_table() -> TableDefinition<'static, &'static [u8], &'static [u8]> {
    ANALYSIS_OUTPUTS_TABLE
}

/// Returns the `TRANSFORM_OUTPUTS_TABLE` definition.
#[must_use]
pub const fn transform_outputs_table() -> TableDefinition<'static, &'static [u8], &'static [u8]> {
    TRANSFORM_OUTPUTS_TABLE
}

/// Returns the `CHUNK_OUTPUTS_TABLE` definition.
#[must_use]
pub const fn chunk_outputs_table() -> TableDefinition<'static, &'static [u8], &'static [u8]> {
    CHUNK_OUTPUTS_TABLE
}

/// Returns the `SCRAPE_OUTPUTS_TABLE` definition.
#[must_use]
pub const fn scrape_outputs_table() -> TableDefinition<'static, &'static [u8], &'static [u8]> {
    SCRAPE_OUTPUTS_TABLE
}

/// Returns the `SNAPSHOTS_TABLE` definition.
#[must_use]
pub const fn snapshots_table() -> TableDefinition<'static, &'static [u8], &'static [u8]> {
    SNAPSHOTS_TABLE
}

/// Returns the `METADATA_TABLE` definition.
#[must_use]
pub const fn metadata_table() -> TableDefinition<'static, &'static str, &'static str> {
    METADATA_TABLE
}

// ---------------------------------------------------------------------------
// Database initialization
// ---------------------------------------------------------------------------

/// Create all 8 tables in a single write transaction.
///
/// Called once during `StateDb::open()` on a new database. Idempotent: redb's
/// `open_table` on a `WriteTransaction` creates the table if absent, succeeds
/// silently if present.
///
/// # Errors
///
/// - [`StateError::WriteTransactionFailed`] if `db.begin_write()` fails.
/// - [`StateError::TableOpenFailed`] if any table creation fails.
/// - [`StateError::CommitFailed`] if the write transaction commit fails.
pub fn initialize_tables(db: &Database) -> Result<(), StateError> {
    let write_tx = db
        .begin_write()
        .map_err(|e| StateError::WriteTransactionFailed {
            message: e.to_string(),
        })?;

    {
        let _ = write_tx
            .open_table(FILE_STATE_TABLE)
            .map_err(|e| StateError::TableOpenFailed {
                table: TABLE_NAME_FILE_STATE,
                message: e.to_string(),
            })?;
        let _ = write_tx
            .open_table(URL_STATE_TABLE)
            .map_err(|e| StateError::TableOpenFailed {
                table: TABLE_NAME_URL_STATE,
                message: e.to_string(),
            })?;
        let _ = write_tx.open_table(ANALYSIS_OUTPUTS_TABLE).map_err(|e| {
            StateError::TableOpenFailed {
                table: TABLE_NAME_ANALYSIS_OUTPUTS,
                message: e.to_string(),
            }
        })?;
        let _ = write_tx.open_table(TRANSFORM_OUTPUTS_TABLE).map_err(|e| {
            StateError::TableOpenFailed {
                table: TABLE_NAME_TRANSFORM_OUTPUTS,
                message: e.to_string(),
            }
        })?;
        let _ =
            write_tx
                .open_table(CHUNK_OUTPUTS_TABLE)
                .map_err(|e| StateError::TableOpenFailed {
                    table: TABLE_NAME_CHUNK_OUTPUTS,
                    message: e.to_string(),
                })?;
        let _ =
            write_tx
                .open_table(SCRAPE_OUTPUTS_TABLE)
                .map_err(|e| StateError::TableOpenFailed {
                    table: TABLE_NAME_SCRAPE_OUTPUTS,
                    message: e.to_string(),
                })?;
        let _ = write_tx
            .open_table(SNAPSHOTS_TABLE)
            .map_err(|e| StateError::TableOpenFailed {
                table: TABLE_NAME_SNAPSHOTS,
                message: e.to_string(),
            })?;
        let _ = write_tx
            .open_table(METADATA_TABLE)
            .map_err(|e| StateError::TableOpenFailed {
                table: TABLE_NAME_METADATA,
                message: e.to_string(),
            })?;
    }

    write_tx.commit().map_err(|e| StateError::CommitFailed {
        message: e.to_string(),
    })?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Key validation (pure Calculations)
// ---------------------------------------------------------------------------

/// Validate that a hash key is exactly 32 bytes.
///
/// # Errors
///
/// Returns [`StateError::InvalidHashKeyLength`] if `key.len() != 32`.
pub fn validate_hash_key(key: &[u8]) -> Result<(), StateError> {
    if key.len() != 32 {
        return Err(StateError::InvalidHashKeyLength { actual: key.len() });
    }
    Ok(())
}

/// Validate that a source path key is relative and normalized.
///
/// Rules: must not be empty, must not start with `/`, must not contain `..`.
///
/// # Errors
///
/// Returns [`StateError::InvalidSourcePath`] if the path violates any rule.
pub fn validate_source_path(path: &str) -> Result<(), StateError> {
    if path.is_empty() {
        return Err(StateError::InvalidSourcePath {
            reason: "source path must not be empty".to_string(),
        });
    }
    if path.as_bytes().first() == Some(&b'/') {
        return Err(StateError::InvalidSourcePath {
            reason: "source path must not start with '/' (must be relative)".to_string(),
        });
    }
    if path.split('/').any(|component| component == "..") {
        return Err(StateError::InvalidSourcePath {
            reason: "source path must not contain '..' components".to_string(),
        });
    }
    Ok(())
}

/// Validate that a URL key has a scheme (contains `"://"`).
///
/// Rules: must not be empty, must contain `"://"` to indicate a scheme.
///
/// # Errors
///
/// Returns [`StateError::InvalidUrlKey`] if the URL violates any rule.
pub fn validate_url_key(url: &str) -> Result<(), StateError> {
    if url.is_empty() {
        return Err(StateError::InvalidUrlKey {
            reason: "URL key must not be empty".to_string(),
        });
    }
    if !url.contains("://") {
        return Err(StateError::InvalidUrlKey {
            reason: "URL key must contain a scheme (e.g. \"https://\")".to_string(),
        });
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Pod read helpers
// ---------------------------------------------------------------------------

/// Read and validate a `FileStateRaw` from raw redb bytes.
///
/// # Errors
///
/// - [`StateError::PodSizeMismatch`] if `bytes.len() != 200`.
/// - [`StateError::PodCastFailed`] if field extraction fails.
pub fn read_file_state_raw(bytes: &[u8]) -> Result<FileStateRaw, StateError> {
    FileStateRaw::from_bytes(bytes)
}

/// Read and validate a `UrlStateRaw` from raw redb bytes.
///
/// # Errors
///
/// - [`StateError::PodSizeMismatch`] if `bytes.len() != 120`.
/// - [`StateError::PodCastFailed`] if field extraction fails.
pub fn read_url_state_raw(bytes: &[u8]) -> Result<UrlStateRaw, StateError> {
    UrlStateRaw::from_bytes(bytes)
}

// ---------------------------------------------------------------------------
// Snapshot serialization stub
// ---------------------------------------------------------------------------

/// Serialize a [`crate::watch::Snapshot`] into rkyv bytes.
///
/// # Errors
///
/// Returns [`StateError::SerializationFailed`] if rkyv serialization fails.
///
/// # TODO
///
/// This is a stub — implementation deferred to snapshot API bead.
pub fn serialize_snapshot(_snapshot: &crate::watch::Snapshot) -> Result<Vec<u8>, StateError> {
    todo!()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use redb::{ReadableTableMetadata, TableHandle};
    use tempfile::TempDir;

    // =======================================================================
    // Helper: open a fresh database + initialize tables
    // =======================================================================

    fn open_fresh_db() -> (TempDir, Database) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("state.redb");
        let db = Database::create(&db_path).unwrap();
        initialize_tables(&db).unwrap();
        (temp_dir, db)
    }

    // =======================================================================
    // B01: FileStateRaw is exactly 200 bytes
    // B02: UrlStateRaw is exactly 120 bytes
    // =======================================================================

    #[test]
    fn file_state_raw_size_is_200_bytes() {
        assert_eq!(size_of::<FileStateRaw>(), 200);
    }

    #[test]
    fn url_state_raw_size_is_120_bytes() {
        assert_eq!(size_of::<UrlStateRaw>(), 120);
    }

    // =======================================================================
    // B03/B04: Trait bound satisfaction (Copy, Clone, Debug, PartialEq, Eq)
    // =======================================================================

    #[test]
    fn file_state_raw_satisfies_required_traits() {
        fn assert_traits<T: Copy + Clone + std::fmt::Debug + PartialEq + Eq>() {}
        assert_traits::<FileStateRaw>();
    }

    #[test]
    fn url_state_raw_satisfies_required_traits() {
        fn assert_traits<T: Copy + Clone + std::fmt::Debug + PartialEq + Eq>() {}
        assert_traits::<UrlStateRaw>();
    }

    // =======================================================================
    // B05: FileStateRaw zeroed state is valid
    // B06: UrlStateRaw zeroed state is valid
    // =======================================================================

    #[test]
    fn file_state_raw_zeroed_is_valid() {
        let zeroed = FileStateRaw::zeroed();
        assert_eq!(zeroed.content_hash, [0u8; 32]);
        assert_eq!(zeroed.config_hash, [0u8; 32]);
        assert_eq!(zeroed.analysis_hash, [0u8; 32]);
        assert_eq!(zeroed.transform_hash, [0u8; 32]);
        assert_eq!(zeroed.chunk_hash, [0u8; 32]);
        assert_eq!(zeroed.last_processed_secs, 0);
        assert_eq!(zeroed.reserved, [0u8; 32]);

        // Round-trip through bytes preserves zeroed state
        let bytes = zeroed.to_bytes();
        let restored = FileStateRaw::from_bytes(&bytes).unwrap();
        assert_eq!(restored, zeroed);
    }

    #[test]
    fn url_state_raw_zeroed_is_valid() {
        let zeroed = UrlStateRaw::zeroed();
        assert_eq!(zeroed.content_hash, [0u8; 32]);
        assert_eq!(zeroed.url_hash, [0u8; 32]);
        assert_eq!(zeroed.last_fetched_secs, 0);
        assert_eq!(zeroed.status_code, 0);
        assert_eq!(zeroed.reserved, [0u8; 46]);

        // Round-trip through bytes preserves zeroed state
        let bytes = zeroed.to_bytes();
        let restored = UrlStateRaw::from_bytes(&bytes).unwrap();
        assert_eq!(restored, zeroed);
    }

    // =======================================================================
    // B07: FileStateRaw Pod read/write symmetry
    // B08: UrlStateRaw Pod read/write symmetry
    // =======================================================================

    #[test]
    fn file_state_raw_pod_roundtrip_returns_original() {
        let state = FileStateRaw {
            content_hash: [0xAA; 32],
            config_hash: [0xBB; 32],
            analysis_hash: [0xCC; 32],
            transform_hash: [0xDD; 32],
            chunk_hash: [0xEE; 32],
            last_processed_secs: 1_700_000_000,
            reserved: [0xFF; 32],
        };

        let bytes = state.to_bytes();
        assert_eq!(bytes.len(), 200);

        let restored = FileStateRaw::from_bytes(&bytes).unwrap();
        assert_eq!(restored, state);
    }

    #[test]
    fn url_state_raw_pod_roundtrip_returns_original() {
        let state = UrlStateRaw {
            content_hash: [0x11; 32],
            url_hash: [0x22; 32],
            last_fetched_secs: 1_700_000_001,
            status_code: 200,
            reserved: [0x33; 46],
        };

        let bytes = state.to_bytes();
        assert_eq!(bytes.len(), 120);

        let restored = UrlStateRaw::from_bytes(&bytes).unwrap();
        assert_eq!(restored, state);
    }

    // =======================================================================
    // B09: All 8 table definition names are unique
    // =======================================================================

    #[test]
    fn table_definition_names_are_all_unique() {
        use std::collections::HashSet;

        let t1 = file_state_table();
        let t2 = url_state_table();
        let t3 = analysis_outputs_table();
        let t4 = transform_outputs_table();
        let t5 = chunk_outputs_table();
        let t6 = scrape_outputs_table();
        let t7 = snapshots_table();
        let t8 = metadata_table();

        let names: HashSet<&str> = [
            t1.name(),
            t2.name(),
            t3.name(),
            t4.name(),
            t5.name(),
            t6.name(),
            t7.name(),
            t8.name(),
        ]
        .into_iter()
        .collect();

        assert_eq!(names.len(), 8, "expected exactly 8 unique table names");
    }

    // =======================================================================
    // B10: Table names match architecture spec exactly
    // =======================================================================

    #[test]
    fn table_names_match_architecture_spec_exactly() {
        assert_eq!(file_state_table().name(), "file_state");
        assert_eq!(url_state_table().name(), "url_state");
        assert_eq!(analysis_outputs_table().name(), "analysis_outputs");
        assert_eq!(transform_outputs_table().name(), "transform_outputs");
        assert_eq!(chunk_outputs_table().name(), "chunk_outputs");
        assert_eq!(scrape_outputs_table().name(), "scrape_outputs");
        assert_eq!(snapshots_table().name(), "snapshots");
        assert_eq!(metadata_table().name(), "metadata");
    }

    // =======================================================================
    // B14: New table names disjoint from legacy (except metadata)
    // =======================================================================

    #[test]
    fn new_table_names_disjoint_from_legacy_except_metadata() {
        use std::collections::HashSet;

        let new_names: HashSet<&str> = [
            "file_state",
            "analysis_outputs",
            "transform_outputs",
            "chunk_outputs",
            "url_state",
            "scrape_outputs",
            "snapshots",
            "metadata",
        ]
        .into_iter()
        .collect();

        let legacy_names: HashSet<&str> = [
            "documents",
            "scrape",
            "transforms",
            "snapshots",
            "analysis",
            "chunks",
            "metadata",
        ]
        .into_iter()
        .collect();

        let intersection: HashSet<&str> = new_names.intersection(&legacy_names).copied().collect();

        assert_eq!(
            intersection,
            HashSet::from(["metadata", "snapshots"]),
            "only 'metadata' and 'snapshots' should be shared between new and legacy table names"
        );
    }

    // =======================================================================
    // B15: metadata table definition identical to legacy
    // =======================================================================

    #[test]
    fn metadata_table_definition_identical_to_legacy() {
        assert_eq!(metadata_table().name(), "metadata");
        // Legacy DocCache also uses TableDefinition<&str, &str> with name "metadata"
        // Same name + same type = same redb table (shared during migration)
        let legacy_metadata_name = "metadata";
        assert_eq!(metadata_table().name(), legacy_metadata_name);
    }

    // =======================================================================
    // B16-B23: Accessor functions return correct definitions
    // =======================================================================

    #[test]
    fn file_state_table_returns_definition_named_file_state() {
        assert_eq!(file_state_table().name(), "file_state");
    }

    #[test]
    fn url_state_table_returns_definition_named_url_state() {
        assert_eq!(url_state_table().name(), "url_state");
    }

    #[test]
    fn analysis_outputs_table_returns_definition_named_analysis_outputs() {
        assert_eq!(analysis_outputs_table().name(), "analysis_outputs");
    }

    #[test]
    fn transform_outputs_table_returns_definition_named_transform_outputs() {
        assert_eq!(transform_outputs_table().name(), "transform_outputs");
    }

    #[test]
    fn chunk_outputs_table_returns_definition_named_chunk_outputs() {
        assert_eq!(chunk_outputs_table().name(), "chunk_outputs");
    }

    #[test]
    fn scrape_outputs_table_returns_definition_named_scrape_outputs() {
        assert_eq!(scrape_outputs_table().name(), "scrape_outputs");
    }

    #[test]
    fn snapshots_table_returns_definition_named_snapshots() {
        assert_eq!(snapshots_table().name(), "snapshots");
    }

    #[test]
    fn metadata_table_returns_definition_named_metadata() {
        assert_eq!(metadata_table().name(), "metadata");
    }

    // =======================================================================
    // B24: initialize_tables creates all 8 tables on fresh database
    // =======================================================================

    #[test]
    fn initialize_tables_creates_all_8_tables_on_fresh_db() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("fresh.redb");
        let db = Database::create(&db_path).unwrap();

        initialize_tables(&db).unwrap();

        let read_tx = db.begin_read().unwrap();
        read_tx.open_table(file_state_table()).unwrap();
        read_tx.open_table(url_state_table()).unwrap();
        read_tx.open_table(analysis_outputs_table()).unwrap();
        read_tx.open_table(transform_outputs_table()).unwrap();
        read_tx.open_table(chunk_outputs_table()).unwrap();
        read_tx.open_table(scrape_outputs_table()).unwrap();
        read_tx.open_table(snapshots_table()).unwrap();
        read_tx.open_table(metadata_table()).unwrap();
    }

    // =======================================================================
    // B25: initialize_tables is idempotent
    // =======================================================================

    #[test]
    fn initialize_tables_is_idempotent_on_second_call() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("idempotent.redb");
        let db = Database::create(&db_path).unwrap();

        initialize_tables(&db).unwrap();

        // Write data between calls
        {
            let write_tx = db.begin_write().unwrap();
            {
                let mut table = write_tx.open_table(file_state_table()).unwrap();
                let state = FileStateRaw::zeroed();
                table
                    .insert("test/key.md", state.to_bytes().as_slice())
                    .unwrap();
            }
            write_tx.commit().unwrap();
        }

        // Second call — succeeds without destroying data
        initialize_tables(&db).unwrap();

        // Verify data still present
        let read_tx = db.begin_read().unwrap();
        let table = read_tx.open_table(file_state_table()).unwrap();
        let guard = table.get("test/key.md").unwrap();
        assert!(
            guard.is_some(),
            "data should persist across idempotent init"
        );
    }

    // =======================================================================
    // B28: All 8 tables survive database reopen
    // =======================================================================

    #[test]
    fn all_8_tables_survive_database_reopen() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("reopen.redb");

        // Open, init, close
        {
            let db = Database::create(&db_path).unwrap();
            initialize_tables(&db).unwrap();
        }

        // Reopen on same path
        let db = Database::create(&db_path).unwrap();
        let read_tx = db.begin_read().unwrap();

        // Pod state tables (&str keys)
        let pod_table_defs = [file_state_table(), url_state_table()];
        for def in pod_table_defs {
            let table = read_tx.open_table(def).unwrap();
            assert_eq!(
                table.len().unwrap(),
                0,
                "table '{}' should be empty",
                def.name()
            );
        }

        // rkyv output tables (&[u8] keys)
        let rkyv_table_defs = [
            analysis_outputs_table(),
            transform_outputs_table(),
            chunk_outputs_table(),
            scrape_outputs_table(),
            snapshots_table(),
        ];
        for def in rkyv_table_defs {
            let table = read_tx.open_table(def).unwrap();
            assert_eq!(
                table.len().unwrap(),
                0,
                "table '{}' should be empty",
                def.name()
            );
        }

        // Metadata table (&str keys, &str values)
        let meta_table = read_tx.open_table(metadata_table()).unwrap();
        assert_eq!(
            meta_table.len().unwrap(),
            0,
            "metadata table should be empty"
        );
    }

    // =======================================================================
    // B29: Written data survives across open/close/reopen
    // =======================================================================

    #[test]
    fn written_data_survives_across_reopen_cycle() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("persist.redb");
        let key = "concept/general/test.md";

        let original = FileStateRaw {
            content_hash: [0xAB; 32],
            config_hash: [0xCD; 32],
            analysis_hash: [0xEF; 32],
            transform_hash: [0x01; 32],
            chunk_hash: [0x23; 32],
            last_processed_secs: 1_700_000_000,
            reserved: [0x00; 32],
        };

        // Write + close
        {
            let db = Database::create(&db_path).unwrap();
            initialize_tables(&db).unwrap();
            let write_tx = db.begin_write().unwrap();
            {
                let mut table = write_tx.open_table(file_state_table()).unwrap();
                table.insert(key, original.to_bytes().as_slice()).unwrap();
            }
            write_tx.commit().unwrap();
        }

        // Reopen and read
        let db = Database::create(&db_path).unwrap();
        let read_tx = db.begin_read().unwrap();
        let table = read_tx.open_table(file_state_table()).unwrap();
        let guard = table.get(key).unwrap().unwrap();
        let bytes = guard.value();

        assert_eq!(bytes.len(), 200);
        let restored = FileStateRaw::from_bytes(bytes).unwrap();
        assert_eq!(restored, original);
    }

    // =======================================================================
    // B30: Data survives 10 sequential open/write/close cycles (E2E)
    // =======================================================================

    #[test]
    fn data_survives_ten_sequential_open_write_close_cycles() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("ten_cycles.redb");

        for cycle in 0..10u8 {
            let db = Database::create(&db_path).unwrap();
            initialize_tables(&db).unwrap();

            let key = format!("cycle_{cycle}.md");
            let state = FileStateRaw {
                content_hash: [cycle; 32],
                config_hash: [cycle.saturating_add(1); 32],
                analysis_hash: [cycle.saturating_add(2); 32],
                transform_hash: [cycle.saturating_add(3); 32],
                chunk_hash: [cycle.saturating_add(4); 32],
                last_processed_secs: u64::from(cycle),
                reserved: [0u8; 32],
            };

            let write_tx = db.begin_write().unwrap();
            {
                let mut table = write_tx.open_table(file_state_table()).unwrap();
                table
                    .insert(key.as_str(), state.to_bytes().as_slice())
                    .unwrap();
            }
            write_tx.commit().unwrap();
        }

        // Final open: verify all 10 entries
        let db = Database::create(&db_path).unwrap();
        let read_tx = db.begin_read().unwrap();
        let table = read_tx.open_table(file_state_table()).unwrap();

        for cycle in 0..10u8 {
            let key = format!("cycle_{cycle}.md");
            let guard = table.get(key.as_str()).unwrap().unwrap();
            let restored = FileStateRaw::from_bytes(guard.value()).unwrap();
            assert_eq!(restored.content_hash, [cycle; 32], "cycle {cycle}");
        }
    }

    // =======================================================================
    // B31: New state tables coexist with legacy DocCache tables
    // =======================================================================

    #[test]
    fn new_state_tables_coexist_with_legacy_doc_cache_tables() {
        use crate::cache::{CacheConfig, DocCache};

        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("coexist.redb");

        // Initialize legacy tables via DocCache
        {
            let config = CacheConfig::new(&db_path);
            let cache = DocCache::open(config).unwrap();
            cache.put_document(b"legacy_key", &"legacy_value").unwrap();
        }

        // Initialize new state tables on same database
        let db = Database::create(&db_path).unwrap();
        initialize_tables(&db).unwrap();

        // Verify legacy tables still accessible
        let read_tx = db.begin_read().unwrap();
        let legacy_doc_def: TableDefinition<&[u8], &[u8]> = TableDefinition::new("documents");
        let legacy_table = read_tx.open_table(legacy_doc_def).unwrap();
        let legacy_key: &[u8] = b"legacy_key";
        let legacy_guard = legacy_table.get(legacy_key).unwrap();
        assert!(legacy_guard.is_some(), "legacy data should be preserved");

        // New state tables accessible
        read_tx.open_table(file_state_table()).unwrap();
        read_tx.open_table(metadata_table()).unwrap();
    }

    // =======================================================================
    // B32: Hash key wrong length -> InvalidHashKeyLength
    // =======================================================================

    #[test]
    fn hash_key_wrong_length_returns_invalid_hash_key_length() {
        // Too short
        let result = validate_hash_key(&[0u8; 16]);
        assert!(matches!(
            result,
            Err(StateError::InvalidHashKeyLength { actual: 16 })
        ));

        // Too long
        let result = validate_hash_key(&[0u8; 33]);
        assert!(matches!(
            result,
            Err(StateError::InvalidHashKeyLength { actual: 33 })
        ));

        // Empty
        let result = validate_hash_key(&[]);
        assert!(matches!(
            result,
            Err(StateError::InvalidHashKeyLength { actual: 0 })
        ));

        // Valid
        assert!(validate_hash_key(&[0u8; 32]).is_ok());
    }

    // =======================================================================
    // B33: Source path with leading / -> InvalidSourcePath
    // =======================================================================

    #[test]
    fn source_path_with_leading_slash_returns_invalid_source_path() {
        let result = validate_source_path("/absolute/path.md");
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("must not start with '/'"),
            "error should mention leading slash: {err_msg}"
        );
    }

    // =======================================================================
    // B34: Source path with .. -> InvalidSourcePath
    // =======================================================================

    #[test]
    fn source_path_with_dot_dot_returns_invalid_source_path() {
        let result = validate_source_path("foo/../bar.md");
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("'..'"),
            "error should mention '..': {err_msg}"
        );
    }

    // =======================================================================
    // B35: URL key without scheme -> InvalidUrlKey
    // =======================================================================

    #[test]
    fn url_key_without_scheme_returns_invalid_url_key() {
        let result = validate_url_key("example.com/page");
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("scheme"),
            "error should mention scheme: {err_msg}"
        );
    }

    // =======================================================================
    // B36: file_state wrong value size -> PodSizeMismatch
    // =======================================================================

    #[test]
    fn file_state_wrong_value_size_returns_pod_size_mismatch() {
        let result = read_file_state_raw(&[0u8; 199]);
        assert!(matches!(
            result,
            Err(StateError::PodSizeMismatch {
                table: "file_state",
                expected: 200,
                actual: 199,
            })
        ));

        let result = read_file_state_raw(&[0u8; 201]);
        assert!(matches!(
            result,
            Err(StateError::PodSizeMismatch {
                table: "file_state",
                expected: 200,
                actual: 201,
            })
        ));
    }

    // =======================================================================
    // B37: url_state wrong value size -> PodSizeMismatch
    // =======================================================================

    #[test]
    fn url_state_wrong_value_size_returns_pod_size_mismatch() {
        let result = read_url_state_raw(&[0u8; 119]);
        assert!(matches!(
            result,
            Err(StateError::PodSizeMismatch {
                table: "url_state",
                expected: 120,
                actual: 119,
            })
        ));

        let result = read_url_state_raw(&[0u8; 121]);
        assert!(matches!(
            result,
            Err(StateError::PodSizeMismatch {
                table: "url_state",
                expected: 120,
                actual: 121,
            })
        ));
    }

    // =======================================================================
    // B39: Key not found in table
    // =======================================================================

    #[test]
    fn missing_key_returns_none_from_redb() {
        let (_temp_dir, db) = open_fresh_db();

        let read_tx = db.begin_read().unwrap();
        let table = read_tx.open_table(analysis_outputs_table()).unwrap();

        let missing_key: [u8; 32] = [1u8; 32];
        let result = table.get(missing_key.as_slice()).unwrap();
        assert!(result.is_none(), "missing key should return None");
    }

    // =======================================================================
    // Additional key validation edge cases
    // =======================================================================

    #[test]
    fn source_path_empty_returns_invalid_source_path() {
        let result = validate_source_path("");
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("empty"),
            "error should mention empty: {err_msg}"
        );
    }

    #[test]
    fn source_path_valid_relative() {
        assert!(validate_source_path("concept/general/test.md").is_ok());
    }

    #[test]
    fn url_key_valid_with_scheme() {
        assert!(validate_url_key("https://docs.example.com/api").is_ok());
    }

    #[test]
    fn url_key_empty_returns_invalid_url_key() {
        let result = validate_url_key("");
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("empty"),
            "error should mention empty: {err_msg}"
        );
    }

    // =======================================================================
    // FileStateRaw byte layout consistency (matches Proptest 3)
    // =======================================================================

    #[test]
    fn file_state_raw_byte_layout_matches_offsets() {
        let state = FileStateRaw {
            content_hash: [0x0A; 32],
            config_hash: [0x0B; 32],
            analysis_hash: [0x0C; 32],
            transform_hash: [0x0D; 32],
            chunk_hash: [0x0E; 32],
            last_processed_secs: 0x1122_3344_5566_7788,
            reserved: [0xFF; 32],
        };

        let bytes = state.to_bytes();

        assert_eq!(&bytes[0..32], state.content_hash.as_slice());
        assert_eq!(&bytes[32..64], state.config_hash.as_slice());
        assert_eq!(&bytes[64..96], state.analysis_hash.as_slice());
        assert_eq!(&bytes[96..128], state.transform_hash.as_slice());
        assert_eq!(&bytes[128..160], state.chunk_hash.as_slice());
        assert_eq!(&bytes[160..168], state.last_processed_secs.to_le_bytes());
        assert_eq!(&bytes[168..200], state.reserved.as_slice());
    }

    // =======================================================================
    // UrlStateRaw byte layout consistency (matches Proptest 4)
    // =======================================================================

    #[test]
    fn url_state_raw_byte_layout_matches_offsets() {
        let state = UrlStateRaw {
            content_hash: [0xAA; 32],
            url_hash: [0xBB; 32],
            last_fetched_secs: 0x8877_6655_4433_2211,
            status_code: 404,
            reserved: [0xCC; 46],
        };

        let bytes = state.to_bytes();

        assert_eq!(&bytes[0..32], state.content_hash.as_slice());
        assert_eq!(&bytes[32..64], state.url_hash.as_slice());
        assert_eq!(&bytes[64..72], state.last_fetched_secs.to_le_bytes());
        assert_eq!(&bytes[72..74], state.status_code.to_le_bytes());
        assert_eq!(&bytes[74..120], state.reserved.as_slice());
    }

    // =======================================================================
    // Error variant display coverage
    // =======================================================================

    #[test]
    fn state_error_variants_display_correctly() {
        let errors: Vec<StateError> = vec![
            StateError::OpenFailed {
                path: std::path::PathBuf::from("/tmp/test.redb"),
                detail: "permission denied".to_string(),
            },
            StateError::ReadTransactionFailed {
                message: "mvcc conflict".to_string(),
            },
            StateError::WriteTransactionFailed {
                message: "already locked".to_string(),
            },
            StateError::PodSizeMismatch {
                table: "file_state",
                expected: 200,
                actual: 199,
            },
            StateError::PodCastFailed {
                type_name: "FileStateRaw",
                message: "alignment".to_string(),
            },
            StateError::InvalidArchive {
                type_name: "Analysis",
                message: "bad bytes".to_string(),
            },
            StateError::DeserializationFailed {
                type_name: "Analysis",
                message: "type mismatch".to_string(),
            },
            StateError::SerializationFailed {
                type_name: "Analysis",
                message: "oom".to_string(),
            },
            StateError::TableOpenFailed {
                table: "file_state",
                message: "corrupt".to_string(),
            },
            StateError::KeyNotFound {
                table: "analysis_outputs",
            },
            StateError::StorageError {
                operation: "get",
                message: "io error".to_string(),
            },
            StateError::CommitFailed {
                message: "disk full".to_string(),
            },
            StateError::InvalidHashKeyLength { actual: 16 },
            StateError::InvalidSourcePath {
                reason: "leading /".to_string(),
            },
            StateError::InvalidUrlKey {
                reason: "no scheme".to_string(),
            },
        ];

        for err in &errors {
            let display = format!("{err}");
            assert!(!display.is_empty(), "error display should not be empty");
        }
    }

    // =======================================================================
    // Proptest: FileStateRaw round-trip
    // =======================================================================

    #[test]
    fn proptest_file_state_raw_roundtrip() {
        use proptest::prelude::*;

        proptest!(|(
            content_hash in proptest::array::uniform32(0u8..=255u8),
            config_hash in proptest::array::uniform32(0u8..=255u8),
            analysis_hash in proptest::array::uniform32(0u8..=255u8),
            transform_hash in proptest::array::uniform32(0u8..=255u8),
            chunk_hash in proptest::array::uniform32(0u8..=255u8),
            last_processed_secs: u64,
            reserved in proptest::array::uniform32(0u8..=255u8),
        )| {
            let state = FileStateRaw {
                content_hash,
                config_hash,
                analysis_hash,
                transform_hash,
                chunk_hash,
                last_processed_secs,
                reserved: reserved,
            };

            let bytes = state.to_bytes();
            prop_assert_eq!(bytes.len(), 200);

            let restored = FileStateRaw::from_bytes(&bytes)
                .expect("round-trip should succeed for any FileStateRaw");
            prop_assert_eq!(restored, state);
        });
    }

    // =======================================================================
    // Proptest: UrlStateRaw round-trip
    // =======================================================================

    #[test]
    fn proptest_url_state_raw_roundtrip() {
        use proptest::prelude::*;

        proptest!(|(
            content_hash in proptest::array::uniform32(0u8..=255u8),
            url_hash in proptest::array::uniform32(0u8..=255u8),
            last_fetched_secs: u64,
            status_code: u16,
            reserved in any::<[u8; 46]>(),
        )| {
            let state = UrlStateRaw {
                content_hash,
                url_hash,
                last_fetched_secs,
                status_code,
                reserved: reserved,
            };

            let bytes = state.to_bytes();
            prop_assert_eq!(bytes.len(), 120);

            let restored = UrlStateRaw::from_bytes(&bytes)
                .expect("round-trip should succeed for any UrlStateRaw");
            prop_assert_eq!(restored, state);
        });
    }

    // =======================================================================
    // Proptest: FileStateRaw byte layout consistency
    // =======================================================================

    #[test]
    fn proptest_file_state_raw_byte_layout() {
        use proptest::prelude::*;

        proptest!(|(
            content_hash in proptest::array::uniform32(0u8..=255u8),
            config_hash in proptest::array::uniform32(0u8..=255u8),
            analysis_hash in proptest::array::uniform32(0u8..=255u8),
            transform_hash in proptest::array::uniform32(0u8..=255u8),
            chunk_hash in proptest::array::uniform32(0u8..=255u8),
            last_processed_secs: u64,
            reserved in proptest::array::uniform32(0u8..=255u8),
        )| {
            let state = FileStateRaw {
                content_hash,
                config_hash,
                analysis_hash,
                transform_hash,
                chunk_hash,
                last_processed_secs,
                reserved: reserved,
            };

            let bytes = state.to_bytes();

            prop_assert_eq!(&bytes[0..32], content_hash.as_slice());
            prop_assert_eq!(&bytes[32..64], config_hash.as_slice());
            prop_assert_eq!(&bytes[64..96], analysis_hash.as_slice());
            prop_assert_eq!(&bytes[96..128], transform_hash.as_slice());
            prop_assert_eq!(&bytes[128..160], chunk_hash.as_slice());
            prop_assert_eq!(&bytes[160..168], last_processed_secs.to_le_bytes());
            prop_assert_eq!(&bytes[168..200], reserved.as_slice());
        });
    }

    // =======================================================================
    // Proptest: UrlStateRaw byte layout consistency
    // =======================================================================

    #[test]
    fn proptest_url_state_raw_byte_layout() {
        use proptest::prelude::*;

        proptest!(|(
            content_hash in proptest::array::uniform32(0u8..=255u8),
            url_hash in proptest::array::uniform32(0u8..=255u8),
            last_fetched_secs: u64,
            status_code: u16,
            reserved in any::<[u8; 46]>(),
        )| {
            let state = UrlStateRaw {
                content_hash,
                url_hash,
                last_fetched_secs,
                status_code,
                reserved: reserved,
            };

            let bytes = state.to_bytes();

            prop_assert_eq!(&bytes[0..32], content_hash.as_slice());
            prop_assert_eq!(&bytes[32..64], url_hash.as_slice());
            prop_assert_eq!(&bytes[64..72], last_fetched_secs.to_le_bytes());
            prop_assert_eq!(&bytes[72..74], status_code.to_le_bytes());
            prop_assert_eq!(&bytes[74..120], reserved.as_slice());
        });
    }

    // =======================================================================
    // Integration: write/read FileStateRaw through redb round-trip
    // =======================================================================

    #[test]
    fn file_state_write_read_roundtrip_through_redb() {
        let (_temp_dir, db) = open_fresh_db();

        let original = FileStateRaw {
            content_hash: [0xFE; 32],
            config_hash: [0xDC; 32],
            analysis_hash: [0xBA; 32],
            transform_hash: [0x98; 32],
            chunk_hash: [0x76; 32],
            last_processed_secs: 999,
            reserved: [0x00; 32],
        };

        let key = "concept/test.md";

        // Write
        let write_tx = db.begin_write().unwrap();
        {
            let mut table = write_tx.open_table(file_state_table()).unwrap();
            table.insert(key, original.to_bytes().as_slice()).unwrap();
        }
        write_tx.commit().unwrap();

        // Read
        let read_tx = db.begin_read().unwrap();
        let table = read_tx.open_table(file_state_table()).unwrap();
        let guard = table.get(key).unwrap().unwrap();
        let bytes = guard.value();

        assert_eq!(bytes.len(), 200);
        let restored = FileStateRaw::from_bytes(bytes).unwrap();
        assert_eq!(restored, original);
    }

    // =======================================================================
    // Integration: write/read UrlStateRaw through redb round-trip
    // =======================================================================

    #[test]
    fn url_state_write_read_roundtrip_through_redb() {
        let (_temp_dir, db) = open_fresh_db();

        let original = UrlStateRaw {
            content_hash: [0x11; 32],
            url_hash: [0x22; 32],
            last_fetched_secs: 12345,
            status_code: 200,
            reserved: [0x00; 46],
        };

        let key = "https://docs.example.com/api";

        // Write
        let write_tx = db.begin_write().unwrap();
        {
            let mut table = write_tx.open_table(url_state_table()).unwrap();
            table.insert(key, original.to_bytes().as_slice()).unwrap();
        }
        write_tx.commit().unwrap();

        // Read
        let read_tx = db.begin_read().unwrap();
        let table = read_tx.open_table(url_state_table()).unwrap();
        let guard = table.get(key).unwrap().unwrap();
        let bytes = guard.value();

        assert_eq!(bytes.len(), 120);
        let restored = UrlStateRaw::from_bytes(bytes).unwrap();
        assert_eq!(restored, original);
    }

    // =======================================================================
    // Integration: metadata table string read/write
    // =======================================================================

    #[test]
    fn metadata_table_string_read_write_roundtrip() {
        let (_temp_dir, db) = open_fresh_db();

        // Write
        let write_tx = db.begin_write().unwrap();
        {
            let mut table = write_tx.open_table(metadata_table()).unwrap();
            table.insert("schema_version", "1").unwrap();
            table.insert("created_by", "ctd").unwrap();
        }
        write_tx.commit().unwrap();

        // Read
        let read_tx = db.begin_read().unwrap();
        let table = read_tx.open_table(metadata_table()).unwrap();
        assert_eq!(table.get("schema_version").unwrap().unwrap().value(), "1");
        assert_eq!(table.get("created_by").unwrap().unwrap().value(), "ctd");
        assert!(table.get("nonexistent").unwrap().is_none());
    }

    // =======================================================================
    // Integration: hash-keyed output table write/read
    // =======================================================================

    #[test]
    fn hash_keyed_output_table_write_read_roundtrip() {
        let (_temp_dir, db) = open_fresh_db();

        let hash_key: [u8; 32] = [0xAB; 32];
        let value: &[u8] = b"test analysis output bytes";

        // Write
        let write_tx = db.begin_write().unwrap();
        {
            let mut table = write_tx.open_table(analysis_outputs_table()).unwrap();
            table.insert(hash_key.as_slice(), value).unwrap();
        }
        write_tx.commit().unwrap();

        // Read
        let read_tx = db.begin_read().unwrap();
        let table = read_tx.open_table(analysis_outputs_table()).unwrap();
        let guard = table.get(hash_key.as_slice()).unwrap().unwrap();
        assert_eq!(guard.value(), value);
    }

    // =======================================================================
    // G14 (B93): validate_source_path accepts path with three dots
    // =======================================================================

    #[test]
    fn validate_source_path_accepts_three_dots_in_path() {
        let result = validate_source_path("foo/.../bar");
        assert!(
            result.is_ok(),
            "three dots (not '..') should be accepted: {result:?}"
        );
    }

    // =======================================================================
    // G14 (B94): validate_source_path accepts single dot segment
    // =======================================================================

    #[test]
    fn validate_source_path_accepts_single_dot_segment() {
        let result = validate_source_path("./foo");
        assert!(
            result.is_ok(),
            "single dot segment './foo' should be accepted: {result:?}"
        );
    }

    // =======================================================================
    // G14 (B95): validate_source_path accepts dot-dot prefix in filename
    // =======================================================================

    #[test]
    fn validate_source_path_accepts_dot_dot_prefix_in_filename() {
        let result = validate_source_path("..hidden");
        assert!(
            result.is_ok(),
            "dot-dot prefix in filename '..hidden' should be accepted: {result:?}"
        );
    }

    // =======================================================================
    // G14 (B96): validate_source_path accepts unicode path
    // =======================================================================

    #[test]
    fn validate_source_path_accepts_unicode_path() {
        let result = validate_source_path("概念/一般/test.md");
        assert!(
            result.is_ok(),
            "unicode path should be accepted: {result:?}"
        );
    }

    // =======================================================================
    // G14 (B97): validate_source_path accepts very long path
    // =======================================================================

    #[test]
    fn validate_source_path_accepts_very_long_path() {
        let long_path: String = "a".repeat(4096);
        let result = validate_source_path(&long_path);
        assert!(
            result.is_ok(),
            "4096-char path should be accepted: {result:?}"
        );
    }

    // =======================================================================
    // G21 (B72): StateError::WriteTransactionFailed field-level assertion
    // =======================================================================

    #[test]
    fn state_error_write_transaction_failed_exact_fields() {
        let err = StateError::WriteTransactionFailed {
            message: "already locked".to_string(),
        };
        assert!(
            matches!(
                &err,
                StateError::WriteTransactionFailed { message }
                if message == "already locked"
            ),
            "WriteTransactionFailed must carry exact message"
        );
        let display = format!("{err}");
        assert!(
            display.contains("already locked"),
            "Display must contain message: {display}"
        );
        assert!(
            display.contains("write transaction"),
            "Display must mention write transaction: {display}"
        );
    }

    // =======================================================================
    // G21 (B73): StateError::TableOpenFailed field-level assertion
    // =======================================================================

    #[test]
    fn state_error_table_open_failed_exact_fields() {
        let err = StateError::TableOpenFailed {
            table: "file_state",
            message: "corrupt".to_string(),
        };
        assert!(
            matches!(
                &err,
                StateError::TableOpenFailed { table: "file_state", message }
                if message == "corrupt"
            ),
            "TableOpenFailed must carry exact table and message"
        );
        let display = format!("{err}");
        assert!(
            display.contains("file_state"),
            "Display must contain table name: {display}"
        );
        assert!(
            display.contains("corrupt"),
            "Display must contain message: {display}"
        );
    }

    // =======================================================================
    // G21 (B74): StateError::CommitFailed field-level assertion
    // =======================================================================

    #[test]
    fn state_error_commit_failed_exact_fields() {
        let err = StateError::CommitFailed {
            message: "disk full".to_string(),
        };
        assert!(
            matches!(
                &err,
                StateError::CommitFailed { message }
                if message == "disk full"
            ),
            "CommitFailed must carry exact message"
        );
        let display = format!("{err}");
        assert!(
            display.contains("disk full"),
            "Display must contain message: {display}"
        );
        assert!(
            display.contains("commit"),
            "Display must mention commit: {display}"
        );
    }
}

// ---------------------------------------------------------------------------
// Commit pipeline submodule
// ---------------------------------------------------------------------------

pub mod commit;
pub use commit::*;
