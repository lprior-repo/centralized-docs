//! Cache key and partition tests (B01-B07).

use super::*;
use crate::cache::{CacheType, DocCache};

#[test]
fn compute_chunker_config_hash_returns_identical_hash_for_same_max_bytes() {
    let hash1 = compute_chunker_config_hash(1_048_576);
    let hash2 = compute_chunker_config_hash(1_048_576);
    assert_eq!(hash1, hash2);
}

#[test]
fn compute_chunker_config_hash_returns_different_hash_when_max_bytes_differs() {
    let hash_a = compute_chunker_config_hash(1_048_576);
    let hash_b = compute_chunker_config_hash(2_097_152);
    assert_ne!(hash_a, hash_b);
}

#[test]
fn chunk_cache_key_returns_identical_hash_for_identical_triple() {
    let config_hash = compute_chunker_config_hash(1024);
    let key1 = chunk_cache_key("concept/general/test.md", "file body", &config_hash);
    let key2 = chunk_cache_key("concept/general/test.md", "file body", &config_hash);
    assert_eq!(key1, key2);
}

#[test]
fn chunk_cache_key_returns_different_hash_when_source_path_differs() {
    let config_hash = compute_chunker_config_hash(1024);
    let key1 = chunk_cache_key("a.md", "content", &config_hash);
    let key2 = chunk_cache_key("b.md", "content", &config_hash);
    assert_ne!(key1, key2);
}

#[test]
fn chunk_cache_key_returns_different_hash_when_content_differs() {
    let config_hash = compute_chunker_config_hash(1024);
    let key1 = chunk_cache_key("test.md", "body A", &config_hash);
    let key2 = chunk_cache_key("test.md", "body B", &config_hash);
    assert_ne!(key1, key2);
}

#[test]
fn chunk_cache_key_returns_different_hash_when_config_hash_differs() {
    let config_a = compute_chunker_config_hash(1024);
    let config_b = compute_chunker_config_hash(2048);
    let key1 = chunk_cache_key("test.md", "content", &config_a);
    let key2 = chunk_cache_key("test.md", "content", &config_b);
    assert_ne!(key1, key2);
}

#[test]
fn chunk_cache_key_returns_different_hash_for_same_content_at_different_paths() {
    let config_hash = compute_chunker_config_hash(1024);
    let key1 = chunk_cache_key("dir/a.md", "same content", &config_hash);
    let key2 = chunk_cache_key("dir/b.md", "same content", &config_hash);
    assert_ne!(key1, key2);
}

#[test]
fn partition_returns_all_changed_when_cache_is_empty() {
    let config = crate::cache::CacheConfig::in_memory();
    let cache = DocCache::open(config).unwrap();
    let config_hash = compute_chunker_config_hash(1024);

    let analyses = vec![
        make_analysis("a.md", "A", "content a", "concept"),
        make_analysis("b.md", "B", "content b", "concept"),
        make_analysis("c.md", "C", "content c", "concept"),
    ];

    let (unchanged, changed) = partition_by_cache_status(&analyses, &cache, &config_hash).unwrap();

    assert!(unchanged.is_empty(), "unchanged should be empty");
    assert_eq!(changed.len(), 3, "all 3 should be changed");
}

#[test]
fn partition_returns_unchanged_when_cache_key_matches_existing_entry() {
    let config = crate::cache::CacheConfig::in_memory();
    let cache = DocCache::open(config).unwrap();
    let config_hash = compute_chunker_config_hash(1024);

    let cached_chunks = vec![
        make_test_chunk(
            "doc#0",
            "doc",
            0,
            contextual_chunker::ChunkLevel::Standard,
            "body 0",
            Some("H0"),
        ),
        make_test_chunk(
            "doc#1",
            "doc",
            1,
            contextual_chunker::ChunkLevel::Standard,
            "body 1",
            Some("H1"),
        ),
    ];

    let key = chunk_cache_key("a.md", "content-a", &config_hash);
    cache
        .put(CacheType::Chunk, key.as_bytes(), &cached_chunks)
        .unwrap();

    let analyses = vec![make_analysis("a.md", "A", "content-a", "concept")];
    let (unchanged, changed) = partition_by_cache_status(&analyses, &cache, &config_hash).unwrap();

    assert_eq!(unchanged.len(), 1, "should have 1 unchanged");
    assert_eq!(unchanged[0].1.len(), 2, "should have 2 cached chunks");
    assert!(changed.is_empty(), "changed should be empty");
}

#[test]
fn partition_downgrades_deserialization_failure_to_changed() {
    let config = crate::cache::CacheConfig::in_memory();
    let cache = DocCache::open(config).unwrap();
    let config_hash = compute_chunker_config_hash(1024);

    let key = chunk_cache_key("deser-fail.md", "some-content", &config_hash);
    cache
        .put(CacheType::Chunk, key.as_bytes(), b"NOT VALID JSON")
        .unwrap();

    let analyses = vec![make_analysis(
        "deser-fail.md",
        "F",
        "some-content",
        "concept",
    )];
    let (unchanged, changed) = partition_by_cache_status(&analyses, &cache, &config_hash).unwrap();

    assert!(unchanged.is_empty(), "unchanged should be empty");
    assert_eq!(
        changed.len(),
        1,
        "deser failure should downgrade to changed"
    );
}

#[test]
fn partition_preserves_analysis_order_in_both_vectors() {
    let config = crate::cache::CacheConfig::in_memory();
    let cache = DocCache::open(config).unwrap();
    let config_hash = compute_chunker_config_hash(1024);

    let key_b = chunk_cache_key("b.md", "content-b", &config_hash);
    cache
        .put(
            CacheType::Chunk,
            key_b.as_bytes(),
            &vec![make_test_chunk(
                "b#0",
                "b",
                0,
                contextual_chunker::ChunkLevel::Standard,
                "b0",
                None,
            )],
        )
        .unwrap();

    let key_d = chunk_cache_key("d.md", "content-d", &config_hash);
    cache
        .put(
            CacheType::Chunk,
            key_d.as_bytes(),
            &vec![make_test_chunk(
                "d#0",
                "d",
                0,
                contextual_chunker::ChunkLevel::Standard,
                "d0",
                None,
            )],
        )
        .unwrap();

    let analyses = vec![
        make_analysis("a.md", "A", "content-a", "concept"),
        make_analysis("b.md", "B", "content-b", "concept"),
        make_analysis("c.md", "C", "content-c", "concept"),
        make_analysis("d.md", "D", "content-d", "concept"),
    ];

    let (unchanged, changed) = partition_by_cache_status(&analyses, &cache, &config_hash).unwrap();

    assert_eq!(unchanged[0].0.source_path, "b.md");
    assert_eq!(unchanged[1].0.source_path, "d.md");
    assert_eq!(changed[0].source_path, "a.md");
    assert_eq!(changed[1].source_path, "c.md");
}
