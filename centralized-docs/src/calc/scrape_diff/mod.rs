//! Calc: Scrape diff classification and state changes builder.
//!
//! Pure functions that classify scraped pages against stored URL state
//! and build atomic `StateChanges` batches for the scrape commit phase.

#![cfg_attr(
    not(test),
    deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)
)]
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
use crate::scrape::validation::{ScrapeResult, ScrapedPage};
use crate::state::commit::StateChanges;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};

// ---------------------------------------------------------------------------
// Domain Types
// ---------------------------------------------------------------------------

/// Partition of scraped pages into unchanged, changed, and new buckets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrapeDiff {
    /// Pages whose content hash matches the stored value exactly.
    pub unchanged: Vec<String>,
    /// Pages whose content hash differs from the stored value.
    pub changed: Vec<String>,
    /// Pages not found in stored URL states.
    pub new: Vec<String>,
}

impl ScrapeDiff {
    /// Total number of pages across all partitions.
    #[must_use]
    pub fn total_len(&self) -> usize {
        self.new
            .len()
            .saturating_add(self.changed.len())
            .saturating_add(self.unchanged.len())
    }
}

// ---------------------------------------------------------------------------
// Pure Functions
// ---------------------------------------------------------------------------

/// Classify scraped pages against stored URL states into unchanged/changed/new buckets.
///
/// Duplicate URLs in `scraped_pages` are classified on first occurrence only.
#[must_use]
pub fn classify_scrape_diff(
    stored_url_states: &HashMap<String, crate::state::UrlStateRaw>,
    scraped_pages: &[ScrapedPage],
) -> ScrapeDiff {
    let mut seen: HashSet<&str> = HashSet::new();
    scraped_pages.iter().fold(
        ScrapeDiff {
            new: vec![],
            changed: vec![],
            unchanged: vec![],
        },
        |mut acc, page| {
            if seen.contains(page.url.as_str()) {
                return acc;
            }
            seen.insert(page.url.as_str());
            let page_hash = hash_content(page.markdown.as_bytes());
            match stored_url_states.get(&page.url) {
                None => acc.new.push(page.url.clone()),
                Some(stored) if stored.content_hash == page_hash => {
                    acc.unchanged.push(page.url.clone());
                }
                Some(_) => {
                    acc.changed.push(page.url.clone());
                }
            }
            acc
        },
    )
}

/// Build a `StateChanges` batch from a `ScrapeDiff` and the scraped pages.
///
/// Only new and changed pages produce entries. Unchanged pages are excluded.
#[must_use]
pub fn build_scrape_state_changes(
    scrape_diff: &ScrapeDiff,
    scraped_pages: &[ScrapedPage],
    timestamp: u64,
) -> StateChanges {
    let lookup = page_lookup(scraped_pages);
    let active_urls: Vec<&str> = scrape_diff
        .new
        .iter()
        .chain(scrape_diff.changed.iter())
        .map(String::as_str)
        .collect();

    let updated_urls: Vec<(String, crate::state::UrlStateRaw)> = active_urls
        .iter()
        .filter_map(|url| {
            lookup.get(url).map(|page| {
                let content_hash = hash_content(page.markdown.as_bytes());
                (
                    (*url).to_string(),
                    crate::state::UrlStateRaw {
                        content_hash,
                        url_hash: [0u8; 32],
                        last_fetched_secs: timestamp,
                        status_code: 200,
                        reserved: [0u8; 46],
                    },
                )
            })
        })
        .collect();

    let new_scrapes: Vec<([u8; 32], Vec<u8>)> = active_urls
        .iter()
        .filter_map(|url| {
            lookup.get(url).and_then(|page| {
                let persisted = crate::persisted::PersistedScrapeResult {
                    schema_version: 1,
                    pages: vec![crate::persisted::scraped_page_to_persisted(page)],
                    total_urls: 1,
                    success_count: 1,
                    error_count: 0,
                    errors: vec![],
                    base_url: url.to_string(),
                };
                rkyv::to_bytes::<rkyv::rancor::Error>(&persisted)
                    .ok()
                    .map(|bytes| {
                        let bytes_vec = bytes.to_vec();
                        let hash = hash_content(&bytes_vec);
                        (hash, bytes_vec)
                    })
            })
        })
        .collect();

    let scrape_hash_lookup: HashMap<[u8; 32], [u8; 32]> = active_urls
        .iter()
        .zip(new_scrapes.iter())
        .filter_map(|(url, (scrape_hash, _))| {
            lookup.get(url).map(|page| {
                let content_hash = hash_content(page.markdown.as_bytes());
                (content_hash, *scrape_hash)
            })
        })
        .collect();

    let updated_urls: Vec<(String, crate::state::UrlStateRaw)> = updated_urls
        .into_iter()
        .map(|(url, mut state)| {
            if let Some(&scrape_key) = scrape_hash_lookup.get(&state.content_hash) {
                state.url_hash = scrape_key;
            }
            (url, state)
        })
        .collect();

    StateChanges {
        updated_urls,
        new_scrapes,
        ..StateChanges::empty()
    }
}

/// Compute SHA-256 hash of a byte slice.
#[must_use]
pub fn hash_content(bytes: &[u8]) -> [u8; 32] {
    let digest = Sha256::digest(bytes);
    let mut array = [0u8; 32];
    array.copy_from_slice(&digest);
    array
}

/// Build a page lookup map from scraped pages.
fn page_lookup(pages: &[ScrapedPage]) -> HashMap<&str, &ScrapedPage> {
    pages.iter().map(|p| (p.url.as_str(), p)).collect()
}

/// Build a `ScrapeResult` from combined reused and freshly scraped pages.
#[must_use]
pub fn build_combined_scrape_result(
    reused_pages: Vec<ScrapedPage>,
    fresh_pages: Vec<ScrapedPage>,
    base_url: &str,
) -> ScrapeResult {
    let all_pages: Vec<ScrapedPage> = reused_pages
        .into_iter()
        .chain(fresh_pages.into_iter())
        .collect();
    let success_count = all_pages.len();
    ScrapeResult {
        pages: all_pages,
        total_urls: success_count,
        success_count,
        error_count: 0,
        errors: vec![],
        base_url: base_url.to_string(),
    }
}

#[cfg(test)]
mod tests;
