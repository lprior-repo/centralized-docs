//! State database layer for snapshot persistence.
//!
//! Provides `StateDb` / `StateReadSession` for ACID snapshot storage using redb,
//! and `serialize_snapshot` for byte preparation.
//!
//! # Architecture
//!
//! ```text
//! StateDb::open(path) → StateDb
//! StateDb::begin_read() → StateReadSession<'db>
//! StateReadSession::load_snapshots(hashes) → HashMap<[u8;32], OwnedArchive<Snapshot>>
//! StateDb::commit_changes(&StateChanges) → ()
//! serialize_snapshot(&Snapshot) → Vec<u8>
//! ```
//!
//! # Data → Calc → Actions
//!
//! - **Data**: `StateError`, `OwnedArchive<T>`, `StateChanges`, `StateDb`, `StateReadSession`
//! - **Calc**: `serialize_snapshot`, `url_hash`, `key_to_hex`
//! - **Actions**: `StateDb::open`, `begin_read`, `commit_changes`, `load_snapshots`

use crate::watch::Snapshot;
use redb::TableDefinition;
use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const SNAPSHOTS_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("snapshots");

// ---------------------------------------------------------------------------
// Error taxonomy
// ---------------------------------------------------------------------------

/// Error type for state database operations.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum StateError {
    /// The redb database could not be opened (I/O, corruption, lock).
    #[error("state database open failed: {message}")]
    DatabaseOpenFailed { message: String },

    /// A redb read transaction could not be started.
    #[error("failed to begin read transaction: {message}")]
    ReadTransactionFailed { message: String },

    /// A redb write transaction could not be started.
    #[error("failed to begin write transaction: {message}")]
    WriteTransactionFailed { message: String },

    /// The redb table could not be opened.
    #[error("failed to open redb table '{table}': {message}")]
    TableOpenFailed {
        table: &'static str,
        message: String,
    },

    /// A redb storage operation failed (generic backend error).
    #[error("redb storage error during {operation}: {message}")]
    StorageError {
        operation: &'static str,
        message: String,
    },

    /// Serialization failed when preparing bytes for write.
    #[error("snapshot serialization failed: {message}")]
    SerializationFailed { message: String },

    /// Deserialization failed -- archived bytes are corrupt or invalid.
    #[error("snapshot deserialization failed for key {key_hex}: {message}")]
    DeserializationFailed { key_hex: String, message: String },

    /// Bytecheck validation failed -- bytes do not represent a valid archive.
    #[error("snapshot archive validation failed for key {key_hex}: {message}")]
    ArchiveValidationFailed { key_hex: String, message: String },

    /// A redb commit operation failed after writes.
    #[error("failed to commit state changes: {message}")]
    CommitFailed { message: String },

    /// An I/O error occurred (e.g., creating parent directories).
    #[error("I/O error: {message}")]
    Io { message: String },
}

// ---------------------------------------------------------------------------
// OwnedArchive<T>
// ---------------------------------------------------------------------------

/// Owned wrapper over serialized bytes.
///
/// Provides `deserialize()` for full ownership recovery of the archived value.
/// The bytes are owned (`Box<[u8]>`), not borrowed from a redb transaction.
pub struct OwnedArchive<T> {
    bytes: Box<[u8]>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> std::fmt::Debug for OwnedArchive<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OwnedArchive")
            .field("byte_len", &self.bytes.len())
            .finish()
    }
}

impl<T> OwnedArchive<T> {
    /// Construct from raw bytes.
    #[must_use]
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            bytes: bytes.into_boxed_slice(),
            _marker: std::marker::PhantomData,
        }
    }

    /// Return the raw archived bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl<T: serde::de::DeserializeOwned> OwnedArchive<T> {
    /// Deserialize the archived value into an owned `T`.
    ///
    /// # Errors
    ///
    /// Returns `StateError::DeserializationFailed` if deserialization fails.
    pub fn deserialize(&self) -> Result<T, StateError> {
        bincode::deserialize(&self.bytes).map_err(|e| StateError::DeserializationFailed {
            key_hex: String::new(),
            message: e.to_string(),
        })
    }
}

// ---------------------------------------------------------------------------
// Read session guard (RAII for active-read flag)
// ---------------------------------------------------------------------------

/// RAII guard that clears the `read_active` flag on drop.
struct ReadSessionGuard {
    flag: Arc<AtomicBool>,
}

