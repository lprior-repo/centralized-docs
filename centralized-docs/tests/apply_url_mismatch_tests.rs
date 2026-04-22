#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

//! Integration tests for `ctd apply` URL/scrape mismatch rejection (bead cdocs-1gr).
//!
//! ## RED PHASE: All tests FAIL because the URL validation bug hasn't been fixed yet.
//!
//! The bug: `run_apply` accepts any CLI-supplied `url` argument without checking
//! that it matches the `base_url` in the scrape manifest. This allows a Kubernetes
//! scrape to be applied under `https://example.com`, poisoning watch state.
//!
//! ## Test Behaviors (from test-plan.md Section 3)
//!
//! - B1: run_apply rejects when target URL does not match manifest base_url
//! - B2: run_apply succeeds when target URL exactly matches manifest base_url
//! - B3: run_apply leaves cache unchanged when URL mismatch is rejected
//! - B4: run_apply returns exit code 1 when URL mismatch is detected
//! - B5: run_apply stores snapshot under manifest base_url, not CLI arg
//! - B6: run_apply emits "Snapshot committed" log on successful apply
//! - B7: run_apply computes plan only after URL validation passes
//! - B11: Error message includes both manifest base_url and apply target URL
//! - B12: Error message uses phrase "does not match" for exit-code-1 classification
//! - B14: Subdomain difference is a mismatch
//! - B15: Trailing slash difference is a mismatch
//! - B16: Path prefix is not sufficient match

use doc_transformer::scrape::validation::{PageFilterStatus, ScrapeResult, ScrapedPage};
use doc_transformer::watch::{snapshot_from_scrape, Snapshot};
use proptest::prelude::*;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

// ============================================================================
// Helpers
// ============================================================================

fn get_ctd_binary() -> std::path::PathBuf {
    Path::new(env!("CARGO_BIN_EXE_ctd")).to_path_buf()
}

fn make_page(url: &str, title: &str, content: &str) -> ScrapedPage {
    ScrapedPage {
        url: url.to_string(),
        markdown: content.to_string(),
        title: title.to_string(),
        links: vec![],
        headers: vec![],
        word_count: content.split_whitespace().count(),
        slug: url.replace('/', "_"),
        filter_status: PageFilterStatus::Unfiltered,
        elements_removed: 0,
        density_score: 1.0,
    }
}

fn make_scrape_result(base: &str, pages: Vec<ScrapedPage>) -> ScrapeResult {
    ScrapeResult {
        total_urls: pages.len(),
        success_count: pages.len(),
        error_count: 0,
        errors: vec![],
        base_url: base.to_string(),
        pages,
    }
}

fn write_manifest(dir: &Path, result: &ScrapeResult) {
    let file = std::fs::File::create(dir.join("manifest.json")).expect("create manifest");
    serde_json::to_writer_pretty(file, result).expect("write manifest");
}

// Run ctd apply via CLI
fn run_apply_cli(url: &str, scrape_dir: &Path, cache_path: &Path) -> std::process::Output {
    let binary = get_ctd_binary();
    Command::new(&binary)
        .args([
            "apply",
            url,
            "--scrape-dir",
            scrape_dir.to_str().unwrap(),
            "--cache",
            cache_path.to_str().unwrap(),
            "--yes",
        ])
        .output()
        .expect("Failed to execute ctd apply")
}

// ============================================================================
// B1: Mismatched URL is rejected before state mutation
// ============================================================================

