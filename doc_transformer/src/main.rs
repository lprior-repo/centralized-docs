//! `doc_transformer` v5.0 — AI-Optimized Documentation Indexer
//!
//! CLI entry point for the `doc_transformer` pipeline. Exposes four sub-commands
//! that can be composed to go from a raw documentation source (local files **or**
//! a live website) to a fully indexed, AI-queryable knowledge base.
//!
//! # Commands
//!
//! | Command | Input | Output |
//! |---------|-------|--------|
//! | `scrape <URL>` | Live website | `.scrape/` directory of markdown pages |
//! | `index  <DIR>` | Markdown directory | `.index/` search + chunk files + llms.txt |
//! | `ingest <URL>` | Live website | Scrape + index in one step |
//! | `search <QUERY>` | `.index/` directory | Ranked results to stdout |
//!
//! # Typical Workflow
//!
//! ```text
//! # Option A — local docs
//! doc_transformer index ./my-docs --output ./output
//!
//! # Option B — remote docs
//! doc_transformer scrape https://docs.example.com --output ./scraped
//! doc_transformer index ./scraped --output ./output
//!
//! # Option C — one-shot ingest
//! doc_transformer ingest https://docs.example.com --output ./output
//!
//! # Search
//! doc_transformer search "how does authentication work" --index ./output
//! ```
//!
//! # Configuration
//!
//! All numeric parameters are validated at the CLI boundary and converted into
//! typed domain values before reaching library code — see [`types`] for the
//! validated newtype wrappers (`HnswM`, `MaxRelatedChunks`, etc.).
//!
//! Scraping behaviour is controlled by enum flags rather than raw booleans:
//! [`scrape::SitemapStrategy`], [`scrape::RobotsPolicy`],
//! [`scrape::FilteringMode`], [`scrape::RetryStrategy`], [`scrape::StealthMode`].
//!
//! # Exit Codes
//!
//! - `0` — success
//! - `1` — user error (bad arguments, missing files, invalid URL)
//! - `2` — pipeline error (transform failed, index corrupt, network error)

// Strict functional programming constraints
#![allow(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::expect_used)]

mod analyze;
mod assign;
mod chunk;
mod chunking_adapter;
mod config;
mod discover;
#[cfg(feature = "enhanced")]
mod features;
mod filter;
mod graph;
mod highlight;
mod index;
mod llms;
mod scrape;
mod search;
mod similarity;
mod transform;
mod types;
mod validate;

use anyhow::Result;
use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};
use fs2::FileExt;
use scrape::SitemapStrategy;
use serde::{Deserialize, Serialize};
use spider::configuration::RedirectPolicy;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

/// Configuration for the index command
#[derive(Debug, Clone)]
struct IndexConfig {
    generate_llms: bool,
    project_name: String,
    project_desc: String,
    category_config: Option<PathBuf>,
    max_related_chunks: usize,
    max_chunk_keywords: usize,
    hnsw_m: usize,
    hnsw_ef_construction: usize,
    max_document_bytes: u64,
}

impl Default for IndexConfig {
    fn default() -> Self {
        Self {
            generate_llms: true,
            project_name: "Documentation".to_string(),
            project_desc: "AI-optimized documentation index".to_string(),
            category_config: None,
            max_related_chunks: 20,
            max_chunk_keywords: 12,
            hnsw_m: 16,
            hnsw_ef_construction: 200,
            max_document_bytes: 10 * 1024 * 1024, // 10MB default
        }
    }
}

/// Configuration for the scrape command
#[derive(Debug, Clone)]
struct ScrapeCommandConfig {
    sitemap_strategy: SitemapStrategy,
    filter: Option<String>,
    delay: u64,
    query: Option<String>,
    threshold: f32,
    request_timeout_secs: u64,
    max_retries: u32,
    redirect_policy: RedirectPolicy,
    max_page_bytes: Option<u64>,
    max_total_bytes: Option<u64>,
    concurrency_limit: usize,
}

impl Default for ScrapeCommandConfig {
    fn default() -> Self {
        Self {
            sitemap_strategy: SitemapStrategy::UseSitemap,
            filter: None,
            delay: 250,
            query: None,
            threshold: 0.1,
            request_timeout_secs: 30,
            max_retries: 3,
            redirect_policy: RedirectPolicy::Loose,
            max_page_bytes: None,
            max_total_bytes: None,
            concurrency_limit: 1,
        }
    }
}

/// Configuration for the ingest command
#[derive(Debug, Clone)]
struct IngestConfig {
    filter: Option<String>,
    delay: u64,
    query: Option<String>,
    threshold: f32,
    project_name: Option<String>,
    request_timeout_secs: u64,
    max_retries: u32,
    redirect_policy: RedirectPolicy,
    max_page_bytes: Option<u64>,
    max_total_bytes: Option<u64>,
    concurrency_limit: usize,
}

impl Default for IngestConfig {
    fn default() -> Self {
        Self {
            filter: None,
            delay: 250,
            query: None,
            threshold: 0.1,
            project_name: None,
            request_timeout_secs: 30,
            max_retries: 3,
            redirect_policy: RedirectPolicy::Loose,
            max_page_bytes: None,
            max_total_bytes: None,
            concurrency_limit: 1,
        }
    }
}

const DEFAULT_MAX_PAGE_SIZE_BYTES: u64 = 10 * 1024 * 1024;
const DEFAULT_MAX_TOTAL_SIZE_BYTES: u64 = 500 * 1024 * 1024;
const SEARCH_JSON_ALREADY_EMITTED_PREFIX: &str = "__SEARCH_JSON_ALREADY_EMITTED__";

// Validation functions for HNSW graph parameters
//
// Parse as i64 first to properly detect and report negative numbers,
// then validate range before converting to usize.

pub(crate) fn validate_max_related_chunks(s: &str) -> Result<usize, String> {
    let value = s
        .parse::<i64>()
        .map_err(|_| format!("max_related_chunks must be an integer, got '{s}'"))?;

    if value < 1 {
        return Err(format!("max_related_chunks must be at least 1, got '{s}'"));
    }
    if value > 100 {
        return Err(format!("max_related_chunks must be at most 100, got '{s}'"));
    }

    value
        .try_into()
        .map_err(|_| format!("max_related_chunks value too large: {value}"))
}

pub(crate) fn validate_max_chunk_keywords(s: &str) -> Result<usize, String> {
    let value = s
        .parse::<i64>()
        .map_err(|_| format!("max_chunk_keywords must be an integer, got '{s}'"))?;

    if value < 0 {
        return Err(format!("max_chunk_keywords must be at least 0, got '{s}'"));
    }
    if value > 50 {
        return Err(format!("max_chunk_keywords must be at most 50, got '{s}'"));
    }

    value
        .try_into()
        .map_err(|_| format!("max_chunk_keywords value too large: {value}"))
}

pub(crate) fn validate_hnsw_m(s: &str) -> Result<usize, String> {
    let value = s
        .parse::<i64>()
        .map_err(|_| format!("hnsw_m must be an integer, got '{s}'"))?;

    if value < 4 {
        return Err(format!(
            "hnsw_m must be at least 4 for proper connectivity, got '{s}'"
        ));
    }
    if value > 64 {
        return Err(format!(
            "hnsw_m must be at most 64 for reasonable performance, got '{s}'"
        ));
    }

    value
        .try_into()
        .map_err(|_| format!("hnsw_m value too large: {value}"))
}

pub(crate) fn validate_hnsw_ef_construction(s: &str) -> Result<usize, String> {
    let value = s
        .parse::<i64>()
        .map_err(|_| format!("hnsw_ef_construction must be an integer, got '{s}'"))?;

    if value < 50 {
        return Err(format!(
            "hnsw_ef_construction must be at least 50 for acceptable build quality, got '{s}'"
        ));
    }
    if value > 1000 {
        return Err(format!(
            "hnsw_ef_construction must be at most 1000 for reasonable build times, got '{s}'"
        ));
    }

    value
        .try_into()
        .map_err(|_| format!("hnsw_ef_construction value too large: {value}"))
}

/// Validate threshold value for BM25 filtering
///
/// BM25 scores range from 0.0 (no relevance) to positive values.
/// Negative thresholds are meaningless for BM25 and indicate user error.
/// Upper bound is set to 10.0 to allow for flexible filtering while preventing obvious errors.
pub fn validate_threshold(s: &str) -> Result<f32, String> {
    let value = s
        .parse::<f32>()
        .map_err(|_| format!("threshold must be a number, got '{s}'"))?;

    if !value.is_finite() {
        return Err(format!(
            "threshold must be a finite number between 0.0 and 10.0, got {value}"
        ));
    }

    if value < 0.0 {
        return Err(format!(
            "threshold must be non-negative (BM25 scores are >= 0.0), got {value}"
        ));
    }

    if value > 10.0 {
        return Err(format!(
            "threshold must be at most 10.0 for practical filtering, got {value}"
        ));
    }

    Ok(value)
}

/// Validate retry count (0-255 inclusive)
fn validate_retry_count(s: &str) -> Result<u32, String> {
    let value = s
        .parse::<i64>()
        .map_err(|_| format!("retry must be an integer, got '{s}'"))?;

    if value < 0 {
        return Err(format!(
            "retry must be non-negative (0 disables spider retry), got {value}"
        ));
    }

    if value > u8::MAX as i64 {
        return Err(format!("retry must be at most {}, got {value}", u8::MAX));
    }

    value
        .try_into()
        .map_err(|_| format!("retry value too large: {value}"))
}

/// Validate timeout seconds (1-600)
fn validate_timeout_secs(s: &str) -> Result<u64, String> {
    let value = s
        .parse::<i64>()
        .map_err(|_| format!("timeout must be an integer, got '{s}'"))?;

    if value < 1 {
        return Err("timeout must be at least 1 second".to_string());
    }

    if value > 600 {
        return Err(format!(
            "timeout must be at most 600 seconds (10 minutes), got {value}"
        ));
    }

    value
        .try_into()
        .map_err(|_| format!("timeout value too large: {value}"))
}

/// Validate positive byte limits (>=1)
fn validate_positive_bytes(s: &str) -> Result<u64, String> {
    let value = s
        .parse::<i64>()
        .map_err(|_| format!("bytes must be an integer, got '{s}'"))?;

    if value < 1 {
        return Err(format!("bytes must be at least 1, got {value}"));
    }

    value
        .try_into()
        .map_err(|_| format!("bytes value too large: {value}"))
}

/// Validate concurrency (1-2 inclusive)
fn validate_concurrency_limit(s: &str) -> Result<usize, String> {
    let value = s
        .parse::<i64>()
        .map_err(|_| format!("concurrency must be an integer, got '{s}'"))?;

    if value < 1 {
        return Err(format!("concurrency must be at least 1, got {value}"));
    }

    if value > 2 {
        return Err(format!(
            "concurrency must be at most 2 for safety, got {value}"
        ));
    }

    value
        .try_into()
        .map_err(|_| format!("concurrency value too large: {value}"))
}

/// Parse redirect policy (loose|strict|none)
fn parse_redirect_policy(s: &str) -> Result<RedirectPolicy, String> {
    match s.to_ascii_lowercase().as_str() {
        "loose" => Ok(RedirectPolicy::Loose),
        "strict" => Ok(RedirectPolicy::Strict),
        "none" => Ok(RedirectPolicy::None),
        other => Err(format!(
            "redirect policy must be one of: loose, strict, none (got '{other}')"
        )),
    }
}

/// Delay between HTTP requests in milliseconds.
/// Negative delays are meaningless and indicate user error.
/// Upper bound prevents impractically long delays.
pub fn validate_delay(s: &str) -> Result<u64, String> {
    let value = s
        .parse::<i64>()
        .map_err(|_| format!("delay must be an integer, got '{s}'"))?;

    if value < 0 {
        return Err(format!(
            "delay must be non-negative (milliseconds), got {value}"
        ));
    }

    if value > 60_000 {
        return Err(format!(
            "delay must be at most 60000 milliseconds (60 seconds), got {value}"
        ));
    }

    value
        .try_into()
        .map_err(|_| format!("delay value too large: {value}"))
}

