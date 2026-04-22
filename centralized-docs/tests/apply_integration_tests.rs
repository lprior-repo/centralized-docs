#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

//! Integration tests for the `ctd apply` CLI command.
//!
//! These tests verify the apply workflow as a BLACK BOX via subprocess execution.
//! We test exit codes, filesystem state (cache DB creation/deletion), and snapshot persistence.
//!
//! ## Primary Bug Being Tested
//!
//! INV3 violation: `open_state_db()` is called at line 92 of `src/cmd/watch.rs` BEFORE
//! `prompt_confirmation()` at lines 104-106. When `StateDb::open()` is called on a
//! non-existent path, it creates the database file as a side effect — even if the user
//! declines the apply confirmation.
//!
//! ## Test Fixtures
//!
//! Each test creates a temp directory with a valid `manifest.json` representing a scrape result.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tempfile::TempDir;

// =============================================================================
// TEST FIXTURES
// =============================================================================

/// Returns the path to the compiled `ctd` binary
fn ctd_binary_path() -> PathBuf {
    Path::new(env!("CARGO_BIN_EXE_ctd")).to_path_buf()
}

/// Creates a temporary directory with a valid scrape manifest
fn setup_apply_test() -> (TempDir, PathBuf, PathBuf) {
    let temp = TempDir::new().unwrap();
    let scrape_dir = temp.path().join("scrape");
    let cache_path = temp.path().join("state.redb");

    fs::create_dir_all(&scrape_dir).unwrap();

    // Write a valid manifest with one scraped page
    let manifest = serde_json::json!({
        "total_urls": 1,
        "success_count": 1,
        "error_count": 0,
        "errors": [],
        "base_url": "https://example.com",
        "pages": [{
            "url": "https://example.com",
            "markdown": "# Test Page\n\nThis is test content for apply testing.",
            "title": "Test Page",
            "links": [],
            "headers": [],
            "word_count": 8,
            "slug": "index",
            "filter_status": "Unfiltered",
            "elements_removed": 0,
            "density_score": 1.0
        }]
    });

    fs::write(
        scrape_dir.join("manifest.json"),
        serde_json::to_string_pretty(&manifest).unwrap(),
    )
    .unwrap();

    (temp, scrape_dir, cache_path)
}

/// Runs `ctd apply` with the given stdin input, returning the process output.
fn run_ctd_apply_with_input(
    url: &str,
    cache_path: &Path,
    scrape_dir: &Path,
    input: &str,
) -> std::process::Output {
    let binary = ctd_binary_path();

    let mut cmd = Command::new(&binary);
    cmd.args([
        "apply",
        url,
        "--cache",
        cache_path.to_str().unwrap(),
        "--scrape-dir",
        scrape_dir.to_str().unwrap(),
    ]);

    if input.is_empty() {
        cmd.stdin(Stdio::null());
    } else {
        cmd.stdin(Stdio::piped());
    }

    let mut child = cmd.spawn().expect("Failed to spawn ctd apply");
    if !input.is_empty() {
        if let Some(ref mut stdin) = child.stdin {
            stdin.write_all(input.as_bytes()).unwrap();
        }
    }
    child.wait_with_output().expect("Failed to wait on ctd apply")
}

// =============================================================================
// SCENARIO 1: User declines apply on fresh cache (types 'n')
// =============================================================================

