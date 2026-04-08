//! CLI type definitions: top-level parser and command enum.
//!
//! The `Commands` enum uses `#[command(flatten)]` with [`SpiderCoreArgs`] and
//! [`SpiderCrawlArgs`] to deduplicate the 11 shared web-scraping fields across
//! the Scrape, Ingest, and Watch variants.

use super::mcp_cmd::McpCommand;
use super::spider_args::{SpiderCoreArgs, SpiderCrawlArgs};
use super::validation::{
    validate_hnsw_ef_construction, validate_hnsw_m, validate_limit_cli,
    validate_max_chunk_keywords, validate_max_related_chunks, validate_positive_bytes,
};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

const LONG_ABOUT: &str = concat!(
    "\nctd v",
    env!("CARGO_PKG_VERSION"),
    " - The AI-Optimized Documentation Indexer\n\n\
     USAGE:\n\
       ctd scrape <URL> --output <DIR>    # Scrape a documentation site\n\
       ctd index <SOURCE> --output <DIR>  # Index local markdown files\n\
       ctd ingest <URL> --output <DIR>    # Scrape + index in one step\n\n\
     OUTPUT:\n\
       llms.txt      - AI entry point (read this first)\n\
       INDEX.json    - Machine-readable index with chunks and DAG\n\
       NAVIGATION.md - Human-readable navigation\n\
       docs/         - Transformed documents with frontmatter\n\
       chunks/       - Semantic chunks with context prefix\n \
    "
);

#[derive(Parser, Debug)]
#[command(
    name = "ctd",
    version = env!("CARGO_PKG_VERSION"),
    about = "Transform documentation into AI-optimized knowledge structures",
    long_about = LONG_ABOUT,
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
        #[arg(short = 'n', long, default_value = "10", value_parser = validate_limit_cli, allow_hyphen_values = true)]
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

        #[command(flatten)]
        spider: SpiderCoreArgs,

        #[command(flatten)]
        crawl: SpiderCrawlArgs,
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

        #[command(flatten)]
        spider: SpiderCoreArgs,

        #[command(flatten)]
        crawl: SpiderCrawlArgs,

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

        #[command(flatten)]
        spider: SpiderCoreArgs,

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

    /// Compact the state database to reclaim disk space
    Compact {
        /// Path to the state database file (e.g., .cache/ctd_cache.redb)
        #[arg(value_name = "STATE_DB_PATH")]
        path: PathBuf,
    },
}
