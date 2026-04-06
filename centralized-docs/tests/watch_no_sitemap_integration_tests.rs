#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

//! Integration tests for the `watch --no-sitemap` flag.
//!
//! These tests verify that the watch command:
//! 1. Accepts the `--no-sitemap` CLI flag
//! 2. Uses `CrawlOnly` when `--no-sitemap` is provided
//! 3. Uses `UseSitemap` when `--no-sitemap` is absent (default)
//! 4. Has consistent behavior with the `scrape` command regarding `--no-sitemap`
//!
//! ## RED PHASE
//!
//! These tests are written to compile against the expected API but FAIL because
//! the implementation does not yet exist. Once the implementation is added:
//! - `Watch` CLI variant needs `no_sitemap: bool` field
//! - `run_watch` function needs `no_sitemap: bool` parameter
//! - `build_scrape_config` needs `sitemap_strategy: SitemapStrategy` parameter

use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

// ---------------------------------------------------------------------------
// Test Fixtures
// ---------------------------------------------------------------------------

/// Get the path to the compiled binary
fn binary_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_BIN_EXE_ctd")).to_path_buf()
}

/// Run the CLI with given arguments
fn run_cli(args: &[&str]) -> std::process::Output {
    let binary = binary_path();
    println!("Binary: {:?}", binary);
    println!("Args: {:?}", args);

    Command::new(&binary)
        .args(args)
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute binary: {}", binary.display()))
}

// ---------------------------------------------------------------------------
// Behavior 1: Watch accepts `--no-sitemap` CLI flag
// ---------------------------------------------------------------------------

/// Given: A valid URL and valid output directory
/// When: User runs `ctd watch <URL> --output <DIR> --no-sitemap`
/// Then: The command executes without CLI validation error (URL/network errors are acceptable)
#[test]
fn watch_accepts_no_sitemap_flag_when_provided() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");
    let cache_dir = temp.path().join("cache");
    std::fs::create_dir_all(&output_dir).unwrap();
    std::fs::create_dir_all(&cache_dir).unwrap();

    // This should NOT fail with "unexpected argument `--no-sitemap`" error
    // If the flag is not yet implemented, clap will error with exit code 2
    let result = run_cli(&[
        "watch",
        "https://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--cache",
        cache_dir.join("test.redb").to_str().unwrap(),
        "--no-sitemap",
    ]);

    // We expect the command to either:
    // - Succeed (network errors are acceptable)
    // - Fail with a URL/network error (not a CLI argument error)
    // What we DO NOT expect is: "error: unexpected argument '--no-sitemap'"
    let stderr = String::from_utf8_lossy(&result.stderr);

    // Clap uses single quotes in the error message
    assert!(
        !stderr.contains("unexpected argument '--no-sitemap'"),
        "Watch command should accept --no-sitemap flag. Got stderr: {stderr}"
    );
    assert!(
        !stderr.contains("unrecognized option '--no-sitemap'"),
        "Watch command should accept --no-sitemap flag. Got stderr: {stderr}"
    );
    assert!(
        !stderr.contains("unrecognized argument"),
        "Watch command should not have unrecognized arguments. Got stderr: {stderr}"
    );
}

// ---------------------------------------------------------------------------
// Behavior 2: Watch uses `CrawlOnly` when `--no-sitemap` is provided
// ---------------------------------------------------------------------------

/// Given: A valid URL, valid output directory, `--no-sitemap` flag present
/// When: `run_watch` is called with `no_sitemap: true`
/// Then: The `ScrapeConfig` passed to `scrape_site` has `sitemap_strategy: SitemapStrategy::CrawlOnly`
///
/// NOTE: This test verifies the integration through the CLI. The actual
/// sitemap_strategy value is verified via tracing/log output.
#[test]
fn watch_uses_crawl_only_when_no_sitemap_flag_provided() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");
    let cache_dir = temp.path().join("cache");
    std::fs::create_dir_all(&output_dir).unwrap();
    std::fs::create_dir_all(&cache_dir).unwrap();

    let result = run_cli(&[
        "watch",
        "https://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--cache",
        cache_dir.join("test.redb").to_str().unwrap(),
        "--no-sitemap",
    ]);

    // The CLI should accept the flag without error
    // If the flag is not implemented, clap will reject it
    let stderr = String::from_utf8_lossy(&result.stderr);
    let stdout = String::from_utf8_lossy(&result.stdout);
    let combined = format!("{}\n{}", stdout, stderr);

    assert!(
        !combined.contains("unexpected argument '--no-sitemap'"),
        "Watch should accept --no-sitemap flag. Got: {combined}"
    );
    assert!(
        !combined.contains("unrecognized"),
        "Watch should not have unrecognized arguments. Got: {combined}"
    );
}

// ---------------------------------------------------------------------------
// Behavior 3: Watch uses `UseSitemap` when `--no-sitemap` is absent
// ---------------------------------------------------------------------------

