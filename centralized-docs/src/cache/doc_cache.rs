//! `DocCache`: thread-safe redb cache (Actions layer).

#![allow(clippy::wildcard_imports)]

use super::config::CacheConfig;
use super::types::*;
use anyhow::Result;
use redb::{backends::InMemoryBackend, Builder, Database};

// ---------------------------------------------------------------------------
// Internal backend
// ---------------------------------------------------------------------------

/// Internal cache backend — newtype wrapping a redb `Database`.
struct CacheBackendInner(Database);

impl std::fmt::Debug for CacheBackendInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CacheBackendInner(..)")
    }
}

// ---------------------------------------------------------------------------
// DocCache
// ---------------------------------------------------------------------------

/// Thread-safe cache using redb (in-memory or file-backed).
///
/// All public methods take `&self`, enabling safe concurrent access
/// from multiple threads without external synchronization.
#[derive(Debug)]
pub struct DocCache {
    inner: CacheBackendInner,
    config: CacheConfig,
}

impl DocCache {
    /// Open a cache with the given configuration.
    pub fn open(config: CacheConfig) -> Result<Self> {
        let inner = match &config.backend {
            super::config::CacheBackend::Memory => {
                CacheBackendInner(Builder::new().create_with_backend(InMemoryBackend::new())?)
            }
            super::config::CacheBackend::File(path) => {
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                CacheBackendInner(Database::create(path)?)
            }
        };
        let cache = Self { inner, config };
        cache.initialize_tables()?;
        Ok(cache)
    }

    fn initialize_tables(&self) -> Result<()> {
        let write_tx = self.inner.0.begin_write()?;
        {
            let _ = write_tx.open_table(DOCUMENT_TABLE)?;
            let _ = write_tx.open_table(SCRAPE_TABLE)?;
            let _ = write_tx.open_table(TRANSFORM_TABLE)?;
            let _ = write_tx.open_table(SNAPSHOTS_TABLE)?;
            let _ = write_tx.open_table(ANALYSIS_TABLE)?;
            let _ = write_tx.open_table(CHUNK_TABLE)?;
            let _ = write_tx.open_table(METADATA_TABLE)?;
        }
        write_tx.commit()?;
        Ok(())
    }

    // --- Core typed get/put ---

    /// Retrieve a cached value by the given table.
    pub fn get<V: serde::de::DeserializeOwned>(
        &self,
        cache_type: CacheType,
        key: &[u8],
    ) -> Result<Option<V>> {
        if !self.config.enabled.is_enabled(cache_type) {
            return Ok(None);
        }
        let read_tx = self.inner.0.begin_read()?;
        read_cached(&read_tx, table_for_type(cache_type), key)
    }

    /// Store a value in the given table.
    pub fn put<V: serde::Serialize>(
        &self,
        cache_type: CacheType,
        key: &[u8],
        value: &V,
    ) -> Result<()> {
        if !self.config.enabled.is_enabled(cache_type) {
            return Ok(());
        }
        let mut write_tx = self.inner.0.begin_write()?;
        write_cached(&mut write_tx, table_for_type(cache_type), key, value)?;
        write_tx.commit()?;
        Ok(())
    }

    /// Get or compute: return cached value or compute if missing.
    pub fn get_or_compute<V, F>(&self, cache_type: CacheType, key: &[u8], compute: F) -> Result<V>
    where
        V: serde::Serialize + serde::de::DeserializeOwned,
        F: FnOnce() -> Result<V>,
    {
        if let Some(cached) = self.get::<V>(cache_type, key)? {
            return Ok(cached);
        }
        let value = compute()?;
        self.put(cache_type, key, &value)?;
        Ok(value)
    }

    // --- Typed convenience methods ---

    /// Retrieve a cached document by key.
    pub fn get_document<V: serde::de::DeserializeOwned>(&self, key: &[u8]) -> Result<Option<V>> {
        self.get(CacheType::Document, key)
    }

    /// Store a document by key.
    pub fn put_document<V: serde::Serialize>(&self, key: &[u8], value: &V) -> Result<()> {
        self.put(CacheType::Document, key, value)
    }

    /// Retrieve a cached scrape result by URL hash.
    pub fn get_scrape<V: serde::de::DeserializeOwned>(&self, url_hash: &[u8]) -> Result<Option<V>> {
        self.get(CacheType::Scrape, url_hash)
    }

    /// Store a scrape result by URL hash.
    pub fn put_scrape<V: serde::Serialize>(&self, url_hash: &[u8], value: &V) -> Result<()> {
        self.put(CacheType::Scrape, url_hash, value)
    }

    /// Retrieve a cached transform result by key.
    pub fn get_transform<V: serde::de::DeserializeOwned>(&self, key: &[u8]) -> Result<Option<V>> {
        self.get(CacheType::Transform, key)
    }

    /// Store a transform result by key.
    pub fn put_transform<V: serde::Serialize>(&self, key: &[u8], value: &V) -> Result<()> {
        self.put(CacheType::Transform, key, value)
    }

    /// Retrieve a snapshot by key (watch/apply subsystem).
    pub fn get_snapshot<V: serde::de::DeserializeOwned>(&self, key: &[u8]) -> Result<Option<V>> {
        let read_tx = self.inner.0.begin_read()?;
        read_cached(&read_tx, SNAPSHOTS_TABLE, key)
    }

    /// Store a snapshot by key (watch/apply subsystem).
    pub fn put_snapshot<V: serde::Serialize>(&self, key: &[u8], value: &V) -> Result<()> {
        let mut write_tx = self.inner.0.begin_write()?;
        write_cached(&mut write_tx, SNAPSHOTS_TABLE, key, value)?;
        write_tx.commit()?;
        Ok(())
    }

    // --- Maintenance ---

    /// Clear all cache entries (delete tables + reinit).
    pub fn clear_all(&self) -> Result<()> {
        let write_tx = self.inner.0.begin_write()?;
        {
            write_tx.delete_table(DOCUMENT_TABLE)?;
            write_tx.delete_table(SCRAPE_TABLE)?;
            write_tx.delete_table(TRANSFORM_TABLE)?;
            write_tx.delete_table(SNAPSHOTS_TABLE)?;
            write_tx.delete_table(ANALYSIS_TABLE)?;
            write_tx.delete_table(CHUNK_TABLE)?;
            write_tx.delete_table(METADATA_TABLE)?;
        }
        write_tx.commit()?;
        self.initialize_tables()
    }

    /// Return entry counts for each table.
    pub fn stats(&self) -> Result<CacheStats> {
        let read_tx = self.inner.0.begin_read()?;
        Ok(CacheStats {
            document_entries: table_len(&read_tx, DOCUMENT_TABLE)?,
            scrape_entries: table_len(&read_tx, SCRAPE_TABLE)?,
            transform_entries: table_len(&read_tx, TRANSFORM_TABLE)?,
            snapshot_entries: table_len(&read_tx, SNAPSHOTS_TABLE)?,
            analysis_entries: table_len(&read_tx, ANALYSIS_TABLE)?,
            chunk_entries: table_len(&read_tx, CHUNK_TABLE)?,
        })
    }
}