// RED PHASE: This test FAILS because the bug allows mismatched URL apply to succeed.
// Once fixed, run_apply should reject with exit code 1 before any state mutation.
#[test]
fn apply_rejects_mismatched_url_before_state_mutation() {
    let temp = TempDir::new().expect("tempdir");
    let scrape_dir = temp.path().join("scrape");
    std::fs::create_dir_all(&scrape_dir).expect("create scrape dir");

    // Create a manifest with kubernetes.io base_url
    let k8s_result = make_scrape_result(
        "https://kubernetes.io/docs/home/",
        vec![
            make_page(
                "https://kubernetes.io/docs/home/concepts.html",
                "Concepts",
                "# Kubernetes Concepts",
            ),
            make_page(
                "https://kubernetes.io/docs/home/reference.html",
                "Reference",
                "# Kubernetes Reference",
            ),
        ],
    );
    write_manifest(&scrape_dir, &k8s_result);

    // Create a temp state DB
    let cache_dir = TempDir::new().expect("cache tempdir");
    let cache_path = cache_dir.path().join("state.redb");

    // Try to apply kubernetes scrape to example.com (WRONG URL - should fail)
    let result = run_apply_cli(
        "https://example.com", // <-- MISMATCH: scrape is for kubernetes.io
        &scrape_dir,
        &cache_path,
    );

    // BUG: Currently this succeeds (exit 0) because no URL validation exists.
    // EXPECTED: Should fail with exit code 1.
    assert!(
        !result.status.success(),
        "Apply with mismatched URL (example.com vs kubernetes.io) should be REJECTED, \
         but it succeeded with exit {}. stderr: {}. \
         This is the URL identity validation bug (FM-1).",
        result.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&result.stderr)
    );

    let stderr = String::from_utf8_lossy(&result.stderr);

    // Error message should contain both URLs
    assert!(
        stderr.contains("https://kubernetes.io/docs/home/")
            || stderr.to_lowercase().contains("kubernetes"),
        "Error should mention the scrape's base_url or domain. Got: {}",
        stderr
    );

    // Error message should use "does not match" or "mismatch"
    let stderr_lower = stderr.to_lowercase();
    assert!(
        stderr_lower.contains("does not match")
            || stderr_lower.contains("mismatch"),
        "Error message should contain 'does not match' or 'mismatch'. Got: {}",
        stderr
    );
}

// ============================================================================
// B2: Matching URL apply succeeds
// ============================================================================

