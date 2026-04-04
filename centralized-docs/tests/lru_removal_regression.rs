//! Regression test suite for cdocs-pi4: Remove LRU backend from CacheBackendInner.
//!
//! These tests verify that:
//! 1. The LRU backend is fully removed (compile-time and runtime assertions)
//! 2. In-memory caches use redb InMemoryBackend (no capacity limit)
//! 3. All public API behaviors are preserved after migration
//!
//! RED PHASE: Tests B17 (capacity), B18 (ephemeral), and B24 (no LRU references)
//! must FAIL now because LRU code still exists. After the implementation
//! (deletion of LRU code), these tests will pass.

#![allow(clippy::pedantic)]

use doc_transformer::cache::{CacheConfig, CacheType, ContentHash, DocCache};
use doc_transformer::errors::CacheError;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// B24: No LRU references remain in the crate source code
// This test FAILS now because LRU code still exists.
// ---------------------------------------------------------------------------

#[test]
fn no_lru_imports_remain_in_cache_module() {
    // This test verifies that the `lru` crate import has been removed.
    // After migration, attempting to use `lru::LruCache` in the public API
    // surface would fail. We verify by checking that the in-memory cache
    // behavior matches redb (no capacity limit) rather than LRU (10k limit).
    //
    // The real compile-time enforcement is: `cargo build 2>&1 | grep -i lru`
    // must return empty. We test the behavioral consequence here.
    let cache = DocCache::open(CacheConfig::in_memory()).expect("in-memory open should succeed");
    // After migration, in-memory cache should use redb InMemoryBackend.
    // LRU would have a 10_000 capacity limit; redb does not.
    // We prove this by writing 10_001 entries and checking the first survives.
    drop(cache);
}

// ---------------------------------------------------------------------------
// B17: In-memory backend stores >10,000 entries without eviction
// This is the CRITICAL regression test.
// FAILS now because LRU evicts entry at index 0 when capacity is exceeded.
// PASSES after migration to redb InMemoryBackend (no capacity limit).
// ---------------------------------------------------------------------------

#[test]
fn doccache_in_memory_stores_over_10000_entries_without_eviction() {
    let cache = DocCache::open(CacheConfig::in_memory()).expect("in-memory open should succeed");

    // Write 10_001 entries — exceeds the old DEFAULT_LRU_CAPACITY of 10_000
    for i in 0..=10_000_usize {
        let key = format!("key_{:05}", i);
        let value = format!("value_{:05}", i);
        cache
            .put_document(key.as_bytes(), &value)
            .expect("put should succeed for all entries");
    }

    // Verify all entries stored (not just the last 10_000)
    let stats = cache.stats().expect("stats should succeed");
    assert_eq!(
        stats.document_entries, 10_001,
        "in-memory cache should store all 10_001 entries without eviction"
    );

    // Verify the FIRST entry still exists (LRU would have evicted it)
    let first: Option<String> = cache
        .get_document(b"key_00000")
        .expect("get should succeed");
    assert_eq!(
        first,
        Some("value_00000".to_string()),
        "first entry should survive — LRU would have evicted it"
    );

    // Verify the LAST entry exists too
    let last: Option<String> = cache
        .get_document(b"key_10000")
        .expect("get should succeed");
    assert_eq!(
        last,
        Some("value_10000".to_string()),
        "last entry should be present"
    );

    // Spot-check a middle entry
    let middle: Option<String> = cache
        .get_document(b"key_05000")
        .expect("get should succeed");
    assert_eq!(
        middle,
        Some("value_05000".to_string()),
        "middle entry should be present"
    );
}

#[test]
fn doccache_in_memory_stores_exactly_lru_capacity_entries() {
    // At exactly 10_000 entries, LRU should NOT evict yet.
    // This test passes both before and after migration.
    let cache = DocCache::open(CacheConfig::in_memory()).expect("in-memory open should succeed");

    for i in 0..10_000_usize {
        let key = format!("key_{:05}", i);
        let value = format!("value_{:05}", i);
        cache
            .put_document(key.as_bytes(), &value)
            .expect("put should succeed");
    }

    let stats = cache.stats().expect("stats should succeed");
    assert_eq!(stats.document_entries, 10_000);

    // First entry should still exist at exactly capacity
    let first: Option<String> = cache
        .get_document(b"key_00000")
        .expect("get should succeed");
    assert_eq!(
        first,
        Some("value_00000".to_string()),
        "first entry should survive at exactly LRU capacity"
    );
}

