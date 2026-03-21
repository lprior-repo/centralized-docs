//! Pure hash and helper functions for cache operations.
//!
//! Provides content-addressed hashing (`content_hash`, `url_hash`, `path_hash`)
//! and low-level redb table operations (`get_cached_value`, `put_cached_value_with_limit`,
//! `validate_key_size`, `validate_value_size`, `table_len`).

use crate::errors::CacheError;
use anyhow::Result;
use redb::{ReadTransaction, ReadableTableMetadata, Table, TableDefinition};
use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;

use super::config::{MAX_KEY_SIZE, MAX_VALUE_SIZE};

pub(crate) fn get_cached_value<V: DeserializeOwned>(
    read_tx: &ReadTransaction,
    table_def: TableDefinition<&[u8], &[u8]>,
    key: &[u8],
) -> Result<Option<V>> {
    let table = read_tx
        .open_table(table_def)
        .map_err(|e| CacheError::BackendError {
            operation: "open_table",
            message: e.to_string(),
        })?;

    let Some(access_guard) = table.get(key)? else {
        return Ok(None);
    };

    let bytes = access_guard.value();
    Ok(bincode::deserialize::<V>(bytes).ok())
}

/// Validates key size against the maximum allowed limit.
/// Pure calculation - no side effects.
pub(crate) fn validate_key_size(key: &[u8]) -> Result<(), CacheError> {
    let len = key.len();
    if len == 0 || len > MAX_KEY_SIZE {
        return Err(CacheError::KeyTooLarge {
            size: len,
            max: MAX_KEY_SIZE,
        });
    }
    Ok(())
}

/// Validates serialized value size against the maximum allowed limit.
/// Pure calculation - no side effects.
pub(crate) fn validate_value_size(bytes: &[u8]) -> Result<(), CacheError> {
    if bytes.len() > MAX_VALUE_SIZE {
        return Err(CacheError::ValueTooLarge {
            size: bytes.len(),
            max: MAX_VALUE_SIZE,
        });
    }
    Ok(())
}

/// Validates value size and inserts into the table — single combined path
/// used by both `put_cached_value_with_limit` and `put_raw` to avoid
/// dual serialization/validation divergence (DEFECT-006).
pub(crate) fn validate_and_insert(
    table: &mut Table<&[u8], &[u8]>,
    key: &[u8],
    bytes: &[u8],
) -> Result<()> {
    validate_value_size(bytes)?;
    table.insert(key, bytes)?;
    Ok(())
}

/// Stores a cached value with size limit validation.
pub(crate) fn put_cached_value_with_limit<V: Serialize>(
    table: &mut Table<&[u8], &[u8]>,
    key: &[u8],
    value: &V,
) -> Result<()> {
    let bytes = bincode::serialize(value).map_err(|e| anyhow::anyhow!("bincode serialize: {e}"))?;
    validate_and_insert(table, key, &bytes)
}

pub(crate) fn table_len(
    read_tx: &ReadTransaction,
    table_def: TableDefinition<&[u8], &[u8]>,
) -> Result<u64> {
    let table = read_tx
        .open_table(table_def)
        .map_err(|e| CacheError::BackendError {
            operation: "open_table",
            message: e.to_string(),
        })?;
    Ok(table.len()?)
}

#[must_use]
pub fn content_hash(content: &[u8]) -> u128 {
    xxhash_rust::xxh3::xxh3_128(content)
}

#[must_use]
pub fn url_hash(url: &str) -> u128 {
    content_hash(url.as_bytes())
}

#[must_use]
#[cfg(unix)]
pub fn path_hash(path: &Path) -> u128 {
    use std::os::unix::ffi::OsStrExt;
    content_hash(path.as_os_str().as_bytes())
}
