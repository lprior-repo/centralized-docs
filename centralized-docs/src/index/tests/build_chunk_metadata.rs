//! Tests for build_chunk_metadata.

use super::*;
use crate::graph::{EdgeType, GraphEdge, GraphNode, KnowledgeDAG, NodeType};

#[test]
fn test_chunk_metadata_no_duplicate_ids() {
    let chunks = vec![
        make_chunk(
            "doc1#0-standard",
            "doc1",
            "Doc 1",
            "Content 1",
            Some("Section 1"),
            ChunkLevel::Standard,
        ),
        make_chunk(
            "doc1#1-standard",
            "doc1",
            "Doc 1",
            "Content 2",
            Some("Section 2"),
            ChunkLevel::Standard,
        ),
        make_chunk(
            "doc1#0-standard",
            "doc1",
            "Doc 1",
            "Content 1",
            Some("Section 1"),
            ChunkLevel::Standard,
        ),
    ];
    let result = build_chunk_metadata(&chunks, &KnowledgeDAG::new());
    match result {
        Err(e) => {
            assert!(e.to_string().contains("Duplicate chunk_id"));
            assert!(e.to_string().contains("doc1#0-standard"));
        }
        Ok(_) => panic!("Should fail when duplicate chunk_ids exist"),
    }
}

#[test]
fn test_build_chunk_metadata_valid() {
    let chunks = vec![
        make_chunk(
            "doc1#0-standard",
            "doc1",
            "Doc 1",
            "Content 1",
            Some("Intro"),
            ChunkLevel::Standard,
        ),
        make_chunk(
            "doc1#1-standard",
            "doc1",
            "Doc 1",
            "Content 2",
            Some("Body"),
            ChunkLevel::Standard,
        ),
    ];
    let metadata = build_chunk_metadata(&chunks, &KnowledgeDAG::new()).unwrap();
    assert_eq!(metadata.len(), 2);
    assert_eq!(metadata[0].chunk_id, "doc1#0-standard");
    assert_eq!(metadata[0].doc_id, "doc1");
    assert_eq!(metadata[0].heading, Some("Intro".to_string()));
    assert!(metadata[0].related_chunks.is_empty());
    assert!(metadata[0].path.contains("doc1"));
    assert!(metadata[0].path.contains("standard"));
}

#[test]
fn test_build_chunk_metadata_empty_heading_path_gets_intro() {
    let chunks = vec![make_chunk(
        "doc1#0-standard",
        "doc1",
        "Doc 1",
        "Content",
        None,
        ChunkLevel::Standard,
    )];
    let metadata = build_chunk_metadata(&chunks, &KnowledgeDAG::new()).unwrap();
    assert_eq!(metadata[0].heading_path, vec!["Intro".to_string()]);
    assert!(
        metadata[0].heading_anchor.is_none(),
        "Intro heading should not produce anchor"
    );
}

#[test]
fn test_build_chunk_metadata_with_heading_path() {
    let mut chunk = make_chunk(
        "doc1#0-standard",
        "doc1",
        "Doc 1",
        "Content",
        Some("Section"),
        ChunkLevel::Standard,
    );
    chunk.heading_path = vec!["Chapter 1".to_string(), "Section A".to_string()];
    let metadata = build_chunk_metadata(&[chunk], &KnowledgeDAG::new()).unwrap();
    assert_eq!(
        metadata[0].heading_path,
        vec!["Chapter 1".to_string(), "Section A".to_string()]
    );
    assert!(metadata[0].heading_anchor.is_some());
}

#[test]
fn test_build_chunk_metadata_siblings() {
    let chunks = vec![
        make_chunk(
            "doc1#0-standard",
            "doc1",
            "Doc 1",
            "C1",
            None,
            ChunkLevel::Standard,
        ),
        make_chunk(
            "doc1#1-standard",
            "doc1",
            "Doc 1",
            "C2",
            None,
            ChunkLevel::Standard,
        ),
        make_chunk(
            "doc1#2-standard",
            "doc1",
            "Doc 1",
            "C3",
            None,
            ChunkLevel::Standard,
        ),
    ];
    let metadata = build_chunk_metadata(&chunks, &KnowledgeDAG::new()).unwrap();
    for m in &metadata {
        assert_eq!(
            m.sibling_chunk_ids.len(),
            2,
            "Each chunk should have 2 siblings"
        );
    }
}

#[test]
fn test_build_chunk_metadata_siblings_different_levels() {
    let chunks = vec![
        make_chunk(
            "doc1#0-standard",
            "doc1",
            "Doc 1",
            "C1",
            None,
            ChunkLevel::Standard,
        ),
        make_chunk(
            "doc1#0-summary",
            "doc1",
            "Doc 1",
            "C2",
            None,
            ChunkLevel::Summary,
        ),
    ];
    let metadata = build_chunk_metadata(&chunks, &KnowledgeDAG::new()).unwrap();
    assert!(
        metadata[0].sibling_chunk_ids.is_empty(),
        "Different levels should not be siblings"
    );
    assert!(metadata[1].sibling_chunk_ids.is_empty());
}

#[test]
fn test_build_chunk_metadata_with_parent_child() {
    let mut chunk = make_chunk(
        "doc1#0-standard",
        "doc1",
        "Doc 1",
        "Content",
        None,
        ChunkLevel::Standard,
    );
    chunk.parent_chunk_id = Some("doc1#0-summary".to_string());
    chunk.child_chunk_ids = vec!["doc1#1-standard".to_string()];
    let metadata = build_chunk_metadata(&[chunk], &KnowledgeDAG::new()).unwrap();
    assert_eq!(
        metadata[0].parent_chunk_id,
        Some("doc1#0-summary".to_string())
    );
    assert_eq!(
        metadata[0].child_chunk_ids,
        vec!["doc1#1-standard".to_string()]
    );
}

#[test]
fn test_build_chunk_metadata_with_related_chunks_from_dag() {
    let chunks = vec![make_chunk(
        "doc1#0-standard",
        "doc1",
        "Doc 1",
        "Content",
        None,
        ChunkLevel::Standard,
    )];
    let mut dag = KnowledgeDAG::new();
    dag.add_node(GraphNode {
        id: "doc1#0-standard".to_string(),
        node_type: NodeType::Chunk,
        title: "Doc 1".to_string(),
        category: None,
    });
    dag.add_node(GraphNode {
        id: "doc2#0-standard".to_string(),
        node_type: NodeType::Chunk,
        title: "Doc 2".to_string(),
        category: None,
    });
    dag.add_edge(GraphEdge {
        from: "doc1#0-standard".to_string(),
        to: "doc2#0-standard".to_string(),
        edge_type: EdgeType::Related,
        weight: 0.8,
    });
    let metadata = build_chunk_metadata(&chunks, &dag).unwrap();
    assert_eq!(metadata[0].related_chunks.len(), 1);
    assert_eq!(metadata[0].related_chunks[0].chunk_id, "doc2#0-standard");
    assert!((metadata[0].related_chunks[0].similarity - 0.8).abs() < f32::EPSILON);
}

#[test]
fn test_build_chunk_metadata_empty() {
    let metadata = build_chunk_metadata(&[], &KnowledgeDAG::new()).unwrap();
    assert!(metadata.is_empty());
}
