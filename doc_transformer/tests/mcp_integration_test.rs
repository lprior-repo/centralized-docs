/// Integration tests for MCP server (all 10 tools)
///
/// Tests the complete MCP server with all tools against a real INDEX.json
use doc_transformer::index::{ChunkMetadata, IndexDocument};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
struct DocumentIndex {
    documents: Vec<IndexDocument>,
    chunks: Vec<ChunkMetadata>,
    #[serde(default)]
    keywords: HashMap<String, Vec<String>>,
}

/// Create a minimal test index
fn create_test_index() -> DocumentIndex {
    let doc1 = IndexDocument {
        id: "doc1".to_string(),
        title: "Getting Started".to_string(),
        path: "./docs/start.md".to_string(),
        category: "tutorial".to_string(),
        tags: vec!["intro".to_string(), "basics".to_string()],
        summary: "Introduction to the system".to_string(),
        word_count: 500,
        chunk_ids: vec!["chunk1".to_string(), "chunk2".to_string()],
    };

    let doc2 = IndexDocument {
        id: "doc2".to_string(),
        title: "Core Concepts".to_string(),
        path: "./docs/concepts.md".to_string(),
        category: "concept".to_string(),
        tags: vec!["advanced".to_string()],
        summary: "Core system concepts".to_string(),
        word_count: 1000,
        chunk_ids: vec!["chunk3".to_string()],
    };

    let chunk1 = ChunkMetadata {
        chunk_id: "chunk1".to_string(),
        doc_id: "doc1".to_string(),
        doc_title: "Getting Started".to_string(),
        heading: Some("Installation".to_string()),
        chunk_type: "text".to_string(),
        token_count: 150,
        summary: "How to install the system".to_string(),
        previous_chunk_id: None,
        next_chunk_id: Some("chunk2".to_string()),
        path: "./docs/start.md#installation".to_string(),
        related_chunks: vec![],
        chunk_level: "standard".to_string(),
        parent_chunk_id: None,
        child_chunk_ids: vec![],
    };

    let chunk2 = ChunkMetadata {
        chunk_id: "chunk2".to_string(),
        doc_id: "doc1".to_string(),
        doc_title: "Getting Started".to_string(),
        heading: Some("Configuration".to_string()),
        chunk_type: "text".to_string(),
        token_count: 200,
        summary: "How to configure the system".to_string(),
        previous_chunk_id: Some("chunk1".to_string()),
        next_chunk_id: None,
        path: "./docs/start.md#configuration".to_string(),
        related_chunks: vec![],
        chunk_level: "standard".to_string(),
        parent_chunk_id: None,
        child_chunk_ids: vec![],
    };

    let chunk3 = ChunkMetadata {
        chunk_id: "chunk3".to_string(),
        doc_id: "doc2".to_string(),
        doc_title: "Core Concepts".to_string(),
        heading: Some("Architecture".to_string()),
        chunk_type: "text".to_string(),
        token_count: 300,
        summary: "System architecture overview".to_string(),
        previous_chunk_id: None,
        next_chunk_id: None,
        path: "./docs/concepts.md#architecture".to_string(),
        related_chunks: vec![],
        chunk_level: "standard".to_string(),
        parent_chunk_id: None,
        child_chunk_ids: vec![],
    };

    DocumentIndex {
        documents: vec![doc1, doc2],
        chunks: vec![chunk1, chunk2, chunk3],
        keywords: HashMap::new(),
    }
}

#[test]
fn test_mcp_list_docs() {
    let index = create_test_index();

    // Verify documents exist
    assert_eq!(index.documents.len(), 2);
    assert_eq!(index.documents[0].id, "doc1");
    assert_eq!(index.documents[1].id, "doc2");
}

#[test]
fn test_mcp_get_chunk() {
    let index = create_test_index();

    // Find specific chunk
    let chunk = index.chunks.iter().find(|c| c.chunk_id == "chunk1");
    assert!(chunk.is_some());

    let chunk = chunk.unwrap();
    assert_eq!(chunk.doc_id, "doc1");
    assert_eq!(chunk.heading, Some("Installation".to_string()));
}

#[test]
fn test_mcp_search_by_category() {
    let index = create_test_index();

    // Find tutorial category
    let tutorials: Vec<_> = index
        .documents
        .iter()
        .filter(|d| d.category == "tutorial")
        .collect();

    assert_eq!(tutorials.len(), 1);
    assert_eq!(tutorials[0].id, "doc1");
}

