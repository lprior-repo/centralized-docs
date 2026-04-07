//! Extraction pipeline — functional fold over scraped pages.

use super::checks;
use super::types::{ExtractionStatus, HaltReason, ScrapeError, UrlSet};
use crate::scrape::validation::{ScrapeConfig, ScrapeResult, ScrapedPage};
use rayon::prelude::*;

/// State used during the functional fold of scraped pages.
#[derive(Debug, Clone, Default)]
struct ExtractionState {
    pages: Vec<ScrapedPage>,
    errors: Vec<ScrapeError>,
    seen_urls: UrlSet,
    total_content_size: u64,
    status: ExtractionStatus,
}

/// Merge two extraction states into a new state without mutation.
fn merge_extraction_states(a: ExtractionState, b: ExtractionState) -> ExtractionState {
    let merged_seen = a.seen_urls.union(&b.seen_urls);

    ExtractionState {
        pages: a.pages.into_iter().chain(b.pages).collect(),
        errors: a.errors.into_iter().chain(b.errors).collect(),
        seen_urls: merged_seen,
        total_content_size: a.total_content_size.saturating_add(b.total_content_size),
        status: match (a.status, b.status) {
            (ExtractionStatus::Halted(r), _) | (_, ExtractionStatus::Halted(r)) => {
                ExtractionStatus::Halted(r)
            }
            _ => ExtractionStatus::Active,
        },
    }
}

/// Initial extraction state factory.
fn initial_extraction_state() -> ExtractionState {
    ExtractionState {
        pages: Vec::new(),
        errors: Vec::new(),
        seen_urls: UrlSet::new(),
        total_content_size: 0,
        status: ExtractionStatus::Active,
    }
}

/// Handle transformed page content - check empty and size limits.
fn handle_transformed_content(
    url: String,
    scraped: ScrapedPage,
    state: ExtractionState,
    config: &ScrapeConfig,
) -> ExtractionState {
    if scraped.markdown.trim().is_empty() {
        return ExtractionState {
            pages: state.pages,
            errors: checks::append_error(
                state.errors,
                url,
                "Skipped page with empty markdown content".to_string(),
            ),
            seen_urls: state.seen_urls,
            total_content_size: state.total_content_size,
            status: state.status,
        };
    }

    check_size_and_accumulate(url, scraped, state, config)
}

/// Check if total size limit would be exceeded — wraps check into state transition.
fn check_total_size_exceeded(
    state: ExtractionState,
    page_size: u64,
    max_total: u64,
) -> Option<ExtractionState> {
    checks::check_total_size_exceeded(state.status, state.total_content_size, page_size, max_total)
        .map(|halted_status| ExtractionState {
            pages: state.pages,
            errors: state.errors,
            seen_urls: state.seen_urls,
            total_content_size: state.total_content_size,
            status: halted_status,
        })
}

/// Accumulate a valid page into state.
fn accumulate_page(
    url: String,
    scraped: ScrapedPage,
    state: ExtractionState,
    page_size: u64,
) -> ExtractionState {
    let new_seen = state.seen_urls.insert(url);
    let new_pages = state
        .pages
        .into_iter()
        .chain(std::iter::once(scraped))
        .collect();

    ExtractionState {
        pages: new_pages,
        errors: state.errors,
        seen_urls: new_seen,
        total_content_size: state.total_content_size.saturating_add(page_size),
        status: state.status,
    }
}

/// Check size limit - returns Some with error state if limit exceeded, None otherwise.
fn check_size_limit(
    state: ExtractionState,
    url: String,
    page_size: u64,
    max_total: u64,
) -> Option<ExtractionState> {
    check_total_size_exceeded(state, page_size, max_total).map(|error_state| ExtractionState {
        pages: error_state.pages,
        errors: checks::append_error(
            error_state.errors,
            url,
            format!("Total content size would exceed limit ({max_total} bytes)"),
        ),
        seen_urls: error_state.seen_urls,
        total_content_size: error_state.total_content_size,
        status: error_state.status,
    })
}

