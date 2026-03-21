//! Idempotent cache layer using redb for high-performance ACID storage
//!
//! Provides caching for:
//! - Document content hashes (skip reprocessing unchanged files)
//! - Scraped URLs (avoid re-fetching)
//! - Transform results (idempotent pipeline runs)
//!
//! # Design Principles
//!
//! - **Idempotency**: Same input always produces same cached result
//! - **Content-addressed**: Cache keys are SHA-256 hashes of input
//! - **ACID guarantees**: redb provides transactional safety
//! - **Zero-panic**: All operations return Result, no unwrap/expect
//! - **Size limits**: Keys and values are bounded to prevent DoS attacks

use crate::errors::CacheError;
use anyhow::Result;
use redb::{Database, ReadTransaction, ReadableTableMetadata, TableDefinition};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::path::Path;

/// Maximum allowed key size in bytes (256 bytes).
/// Prevents memory exhaustion from oversized keys.
const MAX_KEY_SIZE: usize = 256;

/// Maximum allowed value size in bytes (10 MB).
/// Prevents memory exhaustion from oversized cached values.
const MAX_VALUE_SIZE: usize = 10 * 1024 * 1024;

const DOCUMENT_CACHE_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("documents");
const SCRAPE_CACHE_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("scrape");
const TRANSFORM_CACHE_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("transforms");
const METADATA_TABLE: TableDefinition<&str, &str> = TableDefinition::new("metadata");

/// Cache backend selection - eliminates magic strings for type safety.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum CacheBackend {
    /// In-memory cache using redb's InMemoryBackend.
    /// Data is lost when the process exits.
    Memory,
    /// Persistent file-based cache stored at the given path.
    File(std::path::PathBuf),
}

#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub backend: CacheBackend,
    pub cache_document_content: bool,
    pub cache_scrape_results: bool,
    pub cache_transforms: bool,
}

impl CacheConfig {
    #[must_use]
    pub fn new(db_path: &Path) -> Self {
        Self {
            backend: CacheBackend::File(db_path.to_path_buf()),
            cache_document_content: true,
            cache_scrape_results: true,
            cache_transforms: true,
        }
    }

    #[must_use]
    pub fn in_memory() -> Self {
        Self {
            backend: CacheBackend::Memory,
            cache_document_content: true,
            cache_scrape_results: true,
            cache_transforms: true,
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            backend: CacheBackend::File(std::path::PathBuf::from(".cache/ctd_cache.redb")),
            cache_document_content: true,
            cache_scrape_results: true,
            cache_transforms: true,
        }
    }
}

/// Thread-safe cache using redb's MVCC.
///
/// # Thread Safety
///
/// This type is `Send + Sync`. redb uses MVCC (Multi-Version Concurrency Control)
/// for concurrent read access, allowing multiple readers without blocking.
/// Write transactions are serialized at the database level.
///
/// All public methods take `&self` (not `&mut self`), enabling safe concurrent
/// access from multiple threads without external synchronization.
#[derive(Debug)]
pub struct DocCache {
    db: Database,
    config: CacheConfig,
}

impl DocCache {
    #[allow(clippy::result_large_err)]
    pub fn open(config: CacheConfig) -> Result<Self> {
        let db = match &config.backend {
            CacheBackend::Memory => {
                Database::builder().create_with_backend(redb::backends::InMemoryBackend::new())?
            }
            CacheBackend::File(path) => {
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                Database::create(path)?
            }
        };
        let cache = Self { db, config };
        cache.initialize_tables()?;
        Ok(cache)
    }

    fn initialize_tables(&self) -> Result<()> {
        let write_tx = self.db.begin_write()?;
        {
            let _ = write_tx.open_table(DOCUMENT_CACHE_TABLE)?;
            let _ = write_tx.open_table(SCRAPE_CACHE_TABLE)?;
            let _ = write_tx.open_table(TRANSFORM_CACHE_TABLE)?;
            let _ = write_tx.open_table(METADATA_TABLE)?;
        }
        write_tx.commit()?;
        Ok(())
    }

    pub fn get_document<V: DeserializeOwned>(&self, key: &[u8]) -> Result<Option<V>> {
        if !self.config.cache_document_content {
            return Ok(None);
        }
        let read_tx = self.db.begin_read()?;
        get_cached_value(&read_tx, DOCUMENT_CACHE_TABLE, key)
    }

