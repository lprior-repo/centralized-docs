//! Cache key edge cases, equivalence tests (B24-B27).

use super::*;
use crate::cache::{CacheType, DocCache};

#[test]
fn chunk_cache_key_returns_valid_hash_for_empty_source_path() {
    let config_hash = compute_chunker_config_hash(1024);
    let key1 = chunk_cache_key("", "some content", &config_hash);
    let key2 = chunk_cache_key("", "some content", &config_hash);
    assert_eq!(key1, key2, "empty path should produce deterministic hash");
    assert_eq!(key1.as_bytes().len(), 32, "should be valid SHA-256");
}

#[test]
fn chunk_cache_key_returns_valid_hash_for_empty_content() {
    let config_hash = compute_chunker_config_hash(1024);
    let key1 = chunk_cache_key("test.md", "", &config_hash);
    let key2 = chunk_cache_key("test.md", "", &config_hash);
    assert_eq!(
        key1, key2,
        "empty content should produce deterministic hash"
    );
    assert_eq!(key1.as_bytes().len(), 32);
}

#[test]
fn partition_returns_all_unchanged_when_all_analyses_cached() {
    let config = crate::cache::CacheConfig::in_memory();
    let cache = DocCache::open(config).unwrap();
    let config_hash = compute_chunker_config_hash(1024);

    let analyses = vec![
        make_analysis("a.md", "A", "content-a", "concept"),
        make_analysis("b.md", "B", "content-b", "concept"),
        make_analysis("c.md", "C", "content-c", "concept"),
    ];

    for (i, analysis) in analyses.iter().enumerate() {
        let key = chunk_cache_key(
            &analysis.source_path,
            analysis.content.as_ref(),
            &config_hash,
        );
        let chunks = vec![make_test_chunk(
            &format!("d{i}#0"),
            &format!("d{i}"),
            0,
            contextual_chunker::ChunkLevel::Standard,
            "body",
            None,
        )];
        cache
            .put(CacheType::Chunk, key.as_bytes(), &chunks)
            .unwrap();
    }

    let (unchanged, changed) = partition_by_cache_status(&analyses, &cache, &config_hash).unwrap();
    assert_eq!(unchanged.len(), 3, "all 3 should be unchanged");
    assert!(changed.is_empty(), "changed should be empty");
    assert_eq!(unchanged[0].1.len(), 1, "A should have 1 chunk");
    assert_eq!(unchanged[1].1.len(), 1, "B should have 1 chunk");
    assert_eq!(unchanged[2].1.len(), 1, "C should have 1 chunk");
}

#[test]
fn partition_returns_empty_vectors_when_analyses_slice_is_empty() {
    let config = crate::cache::CacheConfig::in_memory();
    let cache = DocCache::open(config).unwrap();
    let config_hash = compute_chunker_config_hash(1024);

    let (unchanged, changed) = partition_by_cache_status(&[], &cache, &config_hash).unwrap();
    assert!(unchanged.is_empty());
    assert!(changed.is_empty());
}
