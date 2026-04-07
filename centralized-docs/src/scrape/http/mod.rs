#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]
#![cfg_attr(
    test,
    allow(
        clippy::unreadable_literal,
        clippy::field_reassign_with_default,
        clippy::float_cmp
    )
)]

//! HTTP client and website configuration
//!
//! Provides spider-rs website building and HTTP request configuration.

mod checks;
mod client;
mod extraction;
mod types;

#[cfg(test)]
mod tests;

pub use client::{
    build_website_base, check_connectivity_with_timeout, execute_scrape_with_website,
};
pub use extraction::extract_pages_from_website;
pub use types::{
    ExtractionStatus, HaltReason, HttpError, SafeByteLimit, ScrapeError, ScrapeStrategy, UrlSet,
    ValidatedUrl,
};
