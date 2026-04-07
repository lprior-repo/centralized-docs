//! Batch-commit pipeline for atomic state mutations.
//!
//! Implements the two-transaction architecture:
//! 1. **Read transaction**: bulk load all state into memory via [`StateReadSession`]
//! 2. **Write transaction**: commit all changes atomically via [`StateDb::commit_changes`]

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

mod compaction;
mod error;
mod multimap;
mod reads;
mod state_db;
mod validation;
mod writes;

pub use compaction::{
    compact_state_db, log_compaction_suggestion, should_suggest_compaction, StateDbBuilder,
    COMPACTION_THRESHOLD_RATIO,
};
pub use error::CommitError;
pub use reads::{ArchivedRaw, StateReadSession};
pub use state_db::StateDb;
pub use validation::should_skip_write;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Maximum payload value size (50 `MiB`).
pub const MAX_VALUE_SIZE: usize = 50 * 1024 * 1024;

/// The zero hash `[0u8; 32]`, representing "no output yet".
pub(crate) const ZERO_HASH: [u8; 32] = [0u8; 32];

// ---------------------------------------------------------------------------
// StateChanges — batch of mutations (consumed by commit_changes)
// ---------------------------------------------------------------------------

/// Batch of state mutations to commit atomically in a single redb write transaction.
pub struct StateChanges {
    /// Files to upsert: `(source_path, FileStateRaw)`.
    pub updated_files: Vec<(String, super::FileStateRaw)>,
    /// File `source_path`s to delete.
    pub deleted_files: Vec<String>,
    /// New/updated analysis outputs: `(hash_key, rkyv_bytes)`.
    pub new_analyses: Vec<([u8; 32], Vec<u8>)>,
    /// New/updated transform outputs: `(hash_key, rkyv_bytes)`.
    pub new_transforms: Vec<([u8; 32], Vec<u8>)>,
    /// New/updated chunk outputs: `(hash_key, rkyv_bytes)`.
    pub new_chunks: Vec<([u8; 32], Vec<u8>)>,
    /// URLs to upsert: `(url, UrlStateRaw)`.
    pub updated_urls: Vec<(String, super::UrlStateRaw)>,
    /// URLs to delete.
    pub deleted_urls: Vec<String>,
    /// New/updated scrape outputs: `(hash_key, rkyv_bytes)`.
    pub new_scrapes: Vec<([u8; 32], Vec<u8>)>,
    /// New/updated snapshot outputs: `(hash_key, rkyv_bytes)`.
    pub new_snapshots: Vec<([u8; 32], Vec<u8>)>,
    /// Snapshot hash keys to delete.
    pub deleted_snapshots: Vec<[u8; 32]>,
}

impl StateChanges {
    /// Create an empty `StateChanges` with all vecs empty.
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
// Tests — split across files in the tests/ subdirectory
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests;