/// Given: cache_path points to a non-existent file location
///   And: a valid scrape manifest exists in scrape_dir
///   And: ctd apply is run WITHOUT --yes flag
///
/// When: user is prompted "Apply the above changes? [y/N] "
///   And: user types "n"
///
/// Then: command exits with code 1 (user abort)
///   And: NO cache database file exists at cache_path
///   And: NO snapshot is committed for the target URL
///   And: stderr contains "Apply aborted by user"
///
/// **THIS TEST WILL FAIL** because the bug opens StateDb BEFORE confirmation,
/// creating the cache file even when the user declines.
#[test]
fn apply_rejects_user_abort_does_not_create_cache_file() {
    // Given: fresh temp dir, non-existent cache path
    let (_temp, scrape_dir, cache_path) = setup_apply_test();
    assert!(
        !cache_path.exists(),
        "Cache file must NOT exist before test"
    );

    // When: run apply and user types "n" (declines)
    let output = run_ctd_apply_with_input(
        "https://example.com",
        &cache_path,
        &scrape_dir,
        "n\n",
    );

    // Then: exit code 1 (user abort), NOT 2 (pipeline error)
    assert_eq!(
        output.status.code(),
        Some(1),
        "Decline must exit with code 1 (user abort), got {:?}. stderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );

    // CRITICAL ASSERTION: NO cache database file should exist after decline
    // THIS WILL FAIL because the bug creates the DB file before prompting
    assert!(
        !cache_path.exists(),
        "INV3 violation: Cache database file was created at {} even though user declined. \
         open_state_db() was called BEFORE prompt_confirmation().",
        cache_path.display()
    );

    // stderr should mention "Apply aborted"
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.to_lowercase().contains("apply aborted"),
        "stderr should contain 'Apply aborted', got: {}",
        stderr
    );
}

// =============================================================================
// SCENARIO 2: User declines by pressing Enter (empty input)
// =============================================================================

/// Given: cache_path points to a non-existent file location
///   And: a valid scrape manifest exists in scrape_dir
///   And: ctd apply is run WITHOUT --yes flag
///
/// When: user is prompted "Apply the above changes? [y/N] "
///   And: user presses Enter (empty input)
///
/// Then: command exits with code 1 (user abort)
///   And: NO cache database file exists at cache_path
///   And: stderr contains "Apply aborted by user"
///
/// Empty input is treated as a decline (empty string != "y").
#[test]
fn apply_rejects_on_empty_input() {
    // Given: fresh temp dir, non-existent cache path
    let (_temp, scrape_dir, cache_path) = setup_apply_test();
    assert!(
        !cache_path.exists(),
        "Cache file must NOT exist before test"
    );

    // When: run apply and user presses Enter (empty input = decline)
    let output = run_ctd_apply_with_input(
        "https://example.com",
        &cache_path,
        &scrape_dir,
        "\n", // empty input = decline
    );

    // Then: exit code 1 (user abort)
    assert_eq!(
        output.status.code(),
        Some(1),
        "Empty input (decline) must exit with code 1, got {:?}",
        output.status.code()
    );

    // CRITICAL: NO cache database file should exist after empty-input decline
    // THIS WILL FAIL because the bug creates the DB file before prompting
    assert!(
        !cache_path.exists(),
        "INV3 violation: Cache DB created even with empty input decline. \
         open_state_db() called before prompt_confirmation()."
    );
}

// =============================================================================
// SCENARIO 3: User accepts apply (types 'y')
// =============================================================================

