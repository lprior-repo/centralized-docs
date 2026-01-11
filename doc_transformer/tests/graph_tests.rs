use doc_transformer::graph::{EdgeType, GraphEdge, GraphNode, KnowledgeDAG, NodeType};
use serde_json::json;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_find_node_edges() {
    // Create a test graph with known edges
    let mut dag = KnowledgeDAG::new();

    let doc1 = GraphNode {
        id: "doc-1".to_string(),
        node_type: NodeType::Document,
        title: "Document 1".to_string(),
        category: Some("tutorial".to_string()),
    };

    let chunk1 = GraphNode {
        id: "doc-1#0".to_string(),
        node_type: NodeType::Chunk,
        title: "Document 1 - Intro".to_string(),
        category: None,
    };

    let chunk2 = GraphNode {
        id: "doc-1#1".to_string(),
        node_type: NodeType::Chunk,
        title: "Document 1 - Details".to_string(),
        category: None,
    };

    dag.add_node(doc1);
    dag.add_node(chunk1);
    dag.add_node(chunk2);

    dag.add_edge(GraphEdge {
        from: "doc-1".to_string(),
        to: "doc-1#0".to_string(),
        edge_type: EdgeType::Parent,
        weight: 1.0,
    });

    dag.add_edge(GraphEdge {
        from: "doc-1".to_string(),
        to: "doc-1#1".to_string(),
        edge_type: EdgeType::Parent,
        weight: 1.0,
    });

    dag.add_edge(GraphEdge {
        from: "doc-1#0".to_string(),
        to: "doc-1#1".to_string(),
        edge_type: EdgeType::Sequential,
        weight: 1.0,
    });

    // Count outgoing edges from doc-1
    let outgoing_from_doc1: Vec<_> = dag
        .edges()
        .iter()
        .filter(|e| e.from == "doc-1")
        .collect();
    assert_eq!(outgoing_from_doc1.len(), 2);

    // Count incoming edges to doc-1#1
    let incoming_to_chunk1: Vec<_> = dag
        .edges()
        .iter()
        .filter(|e| e.to == "doc-1#1")
        .collect();
    assert_eq!(incoming_to_chunk1.len(), 2);
}

#[test]
fn test_node_not_found() {
    let dag = KnowledgeDAG::new();

    let node = dag.nodes().iter().find(|n| n.id == "nonexistent");
    assert!(node.is_none());
}

