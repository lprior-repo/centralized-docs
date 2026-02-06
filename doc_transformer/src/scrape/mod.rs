//! Scrape module stub
//!
//! This module is temporarily stubbed out in the br branch.
//! The full implementation exists in the main branch.

#![allow(dead_code)]
#![allow(unused_variables)]

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Placeholder for `ScrapedPage` type used in main.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedPage {
    pub url: String,
    pub content: String,
    pub title: Option<String>,
}

/// Placeholder for `ScrapeConfig`
#[derive(Debug, Clone)]
pub struct ScrapeConfig {
    pub url: String,
    pub use_sitemap: bool,
    pub filter: Option<String>,
    pub delay_ms: u64,
    pub request_timeout_secs: u64,
    pub max_retries: u32,
    pub redirect_policy: String,
    pub max_page_bytes: Option<u64>,
    pub max_total_bytes: Option<u64>,
    pub concurrency: usize,
}

/// Placeholder for `ScrapeResult`
#[derive(Debug)]
pub struct ScrapeResult {
    pub pages: Vec<ScrapedPage>,
    pub success_count: usize,
}

/// Stub error type
#[derive(Debug)]
pub struct ScrapeError;

impl std::fmt::Display for ScrapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scraping is not available in this branch")
    }
}

impl std::error::Error for ScrapeError {}

/// Stub function
pub async fn scrape_site(_config: &ScrapeConfig) -> Result<ScrapeResult> {
    Err(anyhow::anyhow!("Scraping is not available in this branch"))
}

/// Stub function
pub fn validate_scrape_result(_result: &ScrapeResult) -> Result<()> {
    Ok(())
}

/// Stub function  
pub fn write_scraped_pages(_result: &ScrapeResult, _output: &Path) -> Result<()> {
    Ok(())
}