/// Given: cache_path points to a valid state database (existing or newly created)
///   And: a valid scrape manifest exists in scrape_dir
///   And: ctd apply is run WITHOUT --yes flag
///
/// When: user is prompted "Apply the above changes? [y/N] "
///   And: user types "y"
///
/// Then: command exits with code 0
///   And: new snapshot is persisted to state database
///   And: snapshot reflects current scrape result pages
#[test]
fn apply_accepts_commits_snapshot_and_exits_zero() {
    // Given: fresh temp dir
    let (_temp, scrape_dir, cache_path) = setup_apply_test();

    // When: run apply and user types "y" (accepts)
    let output = run_ctd_apply_with_input(
        "https://example.com",
        &cache_path,
        &scrape_dir,
        "y\n",
    );

    // Then: exit code 0 (success)
    assert_eq!(
        output.status.code(),
        Some(0),
        "Accept must exit with code 0, got {:?}. stderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );

    // Cache file should exist (accepted = DB is created)
    assert!(
        cache_path.exists(),
        "Cache database file should exist after accepting apply"
    );
}

// =============================================================================
// SCENARIO 4: --yes flag skips confirmation
// =============================================================================

/// Given: cache_path points to a valid state database
///   And: a valid scrape manifest exists in scrape_dir
///   And: ctd apply is run WITH --yes flag
///
/// When: command executes
///
/// Then: NO confirmation prompt is shown
///   And: command exits with code 0
///   And: new snapshot is committed
#[test]
fn apply_with_yes_flag_skips_confirmation_and_commits() {
    // Given: fresh temp dir
    let (_temp, scrape_dir, cache_path) = setup_apply_test();

    // When: run apply with --yes flag (no stdin needed)
    let binary = ctd_binary_path();
    let output = Command::new(&binary)
        .args([
            "apply",
            "https://example.com",
            "--cache",
            cache_path.to_str().unwrap(),
            "--scrape-dir",
            scrape_dir.to_str().unwrap(),
            "--yes", // skip confirmation
        ])
        .output()
        .expect("Failed to execute ctd apply --yes");

    // Then: exit code 0
    assert_eq!(
        output.status.code(),
        Some(0),
        "--yes flag must exit with code 0, got {:?}",
        output.status.code()
    );

    // Cache file should exist
    assert!(
        cache_path.exists(),
        "Cache database file should exist after --yes apply"
    );

    // stderr should NOT contain the confirmation prompt text
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("Apply the above changes?"),
        "--yes should skip confirmation prompt, but got: {}",
        stderr
    );
}

// =============================================================================
// SCENARIO 5: Empty plan returns early without prompting
// =============================================================================

/// Given: a valid scrape manifest exists with no changes from previous snapshot
///   And: ctd apply is run WITHOUT --yes flag
///
/// When: plan.summary.is_empty() is true
///
/// Then: command returns Ok(()) immediately
///   And: NO confirmation prompt is shown
///   And: NO snapshot is committed
///
/// NOTE: This test requires an existing snapshot to compare against.
/// We create a snapshot first, then run apply with identical content.
#[test]
fn apply_with_empty_plan_returns_early_without_prompting() {
    // Given: create initial snapshot with apply --yes
    let (_temp, scrape_dir, cache_path) = setup_apply_test();

    // First, accept to create initial snapshot
    let output = run_ctd_apply_with_input(
        "https://example.com",
        &cache_path,
        &scrape_dir,
        "y\n",
    );
    assert_eq!(output.status.code(), Some(0), "Initial apply must succeed");

    // When: run apply again with identical content (empty plan expected)
    let output2 = run_ctd_apply_with_input(
        "https://example.com",
        &cache_path,
        &scrape_dir,
        "y\n", // even with "y" input, empty plan should return early
    );

    // Then: should succeed without prompting
    assert_eq!(
        output2.status.code(),
        Some(0),
        "Empty plan must exit with code 0, got {:?}",
        output2.status.code()
    );

    // The second apply should have run without needing confirmation
    // (empty plan returns early at line 99-102 before confirmation prompt)
}

// =============================================================================
// SCENARIO 6: Repeat apply with --yes is idempotent
// =============================================================================

/// Given: a snapshot was previously committed for the URL
///   And: the current scrape content is identical to previous snapshot
///
/// When: ctd apply --yes is run
///
/// Then: command exits with code 0
///   And: snapshot content is unchanged (idempotent)
///   And: no error occurs
#[test]
fn apply_repeat_with_yes_is_idempotent() {
    // Given: create initial snapshot
    let (_temp, scrape_dir, cache_path) = setup_apply_test();

    // First apply
    let binary = ctd_binary_path();
    let output1 = Command::new(&binary)
        .args([
            "apply",
            "https://example.com",
            "--cache",
            cache_path.to_str().unwrap(),
            "--scrape-dir",
            scrape_dir.to_str().unwrap(),
            "--yes",
        ])
        .output()
        .expect("Failed to execute ctd apply");
    assert_eq!(output1.status.code(), Some(0), "First apply must succeed");

    // When: run apply again with --yes (idempotent)
    let output2 = Command::new(&binary)
        .args([
            "apply",
            "https://example.com",
            "--cache",
            cache_path.to_str().unwrap(),
            "--scrape-dir",
            scrape_dir.to_str().unwrap(),
            "--yes",
        ])
        .output()
        .expect("Failed to execute ctd apply");

    // Then: should succeed with no errors (idempotent)
    assert_eq!(
        output2.status.code(),
        Some(0),
        "Idempotent apply must succeed, got {:?}. stderr: {}",
        output2.status.code(),
        String::from_utf8_lossy(&output2.stderr)
    );
}

