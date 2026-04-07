//! Watch, Apply, and Diff commands for change tracking.
//!
//! Functional architecture: Data → Calc → Actions.
//! Pure calculations in `watch.rs`, I/O isolated here.

use anyhow::Result;
use std::path::Path;
use tracing::instrument;

use crate::cache::url_hash;
use crate::cli::config::{DEFAULT_MAX_PAGE_SIZE_BYTES, DEFAULT_MAX_TOTAL_SIZE_BYTES};
use crate::scrape::validation::ScrapeResult;
use crate::scrape::SitemapStrategy;
use crate::state::commit::{StateChanges, StateDb};
use crate::state::serialize_snapshot;
use crate::watch::{
    compute_plan, diff_directories, format_plan_json, format_plan_markdown, snapshot_from_scrape,
    write_plan_reports, ChangePlan, Snapshot,
};

/// Output format for plan display.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Machine-readable JSON output.
    Json,
    /// Human-readable Markdown output.
    Markdown,
}

/// Confirmation mode for the apply command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmMode {
    /// Skip interactive confirmation and apply immediately.
    AutoConfirm,
    /// Prompt the user for confirmation before applying.
    Interactive,
}

/// Run the `watch` command: scrape + diff → change plan (terraform plan equivalent).
#[instrument(skip_all, fields(url = %url))]
pub async fn run_watch(
    url: &str,
    output: &Path,
    cache_path: &Path,
    filter: Option<&str>,
    delay: u64,
    request_timeout_secs: u64,
    connect_timeout_secs: u64,
    max_retries: u32,
    redirect_policy: spider::configuration::RedirectPolicy,
    concurrency: usize,
    output_format: OutputFormat,
    sitemap_strategy: SitemapStrategy,
) -> Result<()> {
    let state_db = open_state_db(cache_path)?;
    let previous = load_snapshot(&state_db, url)?;
    print_watch_header(url, &previous);

    let scrape_config = build_scrape_config(
        url,
        filter,
        delay,
        request_timeout_secs,
        connect_timeout_secs,
        max_retries,
        redirect_policy,
        concurrency,
        sitemap_strategy,
    );
    let scrape_result = execute_scrape(&scrape_config).await?;

    // Check if domain was reachable BEFORE computing plan
    // If total_urls == 0, the domain couldn't be reached (DNS failure, connection refused, etc.)
    if scrape_result.total_urls == 0 && scrape_result.success_count == 0 {
        tracing::error!("Domain unreachable: {}", scrape_result.base_url);
        anyhow::bail!("Domain unreachable: {}", scrape_result.base_url);
    }

    let plan = compute_plan(url, &previous, &scrape_result);

    write_plan_reports(&plan, output)?;
    emit_plan(&plan, output_format);
    print_watch_footer(output, &plan);

    Ok(())
}

/// Run the `apply` command: commit current scrape as new snapshot.
#[instrument(skip_all, fields(url = %url))]
pub async fn run_apply(
    url: &str,
    cache_path: &Path,
    scrape_dir: &Path,
    confirm_mode: ConfirmMode,
) -> Result<()> {
    let state_db = open_state_db(cache_path)?;
    let previous = load_snapshot(&state_db, url)?;
    let scrape_result = read_manifest(scrape_dir)?;

    let plan = compute_plan(url, &previous, &scrape_result);
    print_apply_summary(url, &plan);

    if plan.summary.is_empty() {
        tracing::info!("No changes...");
        return Ok(());
    }

    if confirm_mode == ConfirmMode::Interactive {
        prompt_confirmation()?;
    }

    let new_snapshot = snapshot_from_scrape(url, &scrape_result);
    store_snapshot(&state_db, url, &new_snapshot)?;

    tracing::info!(
        pages = new_snapshot.pages.len(),
        timestamp = %new_snapshot.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
        "Snapshot committed"
    );

    Ok(())
}

/// Run the `diff` command: compare two .scrape directories.
pub fn run_diff(
    dir_a: &Path,
    dir_b: &Path,
    output: Option<&Path>,
    output_format: OutputFormat,
) -> Result<()> {
    let plan = diff_directories(dir_a, dir_b)?;

    if let Some(out_dir) = output {
        write_plan_reports(&plan, out_dir)?;
        println!("[DIFF] Reports written to: {}/", out_dir.display());
    }

    emit_plan(&plan, output_format);
    Ok(())
}

fn open_state_db(state_db_path: &Path) -> Result<StateDb> {
    StateDb::open(state_db_path).map_err(|e| anyhow::anyhow!("{e}"))
}

