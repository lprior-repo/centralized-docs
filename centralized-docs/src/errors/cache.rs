//! Cache-related error types.

use thiserror::Error;

/// Cache-related errors.
#[non_exhaustive]
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CacheError {
    #[error("cache key too large: {size} bytes (max {max})")]
    KeyTooLarge { size: usize, max: usize },

    #[error("cache value too large: {size} bytes (max {max})")]
    ValueTooLarge { size: usize, max: usize },

    #[error("cache backend error during {operation}: {message}")]
    BackendError {
        operation: &'static str,
        message: String,
    },

    #[error("cache not initialized")]
    NotInitialized,

    #[error("cache already open at path: {0}")]
    AlreadyOpen(String),

    #[error("cache I/O error: {0}")]
    Io(String),

    #[error("cache serialization error: {0}")]
    Serialization(String),

    #[error("cache deserialization error: {0}")]
    Deserialization(String),
}