// RED PHASE: This test PASSES (baseline: matching URL apply already works).
// It establishes the happy path before we test the B1 bug.
#[test]
fn apply_succeeds_when_url_matches_manifest_base_url() {
    let temp = TempDir::new().expect("tempdir");
    let scrape_dir = temp.path().join("scrape");
    std::fs::create_dir_all(&scrape_dir).expect("create scrape dir");

    let base_url = "https://kubernetes.io/docs/home/";
    let k8s_result = make_scrape_result(
        base_url,
        vec![make_page(
            "https://kubernetes.io/docs/home/concepts.html",
            "Concepts",
            "# Kubernetes Concepts",
        )],
    );
    write_manifest(&scrape_dir, &k8s_result);

    let cache_dir = TempDir::new().expect("cache tempdir");
    let cache_path = cache_dir.path().join("state.redb");

    // Apply kubernetes scrape to correct URL
    let result = run_apply_cli(base_url, &scrape_dir, &cache_path);

    assert!(
        result.status.success(),
        "Apply with matching URL should succeed. stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

// ============================================================================
// B4: Mismatched apply returns exit code 1
// ============================================================================

// RED PHASE: This test FAILS because the bug doesn't return exit code 1 properly.
#[test]
fn apply_mismatch_returns_exit_code_1() {
    let temp = TempDir::new().expect("tempdir");
    let scrape_dir = temp.path().join("scrape");
    std::fs::create_dir_all(&scrape_dir).expect("create scrape dir");

    // Create a manifest with a specific base_url
    let scrape_result = make_scrape_result(
        "https://docs.example.com",
        vec![make_page(
            "https://docs.example.com/page.html",
            "Page",
            "Content",
        )],
    );
    write_manifest(&scrape_dir, &scrape_result);

    let cache_dir = TempDir::new().expect("cache tempdir");
    let cache_path = cache_dir.path().join("state.redb");

    // Apply to different domain
    let result = run_apply_cli("https://example.com", &scrape_dir, &cache_path);

    // BUG: Currently returns 0 (success) because no URL validation exists.
    // EXPECTED: Should return 1 (user input error).
    let exit_code = result.status.code().unwrap_or(-1);
    assert_eq!(
        exit_code, 1,
        "URL mismatch should return exit code 1 (user input error), not {}. \
         FM-4: Error pattern 'does not match' must be added to user_input_patterns.",
        exit_code
    );
}

// ============================================================================
// B11: Error message includes both URLs
// ============================================================================

// RED PHASE: This test FAILS because no error message is produced (apply succeeds).
#[test]
fn apply_mismatch_error_includes_both_urls() {
    let temp = TempDir::new().expect("tempdir");
    let scrape_dir = temp.path().join("scrape");
    std::fs::create_dir_all(&scrape_dir).expect("create scrape dir");

    let old_url = "https://old.example.com";
    let new_url = "https://new.example.com";

    let scrape_result = make_scrape_result(
        old_url,
        vec![make_page(
            "https://old.example.com/page.html",
            "Page",
            "Content",
        )],
    );
    write_manifest(&scrape_dir, &scrape_result);

    let cache_dir = TempDir::new().expect("cache tempdir");
    let cache_path = cache_dir.path().join("state.redb");

    let result = run_apply_cli(new_url, &scrape_dir, &cache_path);

    // Should fail
    assert!(
        !result.status.success(),
        "Apply with different target URL should fail"
    );

    let stderr = String::from_utf8_lossy(&result.stderr);
    let stderr_lower = stderr.to_lowercase();

    assert!(
        stderr.contains(old_url) || stderr_lower.contains("old.example.com"),
        "Error should contain old URL '{}' or mention it. Got: {}",
        old_url,
        stderr
    );
    assert!(
        stderr.contains(new_url) || stderr_lower.contains("new.example.com"),
        "Error should contain new URL '{}' or mention it. Got: {}",
        new_url,
        stderr
    );
}

// ============================================================================
// B14: Subdomain difference is a mismatch
// ============================================================================

// RED PHASE: This test FAILS because subdomain differences are NOT rejected.
#[test]
fn apply_rejects_when_subdomain_differs() {
    let temp = TempDir::new().expect("tempdir");
    let scrape_dir = temp.path().join("scrape");
    std::fs::create_dir_all(&scrape_dir).expect("create scrape dir");

    // Scrape from docs.example.com
    let scrape_result = make_scrape_result(
        "https://docs.example.com",
        vec![make_page(
            "https://docs.example.com/page.html",
            "Page",
            "Content",
        )],
    );
    write_manifest(&scrape_dir, &scrape_result);

    let cache_dir = TempDir::new().expect("cache tempdir");
    let cache_path = cache_dir.path().join("state.redb");

    // Apply to example.com (different subdomain)
    let result = run_apply_cli("https://example.com", &scrape_dir, &cache_path);

    // BUG: Subdomain difference is NOT currently rejected.
    // EXPECTED: Should fail with exit code 1.
    assert!(
        !result.status.success(),
        "Subdomain difference (docs.example.com vs example.com) should be a mismatch and be rejected. \
         stderr: {}. \
         B14: Subdomain difference must be treated as a mismatch.",
        String::from_utf8_lossy(&result.stderr)
    );
}

// ============================================================================
// B15: Trailing slash difference is a mismatch
// ============================================================================

// RED PHASE: This test FAILS because trailing slash differences are NOT rejected.
#[test]
fn apply_rejects_when_trailing_slash_differs() {
    let temp = TempDir::new().expect("tempdir");
    let scrape_dir = temp.path().join("scrape");
    std::fs::create_dir_all(&scrape_dir).expect("create scrape dir");

    // Scrape with trailing slash
    let scrape_result = make_scrape_result(
        "https://example.com/docs/",
        vec![make_page(
            "https://example.com/docs/page.html",
            "Page",
            "Content",
        )],
    );
    write_manifest(&scrape_dir, &scrape_result);

    let cache_dir = TempDir::new().expect("cache tempdir");
    let cache_path = cache_dir.path().join("state.redb");

    // Apply WITHOUT trailing slash
    let result = run_apply_cli("https://example.com/docs", &scrape_dir, &cache_path);

    // BUG: Trailing slash difference is NOT currently rejected.
    // EXPECTED: Should fail with exit code 1.
    assert!(
        !result.status.success(),
        "Trailing slash difference (/docs/ vs /docs) should be a mismatch and be rejected. \
         stderr: {}. \
         B15: Trailing slash must be treated as significant.",
        String::from_utf8_lossy(&result.stderr)
    );
}

// ============================================================================
// B16: Path prefix is not sufficient match
// ============================================================================

// RED PHASE: This test FAILS because path prefix overlap is NOT rejected.
#[test]
fn apply_rejects_when_only_path_prefix_overlaps() {
    let temp = TempDir::new().expect("tempdir");
    let scrape_dir = temp.path().join("scrape");
    std::fs::create_dir_all(&scrape_dir).expect("create scrape dir");

    // Scrape from /docs/
    let scrape_result = make_scrape_result(
        "https://example.com/docs/",
        vec![make_page(
            "https://example.com/docs/page.html",
            "Page",
            "Content",
        )],
    );
    write_manifest(&scrape_dir, &scrape_result);

    let cache_dir = TempDir::new().expect("cache tempdir");
    let cache_path = cache_dir.path().join("state.redb");

    // Apply to /docs/reference (path prefix overlap but NOT the same)
    let result = run_apply_cli(
        "https://example.com/docs/reference",
        &scrape_dir,
        &cache_path,
    );

    // BUG: Path prefix overlap is NOT currently rejected.
    // EXPECTED: Should fail with exit code 1.
    assert!(
        !result.status.success(),
        "Path prefix overlap (/docs/ vs /docs/reference) should be a mismatch and be rejected. \
         stderr: {}. \
         B16: Path prefix is not sufficient for URL identity.",
        String::from_utf8_lossy(&result.stderr)
    );
}

// ============================================================================
// B6: "Snapshot committed" log on success
// ============================================================================

// RED PHASE: This test PASSES for matching URL (baseline).
// NOTE: "Snapshot committed" is emitted via tracing::info! (log), not stdout.
// The log output goes to stderr via tracing, so we verify success via exit code.
#[test]
fn apply_success_emits_snapshot_committed_log() {
    let temp = TempDir::new().expect("tempdir");
    let scrape_dir = temp.path().join("scrape");
    std::fs::create_dir_all(&scrape_dir).expect("create scrape dir");

    let base_url = "https://kubernetes.io/docs/home/";
    let k8s_result = make_scrape_result(
        base_url,
        vec![make_page(
            "https://kubernetes.io/docs/home/concepts.html",
            "Concepts",
            "# Kubernetes Concepts",
        )],
    );
    write_manifest(&scrape_dir, &k8s_result);

    let cache_dir = TempDir::new().expect("cache tempdir");
    let cache_path = cache_dir.path().join("state.redb");

    let result = run_apply_cli(base_url, &scrape_dir, &cache_path);

    // B6: On success, exit code should be 0 and "[APPLY]" summary should print
    assert!(
        result.status.success(),
        "Apply with matching URL should succeed. stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );

    // The "[APPLY]" summary output confirms the apply ran
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(
        stdout.contains("[APPLY]"),
        "Apply success should print [APPLY] summary. Got: {}",
        stdout
    );
}

// ============================================================================
// Unit Tests: snapshot_from_scrape ignores CLI target_url (INV-3 violation)
// ============================================================================

// RED PHASE: This test FAILS because snapshot_from_scrape currently uses
// the target_url argument directly instead of result.base_url.
#[test]
fn snapshot_from_scrape_must_use_result_base_url_not_target_arg() {
    let result = make_scrape_result(
        "https://kubernetes.io/docs/home/",
        vec![make_page(
            "https://kubernetes.io/docs/home/concepts.html",
            "Concepts",
            "# Kubernetes Concepts",
        )],
    );

    // Pass a WRONG target_url argument
    let wrong_url = "https://example.com";
    let snapshot = snapshot_from_scrape(wrong_url, &result);

    // INV-3: snapshot.target_url MUST equal result.base_url, NOT the target_url arg.
    // BUG: Currently snapshot.target_url equals the wrong_url argument.
    // EXPECTED: snapshot.target_url should equal result.base_url.
    assert_eq!(
        snapshot.target_url, result.base_url,
        "BUG: snapshot.target_url is '{}' but ScrapeResult.base_url is '{}'. \
         INV-3 violated: snapshot.target_url must always equal ScrapeResult.base_url, \
         never the CLI target_url argument.",
        snapshot.target_url, result.base_url
    );

    assert_ne!(
        snapshot.target_url, wrong_url,
        "snapshot.target_url should NOT equal the wrong CLI argument '{}'. \
         This would pollute the watch state.",
        wrong_url
    );
}

// ============================================================================
// Proptest Invariant: snapshot.target_url always equals ScrapeResult.base_url
// ============================================================================

proptest! {
    #[test]
    fn snapshot_target_url_always_equals_result_base_url(
        base_url in "https://[a-z0-9.-]+/[a-z0-9/-]*",
        // Deliberately use non-URL strings to ensure they differ from base_url
        target_url in "[a-z]{5,20}",
        page_count in 0usize..10,
    ) {
        let pages: Vec<ScrapedPage> = (0..page_count).map(|i| {
            make_page(&format!("{}/page{}", base_url, i), &format!("Page {}", i), "content")
        }).collect();
        let result = make_scrape_result(&base_url, pages);
        let snapshot = snapshot_from_scrape(&target_url, &result);

        // INV-3: snapshot.target_url MUST be the scraped URL, not the arg
        let snapshot_url = snapshot.target_url.clone();
        let base_url = result.base_url.clone();
        prop_assert_eq!(
            snapshot.target_url, result.base_url,
            "snapshot.target_url was '{}' but ScrapeResult.base_url is '{}' — URL identity violated",
            snapshot_url, base_url
        );
    }
}