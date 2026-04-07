//! Write helpers for applying mutations within a redb write transaction.
//!
//! All functions take a `&WriteTransaction` and a slice of changes,
//! performing atomic writes to the appropriate redb tables.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![forbid(unsafe_code)]

use super::{
    super::{
        analysis_outputs_table, chunk_outputs_table, file_state_table, scrape_outputs_table,
        snapshots_table, transform_outputs_table, url_state_table, FileStateRaw, UrlStateRaw,
    },
    CommitError, StateChanges,
};
use redb::{ReadableTable, TableDefinition, WriteTransaction};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Top-level write orchestrator
// ---------------------------------------------------------------------------

/// Apply all mutations within the given write transaction.
pub(crate) fn apply_all_writes(
    write_tx: &WriteTransaction,
    changes: &StateChanges,
) -> Result<(), CommitError> {
    // Payload writes first (they're referenced by state entries)
    write_payload_entries(
        write_tx,
        &changes.new_analyses,
        analysis_outputs_table(),
        "analysis_outputs",
    )?;
    write_payload_entries(
        write_tx,
        &changes.new_transforms,
        transform_outputs_table(),
        "transform_outputs",
    )?;
    write_payload_entries(
        write_tx,
        &changes.new_chunks,
        chunk_outputs_table(),
        "chunk_outputs",
    )?;
    write_payload_entries(
        write_tx,
        &changes.new_scrapes,
        scrape_outputs_table(),
        "scrape_outputs",
    )?;
    write_payload_entries(
        write_tx,
        &changes.new_snapshots,
        snapshots_table(),
        "snapshots",
    )?;

    // State upserts
    write_file_states(write_tx, &changes.updated_files)?;
    write_url_states(write_tx, &changes.updated_urls)?;

    // Multimap: track source_path -> chunk_hash for upserted files
    super::multimap::write_source_path_chunks(write_tx, &changes.updated_files)?;

    // Deletes: orphaned chunks must be cleaned BEFORE deleting file_state rows
    super::multimap::delete_orphaned_chunks(write_tx, &changes.deleted_files)?;
    delete_entries(
        write_tx,
        &changes.deleted_files,
        file_state_table(),
        "file_state",
    )?;
    delete_entries(
        write_tx,
        &changes.deleted_urls,
        url_state_table(),
        "url_state",
    )?;
    delete_snapshot_entries(write_tx, &changes.deleted_snapshots)?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Payload writes
// ---------------------------------------------------------------------------

/// Write deduplicated payload entries to a hash-keyed table.
/// Last-write-wins semantics for duplicate keys within the batch.
fn write_payload_entries(
    write_tx: &WriteTransaction,
    entries: &[([u8; 32], Vec<u8>)],
    table_def: TableDefinition<&[u8], &[u8]>,
    table_name: &'static str,
) -> Result<(), CommitError> {
    if entries.is_empty() {
        return Ok(());
    }

    // Dedup: last-write-wins (HashMap::insert overwrites)
    let deduped: HashMap<[u8; 32], &[u8]> = entries
        .iter()
        .map(|(hash, value)| (*hash, value.as_slice()))
        .collect();

    let mut table = open_table_for_write(write_tx, table_def, table_name)?;

    for (hash, new_value) in &deduped {
        let skip = read_and_compare(&table, hash.as_slice(), new_value, table_name)?;
        if !skip {
            table
                .insert(hash.as_slice(), *new_value)
                .map_err(|e| CommitError::WriteFailed {
                    table: table_name,
                    reason: e.to_string(),
                })?;
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// State table writes
// ---------------------------------------------------------------------------

/// Write file state entries to the string-keyed `file_state` table.
fn write_file_states(
    write_tx: &WriteTransaction,
    entries: &[(String, FileStateRaw)],
) -> Result<(), CommitError> {
    if entries.is_empty() {
        return Ok(());
    }

    let mut table = open_table_for_write(write_tx, file_state_table(), "file_state")?;

    for (path, state) in entries {
        let new_bytes = state.to_bytes();
        let skip = read_and_compare(&table, path.as_str(), &new_bytes, "file_state")?;
        if !skip {
            table
                .insert(path.as_str(), new_bytes.as_slice())
                .map_err(|e| CommitError::WriteFailed {
                    table: "file_state",
                    reason: e.to_string(),
                })?;
        }
    }

    Ok(())
}

/// Write URL state entries to the string-keyed `url_state` table.
fn write_url_states(
    write_tx: &WriteTransaction,
    entries: &[(String, UrlStateRaw)],
) -> Result<(), CommitError> {
    if entries.is_empty() {
        return Ok(());
    }

    let mut table = open_table_for_write(write_tx, url_state_table(), "url_state")?;

    for (url, state) in entries {
        let new_bytes = state.to_bytes();
        let skip = read_and_compare(&table, url.as_str(), &new_bytes, "url_state")?;
        if !skip {
            table
                .insert(url.as_str(), new_bytes.as_slice())
                .map_err(|e| CommitError::WriteFailed {
                    table: "url_state",
                    reason: e.to_string(),
                })?;
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Delete helpers
// ---------------------------------------------------------------------------

/// Delete string-keyed entries from a state table. Silently skips non-existent keys.
fn delete_entries(
    write_tx: &WriteTransaction,
    keys: &[String],
    table_def: TableDefinition<&str, &[u8]>,
    table_name: &'static str,
) -> Result<(), CommitError> {
    if keys.is_empty() {
        return Ok(());
    }

    let mut table = open_table_for_write(write_tx, table_def, table_name)?;

    for key in keys {
        // Fix 3: Remove unnecessary `let _ =` before `?` expression
        table
            .remove(key.as_str())
            .map_err(|e: redb::StorageError| CommitError::WriteFailed {
                table: table_name,
                reason: e.to_string(),
            })?;
    }

    Ok(())
}

/// Delete hash-keyed snapshot entries. Silently skips non-existent keys.
fn delete_snapshot_entries(
    write_tx: &WriteTransaction,
    hashes: &[[u8; 32]],
) -> Result<(), CommitError> {
    if hashes.is_empty() {
        return Ok(());
    }

    let mut table = open_table_for_write(write_tx, snapshots_table(), "snapshots")?;

    for hash in hashes {
        // Fix 3: Remove unnecessary `let _ =` before `?` expression
        table
            .remove(hash.as_slice())
            .map_err(|e: redb::StorageError| CommitError::WriteFailed {
                table: "snapshots",
                reason: e.to_string(),
            })?;
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Shared table access helpers (used by this module and multimap)
// ---------------------------------------------------------------------------

/// Open a table within a write transaction.
pub(crate) fn open_table_for_write<'a, K: redb::Key + 'static, V: redb::Value + 'static>(
    write_tx: &'a WriteTransaction,
    table_def: TableDefinition<'a, K, V>,
    table_name: &'static str,
) -> Result<redb::Table<'a, K, V>, CommitError> {
    write_tx
        .open_table(table_def)
        .map_err(|e: redb::TableError| CommitError::WriteFailed {
            table: table_name,
            reason: e.to_string(),
        })
}

/// Read existing value and compare with new value. Returns `true` if write should be skipped.
pub(crate) fn read_and_compare<K: redb::Key>(
    table: &redb::Table<'_, K, &'static [u8]>,
    key: K::SelfType<'_>,
    new_value: &[u8],
    table_name: &'static str,
) -> Result<bool, CommitError> {
    let existing = table
        .get(key)
        .map_err(|e: redb::StorageError| CommitError::WriteFailed {
            table: table_name,
            reason: e.to_string(),
        })?;

    Ok(existing.is_some_and(|guard| super::should_skip_write(guard.value(), new_value)))
}