    pub fn put_document<V: Serialize>(&self, key: &[u8], value: &V) -> Result<()> {
        if !self.config.cache_document_content {
            return Ok(());
        }
        validate_key_size(key)?;
        let write_tx = self.db.begin_write()?;
        {
            let mut table = write_tx.open_table(DOCUMENT_CACHE_TABLE)?;
            put_cached_value_with_limit(&mut table, key, value)?;
        }
        write_tx.commit()?;
        Ok(())
    }

    pub fn get_scrape<V: DeserializeOwned>(&self, url_hash: &[u8]) -> Result<Option<V>> {
        if !self.config.cache_scrape_results {
            return Ok(None);
        }
        let read_tx = self.db.begin_read()?;
        get_cached_value(&read_tx, SCRAPE_CACHE_TABLE, url_hash)
    }

    pub fn put_scrape<V: Serialize>(&self, url_hash: &[u8], value: &V) -> Result<()> {
        if !self.config.cache_scrape_results {
            return Ok(());
        }
        validate_key_size(url_hash)?;
        let write_tx = self.db.begin_write()?;
        {
            let mut table = write_tx.open_table(SCRAPE_CACHE_TABLE)?;
            put_cached_value_with_limit(&mut table, url_hash, value)?;
        }
        write_tx.commit()?;
        Ok(())
    }

    pub fn get_transform<V: DeserializeOwned>(&self, key: &[u8]) -> Result<Option<V>> {
        if !self.config.cache_transforms {
            return Ok(None);
        }
        let read_tx = self.db.begin_read()?;
        get_cached_value(&read_tx, TRANSFORM_CACHE_TABLE, key)
    }

    pub fn put_transform<V: Serialize>(&self, key: &[u8], value: &V) -> Result<()> {
        if !self.config.cache_transforms {
            return Ok(());
        }
        validate_key_size(key)?;
        let write_tx = self.db.begin_write()?;
        {
            let mut table = write_tx.open_table(TRANSFORM_CACHE_TABLE)?;
            put_cached_value_with_limit(&mut table, key, value)?;
        }
        write_tx.commit()?;
        Ok(())
    }

    pub fn clear_all(&self) -> Result<()> {
        let write_tx = self.db.begin_write()?;
        {
            write_tx.delete_table(DOCUMENT_CACHE_TABLE)?;
            write_tx.delete_table(SCRAPE_CACHE_TABLE)?;
            write_tx.delete_table(TRANSFORM_CACHE_TABLE)?;
            write_tx.delete_table(METADATA_TABLE)?;
        }
        write_tx.commit()?;
        self.initialize_tables()
    }

    pub fn stats(&self) -> Result<CacheStats> {
        let read_tx = self.db.begin_read()?;

        let doc_count = table_len(&read_tx, DOCUMENT_CACHE_TABLE)?;
        let scrape_count = table_len(&read_tx, SCRAPE_CACHE_TABLE)?;
        let transform_count = table_len(&read_tx, TRANSFORM_CACHE_TABLE)?;

        Ok(CacheStats {
            document_entries: doc_count,
            scrape_entries: scrape_count,
            transform_entries: transform_count,
        })
    }

    pub fn get_or_compute<V, F>(&self, cache_type: CacheType, key: &[u8], compute: F) -> Result<V>
    where
        V: Serialize + DeserializeOwned,
        F: FnOnce() -> Result<V>,
    {
        if let Some(cached) = self.get::<V>(cache_type, key)? {
            return Ok(cached);
        }

        let value = compute()?;
        self.put(cache_type, key, &value)?;
        Ok(value)
    }

    fn get<V: DeserializeOwned>(&self, cache_type: CacheType, key: &[u8]) -> Result<Option<V>> {
        match cache_type {
            CacheType::Document => self.get_document(key),
            CacheType::Scrape => self.get_scrape(key),
            CacheType::Transform => self.get_transform(key),
        }
    }

    fn put<V: Serialize>(&self, cache_type: CacheType, key: &[u8], value: &V) -> Result<()> {
        match cache_type {
            CacheType::Document => self.put_document(key, value),
            CacheType::Scrape => self.put_scrape(key, value),
            CacheType::Transform => self.put_transform(key, value),
        }
    }
}

