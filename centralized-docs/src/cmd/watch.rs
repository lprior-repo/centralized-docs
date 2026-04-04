//! Watch, Apply, and Diff commands for change tracking.
//!
//! Functional architecture: Data → Calc → Actions.
//! Pure calculations in `watch.rs`, I/O isolated here.

use anyhow::Result;
use std::path::Path;
use std::process;
use tracing::instrument;

use crate::cache::url_hash;
use crate::cli::config::{DEFAULT_MAX_PAGE_SIZE_BYTES, DEFAULT_MAX_TOTAL_SIZE_BYTES};
use crate::scrape::validation::ScrapeResult;
use crate::state::commit::{StateChanges, StateDb};
use crate::state::serialize_snapshot;
use crate::watch::{
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
    let state_db = open_state_db(cache_path)?;
    let previous = load_snapshot(&state_db, url)?;
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
    let state_db = open_state_db(cache_path)?;
    let previous = load_snapshot(&state_db, url)?;
    let scrape_result = read_manifest(scrape_dir)?;

    // ── Calculation: compute plan to show what changes ──────────────────
    let plan = compute_plan(url, &previous, &scrape_result);

    // ── Actions: display, confirm, commit ───────────────────────────────
    print_apply_summary(url, &plan);

    if plan.summary.is_empty() {
        tracing::info!("No changes...");
        process::exit(0);
    }

    if !yes {
        prompt_confirmation()?;
    }

    let new_snapshot = snapshot_from_scrape(url, &scrape_result);
    store_snapshot(&state_db, url, &new_snapshot)?;

    tracing::info!(
        pages = new_snapshot.pages.len(),
        timestamp = %new_snapshot.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
        "Snapshot committed"
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
    // session dropped here — borrow released before any potential write

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

fn emit_plan(plan: &ChangePlan, json_output: bool) {
    if json_output {
        match format_plan_json(plan) {
            Ok(json) => println!("{json}"),
            Err(e) => tracing::error!(error = %e, "JSON serialization failed"),
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
        tracing::info!("Apply aborted by user");
        process::exit(1);
    }
    Ok(())
}

// ── Display helpers (pure formatting, no side effects beyond println) ──

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

// ── Config builder (pure calculation, returns data) ─────────────────────

fn build_scrape_config(
    url: &str,
    filter: Option<&str>,
    delay: u64,
    request_timeout_secs: u64,
    max_retries: u32,
    redirect_policy: spider::configuration::RedirectPolicy,
    concurrency: usize,
) -> crate::scrape::ScrapeConfig {
    crate::scrape::ScrapeConfig {
        base_url: url.to_string(),
        sitemap_strategy: crate::scrape::SitemapStrategy::UseSitemap,
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
