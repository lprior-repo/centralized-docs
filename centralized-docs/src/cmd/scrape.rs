//! Scrape command - orchestrates site scraping with state management.
//!
//! State machine:
//! 1. VALIDATE: URL, query length, filter regex
//! 2. LOAD_STATE: Open StateDb, load stored URL states
//! 3. SCRAPE: Crawl site
//! 4. CLASSIFY: Compare against stored state (new/changed/unchanged)
//! 5. REUSE: Load persisted scrape outputs for unchanged pages
//! 6. FILTER: Apply BM25 query filtering if query provided
//! 7. DETECT_SPA: Check for JavaScript SPA indicators
//! 8. WRITE: Save scrape results to disk
//! 9. COMMIT: Atomically commit state changes

use crate::calc::scrape_diff::{
    build_combined_scrape_result, build_scrape_state_changes, classify_scrape_diff,
};
use crate::cli::config::{
    ScrapeCommandConfig, DEFAULT_MAX_PAGE_SIZE_BYTES, DEFAULT_MAX_TOTAL_SIZE_BYTES,
};
use crate::cli::validation::validate_filter_regex;
use crate::scrape;
use crate::state::bulk_load::StateReadSession;
use crate::state::commit::StateDb;
use anyhow::Result;
use std::path::Path;
use tracing::instrument;

// Re-export for backwards compatibility during migration
pub use crate::cmd::scrape_query::{apply_query_filter, validate_query_length};

/// Number of URLs spider discovers before suspecting a JavaScript SPA.
const SPA_DETECTION_THRESHOLD: usize = 5;

