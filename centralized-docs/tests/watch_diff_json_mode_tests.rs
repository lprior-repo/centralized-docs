#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

//! Integration tests for `ctd watch --json` and `ctd diff --json` JSON error mode.
//!
//! These tests verify that when `watch` or `diff` commands fail in JSON mode,
//! they emit machine-readable JSON error payloads to stdout instead of plain-text
//! errors to stderr.
//!
//! ## The Bug
//!
//! In `dispatch.rs`, the `Watch` and `Diff` command variants return `None` as the
//! `search_context`, while only `Search` populates it with `Some((json, query))`.
//! When `handle_error()` sees `None`, it falls through to plain text error output
//! instead of JSON.
//!
//! ## Expected Behavior (POST-FIX)
//!
//! - `ctd watch --json` failure → JSON error to stdout with `{"status": "error", "error": "...", "command": "watch"}`
//! - `ctd diff --json` failure → JSON error to stdout with `{"status": "error", "error": "...", "command": "diff"}`
//! - Exit codes remain stable: 1 for user input errors, 2 for pipeline/network errors
//! - Plain-text error mode (no `--json`) remains unchanged

use std::fs;
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

/// Creates a minimal scrape directory with manifest.json and INDEX.json
fn create_valid_scrape_dir(dir: &Path, name: &str) {
    let docs_dir = dir.join("docs");
    fs::create_dir_all(&docs_dir).unwrap();

    // Create a simple manifest.json
    let manifest = serde_json::json!({
        "version": "1.0",
        "name": name,
        "url": "https://example.com",
        "scraped_at": "2024-01-01T00:00:00Z"
    });
    fs::write(dir.join("manifest.json"), serde_json::to_string_pretty(&manifest).unwrap()).unwrap();

    // Create INDEX.json with empty chunks
    let index = serde_json::json!({
        "documents": [],
        "chunks": [],
        "keywords": {}
    });
    fs::write(dir.join("INDEX.json"), serde_json::to_string_pretty(&index).unwrap()).unwrap();

    // Create a doc file
    fs::write(docs_dir.join("test.md"), "# Test Document\n\nTest content.").unwrap();
}

/// Creates a corrupt scrape directory with malformed manifest.json
fn create_corrupt_scrape_dir(dir: &Path) {
    let docs_dir = dir.join("docs");
    fs::create_dir_all(&docs_dir).unwrap();

    // Write a corrupt manifest.json (not valid JSON)
    fs::write(dir.join("manifest.json"), "this is not json{").unwrap();

    fs::write(docs_dir.join("test.md"), "# Test\n\nContent.").unwrap();
}

// ---------------------------------------------------------------------------
// BDD Scenario: `diff --json` emits JSON error on manifest-not-found
// ---------------------------------------------------------------------------

/// Given: Directory `/nope-a` does not exist or has no `manifest.json`
/// When: User runs `ctd diff /nope-a /nope-b --json`
/// Then:
///   - stdout is valid JSON (parseable by `serde_json::from_str`)
///   - stdout contains `"status": "error"`
///   - stdout contains `"command": "diff"`
///   - stdout contains `"error":` with non-empty error message
///   - stderr is empty (NOT plain-text "Error: ...")
///   - exit code is 1
#[test]
fn diff_json_error_manifest_not_found_emits_json_error_payload() {
    let result = run_cli(&["diff", "/nope-a", "/nope-b", "--json"]);

    let stdout = String::from_utf8_lossy(&result.stdout);
    let stderr = String::from_utf8_lossy(&result.stderr);

    // Exit code must be 1 (user input error: directory not found)
    assert_eq!(
        result.status.code(),
        Some(1),
        "diff --json with missing dirs should exit with code 1, got {:?}. stderr: {stderr}",
        result.status.code()
    );

    // stdout must be valid JSON
    let json: serde_json::Value = serde_json::from_str(&stdout).expect(
        format!(
            "diff --json error output must be valid JSON. stdout: {stdout}, stderr: {stderr}"
        )
        .as_str(),
    );

    // Must have "status": "error"
    assert_eq!(
        json.get("status").and_then(|v| v.as_str()),
        Some("error"),
        "JSON error must have status='error'. Full JSON: {json}"
    );

    // Must have "command": "diff"
    assert_eq!(
        json.get("command").and_then(|v| v.as_str()),
        Some("diff"),
        "JSON error must have command='diff'. Full JSON: {json}"
    );

    // Must have non-empty "error" field
    let error_msg = json
        .get("error")
        .and_then(|v| v.as_str())
        .expect("JSON error must have error field");
    assert!(
        !error_msg.is_empty(),
        "JSON error message must be non-empty. Full JSON: {json}"
    );

    // stderr must NOT contain "Error:" (plain-text fallback bug)
    assert!(
        !stderr.contains("Error:"),
        "diff --json should NOT emit plain-text 'Error:' to stderr. Got stderr: {stderr}"
    );

    // stderr should be empty (JSON goes to stdout, not stderr)
    assert!(
        stderr.is_empty(),
        "diff --json error should emit JSON to stdout, not stderr. stderr: {stderr}"
    );
}

