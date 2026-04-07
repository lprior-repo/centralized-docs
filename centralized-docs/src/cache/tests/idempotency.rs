use super::*;
use serde::{Deserialize, Serialize};
use tempfile::TempDir;

#[test]
fn test_cache_open_idempotent_single_open_close_cycle() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("idempotent_single.redb");
    let config = CacheConfig::new(&db_path);
    let cache = DocCache::open(config.clone())?;
    cache.put_document(b"key1", &"value1")?;
    drop(cache);
    let cache2 = DocCache::open(config)?;
    let retrieved: Option<String> = cache2.get_document(b"key1")?;
    assert_eq!(
        retrieved,
        Some("value1".to_string()),
        "Data should persist across open/close"
    );
    Ok(())
}

#[test]
fn test_cache_open_idempotent_ten_open_cycles() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("idempotent_10cycles.redb");
    for cycle in 0..10 {
        let config = CacheConfig::new(&db_path);
        let cache = DocCache::open(config)?;
        let key = format!("key_cycle_{cycle}");
        let value = format!("value_cycle_{cycle}");
        cache.put_document(key.as_bytes(), &value)?;
        let retrieved: Option<String> = cache.get_document(key.as_bytes())?;
        assert_eq!(retrieved, Some(value), "Data visible within cycle {cycle}");
    }
    let config = CacheConfig::new(&db_path);
    let final_cache = DocCache::open(config)?;
    for cycle in 0..10 {
        let key = format!("key_cycle_{cycle}");
        let expected = format!("value_cycle_{cycle}");
        let retrieved: Option<String> = final_cache.get_document(key.as_bytes())?;
        assert_eq!(
            retrieved,
            Some(expected),
            "Data from cycle {cycle} should persist"
        );
    }
    Ok(())
}

#[test]
fn test_cache_open_idempotent_hundred_open_cycles() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("idempotent_100cycles.redb");
    for cycle in 0..100 {
        let config = CacheConfig::new(&db_path);
        let cache = DocCache::open(config)?;
        let key = format!("stress_key_{cycle}");
        let value = format!("stress_value_{cycle}");
        cache.put_document(key.as_bytes(), &value)?;
    }
    let config = CacheConfig::new(&db_path);
    let final_cache = DocCache::open(config)?;
    for cycle in 0..100 {
        let key = format!("stress_key_{cycle}");
        let expected = format!("stress_value_{cycle}");
        let retrieved: Option<String> = final_cache.get_document(key.as_bytes())?;
        assert_eq!(
            retrieved,
            Some(expected),
            "Data from cycle {cycle} should persist"
        );
    }
    Ok(())
}

#[test]
fn test_cache_open_idempotent_consecutive_opens_without_close() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("consecutive_opens.redb");
    let config = CacheConfig::new(&db_path);
    {
        let cache1 = DocCache::open(config.clone())?;
        cache1.put_document(b"persistent_key", &"persistent_value")?;
    }
    {
        let cache2 = DocCache::open(config.clone())?;
        let retrieved: Option<String> = cache2.get_document(b"persistent_key")?;
        assert_eq!(retrieved, Some("persistent_value".to_string()));
        cache2.put_document(b"key2", &"value2")?;
    }
    {
        let cache3 = DocCache::open(config)?;
        assert_eq!(
            cache3.get_document(b"persistent_key")?,
            Some("persistent_value".to_string())
        );
        assert_eq!(cache3.get_document(b"key2")?, Some("value2".to_string()));
    }
    Ok(())
}

#[test]
fn test_cache_open_idempotent_all_table_types() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("all_tables_idempotent.redb");
    {
        let config = CacheConfig::new(&db_path);
        let cache = DocCache::open(config)?;
        cache.put_document(b"doc_key", &"doc_value")?;
        cache.put_scrape(b"https://example.com", &"scraped_html")?;
        cache.put_transform(b"transform_key", &"transformed_content")?;
        cache.put_snapshot(b"/path/to/file.md", &"snapshot_data")?;
    }
    {
        let config = CacheConfig::new(&db_path);
        let cache = DocCache::open(config)?;
        assert_eq!(
            cache.get_document(b"doc_key")?,
            Some("doc_value".to_string())
        );
        assert_eq!(
            cache.get_scrape(b"https://example.com")?,
            Some("scraped_html".to_string())
        );
        assert_eq!(
            cache.get_transform(b"transform_key")?,
            Some("transformed_content".to_string())
        );
        assert_eq!(
            cache.get_snapshot(b"/path/to/file.md")?,
            Some("snapshot_data".to_string())
        );
    }
    Ok(())
}

#[test]
fn test_cache_open_idempotent_data_integrity() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("data_integrity.redb");

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct ComplexData {
        name: String,
        items: Vec<i32>,
        nested: NestedData,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct NestedData {
        value: f64,
        flag: bool,
    }

    let original = ComplexData {
        name: "test".to_string(),
        items: vec![1, 2, 3, 4, 5],
        nested: NestedData {
            value: std::f64::consts::PI,
            flag: true,
        },
    };
    {
        let config = CacheConfig::new(&db_path);
        let cache = DocCache::open(config)?;
        cache.put_document(b"complex", &original)?;
    }
    for _ in 0..50 {
        let config = CacheConfig::new(&db_path);
        let cache = DocCache::open(config)?;
        let retrieved: Option<ComplexData> = cache.get_document(b"complex")?;
        assert_eq!(
            retrieved,
            Some(original.clone()),
            "Data integrity check failed"
        );
    }
    Ok(())
}