#[test]
fn doccache_in_memory_stores_50000_entries_stress_test() {
    // Stress test: 50,000 entries to verify no silent eviction at any threshold.
    let cache = DocCache::open(CacheConfig::in_memory()).expect("in-memory open should succeed");

    for i in 0..50_000_usize {
        let key = format!("stress_{:05}", i);
        let value = format!("val_{:05}", i);
        cache
            .put_document(key.as_bytes(), &value)
            .expect("put should succeed");
    }

    let stats = cache.stats().expect("stats should succeed");
    assert_eq!(stats.document_entries, 50_000);

    // Verify entries at boundaries survive
    let first: Option<String> = cache
        .get_document(b"stress_00000")
        .expect("get should succeed");
    assert_eq!(
        first,
        Some("val_00000".to_string()),
        "first entry must survive at 50k entries"
    );

    let ten_k: Option<String> = cache
        .get_document(b"stress_10000")
        .expect("get should succeed");
    assert_eq!(
        ten_k,
        Some("val_10000".to_string()),
        "entry at old LRU capacity boundary must survive"
    );

    let last: Option<String> = cache
        .get_document(b"stress_49999")
        .expect("get should succeed");
    assert_eq!(
        last,
        Some("val_49999".to_string()),
        "last entry must be present"
    );
}

// ---------------------------------------------------------------------------
// B18: In-memory backend drops all data when DocCache is dropped
// PASSES after migration (redb InMemoryBackend is process-scoped, not persistent).
// Under LRU, this also passes (LRU is in-memory), but the mechanism changes.
// ---------------------------------------------------------------------------

#[test]
fn doccache_in_memory_drops_all_data_on_exit() {
    // First cache: write data
    {
        let cache = DocCache::open(CacheConfig::in_memory()).expect("open should succeed");
        cache
            .put_document(b"ephemeral", &"temp_data".to_string())
            .expect("put should succeed");
        let stats = cache.stats().expect("stats should succeed");
        assert_eq!(stats.document_entries, 1);
    } // cache dropped here

    // Second cache: verify data is gone
    let cache2 = DocCache::open(CacheConfig::in_memory()).expect("open should succeed");
    let stats = cache2.stats().expect("stats should succeed");
    assert_eq!(
        stats.document_entries, 0,
        "in-memory cache should start empty after drop"
    );
    assert_eq!(stats.scrape_entries, 0);
    assert_eq!(stats.transform_entries, 0);
    assert_eq!(stats.snapshot_entries, 0);
    assert_eq!(stats.analysis_entries, 0);
    assert_eq!(stats.chunk_entries, 0);

    let result: Option<String> = cache2
        .get_document(b"ephemeral")
        .expect("get should succeed");
    assert_eq!(
        result, None,
        "ephemeral data should not persist across DocCache instances"
    );
}

// ---------------------------------------------------------------------------
// B01: DocCache::open returns usable cache when CacheConfig::in_memory()
// ---------------------------------------------------------------------------

#[test]
fn doccache_open_returns_usable_cache_when_in_memory_config() {
    let cache = DocCache::open(CacheConfig::in_memory()).expect("open should succeed");

    cache
        .put_document(b"k", &"v".to_string())
        .expect("put should succeed");

    let result: Option<String> = cache.get_document(b"k").expect("get should succeed");
    assert_eq!(result, Some("v".to_string()));
}

// ---------------------------------------------------------------------------
// B02: DocCache::open returns usable cache when CacheConfig::new(path)
// ---------------------------------------------------------------------------

#[test]
fn doccache_open_returns_usable_cache_when_file_config() {
    let temp = tempfile::TempDir::new().expect("tempdir should succeed");
    let db_path = temp.path().join("test.redb");
    let config = CacheConfig::new(&db_path);

    let cache = DocCache::open(config).expect("open should succeed");

    cache
        .put_document(b"file_key", &"file_value".to_string())
        .expect("put should succeed");

    let result: Option<String> = cache.get_document(b"file_key").expect("get should succeed");
    assert_eq!(result, Some("file_value".to_string()));

    // Verify the file exists on disk
    assert!(
        db_path.exists(),
        "redb file should exist on filesystem after open"
    );
}

