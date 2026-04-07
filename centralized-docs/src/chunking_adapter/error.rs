//! Error taxonomy for the chunk-reuse (cached chunking) pathway.
//!
//! Fatal variants are returned via `anyhow::Error`.
//! Non-fatal variants are logged and result in re-chunking.

use crate::cache::ContentHash;
use std::fmt;
use std::io;
use std::path::PathBuf;

/// Errors specific to the chunk-reuse (cached chunking) pathway.
#[non_exhaustive]
#[derive(Debug)]
pub enum ChunkReuseError {
    /// The chunks output directory could not be created.
    ChunksDirCreationFailed { path: PathBuf, source: io::Error },

    /// A document exceeds the configured byte limit.
    DocumentExceedsSizeLimit {
        source_path: String,
        content_size: u64,
        max_bytes: u64,
    },

    /// The `contextual_chunker` returned an error for a changed file.
    ChunkerFailed { source_path: String, reason: String },

    /// A cached chunk entry could not be deserialized (non-fatal, logged).
    CacheDeserializationFailed { key: ContentHash, reason: String },

    /// Writing a chunk `.md` file to disk failed.
    ChunkWriteFailed { path: PathBuf, source: io::Error },

    /// Cache read I/O error (non-fatal, logged, results in re-chunking).
    CacheReadFailed { key: ContentHash, reason: String },

    /// Cache write failed after fresh chunking (non-fatal, logged).
    CacheWriteFailed { key: ContentHash, reason: String },
}

impl fmt::Display for ChunkReuseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ChunksDirCreationFailed { path, source } => {
                write!(
                    f,
                    "chunks dir creation failed for '{}': {source}",
                    path.display()
                )
            }
            Self::DocumentExceedsSizeLimit {
                source_path,
                content_size,
                max_bytes,
            } => {
                write!(
                    f,
                    "document '{source_path}' ({content_size} bytes) exceeds maximum document size limit ({max_bytes} bytes)"
                )
            }
            Self::ChunkerFailed {
                source_path,
                reason,
            } => {
                write!(f, "chunker failed for '{source_path}': {reason}")
            }
            Self::CacheDeserializationFailed { key, reason } => {
                write!(f, "cache deserialization failed for key {key}: {reason}")
            }
            Self::ChunkWriteFailed { path, source } => {
                write!(f, "chunk write failed for '{}': {source}", path.display())
            }
            Self::CacheReadFailed { key, reason } => {
                write!(f, "cache read failed for key {key}: {reason}")
            }
            Self::CacheWriteFailed { key, reason } => {
                write!(f, "cache write failed for key {key}: {reason}")
            }
        }
    }
}

impl std::error::Error for ChunkReuseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ChunksDirCreationFailed { source, .. }
            | Self::ChunkWriteFailed { source, .. } => Some(source),
            _ => None,
        }
    }
}
