//! Database initialization.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

use super::error::StateError;
use super::tables::{
    analysis_outputs_table, chunk_outputs_table, file_state_table, metadata_table,
    scrape_outputs_table, snapshots_table, source_path_chunks_table, transform_outputs_table,
    url_state_table, TABLE_NAME_ANALYSIS_OUTPUTS, TABLE_NAME_CHUNK_OUTPUTS, TABLE_NAME_FILE_STATE,
    TABLE_NAME_METADATA, TABLE_NAME_SCRAPE_OUTPUTS, TABLE_NAME_SNAPSHOTS,
    TABLE_NAME_SOURCE_PATH_CHUNKS, TABLE_NAME_TRANSFORM_OUTPUTS, TABLE_NAME_URL_STATE,
};
use redb::Database;

/// Create all 9 tables in a single write transaction.
///
/// Idempotent: redb's `open_table`/`open_multimap_table` creates if absent,
/// succeeds silently if present.
///
/// # Errors
///
/// - [`StateError::WriteTransactionFailed`] if `db.begin_write()` fails.
/// - [`StateError::TableOpenFailed`] if any table creation fails.
/// - [`StateError::CommitFailed`] if the write transaction commit fails.
pub fn initialize_tables(db: &Database) -> Result<(), StateError> {
    let write_tx = db
        .begin_write()
        .map_err(|e| StateError::WriteTransactionFailed {
            message: e.to_string(),
        })?;

    {
        let _ =
            write_tx
                .open_table(file_state_table())
                .map_err(|e| StateError::TableOpenFailed {
                    table: TABLE_NAME_FILE_STATE,
                    message: e.to_string(),
                })?;
        let _ =
            write_tx
                .open_table(url_state_table())
                .map_err(|e| StateError::TableOpenFailed {
                    table: TABLE_NAME_URL_STATE,
                    message: e.to_string(),
                })?;
        let _ = write_tx.open_table(analysis_outputs_table()).map_err(|e| {
            StateError::TableOpenFailed {
                table: TABLE_NAME_ANALYSIS_OUTPUTS,
                message: e.to_string(),
            }
        })?;
        let _ = write_tx
            .open_table(transform_outputs_table())
            .map_err(|e| StateError::TableOpenFailed {
                table: TABLE_NAME_TRANSFORM_OUTPUTS,
                message: e.to_string(),
            })?;
        let _ = write_tx.open_table(chunk_outputs_table()).map_err(|e| {
            StateError::TableOpenFailed {
                table: TABLE_NAME_CHUNK_OUTPUTS,
                message: e.to_string(),
            }
        })?;
        let _ = write_tx.open_table(scrape_outputs_table()).map_err(|e| {
            StateError::TableOpenFailed {
                table: TABLE_NAME_SCRAPE_OUTPUTS,
                message: e.to_string(),
            }
        })?;
        let _ =
            write_tx
                .open_table(snapshots_table())
                .map_err(|e| StateError::TableOpenFailed {
                    table: TABLE_NAME_SNAPSHOTS,
                    message: e.to_string(),
                })?;
        let _ = write_tx
            .open_table(metadata_table())
            .map_err(|e| StateError::TableOpenFailed {
                table: TABLE_NAME_METADATA,
                message: e.to_string(),
            })?;
        let _ = write_tx
            .open_multimap_table(source_path_chunks_table())
            .map_err(|e| StateError::TableOpenFailed {
                table: TABLE_NAME_SOURCE_PATH_CHUNKS,
                message: e.to_string(),
            })?;
    }

    write_tx.commit().map_err(|e| StateError::CommitFailed {
        message: e.to_string(),
    })?;

    Ok(())
}
