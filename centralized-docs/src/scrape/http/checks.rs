//! Pure predicate functions for extraction pipeline limits and checks.

use super::types::{ExtractionStatus, HaltReason, ScrapeError};

/// Append error to error vector - pure function.
pub(super) fn append_error(errors: Vec<ScrapeError>, url: String, msg: String) -> Vec<ScrapeError> {
    errors
        .into_iter()
        .chain(std::iter::once(ScrapeError::generic(url, msg)))
        .collect()
}

/// Check if page exceeds size limit.
pub(super) fn check_page_size_limit(page: &spider::page::Page, limit: u64) -> Option<String> {
    let html = page.get_html();
    let html_size = html.len() as u64;
    (html_size > limit)
        .then(|| format!("Page exceeds max-page-bytes limit ({html_size} bytes > {limit} bytes)"))
}

/// Check if page limit is reached - pure function.
pub(super) fn is_page_limit_reached(current_count: usize, max_pages: usize) -> bool {
    current_count >= max_pages
}

/// Check if total size limit would be exceeded - pure function.
pub(super) fn would_exceed_total_size(current_size: u64, page_size: u64, max_total: u64) -> bool {
    if page_size > u64::MAX.saturating_sub(current_size) {
        return true;
    }
    current_size.saturating_add(page_size) > max_total
}

/// Check if page is eligible for processing - returns error and halt reason if not.
pub(super) fn check_page_eligibility(
    page: &spider::page::Page,
    config: &crate::scrape::validation::ScrapeConfig,
    current_count: usize,
    max_pages: usize,
) -> Option<(String, String, Option<HaltReason>)> {
    if is_page_limit_reached(current_count, max_pages) {
        let url = page.get_url().to_string();
        let error_msg = format!("Reached page limit ({max_pages}), stopping scrape");
        return Some((url, error_msg, Some(HaltReason::PageLimitReached)));
    }

    if let Some(limit) = config.spider_max_page_bytes {
        if let Some(error_msg) = check_page_size_limit(page, limit) {
            let url = page.get_url().to_string();
            return Some((url, error_msg, None));
        }
    }

    None
}

/// Check if total size limit would be exceeded, returning halted state.
pub(super) fn check_total_size_exceeded(
    _status: ExtractionStatus,
    total_content_size: u64,
    page_size: u64,
    max_total: u64,
) -> Option<ExtractionStatus> {
    would_exceed_total_size(total_content_size, page_size, max_total)
        .then_some(ExtractionStatus::Halted(HaltReason::TotalSizeExceeded))
}
