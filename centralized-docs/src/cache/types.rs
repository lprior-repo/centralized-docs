//! Cache domain types: ContentHash newtype, table definitions, CacheType selector, hash functions.

use crate::errors::CacheError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::path::Path;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Maximum allowed key size in bytes.
pub(crate) const MAX_KEY_SIZE: usize = 256;

/// Maximum allowed value size in bytes (50 MB — per-document chunks can be large).
pub(crate) const MAX_VALUE_SIZE: usize = 50 * 1024 * 1024;

// ---------------------------------------------------------------------------
// Table definitions
// ---------------------------------------------------------------------------

pub(crate) const DOCUMENT_TABLE: redb::TableDefinition<&[u8], &[u8]> =
    redb::TableDefinition::new("documents");
pub(crate) const SCRAPE_TABLE: redb::TableDefinition<&[u8], &[u8]> =
    redb::TableDefinition::new("scrape");
pub(crate) const TRANSFORM_TABLE: redb::TableDefinition<&[u8], &[u8]> =
    redb::TableDefinition::new("transforms");
pub(crate) const SNAPSHOTS_TABLE: redb::TableDefinition<&[u8], &[u8]> =
    redb::TableDefinition::new("snapshots");
pub(crate) const ANALYSIS_TABLE: redb::TableDefinition<&[u8], &[u8]> =
    redb::TableDefinition::new("analysis");
pub(crate) const CHUNK_TABLE: redb::TableDefinition<&[u8], &[u8]> =
    redb::TableDefinition::new("chunks");
pub(crate) const METADATA_TABLE: redb::TableDefinition<&str, &str> =
    redb::TableDefinition::new("metadata");

// ---------------------------------------------------------------------------
// ContentHash newtype
// ---------------------------------------------------------------------------

/// SHA-256 digest wrapped as a newtype. Prevents accidental `[u8; 32]` leakage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentHash([u8; 32]);

impl ContentHash {
    /// Compute SHA-256 hash of arbitrary bytes.
    #[must_use]
    pub fn compute(content: &[u8]) -> Self {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(content);
        let result = hasher.finalize();
        let mut array = [0u8; 32];
        array.copy_from_slice(&result);
        Self(array)
    }

    /// Return the raw SHA-256 bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl std::fmt::Display for ContentHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.0 {
            write!(f, "{byte:02X}")?;
        }
        Ok(())
    }
}

impl AsRef<[u8]> for ContentHash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; 32]> for ContentHash {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl From<ContentHash> for [u8; 32] {
    fn from(hash: ContentHash) -> Self {
        hash.0
    }
}

// ---------------------------------------------------------------------------
// Cache type selector and statistics
// ---------------------------------------------------------------------------

/// Cache type selector for typed get/put operations.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheType {
    Document,
    Scrape,
    Transform,
    Snapshot,
    Analysis,
    Chunk,
}

/// Cache statistics.
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub document_entries: u64,
    pub scrape_entries: u64,
    pub transform_entries: u64,
    pub snapshot_entries: u64,
    pub analysis_entries: u64,
    pub chunk_entries: u64,
}

// ---------------------------------------------------------------------------
// Pure hash functions
// ---------------------------------------------------------------------------

/// Compute SHA-256 content hash.
#[must_use]
pub fn content_hash(content: &[u8]) -> ContentHash {
    ContentHash::compute(content)
}

/// Compute SHA-256 hash of a URL string.
#[must_use]
pub fn url_hash(url: &str) -> ContentHash {
    ContentHash::compute(url.as_bytes())
}

/// Compute SHA-256 hash of a filesystem path.
#[must_use]
pub fn path_hash(path: &Path) -> ContentHash {
    ContentHash::compute(path.to_string_lossy().as_bytes())
}

/// Compute SHA-256 hash of multiple byte slices concatenated.
#[must_use]
pub fn composite_hash(parts: &[&[u8]]) -> ContentHash {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part);
    }
    let result = hasher.finalize();
    let mut array = [0u8; 32];
    array.copy_from_slice(&result);
    ContentHash(array)
}

// ---------------------------------------------------------------------------
// Internal helpers (visible to doc_cache submodule)
// ---------------------------------------------------------------------------

/// Validates key size against `MAX_KEY_SIZE`.
pub(crate) fn validate_key_size(key: &[u8]) -> Result<(), CacheError> {
    if key.len() > MAX_KEY_SIZE {
        return Err(CacheError::KeyTooLarge {
            size: key.len(),
            max: MAX_KEY_SIZE,
        });
    }
    Ok(())
}

/// Validates serialized value size against `MAX_VALUE_SIZE`.
pub(crate) fn validate_value_size(bytes: &[u8]) -> Result<(), CacheError> {
    if bytes.len() > MAX_VALUE_SIZE {
        return Err(CacheError::ValueTooLarge {
            size: bytes.len(),
            max: MAX_VALUE_SIZE,
        });
    }
    Ok(())
}

/// Maps `CacheType` to redb table definition.
pub(crate) const fn table_for_type(
    cache_type: CacheType,
) -> redb::TableDefinition<'static, &'static [u8], &'static [u8]> {
    match cache_type {
        CacheType::Document => DOCUMENT_TABLE,
        CacheType::Scrape => SCRAPE_TABLE,
        CacheType::Transform => TRANSFORM_TABLE,
        CacheType::Snapshot => SNAPSHOTS_TABLE,
        CacheType::Analysis => ANALYSIS_TABLE,
        CacheType::Chunk => CHUNK_TABLE,
    }
}

/// Reads a cached value from a redb table (I/O boundary).
pub(crate) fn read_cached<V: DeserializeOwned>(
    read_tx: &redb::ReadTransaction,
    table_def: redb::TableDefinition<&[u8], &[u8]>,
    key: &[u8],
) -> anyhow::Result<Option<V>> {
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
    let value: V = serde_json::from_slice(bytes)?;
    Ok(Some(value))
}

/// Writes a cached value with size validation (I/O boundary).
pub(crate) fn write_cached<V: Serialize>(
    write_tx: &mut redb::WriteTransaction,
    table_def: redb::TableDefinition<&[u8], &[u8]>,
    key: &[u8],
    value: &V,
) -> anyhow::Result<()> {
    validate_key_size(key)?;
    let bytes = serde_json::to_vec(value)?;
    validate_value_size(&bytes)?;
    let mut table = write_tx.open_table(table_def)?;
    table.insert(key, bytes.as_slice())?;
    Ok(())
}

/// Reads table entry count (I/O boundary).
pub(crate) fn table_len(
    read_tx: &redb::ReadTransaction,
    table_def: redb::TableDefinition<&[u8], &[u8]>,
) -> anyhow::Result<u64> {
    use redb::ReadableTableMetadata;
    let table = read_tx
        .open_table(table_def)
        .map_err(|e| CacheError::BackendError {
            operation: "open_table",
            message: e.to_string(),
        })?;
    Ok(table.len()?)
}
