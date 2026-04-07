//! Serialization and full pipeline tests.

use std::fs;

use super::*;

#[test]
fn test_index_document_serialization() {
    let doc = IndexDocument {
        id: "id".to_string(),
        title: "Title".to_string(),
        path: "p".to_string(),
        category: "cat".to_string(),
        tags: vec!["tag1".to_string()],
        summary: "sum".to_string(),
        word_count: 42,
        chunk_ids: vec![],
        headings: vec![],
        content: "c".into(),
    };
    let json = serde_json::to_string(&doc).unwrap();
    let deserialized: IndexDocument = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.id, "id");
    assert_eq!(deserialized.tags, vec!["tag1"]);
}

#[test]
fn test_chunk_metadata_serialization() {
    let meta = ChunkMetadata {
        chunk_id: "c1".to_string(),
        doc_id: "d1".to_string(),
        doc_title: "D".to_string(),
        heading: Some("H".to_string()),
        heading_path: vec!["A".to_string()],
        heading_anchor: Some("a".to_string()),
        chunk_type: contextual_chunker::ChunkType::Prose,
        token_count: 100,
        summary: "S".to_string(),
        previous_chunk_id: Some("prev".to_string()),
        next_chunk_id: Some("next".to_string()),
        section_index: 0,
        path: "chunks/c.md".to_string(),
        related_chunks: vec![RelatedChunk {
            chunk_id: "r1".to_string(),
            similarity: 0.5,
        }],
        chunk_level: ChunkLevel::Standard,
        parent_chunk_id: None,
        child_chunk_ids: vec![],
        sibling_chunk_ids: vec![],
    };
    let json = serde_json::to_string(&meta).unwrap();
    let deserialized: ChunkMetadata = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.chunk_id, "c1");
    assert_eq!(deserialized.related_chunks.len(), 1);
}

#[test]
fn test_related_chunk_struct() {
    let rc = RelatedChunk {
        chunk_id: "chunk-1".to_string(),
        similarity: 0.85,
    };
    let cloned = rc.clone();
    assert_eq!(cloned.chunk_id, rc.chunk_id);
    assert!((cloned.similarity - rc.similarity).abs() < f32::EPSILON);
}

#[test]
fn test_build_and_write_index_full_pipeline() {
    let analyses = vec![make_analysis(
        "docs/tutorial/guide.md",
        "Guide",
        "tutorial",
        vec![make_heading(1, "Guide Title")],
        "Guide first paragraph with keywords.",
        100,
    )];
    let link_map = make_link_map(vec![(
        "docs/tutorial/guide.md",
        "tutorial/guide",
        "tutorial-guide.md",
        "tutorial",
    )]);
    let chunks_result = ChunksResult {
        total_chunks: 1,
        document_count: 1,
        chunks_metadata: vec![make_chunk(
            "tutorial/guide#0-standard",
            "tutorial/guide",
            "Guide",
            "Guide chunk content.",
            None,
            ChunkLevel::Standard,
        )],
        summary_chunks: 0,
        standard_chunks: 1,
        detailed_chunks: 0,
    };
    let dir = tempfile::TempDir::new().unwrap();
    build_and_write_index(
        &analyses,
        &link_map,
        &chunks_result,
        dir.path(),
        "test-proj",
        None,
        None,
        None,
        None,
    )
    .unwrap();
    assert!(dir.path().join("INDEX.json").exists());
    assert!(dir.path().join(".tantivy_index").exists());
    let index_content = fs::read_to_string(dir.path().join("INDEX.json")).unwrap();
    let index_json: serde_json::Value = serde_json::from_str(&index_content).unwrap();
    assert_eq!(index_json["version"], "5.0");
    assert_eq!(index_json["project"], "test-proj");
}
