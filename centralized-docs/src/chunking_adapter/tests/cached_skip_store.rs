//! Cached orchestration tests: skip chunker, store, write (B16-B18).

use std::fs;

use super::*;
use crate::cache::{CacheType, DocCache};

#[test]
fn chunk_all_cached_skips_contextual_chunker_for_unchanged_files() {
    let config = crate::cache::CacheConfig::in_memory();
    let cache = DocCache::open(config).unwrap();
    let config_hash = compute_chunker_config_hash(1_048_576);
    let temp_dir = tempfile::TempDir::new().unwrap();

    let analyses = vec![
        make_analysis(
            "a.md",
            "A",
            "# Hello World\nSome content for doc A",
            "concept",
        ),
        make_analysis("b.md", "B", "# Another Doc\nContent for B", "concept"),
    ];
    let link_map = make_link_map(&analyses);

    let cached_a = vec![
        make_test_chunk(
            "concept/a#0",
            "concept/a",
            0,
            contextual_chunker::ChunkLevel::Summary,
            "A summary",
            Some("A"),
        ),
        make_test_chunk(
            "concept/a#1",
            "concept/a",
            1,
            contextual_chunker::ChunkLevel::Standard,
            "A body 1",
            Some("A1"),
        ),
        make_test_chunk(
            "concept/a#2",
            "concept/a",
            2,
            contextual_chunker::ChunkLevel::Standard,
            "A body 2",
            Some("A2"),
        ),
    ];
    let key_a = chunk_cache_key("a.md", analyses[0].content.as_ref(), &config_hash);
    cache
        .put(CacheType::Chunk, key_a.as_bytes(), &cached_a)
        .unwrap();

    let result = chunk_all_cached(
        &analyses,
        &link_map,
        temp_dir.path(),
        1_048_576,
        &cache,
        config_hash,
    )
    .unwrap();
    assert!(result.chunks_metadata.len() >= 3);
    assert_eq!(result.chunks_metadata[0].doc_id, "concept/a");
    assert_eq!(result.chunks_metadata[1].doc_id, "concept/a");
    assert_eq!(result.chunks_metadata[2].doc_id, "concept/a");

    let b_chunks: Vec<&Chunk> = result
        .chunks_metadata
        .iter()
        .filter(|c| c.doc_id.contains('b'))
        .collect();
    assert!(!b_chunks.is_empty());
}

#[test]
fn chunk_all_cached_stores_fresh_chunks_in_cache_for_changed_files() {
    let config = crate::cache::CacheConfig::in_memory();
    let cache = DocCache::open(config).unwrap();
    let config_hash = compute_chunker_config_hash(1_048_576);
    let temp_dir = tempfile::TempDir::new().unwrap();

    let analyses = vec![make_analysis(
        "fresh.md",
        "Fresh Doc",
        "# Fresh\nSome content for fresh doc",
        "concept",
    )];
    let link_map = make_link_map(&analyses);

    let result = chunk_all_cached(
        &analyses,
        &link_map,
        temp_dir.path(),
        1_048_576,
        &cache,
        config_hash,
    )
    .unwrap();

    let key = chunk_cache_key("fresh.md", analyses[0].content.as_ref(), &config_hash);
    let cached: Option<Vec<Chunk>> = cache.get(CacheType::Chunk, key.as_bytes()).unwrap();
    assert!(cached.is_some());
    assert_eq!(cached.unwrap().len(), result.chunks_metadata.len());
}

#[test]
fn chunk_all_cached_writes_all_chunk_files_to_disk_for_mixed_cache_hits() {
    let config = crate::cache::CacheConfig::in_memory();
    let cache = DocCache::open(config).unwrap();
    let config_hash = compute_chunker_config_hash(1_048_576);
    let temp_dir = tempfile::TempDir::new().unwrap();

    let analyses = vec![
        make_analysis("a.md", "A", "# Doc A\nContent A", "concept"),
        make_analysis("b.md", "B", "# Doc B\nContent B", "concept"),
        make_analysis("c.md", "C", "# Doc C\nContent C", "concept"),
    ];
    let link_map = make_link_map(&analyses);

    for (name, doc_id) in &[("a.md", "concept/a"), ("b.md", "concept/b")] {
        let cached = vec![
            make_test_chunk(
                &format!("{doc_id}#0"),
                *doc_id,
                0,
                contextual_chunker::ChunkLevel::Standard,
                "body0",
                Some("H0"),
            ),
            make_test_chunk(
                &format!("{doc_id}#1"),
                *doc_id,
                1,
                contextual_chunker::ChunkLevel::Standard,
                "body1",
                Some("H1"),
            ),
        ];
        let matching = analyses.iter().find(|a| a.source_path == *name).unwrap();
        let key = chunk_cache_key(name, matching.content.as_ref(), &config_hash);
        cache
            .put(CacheType::Chunk, key.as_bytes(), &cached)
            .unwrap();
    }

    let result = chunk_all_cached(
        &analyses,
        &link_map,
        temp_dir.path(),
        1_048_576,
        &cache,
        config_hash,
    )
    .unwrap();

    let chunks_dir = temp_dir.path().join("chunks");
    let md_files: Vec<_> = fs::read_dir(&chunks_dir)
        .unwrap()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        .collect();
    assert_eq!(md_files.len(), result.chunks_metadata.len());
}