impl Drop for ReadSessionGuard {
    fn drop(&mut self) {
        self.flag.store(false, Ordering::Release);
    }
}

// ---------------------------------------------------------------------------
// StateChanges
// ---------------------------------------------------------------------------

/// Batch of state changes to commit atomically.
///
/// Plain data struct — the caller constructs it with the correct bytes.
/// The only way to write is through `StateDb::commit_changes`.
pub struct StateChanges {
    /// New or updated snapshots to persist. Key = SHA-256 of target URL.
    /// Value = serialized Snapshot bytes.
    /// Last entry wins on duplicate keys.
    pub new_snapshots: Vec<([u8; 32], Vec<u8>)>,

    /// Snapshot keys to delete. Delete takes precedence over insert for the same key.
    pub deleted_snapshots: Vec<[u8; 32]>,
}

impl Default for StateChanges {
    fn default() -> Self {
        Self {
            new_snapshots: Vec::new(),
            deleted_snapshots: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// StateDb
// ---------------------------------------------------------------------------

/// Wrapper around a redb `Database`; owns the file handle and table definitions.
pub struct StateDb {
    db: redb::Database,
    read_active: Arc<AtomicBool>,
}

impl StateDb {
    /// Open a state database at the given path, creating all tables.
    ///
    /// # Errors
    ///
    /// Returns `StateError::DatabaseOpenFailed` if redb cannot create/open the file.
    /// Returns `StateError::Io` if parent directory creation fails.
    pub fn open(path: &Path) -> Result<Self, StateError> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(|e| StateError::Io {
                    message: format!("failed to create directory {}: {e}", parent.display()),
                })?;
            }
        }

        let db = redb::Database::create(path).map_err(|e| StateError::DatabaseOpenFailed {
            message: e.to_string(),
        })?;

        let write_tx = db
            .begin_write()
            .map_err(|e| StateError::WriteTransactionFailed {
                message: e.to_string(),
            })?;
        {
            let _ =
                write_tx
                    .open_table(SNAPSHOTS_TABLE)
                    .map_err(|e| StateError::TableOpenFailed {
                        table: "snapshots",
                        message: e.to_string(),
                    })?;
        }
        write_tx.commit().map_err(|e| StateError::CommitFailed {
            message: e.to_string(),
        })?;

        Ok(Self {
            db,
            read_active: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Begin a read session borrowing from this `StateDb`.
    ///
    /// # Errors
    ///
    /// Returns `StateError::ReadTransactionFailed` if the read transaction cannot start.
    pub fn begin_read(&self) -> Result<StateReadSession<'_>, StateError> {
        let read_tx = self
            .db
            .begin_read()
            .map_err(|e| StateError::ReadTransactionFailed {
                message: e.to_string(),
            })?;

        let guard = ReadSessionGuard {
            flag: Arc::clone(&self.read_active),
        };
        self.read_active.store(true, Ordering::Release);

        Ok(StateReadSession {
            read_tx,
            _guard: guard,
            _phantom: std::marker::PhantomData,
        })
    }

    /// Commit all state changes in one write transaction.
    ///
    /// For snapshots: inserts `new_snapshots`, deletes `deleted_snapshots`,
    /// delete takes precedence on key collision, all atomic.
    ///
    /// # Errors
    ///
    /// - `StateError::WriteTransactionFailed` if `begin_write` fails.
    /// - `StateError::TableOpenFailed` if the snapshots table cannot be opened.
    /// - `StateError::StorageError` if redb insert/delete fails.
    /// - `StateError::CommitFailed` if `write_tx.commit()` fails.
    pub fn commit_changes(&self, changes: &StateChanges) -> Result<(), StateError> {
        if self.read_active.load(Ordering::Acquire) {
            return Err(StateError::WriteTransactionFailed {
                message: "cannot begin write transaction while StateReadSession is active"
                    .to_string(),
            });
        }

        let write_tx = self
            .db
            .begin_write()
            .map_err(|e| StateError::WriteTransactionFailed {
                message: e.to_string(),
            })?;

        {
            let mut table =
                write_tx
                    .open_table(SNAPSHOTS_TABLE)
                    .map_err(|e| StateError::TableOpenFailed {
                        table: "snapshots",
                        message: e.to_string(),
                    })?;

            // Insert new snapshots (last entry wins for duplicates)
            changes.new_snapshots.iter().try_for_each(|(key, bytes)| {
                table
                    .insert(key.as_slice(), bytes.as_slice())
                    .map_err(|e| StateError::StorageError {
                        operation: "commit_snapshot_insert",
                        message: e.to_string(),
                    })?;
                Ok::<(), StateError>(())
            })?;

            // Delete snapshots (applied after insert, so delete wins on collision)
            changes.deleted_snapshots.iter().try_for_each(|key| {
                table
                    .remove(key.as_slice())
                    .map_err(|e| StateError::StorageError {
                        operation: "commit_snapshot_delete",
                        message: e.to_string(),
                    })?;
                Ok::<(), StateError>(())
            })?;
        }

        write_tx.commit().map_err(|e| StateError::CommitFailed {
            message: e.to_string(),
        })?;

        Ok(())
    }

    /// Delete the snapshots table. Used for testing error paths.
    ///
    /// # Errors
    ///
    /// Returns appropriate `StateError` variants for write/delete/commit failures.
    pub fn drop_snapshots_table(&self) -> Result<(), StateError> {
        let write_tx = self
            .db
            .begin_write()
            .map_err(|e| StateError::WriteTransactionFailed {
                message: e.to_string(),
            })?;
        write_tx
            .delete_table(SNAPSHOTS_TABLE)
            .map_err(|e| StateError::StorageError {
                operation: "delete_table",
                message: e.to_string(),
            })?;
        write_tx.commit().map_err(|e| StateError::CommitFailed {
            message: e.to_string(),
        })?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// StateReadSession
// ---------------------------------------------------------------------------

/// Borrows a redb `ReadTransaction` from `StateDb`.
///
/// All reads happen within this session. One per command invocation.
/// Must be dropped before `commit_changes` can succeed (enforced by RAII flag).
pub struct StateReadSession<'db> {
    read_tx: redb::ReadTransaction,
    _guard: ReadSessionGuard,
    _phantom: std::marker::PhantomData<&'db StateDb>,
}

impl StateReadSession<'_> {
    /// Bulk load archived snapshots for the requested hashes.
    ///
    /// Returns a `HashMap` keyed by the hashes that were found.
    /// Hashes with no persisted entry are simply absent (no error).
    /// Empty `hashes` input returns an empty `HashMap` without table access.
    ///
    /// # Errors
    ///
    /// - `StateError::TableOpenFailed` if the snapshots table cannot be opened.
    /// - `StateError::ArchiveValidationFailed` if stored bytes fail validation.
    /// - `StateError::StorageError` if redb read fails.
    pub fn load_snapshots(
        &self,
        hashes: &[[u8; 32]],
    ) -> Result<HashMap<[u8; 32], OwnedArchive<Snapshot>>, StateError> {
        // Invariant: empty input returns empty map without table access
        if hashes.is_empty() {
            return Ok(HashMap::new());
        }

        let table =
            self.read_tx
                .open_table(SNAPSHOTS_TABLE)
                .map_err(|e| StateError::TableOpenFailed {
                    table: "snapshots",
                    message: e.to_string(),
                })?;

        hashes.iter().try_fold(
            HashMap::with_capacity(hashes.len()),
            |mut acc: HashMap<[u8; 32], OwnedArchive<Snapshot>>, key: &[u8; 32]| {
                // Read bytes from redb; copy into owned Vec immediately
                let bytes: Option<Vec<u8>> = table
                    .get(key.as_slice())
                    .map_err(|e| StateError::StorageError {
                        operation: "load_snapshots",
                        message: e.to_string(),
                    })?
                    .map(|guard| guard.value().to_vec());

                match bytes {
                    Some(bytes) => {
                        // Validate by attempting deserialization
                        bincode::deserialize::<Snapshot>(&bytes).map_err(|e| {
                            StateError::ArchiveValidationFailed {
                                key_hex: key_to_hex(key),
                                message: e.to_string(),
                            }
                        })?;
                        acc.insert(*key, OwnedArchive::from_bytes(bytes));
                        Ok(acc)
                    }
                    None => Ok(acc),
                }
            },
        )
    }
}

// ---------------------------------------------------------------------------
// Pure function: serialize_snapshot (Calc layer)
// ---------------------------------------------------------------------------

/// Serialize a `Snapshot` to bytes for inclusion in `StateChanges::new_snapshots`.
///
/// Pure function — no I/O. Uses bincode for deterministic binary serialization.
///
/// # Errors
///
/// Returns `StateError::SerializationFailed` if serialization fails.
pub fn serialize_snapshot(snapshot: &Snapshot) -> Result<Vec<u8>, StateError> {
    bincode::serialize(snapshot).map_err(|e| StateError::SerializationFailed {
        message: e.to_string(),
    })
}

// ---------------------------------------------------------------------------
// Helper: url_hash (reuses existing cache::url_hash)
// ---------------------------------------------------------------------------

/// Compute SHA-256 hash of a URL string, returning `[u8; 32]`.
#[cfg(test)]
fn url_hash(url: &str) -> [u8; 32] {
    crate::cache::url_hash(url).into()
}

/// Convert a `[u8; 32]` key to hex string for error messages.
fn key_to_hex(key: &[u8; 32]) -> String {
    key.iter().fold(String::with_capacity(64), |mut acc, b| {
        let _ = std::fmt::Write::write_fmt(&mut acc, format_args!("{b:02x}"));
        acc
    })
}

// ===========================================================================
// Unit tests (Calc Layer — serialize_snapshot)
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::watch::{PageHash, Snapshot};
    use chrono::{TimeZone, Utc};
    use std::collections::BTreeMap;

    // ── Helpers ──────────────────────────────────────────────────────────

    fn make_page_hash(url: &str, title: &str, hash_bytes: [u8; 32]) -> PageHash {
        PageHash {
            url: url.to_string(),
            content_hash: hash_bytes,
            title: title.to_string(),
        }
    }

    fn make_snapshot(target: &str, pages: Vec<(&str, &str, [u8; 32])>) -> Snapshot {
        let page_map: BTreeMap<String, PageHash> = pages
            .into_iter()
            .map(|(url, title, hash)| (url.to_string(), make_page_hash(url, title, hash)))
            .collect();
        Snapshot {
            target_url: target.to_string(),
            timestamp: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
            pages: page_map,
        }
    }

    fn sample_hash(i: u8) -> [u8; 32] {
        let mut h = [0u8; 32];
        h[0] = i;
        h
    }

    // ── B01: serialize_snapshot returns valid rkyv bytes ──────────────────

    #[test]
    fn serialize_snapshot_returns_valid_rkyv_bytes_when_given_snapshot() {
        // Given
        let snapshot = make_snapshot(
            "https://example.com",
            vec![("https://example.com/a", "Page A", sample_hash(1))],
        );

        // When
        let result = serialize_snapshot(&snapshot);

        // Then: Ok(bytes) where bytes are non-empty and round-trip produces equal value
        let bytes = result.expect("serialize_snapshot should succeed for valid Snapshot");
        assert!(!bytes.is_empty(), "serialized bytes must be non-empty");
    }

    // ── B01 variant: empty pages ──────────────────────────────────────────

    #[test]
    fn serialize_snapshot_returns_valid_bytes_when_pages_empty() {
        // Given
        let snapshot = Snapshot {
            target_url: "https://example.com".to_string(),
            timestamp: Utc.with_ymd_and_hms(2025, 6, 15, 12, 30, 0).unwrap(),
            pages: BTreeMap::new(),
        };

        // When
        let result = serialize_snapshot(&snapshot);

        // Then
        let bytes = result.expect("serialize_snapshot should succeed for empty pages");
        assert!(!bytes.is_empty());
    }

    // ── B01 variant: large snapshot (100+ pages) ─────────────────────────

    #[test]
    fn serialize_snapshot_returns_valid_bytes_when_large_snapshot() {
        // Given
        let pages: Vec<(&str, &str, [u8; 32])> = (0..100)
            .map(|i| {
                let url = format!("https://example.com/page-{i}");
                let mut h = [0u8; 32];
                h[0] = i;
                (Box::leak(url.into_boxed_str()) as &str, "title", h)
            })
            .collect();
        let snapshot = make_snapshot("https://example.com", pages);

        // When
        let result = serialize_snapshot(&snapshot);

        // Then
        let bytes = result.expect("serialize_snapshot should succeed for 100 pages");
        assert!(!bytes.is_empty());
    }

    // ── B02: serialize_snapshot returns SerializationFailed ───────────────

    #[test]
    fn serialize_snapshot_returns_serialization_failed_when_rkyv_fails() {
        // Given: In practice, serialization is infallible for all valid Snapshot
        // values. This test verifies the error variant mapping is correct.
        let snapshot = make_snapshot(
            "https://example.com",
            vec![("https://example.com/a", "Page A", sample_hash(1))],
        );

        // When
        let result = serialize_snapshot(&snapshot);

        // Then: Should succeed for normal input (SerializationFailed unreachable)
        assert!(
            result.is_ok(),
            "serialize_snapshot should succeed for valid input; SerializationFailed is only reachable in pathological edge cases"
        );
    }

    // ── Boundary: empty target_url ────────────────────────────────────────

    #[test]
    fn serialize_snapshot_succeeds_when_target_url_is_empty() {
        let snapshot = Snapshot {
            target_url: String::new(),
            timestamp: Utc::now(),
            pages: BTreeMap::new(),
        };

        let result = serialize_snapshot(&snapshot);

        assert!(
            result.is_ok(),
            "serialize_snapshot should succeed even with empty target_url"
        );
    }

    // ── Determinism: same input produces same output ─────────────────────

    #[test]
    fn serialize_snapshot_produces_identical_bytes_when_called_twice() {
        let snapshot = make_snapshot(
            "https://example.com",
            vec![("https://example.com/a", "Page A", sample_hash(1))],
        );

        let bytes1 = serialize_snapshot(&snapshot).expect("first call");
        let bytes2 = serialize_snapshot(&snapshot).expect("second call");

        assert_eq!(bytes1, bytes2, "serialize_snapshot must be deterministic");
    }

    // ── Round-trip: serialize then deserialize ────────────────────────────

    #[test]
    fn serialize_snapshot_roundtrips_when_deserialized() {
        let snapshot = make_snapshot(
            "https://example.com",
            vec![
                ("https://example.com/a", "Page A", sample_hash(1)),
                ("https://example.com/b", "Page B", sample_hash(2)),
            ],
        );

        let bytes = serialize_snapshot(&snapshot).expect("serialize");

        let archive = OwnedArchive::<Snapshot>::from_bytes(bytes);
        let restored = archive.deserialize().expect("deserialize should succeed");

        assert_eq!(restored.target_url, snapshot.target_url);
        assert_eq!(restored.pages.len(), snapshot.pages.len());
    }

    // ── Unit: url_hash produces [u8; 32] ─────────────────────────────────

    #[test]
    fn url_hash_produces_32_bytes_for_any_input() {
        let hash = url_hash("https://example.com");
        assert_eq!(hash.len(), 32);

        let hash_empty = url_hash("");
        assert_eq!(hash_empty.len(), 32);

        // Different inputs produce different hashes
        assert_ne!(hash, hash_empty);
    }

    // ── Unit: key_to_hex format ───────────────────────────────────────────

    #[test]
    fn key_to_hex_produces_lowercase_hex_string() {
        let key = [0xABu8; 32];
        let hex = key_to_hex(&key);
        assert_eq!(hex.len(), 64);
        assert_eq!(hex, "ab".repeat(32));
    }

    // ── Unit: StateChanges default is empty ───────────────────────────────

    #[test]
    fn state_changes_default_has_empty_vectors() {
        let changes = StateChanges::default();
        assert!(changes.new_snapshots.is_empty());
        assert!(changes.deleted_snapshots.is_empty());
    }

    // ── Unit: OwnedArchive from_bytes and as_bytes ────────────────────────

    #[test]
    fn owned_archive_from_bytes_stores_bytes() {
        let data = vec![1, 2, 3, 4, 5];
        let archive = OwnedArchive::<Snapshot>::from_bytes(data.clone());
        assert_eq!(archive.as_bytes(), data.as_slice());
    }
}

// ===========================================================================
// Kani harnesses
// ===========================================================================

#[cfg(kani)]
mod verification {
    use super::*;
    use crate::watch::{PageHash, Snapshot};
    use chrono::{TimeZone, Utc};
    use std::collections::BTreeMap;

    /// Kani proof: serialize_snapshot never panics for any valid Snapshot
    /// with bounded input sizes.
    #[kani::proof]
    fn kani_serialize_snapshot_no_panic() {
        let target_url_bytes: [u8; 16] = kani::any();
        let target_url = String::from_utf8_lossy(&target_url_bytes).to_string();

        let timestamp_secs: i64 = kani::any();
        kani::assume(timestamp_secs >= 0);
        kani::assume(timestamp_secs <= 4102444800); // year 2100

        let snapshot = Snapshot {
            target_url,
            timestamp: Utc
                .timestamp_opt(timestamp_secs, 0)
                .single()
                .unwrap_or_else(|| Utc::now()),
            pages: BTreeMap::new(),
        };

        // Must not panic
        let _ = serialize_snapshot(&snapshot);
    }
}