/// Cache type selector for `get_or_compute` operations.
///
/// Marked `#[non_exhaustive]` to allow adding new cache types in future
/// versions without breaking changes.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheType {
    Document,
    Scrape,
    Transform,
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub document_entries: u64,
    pub scrape_entries: u64,
    pub transform_entries: u64,
}

fn get_cached_value<V: DeserializeOwned>(
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
    let value: V = serde_json::from_slice(bytes)?;
    Ok(Some(value))
}

/// Stores a cached value without size validation.
// TODO: Kept for future batch API that may need unchecked inserts
#[allow(dead_code)]
fn put_cached_value<V: Serialize>(
    table: &mut redb::Table<&[u8], &[u8]>,
    key: &[u8],
    value: &V,
) -> Result<()> {
    let bytes = serde_json::to_vec(value)?;
    table.insert(key, bytes.as_slice())?;
    Ok(())
}

/// Validates key size against the maximum allowed limit.
/// Pure calculation - no side effects.
fn validate_key_size(key: &[u8]) -> Result<(), CacheError> {
    if key.len() > MAX_KEY_SIZE {
        return Err(CacheError::KeyTooLarge {
            size: key.len(),
            max: MAX_KEY_SIZE,
        });
    }
    Ok(())
}

/// Validates serialized value size against the maximum allowed limit.
/// Pure calculation - no side effects.
fn validate_value_size(bytes: &[u8]) -> Result<(), CacheError> {
    if bytes.len() > MAX_VALUE_SIZE {
        return Err(CacheError::ValueTooLarge {
            size: bytes.len(),
            max: MAX_VALUE_SIZE,
        });
    }
    Ok(())
}

/// Stores a cached value with size limit validation.
fn put_cached_value_with_limit<V: Serialize>(
    table: &mut redb::Table<&[u8], &[u8]>,
    key: &[u8],
    value: &V,
) -> Result<()> {
    let bytes = serde_json::to_vec(value)?;
    validate_value_size(&bytes)?;
    table.insert(key, bytes.as_slice())?;
    Ok(())
}

fn table_len(read_tx: &ReadTransaction, table_def: TableDefinition<&[u8], &[u8]>) -> Result<u64> {
    let table = read_tx
        .open_table(table_def)
        .map_err(|e| CacheError::BackendError {
            operation: "open_table",
            message: e.to_string(),
        })?;
    Ok(table.len()?)
}

pub fn content_hash(content: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(content);
    let result = hasher.finalize();
    let mut array = [0u8; 32];
    array.copy_from_slice(&result);
    array
}

pub fn url_hash(url: &str) -> [u8; 32] {
    content_hash(url.as_bytes())
}

