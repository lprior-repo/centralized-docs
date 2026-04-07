use super::*;

#[test]
fn test_build_related_chunks_index() {
    let mut dag = KnowledgeDAG::new();
    dag.add_node(GraphNode {
        id: "chunk1".to_string(),
        node_type: NodeType::Chunk,
        title: "Chunk 1".to_string(),
        category: None,
    });
    dag.add_node(GraphNode {
        id: "chunk2".to_string(),
        node_type: NodeType::Chunk,
        title: "Chunk 2".to_string(),
        category: None,
    });
    dag.add_node(GraphNode {
        id: "chunk3".to_string(),
        node_type: NodeType::Chunk,
        title: "Chunk 3".to_string(),
        category: None,
    });

    dag.add_edge(GraphEdge {
        from: "chunk1".to_string(),
        to: "chunk2".to_string(),
        edge_type: EdgeType::Related,
        weight: 0.8,
    });
    dag.add_edge(GraphEdge {
        from: "chunk1".to_string(),
        to: "chunk3".to_string(),
        edge_type: EdgeType::Related,
        weight: 0.3,
    });
    dag.add_edge(GraphEdge {
        from: "chunk2".to_string(),
        to: "chunk3".to_string(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });

    let index = dag.build_related_chunks_index();

    assert!(index.contains_key("chunk1"));
    let chunk1_related = &index["chunk1"];
    assert_eq!(chunk1_related.len(), 2);
    assert_eq!(chunk1_related[0].0, "chunk2");
    assert!((chunk1_related[0].1 - 0.8).abs() < 0.001);
    assert_eq!(chunk1_related[1].0, "chunk3");
    assert!((chunk1_related[1].1 - 0.3).abs() < 0.001);

    assert!(!index.contains_key("chunk2"));
}

#[test]
fn test_batch_reachable() {
    let mut dag = KnowledgeDAG::new();
    for i in 0..4 {
        dag.add_node(GraphNode {
            id: format!("n{i}"),
            node_type: NodeType::Chunk,
            title: format!("Node {i}"),
            category: None,
        });
    }
    dag.add_edge(GraphEdge {
        from: "n0".to_string(),
        to: "n1".to_string(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });
    dag.add_edge(GraphEdge {
        from: "n1".to_string(),
        to: "n2".to_string(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });
    dag.add_edge(GraphEdge {
        from: "n2".to_string(),
        to: "n3".to_string(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });

    // batch_reachable uses incoming edges, so it collects ancestors of each node.
    let result = dag.batch_reachable(&["n3".to_string()], None);
    assert!(result.contains_key("n3"));
    let reachable = &result["n3"];
    assert_eq!(reachable.len(), 3);
    assert_eq!(reachable[0], "n0");
    assert_eq!(reachable[1], "n1");
    assert_eq!(reachable[2], "n2");
}

#[test]
fn test_batch_reachable_with_precomputed_topo() {
    let mut dag = KnowledgeDAG::new();
    dag.add_node(GraphNode {
        id: "a".to_string(),
        node_type: NodeType::Chunk,
        title: "A".to_string(),
        category: None,
    });
    dag.add_node(GraphNode {
        id: "b".to_string(),
        node_type: NodeType::Chunk,
        title: "B".to_string(),
        category: None,
    });
    dag.add_edge(GraphEdge {
        from: "a".to_string(),
        to: "b".to_string(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });

    let topo = dag.topo_node_indices();
    let result = dag.batch_reachable(&["b".to_string()], Some(topo));
    assert!(result.contains_key("b"));
    assert_eq!(result["b"].len(), 1);
    assert_eq!(result["b"][0], "a");
}

#[test]
fn test_batch_reachable_multiple_sources() {
    let mut dag = KnowledgeDAG::new();
    for id in ["src1", "src2", "shared", "leaf"] {
        dag.add_node(GraphNode {
            id: id.to_string(),
            node_type: NodeType::Chunk,
            title: id.to_string(),
            category: None,
        });
    }
    dag.add_edge(GraphEdge {
        from: "src1".to_string(),
        to: "shared".to_string(),
        edge_type: EdgeType::References,
        weight: 1.0,
    });
    dag.add_edge(GraphEdge {
        from: "src2".to_string(),
        to: "shared".to_string(),
        edge_type: EdgeType::References,
        weight: 1.0,
    });
    dag.add_edge(GraphEdge {
        from: "shared".to_string(),
        to: "leaf".to_string(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });

    let result = dag.batch_reachable(&["leaf".to_string(), "shared".to_string()], None);
    assert_eq!(result["leaf"].len(), 3);
    assert_eq!(result["shared"].len(), 2);
}

#[test]
fn test_reachable_from() {
    let mut dag = KnowledgeDAG::new();
    for id in ["root", "mid", "leaf"] {
        dag.add_node(GraphNode {
            id: id.to_string(),
            node_type: NodeType::Chunk,
            title: id.to_string(),
            category: None,
        });
    }
    dag.add_edge(GraphEdge {
        from: "root".to_string(),
        to: "mid".to_string(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });
    dag.add_edge(GraphEdge {
        from: "mid".to_string(),
        to: "leaf".to_string(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });

    let reachable = dag.reachable_from("root");
    assert_eq!(reachable.len(), 3);
    assert!(reachable.contains("root"));
    assert!(reachable.contains("mid"));
    assert!(reachable.contains("leaf"));

    let leaf_reachable = dag.reachable_from("leaf");
    assert_eq!(leaf_reachable.len(), 1);
    assert!(leaf_reachable.contains("leaf"));

    let missing_reachable = dag.reachable_from("nonexistent");
    assert!(missing_reachable.is_empty());
}

#[test]
fn test_reachable_from_via_edge_types() {
    let mut dag = KnowledgeDAG::new();
    for id in ["a", "b", "c"] {
        dag.add_node(GraphNode {
            id: id.to_string(),
            node_type: NodeType::Chunk,
            title: id.to_string(),
            category: None,
        });
    }
    dag.add_edge(GraphEdge {
        from: "a".to_string(),
        to: "b".to_string(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });
    dag.add_edge(GraphEdge {
        from: "b".to_string(),
        to: "c".to_string(),
        edge_type: EdgeType::Related,
        weight: 0.5,
    });

    let via_sequential = dag.reachable_from_via_edge_types("a", &[EdgeType::Sequential]);
    assert_eq!(via_sequential.len(), 2);
    assert!(via_sequential.contains("a"));
    assert!(via_sequential.contains("b"));
    assert!(!via_sequential.contains("c"));

    let via_related = dag.reachable_from_via_edge_types("b", &[EdgeType::Related]);
    assert_eq!(via_related.len(), 2);
    assert!(via_related.contains("b"));
    assert!(via_related.contains("c"));

    let via_both =
        dag.reachable_from_via_edge_types("a", &[EdgeType::Sequential, EdgeType::Related]);
    assert_eq!(via_both.len(), 3);
}