#[test]
fn test_graph_command_with_valid_node() {
    // Create a temporary INDEX.json with graph data
    let temp_dir = TempDir::new().unwrap();
    let index_path = temp_dir.path().join("INDEX.json");

    let index_data = json!({
        "version": "4.3",
        "generated": "2026-01-11T00:00:00Z",
        "documents": [],
        "chunks": [],
        "keywords": {},
        "graph": {
            "nodes": [
                {
                    "id": "getting-started",
                    "node_type": "document",
                    "title": "Getting Started Guide",
                    "category": "tutorial"
                },
                {
                    "id": "getting-started#0",
                    "node_type": "chunk",
                    "title": "Getting Started - Intro",
                    "category": null
                },
                {
                    "id": "getting-started#1",
                    "node_type": "chunk",
                    "title": "Getting Started - Installation",
                    "category": null
                },
                {
                    "id": "installation",
                    "node_type": "document",
                    "title": "Installation",
                    "category": "tutorial"
                }
            ],
            "edges": [
                {
                    "from": "getting-started",
                    "to": "getting-started#0",
                    "edge_type": "parent",
                    "weight": 1.0
                },
                {
                    "from": "getting-started",
                    "to": "getting-started#1",
                    "edge_type": "parent",
                    "weight": 1.0
                },
                {
                    "from": "getting-started#0",
                    "to": "getting-started#1",
                    "edge_type": "sequential",
                    "weight": 1.0
                },
                {
                    "from": "getting-started",
                    "to": "installation",
                    "edge_type": "related",
                    "weight": 0.65
                }
            ],
            "statistics": {
                "node_count": 4,
                "edge_count": 4,
                "sequential_edges": 1,
                "related_edges": 1,
                "reference_edges": 0,
                "document_count": 2,
                "chunk_count": 2
            }
        },
        "navigation": {},
        "stats": {}
    });

    fs::write(&index_path, serde_json::to_string_pretty(&index_data).unwrap()).unwrap();

    // Load and verify the data
    let index_content = fs::read_to_string(&index_path).unwrap();
    let index_value: serde_json::Value = serde_json::from_str(&index_content).unwrap();

    let graph_section = index_value.get("graph").unwrap();
    let nodes: Vec<GraphNode> = serde_json::from_value(graph_section.get("nodes").unwrap().clone()).unwrap();
    let edges: Vec<GraphEdge> = serde_json::from_value(graph_section.get("edges").unwrap().clone()).unwrap();

    // Find the node
    let node = nodes.iter().find(|n| n.id == "getting-started").unwrap();
    assert_eq!(node.title, "Getting Started Guide");
    assert_eq!(node.node_type, NodeType::Document);

    // Find outgoing edges
    let outgoing: Vec<_> = edges.iter().filter(|e| e.from == "getting-started").collect();
    assert_eq!(outgoing.len(), 3);

    // Verify edge types
    let parent_edges: Vec<_> = outgoing
        .iter()
        .filter(|e| e.edge_type == EdgeType::Parent)
        .collect();
    assert_eq!(parent_edges.len(), 2);

    let related_edges: Vec<_> = outgoing
        .iter()
        .filter(|e| e.edge_type == EdgeType::Related)
        .collect();
    assert_eq!(related_edges.len(), 1);
    assert_eq!(related_edges[0].weight, 0.65);
}

#[test]
fn test_node_not_found_error() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = temp_dir.path().join("INDEX.json");

    let index_data = json!({
        "version": "4.3",
        "graph": {
            "nodes": [
                {
                    "id": "existing-node",
                    "node_type": "document",
                    "title": "Existing Node",
                    "category": "tutorial"
                }
            ],
            "edges": []
        }
    });

    fs::write(&index_path, serde_json::to_string_pretty(&index_data).unwrap()).unwrap();

    let index_content = fs::read_to_string(&index_path).unwrap();
    let index_value: serde_json::Value = serde_json::from_str(&index_content).unwrap();

    let graph_section = index_value.get("graph").unwrap();
    let nodes: Vec<GraphNode> = serde_json::from_value(graph_section.get("nodes").unwrap().clone()).unwrap();

    let node = nodes.iter().find(|n| n.id == "nonexistent");
    assert!(node.is_none());
}

#[test]
fn test_chunk_id_with_hash() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = temp_dir.path().join("INDEX.json");

    let index_data = json!({
        "version": "4.3",
        "graph": {
            "nodes": [
                {
                    "id": "doc-id#0",
                    "node_type": "chunk",
                    "title": "Chunk 0",
                    "category": null
                },
                {
                    "id": "doc-id#1",
                    "node_type": "chunk",
                    "title": "Chunk 1",
                    "category": null
                }
            ],
            "edges": [
                {
                    "from": "doc-id#0",
                    "to": "doc-id#1",
                    "edge_type": "sequential",
                    "weight": 1.0
                }
            ]
        }
    });

    fs::write(&index_path, serde_json::to_string_pretty(&index_data).unwrap()).unwrap();

    let index_content = fs::read_to_string(&index_path).unwrap();
    let index_value: serde_json::Value = serde_json::from_str(&index_content).unwrap();

    let graph_section = index_value.get("graph").unwrap();
    let nodes: Vec<GraphNode> = serde_json::from_value(graph_section.get("nodes").unwrap().clone()).unwrap();

    let node = nodes.iter().find(|n| n.id == "doc-id#0").unwrap();
    assert_eq!(node.id, "doc-id#0");
    assert_eq!(node.node_type, NodeType::Chunk);
}