/// CLI wrapper for validate_limit that returns String error for clap compatibility.
fn validate_limit_cli(s: &str) -> Result<usize, String> {
    validate::validate_limit(s).map_err(|e| e.to_string())
}

/// Validate and compile regex pattern with ReDoS protection (BEAD-004).
///
/// Implements safety measures against ReDoS attacks:
/// - Maximum pattern length (500 characters)
/// - Detection of known ReDoS patterns (nested quantifiers)
/// - Compilation size limits via RegexBuilder
///
/// Returns Ok(()) if pattern is safe and compiles successfully.
fn validate_filter_regex(pattern: &str) -> Result<(), String> {
    // BEAD-004: Reject patterns that are too long
    if pattern.len() > 500 {
        return Err(format!(
            "Regex pattern too long: {} chars (max 500)",
            pattern.len()
        ));
    }

    // BEAD-004: Check for known ReDoS patterns
    let redos_patterns = [
        (r"\(\.\*\)\*", "nested .* quantifiers: (.*)"),
        (r"\(\.\+\)\+", "nested .+ quantifiers: (.+)+"),
        (r"\([^)]+\+\)\+", "nested + quantifiers on groups"),
        (r"\([^)]+\*\)\*", "nested * quantifiers on groups"),
    ];

    for (pattern_re, description) in &redos_patterns {
        if let Ok(re) = regex::Regex::new(pattern_re) {
            if re.is_match(pattern) {
                return Err(format!(
                    "Regex contains potentially slow pattern (ReDoS risk): {description}"
                ));
            }
        }
    }

    // BEAD-004: Compile with size limits to prevent excessive memory usage
    regex::RegexBuilder::new(pattern)
        .size_limit(1024 * 1024) // 1MB compiled size limit
        .dfa_size_limit(1024 * 1024) // 1MB DFA size limit
        .build()
        .map(|_| ())
        .map_err(|e| format!("Invalid or too complex regex pattern '{pattern}': {e}"))
}

#[derive(Parser, Debug)]
#[command(
    name = "doc_transformer",
    version = "5.0",
    about = "Transform documentation into AI-optimized knowledge structures",
    long_about = "
doc_transformer v5.0 - The AI-Optimized Documentation Indexer

USAGE:
  doc_transformer scrape <URL> --output <DIR>    # Scrape a documentation site
  doc_transformer index <SOURCE> --output <DIR>  # Index local markdown files
  doc_transformer ingest <URL> --output <DIR>    # Scrape + index in one step
  doc_transformer <SOURCE> <OUTPUT>              # Legacy mode (same as index)

OUTPUT:
  llms.txt      - AI entry point (read this first)
  INDEX.json    - Machine-readable index with chunks and DAG
  COMPASS.md    - Human-readable navigation
  docs/         - Transformed documents with frontmatter
  chunks/       - Semantic chunks with context prefix
",
    // Disable automatic exit on error so we can return exit code 1 for validation errors
    // instead of clap's default exit code 2
    disable_help_subcommand = true,
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Legacy: Source directory (use 'index' subcommand instead)
    #[arg(value_name = "SOURCE", required = false)]
    source_dir: Option<PathBuf>,

    /// Legacy: Output directory (use 'index' subcommand instead)
    #[arg(value_name = "OUTPUT", required = false)]
    output_dir: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Search indexed documentation using BM25
    Search {
        /// Query string to search for
        #[arg(value_name = "QUERY")]
        query: String,

        /// Directory containing INDEX.json (required)
        #[arg(short, long, value_name = "DIR")]
        index_dir: PathBuf,

        /// Maximum number of results to return
        #[arg(
            short = 'n',
            long,
            default_value = "10",
            value_parser = validate_limit_cli,
            allow_hyphen_values = true
        )]
        limit: usize,

        /// Disable colored output
        #[arg(long)]
        no_color: bool,

        /// Output structured JSON for machine parsing
        #[arg(long)]
        json: bool,
    },

    /// Scrape a documentation website to local markdown files
    Scrape {
        /// URL of the documentation site to scrape
        #[arg(value_name = "URL")]
        url: String,

        /// Output directory for scraped content
        #[arg(short, long, value_name = "DIR")]
        output: PathBuf,

        /// Disable sitemap.xml discovery (use crawling instead)
        #[arg(long = "no-sitemap", action = clap::ArgAction::SetTrue)]
        no_sitemap: bool,

        /// Regex pattern to filter URLs by path
        #[arg(short, long, value_name = "REGEX")]
        filter: Option<String>,

        /// Delay between requests in milliseconds (0-60000)
        #[arg(short, long, default_value = "250", value_parser = validate_delay, allow_hyphen_values = true)]
        delay: u64,

        /// Request timeout in seconds (1-600)
        #[arg(long, default_value = "30", value_parser = validate_timeout_secs, allow_hyphen_values = true)]
        request_timeout_secs: u64,

        /// Max spider retries (0 disables spider retry)
        #[arg(long, default_value = "3", value_parser = validate_retry_count, allow_hyphen_values = true)]
        max_retries: u32,

        /// Redirect policy: loose (default), strict, none
        #[arg(long, default_value = "loose", value_parser = parse_redirect_policy)]
        redirect_policy: RedirectPolicy,

        /// Max bytes per page (spider-level, before transform)
        #[arg(long, value_parser = validate_positive_bytes)]
        max_page_bytes: Option<u64>,

        /// Max total bytes across crawl (spider-level)
        #[arg(long, value_parser = validate_positive_bytes)]
        max_total_bytes: Option<u64>,

        /// Concurrency (1-2, default 1) capped for politeness
        #[arg(long, default_value = "1", value_parser = validate_concurrency_limit, allow_hyphen_values = true)]
        concurrency: usize,

        /// Filter pages by BM25 relevance to query
        #[arg(short, long, value_name = "QUERY")]
        query: Option<String>,

        /// Minimum BM25 score to keep a page (default: 0.1, range: 0.0-10.0)
        #[arg(long, default_value = "0.1", value_parser = validate_threshold, allow_hyphen_values = true)]
        threshold: f32,
    },

    /// Clone and index Git-hosted documentation
    IngestGit {
        /// Git repository URL to clone
        #[arg(value_name = "REPO_URL")]
        repo_url: String,

        /// Output directory for indexed content
        #[arg(short, long, value_name = "DIR")]
        output: PathBuf,

        /// Git branch to checkout (default: main)
        #[arg(long)]
        branch: Option<String>,

        /// Clone depth (0 = full, 1 = shallow/faster)
        #[arg(long, default_value = "1")]
        depth: u32,

        /// Project name for llms.txt header
        #[arg(long)]
        project_name: Option<String>,
    },

    /// Index local markdown files into AI-optimized structure
    Index {
        /// Source directory containing markdown files
        #[arg(value_name = "SOURCE")]
        source: PathBuf,

        /// Output directory for indexed content
        #[arg(short, long, value_name = "DIR")]
        output: PathBuf,

        /// Generate llms.txt entry point files
        #[arg(long, default_value = "true")]
        llms_txt: bool,

        /// Project name for llms.txt header
        #[arg(long, default_value = "Documentation")]
        project_name: String,

        /// Project description for llms.txt
        #[arg(long, default_value = "AI-optimized documentation index")]
        project_desc: String,

        /// Path to category rules config file
        #[arg(long, value_name = "FILE")]
        category_config: Option<PathBuf>,

        /// Maximum number of related chunks per document (1-100, default: 20)
        #[arg(long, value_name = "N", default_value = "20", value_parser = validate_max_related_chunks, allow_hyphen_values = true)]
        max_related_chunks: usize,

        /// Maximum number of chunk keywords to include in similarity (0-50, default: 12)
        #[arg(long, value_name = "N", default_value = "12", value_parser = validate_max_chunk_keywords, allow_hyphen_values = true)]
        max_chunk_keywords: usize,

        /// HNSW graph connectivity parameter (4-64, default: 16)
        #[arg(long, value_name = "M", default_value = "16", value_parser = validate_hnsw_m, allow_hyphen_values = true)]
        hnsw_m: usize,

        /// HNSW graph construction effort (50-1000, default: 200)
        #[arg(long, value_name = "EF", default_value = "200", value_parser = validate_hnsw_ef_construction, allow_hyphen_values = true)]
        hnsw_ef_construction: usize,

        /// Maximum document size in bytes (default: 10MB, warn at 5MB)
        #[arg(long, value_parser = validate_positive_bytes)]
        max_document_bytes: Option<u64>,
    },

    /// Scrape and index in one step
    Ingest {
        /// URL of the documentation site
        #[arg(value_name = "URL")]
        url: String,

        /// Output directory for final indexed content
        #[arg(short, long, value_name = "DIR")]
        output: PathBuf,

        /// Regex pattern to filter URLs by path
        #[arg(short, long, value_name = "REGEX")]
        filter: Option<String>,

        /// Delay between requests in milliseconds (0-60000)
        #[arg(short, long, default_value = "250", value_parser = validate_delay, allow_hyphen_values = true)]
        delay: u64,

        /// Request timeout in seconds (1-600)
        #[arg(long, default_value = "30", value_parser = validate_timeout_secs, allow_hyphen_values = true)]
        request_timeout_secs: u64,

        /// Max spider retries (0 disables spider retry)
        #[arg(long, default_value = "3", value_parser = validate_retry_count, allow_hyphen_values = true)]
        max_retries: u32,

        /// Redirect policy: loose (default), strict, none
        #[arg(long, default_value = "loose", value_parser = parse_redirect_policy)]
        redirect_policy: RedirectPolicy,

        /// Max bytes per page (spider-level, before transform)
        #[arg(long, value_parser = validate_positive_bytes)]
        max_page_bytes: Option<u64>,

        /// Max total bytes across crawl (spider-level)
        #[arg(long, value_parser = validate_positive_bytes)]
        max_total_bytes: Option<u64>,

        /// Concurrency (1-2, default 1) capped for politeness
        #[arg(long, default_value = "1", value_parser = validate_concurrency_limit, allow_hyphen_values = true)]
        concurrency: usize,

        /// Filter pages by BM25 relevance to query
        #[arg(short, long, value_name = "QUERY")]
        query: Option<String>,

        /// Minimum BM25 score to keep a page (default: 0.1, range: 0.0-10.0)
        #[arg(long, default_value = "0.1", value_parser = validate_threshold, allow_hyphen_values = true)]
        threshold: f32,

        /// Project name for llms.txt header
        #[arg(long)]
        project_name: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Build command with error handling that returns exit code 1 for validation errors
    // instead of clap's default exit code 2
    let cmd = Cli::command();

    // Try to parse, handling validation errors with exit code 2
    // (per contract: invalid argument values like --max-related-chunks outside 1-100 exit with code 2)
    let cli = match cmd.try_get_matches() {
        Ok(matches) => matches,
        Err(e) => {
            // Check if it's a help/version request (these should exit with code 0)
            if e.kind() == clap::error::ErrorKind::DisplayHelp
                || e.kind() == clap::error::ErrorKind::DisplayVersion
            {
                // Print help/version and exit with code 0
                eprintln!("{}", e);
                process::exit(0);
            }
            // User input errors: validation errors and missing required args (exit code 1)
            // Pipeline errors: everything else (exit code 2)
            let user_input_errors = [
                clap::error::ErrorKind::ValueValidation,
                clap::error::ErrorKind::InvalidValue,
                clap::error::ErrorKind::MissingRequiredArgument,
            ];
            let exit_code = if user_input_errors.contains(&e.kind()) {
                1
            } else {
                2
            };
            eprintln!("{}", e);
            process::exit(exit_code);
        }
    };

    // Re-parse as Cli with the matches we already got
    let cli = match Cli::from_arg_matches(&cli) {
        Ok(cli) => cli,
        Err(e) => {
            // User input errors: validation errors and missing required args (exit code 1)
            // Pipeline errors: everything else (exit code 2)
            let user_input_errors = [
                clap::error::ErrorKind::ValueValidation,
                clap::error::ErrorKind::InvalidValue,
                clap::error::ErrorKind::MissingRequiredArgument,
            ];
            let exit_code = if user_input_errors.contains(&e.kind()) {
                1
            } else {
                2
            };
            eprintln!("{}", e);
            process::exit(exit_code);
        }
    };

    let mut search_context: Option<(bool, String)> = None;

    let result = match cli.command {
        Some(Commands::Search {
            query,
            index_dir,
            limit,
            no_color,
            json,
        }) => {
            search_context = Some((json, query.clone()));
            run_search(&query, &index_dir, limit, !no_color, json)
        }

        Some(Commands::Scrape {
            url,
            output,
            no_sitemap,
            filter,
            delay,
            query,
            threshold,
            request_timeout_secs,
            max_retries,
            redirect_policy,
            max_page_bytes,
            max_total_bytes,
            concurrency,
        }) => {
            let config = ScrapeCommandConfig {
                sitemap_strategy: if no_sitemap {
                    SitemapStrategy::CrawlOnly
                } else {
                    SitemapStrategy::UseSitemap
                },
                filter,
                delay,
                query,
                threshold,
                request_timeout_secs,
                max_retries,
                redirect_policy,
                max_page_bytes,
                max_total_bytes,
                concurrency_limit: concurrency,
            };
            run_scrape(&url, &output, &config).await
        }

        Some(Commands::Index {
            source,
            output,
            llms_txt,
            project_name,
            project_desc,
            category_config,
            max_related_chunks,
            max_chunk_keywords,
            hnsw_m,
            hnsw_ef_construction,
            max_document_bytes,
        }) => {
            let config = IndexConfig {
                generate_llms: llms_txt,
                project_name,
                project_desc,
                category_config,
                max_related_chunks,
                max_chunk_keywords,
                hnsw_m,
                hnsw_ef_construction,
                max_document_bytes: max_document_bytes.unwrap_or(10 * 1024 * 1024),
            };
            run_index(&source, &output, &config)
        }

        Some(Commands::IngestGit {
            repo_url,
            output,
            branch,
            depth: _,
            project_name,
        }) => {
            // Git ingestion using git2 with functional principles
            let temp_dir = output.join(".git-clone");
            std::fs::create_dir_all(&temp_dir)?;

            // Idempotency check: skip clone if .git exists
            let git_dir = temp_dir.join(".git");
            if git_dir.exists() {
                println!("[GIT CLONE] Existing .git directory detected");
                println!("  Checking for markdown files...");
            } else {
                println!("[GIT CLONE] Cloning repository...");

                // Build repo builder with branch configuration
                let mut builder = git2::build::RepoBuilder::new();

                // Configure branch if specified
                if let Some(branch_name) = branch.as_deref() {
                    builder.branch(branch_name);
                }

                // Clone the repository
                builder
                    .clone(&repo_url, &temp_dir)
                    .map_err(|e| anyhow::anyhow!("Failed to clone repository: {e}"))?;

                println!("  ✓ Clone successful");
                println!();
            }

            // Collect markdown files using functional collection
            let markdown_files: Vec<_> = walkdir::WalkDir::new(&temp_dir)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|entry| entry.file_type().is_file())
                .filter_map(|entry| {
                    entry.path().extension().and_then(|ext| {
                        ext.eq_ignore_ascii_case("md")
                            .then(|| entry.path().to_path_buf())
                    })
                })
                .collect();

            println!("[DISCOVER] Found {} markdown files", markdown_files.len());
            println!();

            let index_config = IndexConfig {
                generate_llms: true,
                project_name: project_name.as_ref().cloned().unwrap_or_else(|| {
                    url::Url::parse(&repo_url)
                        .ok()
                        .and_then(|u| {
                            u.path_segments()
                                .and_then(|mut s| s.next_back())
                                .map(|s| s.to_string())
                        })
                        .unwrap_or_else(|| "Documentation".to_string())
                }),
                project_desc: format!("Documentation cloned from {repo_url}"),
                ..Default::default()
            };

            run_index(&temp_dir, &output, &index_config)?;

            println!();
            println!("{}", "=".repeat(70));
            println!("GIT INGEST COMPLETE");
            println!("{}", "=".repeat(70));
            println!("Source:     {repo_url}");
            println!("Output:     {}", output.display());
            println!("Documents:  {}", markdown_files.len());
            println!("Entry:      llms.txt (AI should read this first)");
            println!("{}", "=".repeat(70));
            println!();
            Ok(())
        }

        Some(Commands::Ingest {
            url,
            output,
            filter,
            delay,
            request_timeout_secs,
            max_retries,
            redirect_policy,
            max_page_bytes,
            max_total_bytes,
            concurrency,
            query,
            threshold,
            project_name,
        }) => {
            let config = IngestConfig {
                filter,
                delay,
                request_timeout_secs,
                max_retries,
                redirect_policy,
                max_page_bytes,
                max_total_bytes,
                concurrency_limit: concurrency,
                query,
                threshold,
                project_name,
            };
            run_ingest(&url, &output, &config).await
        }

        None => {
            // Legacy mode: two positional arguments
            if let (Some(source), Some(output)) = (cli.source_dir, cli.output_dir) {
                run_index(&source, &output, &IndexConfig::default())
            } else {
                anyhow::bail!(
                    "Usage: doc_transformer <SOURCE> <OUTPUT>\n   or: doc_transformer scrape <URL> --output <DIR>\n   or: doc_transformer index <SOURCE> --output <DIR>\n   or: doc_transformer ingest <URL> --output <DIR>\n\nRun 'doc_transformer --help' for more information."
                );
            }
        }
    };

    // Handle result with proper exit code mapping
    match result {
        Ok(()) => Ok(()),
        Err(err) => {
            if let Some((json_mode, search_query)) = search_context {
                if json_mode {
                    let error_message = err.to_string();

                    if error_message.starts_with(SEARCH_JSON_ALREADY_EMITTED_PREFIX) {
                        // JSON was already successfully emitted with status "no_results" or "partial"
                        // Exit code 0 indicates successful completion (consistent with JSON status)
                        process::exit(0);
                    }

                    let json_error = serde_json::json!({
                        "status": "error",
                        "query": search_query,
                        "error": error_message,
                    });
                    println!("{}", serde_json::to_string_pretty(&json_error)?);
                    process::exit(1);
                }
            }

            // Map error to exit code for consistent error handling
            // Exit 1: user input errors, Exit 2: pipeline errors
            let exit_code = map_error_to_exit_code(&err);
            eprintln!("Error: {err}");
            process::exit(exit_code);
        }
    }
}

