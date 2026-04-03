#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

//! Unified error type hierarchy for the `ctd` library.
//!
//! This module provides a comprehensive error taxonomy that covers all error conditions
//! across the entire codebase. All errors implement `thiserror::Error` for ergonomic
//! error handling and context propagation.

use thiserror::Error;

#[cfg(feature = "enhanced")]
pub use crate::features::FeatureError;

pub use crate::types::{
    ChunkIdError, ConfigError as TypesConfigError, DocumentIdError, KeywordError, TagError,
};

mod cache;
mod config;
mod embedding;
mod transformer;
mod validation;

pub use cache::CacheError;
pub use config::ConfigError;
pub use embedding::EmbeddingError;
pub use transformer::{DocumentError, IndexError, IoError};
pub use validation::ValidationError;

/// The primary error type for the `ctd` library.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)]
#[allow(dead_code)]
pub enum DocTransformerError {
    /// Configuration-related errors.
    #[error(transparent)]
    Config(#[from] ConfigError),

    /// Validation-related errors.
    #[error(transparent)]
    Validation(#[from] ValidationError),

    /// Document processing errors.
    #[error(transparent)]
    Document(#[from] DocumentError),

    /// Index-related errors.
    #[error(transparent)]
    Index(#[from] IndexError),

    /// Input/Output errors.
    #[error(transparent)]
    Io(#[from] IoError),

    /// Embedding and similarity search errors.
    #[error(transparent)]
    Embedding(#[from] EmbeddingError),

    /// Chunk ID validation errors.
    #[error(transparent)]
    ChunkId(#[from] ChunkIdError),

    /// Tag validation errors.
    #[error(transparent)]
    Tag(#[from] TagError),

    /// Keyword validation errors.
    #[error(transparent)]
    Keyword(#[from] KeywordError),

    /// Cache-related errors.
    #[error(transparent)]
    Cache(#[from] CacheError),

    #[cfg(feature = "enhanced")]
    #[error(transparent)]
    Features(#[from] FeatureError),

    /// A catch-all error for errors that don't fit other categories.
    #[error("operation failed: {message}")]
    Operation { message: String },
}

impl DocTransformerError {
    /// Create an operation error with just a message.
    pub fn operation(message: impl Into<String>) -> Self {
        DocTransformerError::Operation {
            message: message.into(),
        }
    }
}

impl From<std::io::Error> for DocTransformerError {
    fn from(error: std::io::Error) -> Self {
        DocTransformerError::Io(error.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file.txt");
        let doc_error: DocTransformerError = io_error.into();
        // IoError from std::io::Error now returns IoError::Other
        assert!(matches!(
            doc_error,
            DocTransformerError::Io(IoError::Other { .. })
        ));
    }

    #[test]
    fn test_operation_error() {
        let error = DocTransformerError::operation("test failed");
        assert_eq!(error.to_string(), "operation failed: test failed");
    }

    #[test]
    fn test_validation_error_display() {
        let error = ValidationError::QueryTooLong {
            length: 2000,
            max: 1000,
        };
        assert_eq!(error.to_string(), "query too long: 2000 bytes (max 1000)");
    }
}
