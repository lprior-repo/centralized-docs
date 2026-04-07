//! Knowledge DAG construction.

use super::types::IndexDocument;
use crate::chunking_adapter::Chunk;
use crate::graph::{EdgeType, GraphEdge, GraphNode, KnowledgeDAG, NodeType};
use anyhow::Result;

/// Build a knowledge graph DAG from documents and chunks.
#[allow(clippy::too_many_arguments)]
pub fn build_knowledge_dag(
    documents: &[IndexDocument],
    chunks: &[Chunk],
    _document_tags: &[(String, Vec<String>, String)],
    _max_related_chunks: Option<usize>,
    _hnsw_m: Option<usize>,
    _hnsw_ef_construction: Option<usize>,
    _max_chunk_keywords: Option<usize>,
) -> Result<KnowledgeDAG> {
    let dag = documents
        .iter()
        .fold(KnowledgeDAG::new(), |mut dag: KnowledgeDAG, doc| {
            dag.add_node(GraphNode {
                id: doc.id.clone(),
                node_type: NodeType::Document,
                title: doc.title.clone(),
                category: Some(doc.category.clone()),
            });
            dag
        });

    let chunk_nodes: Vec<GraphNode> = chunks
        .iter()
        .map(|chunk| GraphNode {
            id: chunk.chunk_id.clone(),
            node_type: NodeType::Chunk,
            title: match &chunk.heading {
                Some(h) => format!("{} - {}", chunk.doc_title, h),
                None => format!("{} - Intro", chunk.doc_title),
            },
            category: None,
        })
        .collect();

    let parent_edges: Vec<GraphEdge> = chunks
        .iter()
        .map(|chunk| GraphEdge {
            from: chunk.doc_id.clone(),
            to: chunk.chunk_id.clone(),
            edge_type: EdgeType::Parent,
            weight: 1.0,
        })
        .collect();

    let sequential_edges: Vec<GraphEdge> = chunks
        .iter()
        .filter_map(|chunk| {
            chunk.next_chunk_id.as_ref().map(|next_id| GraphEdge {
                from: chunk.chunk_id.clone(),
                to: next_id.clone(),
                edge_type: EdgeType::Sequential,
                weight: 1.0,
            })
        })
        .collect();

    let dag = chunk_nodes.into_iter().fold(dag, |mut dag, node| {
        dag.add_node(node);
        dag
    });
    let dag = parent_edges.into_iter().fold(dag, |mut dag, edge| {
        dag.add_edge(edge);
        dag
    });
    let dag = sequential_edges.into_iter().fold(dag, |mut dag, edge| {
        dag.add_edge(edge);
        dag
    });

    Ok(dag)
}
