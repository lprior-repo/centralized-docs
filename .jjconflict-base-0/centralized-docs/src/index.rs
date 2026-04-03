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
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::sync::Arc;

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
    pub content: Arc<str>,
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
    // Pre-build doc_id → chunk_ids index for O(1) lookup (avoids O(A×C) scan)
    let doc_chunks_index: HashMap<&str, Vec<String>> = chunks_result
        .chunks_metadata
        .iter()
        .map(|chunk| (chunk.doc_id.as_str(), chunk.chunk_id.clone()))
        .into_group_map();

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

            let chunk_ids: Vec<String> = match doc_chunks_index.get(mapping.id.as_str()) {
                Some(ids) => ids.clone(),
                None => Vec::new(),
            };

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
        .into_group_map();

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
    let chunk_ids: Vec<&str> = chunks.iter().map(|c| c.chunk_id.as_str()).collect();
    let unique_ids: HashSet<&str> = chunk_ids.iter().copied().collect();
    if chunk_ids.len() != unique_ids.len() {
        let dup = chunk_ids
            .iter()
            .copied()
            .counts()
            .into_iter()
            .find(|(_, c)| *c > 1)
            .map(|(id, _)| id);
        match dup {
            Some(d) => anyhow::bail!("Duplicate chunk_id found: {d}"),
            None => anyhow::bail!("Duplicate chunk_id found"),
        }
    }

    let siblings_map: HashMap<String, Vec<String>> = chunks
        .iter()
        .map(|chunk| {
            (
                format!("{}::{}", chunk.doc_id, chunk.chunk_level.as_str()),
                chunk.chunk_id.clone(),
            )
        })
        .into_group_map();

    let related_index = dag.build_related_chunks_index();

    Ok(chunks
        .iter()
        .map(|chunk| {
            let related_chunks: Vec<RelatedChunk> = related_index
                .get(chunk.chunk_id.as_str())
                .map_or_else(Vec::new, |pairs| {
                    pairs
                        .iter()
                        .take(5)
                        .map(|(id, similarity)| RelatedChunk {
                            chunk_id: id.clone(),
                            similarity: *similarity,
                        })
                        .collect()
                });

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
            let sibling_chunk_ids = siblings_map.get(&sibling_key).map_or(Vec::new(), |ids| {
                ids.iter()
                    .filter(|id| *id != &chunk.chunk_id)
                    .cloned()
                    .collect()
            });

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
        .map_or(0, |v| v);

    json!({
        "version": "5.0",
        "project": ctx.project_name,
        "updated": timestamp,
        "metadata": {
            "generated_at": timestamp,
            "generator": "ctd",
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
    let file = std::fs::File::create(index_file)
        .map_err(|e| anyhow::anyhow!("Failed to create INDEX.json: {e}"))?;
    serde_json::to_writer_pretty(file, index)
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
            #[allow(unused_mut)] // tantivy IndexWriter API requires &mut self
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

/// Writes documentation navigation file with category-based navigation
/// # Errors
/// Returns error if file writing fails
#[allow(clippy::implicit_hasher)]
pub fn build_and_write_navigation<S: std::hash::BuildHasher>(
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

    let navigation_content = ["tutorial", "concept", "ref", "ops", "meta"]
        .into_iter()
        .filter_map(|category| {
            by_category.get(category).map(|docs| {
                let section_docs =
                    docs.iter()
                        .take(5)
                        .fold(String::new(), |mut acc, (title, filename, tags)| {
                            let tag_str = tags
                                .iter()
                                .take(2)
                                .map(|t| format!("`{t}`"))
                                .collect::<Vec<_>>()
                                .join(" ");
                            use std::fmt::Write;
                            let _ = writeln!(acc, "- [{title}](./docs/{filename}) {tag_str}");
                            acc
                        });
                format!("## {}\n\n{}\n", category.to_uppercase(), section_docs)
            })
        })
        .collect::<String>();

    let navigation_content = format!(
        "# Documentation Navigation\n\n> **{} documents**\n\n{}",
        analyses.len(),
        navigation_content
    );

    let navigation_file = output_dir.join("NAVIGATION.md");
    fs::write(navigation_file, navigation_content)?;

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

    // Single pass over chunks: collect nodes and edges
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

    // Detect and add related chunk edges using HNSW (O(n log n) instead of O(n²))
    // We used to do sparse BOW embeddings via HNSW here, but it's a terrible idea for sparse vectors!
    // Since we don't have dense embeddings yet, we'll skip adding related edges to the DAG for now.
    // When we integrate real embeddings (like text-embedding-3-small), we will re-enable this.

    Ok(dag)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::analyze::{Analysis, Heading};
    use crate::chunking_adapter::Chunk;
    use contextual_chunker::ChunkLevel;

    fn make_heading(level: u32, text: &str) -> Heading {
        Heading {
            level,
            text: text.to_string(),
            line: 0,
        }
    }

    fn make_analysis(
        source_path: &str,
        title: &str,
        category: &str,
        headings: Vec<Heading>,
        first_paragraph: &str,
        word_count: usize,
    ) -> Analysis {
        Analysis {
            source_path: source_path.to_string(),
            title: title.to_string(),
            frontmatter: None,
            headings,
            links: vec![],
            first_paragraph: first_paragraph.to_string(),
            word_count,
            has_code: false,
            has_tables: false,
            category: category.to_string(),
            content: Arc::from(format!(
                "{first_paragraph} Additional content for testing purposes."
            )),
        }
    }

    fn make_chunk(
        chunk_id: &str,
        doc_id: &str,
        doc_title: &str,
        content: &str,
        heading: Option<&str>,
        level: ChunkLevel,
    ) -> Chunk {
        Chunk {
            chunk_id: chunk_id.to_string(),
            doc_id: doc_id.to_string(),
            doc_title: doc_title.to_string(),
            chunk_index: 0,
            content: content.to_string(),
            token_count: content.split_whitespace().count(),
            heading: heading.map(String::from),
            heading_path: vec![],
            chunk_type: contextual_chunker::ChunkType::Prose,
            previous_chunk_id: None,
            next_chunk_id: None,
            related_chunk_ids: vec![],
            summary: content.to_string(),
            chunk_level: level,
            parent_chunk_id: None,
            child_chunk_ids: vec![],
            context_prefix: None,
        }
    }

    fn make_link_map(mappings: Vec<(&str, &str, &str, &str)>) -> HashMap<String, IdMapping> {
        mappings
            .into_iter()
            .map(|(source_path, id, filename, subcategory)| {
                (
                    source_path.to_string(),
                    IdMapping {
                        id: id.to_string(),
                        filename: filename.to_string(),
                        subcategory: subcategory.to_string(),
                        slug: filename.to_string(),
                    },
                )
            })
            .collect()
    }

    #[test]
    fn test_chunk_metadata_no_duplicate_ids() {
        let chunks = vec![
            make_chunk(
                "doc1#0-standard",
                "doc1",
                "Doc 1",
                "Content 1",
                Some("Section 1"),
                ChunkLevel::Standard,
            ),
            make_chunk(
                "doc1#1-standard",
                "doc1",
                "Doc 1",
                "Content 2",
                Some("Section 2"),
                ChunkLevel::Standard,
            ),
            make_chunk(
                "doc1#0-standard",
                "doc1",
                "Doc 1",
                "Content 1",
                Some("Section 1"),
                ChunkLevel::Standard,
            ),
        ];

        let result = build_chunk_metadata(&chunks, &KnowledgeDAG::new());

        match result {
            Err(e) => {
                let err_msg = e.to_string();
                assert!(err_msg.contains("Duplicate chunk_id"));
                assert!(err_msg.contains("doc1#0-standard"));
            }
            Ok(_) => panic!("Should fail when duplicate chunk_ids exist"),
        }
    }

    #[test]
    fn test_empty_chunks_no_crash() {
        let documents = vec![];
        let chunks = vec![];
        let document_tags = vec![];

        let dag = build_knowledge_dag(&documents, &chunks, &document_tags, None, None, None, None)
            .expect("Failed to build knowledge DAG with empty chunks");

        let stats = dag.statistics();
        assert_eq!(stats.node_count, 0);
        assert_eq!(stats.edge_count, 0);
    }

    #[test]
    fn test_build_document_index_basic() {
        let analyses = vec![make_analysis(
            "docs/tutorial/rust-guide.md",
            "Rust Guide",
            "tutorial",
            vec![
                make_heading(1, "Introduction"),
                make_heading(2, "Getting Started"),
            ],
            "Rust is a systems language.",
            100,
        )];
        let link_map = make_link_map(vec![(
            "docs/tutorial/rust-guide.md",
            "tutorial/rust-guide",
            "tutorial-rust-guide.md",
            "tutorial",
        )]);
        let chunks_result = ChunksResult {
            total_chunks: 0,
            document_count: 0,
            chunks_metadata: vec![],
            summary_chunks: 0,
            standard_chunks: 0,
            detailed_chunks: 0,
        };

        let result = build_document_index(&analyses, &link_map, &chunks_result);

        assert_eq!(result.documents.len(), 1);
        assert_eq!(result.documents[0].id, "tutorial/rust-guide");
        assert_eq!(result.documents[0].title, "Rust Guide");
        assert_eq!(result.documents[0].category, "tutorial");
        assert_eq!(result.documents[0].path, "docs/tutorial-rust-guide.md");
    }

    #[test]
    fn test_build_document_index_with_chunk_ids() {
        let analyses = vec![make_analysis(
            "docs/concept/design.md",
            "Design Patterns",
            "concept",
            vec![make_heading(1, "Patterns")],
            "Design patterns overview.",
            50,
        )];
        let link_map = make_link_map(vec![(
            "docs/concept/design.md",
            "concept/design",
            "concept-design.md",
            "concept",
        )]);
        let chunks_result = ChunksResult {
            total_chunks: 2,
            document_count: 1,
            chunks_metadata: vec![
                make_chunk(
                    "concept/design#0-standard",
                    "concept/design",
                    "Design Patterns",
                    "content 1",
                    None,
                    ChunkLevel::Standard,
                ),
                make_chunk(
                    "concept/design#1-standard",
                    "concept/design",
                    "Design Patterns",
                    "content 2",
                    None,
                    ChunkLevel::Standard,
                ),
            ],
            summary_chunks: 0,
            standard_chunks: 2,
            detailed_chunks: 0,
        };

        let result = build_document_index(&analyses, &link_map, &chunks_result);

        assert_eq!(result.documents[0].chunk_ids.len(), 2);
        assert_eq!(
            result.documents[0].chunk_ids[0],
            "concept/design#0-standard"
        );
        assert_eq!(
            result.documents[0].chunk_ids[1],
            "concept/design#1-standard"
        );
    }

    #[test]
    fn test_build_document_index_empty_analyses() {
        let link_map: HashMap<String, IdMapping> = HashMap::new();
        let chunks_result = ChunksResult {
            total_chunks: 0,
            document_count: 0,
            chunks_metadata: vec![],
            summary_chunks: 0,
            standard_chunks: 0,
            detailed_chunks: 0,
        };

        let result = build_document_index(&[], &link_map, &chunks_result);

        assert!(result.documents.is_empty());
        assert!(result.keywords.is_empty());
        assert!(result.document_tags.is_empty());
    }

    #[test]
    fn test_build_document_index_no_matching_link_map() {
        let analyses = vec![make_analysis(
            "docs/orphan.md",
            "Orphan Doc",
            "concept",
            vec![],
            "No mapping exists.",
            10,
        )];
        let link_map: HashMap<String, IdMapping> = HashMap::new();
        let chunks_result = ChunksResult {
            total_chunks: 0,
            document_count: 0,
            chunks_metadata: vec![],
            summary_chunks: 0,
            standard_chunks: 0,
            detailed_chunks: 0,
        };

        let result = build_document_index(&analyses, &link_map, &chunks_result);

        assert!(result.documents.is_empty());
    }

    #[test]
    fn test_build_document_index_keywords_extraction() {
        let analyses = vec![make_analysis(
            "docs/tutorial/rust-guide.md",
            "Rust Guide",
            "tutorial",
            vec![
                make_heading(1, "Introduction"),
                make_heading(2, "Advanced Programming"),
                make_heading(3, "Error Handling"),
            ],
            "Rust guide intro.",
            200,
        )];
        let link_map = make_link_map(vec![(
            "docs/tutorial/rust-guide.md",
            "tutorial/rust-guide",
            "tutorial-rust-guide.md",
            "tutorial",
        )]);
        let chunks_result = ChunksResult {
            total_chunks: 0,
            document_count: 0,
            chunks_metadata: vec![],
            summary_chunks: 0,
            standard_chunks: 0,
            detailed_chunks: 0,
        };

        let result = build_document_index(&analyses, &link_map, &chunks_result);

        assert!(!result.keywords.is_empty());
        let all_keywords: Vec<_> = result.keywords.keys().collect();
        assert!(all_keywords
            .iter()
            .any(|k| **k == "introduction" || **k == "advanced"));
    }

    #[test]
    fn test_build_document_index_document_tags() {
        let analyses = vec![make_analysis(
            "docs/ref/api.md",
            "API Reference",
            "ref",
            vec![
                make_heading(1, "HTTP Endpoints"),
                make_heading(2, "Functions"),
            ],
            "API docs.",
            300,
        )];
        let link_map = make_link_map(vec![("docs/ref/api.md", "ref/api", "ref-api.md", "ref")]);
        let chunks_result = ChunksResult {
            total_chunks: 0,
            document_count: 0,
            chunks_metadata: vec![],
            summary_chunks: 0,
            standard_chunks: 0,
            detailed_chunks: 0,
        };

        let result = build_document_index(&analyses, &link_map, &chunks_result);

        assert_eq!(result.document_tags.len(), 1);
        assert_eq!(result.document_tags[0].0, "ref/api");
        assert!(!result.document_tags[0].1.is_empty());
        assert_eq!(result.document_tags[0].2, "ref");
    }

    #[test]
    fn test_build_chunk_metadata_valid() {
        let chunks = vec![
            make_chunk(
                "doc1#0-standard",
                "doc1",
                "Doc 1",
                "Content 1",
                Some("Intro"),
                ChunkLevel::Standard,
            ),
            make_chunk(
                "doc1#1-standard",
                "doc1",
                "Doc 1",
                "Content 2",
                Some("Body"),
                ChunkLevel::Standard,
            ),
        ];

        let metadata = build_chunk_metadata(&chunks, &KnowledgeDAG::new()).unwrap();

        assert_eq!(metadata.len(), 2);
        assert_eq!(metadata[0].chunk_id, "doc1#0-standard");
        assert_eq!(metadata[0].doc_id, "doc1");
        assert_eq!(metadata[0].doc_title, "Doc 1");
        assert_eq!(metadata[0].heading, Some("Intro".to_string()));
        assert_eq!(metadata[0].token_count, 2);
        assert_eq!(metadata[0].summary, "Content 1");
        assert!(metadata[0].related_chunks.is_empty());
        assert_eq!(metadata[0].chunk_level, ChunkLevel::Standard);
        assert!(metadata[0].path.contains("doc1"));
        assert!(metadata[0].path.contains("standard"));
    }

    #[test]
    fn test_build_chunk_metadata_empty_heading_path_gets_intro() {
        let chunks = vec![make_chunk(
            "doc1#0-standard",
            "doc1",
            "Doc 1",
            "Content",
            None,
            ChunkLevel::Standard,
        )];

        let metadata = build_chunk_metadata(&chunks, &KnowledgeDAG::new()).unwrap();

        assert_eq!(metadata[0].heading_path, vec!["Intro".to_string()]);
        assert!(
            metadata[0].heading_anchor.is_none(),
            "Intro heading should not produce anchor"
        );
    }

    #[test]
    fn test_build_chunk_metadata_with_heading_path() {
        let mut chunk = make_chunk(
            "doc1#0-standard",
            "doc1",
            "Doc 1",
            "Content",
            Some("Section"),
            ChunkLevel::Standard,
        );
        chunk.heading_path = vec!["Chapter 1".to_string(), "Section A".to_string()];

        let metadata = build_chunk_metadata(&[chunk], &KnowledgeDAG::new()).unwrap();

        assert_eq!(
            metadata[0].heading_path,
            vec!["Chapter 1".to_string(), "Section A".to_string()]
        );
        assert!(metadata[0].heading_anchor.is_some());
    }

    #[test]
    fn test_build_chunk_metadata_siblings() {
        let chunks = vec![
            make_chunk(
                "doc1#0-standard",
                "doc1",
                "Doc 1",
                "C1",
                None,
                ChunkLevel::Standard,
            ),
            make_chunk(
                "doc1#1-standard",
                "doc1",
                "Doc 1",
                "C2",
                None,
                ChunkLevel::Standard,
            ),
            make_chunk(
                "doc1#2-standard",
                "doc1",
                "Doc 1",
                "C3",
                None,
                ChunkLevel::Standard,
            ),
        ];

        let metadata = build_chunk_metadata(&chunks, &KnowledgeDAG::new()).unwrap();

        for m in &metadata {
            assert_eq!(
                m.sibling_chunk_ids.len(),
                2,
                "Each chunk should have 2 siblings"
            );
        }
    }

    #[test]
    fn test_build_chunk_metadata_siblings_different_levels() {
        let chunks = vec![
            make_chunk(
                "doc1#0-standard",
                "doc1",
                "Doc 1",
                "C1",
                None,
                ChunkLevel::Standard,
            ),
            make_chunk(
                "doc1#0-summary",
                "doc1",
                "Doc 1",
                "C2",
                None,
                ChunkLevel::Summary,
            ),
        ];

        let metadata = build_chunk_metadata(&chunks, &KnowledgeDAG::new()).unwrap();

        assert!(
            metadata[0].sibling_chunk_ids.is_empty(),
            "Different levels should not be siblings"
        );
        assert!(metadata[1].sibling_chunk_ids.is_empty());
    }

    #[test]
    fn test_build_chunk_metadata_with_parent_child() {
        let mut chunk = make_chunk(
            "doc1#0-standard",
            "doc1",
            "Doc 1",
            "Content",
            None,
            ChunkLevel::Standard,
        );
        chunk.parent_chunk_id = Some("doc1#0-summary".to_string());
        chunk.child_chunk_ids = vec!["doc1#1-standard".to_string()];

        let metadata = build_chunk_metadata(&[chunk], &KnowledgeDAG::new()).unwrap();

        assert_eq!(
            metadata[0].parent_chunk_id,
            Some("doc1#0-summary".to_string())
        );
        assert_eq!(
            metadata[0].child_chunk_ids,
            vec!["doc1#1-standard".to_string()]
        );
    }

    #[test]
    fn test_build_chunk_metadata_with_related_chunks_from_dag() {
        let chunks = vec![make_chunk(
            "doc1#0-standard",
            "doc1",
            "Doc 1",
            "Content",
            None,
            ChunkLevel::Standard,
        )];

        let mut dag = KnowledgeDAG::new();
        dag.add_node(GraphNode {
            id: "doc1#0-standard".to_string(),
            node_type: NodeType::Chunk,
            title: "Doc 1".to_string(),
            category: None,
        });
        dag.add_node(GraphNode {
            id: "doc2#0-standard".to_string(),
            node_type: NodeType::Chunk,
            title: "Doc 2".to_string(),
            category: None,
        });
        dag.add_edge(GraphEdge {
            from: "doc1#0-standard".to_string(),
            to: "doc2#0-standard".to_string(),
            edge_type: EdgeType::Related,
            weight: 0.8,
        });

        let metadata = build_chunk_metadata(&chunks, &dag).unwrap();

        assert_eq!(metadata[0].related_chunks.len(), 1);
        assert_eq!(metadata[0].related_chunks[0].chunk_id, "doc2#0-standard");
        assert!((metadata[0].related_chunks[0].similarity - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn test_build_chunk_metadata_empty() {
        let metadata = build_chunk_metadata(&[], &KnowledgeDAG::new()).unwrap();
        assert!(metadata.is_empty());
    }

    #[test]
    fn test_build_knowledge_dag_with_documents_and_chunks() {
        let documents = vec![IndexDocument {
            id: "doc1".to_string(),
            title: "Doc 1".to_string(),
            path: "docs/doc1.md".to_string(),
            category: "tutorial".to_string(),
            tags: vec![],
            summary: "Summary".to_string(),
            word_count: 100,
            chunk_ids: vec![],
            headings: vec![],
            content: "Content".into(),
        }];

        let mut chunk0 = make_chunk(
            "doc1#0-standard",
            "doc1",
            "Doc 1",
            "C0",
            Some("Intro"),
            ChunkLevel::Standard,
        );
        chunk0.next_chunk_id = Some("doc1#1-standard".to_string());
        let mut chunk1 = make_chunk(
            "doc1#1-standard",
            "doc1",
            "Doc 1",
            "C1",
            Some("Body"),
            ChunkLevel::Standard,
        );
        chunk1.previous_chunk_id = Some("doc1#0-standard".to_string());

        let dag = build_knowledge_dag(&documents, &[chunk0, chunk1], &[], None, None, None, None)
            .unwrap();
        let stats = dag.statistics();

        assert_eq!(stats.node_count, 3);
        assert!(
            stats.edge_count >= 2,
            "Should have parent edges + sequential edges"
        );
    }

    #[test]
    fn test_build_knowledge_dag_chunk_heading_titles() {
        let documents = vec![IndexDocument {
            id: "doc1".to_string(),
            title: "Test Doc".to_string(),
            path: "docs/test.md".to_string(),
            category: "concept".to_string(),
            tags: vec![],
            summary: "Sum".to_string(),
            word_count: 10,
            chunk_ids: vec![],
            headings: vec![],
            content: "C".into(),
        }];
        let chunk_with_heading = make_chunk(
            "doc1#0",
            "doc1",
            "Test Doc",
            "Content",
            Some("Chapter One"),
            ChunkLevel::Standard,
        );
        let chunk_no_heading = make_chunk(
            "doc1#1",
            "doc1",
            "Test Doc",
            "Content",
            None,
            ChunkLevel::Standard,
        );

        let dag = build_knowledge_dag(
            &documents,
            &[chunk_with_heading, chunk_no_heading],
            &[],
            None,
            None,
            None,
            None,
        )
        .unwrap();
        let nodes = dag.nodes();

        assert!(nodes.iter().any(|n| n.title == "Test Doc - Chapter One"));
        assert!(nodes.iter().any(|n| n.title == "Test Doc - Intro"));
    }

    #[test]
    fn test_compute_graph_analytics_empty() {
        let dag = KnowledgeDAG::new();
        let documents: Vec<IndexDocument> = vec![];

        let analytics = compute_graph_analytics(&dag, &documents);

        assert!(analytics.topo_order.is_empty());
        assert!(analytics.reachability.is_empty());
        assert!(analytics.node_importance.is_empty());
    }

    #[test]
    fn test_compute_graph_analytics_with_nodes() {
        let mut dag = KnowledgeDAG::new();
        dag.add_node(GraphNode {
            id: "doc1".to_string(),
            node_type: NodeType::Document,
            title: "Doc 1".to_string(),
            category: Some("tutorial".to_string()),
        });
        dag.add_node(GraphNode {
            id: "doc2".to_string(),
            node_type: NodeType::Document,
            title: "Doc 2".to_string(),
            category: Some("ref".to_string()),
        });
        dag.add_edge(GraphEdge {
            from: "doc1".to_string(),
            to: "doc2".to_string(),
            edge_type: EdgeType::Sequential,
            weight: 1.0,
        });

        let documents = vec![
            IndexDocument {
                id: "doc1".to_string(),
                title: "Doc 1".to_string(),
                path: "d1".to_string(),
                category: "t".to_string(),
                tags: vec![],
                summary: "s".to_string(),
                word_count: 10,
                chunk_ids: vec![],
                headings: vec![],
                content: "c".into(),
            },
            IndexDocument {
                id: "doc2".to_string(),
                title: "Doc 2".to_string(),
                path: "d2".to_string(),
                category: "r".to_string(),
                tags: vec![],
                summary: "s".to_string(),
                word_count: 20,
                chunk_ids: vec![],
                headings: vec![],
                content: "c".into(),
            },
        ];

        let analytics = compute_graph_analytics(&dag, &documents);

        assert_eq!(analytics.topo_order.len(), 2);
        assert!(analytics.reachability.contains_key("doc1"));
        assert!(analytics.node_importance.contains_key("doc1"));
        assert!(analytics.node_importance.contains_key("doc2"));
    }

    #[test]
    fn test_assemble_index_json_structure() {
        let documents = vec![IndexDocument {
            id: "doc1".to_string(),
            title: "Test".to_string(),
            path: "docs/test.md".to_string(),
            category: "concept".to_string(),
            tags: vec![],
            summary: "Sum".to_string(),
            word_count: 100,
            chunk_ids: vec![],
            headings: vec![],
            content: "Content".into(),
        }];
        let chunks_metadata = vec![];
        let keywords = HashMap::new();
        let dag = KnowledgeDAG::new();
        let analytics = GraphAnalytics {
            topo_order: vec![],
            reachability: HashMap::new(),
            node_importance: HashMap::new(),
        };

        let ctx = IndexAssemblyContext {
            documents: &documents,
            chunks_metadata: &chunks_metadata,
            keywords: &keywords,
            dag: &dag,
            analytics: &analytics,
            total_chunks: 0,
            project_name: "test-project",
        };

        let json = assemble_index_json(&ctx);

        assert_eq!(json["version"], "5.0");
        assert_eq!(json["project"], "test-project");
        assert!(json["metadata"].is_object());
        assert!(json["stats"].is_object());
        assert!(json["navigation"].is_object());
        assert_eq!(json["stats"]["doc_count"], 1);
        assert_eq!(json["stats"]["chunk_count"], 0);
        assert_eq!(json["navigation"]["graph_enabled"], true);
    }

    #[test]
    fn test_assemble_index_json_with_chunks() {
        let documents = vec![IndexDocument {
            id: "d1".to_string(),
            title: "D".to_string(),
            path: "p".to_string(),
            category: "c".to_string(),
            tags: vec![],
            summary: "s".to_string(),
            word_count: 100,
            chunk_ids: vec![],
            headings: vec![],
            content: "c".into(),
        }];
        let chunks_metadata = vec![ChunkMetadata {
            chunk_id: "d1#0".to_string(),
            doc_id: "d1".to_string(),
            doc_title: "D".to_string(),
            heading: Some("Intro".to_string()),
            heading_path: vec![],
            heading_anchor: None,
            chunk_type: contextual_chunker::ChunkType::Prose,
            token_count: 200,
            summary: "Chunk sum".to_string(),
            previous_chunk_id: None,
            next_chunk_id: None,
            section_index: 0,
            path: "chunks/d1--0-standard.md".to_string(),
            related_chunks: vec![],
            chunk_level: ChunkLevel::Standard,
            parent_chunk_id: None,
            child_chunk_ids: vec![],
            sibling_chunk_ids: vec![],
        }];
        let keywords: HashMap<String, Vec<String>> = HashMap::new();
        let dag = KnowledgeDAG::new();
        let analytics = GraphAnalytics {
            topo_order: vec![],
            reachability: HashMap::new(),
            node_importance: HashMap::new(),
        };

        let ctx = IndexAssemblyContext {
            documents: &documents,
            chunks_metadata: &chunks_metadata,
            keywords: &keywords,
            dag: &dag,
            analytics: &analytics,
            total_chunks: 1,
            project_name: "p",
        };

        let json = assemble_index_json(&ctx);

        assert_eq!(json["stats"]["doc_count"], 1);
        assert_eq!(json["stats"]["chunk_count"], 1);
        assert_eq!(json["stats"]["avg_chunk_size_tokens"], 200);
        assert_eq!(json["chunks"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_write_index_file() {
        let dir = tempfile::TempDir::new().unwrap();
        let json = serde_json::json!({"test": "data"});

        write_index_file(dir.path(), &json).unwrap();

        let content = fs::read_to_string(dir.path().join("INDEX.json")).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed["test"], "data");
    }

    #[test]
    fn test_build_and_write_navigation() {
        let analyses = vec![
            make_analysis(
                "docs/tutorial/rust.md",
                "Rust Tutorial",
                "tutorial",
                vec![],
                "Learn Rust.",
                100,
            ),
            make_analysis(
                "docs/ref/api.md",
                "API Reference",
                "ref",
                vec![],
                "HTTP API docs.",
                200,
            ),
        ];
        let link_map = make_link_map(vec![
            (
                "docs/tutorial/rust.md",
                "tutorial/rust",
                "tutorial-rust.md",
                "tutorial",
            ),
            ("docs/ref/api.md", "ref/api", "ref-api.md", "ref"),
        ]);

        let dir = tempfile::TempDir::new().unwrap();
        build_and_write_navigation(&analyses, &link_map, dir.path()).unwrap();

        let content = fs::read_to_string(dir.path().join("NAVIGATION.md")).unwrap();
        assert!(content.contains("Documentation Navigation"));
        assert!(content.contains("2 documents"));
        assert!(content.contains("Rust Tutorial"));
        assert!(content.contains("API Reference"));
    }

    #[test]
    fn test_build_and_write_navigation_empty_analyses() {
        let analyses: Vec<Analysis> = vec![];
        let link_map: HashMap<String, IdMapping> = HashMap::new();

        let dir = tempfile::TempDir::new().unwrap();
        build_and_write_navigation(&analyses, &link_map, dir.path()).unwrap();

        let content = fs::read_to_string(dir.path().join("NAVIGATION.md")).unwrap();
        assert!(content.contains("Documentation Navigation"));
        assert!(content.contains("0 documents"));
    }

    #[test]
    fn test_build_and_write_navigation_multiple_categories() {
        let analyses = vec![
            make_analysis(
                "a",
                "Ops Guide",
                "ops",
                vec![make_heading(1, "Deployment")],
                "Deploy app.",
                50,
            ),
            make_analysis(
                "b",
                "Concepts",
                "concept",
                vec![make_heading(1, "Architecture")],
                "System design.",
                80,
            ),
            make_analysis(
                "c",
                "Tutorial",
                "tutorial",
                vec![make_heading(1, "Getting Started")],
                "Start here.",
                30,
            ),
            make_analysis(
                "d",
                "Reference",
                "ref",
                vec![make_heading(1, "API")],
                "API docs.",
                200,
            ),
            make_analysis("e", "Meta Readme", "meta", vec![], "Project overview.", 20),
        ];
        let link_map = make_link_map(vec![
            ("a", "ops/g", "ops-g.md", "ops"),
            ("b", "concept/c", "concept-c.md", "concept"),
            ("c", "tutorial/t", "tutorial-t.md", "tutorial"),
            ("d", "ref/r", "ref-r.md", "ref"),
            ("e", "meta/m", "meta-m.md", "meta"),
        ]);

        let dir = tempfile::TempDir::new().unwrap();
        build_and_write_navigation(&analyses, &link_map, dir.path()).unwrap();

        let content = fs::read_to_string(dir.path().join("NAVIGATION.md")).unwrap();
        assert!(content.contains("TUTORIAL"));
        assert!(content.contains("CONCEPT"));
        assert!(content.contains("REF"));
        assert!(content.contains("OPS"));
        assert!(content.contains("META"));
    }

    #[test]
    fn test_extract_tags_basic() {
        let analysis = make_analysis(
            "test.md",
            "Test",
            "tutorial",
            vec![make_heading(1, "Introduction")],
            "First paragraph.",
            50,
        );

        let tags = extract_tags(&analysis);

        assert!(!tags.is_empty());
        assert!(tags.contains(&"tutorial".to_string()));
    }

    #[test]
    fn test_extract_tags_with_long_heading_words() {
        let analysis = make_analysis(
            "test.md",
            "Test",
            "concept",
            vec![
                make_heading(1, "Advanced Programming"),
                make_heading(2, "Database Architecture"),
                make_heading(3, "Microservice Deployment"),
            ],
            "Text.",
            50,
        );

        let tags = extract_tags(&analysis);

        assert!(tags.contains(&"concept".to_string()));
        assert!(tags.len() <= 5, "Tags should be capped at 5");
    }

    #[test]
    fn test_extract_tags_filters_short_words_and_stopwords() {
        let analysis = make_analysis(
            "test.md",
            "Test",
            "ref",
            vec![make_heading(1, "The And For But A")],
            "Text.",
            10,
        );

        let tags = extract_tags(&analysis);

        for tag in &tags {
            assert!(
                tag.len() > 4 || tag == "ref",
                "Short words and stopwords should be filtered: {tag}"
            );
        }
    }

    #[test]
    fn test_extract_tags_sorted_and_deduped() {
        let analysis = make_analysis(
            "test.md",
            "Test",
            "tutorial",
            vec![
                make_heading(1, "Programming"),
                make_heading(2, "Programming"),
            ],
            "Text.",
            10,
        );

        let tags = extract_tags(&analysis);

        let mut sorted = tags.clone();
        sorted.sort();
        assert_eq!(tags, sorted, "Tags should be sorted");
        let unique: HashSet<_> = tags.iter().collect();
        assert_eq!(unique.len(), tags.len(), "Tags should be deduped");
    }

    #[test]
    fn test_slugify_heading_basic() {
        assert_eq!(slugify_heading("Hello World"), "hello-world");
    }

    #[test]
    fn test_slugify_heading_special_chars() {
        assert_eq!(slugify_heading("API Reference (v2)"), "api-reference-v2");
    }

    #[test]
    fn test_slugify_heading_multiple_spaces() {
        assert_eq!(slugify_heading("  Multiple   Spaces  "), "multiple-spaces");
    }

    #[test]
    fn test_slugify_heading_numbers() {
        assert_eq!(
            slugify_heading("Section 3.2 - Advanced"),
            "section-3-2-advanced"
        );
    }

    #[test]
    fn test_index_document_serialization() {
        let doc = IndexDocument {
            id: "id".to_string(),
            title: "Title".to_string(),
            path: "p".to_string(),
            category: "cat".to_string(),
            tags: vec!["tag1".to_string()],
            summary: "sum".to_string(),
            word_count: 42,
            chunk_ids: vec![],
            headings: vec![],
            content: "c".into(),
        };

        let json = serde_json::to_string(&doc).unwrap();
        let deserialized: IndexDocument = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "id");
        assert_eq!(deserialized.title, "Title");
        assert_eq!(deserialized.tags, vec!["tag1"]);
    }

    #[test]
    fn test_chunk_metadata_serialization() {
        let meta = ChunkMetadata {
            chunk_id: "c1".to_string(),
            doc_id: "d1".to_string(),
            doc_title: "D".to_string(),
            heading: Some("H".to_string()),
            heading_path: vec!["A".to_string()],
            heading_anchor: Some("a".to_string()),
            chunk_type: contextual_chunker::ChunkType::Prose,
            token_count: 100,
            summary: "S".to_string(),
            previous_chunk_id: Some("prev".to_string()),
            next_chunk_id: Some("next".to_string()),
            section_index: 0,
            path: "chunks/c.md".to_string(),
            related_chunks: vec![RelatedChunk {
                chunk_id: "r1".to_string(),
                similarity: 0.5,
            }],
            chunk_level: ChunkLevel::Standard,
            parent_chunk_id: None,
            child_chunk_ids: vec![],
            sibling_chunk_ids: vec![],
        };

        let json = serde_json::to_string(&meta).unwrap();
        let deserialized: ChunkMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.chunk_id, "c1");
        assert_eq!(deserialized.related_chunks.len(), 1);
    }

    #[test]
    fn test_related_chunk_struct() {
        let rc = RelatedChunk {
            chunk_id: "chunk-1".to_string(),
            similarity: 0.85,
        };
        let cloned = rc.clone();
        assert_eq!(cloned.chunk_id, rc.chunk_id);
        assert!((cloned.similarity - rc.similarity).abs() < f32::EPSILON);

        let debug_str = format!("{rc:?}");
        assert!(debug_str.contains("chunk-1"));
    }

    #[test]
    fn test_build_and_write_index_full_pipeline() {
        let analyses = vec![make_analysis(
            "docs/tutorial/guide.md",
            "Guide",
            "tutorial",
            vec![make_heading(1, "Guide Title")],
            "Guide first paragraph with keywords.",
            100,
        )];
        let link_map = make_link_map(vec![(
            "docs/tutorial/guide.md",
            "tutorial/guide",
            "tutorial-guide.md",
            "tutorial",
        )]);
        let chunks_result = ChunksResult {
            total_chunks: 1,
            document_count: 1,
            chunks_metadata: vec![make_chunk(
                "tutorial/guide#0-standard",
                "tutorial/guide",
                "Guide",
                "Guide chunk content.",
                None,
                ChunkLevel::Standard,
            )],
            summary_chunks: 0,
            standard_chunks: 1,
            detailed_chunks: 0,
        };

        let dir = tempfile::TempDir::new().unwrap();
        build_and_write_index(
            &analyses,
            &link_map,
            &chunks_result,
            dir.path(),
            "test-proj",
            None,
            None,
            None,
            None,
        )
        .unwrap();

        assert!(dir.path().join("INDEX.json").exists());
        assert!(dir.path().join(".tantivy_index").exists());

        let index_content = fs::read_to_string(dir.path().join("INDEX.json")).unwrap();
        let index_json: serde_json::Value = serde_json::from_str(&index_content).unwrap();
        assert_eq!(index_json["version"], "5.0");
        assert_eq!(index_json["project"], "test-proj");
        assert_eq!(index_json["documents"].as_array().unwrap().len(), 1);
    }
}
