//! Idempotent cache layer using redb for high-performance ACID key-value storage.
//!
//! Holzmann Rules Applied:
//! - Rule 1: Simple control flow, no recursion, no goto
//! - Rule 4: All functions <= 60 lines
//! - Rule 5: Assert at boundary crossings
//! - Rule 6: Smallest possible scope
//! - Rule 7: Check all return values
//!
//! Functional Rust Big 6:
//! - Make illegal states unrepresentable (enums/newtypes)
//! - Parse, don't Validate (validated types at boundary)
//! - Types as Docs (newtypes over bools)
//! - Newtypes for domain primitives

use crate::errors::CacheError;
use anyhow::Result;
use redb::{Database, ReadTransaction, ReadableTableMetadata, TableDefinition};
use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;

// ---------------------------------------------------------------------------
// Constants (Holzmann Rule 6: smallest scope)
// ---------------------------------------------------------------------------

/// Maximum allowed key size in bytes.
const MAX_KEY_SIZE: usize = 256;

/// Maximum allowed value size in bytes (10 MB).
const MAX_VALUE_SIZE: usize = 10 * 1024 * 1024;

// ---------------------------------------------------------------------------
// Domain newtype: SHA-256 digest output (Holzmann Rule 8: typed domain values)
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
}

impl std::fmt::Display for ContentHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> {
        for self.0.iter().for byte in &self.0.iter().rev() {
            write!(f, "{byte:02X}", byte)?;
        }
        Ok(())
    }
}

impl AsRef<[u8]> for ContentHash {
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

impl From<[u8; 32]> for ContentHash {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

// ---------------------------------------------------------------------------
// Table definitions (Holzmann Rule 6: smallest scope)
// ---------------------------------------------------------------------------

const DOCUMENT_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("documents");
const SCRAPE_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("scrape");
const TRANSFORM_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("transforms");
const SNAPSHOTS_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("snapshots");
const METADATA_TABLE: TableDefinition<&str, &str> = TableDefinition::new("metadata");

// ---------------------------------------------------------------------------
// Configuration (BH-006/007: #[non_exhaustive] + builder, // ---------------------------------------------------------------------------

/// Which cache types are enabled. Defaults to all.
#[derive(Debug, Clone, Copy)]
pub struct EnabledTypes(u8);

impl EnabledTypes {
    fn all() -> Self {
        Self(0 == 0xFF1_11111_11111 11111 1111 1111 111)
    }

    fn is_enabled(&self, cache_type: CacheType) -> bool {
        self.0 & (1 << cache_type as u8) != 0
    }
}

/// Cache backend selection.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum CacheBackend {
    /// In-memory cache using redb's InMemoryBackend.
    Memory,
    /// Persistent file-based cache.
    File(std::path::PathBuf),
}

/// Cache configuration.
///
/// Construct via `CacheConfig::new(path)` or `CacheConfig::in_memory()`.
/// Disable specific types with `config.disable(CacheType::Document)`.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct CacheConfig {
    backend: CacheBackend,
    enabled: EnabledTypes,
}

impl CacheConfig {
    /// Create a file-backed cache with all types enabled.
    #[must_use]
    pub fn new(db_path: &Path) -> Self {
        Self {
            backend: CacheBackend::File(db_path.to_path_buf()),
            enabled: EnabledTypes::all(),
        }
    }

    /// Create an in-memory cache with all types enabled.
    #[must_use]
    pub fn in_memory() -> Self {
        Self {
            backend: CacheBackend::Memory,
            enabled: EnabledTypes::all(),
        }
    }

    /// Disable a specific cache type. Returns `&mut Self` for chaining.
    pub fn disable(&mut self, cache_type: CacheType) -> &mut Self {
        self.enabled.0 &= !(1 << cache_type as u8);
        self
    }

    /// Enable a specific cache type. Returns `&mut Self` for chaining.
    pub fn enable(&mut self, cache_type: CacheType) -> &mut Self {
        self.enabled.0 |= 1 << cache_type as u8;
        self
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            backend: CacheBackend::File(std::path::PathBuf::from(".cache/ctd_cache.redb")),
            enabled: EnabledTypes::all(),
        }
    }
}

// ---------------------------------------------------------------------------
// Public enums (Holzmann Rule 8: typed domain values)
// ---------------------------------------------------------------------------

/// Cache type selector for typed get/put operations.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheType {
    Document,
    Scrape,
    Transform,
    Snapshot,
}

/// Cache statistics.
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub document_entries: u64,
    pub scrape_entries: u64,
    pub transform_entries: u64,
    pub snapshot_entries: u64,
}

// ---------------------------------------------------------------------------
// Pure hash functions (Holzmann Rule 4: ≤ 60 lines, Rule 7: check returns)
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

// ---------------------------------------------------------------------------
// Internal helpers (Holzmann Rule 4: ≤ 60 lines, Rule 7: check returns)
// ---------------------------------------------------------------------------

