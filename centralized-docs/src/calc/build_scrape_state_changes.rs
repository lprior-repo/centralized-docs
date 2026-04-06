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
    /// Serialized scrape payload bytes (ready for storage in `scrape_outputs` table).
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

/// Build a deterministic URL-state change batch from scrape classification results
/// and processed scrape outputs.
///
/// Changed and new URLs produce updated `UrlStateRaw` rows and scrape payload blobs.
/// Deleted URLs produce only delete entries. Unchanged URLs are not rewritten.
///
/// # Errors
///
/// Returns `Err(ScrapeBatchBuildError)` if preconditions are violated:
/// - `EmptyDiff` when all four categories are empty
/// - `DuplicateUrl` when a URL appears in multiple categories
/// - `MissingScrapeArtifact` when a changed or new URL has no artifact
/// - `EmptyScrapePayload` when an artifact's payload is zero-length
/// - `PayloadProcessingFailed` when payload processing fails
pub fn build_scrape_state_changes(
    diff: &ScrapeDiff,
    outputs: &ScrapeOutputs,
    config: &ScrapeBatchConfig,
) -> Result<StateChanges, ScrapeBatchBuildError> {
    // PRE-1: Check for completely empty diff (all four categories empty)
    if diff.unchanged.is_empty()
        && diff.changed.is_empty()
        && diff.new_urls.is_empty()
        && diff.deleted.is_empty()
    {
        return Err(ScrapeBatchBuildError::EmptyDiff);
    }

    // PRE-2: Validate no duplicate URLs across categories
    check_no_duplicate_urls(diff)?;

    // Process all changed + new URLs through artifact validation
    let processed: Vec<ScrapeEntry> = diff
        .changed
        .iter()
        .chain(diff.new_urls.iter())
        .map(|url| process_single_url(url, outputs, config))
        .collect::<Result<Vec<_>, _>>()?;

    // Build output collections from processed results (deterministic ordering)
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
///
/// All fields are set to the provided values. `reserved` is zeroed.
/// Total struct size is exactly 120 bytes.
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

/// Processed scrape entry for a single changed/new URL.
struct ScrapeEntry {
    url: String,
    state: UrlStateRaw,
    url_hash: [u8; 32],
    payload_bytes: Vec<u8>,
}

/// Check that no URL appears in more than one diff category.
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

    // Find first URL that appears more than once (O(n²) but n is small)
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

/// Process a single changed/new URL: validate artifact, build state row.
fn process_single_url(
    url: &str,
    outputs: &ScrapeOutputs,
    config: &ScrapeBatchConfig,
) -> Result<ScrapeEntry, ScrapeBatchBuildError> {
    // PRE-3: Artifact must exist for every changed/new URL
    let artifact =
        outputs
            .artifacts
            .get(url)
            .ok_or_else(|| ScrapeBatchBuildError::MissingScrapeArtifact {
                url: url.to_string(),
            })?;

    // PRE-4: Artifact payload must be non-empty
    if artifact.payload_bytes.is_empty() {
        return Err(ScrapeBatchBuildError::EmptyScrapePayload {
            url: url.to_string(),
        });
    }

    // Compute FK hash from payload bytes (infallible SHA-256)
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

// ===========================================================================
// Unit Tests — RED PHASE (must fail)
// ===========================================================================

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // -----------------------------------------------------------------------
    // Test Helpers
    // -----------------------------------------------------------------------

    /// Create a valid `ScrapeArtifact` with non-default values.
    fn make_artifact(url: &str, payload: &[u8]) -> ScrapeArtifact {
        ScrapeArtifact {
            content_hash: hash_payload(url.as_bytes()),
            status_code: 200,
            payload_bytes: payload.to_vec(),
        }
    }

    /// Create a `ScrapeArtifact` with a specific content hash.
    fn make_artifact_with_content_hash(content_hash: [u8; 32], payload: &[u8]) -> ScrapeArtifact {
        ScrapeArtifact {
            content_hash,
            status_code: 200,
            payload_bytes: payload.to_vec(),
        }
    }

    /// Create a `ScrapeArtifact` with a specific status code.
    fn make_artifact_with_status(status_code: u16, payload: &[u8]) -> ScrapeArtifact {
        ScrapeArtifact {
            content_hash: [0x42; 32],
            status_code,
            payload_bytes: payload.to_vec(),
        }
    }

    /// Create a `ScrapeOutputs` with artifacts for the given URLs.
    fn make_scrape_outputs(urls: &[&str]) -> ScrapeOutputs {
        let mut artifacts = HashMap::new();
        for &url in urls {
            artifacts.insert(
                url.to_string(),
                make_artifact(url, format!("payload_for_{url}").as_bytes()),
            );
        }
        ScrapeOutputs { artifacts }
    }

    fn make_config(now_secs: u64) -> ScrapeBatchConfig {
        ScrapeBatchConfig { now_secs }
    }

    // ===================================================================
    // B01: Changed URLs produce updated rows and payload blobs
    // ===================================================================

    #[test]
    fn scrape_batch_produces_updated_rows_for_changed_urls() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec!["https://a.com".to_string(), "https://b.com".to_string()],
            new_urls: vec![],
            deleted: vec![],
        };
        let outputs = make_scrape_outputs(&["https://a.com", "https://b.com"]);
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("should succeed for valid changed URLs");
        assert_eq!(changes.updated_urls.len(), 2);
        assert_eq!(changes.updated_urls[0].0, "https://a.com");
        assert_eq!(changes.updated_urls[1].0, "https://b.com");
        assert_eq!(changes.new_scrapes.len(), 2);
        assert!(changes.deleted_urls.is_empty());
    }

    // ===================================================================
    // B02: New URLs produce updated rows and payload blobs
    // ===================================================================

    #[test]
    fn scrape_batch_produces_updated_rows_for_new_urls() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec![],
            new_urls: vec!["https://new.com".to_string()],
            deleted: vec![],
        };
        let outputs = make_scrape_outputs(&["https://new.com"]);
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("should succeed for valid new URLs");
        assert_eq!(changes.updated_urls.len(), 1);
        assert_eq!(changes.updated_urls[0].0, "https://new.com");
        assert_eq!(changes.new_scrapes.len(), 1);
        assert!(changes.deleted_urls.is_empty());
    }

    // ===================================================================
    // B03: Payload blobs are produced in new_scrapes
    // ===================================================================

    #[test]
    fn scrape_batch_produces_payload_blobs_for_changed_and_new_urls() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec!["https://a.com".to_string()],
            new_urls: vec!["https://b.com".to_string()],
            deleted: vec![],
        };
        let mut artifacts = HashMap::new();
        artifacts.insert(
            "https://a.com".to_string(),
            ScrapeArtifact {
                content_hash: [0x01; 32],
                status_code: 200,
                payload_bytes: b"serialized_page_1".to_vec(),
            },
        );
        artifacts.insert(
            "https://b.com".to_string(),
            ScrapeArtifact {
                content_hash: [0x02; 32],
                status_code: 200,
                payload_bytes: b"serialized_page_2".to_vec(),
            },
        );
        let outputs = ScrapeOutputs { artifacts };
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("should succeed");
        assert_eq!(changes.new_scrapes.len(), 2);
        assert_eq!(changes.new_scrapes[0].1, b"serialized_page_1");
        assert_eq!(changes.new_scrapes[1].1, b"serialized_page_2");
    }

    // ===================================================================
    // B04: Deleted URLs produce only delete entries
    // ===================================================================

    #[test]
    fn scrape_batch_produces_only_delete_entries_for_deleted_urls() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec![],
            new_urls: vec![],
            deleted: vec![
                "https://old1.com".to_string(),
                "https://old2.com".to_string(),
                "https://old3.com".to_string(),
            ],
        };
        let outputs = ScrapeOutputs {
            artifacts: HashMap::new(),
        };
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("should succeed for deleted-only diff");
        assert_eq!(changes.deleted_urls.len(), 3);
        assert!(changes
            .deleted_urls
            .contains(&"https://old1.com".to_string()));
        assert!(changes
            .deleted_urls
            .contains(&"https://old2.com".to_string()));
        assert!(changes
            .deleted_urls
            .contains(&"https://old3.com".to_string()));
        assert!(changes.updated_urls.is_empty());
        assert!(changes.new_scrapes.is_empty());
    }

    // ===================================================================
    // B05: Unchanged URLs produce no output
    // ===================================================================

    #[test]
    fn scrape_batch_excludes_unchanged_urls_from_all_outputs() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![
                "https://u1.com".to_string(),
                "https://u2.com".to_string(),
                "https://u3.com".to_string(),
            ],
            changed: vec!["https://c.com".to_string()],
            new_urls: vec![],
            deleted: vec![],
        };
        let outputs = make_scrape_outputs(&["https://c.com"]);
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("should succeed");
        assert_eq!(changes.updated_urls.len(), 1);
        assert_eq!(changes.updated_urls[0].0, "https://c.com");

        // No unchanged URL appears in any output vec
        for (url, _) in &changes.updated_urls {
            assert_ne!(url, "https://u1.com");
            assert_ne!(url, "https://u2.com");
            assert_ne!(url, "https://u3.com");
        }
        assert!(!changes.deleted_urls.contains(&"https://u1.com".to_string()));
        assert!(!changes.deleted_urls.contains(&"https://u2.com".to_string()));
        assert!(!changes.deleted_urls.contains(&"https://u3.com".to_string()));
        assert_eq!(changes.new_scrapes.len(), 1);
    }

    // ===================================================================
    // B06: content_hash fidelity
    // ===================================================================

    #[test]
    fn scrape_batch_sets_content_hash_from_artifact() {
        // Given
        let specific_hash: [u8; 32] = [0xAB; 32];
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec!["https://a.com".to_string()],
            new_urls: vec![],
            deleted: vec![],
        };
        let mut artifacts = HashMap::new();
        artifacts.insert(
            "https://a.com".to_string(),
            make_artifact_with_content_hash(specific_hash, b"payload"),
        );
        let outputs = ScrapeOutputs { artifacts };
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("should succeed");
        assert_eq!(changes.updated_urls[0].1.content_hash, [0xAB; 32]);
    }

    // ===================================================================
    // B07: url_hash equals hash_payload of payload_bytes
    // ===================================================================

    #[test]
    fn scrape_batch_sets_url_hash_to_hash_of_payload_bytes() {
        // Given
        let payload = b"test_payload";
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec!["https://a.com".to_string()],
            new_urls: vec![],
            deleted: vec![],
        };
        let mut artifacts = HashMap::new();
        artifacts.insert(
            "https://a.com".to_string(),
            ScrapeArtifact {
                content_hash: [0x01; 32],
                status_code: 200,
                payload_bytes: payload.to_vec(),
            },
        );
        let outputs = ScrapeOutputs { artifacts };
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("should succeed");
        let expected_hash = hash_payload(payload);
        assert_eq!(changes.updated_urls[0].1.url_hash, expected_hash);
        assert_eq!(changes.new_scrapes[0].0, expected_hash);
    }

    // ===================================================================
    // B08: timestamp fidelity
    // ===================================================================

    #[test]
    fn scrape_batch_sets_last_fetched_secs_from_config() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec!["https://a.com".to_string()],
            new_urls: vec![],
            deleted: vec![],
        };
        let outputs = make_scrape_outputs(&["https://a.com"]);
        let config = make_config(1_712_345_678);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("should succeed");
        assert_eq!(changes.updated_urls[0].1.last_fetched_secs, 1_712_345_678);
    }

    // ===================================================================
    // B09: status_code fidelity
    // ===================================================================

    #[test]
    fn scrape_batch_sets_status_code_from_artifact() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec!["https://a.com".to_string()],
            new_urls: vec![],
            deleted: vec![],
        };
        let mut artifacts = HashMap::new();
        artifacts.insert(
            "https://a.com".to_string(),
            make_artifact_with_status(301, b"payload"),
        );
        let outputs = ScrapeOutputs { artifacts };
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("should succeed");
        assert_eq!(changes.updated_urls[0].1.status_code, 301);
    }

    // ===================================================================
    // B10: reserved field is zeroed
    // ===================================================================

    #[test]
    fn scrape_batch_zeroes_reserved_field_in_url_state_raw() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec!["https://a.com".to_string()],
            new_urls: vec![],
            deleted: vec![],
        };
        let outputs = make_scrape_outputs(&["https://a.com"]);
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("should succeed");
        assert_eq!(changes.updated_urls[0].1.reserved, [0u8; 46]);
    }

    // ===================================================================
    // B11: Non-URL fields are empty
    // ===================================================================

    #[test]
    fn scrape_batch_leaves_file_state_fields_empty() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec!["https://a.com".to_string()],
            new_urls: vec![],
            deleted: vec!["https://b.com".to_string()],
        };
        let outputs = make_scrape_outputs(&["https://a.com"]);
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("should succeed");
        assert!(changes.updated_files.is_empty());
        assert!(changes.deleted_files.is_empty());
        assert!(changes.new_analyses.is_empty());
        assert!(changes.new_transforms.is_empty());
        assert!(changes.new_chunks.is_empty());
        assert!(changes.new_snapshots.is_empty());
        assert!(changes.deleted_snapshots.is_empty());
    }

    // ===================================================================
    // B12: Determinism
    // ===================================================================

    #[test]
    fn scrape_batch_produces_identical_output_for_identical_inputs() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec!["https://a.com".to_string(), "https://b.com".to_string()],
            new_urls: vec!["https://c.com".to_string()],
            deleted: vec!["https://d.com".to_string()],
        };
        let outputs = make_scrape_outputs(&["https://a.com", "https://b.com", "https://c.com"]);
        let config = make_config(1_700_000_000);

        // When
        let result1 = build_scrape_state_changes(&diff, &outputs, &config);
        let result2 = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes1 = result1.expect("first call should succeed");
        let changes2 = result2.expect("second call should succeed");
        assert_eq!(changes1.updated_urls, changes2.updated_urls);
        assert_eq!(changes1.deleted_urls, changes2.deleted_urls);
        assert_eq!(changes1.new_scrapes, changes2.new_scrapes);
        assert_eq!(changes1.updated_files, changes2.updated_files);
    }

    // ===================================================================
    // B13: Reference integrity
    // ===================================================================

    #[test]
    fn scrape_batch_url_hash_appears_as_key_in_new_scrapes() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec!["https://a.com".to_string()],
            new_urls: vec!["https://b.com".to_string()],
            deleted: vec![],
        };
        let mut artifacts = HashMap::new();
        artifacts.insert(
            "https://a.com".to_string(),
            ScrapeArtifact {
                content_hash: [0x01; 32],
                status_code: 200,
                payload_bytes: b"payload_a".to_vec(),
            },
        );
        artifacts.insert(
            "https://b.com".to_string(),
            ScrapeArtifact {
                content_hash: [0x02; 32],
                status_code: 200,
                payload_bytes: b"payload_b".to_vec(),
            },
        );
        let outputs = ScrapeOutputs { artifacts };
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("should succeed");
        let expected_hash_a = hash_payload(b"payload_a");
        let expected_hash_b = hash_payload(b"payload_b");

        let scrape_keys: Vec<&[u8; 32]> = changes.new_scrapes.iter().map(|(k, _)| k).collect();
        assert!(scrape_keys.contains(&&expected_hash_a));
        assert!(scrape_keys.contains(&&expected_hash_b));
        assert_eq!(changes.new_scrapes.len(), 2);

        let state_a = changes
            .updated_urls
            .iter()
            .find(|(u, _)| u == "https://a.com")
            .expect("should find a.com")
            .1;
        assert_eq!(state_a.url_hash, expected_hash_a);

        let state_b = changes
            .updated_urls
            .iter()
            .find(|(u, _)| u == "https://b.com")
            .expect("should find b.com")
            .1;
        assert_eq!(state_b.url_hash, expected_hash_b);
    }

    // ===================================================================
    // B14: Output ordering (changed then new_urls then deleted)
    // ===================================================================

    #[test]
    fn scrape_batch_maintains_changed_then_new_then_deleted_ordering() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec!["https://c1.com".to_string()],
            new_urls: vec!["https://n1.com".to_string()],
            deleted: vec!["https://d1.com".to_string()],
        };
        let outputs = make_scrape_outputs(&["https://c1.com", "https://n1.com"]);
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("should succeed");
        assert_eq!(changes.updated_urls[0].0, "https://c1.com");
        assert_eq!(changes.updated_urls[1].0, "https://n1.com");
        assert_eq!(changes.deleted_urls[0], "https://d1.com");
    }

    // ===================================================================
    // B15: EmptyDiff error
    // ===================================================================

    #[test]
    fn scrape_batch_returns_empty_diff_error_when_all_categories_empty() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec![],
            new_urls: vec![],
            deleted: vec![],
        };
        let outputs = ScrapeOutputs {
            artifacts: HashMap::new(),
        };
        let config = make_config(0);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        match result {
            Err(ScrapeBatchBuildError::EmptyDiff) => {}
            other => panic!("expected EmptyDiff, got: {other:?}"),
        }
    }

    // ===================================================================
    // B15b: Only unchanged is not empty diff
    // ===================================================================

    #[test]
    fn scrape_batch_returns_empty_ok_when_only_unchanged_urls_present() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec!["https://u1.com".to_string()],
            changed: vec![],
            new_urls: vec![],
            deleted: vec![],
        };
        let outputs = ScrapeOutputs {
            artifacts: HashMap::new(),
        };
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("unchanged-only diff should return Ok");
        assert!(changes.updated_urls.is_empty());
        assert!(changes.deleted_urls.is_empty());
        assert!(changes.new_scrapes.is_empty());
        assert!(changes.updated_files.is_empty());
        assert!(changes.deleted_files.is_empty());
        assert!(changes.new_analyses.is_empty());
        assert!(changes.new_transforms.is_empty());
        assert!(changes.new_chunks.is_empty());
        assert!(changes.new_snapshots.is_empty());
        assert!(changes.deleted_snapshots.is_empty());
    }

    // ===================================================================
    // B16a: DuplicateUrl (changed + new_urls)
    // ===================================================================

    #[test]
    fn scrape_batch_returns_duplicate_url_when_in_changed_and_new() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec!["https://dup.com".to_string()],
            new_urls: vec!["https://dup.com".to_string()],
            deleted: vec![],
        };
        let outputs = make_scrape_outputs(&["https://dup.com"]);
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        match result {
            Err(ScrapeBatchBuildError::DuplicateUrl { ref url }) => {
                assert_eq!(url, "https://dup.com");
            }
            other => panic!("expected DuplicateUrl, got: {other:?}"),
        }
    }

    // ===================================================================
    // B16b: DuplicateUrl (unchanged + changed)
    // ===================================================================

    #[test]
    fn scrape_batch_returns_duplicate_url_when_in_unchanged_and_changed() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec!["https://dup.com".to_string()],
            changed: vec!["https://dup.com".to_string()],
            new_urls: vec![],
            deleted: vec![],
        };
        let outputs = make_scrape_outputs(&["https://dup.com"]);
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        match result {
            Err(ScrapeBatchBuildError::DuplicateUrl { ref url }) => {
                assert_eq!(url, "https://dup.com");
            }
            other => panic!("expected DuplicateUrl, got: {other:?}"),
        }
    }

    // ===================================================================
    // B16c: DuplicateUrl (unchanged + new_urls)
    // ===================================================================

    #[test]
    fn scrape_batch_returns_duplicate_url_when_in_unchanged_and_new() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec!["https://dup.com".to_string()],
            changed: vec![],
            new_urls: vec!["https://dup.com".to_string()],
            deleted: vec![],
        };
        let outputs = make_scrape_outputs(&["https://dup.com"]);
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        match result {
            Err(ScrapeBatchBuildError::DuplicateUrl { ref url }) => {
                assert_eq!(url, "https://dup.com");
            }
            other => panic!("expected DuplicateUrl, got: {other:?}"),
        }
    }

    // ===================================================================
    // B16d: DuplicateUrl (unchanged + deleted)
    // ===================================================================

    #[test]
    fn scrape_batch_returns_duplicate_url_when_in_unchanged_and_deleted() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec!["https://dup.com".to_string()],
            changed: vec![],
            new_urls: vec![],
            deleted: vec!["https://dup.com".to_string()],
        };
        let outputs = ScrapeOutputs {
            artifacts: HashMap::new(),
        };
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        match result {
            Err(ScrapeBatchBuildError::DuplicateUrl { ref url }) => {
                assert_eq!(url, "https://dup.com");
            }
            other => panic!("expected DuplicateUrl, got: {other:?}"),
        }
    }

    // ===================================================================
    // B16e: DuplicateUrl (changed + deleted)
    // ===================================================================

    #[test]
    fn scrape_batch_returns_duplicate_url_when_in_changed_and_deleted() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec!["https://dup.com".to_string()],
            new_urls: vec![],
            deleted: vec!["https://dup.com".to_string()],
        };
        let outputs = make_scrape_outputs(&["https://dup.com"]);
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        match result {
            Err(ScrapeBatchBuildError::DuplicateUrl { ref url }) => {
                assert_eq!(url, "https://dup.com");
            }
            other => panic!("expected DuplicateUrl, got: {other:?}"),
        }
    }

    // ===================================================================
    // B16f: DuplicateUrl (new_urls + deleted)
    // ===================================================================

    #[test]
    fn scrape_batch_returns_duplicate_url_when_in_new_and_deleted() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec![],
            new_urls: vec!["https://dup.com".to_string()],
            deleted: vec!["https://dup.com".to_string()],
        };
        let outputs = make_scrape_outputs(&["https://dup.com"]);
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        match result {
            Err(ScrapeBatchBuildError::DuplicateUrl { ref url }) => {
                assert_eq!(url, "https://dup.com");
            }
            other => panic!("expected DuplicateUrl, got: {other:?}"),
        }
    }

    // ===================================================================
    // B17: MissingScrapeArtifact for changed URL
    // ===================================================================

    #[test]
    fn scrape_batch_returns_missing_artifact_when_changed_url_has_no_artifact() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec!["https://missing.com".to_string()],
            new_urls: vec![],
            deleted: vec![],
        };
        let outputs = ScrapeOutputs {
            artifacts: HashMap::new(),
        };
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        match result {
            Err(ScrapeBatchBuildError::MissingScrapeArtifact { ref url }) => {
                assert_eq!(url, "https://missing.com");
            }
            other => panic!("expected MissingScrapeArtifact, got: {other:?}"),
        }
    }

    // ===================================================================
    // B18: MissingScrapeArtifact for new URL
    // ===================================================================

    #[test]
    fn scrape_batch_returns_missing_artifact_when_new_url_has_no_artifact() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec![],
            new_urls: vec!["https://missing.com".to_string()],
            deleted: vec![],
        };
        let outputs = ScrapeOutputs {
            artifacts: HashMap::new(),
        };
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        match result {
            Err(ScrapeBatchBuildError::MissingScrapeArtifact { ref url }) => {
                assert_eq!(url, "https://missing.com");
            }
            other => panic!("expected MissingScrapeArtifact, got: {other:?}"),
        }
    }

    // ===================================================================
    // B19: EmptyScrapePayload for changed URL
    // ===================================================================

    #[test]
    fn scrape_batch_returns_empty_payload_when_changed_url_artifact_has_zero_bytes() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec!["https://empty.com".to_string()],
            new_urls: vec![],
            deleted: vec![],
        };
        let mut artifacts = HashMap::new();
        artifacts.insert(
            "https://empty.com".to_string(),
            ScrapeArtifact {
                content_hash: [0x01; 32],
                status_code: 200,
                payload_bytes: vec![],
            },
        );
        let outputs = ScrapeOutputs { artifacts };
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        match result {
            Err(ScrapeBatchBuildError::EmptyScrapePayload { ref url }) => {
                assert_eq!(url, "https://empty.com");
            }
            other => panic!("expected EmptyScrapePayload, got: {other:?}"),
        }
    }

    // ===================================================================
    // B20: EmptyScrapePayload for new URL
    // ===================================================================

    #[test]
    fn scrape_batch_returns_empty_payload_when_new_url_artifact_has_zero_bytes() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec![],
            new_urls: vec!["https://empty.com".to_string()],
            deleted: vec![],
        };
        let mut artifacts = HashMap::new();
        artifacts.insert(
            "https://empty.com".to_string(),
            ScrapeArtifact {
                content_hash: [0x01; 32],
                status_code: 200,
                payload_bytes: vec![],
            },
        );
        let outputs = ScrapeOutputs { artifacts };
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        match result {
            Err(ScrapeBatchBuildError::EmptyScrapePayload { ref url }) => {
                assert_eq!(url, "https://empty.com");
            }
            other => panic!("expected EmptyScrapePayload, got: {other:?}"),
        }
    }

    // ===================================================================
    // B21: build_url_state_raw constructs correctly
    // ===================================================================

    #[test]
    fn build_url_state_raw_sets_all_fields_correctly() {
        // Given
        let content_hash = [0xAA; 32];
        let url_hash = [0xBB; 32];
        let last_fetched_secs = 1_700_000_000u64;
        let status_code: u16 = 200;

        // When
        let result = build_url_state_raw(content_hash, url_hash, last_fetched_secs, status_code);

        // Then — all fields must match inputs, NOT zeroed stub values
        assert_eq!(result.content_hash, [0xAA; 32], "content_hash must be 0xAA");
        assert_eq!(result.url_hash, [0xBB; 32], "url_hash must be 0xBB");
        assert_eq!(
            result.last_fetched_secs, 1_700_000_000,
            "last_fetched_secs must be 1.7B"
        );
        assert_eq!(result.status_code, 200, "status_code must be 200");
        assert_eq!(std::mem::size_of::<UrlStateRaw>(), 120);
    }

    // ===================================================================
    // B22: build_url_state_raw zeroes reserved
    // ===================================================================

    #[test]
    fn build_url_state_raw_zeroes_reserved_field() {
        // Given — non-zero inputs for all other fields to prove reserved is independently zeroed
        let content_hash = [0x11; 32];
        let url_hash = [0x22; 32];

        // When
        let result = build_url_state_raw(content_hash, url_hash, 42, 0);

        // Then — reserved must be zeroed EVEN THOUGH other fields are non-zero
        assert_eq!(result.reserved, [0u8; 46]);
        // Also verify non-zero fields are set (not just relying on zeroed stub)
        assert_eq!(result.content_hash, [0x11; 32]);
        assert_eq!(result.url_hash, [0x22; 32]);
    }

    // ===================================================================
    // B23: PayloadProcessingFailed (fallback: variant constructable + Display)
    // ===================================================================

    #[test]
    fn payload_processing_failed_displays_url_and_reason() {
        // Given
        let error = ScrapeBatchBuildError::PayloadProcessingFailed {
            url: "https://fail.com".to_string(),
            reason: "hash function returned error".to_string(),
        };

        // When
        let display = format!("{error}");

        // Then
        assert!(
            display.contains("https://fail.com"),
            "display must contain URL: {display}"
        );
        assert!(
            display.contains("hash function returned error"),
            "display must contain reason: {display}"
        );
    }

    // ===================================================================
    // MIX: Mixed diff categories
    // ===================================================================

    #[test]
    fn scrape_batch_handles_mixed_diff_categories_correctly() {
        // Given: 2 unchanged, 3 changed, 1 new, 2 deleted
        let diff = ScrapeDiff {
            unchanged: vec!["https://u1.com".to_string(), "https://u2.com".to_string()],
            changed: vec![
                "https://c1.com".to_string(),
                "https://c2.com".to_string(),
                "https://c3.com".to_string(),
            ],
            new_urls: vec!["https://n1.com".to_string()],
            deleted: vec!["https://d1.com".to_string(), "https://d2.com".to_string()],
        };
        let outputs = make_scrape_outputs(&[
            "https://c1.com",
            "https://c2.com",
            "https://c3.com",
            "https://n1.com",
        ]);
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("should succeed for mixed diff");
        assert_eq!(changes.updated_urls.len(), 4); // 3 changed + 1 new
        assert_eq!(changes.deleted_urls.len(), 2);
        assert_eq!(changes.new_scrapes.len(), 4);

        // No unchanged URL in updated_urls
        for (url, _) in &changes.updated_urls {
            assert_ne!(url, "https://u1.com");
            assert_ne!(url, "https://u2.com");
        }
        assert!(!changes.deleted_urls.contains(&"https://u1.com".to_string()));
        assert!(!changes.deleted_urls.contains(&"https://u2.com".to_string()));
    }

    // ===================================================================
    // B23-alt: Single changed URL with one-byte payload
    // ===================================================================

    #[test]
    fn scrape_batch_handles_single_changed_url_with_one_byte_payload() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec!["https://single.com".to_string()],
            new_urls: vec![],
            deleted: vec![],
        };
        let mut artifacts = HashMap::new();
        artifacts.insert(
            "https://single.com".to_string(),
            ScrapeArtifact {
                content_hash: [0x01; 32],
                status_code: 200,
                payload_bytes: vec![0x42],
            },
        );
        let outputs = ScrapeOutputs { artifacts };
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("should succeed");
        assert_eq!(changes.updated_urls.len(), 1);
        assert_eq!(changes.new_scrapes.len(), 1);
        assert_eq!(changes.new_scrapes[0].1, vec![0x42]);
    }

    // ===================================================================
    // B23-alt: Deleted-only diff produces correct StateChanges
    // ===================================================================

    #[test]
    fn scrape_batch_deleted_only_produces_correct_state_changes() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec![],
            new_urls: vec![],
            deleted: vec!["https://gone.com".to_string()],
        };
        let outputs = ScrapeOutputs {
            artifacts: HashMap::new(),
        };
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("should succeed");
        assert_eq!(changes.updated_urls.len(), 0);
        assert_eq!(changes.deleted_urls.len(), 1);
        assert_eq!(changes.new_scrapes.len(), 0);
        assert_eq!(changes.deleted_urls[0], "https://gone.com");
    }

    // ===================================================================
    // B23-alt: Timestamp zero is valid
    // ===================================================================

    #[test]
    fn scrape_batch_accepts_zero_timestamp_in_config() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec!["https://a.com".to_string()],
            new_urls: vec![],
            deleted: vec![],
        };
        let outputs = make_scrape_outputs(&["https://a.com"]);
        let config = make_config(0);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("zero timestamp should be valid");
        assert_eq!(changes.updated_urls[0].1.last_fetched_secs, 0);
    }

    // ===================================================================
    // B23-alt: Max u64 timestamp
    // ===================================================================

    #[test]
    fn scrape_batch_accepts_max_u64_timestamp_in_config() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec!["https://a.com".to_string()],
            new_urls: vec![],
            deleted: vec![],
        };
        let outputs = make_scrape_outputs(&["https://a.com"]);
        let config = make_config(u64::MAX);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("max u64 timestamp should be valid");
        assert_eq!(changes.updated_urls[0].1.last_fetched_secs, u64::MAX);
    }

    // ===================================================================
    // B23-alt: Status code 0 is valid
    // ===================================================================

    #[test]
    fn scrape_batch_accepts_status_code_zero() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec!["https://a.com".to_string()],
            new_urls: vec![],
            deleted: vec![],
        };
        let mut artifacts = HashMap::new();
        artifacts.insert(
            "https://a.com".to_string(),
            ScrapeArtifact {
                content_hash: [0x01; 32],
                status_code: 0,
                payload_bytes: b"payload".to_vec(),
            },
        );
        let outputs = ScrapeOutputs { artifacts };
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("status code 0 should be valid");
        assert_eq!(changes.updated_urls[0].1.status_code, 0);
    }

    // ===================================================================
    // B23-alt: Status code 599 is valid
    // ===================================================================

    #[test]
    fn scrape_batch_accepts_status_code_599() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec!["https://a.com".to_string()],
            new_urls: vec![],
            deleted: vec![],
        };
        let mut artifacts = HashMap::new();
        artifacts.insert(
            "https://a.com".to_string(),
            ScrapeArtifact {
                content_hash: [0x01; 32],
                status_code: 599,
                payload_bytes: b"payload".to_vec(),
            },
        );
        let outputs = ScrapeOutputs { artifacts };
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("status code 599 should be valid");
        assert_eq!(changes.updated_urls[0].1.status_code, 599);
    }

    // ===================================================================
    // B23-alt: All-zero content hash is valid
    // ===================================================================

    #[test]
    fn scrape_batch_accepts_all_zero_content_hash() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec!["https://a.com".to_string()],
            new_urls: vec![],
            deleted: vec![],
        };
        let mut artifacts = HashMap::new();
        artifacts.insert(
            "https://a.com".to_string(),
            ScrapeArtifact {
                content_hash: [0u8; 32],
                status_code: 200,
                payload_bytes: b"payload".to_vec(),
            },
        );
        let outputs = ScrapeOutputs { artifacts };
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("zero content hash should be valid");
        assert_eq!(changes.updated_urls[0].1.content_hash, [0u8; 32]);
    }

    // ===================================================================
    // B23-alt: Multiple changed URLs produce correct new_scrapes entries
    // ===================================================================

    #[test]
    fn scrape_batch_multiple_changed_urls_produce_correct_new_scrapes() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec![
                "https://x.com".to_string(),
                "https://y.com".to_string(),
                "https://z.com".to_string(),
            ],
            new_urls: vec![],
            deleted: vec![],
        };
        let outputs = make_scrape_outputs(&["https://x.com", "https://y.com", "https://z.com"]);
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("should succeed");
        assert_eq!(changes.new_scrapes.len(), 3);

        // Each payload hash should be non-zero
        for (hash, _payload) in &changes.new_scrapes {
            assert_ne!(*hash, [0u8; 32], "scrape payload hash must be non-zero");
        }
    }

    // ===================================================================
    // B23-alt: Only new_urls with no changed produces correct output
    // ===================================================================

    #[test]
    fn scrape_batch_new_urls_only_produces_correct_output() {
        // Given
        let diff = ScrapeDiff {
            unchanged: vec![],
            changed: vec![],
            new_urls: vec![
                "https://new1.com".to_string(),
                "https://new2.com".to_string(),
            ],
            deleted: vec![],
        };
        let outputs = make_scrape_outputs(&["https://new1.com", "https://new2.com"]);
        let config = make_config(1_700_000_000);

        // When
        let result = build_scrape_state_changes(&diff, &outputs, &config);

        // Then
        let changes = result.expect("should succeed");
        assert_eq!(changes.updated_urls.len(), 2);
        assert_eq!(changes.new_scrapes.len(), 2);
        assert!(changes.deleted_urls.is_empty());
    }

    // ===================================================================
    // Error Display tests for all variants
    // ===================================================================

    #[test]
    fn missing_scrape_artifact_error_displays_url() {
        let error = ScrapeBatchBuildError::MissingScrapeArtifact {
            url: "https://missing.com".to_string(),
        };
        let display = format!("{error}");
        assert!(display.contains("https://missing.com"));
    }

    #[test]
    fn empty_scrape_payload_error_displays_url() {
        let error = ScrapeBatchBuildError::EmptyScrapePayload {
            url: "https://empty.com".to_string(),
        };
        let display = format!("{error}");
        assert!(display.contains("https://empty.com"));
    }

    #[test]
    fn duplicate_url_error_displays_url() {
        let error = ScrapeBatchBuildError::DuplicateUrl {
            url: "https://dup.com".to_string(),
        };
        let display = format!("{error}");
        assert!(display.contains("https://dup.com"));
    }

    #[test]
    fn empty_diff_error_displays_message() {
        let error = ScrapeBatchBuildError::EmptyDiff;
        let display = format!("{error}");
        assert!(display.contains("empty"));
    }

    // ===================================================================
    // build_url_state_raw: size assertion
    // ===================================================================

    #[test]
    fn build_url_state_raw_output_is_120_bytes() {
        let result = build_url_state_raw([0u8; 32], [0u8; 32], 0, 0);
        let bytes = result.to_bytes();
        assert_eq!(bytes.len(), 120);
    }

    // ===================================================================
    // build_url_state_raw: byte round-trip
    // ===================================================================

    #[test]
    fn build_url_state_raw_roundtrips_through_bytes() {
        // Use non-zero values to ensure the stub isn't accidentally passing
        let original = build_url_state_raw([0xAA; 32], [0xBB; 32], 1_700_000_000, 301);
        let bytes = original.to_bytes();
        assert_eq!(bytes.len(), 120);
        let restored = UrlStateRaw::from_bytes(&bytes).expect("from_bytes should succeed");
        assert_eq!(restored.content_hash, [0xAA; 32]);
        assert_eq!(restored.url_hash, [0xBB; 32]);
        assert_eq!(restored.last_fetched_secs, 1_700_000_000);
        assert_eq!(restored.status_code, 301);
        assert_eq!(restored.reserved, [0u8; 46]);
    }

    // ===================================================================
    // Proptest P1: Reference integrity for arbitrary valid inputs
    // ===================================================================

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 1000,
            ..ProptestConfig::default()
        })]

        #[test]
        fn proptest_reference_integrity_for_valid_inputs(
            changed_urls in proptest::collection::vec(
                proptest::string::string_regex("https://[a-z]+\\.com/[a-z]+").unwrap(),
                0..5
            ),
            new_urls in proptest::collection::vec(
                proptest::string::string_regex("https://[a-z]+\\.com/[a-z]+").unwrap(),
                0..5
            ),
        ) {
            // Skip empty diff (that's a precondition error)
            prop_assume!(!changed_urls.is_empty() || !new_urls.is_empty());

            let diff = ScrapeDiff {
                unchanged: vec![],
                changed: changed_urls.clone(),
                new_urls: new_urls.clone(),
                deleted: vec![],
            };

            let mut artifacts = HashMap::new();
            for url in changed_urls.iter().chain(new_urls.iter()) {
                artifacts.insert(url.clone(), ScrapeArtifact {
                    content_hash: [0x42; 32],
                    status_code: 200,
                    payload_bytes: url.as_bytes().to_vec(),
                });
            }
            let outputs = ScrapeOutputs { artifacts };
            let config = make_config(1_700_000_000);

            let result = build_scrape_state_changes(&diff, &outputs, &config);
            let changes = result.expect("should succeed");

            // INV: every url_hash in updated_urls is a key in new_scrapes
            let scrape_keys: std::collections::HashSet<[u8; 32]> =
                changes.new_scrapes.iter().map(|(k, _)| *k).collect();

            for (_, state) in &changes.updated_urls {
                prop_assert!(
                    scrape_keys.contains(&state.url_hash),
                    "url_hash {:?} not found in new_scrapes keys",
                    state.url_hash
                );
            }
        }
    }

    // ===================================================================
    // Proptest P2: One-to-one URL-to-row mapping
    // ===================================================================

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 1000,
            ..ProptestConfig::default()
        })]

        #[test]
        fn proptest_one_to_one_url_mapping(
            changed_count in 0usize..10,
            new_count in 0usize..10,
        ) {
            prop_assume!(changed_count > 0 || new_count > 0);

            let changed_urls: Vec<String> = (0..changed_count)
                .map(|i| format!("https://changed{i}.com/page"))
                .collect();
            let new_urls: Vec<String> = (0..new_count)
                .map(|i| format!("https://new{i}.com/page"))
                .collect();

            let diff = ScrapeDiff {
                unchanged: vec![],
                changed: changed_urls.clone(),
                new_urls: new_urls.clone(),
                deleted: vec![],
            };

            let mut artifacts = HashMap::new();
            for url in changed_urls.iter().chain(new_urls.iter()) {
                artifacts.insert(url.clone(), ScrapeArtifact {
                    content_hash: [0x01; 32],
                    status_code: 200,
                    payload_bytes: url.as_bytes().to_vec(),
                });
            }
            let outputs = ScrapeOutputs { artifacts };
            let config = make_config(1_700_000_000);

            let result = build_scrape_state_changes(&diff, &outputs, &config);
            let changes = result.expect("should succeed");

            prop_assert_eq!(
                changes.updated_urls.len(),
                changed_count + new_count,
                "updated_urls count must equal changed + new_urls count"
            );
        }
    }

    // ===================================================================
    // Proptest P3: Determinism under permutation-invariant inputs
    // ===================================================================

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 1000,
            ..ProptestConfig::default()
        })]

        #[test]
        fn proptest_determinism(
            changed_urls in proptest::collection::vec(
                proptest::string::string_regex("https://[a-z]+\\.com/[a-z]+").unwrap(),
                1..5
            ),
        ) {
            let diff = ScrapeDiff {
                unchanged: vec![],
                changed: changed_urls.clone(),
                new_urls: vec![],
                deleted: vec![],
            };

            let mut artifacts = HashMap::new();
            for url in &changed_urls {
                artifacts.insert(url.clone(), ScrapeArtifact {
                    content_hash: [0x01; 32],
                    status_code: 200,
                    payload_bytes: url.as_bytes().to_vec(),
                });
            }
            let outputs = ScrapeOutputs { artifacts };
            let config = make_config(1_700_000_000);

            let result1 = build_scrape_state_changes(&diff, &outputs, &config);
            let result2 = build_scrape_state_changes(&diff, &outputs, &config);

            let changes1 = result1.expect("first call should succeed");
            let changes2 = result2.expect("second call should succeed");

            prop_assert_eq!(changes1.updated_urls, changes2.updated_urls);
            prop_assert_eq!(changes1.new_scrapes, changes2.new_scrapes);
        }
    }

    // ===================================================================
    // Proptest P4: build_url_state_raw round-trip through bytes
    // ===================================================================

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 1000,
            ..ProptestConfig::default()
        })]

        #[test]
        fn proptest_url_state_raw_roundtrip(
            content_hash in proptest::array::uniform32(proptest::num::u8::ANY),
            url_hash in proptest::array::uniform32(proptest::num::u8::ANY),
            last_fetched_secs in proptest::num::u64::ANY,
            status_code in proptest::num::u16::ANY,
        ) {
            let original = build_url_state_raw(content_hash, url_hash, last_fetched_secs, status_code);

            // Assert fields match inputs (NOT zeroed stub)
            prop_assert_eq!(original.content_hash, content_hash);
            prop_assert_eq!(original.url_hash, url_hash);
            prop_assert_eq!(original.last_fetched_secs, last_fetched_secs);
            prop_assert_eq!(original.status_code, status_code);
            prop_assert_eq!(original.reserved, [0u8; 46]);

            // Also verify byte round-trip
            let bytes = original.to_bytes();
            prop_assert_eq!(bytes.len(), 120);
            let restored = UrlStateRaw::from_bytes(&bytes).expect("from_bytes should succeed");
            prop_assert_eq!(restored.content_hash, content_hash);
        }
    }
}
