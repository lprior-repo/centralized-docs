use super::super::config::{CacheConfig, MAX_KEY_SIZE, MAX_VALUE_SIZE};
use super::super::store::DocCache;

#[test]
fn test_key_too_large_returns_error() -> anyhow::Result<()> {
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
fn test_value_too_large_returns_error() -> anyhow::Result<()> {
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
fn test_key_at_max_size_succeeds() -> anyhow::Result<()> {
    let config = CacheConfig::in_memory();
    let cache = DocCache::open(config)?;

    let max_key = vec![0u8; MAX_KEY_SIZE];
    cache.put_document(&max_key, &"value")?;

    let result: Option<String> = cache.get_document(&max_key)?;
    assert_eq!(result, Some("value".to_string()));

    Ok(())
}

#[test]
fn test_scrape_key_size_validation() -> anyhow::Result<()> {
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
fn test_transform_key_size_validation() -> anyhow::Result<()> {
    let config = CacheConfig::in_memory();
    let cache = DocCache::open(config)?;

    let oversized_key = vec![0u8; MAX_KEY_SIZE + 1];
    let result = cache.put_transform(&oversized_key, &"value");

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("key too large"));

    Ok(())
}

// ============================================================
// RED QUEEN ADVERSARIAL TESTS — Limit boundaries
// ============================================================

/// ATTACK 6: Maximum limits — key at exactly 256 bytes, value at exactly 10MB.
#[test]
fn rq_attack_6_maximum_limits_boundary() -> anyhow::Result<()> {
    let config = CacheConfig::in_memory();
    let cache = DocCache::open(config)?;

    let max_key = vec![b'K'; MAX_KEY_SIZE];
    cache.put_document(&max_key, &"max_key_value")?;
    let result: Option<String> = cache.get_document(&max_key)?;
    assert_eq!(result, Some("max_key_value".to_string()));

    let max_value = "V".repeat(MAX_VALUE_SIZE - 8);
    cache.put_document(b"max_val_key", &max_value)?;
    let result: Option<String> = cache.get_document(b"max_val_key")?;
    assert_eq!(result, Some(max_value));

    Ok(())
}

/// ATTACK 7: Oversized inputs — key at 257 bytes, value at 10MB+1.
#[test]
fn rq_attack_7_oversized_inputs_rejected() -> anyhow::Result<()> {
    let config = CacheConfig::in_memory();
    let cache = DocCache::open(config)?;

    let oversized_key = vec![b'X'; MAX_KEY_SIZE + 1];
    let key_result = cache.put_document(&oversized_key, &"val");
    assert!(
        key_result.is_err(),
        "key of {} bytes should be rejected",
        MAX_KEY_SIZE + 1
    );

    let oversized_value = vec![b'Y'; MAX_VALUE_SIZE + 1];
    let val_result = cache.put_document(b"ok_key", &oversized_value);
    assert!(
        val_result.is_err(),
        "value of {} bytes should be rejected",
        MAX_VALUE_SIZE + 1
    );

    Ok(())
}