/// Run the scrape command
#[instrument(skip_all, fields(url = %url))]
pub async fn run_scrape(url: &str, output: &Path, config: &ScrapeCommandConfig) -> Result<()> {
    let _validated_url = scrape::validate_url(url)?;

    // Validate query length before processing (prevents DoS)
    let query_ref = config.query.as_deref();
    validate_query_length(&query_ref)?;

    // Validate filter regex pattern if provided
    if let Some(ref filter) = config.filter {
        validate_filter_regex(filter).map_err(|e: String| anyhow::anyhow!(e))?;
    }

    tracing::info!(
        url = %url,
        sitemap = ?config.sitemap_strategy,
        delay_ms = config.delay,
        timeout_s = config.request_timeout_secs,
        retries = config.max_retries,
        concurrency = config.concurrency_limit,
        "Starting scrape"
    );
    tracing::info!(
        redirect = ?config.redirect_policy,
        page_bytes = ?config.max_page_bytes,
        total_bytes = ?config.max_total_bytes,
        "Scrape config"
    );
    if let Some(ref f) = config.filter {
        tracing::info!(filter = %f, "Filter configured");
    }

    let scrape_config = scrape::ScrapeConfig {
        base_url: url.to_string(),
        sitemap_strategy: config.sitemap_strategy,
        path_filter: config.filter.clone(),
        delay_ms: config.delay,
        max_page_size_bytes: config
            .max_page_bytes
            .map_or(DEFAULT_MAX_PAGE_SIZE_BYTES, |v| v),
        max_total_size_bytes: config
            .max_total_bytes
            .map_or(DEFAULT_MAX_TOTAL_SIZE_BYTES, |v| v),
        spider_max_page_bytes: config.max_page_bytes,
        spider_max_total_bytes: config.max_total_bytes,
        request_timeout_secs: config.request_timeout_secs,
        connect_timeout_secs: config.connect_timeout_secs,
        max_retries: config.max_retries,
        redirect_policy: config.redirect_policy.clone(),
        concurrency_limit: config.concurrency_limit,
        ..Default::default()
    };

    // --- STATE: Open StateDb, create read session, load stored URL states ---
    let state_db = StateDb::open(&output.join("state.redb"))
        .map_err(|e| anyhow::anyhow!("failed to open state database: {e}"))?;
    let session = StateReadSession::new(state_db.database())
        .map_err(|e| anyhow::anyhow!("failed to create read session: {e}"))?;
    let stored_url_states = session
        .load_url_states()
        .map_err(|e| anyhow::anyhow!("failed to load URL states: {e}"))?;

    tracing::info!("Starting crawl");
    let initial_result = scrape::scrape_site(&scrape_config).await?;

    // Check if domain was reachable BEFORE checking for partial failures
    // If total_urls == 0, the domain couldn't be reached (DNS failure, connection refused, etc.)
    if initial_result.total_urls == 0 && initial_result.success_count == 0 {
        println!();
        println!("{}", "=".repeat(70));
        tracing::error!("Domain unreachable: {}", initial_result.base_url);
        println!("{}", "=".repeat(70));
        println!(
            "Could not reach '{}'. The domain may not exist or DNS resolution failed.",
            initial_result.base_url
        );
        println!();
        println!("Please verify:");
        println!("  - The URL is correct and accessible in a browser");
        println!("  - The domain exists and is spelled correctly");
        println!("{}\n", "=".repeat(70));
        anyhow::bail!("Domain unreachable: {}", initial_result.base_url);
    }

    // Check for total failure BEFORE further processing
    // Note: total_urls == 0 && success_count == 0 already handled above,
    // so only total_urls > 0 && success_count == 0 reaches here.
    if initial_result.success_count == 0 {
        if initial_result.total_urls == SPA_DETECTION_THRESHOLD {
            println!();
            println!("{}", "=".repeat(70));
            tracing::warn!("No pages extracted from '{url}' - site may be a JavaScript SPA");
            println!("{}", "=".repeat(70));
            println!("No pages extracted from '{url}'.");
            println!("The site may be a JavaScript SPA (Single Page Application)");
            println!("Consider using --spa-mode or --browser for dynamic rendering");
            println!("{}\n", "=".repeat(70));
            anyhow::bail!("No pages extracted from '{url}': site may require JavaScript rendering");
        }
        // Partial success: some pages failed, but we got results
        tracing::info!(
            pages = initial_result.success_count,
            errors = initial_result.error_count,
            "Scraped pages (with errors)"
        );
    } else {
        // Total success: continue normally
        tracing::info!(pages = initial_result.success_count, "Pages scraped");
    }

    // --- STATE: Classify scraped pages against stored state ---
    let scrape_diff = classify_scrape_diff(&stored_url_states, &initial_result.pages);

    // Load persisted scrape outputs for unchanged pages
    let unchanged_hashes: Vec<[u8; 32]> = scrape_diff
        .unchanged
        .iter()
        .filter_map(|u| stored_url_states.get(u).map(|s| s.url_hash))
        .filter(|h| *h != [0u8; 32])
        .collect();

    let persisted_scrapes = if unchanged_hashes.is_empty() {
        std::collections::HashMap::new()
    } else {
        session
            .load_scrapes(&unchanged_hashes)
            .map_err(|e| anyhow::anyhow!("failed to load scrape outputs: {e}"))?
    };

    // Convert persisted scrape outputs back to runtime ScrapedPages for reuse
    let reused_pages: Vec<scrape::ScrapedPage> = scrape_diff
        .unchanged
        .iter()
        .filter_map(|page_url| {
            let stored = stored_url_states.get(page_url)?;
            if stored.url_hash == [0u8; 32] {
                return None;
            }
            let archive = match persisted_scrapes.get(&stored.url_hash) {
                Some(a) => a,
                None => {
                    tracing::warn!(
                        url = %page_url,
                        "Persisted scrape not found in state DB — re-scraping"
                    );
                    return None;
                }
            };
            let persisted = match rkyv::from_bytes::<
                crate::persisted::PersistedScrapeResult,
                rkyv::rancor::Error,
            >(archive.as_bytes())
            {
                Ok(p) => p,
                Err(e) => {
                    tracing::warn!(
                        url = %page_url,
                        error = %e,
                        "Failed to deserialize persisted scrape data — re-scraping"
                    );
                    return None;
                }
            };
            // Find the matching page in the persisted result
            persisted
                .pages
                .iter()
                .find(|p| p.url == *page_url)
                .and_then(|p| {
                    crate::persisted::persisted_scraped_page_to_runtime(p)
                        .map_err(|e| {
                            tracing::warn!(
                                url = %page_url,
                                error = %e,
                                "Failed to convert persisted page to runtime — re-scraping"
                            );
                            e
                        })
                        .ok()
                })
        })
        .collect();

    // Freshly scraped pages (new + changed) from the current crawl
    let active_urls: std::collections::HashSet<&str> = scrape_diff
        .new
        .iter()
        .chain(scrape_diff.changed.iter())
        .map(String::as_str)
        .collect();
    let fresh_pages: Vec<scrape::ScrapedPage> = initial_result
        .pages
        .into_iter()
        .filter(|p| active_urls.contains(p.url.as_str()))
        .collect();

    // Build combined ScrapeResult from reused + fresh pages
    let result = build_combined_scrape_result(reused_pages, fresh_pages, &initial_result.base_url);

    // Drop the read session BEFORE commit (INV-3: redb constraint)
    drop(session);
    drop(stored_url_states);

    // Clone pages for state tracking before query filter consumes them
    let all_pages = result.pages.clone();

    // Apply BM25 filtering if query is provided (extracted common logic)
    let filtered_pages = apply_query_filter(result.pages, query_ref, config.threshold)?;
    let filtered_result = scrape::ScrapeResult {
        success_count: filtered_pages.len(),
        pages: filtered_pages,
        ..result
    };

    // Detect potential SPA (JavaScript-rendered site) BEFORE validation
    // This ensures we show helpful message even when scraping fails
    let spa_detection = scrape::detect_potential_spa(&filtered_result);
    if let Some(ref warning) = spa_detection.warning_message {
        println!();
        println!("{}", "=".repeat(70));
        println!("{warning}");
        println!("{}\n", "=".repeat(70));
    }

    // Validate that at least one page was scraped (fail fast on invalid URLs)
    scrape::validate_scrape_result(&filtered_result)?;

    tracing::info!(path = %output.display(), "Saving scrape results");
    std::fs::create_dir_all(output)?;
    scrape::write_scraped_pages(&filtered_result, output)?;

    // --- STATE: Build commit batch and commit atomically ---
    let now_secs = chrono::Utc::now().timestamp().saturating_abs() as u64;

    // Use the original scrape_diff for state changes (computed against stored state)
    let state_changes = build_scrape_state_changes(&scrape_diff, &all_pages, now_secs);

    state_db
        .commit_changes(state_changes)
        .map_err(|e| anyhow::anyhow!("failed to commit scrape state: {e}"))?;

    println!("\n{}", "=".repeat(70));
    tracing::info!("Scrape complete");
    println!("{}", "=".repeat(70));
    println!("Output:  {}", output.display());
    tracing::info!(pages = filtered_result.success_count, "Pages scraped");
    if filtered_result.error_count > 0 {
        tracing::warn!(errors = filtered_result.error_count, "Pages failed");
    }
    println!("Files:   .scrape/*.md + manifest.json");
    println!("{}\n", "=".repeat(70));

    Ok(())
}