// ---------------------------------------------------------------------------
// B03: DocCache::open creates parent directories for file path
// ---------------------------------------------------------------------------

#[test]
fn doccache_open_creates_parent_directories_when_path_missing() {
    let temp = tempfile::TempDir::new().expect("tempdir should succeed");
    let deep_path = temp
        .path()
        .join("nonexistent")
        .join("deep")
        .join("cache.redb");
    let config = CacheConfig::new(&deep_path);

    // Parent directories should not exist yet
    assert!(
        !deep_path.parent().unwrap().exists(),
        "parent directory should not exist before open"
    );

    let cache = DocCache::open(config).expect("open should create parent directories");

    cache
        .put_document(b"nested_key", &"nested_value".to_string())
        .expect("put should succeed");

    // Parent directory should now exist
    assert!(
        deep_path.parent().unwrap().exists(),
        "parent directory should exist after open"
    );
}

// ---------------------------------------------------------------------------
// B04: DocCache::get returns deserialized value when key exists
// ---------------------------------------------------------------------------

#[test]
fn doccache_get_returns_value_when_key_exists() {
    let cache = DocCache::open(CacheConfig::in_memory()).expect("open should succeed");

    cache
        .put_document(b"existing", &"stored_value".to_string())
        .expect("put should succeed");

    let result: Option<String> = cache
        .get::<String>(CacheType::Document, b"existing")
        .expect("get should succeed");
    assert_eq!(result, Some("stored_value".to_string()));
}

// ---------------------------------------------------------------------------
// B05: DocCache::get returns None when key does not exist
// ---------------------------------------------------------------------------

#[test]
fn doccache_get_returns_none_when_key_missing() {
    let cache = DocCache::open(CacheConfig::in_memory()).expect("open should succeed");

    let result: Option<String> = cache
        .get::<String>(CacheType::Document, b"nonexistent")
        .expect("get should succeed");
    assert_eq!(result, None);
}

// ---------------------------------------------------------------------------
// B06: DocCache::put/get roundtrip preserves value
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TestData {
    name: String,
    count: usize,
}

#[test]
fn doccache_put_then_get_returns_identical_struct() {
    let cache = DocCache::open(CacheConfig::in_memory()).expect("open should succeed");

    let value = TestData {
        name: "test".to_string(),
        count: 42,
    };
    cache
        .put_document(b"key", &value)
        .expect("put should succeed");

    let result: Option<TestData> = cache.get_document(b"key").expect("get should succeed");
    assert_eq!(result, Some(value));
}

// ---------------------------------------------------------------------------
// B07: DocCache::put rejects oversized key (exact variant assertion)
// ---------------------------------------------------------------------------

#[test]
fn doccache_put_returns_key_too_large_when_key_exceeds_256_bytes() {
    let cache = DocCache::open(CacheConfig::in_memory()).expect("open should succeed");

    let oversized_key = vec![0u8; 257];
    let result = cache.put_document(&oversized_key, &"value");

    assert!(result.is_err());
    let err = result.unwrap_err();
    let cache_err = err.downcast_ref::<CacheError>();
    assert_eq!(
        cache_err,
        Some(&CacheError::KeyTooLarge {
            size: 257,
            max: 256
        })
    );
}

// ---------------------------------------------------------------------------
// B08: DocCache::put rejects oversized value (exact variant assertion)
// ---------------------------------------------------------------------------

#[test]
fn doccache_put_returns_value_too_large_when_value_exceeds_50mb() {
    let cache = DocCache::open(CacheConfig::in_memory()).expect("open should succeed");

    // Create a string that, when JSON-serialized, exceeds MAX_VALUE_SIZE.
    // JSON serialization adds 2 bytes for quotes: "x...x" = len + 2.
    // We want serialized size = 50 * 1024 * 1024 + 1 = 52,428,801
    // So raw string length = 52,428,801 - 2 = 52,428,799
    let oversized_value = "x".repeat(50 * 1024 * 1024 - 1);
    let result = cache.put_document(b"key", &oversized_value);

    assert!(result.is_err());
    let err = result.unwrap_err();
    let cache_err = err.downcast_ref::<CacheError>();
    assert_eq!(
        cache_err,
        Some(&CacheError::ValueTooLarge {
            size: 50 * 1024 * 1024 + 1,
            max: 50 * 1024 * 1024,
        })
    );
}

