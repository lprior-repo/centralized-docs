//! `DocCache` — the primary cache store backed by `redb`.
//!
//! Provides typed get/put operations for documents, scraped URLs, and transforms,
//! plus `get_or_compute` with lock-free in-flight deduplication, and
//! `stats`/`clear_all` maintenance methods.

mod dedup;

use std::sync::Arc;

use anyhow::Result;
use dashmap::DashMap;
use redb::Database;
use serde::{de::DeserializeOwned, Serialize};

use super::config::{
    CacheBackend, CacheConfig, CacheStats, CacheType, DOCUMENT_CACHE_TABLE, METADATA_TABLE,
    SCRAPE_CACHE_TABLE, TRANSFORM_CACHE_TABLE,
};
use super::hash::{
    get_cached_value, put_cached_value_with_limit, table_len, validate_and_insert,
    validate_key_size,
};

/// Thread-safe cache using redb's MVCC with lock-free in-flight deduplication.
///
/// # Thread Safety
///
/// This type is `Send + Sync` and `Clone`. redb uses MVCC for concurrent read access.
/// Write transactions are serialized at the database level. The inner `Database` is
/// wrapped in `Arc` to enable cheap cloning for sharing across threads.
///
/// `get_or_compute` uses `DashMap` + `OnceLock` for exact-once computation:
/// multiple threads with the same key will see the compute closure invoked
/// exactly once, with all other threads yielding via `thread::yield_now()`
/// and receiving the result once the owner publishes.
///
/// No `Mutex` is used anywhere in this type.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct DocCache {
    db: Arc<Database>,
    config: CacheConfig,
    in_flight: Arc<DashMap<dedup::InFlightKey, dedup::ComputeSlot>>,
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
        let cache = Self {
            db: Arc::new(db),
            config,
            in_flight: Arc::new(DashMap::new()),
        };
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
            #[allow(unused_mut)]
            // redb WriteTransaction::open_table returns &mut Table — API constraint
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
            #[allow(unused_mut)]
            // redb WriteTransaction::open_table returns &mut Table — API constraint
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
            #[allow(unused_mut)]
            // redb WriteTransaction::open_table returns &mut Table — API constraint
            let mut table = write_tx.open_table(TRANSFORM_CACHE_TABLE)?;
            put_cached_value_with_limit(&mut table, key, value)?;
        }
        write_tx.commit()?;
        Ok(())
    }

    pub fn clear_all(&self) -> Result<()> {
        self.in_flight.clear();
        let write_tx = self.db.begin_write()?;
        {
            write_tx.delete_table(DOCUMENT_CACHE_TABLE)?;
            write_tx.delete_table(SCRAPE_CACHE_TABLE)?;
            write_tx.delete_table(TRANSFORM_CACHE_TABLE)?;
            write_tx.delete_table(METADATA_TABLE)?;
            let _ = write_tx.open_table(DOCUMENT_CACHE_TABLE)?;
            let _ = write_tx.open_table(SCRAPE_CACHE_TABLE)?;
            let _ = write_tx.open_table(TRANSFORM_CACHE_TABLE)?;
            let _ = write_tx.open_table(METADATA_TABLE)?;
        }
        write_tx.commit()?;
        Ok(())
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

    /// Get a cached value, or compute and cache it — with exact-once deduplication.
    ///
    /// # Guarantees
    ///
    /// When N threads call this with the same `(cache_type, key)` simultaneously
    /// and the value is not cached, the `compute` closure is invoked **exactly once**.
    /// The other N-1 threads yield (via `thread::yield_now()`) and receive the
    /// result once the owner completes.
    pub fn get_or_compute<V, F>(&self, cache_type: CacheType, key: &[u8], compute: F) -> Result<V>
    where
        V: Serialize + DeserializeOwned,
        F: FnOnce() -> Result<V>,
    {
        validate_key_size(key)?;

        let decision =
            dedup::check_cache_and_inflight(&self.in_flight, cache_type, key, |ct, k| {
                self.get(ct, k)
            })?;

        match decision {
            dedup::InflightDecision::Cached(value)
            | dedup::InflightDecision::WaiterResult(value) => Ok(value),
            dedup::InflightDecision::Owner {
                in_flight_key,
                slot,
            } => {
                let compute_result =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(compute));
                let compute_result = match compute_result {
                    Ok(result) => result,
                    Err(panic_payload) => {
                        let msg = if let Some(s) = panic_payload.downcast_ref::<&str>() {
                            s.to_string()
                        } else if let Some(s) = panic_payload.downcast_ref::<String>() {
                            s.clone()
                        } else {
                            "compute panicked with unknown payload".to_string()
                        };
                        Err(anyhow::anyhow!("compute panicked: {msg}"))
                    }
                };
                let cache_enabled = self.is_cache_type_enabled(cache_type);
                dedup::finalize_compute(
                    &self.in_flight,
                    in_flight_key,
                    &slot,
                    &compute_result,
                    |ct, k, b| {
                        if cache_enabled {
                            self.put_raw(ct, k, b)
                        } else {
                            Ok(())
                        }
                    },
                    cache_type,
                    key,
                )?;
                compute_result
            }
        }
    }

    fn get<V: DeserializeOwned>(&self, cache_type: CacheType, key: &[u8]) -> Result<Option<V>> {
        match cache_type {
            CacheType::Document => self.get_document(key),
            CacheType::Scrape => self.get_scrape(key),
            CacheType::Transform => self.get_transform(key),
        }
    }

    fn is_cache_type_enabled(&self, cache_type: CacheType) -> bool {
        match cache_type {
            CacheType::Document => self.config.cache_document_content,
            CacheType::Scrape => self.config.cache_scrape_results,
            CacheType::Transform => self.config.cache_transforms,
        }
    }

    /// Write pre-serialized bytes directly to the cache — avoids double serialization.
    ///
    /// Value size is validated inside `validate_and_insert` (DEFECT-006).
    fn put_raw(&self, cache_type: CacheType, key: &[u8], bytes: &[u8]) -> Result<()> {
        validate_key_size(key)?;
        let write_tx = self.db.begin_write()?;
        {
            let table_def = match cache_type {
                CacheType::Document => DOCUMENT_CACHE_TABLE,
                CacheType::Scrape => SCRAPE_CACHE_TABLE,
                CacheType::Transform => TRANSFORM_CACHE_TABLE,
            };
            #[allow(unused_mut)]
            // redb WriteTransaction::open_table returns &mut Table — API constraint
            let mut table = write_tx.open_table(table_def)?;
            validate_and_insert(&mut table, key, bytes)?;
        }
        write_tx.commit()?;
        Ok(())
    }
}