pub fn path_hash(path: &Path) -> [u8; 32] {
    content_hash(path.to_string_lossy().as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_cache_basic_roundtrip() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test.redb");
        let config = CacheConfig::new(&db_path);
        let cache = DocCache::open(config)?;

        let key = b"test_key";
        let value = "test_value".to_string();

        cache.put_document(key, &value)?;

        let retrieved: Option<String> = cache.get_document(key)?;
        assert_eq!(retrieved, Some(value));

        Ok(())
    }

    #[test]
    fn test_cache_miss_returns_none() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test.redb");
        let config = CacheConfig::new(&db_path);
        let cache = DocCache::open(config)?;

        let result: Option<String> = cache.get_document(b"nonexistent")?;
        assert!(result.is_none());

        Ok(())
    }

    #[test]
    fn test_cache_struct_value() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test.redb");
        let config = CacheConfig::new(&db_path);
        let cache = DocCache::open(config)?;

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        struct TestData {
            name: String,
            count: usize,
        }

        let key = b"struct_key";
        let value = TestData {
            name: "example".to_string(),
            count: 42,
        };

        cache.put_document(key, &value)?;

        let retrieved: Option<TestData> = cache.get_document(key)?;
        assert_eq!(retrieved, Some(value));

        Ok(())
    }

    #[test]
    fn test_cache_stats() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test.redb");
        let config = CacheConfig::new(&db_path);
        let cache = DocCache::open(config)?;

        cache.put_document(b"key1", &"value1")?;
        cache.put_document(b"key2", &"value2")?;
        cache.put_scrape(b"url1", &"scraped_data")?;

        let stats = cache.stats()?;
        assert_eq!(stats.document_entries, 2);
        assert_eq!(stats.scrape_entries, 1);
        assert_eq!(stats.transform_entries, 0);

        Ok(())
    }

    #[test]
    fn test_get_or_compute_caches_result() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test.redb");
        let config = CacheConfig::new(&db_path);
        let cache = DocCache::open(config)?;

        let key = b"compute_key";
        let mut call_count = 0;

        let result1 = cache.get_or_compute(CacheType::Document, key, || {
            call_count += 1;
            Ok("computed".to_string())
        })?;

        let result2 = cache.get_or_compute(CacheType::Document, key, || {
            call_count += 1;
            Ok("should_not_compute".to_string())
        })?;

        assert_eq!(result1, "computed");
        assert_eq!(result2, "computed");
        assert_eq!(call_count, 1, "compute function should only be called once");

        Ok(())
    }

    #[test]
    fn test_content_hash_consistency() {
        let content = b"test content";
        let hash1 = content_hash(content);
        let hash2 = content_hash(content);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_content_hash_different_inputs() {
        let hash1 = content_hash(b"content1");
        let hash2 = content_hash(b"content2");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_clear_all() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test.redb");
        let config = CacheConfig::new(&db_path);
        let cache = DocCache::open(config)?;

        cache.put_document(b"key1", &"value1")?;
        cache.put_scrape(b"url1", &"scraped")?;

        cache.clear_all()?;

        let stats = cache.stats()?;
        assert_eq!(stats.document_entries, 0);
        assert_eq!(stats.scrape_entries, 0);

        Ok(())
    }

    #[test]
    fn test_in_memory_cache() -> Result<()> {
        let config = CacheConfig::in_memory();
        let cache = DocCache::open(config)?;

        cache.put_document(b"key", &"value")?;
        let result: Option<String> = cache.get_document(b"key")?;
        assert_eq!(result, Some("value".to_string()));

        Ok(())
    }

    #[test]
    fn test_disabled_cache_skips_operations() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test.redb");
        let mut config = CacheConfig::new(&db_path);
        config.cache_document_content = false;

        let cache = DocCache::open(config)?;

        cache.put_document(b"key", &"value")?;

        let result: Option<String> = cache.get_document(b"key")?;
        assert!(result.is_none(), "Disabled cache should return None");

        Ok(())
    }

    #[test]
    fn test_key_too_large_returns_error() -> Result<()> {
        let config = CacheConfig::in_memory();
        let cache = DocCache::open(config)?;

        let oversized_key = vec![0u8; MAX_KEY_SIZE + 1];
        let result = cache.put_document(&oversized_key, &"value");

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("key too large"),
            "Error message should mention key too large: {err_msg}"
        );

        Ok(())
    }

    #[test]
    fn test_value_too_large_returns_error() -> Result<()> {
        let config = CacheConfig::in_memory();
        let cache = DocCache::open(config)?;

        // Create a value that exceeds MAX_VALUE_SIZE when serialized
        let oversized_value = "x".repeat(MAX_VALUE_SIZE + 1);
        let result = cache.put_document(b"key", &oversized_value);

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("value too large"),
            "Error message should mention value too large: {err_msg}"
        );

        Ok(())
    }

    #[test]
    fn test_key_at_max_size_succeeds() -> Result<()> {
        let config = CacheConfig::in_memory();
        let cache = DocCache::open(config)?;

        let max_key = vec![0u8; MAX_KEY_SIZE];
        cache.put_document(&max_key, &"value")?;

        let result: Option<String> = cache.get_document(&max_key)?;
        assert_eq!(result, Some("value".to_string()));

        Ok(())
    }

    #[test]
    fn test_scrape_key_size_validation() -> Result<()> {
        let config = CacheConfig::in_memory();
        let cache = DocCache::open(config)?;

        let oversized_key = vec![0u8; MAX_KEY_SIZE + 1];
        let result = cache.put_scrape(&oversized_key, &"value");

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("key too large"));

        Ok(())
    }

    #[test]
    fn test_transform_key_size_validation() -> Result<()> {
        let config = CacheConfig::in_memory();
        let cache = DocCache::open(config)?;

        let oversized_key = vec![0u8; MAX_KEY_SIZE + 1];
        let result = cache.put_transform(&oversized_key, &"value");

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("key too large"));

        Ok(())
    }
}
