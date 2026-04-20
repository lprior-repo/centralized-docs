//! Shared test helpers and submodule declarations.

#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::redundant_closure_for_method_calls)]

mod basic_open;
mod builder_durability;
mod compact;
mod integration;
mod multimap;
mod paranoid;
mod proptests;
mod validation;
mod writes;
mod writes_extra;

use crate::state::{
    analysis_outputs_table, chunk_outputs_table, file_state_table, scrape_outputs_table,
    snapshots_table, source_path_chunks_table, transform_outputs_table, url_state_table,
    FileStateRaw, UrlStateRaw,
};
use redb::{Database, TableDefinition};
use tempfile::TempDir;

use super::{CommitError, StateChanges, StateDb, StateDbBuilder};

pub(super) fn create_temp_state_db() -> (StateDb, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("state.redb");
    let state_db = StateDb::open(&db_path).unwrap();
    (state_db, temp_dir)
}

pub(super) fn make_file_state_raw(
    analysis: [u8; 32],
    transform: [u8; 32],
    chunk: [u8; 32],
) -> FileStateRaw {
    FileStateRaw {
        content_hash: [0xAA; 32],
        config_hash: [0xBB; 32],
        analysis_hash: analysis,
        transform_hash: transform,
        chunk_hash: chunk,
        last_processed_secs: 12345,
        reserved: [0u8; 32],
    }
}

pub(super) fn make_url_state_raw(url_hash: [u8; 32]) -> UrlStateRaw {
    UrlStateRaw {
        content_hash: [0xCC; 32],
        url_hash,
        last_fetched_secs: 67890,
        status_code: 200,
        reserved: [0u8; 46],
    }
}

pub(super) fn make_minimal_valid_state_changes() -> StateChanges {
    StateChanges::empty()
}

pub(super) fn read_hash_table(
    db: &Database,
    table_def: TableDefinition<&[u8], &[u8]>,
    key: &[u8; 32],
) -> Option<Vec<u8>> {
    let read_tx = db.begin_read().unwrap();
    let table = read_tx.open_table(table_def).unwrap();
    table
        .get(key.as_slice())
        .unwrap()
        .map(|g| g.value().to_vec())
}

pub(super) fn read_string_table(
    db: &Database,
    table_def: TableDefinition<&str, &[u8]>,
    key: &str,
) -> Option<Vec<u8>> {
    let read_tx = db.begin_read().unwrap();
    let table = read_tx.open_table(table_def).unwrap();
    table.get(key).unwrap().map(|g| g.value().to_vec())
}

pub(super) fn read_multimap_entries(db: &Database, source_path: &str) -> Vec<[u8; 32]> {
    let read_tx = db.begin_read().unwrap();
    let table = read_tx
        .open_multimap_table(source_path_chunks_table())
        .unwrap();
    table
        .get(source_path)
        .unwrap()
        .filter_map(std::result::Result::ok)
        .filter_map(|entry| <[u8; 32]>::try_from(entry.value()).ok())
        .collect()
}

pub(super) fn count_table_entries(db: &Database, table_name: &str) -> u64 {
    use redb::ReadableTableMetadata;
    let read_tx = db.begin_read().unwrap();
    match table_name {
        "file_state" => read_tx
            .open_table(file_state_table())
            .unwrap()
            .len()
            .unwrap(),
        "url_state" => read_tx
            .open_table(url_state_table())
            .unwrap()
            .len()
            .unwrap(),
        "analysis_outputs" => read_tx
            .open_table(analysis_outputs_table())
            .unwrap()
            .len()
            .unwrap(),
        "transform_outputs" => read_tx
            .open_table(transform_outputs_table())
            .unwrap()
            .len()
            .unwrap(),
        "chunk_outputs" => read_tx
            .open_table(chunk_outputs_table())
            .unwrap()
            .len()
            .unwrap(),
        "scrape_outputs" => read_tx
            .open_table(scrape_outputs_table())
            .unwrap()
            .len()
            .unwrap(),
        "snapshots" => read_tx
            .open_table(snapshots_table())
            .unwrap()
            .len()
            .unwrap(),
        _ => 0,
    }
}