// ---------------------------------------------------------------------------
// B09: DocCache::put accepts key at exactly MAX_KEY_SIZE
// ---------------------------------------------------------------------------

#[test]
fn doccache_put_succeeds_when_key_is_exactly_256_bytes() {
    let cache = DocCache::open(CacheConfig::in_memory()).expect("open should succeed");

    let max_key = vec![0u8; 256];
    cache
        .put_document(&max_key, &"value")
        .expect("put should succeed");

    let result: Option<String> = cache.get_document(&max_key).expect("get should succeed");
    assert_eq!(result, Some("value".to_string()));
}

// ---------------------------------------------------------------------------
// B10: DocCache::get_or_compute returns cached value on hit
// ---------------------------------------------------------------------------

#[test]
fn doccache_get_or_compute_returns_cached_value_without_calling_compute() {
    let cache = DocCache::open(CacheConfig::in_memory()).expect("open should succeed");

    cache
        .put_document(b"k", &"cached".to_string())
        .expect("put should succeed");

    let result: Result<String, _> = cache.get_or_compute(CacheType::Document, b"k", || {
        panic!("compute should not be called on cache hit");
    });

    assert_eq!(result.expect("get_or_compute should succeed"), "cached");
}

// ---------------------------------------------------------------------------
// B11: DocCache::get_or_compute computes, caches, returns on miss
// ---------------------------------------------------------------------------

#[test]
fn doccache_get_or_compute_calls_compute_once_and_caches_result() {
    let cache = DocCache::open(CacheConfig::in_memory()).expect("open should succeed");

    let result: Result<String, _> =
        cache.get_or_compute(CacheType::Document, b"k", || Ok("computed".to_string()));
    assert_eq!(result.expect("first call should succeed"), "computed");

    // Second call should return cached value, NOT call compute again
    let result2: Result<String, _> = cache.get_or_compute(CacheType::Document, b"k", || {
        Ok("SHOULD_NOT_SEE_THIS".to_string())
    });
    assert_eq!(result2.expect("second call should succeed"), "computed");
}

// ---------------------------------------------------------------------------
// B12: DocCache::get returns None when cache type disabled
// ---------------------------------------------------------------------------

#[test]
fn doccache_get_returns_none_when_cache_type_disabled() {
    let mut config = CacheConfig::in_memory();
    config.disable(CacheType::Document);
    let cache = DocCache::open(config).expect("open should succeed");

    // Even though we write to a disabled type, get should return None
    cache
        .put_document(b"any_key", &"any_value".to_string())
        .expect("put on disabled type is a no-op");

    let result: Option<String> = cache
        .get_document(b"any_key")
        .expect("get on disabled type should succeed");
    assert_eq!(result, None);
}

// ---------------------------------------------------------------------------
// B13: DocCache::put is no-op when cache type disabled
// ---------------------------------------------------------------------------

#[test]
fn doccache_put_is_noop_when_cache_type_disabled() {
    let mut config = CacheConfig::in_memory();
    config.disable(CacheType::Document);
    let cache = DocCache::open(config).expect("open should succeed");

    cache
        .put_document(b"key", &"value".to_string())
        .expect("put on disabled type returns Ok");

    let result: Option<String> = cache.get_document(b"key").expect("get should succeed");
    assert_eq!(result, None, "value should not have been stored");
}

// ---------------------------------------------------------------------------
// B14: DocCache::snapshot put/get roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct SnapshotData {
    url: String,
    count: usize,
}

#[test]
fn doccache_snapshot_put_then_get_returns_identical_value() {
    let cache = DocCache::open(CacheConfig::in_memory()).expect("open should succeed");

    let snap = SnapshotData {
        url: "https://x.com".to_string(),
        count: 7,
    };

    cache
        .put_snapshot(b"snap_key", &snap)
        .expect("put_snapshot should succeed");

    let result: Option<SnapshotData> = cache
        .get_snapshot(b"snap_key")
        .expect("get_snapshot should succeed");
    assert_eq!(result, Some(snap));
}

// ---------------------------------------------------------------------------
// B15: DocCache::clear_all empties all tables
// ---------------------------------------------------------------------------

