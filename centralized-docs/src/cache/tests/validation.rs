use super::*;
use tempfile::TempDir;

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
    assert!(err_msg.contains("key too large"), "Error: {err_msg}");
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
    assert!(err_msg.contains("value too large"), "Error: {err_msg}");
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
fn test_builder_pattern_disable() -> Result<()> {
    let mut config = CacheConfig::in_memory();
    config
        .disable(CacheType::Document)
        .disable(CacheType::Scrape);
    let cache = DocCache::open(config)?;
    cache.put_document(b"key", &"value")?;
    let result: Option<String> = cache.get_document(b"key")?;
    assert!(result.is_none());
    cache.put_scrape(b"key", &"value")?;
    let result: Option<String> = cache.get_scrape(b"key")?;
    assert!(result.is_none());
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
        content_hash(
            std::path::Path::new("/foo/bar.md")
                .to_string_lossy()
                .as_bytes()
        )
    );
}
