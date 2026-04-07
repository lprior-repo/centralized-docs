//! Tests for knowledge DAG construction and graph analytics.

use super::*;
use crate::graph::{EdgeType, GraphEdge, GraphNode, KnowledgeDAG, NodeType};
use crate::index::index_assembly::compute_graph_analytics;

#[test]
fn test_empty_chunks_no_crash() {
    let dag = build_knowledge_dag(&[], &[], &[], None, None, None, None)
        .expect("Failed with empty chunks");
    let stats = dag.statistics();
    assert_eq!(stats.node_count, 0);
    assert_eq!(stats.edge_count, 0);
}

#[test]
fn test_build_knowledge_dag_with_documents_and_chunks() {
    let documents = vec![IndexDocument {
        id: "doc1".to_string(),
        title: "Doc 1".to_string(),
        path: "docs/doc1.md".to_string(),
        category: "tutorial".to_string(),
        tags: vec![],
        summary: "Summary".to_string(),
        word_count: 100,
        chunk_ids: vec![],
        headings: vec![],
        content: "Content".into(),
    }];
    let mut chunk0 = make_chunk(
        "doc1#0-standard",
        "doc1",
        "Doc 1",
        "C0",
        Some("Intro"),
        ChunkLevel::Standard,
    );
    chunk0.next_chunk_id = Some("doc1#1-standard".to_string());
    let mut chunk1 = make_chunk(
        "doc1#1-standard",
        "doc1",
        "Doc 1",
        "C1",
        Some("Body"),
        ChunkLevel::Standard,
    );
    chunk1.previous_chunk_id = Some("doc1#0-standard".to_string());
    let dag =
        build_knowledge_dag(&documents, &[chunk0, chunk1], &[], None, None, None, None).unwrap();
    let stats = dag.statistics();
    assert_eq!(stats.node_count, 3);
    assert!(
        stats.edge_count >= 2,
        "Should have parent edges + sequential edges"
    );
}

#[test]
fn test_build_knowledge_dag_chunk_heading_titles() {
    let documents = vec![IndexDocument {
        id: "doc1".to_string(),
        title: "Test Doc".to_string(),
        path: "docs/test.md".to_string(),
        category: "concept".to_string(),
        tags: vec![],
        summary: "Sum".to_string(),
        word_count: 10,
        chunk_ids: vec![],
        headings: vec![],
        content: "C".into(),
    }];
    let chunk_with = make_chunk(
        "doc1#0",
        "doc1",
        "Test Doc",
        "Content",
        Some("Chapter One"),
        ChunkLevel::Standard,
    );
    let chunk_no = make_chunk(
        "doc1#1",
        "doc1",
        "Test Doc",
        "Content",
        None,
        ChunkLevel::Standard,
    );
    let dag = build_knowledge_dag(
        &documents,
        &[chunk_with, chunk_no],
        &[],
        None,
        None,
        None,
        None,
    )
    .unwrap();
    let nodes = dag.nodes();
    assert!(nodes.iter().any(|n| n.title == "Test Doc - Chapter One"));
    assert!(nodes.iter().any(|n| n.title == "Test Doc - Intro"));
}

#[test]
fn test_compute_graph_analytics_empty() {
    let dag = KnowledgeDAG::new();
    let documents: Vec<IndexDocument> = vec![];
    let analytics = index_assembly::compute_graph_analytics(&dag, &documents);
    assert!(analytics.topo_order.is_empty());
    assert!(analytics.reachability.is_empty());
    assert!(analytics.node_importance.is_empty());
}

#[test]
fn test_compute_graph_analytics_with_nodes() {
    let mut dag = KnowledgeDAG::new();
    dag.add_node(GraphNode {
        id: "doc1".to_string(),
        node_type: NodeType::Document,
        title: "Doc 1".to_string(),
        category: Some("tutorial".to_string()),
    });
    dag.add_node(GraphNode {
        id: "doc2".to_string(),
        node_type: NodeType::Document,
        title: "Doc 2".to_string(),
        category: Some("ref".to_string()),
    });
    dag.add_edge(GraphEdge {
        from: "doc1".to_string(),
        to: "doc2".to_string(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });
    let documents = vec![
        IndexDocument {
            id: "doc1".to_string(),
            title: "Doc 1".to_string(),
            path: "d1".to_string(),
            category: "t".to_string(),
            tags: vec![],
            summary: "s".to_string(),
            word_count: 10,
            chunk_ids: vec![],
            headings: vec![],
            content: "c".into(),
        },
        IndexDocument {
            id: "doc2".to_string(),
            title: "Doc 2".to_string(),
            path: "d2".to_string(),
            category: "r".to_string(),
            tags: vec![],
            summary: "s".to_string(),
            word_count: 20,
            chunk_ids: vec![],
            headings: vec![],
            content: "c".into(),
        },
    ];
    let analytics = compute_graph_analytics(&dag, &documents);
    assert_eq!(analytics.topo_order.len(), 2);
    assert!(analytics.reachability.contains_key("doc1"));
    assert!(analytics.node_importance.contains_key("doc1"));
    assert!(analytics.node_importance.contains_key("doc2"));
}
