use super::*;

#[test]
fn test_batch_reachable_diamond_graph() {
    // Diamond: n0 -> n1, n0 -> n2, n1 -> n3, n2 -> n3
    let mut dag = KnowledgeDAG::new();
    for id in ["n0", "n1", "n2", "n3"] {
        dag.add_node(GraphNode {
            id: id.to_string(),
            node_type: NodeType::Chunk,
            title: id.to_string(),
            category: None,
        });
    }
    dag.add_edge(GraphEdge {
        from: "n0".into(),
        to: "n1".into(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });
    dag.add_edge(GraphEdge {
        from: "n0".into(),
        to: "n2".into(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });
    dag.add_edge(GraphEdge {
        from: "n1".into(),
        to: "n3".into(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });
    dag.add_edge(GraphEdge {
        from: "n2".into(),
        to: "n3".into(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });

    let result = dag.batch_reachable(&["n3".into(), "n1".into(), "n2".into()], None);
    assert!(result.contains_key("n3"));
    assert_eq!(result["n3"].len(), 3);
    assert!(result["n3"].contains(&"n0".into()));
    assert!(result["n3"].contains(&"n1".into()));
    assert!(result["n3"].contains(&"n2".into()));
    assert!(result.contains_key("n1"));
    assert_eq!(result["n1"].len(), 1);
    assert!(result["n1"].contains(&"n0".into()));
    assert!(result.contains_key("n2"));
    assert_eq!(result["n2"].len(), 1);
    assert!(result["n2"].contains(&"n0".into()));
}

#[test]
fn test_edges_by_type() {
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
    dag.add_edge(GraphEdge {
        from: "a".to_string(),
        to: "b".to_string(),
        edge_type: EdgeType::Related,
        weight: 0.5,
    });

    let sequential = dag.edges_by_type(&EdgeType::Sequential);
    assert_eq!(sequential.len(), 1);
    assert_eq!(sequential[0].edge_type, EdgeType::Sequential);

    let related = dag.edges_by_type(&EdgeType::Related);
    assert_eq!(related.len(), 1);

    let parent = dag.edges_by_type(&EdgeType::Parent);
    assert!(parent.is_empty());
}

#[test]
fn test_edge_type_display() {
    assert_eq!(format!("{}", EdgeType::Sequential), "sequential");
    assert_eq!(format!("{}", EdgeType::Parent), "parent");
    assert_eq!(format!("{}", EdgeType::Hierarchical), "hierarchical");
    assert_eq!(format!("{}", EdgeType::Related), "related");
    assert_eq!(format!("{}", EdgeType::References), "references");
    assert_eq!(format!("{}", EdgeType::ReferencedBy), "referenced_by");
    assert_eq!(format!("{}", EdgeType::CoAuthored), "co_authored");
}

#[test]
fn test_edge_discriminants() {
    assert_eq!(EdgeTypeKind::Sequential as usize, 0);
    assert_eq!(EdgeTypeKind::Parent as usize, 1);
    assert_eq!(EdgeTypeKind::Hierarchical as usize, 2);
    assert_eq!(EdgeTypeKind::Related as usize, 3);
    assert_eq!(EdgeTypeKind::References as usize, 4);
    assert_eq!(EdgeTypeKind::ReferencedBy as usize, 5);
    assert_eq!(EdgeTypeKind::CoAuthored as usize, 6);
}

#[test]
fn test_add_edge_missing_nodes() {
    let mut dag = KnowledgeDAG::new();
    dag.add_node(GraphNode {
        id: "exists".to_string(),
        node_type: NodeType::Chunk,
        title: "Exists".to_string(),
        category: None,
    });

    dag.add_edge(GraphEdge {
        from: "exists".to_string(),
        to: "missing".to_string(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });
    assert_eq!(dag.edges().len(), 0);

    dag.add_edge(GraphEdge {
        from: "missing".to_string(),
        to: "exists".to_string(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });
    assert_eq!(dag.edges().len(), 0);
}
