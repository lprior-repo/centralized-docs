//! Read session and snapshot deserialization.
//!
//! [`StateReadSession`] wraps a redb read transaction for bulk-loading
//! state into memory. [`ArchivedRaw`] provides owned deserialization
//! of rkyv-archived bytes.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![forbid(unsafe_code)]

use super::{super::snapshots_table, validation::hash_to_hex, CommitError};
use std::sync::atomic::Ordering;

// ---------------------------------------------------------------------------
// ArchivedRaw — owned wrapper for raw archived bytes
// ---------------------------------------------------------------------------

/// Owned wrapper around raw archived bytes.
///
/// Stores rkyv-archived bytes independently of any redb transaction lifetime.
/// Use [`deserialize`](ArchivedRaw::deserialize) to materialize a typed value.
#[derive(Debug)]
pub struct ArchivedRaw {
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
    /// - Returns [`super::super::StateError::InvalidArchive`] if bytes are not a valid rkyv archive.
    /// - Returns [`super::super::StateError::DeserializationFailed`] if deserialization fails.
    pub fn deserialize<T>(&self) -> Result<T, super::super::StateError>
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
            > + rkyv::Deserialize<T, rkyv::api::high::HighDeserializer<rkyv::rancor::Error>>,
    {
        rkyv::access::<T::Archived, rkyv::rancor::Error>(&self.bytes).map_err(|e| {
            super::super::StateError::InvalidArchive {
                type_name: std::any::type_name::<T>(),
                message: e.to_string(),
            }
        })?;

        rkyv::from_bytes::<T, rkyv::rancor::Error>(&self.bytes).map_err(|e| {
            super::super::StateError::DeserializationFailed {
                type_name: std::any::type_name::<T>(),
                message: e.to_string(),
            }
        })
    }
}

// ---------------------------------------------------------------------------
// StateReadSession — scoped read transaction
// ---------------------------------------------------------------------------

/// A scoped read transaction. One per command run.
/// Must be dropped before calling [`super::StateDb::commit_changes`].
pub struct StateReadSession<'db> {
    /// Underlying redb read transaction.
    read_txn: redb::ReadTransaction,
    /// Reference to the parent `StateDb` for session counting.
    state_db: &'db super::StateDb,
}

impl Drop for StateReadSession<'_> {
    fn drop(&mut self) {
        // Fix 2: Use Release ordering for decrement (pairs with Acquire on increment)
        self.state_db
            .active_read_sessions
            .fetch_sub(1, Ordering::Release);
    }
}

impl StateReadSession<'_> {
    /// Bulk-load archived snapshots for the requested hash keys.
    ///
    /// Returns a `HashMap` keyed by the same `[u8; 32]` hashes, with
    /// [`ArchivedRaw`] values that own their bytes independently of the
    /// redb transaction lifetime.
    ///
    /// # Errors
    ///
    /// - [`super::super::StateError::TableOpenFailed`] if the snapshots table cannot be opened.
    /// - [`super::super::StateError::StorageError`] if a redb read fails.
    /// - [`super::super::StateError::ArchiveValidationFailed`] if stored bytes fail rkyv validation.
    pub fn load_snapshots(
        &self,
        keys: &[[u8; 32]],
    ) -> Result<std::collections::HashMap<[u8; 32], ArchivedRaw>, super::super::StateError> {
        use super::super::TABLE_NAME_SNAPSHOTS;

        let table = self.read_txn.open_table(snapshots_table()).map_err(|e| {
            super::super::StateError::TableOpenFailed {
                table: TABLE_NAME_SNAPSHOTS,
                message: e.to_string(),
            }
        })?;

        keys.iter()
            .try_fold(std::collections::HashMap::new(), |mut acc, key| {
                let bytes = {
                    let guard = table.get(key.as_slice()).map_err(|e| {
                        super::super::StateError::StorageError {
                            operation: "load_snapshots::get",
                            message: e.to_string(),
                        }
                    })?;
                    match guard {
                        Some(g) => g.value().to_vec(),
                        None => return Ok(acc),
                    }
                }; // guard dropped here, releasing table borrow

                rkyv::access::<
                    <crate::watch::Snapshot as rkyv::Archive>::Archived,
                    rkyv::rancor::Error,
                >(&bytes)
                .map_err(|e| super::super::StateError::ArchiveValidationFailed {
                    key_hex: hash_to_hex(key),
                    message: e.to_string(),
                })?;

                acc.insert(*key, ArchivedRaw::from_bytes(bytes));
                Ok(acc)
            })
    }
}

/// Helper: create a `StateReadSession` from a read transaction and a reference to the parent `StateDb`.
/// Called by [`super::StateDb::begin_read`].
pub(crate) fn create_read_session<'db>(
    read_txn: redb::ReadTransaction,
    state_db: &'db super::StateDb,
) -> Result<StateReadSession<'db>, CommitError> {
    // Fix 2: Use Acquire ordering for increment (pairs with Release on decrement + Acquire on load)
    state_db
        .active_read_sessions
        .fetch_add(1, Ordering::Acquire);
    Ok(StateReadSession { read_txn, state_db })
}