// ---------------------------------------------------------------------------
// BDD Scenario: `diff --json` emits JSON error on pipeline failure
// ---------------------------------------------------------------------------

/// Given: Directories exist but contain corrupt `manifest.json`
/// When: User runs `ctd diff <corrupt_dir_a> <corrupt_dir_b> --json`
/// Then:
///   - stdout is valid JSON with `"status": "error"`, `"command": "diff"`, `"error":`
///   - stderr is empty
///   - exit code is 2
#[test]
fn diff_json_pipeline_error_emits_json_error_payload_and_exit_2() {
    let temp = TempDir::new().unwrap();
    let corrupt_a = temp.path().join("corrupt_a");
    let corrupt_b = temp.path().join("corrupt_b");
    fs::create_dir_all(&corrupt_a).unwrap();
    fs::create_dir_all(&corrupt_b).unwrap();
    create_corrupt_scrape_dir(&corrupt_a);
    create_corrupt_scrape_dir(&corrupt_b);

    let result = run_cli(&[
        "diff",
        corrupt_a.to_str().unwrap(),
        corrupt_b.to_str().unwrap(),
        "--json",
    ]);

    let stdout = String::from_utf8_lossy(&result.stdout);
    let stderr = String::from_utf8_lossy(&result.stderr);

    // Exit code must be 2 (pipeline/internal error with corrupt data)
    assert_eq!(
        result.status.code(),
        Some(2),
        "diff --json with corrupt data should exit with code 2, got {:?}. stderr: {stderr}",
        result.status.code()
    );

    // stdout must be valid JSON
    let json: serde_json::Value = serde_json::from_str(&stdout).expect(
        format!(
            "diff --json error output must be valid JSON. stdout: {stdout}, stderr: {stderr}"
        )
        .as_str(),
    );

    // Must have "status": "error"
    assert_eq!(
        json.get("status").and_then(|v| v.as_str()),
        Some("error"),
        "JSON error must have status='error'. Full JSON: {json}"
    );

    // Must have "command": "diff"
    assert_eq!(
        json.get("command").and_then(|v| v.as_str()),
        Some("diff"),
        "JSON error must have command='diff'. Full JSON: {json}"
    );

    // Must have non-empty "error" field
    let error_msg = json
        .get("error")
        .and_then(|v| v.as_str())
        .expect("JSON error must have error field");
    assert!(
        !error_msg.is_empty(),
        "JSON error message must be non-empty. Full JSON: {json}"
    );

    // stderr must NOT contain "Error:" (plain-text fallback bug)
    assert!(
        !stderr.contains("Error:"),
        "diff --json should NOT emit plain-text 'Error:' to stderr. Got stderr: {stderr}"
    );
}

// ---------------------------------------------------------------------------
// BDD Scenario: `watch --json` emits JSON error on network failure
// ---------------------------------------------------------------------------

