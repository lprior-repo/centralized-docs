//! Calc: Build deterministic file-state commit batches from index outputs.
//!
//! Pure functions that consume `FileDiff` and pipeline artifacts to produce
//! a `StateChanges` batch ready for atomic commit.

mod build;
pub mod error;
pub mod pure;
pub mod types;

pub use build::build_file_state_changes;
pub use error::BatchBuildError;
pub use pure::{build_file_state_raw, hash_payload, serialize_and_hash};
pub use types::{FileDiff, FileStateRaw, PipelineOutputs, StateChanges, UrlStateRaw};

#[cfg(test)]
mod tests;

#[cfg(kani)]
mod verification;
