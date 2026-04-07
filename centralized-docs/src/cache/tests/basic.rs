use super::*;
use serde::{Deserialize, Serialize};
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
    assert_eq!(stats.analysis_entries, 0);
    assert_eq!(stats.chunk_entries, 0);
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
}
