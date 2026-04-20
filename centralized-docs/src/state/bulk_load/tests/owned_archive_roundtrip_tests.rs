//! `OwnedArchive` roundtrip tests for all persisted types.

use super::*;

#[test]
fn owned_archive_deserialize_roundtrip_produces_original_value() {
    use crate::persisted::PersistedTransformResult;
    let original = PersistedTransformResult {
        schema_version: 1,
        success_count: 99,
        total_count: 100,
        error_count: 1,
        errors: vec![],
    };
    let rkyv_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&original)
        .expect("serialization should succeed")
        .to_vec()
        .into_boxed_slice();
    let key: [u8; 32] = [0x33; 32];

    let archive = OwnedArchive::<PersistedTransformResult>::try_from_bytes(
        "transform_outputs",
        &key,
        rkyv_bytes,
    )
    .expect("valid rkyv bytes should construct archive");

    let deserialized = archive.deserialize().expect("deserialize should succeed");
    assert_eq!(deserialized.schema_version, original.schema_version);
    assert_eq!(deserialized.success_count, original.success_count);
    assert_eq!(deserialized.total_count, original.total_count);
    assert_eq!(deserialized.error_count, original.error_count);
}

#[test]
fn owned_archive_deserialize_roundtrip_preserves_chunk_data() {
    use crate::persisted::{
        PersistedChunk, PersistedChunkLevel, PersistedChunkType, PersistedChunksResult,
    };
    let original = PersistedChunksResult {
        schema_version: 1,
        total_chunks: 2,
        document_count: 1,
        chunks_metadata: vec![PersistedChunk {
            schema_version: 1,
            chunk_id: "doc1#0".to_string(),
            doc_id: "doc1".to_string(),
            doc_title: "Test".to_string(),
            chunk_index: 0,
            content: "chunk 0 content".to_string(),
            token_count: 5,
            heading: None,
            heading_path: vec![],
            chunk_type: PersistedChunkType::Prose,
            previous_chunk_id: None,
            next_chunk_id: Some("doc1#1".to_string()),
            related_chunk_ids: vec![],
            summary: "summary 0".to_string(),
            chunk_level: PersistedChunkLevel::Standard,
            parent_chunk_id: None,
            child_chunk_ids: vec![],
            context_prefix: None,
        }],
        summary_chunks: 0,
        standard_chunks: 2,
        detailed_chunks: 0,
    };
    let rkyv_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&original)
        .expect("serialization should succeed")
        .to_vec()
        .into_boxed_slice();
    let key: [u8; 32] = [0x44; 32];

    let archive =
        OwnedArchive::<PersistedChunksResult>::try_from_bytes("chunk_outputs", &key, rkyv_bytes)
            .expect("valid rkyv bytes should construct archive");

    let deserialized = archive.deserialize().expect("deserialize should succeed");
    assert_eq!(deserialized.total_chunks, 2);
    assert_eq!(deserialized.chunks_metadata.len(), 1);
    assert_eq!(deserialized.chunks_metadata[0].chunk_id, "doc1#0");
}

#[test]
fn owned_archive_deserialize_roundtrip_preserves_scrape_data() {
    use crate::persisted::{
        PersistedHeader, PersistedPageFilterStatus, PersistedScrapeResult, PersistedScrapedPage,
    };
    let original = PersistedScrapeResult {
        schema_version: 1,
        pages: vec![PersistedScrapedPage {
            url: "https://example.com".to_string(),
            markdown: "content".to_string(),
            title: "Example".to_string(),
            links: vec!["https://other.com".to_string()],
            headers: vec![PersistedHeader {
                level: 1,
                text: "Title".to_string(),
            }],
            word_count: 100,
            slug: "example".to_string(),
            filter_status: PersistedPageFilterStatus::Unfiltered,
            elements_removed: 0,
            density_score: 1.0,
        }],
        total_urls: 1,
        success_count: 1,
        error_count: 0,
        errors: vec![],
        base_url: "https://example.com".to_string(),
    };
    let rkyv_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&original)
        .expect("serialization should succeed")
        .to_vec()
        .into_boxed_slice();
    let key: [u8; 32] = [0x55; 32];

    let archive =
        OwnedArchive::<PersistedScrapeResult>::try_from_bytes("scrape_outputs", &key, rkyv_bytes)
            .expect("valid rkyv bytes should construct archive");

    let deserialized = archive.deserialize().expect("deserialize should succeed");
    assert_eq!(deserialized.pages.len(), 1);
    assert_eq!(deserialized.pages[0].url, "https://example.com");
    assert_eq!(deserialized.pages[0].links.len(), 1);
}

#[test]
fn owned_archive_deserialize_roundtrip_preserves_analysis_data() {
    use crate::persisted::{
        PersistedAnalysis, PersistedAnalyzeResult, PersistedFailedFile, PersistedLink,
        PersistedLinkKind,
    };
    let original = PersistedAnalyzeResult {
        schema_version: 1,
        analyses: vec![PersistedAnalysis {
            schema_version: 1,
            source_path: "src/main.rs".to_string(),
            title: "Main".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![PersistedLink {
                text: "docs".to_string(),
                target: "https://docs.rs".to_string(),
                kind: PersistedLinkKind::External,
            }],
            first_paragraph: "intro".to_string(),
            word_count: 500,
            has_code: true,
            has_tables: false,
            category: "rust".to_string(),
            content: "body text".to_string(),
        }],
        failed_files: vec![PersistedFailedFile {
            source_path: "broken.md".to_string(),
            error: "parse error".to_string(),
        }],
        total_discovered: 2,
    };
    let rkyv_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&original)
        .expect("serialization should succeed")
        .to_vec()
        .into_boxed_slice();
    let key: [u8; 32] = [0x66; 32];

    let archive = OwnedArchive::<PersistedAnalyzeResult>::try_from_bytes(
        "analysis_outputs",
        &key,
        rkyv_bytes,
    )
    .expect("valid rkyv bytes should construct archive");

    let deserialized = archive.deserialize().expect("deserialize should succeed");
    assert_eq!(deserialized.analyses.len(), 1);
    assert_eq!(deserialized.analyses[0].source_path, "src/main.rs");
    assert_eq!(deserialized.analyses[0].word_count, 500);
    assert_eq!(deserialized.failed_files.len(), 1);
    assert_eq!(deserialized.total_discovered, 2);
}
