//! Transformation phase of the documentation transformation pipeline.
//!
//! This module transforms analyzed documents into the final output format.
//! It forms the third phase of the pipeline, following [`analyze`] and
//! preceding indexing and output generation.

#![deny(clippy::unwrap_used)]
#![allow(clippy::match_same_arms)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![forbid(unsafe_code)]
#![allow(clippy::wildcard_enum_match_arm)]

pub mod artifact_cache;
pub mod ast_context;
pub mod ast_transforms;
pub mod pipeline;
pub mod types;

// Re-exports for backward compatibility
pub use artifact_cache::{
    compute_link_map_fingerprint, load_cached_artifact, process_single_cached, store_artifact,
    transform_all_cached, write_artifact_to_output,
};
pub use pipeline::{transform_all, transform_file};
pub use types::{
    TransformArtifact, TransformArtifactError, TransformArtifactKey, TransformError,
    TransformResult,
};

#[cfg(test)]
mod tests;
