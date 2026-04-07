//! Merge (pure calculation) and primary entry point.

use std::collections::HashMap;

use crate::scrape::validation::{ScrapeResult, ScrapedPage};
use crate::state::bulk_load::StateReadSession;

use super::archive::load_archived_scrape_pages;
use super::classify::{classify_scraped_pages, compute_page_content_hash};
use super::types::{ScrapeReuseError, ScrapeReuseStats};

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