/// Map errors to exit codes per contract requirements:
///
/// - Exit 0: Success
/// - Exit 1: User input errors (invalid arguments, bad format, missing files)
/// - Exit 2: Pipeline/internal errors (transform failures, corrupt data, network errors)
///
/// This ensures consistent exit codes across all validation layers:
/// - Parser-level validation (via clap value_parser) now exits with 1 (user error)
/// - Runtime validation also exits with 1 for user input errors
fn map_error_to_exit_code(err: &anyhow::Error) -> i32 {
    // Check for validation errors (user input) - these should exit with 1
    let error_string = err.to_string();
    let error_string_lower = error_string.to_lowercase();

    // User input error patterns (explicit matches - high precision)
    // These are errors where the user provided invalid input
    let user_input_patterns = [
        "must be",
        "cannot be",
        "missing",
        "required",
        "not found",
        "must be at least",
        "must be at most",
        "must be positive",
        "too long",
        "too short",
        "out of range",
        "query cannot be empty",
        "query too long",
        "limit must be",
        "another index operation appears to be running",
        "invalid url",
        "invalid config",
        "invalid or too complex regex",
        "regex parse error",
        "permission denied",
        "no markdown files found",
        "cannot index empty",
    ];

    let is_user_input = user_input_patterns
        .iter()
        .any(|pattern| error_string_lower.contains(pattern));

    if is_user_input {
        // User input error -> exit 1
        return 1;
    }

    // "no results found" is NOT an error - it's a valid result state
    // Exit code 0 means success (even with empty results)
    // Exit code 1 is for actual errors (invalid index, missing args, etc.)
    if error_string_lower.contains("no results found") {
        // No results is a valid result -> exit 0 (success)
        return 0;
    }

    // Pipeline error -> exit 2
    // These include: IO errors, transform failures, network errors, corrupt data
    // Anything that isn't a user input error is a pipeline error
    2
}

/// Validate query length to prevent DoS attacks and resource exhaustion
///
/// Constraints:
/// - Maximum 1000 bytes (prevents regex compilation timeouts)
/// - None/empty queries allowed (no filtering)
fn validate_query_length(query: &Option<&str>) -> Result<()> {
    const MAX_QUERY_LENGTH: usize = 1000;

    if let Some(q) = query {
        let byte_count = q.len();
        if byte_count > MAX_QUERY_LENGTH {
            anyhow::bail!("Query too long ({byte_count} bytes, maximum {MAX_QUERY_LENGTH})");
        }
    }

    Ok(())
}

/// Apply BM25 query filtering to scraped pages (extracted common logic)
///
/// Design by Contract:
/// - **Preconditions:**
///   - pages may be empty (returns empty with count 0)
///   - query may be None (returns pages unchanged)
///   - threshold and pages are valid
/// - **Postconditions:**
///   - Returns filtered pages and count of removed pages
///   - All returned pages scored >= threshold (if query provided)
///   - Logs filtering statistics
///
/// Edge Cases Handled:
/// - Query is None → returns all pages unchanged
/// - Query is empty string → returns all pages (empty query scores all = 0)
/// - threshold <= 0.0 → no filtering applied
/// - threshold = 1.0 → very strict (only highly relevant pages)
/// - All pages filtered out → logs warning and returns empty
/// - Pages with identical content → same score, all kept or all removed together
fn apply_query_filter(
    pages: Vec<scrape::ScrapedPage>,
    query: Option<&str>,
    threshold: f32,
) -> Result<Vec<scrape::ScrapedPage>> {
    let Some(raw_query) = query else {
        return Ok(pages);
    };

    let query = raw_query.trim();
    if query.is_empty() || threshold <= 0.0 {
        return Ok(pages);
    }

    let avg_doc_length = if pages.is_empty() {
        0.0
    } else {
        let total_words: usize = pages.iter().map(|page| page.word_count).sum();
        total_words as f32 / pages.len() as f32
    };

    let original_len = pages.len();
    let kept_pages: Vec<scrape::ScrapedPage> = pages
        .into_iter()
        .filter(|page| {
            let score = filter::bm25_score(&page.markdown, query, avg_doc_length);
            score.is_finite() && score >= threshold
        })
        .collect();

    let removed_count = original_len.saturating_sub(kept_pages.len());
    println!(
        "  Kept: {} pages matching \"{}\" (removed: {})",
        kept_pages.len(),
        query,
        removed_count
    );

    if kept_pages.is_empty() {
        println!("\n  WARNING: All pages filtered out by query.");
        println!("  Consider lowering the --threshold value.");
        anyhow::bail!("All pages filtered out by query '{query}' (threshold: {threshold})");
    }

    Ok(kept_pages)
}