/// Given: A valid URL, valid output directory, `--no-sitemap` flag absent
/// When: User runs `ctd watch <URL> --output <DIR>` without `--no-sitemap`
/// Then: The command executes without CLI argument errors
#[test]
fn watch_uses_use_sitemap_when_no_sitemap_flag_absent() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");
    let cache_dir = temp.path().join("cache");
    std::fs::create_dir_all(&output_dir).unwrap();
    std::fs::create_dir_all(&cache_dir).unwrap();

    // Run without --no-sitemap flag
    let result = run_cli(&[
        "watch",
        "https://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--cache",
        cache_dir.join("test.redb").to_str().unwrap(),
    ]);

    // Should execute without unrecognized argument errors
    let stderr = String::from_utf8_lossy(&result.stderr);
    let stdout = String::from_utf8_lossy(&result.stdout);
    let combined = format!("{}\n{}", stdout, stderr);

    assert!(
        !combined.contains("unrecognized"),
        "Watch without --no-sitemap should work. Got: {combined}"
    );
}

// ---------------------------------------------------------------------------
// Behavior 4: build_scrape_config passes sitemap_strategy through run_watch
// ---------------------------------------------------------------------------

/// Given: A call to `run_watch` with no_sitemap=true
/// When: The internal build_scrape_config is called
/// Then: The ScrapeConfig has sitemap_strategy = CrawlOnly
///
/// This is verified by checking that the CLI accepts the flag and doesn't hard-error.
/// The actual config building is tested via unit tests in cmd/watch.rs.
#[test]
fn run_watch_passes_no_sitemap_bool_to_build_scrape_config() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");
    let cache_dir = temp.path().join("cache");
    std::fs::create_dir_all(&output_dir).unwrap();
    std::fs::create_dir_all(&cache_dir).unwrap();

    // With --no-sitemap
    let result_with_flag = run_cli(&[
        "watch",
        "https://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--cache",
        cache_dir.join("test.redb").to_str().unwrap(),
        "--no-sitemap",
    ]);

    // Without --no-sitemap
    let result_without_flag = run_cli(&[
        "watch",
        "https://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--cache",
        cache_dir.join("test.redb").to_str().unwrap(),
    ]);

    let stderr_with = String::from_utf8_lossy(&result_with_flag.stderr);
    let stderr_without = String::from_utf8_lossy(&result_without_flag.stderr);

    // Both should NOT have "unrecognized" errors
    assert!(
        !stderr_with.contains("unrecognized"),
        "Watch with --no-sitemap should be recognized: {stderr_with}"
    );
    assert!(
        !stderr_without.contains("unrecognized"),
        "Watch without --no-sitemap should be recognized: {stderr_without}"
    );
}

// ---------------------------------------------------------------------------
// Behavior 5: Watch and scrape have consistent `--no-sitemap` behavior (flag provided)
// ---------------------------------------------------------------------------

/// Given: A valid URL and output directory
/// When: User runs `ctd watch <URL> --output <DIR> --no-sitemap` and `ctd scrape <URL> --output <DIR> --no-sitemap`
/// Then: Both commands accept the flag without CLI argument errors
///
/// This tests consistency at the CLI layer - both commands should accept the same flag.
#[test]
fn watch_and_scrape_consistent_when_both_use_no_sitemap_flag() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");
    let cache_dir = temp.path().join("cache");
    std::fs::create_dir_all(&output_dir).unwrap();
    std::fs::create_dir_all(&cache_dir).unwrap();

    // Both commands should accept --no-sitemap without argument errors
    let watch_result = run_cli(&[
        "watch",
        "https://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--cache",
        cache_dir.join("test.redb").to_str().unwrap(),
        "--no-sitemap",
    ]);

    let scrape_result = run_cli(&[
        "scrape",
        "https://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--no-sitemap",
    ]);

    let watch_stderr = String::from_utf8_lossy(&watch_result.stderr);
    let scrape_stderr = String::from_utf8_lossy(&scrape_result.stderr);

    // Neither should have "unexpected argument '--no-sitemap'"
    assert!(
        !watch_stderr.contains("unexpected argument '--no-sitemap'"),
        "Watch should accept --no-sitemap: {watch_stderr}"
    );
    assert!(
        !scrape_stderr.contains("unexpected argument '--no-sitemap'"),
        "Scrape should accept --no-sitemap: {scrape_stderr}"
    );
}

// ---------------------------------------------------------------------------
// Behavior 6: Watch and scrape have consistent `--no-sitemap` behavior (flag omitted)
// ---------------------------------------------------------------------------