/// Given: No server is listening on `127.0.0.1:9` (connection refused)
/// When: User runs `ctd watch http://127.0.0.1:9 --output <dir> --cache <db> --json`
/// Then:
///   - stdout is valid JSON with `"status": "error"`, `"command": "watch"`, `"error":`
///   - stderr is empty
///   - exit code is 2
#[test]
fn watch_json_network_error_emits_json_error_payload_and_exit_2() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");
    let cache_dir = temp.path().join("cache");
    fs::create_dir_all(&output_dir).unwrap();
    fs::create_dir_all(&cache_dir).unwrap();

    let result = run_cli(&[
        "watch",
        "http://127.0.0.1:9",
        "--output",
        output_dir.to_str().unwrap(),
        "--cache",
        cache_dir.join("test.redb").to_str().unwrap(),
        "--json",
    ]);

    let stdout = String::from_utf8_lossy(&result.stdout);
    let stderr = String::from_utf8_lossy(&result.stderr);

    // Exit code must be 2 (pipeline/network error: connection refused)
    assert_eq!(
        result.status.code(),
        Some(2),
        "watch --json with connection refused should exit with code 2, got {:?}. stderr: {stderr}",
        result.status.code()
    );

    // stdout must be valid JSON
    let json: serde_json::Value = serde_json::from_str(&stdout).expect(
        format!(
            "watch --json error output must be valid JSON. stdout: {stdout}, stderr: {stderr}"
        )
        .as_str(),
    );

    // Must have "status": "error"
    assert_eq!(
        json.get("status").and_then(|v| v.as_str()),
        Some("error"),
        "JSON error must have status='error'. Full JSON: {json}"
    );

    // Must have "command": "watch"
    assert_eq!(
        json.get("command").and_then(|v| v.as_str()),
        Some("watch"),
        "JSON error must have command='watch'. Full JSON: {json}"
    );

    // Must have non-empty "error" field
    let error_msg = json
        .get("error")
        .and_then(|v| v.as_str())
        .expect("JSON error must have error field");
    assert!(
        !error_msg.is_empty(),
        "JSON error message must be non-empty. Full JSON: {json}"
    );

    // stderr must NOT contain "Error:" (plain-text fallback bug)
    assert!(
        !stderr.contains("Error:"),
        "watch --json should NOT emit plain-text 'Error:' to stderr. Got stderr: {stderr}"
    );
}

// ---------------------------------------------------------------------------
// BDD Scenario: `watch --json` emits JSON error on user input failure
// ---------------------------------------------------------------------------

/// Given: Invalid URL that fails validation before network
/// When: User runs `ctd watch "not-a-url" --output <dir> --cache <db> --json`
/// Then:
///   - stdout is valid JSON with `"status": "error"`, `"command": "watch"`
///   - stderr is empty
///   - exit code is 1
#[test]
fn watch_json_user_input_error_emits_json_error_payload_and_exit_1() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");
    let cache_dir = temp.path().join("cache");
    fs::create_dir_all(&output_dir).unwrap();
    fs::create_dir_all(&cache_dir).unwrap();

    let result = run_cli(&[
        "watch",
        "not-a-url",
        "--output",
        output_dir.to_str().unwrap(),
        "--cache",
        cache_dir.join("test.redb").to_str().unwrap(),
        "--json",
    ]);

    let stdout = String::from_utf8_lossy(&result.stdout);
    let stderr = String::from_utf8_lossy(&result.stderr);

    // Exit code must be 1 (user input error: invalid URL)
    assert_eq!(
        result.status.code(),
        Some(1),
        "watch --json with invalid URL should exit with code 1, got {:?}. stderr: {stderr}",
        result.status.code()
    );

    // stdout must be valid JSON
    let json: serde_json::Value = serde_json::from_str(&stdout).expect(
        format!(
            "watch --json error output must be valid JSON. stdout: {stdout}, stderr: {stderr}"
        )
        .as_str(),
    );

    // Must have "status": "error"
    assert_eq!(
        json.get("status").and_then(|v| v.as_str()),
        Some("error"),
        "JSON error must have status='error'. Full JSON: {json}"
    );

    // Must have "command": "watch"
    assert_eq!(
        json.get("command").and_then(|v| v.as_str()),
        Some("watch"),
        "JSON error must have command='watch'. Full JSON: {json}"
    );

    // Must have non-empty "error" field
    let error_msg = json
        .get("error")
        .and_then(|v| v.as_str())
        .expect("JSON error must have error field");
    assert!(
        !error_msg.is_empty(),
        "JSON error message must be non-empty. Full JSON: {json}"
    );

    // stderr must NOT contain "Error:" (plain-text fallback bug)
    assert!(
        !stderr.contains("Error:"),
        "watch --json should NOT emit plain-text 'Error:' to stderr. Got stderr: {stderr}"
    );
}

// ---------------------------------------------------------------------------
// BDD Scenario: `diff` without `--json` still emits plain-text error
// ---------------------------------------------------------------------------