fn load_snapshot(state_db: &StateDb, url: &str) -> Result<Snapshot> {
    let url_key = url_hash(url);
    let key_bytes: [u8; 32] = url_key.as_bytes().try_into().map_err(|_| {
        anyhow::anyhow!(
            "url_hash produced {} bytes, expected 32",
            url_key.as_bytes().len()
        )
    })?;

    let session = state_db.begin_read().map_err(|e| anyhow::anyhow!("{e}"))?;
    let results = session
        .load_snapshots(&[key_bytes])
        .map_err(|e| anyhow::anyhow!("{e}"))?;

    Ok(results
        .get(&key_bytes)
        .map(|archived| {
            archived
                .deserialize::<crate::watch::Snapshot>()
                .map_err(|e| anyhow::anyhow!("{e}"))
        })
        .transpose()?
        .unwrap_or_else(|| Snapshot {
            target_url: url.to_string(),
            timestamp: chrono::Utc::now(),
            pages: std::collections::BTreeMap::new(),
        }))
}

fn store_snapshot(state_db: &StateDb, url: &str, snapshot: &Snapshot) -> Result<()> {
    let url_key = url_hash(url);
    let key_bytes: [u8; 32] = url_key.as_bytes().try_into().map_err(|_| {
        anyhow::anyhow!(
            "url_hash produced {} bytes, expected 32",
            url_key.as_bytes().len()
        )
    })?;

    let rkyv_bytes = serialize_snapshot(snapshot).map_err(|e| anyhow::anyhow!("{e}"))?;

    let changes = StateChanges {
        new_snapshots: vec![(key_bytes, rkyv_bytes)],
        ..StateChanges::empty()
    };

    state_db
        .commit_changes(changes)
        .map_err(|e| anyhow::anyhow!("{e}"))
}

fn read_manifest(scrape_dir: &Path) -> Result<ScrapeResult> {
    let manifest_path = scrape_dir.join("manifest.json");
    let file = std::fs::File::open(&manifest_path)
        .map_err(|e| anyhow::anyhow!("Cannot read {}: {e}", manifest_path.display()))?;
    serde_json::from_reader(file)
        .map_err(|e| anyhow::anyhow!("Invalid manifest at {}: {e}", manifest_path.display()))
}

#[instrument(skip_all)]
async fn execute_scrape(config: &crate::scrape::ScrapeConfig) -> Result<ScrapeResult> {
    crate::scrape::scrape_site(config).await
}

fn emit_plan(plan: &ChangePlan, output_format: OutputFormat) {
    match output_format {
        OutputFormat::Json => match format_plan_json(plan) {
            Ok(json) => println!("{json}"),
            Err(e) => tracing::error!(error = %e, "JSON serialization failed"),
        },
        OutputFormat::Markdown => {
            let md = format_plan_markdown(plan);
            println!("\n{md}");
        }
    }
}

fn prompt_confirmation() -> Result<()> {
    println!("\nApply the above changes? [y/N] ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if !input.trim().eq_ignore_ascii_case("y") {
        tracing::info!("Apply aborted by user");
        anyhow::bail!("Apply aborted by user");
    }
    Ok(())
}

fn print_watch_header(url: &str, previous: &Snapshot) {
    tracing::info!(url = %url, "Watch target");
    tracing::info!(pages = previous.pages.len(), "Previous snapshot");
}

fn print_watch_footer(output: &Path, plan: &ChangePlan) {
    tracing::info!(dir = %output.display(), "Reports written");
    if plan.summary.is_empty() {
        tracing::info!("No changes detected");
    } else {
        tracing::info!(
            added = plan.summary.added,
            removed = plan.summary.removed,
            modified = plan.summary.modified,
            "Changes detected"
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

fn build_scrape_config(
    url: &str,
    filter: Option<&str>,
    delay: u64,
    request_timeout_secs: u64,
    connect_timeout_secs: u64,
    max_retries: u32,
    redirect_policy: spider::configuration::RedirectPolicy,
    concurrency: usize,
    sitemap_strategy: SitemapStrategy,
) -> crate::scrape::ScrapeConfig {
    crate::scrape::ScrapeConfig {
        base_url: url.to_string(),
        sitemap_strategy,
        path_filter: filter.map(String::from),
        delay_ms: delay,
        max_page_size_bytes: DEFAULT_MAX_PAGE_SIZE_BYTES,
        max_total_size_bytes: DEFAULT_MAX_TOTAL_SIZE_BYTES,
        spider_max_page_bytes: None,
        spider_max_total_bytes: None,
        request_timeout_secs,
        connect_timeout_secs,
        max_retries,
        redirect_policy,
        concurrency_limit: concurrency,
        ..Default::default()
    }
}