/// Check size limit and accumulate valid page.
fn check_size_and_accumulate(
    url: String,
    scraped: ScrapedPage,
    state: ExtractionState,
    config: &ScrapeConfig,
) -> ExtractionState {
    let page_size = scraped.markdown.len() as u64;

    if let Some(error_state) = check_size_limit(
        state.clone(),
        url.clone(),
        page_size,
        config.max_total_size_bytes,
    ) {
        return error_state;
    }

    accumulate_page(url, scraped, state, page_size)
}

/// Handle transform error - return state with error appended.
fn handle_transform_error(
    url: String,
    error: &anyhow::Error,
    state: ExtractionState,
) -> ExtractionState {
    ExtractionState {
        pages: state.pages,
        errors: checks::append_error(state.errors, url, error.to_string()),
        seen_urls: state.seen_urls,
        total_content_size: state.total_content_size,
        status: state.status,
    }
}

/// Build error state from eligibility check failure.
fn build_eligibility_error_state(
    url: String,
    error_msg: String,
    halt_reason: Option<HaltReason>,
    state: ExtractionState,
) -> ExtractionState {
    ExtractionState {
        pages: state.pages,
        errors: checks::append_error(state.errors, url, error_msg),
        seen_urls: state.seen_urls,
        total_content_size: state.total_content_size,
        status: halt_reason.map_or(state.status, ExtractionStatus::Halted),
    }
}

/// Transform and accumulate a single page - returns new state.
fn transform_and_accumulate_page(
    state: ExtractionState,
    page: &spider::page::Page,
    config: &ScrapeConfig,
    max_pages: usize,
) -> ExtractionState {
    if let Some((url, error_msg, halt_reason)) =
        checks::check_page_eligibility(page, config, state.pages.len(), max_pages)
    {
        return build_eligibility_error_state(url, error_msg, halt_reason, state);
    }

    let url = page.get_url().to_string();
    let transformed = super::super::transformers::transform_page(
        page,
        &config.base_url,
        config,
        config.filtering_mode,
    );

    match transformed {
        Ok(scraped) => handle_transformed_content(url, scraped, state, config),
        Err(error) => handle_transform_error(url, &error, state),
    }
}

/// Process all spider pages using functional fold - pure function.
fn process_pages_with_fold(
    spider_pages: &[spider::page::Page],
    config: &ScrapeConfig,
) -> ExtractionState {
    spider_pages
        .into_par_iter()
        .fold(initial_extraction_state, |state, page| {
            if matches!(state.status, ExtractionStatus::Halted(_)) {
                return state;
            }
            transform_and_accumulate_page(state, page, config, config.max_pages)
        })
        .reduce(initial_extraction_state, merge_extraction_states)
}

/// Extract the total URL count from website pages.
fn get_total_url_count(website: &spider::website::Website) -> usize {
    website.get_pages().as_ref().map_or(0, |p| p.len())
}

/// Build ScrapeResult from extraction state.
fn build_scrape_result(
    final_state: ExtractionState,
    total_urls: usize,
    base_url: String,
) -> ScrapeResult {
    let success_count = final_state.pages.len();
    ScrapeResult {
        pages: final_state.pages,
        total_urls,
        success_count,
        error_count: final_state.errors.len(),
        errors: final_state
            .errors
            .into_iter()
            .map(|e| {
                let url = e.url().unwrap_or("unknown").to_string();
                let message = e.to_string();
                (url, message)
            })
            .collect(),
        base_url,
    }
}

/// Extract pages from `website`.
pub fn extract_pages_from_website(
    website: &spider::website::Website,
    config: &ScrapeConfig,
) -> ScrapeResult {
    let total_urls = get_total_url_count(website);

    let final_state = website
        .get_pages()
        .as_ref()
        .map_or_else(initial_extraction_state, |spider_pages| {
            process_pages_with_fold(spider_pages.as_slice(), config)
        });

    build_scrape_result(final_state, total_urls, config.base_url.clone())
}
