//! Compaction logic, builder pattern, and database construction helpers.
//!
//! Contains [`StateDbBuilder`], [`compact_state_db`], [`should_suggest_compaction`],
//! and [`log_compaction_suggestion`].

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![forbid(unsafe_code)]

use super::{super::DurabilityConfig, CommitError, StateDb};
use redb::{Database, ReadableTable, TableDefinition};
use std::path::Path;

/// Default cache size: 64 `MiB` (67108864 bytes).
pub(crate) const DEFAULT_CACHE_SIZE: usize = 64 * 1024 * 1024;

/// Ratio threshold for compaction warning. If the database file is larger
/// than `logical_size * COMPACTION_THRESHOLD_RATIO`, a warning is logged
/// suggesting the user run `ctd compact`.
pub const COMPACTION_THRESHOLD_RATIO: f64 = 10.0;

// ---------------------------------------------------------------------------
// Pure Calculation: compaction threshold check
// ---------------------------------------------------------------------------

/// Returns `true` when the file size exceeds the logical data size by the
/// configured ratio, indicating significant garbage from deletes/updates.
#[must_use]
pub fn should_suggest_compaction(file_size: u64, logical_data_size: u64) -> bool {
    if logical_data_size == 0 || file_size == 0 {
        return false;
    }
    // Fix 1: Use f64 directly instead of u32 truncation
    let ratio = (file_size as f64) / (logical_data_size.max(1) as f64);
    ratio > COMPACTION_THRESHOLD_RATIO
}

// ---------------------------------------------------------------------------
// Action: compact_state_db
// ---------------------------------------------------------------------------

/// Compact an on-disk redb state database, reclaiming space from deleted
/// and updated entries.
///
/// # Errors
///
/// - [`CommitError::CompactFailed`] if the database cannot be opened or
///   compaction fails.
pub fn compact_state_db(path: &Path) -> Result<bool, CommitError> {
    if !path.exists() {
        return Err(CommitError::CompactFailed {
            path: path.display().to_string(),
            reason: "file does not exist".to_string(),
        });
    }

    let metadata = std::fs::metadata(path).map_err(|e| CommitError::CompactFailed {
        path: path.display().to_string(),
        reason: format!("cannot read file metadata: {e}"),
    })?;
    if metadata.len() == 0 {
        return Err(CommitError::CompactFailed {
            path: path.display().to_string(),
            reason: "file is empty (0 bytes)".to_string(),
        });
    }

    let mut builder = redb::Builder::new();
    builder.set_cache_size(DEFAULT_CACHE_SIZE);

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(
        || -> Result<bool, CommitError> {
            let mut db = builder.open(path).map_err(|e| CommitError::CompactFailed {
                path: path.display().to_string(),
                reason: format!("failed to open database: {e}"),
            })?;

            db.compact().map_err(|e| CommitError::CompactFailed {
                path: path.display().to_string(),
                reason: e.to_string(),
            })
        },
    ));

    match result {
        Ok(inner) => inner,
        Err(panic_payload) => {
            let reason = if let Some(s) = panic_payload.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = panic_payload.downcast_ref::<String>() {
                s.clone()
            } else {
                "unknown panic during compaction".to_string()
            };
            Err(CommitError::CompactFailed {
                path: path.display().to_string(),
                reason: format!("database appears corrupt: {reason}"),
            })
        }
    }
}

// ---------------------------------------------------------------------------
// Action: log_compaction_suggestion
// ---------------------------------------------------------------------------

