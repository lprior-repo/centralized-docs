//! Calc layer: pure deterministic functions for building commit batches.
//!
//! This module contains zero-I/O, zero-side-effect functions that derive
//! `StateChanges` batches from diff results and pipeline outputs.
//!
//! - `build_state_changes`: file-state domain (`FileDiff` -> `StateChanges`)
//! - `build_scrape_state_changes`: URL-state domain (`ScrapeDiff` -> `StateChanges`)

pub mod build_scrape_state_changes;
pub mod build_state_changes;
pub mod scrape_diff;

pub use build_scrape_state_changes::{
    build_scrape_state_changes as scrape_batch, build_url_state_raw, ScrapeArtifact,
    ScrapeBatchBuildError, ScrapeBatchConfig, ScrapeDiff, ScrapeOutputs,
};
pub use build_state_changes::{
    build_file_state_changes, build_file_state_raw, hash_payload, serialize_and_hash,
    BatchBuildError, FileDiff, FileStateRaw, PipelineOutputs, StateChanges, UrlStateRaw,
};
pub use scrape_diff::{
    build_combined_scrape_result, classify_scrape_diff, ScrapeDiff as RawScrapeDiff,
};
