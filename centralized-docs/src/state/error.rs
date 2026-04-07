//! Error types and durability configuration for state database operations.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

use std::path::PathBuf;

// ---------------------------------------------------------------------------
// DurabilityConfig — domain enum for write-transaction crash safety
// ---------------------------------------------------------------------------

/// Crash-safety configuration for [`commit::StateDb`] write transactions.
///
/// Controls whether each `commit_changes` call performs an extra fsync
/// (two-phase commit) for maximum durability guarantees.
///
/// # Variants
///
/// - [`Default`](DurabilityConfig::Default): redb's built-in `Immediate` durability.
///   Single fsync on commit. Fastest safe option.
/// - [`Paranoid`](DurabilityConfig::Paranoid): enables redb two-phase commit
///   (`set_two_phase_commit(true)`). Extra fsync after every commit for
///   maximum crash safety.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurabilityConfig {
    /// Default redb durability (`Immediate`). Single fsync on commit.
    Default,
    /// Paranoid: two-phase commit. Extra fsync for maximum crash safety.
    Paranoid,
}

impl Default for DurabilityConfig {
    fn default() -> Self {
        Self::Default
    }
}

// ---------------------------------------------------------------------------
// StateError -- error taxonomy for state database operations
// ---------------------------------------------------------------------------

/// Error type for state database operations.
#[non_exhaustive]
#[derive(Debug, thiserror::Error, Clone)]
pub enum StateError {
    #[error("failed to open state database at {path}: {detail}")]
    OpenFailed { path: PathBuf, detail: String },

    #[error("failed to begin read transaction: {message}")]
    ReadTransactionFailed { message: String },

    #[error("failed to begin write transaction: {message}")]
    WriteTransactionFailed { message: String },

    #[error("pod value size mismatch for table {table}: expected {expected} bytes, got {actual}")]
    PodSizeMismatch {
        table: &'static str,
        expected: usize,
        actual: usize,
    },

    #[error("pod cast failed for type {type_name}: {message}")]
    PodCastFailed {
        type_name: &'static str,
        message: String,
    },

    #[error("invalid rkyv archive for type {type_name}: {message}")]
    InvalidArchive {
        type_name: &'static str,
        message: String,
    },

    #[error("rkyv deserialization failed for type {type_name}: {message}")]
    DeserializationFailed {
        type_name: &'static str,
        message: String,
    },

    #[error("archive validation failed for key {key_hex}: {message}")]
    ArchiveValidationFailed { key_hex: String, message: String },

    #[error("rkyv serialization failed for type {type_name}: {message}")]
    SerializationFailed {
        type_name: &'static str,
        message: String,
    },

    #[error("failed to open table {table}: {message}")]
    TableOpenFailed {
        table: &'static str,
        message: String,
    },

    #[error("key not found in {table}")]
    KeyNotFound { table: &'static str },

    #[error("redb storage error during {operation}: {message}")]
    StorageError {
        operation: &'static str,
        message: String,
    },

    #[error("failed to commit state changes: {message}")]
    CommitFailed { message: String },

    #[error("hash key has wrong length: expected 32 bytes, got {actual}")]
    InvalidHashKeyLength { actual: usize },

    #[error("invalid source path key: {reason}")]
    InvalidSourcePath { reason: String },

    #[error("invalid URL key: {reason}")]
    InvalidUrlKey { reason: String },
}

// ---------------------------------------------------------------------------
// StateLoadError -- error taxonomy for bulk state loading
// ---------------------------------------------------------------------------

/// Errors during bulk state loading.
#[non_exhaustive]
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum StateLoadError {
    #[error("malformed raw state row for key {key:?}: got {actual} bytes, expected {expected}")]
    MalformedRow {
        key: String,
        actual: usize,
        expected: usize,
    },

    #[error("non-UTF-8 key in state table: {bytes_lossy:?}")]
    Utf8KeyError { bytes_lossy: String },

    #[error("cache backend error during {operation}: {message}")]
    BackendError {
        operation: &'static str,
        message: String,
    },
}
