//! Archive loading (I/O Action) — loading archived scrape outputs for unchanged pages.

use std::collections::HashMap;

use crate::scrape::validation::ScrapedPage;
use crate::state::bulk_load::{BulkLoadError, OwnedArchive, StateReadSession};
use crate::state::UrlStateRaw;
use itertools::Itertools;

use super::classify::compute_page_content_hash;
use super::types::{
    persisted_scraped_page_to_runtime, PersistedScrapeResult, ScrapePageDiff, ScrapeReuseError,
};

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
                fallback_indices.push(idx);
            }
            Some(archive) => {
                let result = load_single_page_from_batch(archive, url, stored_content_hash);
                match result {
                    Ok(page) => {
                        loaded_pages.insert(idx, page);
                    }
                    Err(_) => {
                        fallback_indices.push(idx);
                    }
                }
            }
        }
    }

    Ok((loaded_pages, fallback_indices))
}

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

    let matched_page = persisted
        .pages
        .iter()
        .find(|p| p.url == expected_url)
        .ok_or(())?;

    let runtime_page = persisted_scraped_page_to_runtime(matched_page).map_err(|_| ())?;

    // Verify hash integrity (INV-8)
    let loaded_hash = compute_page_content_hash(&runtime_page.markdown);
    if loaded_hash != stored_content_hash {
        return Err(());
    }

    Ok(runtime_page)
}
