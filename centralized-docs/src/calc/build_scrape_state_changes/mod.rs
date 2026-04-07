//! Calc: Build deterministic URL-state commit batches from scrape classification results.
//!
//! Pure functions that consume `ScrapeDiff` and `ScrapeOutputs` to produce
//! a `StateChanges` batch ready for atomic commit via `StateDb::commit_changes`.
//!
//! This mirrors `build_file_state_changes` but for the URL-state domain:
//! - Changed/new URLs produce `UrlStateRaw` rows + scrape payload blobs
//! - Deleted URLs produce only delete entries
//! - Unchanged URLs produce no output

#![cfg_attr(
    not(test),
    deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)
)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
#[cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::arithmetic_side_effects
    )
)]
use crate::calc::build_state_changes::hash_payload;
use crate::state::commit::StateChanges;
use crate::state::UrlStateRaw;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

// ---------------------------------------------------------------------------
// Domain Types
// ---------------------------------------------------------------------------

/// Partition of scraped URLs into unchanged, changed, new, and deleted buckets.
/// Every URL appears in exactly one bucket.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrapeDiff {
    /// URLs whose markdown content hash matches the stored state (no rewrite needed).
    pub unchanged: Vec<String>,
    /// URLs present in both current scrape and stored state, but with different content hash.
    pub changed: Vec<String>,
    /// URLs present in current scrape but not in stored state.
    pub new_urls: Vec<String>,
    /// URLs present in stored state but not in current scrape.
    pub deleted: Vec<String>,
}

/// Processed scrape artifacts keyed by URL string.
#[derive(Debug, Clone)]
pub struct ScrapeOutputs {
    /// Map of URL -> scrape artifact (content hash, status code, serialized payload).
    pub artifacts: HashMap<String, ScrapeArtifact>,
}

/// Single page's processed scrape artifact.
#[derive(Debug, Clone)]
pub struct ScrapeArtifact {
    /// SHA-256 of the page's markdown content.
    pub content_hash: [u8; 32],
    /// HTTP status code from the scrape (e.g., 200).
    pub status_code: u16,
    /// Serialized scrape payload bytes (ready for storage).
    pub payload_bytes: Vec<u8>,
}

/// Configuration for the scrape batch builder.
#[derive(Debug, Clone)]
pub struct ScrapeBatchConfig {
    /// Unix timestamp (seconds) for `last_fetched_secs` in `UrlStateRaw`.
    pub now_secs: u64,
}

// ---------------------------------------------------------------------------
// Error Taxonomy
// ---------------------------------------------------------------------------

/// Exhaustive error taxonomy for `build_scrape_state_changes`.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ScrapeBatchBuildError {
    /// A changed or new URL has no corresponding scrape artifact.
    #[error("missing scrape artifact for URL: {url}")]
    MissingScrapeArtifact { url: String },
    /// A changed or new URL's artifact has zero-length payload bytes.
    #[error("empty scrape payload for URL: {url}")]
    EmptyScrapePayload { url: String },
    /// Serialization of a scrape payload failed.
    #[error("scrape payload processing failed for URL {url}: {reason}")]
    PayloadProcessingFailed { url: String, reason: String },
    /// A URL appears in more than one diff category.
    #[error("duplicate URL in scrape diff: {url} appears in multiple categories")]
    DuplicateUrl { url: String },
    /// The input `ScrapeDiff` was empty (no URLs in any category).
    #[error("scrape diff is empty: no unchanged, changed, new, or deleted URLs")]
    EmptyDiff,
}

// ---------------------------------------------------------------------------
// Pure Functions
// ---------------------------------------------------------------------------

/// Build a deterministic URL-state change batch from scrape classification results.
///
/// # Errors
/// Returns `Err(ScrapeBatchBuildError)` if preconditions are violated.
pub fn build_scrape_state_changes(
    diff: &ScrapeDiff,
    outputs: &ScrapeOutputs,
    config: &ScrapeBatchConfig,
) -> Result<StateChanges, ScrapeBatchBuildError> {
    if diff.unchanged.is_empty()
        && diff.changed.is_empty()
        && diff.new_urls.is_empty()
        && diff.deleted.is_empty()
    {
        return Err(ScrapeBatchBuildError::EmptyDiff);
    }
    check_no_duplicate_urls(diff)?;
    let processed: Vec<ScrapeEntry> = diff
        .changed
        .iter()
        .chain(diff.new_urls.iter())
        .map(|url| process_single_url(url, outputs, config))
        .collect::<Result<Vec<_>, _>>()?;
    let updated_urls = processed
        .iter()
        .map(|entry| (entry.url.clone(), entry.state))
        .collect();
    let new_scrapes = processed
        .iter()
        .map(|entry| (entry.url_hash, entry.payload_bytes.clone()))
        .collect();
    Ok(StateChanges {
        updated_files: vec![],
        deleted_files: vec![],
        new_analyses: vec![],
        new_transforms: vec![],
        new_chunks: vec![],
        updated_urls,
        deleted_urls: diff.deleted.clone(),
        new_scrapes,
        new_snapshots: vec![],
        deleted_snapshots: vec![],
    })
}

/// Construct a `UrlStateRaw` from individual hash and timestamp components.
#[must_use]
pub fn build_url_state_raw(
    content_hash: [u8; 32],
    url_hash: [u8; 32],
    last_fetched_secs: u64,
    status_code: u16,
) -> UrlStateRaw {
    UrlStateRaw {
        content_hash,
        url_hash,
        last_fetched_secs,
        status_code,
        reserved: [0u8; 46],
    }
}

// ---------------------------------------------------------------------------
// Internal Helpers
// ---------------------------------------------------------------------------

struct ScrapeEntry {
    url: String,
    state: UrlStateRaw,
    url_hash: [u8; 32],
    payload_bytes: Vec<u8>,
}

fn check_no_duplicate_urls(diff: &ScrapeDiff) -> Result<(), ScrapeBatchBuildError> {
    let all_urls: Vec<&str> = diff
        .unchanged
        .iter()
        .map(String::as_str)
        .chain(diff.changed.iter().map(String::as_str))
        .chain(diff.new_urls.iter().map(String::as_str))
        .chain(diff.deleted.iter().map(String::as_str))
        .collect();
    let unique: HashSet<&str> = all_urls.iter().copied().collect();
    if unique.len() == all_urls.len() {
        return Ok(());
    }
    let duplicate = all_urls
        .iter()
        .find(|&&url| all_urls.iter().filter(|&&u| u == url).count() > 1)
        .copied();
    match duplicate {
        Some(url) => Err(ScrapeBatchBuildError::DuplicateUrl {
            url: url.to_string(),
        }),
        None => Ok(()),
    }
}

fn process_single_url(
    url: &str,
    outputs: &ScrapeOutputs,
    config: &ScrapeBatchConfig,
) -> Result<ScrapeEntry, ScrapeBatchBuildError> {
    let artifact =
        outputs
            .artifacts
            .get(url)
            .ok_or_else(|| ScrapeBatchBuildError::MissingScrapeArtifact {
                url: url.to_string(),
            })?;
    if artifact.payload_bytes.is_empty() {
        return Err(ScrapeBatchBuildError::EmptyScrapePayload {
            url: url.to_string(),
        });
    }
    let url_hash = hash_payload(&artifact.payload_bytes);
    let state = build_url_state_raw(
        artifact.content_hash,
        url_hash,
        config.now_secs,
        artifact.status_code,
    );
    Ok(ScrapeEntry {
        url: url.to_string(),
        state,
        url_hash,
        payload_bytes: artifact.payload_bytes.clone(),
    })
}

#[cfg(test)]
mod tests;