/// Given: Directory `/nope-a` does not exist
/// When: User runs `ctd diff /nope-a /nope-b` (no `--json` flag)
/// Then:
///   - stderr contains `Error: ...`
///   - stdout is empty (no JSON)
///   - exit code is 1
#[test]
fn diff_no_json_still_emits_plain_text_error() {
    let result = run_cli(&["diff", "/nope-a", "/nope-b"]);

    let stdout = String::from_utf8_lossy(&result.stdout);
    let stderr = String::from_utf8_lossy(&result.stderr);

    // Exit code must be 1
    assert_eq!(
        result.status.code(),
        Some(1),
        "diff without --json with missing dirs should exit with code 1"
    );

    // stderr must contain "Error:"
    assert!(
        stderr.contains("Error:"),
        "diff without --json should emit plain-text error to stderr. stderr: {stderr}"
    );

    // stdout should be empty (no JSON)
    assert!(
        stdout.is_empty(),
        "diff without --json should NOT emit JSON to stdout. stdout: {stdout}"
    );
}

// ---------------------------------------------------------------------------
// BDD Scenario: `watch` without `--json` still emits plain-text error
// ---------------------------------------------------------------------------

/// Given: No server on `127.0.0.1:9`
/// When: User runs `ctd watch http://127.0.0.1:9 --output <dir> --cache <db>` (no `--json`)
/// Then:
///   - stderr contains `Error: ...`
///   - stdout is empty
///   - exit code is 2
#[test]
fn watch_no_json_still_emits_plain_text_error() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");
    let cache_dir = temp.path().join("cache");
    fs::create_dir_all(&output_dir).unwrap();
    fs::create_dir_all(&cache_dir).unwrap();

    let result = run_cli(&[
        "watch",
        "http://127.0.0.1:9",
        "--output",
        output_dir.to_str().unwrap(),
        "--cache",
        cache_dir.join("test.redb").to_str().unwrap(),
    ]);

    let stdout = String::from_utf8_lossy(&result.stdout);
    let stderr = String::from_utf8_lossy(&result.stderr);

    // Exit code must be 2
    assert_eq!(
        result.status.code(),
        Some(2),
        "watch without --json with connection refused should exit with code 2"
    );

    // stderr must contain "Error:"
    assert!(
        stderr.contains("Error:"),
        "watch without --json should emit plain-text error to stderr. stderr: {stderr}"
    );

    // stdout should be empty (no JSON)
    assert!(
        stdout.is_empty(),
        "watch without --json should NOT emit JSON to stdout. stdout: {stdout}"
    );
}

// ---------------------------------------------------------------------------
// BDD Scenario: `watch --json` JSON error payload schema is exact (no extraneous fields)
// ---------------------------------------------------------------------------

/// Given: `ctd watch --json` fails with any error (e.g., connection refused)
/// When: Command fails with JSON mode requested
/// Then: JSON payload contains exactly 3 fields:
///   - `status`: string value `"error"` exactly
///   - `error`: non-empty string
///   - `command`: string value `"watch"` exactly
/// And NO other fields exist (e.g., no `query` field)
#[test]
fn watch_json_error_payload_schema_is_exact_no_extra_fields() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");
    let cache_dir = temp.path().join("cache");
    fs::create_dir_all(&output_dir).unwrap();
    fs::create_dir_all(&cache_dir).unwrap();

    let result = run_cli(&[
        "watch",
        "http://127.0.0.1:9",
        "--output",
        output_dir.to_str().unwrap(),
        "--cache",
        cache_dir.join("test.redb").to_str().unwrap(),
        "--json",
    ]);

    let stdout = String::from_utf8_lossy(&result.stdout);

    // stdout must be valid JSON
    let json: serde_json::Value = serde_json::from_str(&stdout).expect(
        format!("watch --json error output must be valid JSON. stdout: {stdout}").as_str(),
    );

    // Must have exactly 3 fields (no extras like "query")
    let field_count = json.as_object().expect("JSON must be an object").len();
    assert_eq!(
        field_count, 3,
        "watch JSON error must have exactly 3 fields, got {field_count}. Fields: {json}"
    );

    // Must have "status": "error"
    assert_eq!(
        json.get("status").and_then(|v| v.as_str()),
        Some("error"),
        "status must be 'error'. Full JSON: {json}"
    );

    // Must have "command": "watch"
    assert_eq!(
        json.get("command").and_then(|v| v.as_str()),
        Some("watch"),
        "command must be 'watch'. Full JSON: {json}"
    );

    // Must have non-empty "error" string
    let error_val = json.get("error").expect("JSON must have error field");
    assert!(
        error_val.is_string() && !error_val.as_str().unwrap().is_empty(),
        "error must be non-empty string. Full JSON: {json}"
    );

    // Must NOT have "query" field (watch/diff don't use query)
    assert!(
        json.get("query").is_none(),
        "watch JSON error must NOT have 'query' field. Full JSON: {json}"
    );
}

