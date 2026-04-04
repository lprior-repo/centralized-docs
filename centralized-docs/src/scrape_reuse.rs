//! Implementation for bead cdocs-90e: Load archived scrape outputs for unchanged pages.
//!
//! Data flow:
//! ```text
//! Input:  fresh_result (ScrapeResult), session (StateReadSession)
//!             |
//!             v
//!     [compute_page_content_hash for each page]
//!             |
//!             v
//!     [[u8; 32]] fresh_hashes
//!             |
//!             v
//!     [session.load_url_states()]
//!             |
//!             v
//!     HashMap<String, UrlStateRaw>
//!             |
//!             v
//!     [classify_scraped_pages]
//!             |
//!             v
//!     ScrapePageDiff { unchanged, changed_or_new }
//!             |
//!             v
//!     [load_archived_scrape_pages]
//!             |
//!        _____|_____
//!       |           |
//!       v           v
//!  archived       changed+new
//!  pages          pages
//!       |           |
//!       v           v
//!   [merge_scrape_pages_in_order]
//!             |
//!             v
//!   (ScrapeResult, ScrapeReuseStats)
//! ```

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![forbid(unsafe_code)]
#![allow(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::uninlined_format_args)]

use std::collections::HashMap;

use crate::persisted::{persisted_scraped_page_to_runtime, PersistedScrapeResult};
use crate::scrape::validation::{ScrapeResult, ScrapedPage};
use crate::state::bulk_load::{BulkLoadError, OwnedArchive, StateReadSession};
use crate::state::{StateLoadError, UrlStateRaw};
use itertools::Itertools;

// ---------------------------------------------------------------------------
// Domain types (Data Layer)
// ---------------------------------------------------------------------------

/// Partition of scraped page indices into unchanged vs changed-or-new.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScrapePageDiff {
    /// Indices into the original pages vec for unchanged pages.
    pub unchanged: Vec<usize>,
    /// Indices for changed or new pages.
    pub changed_or_new: Vec<usize>,
}

/// Statistics about scrape reuse within a single command invocation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScrapeReuseStats {
    /// Number of pages loaded from archived state (zero-cost reuse).
    pub reused: usize,
    /// Number of pages freshly scraped and processed through the pipeline.
    pub scraped: usize,
}

// ---------------------------------------------------------------------------
// Error taxonomy
// ---------------------------------------------------------------------------

