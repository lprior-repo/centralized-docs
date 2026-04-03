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
    #[allow(dead_code)]
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

    /// Build a pre-indexed lookup map for related-chunk queries.
    /// Keyed by `chunk_id`, each value is the sorted list of (target, weight) pairs.
    /// O(E) build, O(1) per-chunk lookup — replaces O(C×E) linear scan.
    #[must_use]
    pub fn build_related_chunks_index(&self) -> HashMap<&str, Vec<(String, f32)>> {
        self.edges_vec
            .iter()
            .filter(|edge| edge.edge_type == EdgeType::Related)
            .map(|edge| (edge.from.as_str(), (edge.to.clone(), edge.weight)))
            .into_group_map()
            .into_iter()
            .map(|(from, mut pairs)| {
                pairs.sort_by(|a, b| b.1.total_cmp(&a.1));
                (from, pairs)
            })
            .collect()
    }

    /// Compute the raw topological ordering as `NodeIndex` values.
    /// Shared by `topological_order` and `batch_reachable` to avoid duplicate traversal.
    #[must_use]
    pub fn topo_node_indices(&self) -> Vec<NodeIndex> {
        match toposort(&self.graph, None) {
            Ok(order) => order,
            Err(_) => self
                .nodes_vec
                .iter()
                .filter_map(|n| self.node_map.get(&n.id).copied())
                .collect(),
        }
    }

    /// Compute topological order returning both node IDs and their `NodeIndex` values.
    /// Use this when you need the order for display AND for further graph traversal.
    #[must_use]
    pub fn topological_order_with_indices(&self) -> (Vec<String>, Vec<NodeIndex>) {
        let indices = self.topo_node_indices();
        let ids = indices
            .iter()
            .filter_map(|&idx| self.graph.node_weight(idx).map(|node| node.id.clone()))
            .collect();
        (ids, indices)
    }

    /// Get topologically sorted nodes (respects dependencies)
    /// Uses functional composition for cleaner flow
    #[allow(dead_code)]
    #[must_use]
    pub fn topological_order(&self) -> Vec<String> {
        let (ids, _) = self.topological_order_with_indices();
        ids
    }

    /// Get all nodes reachable from a given node (transitive closure).
    ///
    /// Note: prefer `batch_reachable` for O(V+E) multi-source queries.
    /// This method does O(V+E) per call — use it only for single-node queries.
    #[must_use]
    #[allow(dead_code)]
    pub fn reachable_from(&self, node_id: &str) -> HashSet<String> {
        self.node_map
            .get(node_id)
            .map(|&start_idx| {
                self.dfs_reachable(start_idx, im::HashSet::new())
                    .into_iter()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Persistent DFS — no `mut`, threads `im::HashSet` through recursion
    fn dfs_reachable(&self, idx: NodeIndex, visited: im::HashSet<String>) -> im::HashSet<String> {
        self.graph.node_weight(idx).map_or(visited.clone(), |node| {
            if visited.contains(&node.id) {
                visited
            } else {
                let visited = visited.update(node.id.clone());
                self.graph
                    .edges(idx)
                    .fold(visited, |vis, edge| self.dfs_reachable(edge.target(), vis))
            }
        })
    }

    /// Get all nodes reachable from a given node using only specific edge types.
    #[allow(dead_code)]
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
                #[allow(unused_mut)] // petgraph DFS requires &mut visited set
                let mut visited = HashSet::new();
                self.dfs_reachable_with_edge_types(start_idx, &edge_type_set, &mut visited);
                visited
            })
            .unwrap_or_default()
    }

    #[allow(dead_code)]
    fn dfs_reachable_with_edge_types(
        &self,
        idx: NodeIndex,
        edge_types: &HashSet<EdgeType>,
        visited: &mut HashSet<String>,
    ) {
        if let Some(node) = self.graph.node_weight(idx) {
            if !visited.insert(node.id.clone()) {
                return;
            }

            self.graph
                .edges(idx)
                .filter(|edge| edge_types.contains(&edge.weight().edge_type))
                .for_each(|edge| {
                    self.dfs_reachable_with_edge_types(edge.target(), edge_types, visited);
                });
        }
    }

    /// Batch reachability: O(V+E) single forward-topo pass.
    ///
    /// Uses `im::HashSet` for O(1) structural-sharing `.union()` instead of
    /// cloning entire sets per node. Thread-safe via Arc internals.
    ///
    /// If `topo_order` is `Some`, uses it directly instead of computing a new
    /// topological sort. Pass `None` to compute internally.
    #[must_use]
    pub fn batch_reachable(
        &self,
        source_ids: &[String],
        topo_order: Option<Vec<NodeIndex>>,
    ) -> HashMap<String, Vec<String>> {
        let topo = topo_order.map_or_else(|| self.topo_node_indices(), std::convert::identity);

        // Forward pass: each node inherits predecessors' reachability via im::HashSet union
        let reachable: im::HashMap<NodeIndex, im::HashSet<String>> =
            topo.into_iter().fold(im::HashMap::new(), |accum, idx| {
                let self_id = self
                    .graph
                    .node_weight(idx)
                    .map_or_else(String::new, |n| n.id.clone());
                let base = accum
                    .get(&idx)
                    .cloned()
                    .map_or_else(im::HashSet::new, std::convert::identity)
                    .update(self_id);
                let merged = self
                    .graph
                    .edges_directed(idx, petgraph::Direction::Incoming)
                    .fold(base, |vis, edge| {
                        accum
                            .get(&edge.source())
                            .map_or(vis.clone(), |pred| vis.union(pred.clone()))
                    });
                accum.update(idx, merged)
            });

        // Extract only requested source_ids
        source_ids
            .iter()
            .filter_map(|id| {
                self.node_map.get(id.as_str()).and_then(|&idx| {
                    reachable.get(&idx).map(|r| {
                        let list: Vec<String> = r
                            .iter()
                            .filter(|rid| *rid != id)
                            .cloned()
                            .sorted()
                            .collect();
                        (id.clone(), list)
                    })
                })
            })
            .collect()
    }

    /// Calculate graph statistics using functional composition
    #[must_use]
    pub fn statistics(&self) -> GraphStatistics {
        // Count nodes by type using partition
        let (documents, chunks): (Vec<_>, Vec<_>) = self
            .nodes_vec
            .iter()
            .partition(|n| n.node_type == NodeType::Document);

        // Count edges by type in a single O(E) pass
        let edge_type_counts: HashMap<EdgeType, usize> = self
            .edges_vec
            .iter()
            .map(|e| (e.edge_type.clone(), ()))
            .into_group_map()
            .into_iter()
            .map(|(k, v)| (k, v.len()))
            .collect();
        let sequential_edges = edge_type_counts
            .get(&EdgeType::Sequential)
            .map_or(0, |v| *v);
        let related_edges = edge_type_counts.get(&EdgeType::Related).map_or(0, |v| *v);
        let reference_edges = edge_type_counts
            .get(&EdgeType::References)
            .map_or(0, |v| *v);

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
#[allow(clippy::float_cmp, clippy::map_clone)]
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
            let vec1: Vec<String> = tags1.into_iter().map(|s| s.clone()).collect();
            let vec2: Vec<String> = tags2.into_iter().map(|s| s.clone()).collect();

            let result1 = jaccard_similarity(&vec1, &vec2);
            let result2 = jaccard_similarity(&vec2, &vec1);

            prop_assert_eq!(result1, result2);
        }
    }

    // Property 2: Reflexivity - jaccard(a, a) == 1.0
    proptest! {
        #[test]
        fn prop_jaccard_reflexivity(tags in prop::collection::vec(".*", 0..20)) {
            let vec: Vec<String> = tags.into_iter().map(|s| s.clone()).collect();
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
            let vec1: Vec<String> = tags1.into_iter().map(|s| s.clone()).collect();
            let vec2: Vec<String> = tags2.into_iter().map(|s| s.clone()).collect();

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
        // n3 can be reached from n0, n1, n2 — so querying n3 gives ancestors.
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
        // b has ancestor a
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

        // leaf has ancestors: src1, src2, shared
        let result = dag.batch_reachable(&["leaf".to_string(), "shared".to_string()], None);
        assert_eq!(result["leaf"].len(), 3);
        assert_eq!(result["shared"].len(), 2);
    }

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
            from: "n0".to_string(),
            to: "n1".to_string(),
            edge_type: EdgeType::Sequential,
            weight: 1.0,
        });
        dag.add_edge(GraphEdge {
            from: "n0".to_string(),
            to: "n2".to_string(),
            edge_type: EdgeType::Sequential,
            weight: 1.0,
        });
        dag.add_edge(GraphEdge {
            from: "n1".to_string(),
            to: "n3".to_string(),
            edge_type: EdgeType::Sequential,
            weight: 1.0,
        });
        dag.add_edge(GraphEdge {
            from: "n2".to_string(),
            to: "n3".to_string(),
            edge_type: EdgeType::Sequential,
            weight: 1.0,
        });

        let result = dag.batch_reachable(
            &["n3".to_string(), "n1".to_string(), "n2".to_string()],
            None,
        );

        // n3 has ancestors: n0, n1, n2 (diamond merge)
        assert!(result.contains_key("n3"), "n3 should be in result");
        let n3_reachable = &result["n3"];
        assert!(
            n3_reachable.contains(&"n0".to_string()),
            "n3 reachable from n0"
        );
        assert!(
            n3_reachable.contains(&"n1".to_string()),
            "n3 reachable from n1"
        );
        assert!(
            n3_reachable.contains(&"n2".to_string()),
            "n3 reachable from n2"
        );
        assert_eq!(n3_reachable.len(), 3);

        // n1 has ancestor: n0
        assert!(result.contains_key("n1"), "n1 should be in result");
        let n1_reachable = &result["n1"];
        assert!(
            n1_reachable.contains(&"n0".to_string()),
            "n1 reachable from n0"
        );
        assert_eq!(n1_reachable.len(), 1);

        // n2 has ancestor: n0
        assert!(result.contains_key("n2"), "n2 should be in result");
        let n2_reachable = &result["n2"];
        assert!(
            n2_reachable.contains(&"n0".to_string()),
            "n2 reachable from n0"
        );
        assert_eq!(n2_reachable.len(), 1);
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
}