// ---------------------------------------------------------------------------
// BDD Scenario: `diff --json` JSON error payload schema is exact (no extraneous fields)
// ---------------------------------------------------------------------------

/// Given: `ctd diff --json` fails with any error (e.g., manifest not found)
/// When: Command fails with JSON mode requested
/// Then: JSON payload contains exactly 3 fields:
///   - `status`: string value `"error"` exactly
///   - `error`: non-empty string
///   - `command`: string value `"diff"` exactly
/// And NO other fields exist (e.g., no `query` field)
#[test]
fn diff_json_error_payload_schema_is_exact_no_extra_fields() {
    let result = run_cli(&["diff", "/nope-a", "/nope-b", "--json"]);

    let stdout = String::from_utf8_lossy(&result.stdout);

    // stdout must be valid JSON
    let json: serde_json::Value = serde_json::from_str(&stdout).expect(
        format!("diff --json error output must be valid JSON. stdout: {stdout}").as_str(),
    );

    // Must have exactly 3 fields (no extras like "query")
    let field_count = json.as_object().expect("JSON must be an object").len();
    assert_eq!(
        field_count, 3,
        "diff JSON error must have exactly 3 fields, got {field_count}. Fields: {json}"
    );

    // Must have "status": "error"
    assert_eq!(
        json.get("status").and_then(|v| v.as_str()),
        Some("error"),
        "status must be 'error'. Full JSON: {json}"
    );

    // Must have "command": "diff"
    assert_eq!(
        json.get("command").and_then(|v| v.as_str()),
        Some("diff"),
        "command must be 'diff'. Full JSON: {json}"
    );

    // Must have non-empty "error" string
    let error_val = json.get("error").expect("JSON must have error field");
    assert!(
        error_val.is_string() && !error_val.as_str().unwrap().is_empty(),
        "error must be non-empty string. Full JSON: {json}"
    );

    // Must NOT have "query" field (watch/diff don't use query)
    assert!(
        json.get("query").is_none(),
        "diff JSON error must NOT have 'query' field. Full JSON: {json}"
    );
}

// ---------------------------------------------------------------------------
// BDD Scenario: Exit code parity — `watch --json` and `watch` produce same exit code
// ---------------------------------------------------------------------------

/// Given: `127.0.0.1:9` (connection refused — pipeline error)
/// When: User runs `ctd watch http://127.0.0.1:9 --output <dir> --cache <db> --json`
///        AND `ctd watch http://127.0.0.1:9 --output <dir> --cache <db>` (without `--json`)
/// Then: Both exit with code 2. JSON mode additionally emits JSON error to stdout;
///       non-JSON mode emits plain-text error to stderr.
#[test]
fn watch_json_and_no_json_exit_code_parity_network_error() {
    let temp = TempDir::new().unwrap();
    let output_dir = temp.path().join("output");
    let cache_dir = temp.path().join("cache");
    fs::create_dir_all(&output_dir).unwrap();
    fs::create_dir_all(&cache_dir).unwrap();

    let cache_path = cache_dir.join("test.redb");

    // Run with --json
    let json_result = run_cli(&[
        "watch",
        "http://127.0.0.1:9",
        "--output",
        output_dir.to_str().unwrap(),
        "--cache",
        cache_path.to_str().unwrap(),
        "--json",
    ]);

    // Run without --json
    let no_json_result = run_cli(&[
        "watch",
        "http://127.0.0.1:9",
        "--output",
        output_dir.to_str().unwrap(),
        "--cache",
        cache_path.to_str().unwrap(),
    ]);

    // Both must exit with code 2 (parity requirement)
    assert_eq!(
        json_result.status.code(),
        Some(2),
        "watch --json with connection refused must exit 2"
    );
    assert_eq!(
        no_json_result.status.code(),
        Some(2),
        "watch without --json with connection refused must exit 2"
    );

    // JSON mode emits JSON to stdout (not plain text to stderr)
    let json_stdout = String::from_utf8_lossy(&json_result.stdout);
    let json_stderr = String::from_utf8_lossy(&json_result.stderr);
    let _: serde_json::Value = serde_json::from_str(&json_stdout).expect(
        format!("watch --json should emit valid JSON to stdout. stdout: {json_stdout}").as_str(),
    );
    assert!(
        !json_stderr.contains("Error:"),
        "watch --json should NOT emit Error: to stderr. stderr: {json_stderr}"
    );

    // Non-JSON mode emits plain text to stderr
    let no_json_stderr = String::from_utf8_lossy(&no_json_result.stderr);
    assert!(
        no_json_stderr.contains("Error:"),
        "watch without --json should emit Error: to stderr. stderr: {no_json_stderr}"
    );
}