/// Error type for the scrape-reuse pipeline.
#[derive(Debug, thiserror::Error)]
pub enum ScrapeReuseError {
    /// Failed to load URL states from the state database.
    #[error("failed to load url states: {0}")]
    StateLoad(#[from] StateLoadError),

    /// Failed to load archived scrape outputs from the state database.
    #[error("failed to load archived scrape outputs: {0}")]
    BulkLoad(#[from] BulkLoadError),

    /// A loaded PersistedScrapeResult failed schema validation or deserialization.
    #[error("failed to deserialize archived scrape output for url_hash {key_hex}: {message}")]
    DeserializationFailed {
        /// Hex-encoded key of the corrupt archive.
        key_hex: String,
        /// Error description from rkyv or schema validation.
        message: String,
    },

    /// Hash integrity violation: loaded page's content_hash does not match
    /// the stored UrlStateRaw.content_hash for the same URL.
    #[error("hash mismatch for '{url}': stored={stored_hex}, loaded={loaded_hex}")]
    HashMismatch {
        /// URL of the affected page.
        url: String,
        /// Hex-encoded stored content hash from UrlStateRaw.
        stored_hex: String,
        /// Hex-encoded content hash from the loaded scrape page.
        loaded_hex: String,
    },

    /// No url_state entry exists for a URL that was expected to be unchanged.
    /// This should not occur if classification is correct; indicates a logic bug.
    #[error("missing url_state for expected-unchanged URL '{url}'")]
    MissingUrlState {
        /// URL with no url_state entry.
        url: String,
    },
}

// ---------------------------------------------------------------------------
// Classification (Pure Calculation)
// ---------------------------------------------------------------------------

/// Compute the content hash of a scraped page's markdown.
/// Pure function: deterministic, no I/O.
pub fn compute_page_content_hash(markdown: &str) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(markdown.as_bytes());
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}

/// Classify each scraped page as Unchanged or ChangedOrNew by comparing
/// its content hash against the stored UrlStateRaw.
///
/// Pure calculation: no I/O, no errors.
///
/// Returns a partition of page indices:
/// - `unchanged`: indices into `fresh_pages` where content hash matches stored state
/// - `changed_or_new`: all other indices (missing state, hash mismatch, zero hash)
///
/// # Panics
///
/// Panics if `fresh_pages.len() != fresh_hashes.len()`.
pub fn classify_scraped_pages(
    fresh_pages: &[ScrapedPage],
    fresh_hashes: &[[u8; 32]],
    url_states: &HashMap<String, UrlStateRaw>,
) -> ScrapePageDiff {
    assert_eq!(
        fresh_pages.len(),
        fresh_hashes.len(),
        "input length mismatch: fresh_pages.len()={} != fresh_hashes.len()={}",
        fresh_pages.len(),
        fresh_hashes.len()
    );

    let mut diff = ScrapePageDiff::default();

    for (i, page) in fresh_pages.iter().enumerate() {
        let fresh_hash = &fresh_hashes[i];

        let state = match url_states.get(&page.url) {
            Some(s) => s,
            None => {
                // INV-7: missing url_state → new page
                diff.changed_or_new.push(i);
                continue;
            }
        };

        // INV-6: zero url_hash means "never archived"
        if state.url_hash == [0u8; 32] {
            diff.changed_or_new.push(i);
            continue;
        }

        // INV-8: hash mismatch means changed
        if state.content_hash != *fresh_hash {
            diff.changed_or_new.push(i);
            continue;
        }

        // Hash matches → unchanged
        diff.unchanged.push(i);
    }

    diff
}

// ---------------------------------------------------------------------------
// Archive Loading (I/O Action)
// ---------------------------------------------------------------------------

/// Load archived scrape outputs for unchanged pages.
///
/// For each unchanged page, looks up its UrlStateRaw.url_hash and loads the
/// corresponding PersistedScrapeResult from the scrape_outputs table.
/// Deserializes individual pages and verifies hash integrity.
///
/// # Arguments
/// * `page_diff` - Classification result with unchanged page indices.
/// * `fresh_pages` - The freshly scraped pages (for URL lookup).
/// * `url_states` - Loaded URL state entries (provides url_hash and content_hash).
/// * `session` - Shared read session for archive access.
///
/// # Returns
/// * `HashMap<usize, ScrapedPage>` - Index into fresh_pages -> loaded archived page
/// * `Vec<usize>` - Page indices that failed to load (fallback to fresh)
pub fn load_archived_scrape_pages(
    page_diff: &ScrapePageDiff,
    fresh_pages: &[ScrapedPage],
    url_states: &HashMap<String, UrlStateRaw>,
    session: &StateReadSession<'_>,
) -> Result<(HashMap<usize, ScrapedPage>, Vec<usize>), ScrapeReuseError> {
    // Early return for empty unchanged
    if page_diff.unchanged.is_empty() {
        return Ok((HashMap::new(), Vec::new()));
    }

    // Collect (index, url, url_hash, content_hash) for unchanged pages
    let indices_and_hashes: Vec<(usize, &str, [u8; 32], [u8; 32])> = page_diff
        .unchanged
        .iter()
        .filter_map(|&idx| {
            let page = fresh_pages.get(idx)?;
            let state = url_states.get(&page.url)?;
            // Zero url_hash → never archived, skip
            if state.url_hash == [0u8; 32] {
                return None;
            }
            Some((idx, page.url.as_str(), state.url_hash, state.content_hash))
        })
        .collect();

    // If all unchanged pages have zero url_hash, all are fallback
    if indices_and_hashes.is_empty() {
        return Ok((HashMap::new(), page_diff.unchanged.clone()));
    }

    // Bulk load all scrape batches (deduplicated url_hashes)
    let url_hashes: Vec<[u8; 32]> = indices_and_hashes
        .iter()
        .map(|(_, _, uh, _)| *uh)
        .unique()
        .collect();

    let archived_batches: HashMap<[u8; 32], OwnedArchive<PersistedScrapeResult>> =
        session.load_scrapes(&url_hashes).map_err(|e| match e {
            BulkLoadError::CorruptPayload {
                table: _,
                key_hex,
                message,
            } => ScrapeReuseError::DeserializationFailed { key_hex, message },
            other => ScrapeReuseError::BulkLoad(other),
        })?;

    // Extract individual pages from batches and verify hash integrity
    let mut loaded_pages: HashMap<usize, ScrapedPage> = HashMap::new();
    let mut fallback_indices: Vec<usize> = Vec::new();

    for &(idx, url, url_hash, stored_content_hash) in &indices_and_hashes {
        match archived_batches.get(&url_hash) {
            None => {
                // No scrape_output row for this url_hash → fallback
                fallback_indices.push(idx);
            }
            Some(archive) => {
                let result = load_single_page_from_batch(archive, url, stored_content_hash);
                match result {
                    Ok(page) => {
                        loaded_pages.insert(idx, page);
                    }
                    Err(_) => {
                        // Deserialization failure or empty batch → fallback
                        fallback_indices.push(idx);
                    }
                }
            }
        }
    }

    Ok((loaded_pages, fallback_indices))
}

// ---------------------------------------------------------------------------
// Helper: load single page from batch (with hash verification)
// ---------------------------------------------------------------------------

/// Try to extract a single `ScrapedPage` from a `PersistedScrapeResult` batch.
///
/// Returns `Ok(page)` if the page is found by URL and passes hash verification.
/// Returns `Err(())` if:
/// - Batch deserialization fails
/// - The batch contains zero pages
/// - No page matches the expected URL
/// - Hash mismatch between loaded page content and stored content hash
fn load_single_page_from_batch(
    archive: &OwnedArchive<PersistedScrapeResult>,
    expected_url: &str,
    stored_content_hash: [u8; 32],
) -> Result<ScrapedPage, ()> {
    let persisted = archive.deserialize().map_err(|_| ())?;

    // Find the page matching the expected URL
    let matched_page = persisted
        .pages
        .iter()
        .find(|p| p.url == expected_url)
        .ok_or(())?;

    // Convert to runtime type
    let runtime_page = persisted_scraped_page_to_runtime(matched_page).map_err(|_| ())?;

    // Verify hash integrity (INV-8)
    let loaded_hash = compute_page_content_hash(&runtime_page.markdown);
    if loaded_hash != stored_content_hash {
        return Err(());
    }

    Ok(runtime_page)
}

// ---------------------------------------------------------------------------
// Merge (Pure Calculation)
// ---------------------------------------------------------------------------

/// Merge reused archived pages and fresh pages into a single vec in crawl order.
///
/// For each position in the original fresh_pages list:
/// - If the index is in `archived_pages`, use the archived version.
/// - Otherwise, use the fresh version.
///
/// Pure calculation: no I/O, no errors.
///
/// # Postconditions
/// - Output vec length == fresh_pages length.
/// - Output order matches fresh_pages order.
pub fn merge_scrape_pages_in_order(
    fresh_pages: Vec<ScrapedPage>,
    archived_pages: HashMap<usize, ScrapedPage>,
) -> Vec<ScrapedPage> {
    fresh_pages
        .into_iter()
        .enumerate()
        .map(|(idx, fresh)| archived_pages.get(&idx).cloned().unwrap_or(fresh))
        .collect()
}

// ---------------------------------------------------------------------------
// Primary Entry Point
// ---------------------------------------------------------------------------

/// Classify scraped pages, load archived outputs for unchanged pages, and
/// merge into a final page list with reuse statistics.
///
/// # Arguments
/// * `fresh_result` - The freshly scraped result from scrape_site.
/// * `session` - Shared read session for state database access.
///
/// # Errors
/// Returns `ScrapeReuseError` for state database failures or archive corruption.
///
/// # Guarantees
/// - Every page in fresh_result appears in the output exactly once.
/// - Output order matches input order.
/// - Unchanged pages are loaded from archive, not re-processed.
pub fn scrape_with_reuse(
    fresh_result: ScrapeResult,
    session: &StateReadSession<'_>,
) -> Result<(ScrapeResult, ScrapeReuseStats), ScrapeReuseError> {
    let total_pages = fresh_result.pages.len();

    // Early return for empty input
    if total_pages == 0 {
        return Ok((
            ScrapeResult {
                pages: Vec::new(),
                total_urls: fresh_result.total_urls,
                success_count: fresh_result.success_count,
                error_count: fresh_result.error_count,
                errors: fresh_result.errors,
                base_url: fresh_result.base_url,
            },
            ScrapeReuseStats::default(),
        ));
    }

    // Step 1: Compute content hashes for each page
    let fresh_hashes: Vec<[u8; 32]> = fresh_result
        .pages
        .iter()
        .map(|p| compute_page_content_hash(&p.markdown))
        .collect();

    // Step 2: Load url_states from the database
    let url_states = session.load_url_states()?;

    // Step 3: Classify pages as unchanged vs changed/new
    let page_diff = classify_scraped_pages(&fresh_result.pages, &fresh_hashes, &url_states);

    // Step 4: Load archived scrape outputs for unchanged pages
    let (archived_pages, _fallback_indices) =
        load_archived_scrape_pages(&page_diff, &fresh_result.pages, &url_states, session)?;

    let reused_count = archived_pages.len();

    // Step 5: Merge archived and fresh pages in order
    let merged_pages = merge_scrape_pages_in_order(fresh_result.pages, archived_pages);

    let scraped_count = total_pages - reused_count;

    let stats = ScrapeReuseStats {
        reused: reused_count,
        scraped: scraped_count,
    };

    Ok((
        ScrapeResult {
            pages: merged_pages,
            total_urls: fresh_result.total_urls,
            success_count: fresh_result.success_count,
            error_count: fresh_result.error_count,
            errors: fresh_result.errors,
            base_url: fresh_result.base_url,
        },
        stats,
    ))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    #![allow(clippy::expect_used)]
    #![allow(clippy::panic)]

    use super::*;
    use crate::scrape::validation::{Header, PageFilterStatus};

    // =======================================================================
    // Helper: make a ScrapedPage
    // =======================================================================

    fn make_page(url: &str, markdown: &str) -> ScrapedPage {
        ScrapedPage {
            url: url.to_string(),
            markdown: markdown.to_string(),
            title: url.to_string(),
            links: Vec::new(),
            headers: vec![Header {
                level: 1,
                text: url.to_string(),
            }],
            word_count: markdown.split_whitespace().count(),
            slug: url.to_string(),
            filter_status: PageFilterStatus::Unfiltered,
            elements_removed: 0,
            density_score: 1.0,
        }
    }

    fn make_url_state(content_hash: [u8; 32], url_hash: [u8; 32]) -> UrlStateRaw {
        UrlStateRaw {
            content_hash,
            url_hash,
            last_fetched_secs: 1_700_000_000,
            status_code: 200,
            reserved: [0u8; 46],
        }
    }

    // =======================================================================
    // Behavior 1: compute_page_content_hash returns SHA-256 for non-empty
    // =======================================================================

    #[test]
    fn compute_page_content_hash_returns_sha256_when_given_nonempty_markdown() {
        let hash = compute_page_content_hash("# Hello\n\nWorld");
        // SHA-256("# Hello\n\nWorld") = ad6e0bf888da964ab57992e86c6f894aaec3325d7b18355ab92c81babe81c4a3
        let expected: [u8; 32] = [
            0xad, 0x6e, 0x0b, 0xf8, 0x88, 0xda, 0x96, 0x4a, 0xb5, 0x79, 0x92, 0xe8, 0x6c, 0x6f,
            0x89, 0x4a, 0xae, 0xc3, 0x32, 0x5d, 0x7b, 0x18, 0x35, 0x5a, 0xb9, 0x2c, 0x81, 0xba,
            0xbe, 0x81, 0xc4, 0xa3,
        ];
        assert_eq!(hash, expected);
        assert_ne!(hash, [0u8; 32]);
    }

    // =======================================================================
    // Behavior 2: compute_page_content_hash returns SHA-256 of empty string
    // =======================================================================

    #[test]
    fn compute_page_content_hash_returns_sha256_of_empty_when_given_empty_string() {
        let hash = compute_page_content_hash("");
        // SHA-256("") = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
        let expected: [u8; 32] = [
            0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f,
            0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b,
            0x78, 0x52, 0xb8, 0x55,
        ];
        assert_eq!(hash, expected);
        assert_ne!(hash, [0u8; 32]);
    }

    // =======================================================================
    // Behavior 3: classify_scraped_pages returns empty partitions when empty
    // =======================================================================

    #[test]
    fn classify_scraped_pages_returns_empty_partitions_when_no_pages_provided() {
        let pages: Vec<ScrapedPage> = Vec::new();
        let hashes: Vec<[u8; 32]> = Vec::new();
        let url_states: HashMap<String, UrlStateRaw> = HashMap::new();

        let diff = classify_scraped_pages(&pages, &hashes, &url_states);

        assert_eq!(diff.unchanged, Vec::<usize>::new());
        assert_eq!(diff.changed_or_new, Vec::<usize>::new());
    }

    // =======================================================================
    // Behavior 4: classify_scraped_pages — unchanged when hash matches
    // =======================================================================

    #[test]
    fn classify_scraped_pages_classifies_unchanged_when_hash_matches_stored() {
        let page = make_page("https://a.com", "hello");
        let content_hash = compute_page_content_hash("hello");
        let url_state = make_url_state(content_hash, [1u8; 32]);

        let mut url_states = HashMap::new();
        url_states.insert("https://a.com".to_string(), url_state);

        let diff = classify_scraped_pages(&[page], &[content_hash], &url_states);

        assert_eq!(diff.unchanged, vec![0]);
        assert_eq!(diff.changed_or_new, Vec::<usize>::new());
    }

    // =======================================================================
    // Behavior 5: classify_scraped_pages — changed when hash mismatches
    // =======================================================================

    #[test]
    fn classify_scraped_pages_classifies_changed_when_hash_mismatches_stored() {
        let page = make_page("https://a.com", "new content");
        let fresh_hash = compute_page_content_hash("new content");
        let stored_hash = compute_page_content_hash("old content");
        let url_state = make_url_state(stored_hash, [1u8; 32]);

        let mut url_states = HashMap::new();
        url_states.insert("https://a.com".to_string(), url_state);

        let diff = classify_scraped_pages(&[page], &[fresh_hash], &url_states);

        assert_eq!(diff.unchanged, Vec::<usize>::new());
        assert_eq!(diff.changed_or_new, vec![0]);
    }

    // =======================================================================
    // Behavior 5b: lexicographic trap (>= vs == mutation)
    // =======================================================================

    #[test]
    fn classify_scraped_pages_classifies_changed_when_stored_hash_is_lexicographically_greater() {
        let page = make_page("https://trap.com", "aaa");
        let fresh_hash = compute_page_content_hash("aaa");
        // [0xFF; 32] is lexicographically greater than any typical SHA-256 output
        // and is NOT equal to fresh_hash. If code uses >= instead of ==,
        // this would incorrectly classify as unchanged.
        let url_state = make_url_state([0xFF; 32], [1u8; 32]);

        let mut url_states = HashMap::new();
        url_states.insert("https://trap.com".to_string(), url_state);

        let diff = classify_scraped_pages(&[page], &[fresh_hash], &url_states);

        assert_eq!(diff.unchanged, Vec::<usize>::new());
        assert_eq!(diff.changed_or_new, vec![0]);
    }

    // =======================================================================
    // Behavior 6: classify_scraped_pages — changed_or_new when missing state
    // =======================================================================

    #[test]
    fn classify_scraped_pages_classifies_changed_or_new_when_url_state_missing() {
        let page = make_page("https://new.com", "content");
        let hash = compute_page_content_hash("content");
        let url_states: HashMap<String, UrlStateRaw> = HashMap::new();

        let diff = classify_scraped_pages(&[page], &[hash], &url_states);

        assert_eq!(diff.unchanged, Vec::<usize>::new());
        assert_eq!(diff.changed_or_new, vec![0]);
    }

    // =======================================================================
    // Behavior 7: classify_scraped_pages — changed_or_new when url_hash is zero
    // =======================================================================

    #[test]
    fn classify_scraped_pages_classifies_changed_or_new_when_url_hash_is_zero() {
        let page = make_page("https://a.com", "hello");
        let content_hash = compute_page_content_hash("hello");
        let url_state = make_url_state(content_hash, [0u8; 32]); // zero url_hash

        let mut url_states = HashMap::new();
        url_states.insert("https://a.com".to_string(), url_state);

        let diff = classify_scraped_pages(&[page], &[content_hash], &url_states);

        assert_eq!(diff.unchanged, Vec::<usize>::new());
        assert_eq!(diff.changed_or_new, vec![0]);
    }

    // =======================================================================
    // Behavior 8: classify_scraped_pages — MCE partition
    // =======================================================================

    #[test]
    fn classify_scraped_pages_produces_mutually_exclusive_collectively_exhaustive_partition() {
        let pages = vec![
            make_page("https://a.com", "content_a"),
            make_page("https://b.com", "content_b"),
            make_page("https://c.com", "content_c"),
        ];
        let hashes: Vec<[u8; 32]> = pages
            .iter()
            .map(|p| compute_page_content_hash(&p.markdown))
            .collect();

        // Only a.com has matching state; b.com has mismatching; c.com has no state
        let mut url_states = HashMap::new();
        url_states.insert(
            "https://a.com".to_string(),
            make_url_state(hashes[0], [1u8; 32]),
        );
        url_states.insert(
            "https://b.com".to_string(),
            make_url_state(compute_page_content_hash("different"), [1u8; 32]),
        );

        let diff = classify_scraped_pages(&pages, &hashes, &url_states);

        // INV-5: collectively exhaustive
        assert_eq!(
            diff.unchanged.len() + diff.changed_or_new.len(),
            3,
            "partition must cover all 3 pages"
        );

        // INV-5: mutually exclusive
        let unchanged_set: std::collections::HashSet<usize> =
            diff.unchanged.iter().copied().collect();
        let changed_set: std::collections::HashSet<usize> =
            diff.changed_or_new.iter().copied().collect();
        assert!(
            unchanged_set.is_disjoint(&changed_set),
            "partitions must be disjoint"
        );

        // Every index 0..2 must appear in exactly one partition
        for i in 0..3 {
            assert!(
                unchanged_set.contains(&i) || changed_set.contains(&i),
                "index {i} must be in exactly one partition"
            );
        }
    }

    // =======================================================================
    // Behavior 8b: classify_scraped_pages — mismatched lengths panic
    // =======================================================================

    #[test]
    #[should_panic(expected = "length")]
    fn classify_scraped_pages_panics_or_errors_when_input_lengths_mismatch() {
        let pages = vec![
            make_page("https://a.com", "a"),
            make_page("https://b.com", "b"),
        ];
        let hashes = vec![[0u8; 32]]; // only 1 hash for 2 pages
        let url_states: HashMap<String, UrlStateRaw> = HashMap::new();

        let _ = classify_scraped_pages(&pages, &hashes, &url_states);
    }

    // =======================================================================
    // Behavior 17: merge_scrape_pages_in_order — empty archived
    // =======================================================================

    #[test]
    fn merge_scrape_pages_in_order_returns_fresh_pages_when_archived_is_empty() {
        let page_a = make_page("https://a.com", "a");
        let page_b = make_page("https://b.com", "b");
        let page_c = make_page("https://c.com", "c");

        let result = merge_scrape_pages_in_order(
            vec![page_a.clone(), page_b.clone(), page_c.clone()],
            HashMap::new(),
        );

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].url, page_a.url);
        assert_eq!(result[1].url, page_b.url);
        assert_eq!(result[2].url, page_c.url);
    }

    // =======================================================================
    // Behavior 18: merge_scrape_pages_in_order — substitution preserving order
    // =======================================================================

    #[test]
    fn merge_scrape_pages_in_order_substitutes_archived_at_correct_indices_preserving_order() {
        let page_a = make_page("https://a.com", "fresh_a");
        let page_b = make_page("https://b.com", "fresh_b");
        let page_c = make_page("https://c.com", "fresh_c");

        let archived_a = make_page("https://a.com", "archived_a");
        let archived_c = make_page("https://c.com", "archived_c");

        let mut archived_pages = HashMap::new();
        archived_pages.insert(0, archived_a.clone());
        archived_pages.insert(2, archived_c.clone());

        let result =
            merge_scrape_pages_in_order(vec![page_a, page_b.clone(), page_c], archived_pages);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].url, archived_a.url);
        assert_eq!(result[0].markdown, "archived_a");
        assert_eq!(result[1].url, page_b.url);
        assert_eq!(result[1].markdown, "fresh_b");
        assert_eq!(result[2].url, archived_c.url);
        assert_eq!(result[2].markdown, "archived_c");
    }

    // =======================================================================
    // Behavior 24: ScrapePageDiff::default
    // =======================================================================

    #[test]
    fn scrape_page_diff_default_returns_empty_partitions() {
        let diff = ScrapePageDiff::default();
        assert_eq!(diff.unchanged, Vec::<usize>::new());
        assert_eq!(diff.changed_or_new, Vec::<usize>::new());
    }

    // =======================================================================
    // Behavior 25: ScrapeReuseStats::default
    // =======================================================================

    #[test]
    fn scrape_reuse_stats_default_returns_zero_counts() {
        let stats = ScrapeReuseStats::default();
        assert_eq!(stats.reused, 0);
        assert_eq!(stats.scraped, 0);
    }

    // =======================================================================
    // Behavior 26: ScrapeReuseError::StateLoad Display
    // =======================================================================

    #[test]
    fn scrape_reuse_error_state_load_displays_correctly() {
        let inner = StateLoadError::BackendError {
            operation: "open_table",
            message: "table missing".to_string(),
        };
        let error = ScrapeReuseError::StateLoad(inner);
        let msg = format!("{error}");
        assert!(
            msg.contains("failed to load url states"),
            "message should contain 'failed to load url states': {msg}"
        );
        assert!(
            msg.contains("table missing"),
            "message should contain 'table missing': {msg}"
        );
    }

    // =======================================================================
    // Behavior 27: ScrapeReuseError::BulkLoad Display
    // =======================================================================

    #[test]
    fn scrape_reuse_error_bulk_load_displays_correctly() {
        let inner = BulkLoadError::TableOpen {
            table: "scrape_outputs",
            message: "not found".to_string(),
        };
        let error = ScrapeReuseError::BulkLoad(inner);
        let msg = format!("{error}");
        assert!(
            msg.contains("failed to load archived scrape outputs"),
            "message should contain 'failed to load archived scrape outputs': {msg}"
        );
        assert!(
            msg.contains("not found"),
            "message should contain 'not found': {msg}"
        );
    }

    // =======================================================================
    // Behavior 28: ScrapeReuseError::DeserializationFailed Display
    // =======================================================================

    #[test]
    fn scrape_reuse_error_deserialization_failed_displays_key_hex_and_message() {
        let error = ScrapeReuseError::DeserializationFailed {
            key_hex: "deadbeef".to_string(),
            message: "invalid archive".to_string(),
        };
        let msg = format!("{error}");
        assert!(
            msg.contains("deadbeef"),
            "message should contain 'deadbeef': {msg}"
        );
        assert!(
            msg.contains("invalid archive"),
            "message should contain 'invalid archive': {msg}"
        );
    }

    // =======================================================================
    // Behavior 29: ScrapeReuseError::HashMismatch Display
    // =======================================================================

    #[test]
    fn scrape_reuse_error_hash_mismatch_displays_url_and_hashes() {
        let error = ScrapeReuseError::HashMismatch {
            url: "https://a.com".to_string(),
            stored_hex: "aa".to_string(),
            loaded_hex: "bb".to_string(),
        };
        let msg = format!("{error}");
        assert!(
            msg.contains("https://a.com"),
            "message should contain 'https://a.com': {msg}"
        );
        assert!(msg.contains("aa"), "message should contain 'aa': {msg}");
        assert!(msg.contains("bb"), "message should contain 'bb': {msg}");
    }

    // =======================================================================
    // Behavior 30: ScrapeReuseError::MissingUrlState Display
    // =======================================================================

    #[test]
    fn scrape_reuse_error_missing_url_state_displays_url() {
        let error = ScrapeReuseError::MissingUrlState {
            url: "https://missing.com".to_string(),
        };
        let msg = format!("{error}");
        assert!(
            msg.contains("https://missing.com"),
            "message should contain 'https://missing.com': {msg}"
        );
        assert!(
            msg.contains("missing url_state"),
            "message should contain 'missing url_state': {msg}"
        );
    }

    // =======================================================================
    // Additional unit tests for mutation killing
    // =======================================================================

    #[test]
    fn classify_scraped_pages_all_unchanged_when_all_match() {
        let pages = vec![
            make_page("https://a.com", "a"),
            make_page("https://b.com", "b"),
        ];
        let hashes: Vec<[u8; 32]> = pages
            .iter()
            .map(|p| compute_page_content_hash(&p.markdown))
            .collect();

        let mut url_states = HashMap::new();
        for (i, page) in pages.iter().enumerate() {
            url_states.insert(
                page.url.clone(),
                make_url_state(hashes[i], [i as u8 + 1; 32]),
            );
        }

        let diff = classify_scraped_pages(&pages, &hashes, &url_states);

        assert_eq!(diff.unchanged, vec![0, 1]);
        assert_eq!(diff.changed_or_new, Vec::<usize>::new());
    }

    #[test]
    fn classify_scraped_pages_all_changed_when_all_mismatch() {
        let pages = vec![
            make_page("https://a.com", "new_a"),
            make_page("https://b.com", "new_b"),
        ];
        let hashes: Vec<[u8; 32]> = pages
            .iter()
            .map(|p| compute_page_content_hash(&p.markdown))
            .collect();

        let mut url_states = HashMap::new();
        for page in &pages {
            url_states.insert(
                page.url.clone(),
                make_url_state(compute_page_content_hash("old"), [1u8; 32]),
            );
        }

        let diff = classify_scraped_pages(&pages, &hashes, &url_states);

        assert_eq!(diff.unchanged, Vec::<usize>::new());
        assert_eq!(diff.changed_or_new, vec![0, 1]);
    }

    #[test]
    fn merge_scrape_pages_in_order_single_page_no_archive() {
        let page = make_page("https://a.com", "a");
        let result = merge_scrape_pages_in_order(vec![page.clone()], HashMap::new());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].url, page.url);
    }

    #[test]
    fn merge_scrape_pages_in_order_all_archived() {
        let page_a = make_page("https://a.com", "fresh");
        let page_b = make_page("https://b.com", "fresh");
        let archived_a = make_page("https://a.com", "archived_a");
        let archived_b = make_page("https://b.com", "archived_b");

        let mut archived = HashMap::new();
        archived.insert(0, archived_a);
        archived.insert(1, archived_b);

        let result = merge_scrape_pages_in_order(vec![page_a, page_b], archived);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].markdown, "archived_a");
        assert_eq!(result[1].markdown, "archived_b");
    }

    #[test]
    fn compute_page_content_hash_is_deterministic() {
        let h1 = compute_page_content_hash("test string");
        let h2 = compute_page_content_hash("test string");
        assert_eq!(h1, h2);
    }

    #[test]
    fn compute_page_content_hash_differs_for_different_inputs() {
        let h1 = compute_page_content_hash("string a");
        let h2 = compute_page_content_hash("string b");
        assert_ne!(h1, h2);
    }

    #[test]
    fn compute_page_content_hash_returns_32_bytes() {
        let hash = compute_page_content_hash("anything");
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn classify_scraped_pages_preserves_indices_in_changed_or_new() {
        // Pages at index 1 and 3 are changed, others unchanged
        let pages = vec![
            make_page("https://a.com", "same"),
            make_page("https://b.com", "changed"),
            make_page("https://c.com", "same"),
            make_page("https://d.com", "changed"),
        ];
        let hashes: Vec<[u8; 32]> = pages
            .iter()
            .map(|p| compute_page_content_hash(&p.markdown))
            .collect();

        let mut url_states = HashMap::new();
        // a.com matches (index 0)
        url_states.insert(
            "https://a.com".to_string(),
            make_url_state(hashes[0], [1u8; 32]),
        );
        // b.com mismatched (index 1)
        url_states.insert(
            "https://b.com".to_string(),
            make_url_state(compute_page_content_hash("different"), [1u8; 32]),
        );
        // c.com matches (index 2)
        url_states.insert(
            "https://c.com".to_string(),
            make_url_state(hashes[2], [1u8; 32]),
        );
        // d.com has zero url_hash (index 3)
        url_states.insert(
            "https://d.com".to_string(),
            make_url_state(hashes[3], [0u8; 32]),
        );

        let diff = classify_scraped_pages(&pages, &hashes, &url_states);

        assert_eq!(diff.unchanged, vec![0, 2]);
        assert_eq!(diff.changed_or_new, vec![1, 3]);
    }
}