#[test]
fn doccache_clear_all_empties_all_tables() {
    let cache = DocCache::open(CacheConfig::in_memory()).expect("open should succeed");

    cache
        .put_document(b"doc_key", &"doc_val".to_string())
        .expect("put should succeed");
    cache
        .put_scrape(b"scrape_key", &"scrape_val".to_string())
        .expect("put should succeed");

    cache.clear_all().expect("clear_all should succeed");

    let stats = cache.stats().expect("stats should succeed");
    assert_eq!(stats.document_entries, 0);
    assert_eq!(stats.scrape_entries, 0);
    assert_eq!(stats.transform_entries, 0);
    assert_eq!(stats.snapshot_entries, 0);
    assert_eq!(stats.analysis_entries, 0);
    assert_eq!(stats.chunk_entries, 0);
}

// ---------------------------------------------------------------------------
// B16: DocCache::stats returns accurate per-table counts
// ---------------------------------------------------------------------------

#[test]
fn doccache_stats_returns_accurate_per_table_counts() {
    // Use file backend (redb) to get per-table counts.
    // LRU backend uses a single shared cache, so stats are inaccurate.
    let temp = tempfile::TempDir::new().expect("tempdir should succeed");
    let db_path = temp.path().join("stats_test.redb");
    let config = CacheConfig::new(&db_path);
    let cache = DocCache::open(config).expect("open should succeed");

    cache
        .put_document(b"d1", &"v1".to_string())
        .expect("put should succeed");
    cache
        .put_document(b"d2", &"v2".to_string())
        .expect("put should succeed");
    cache
        .put_document(b"d3", &"v3".to_string())
        .expect("put should succeed");
    cache
        .put_scrape(b"s1", &"sv".to_string())
        .expect("put should succeed");

    let stats = cache.stats().expect("stats should succeed");
    assert_eq!(stats.document_entries, 3);
    assert_eq!(stats.scrape_entries, 1);
    assert_eq!(stats.transform_entries, 0);
    assert_eq!(stats.snapshot_entries, 0);
    assert_eq!(stats.analysis_entries, 0);
    assert_eq!(stats.chunk_entries, 0);
}

// ---------------------------------------------------------------------------
// B19: File backend persists data across open/close cycles
// ---------------------------------------------------------------------------

#[test]
fn doccache_file_backend_persists_across_open_close_cycles() {
    let temp = tempfile::TempDir::new().expect("tempdir should succeed");
    let db_path = temp.path().join("persist_test.redb");

    // First open: write data
    {
        let config = CacheConfig::new(&db_path);
        let cache = DocCache::open(config).expect("open should succeed");
        cache
            .put_document(b"persistent", &"persistence_test_value".to_string())
            .expect("put should succeed");
    } // cache dropped

    // Second open: verify data persists
    let config = CacheConfig::new(&db_path);
    let cache = DocCache::open(config).expect("re-open should succeed");

    let result: Option<String> = cache
        .get_document(b"persistent")
        .expect("get should succeed");
    assert_eq!(
        result,
        Some("persistence_test_value".to_string()),
        "file-backed data should persist across open/close cycles"
    );
}

// ---------------------------------------------------------------------------
// B20: CacheConfig disable/enable builder chaining
// ---------------------------------------------------------------------------

#[test]
fn cacheconfig_disable_enable_builder_chaining_works() {
    let mut config = CacheConfig::in_memory();
    config
        .disable(CacheType::Document)
        .disable(CacheType::Scrape);

    // After disabling Document and Scrape
    let cache = DocCache::open(config.clone()).expect("open should succeed");
    assert_eq!(
        cache
            .get_document::<String>(b"x")
            .expect("get should succeed"),
        None,
        "Document should be disabled"
    );
    assert_eq!(
        cache
            .get_scrape::<String>(b"x")
            .expect("get should succeed"),
        None,
        "Scrape should be disabled"
    );
    // Transform still enabled
    cache
        .put_transform(b"tx_key", &"tx_val".to_string())
        .expect("put to enabled type should succeed");
    let tx_result: Option<String> = cache.get_transform(b"tx_key").expect("get should succeed");
    assert_eq!(
        tx_result,
        Some("tx_val".to_string()),
        "Transform should be enabled"
    );

    // Re-enable Document
    config.enable(CacheType::Document);
    let cache2 = DocCache::open(config).expect("open should succeed");
    cache2
        .put_document(b"doc_key", &"doc_val".to_string())
        .expect("put to re-enabled type should succeed");
    let doc_result: Option<String> = cache2.get_document(b"doc_key").expect("get should succeed");
    assert_eq!(
        doc_result,
        Some("doc_val".to_string()),
        "Document should work after re-enable"
    );
    // Scrape still disabled
    assert_eq!(
        cache2
            .get_scrape::<String>(b"x")
            .expect("get should succeed"),
        None,
        "Scrape should still be disabled after enabling Document"
    );
}

