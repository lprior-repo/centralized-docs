use itertools::Itertools;
use petgraph::algo::toposort;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use strum::EnumDiscriminants;
use tap::Pipe;

/// Node in the knowledge graph - represents a document or chunk
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct GraphNode {
    pub id: String,
    pub node_type: NodeType,
    pub title: String,
    pub category: Option<String>,
}

/// Type of graph node
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum NodeType {
    Document,
    Chunk,
}

/// Edge in the knowledge graph - represents a relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
    pub edge_type: EdgeType,
    pub weight: f32, // 0.0-1.0, higher = stronger relationship
}

/// Types of edges in the graph
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, EnumDiscriminants)]
#[strum_discriminants(name(EdgeTypeKind))]
#[serde(rename_all = "snake_case")]
pub enum EdgeType {
    Sequential,   // Next chunk in document (natural order)
    Parent,       // Document contains chunk
    Hierarchical, // Higher-level organization
    Related,      // Topically related (semantic similarity)
    References,   // Explicit link in document
    ReferencedBy, // Document links to this one
    CoAuthored,   // Share tags or category
}

impl std::fmt::Display for EdgeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EdgeType::Sequential => write!(f, "sequential"),
            EdgeType::Parent => write!(f, "parent"),
            EdgeType::Hierarchical => write!(f, "hierarchical"),
            EdgeType::Related => write!(f, "related"),
            EdgeType::References => write!(f, "references"),
            EdgeType::ReferencedBy => write!(f, "referenced_by"),
            EdgeType::CoAuthored => write!(f, "co_authored"),
        }
    }
}

/// Directed Acyclic Graph for knowledge representation using petgraph
pub struct KnowledgeDAG {
    graph: DiGraph<GraphNode, GraphEdgeData>,
    node_map: HashMap<String, NodeIndex>,
    nodes_vec: Vec<GraphNode>,
    edges_vec: Vec<GraphEdge>,
}

/// Edge data for petgraph
#[derive(Debug, Clone)]
struct GraphEdgeData {
    #[allow(dead_code)] // Stored for graph structure, not currently accessed
    edge_type: EdgeType,
    weight: f32,
}

