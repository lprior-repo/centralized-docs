#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

//! Unified error type hierarchy for the `doc_transformer` library.
//!
//! This module provides a comprehensive error taxonomy that covers all error conditions
//! across the entire codebase. All errors implement `thiserror::Error` for ergonomic
//! error handling and context propagation.

use thiserror::Error;

#[cfg(feature = "enhanced")]
pub use crate::features::FeatureError;

pub use crate::similarity::SimilarityError;
pub use crate::types::{
    ChunkIdError, ConfigError as TypesConfigError, DocumentIdError, KeywordError, TagError,
};

/// The primary error type for the `doc_transformer` library.
#[derive(Debug, Error)]
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

    /// HNSW similarity index errors.
    #[error(transparent)]
    Similarity(#[from] SimilarityError),

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

    /// Convert an `std::io::Error` to a `DocTransformerError`.
    #[inline]
    #[must_use]
    pub fn from_io_error(error: std::io::Error) -> Self {
        DocTransformerError::Io(error.into())
    }
}

impl From<std::io::Error> for DocTransformerError {
    fn from(error: std::io::Error) -> Self {
        DocTransformerError::Io(error.into())
    }
}

/// Configuration-related errors.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[allow(dead_code)]
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

/// Validation-related errors.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ValidationError {
    #[error("query cannot be empty")]
    EmptyQuery,

    #[error("query too long: {length} bytes (max {max})")]
    QueryTooLong { length: usize, max: usize },

    #[error("query contains regex patterns which are not allowed")]
    RegexQuery,

    #[error("search limit must be positive, got {limit}")]
    InvalidLimit { limit: i64 },

    #[error("file validation failed: {path}")]
    FileValidation { path: String, message: String },

    #[error("document has {h1_count} H1 heading(s); expected exactly 1")]
    MultipleH1Headings { h1_count: usize },

    #[error("document has no H1 heading")]
    MissingH1Heading,

    #[error("document too short: {word_count} words (min {min})")]
    DocumentTooShort { word_count: usize, min: usize },

    #[error("missing required frontmatter field: {field}")]
    MissingFrontmatter { field: String },

    #[error("invalid frontmatter format: {message}")]
    InvalidFrontmatter { message: String },

    #[error("category '{category}' is not defined in configuration")]
    UnknownCategory { category: String },

    #[error("broken link detected: {link} -> {target} (file not found)")]
    BrokenLink {
        link: String,
        target: String,
        source_file: String,
    },
}

/// Document processing errors.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum DocumentError {
    #[error("document not found: {id}")]
    NotFound { id: String },

    #[error("document ID conflict: '{id}' is already in use")]
    IdConflict { id: String },

    #[error("failed to parse document: {path}")]
    ParseError { path: String, message: String },

    #[error("failed to transform document: {path}")]
    TransformError { path: String, message: String },

    #[error("document chunking failed: {document_id}")]
    ChunkingError {
        document_id: String,
        message: String,
    },

    #[error("invalid chunk ID: {id}")]
    InvalidChunkId { id: String },

    #[error("chunk not found: {id}")]
    ChunkNotFound { id: String },
}

/// Index-related errors.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum IndexError {
    #[error("index not found: {path}")]
    NotFound { path: String },

    #[error("invalid index format: {path}")]
    InvalidFormat { path: String, message: String },

    #[error("index version mismatch: expected {expected}, got {got}")]
    VersionMismatch { expected: String, got: String },

    #[error("index build failed: {message}")]
    BuildFailed { message: String },

    #[error("index search failed: {message}")]
    SearchFailed { message: String },

    #[error("Tantivy index error: {message}")]
    TantivyError { message: String },

    #[error("graph construction failed: {message}")]
    GraphConstructionFailed { message: String },

    #[error("HNSW index error: {message}")]
    HnswError { message: String },
}

/// Input/Output errors.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum IoError {
    #[error("file not found: {path}")]
    NotFound { path: String },

    #[error("permission denied: {path}")]
    PermissionDenied { path: String },

    #[error("path is not a directory: {path}")]
    NotADirectory { path: String },

    #[error("I/O error: {message}")]
    Other { message: String },
}

impl From<std::io::Error> for IoError {
    fn from(error: std::io::Error) -> Self {
        IoError::Other {
            message: error.to_string(),
        }
    }
}

/// Embedding and similarity search errors.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum EmbeddingError {
    #[error("failed to generate embedding: {message}")]
    GenerationFailed { message: String },

    #[error("embedding API error: {message}")]
    ApiError {
        message: String,
        status_code: Option<u16>,
    },

    #[error("embedding rate limited: retry after {retry_after}s")]
    RateLimited { retry_after: u64 },

    #[error("embedding dimension mismatch: expected {expected}, got {got}")]
    DimensionMismatch { expected: usize, got: usize },

    #[error("invalid embedding: contains NaN or Infinity")]
    InvalidEmbedding,

    #[error("no embedding provider configured")]
    NoProviderConfigured,

    #[error("embedding provider not supported: {provider}")]
    UnsupportedProvider { provider: String },
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
