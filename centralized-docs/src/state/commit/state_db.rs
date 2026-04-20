//! `StateDb` — newtype wrapper over `redb::Database`.
//!
//! Provides the two-transaction architecture:
//! - Transaction 1 (read): bulk load all state into memory
//! - Transaction 2 (write): commit all changes atomically

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![forbid(unsafe_code)]

use super::{
    super::{snapshots_table, DurabilityConfig},
    compaction::log_compaction_suggestion,
    validation::validate_all,
    writes::apply_all_writes,
    CommitError, StateChanges, StateDbBuilder, StateReadSession,
};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

/// State database providing the two-transaction architecture.
pub struct StateDb {
    pub(crate) db: redb::Database,
    /// Number of active `StateReadSession` instances.
    pub(crate) active_read_sessions: AtomicUsize,
    /// Durability configuration applied to every write transaction.
    pub(crate) durability_config: DurabilityConfig,
    /// Path to the on-disk database file. `None` for in-memory databases.
    pub(crate) db_path: Option<std::path::PathBuf>,
}

impl std::fmt::Debug for StateDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StateDb")
            .field("db", &self.db)
            .field(
                "active_read_sessions",
                &self.active_read_sessions.load(Ordering::Relaxed),
            )
            .field("durability_config", &self.durability_config)
            .field("db_path", &self.db_path)
            .finish()
    }
}

impl StateDb {
    /// Create a new `StateDb` wrapping the given database.
    pub(crate) fn new(
        db: redb::Database,
        durability_config: DurabilityConfig,
        db_path: Option<std::path::PathBuf>,
    ) -> Self {
        Self {
            db,
            active_read_sessions: AtomicUsize::new(0),
            durability_config,
            db_path,
        }
    }

    /// Open the state database at the given path with default settings.
    ///
    /// # Errors
    ///
    /// - [`CommitError::DatabaseOpen`] if redb cannot create/open the file.
    /// - [`CommitError::TableInit`] if any table cannot be created.
    pub fn open(path: &Path) -> Result<Self, CommitError> {
        StateDbBuilder::new().open(path)
    }

    /// Open an in-memory state database with default settings.
    ///
    /// # Errors
    ///
    /// - [`CommitError::DatabaseOpen`] if redb cannot create the in-memory database.
    /// - [`CommitError::TableInit`] if any table cannot be created.
    pub fn open_in_memory() -> Result<Self, CommitError> {
        StateDbBuilder::new().open_in_memory()
    }

    /// Returns the active durability configuration.
    #[must_use]
    pub fn durability_config(&self) -> DurabilityConfig {
        self.durability_config
    }

    /// Open a single shared read transaction for the command's lifetime.
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
        Ok(super::reads::create_read_session(read_txn, self))
    }

    /// Commit all state changes in exactly one redb write transaction.
    ///
    /// # Errors
    ///
    /// See [`CommitError`] variants.
    #[allow(clippy::needless_pass_by_value)]
    pub fn commit_changes(&self, changes: StateChanges) -> Result<(), CommitError> {
        // Phase 0: Check one-read, one-write invariant
        // Fix 2: Use Acquire ordering for load (pairs with Acquire on fetch_add)
        let active = self.active_read_sessions.load(Ordering::Acquire);
        if active > 0 {
            return Err(CommitError::WriteTransaction {
                reason: format!(
                    "cannot commit while {active} read session(s) are active; drop all StateReadSession instances first"
                ),
            });
        }

        // Phase 1: Pure precondition validation (before write transaction)
        validate_all(&changes)?;

        // Phase 2: Open write transaction
        let mut write_tx = self
            .db
            .begin_write()
            .map_err(|e| CommitError::WriteTransaction {
                reason: e.to_string(),
            })?;

        // Phase 2b: Apply durability configuration
        if self.durability_config == DurabilityConfig::Paranoid {
            write_tx.set_two_phase_commit(true);
        }

        // Phase 3: Apply all writes within transaction
        apply_all_writes(&write_tx, &changes)?;

        // Phase 4: Commit (transaction is dropped/aborted on any earlier error)
        write_tx.commit().map_err(|e| CommitError::CommitFailed {
            reason: e.to_string(),
        })?;

        // Phase 5: Post-commit compaction suggestion check
        log_compaction_suggestion(&self.db, self.db_path.as_deref());

        Ok(())
    }

    /// Get a reference to the underlying redb database.
    #[must_use]
    pub fn database(&self) -> &redb::Database {
        &self.db
    }

    /// Returns the on-disk path of the database, or `None` for in-memory databases.
    #[must_use]
    pub fn db_path(&self) -> Option<&Path> {
        self.db_path.as_deref()
    }

    /// Drop the snapshots table.
    ///
    /// # Errors
    ///
    /// Returns [`super::super::StateError`] if the table cannot be dropped.
    pub fn drop_snapshots_table(&self) -> Result<(), super::super::StateError> {
        let write_tx = self.db.begin_write().map_err(|e| {
            super::super::StateError::WriteTransactionFailed {
                message: e.to_string(),
            }
        })?;
        {
            write_tx.delete_table(snapshots_table()).map_err(|e| {
                super::super::StateError::StorageError {
                    operation: "drop_snapshots_table",
                    message: e.to_string(),
                }
            })?;
        }
        write_tx
            .commit()
            .map_err(|e| super::super::StateError::CommitFailed {
                message: e.to_string(),
            })?;

        Ok(())
    }
}
