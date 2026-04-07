//! Error taxonomy for the commit pipeline.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![forbid(unsafe_code)]

/// Errors from the [`super::StateDb`] two-transaction state commit pipeline.
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
    /// A payload value exceeds [`super::MAX_VALUE_SIZE`].
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
    /// Compaction of the state database failed.
    #[error("compaction failed for database at {path}: {reason}")]
    CompactFailed { path: String, reason: String },
}