/// Run the scrape command
async fn run_scrape(url: &str, output: &Path, config: &ScrapeCommandConfig) -> Result<()> {
    let _validated_url = scrape::validate_url(url)?;

    // Validate query length before processing (prevents DoS)
    let query_ref = config.query.as_deref();
    validate_query_length(&query_ref)?;

    // Validate filter regex pattern if provided
    if let Some(ref filter) = config.filter {
        validate_filter_regex(filter).map_err(|e| anyhow::anyhow!(e))?;
    }

    println!("\n{}", "=".repeat(70));
    println!("DOC_TRANSFORMER v5.0 - SCRAPE");
    println!("{}\n", "=".repeat(70));

    println!("[SCRAPE] Target: {url}");
    println!(
        "  Options: sitemap={:?}, delay={}ms, timeout={}s, retries={}, concurrency={}",
        config.sitemap_strategy,
        config.delay,
        config.request_timeout_secs,
        config.max_retries,
        config.concurrency_limit
    );
    println!(
        "  Redirect: {:?}, page_bytes={:?}, total_bytes={:?}",
        config.redirect_policy, config.max_page_bytes, config.max_total_bytes
    );
    if let Some(ref f) = config.filter {
        println!("  Filter: {f}");
    }
    println!();

    let scrape_config = scrape::ScrapeConfig {
        base_url: url.to_string(),
        sitemap_strategy: config.sitemap_strategy,
        path_filter: config.filter.clone(),
        delay_ms: config.delay,
        max_page_size_bytes: config.max_page_bytes.unwrap_or(DEFAULT_MAX_PAGE_SIZE_BYTES),
        max_total_size_bytes: config
            .max_total_bytes
            .unwrap_or(DEFAULT_MAX_TOTAL_SIZE_BYTES),
        spider_max_page_bytes: config.max_page_bytes,
        spider_max_total_bytes: config.max_total_bytes,
        request_timeout_secs: config.request_timeout_secs,
        max_retries: config.max_retries,
        redirect_policy: config.redirect_policy.clone(),
        concurrency_limit: config.concurrency_limit,
        ..Default::default()
    };

    println!("[SCRAPE] Starting crawl...");
    let mut result = scrape::scrape_site(&scrape_config).await?;

    // Check for partial/total failure BEFORE further processing
    // Exit with code 2 if any pages failed to scrape
    if result.error_count > 0 {
        println!();
        println!("{}", "=".repeat(70));
        println!("SCRAPE COMPLETE (PARTIAL FAILURE)");
        println!("{}", "=".repeat(70));
        println!("Success: {} pages", result.success_count);
        println!("Errors:  {} pages failed", result.error_count);
        println!();
        println!("Hint: Check .scrape/manifest.json for error details");
        println!("{}\n", "=".repeat(70));
        process::exit(2);
    }

    println!("  Scraped: {} pages", result.success_count);

    // Apply BM25 filtering if query is provided (extracted common logic)
    result.pages = apply_query_filter(result.pages, query_ref, config.threshold)?;
    result.success_count = result.pages.len();

    // Detect potential SPA (JavaScript-rendered site) BEFORE validation
    // This ensures we show helpful message even when scraping fails
    let spa_detection = scrape::detect_potential_spa(&result);
    if let Some(ref warning) = spa_detection.warning_message {
        println!();
        println!("{}", "=".repeat(70));
        println!("{}", warning);
        println!("{}\n", "=".repeat(70));
    }

    // Validate that at least one page was scraped (fail fast on invalid URLs)
    scrape::validate_scrape_result(&result)?;

    println!("\n[WRITE] Saving to {}", output.display());
    std::fs::create_dir_all(output)?;
    scrape::write_scraped_pages(&result, output)?;

    println!("\n{}", "=".repeat(70));
    println!("SCRAPE COMPLETE");
    println!("{}", "=".repeat(70));
    println!("Output:  {}", output.display());
    println!("Pages:   {} scraped", result.success_count);
    if result.error_count > 0 {
        println!("Errors:  {} pages failed", result.error_count);
    }
    println!("Files:   .scrape/*.md + manifest.json");
    println!("{}\n", "=".repeat(70));

    Ok(())
}

/// Validate output path is a directory or can be created
fn validate_output_path(path: &Path) -> Result<()> {
    if path.exists() {
        if !path.is_dir() {
            anyhow::bail!(
                "Output path must be a directory, but got: {}",
                path.display()
            );
        }

        // Check write permission on existing directory
        check_write_permission(path)?;
    } else {
        let parent = path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid output path: {}", path.display()))?;

        if !parent.exists() {
            anyhow::bail!("Parent directory does not exist: {}", parent.display());
        }

        if !parent.is_dir() {
            anyhow::bail!("Parent path is not a directory: {}", parent.display());
        }

        // Check write permission on parent directory (where we'll create the new dir)
        check_write_permission(parent)?;
    }

    Ok(())
}

/// Check if we have write permission to a directory
/// Attempts to create a temporary file to verify write access
fn check_write_permission(dir: &Path) -> Result<()> {
    // Try to create a temporary file to verify write access
    // Using .permission_check.tmp as a unique name unlikely to conflict
    let test_file = dir.join(".permission_check.tmp");

    match std::fs::write(&test_file, b"") {
        Ok(_) => {
            // Clean up the test file
            let _ = std::fs::remove_file(&test_file);
            Ok(())
        }
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            anyhow::bail!(
                "Permission denied: cannot write to output directory '{}'\n  \
                 Hint: Check directory permissions or run with appropriate access",
                dir.display()
            );
        }
        Err(e) => {
            // Other errors (e.g., read-only filesystem) - still report but with context
            anyhow::bail!(
                "Cannot write to output directory '{}': {}\n  \
                 Hint: Check if the directory exists and you have write access",
                dir.display(),
                e
            );
        }
    }
}

struct OutputLock {
    lock_path: PathBuf,
    file: std::fs::File,
}

#[derive(Debug, Serialize, Deserialize)]
struct OutputLockMetadata {
    pid: u32,
    start_time: u64,
    created_at_unix_secs: u64,
}

const OUTPUT_LOCK_STALE_AFTER_SECS: u64 = 60 * 30;

impl Drop for OutputLock {
    fn drop(&mut self) {
        // Unlock the file first (release lock)
        let _ = self.file.unlock();
        // Then remove the lock file
        let _ = std::fs::remove_file(&self.lock_path);
    }
}

fn acquire_output_lock(output: &Path) -> Result<OutputLock> {
    std::fs::create_dir_all(output)?;
    let lock_path = output.join(".doc_transformer.lock");

    // Attempt to create lock file - this is atomic at the OS level
    // Use a loop to handle stale lock reclamation race condition
    let mut retries = 0;
    const MAX_RETRIES: usize = 3;

    while retries < MAX_RETRIES {
        // Check and reclaim stale lock if needed BEFORE trying to create
        if lock_path.exists() && should_reclaim_stale_lock(&lock_path) {
            eprintln!("[WARN] Reclaiming stale lock at {}", lock_path.display());
            // Try to remove stale lock - ignore error as another process may have taken it
            let _ = std::fs::remove_file(&lock_path);
        }

        // Try to create the lock file atomically
        match OpenOptions::new()
            .create_new(true)
            .write(true)
            .read(true)
            .open(&lock_path)
        {
            Ok(mut file) => {
                // Write lock metadata
                let metadata = OutputLockMetadata {
                    pid: process::id(),
                    start_time: get_process_start_time(process::id()).unwrap_or(0),
                    created_at_unix_secs: now_unix_secs(),
                };

                if let Err(error) = serde_json::to_writer(&mut file, &metadata) {
                    let _ = std::fs::remove_file(&lock_path);
                    return Err(anyhow::anyhow!("Failed to write lock metadata: {error}"));
                }

                // Flush to ensure metadata is written before acquiring lock
                if let Err(error) = file.flush() {
                    let _ = std::fs::remove_file(&lock_path);
                    return Err(anyhow::anyhow!("Failed to flush lock file: {error}"));
                }

                // Acquire exclusive file lock - this is the key to preventing race conditions
                // The lock is automatically released when the file is closed (in Drop)
                if let Err(error) = file.lock_exclusive() {
                    let _ = std::fs::remove_file(&lock_path);
                    return Err(anyhow::anyhow!("Failed to acquire file lock: {error}"));
                }

                return Ok(OutputLock {
                    lock_path: lock_path.clone(),
                    file,
                });
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                // Lock file exists - check if we should retry (stale lock was reclaimed)
                retries = retries.saturating_add(1);
                if retries < MAX_RETRIES {
                    // Brief sleep to allow other process to release lock
                    std::thread::sleep(std::time::Duration::from_millis(10));
                    continue;
                }
                // Max retries exceeded - report lock conflict
                return Err(anyhow::anyhow!(
                    "Another index operation appears to be running for '{}'. Remove '{}' if stale.",
                    output.display(),
                    lock_path.display()
                ));
            }
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to acquire output lock '{}': {e}",
                    lock_path.display()
                ));
            }
        }
    }

    // Should not reach here, but just in case
    Err(anyhow::anyhow!(
        "Failed to acquire output lock after {} retries",
        MAX_RETRIES
    ))
}

fn now_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs())
}

/// Get process start time in clock ticks since system boot.
/// Reads from /proc/<pid>/stat, field 22 (starttime).
fn get_process_start_time(pid: u32) -> Option<u64> {
    let stat_path = PathBuf::from("/proc").join(pid.to_string()).join("stat");
    std::fs::read_to_string(&stat_path)
        .ok()
        .and_then(|content| {
            content.split(')').nth(1).and_then(|rest| {
                let fields: Vec<&str> = rest.split_whitespace().collect();
                fields.get(19).and_then(|s| s.parse::<u64>().ok())
            })
        })
}

fn process_is_alive(pid: u32, start_time: u64) -> bool {
    let current_start_time = get_process_start_time(process::id()).unwrap_or(0);

    if pid == process::id() {
        return current_start_time == start_time;
    }

    get_process_start_time(pid)
        .map(|actual_start_time| actual_start_time == start_time)
        .unwrap_or(false)
}

fn read_lock_metadata(lock_path: &Path) -> Option<OutputLockMetadata> {
    std::fs::File::open(lock_path)
        .ok()
        .and_then(|file| serde_json::from_reader(file).ok())
}

fn should_reclaim_stale_lock(lock_path: &Path) -> bool {
    if let Some(metadata) = read_lock_metadata(lock_path) {
        let age_secs = now_unix_secs().saturating_sub(metadata.created_at_unix_secs);
        return !process_is_alive(metadata.pid, metadata.start_time)
            || age_secs > OUTPUT_LOCK_STALE_AFTER_SECS;
    }

    // FIX: If we can't read the lock metadata (empty/malformed),
    // treat it as stale immediately - likely from crashed process
    true
}

