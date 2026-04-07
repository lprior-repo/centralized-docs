//! Index assembly: graph analytics, JSON serialization, file persistence, and Tantivy search index.

use super::types::{ChunkMetadata, IndexDocument};
use crate::chunking_adapter::Chunk;
use crate::graph::KnowledgeDAG;
use crate::search;
use anyhow::Result;
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;

/// Graph analytics computed from the knowledge DAG.
pub(crate) struct GraphAnalytics {
    pub topo_order: Vec<String>,
    pub reachability: HashMap<String, Vec<String>>,
    pub node_importance: HashMap<String, f32>,
}

pub(crate) fn compute_graph_analytics(
    dag: &KnowledgeDAG,
    documents: &[IndexDocument],
) -> GraphAnalytics {
    let (topo_order, topo_node_indices) = dag.topological_order_with_indices();
    let doc_ids: Vec<String> = documents.iter().map(|doc| doc.id.clone()).collect();
    let reachability = dag.batch_reachable(&doc_ids, Some(topo_node_indices));
    let node_importance: HashMap<String, f32> = documents
        .iter()
        .map(|doc| (doc.id.clone(), dag.node_importance(&doc.id)))
        .collect();
    GraphAnalytics {
        topo_order,
        reachability,
        node_importance,
    }
}

pub(crate) fn assemble_index_json(
    documents: &[IndexDocument],
    chunks_metadata: &[ChunkMetadata],
    keywords: &HashMap<String, Vec<String>>,
    dag: &KnowledgeDAG,
    analytics: &GraphAnalytics,
    total_chunks: usize,
    project_name: &str,
) -> serde_json::Value {
    let dag_stats = dag.statistics();
    let timestamp = chrono::Utc::now().to_rfc3339();
    let avg_chunk_size_tokens = chunks_metadata
        .iter()
        .map(|c| c.token_count)
        .sum::<usize>()
        .checked_div(total_chunks)
        .map_or(0, |v| v);
    json!({
        "version": "5.0", "project": project_name, "updated": timestamp,
        "metadata": { "generated_at": timestamp, "generator": "ctd", "schema": "index-v5" },
        "stats": { "doc_count": documents.len(), "chunk_count": total_chunks, "avg_chunk_size_tokens": avg_chunk_size_tokens,
            "graph": { "node_count": dag_stats.node_count, "edge_count": dag_stats.edge_count,
                "sequential_edges": dag_stats.sequential_edges, "related_edges": dag_stats.related_edges,
                "reference_edges": dag_stats.reference_edges } },
        "documents": documents, "chunks": chunks_metadata, "keywords": keywords,
        "graph": { "nodes": dag.nodes(), "edges": dag.edges(), "topological_order": analytics.topo_order,
            "reachability": analytics.reachability, "node_importance": analytics.node_importance, "statistics": dag_stats },
        "navigation": { "type": "contextual_retrieval_with_dag",
            "strategy": "50-100 token context prefix + H2/H3/H1 boundaries + knowledge DAG with semantic similarity",
            "avg_tokens_per_chunk": avg_chunk_size_tokens, "graph_enabled": true,
            "similarity_metric": "weighted_terms_on_tags_heading_summary", "min_similarity_threshold": 0.3 }
    })
}

pub(crate) fn write_index_file(output_dir: &Path, index: &serde_json::Value) -> Result<()> {
    let index_file = output_dir.join("INDEX.json");
    let file = std::fs::File::create(index_file)
        .map_err(|e| anyhow::anyhow!("Failed to create INDEX.json: {e}"))?;
    serde_json::to_writer_pretty(file, index)
        .map_err(|e| anyhow::anyhow!("Failed to write INDEX.json: {e}"))
}

pub(crate) fn build_tantivy_index(
    output_dir: &Path,
    documents: &[IndexDocument],
    chunks: &[Chunk],
) -> Result<()> {
    search::open_or_create_index(output_dir)
        .and_then(|index| {
            #[allow(unused_mut)]
            let mut writer = index
                .writer(50_000_000)
                .map_err(|e| anyhow::anyhow!("Failed to create writer: {e}"))?;
            search::index_chunks(&mut writer, documents, chunks)
                .map_err(|e| anyhow::anyhow!("Indexing failed: {e}"))?;
            writer
                .commit()
                .map_err(|e| anyhow::anyhow!("Commit failed: {e}"))?;
            Ok(())
        })
        .map_err(|e| {
            eprintln!("Error: Failed to build Tantivy index: {e}");
            eprintln!("Search will fall back to INDEX.json, but will be slower");
            anyhow::anyhow!("Tantivy index build failed (non-fatal): {e}")
        })
}
