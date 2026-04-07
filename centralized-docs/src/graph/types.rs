//! Graph domain types: nodes, edges, edge types, statistics, and similarity.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
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
