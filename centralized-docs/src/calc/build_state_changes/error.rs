//! Error taxonomy for state change computation.

use thiserror::Error;

/// Exhaustive error taxonomy for `build_file_state_changes`.
#[derive(Debug, Error)]
pub enum BatchBuildError {
    /// A changed or new file has no analysis artifact.
    #[error("missing analysis artifact for source path: {path}")]
    MissingAnalysis { path: String },

    /// A changed or new file has no transform artifact.
    #[error("missing transform artifact for source path: {path}")]
    MissingTransform { path: String },

    /// A changed or new file has no chunk artifact.
    #[error("missing chunk artifact for source path: {path}")]
    MissingChunk { path: String },

    /// A changed or new file has no content hash.
    #[error("missing content hash for source path: {path}")]
    MissingContentHash { path: String },

    /// Serialization of an Analysis value failed.
    #[error("rkyv serialization failed for analysis of {path}: {reason}")]
    AnalysisSerializationFailed { path: String, reason: String },

    /// Serialization of a transform value failed.
    #[error("rkyv serialization failed for transform of {path}: {reason}")]
    TransformSerializationFailed { path: String, reason: String },

    /// Serialization of a chunk value failed.
    #[error("rkyv serialization failed for chunks of {path}: {reason}")]
    ChunkSerializationFailed { path: String, reason: String },

    /// A duplicate `source_path` was detected across diff categories.
    #[error("duplicate source_path in diff: {path} appears in multiple categories")]
    DuplicateSourcePath { path: String },

    /// The input `FileDiff` was empty (no files in any category).
    #[error("file diff is empty: no unchanged, changed, new, or deleted files")]
    EmptyDiff,
}
