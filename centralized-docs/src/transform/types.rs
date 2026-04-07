//! Core types for the transform pipeline.

use crate::cache::{composite_hash, ContentHash};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;

/// Error type for transform operations with full context
#[derive(Debug, Clone)]
pub struct TransformError {
    #[allow(dead_code)]
    pub source_path: String,
    #[allow(dead_code)]
    pub error: String,
}

#[derive(Debug, Clone)]
pub struct TransformResult {
    pub success_count: usize,
    pub total_count: usize,
    pub error_count: usize,
    /// Detailed errors from failed transformations. Empty if all succeeded.
    #[allow(dead_code)]
    pub errors: Vec<TransformError>,
}

/// Create directory with improved error context for permission issues
pub(crate) fn create_dir_with_context(path: &Path, context: &str) -> Result<()> {
    fs::create_dir_all(path).map_err(|e| {
        if e.kind() == io::ErrorKind::PermissionDenied {
            anyhow::anyhow!(
                "Permission denied: cannot create {} directory '{}'\n  \
                 Hint: Check directory permissions or run with appropriate access",
                context,
                path.display()
            )
        } else {
            anyhow::anyhow!(
                "Failed to create {} directory '{}': {}",
                context,
                path.display(),
                e
            )
        }
    })
}

/// A single persisted transform output, keyed by source path.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransformArtifact {
    pub source_path: String,
    pub content_hash: ContentHash,
    pub link_map_fingerprint: ContentHash,
    pub transformed_markdown: String,
}

/// Deterministic cache key for a transform artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformArtifactKey(Vec<u8>);

impl TransformArtifactKey {
    /// Compute the artifact key from its constituent parts.
    #[must_use]
    pub fn compute(
        source_path: &str,
        content_hash: &ContentHash,
        link_map_fingerprint: &ContentHash,
    ) -> Self {
        let hash = composite_hash(&[
            source_path.as_bytes(),
            content_hash.as_bytes(),
            link_map_fingerprint.as_bytes(),
        ]);
        Self(hash.as_bytes().to_vec())
    }

    /// Return the raw bytes for use as a cache key.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// Errors specific to transform artifact capture and reuse.
#[non_exhaustive]
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum TransformArtifactError {
    #[error("empty source path: source path must be a non-empty string")]
    EmptySourcePath,

    #[error("no IdMapping found for source path: {source_path}")]
    MissingIdMapping { source_path: String },

    #[error("failed to serialize link map for fingerprinting: {message}")]
    LinkMapFingerprintFailed { message: String },

    #[error("cache read failed for transform artifact (source: {source_path}): {message}")]
    CacheReadFailed {
        source_path: String,
        message: String,
    },

    #[error("cache write failed for transform artifact (source: {source_path}): {message}")]
    CacheWriteFailed {
        source_path: String,
        message: String,
    },

    #[error("cached artifact deserialization failed for source path {source_path}: {message}")]
    DeserializationFailed {
        source_path: String,
        message: String,
    },

    #[error("failed to read file for content hashing: {source_path}: {message}")]
    FileReadFailed {
        source_path: String,
        message: String,
    },

    #[error("transform computation failed for source path {source_path}: {message}")]
    TransformComputationFailed {
        source_path: String,
        message: String,
    },

    #[error("failed to write output file for source path {source_path}: {message}")]
    OutputWriteFailed {
        source_path: String,
        message: String,
    },
}
