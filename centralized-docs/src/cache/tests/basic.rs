use super::super::config::CacheConfig;
use super::super::hash::content_hash;
use super::super::store::DocCache;
use serde::{Deserialize, Serialize};
use tempfile::TempDir;

#[test]
fn test_cache_basic_roundtrip() -> anyhow::Result<()> {
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
fn test_cache_miss_returns_none() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("test.redb");
    let config = CacheConfig::new(&db_path);
    let cache = DocCache::open(config)?;

    let result: Option<String> = cache.get_document(b"nonexistent")?;
    assert!(result.is_none());

    Ok(())
}

#[test]
fn test_cache_struct_value() -> anyhow::Result<()> {
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
fn test_cache_stats() -> anyhow::Result<()> {
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
fn test_clear_all() -> anyhow::Result<()> {
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
fn test_in_memory_cache() -> anyhow::Result<()> {
    let config = CacheConfig::in_memory();
    let cache = DocCache::open(config)?;

    cache.put_document(b"key", &"value")?;
    let result: Option<String> = cache.get_document(b"key")?;
    assert_eq!(result, Some("value".to_string()));

    Ok(())
}

#[test]
fn test_disabled_cache_skips_operations() -> anyhow::Result<()> {
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
