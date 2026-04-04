use crate::cli::config::{
    IndexConfig, IngestConfig, DEFAULT_MAX_PAGE_SIZE_BYTES, DEFAULT_MAX_TOTAL_SIZE_BYTES,
};
use crate::cli::validation::validate_filter_regex;
use crate::cmd::index::run_index;
use crate::cmd::scrape::{apply_query_filter, validate_query_length};
use crate::scrape::{self, SitemapStrategy};
use anyhow::Result;
use std::path::Path;
use tracing::instrument;

/// Run the ingest command (scrape + index)
#[instrument(skip_all, fields(url = %url))]
pub async fn run_ingest(url: &str, output: &Path, config: &IngestConfig) -> Result<()> {
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
        validate_filter_regex(f).map_err(|e: String| anyhow::anyhow!(e))?;
    }

    // Phase 1: Scrape
    tracing::info!("Starting scrape phase");

    let scrape_config = scrape::ScrapeConfig {
        base_url: url.to_string(),
        sitemap_strategy: SitemapStrategy::UseSitemap,
        path_filter: filter,
        delay_ms: delay,
        max_page_size_bytes: max_page_bytes.map_or(DEFAULT_MAX_PAGE_SIZE_BYTES, |v| v),
        max_total_size_bytes: max_total_bytes.map_or(DEFAULT_MAX_TOTAL_SIZE_BYTES, |v| v),
        spider_max_page_bytes: max_page_bytes,
        spider_max_total_bytes: max_total_bytes,
        request_timeout_secs: config.request_timeout_secs,
        max_retries: config.max_retries,
        redirect_policy: config.redirect_policy.clone(),
        concurrency_limit: config.concurrency_limit,
        ..Default::default()
    };

    let initial_scrape_result = scrape::scrape_site(&scrape_config).await?;

    // Log partial failures but continue processing with successful pages
    if initial_scrape_result.error_count > 0 {
        println!();
        println!("{}", "=".repeat(70));
        tracing::warn!("Scrape completed with partial failure");
        println!("{}", "=".repeat(70));
        tracing::info!(
            pages = initial_scrape_result.success_count,
            "Scrape success"
        );
        tracing::error!(errors = initial_scrape_result.error_count, "Scrape errors");
        println!("Hint: Check .scrape/manifest.json for error details");
        println!("{}\n", "=".repeat(70));
        // Continue with successful pages instead of exiting
    }

    tracing::info!(
        pages = initial_scrape_result.success_count,
        "Scrape complete"
    );

    // Apply BM25 filtering if query is provided (extracted common logic)
    let filtered_pages = apply_query_filter(initial_scrape_result.pages, query_ref, threshold)?;
    let scrape_result = scrape::ScrapeResult {
        success_count: filtered_pages.len(),
        pages: filtered_pages,
        ..initial_scrape_result
    };

    // Validate that at least one page was scraped (fail fast on invalid URLs)
    scrape::validate_scrape_result(&scrape_result)?;

    // Detect potential SPA (JavaScript-rendered site) and warn user
    let spa_detection = scrape::detect_potential_spa(&scrape_result);
    if let Some(ref warning) = spa_detection.warning_message {
        println!();
        println!("{}", "=".repeat(70));
        println!("{warning}");
        println!("{}", "=".repeat(70));
    }

    println!();

    // Write scraped content to temp location within output
    let scrape_dir = output.join(".scrape");
    std::fs::create_dir_all(&scrape_dir)?;
    scrape::write_scraped_pages(&scrape_result, output)?;

    // Phase 2: Index
    tracing::info!("Starting index phase");

    // Derive project name from URL if not provided
    let name = project_name.map_or_else(
        || {
            url::Url::parse(url)
                .map(|u| {
                    u.host_str().map_or_else(
                        || "Documentation".to_string(),
                        std::string::ToString::to_string,
                    )
                })
                .map_or_else(|_| "Documentation".to_string(), |s| s)
        },
        |n| n,
    );

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
