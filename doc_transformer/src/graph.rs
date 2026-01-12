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
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, EnumDiscriminants)]
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
    pub nodes_vec: Vec<GraphNode>,
    pub edges_vec: Vec<GraphEdge>,
}

/// Edge data for petgraph
#[derive(Debug, Clone)]
struct GraphEdgeData {
    /// Edge type for graph queries (used by get_edge_data)
    #[allow(dead_code)]
    edge_type: EdgeType,
    weight: f32,
}

impl KnowledgeDAG {
    /// Create a new empty DAG
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
        let idx = self.graph.add_node(node.clone());
        self.node_map.insert(node.id.clone(), idx);
        self.nodes_vec.push(node);
    }

    /// Add an edge to the graph
    pub fn add_edge(&mut self, edge: GraphEdge) {
        if let (Some(&from_idx), Some(&to_idx)) =
            (self.node_map.get(&edge.from), self.node_map.get(&edge.to))
        {
            self.graph.add_edge(
                from_idx,
                to_idx,
                GraphEdgeData {
                    edge_type: edge.edge_type.clone(),
                    weight: edge.weight,
                },
            );
            self.edges_vec.push(edge);
        }
    }

    /// Get all edges of a specific type
    pub fn edges_by_type(&self, edge_type: &EdgeType) -> Vec<&GraphEdge> {
        self.edges_vec
            .iter()
            .filter(|e| &e.edge_type == edge_type)
            .collect()
    }

    /// Get edge properties between two nodes from the petgraph
    /// Returns (edge_type, weight) if edge exists
    #[allow(dead_code)]
    pub fn get_edge_data(&self, from: &str, to: &str) -> Option<(EdgeType, f32)> {
        let from_idx = self.node_map.get(from)?;
        let to_idx = self.node_map.get(to)?;

        // Find edge in petgraph
        for edge in self.graph.edges(*from_idx) {
            if edge.target() == *to_idx {
                let data = edge.weight();
                return Some((data.edge_type.clone(), data.weight));
            }
        }
        None
    }

    /// Get total edge weight for a node (sum of outgoing edge weights)
    pub fn node_importance(&self, node_id: &str) -> f32 {
        if let Some(&idx) = self.node_map.get(node_id) {
            self.graph.edges(idx)
                .map(|e| e.weight().weight)
                .sum()
        } else {
            0.0
        }
    }

    /// Find related chunks for a given chunk (via semantic links)
    /// Uses functional composition with itertools for sorted results
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
    pub fn topological_order(&self) -> Vec<String> {
        toposort(&self.graph, None)
            .map(|sorted| {
                sorted
                    .into_iter()
                    .filter_map(|idx| self.graph.node_weight(idx).map(|node| node.id.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all nodes reachable from a given node (transitive closure)
    pub fn reachable_from(&self, node_id: &str) -> HashSet<String> {
        let mut visited = HashSet::new();
        if let Some(&start_idx) = self.node_map.get(node_id) {
            self._dfs_reachable(start_idx, &mut visited);
        }
        visited
    }

    fn _dfs_reachable(&self, idx: NodeIndex, visited: &mut HashSet<String>) {
        if let Some(node) = self.graph.node_weight(idx) {
            if !visited.insert(node.id.clone()) {
                return; // Already visited
            }

            for edge in self.graph.edges(idx) {
                let target_idx = edge.target();
                self._dfs_reachable(target_idx, visited);
            }
        }
    }

    /// Calculate graph statistics using functional composition
    pub fn statistics(&self) -> GraphStatistics {
        // Count nodes by type using partition
        let (documents, chunks): (Vec<_>, Vec<_>) = self
            .nodes_vec
            .iter()
            .partition(|n| n.node_type == NodeType::Document);

        // Count edges by type using functional style
        let edge_counts = [EdgeType::Sequential, EdgeType::Related, EdgeType::References]
            .into_iter()
            .map(|t| self.edges_by_type(&t).len())
            .collect::<Vec<_>>();

        GraphStatistics {
            node_count: self.nodes_vec.len(),
            document_count: documents.len(),
            chunk_count: chunks.len(),
            edge_count: self.edges_vec.len(),
            sequential_edges: edge_counts[0],
            related_edges: edge_counts[1],
            reference_edges: edge_counts[2],
        }
    }

    /// Get nodes as vector for serialization
    pub fn nodes(&self) -> &[GraphNode] {
        &self.nodes_vec
    }

    /// Get edges as vector for serialization
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

/// Relationship suggestion between chunks based on common tags/categories
pub struct RelationshipDetector {
    pub min_similarity: f32,
}

impl RelationshipDetector {
    pub fn new(min_similarity: f32) -> Self {
        Self { min_similarity }
    }

    /// Calculate Jaccard similarity between two tag sets using functional composition
    pub fn jaccard_similarity(tags1: &[String], tags2: &[String]) -> f32 {
        if tags1.is_empty() && tags2.is_empty() {
            return 1.0;
        }

        let set1: HashSet<_> = tags1.iter().collect();
        let set2: HashSet<_> = tags2.iter().collect();

        (set1.intersection(&set2).count() as f32, set1.union(&set2).count() as f32)
            .pipe(|(intersection, union)| {
                if union == 0.0 {
                    0.0
                } else {
                    intersection / union
                }
            })
    }

    /// Detect relationships between chunks based on metadata using functional composition
    pub fn detect_relationships(
        &self,
        chunk_id: &str,
        chunk_tags: &[String],
        chunk_category: &str,
        all_chunks: &[(String, Vec<String>, String)],
    ) -> Vec<(String, f32)> {
        all_chunks
            .iter()
            .filter(|(id, _, _)| id != chunk_id)
            .filter_map(|(id, tags, category)| {
                let tag_similarity = Self::jaccard_similarity(chunk_tags, tags);
                let category_match = if category == chunk_category { 0.3 } else { 0.0 };

                (tag_similarity * 0.7 + category_match)
                    .pipe(|combined| {
                        (combined >= self.min_similarity).then(|| (id.clone(), combined))
                    })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(dag.nodes_vec.len(), 1);
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
        assert_eq!(dag.edges_vec.len(), 1);
    }

    #[test]
    fn test_jaccard_similarity() {
        let tags1 = vec!["rust".to_string(), "cue".to_string()];
        let tags2 = vec!["rust".to_string(), "tour".to_string()];

        let similarity = RelationshipDetector::jaccard_similarity(&tags1, &tags2);
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
                id: format!("node{}", i),
                node_type: NodeType::Chunk,
                title: format!("Node {}", i),
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
    fn test_get_edge_data() {
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
            edge_type: EdgeType::Related,
            weight: 0.75,
        });

        let edge_data = dag.get_edge_data("a", "b");
        assert!(edge_data.is_some());
        let (edge_type, weight) = edge_data.expect("edge should exist");
        assert_eq!(edge_type, EdgeType::Related);
        assert!((weight - 0.75).abs() < 0.001);

        // Non-existent edge
        assert!(dag.get_edge_data("b", "a").is_none());
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
                id: format!("spoke{}", i),
                node_type: NodeType::Chunk,
                title: format!("Spoke {}", i),
                category: None,
            });

            dag.add_edge(GraphEdge {
                from: "hub".to_string(),
                to: format!("spoke{}", i),
                edge_type: EdgeType::Parent,
                weight: 0.5,
            });
        }

        let importance = dag.node_importance("hub");
        assert!((importance - 1.5).abs() < 0.001); // 3 edges * 0.5 weight

        let no_importance = dag.node_importance("nonexistent");
        assert_eq!(no_importance, 0.0);
    }
}
