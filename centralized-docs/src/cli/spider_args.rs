//! Shared argument structs for web-scraping CLI commands.
//!
//! `SpiderCoreArgs` holds the 7 fields common to Scrape, Ingest, and Watch.
//! `SpiderCrawlArgs` holds the 4 BM25 / byte-limit fields shared by Scrape
//! and Ingest only.

use super::validation::{
    parse_redirect_policy, validate_concurrency_limit, validate_connect_timeout_secs,
    validate_delay, validate_positive_bytes, validate_retry_count, validate_threshold,
    validate_timeout_secs,
};
use clap::Args;
use spider::configuration::RedirectPolicy;

/// Core arguments shared by all scraping commands (Scrape, Ingest, Watch).
#[derive(Args, Debug)]
pub struct SpiderCoreArgs {
    /// Regex pattern to filter URLs by path
    #[arg(short, long, value_name = "REGEX")]
    pub filter: Option<String>,

    /// Delay between requests in milliseconds (0-60000)
    #[arg(short, long, default_value = "0", value_parser = validate_delay, allow_hyphen_values = true)]
    pub delay: u64,

    /// Request timeout in seconds (1-600)
    #[arg(long, default_value = "30", value_parser = validate_timeout_secs, allow_hyphen_values = true)]
    pub request_timeout_secs: u64,

    /// TCP connect timeout in seconds (1-60, default: 10)
    #[arg(long, default_value = "10", value_parser = validate_connect_timeout_secs, allow_hyphen_values = true)]
    pub connect_timeout_secs: u64,

    /// Max spider retries (0 disables spider retry)
    #[arg(long, default_value = "3", value_parser = validate_retry_count, allow_hyphen_values = true)]
    pub max_retries: u32,

    /// Redirect policy: loose (default), strict, none
    #[arg(long, default_value = "loose", value_parser = parse_redirect_policy)]
    pub redirect_policy: RedirectPolicy,

    /// Concurrency (1-128, default 4) capped for politeness
    #[arg(long, default_value = "4", value_parser = validate_concurrency_limit, allow_hyphen_values = true)]
    pub concurrency: usize,
}

/// BM25 and byte-limit arguments shared by Scrape and Ingest (not Watch).
#[derive(Args, Debug)]
pub struct SpiderCrawlArgs {
    /// Max bytes per page (spider-level, before transform)
    #[arg(long, value_parser = validate_positive_bytes)]
    pub max_page_bytes: Option<u64>,

    /// Max total bytes across crawl (spider-level)
    #[arg(long, value_parser = validate_positive_bytes)]
    pub max_total_bytes: Option<u64>,

    /// Filter pages by BM25 relevance to query
    #[arg(short, long, value_name = "QUERY")]
    pub query: Option<String>,

    /// Minimum BM25 score to keep a page (default: 0.1, range: 0.0-10.0)
    #[arg(long, default_value = "0.1", value_parser = validate_threshold, allow_hyphen_values = true)]
    pub threshold: f32,
}