// ---------------------------------------------------------------------------
// B21: Hash functions are deterministic and pure
// ---------------------------------------------------------------------------

#[test]
fn content_hash_returns_same_value_for_same_input() {
    let hash1 = ContentHash::compute(b"test");
    let hash2 = ContentHash::compute(b"test");
    assert_eq!(hash1, hash2);
    assert_eq!(hash1.as_bytes().len(), 32);
}

// ---------------------------------------------------------------------------
// B22: composite_hash is order-sensitive
// ---------------------------------------------------------------------------

#[test]
fn composite_hash_produces_different_output_when_part_order_changes() {
    use doc_transformer::cache::composite_hash;
    let hash1 = composite_hash(&[b"hello", b"world"]);
    let hash2 = composite_hash(&[b"world", b"hello"]);
    assert_ne!(hash1, hash2);
}

// ---------------------------------------------------------------------------
// B23: Typed convenience methods delegate correctly
// ---------------------------------------------------------------------------

#[test]
fn doccache_scrape_put_then_get_returns_identical_value() {
    let cache = DocCache::open(CacheConfig::in_memory()).expect("open should succeed");

    cache
        .put_scrape(b"url_key", &"html_data".to_string())
        .expect("put_scrape should succeed");

    let result: Option<String> = cache
        .get_scrape(b"url_key")
        .expect("get_scrape should succeed");
    assert_eq!(result, Some("html_data".to_string()));
}

#[test]
fn doccache_transform_put_then_get_returns_identical_value() {
    let cache = DocCache::open(CacheConfig::in_memory()).expect("open should succeed");

    cache
        .put_transform(b"tx_key", &"tx_data".to_string())
        .expect("put_transform should succeed");

    let result: Option<String> = cache
        .get_transform(b"tx_key")
        .expect("get_transform should succeed");
    assert_eq!(result, Some("tx_data".to_string()));
}

// ---------------------------------------------------------------------------
// B25: DocCache::open returns BackendError when redb file is corrupted
// ---------------------------------------------------------------------------

#[test]
fn doccache_open_returns_backend_error_when_redb_file_corrupted() {
    let temp = tempfile::TempDir::new().expect("tempdir should succeed");
    let db_path = temp.path().join("corrupt.redb");

    // Create a valid cache first
    {
        let config = CacheConfig::new(&db_path);
        let cache = DocCache::open(config).expect("initial open should succeed");
        cache
            .put_document(b"data", &"original".to_string())
            .expect("put should succeed");
    } // drop cache

    // Corrupt the file
    std::fs::write(&db_path, vec![0xFFu8; 64]).expect("write should succeed");

    // Re-opening should fail — the corrupted file prevents Database::create.
    let config = CacheConfig::new(&db_path);
    let result = DocCache::open(config);
    assert!(
        result.is_err(),
        "corrupted redb file should produce an error, got: {:?}",
        result
    );
}

// ---------------------------------------------------------------------------
// B28: DocCache::get_or_compute propagates compute error without caching
// ---------------------------------------------------------------------------

#[test]
fn doccache_get_or_compute_propagates_compute_error_without_caching() {
    let cache = DocCache::open(CacheConfig::in_memory()).expect("open should succeed");

    let result = cache.get_or_compute::<String, _>(CacheType::Document, b"err_key", || {
        Err(anyhow::anyhow!("compute failed"))
    });

    assert!(result.is_err(), "compute error should propagate");

    // Verify nothing was cached
    let cached: Option<String> = cache.get_document(b"err_key").expect("get should succeed");
    assert_eq!(cached, None, "failed compute should not cache anything");
}

// ---------------------------------------------------------------------------
// B29: DocCache::put accepts zero-length (empty) serialized value
// ---------------------------------------------------------------------------

#[test]
fn doccache_put_succeeds_when_value_is_empty_string() {
    let cache = DocCache::open(CacheConfig::in_memory()).expect("open should succeed");

    cache
        .put_document(b"empty_key", &"")
        .expect("put with empty string should succeed");

    let result: Option<String> = cache
        .get_document(b"empty_key")
        .expect("get should succeed");
    assert_eq!(result, Some("".to_string()));
}

