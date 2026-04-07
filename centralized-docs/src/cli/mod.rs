pub mod config;
pub mod mcp_cmd;
pub mod validation;

pub use mcp_cmd::McpCommand;

use clap::{Parser, Subcommand};
use spider::configuration::RedirectPolicy;
use std::path::PathBuf;
use validation::{
    parse_redirect_policy, validate_concurrency_limit, validate_connect_timeout_secs,
    validate_delay, validate_hnsw_ef_construction, validate_hnsw_m, validate_limit_cli,
    validate_max_chunk_keywords, validate_max_related_chunks, validate_positive_bytes,
    validate_retry_count, validate_threshold, validate_timeout_secs,
};

#[derive(Parser, Debug)]
#[command(
    name = "ctd",
    version = env!("CARGO_PKG_VERSION"),
    about = "Transform documentation into AI-optimized knowledge structures",
    long_about = concat!(
        "\nctd v",
        env!("CARGO_PKG_VERSION"),
        " - The AI-Optimized Documentation Indexer

USAGE:
  ctd scrape <URL> --output <DIR>    # Scrape a documentation site
  ctd index <SOURCE> --output <DIR>  # Index local markdown files
  ctd ingest <URL> --output <DIR>    # Scrape + index in one step

OUTPUT:
  llms.txt      - AI entry point (read this first)
  INDEX.json    - Machine-readable index with chunks and DAG
  NAVIGATION.md - Human-readable navigation
  docs/         - Transformed documents with frontmatter
  chunks/       - Semantic chunks with context prefix
 "
    ),
    // Disable automatic exit on error so we can return exit code 1 for validation errors
    // instead of clap's default exit code 2
    disable_help_subcommand = true,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
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

    /// Start the MCP server for AI agent integration
    Mcp {
        #[command(subcommand)]
        command: McpCommand,
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
        #[arg(short, long, default_value = "0", value_parser = validate_delay, allow_hyphen_values = true)]
        delay: u64,

        /// Request timeout in seconds (1-600)
        #[arg(long, default_value = "30", value_parser = validate_timeout_secs, allow_hyphen_values = true)]
        request_timeout_secs: u64,

        /// TCP connect timeout in seconds (1-60, default: 10)
        #[arg(long, default_value = "10", value_parser = validate_connect_timeout_secs, allow_hyphen_values = true)]
        connect_timeout_secs: u64,

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

        /// Concurrency (1-128, default 4) capped for politeness
        #[arg(long, default_value = "4", value_parser = validate_concurrency_limit, allow_hyphen_values = true)]
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

        /// Regex pattern to filter file paths (e.g. '^docs/en/' to only index English docs)
        #[arg(short, long, value_name = "REGEX")]
        filter: Option<String>,
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

        /// Generate AGENTS.md file for AI coding agents
        #[arg(long)]
        with_agents: bool,

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
        #[arg(short, long, default_value = "0", value_parser = validate_delay, allow_hyphen_values = true)]
        delay: u64,

        /// Request timeout in seconds (1-600)
        #[arg(long, default_value = "30", value_parser = validate_timeout_secs, allow_hyphen_values = true)]
        request_timeout_secs: u64,

        /// TCP connect timeout in seconds (1-60, default: 10)
        #[arg(long, default_value = "10", value_parser = validate_connect_timeout_secs, allow_hyphen_values = true)]
        connect_timeout_secs: u64,

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

        /// Concurrency (1-128, default 4) capped for politeness
        #[arg(long, default_value = "4", value_parser = validate_concurrency_limit, allow_hyphen_values = true)]
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

    /// Scrape a site and produce a change plan (Terraform-style plan)
    Watch {
        /// URL of the documentation site to watch
        #[arg(value_name = "URL")]
        url: String,

        /// Output directory for change reports
        #[arg(short, long, value_name = "DIR")]
        output: PathBuf,

        /// Path to the redb cache file for snapshots
        #[arg(long, default_value = ".cache/ctd_cache.redb")]
        cache: PathBuf,

        /// Disable sitemap.xml discovery (use crawling instead)
        #[arg(long = "no-sitemap", action = clap::ArgAction::SetTrue)]
        no_sitemap: bool,

        /// Regex pattern to filter URLs by path
        #[arg(short, long, value_name = "REGEX")]
        filter: Option<String>,

        /// Delay between requests in milliseconds (0-60000)
        #[arg(short, long, default_value = "0", value_parser = validate_delay, allow_hyphen_values = true)]
        delay: u64,

        /// Request timeout in seconds (1-600)
        #[arg(long, default_value = "30", value_parser = validate_timeout_secs, allow_hyphen_values = true)]
        request_timeout_secs: u64,

        /// TCP connect timeout in seconds (1-60, default: 10)
        #[arg(long, default_value = "10", value_parser = validate_connect_timeout_secs, allow_hyphen_values = true)]
        connect_timeout_secs: u64,

        /// Max spider retries (0 disables spider retry)
        #[arg(long, default_value = "3", value_parser = validate_retry_count, allow_hyphen_values = true)]
        max_retries: u32,

        /// Redirect policy: loose (default), strict, none
        #[arg(long, default_value = "loose", value_parser = parse_redirect_policy)]
        redirect_policy: RedirectPolicy,

        /// Concurrency (1-128, default 4) capped for politeness
        #[arg(long, default_value = "4", value_parser = validate_concurrency_limit, allow_hyphen_values = true)]
        concurrency: usize,

        /// Output structured JSON to stdout
        #[arg(long)]
        json: bool,
    },

    /// Commit a change plan snapshot (Terraform-style apply)
    Apply {
        /// URL of the documentation site to apply snapshot for
        #[arg(value_name = "URL")]
        url: String,

        /// Path to the redb cache file for snapshots
        #[arg(long, default_value = ".cache/ctd_cache.redb")]
        cache: PathBuf,

        /// The scraped content directory (with manifest.json)
        #[arg(long, value_name = "DIR")]
        scrape_dir: PathBuf,

        /// Skip confirmation prompt
        #[arg(long)]
        yes: bool,
    },

    /// Compare two .scrape directories and show diff
    Diff {
        /// First .scrape directory
        #[arg(value_name = "DIR_A")]
        dir_a: PathBuf,

        /// Second .scrape directory
        #[arg(value_name = "DIR_B")]
        dir_b: PathBuf,

        /// Output directory for diff reports
        #[arg(short, long, value_name = "DIR")]
        output: Option<PathBuf>,

        /// Output structured JSON to stdout
        #[arg(long)]
        json: bool,
    },
}
