//! Cached orchestration tests: counters, order, equivalence (B19-B22).

use super::*;
use crate::analyze::Analysis;
use crate::cache::{CacheType, DocCache};

#[test]
fn chunk_all_cached_returns_accurate_counters_for_mixed_results() {
    let config = crate::cache::CacheConfig::in_memory();
    let cache = DocCache::open(config).unwrap();
    let config_hash = compute_chunker_config_hash(1_048_576);

    let temp_dir = tempfile::TempDir::new().unwrap();

    let analyses = vec![
        make_analysis("a.md", "A", "# Doc A\nContent A", "concept"),
        make_analysis(
            "b.md",
            "B",
            "# Doc B\nContent B here with more text",
            "concept",
        ),
    ];
    let link_map = make_link_map(&analyses);

    let cached_a = vec![
        make_test_chunk(
            "concept/a#0",
            "concept/a",
            0,
            contextual_chunker::ChunkLevel::Standard,
            "body0",
            Some("H0"),
        ),
        make_test_chunk(
            "concept/a#1",
            "concept/a",
            1,
            contextual_chunker::ChunkLevel::Standard,
            "body1",
            Some("H1"),
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

    assert_eq!(result.document_count, 2, "document_count should be 2");

    let total = result.summary_chunks + result.standard_chunks + result.detailed_chunks;
    assert_eq!(
        total, result.total_chunks,
        "total_chunks should equal sum of level counts"
    );
    assert_eq!(
        result.total_chunks,
        result.chunks_metadata.len(),
        "total_chunks should equal chunks_metadata length"
    );
    assert!(
        result.standard_chunks >= 2,
        "should have at least 2 standard chunks from A"
    );
}

#[test]
fn chunk_all_cached_preserves_analysis_order_in_chunks_metadata() {
    let config = crate::cache::CacheConfig::in_memory();
    let cache = DocCache::open(config).unwrap();
    let config_hash = compute_chunker_config_hash(1_048_576);

    let temp_dir = tempfile::TempDir::new().unwrap();

    let analyses = vec![
        make_analysis("a.md", "A", "# Doc A\nContent A here", "concept"),
        make_analysis("b.md", "B", "# Doc B\nContent B here", "concept"),
        make_analysis("c.md", "C", "# Doc C\nContent C here", "concept"),
    ];
    let link_map = make_link_map(&analyses);

    let cached_a = vec![
        make_test_chunk(
            "concept/a#0",
            "concept/a",
            0,
            contextual_chunker::ChunkLevel::Standard,
            "A0",
            Some("H"),
        ),
        make_test_chunk(
            "concept/a#1",
            "concept/a",
            1,
            contextual_chunker::ChunkLevel::Standard,
            "A1",
            Some("H"),
        ),
    ];
    let key_a = chunk_cache_key("a.md", analyses[0].content.as_ref(), &config_hash);
    cache
        .put(CacheType::Chunk, key_a.as_bytes(), &cached_a)
        .unwrap();

    let cached_c = vec![make_test_chunk(
        "concept/c#0",
        "concept/c",
        0,
        contextual_chunker::ChunkLevel::Standard,
        "C0",
        Some("H"),
    )];
    let key_c = chunk_cache_key("c.md", analyses[2].content.as_ref(), &config_hash);
    cache
        .put(CacheType::Chunk, key_c.as_bytes(), &cached_c)
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

    let a_count = result
        .chunks_metadata
        .iter()
        .filter(|c| c.doc_id == "concept/a")
        .count();
    let b_count = result
        .chunks_metadata
        .iter()
        .filter(|c| c.doc_id.contains('b'))
        .count();
    let c_count = result
        .chunks_metadata
        .iter()
        .filter(|c| c.doc_id == "concept/c")
        .count();

    assert_eq!(a_count, 2, "A should have 2 cached chunks");
    assert_eq!(c_count, 1, "C should have 1 cached chunk");
    assert!(b_count > 0, "B should have freshly computed chunks");

    let a_end = a_count;
    let b_end = a_end + b_count;
    for i in 0..a_end {
        assert_eq!(
            result.chunks_metadata[i].doc_id, "concept/a",
            "chunk {i} should belong to A"
        );
    }
    for i in a_end..b_end {
        assert!(
            result.chunks_metadata[i].doc_id.contains('b'),
            "chunk {} should belong to B, got {}",
            i,
            result.chunks_metadata[i].doc_id
        );
    }
    for i in b_end..result.chunks_metadata.len() {
        assert_eq!(
            result.chunks_metadata[i].doc_id, "concept/c",
            "chunk {i} should belong to C"
        );
    }
}

#[test]
fn chunk_all_cached_sets_document_count_equal_to_analyses_len() {
    let config = crate::cache::CacheConfig::in_memory();
    let cache = DocCache::open(config).unwrap();
    let config_hash = compute_chunker_config_hash(1_048_576);

    let temp_dir = tempfile::TempDir::new().unwrap();

    let analyses: Vec<Analysis> = (0..5)
        .map(|i| {
            make_analysis(
                &format!("doc{i}.md"),
                &format!("Doc {i}"),
                &format!("# Doc {i}\nContent {i}"),
                "concept",
            )
        })
        .collect();
    let link_map = make_link_map(&analyses);

    for analysis in &analyses {
        let key = chunk_cache_key(
            &analysis.source_path,
            analysis.content.as_ref(),
            &config_hash,
        );
        let chunks = vec![make_test_chunk(
            &format!(
                "concept/doc{}#0",
                analysis.source_path.chars().nth(3).unwrap()
            ),
            "concept/doc",
            0,
            contextual_chunker::ChunkLevel::Standard,
            "body",
            None,
        )];
        cache
            .put(CacheType::Chunk, key.as_bytes(), &chunks)
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
    assert_eq!(result.document_count, 5);
}

#[test]
fn chunk_all_cached_produces_identical_result_as_chunk_all_for_same_inputs() {
    let config = crate::cache::CacheConfig::in_memory();
    let cache = DocCache::open(config).unwrap();
    let config_hash = compute_chunker_config_hash(1_048_576);

    let temp_dir_cached = tempfile::TempDir::new().unwrap();
    let temp_dir_uncached = tempfile::TempDir::new().unwrap();

    let analyses = vec![
        make_analysis("a.md", "Doc A", "# Doc A\nContent A", "concept"),
        make_analysis("b.md", "Doc B", "# Doc B\nContent B", "concept"),
        make_analysis("c.md", "Doc C", "# Doc C\nContent C", "concept"),
    ];
    let link_map = make_link_map(&analyses);

    let uncached = chunk_all(&analyses, &link_map, temp_dir_uncached.path(), 1_048_576).unwrap();
    let cached = chunk_all_cached(
        &analyses,
        &link_map,
        temp_dir_cached.path(),
        1_048_576,
        &cache,
        config_hash,
    )
    .unwrap();

    assert_eq!(cached.total_chunks, uncached.total_chunks);
    assert_eq!(cached.document_count, uncached.document_count);
    assert_eq!(cached.summary_chunks, uncached.summary_chunks);
    assert_eq!(cached.standard_chunks, uncached.standard_chunks);
    assert_eq!(cached.detailed_chunks, uncached.detailed_chunks);

    for (c, u) in cached
        .chunks_metadata
        .iter()
        .zip(uncached.chunks_metadata.iter())
    {
        assert_eq!(c.chunk_id, u.chunk_id, "chunk_id mismatch");
        assert_eq!(c.doc_id, u.doc_id, "doc_id mismatch");
        assert_eq!(c.chunk_index, u.chunk_index, "chunk_index mismatch");
        assert_eq!(c.content, u.content, "content mismatch");
        assert_eq!(c.chunk_level, u.chunk_level, "chunk_level mismatch");
        assert_eq!(c.chunk_type, u.chunk_type, "chunk_type mismatch");
    }
}
