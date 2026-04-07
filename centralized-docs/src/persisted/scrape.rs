//! Persisted types and conversions for the Scrape pipeline phase.

use super::error::{
    require_finite_f32, require_non_empty, require_range, require_schema_v1, PersistError,
};
use crate::scrape::validation::{Header, PageFilterStatus, ScrapeResult, ScrapedPage};

// ---------------------------------------------------------------------------
// Persisted Record Types — Scrape Family
// ---------------------------------------------------------------------------

/// Persisted header extracted from a scraped page.
#[derive(Debug, Clone, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedHeader {
    /// Header level (1-6).
    pub level: u8,
    /// Header text (non-empty).
    pub text: String,
}

/// Persisted page filter status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum PersistedPageFilterStatus {
    /// Content-density filtering was applied.
    Filtered,
    /// Raw markdown stored without filtering.
    Unfiltered,
}

/// Persisted scraped page.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedScrapedPage {
    /// Page URL.
    pub url: String,
    /// Extracted markdown content.
    pub markdown: String,
    /// Page title.
    pub title: String,
    /// Links found on the page.
    pub links: Vec<String>,
    /// Headers found on the page.
    pub headers: Vec<PersistedHeader>,
    /// Word count of the markdown content.
    pub word_count: usize,
    /// URL-derived slug.
    pub slug: String,
    /// Whether filtering was applied.
    pub filter_status: PersistedPageFilterStatus,
    /// Number of elements removed by filtering.
    pub elements_removed: usize,
    /// Content density score (0.0-1.0, must be finite).
    pub density_score: f32,
}

/// Persisted batch scrape result.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedScrapeResult {
    /// Schema version (always 1).
    pub schema_version: u32,
    /// Successfully scraped pages.
    pub pages: Vec<PersistedScrapedPage>,
    /// Total URLs discovered.
    pub total_urls: usize,
    /// Number of successfully scraped pages.
    pub success_count: usize,
    /// Number of failed scrapes.
    pub error_count: usize,
    /// Errors as (url, `error_message`) pairs.
    pub errors: Vec<(String, String)>,
    /// Base URL of the scrape.
    pub base_url: String,
}

// ===========================================================================
// Conversions: Runtime → Persisted (Infallible)
// ===========================================================================

/// Convert a runtime scrape [`Header`] to its persisted form.
#[must_use]
pub fn header_to_persisted(h: &Header) -> PersistedHeader {
    PersistedHeader {
        level: h.level,
        text: h.text.clone(),
    }
}

/// Convert a runtime [`PageFilterStatus`] to its persisted form.
#[must_use]
pub fn page_filter_status_to_persisted(s: &PageFilterStatus) -> PersistedPageFilterStatus {
    match s {
        PageFilterStatus::Filtered => PersistedPageFilterStatus::Filtered,
        PageFilterStatus::Unfiltered => PersistedPageFilterStatus::Unfiltered,
    }
}

/// Convert a runtime [`ScrapedPage`] to its persisted form.
#[must_use]
pub fn scraped_page_to_persisted(p: &ScrapedPage) -> PersistedScrapedPage {
    PersistedScrapedPage {
        url: p.url.clone(),
        markdown: p.markdown.clone(),
        title: p.title.clone(),
        links: p.links.clone(),
        headers: p.headers.iter().map(header_to_persisted).collect(),
        word_count: p.word_count,
        slug: p.slug.clone(),
        filter_status: page_filter_status_to_persisted(&p.filter_status),
        elements_removed: p.elements_removed,
        density_score: p.density_score,
    }
}

/// Convert a runtime [`ScrapeResult`] to its persisted form.
#[must_use]
pub fn scrape_result_to_persisted(r: &ScrapeResult) -> PersistedScrapeResult {
    PersistedScrapeResult {
        schema_version: 1,
        pages: r.pages.iter().map(scraped_page_to_persisted).collect(),
        total_urls: r.total_urls,
        success_count: r.success_count,
        error_count: r.error_count,
        errors: r.errors.clone(),
        base_url: r.base_url.clone(),
    }
}

// ===========================================================================
// Conversions: Persisted → Runtime (Fallible)
// ===========================================================================

/// Convert a persisted scrape header back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::OutOfRange`] if level is not in 1..=6.
/// Returns [`PersistError::EmptyField`] if text is empty.
pub fn persisted_header_to_runtime(p: &PersistedHeader) -> Result<Header, PersistError> {
    require_range(i64::from(p.level), 1, 6, "level")?;
    require_non_empty(&p.text, "text")?;
    Ok(Header {
        level: p.level,
        text: p.text.clone(),
    })
}

/// Convert a persisted page filter status back to runtime form (1:1 mapping).
pub fn persisted_page_filter_status_to_runtime(
    p: PersistedPageFilterStatus,
) -> Result<PageFilterStatus, PersistError> {
    match p {
        PersistedPageFilterStatus::Filtered => Ok(PageFilterStatus::Filtered),
        PersistedPageFilterStatus::Unfiltered => Ok(PageFilterStatus::Unfiltered),
    }
}

/// Convert a persisted scraped page back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::NonFiniteFloat`] if `density_score` is `NaN` or Infinite.
/// Propagates any errors from nested header conversions.
pub fn persisted_scraped_page_to_runtime(
    p: &PersistedScrapedPage,
) -> Result<ScrapedPage, PersistError> {
    require_finite_f32(p.density_score, "density_score")?;
    let headers = p
        .headers
        .iter()
        .map(persisted_header_to_runtime)
        .collect::<Result<Vec<_>, _>>()?;
    let filter_status = persisted_page_filter_status_to_runtime(p.filter_status)?;
    Ok(ScrapedPage {
        url: p.url.clone(),
        markdown: p.markdown.clone(),
        title: p.title.clone(),
        links: p.links.clone(),
        headers,
        word_count: p.word_count,
        slug: p.slug.clone(),
        filter_status,
        elements_removed: p.elements_removed,
        density_score: p.density_score,
    })
}

/// Convert a persisted scrape result back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::SchemaVersionMismatch`] if `schema_version` != 1.
/// Propagates any errors from nested page conversions.
pub fn persisted_scrape_result_to_runtime(
    p: &PersistedScrapeResult,
) -> Result<ScrapeResult, PersistError> {
    require_schema_v1(p.schema_version)?;
    let pages = p
        .pages
        .iter()
        .map(persisted_scraped_page_to_runtime)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ScrapeResult {
        pages,
        total_urls: p.total_urls,
        success_count: p.success_count,
        error_count: p.error_count,
        errors: p.errors.clone(),
        base_url: p.base_url.clone(),
    })
}
