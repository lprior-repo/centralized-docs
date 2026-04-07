use super::*;

#[test]
fn test_dag_creation() {
    let mut dag = KnowledgeDAG::new();

    let node1 = GraphNode {
        id: "doc1".to_string(),
        node_type: NodeType::Document,
        title: "Document 1".to_string(),
        category: Some("tutorial".to_string()),
    };

    dag.add_node(node1);
    assert_eq!(dag.nodes().len(), 1);
}

#[test]
fn test_edge_addition() {
    let mut dag = KnowledgeDAG::new();

    let node1 = GraphNode {
        id: "chunk1".to_string(),
        node_type: NodeType::Chunk,
        title: "Chunk 1".to_string(),
        category: None,
    };

    let node2 = GraphNode {
        id: "chunk2".to_string(),
        node_type: NodeType::Chunk,
        title: "Chunk 2".to_string(),
        category: None,
    };

    dag.add_node(node1);
    dag.add_node(node2);

    let edge = GraphEdge {
        from: "chunk1".to_string(),
        to: "chunk2".to_string(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    };

    dag.add_edge(edge);
    assert_eq!(dag.edges().len(), 1);
}

#[test]
fn test_topological_sort() {
    let mut dag = KnowledgeDAG::new();

    for i in 1..=3 {
        dag.add_node(GraphNode {
            id: format!("node{i}"),
            node_type: NodeType::Chunk,
            title: format!("Node {i}"),
            category: None,
        });
    }

    dag.add_edge(GraphEdge {
        from: "node1".to_string(),
        to: "node2".to_string(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });

    dag.add_edge(GraphEdge {
        from: "node2".to_string(),
        to: "node3".to_string(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });

    let topo_order = dag.topological_order();
    assert_eq!(topo_order.len(), 3);
    assert_eq!(topo_order[0], "node1");
}

#[test]
fn test_node_importance() {
    let mut dag = KnowledgeDAG::new();

    dag.add_node(GraphNode {
        id: "hub".to_string(),
        node_type: NodeType::Document,
        title: "Hub".to_string(),
        category: None,
    });

    for i in 1..=3 {
        dag.add_node(GraphNode {
            id: format!("spoke{i}"),
            node_type: NodeType::Chunk,
            title: format!("Spoke {i}"),
            category: None,
        });

        dag.add_edge(GraphEdge {
            from: "hub".to_string(),
            to: format!("spoke{i}"),
            edge_type: EdgeType::Parent,
            weight: 0.5,
        });
    }

    let importance = dag.node_importance("hub");
    assert!((importance - 1.5).abs() < 0.001); // 3 edges * 0.5 weight

    let no_importance = dag.node_importance("nonexistent");
    assert_eq!(no_importance, 0.0);
}

#[test]
fn test_statistics() {
    let mut dag = KnowledgeDAG::new();
    dag.add_node(GraphNode {
        id: "doc1".to_string(),
        node_type: NodeType::Document,
        title: "Doc 1".to_string(),
        category: Some("cat".to_string()),
    });
    dag.add_node(GraphNode {
        id: "chunk1".to_string(),
        node_type: NodeType::Chunk,
        title: "Chunk 1".to_string(),
        category: None,
    });
    dag.add_edge(GraphEdge {
        from: "doc1".to_string(),
        to: "chunk1".to_string(),
        edge_type: EdgeType::Parent,
        weight: 1.0,
    });

    let stats = dag.statistics();
    assert_eq!(stats.node_count, 2);
    assert_eq!(stats.document_count, 1);
    assert_eq!(stats.chunk_count, 1);
    assert_eq!(stats.edge_count, 1);
}

#[test]
fn test_default_dag() {
    let dag = KnowledgeDAG::default();
    assert_eq!(dag.nodes().len(), 0);
}

#[test]
fn test_empty_graph() {
    let dag = KnowledgeDAG::new();
    assert_eq!(dag.nodes().len(), 0);
    assert_eq!(dag.edges().len(), 0);
    assert_eq!(dag.topological_order().len(), 0);
    assert_eq!(dag.topo_node_indices().len(), 0);
    assert_eq!(dag.node_importance("anything"), 0.0);
    assert!(dag.reachable_from("anything").is_empty());
    let (ids, indices) = dag.topological_order_with_indices();
    assert!(ids.is_empty());
    assert!(indices.is_empty());
    let stats = dag.statistics();
    assert_eq!(stats.node_count, 0);
    assert_eq!(stats.edge_count, 0);
}

#[test]
fn test_cyclic_graph_fallback() {
    let mut dag = KnowledgeDAG::new();
    for id in ["x", "y", "z"] {
        dag.add_node(GraphNode {
            id: id.to_string(),
            node_type: NodeType::Chunk,
            title: id.to_string(),
            category: None,
        });
    }
    dag.add_edge(GraphEdge {
        from: "x".to_string(),
        to: "y".to_string(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });
    dag.add_edge(GraphEdge {
        from: "y".to_string(),
        to: "z".to_string(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });
    dag.add_edge(GraphEdge {
        from: "z".to_string(),
        to: "x".to_string(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });

    let topo = dag.topo_node_indices();
    assert_eq!(topo.len(), 3);

    let topo_order = dag.topological_order();
    assert_eq!(topo_order.len(), 3);
}

#[test]
fn test_disconnected_components() {
    let mut dag = KnowledgeDAG::new();
    for id in ["comp1_a", "comp1_b", "comp2_a", "comp2_b"] {
        dag.add_node(GraphNode {
            id: id.to_string(),
            node_type: NodeType::Chunk,
            title: id.to_string(),
            category: None,
        });
    }
    dag.add_edge(GraphEdge {
        from: "comp1_a".to_string(),
        to: "comp1_b".to_string(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });
    dag.add_edge(GraphEdge {
        from: "comp2_a".to_string(),
        to: "comp2_b".to_string(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });

    let reachable = dag.reachable_from("comp1_a");
    assert_eq!(reachable.len(), 2);
    assert!(!reachable.contains("comp2_a"));
    assert!(!reachable.contains("comp2_b"));

    let reachable_comp2 = dag.reachable_from("comp2_a");
    assert_eq!(reachable_comp2.len(), 2);
    assert!(!reachable_comp2.contains("comp1_a"));
}
