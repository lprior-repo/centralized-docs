//! Error type for bulk loader operations.

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
