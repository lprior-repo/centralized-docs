use super::super::config::{CacheBackend, CacheConfig, CacheType};
use super::super::store::DocCache;

// ============================================================
// RED QUEEN ADVERSARIAL EDGE CASES — type safety, limits, stats
// ============================================================

/// ATTACK 12: Special bytes in keys — null, high Unicode, non-UTF-8.
#[test]
fn rq_attack_12_special_bytes_in_keys() -> anyhow::Result<()> {
    let config = CacheConfig::in_memory();
    let cache = DocCache::open(config)?;

    let null_key: Vec<u8> = vec![0x00, 0x01, 0x00, 0xFF];
    cache.put_document(&null_key, &"null_bytes_value")?;
    let r: Option<String> = cache.get_document(&null_key)?;
    assert_eq!(r, Some("null_bytes_value".to_string()));

    let unicode_key = "日本語キーñáéíóú".as_bytes();
    cache.put_document(unicode_key, &"unicode_value")?;
    let r2: Option<String> = cache.get_document(unicode_key)?;
    assert_eq!(r2, Some("unicode_value".to_string()));

    let non_utf8_key: Vec<u8> = vec![0x80, 0xFF, 0xFE, 0xFD, 0xFC];
    cache.put_document(&non_utf8_key, &42i64)?;
    let r3: Option<i64> = cache.get_document(&non_utf8_key)?;
    assert_eq!(r3, Some(42));

    Ok(())
}

/// ATTACK 13: Store different types with same key — put String then get as i64.
///
/// NOTE: bincode is a type-erased binary format — it does NOT encode type tags.
/// Deserializing a String as i64 silently produces garbage instead of an error.
/// This is a known trade-off: bincode is ~5-10x faster than JSON but sacrifices
/// type-mismatch detection. Callers MUST use consistent types per cache key.
#[test]
fn rq_attack_13_type_mismatch_same_key() -> anyhow::Result<()> {
    let config = CacheConfig::in_memory();
    let cache = DocCache::open(config)?;

    let key = b"polymorphic_key";
    cache.put_document(key, &"string_value".to_string())?;

    let wrong_type_result: Option<i64> = cache.get_document(key)?;
    assert!(
        wrong_type_result.is_some(),
        "bincode does not encode type info — type mismatch produces garbage, not None"
    );

    let string_result: Option<String> = cache.get_document(key)?;
    assert_eq!(string_result, Some("string_value".to_string()));

    Ok(())
}

/// ATTACK 14: Stats accuracy — insert N items, verify stats returns N.
#[test]
fn rq_attack_14_stats_accuracy_large_n() -> anyhow::Result<()> {
    let config = CacheConfig::in_memory();
    let cache = DocCache::open(config)?;

    const DOC_N: u32 = 200;
    const SCRAPE_N: u32 = 150;
    const TRANSFORM_N: u32 = 100;

    for i in 0..DOC_N {
        cache.put_document(format!("d_{i}").as_bytes(), &format!("v_{i}"))?;
    }
    for i in 0..SCRAPE_N {
        cache.put_scrape(format!("s_{i}").as_bytes(), &format!("sv_{i}"))?;
    }
    for i in 0..TRANSFORM_N {
        cache.put_transform(format!("t_{i}").as_bytes(), &format!("tv_{i}"))?;
    }

    let stats = cache.stats()?;
    assert_eq!(
        stats.document_entries,
        u64::from(DOC_N),
        "document count mismatch"
    );
    assert_eq!(
        stats.scrape_entries,
        u64::from(SCRAPE_N),
        "scrape count mismatch"
    );
    assert_eq!(
        stats.transform_entries,
        u64::from(TRANSFORM_N),
        "transform count mismatch"
    );

    cache.clear_all()?;
    let empty_stats = cache.stats()?;
    assert_eq!(empty_stats.document_entries, 0);
    assert_eq!(empty_stats.scrape_entries, 0);
    assert_eq!(empty_stats.transform_entries, 0);

    Ok(())
}

/// ATTACK 15: All tables disabled — `get_or_compute` should still invoke compute.
#[test]
fn rq_attack_15_all_tables_disabled_get_or_compute_still_works() -> anyhow::Result<()> {
    let config = CacheConfig {
        backend: CacheBackend::Memory,
        cache_document_content: false,
        cache_scrape_results: false,
        cache_transforms: false,
    };
    let cache = DocCache::open(config)?;

    let mut call_count = 0;
    let result1 = cache.get_or_compute(CacheType::Document, b"disabled_key", || {
        call_count += 1;
        Ok("computed_despite_disabled".to_string())
    })?;
    assert_eq!(result1, "computed_despite_disabled");
    assert_eq!(call_count, 1);

    let result2 = cache.get_or_compute(CacheType::Document, b"disabled_key", || {
        call_count += 1;
        Ok("second_compute".to_string())
    })?;
    // After DEFECT-004 fix: in_flight entries persist (not removed after compute).
    // The second call finds the already-set OnceLock and returns the first result,
    // so compute is NOT invoked again. This is correct — deduplication still works
    // even when caching is disabled; the value is just not persisted to redb.
    assert_eq!(result2, "computed_despite_disabled");
    assert_eq!(
        call_count, 1,
        "in_flight deduplication still works with caching disabled"
    );

    Ok(())
}
