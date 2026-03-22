use crate::scrape::SitemapStrategy;
use spider::configuration::RedirectPolicy;
use std::path::PathBuf;

pub const DEFAULT_MAX_PAGE_SIZE_BYTES: u64 = 10 * 1024 * 1024;
pub const DEFAULT_MAX_TOTAL_SIZE_BYTES: u64 = 500 * 1024 * 1024;

/// Configuration for the index command
#[derive(Debug, Clone)]
pub struct IndexConfig {
    pub generate_llms: bool,
    pub project_name: String,
    pub project_desc: String,
    pub category_config: Option<PathBuf>,
    pub max_related_chunks: usize,
    pub max_chunk_keywords: usize,
    pub hnsw_m: usize,
    pub hnsw_ef_construction: usize,
    pub max_document_bytes: u64,
    pub path_filter: Option<String>,
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
            path_filter: None,
        }
    }
}

/// Configuration for the scrape command
#[derive(Debug, Clone)]
pub struct ScrapeCommandConfig {
    pub sitemap_strategy: SitemapStrategy,
    pub filter: Option<String>,
    pub delay: u64,
    pub query: Option<String>,
    pub threshold: f32,
    pub request_timeout_secs: u64,
    pub max_retries: u32,
    pub redirect_policy: RedirectPolicy,
    pub max_page_bytes: Option<u64>,
    pub max_total_bytes: Option<u64>,
    pub concurrency_limit: usize,
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
pub struct IngestConfig {
    pub filter: Option<String>,
    pub delay: u64,
    pub query: Option<String>,
    pub threshold: f32,
    pub project_name: Option<String>,
    pub request_timeout_secs: u64,
    pub max_retries: u32,
    pub redirect_policy: RedirectPolicy,
    pub max_page_bytes: Option<u64>,
    pub max_total_bytes: Option<u64>,
    pub concurrency_limit: usize,
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
