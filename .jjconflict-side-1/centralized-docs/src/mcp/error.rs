use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CtdMcpError {
    #[error("INDEX.json not found in {path}")]
    IndexNotFound { path: PathBuf },

    #[error("Failed to parse INDEX.json: {reason}")]
    IndexCorrupted { reason: String },

    #[error("Invalid input: {detail}")]
    InvalidInput { detail: String },

    #[error("Search index error: {reason}")]
    SearchIndexError { reason: String },

    #[error("Query error: {reason}")]
    QueryError { reason: String },

    #[error("I/O error: {reason}")]
    IoError { reason: String },

    #[error("Internal error: {reason}")]
    Internal { reason: String },
}
