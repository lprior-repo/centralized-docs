//! Boundary and full pipeline tests (B28-B33).

use std::collections::HashMap;
use std::fs;

use super::*;
use crate::cache::DocCache;

#[test]
fn write_chunk_file_omits_heading_field_when_heading_is_none() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let chunks_dir = temp_dir.path().join("chunks");
    fs::create_dir_all(&chunks_dir).unwrap();

    let chunk = Chunk {
        heading: None,
        summary: "sum".to_string(),
        content: "body".to_string(),
        ..make_test_chunk(
            "doc#0",
            "doc",
            0,
            contextual_chunker::ChunkLevel::Standard,
            "body",
            None,
        )
    };

    write_chunk_file(&chunk, &chunks_dir).unwrap();

    let file_path = chunks_dir.join("doc-0-standard.md");
    let contents = fs::read_to_string(file_path).unwrap();
    assert!(
        !contents.contains("heading:"),
        "frontmatter should NOT contain 'heading:' field. Contents: {contents}"
    );
    assert!(
        contents.contains("body"),
        "should contain body after frontmatter"
    );
}

#[test]
fn write_chunk_file_creates_md_with_empty_body_when_content_is_empty() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let chunks_dir = temp_dir.path().join("chunks");
    fs::create_dir_all(&chunks_dir).unwrap();

    let chunk = Chunk {
        content: String::new(),
        heading: Some("H".to_string()),
        summary: "sum".to_string(),
        ..make_test_chunk(
            "doc#0",
            "doc",
            0,
            contextual_chunker::ChunkLevel::Standard,
            "",
            Some("H"),
        )
    };

    write_chunk_file(&chunk, &chunks_dir).unwrap();

    let file_path = chunks_dir.join("doc-0-standard.md");
    let contents = fs::read_to_string(file_path).unwrap();
    assert!(contents.starts_with("---"), "should start with frontmatter");
    assert!(contents.contains("---\n"), "should have closing ---");
    assert!(
        !contents.is_empty(),
        "file should have content (frontmatter)"
    );
}

#[test]
fn write_chunk_file_escapes_yaml_special_characters_in_summary_and_heading() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let chunks_dir = temp_dir.path().join("chunks");
    fs::create_dir_all(&chunks_dir).unwrap();

    let chunk = Chunk {
        summary: "line1\nline2".to_string(),
        heading: Some("He said \"hello\" and: goodbye".to_string()),
        content: "body".to_string(),
        ..make_test_chunk(
            "doc#0",
            "doc",
            0,
            contextual_chunker::ChunkLevel::Standard,
            "body",
            Some("H"),
        )
    };

    write_chunk_file(&chunk, &chunks_dir).unwrap();

    let file_path = chunks_dir.join("doc-0-standard.md");
    let contents = fs::read_to_string(file_path).unwrap();
    assert!(
        contents.contains("line1 line2"),
        "newlines in summary should be escaped"
    );
    assert!(
        contents.contains(r#"He said \"hello\" and: goodbye"#),
        "quotes in heading should be escaped"
    );
}

#[test]
fn chunk_all_cached_accepts_document_at_exact_size_limit() {
    let config = crate::cache::CacheConfig::in_memory();
    let cache = DocCache::open(config).unwrap();
    let config_hash = compute_chunker_config_hash(1_048_576);

    let temp_dir = tempfile::TempDir::new().unwrap();
    let content = "x".repeat(1000);
    let analyses = vec![make_analysis("exact.md", "Exact", &content, "concept")];
    let link_map = make_link_map(&analyses);

    let result = chunk_all_cached(
        &analyses,
        &link_map,
        temp_dir.path(),
        1000,
        &cache,
        config_hash,
    );
    assert!(
        result.is_ok(),
        "exact size should be accepted: {:?}",
        result.err()
    );
    let result = result.unwrap();
    assert!(result.total_chunks >= 1, "should produce at least 1 chunk");
    assert_eq!(result.document_count, 1);
}

#[test]
fn chunk_all_cached_returns_zero_count_result_when_analyses_is_empty() {
    let config = crate::cache::CacheConfig::in_memory();
    let cache = DocCache::open(config).unwrap();
    let config_hash = compute_chunker_config_hash(1024);

    let temp_dir = tempfile::TempDir::new().unwrap();
    let link_map = HashMap::new();

    let result =
        chunk_all_cached(&[], &link_map, temp_dir.path(), 1024, &cache, config_hash).unwrap();

    assert_eq!(result.total_chunks, 0);
    assert_eq!(result.document_count, 0);
    assert!(result.chunks_metadata.is_empty());

    let chunks_dir = temp_dir.path().join("chunks");
    assert!(chunks_dir.exists(), "chunks dir should exist");
    let entries: Vec<_> = fs::read_dir(&chunks_dir)
        .unwrap()
        .filter_map(std::result::Result::ok)
        .collect();
    assert!(entries.is_empty(), "chunks dir should be empty");
}

#[test]
fn chunk_all_cached_reuses_cached_chunks_on_second_run_for_same_files() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let db_path = temp_dir.path().join("cache.redb");
    let config = crate::cache::CacheConfig::new(&db_path);

    let output_dir1 = tempfile::TempDir::new().unwrap();
    let output_dir2 = tempfile::TempDir::new().unwrap();

    let analyses = vec![
        make_analysis("a.md", "Doc A", "# Doc A\nContent A", "concept"),
        make_analysis("b.md", "Doc B", "# Doc B\nContent B", "concept"),
        make_analysis("c.md", "Doc C", "# Doc C\nContent C", "concept"),
    ];
    let link_map = make_link_map(&analyses);
    let config_hash = compute_chunker_config_hash(1_048_576);

    let cache1 = DocCache::open(config.clone()).unwrap();
    let result1 = chunk_all_cached(
        &analyses,
        &link_map,
        output_dir1.path(),
        1_048_576,
        &cache1,
        config_hash,
    )
    .unwrap();
    drop(cache1);

    let cache2 = DocCache::open(config).unwrap();
    let result2 = chunk_all_cached(
        &analyses,
        &link_map,
        output_dir2.path(),
        1_048_576,
        &cache2,
        config_hash,
    )
    .unwrap();

    assert_eq!(result2.total_chunks, result1.total_chunks);
    assert_eq!(result2.document_count, result1.document_count);
    assert_eq!(result2.chunks_metadata.len(), result1.chunks_metadata.len());

    for (c2, c1) in result2
        .chunks_metadata
        .iter()
        .zip(result1.chunks_metadata.iter())
    {
        assert_eq!(c2.chunk_id, c1.chunk_id);
        assert_eq!(c2.doc_id, c1.doc_id);
        assert_eq!(c2.chunk_index, c1.chunk_index);
        assert_eq!(c2.content, c1.content);
    }
}