/// Check if the database file has significant overhead relative to its
/// logical content, and log a warning suggesting `ctd compact` if so.
pub fn log_compaction_suggestion(db: &Database, db_path: Option<&Path>) {
    let path_str = db_path.map_or_else(|| "<in-memory>".to_string(), |p| p.display().to_string());

    // In-memory databases don't need compaction
    let file_size = match db_path.and_then(|p| std::fs::metadata(p).ok()) {
        Some(meta) => meta.len(),
        None => return,
    };

    let Ok(read_tx) = db.begin_read() else {
        return;
    };

    use super::super::{
        analysis_outputs_table, chunk_outputs_table, file_state_table, scrape_outputs_table,
        snapshots_table, transform_outputs_table, url_state_table,
    };

    let tables: &[TableDefinition<&[u8], &[u8]>] = &[
        analysis_outputs_table(),
        transform_outputs_table(),
        chunk_outputs_table(),
        scrape_outputs_table(),
        snapshots_table(),
    ];

    let hash_payload_size: u64 = tables
        .iter()
        .filter_map(|def| read_tx.open_table(*def).ok())
        .map(|table| {
            table
                .iter()
                .map(|iter| {
                    iter.filter_map(std::result::Result::ok)
                        .map(|(_, v)| u64::try_from(v.value().len()).unwrap_or(0))
                        .sum::<u64>()
                })
                .unwrap_or(0)
        })
        .sum();

    let state_tables: &[TableDefinition<&str, &[u8]>] = &[file_state_table(), url_state_table()];

    let state_payload_size: u64 = state_tables
        .iter()
        .filter_map(|def| read_tx.open_table(*def).ok())
        .map(|table| {
            table
                .iter()
                .map(|iter| {
                    iter.filter_map(std::result::Result::ok)
                        .map(|(_, v)| u64::try_from(v.value().len()).unwrap_or(0))
                        .sum::<u64>()
                })
                .unwrap_or(0)
        })
        .sum();

    let logical_size = hash_payload_size.saturating_add(state_payload_size);

    if should_suggest_compaction(file_size, logical_size) {
        tracing::warn!(
            file_size_mb = file_size / (1024 * 1024),
            logical_size_kb = logical_size / 1024,
            db_path = %path_str,
            "State database has high overhead. Consider running `ctd compact {}` to reclaim space.",
            path_str
        );
    }
}

// ---------------------------------------------------------------------------
// StateDbBuilder — builder pattern for StateDb construction
// ---------------------------------------------------------------------------

/// Builder for [`StateDb`] with configurable cache size and durability.
#[derive(Debug)]
pub struct StateDbBuilder {
    pub(crate) cache_size: usize,
    pub(crate) durability: DurabilityConfig,
}

impl StateDbBuilder {
    /// Create a new builder with defaults (64 `MiB` cache, Default durability).
    #[must_use]
    pub fn new() -> Self {
        Self {
            cache_size: DEFAULT_CACHE_SIZE,
            durability: DurabilityConfig::Default,
        }
    }

    /// Set the redb page cache size in bytes.
    #[must_use]
    pub fn cache_size(mut self, bytes: usize) -> Self {
        self.cache_size = bytes;
        self
    }

    /// Set the durability configuration for write transactions.
    #[must_use]
    pub fn durability(mut self, config: DurabilityConfig) -> Self {
        self.durability = config;
        self
    }

    /// Open or create the state database at `path` with configured settings.
    ///
    /// # Errors
    ///
    /// - [`CommitError::DatabaseOpen`] if redb cannot create/open the file.
    /// - [`CommitError::TableInit`] if any table cannot be created.
    pub fn open(self, path: &Path) -> Result<StateDb, CommitError> {
        create_parent_dirs(path)?;

        let mut builder = redb::Builder::new();
        builder.set_cache_size(self.cache_size);

        let db = builder
            .open(path)
            .or_else(|_| builder.create(path))
            .map_err(|e| CommitError::DatabaseOpen {
                path: path.display().to_string(),
                reason: e.to_string(),
            })?;

        super::super::initialize_tables(&db).map_err(|e| CommitError::TableInit {
            reason: e.to_string(),
        })?;

        Ok(StateDb::new(db, self.durability, Some(path.to_path_buf())))
    }

    /// Open an in-memory state database (no file on disk).
    ///
    /// # Errors
    ///
    /// - [`CommitError::DatabaseOpen`] if redb cannot create the in-memory database.
    /// - [`CommitError::TableInit`] if any table cannot be created.
    pub fn open_in_memory(self) -> Result<StateDb, CommitError> {
        let mut builder = redb::Builder::new();
        builder.set_cache_size(self.cache_size);

        let backend = redb::backends::InMemoryBackend::new();
        let db = builder
            .create_with_backend(backend)
            .map_err(|e| CommitError::DatabaseOpen {
                path: ":memory:".to_string(),
                reason: e.to_string(),
            })?;

        super::super::initialize_tables(&db).map_err(|e| CommitError::TableInit {
            reason: e.to_string(),
        })?;

        Ok(StateDb::new(db, self.durability, None))
    }
}

impl Default for StateDbBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Create parent directories for the database path if they do not exist.
fn create_parent_dirs(path: &Path) -> Result<(), CommitError> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|e| CommitError::DatabaseOpen {
                path: path.display().to_string(),
                reason: e.to_string(),
            })?;
        }
    }
    Ok(())
}
