//! `KnowledgeDAG`: directed acyclic graph for knowledge representation using petgraph.

use super::{EdgeType, GraphEdge, GraphNode, NodeType};
use itertools::Itertools;
use petgraph::algo::toposort;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::HashMap;

/// Edge data for petgraph
#[derive(Debug, Clone)]
struct GraphEdgeData {
    #[allow(dead_code)] // Stored for graph structure, not currently accessed
    edge_type: EdgeType,
    weight: f32,
}

/// Directed Acyclic Graph for knowledge representation using petgraph
pub struct KnowledgeDAG {
    graph: DiGraph<GraphNode, GraphEdgeData>,
    node_map: HashMap<String, NodeIndex>,
    nodes_vec: Vec<GraphNode>,
    edges_vec: Vec<GraphEdge>,
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

    /// Add a node to the graph.
    pub fn add_node(&mut self, node: GraphNode) {
        let id = node.id.clone();
        let idx = self.graph.add_node(node.clone());
        self.node_map.insert(id, idx);
        self.nodes_vec.push(node);
    }

    /// Add an edge to the graph (silently ignores if source/target nodes are missing).
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

    /// Get all edges of a specific type.
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
    #[must_use]
    pub fn topological_order_with_indices(&self) -> (Vec<String>, Vec<NodeIndex>) {
        let indices = self.topo_node_indices();
        let ids = indices
            .iter()
            .filter_map(|&idx| self.graph.node_weight(idx).map(|node| node.id.clone()))
            .collect();
        (ids, indices)
    }

    /// Get topologically sorted nodes (respects dependencies).
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
    pub fn reachable_from(&self, node_id: &str) -> std::collections::HashSet<String> {
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
    ) -> std::collections::HashSet<String> {
        let edge_type_set: std::collections::HashSet<EdgeType> =
            edge_types.iter().cloned().collect();
        self.node_map
            .get(node_id)
            .map(|&start_idx| {
                #[allow(unused_mut)]
                let mut visited = std::collections::HashSet::new();
                self.dfs_reachable_with_edge_types(start_idx, &edge_type_set, &mut visited);
                visited
            })
            .unwrap_or_default()
    }

    #[allow(dead_code)]
    fn dfs_reachable_with_edge_types(
        &self,
        idx: NodeIndex,
        edge_types: &std::collections::HashSet<EdgeType>,
        visited: &mut std::collections::HashSet<String>,
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

    /// Batch reachability: O(V+E) single forward-topo pass using `im::HashSet`.
    #[must_use]
    pub fn batch_reachable(
        &self,
        source_ids: &[String],
        topo_order: Option<Vec<NodeIndex>>,
    ) -> HashMap<String, Vec<String>> {
        let topo = topo_order.map_or_else(|| self.topo_node_indices(), std::convert::identity);
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

    /// Calculate graph statistics using functional composition.
    #[must_use]
    pub fn statistics(&self) -> super::GraphStatistics {
        let (documents, chunks): (Vec<_>, Vec<_>) = self
            .nodes_vec
            .iter()
            .partition(|n| n.node_type == NodeType::Document);
        let edge_type_counts: HashMap<EdgeType, usize> = self
            .edges_vec
            .iter()
            .map(|e| (e.edge_type.clone(), ()))
            .into_group_map()
            .into_iter()
            .map(|(k, v)| (k, v.len()))
            .collect();
        super::GraphStatistics {
            node_count: self.nodes_vec.len(),
            document_count: documents.len(),
            chunk_count: chunks.len(),
            edge_count: self.edges_vec.len(),
            sequential_edges: edge_type_counts
                .get(&EdgeType::Sequential)
                .map_or(0, |v| *v),
            related_edges: edge_type_counts.get(&EdgeType::Related).map_or(0, |v| *v),
            reference_edges: edge_type_counts
                .get(&EdgeType::References)
                .map_or(0, |v| *v),
        }
    }

    /// Get nodes as vector for serialization.
    #[must_use]
    pub fn nodes(&self) -> &[GraphNode] {
        &self.nodes_vec
    }

    /// Get edges as vector for serialization.
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
