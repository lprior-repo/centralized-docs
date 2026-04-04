//! Watch, Apply, and Diff commands for change tracking.
//!
//! Functional architecture: Data → Calc → Actions.
//! Pure calculations in `watch.rs`, I/O isolated here.

use anyhow::Result;
use std::path::Path;
use std::process;
use tracing::instrument;

use crate::cli::config::{DEFAULT_MAX_PAGE_SIZE_BYTES, DEFAULT_MAX_TOTAL_SIZE_BYTES};
use doc_transformer::cache::{url_hash, CacheConfig, DocCache};
use doc_transformer::scrape::validation::ScrapeResult;
use doc_transformer::watch::{
    compute_plan, diff_directories, format_plan_json, format_plan_markdown, snapshot_from_scrape,
    write_plan_reports, ChangePlan, Snapshot,
};

/// Run the `watch` command: scrape + diff → change plan (terraform plan equivalent).
///
/// # Contract
/// - **Preconditions**: url is valid, `cache_path` writable, output writable
/// - **Postconditions**: change-plan.json + change-plan.md written, snapshot NOT mutated
/// - **Invariant**: calling watch twice with same content produces identical plans
#[instrument(skip_all, fields(url = %url))]
pub async fn run_watch(
    url: &str,
    output: &Path,
    cache_path: &Path,
    filter: Option<&str>,
    delay: u64,
    request_timeout_secs: u64,
    max_retries: u32,
    redirect_policy: spider::configuration::RedirectPolicy,
    concurrency: usize,
    json_output: bool,
) -> Result<()> {
    // ── Actions: I/O boundary ──────────────────────────────────────────
    let cache = open_cache(cache_path)?;
    let previous = load_snapshot(&cache, url)?;
    print_watch_header(url, &previous);

    let scrape_config = build_scrape_config(
        url,
        filter,
        delay,
        request_timeout_secs,
        max_retries,
        redirect_policy,
        concurrency,
    );
    let scrape_result = execute_scrape(&scrape_config).await?;

    // ── Calculations: pure, no I/O ─────────────────────────────────────
    let plan = compute_plan(url, &previous, &scrape_result);

    // ── Actions: write reports + output ─────────────────────────────────
    write_plan_reports(&plan, output)?;
    emit_plan(&plan, json_output);

    print_watch_footer(output, &plan);

    process::exit(i32::from(!plan.summary.is_empty()));
}

/// Run the `apply` command: commit current scrape as new snapshot (terraform apply).
///
/// # Contract
/// - **Preconditions**: `scrape_dir` contains valid manifest.json, cache writable
/// - **Postconditions**: snapshot stored in redb, idempotent on re-run
/// - **Invariant**: apply twice with same content is a no-op
#[instrument(skip_all, fields(url = %url))]
pub async fn run_apply(url: &str, cache_path: &Path, scrape_dir: &Path, yes: bool) -> Result<()> {
    // ── Actions: load previous + read manifest ─────────────────────────
    let cache = open_cache(cache_path)?;
    let previous = load_snapshot(&cache, url)?;
    let scrape_result = read_manifest(scrape_dir)?;

    // ── Calculation: compute plan to show what changes ──────────────────
    let plan = compute_plan(url, &previous, &scrape_result);

    // ── Actions: display, confirm, commit ───────────────────────────────
    print_apply_summary(url, &plan);

    if plan.summary.is_empty() {
        println!("[APPLY] No changes. Snapshot is already up to date.");
        process::exit(0);
    }

    if !yes {
        prompt_confirmation()?;
    }

    let new_snapshot = snapshot_from_scrape(url, &scrape_result);
    store_snapshot(&cache, url, &new_snapshot)?;

    println!(
        "[APPLY] Snapshot committed: {} pages at {}",
        new_snapshot.pages.len(),
        new_snapshot.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
    );

    process::exit(0);
}

/// Run the `diff` command: compare two .scrape directories.
///
/// # Contract
/// - **Preconditions**: both dirs contain valid manifest.json
/// - **Postconditions**: plan emitted to stdout (and optionally to files)
pub fn run_diff(
    dir_a: &Path,
    dir_b: &Path,
    output: Option<&Path>,
    json_output: bool,
) -> Result<()> {
    // ── Calculation: pure diff ──────────────────────────────────────────
    let plan = diff_directories(dir_a, dir_b)?;

    // ── Actions: write + emit ───────────────────────────────────────────
    if let Some(out_dir) = output {
        write_plan_reports(&plan, out_dir)?;
        println!("[DIFF] Reports written to: {}/", out_dir.display());
    }

    emit_plan(&plan, json_output);

    process::exit(i32::from(!plan.summary.is_empty()));
}

