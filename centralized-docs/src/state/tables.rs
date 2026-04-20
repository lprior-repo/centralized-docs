//! Table definitions, accessor functions, and key validation.
//!
//! All 9 redb table definitions + accessor functions + key validation helpers.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

use super::error::StateError;
use redb::{MultimapTableDefinition, TableDefinition};

// ---------------------------------------------------------------------------
// Table name constants
// ---------------------------------------------------------------------------

pub const TABLE_NAME_FILE_STATE: &str = "file_state";
pub const TABLE_NAME_URL_STATE: &str = "url_state";
pub const TABLE_NAME_ANALYSIS_OUTPUTS: &str = "analysis_outputs";
pub const TABLE_NAME_TRANSFORM_OUTPUTS: &str = "transform_outputs";
pub const TABLE_NAME_CHUNK_OUTPUTS: &str = "chunk_outputs";
pub const TABLE_NAME_SCRAPE_OUTPUTS: &str = "scrape_outputs";
pub const TABLE_NAME_SNAPSHOTS: &str = "snapshots";
pub const TABLE_NAME_METADATA: &str = "metadata";
pub const TABLE_NAME_SOURCE_PATH_CHUNKS: &str = "source_path_chunks";

// ---------------------------------------------------------------------------
// Table definitions (compile-time constants)
// ---------------------------------------------------------------------------

const FILE_STATE_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new(TABLE_NAME_FILE_STATE);
const URL_STATE_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new(TABLE_NAME_URL_STATE);
const ANALYSIS_OUTPUTS_TABLE: TableDefinition<&[u8], &[u8]> =
    TableDefinition::new(TABLE_NAME_ANALYSIS_OUTPUTS);
const TRANSFORM_OUTPUTS_TABLE: TableDefinition<&[u8], &[u8]> =
    TableDefinition::new(TABLE_NAME_TRANSFORM_OUTPUTS);
const CHUNK_OUTPUTS_TABLE: TableDefinition<&[u8], &[u8]> =
    TableDefinition::new(TABLE_NAME_CHUNK_OUTPUTS);
const SCRAPE_OUTPUTS_TABLE: TableDefinition<&[u8], &[u8]> =
    TableDefinition::new(TABLE_NAME_SCRAPE_OUTPUTS);
const SNAPSHOTS_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new(TABLE_NAME_SNAPSHOTS);
const METADATA_TABLE: TableDefinition<&str, &str> = TableDefinition::new(TABLE_NAME_METADATA);
const SOURCE_PATH_CHUNKS_TABLE: MultimapTableDefinition<&str, &[u8]> =
    MultimapTableDefinition::new(TABLE_NAME_SOURCE_PATH_CHUNKS);

// ---------------------------------------------------------------------------
// Table accessor functions
// ---------------------------------------------------------------------------

#[must_use]
pub const fn file_state_table() -> TableDefinition<'static, &'static str, &'static [u8]> {
    FILE_STATE_TABLE
}

#[must_use]
pub const fn url_state_table() -> TableDefinition<'static, &'static str, &'static [u8]> {
    URL_STATE_TABLE
}

#[must_use]
pub const fn analysis_outputs_table() -> TableDefinition<'static, &'static [u8], &'static [u8]> {
    ANALYSIS_OUTPUTS_TABLE
}

#[must_use]
pub const fn transform_outputs_table() -> TableDefinition<'static, &'static [u8], &'static [u8]> {
    TRANSFORM_OUTPUTS_TABLE
}

#[must_use]
pub const fn chunk_outputs_table() -> TableDefinition<'static, &'static [u8], &'static [u8]> {
    CHUNK_OUTPUTS_TABLE
}

#[must_use]
pub const fn scrape_outputs_table() -> TableDefinition<'static, &'static [u8], &'static [u8]> {
    SCRAPE_OUTPUTS_TABLE
}

#[must_use]
pub const fn snapshots_table() -> TableDefinition<'static, &'static [u8], &'static [u8]> {
    SNAPSHOTS_TABLE
}

#[must_use]
pub const fn metadata_table() -> TableDefinition<'static, &'static str, &'static str> {
    METADATA_TABLE
}

#[must_use]
pub const fn source_path_chunks_table(
) -> MultimapTableDefinition<'static, &'static str, &'static [u8]> {
    SOURCE_PATH_CHUNKS_TABLE
}

// ---------------------------------------------------------------------------
// Key validation
// ---------------------------------------------------------------------------

/// Validate that a hash key is exactly 32 bytes.
///
/// # Errors
///
/// Returns an error when `key` is not exactly 32 bytes.
pub fn validate_hash_key(key: &[u8]) -> Result<(), StateError> {
    if key.len() != 32 {
        return Err(StateError::InvalidHashKeyLength { actual: key.len() });
    }
    Ok(())
}

/// Validate that a source path is relative and normalized.
///
/// # Errors
///
/// Returns an error when `path` is empty, absolute, or contains parent traversal.
pub fn validate_source_path(path: &str) -> Result<(), StateError> {
    if path.is_empty() {
        return Err(StateError::InvalidSourcePath {
            reason: "source path must not be empty".to_string(),
        });
    }
    if path.as_bytes().first() == Some(&b'/') {
        return Err(StateError::InvalidSourcePath {
            reason: "source path must not start with '/' (must be relative)".to_string(),
        });
    }
    if path.split('/').any(|c| c == "..") {
        return Err(StateError::InvalidSourcePath {
            reason: "source path must not contain '..' components".to_string(),
        });
    }
    Ok(())
}

/// Validate that a URL key has a scheme (contains `"://"`).
///
/// # Errors
///
/// Returns an error when `url` is empty or lacks a URL scheme.
pub fn validate_url_key(url: &str) -> Result<(), StateError> {
    if url.is_empty() {
        return Err(StateError::InvalidUrlKey {
            reason: "URL key must not be empty".to_string(),
        });
    }
    if !url.contains("://") {
        return Err(StateError::InvalidUrlKey {
            reason: "URL key must contain a scheme (e.g. \"https://\")".to_string(),
        });
    }
    Ok(())
}