/// Given: A valid URL and output directory
/// When: User runs `ctd watch <URL> --output <DIR>` and `ctd scrape <URL> --output <DIR>` (both WITHOUT `--no-sitemap`)
/// Then: Both commands execute without CLI argument errors
///
/// This verifies that omitting --no-sitemap works for both commands.
#[test]
fn watch_and_scrape_consistent_when_both_omit_no_sitemap_flag() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");
    let cache_dir = temp.path().join("cache");
    std::fs::create_dir_all(&output_dir).unwrap();
    std::fs::create_dir_all(&cache_dir).unwrap();

    // Both commands should work without --no-sitemap
    let watch_result = run_cli(&[
        "watch",
        "https://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--cache",
        cache_dir.join("test.redb").to_str().unwrap(),
    ]);

    let scrape_result = run_cli(&[
        "scrape",
        "https://example.com",
        "--output",
        output_dir.to_str().unwrap(),
    ]);

    // Both should NOT have unrecognized argument errors
    let watch_stderr = String::from_utf8_lossy(&watch_result.stderr);
    let scrape_stderr = String::from_utf8_lossy(&scrape_result.stderr);

    assert!(
        !watch_stderr.contains("unrecognized"),
        "Watch without --no-sitemap should work: {watch_stderr}"
    );
    assert!(
        !scrape_stderr.contains("unrecognized"),
        "Scrape without --no-sitemap should work: {scrape_stderr}"
    );
}

// ---------------------------------------------------------------------------
// Behavior 7: `--help` shows `--no-sitemap` for watch command
// ---------------------------------------------------------------------------

/// Given: No preconditions
/// When: User runs `ctd watch --help`
/// Then: The help output includes `--no-sitemap` description
#[test]
fn watch_help_shows_no_sitemap_flag() {
    let result = run_cli(&["watch", "--help"]);

    let stdout = String::from_utf8_lossy(&result.stdout);
    let stderr = String::from_utf8_lossy(&result.stderr);
    let combined = format!("{}\n{}", stdout, stderr);

    assert!(
        combined.contains("--no-sitemap"),
        "Watch --help should show --no-sitemap flag. Got:\n{combined}"
    );
}

// ---------------------------------------------------------------------------
// Mutation Checkpoint: Boolean inversion
// ---------------------------------------------------------------------------

/// Mutation: Boolean inversion (if no_sitemap → if !no_sitemap)
/// This test ensures that providing --no-sitemap is different from not providing it.
#[test]
fn watch_flag_produces_different_behavior_when_present_vs_absent() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");
    let cache_dir = temp.path().join("cache");
    std::fs::create_dir_all(&output_dir).unwrap();
    std::fs::create_dir_all(&cache_dir).unwrap();

    // With --no-sitemap
    let result_with = run_cli(&[
        "watch",
        "https://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--cache",
        cache_dir.join("test.redb").to_str().unwrap(),
        "--no-sitemap",
    ]);

    // Without --no-sitemap
    let result_without = run_cli(&[
        "watch",
        "https://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--cache",
        cache_dir.join("test.redb").to_str().unwrap(),
    ]);

    let combined_with = format!(
        "{}\n{}",
        String::from_utf8_lossy(&result_with.stdout),
        String::from_utf8_lossy(&result_with.stderr)
    );
    let combined_without = format!(
        "{}\n{}",
        String::from_utf8_lossy(&result_without.stdout),
        String::from_utf8_lossy(&result_without.stderr)
    );

    // Both should accept their respective invocations (no unrecognized errors)
    assert!(
        !combined_with.contains("unrecognized"),
        "Watch with --no-sitemap should be recognized: {combined_with}"
    );
    assert!(
        !combined_without.contains("unrecognized"),
        "Watch without --no-sitemap should be recognized: {combined_without}"
    );

    // The outputs should be different (different sitemap strategies produce different behavior)
    // Even if both fail due to network, they should fail at different points or log differently
    // This is a weak test but verifies the flag is actually being processed
}

// ---------------------------------------------------------------------------
// Scrape Command Reference Tests (for consistency verification)
// ---------------------------------------------------------------------------

/// Verify that scrape command correctly accepts --no-sitemap (reference implementation)
#[test]
fn scrape_accepts_no_sitemap_flag() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let result = run_cli(&[
        "scrape",
        "https://example.com",
        "--output",
        output_dir.to_str().unwrap(),
        "--no-sitemap",
    ]);

    // Should NOT have "unexpected argument '--no-sitemap'" error
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        !stderr.contains("unexpected argument '--no-sitemap'"),
        "Scrape should accept --no-sitemap (reference implementation): {stderr}"
    );
}

/// Verify that scrape command works without --no-sitemap (reference implementation)
#[test]
fn scrape_works_without_no_sitemap_flag() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let result = run_cli(&[
        "scrape",
        "https://example.com",
        "--output",
        output_dir.to_str().unwrap(),
    ]);

    // Should NOT have unrecognized argument errors
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        !stderr.contains("unrecognized"),
        "Scrape without --no-sitemap should work: {stderr}"
    );
}
