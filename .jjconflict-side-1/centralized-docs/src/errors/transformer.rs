//! Document processing, I/O, and index error types.

use thiserror::Error;

/// Document processing errors.
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
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