impl KnowledgeDAG {
    /// Create a new empty DAG
    #[must_use]
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
            nodes_vec: Vec::new(),
            edges_vec: Vec::new(),
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: GraphNode) {
        let id = node.id.clone();
        let idx = self.graph.add_node(node.clone());
        self.node_map.insert(id, idx);
        self.nodes_vec.push(node);
    }

    /// Add an edge to the graph
    pub fn add_edge(&mut self, edge: GraphEdge) {
        let from_idx = self.node_map.get(&edge.from).copied();
        let to_idx = self.node_map.get(&edge.to).copied();

        if let (Some(from), Some(to)) = (from_idx, to_idx) {
            self.graph.add_edge(
                from,
                to,
                GraphEdgeData {
                    edge_type: edge.edge_type.clone(),
                    weight: edge.weight,
                },
            );
            self.edges_vec.push(edge);
        }
    }

    /// Get all edges of a specific type
    #[must_use]
    pub fn edges_by_type(&self, edge_type: &EdgeType) -> Vec<&GraphEdge> {
        self.edges_vec
            .iter()
            .filter(|e| &e.edge_type == edge_type)
            .collect()
    }

    /// Get total edge weight for a node (sum of outgoing edge weights)
    #[must_use]
    pub fn node_importance(&self, node_id: &str) -> f32 {
        if let Some(&idx) = self.node_map.get(node_id) {
            self.graph.edges(idx).map(|e| e.weight().weight).sum()
        } else {
            0.0
        }
    }

    /// Find related chunks for a given chunk (via semantic links)
    /// Uses functional composition with itertools for sorted results
    #[must_use]
    pub fn get_related_chunks(&self, chunk_id: &str) -> Vec<(String, f32)> {
        self.edges_vec
            .iter()
            .filter(|edge| edge.from == chunk_id && edge.edge_type == EdgeType::Related)
            .map(|edge| (edge.to.clone(), edge.weight))
            .sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal))
            .collect()
    }

    /// Get topologically sorted nodes (respects dependencies)
    /// Uses functional composition for cleaner flow
    #[must_use]
    pub fn topological_order(&self) -> Vec<String> {
        if let Ok(sorted) = toposort(&self.graph, None) {
            sorted
                .into_iter()
                .filter_map(|idx| self.graph.node_weight(idx).map(|node| node.id.clone()))
                .collect()
        } else {
            eprintln!(
                "Warning: Graph contains cycles, using deterministic fallback order by node id"
            );
            self.nodes_vec
                .iter()
                .map(|node| node.id.clone())
                .sorted()
                .collect()
        }
    }

    /// Get all nodes reachable from a given node (transitive closure)
    #[must_use]
    pub fn reachable_from(&self, node_id: &str) -> HashSet<String> {
        self.node_map
            .get(node_id)
            .map(|&start_idx| {
                let mut visited = HashSet::new();
                self.dfs_reachable(start_idx, &mut visited);
                visited
            })
            .unwrap_or_default()
    }

    fn dfs_reachable(&self, idx: NodeIndex, visited: &mut HashSet<String>) {
        if let Some(node) = self.graph.node_weight(idx) {
            if !visited.insert(node.id.clone()) {
                return; // Already visited
            }

            for edge in self.graph.edges(idx) {
                let target_idx = edge.target();
                self.dfs_reachable(target_idx, visited);
            }
        }
    }

    /// Check if adding an edge from source to target would create a cycle,
    /// considering only specific edge types for the path check.
    /// Returns true if adding the edge would create a cycle.
    #[must_use]
    pub fn would_create_cycle_with_edge_types(
        &self,
        source: &str,
        target: &str,
        edge_types: &[EdgeType],
    ) -> bool {
        // If there's already a path from target to source via the specified edge types,
        // adding source->target would create a cycle
        self.reachable_from_via_edge_types(target, edge_types)
            .contains(source)
    }

    /// Get all nodes reachable from a given node using only specific edge types.
    #[must_use]
    pub fn reachable_from_via_edge_types(
        &self,
        node_id: &str,
        edge_types: &[EdgeType],
    ) -> HashSet<String> {
        let edge_type_set: HashSet<EdgeType> = edge_types.iter().cloned().collect();

        self.node_map
            .get(node_id)
            .map(|&start_idx| {
                let mut visited = HashSet::new();
                self.dfs_reachable_with_edge_types(start_idx, &edge_type_set, &mut visited);
                visited
            })
            .unwrap_or_default()
    }

    fn dfs_reachable_with_edge_types(
        &self,
        idx: NodeIndex,
        edge_types: &HashSet<EdgeType>,
        visited: &mut HashSet<String>,
    ) {
        if let Some(node) = self.graph.node_weight(idx) {
            if !visited.insert(node.id.clone()) {
                return; // Already visited
            }

            // Find edges from this node that match the allowed edge types in O(E_node)
            for edge in self.graph.edges(idx) {
                if edge_types.contains(&edge.weight().edge_type) {
                    self.dfs_reachable_with_edge_types(edge.target(), edge_types, visited);
                }
            }
        }
    }

    /// Calculate graph statistics using functional composition
    #[must_use]
    pub fn statistics(&self) -> GraphStatistics {
        // Count nodes by type using partition
        let (documents, chunks): (Vec<_>, Vec<_>) = self
            .nodes_vec
            .iter()
            .partition(|n| n.node_type == NodeType::Document);

        // Count edges by type using functional style
        let [sequential_edges, related_edges, reference_edges] = [
            EdgeType::Sequential,
            EdgeType::Related,
            EdgeType::References,
        ]
        .map(|t| self.edges_by_type(&t).len());

        GraphStatistics {
            node_count: self.nodes_vec.len(),
            document_count: documents.len(),
            chunk_count: chunks.len(),
            edge_count: self.edges_vec.len(),
            sequential_edges,
            related_edges,
            reference_edges,
        }
    }

    /// Get nodes as vector for serialization
    #[must_use]
    pub fn nodes(&self) -> &[GraphNode] {
        &self.nodes_vec
    }

    /// Get edges as vector for serialization
    #[must_use]
    pub fn edges(&self) -> &[GraphEdge] {
        &self.edges_vec
    }
}

