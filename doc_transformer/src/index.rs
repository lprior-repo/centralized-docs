use crate::analyze::Analysis;
use crate::assign::IdMapping;
use crate::chunking_adapter::{Chunk, ChunksResult};
use crate::graph::{EdgeType, GraphEdge, GraphNode, KnowledgeDAG, NodeType};
use crate::search;
use crate::types::is_stopword;
use anyhow::Result;
use contextual_chunker::ChunkType;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexDocument {
    pub id: String,
    pub title: String,
    pub path: String,
    pub category: String,
    pub tags: Vec<String>,
    pub summary: String,
    pub word_count: usize,
    pub chunk_ids: Vec<String>,
    pub headings: Vec<String>,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkMetadata {
    pub chunk_id: String,
    pub doc_id: String,
    pub doc_title: String,
    pub heading: Option<String>,
    #[serde(default)]
    pub heading_path: Vec<String>,
    #[serde(default)]
    pub heading_anchor: Option<String>,
    pub chunk_type: ChunkType,
    pub token_count: usize,
    pub summary: String,
    pub previous_chunk_id: Option<String>,
    pub next_chunk_id: Option<String>,
    #[serde(default)]
    pub section_index: usize,
    pub path: String,
    /// Related chunks with similarity scores (populated from knowledge DAG)
    pub related_chunks: Vec<RelatedChunk>,
    /// Hierarchical chunk level (summary/standard/detailed)
    pub chunk_level: contextual_chunker::ChunkLevel,
    /// Parent chunk ID (for hierarchical navigation)
    pub parent_chunk_id: Option<String>,
    /// Child chunk IDs (for hierarchical navigation)
    pub child_chunk_ids: Vec<String>,
    #[serde(default)]
    pub sibling_chunk_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedChunk {
    pub chunk_id: String,
    pub similarity: f32,
}

/// Intermediate result from document indexing phase
#[derive(Debug)]
struct DocumentIndexResult {
    documents: Vec<IndexDocument>,
    keywords: HashMap<String, Vec<String>>,
    document_tags: Vec<(String, Vec<String>, String)>,
}

/// Build and write the search index to disk.
///
/// # Errors
///
/// Returns an error if the index cannot be built or written to the specified directory.
#[allow(clippy::too_many_arguments)]
pub fn build_and_write_index<S: std::hash::BuildHasher>(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping, S>,
    chunks_result: &ChunksResult,
    output_dir: &Path,
    project_name: &str,
    max_related_chunks: Option<usize>,
    hnsw_m: Option<usize>,
    hnsw_ef_construction: Option<usize>,
    max_chunk_keywords: Option<usize>,
) -> Result<()> {
    // Phase 1: Build document index and extract metadata
    let doc_index = build_document_index(analyses, link_map, chunks_result);

    // Phase 2: Build knowledge graph
    let dag = build_knowledge_dag(
        &doc_index.documents,
        &chunks_result.chunks_metadata,
        &doc_index.document_tags,
        max_related_chunks,
        hnsw_m,
        hnsw_ef_construction,
        max_chunk_keywords,
    )?;

    // Phase 3: Build chunk metadata with related chunks from DAG
    let chunks_metadata = build_chunk_metadata(&chunks_result.chunks_metadata, &dag)?;

    // Phase 4: Compute graph analytics
    let analytics = compute_graph_analytics(&dag, &doc_index.documents);

    // Phase 5: Assemble and write index JSON
    let ctx = IndexAssemblyContext {
        documents: &doc_index.documents,
        chunks_metadata: &chunks_metadata,
        keywords: &doc_index.keywords,
        dag: &dag,
        analytics: &analytics,
        total_chunks: chunks_result.total_chunks,
        project_name,
    };
    let index_json = assemble_index_json(&ctx);
    write_index_file(output_dir, &index_json)?;

    // Phase 6: Build Tantivy index (optional, best-effort)
    build_tantivy_index(
        output_dir,
        &doc_index.documents,
        &chunks_result.chunks_metadata,
    )?;

    Ok(())
}

/// Build document index from analyses and link mapping.
///
/// Extracts documents, keywords, and tags for downstream processing.
/// This is a pure data transformation with no I/O.
#[allow(clippy::implicit_hasher)]
fn build_document_index<S: std::hash::BuildHasher>(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping, S>,
    chunks_result: &ChunksResult,
) -> DocumentIndexResult {
    let analyses_with_mapping: Vec<_> = analyses
        .iter()
        .filter_map(|analysis| {
            link_map
                .get(&analysis.source_path)
                .map(|mapping| (analysis, mapping))
        })
        .collect();

    let documents: Vec<IndexDocument> = analyses_with_mapping
        .iter()
        .map(|(analysis, mapping)| {
            let tags = extract_tags(analysis);

            let chunk_ids: Vec<String> = chunks_result
                .chunks_metadata
                .iter()
                .filter(|c| c.doc_id == mapping.id)
                .map(|c| c.chunk_id.clone())
                .collect();

            IndexDocument {
                id: mapping.id.clone(),
                title: analysis.title.clone(),
                path: format!("docs/{}", mapping.filename),
                category: analysis.category.clone(),
                tags: tags.clone(),
                summary: analysis.first_paragraph.clone(),
                word_count: analysis.word_count,
                chunk_ids,
                headings: analysis.headings.iter().map(|h| h.text.clone()).collect(),
                content: analysis.content.clone(),
            }
        })
        .collect();

    let document_tags: Vec<(String, Vec<String>, String)> = analyses_with_mapping
        .iter()
        .map(|(analysis, mapping)| {
            (
                mapping.id.clone(),
                extract_tags(analysis),
                analysis.category.clone(),
            )
        })
        .collect();

    let keywords: HashMap<String, Vec<String>> = analyses_with_mapping
        .iter()
        .flat_map(|(analysis, mapping)| {
            analysis.headings.iter().flat_map(move |heading| {
                heading.text.split_whitespace().filter_map(move |word| {
                    let word_lower = word.to_lowercase();
                    if word_lower.len() > 3 && !is_stopword(&word_lower) {
                        Some((word_lower, mapping.id.clone()))
                    } else {
                        None
                    }
                })
            })
        })
        .fold(HashMap::new(), |mut acc, (word, id)| {
            acc.entry(word).or_default().push(id);
            acc
        });

    DocumentIndexResult {
        documents,
        keywords,
        document_tags,
    }
}

/// Build chunk metadata enriched with related chunks from the knowledge graph.
///
/// This is a pure data transformation - no I/O performed.
fn build_chunk_metadata(chunks: &[Chunk], dag: &KnowledgeDAG) -> Result<Vec<ChunkMetadata>> {
    // Check for duplicate chunk_ids (BEAD-012 fix)
    chunks
        .iter()
        .try_fold(std::collections::HashSet::new(), |mut seen_ids, chunk| {
            if !seen_ids.insert(&chunk.chunk_id) {
                anyhow::bail!("Duplicate chunk_id found: {}", chunk.chunk_id);
            }
            Ok(seen_ids)
        })?;

    let siblings_map: HashMap<String, Vec<String>> = chunks
        .iter()
        .map(|chunk| {
            (
                format!("{}::{}", chunk.doc_id, chunk.chunk_level.as_str()),
                chunk.chunk_id.clone(),
            )
        })
        .into_group_map();

    Ok(chunks
        .iter()
        .map(|chunk| {
            // Get related chunks from the DAG
            let related = dag.get_related_chunks(&chunk.chunk_id);
            let related_chunks: Vec<RelatedChunk> = related
                .into_iter()
                .take(5) // Limit to top 5 related chunks
                .map(|(id, similarity)| RelatedChunk {
                    chunk_id: id,
                    similarity,
                })
                .collect();

            let heading_path = if chunk.heading_path.is_empty() {
                vec!["Intro".to_string()]
            } else {
                chunk.heading_path.clone()
            };
            let heading_anchor = heading_path
                .last()
                .filter(|heading| heading.as_str() != "Intro")
                .map(|heading| slugify_heading(heading));
            let sibling_key = format!("{}::{}", chunk.doc_id, chunk.chunk_level.as_str());
            let sibling_chunk_ids = siblings_map
                .get(&sibling_key)
                .map(|ids| {
                    ids.iter()
                        .filter(|id| *id != &chunk.chunk_id)
                        .cloned()
                        .collect()
                })
                .unwrap_or_default();

            ChunkMetadata {
                chunk_id: chunk.chunk_id.clone(),
                doc_id: chunk.doc_id.clone(),
                doc_title: chunk.doc_title.clone(),
                heading: chunk.heading.clone(),
                heading_path,
                heading_anchor,
                chunk_type: chunk.chunk_type,
                token_count: chunk.token_count,
                summary: chunk.summary.clone(),
                previous_chunk_id: chunk.previous_chunk_id.clone(),
                next_chunk_id: chunk.next_chunk_id.clone(),
                section_index: chunk.chunk_index,
                path: format!(
                    "chunks/{}-{}.md",
                    chunk.chunk_id.replace(['/', '#'], "-"),
                    chunk.chunk_level.as_str()
                ),
                related_chunks,
                chunk_level: chunk.chunk_level,
                parent_chunk_id: chunk.parent_chunk_id.clone(),
                child_chunk_ids: chunk.child_chunk_ids.clone(),
                sibling_chunk_ids,
            }
        })
        .collect())
}

fn slugify_heading(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == ' ' {
                c
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}

/// Graph analytics computed from the knowledge DAG.
#[derive(Debug)]
struct GraphAnalytics {
    topo_order: Vec<String>,
    reachability: HashMap<String, Vec<String>>,
    node_importance: HashMap<String, f32>,
}

/// Compute topological order, reachability, and node importance from the DAG.
///
/// This is a pure computation - no I/O performed.
fn compute_graph_analytics(dag: &KnowledgeDAG, documents: &[IndexDocument]) -> GraphAnalytics {
    let topo_order = dag.topological_order();

    let (reachability, node_importance): (HashMap<String, Vec<String>>, HashMap<String, f32>) =
        documents
            .iter()
            .map(|doc| {
                let reachable = dag.reachable_from(&doc.id);
                let reachable_list: Vec<String> = reachable
                    .into_iter()
                    .filter(|id| id != &doc.id)
                    .sorted()
                    .collect();

                (
                    (doc.id.clone(), reachable_list),
                    (doc.id.clone(), dag.node_importance(&doc.id)),
                )
            })
            .unzip();

    GraphAnalytics {
        topo_order,
        reachability,
        node_importance,
    }
}

/// Context for index JSON assembly - groups related parameters.
struct IndexAssemblyContext<'a> {
    documents: &'a [IndexDocument],
    chunks_metadata: &'a [ChunkMetadata],
    keywords: &'a HashMap<String, Vec<String>>,
    dag: &'a KnowledgeDAG,
    analytics: &'a GraphAnalytics,
    total_chunks: usize,
    project_name: &'a str,
}

/// Assemble the complete index JSON structure.
///
/// This is a pure data transformation - no I/O performed.
fn assemble_index_json(ctx: &IndexAssemblyContext<'_>) -> serde_json::Value {
    let dag_stats = ctx.dag.statistics();
    let timestamp = chrono::Utc::now().to_rfc3339();
    let avg_chunk_size_tokens = ctx
        .chunks_metadata
        .iter()
        .map(|c| c.token_count)
        .sum::<usize>()
        .checked_div(ctx.total_chunks)
        .unwrap_or(0);

    json!({
        "version": "5.0",
        "project": ctx.project_name,
        "updated": timestamp,
        "metadata": {
            "generated_at": timestamp,
            "generator": "doc_transformer",
            "schema": "index-v5"
        },
        "stats": {
            "doc_count": ctx.documents.len(),
            "chunk_count": ctx.total_chunks,
            "avg_chunk_size_tokens": avg_chunk_size_tokens,
            "graph": {
                "node_count": dag_stats.node_count,
                "edge_count": dag_stats.edge_count,
                "sequential_edges": dag_stats.sequential_edges,
                "related_edges": dag_stats.related_edges,
                "reference_edges": dag_stats.reference_edges
            }
        },
        "documents": ctx.documents,
        "chunks": ctx.chunks_metadata,
        "keywords": ctx.keywords,
        "graph": {
            "nodes": ctx.dag.nodes(),
            "edges": ctx.dag.edges(),
            "topological_order": ctx.analytics.topo_order,
            "reachability": ctx.analytics.reachability,
            "node_importance": ctx.analytics.node_importance,
            "statistics": dag_stats
        },
        "navigation": {
            "type": "contextual_retrieval_with_dag",
            "strategy": "50-100 token context prefix + H2/H3/H1 boundaries + knowledge DAG with semantic similarity",
            "avg_tokens_per_chunk": avg_chunk_size_tokens,
            "graph_enabled": true,
            "similarity_metric": "weighted_terms_on_tags_heading_summary",
            "min_similarity_threshold": 0.3
        }
    })
}

/// Write the index JSON to disk.
fn write_index_file(output_dir: &Path, index: &serde_json::Value) -> Result<()> {
    let index_file = output_dir.join("INDEX.json");
    fs::write(index_file, serde_json::to_string_pretty(index)?)
        .map_err(|e| anyhow::anyhow!("Failed to write INDEX.json: {e}"))
}

/// Build Tantivy full-text search index.
///
/// This is a best-effort operation - failure only logs a warning
/// since search can fall back to INDEX.json.
fn build_tantivy_index(
    output_dir: &Path,
    documents: &[IndexDocument],
    chunks: &[crate::chunking_adapter::Chunk],
) -> Result<()> {
    search::open_or_create_index(output_dir)
        .and_then(|index| {
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

/// Writes documentation compass file with category-based navigation
/// # Errors
/// Returns error if file writing fails
#[allow(clippy::implicit_hasher)]
pub fn build_and_write_compass<S: std::hash::BuildHasher>(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping, S>,
    output_dir: &Path,
) -> Result<()> {
    let by_category: HashMap<String, Vec<(String, String, Vec<String>)>> = analyses
        .iter()
        .filter_map(|analysis| {
            link_map.get(&analysis.source_path).map(|mapping| {
                let tags = extract_tags(analysis);
                (
                    analysis.category.clone(),
                    (analysis.title.clone(), mapping.filename.clone(), tags),
                )
            })
        })
        .into_group_map();

    let compass_content = ["tutorial", "concept", "ref", "ops", "meta"]
        .into_iter()
        .filter_map(|category| {
            by_category.get(category).map(|docs| {
                let section_docs = docs.iter().take(5).fold(
                    String::new(),
                    |mut output, (title, filename, tags)| {
                        let tag_str = tags
                            .iter()
                            .take(2)
                            .map(|t| format!("`{t}`"))
                            .collect::<Vec<_>>()
                            .join(" ");
                        use std::fmt::Write;
                        let _ = writeln!(output, "- [{title}](./docs/{filename}) {tag_str}");
                        output
                    },
                );
                format!("## {}\n\n{}\n", category.to_uppercase(), section_docs)
            })
        })
        .collect::<String>();

    let compass_content = format!(
        "# Documentation Compass\n\n> **{} documents**\n\n{}",
        analyses.len(),
        compass_content
    );

    let compass_file = output_dir.join("COMPASS.md");
    fs::write(compass_file, compass_content)?;

    Ok(())
}

/// Extract tags using functional composition
fn extract_tags(analysis: &Analysis) -> Vec<String> {
    std::iter::once(analysis.category.clone())
        .chain(
            analysis
                .headings
                .iter()
                .take(3)
                .flat_map(|h| h.text.split_whitespace())
                .filter(|word| word.len() > 4 && !is_stopword(&word.to_lowercase()))
                .map(str::to_lowercase),
        )
        .sorted()
        .dedup()
        .take(5)
        .collect()
}

/// Build a knowledge graph DAG from documents and chunks
#[allow(clippy::too_many_arguments)]
/// Build knowledge DAG from documents and chunks
/// # Errors
/// Returns error if DAG construction fails
#[allow(clippy::too_many_lines)]
pub fn build_knowledge_dag(
    documents: &[IndexDocument],
    chunks: &[Chunk],
    _document_tags: &[(String, Vec<String>, String)],
    _max_related_chunks: Option<usize>,
    _hnsw_m: Option<usize>,
    _hnsw_ef_construction: Option<usize>,
    _max_chunk_keywords: Option<usize>,
) -> Result<KnowledgeDAG> {
    let dag = KnowledgeDAG::new();

    // Add document nodes
    let dag = documents.iter().fold(dag, |mut dag: KnowledgeDAG, doc| {
        let node = GraphNode {
            id: doc.id.clone(),
            node_type: NodeType::Document,
            title: doc.title.clone(),
            category: Some(doc.category.clone()),
        };
        dag.add_node(node);
        dag
    });

    // Add chunk nodes
    let dag = chunks.iter().fold(dag, |mut dag: KnowledgeDAG, chunk| {
        let node = GraphNode {
            id: chunk.chunk_id.clone(),
            node_type: NodeType::Chunk,
            title: format!(
                "{} - {}",
                chunk.doc_title,
                chunk.heading.as_ref().unwrap_or(&"Intro".to_string())
            ),
            category: None,
        };
        dag.add_node(node);
        dag
    });

    // Add parent-child edges (document -> chunks)
    let dag = chunks.iter().fold(dag, |mut dag: KnowledgeDAG, chunk| {
        let edge = GraphEdge {
            from: chunk.doc_id.clone(),
            to: chunk.chunk_id.clone(),
            edge_type: EdgeType::Parent,
            weight: 1.0,
        };
        dag.add_edge(edge);
        dag
    });

    // Add sequential edges (previous -> next chunks)
    let dag = chunks.iter().fold(dag, |mut dag: KnowledgeDAG, chunk| {
        if let Some(next_id) = &chunk.next_chunk_id {
            let edge = GraphEdge {
                from: chunk.chunk_id.clone(),
                to: next_id.to_string(),
                edge_type: EdgeType::Sequential,
                weight: 1.0,
            };
            dag.add_edge(edge);
        }
        dag
    });

    // Detect and add related chunk edges using HNSW (O(n log n) instead of O(n²))
    // We used to do sparse BOW embeddings via HNSW here, but it's a terrible idea for sparse vectors!
    // Since we don't have dense embeddings yet, we'll skip adding related edges to the DAG for now.
    // When we integrate real embeddings (like text-embedding-3-small), we will re-enable this.

    let dag = dag;

    Ok(dag)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic)]

    use super::*;
    use crate::chunking_adapter::Chunk;
    use contextual_chunker::ChunkLevel;
    use std::collections::HashMap;

    /// Test that chunk metadata has no duplicate `chunk_ids`
    /// This verifies the fix for BEAD-012
    #[test]
    fn test_chunk_metadata_no_duplicate_ids() {
        // Create test chunks with some duplicates
        let chunks = vec![
            Chunk {
                chunk_id: "doc1#0-standard".to_string(),
                doc_id: "doc1".to_string(),
                doc_title: "Doc 1".to_string(),
                chunk_index: 0,
                content: "Content 1".to_string(),
                token_count: 100,
                heading: Some("Section 1".to_string()),
                heading_path: vec!["Doc 1".to_string()],
                chunk_type: contextual_chunker::ChunkType::Prose,
                previous_chunk_id: None,
                next_chunk_id: Some("doc1#1-standard".to_string()),
                related_chunk_ids: vec![],
                summary: "Summary 1".to_string(),
                chunk_level: ChunkLevel::Standard,
                parent_chunk_id: None,
                child_chunk_ids: vec![],
                context_prefix: None,
            },
            Chunk {
                chunk_id: "doc1#1-standard".to_string(),
                doc_id: "doc1".to_string(),
                doc_title: "Doc 1".to_string(),
                chunk_index: 1,
                content: "Content 2".to_string(),
                token_count: 100,
                heading: Some("Section 2".to_string()),
                heading_path: vec!["Doc 1".to_string()],
                chunk_type: contextual_chunker::ChunkType::Prose,
                previous_chunk_id: Some("doc1#0-standard".to_string()),
                next_chunk_id: None,
                related_chunk_ids: vec![],
                summary: "Summary 2".to_string(),
                chunk_level: ChunkLevel::Standard,
                parent_chunk_id: None,
                child_chunk_ids: vec![],
                context_prefix: None,
            },
            // Intentionally add duplicate to test detection
            Chunk {
                chunk_id: "doc1#0-standard".to_string(), // DUPLICATE ID
                doc_id: "doc1".to_string(),
                doc_title: "Doc 1".to_string(),
                chunk_index: 0,
                content: "Content 1".to_string(),
                token_count: 100,
                heading: Some("Section 1".to_string()),
                heading_path: vec!["Doc 1".to_string()],
                chunk_type: contextual_chunker::ChunkType::Prose,
                previous_chunk_id: None,
                next_chunk_id: Some("doc1#1-standard".to_string()),
                related_chunk_ids: vec![],
                summary: "Summary 1".to_string(),
                chunk_level: ChunkLevel::Standard,
                parent_chunk_id: None,
                child_chunk_ids: vec![],
                context_prefix: None,
            },
        ];

        // Build chunk metadata - this should detect duplicates and fail
        let result = build_chunk_metadata(&chunks, &KnowledgeDAG::new());

        // Should return an error due to duplicate chunk_id
        match result {
            Err(e) => {
                let err_msg = e.to_string();
                assert!(
                    err_msg.contains("Duplicate chunk_id"),
                    "Error should mention duplicate chunk_id"
                );
                assert!(
                    err_msg.contains("doc1#0-standard"),
                    "Error should mention the specific duplicate ID"
                );
            }
            Ok(_) => panic!("Should fail when duplicate chunk_ids exist"),
        }
    }

    #[test]
    fn test_empty_chunks_no_crash() {
        let documents = vec![];
        let chunks = vec![];
        let document_tags = vec![];

        let dag = match build_knowledge_dag(
            &documents,
            &chunks,
            &document_tags,
            None,
            None,
            None,
            None,
        ) {
            Ok(d) => d,
            Err(e) => panic!("Failed to build knowledge DAG with empty chunks: {e}"),
        };

        let stats = dag.statistics();
        assert_eq!(stats.node_count, 0);
        assert_eq!(stats.edge_count, 0);
    }
}
