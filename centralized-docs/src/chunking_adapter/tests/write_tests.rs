//! Write chunk file tests (B11-B14).

use std::collections::HashMap;
use std::fs;

use super::*;
use crate::cache::{CacheType, DocCache};

#[test]
fn write_chunk_file_creates_md_with_frontmatter_and_content() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let chunks_dir = temp_dir.path().join("chunks");
    fs::create_dir_all(&chunks_dir).unwrap();

    let chunk = Chunk {
        token_count: 42,
        summary: "A summary".to_string(),
        ..make_test_chunk(
            "doc#0",
            "doc",
            0,
            contextual_chunker::ChunkLevel::Standard,
            "body text",
            Some("Intro"),
        )
    };

    write_chunk_file(&chunk, &chunks_dir).unwrap();

    let file_path = chunks_dir.join("doc-0-standard.md");
    assert!(file_path.exists(), "chunk file should exist");

    let contents = fs::read_to_string(file_path).unwrap();
    assert!(contents.starts_with("---"), "should start with frontmatter");
    assert!(contents.contains("doc_id: doc"));
    assert!(contents.contains("chunk_id: doc#0"));
    assert!(contents.contains("chunk_level: standard"));
    assert!(contents.contains("heading: Intro"));
    assert!(contents.contains("token_count: 42"));
    assert!(contents.contains("summary: A summary"));
    assert!(contents.contains("body text"));

    let parts: Vec<&str> = contents.splitn(3, "---").collect();
    assert!(parts.len() >= 3, "should have frontmatter delimiters");
    assert!(parts[2].trim().contains("body text"));
}

#[test]
fn write_chunk_file_returns_chunk_write_failed_when_dir_unwritable() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let ro_dir = temp_dir.path().join("ro");
    let chunks_dir = ro_dir.join("chunks");
    fs::create_dir_all(&chunks_dir).unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&ro_dir, fs::Permissions::from_mode(0o444)).unwrap();
    }

    let chunk = make_test_chunk(
        "doc#0",
        "doc",
        0,
        contextual_chunker::ChunkLevel::Standard,
        "body",
        Some("H"),
    );
    let result = write_chunk_file(&chunk, &chunks_dir);
    assert!(result.is_err());

    let err = result.unwrap_err();
    let reuse_err = err.downcast_ref::<ChunkReuseError>();
    assert!(reuse_err.is_some());

    if let Some(ChunkReuseError::ChunkWriteFailed { path, .. }) = reuse_err {
        assert!(path.to_string_lossy().contains("chunks"));
    } else {
        panic!("expected ChunkWriteFailed variant");
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&ro_dir, fs::Permissions::from_mode(0o755)).unwrap();
    }
}

#[test]
fn chunk_all_cached_returns_document_exceeds_size_limit_when_content_too_large() {
    let config = crate::cache::CacheConfig::in_memory();
    let cache = DocCache::open(config).unwrap();
    let config_hash = compute_chunker_config_hash(1000);
    let temp_dir = tempfile::TempDir::new().unwrap();
    let link_map = HashMap::new();
    let analyses = vec![make_analysis("big.md", "Big", &"x".repeat(2000), "concept")];

    let result = chunk_all_cached(
        &analyses,
        &link_map,
        temp_dir.path(),
        1000,
        &cache,
        config_hash,
    );
    assert!(result.is_err());
    let err = result.unwrap_err();
    let reuse_err = err.downcast_ref::<ChunkReuseError>();
    assert!(reuse_err.is_some());

    if let Some(ChunkReuseError::DocumentExceedsSizeLimit {
        source_path,
        content_size,
        max_bytes,
    }) = reuse_err
    {
        assert_eq!(*source_path, "big.md");
        assert_eq!(*content_size, 2000);
        assert_eq!(*max_bytes, 1000);
    } else {
        panic!("expected DocumentExceedsSizeLimit");
    }
}
