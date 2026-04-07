//! Tests for error display and taxonomy.

#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use super::*;
use crate::state::bulk_load::BulkLoadError;
use crate::state::StateLoadError;

#[test]
fn scrape_reuse_error_state_load_displays_correctly() {
    let inner = StateLoadError::BackendError {
        operation: "open_table",
        message: "table missing".to_string(),
    };
    let error = ScrapeReuseError::StateLoad(inner);
    let msg = format!("{error}");
    assert!(
        msg.contains("failed to load url states"),
        "message should contain 'failed to load url states': {msg}"
    );
    assert!(
        msg.contains("table missing"),
        "message should contain 'table missing': {msg}"
    );
}

#[test]
fn scrape_reuse_error_bulk_load_displays_correctly() {
    let inner = BulkLoadError::TableOpen {
        table: "scrape_outputs",
        message: "not found".to_string(),
    };
    let error = ScrapeReuseError::BulkLoad(inner);
    let msg = format!("{error}");
    assert!(
        msg.contains("failed to load archived scrape outputs"),
        "message should contain 'failed to load archived scrape outputs': {msg}"
    );
    assert!(
        msg.contains("not found"),
        "message should contain 'not found': {msg}"
    );
}

#[test]
fn scrape_reuse_error_deserialization_failed_displays_key_hex_and_message() {
    let error = ScrapeReuseError::DeserializationFailed {
        key_hex: "deadbeef".to_string(),
        message: "invalid archive".to_string(),
    };
    let msg = format!("{error}");
    assert!(
        msg.contains("deadbeef"),
        "message should contain 'deadbeef': {msg}"
    );
    assert!(
        msg.contains("invalid archive"),
        "message should contain 'invalid archive': {msg}"
    );
}

#[test]
fn scrape_reuse_error_hash_mismatch_displays_url_and_hashes() {
    let error = ScrapeReuseError::HashMismatch {
        url: "https://a.com".to_string(),
        stored_hex: "aa".to_string(),
        loaded_hex: "bb".to_string(),
    };
    let msg = format!("{error}");
    assert!(
        msg.contains("https://a.com"),
        "message should contain 'https://a.com': {msg}"
    );
    assert!(msg.contains("aa"), "message should contain 'aa': {msg}");
    assert!(msg.contains("bb"), "message should contain 'bb': {msg}");
}

#[test]
fn scrape_reuse_error_missing_url_state_displays_url() {
    let error = ScrapeReuseError::MissingUrlState {
        url: "https://missing.com".to_string(),
    };
    let msg = format!("{error}");
    assert!(
        msg.contains("https://missing.com"),
        "message should contain 'https://missing.com': {msg}"
    );
    assert!(
        msg.contains("missing url_state"),
        "message should contain 'missing url_state': {msg}"
    );
}
