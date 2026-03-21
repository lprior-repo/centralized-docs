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
}