// ════════════════════════════════════════════════════════════════════════
// Actions: I/O helpers (thin wrappers, no business logic)
// ════════════════════════════════════════════════════════════════════════

fn open_cache(cache_path: &Path) -> Result<DocCache> {
    let config = CacheConfig::new(cache_path);
    DocCache::open(config)
}

fn load_snapshot(cache: &DocCache, url: &str) -> Result<Snapshot> {
    let url_key = url_hash(url);
    let snapshot: Option<Snapshot> = cache.get_snapshot(url_key.as_bytes())?;
    Ok(snapshot.unwrap_or_else(|| Snapshot {
        target_url: url.to_string(),
        timestamp: chrono::Utc::now(),
        pages: std::collections::BTreeMap::new(),
    }))
}

fn store_snapshot(cache: &DocCache, url: &str, snapshot: &Snapshot) -> Result<()> {
    let url_key = url_hash(url);
    cache.put_snapshot(url_key.as_bytes(), snapshot)
}

fn read_manifest(scrape_dir: &Path) -> Result<ScrapeResult> {
    let manifest_path = scrape_dir.join("manifest.json");
    let file = std::fs::File::open(&manifest_path)
        .map_err(|e| anyhow::anyhow!("Cannot read {}: {e}", manifest_path.display()))?;
    serde_json::from_reader(file)
        .map_err(|e| anyhow::anyhow!("Invalid manifest at {}: {e}", manifest_path.display()))
}

async fn execute_scrape(config: &doc_transformer::scrape::ScrapeConfig) -> Result<ScrapeResult> {
    doc_transformer::scrape::scrape_site(config).await
}

fn emit_plan(plan: &ChangePlan, json_output: bool) {
    if json_output {
        match format_plan_json(plan) {
            Ok(json) => println!("{json}"),
            Err(e) => eprintln!("[ERROR] JSON serialization failed: {e}"),
        }
    } else {
        let md = format_plan_markdown(plan);
        println!("\n{md}");
    }
}

fn prompt_confirmation() -> Result<()> {
    println!("\nApply the above changes? [y/N] ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if !input.trim().eq_ignore_ascii_case("y") {
        println!("[APPLY] Aborted.");
        process::exit(1);
    }
    Ok(())
}

// ── Display helpers (pure formatting, no side effects beyond println) ──

fn print_watch_header(url: &str, previous: &Snapshot) {
    println!("[WATCH] Target: {url}");
    if previous.pages.is_empty() {
        println!("[WATCH] Previous snapshot: none (first scrape)");
    } else {
        println!(
            "[WATCH] Previous snapshot: {} pages at {}",
            previous.pages.len(),
            previous.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        );
    }
}

fn print_watch_footer(output: &Path, plan: &ChangePlan) {
    println!(
        "[WATCH] Reports written to: {}/change-plan.json and change-plan.md",
        output.display()
    );
    if plan.summary.is_empty() {
        println!("[WATCH] No changes detected.");
    } else {
        println!(
            "[WATCH] Changes: {} added, {} removed, {} modified",
            plan.summary.added, plan.summary.removed, plan.summary.modified
        );
    }
}

fn print_apply_summary(url: &str, plan: &ChangePlan) {
    println!("[APPLY] Target: {url}");
    println!(
        "[APPLY] Pages: {} (was {})",
        plan.summary.total_current, plan.summary.total_previous
    );
    if !plan.summary.is_empty() {
        println!(
            "[APPLY] Changes: {} added, {} removed, {} modified",
            plan.summary.added, plan.summary.removed, plan.summary.modified
        );
    }
}

// ── Config builder (pure calculation, returns data) ─────────────────────

fn build_scrape_config(
    url: &str,
    filter: Option<&str>,
    delay: u64,
    request_timeout_secs: u64,
    max_retries: u32,
    redirect_policy: spider::configuration::RedirectPolicy,
    concurrency: usize,
) -> doc_transformer::scrape::ScrapeConfig {
    doc_transformer::scrape::ScrapeConfig {
        base_url: url.to_string(),
        sitemap_strategy: doc_transformer::scrape::SitemapStrategy::UseSitemap,
        path_filter: filter.map(String::from),
        delay_ms: delay,
        max_page_size_bytes: DEFAULT_MAX_PAGE_SIZE_BYTES,
        max_total_size_bytes: DEFAULT_MAX_TOTAL_SIZE_BYTES,
        spider_max_page_bytes: None,
        spider_max_total_bytes: None,
        request_timeout_secs,
        max_retries,
        redirect_policy,
        concurrency_limit: concurrency,
        ..Default::default()
    }
}
