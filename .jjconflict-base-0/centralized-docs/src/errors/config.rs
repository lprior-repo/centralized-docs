//! Configuration-related error types.

use thiserror::Error;

/// Configuration-related errors.
///
/// All variants are retained as part of the public error API (`#[non_exhaustive]`).
/// They represent the full taxonomy of configuration failure modes that downstream
/// consumers may match against even if the current codebase only signals them via
/// `DocTransformerError::Operation` today.
#[non_exhaustive]
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ConfigError {
    #[error("configuration file not found: {path}")]
    NotFound { path: String },

    #[error("invalid configuration file format: {path}")]
    InvalidFormat { path: String, message: String },

    #[error("missing required configuration key: {key}")]
    MissingKey { key: String },

    #[error("invalid value for {key}: {message}")]
    InvalidValue { key: String, message: String },

    #[error("category rule validation failed: {message}")]
    CategoryRule { message: String },
}
