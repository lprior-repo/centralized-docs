//! Calc layer: pure deterministic functions for building file-state commit batches.
//!
//! This module contains zero-I/O, zero-side-effect functions that derive
//! `StateChanges` batches from `FileDiff` results and pipeline outputs.

pub mod build_state_changes;
pub mod scrape_diff;

pub use build_state_changes::{
    build_file_state_changes, build_file_state_raw, hash_payload, serialize_and_hash,
    BatchBuildError, FileDiff, FileStateRaw, PipelineOutputs, StateChanges, UrlStateRaw,
};
pub use scrape_diff::{
    build_combined_scrape_result, build_scrape_state_changes, classify_scrape_diff, ScrapeDiff,
};