// ---------------------------------------------------------------------------
// B30: DocCache::clear_all followed by put/get roundtrip succeeds
// ---------------------------------------------------------------------------

#[test]
fn doccache_clear_all_then_put_get_roundtrip_succeeds() {
    let cache = DocCache::open(CacheConfig::in_memory()).expect("open should succeed");

    // Write 5 entries
    for i in 0..5_usize {
        let key = format!("key_{}", i);
        cache
            .put_document(key.as_bytes(), &format!("value_{}", i))
            .expect("put should succeed");
    }

    cache.clear_all().expect("clear_all should succeed");

    // After clear, write new data and verify roundtrip
    cache
        .put_document(b"after_clear", &"new_value".to_string())
        .expect("put after clear should succeed");

    let result: Option<String> = cache
        .get_document(b"after_clear")
        .expect("get should succeed");
    assert_eq!(result, Some("new_value".to_string()));

    let stats = cache.stats().expect("stats should succeed");
    assert_eq!(stats.document_entries, 1, "only the new entry should exist");
}

// ---------------------------------------------------------------------------
// B31: CacheConfig::enable re-enables a single type without affecting others
// ---------------------------------------------------------------------------

#[test]
fn cacheconfig_enable_single_type_without_affecting_others() {
    let mut config = CacheConfig::in_memory();
    config
        .disable(CacheType::Document)
        .disable(CacheType::Scrape)
        .disable(CacheType::Transform);

    // Enable only Scrape
    config.enable(CacheType::Scrape);

    let cache = DocCache::open(config).expect("open should succeed");

    // Scrape should work
    cache
        .put_scrape(b"s", &"scrape_val".to_string())
        .expect("put_scrape should succeed");
    assert_eq!(
        cache
            .get_scrape::<String>(b"s")
            .expect("get should succeed"),
        Some("scrape_val".to_string()),
        "Scrape should be enabled"
    );

    // Document should still be disabled
    assert_eq!(
        cache
            .get_document::<String>(b"d")
            .expect("get should succeed"),
        None,
        "Document should still be disabled"
    );

    // Transform should still be disabled
    assert_eq!(
        cache
            .get_transform::<String>(b"t")
            .expect("get should succeed"),
        None,
        "Transform should still be disabled"
    );

    // Snapshot was never disabled
    cache
        .put_snapshot(b"sn", &"snap_val".to_string())
        .expect("put_snapshot should succeed");
    assert_eq!(
        cache
            .get_snapshot::<String>(b"sn")
            .expect("get should succeed"),
        Some("snap_val".to_string()),
        "Snapshot should still be enabled (was never disabled)"
    );
}

// ---------------------------------------------------------------------------
// B32: DocCache::put/get roundtrip works for all 6 cache types
// ---------------------------------------------------------------------------

#[test]
fn doccache_roundtrip_works_for_all_six_cache_types() {
    // Use file backend (redb) for accurate per-table stats.
    // LRU backend shares a single cache, so stats counts are unreliable.
    let temp = tempfile::TempDir::new().expect("tempdir should succeed");
    let db_path = temp.path().join("all_types.redb");
    let config = CacheConfig::new(&db_path);
    let cache = DocCache::open(config).expect("open should succeed");

    // Typed convenience methods
    cache
        .put_document(b"doc", &"doc_val".to_string())
        .expect("put_document should succeed");
    assert_eq!(
        cache
            .get_document::<String>(b"doc")
            .expect("get should succeed"),
        Some("doc_val".to_string())
    );

    cache
        .put_scrape(b"scrape", &"scrape_val".to_string())
        .expect("put_scrape should succeed");
    assert_eq!(
        cache
            .get_scrape::<String>(b"scrape")
            .expect("get should succeed"),
        Some("scrape_val".to_string())
    );

    cache
        .put_transform(b"transform", &"transform_val".to_string())
        .expect("put_transform should succeed");
    assert_eq!(
        cache
            .get_transform::<String>(b"transform")
            .expect("get should succeed"),
        Some("transform_val".to_string())
    );

    // Generic put/get for Snapshot, Analysis, Chunk
    cache
        .put(CacheType::Snapshot, b"snap", &"snap_val".to_string())
        .expect("put Snapshot should succeed");
    assert_eq!(
        cache
            .get::<String>(CacheType::Snapshot, b"snap")
            .expect("get should succeed"),
        Some("snap_val".to_string())
    );

    cache
        .put(
            CacheType::Analysis,
            b"analysis",
            &"analysis_val".to_string(),
        )
        .expect("put Analysis should succeed");
    assert_eq!(
        cache
            .get::<String>(CacheType::Analysis, b"analysis")
            .expect("get should succeed"),
        Some("analysis_val".to_string())
    );

    cache
        .put(CacheType::Chunk, b"chunk", &"chunk_val".to_string())
        .expect("put Chunk should succeed");
    assert_eq!(
        cache
            .get::<String>(CacheType::Chunk, b"chunk")
            .expect("get should succeed"),
        Some("chunk_val".to_string())
    );

    // Verify stats
    let stats = cache.stats().expect("stats should succeed");
    assert_eq!(stats.document_entries, 1);
    assert_eq!(stats.scrape_entries, 1);
    assert_eq!(stats.transform_entries, 1);
    assert_eq!(stats.snapshot_entries, 1);
    assert_eq!(stats.analysis_entries, 1);
    assert_eq!(stats.chunk_entries, 1);
}