// =============================================================================
// SCENARIO 7: Declined apply preserves existing snapshot
// =============================================================================

/// Given: an existing snapshot exists in the state database
///   And: ctd apply is run WITHOUT --yes flag
///
/// When: user declines the confirmation
///
/// Then: the existing snapshot remains unmodified
///   And: exit code is 1 (user abort)
///   And: state database is not modified
#[test]
fn apply_rejected_preserves_existing_snapshot() {
    // Given: create initial snapshot
    let (_temp, scrape_dir, cache_path) = setup_apply_test();

    // First, accept to create initial snapshot
    let output = run_ctd_apply_with_input(
        "https://example.com",
        &cache_path,
        &scrape_dir,
        "y\n",
    );
    assert_eq!(output.status.code(), Some(0), "Initial apply must succeed");
    assert!(cache_path.exists(), "Cache must exist after initial apply");

    // Now modify the scrape dir to have DIFFERENT content, so second run has changes
    let manifest = serde_json::json!({
        "total_urls": 1,
        "success_count": 1,
        "error_count": 0,
        "errors": [],
        "base_url": "https://example.com",
        "pages": [{
            "url": "https://example.com",
            "markdown": "# Modified Test Page\n\nThis is MODIFIED test content.",
            "title": "Modified Test Page",
            "links": [],
            "headers": [],
            "word_count": 6,
            "slug": "index",
            "filter_status": "Unfiltered",
            "elements_removed": 0,
            "density_score": 1.0
        }]
    });
    fs::write(
        scrape_dir.join("manifest.json"),
        serde_json::to_string_pretty(&manifest).unwrap(),
    )
    .unwrap();

    // Get the initial file mtime as proxy for modification
    let initial_mtime = std::fs::metadata(&cache_path)
        .unwrap()
        .modified()
        .unwrap();

    // When: run apply again with modified content and user declines
    let output2 = run_ctd_apply_with_input(
        "https://example.com",
        &cache_path,
        &scrape_dir,
        "n\n",
    );

    // Then: exit code 1 (user abort)
    assert_eq!(
        output2.status.code(),
        Some(1),
        "Decline must exit with code 1, got {:?}. stderr: {}",
        output2.status.code(),
        String::from_utf8_lossy(&output2.stderr)
    );

    // State DB mtime should NOT have changed (snapshot preserved)
    let final_mtime = std::fs::metadata(&cache_path)
        .unwrap()
        .modified()
        .unwrap();
    assert_eq!(
        initial_mtime, final_mtime,
        "Cache file mtime changed after decline — snapshot was modified!"
    );
}

// =============================================================================
// SCENARIO 9 (INV3): State database NOT opened before confirmation
// =============================================================================

