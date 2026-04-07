#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::must_use_candidate)]

//! Scrape configuration, domain types, and validation utilities.
//!
//! Contains all types shared across the scraping sub-system:
//!
//! - **Behaviour enums** — [`SitemapStrategy`], [`RobotsPolicy`], [`FilteringMode`],
//!   [`RetryStrategy`], [`StealthMode`]: replace `bool` flags with explicit intent.
//! - **[`ScrapeConfig`]** — Complete configuration for a single scrape run.
//! - **[`ScrapedPage`]** — One successfully scraped and converted page.
//! - **[`PageFilterStatus`]** — Whether content-density filtering was applied.
//! - **[`ScrapeResult`]** — Aggregate result with success/error counts.
//!
//! Validation utilities ([`compile_safe_regex`], [`validate_url`],
//! [`check_html_size`], [`validate_scrape_result`]) guard the I/O boundary so that
//! domain functions downstream receive only trusted, well-formed inputs.

mod spa_and_title;
mod types;
mod validators;

#[cfg(test)]
mod tests_spa;
#[cfg(test)]
mod tests_title;
#[cfg(test)]
mod tests_url;

// Re-export all public types
pub use spa_and_title::{detect_potential_spa, extract_title, SpaDetectionResult};
pub use types::{
    ConnectTimeoutSecs, FilteringMode, Header, PageFilterStatus, RequestTimeoutSecs, RetryStrategy,
    RobotsPolicy, ScrapeConfig, ScrapeResult, ScrapedPage, SitemapStrategy, StealthMode,
};
pub(crate) use validators::compile_safe_regex;
pub use validators::{
    check_html_size, check_markdown_size, limit_links_per_page, validate_scrape_result,
    validate_slug, validate_url,
};