/// Run the index command (main pipeline)
fn run_index(source: &Path, output: &Path, config: &IndexConfig) -> Result<()> {
    validate_output_path(output)?;
    let _output_lock = acquire_output_lock(output)?;

    println!("\n{}", "=".repeat(70));
    println!("DOC_TRANSFORMER v5.0 (Knowledge DAG + llms.txt)");
    println!("{}\n", "=".repeat(70));

    // Log graph configuration parameters
    println!("[CONFIG] Graph Parameters:");
    println!(
        "  max_related_chunks: {} (default: 20)",
        config.max_related_chunks
    );
    println!(
        "  max_chunk_keywords: {} (default: 12)",
        config.max_chunk_keywords
    );
    println!("  hnsw_m: {} (default: 16)", config.hnsw_m);
    println!(
        "  hnsw_ef_construction: {} (default: 200)",
        config.hnsw_ef_construction
    );
    println!();

    // STEP 1: DISCOVER
    println!("[STEP 1] DISCOVER");
    let (files, discover_manifest) = discover::discover_files(source)?;
    println!("  Found {} files\n", files.len());

    // Exit with error if no markdown files found (user error - exit code 1)
    if files.is_empty() {
        anyhow::bail!(
            "No markdown files found in source directory. Cannot index empty source.\n\
             Hint: Ensure the source directory contains files with .md, .mdx, .markdown, .txt, or .rst extensions."
        );
    }

    // STEP 2: ANALYZE
    // Use manifest.source_dir for analysis (handles both directory and single file cases)
    let analysis_base_path = PathBuf::from(&discover_manifest.source_dir);
    println!("[STEP 2] ANALYZE");
    let analyze_result = analyze::analyze_files(
        &files,
        &analysis_base_path,
        config.category_config.as_deref(),
    )?;

    // Report failed files if any - this is an error condition
    if !analyze_result.failed_files.is_empty() {
        // Collect error messages for comprehensive error reporting
        let error_summary = analyze_result
            .failed_files
            .iter()
            .map(|f| format!("{}: {}", f.source_path, f.error))
            .collect::<Vec<_>>()
            .join("; ");
        anyhow::bail!("analysis failed: {}", error_summary);
    }

    let analyses = analyze_result.analyses;
    let categories = analyze::count_categories(&analyses);
    println!("  Processed {} files\n", analyses.len());
    println!(
        "  Categories: ref={} concept={} tutorial={} ops={} meta={}\n",
        categories.get("ref").unwrap_or(&0),
        categories.get("concept").unwrap_or(&0),
        categories.get("tutorial").unwrap_or(&0),
        categories.get("ops").unwrap_or(&0),
        categories.get("meta").unwrap_or(&0)
    );

    // STEP 3: ASSIGN IDs
    println!("[STEP 3] ASSIGN IDs");
    let (analyses, link_map) = assign::assign_ids(analyses);
    println!("  Generated {} IDs\n", analyses.len());

    // STEP 4: TRANSFORM
    println!("[STEP 4] TRANSFORM");
    let transform_result = transform::transform_all(&analyses, &link_map, output)?;
    println!(
        "  {}/{} files ({} errors)\n",
        transform_result.success_count, transform_result.total_count, transform_result.error_count
    );

    // STEP 5: CHUNK (Hierarchical)
    println!("[STEP 5] CHUNK");
    let chunks_result =
        chunking_adapter::chunk_all(&analyses, &link_map, output, config.max_document_bytes)?;
    println!(
        "  Generated {} chunks from {} documents",
        chunks_result.total_chunks, chunks_result.document_count
    );
    println!(
        "  Hierarchical: {} summary, {} standard, {} detailed",
        chunks_result.summary_chunks, chunks_result.standard_chunks, chunks_result.detailed_chunks
    );
    println!("  ~512 tokens/chunk with contextual prefixes\n");

    // STEP 6: VALIDATE (before artifact writing - ensures atomic failure)
    println!("[STEP 6] VALIDATE");
    let validation_result = validate::validate_all(output)?;
    println!(
        "  {}/{} files passed ({} errors, {} warnings)\n",
        validation_result.files_passed,
        validation_result.files_checked,
        validation_result.total_errors,
        validation_result.total_warnings
    );

    // Bail early if validation fails - no artifacts written yet
    if validation_result.total_errors > 0 {
        anyhow::bail!(
            "Validation failed: {} errors found across {} files",
            validation_result.total_errors,
            validation_result.files_checked
        );
    }

    // STEP 7: INDEX + GRAPH
    println!("[STEP 7] INDEX + GRAPH");
    index::build_and_write_index(
        &analyses,
        &link_map,
        &chunks_result,
        output,
        &config.project_name,
        Some(config.max_related_chunks),
        Some(config.hnsw_m),
        Some(config.hnsw_ef_construction),
        Some(config.max_chunk_keywords),
    )?;
    index::build_and_write_compass(&analyses, &link_map, output)?;
    println!("  Created INDEX.json and COMPASS.md\n");

    // STEP 8: LLMS.TXT + AGENTS.MD
    if config.generate_llms {
        println!("[STEP 8] LLMS.TXT + AGENTS.MD");
        let llms_config = llms::LlmsConfig {
            project_name: config.project_name.clone(),
            project_description: config.project_desc.clone(),
            generate_full: true,
            ..Default::default()
        };
        llms::generate_llms_txt(&analyses, &link_map, &llms_config, output)?;
        llms::generate_agents_md(&analyses, &link_map, &llms_config, output)?;
        if llms_config.generate_full {
            llms::generate_llms_full_txt(&analyses, &link_map, output)?;
            println!("  Created llms.txt, llms-full.txt, and AGENTS.md\n");
        } else {
            println!("  Created llms.txt and AGENTS.md\n");
        }
    }

    // FINAL SUMMARY
    println!("{}", "=".repeat(70));
    println!("COMPLETE");
    println!("{}", "=".repeat(70));
    println!("Source:     {}", source.display());
    println!("Output:     {}", output.display());
    println!("Documents:  {}", analyses.len());
    println!("Chunks:     {}", chunks_result.total_chunks);
    println!(
        "Validation: {}/{} passed",
        validation_result.files_passed, validation_result.files_checked
    );
    if config.generate_llms {
        println!("Entry:      llms.txt (AI should read this first)");
    }
    println!("{}\n", "=".repeat(70));

    Ok(())
}

/// Run the ingest command (scrape + index)
async fn run_ingest(url: &str, output: &Path, config: &IngestConfig) -> Result<()> {
    let _validated_url = scrape::validate_url(url)?;

    // Extract fields from config
    let filter = config.filter.clone();
    let delay = config.delay;
    let max_page_bytes = config.max_page_bytes;
    let max_total_bytes = config.max_total_bytes;
    let query = config.query.clone();
    let threshold = config.threshold;
    let project_name = config.project_name.clone();

    // Validate query length before processing (prevents DoS)
    let query_ref = query.as_deref();
    validate_query_length(&query_ref)?;

    // Validate filter regex pattern if provided
    if let Some(ref f) = filter {
        validate_filter_regex(f).map_err(|e| anyhow::anyhow!(e))?;
    }

    println!("\n{}", "=".repeat(70));
    println!("DOC_TRANSFORMER v5.0 - INGEST (Scrape + Index)");
    println!("{}\n", "=".repeat(70));

    // Phase 1: Scrape
    println!("[PHASE 1] SCRAPE\n");

    let scrape_config = scrape::ScrapeConfig {
        base_url: url.to_string(),
        sitemap_strategy: SitemapStrategy::UseSitemap,
        path_filter: filter,
        delay_ms: delay,
        max_page_size_bytes: max_page_bytes.unwrap_or(DEFAULT_MAX_PAGE_SIZE_BYTES),
        max_total_size_bytes: max_total_bytes.unwrap_or(DEFAULT_MAX_TOTAL_SIZE_BYTES),
        spider_max_page_bytes: max_page_bytes,
        spider_max_total_bytes: max_total_bytes,
        request_timeout_secs: config.request_timeout_secs,
        max_retries: config.max_retries,
        redirect_policy: config.redirect_policy.clone(),
        concurrency_limit: config.concurrency_limit,
        ..Default::default()
    };

    let mut scrape_result = scrape::scrape_site(&scrape_config).await?;
    println!(
        "  Scraped {} pages from {}",
        scrape_result.success_count, url
    );

    // Apply BM25 filtering if query is provided (extracted common logic)
    scrape_result.pages = apply_query_filter(scrape_result.pages, query_ref, threshold)?;
    scrape_result.success_count = scrape_result.pages.len();

    // Validate that at least one page was scraped (fail fast on invalid URLs)
    scrape::validate_scrape_result(&scrape_result)?;

    // Detect potential SPA (JavaScript-rendered site) and warn user
    let spa_detection = scrape::detect_potential_spa(&scrape_result);
    if let Some(ref warning) = spa_detection.warning_message {
        println!();
        println!("{}", "=".repeat(70));
        println!("{}", warning);
        println!("{}", "=".repeat(70));
    }

    println!();

    // Write scraped content to temp location within output
    let scrape_dir = output.join(".scrape");
    std::fs::create_dir_all(&scrape_dir)?;
    scrape::write_scraped_pages(&scrape_result, output)?;

    // Phase 2: Index
    println!("[PHASE 2] INDEX\n");

    // Derive project name from URL if not provided
    let name = project_name.unwrap_or_else(|| {
        url::Url::parse(url)
            .map(|u| u.host_str().unwrap_or("Documentation").to_string())
            .unwrap_or_else(|_| "Documentation".to_string())
    });

    // Use the scrape directory as source for indexing
    let index_config = IndexConfig {
        generate_llms: true,
        project_name: name,
        project_desc: format!("Documentation scraped from {url}"),
        ..Default::default()
    };
    run_index(&scrape_dir, output, &index_config)?;

    Ok(())
}

/// Run the search command using Tantivy (with fallback to BM25)
///
/// Strategy:
/// 1. Try to use Tantivy index if available (faster, better features)
/// 2. Fall back to INDEX.json + manual BM25 scoring if index missing
/// 3. Display results with scores and metadata
///
/// Note: Returns non-zero exit code if advanced search fails, even if fallback succeeds
#[derive(Debug, Serialize)]
struct CliSearchResult {
    rank: usize,
    category: String,
    title: String,
    path: String,
    summary: String,
    score: f32,
    backend: String,
}