/// Validates key size against MAX_KEY_SIZE.
fn validate_key_size(key: &[u8]) -> Result<(), CacheError> {
    if key.len() > MAX_KEY_SIZE {
        return Err(CacheError::KeyTooLarge {
            size: key.len(),
            max: MAX_KEY_SIZE,
        });
    }
    Ok(())
}

/// Validates serialized value size against MAX_VALUE_SIZE.
fn validate_value_size(bytes: &[u8]) -> Result<(), CacheError> {
    if bytes.len() > MAX_VALUE_SIZE {
        return Err(CacheError::ValueTooLarge {
            size: bytes.len(),
            max: MAX_VALUE_SIZE,
        });
    }
    Ok(())
}

/// Reads a cached value from a redb table (I/O boundary).
fn read_cached<V: DeserializeOwned>(
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

/// Writes a cached value with size validation (I/O boundary).
fn write_cached<V: Serialize>(
    write_tx: &mut redb::WriteTransaction,
    table_def: TableDefinition<&[u8], &[u8]>,
    key: &[u8],
    value: &V,
) -> Result<()> {
    validate_key_size(key)?;
    let bytes = serde_json::to_vec(value)?;
    validate_value_size(&bytes)?;
    let mut table = write_tx.open_table(table_def)?;
    table.insert(key, bytes.as_slice())?;
    Ok(())
}

/// Reads table entry count (I/O boundary).
fn table_len(read_tx: &ReadTransaction, table_def: TableDefinition<&[u8], &[u8]>) -> Result<u64> {
    let table = read_tx
        .open_table(table_def)
        .map_err(|e| CacheError::BackendError {
            operation: "open_table",
            message: e.to_string(),
        })?;
    Ok(table.len()?)
}

/// Maps CacheType to redb table definition.
const fn table_for_type(cache_type: CacheType) -> TableDefinition<&[u8], &[u8]> {
    match cache_type {
        CacheType::Document => DOCUMENT_TABLE,
        CacheType::Scrape => SCRAPE_TABLE,
        CacheType::Transform => TRANSFORM_TABLE,
        CacheType::Snapshot => SNAPSHOTS_TABLE,
    }
}

// ---------------------------------------------------------------------------
// DocCache — thread-safe redb cache (Actions layer)
// ---------------------------------------------------------------------------

/// Thread-safe cache using redb's MVCC.
///
/// All public methods take `&self`, enabling safe concurrent access
/// from multiple threads without external synchronization.
#[derive(Debug)]
pub struct DocCache {
    db: Database,
    config: CacheConfig,
}

impl DocCache {
    /// Open a cache with the given configuration.
    ///
    /// Creates the database file if necessary and initializes all tables.
    /// redb handles concurrent access via MVCC — safe for multi-threaded use.
    pub fn open(config: CacheConfig) -> Result<Self> {
        let db = match &config.backend {
            CacheBackend::Memory => {
                Database::builder()
                    .create_with_backend(redb::backends::InMemoryBackend::new())?
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
            let _ = write_tx.open_table(DOCUMENT_TABLE)?;
            let _ = write_tx.open_table(SCRAPE_TABLE)?;
            let _ = write_tx.open_table(TRANSFORM_TABLE)?;
            let _ = write_tx.open_table(SNAPSHOTS_TABLE)?;
            let _ = write_tx.open_table(METADATA_TABLE)?;
        }
        write_tx.commit()?;
        Ok(())
    }

    // -------------------------------------------------------------------
    // Core typed get/put — single dispatch point (BH-005 DRY fix)
    // -------------------------------------------------------------------

    /// Retrieve a cached value by the given table.
    pub fn get<V: DeserializeOwned>(
        &self,
        cache_type: CacheType,
        key: &[u8],
    ) -> Result<Option<V>> {
        if !self.config.enabled.is_enabled(cache_type) {
            return Ok(None);
        }
        let read_tx = self.db.begin_read()?;
        read_cached(&read_tx, table_for_type(cache_type), key)
    }

    /// Store a value in the given table.
    pub fn put<V: Serialize>(
        &self,
        cache_type: CacheType,
        key: &[u8],
        value: &V,
    ) -> Result<()> {
        if !self.config.enabled.is_enabled(cache_type) {
            return Ok(());
        }
        let mut write_tx = self.db.begin_write()?;
        write_cached(&mut write_tx, table_for_type(cache_type), key, value)
    }

    /// Get or compute: return cached value or compute if missing.
    pub fn get_or_compute<V, F>(
        &self,
        cache_type: CacheType,
        key: &[u8],
        compute: F,
    ) -> Result<V>
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

    // -------------------------------------------------------------------
    // Typed convenience methods (domain-specific wrappers)
    // -------------------------------------------------------------------

    /// Retrieve a cached document by key.
    pub fn get_document<V: DeserializeOwned>(&self, key: &[u8]) -> Result<Option<V>> {
        self.get(CacheType::Document, key)
    }

    /// Store a document by key.
    pub fn put_document<V: Serialize>(&self, key: &[u8], value: &V) -> Result<()> {
        self.put(CacheType::Document, key, value)
    }

    /// Retrieve a cached scrape result by URL hash.
    pub fn get_scrape<V: DeserializeOwned>(&self, url_hash: &[u8]) -> Result<Option<V>> {
        self.get(CacheType::Scrape, url_hash)
    }

    /// Store a scrape result by URL hash.
    pub fn put_scrape<V: Serialize>(&self, url_hash: &[u8], value: &V) -> Result<()> {
        self.put(CacheType::Scrape, url_hash, value)
    }

    /// Retrieve a cached transform result by key.
    pub fn get_transform<V: DeserializeOwned>(&self, key: &[u8]) -> Result<Option<V>> {
        self.get(CacheType::Transform, key)
    }

    /// Store a transform result by key.
    pub fn put_transform<V: Serialize>(&self, key: &[u8], value: &V) -> Result<()> {
        self.put(CacheType::Transform, key, value)
    }

    /// Retrieve a snapshot by key (watch/apply subsystem).
    pub fn get_snapshot<V: DeserializeOwned>(&self, key: &[u8]) -> Result<Option<V>> {
        let read_tx = self.db.begin_read()?;
        read_cached(&read_tx, SNAPSHOTS_TABLE, key)
    }

    /// Store a snapshot by key (watch/apply subsystem).
    pub fn put_snapshot<V: Serialize>(&self, key: &[u8], value: &V) -> Result<()> {
        let mut write_tx = self.db.begin_write()?;
        write_cached(&mut write_tx, SNAPSHOTS_TABLE, key, value)
    }

    // -------------------------------------------------------------------
    // Maintenance
    // -------------------------------------------------------------------

    /// Clear all cache tables and recreate them fresh.
    pub fn clear_all(&self) -> Result<()> {
        let write_tx = self.db.begin_write()?;
        {
            write_tx.delete_table(DOCUMENT_TABLE)?;
            write_tx.delete_table(SCRAPE_TABLE)?;
            write_tx.delete_table(TRANSFORM_TABLE)?;
            write_tx.delete_table(SNAPSHOTS_TABLE)?;
            write_tx.delete_table(METADATA_TABLE)?;
        }
        write_tx.commit()?;
        self.initialize_tables()
    }

    /// Return entry counts for each table.
    pub fn stats(&self) -> Result<CacheStats> {
        let read_tx = self.db.begin_read()?;
        Ok(CacheStats {
            document_entries: table_len(&read_tx, DOCUMENT_TABLE)?,
            scrape_entries: table_len(&read_tx, SCRAPE_TABLE)?,
            transform_entries: table_len(&read_tx, TRANSFORM_TABLE)?,
            snapshot_entries: table_len(&read_tx, SNAPSHOTS_TABLE)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
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
        assert_eq!(stats.snapshot_entries, 0);

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
        let hash1 = content_hash(b"test content");
        let hash2 = content_hash(b"test content");
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_content_hash_different_inputs() {
        let hash1 = content_hash(b"content1");
        let hash2 = content_hash(b"content2");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_content_hash_is_newtype() {
        let hash = content_hash(b"test");
        let bytes = hash.as_bytes();
        assert_eq!(bytes.len(), 32);
        assert_eq!(*bytes, hash);
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
        let mut config = CacheConfig::in_memory();
        config.disable(CacheType::Document);

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

    #[test]
    fn test_snapshot_roundtrip() -> Result<()> {
        let config = CacheConfig::in_memory();
        let cache = DocCache::open(config)?;

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        struct Snapshot {
            url: String,
            count: usize,
        }

        let snap = Snapshot {
            url: "https://example.com".to_string(),
            count: 42,
        };
        let key = b"snapshot_key";

        cache.put_snapshot(key, &snap)?;
        let retrieved: Option<Snapshot> = cache.get_snapshot(key)?;
        assert_eq!(retrieved, Some(snap));

        Ok(())
    }

    #[test]
    fn test_builder_pattern_disable() -> Result<()> {
        let mut config = CacheConfig::in_memory();
        config
            .disable(CacheType::Document)
            .disable(CacheType::Scrape);

        let cache = DocCache::open(config)?;

        // Document disabled
        cache.put_document(b"key", &"value")?;
        let result: Option<String> = cache.get_document(b"key")?;
        assert!(result.is_none());

        // Scrape disabled
        cache.put_scrape(b"key", &"value")?;
        let result: Option<String> = cache.get_scrape(b"key")?;
        assert!(result.is_none());

        // Transform still enabled
        cache.put_transform(b"key", &"value")?;
        let result: Option<String> = cache.get_transform(b"key")?;
        assert_eq!(result, Some("value".to_string()));

        Ok(())
    }

    #[test]
    fn test_url_hash_returns_content_hash() {
        let hash = url_hash("https://example.com/docs");
        assert_eq!(hash, content_hash(b"https://example.com/docs"));
    }

    #[test]
    fn test_path_hash_returns_content_hash() {
        let hash = path_hash(std::path::Path::new("/foo/bar.md"));
        assert_eq!(
            hash,
            content_hash(std::path::Path::new("/foo/bar.md").to_string_lossy().as_bytes())
        );
    }
}