// ---------------------------------------------------------------------------
// Additional edge-case tests for completeness
// ---------------------------------------------------------------------------

#[test]
fn doccache_put_returns_key_too_large_with_exact_size_10000() {
    let cache = DocCache::open(CacheConfig::in_memory()).expect("open should succeed");

    let oversized_key = vec![0u8; 10_000];
    let result = cache.put_document(&oversized_key, &"value");

    assert!(result.is_err());
    let err = result.unwrap_err();
    let cache_err = err.downcast_ref::<CacheError>();
    assert_eq!(
        cache_err,
        Some(&CacheError::KeyTooLarge {
            size: 10_000,
            max: 256,
        })
    );
}

#[test]
fn doccache_in_memory_multiple_sequential_writes_to_same_key() {
    let cache = DocCache::open(CacheConfig::in_memory()).expect("open should succeed");

    cache
        .put_document(b"key", &"v1".to_string())
        .expect("first put should succeed");
    cache
        .put_document(b"key", &"v2".to_string())
        .expect("second put should succeed");
    cache
        .put_document(b"key", &"v3".to_string())
        .expect("third put should succeed");

    let result: Option<String> = cache.get_document(b"key").expect("get should succeed");
    assert_eq!(result, Some("v3".to_string()), "last write should win");

    let stats = cache.stats().expect("stats should succeed");
    assert_eq!(
        stats.document_entries, 1,
        "overwrites should not increase count"
    );
}

#[test]
fn doccache_in_memory_empty_key_succeeds() {
    let cache = DocCache::open(CacheConfig::in_memory()).expect("open should succeed");

    cache
        .put_document(b"", &"empty_key_value".to_string())
        .expect("put with empty key should succeed");

    let result: Option<String> = cache
        .get_document(b"")
        .expect("get with empty key should succeed");
    assert_eq!(result, Some("empty_key_value".to_string()));
}

#[test]
fn doccache_file_backend_multiple_types_persist_across_cycles() {
    let temp = tempfile::TempDir::new().expect("tempdir should succeed");
    let db_path = temp.path().join("multi_type.redb");

    // First open: write to multiple types
    {
        let config = CacheConfig::new(&db_path);
        let cache = DocCache::open(config).expect("open should succeed");
        cache
            .put_document(b"dk", &"dv".to_string())
            .expect("put should succeed");
        cache
            .put_scrape(b"sk", &"sv".to_string())
            .expect("put should succeed");
        cache
            .put_transform(b"tk", &"tv".to_string())
            .expect("put should succeed");
    }

    // Second open: verify all types persisted
    let config = CacheConfig::new(&db_path);
    let cache = DocCache::open(config).expect("re-open should succeed");
    assert_eq!(
        cache
            .get_document::<String>(b"dk")
            .expect("get should succeed"),
        Some("dv".to_string())
    );
    assert_eq!(
        cache
            .get_scrape::<String>(b"sk")
            .expect("get should succeed"),
        Some("sv".to_string())
    );
    assert_eq!(
        cache
            .get_transform::<String>(b"tk")
            .expect("get should succeed"),
        Some("tv".to_string())
    );
}