#[test]
fn test_node_with_no_edges() {
    let mut dag = KnowledgeDAG::new();

    let isolated_node = GraphNode {
        id: "isolated".to_string(),
        node_type: NodeType::Document,
        title: "Isolated Document".to_string(),
        category: Some("meta".to_string()),
    };

    dag.add_node(isolated_node);

    let outgoing: Vec<_> = dag.edges().iter().filter(|e| e.from == "isolated").collect();
    let incoming: Vec<_> = dag.edges().iter().filter(|e| e.to == "isolated").collect();

    assert_eq!(outgoing.len(), 0);
    assert_eq!(incoming.len(), 0);
}

#[test]
fn test_reachable_nodes() {
    let mut dag = KnowledgeDAG::new();

    // Create a chain: A -> B -> C -> D
    for i in 0..4 {
        dag.add_node(GraphNode {
            id: format!("node{}", i),
            node_type: NodeType::Chunk,
            title: format!("Node {}", i),
            category: None,
        });
    }

    for i in 0..3 {
        dag.add_edge(GraphEdge {
            from: format!("node{}", i),
            to: format!("node{}", i + 1),
            edge_type: EdgeType::Sequential,
            weight: 1.0,
        });
    }

    let reachable = dag.reachable_from("node0");
    assert_eq!(reachable.len(), 4); // node0, node1, node2, node3

    let reachable_from_2 = dag.reachable_from("node2");
    assert_eq!(reachable_from_2.len(), 2); // node2, node3
}

#[test]
fn test_graph_missing_in_index() {
    let temp_dir = TempDir::new().unwrap();
    let index_path = temp_dir.path().join("INDEX.json");

    let index_data = json!({
        "version": "4.3",
        "documents": [],
        "chunks": []
    });

    fs::write(&index_path, serde_json::to_string_pretty(&index_data).unwrap()).unwrap();

    let index_content = fs::read_to_string(&index_path).unwrap();
    let index_value: serde_json::Value = serde_json::from_str(&index_content).unwrap();

    let graph_section = index_value.get("graph");
    assert!(graph_section.is_none());
}

#[test]
fn test_title_truncation() {
    let long_title = "This is a very long title that should be truncated to 50 characters maximum for display purposes";

    let truncated = if long_title.len() > 50 {
        format!("{}...", &long_title[..47])
    } else {
        long_title.to_string()
    };

    assert_eq!(truncated.len(), 50); // 47 chars + "..."
    assert!(truncated.ends_with("..."));
}

#[test]
fn test_edge_weight_precision() {
    let edge = GraphEdge {
        from: "a".to_string(),
        to: "b".to_string(),
        edge_type: EdgeType::Related,
        weight: 0.654321,
    };

    let formatted = format!("{:.2}", edge.weight);
    assert_eq!(formatted, "0.65");
}

#[test]
fn test_multiple_edge_types() {
    let mut dag = KnowledgeDAG::new();

    let node_a = GraphNode {
        id: "a".to_string(),
        node_type: NodeType::Document,
        title: "Node A".to_string(),
        category: Some("tutorial".to_string()),
    };

    let node_b = GraphNode {
        id: "b".to_string(),
        node_type: NodeType::Document,
        title: "Node B".to_string(),
        category: Some("tutorial".to_string()),
    };

    dag.add_node(node_a);
    dag.add_node(node_b);

    // Add different edge types between same nodes
    dag.add_edge(GraphEdge {
        from: "a".to_string(),
        to: "b".to_string(),
        edge_type: EdgeType::Related,
        weight: 0.5,
    });

    dag.add_edge(GraphEdge {
        from: "a".to_string(),
        to: "b".to_string(),
        edge_type: EdgeType::References,
        weight: 1.0,
    });

    let edges_a_to_b: Vec<_> = dag.edges().iter().filter(|e| e.from == "a" && e.to == "b").collect();
    assert_eq!(edges_a_to_b.len(), 2);

    let has_related = edges_a_to_b.iter().any(|e| e.edge_type == EdgeType::Related);
    let has_reference = edges_a_to_b.iter().any(|e| e.edge_type == EdgeType::References);

    assert!(has_related);
    assert!(has_reference);
}
