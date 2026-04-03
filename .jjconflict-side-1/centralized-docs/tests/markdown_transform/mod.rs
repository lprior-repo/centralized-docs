#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

//! Markdown Transform Tests
//!
//! Tests for the markdown transform capability:
//! - Path edge cases
//! - Regex filtering
//! - Web scraping
//! - Stealth mode and backoff

mod path_edge_cases_tests;
mod regex_filter_tests;
mod scrape_integration_test;
mod stealth_backoff_adversarial;
