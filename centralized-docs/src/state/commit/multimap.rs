//! Multimap operations for `source_path` -> `chunk_hash` reverse index.
//!
//! Manages the `source_path_chunks` multimap table that tracks which chunk
//! hashes belong to which source paths, enabling O(1) orphan cleanup.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![forbid(unsafe_code)]

use super::{
    super::{chunk_outputs_table, source_path_chunks_table, FileStateRaw},
    writes::open_table_for_write,
    CommitError, ZERO_HASH,
};
use redb::{ReadableMultimapTable, WriteTransaction};

/// Populate the `source_path_chunks` multimap for every upserted file
/// with a non-zero `chunk_hash`.
pub(crate) fn write_source_path_chunks(
    write_tx: &WriteTransaction,
    entries: &[(String, FileStateRaw)],
) -> Result<(), CommitError> {
    if entries.is_empty() {
        return Ok(());
    }

    let mut table = open_multimap_for_write(write_tx, "source_path_chunks")?;

    for (path, state) in entries {
        if state.chunk_hash != ZERO_HASH {
            table
                .insert(path.as_str(), state.chunk_hash.as_slice())
                .map_err(|e| CommitError::WriteFailed {
                    table: "source_path_chunks",
                    reason: e.to_string(),
                })?;
        }
    }

    Ok(())
}

/// Delete orphaned chunks and multimap entries for every deleted file.
pub(crate) fn delete_orphaned_chunks(
    write_tx: &WriteTransaction,
    deleted_paths: &[String],
) -> Result<(), CommitError> {
    if deleted_paths.is_empty() {
        return Ok(());
    }

    let mut multimap = open_multimap_for_write(write_tx, "source_path_chunks")?;
    let mut chunk_table = open_table_for_write(write_tx, chunk_outputs_table(), "chunk_outputs")?;

    for path in deleted_paths {
        let chunk_hashes: Vec<[u8; 32]> = {
            let guard = multimap
                .get(path.as_str())
                .map_err(|e: redb::StorageError| CommitError::WriteFailed {
                    table: "source_path_chunks",
                    reason: e.to_string(),
                })?;

            guard
                .filter_map(std::result::Result::ok)
                .filter_map(|entry| {
                    let v: &[u8] = entry.value();
                    <[u8; 32]>::try_from(v).ok()
                })
                .collect()
        };

        for hash in &chunk_hashes {
            chunk_table
                .remove(hash.as_slice())
                .map_err(|e: redb::StorageError| CommitError::WriteFailed {
                    table: "chunk_outputs",
                    reason: e.to_string(),
                })?;

            multimap
                .remove(path.as_str(), hash.as_slice())
                .map_err(|e| CommitError::WriteFailed {
                    table: "source_path_chunks",
                    reason: e.to_string(),
                })?;
        }
    }

    Ok(())
}

/// Open the `source_path_chunks` multimap table within a write transaction.
fn open_multimap_for_write<'a>(
    write_tx: &'a WriteTransaction,
    table_name: &'static str,
) -> Result<redb::MultimapTable<'a, &'static str, &'static [u8]>, CommitError> {
    write_tx
        .open_multimap_table(source_path_chunks_table())
        .map_err(|e: redb::TableError| CommitError::WriteFailed {
            table: table_name,
            reason: e.to_string(),
        })
}
