//! Classification (pure calculation) — content hashing and page diffing.

use std::collections::HashMap;

use crate::scrape::validation::ScrapedPage;
use crate::state::UrlStateRaw;

use super::types::ScrapePageDiff;

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