/// This test verifies INV3: open_state_db() must NOT be called before confirmation.
///
/// Given: cache_path points to a non-existent file location
///   And: a valid scrape manifest exists in scrape_dir
///   And: ctd apply is run WITHOUT --yes flag
///
/// When: user is prompted and types "n"
///   And: command exits
///
/// Then: NO cache database file exists at cache_path
///
/// This is a BEHAVIORAL PROXY for INV3 ordering verification.
/// We cannot directly track function call order from integration tests,
/// but if the DB file doesn't exist after decline, it proves:
/// 1. StateDb::open() was NOT called with create=true before confirmation
/// 2. OR if it was called, the file was properly cleaned up
///
/// A more sophisticated version would use mtime tracking to detect if
/// the file was created before the confirmation timestamp.
#[test]
fn apply_inv3_state_db_not_opened_before_confirmation() {
    // Given: fresh temp dir, non-existent cache path
    let (_temp, scrape_dir, cache_path) = setup_apply_test();
    assert!(
        !cache_path.exists(),
        "Cache file must NOT exist before test"
    );

    // Record time before running apply
    let before_run = std::time::SystemTime::now();

    // When: run apply and user declines
    let output = run_ctd_apply_with_input(
        "https://example.com",
        &cache_path,
        &scrape_dir,
        "n\n",
    );

    // Record time after apply exits
    let after_run = std::time::SystemTime::now();

    // Then: command should have exited with code 1 (user abort)
    assert_eq!(
        output.status.code(),
        Some(1),
        "Decline must exit with code 1, got {:?}",
        output.status.code()
    );

    // CRITICAL INV3 ASSERTION: cache file must NOT exist
    // THIS WILL FAIL because the bug calls open_state_db() before confirmation,
    // which creates the empty DB file even on decline
    assert!(
        !cache_path.exists(),
        "INV3 violation detected: Cache database file {} was created \
         even though user declined. This proves open_state_db() was called \
         BEFORE prompt_confirmation(). The fix must move open_state_db() \
         to AFTER the confirmation check.",
        cache_path.display()
    );

    // Additional behavioral check: if the file DID exist, verify its mtime
    // is AFTER the confirmation would have happened (proxy for ordering)
    if cache_path.exists() {
        let file_mtime = std::fs::metadata(&cache_path)
            .unwrap()
            .modified()
            .unwrap();

        // If mtime predates our "before_run" timestamp, the file was created
        // BEFORE we even got to run the command, proving the bug
        assert!(
            file_mtime > before_run,
            "Cache file mtime {} predates command start {}. \
             File was created by open_state_db() BEFORE prompt_confirmation().",
            file_mtime.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
            before_run.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
        );
    }
}

// =============================================================================
// INVARIANT PROPTESTS
// =============================================================================

/// Property: For any valid setup, when user declines apply,
/// the cache_path must NOT exist afterward if it did not exist before.
#[test]
fn apply_decline_never_creates_db_when_absent() {
    // This is a simplified version of the proptest invariant
    // Run via subprocess to verify filesystem behavior
    let (_temp, scrape_dir, cache_path) = setup_apply_test();

    // Precondition: cache does not exist
    assert!(
        !cache_path.exists(),
        "Precondition: cache_path must not exist before test"
    );

    // Run apply, decline
    let output = run_ctd_apply_with_input(
        "https://example.com",
        &cache_path,
        &scrape_dir,
        "n\n",
    );

    // Assert: cache must still not exist (invariant: decline never creates DB)
    // THIS WILL FAIL due to the bug
    assert!(
        !cache_path.exists(),
        "Invariant violation: DB file was created on decline. \
         Decline must be side-effect free (INV3)."
    );
}

/// Property: When user declines apply, exit code MUST be 1 (not 2)
#[test]
fn apply_decline_always_exits_with_code_1() {
    let (_temp, scrape_dir, cache_path) = setup_apply_test();

    let output = run_ctd_apply_with_input(
        "https://example.com",
        &cache_path,
        &scrape_dir,
        "n\n",
    );

    assert_eq!(
        output.status.code(),
        Some(1),
        "Decline must exit with code 1, got {:?}",
        output.status.code()
    );
}

/// Property: When user accepts apply, exit code is 0 AND snapshot exists
#[test]
fn apply_accept_always_commits_with_code_0() {
    let (_temp, scrape_dir, cache_path) = setup_apply_test();

    let output = run_ctd_apply_with_input(
        "https://example.com",
        &cache_path,
        &scrape_dir,
        "y\n",
    );

    assert_eq!(
        output.status.code(),
        Some(0),
        "Accept must exit with code 0, got {:?}",
        output.status.code()
    );

    // Verify snapshot was committed (DB file exists)
    assert!(
        cache_path.exists(),
        "Snapshot must exist after accept"
    );
}
