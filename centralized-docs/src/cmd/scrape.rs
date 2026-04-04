use crate::cli::config::{
    ScrapeCommandConfig, DEFAULT_MAX_PAGE_SIZE_BYTES, DEFAULT_MAX_TOTAL_SIZE_BYTES,
};
use crate::cli::validation::validate_filter_regex;
use crate::scrape;
use anyhow::Result;
use std::path::Path;
use tracing::instrument;

/// Validate query length to prevent `DoS` attacks and resource exhaustion
///
/// Constraints:
/// - Maximum 1000 bytes (prevents regex compilation timeouts)
/// - None/empty queries allowed (no filtering)
pub fn validate_query_length(query: &Option<&str>) -> Result<()> {
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
pub fn apply_query_filter(
    pages: Vec<scrape::ScrapedPage>,
    query: Option<&str>,
    threshold: f32,
) -> Result<Vec<scrape::ScrapedPage>> {
    let Some(raw_query) = query else {
        return Ok(pages);
    };

    let query = raw_query.trim();
    if query.is_empty() || threshold <= 0.0 || pages.is_empty() {
        return Ok(pages);
    }

    let original_len = pages.len();

    // Create a temporary RAM index for scoring the scraped pages
    use tantivy::collector::TopDocs;
    use tantivy::query::QueryParser;
    use tantivy::schema::{Schema, Value, STORED, TEXT};
    use tantivy::Index;

    #[allow(unused_mut)]
    let mut schema_builder = Schema::builder();
    let title_field = schema_builder.add_text_field("title", TEXT);
    let content_field = schema_builder.add_text_field("content", TEXT);
    let id_field = schema_builder.add_u64_field("id", STORED);
    let schema = schema_builder.build();

    let index = Index::create_in_ram(schema);
    #[allow(unused_mut)]
    let mut writer = index.writer(15_000_000)?;

    pages
        .iter()
        .enumerate()
        .try_for_each(|(id, page)| -> Result<()> {
            let doc = tantivy::doc!(
                title_field => page.title.as_str(),
                content_field => page.markdown.as_str(),
                id_field => id as u64
            );
            writer.add_document(doc)?;
            Ok(())
        })?;
    writer.commit()?;

    let reader = index.reader()?;
    let searcher = reader.searcher();
    let query_parser = QueryParser::for_index(&index, vec![title_field, content_field]);
    let parsed_query = query_parser.parse_query(query)?;

    let top_docs = searcher.search(&parsed_query, &TopDocs::with_limit(pages.len()))?;

    let valid_ids: std::collections::HashSet<usize> = top_docs
        .iter()
        .filter(|(score, _)| *score >= threshold)
        .filter_map(|(_, doc_address)| {
            let fetched = searcher
                .doc::<tantivy::TantivyDocument>(*doc_address)
                .ok()?;
            let val = fetched.get_first(id_field)?;
            val.as_u64().map(|id_val| id_val as usize)
        })
        .collect();

    let kept_pages: Vec<scrape::ScrapedPage> = pages
        .into_iter()
        .enumerate()
        .filter(|(i, _)| valid_ids.contains(i))
        .map(|(_, page)| page)
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

include!("scrape_tests.rs");

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
        max_page_size_bytes: config
            .max_page_bytes
            .map_or(DEFAULT_MAX_PAGE_SIZE_BYTES, |v| v),
        max_total_size_bytes: config
            .max_total_bytes
            .map_or(DEFAULT_MAX_TOTAL_SIZE_BYTES, |v| v),
        spider_max_page_bytes: config.max_page_bytes,
        spider_max_total_bytes: config.max_total_bytes,
        request_timeout_secs: config.request_timeout_secs,
        max_retries: config.max_retries,
        redirect_policy: config.redirect_policy.clone(),
        concurrency_limit: config.concurrency_limit,
        ..Default::default()
    };

    println!("[SCRAPE] Starting crawl...");
    let initial_result = scrape::scrape_site(&scrape_config).await?;

    // Check if domain was reachable BEFORE checking for partial failures
    // If total_urls == 0, the domain couldn't be reached (DNS failure, connection refused, etc.)
    if initial_result.total_urls == 0 && initial_result.success_count == 0 {
        println!();
        println!("{}", "=".repeat(70));
        println!("SCRAPE FAILED - Domain unreachable");
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
    // Exit with code 2 if NO pages were scraped successfully
    if initial_result.success_count == 0 {
        if initial_result.total_urls == 0 {
            println!();
            println!("{}", "=".repeat(70));
            println!("SCRAPE FAILED");
            println!("{}", "=".repeat(70));
            println!("Failed to reach '{url}'. The domain may not exist or DNS resolution failed.");
            println!("Please verify:");
            println!("  - The URL is correct and accessible in a browser");
            println!("  - The domain exists and is spelled correctly");
            println!("{}\n", "=".repeat(70));
            anyhow::bail!("Failed to reach '{url}': DNS or connection error");
        }

        if initial_result.total_urls == 5 {
            println!();
            println!("{}", "=".repeat(70));
            println!("No pages extracted from '{url}'.");
            println!("The site may be a JavaScript SPA (Single Page Application)");
            println!("Consider using --spa-mode or --browser for dynamic rendering");
            println!("{}\n", "=".repeat(70));
            anyhow::bail!("No pages extracted from '{url}': site may require JavaScript rendering");
        }
        // Partial success: some pages failed, but we got results
        println!();
        println!(
            "  Scraped: {} pages ({} errors)",
            initial_result.success_count, initial_result.error_count
        );
    } else {
        // Total success: continue normally
        println!("  Scraped: {} pages", initial_result.success_count);
    }

    println!("  Scraped: {} pages", initial_result.success_count);

    // Apply BM25 filtering if query is provided (extracted common logic)
    let filtered_pages = apply_query_filter(initial_result.pages, query_ref, config.threshold)?;
    let result = scrape::ScrapeResult {
        success_count: filtered_pages.len(),
        pages: filtered_pages,
        ..initial_result
    };

    // Detect potential SPA (JavaScript-rendered site) BEFORE validation
    // This ensures we show helpful message even when scraping fails
    let spa_detection = scrape::detect_potential_spa(&result);
    if let Some(ref warning) = spa_detection.warning_message {
        println!();
        println!("{}", "=".repeat(70));
        println!("{warning}");
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