fn emit_search_output(
    query: &str,
    backend: &str,
    results: &[CliSearchResult],
    limit: usize,
    json_output: bool,
    status: &str,
    advanced_search_failed: bool,
) -> Result<()> {
    if json_output {
        let output = serde_json::json!({
            "status": status,
            "query": query,
            "backend": backend,
            "advanced_search_failed": advanced_search_failed,
            "requested_limit": limit,
            "result_count": results.len(),
            "results": results,
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("\n{}", "=".repeat(70));
        println!("DOC_TRANSFORMER SEARCH - Tantivy + BM25");
        println!("{}\n", "=".repeat(70));
        println!("Query: \"{query}\"");
        println!("Using {backend}\n");

        if results.is_empty() {
            println!("No results found for \"{query}\"");
        } else {
            println!("Results:\n");
            for result in results {
                println!(
                    "{}. [{}] {} (score: {:.2})",
                    result.rank, result.category, result.title, result.score
                );
                println!("   Path: {}", result.path);
                println!("   {}\n", result.summary);
            }

            println!("{}", "=".repeat(70));
            println!(
                "Showing {} of {} results",
                results.len().min(limit),
                results.len()
            );
            println!("{}\n", "=".repeat(70));
        }
    }

    Ok(())
}

fn run_search(
    query: &str,
    index_dir: &Path,
    limit: usize,
    _use_color: bool,
    json_output: bool,
) -> Result<()> {
    const MAX_QUERY_WORDS: usize = 100;

    // Validate query using centralized validation
    let query = validate::validate_query(query).map_err(|e| anyhow::anyhow!("{e}"))?;

    // Validate word count (additional constraint beyond basic validation)
    let word_count = query.split_whitespace().count();
    if word_count > MAX_QUERY_WORDS {
        anyhow::bail!("Query has too many terms ({word_count} words, max {MAX_QUERY_WORDS})");
    }

    let index_path = index_dir.join("INDEX.json");
    if !index_path.exists() {
        anyhow::bail!("INDEX.json not found in {}", index_dir.display());
    }

    // Track whether advanced search failed (for exit code purposes)
    let mut advanced_search_failed = false;

    // Try Tantivy index first (only if it already exists)
    match doc_transformer::search::open_existing_index(index_dir) {
        Ok(Some(index)) => match doc_transformer::search::search_index(&index, query, limit) {
            Ok(results) => {
                let cli_results: Vec<CliSearchResult> = results
                    .iter()
                    .enumerate()
                    .map(|(i, result)| {
                        let summary_short = if result.summary.chars().count() > 80 {
                            let truncated: String = result.summary.chars().take(77).collect();
                            format!("{truncated}...")
                        } else {
                            result.summary.clone()
                        };

                        CliSearchResult {
                            rank: i.saturating_add(1),
                            category: result.category.clone(),
                            title: result.title.clone(),
                            path: result.path.clone(),
                            summary: summary_short,
                            score: result.score,
                            backend: "tantivy".to_string(),
                        }
                    })
                    .collect();

                let status = if cli_results.is_empty() {
                    "no_results"
                } else {
                    "ok"
                };
                emit_search_output(
                    query,
                    "tantivy",
                    &cli_results,
                    limit,
                    json_output,
                    status,
                    false,
                )?;

                if cli_results.is_empty() && json_output {
                    anyhow::bail!("{SEARCH_JSON_ALREADY_EMITTED_PREFIX}:no_results");
                }

                if cli_results.is_empty() {
                    anyhow::bail!("No results found for '{query}'");
                }

                return Ok(());
            }
            Err(e) => {
                // Mark that advanced search failed - we will return error later if fallback succeeds
                advanced_search_failed = true;
                // Fall through to JSON-based search with informative message
                if !json_output {
                    println!(
                        "Note: Query contains special characters unsupported by advanced search."
                    );
                    println!("  Reason: {e}");
                    println!("  Tip: Try simpler terms or remove special characters.");
                    println!("  Falling back to basic search...\n");
                }
            }
        },
        Ok(None) => {}
        Err(e) => {
            advanced_search_failed = true;
            if !json_output {
                println!("Note: Advanced index unavailable or corrupted.");
                println!("  Reason: {e}");
                println!("  Falling back to basic search...\n");
            }
        }
    }

    // Fallback: Use INDEX.json + manual BM25 scoring
    use serde_json::Value;

    let index_content = std::fs::read_to_string(&index_path)?;
    let index: Value = serde_json::from_str(&index_content)?;

    // Extract documents
    let documents = index["documents"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Invalid INDEX.json: missing documents array"))?;

    let _chunks = index["chunks"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Invalid INDEX.json: missing chunks array"))?;

    let avg_doc_length = if !documents.is_empty() {
        let total_words: usize = documents
            .iter()
            .filter_map(|d| d["word_count"].as_u64())
            .filter_map(|c| usize::try_from(c).ok())
            .sum();
        if total_words > 0 {
            total_words as f32 / documents.len() as f32
        } else {
            100.0
        }
    } else {
        100.0
    };

    // Score each document
    let mut results: Vec<(f32, &Value)> = documents
        .iter()
        .map(|doc| {
            let title = doc["title"].as_str().unwrap_or("");
            let summary = doc["summary"].as_str().unwrap_or("");
            let searchable = format!("{title} {summary}");
            let score = filter::bm25_score(&searchable, query, avg_doc_length);
            (score, doc)
        })
        .filter(|(score, _)| *score > 0.0)
        .collect();

    // Sort by score descending
    results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    let cli_results: Vec<CliSearchResult> = results
        .iter()
        .take(limit)
        .enumerate()
        .map(|(i, (score, doc))| {
            let title = doc["title"].as_str().unwrap_or("Untitled").to_string();
            let path = doc["path"].as_str().unwrap_or("").to_string();
            let category = doc["category"].as_str().unwrap_or("").to_string();
            let summary = doc["summary"].as_str().unwrap_or("");
            let summary_short = if summary.chars().count() > 80 {
                let truncated: String = summary.chars().take(77).collect();
                format!("{truncated}...")
            } else {
                summary.to_string()
            };

            CliSearchResult {
                rank: i.saturating_add(1),
                category,
                title,
                path,
                summary: summary_short,
                score: *score,
                backend: "bm25-fallback".to_string(),
            }
        })
        .collect();

    // Determine status: partial when fallback was used (even with zero results),
    // no_results only when primary search succeeded but found nothing,
    // ok when primary search found results
    let status = if advanced_search_failed {
        "partial"
    } else if cli_results.is_empty() {
        "no_results"
    } else {
        "ok"
    };

    emit_search_output(
        query,
        "bm25-fallback",
        &cli_results,
        limit,
        json_output,
        status,
        advanced_search_failed,
    )?;

    if cli_results.is_empty() && json_output {
        anyhow::bail!("{SEARCH_JSON_ALREADY_EMITTED_PREFIX}:no_results");
    }

    if cli_results.is_empty() {
        anyhow::bail!("No results found for '{query}'");
    }

    // If advanced search failed but fallback succeeded, return error to signal partial failure
    if advanced_search_failed && json_output {
        anyhow::bail!("{SEARCH_JSON_ALREADY_EMITTED_PREFIX}:partial");
    }

    if advanced_search_failed {
        anyhow::bail!(
            "Advanced search failed but basic search succeeded - query may need simplification"
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |duration| duration.as_nanos());
        std::env::temp_dir().join(format!("{prefix}-{nanos}"))
    }

    fn write_lock_metadata(lock_path: &Path, metadata: &OutputLockMetadata) {
        let file_result = std::fs::File::create(lock_path);
        assert!(file_result.is_ok());

        if let Ok(file) = file_result {
            let write_result = serde_json::to_writer(file, metadata);
            assert!(write_result.is_ok());
        }
    }

    #[test]
    fn test_should_reclaim_stale_lock_when_pid_not_alive() {
        let temp_dir = unique_temp_dir("lock-reclaim-dead-pid");
        let create_dir_result = std::fs::create_dir_all(&temp_dir);
        assert!(create_dir_result.is_ok());

        let lock_path = temp_dir.join(".doc_transformer.lock");
        let metadata = OutputLockMetadata {
            pid: u32::MAX,
            start_time: 0,
            created_at_unix_secs: now_unix_secs(),
        };

        write_lock_metadata(&lock_path, &metadata);

        assert!(should_reclaim_stale_lock(&lock_path));

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_should_reclaim_stale_lock_when_too_old() {
        let temp_dir = unique_temp_dir("lock-reclaim-old");
        let create_dir_result = std::fs::create_dir_all(&temp_dir);
        assert!(create_dir_result.is_ok());

        let lock_path = temp_dir.join(".doc_transformer.lock");
        let current_start_time = get_process_start_time(process::id()).unwrap_or(0);
        let metadata = OutputLockMetadata {
            pid: process::id(),
            start_time: current_start_time,
            created_at_unix_secs: now_unix_secs().saturating_sub(OUTPUT_LOCK_STALE_AFTER_SECS + 5),
        };

        write_lock_metadata(&lock_path, &metadata);

        assert!(should_reclaim_stale_lock(&lock_path));

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_should_not_reclaim_fresh_live_lock() {
        let temp_dir = unique_temp_dir("lock-reclaim-live");
        let create_dir_result = std::fs::create_dir_all(&temp_dir);
        assert!(create_dir_result.is_ok());

        let lock_path = temp_dir.join(".doc_transformer.lock");
        let current_start_time = get_process_start_time(process::id()).unwrap_or(0);
        let metadata = OutputLockMetadata {
            pid: process::id(),
            start_time: current_start_time,
            created_at_unix_secs: now_unix_secs(),
        };

        write_lock_metadata(&lock_path, &metadata);

        assert!(!should_reclaim_stale_lock(&lock_path));

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_should_reclaim_stale_lock_when_pid_recycled() {
        let temp_dir = unique_temp_dir("lock-reclaim-recycled");
        let create_dir_result = std::fs::create_dir_all(&temp_dir);
        assert!(create_dir_result.is_ok());

        let lock_path = temp_dir.join(".doc_transformer.lock");

        let current_start_time = get_process_start_time(process::id()).unwrap_or(0);
        let wrong_start_time = current_start_time.wrapping_add(1000);

        let metadata = OutputLockMetadata {
            pid: process::id(),
            start_time: wrong_start_time,
            created_at_unix_secs: now_unix_secs(),
        };

        write_lock_metadata(&lock_path, &metadata);

        assert!(should_reclaim_stale_lock(&lock_path));

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_should_reclaim_stale_lock_when_empty_file() {
        let temp_dir = unique_temp_dir("lock-reclaim-empty");
        let create_dir_result = std::fs::create_dir_all(&temp_dir);
        assert!(create_dir_result.is_ok());

        let lock_path = temp_dir.join(".doc_transformer.lock");

        // Create empty lock file
        let file_result = std::fs::File::create(&lock_path);
        assert!(file_result.is_ok());

        // Empty file should be treated as stale
        assert!(should_reclaim_stale_lock(&lock_path));

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_should_reclaim_stale_lock_when_malformed_json() {
        let temp_dir = unique_temp_dir("lock-reclaim-malformed");
        let create_dir_result = std::fs::create_dir_all(&temp_dir);
        assert!(create_dir_result.is_ok());

        let lock_path = temp_dir.join(".doc_transformer.lock");

        // Create lock file with malformed JSON
        let write_result = std::fs::write(&lock_path, "{not valid json");
        assert!(write_result.is_ok());

        // Malformed JSON should be treated as stale
        assert!(should_reclaim_stale_lock(&lock_path));

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_validate_query_none() {
        // None query should always pass (no filtering)
        let query: Option<&str> = None;
        assert!(validate_query_length(&query).is_ok());
    }

    #[test]
    fn test_validate_query_empty_string() {
        // Empty query should pass (no filtering, returns all)
        let query: Option<&str> = Some("");
        assert!(validate_query_length(&query).is_ok());
    }

    #[test]
    fn test_validate_query_single_char() {
        // Single character should pass
        let query: Option<&str> = Some("a");
        assert!(validate_query_length(&query).is_ok());
    }

    #[test]
    fn test_validate_query_short() {
        // Short query well below limit
        let query: Option<&str> = Some("test query");
        assert!(validate_query_length(&query).is_ok());
    }

    #[test]
    fn test_validate_query_at_limit() {
        // Query exactly at 1000 byte limit should pass
        let long_query = "a".repeat(1000);
        let query: Option<&str> = Some(&long_query);
        assert!(validate_query_length(&query).is_ok());
    }

    #[test]
    fn test_validate_query_exceeds_limit() {
        // Query exceeding 1000 bytes should fail
        let too_long_query = "a".repeat(1001);
        let query: Option<&str> = Some(&too_long_query);
        let result = validate_query_length(&query);

        assert!(result.is_err());
        // Convert error to string for validation without unwrap
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("1001"));
            assert!(msg.contains("1000"));
            assert!(msg.contains("too long"));
        }
    }

    #[test]
    fn test_validate_query_unicode_within_limit() {
        // UTF-8 characters: "café" = 5 bytes, should pass
        let query: Option<&str> = Some("café");
        assert!(validate_query_length(&query).is_ok());
    }

    #[test]
    fn test_validate_query_unicode_exceeds_limit() {
        // Euro sign "€" = 3 bytes each, 334 repetitions = 1002 bytes, should fail
        let euro_query = "€".repeat(334);
        assert_eq!(euro_query.len(), 1002); // Verify it's actually 1002 bytes

        let query: Option<&str> = Some(&euro_query);
        let result = validate_query_length(&query);

        assert!(result.is_err());
        // Convert error to string for validation without unwrap
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("1002"));
        }
    }

    #[test]
    fn test_validate_query_unicode_at_byte_limit() {
        // Create a query that's exactly 1000 bytes with Unicode
        // "€" is 3 bytes, so 333 reps = 999 bytes + 1 ASCII char = 1000 bytes
        let euro_query = format!("{}a", "€".repeat(333));
        assert_eq!(euro_query.len(), 1000);

        let query: Option<&str> = Some(&euro_query);
        assert!(validate_query_length(&query).is_ok());
    }

    #[test]
    fn test_validate_query_whitespace_only() {
        // Whitespace-only query should pass (treated as empty after trim)
        let query: Option<&str> = Some("   ");
        // Note: This passes validation, but may be filtered later by BM25
        assert!(validate_query_length(&query).is_ok());
    }

    #[test]
    fn test_validate_query_special_characters() {
        // Query with special characters should pass (no regex issues at validation stage)
        let query: Option<&str> = Some("rust-lang & systems *2025*");
        assert!(validate_query_length(&query).is_ok());
    }

    #[test]
    fn test_validate_query_newlines() {
        // Query with embedded newlines (from CLI) should validate on byte count
        let query: Option<&str> = Some("line1\nline2\nline3");
        assert!(validate_query_length(&query).is_ok());
    }

    #[test]
    fn test_validate_query_near_limit_minus_one() {
        // Query at 999 bytes (one below limit) should pass
        let query_999 = "a".repeat(999);
        let query: Option<&str> = Some(&query_999);
        assert!(validate_query_length(&query).is_ok());
    }

    #[test]
    fn test_validate_query_far_exceeds_limit() {
        // Query way over limit should fail with appropriate message
        let way_too_long = "a".repeat(10000);
        let query: Option<&str> = Some(&way_too_long);
        let result = validate_query_length(&query);

        assert!(result.is_err());
        // Convert error to string for validation without unwrap
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("10000"));
        }
    }

    #[test]
    fn test_validate_query_mixed_unicode_ascii() {
        // Mix of ASCII and Unicode, totaling within limit
        let mixed = "Hello 世界 Rust €uro دعم தமிழ்";
        let query: Option<&str> = Some(mixed);
        // Mixed UTF-8 should pass if under 1000 bytes
        if mixed.len() <= 1000 {
            assert!(validate_query_length(&query).is_ok());
        }
    }

    #[test]
    fn test_validate_query_binary_looking_bytes() {
        // Some control characters and high bytes (valid UTF-8)
        let query: Option<&str> = Some("café\t\n\r ");
        assert!(validate_query_length(&query).is_ok());
    }

    // Threshold validation tests

    #[test]
    fn test_validate_threshold_zero() {
        // Zero threshold should pass (no filtering)
        let result = validate_threshold("0.0");
        assert!(result.is_ok());
        assert_eq!(result.map(|v| v.to_string()).unwrap_or_default(), "0");
    }

    #[test]
    fn test_validate_threshold_positive() {
        // Valid positive threshold
        let result = validate_threshold("0.5");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_threshold_at_upper_bound() {
        // Maximum valid threshold
        let result = validate_threshold("10.0");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_threshold_negative_rejected() {
        // Negative threshold should fail
        let result = validate_threshold("-0.5");
        assert!(result.is_err());
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("non-negative"));
            assert!(msg.contains("-0.5"));
        }
    }

    #[test]
    fn test_validate_threshold_too_large() {
        // Threshold above 10.0 should fail
        let result = validate_threshold("10.1");
        assert!(result.is_err());
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("10.0"));
            assert!(msg.contains("10.1"));
        }
    }

    #[test]
    fn test_validate_threshold_invalid_string() {
        // Non-numeric input should fail
        let result = validate_threshold("invalid");
        assert!(result.is_err());
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("must be a number"));
        }
    }

    #[test]
    fn test_validate_threshold_default_value() {
        // Default value 0.1 should pass
        let result = validate_threshold("0.1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_threshold_small_positive() {
        // Small positive value should pass
        let result = validate_threshold("0.001");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_threshold_very_negative() {
        // Very negative value should fail
        let result = validate_threshold("-100.0");
        assert!(result.is_err());
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("non-negative"));
        }
    }

    // Delay validation tests

    #[test]
    fn test_validate_delay_zero() {
        // Zero delay should pass (no delay between requests)
        let result = validate_delay("0");
        assert!(result.is_ok());
        assert_eq!(result.map(|v| v.to_string()).unwrap_or_default(), "0");
    }

    #[test]
    fn test_validate_delay_positive() {
        // Valid positive delay
        let result = validate_delay("500");
        assert!(result.is_ok());
        assert_eq!(result.map(|v| v.to_string()).unwrap_or_default(), "500");
    }

    #[test]
    fn test_validate_delay_default_value() {
        // Default value 250 should pass
        let result = validate_delay("250");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_delay_negative_one_rejected() {
        // Negative delay -1 should fail with clear message
        let result = validate_delay("-1");
        assert!(result.is_err());
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("non-negative"));
        }
    }

    #[test]
    fn test_validate_delay_very_negative_rejected() {
        // Very negative delay should fail
        let result = validate_delay("-9999");
        assert!(result.is_err());
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("non-negative"));
        }
    }

    #[test]
    fn test_validate_delay_at_upper_bound() {
        // Maximum valid delay (60 seconds)
        let result = validate_delay("60000");
        assert!(result.is_ok());
        assert_eq!(result.map(|v| v.to_string()).unwrap_or_default(), "60000");
    }

    #[test]
    fn test_validate_delay_exceeds_upper_bound() {
        // Delay over 60 seconds should fail
        let result = validate_delay("60001");
        assert!(result.is_err());
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("60000"));
            assert!(msg.contains("60001"));
        }
    }

    #[test]
    fn test_validate_delay_invalid_string() {
        // Non-numeric input should fail
        let result = validate_delay("invalid");
        assert!(result.is_err());
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("must be an integer"));
        }
    }

    #[test]
    fn test_validate_delay_fractional_rejected() {
        // Fractional delay should fail (must be integer)
        let result = validate_delay("250.5");
        assert!(result.is_err());
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("must be an integer"));
        }
    }

    #[test]
    fn test_validate_retry_count_bounds() {
        let zero = validate_retry_count("0");
        assert!(zero.is_ok());
        assert_eq!(zero.unwrap_or_default(), 0);

        let max_ok = validate_retry_count(&u8::MAX.to_string());
        assert!(max_ok.is_ok());
        assert_eq!(max_ok.unwrap_or_default(), u8::MAX as u32);

        let too_high = validate_retry_count(&(u8::MAX as i32 + 1).to_string());
        assert!(too_high.is_err());
    }

    #[test]
    fn test_validate_timeout_secs_bounds() {
        let min_ok = validate_timeout_secs("1");
        assert!(min_ok.is_ok());
        assert_eq!(min_ok.unwrap_or_default(), 1);

        let max_ok = validate_timeout_secs("600");
        assert!(max_ok.is_ok());
        assert_eq!(max_ok.unwrap_or_default(), 600);

        let too_low = validate_timeout_secs("0");
        assert!(too_low.is_err());
        let too_high = validate_timeout_secs("601");
        assert!(too_high.is_err());
    }

    #[test]
    fn test_validate_positive_bytes() {
        let ok = validate_positive_bytes("1024");
        assert!(ok.is_ok());
        assert_eq!(ok.unwrap_or_default(), 1024);

        let zero = validate_positive_bytes("0");
        assert!(zero.is_err());
        let negative = validate_positive_bytes("-5");
        assert!(negative.is_err());
    }

    #[test]
    fn test_parse_redirect_policy_variants() {
        let loose = parse_redirect_policy("loose");
        assert!(matches!(loose, Ok(RedirectPolicy::Loose)));

        let strict = parse_redirect_policy("STRICT");
        assert!(matches!(strict, Ok(RedirectPolicy::Strict)));

        let none = parse_redirect_policy("None");
        assert!(matches!(none, Ok(RedirectPolicy::None)));

        let invalid = parse_redirect_policy("invalid");
        assert!(invalid.is_err());
    }

    #[test]
    fn test_validate_concurrency_limit_bounds() {
        let one = validate_concurrency_limit("1");
        assert!(one.is_ok());
        assert_eq!(one.unwrap_or_default(), 1);

        let two = validate_concurrency_limit("2");
        assert!(two.is_ok());
        assert_eq!(two.unwrap_or_default(), 2);

        let zero = validate_concurrency_limit("0");
        assert!(zero.is_err());

        let too_high = validate_concurrency_limit("3");
        assert!(too_high.is_err());
    }

    // Limit validation tests

    #[test]
    fn test_validate_limit_one() {
        // Minimum valid limit
        let result = validate::validate_limit("1");
        assert!(result.is_ok());
        assert_eq!(result.map(|v| v.to_string()).unwrap_or_default(), "1");
    }

    #[test]
    fn test_validate_limit_positive() {
        // Valid positive limit
        let result = validate::validate_limit("10");
        assert!(result.is_ok());
        assert_eq!(result.map(|v| v.to_string()).unwrap_or_default(), "10");
    }

    #[test]
    fn test_validate_limit_default_value() {
        // Default value 10 should pass
        let result = validate::validate_limit("10");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_limit_at_upper_bound() {
        // Maximum valid limit
        let result = validate::validate_limit("1000");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_limit_negative_one_rejected() {
        // Negative limit -1 should fail with clear message
        let result = validate::validate_limit("-1");
        assert!(result.is_err());
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("positive"));
            assert!(msg.contains("negative"));
        }
    }

    #[test]
    fn test_validate_limit_zero_rejected() {
        // Zero limit should fail
        let result = validate::validate_limit("0");
        assert!(result.is_err());
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("at least 1"));
        }
    }

    #[test]
    fn test_validate_limit_exceeds_upper_bound() {
        // Limit above 1000 should fail
        let result = validate::validate_limit("1001");
        assert!(result.is_err());
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("1000"));
            assert!(msg.contains("1001"));
        }
    }

    #[test]
    fn test_validate_limit_very_negative_rejected() {
        // Very negative value should fail
        let result = validate::validate_limit("-999");
        assert!(result.is_err());
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("positive"));
            assert!(msg.contains("negative"));
        }
    }

    #[test]
    fn test_validate_limit_invalid_string() {
        // Non-numeric input should fail
        let result = validate::validate_limit("invalid");
        assert!(result.is_err());
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("positive") || msg.contains("Limit must"));
        }
    }

    // ==========================================================================
    // COMPREHENSIVE DELAY AND THRESHOLD VALIDATION TESTS (P1 delay-overflow, threshold-overflow)
    // ==========================================================================

    // Additional delay validation tests for P1 delay-overflow

    #[test]
    fn test_delay_very_large_negative_rejected() {
        let result = validate_delay("-99999");
        assert!(result.is_err());
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("non-negative"));
        }
    }

    #[test]
    fn test_delay_boundary_values() {
        // Test 59999 (just under max - should pass)
        let result_under = validate_delay("59999");
        assert!(result_under.is_ok());

        // Test 60001 (just over max - should fail)
        let result_over = validate_delay("60001");
        assert!(result_over.is_err());
    }

    #[test]
    fn test_delay_empty_string_rejected() {
        let result = validate_delay("");
        assert!(result.is_err());
    }

    #[test]
    fn test_delay_whitespace_rejected() {
        let result = validate_delay("   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_delay_various_valid_values() {
        let valid_values = [0, 1, 100, 500, 1000, 5000, 10000, 30000, 59999, 60000];
        for value in valid_values {
            let result = validate_delay(&value.to_string());
            assert!(
                matches!(result, Ok(v) if v == value),
                "delay={value} should be accepted"
            );
        }
    }

    // Additional threshold validation tests for P1 threshold-overflow

    #[test]
    fn test_threshold_very_negative_rejected() {
        let result = validate_threshold("-999.0");
        assert!(result.is_err());
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("non-negative"));
        }
    }

    #[test]
    fn test_threshold_boundary_values() {
        // Test 9.9 (just under max - should pass)
        let result_under = validate_threshold("9.9");
        assert!(result_under.is_ok());

        // Test 10.01 (just over max - should fail)
        let result_over = validate_threshold("10.01");
        assert!(result_over.is_err());
    }

    #[test]
    fn test_threshold_small_positive_values() {
        let values = ["0.001", "0.01", "0.05", "0.099"];
        for value in values {
            let result = validate_threshold(value);
            assert!(result.is_ok(), "threshold={value} should be accepted");
        }
    }

    #[test]
    fn test_threshold_common_filtering_values() {
        // Common BM25 threshold values used in practice
        let values = ["0.0", "0.1", "0.5", "1.0", "2.0", "5.0"];
        for value in values {
            let result = validate_threshold(value);
            assert!(
                result.is_ok(),
                "threshold={value} should be accepted (common filtering value)"
            );
        }
    }

    #[test]
    fn test_threshold_empty_string_rejected() {
        let result = validate_threshold("");
        assert!(result.is_err());
    }

    #[test]
    fn test_threshold_scientific_notation() {
        // Valid scientific notation within range
        let result1 = validate_threshold("1e-1"); // 0.1
        assert!(result1.is_ok(), "threshold=1e-1 should be accepted");

        // Invalid scientific notation exceeding range
        let result2 = validate_threshold("1e2"); // 100.0
        assert!(result2.is_err(), "threshold=1e2 (100.0) should be rejected");
    }

    #[test]
    fn test_threshold_precision_at_boundary() {
        // Test values very close to the boundary
        let result1 = validate_threshold("9.9999");
        assert!(result1.is_ok(), "threshold=9.9999 should be accepted");

        let result2 = validate_threshold("10.0001");
        assert!(result2.is_err(), "threshold=10.0001 should be rejected");
    }

    #[test]
    fn test_threshold_integer_input() {
        let result1 = validate_threshold("0");
        assert!(result1.is_ok(), "threshold=0 (integer) should be accepted");

        let result2 = validate_threshold("5");
        assert!(matches!(result2, Ok(v) if v == 5.0));

        let result3 = validate_threshold("10");
        assert!(result3.is_ok(), "threshold=10 (integer) should be accepted");

        let result4 = validate_threshold("11");
        assert!(
            result4.is_err(),
            "threshold=11 (integer) should be rejected"
        );
    }

    // Overflow protection tests (P1 focus)

    #[test]
    fn test_delay_overflow_protection_u64_max() {
        let huge_value = "18446744073709551615"; // u64::MAX
        let result = validate_delay(huge_value);
        assert!(
            result.is_err(),
            "Huge delay value should be rejected to prevent overflow"
        );
        // The error will be about integer parsing (exceeds i64::MAX) or the 60000 limit
        // Either way, it should be rejected
    }

    #[test]
    fn test_delay_overflow_protection_i64_max() {
        let huge_value = "9223372036854775807"; // i64::MAX
        let result = validate_delay(huge_value);
        assert!(
            result.is_err(),
            "Huge delay value should be rejected to prevent overflow"
        );
    }

    #[test]
    fn test_threshold_overflow_protection() {
        let huge_value = "999999999.9";
        let result = validate_threshold(huge_value);
        assert!(
            result.is_err(),
            "Huge threshold value should be rejected to prevent overflow"
        );
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("10.0"), "Error should mention the 10.0 limit");
        }
    }

    #[test]
    fn test_threshold_infinity_rejected() {
        let result = validate_threshold("inf");
        assert!(result.is_err(), "threshold=inf should be rejected");
    }

    #[test]
    fn test_threshold_nan_handled() {
        let result = validate_threshold("NaN");
        assert!(result.is_err(), "threshold=NaN should be rejected");
    }

    #[test]
    fn test_delay_arithmetic_overflow_prevention() {
        // Test a value that could cause overflow in delay calculations
        let dangerous_value = "100000"; // Would be 100 seconds, over 60 second limit
        let result = validate_delay(dangerous_value);
        assert!(
            result.is_err(),
            "Delay value that could cause arithmetic overflow should be rejected"
        );
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(
                msg.contains("60000"),
                "Error should mention the 60000 limit"
            );
        }
    }

    #[test]
    fn test_delay_i32_max_rejected() {
        let result = validate_delay("2147483647"); // i32::MAX
        assert!(result.is_err(), "i32::MAX delay should be rejected");
    }

    // Edge case tests

    #[test]
    fn test_delay_leading_zeros() {
        let result = validate_delay("000250");
        assert!(
            matches!(result, Ok(v) if v == 250),
            "delay='000250' should parse to 250"
        );
    }

    #[test]
    fn test_delay_plus_sign() {
        let result = validate_delay("+250");
        // Plus sign might be accepted or rejected depending on parser
        match result {
            Ok(v) => {
                assert_eq!(v, 250, "delay='+250' should parse to 250");
            }
            Err(_) => {
                // Also acceptable - parser might reject the plus sign
            }
        }
    }

    #[test]
    fn test_threshold_leading_zeros() {
        let result = validate_threshold("000.5");
        assert!(result.is_ok(), "threshold='000.5' should be accepted");
    }

    #[test]
    fn test_threshold_plus_sign() {
        let result = validate_threshold("+5.0");
        // Plus sign handling - accept whatever the parser does
        // The important thing is we don't crash
        let _ = result;
    }

    #[test]
    fn test_delay_one_millisecond() {
        let result = validate_delay("1");
        assert!(
            matches!(result, Ok(v) if v == 1),
            "delay=1 should be accepted"
        );
    }

    #[test]
    fn test_threshold_very_small_positive() {
        let result = validate_threshold("0.000001");
        assert!(result.is_ok(), "threshold=0.000001 should be accepted");
    }

    // Range combination tests

    #[test]
    fn test_delay_values_outside_range_rejected() {
        let outside_values: &[&str] = &["60001", "70000", "100000", "1000000"];
        for value in outside_values {
            let result = validate_delay(value);
            assert!(result.is_err(), "delay={value} should be rejected");
        }
    }

    #[test]
    fn test_threshold_key_values() {
        let key_values = [
            ("0.0", true),
            ("0.1", true),
            ("1.0", true),
            ("5.0", true),
            ("10.0", true),
            ("10.1", false),
            ("11.0", false),
            ("100.0", false),
        ];

        for (value, should_pass) in key_values {
            let result = validate_threshold(value);
            assert_eq!(
                result.is_ok(),
                should_pass,
                "threshold={value} expectation mismatch"
            );
        }
    }

    #[test]
    fn test_delay_all_negative_rejected() {
        let negative_values = &["-1", "-100", "-1000", "-60000"];
        for value in negative_values {
            let result = validate_delay(value);
            assert!(result.is_err(), "delay={value} should be rejected");
        }
    }

    #[test]
    fn test_threshold_all_negative_rejected() {
        let negative_values = &["-0.001", "-0.1", "-1.0", "-10.0"];
        for value in negative_values {
            let result = validate_threshold(value);
            assert!(result.is_err(), "threshold={value} should be rejected");
        }
    }

    // Comprehensive edge case tests for P1 overflow protection

    #[test]
    fn test_delay_999999_rejected() {
        let result = validate_delay("999999");
        assert!(
            result.is_err(),
            "delay=999999 should be rejected for exceeding 60 second limit"
        );
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(
                msg.contains("60000"),
                "Error should mention the 60000 limit"
            );
        }
    }

    #[test]
    fn test_threshold_100_0_rejected() {
        let result = validate_threshold("100.0");
        assert!(
            result.is_err(),
            "threshold=100.0 should be rejected for exceeding maximum"
        );
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("10.0"), "Error should mention the 10.0 limit");
        }
    }

    #[test]
    fn test_threshold_negative_0_1_rejected() {
        let result = validate_threshold("-0.1");
        assert!(
            result.is_err(),
            "threshold=-0.1 should be rejected as negative"
        );
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(
                msg.contains("non-negative"),
                "Error should mention non-negative requirement"
            );
        }
    }

    #[test]
    fn test_delay_negative_one_rejected() {
        let result = validate_delay("-1");
        assert!(result.is_err(), "delay=-1 should be rejected as negative");
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(
                msg.contains("non-negative"),
                "Error should mention non-negative requirement"
            );
        }
    }

    #[test]
    fn test_delay_60001_exceeds_maximum() {
        let result = validate_delay("60001");
        assert!(
            result.is_err(),
            "delay=60001 should be rejected for exceeding 60 second limit"
        );
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(
                msg.contains("60000"),
                "Error should mention the 60000 limit"
            );
        }
    }

    #[test]
    fn test_threshold_10_1_exceeds_maximum() {
        let result = validate_threshold("10.1");
        assert!(
            result.is_err(),
            "threshold=10.1 should be rejected for exceeding maximum"
        );
        let err_msg = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err_msg {
            assert!(msg.contains("10.0"), "Error should mention the 10.0 limit");
        }
    }

    fn make_scraped_page(url: &str, markdown: &str, word_count: usize) -> scrape::ScrapedPage {
        scrape::ScrapedPage {
            url: url.to_string(),
            markdown: markdown.to_string(),
            title: "Title".to_string(),
            links: Vec::new(),
            headers: Vec::new(),
            word_count,
            slug: "slug".to_string(),
            filter_status: scrape::PageFilterStatus::Unfiltered,
            elements_removed: 0,
            density_score: 1.0,
        }
    }

    #[test]
    fn test_apply_query_filter_no_query_keeps_all_pages() {
        let pages = vec![
            make_scraped_page("https://example.com/a", "alpha beta", 2),
            make_scraped_page("https://example.com/b", "gamma delta", 2),
        ];

        let result = apply_query_filter(pages.clone(), None, 0.1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap_or_default().len(), pages.len());
    }

    #[test]
    fn test_apply_query_filter_filters_non_matching_pages() {
        let pages = vec![
            make_scraped_page("https://example.com/a", "rust async runtime", 3),
            make_scraped_page("https://example.com/b", "python data science", 3),
        ];

        let result = apply_query_filter(pages, Some("rust"), 0.1);
        assert!(result.is_ok());

        let kept = result.unwrap_or_default();
        assert_eq!(kept.len(), 1);
        assert!(kept[0].markdown.contains("rust"));
    }

    #[test]
    fn test_apply_query_filter_errors_when_all_filtered() {
        let pages = vec![
            make_scraped_page("https://example.com/a", "alpha beta", 2),
            make_scraped_page("https://example.com/b", "gamma delta", 2),
        ];

        let result = apply_query_filter(pages, Some("zzzzzz_no_match"), 0.1);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_query_filter_threshold_zero_keeps_all() {
        // When threshold is 0.0, all pages should be kept (no filtering)
        let pages = vec![
            make_scraped_page("https://example.com/a", "rust async", 2),
            make_scraped_page("https://example.com/b", "python data", 2),
        ];

        let result = apply_query_filter(pages.clone(), Some("rust"), 0.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap_or_default().len(), pages.len());
    }

    #[test]
    fn test_apply_query_filter_empty_query_keeps_all() {
        // When query is empty (after trimming), all pages should be kept
        let pages = vec![
            make_scraped_page("https://example.com/a", "rust async", 2),
            make_scraped_page("https://example.com/b", "python data", 2),
        ];

        let result = apply_query_filter(pages.clone(), Some("   "), 0.1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap_or_default().len(), pages.len());
    }

    #[test]
    fn test_apply_query_filter_with_different_thresholds() {
        // Test that higher threshold filters more pages
        let pages = vec![
            make_scraped_page("https://example.com/a", "rust rust rust rust", 4),
            make_scraped_page("https://example.com/b", "rust", 1),
            make_scraped_page("https://example.com/c", "python", 1),
        ];

        // With threshold 0.0, all pages kept
        let result0 = apply_query_filter(pages.clone(), Some("rust"), 0.0);
        assert!(result0.is_ok());
        assert_eq!(result0.unwrap_or_default().len(), 3);

        // With very high threshold, only high-scoring pages kept
        let result_high = apply_query_filter(pages, Some("rust"), 100.0);
        assert!(result_high.is_err()); // All filtered out due to high threshold
    }

    // BEAD-004: ReDoS protection tests

    #[test]
    fn test_filter_regex_rejects_nested_star_quantifiers() {
        let result = validate_filter_regex("(.*)*");
        assert!(result.is_err());
        let err = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err {
            assert!(msg.contains("ReDoS") || msg.contains("slow pattern"));
        }
    }

    #[test]
    fn test_filter_regex_rejects_nested_plus_quantifiers() {
        let result = validate_filter_regex("(.+)+");
        assert!(result.is_err());
        let err = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err {
            assert!(msg.contains("ReDoS") || msg.contains("slow pattern"));
        }
    }

    #[test]
    fn test_filter_regex_rejects_too_long_pattern() {
        let long_pattern = "a".repeat(501);
        let result = validate_filter_regex(&long_pattern);
        assert!(result.is_err());
        let err = result.as_ref().map_err(|e| e.to_string());
        if let Err(msg) = err {
            assert!(msg.contains("too long") || msg.contains("500"));
        }
    }

    #[test]
    fn test_filter_regex_accepts_safe_patterns() {
        let safe_patterns = ["^/docs/", r"\d+", "^api/v[0-9]+", "[a-z]+"];
        for pattern in &safe_patterns {
            let result = validate_filter_regex(pattern);
            assert!(
                result.is_ok(),
                "Safe pattern '{pattern}' should be accepted"
            );
        }
    }

    #[test]
    fn test_filter_regex_rejects_invalid_syntax() {
        let result = validate_filter_regex("[unclosed");
        assert!(result.is_err());
    }
}