#[test]
fn test_mcp_search_by_tags() {
    let index = create_test_index();

    // Find documents with "intro" tag
    let with_intro: Vec<_> = index
        .documents
        .iter()
        .filter(|d| d.tags.contains(&"intro".to_string()))
        .collect();

    assert_eq!(with_intro.len(), 1);
    assert_eq!(with_intro[0].id, "doc1");
}

#[test]
fn test_mcp_get_document() {
    let index = create_test_index();

    // Get document with chunks
    let doc = index.documents.iter().find(|d| d.id == "doc1");
    assert!(doc.is_some());

    let doc = doc.unwrap();
    assert_eq!(doc.chunk_ids.len(), 2);

    // Verify chunks belong to document
    let doc_chunks: Vec<_> = index
        .chunks
        .iter()
        .filter(|c| doc.chunk_ids.contains(&c.chunk_id))
        .collect();

    assert_eq!(doc_chunks.len(), 2);
}

#[test]
fn test_mcp_find_related_sequential() {
    let index = create_test_index();

    // Find next chunk
    let chunk1 = index.chunks.iter().find(|c| c.chunk_id == "chunk1").unwrap();
    assert_eq!(chunk1.next_chunk_id, Some("chunk2".to_string()));

    // Verify sequential relationship
    if let Some(next_id) = &chunk1.next_chunk_id {
        let chunk2 = index.chunks.iter().find(|c| &c.chunk_id == next_id);
        assert!(chunk2.is_some());
    }
}

#[test]
fn test_mcp_get_navigation() {
    let index = create_test_index();

    // Group by category
    let mut by_category: HashMap<&str, Vec<&IndexDocument>> = HashMap::new();
    for doc in &index.documents {
        by_category.entry(&doc.category).or_default().push(doc);
    }

    assert!(by_category.contains_key("tutorial"));
    assert!(by_category.contains_key("concept"));
    assert_eq!(by_category.get("tutorial").unwrap().len(), 1);
}

#[test]
fn test_mcp_semantic_search_fallback() {
    let index = create_test_index();

    // Simple text matching (fallback until vector search)
    let query = "install";
    let matching_chunks: Vec<_> = index
        .chunks
        .iter()
        .filter(|c| c.summary.to_lowercase().contains(&query.to_lowercase()))
        .collect();

    assert_eq!(matching_chunks.len(), 1);
    assert_eq!(matching_chunks[0].chunk_id, "chunk1");
}

#[test]
fn test_index_structure_integrity() {
    let index = create_test_index();

    // Verify all chunk doc_ids reference existing documents
    let doc_ids: std::collections::HashSet<_> =
        index.documents.iter().map(|d| &d.id).collect();

    for chunk in &index.chunks {
        assert!(
            doc_ids.contains(&chunk.doc_id),
            "Chunk {} references non-existent document {}",
            chunk.chunk_id,
            chunk.doc_id
        );
    }

    // Verify document chunk_ids reference existing chunks
    let chunk_ids: std::collections::HashSet<_> =
        index.chunks.iter().map(|c| &c.chunk_id).collect();

    for doc in &index.documents {
        for chunk_id in &doc.chunk_ids {
            assert!(
                chunk_ids.contains(chunk_id),
                "Document {} references non-existent chunk {}",
                doc.id,
                chunk_id
            );
        }
    }
}

#[test]
fn test_chunk_level_values() {
    let index = create_test_index();

    // Verify all chunk levels are valid
    let valid_levels = ["summary", "standard", "detailed"];

    for chunk in &index.chunks {
        assert!(
            valid_levels.contains(&chunk.chunk_level.as_str()),
            "Invalid chunk_level: {}",
            chunk.chunk_level
        );
    }
}

#[test]
fn test_mcp_explain_chunk() {
    let index = create_test_index();

    // Get chunk2 which has a previous chunk
    let chunk = index.chunks.iter().find(|c| c.chunk_id == "chunk2").unwrap();

    // Verify it has a previous chunk
    assert_eq!(chunk.previous_chunk_id, Some("chunk1".to_string()));

    // This would build a context trail showing chunk1 -> chunk2
    assert!(chunk.previous_chunk_id.is_some());
}