// ---------------------------------------------------------------------------
// BDD Scenario: Exit code parity — `diff --json` and `diff` produce same exit code
// ---------------------------------------------------------------------------

/// Given: Directory `/nope-a` does not exist (user input error)
/// When: User runs `ctd diff /nope-a /nope-b --json`
///        AND `ctd diff /nope-a /nope-b` (without `--json`)
/// Then: Both exit with code 1. JSON mode additionally emits JSON error to stdout;
///       non-JSON mode emits plain-text error to stderr.
#[test]
fn diff_json_and_no_json_exit_code_parity_manifest_not_found() {
    // Run with --json
    let json_result = run_cli(&["diff", "/nope-a", "/nope-b", "--json"]);

    // Run without --json
    let no_json_result = run_cli(&["diff", "/nope-a", "/nope-b"]);

    // Both must exit with code 1 (parity requirement)
    assert_eq!(
        json_result.status.code(),
        Some(1),
        "diff --json with missing dirs must exit 1"
    );
    assert_eq!(
        no_json_result.status.code(),
        Some(1),
        "diff without --json with missing dirs must exit 1"
    );

    // JSON mode emits JSON to stdout (not plain text to stderr)
    let json_stdout = String::from_utf8_lossy(&json_result.stdout);
    let json_stderr = String::from_utf8_lossy(&json_result.stderr);
    let _: serde_json::Value = serde_json::from_str(&json_stdout).expect(
        format!("diff --json should emit valid JSON to stdout. stdout: {json_stdout}").as_str(),
    );
    assert!(
        !json_stderr.contains("Error:"),
        "diff --json should NOT emit Error: to stderr. stderr: {json_stderr}"
    );

    // Non-JSON mode emits plain text to stderr
    let no_json_stderr = String::from_utf8_lossy(&no_json_result.stderr);
    assert!(
        no_json_stderr.contains("Error:"),
        "diff without --json should emit Error: to stderr. stderr: {no_json_stderr}"
    );
}

// ---------------------------------------------------------------------------
// Reference Test: Search JSON error still works (unchanged behavior)
// ---------------------------------------------------------------------------

/// Given: Missing index directory
/// When: User runs `ctd search test --index-dir /nonexistent --json`
/// Then: JSON error with `"status": "error"`, `"query": "test"`, `"error":` to stdout, exit 1
///
/// This is a REFERENCE test to ensure search --json error behavior is NOT broken
/// by the watch/diff fix.
#[test]
fn search_json_error_still_works_reference() {
    let result = run_cli(&["search", "test", "--index-dir", "/nonexistent", "--json"]);

    let stdout = String::from_utf8_lossy(&result.stdout);
    let stderr = String::from_utf8_lossy(&result.stderr);

    // Exit code must be 1
    assert_eq!(
        result.status.code(),
        Some(1),
        "search --json with missing index should exit 1. stderr: {stderr}"
    );

    // stdout must be valid JSON
    let json: serde_json::Value = serde_json::from_str(&stdout).expect(
        format!("search --json error must be valid JSON. stdout: {stdout}").as_str(),
    );

    // Must have "status": "error"
    assert_eq!(
        json.get("status").and_then(|v| v.as_str()),
        Some("error"),
        "JSON error must have status='error'. Full JSON: {json}"
    );

    // Must have "query": "test" (search uses query field, not command)
    assert_eq!(
        json.get("query").and_then(|v| v.as_str()),
        Some("test"),
        "search JSON error must have query field. Full JSON: {json}"
    );

    // Must have non-empty "error" field
    let error_msg = json
        .get("error")
        .and_then(|v| v.as_str())
        .expect("JSON error must have error field");
    assert!(
        !error_msg.is_empty(),
        "JSON error message must be non-empty. Full JSON: {json}"
    );

    // Search JSON error has 3 fields: status, query, error
    let field_count = json.as_object().expect("JSON must be an object").len();
    assert_eq!(
        field_count, 3,
        "search JSON error must have exactly 3 fields. Full JSON: {json}"
    );
}