impl Default for KnowledgeDAG {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about the graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphStatistics {
    pub node_count: usize,
    pub document_count: usize,
    pub chunk_count: usize,
    pub edge_count: usize,
    pub sequential_edges: usize,
    pub related_edges: usize,
    pub reference_edges: usize,
}

/// Calculate Jaccard similarity between two tag sets using functional composition
///
/// Returns 1.0 if both tag sets are empty (considered identical).
/// Returns Jaccard coefficient (intersection / union) otherwise.
///
/// # Examples
///
/// ```
/// # use doc_transformer::graph::jaccard_similarity;
/// let tags1 = vec!["rust".to_string(), "async".to_string()];
/// let tags2 = vec!["rust".to_string(), "tokio".to_string()];
/// let similarity = jaccard_similarity(&tags1, &tags2);
/// assert!((similarity - 0.333).abs() < 0.01); // 1 common / 3 total
/// ```
#[allow(dead_code)]
#[allow(clippy::cast_precision_loss)]
#[must_use]
pub fn jaccard_similarity(tags1: &[String], tags2: &[String]) -> f32 {
    if tags1.is_empty() && tags2.is_empty() {
        return 1.0;
    }

    let set1: HashSet<_> = tags1.iter().collect();
    let set2: HashSet<_> = tags2.iter().collect();

    // SAFETY: Tag counts are small (< 100 typically), well within f32 precision (2^24)
    (
        set1.intersection(&set2).count() as f32,
        set1.union(&set2).count() as f32,
    )
        .pipe(|(intersection, union)| {
            if union == 0.0 {
                0.0
            } else {
                intersection / union
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Property 1: Commutativity - jaccard(a, b) == jaccard(b, a)
    proptest! {
        #[test]
        fn prop_jaccard_commutativity(
            tags1 in prop::collection::vec(".*", 0..20),
            tags2 in prop::collection::vec(".*", 0..20)
        ) {
            let vec1: Vec<String> = tags1.into_iter().map(|s| s.to_string()).collect();
            let vec2: Vec<String> = tags2.into_iter().map(|s| s.to_string()).collect();

            let result1 = jaccard_similarity(&vec1, &vec2);
            let result2 = jaccard_similarity(&vec2, &vec1);

            prop_assert_eq!(result1, result2);
        }
    }

    // Property 2: Reflexivity - jaccard(a, a) == 1.0
    proptest! {
        #[test]
        fn prop_jaccard_reflexivity(tags in prop::collection::vec(".*", 0..20)) {
            let vec: Vec<String> = tags.into_iter().map(|s| s.to_string()).collect();
            let result = jaccard_similarity(&vec, &vec);

            prop_assert_eq!(result, 1.0);
        }
    }

    // Property 3: Bounds - result always in [0.0, 1.0]
    proptest! {
        #[test]
        fn prop_jaccard_bounds(
            tags1 in prop::collection::vec(".*", 0..20),
            tags2 in prop::collection::vec(".*", 0..20)
        ) {
            let vec1: Vec<String> = tags1.into_iter().map(|s| s.to_string()).collect();
            let vec2: Vec<String> = tags2.into_iter().map(|s| s.to_string()).collect();

            let result = jaccard_similarity(&vec1, &vec2);

            prop_assert!(result >= 0.0);
            prop_assert!(result <= 1.0);
        }
    }

    // Property 4: Empty sets - jaccard([], []) == 1.0
    #[test]
    fn prop_jaccard_both_empty() {
        let empty: Vec<String> = vec![];
        let result = jaccard_similarity(&empty, &empty);

        assert_eq!(result, 1.0);
    }

    // Property 5: Disjoint sets - jaccard(a, b) == 0.0 when no shared elements
    proptest! {
        #[test]
        fn prop_jaccard_disjoint_sets(
            prefix1 in "[a-m]{1,5}",
            prefix2 in "[n-z]{1,5}",
            count in 1..10usize
        ) {
            // Generate disjoint sets by using different alphabetic ranges
            let set1: Vec<String> = (0..count)
                .map(|i| format!("{prefix1}{i}"))
                .collect();
            let set2: Vec<String> = (0..count)
                .map(|i| format!("{prefix2}{i}"))
                .collect();

            let result = jaccard_similarity(&set1, &set2);

            prop_assert_eq!(result, 0.0);
        }
    }

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
    fn test_jaccard_similarity() {
        let tags1 = vec!["rust".to_string(), "cue".to_string()];
        let tags2 = vec!["rust".to_string(), "tour".to_string()];

        let similarity = jaccard_similarity(&tags1, &tags2);
        // Intersection: ["rust"] = 1
        // Union: ["rust", "cue", "tour"] = 3
        // Jaccard = 1/3 ≈ 0.333
        assert!((similarity - 0.333).abs() < 0.01);
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
    fn test_cycle_detection_sequential_chain() {
        let mut dag = KnowledgeDAG::new();

        dag.add_node(GraphNode {
            id: "c0".to_string(),
            node_type: NodeType::Chunk,
            title: "Chunk 0".to_string(),
            category: None,
        });
        dag.add_node(GraphNode {
            id: "c1".to_string(),
            node_type: NodeType::Chunk,
            title: "Chunk 1".to_string(),
            category: None,
        });
        dag.add_node(GraphNode {
            id: "c2".to_string(),
            node_type: NodeType::Chunk,
            title: "Chunk 2".to_string(),
            category: None,
        });

        // Sequential chain: c0 -> c1 -> c2
        dag.add_edge(GraphEdge {
            from: "c0".to_string(),
            to: "c1".to_string(),
            edge_type: EdgeType::Sequential,
            weight: 1.0,
        });
        dag.add_edge(GraphEdge {
            from: "c1".to_string(),
            to: "c2".to_string(),
            edge_type: EdgeType::Sequential,
            weight: 1.0,
        });

        // Check: adding c2 -> c0 would create cycle
        let would_cycle = dag.would_create_cycle_with_edge_types(
            "c2",
            "c0",
            &[EdgeType::Sequential, EdgeType::Parent],
        );
        assert!(
            would_cycle,
            "c2->c0 should create cycle via sequential chain"
        );

        // Check: adding c0 -> c2 is fine (no cycle)
        let would_cycle = dag.would_create_cycle_with_edge_types(
            "c0",
            "c2",
            &[EdgeType::Sequential, EdgeType::Parent],
        );
        assert!(!would_cycle, "c0->c2 should not create cycle");
    }

    #[test]
    fn test_cycle_detection_parent_edges() {
        let mut dag = KnowledgeDAG::new();

        // Document -> chunks
        dag.add_node(GraphNode {
            id: "doc".to_string(),
            node_type: NodeType::Document,
            title: "Doc".to_string(),
            category: None,
        });
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

        // doc -> chunk1 -> chunk2 (parent chain)
        dag.add_edge(GraphEdge {
            from: "doc".to_string(),
            to: "chunk1".to_string(),
            edge_type: EdgeType::Parent,
            weight: 1.0,
        });
        dag.add_edge(GraphEdge {
            from: "chunk1".to_string(),
            to: "chunk2".to_string(),
            edge_type: EdgeType::Sequential,
            weight: 1.0,
        });

        // Adding chunk2 -> doc would create cycle via parent edges
        let would_cycle = dag.would_create_cycle_with_edge_types(
            "chunk2",
            "doc",
            &[EdgeType::Sequential, EdgeType::Parent],
        );
        assert!(would_cycle, "chunk2->doc should create cycle via parent");
    }

    #[test]
    fn test_related_edges_no_cycle() {
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
        dag.add_node(GraphNode {
            id: "c".to_string(),
            node_type: NodeType::Chunk,
            title: "C".to_string(),
            category: None,
        });

        // a -> b (sequential)
        dag.add_edge(GraphEdge {
            from: "a".to_string(),
            to: "b".to_string(),
            edge_type: EdgeType::Sequential,
            weight: 1.0,
        });

        // Adding b -> c (related) should be fine
        let would_cycle = dag.would_create_cycle_with_edge_types(
            "b",
            "c",
            &[EdgeType::Sequential, EdgeType::Parent],
        );
        assert!(!would_cycle, "b->c should not create cycle");
    }
}
