//! Shared test helpers for bulk_load tests.

#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use super::*;
use crate::state::{
    file_state_table, initialize_tables, url_state_table, FileStateRaw, StateLoadError, UrlStateRaw,
};
use redb::{Database, ReadableTableMetadata};
use rstest::rstest;
use std::collections::HashMap;
use tempfile::TempDir;

pub(super) mod boundary_debug_tests;
pub(super) mod error_variant_tests;
pub(super) mod hex_encode_tests;
pub(super) mod load_file_state_tests;
pub(super) mod load_url_state_tests;
pub(super) mod owned_archive_construction_tests;
pub(super) mod owned_archive_roundtrip_tests;
pub(super) mod session_cardinality_tests;

// =======================================================================
// Helpers
// =======================================================================

/// Open a fresh database with initialized tables.
pub(super) fn fresh_db() -> (TempDir, Database) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.redb");
    let db = Database::create(&db_path).unwrap();
    initialize_tables(&db).unwrap();
    (temp_dir, db)
}

/// Create a valid `FileStateRaw` with a unique content hash derived from `seed`.
pub(super) fn file_state(seed: u8) -> FileStateRaw {
    FileStateRaw {
        content_hash: [seed; 32],
        config_hash: [seed.saturating_add(1); 32],
        analysis_hash: [seed.saturating_add(2); 32],
        transform_hash: [seed.saturating_add(3); 32],
        chunk_hash: [seed.saturating_add(4); 32],
        last_processed_secs: u64::from(seed).saturating_mul(1_000_000),
        reserved: [0u8; 32],
    }
}

/// Create a valid `UrlStateRaw` with a unique content hash derived from `seed`.
pub(super) fn url_state(seed: u8) -> UrlStateRaw {
    UrlStateRaw {
        content_hash: [seed; 32],
        url_hash: [seed.saturating_add(1); 32],
        last_fetched_secs: u64::from(seed).saturating_mul(2_000_000),
        status_code: u16::from(seed),
        reserved: [0u8; 46],
    }
}

/// Write file-state rows to the database.
pub(super) fn write_file_rows(db: &Database, rows: &[(&str, FileStateRaw)]) {
    let write_tx = db.begin_write().unwrap();
    {
        let mut table = write_tx.open_table(file_state_table()).unwrap();
        for (key, state) in rows {
            table.insert(*key, state.to_bytes().as_slice()).unwrap();
        }
    }
    write_tx.commit().unwrap();
}

/// Write URL-state rows to the database.
pub(super) fn write_url_rows(db: &Database, rows: &[(&str, UrlStateRaw)]) {
    let write_tx = db.begin_write().unwrap();
    {
        let mut table = write_tx.open_table(url_state_table()).unwrap();
        for (key, state) in rows {
            table.insert(*key, state.to_bytes().as_slice()).unwrap();
        }
    }
    write_tx.commit().unwrap();
}

/// Write a raw (potentially malformed) byte value to the `file_state` table.
pub(super) fn write_raw_file_row(db: &Database, key: &str, value: &[u8]) {
    let write_tx = db.begin_write().unwrap();
    {
        let mut table = write_tx.open_table(file_state_table()).unwrap();
        table.insert(key, value).unwrap();
    }
    write_tx.commit().unwrap();
}

/// Write a raw (potentially malformed) byte value to the `url_state` table.
pub(super) fn write_raw_url_row(db: &Database, key: &str, value: &[u8]) {
    let write_tx = db.begin_write().unwrap();
    {
        let mut table = write_tx.open_table(url_state_table()).unwrap();
        table.insert(key, value).unwrap();
    }
    write_tx.commit().unwrap();
}
