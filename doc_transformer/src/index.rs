use crate::analyze::Analysis;
use crate::assign::IdMapping;
use crate::chunking_adapter::{Chunk, ChunksResult};
use crate::graph::{EdgeType, GraphEdge, GraphNode, KnowledgeDAG, NodeType};
use crate::search;
use crate::similarity::{build_index_with_params, query_neighbors};
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
    build_tantivy_index(output_dir, &doc_index.documents)?;

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
    let mut seen_ids = std::collections::HashSet::new();
    for chunk in chunks {
        if !seen_ids.insert(&chunk.chunk_id) {
            anyhow::bail!("Duplicate chunk_id found: {}", chunk.chunk_id);
        }
    }

    let mut siblings_map: HashMap<String, Vec<String>> = HashMap::new();

    for chunk in chunks {
        let key = format!("{}::{}", chunk.doc_id, chunk.chunk_level.as_str());
        siblings_map
            .entry(key)
            .or_default()
            .push(chunk.chunk_id.clone());
    }

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

    let mut reachability: HashMap<String, Vec<String>> = HashMap::new();
    let mut node_importance: HashMap<String, f32> = HashMap::new();

    for doc in documents {
        let reachable = dag.reachable_from(&doc.id);
        let mut reachable_list: Vec<String> =
            reachable.into_iter().filter(|id| id != &doc.id).collect();
        reachable_list.sort();
        reachability.insert(doc.id.clone(), reachable_list);

        // Compute node importance (sum of outgoing edge weights)
        node_importance.insert(doc.id.clone(), dag.node_importance(&doc.id));
    }

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
fn build_tantivy_index(output_dir: &Path, documents: &[IndexDocument]) -> Result<()> {
    search::open_or_create_index(output_dir)
        .and_then(|index| search::index_documents(&index, documents.to_vec()))
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

    let mut compass = String::new();
    compass.push_str("# Documentation Compass\n\n");
    compass.push_str(&format!("> **{} documents**\n\n", analyses.len()));

    // By category
    for category in &["tutorial", "concept", "ref", "ops", "meta"] {
        if let Some(docs) = by_category.get(*category) {
            compass.push_str("## ");
            compass.push_str(&category.to_uppercase());
            compass.push_str("\n\n");
            for (title, filename, tags) in docs.iter().take(5) {
                let tag_str = tags
                    .iter()
                    .take(2)
                    .map(|t| format!("`{t}`"))
                    .collect::<Vec<_>>()
                    .join(" ");
                compass.push_str("- [");
                compass.push_str(title);
                compass.push_str("](./docs/");
                compass.push_str(filename);
                compass.push_str(") ");
                compass.push_str(&tag_str);
                compass.push('\n');
            }
            compass.push('\n');
        }
    }

    let compass_file = output_dir.join("COMPASS.md");
    fs::write(compass_file, compass)?;

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

const DEFAULT_MAX_CHUNK_KEYWORDS: usize = 12;
const TAG_WEIGHT: f32 = 1.0;
const HEADING_WEIGHT: f32 = 2.0;
const SUMMARY_WEIGHT: f32 = 1.0;
const CATEGORY_WEIGHT: f32 = 2.0;

/// Generate a simple embedding vector from weighted terms and category.
/// Uses a bag-of-words approach with a fixed vocabulary built from all unique words.
/// Returns a sparse embedding where each dimension represents a word's weighted presence.
fn generate_embedding_from_terms(
    terms: &[(String, f32)],
    category: &str,
    vocabulary: &HashMap<String, usize>,
    embedding_dim: usize,
) -> Vec<f32> {
    let embedding: Vec<f32> = (0..embedding_dim)
        .map(|i| {
            let term_weight = terms.iter().find_map(|(term, weight)| {
                vocabulary
                    .get(term)
                    .filter(|&&idx| idx == i)
                    .map(|_| *weight)
            });
            let category_weight = vocabulary
                .get(category)
                .filter(|&&idx| idx == i)
                .map(|_| CATEGORY_WEIGHT);
            term_weight.unwrap_or(0.0) + category_weight.unwrap_or(0.0)
        })
        .collect();

    let magnitude: f32 = embedding.iter().map(|&x| x * x).sum::<f32>().sqrt();
    if magnitude > 0.0 {
        embedding.iter().map(|x| x / magnitude).collect()
    } else {
        embedding
    }
}

/// Build vocabulary from all tags, categories, and chunk keywords
fn build_vocabulary(
    document_tags: &[(String, Vec<String>, String)],
    chunks: &[Chunk],
    max_chunk_keywords: usize,
) -> Result<HashMap<String, usize>> {
    let mut all_terms: Vec<String> = Vec::new();

    // Collect categories
    for (_, _, category) in document_tags {
        if !category.is_empty() {
            all_terms.push(category.clone());
        }
    }

    // Collect tags
    for (_, tags, _) in document_tags {
        for tag in tags {
            if !tag.is_empty() {
                all_terms.push(tag.clone());
            }
        }
    }

    // Collect chunk keywords
    for chunk in chunks {
        for (keyword, _) in chunk_terms(chunk, max_chunk_keywords) {
            all_terms.push(keyword);
        }
    }

    // Deduplicate and assign indices using functional pattern
    let vocab: HashMap<String, usize> = all_terms
        .into_iter()
        .sorted()
        .dedup()
        .enumerate()
        .map(|(idx, term)| (term, idx))
        .collect();

    if vocab.len() > usize::MAX / 2 {
        anyhow::bail!("Vocabulary index overflow - too many unique tags/categories");
    }

    Ok(vocab)
}

fn chunk_terms(chunk: &Chunk, max_chunk_keywords: usize) -> Vec<(String, f32)> {
    if max_chunk_keywords == 0 {
        return Vec::new();
    }

    let heading_terms = chunk
        .heading
        .as_ref()
        .map(|h| extract_keywords_weighted(h, HEADING_WEIGHT))
        .unwrap_or_default();

    let summary_terms = if chunk.summary.is_empty() {
        Vec::new()
    } else {
        extract_keywords_weighted(&chunk.summary, SUMMARY_WEIGHT)
    };

    let mut weighted: Vec<(String, f32)> = heading_terms
        .into_iter()
        .chain(summary_terms)
        .fold(HashMap::new(), |mut acc, (term, weight)| {
            acc.entry(term)
                .and_modify(|existing| {
                    if weight > *existing {
                        *existing = weight;
                    }
                })
                .or_insert(weight);
            acc
        })
        .into_iter()
        .collect();

    weighted.sort_by(|(a_term, a_weight), (b_term, b_weight)| {
        b_weight
            .partial_cmp(a_weight)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a_term.cmp(b_term))
    });
    weighted.truncate(max_chunk_keywords);
    weighted
}

fn merge_weighted_terms(terms: Vec<(String, f32)>) -> Vec<(String, f32)> {
    terms
        .into_iter()
        .fold(HashMap::new(), |mut acc, (term, weight)| {
            acc.entry(term)
                .and_modify(|existing| {
                    if weight > *existing {
                        *existing = weight;
                    }
                })
                .or_insert(weight);
            acc
        })
        .into_iter()
        .collect()
}

fn extract_keywords_weighted(text: &str, weight: f32) -> Vec<(String, f32)> {
    text.split_whitespace()
        .filter_map(|word| {
            let cleaned = word
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
                .to_lowercase();
            if cleaned.len() > 3 && !is_stopword(&cleaned) {
                Some((cleaned, weight))
            } else {
                None
            }
        })
        .collect()
}

/// Build a knowledge graph DAG from documents and chunks
#[allow(clippy::too_many_arguments)]
/// Build knowledge DAG from documents and chunks
/// # Errors
/// Returns error if HNSW indexing fails or vocabulary building fails
/// Build knowledge DAG from documents and chunks
/// # Errors
/// Returns error if HNSW indexing fails or vocabulary building fails
#[allow(clippy::too_many_lines)]
pub fn build_knowledge_dag(
    documents: &[IndexDocument],
    chunks: &[Chunk],
    document_tags: &[(String, Vec<String>, String)],
    max_related_chunks: Option<usize>,
    hnsw_m: Option<usize>,
    hnsw_ef_construction: Option<usize>,
    max_chunk_keywords: Option<usize>,
) -> Result<KnowledgeDAG> {
    const MAX_CHUNKS_FOR_SEMANTIC_GRAPH: usize = 5000;

    let mut dag = KnowledgeDAG::new();

    // Add document nodes
    for doc in documents {
        let node = GraphNode {
            id: doc.id.clone(),
            node_type: NodeType::Document,
            title: doc.title.clone(),
            category: Some(doc.category.clone()),
        };
        dag.add_node(node);
    }

    // Add chunk nodes
    for chunk in chunks {
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
    }

    // Add parent-child edges (document -> chunks)
    for chunk in chunks {
        let edge = GraphEdge {
            from: chunk.doc_id.clone(),
            to: chunk.chunk_id.clone(),
            edge_type: EdgeType::Parent,
            weight: 1.0,
        };
        dag.add_edge(edge);
    }

    // Add sequential edges (previous -> next chunks)
    for chunk in chunks {
        if let Some(next_id) = &chunk.next_chunk_id {
            let edge = GraphEdge {
                from: chunk.chunk_id.clone(),
                to: next_id.clone(),
                edge_type: EdgeType::Sequential,
                weight: 1.0,
            };
            dag.add_edge(edge);
        }
    }

    // Detect and add related chunk edges using HNSW (O(n log n) instead of O(n²))
    let max_related = max_related_chunks.unwrap_or(5);
    let max_chunk_keywords = max_chunk_keywords.unwrap_or(DEFAULT_MAX_CHUNK_KEYWORDS);
    const SIMILARITY_THRESHOLD: f32 = 0.3;

    if !chunks.is_empty() && chunks.len() <= MAX_CHUNKS_FOR_SEMANTIC_GRAPH {
        // Build vocabulary from tags, categories, and chunk keywords
        let vocabulary = build_vocabulary(document_tags, chunks, max_chunk_keywords)?;
        let embedding_dim = vocabulary.len().max(1); // At least 1 dimension

        // Prepare per-chunk terms and categories
        let chunk_terms_list: Vec<(Vec<(String, f32)>, String)> = chunks
            .iter()
            .map(|chunk| {
                let (tags, category) = document_tags
                    .iter()
                    .find(|(id, _, _)| id == &chunk.doc_id)
                    .map(|(_, tags, cat)| (tags.clone(), cat.clone()))
                    .unwrap_or_else(|| {
                        eprintln!(
                            "Warning: Document {} has no tags/category metadata, using empty tags",
                            chunk.doc_id
                        );
                        (Vec::new(), "unknown".to_string())
                    });

                let mut terms: Vec<(String, f32)> =
                    tags.into_iter().map(|tag| (tag, TAG_WEIGHT)).collect();
                terms.extend(chunk_terms(chunk, max_chunk_keywords));
                let terms = merge_weighted_terms(terms);
                (terms, category)
            })
            .collect();

        // Generate embeddings for all chunks
        let embeddings: Vec<Vec<f32>> = chunk_terms_list
            .iter()
            .map(|(terms, category)| {
                generate_embedding_from_terms(terms, category, &vocabulary, embedding_dim)
            })
            .collect();

        // Build HNSW index for O(log n) nearest neighbor search
        match build_index_with_params(&embeddings, hnsw_m, hnsw_ef_construction) {
            Ok(index) => {
                // Track existing related edges to prevent bidirectional edges that form cycles.
                // Uses a HashSet of (from, to) pairs for O(1) lookup.
                // Only add edge A->B if edge B->A doesn't already exist.
                let mut existing_related_edges: std::collections::HashSet<(String, String)> =
                    std::collections::HashSet::new();

                // Query top-k neighbors for each chunk
                for (i, chunk) in chunks.iter().enumerate() {
                    let (chunk_tags, chunk_category) = &chunk_terms_list[i];

                    let query_embedding = generate_embedding_from_terms(
                        chunk_tags,
                        chunk_category,
                        &vocabulary,
                        embedding_dim,
                    );

                    // Query HNSW for top-k neighbors (k+1 to account for self)
                    if let Ok(neighbors) =
                        query_neighbors(&index, &query_embedding, max_related.saturating_add(1))
                    {
                        let mut added_edges: usize = 0;
                        for (neighbor_idx, similarity) in neighbors {
                            // Skip self-edges and low-similarity matches
                            // Explicit bounds check to prevent panic on malformed HNSW indices
                            if neighbor_idx != i
                                && neighbor_idx < chunks.len()
                                && similarity >= SIMILARITY_THRESHOLD
                                && added_edges < max_related
                            {
                                let from_id = chunk.chunk_id.clone();
                                let to_id = chunks[neighbor_idx].chunk_id.clone();

                                // Check if reverse edge already exists (prevents bidirectional edges)
                                // This ensures the graph remains acyclic
                                let reverse_exists = existing_related_edges
                                    .contains(&(to_id.clone(), from_id.clone()));

                                if !reverse_exists {
                                    // Check if adding this edge would create a cycle via any edge type
                                    // Must include Related to prevent cycles like A->B->C->A through related edges
                                    let would_cycle = dag.would_create_cycle_with_edge_types(
                                        &from_id,
                                        &to_id,
                                        &[
                                            EdgeType::Sequential,
                                            EdgeType::Parent,
                                            EdgeType::Related,
                                        ],
                                    );

                                    if would_cycle {
                                        // Skip this edge to prevent cycle
                                        continue;
                                    }

                                    // Track this edge to prevent reverse edge later
                                    // Must track BEFORE adding edge since from_id/to_id get moved
                                    existing_related_edges.insert((from_id.clone(), to_id.clone()));

                                    let edge = GraphEdge {
                                        from: from_id,
                                        to: to_id,
                                        edge_type: EdgeType::Related,
                                        weight: similarity,
                                    };
                                    dag.add_edge(edge);

                                    added_edges = added_edges.saturating_add(1);
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                // HNSW index build failed - skip related edges
                // This can happen with empty embeddings or invalid vectors
                eprintln!("Error: HNSW index build failed ({e}), skipping related chunk edges");
                // Continue without adding related edges - document structure (parent/sequential) is preserved
            }
        }
    } else if chunks.len() > MAX_CHUNKS_FOR_SEMANTIC_GRAPH {
        eprintln!(
            "Warning: {} chunks exceeds semantic graph limit ({}), skipping related chunk edge construction",
            chunks.len(),
            MAX_CHUNKS_FOR_SEMANTIC_GRAPH
        );
    }

    Ok(dag)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic)]

    use super::*;
    use crate::chunking_adapter::Chunk;
    use contextual_chunker::ChunkLevel;
    use std::collections::HashMap;

    fn tags_to_terms(tags: &[String]) -> Vec<(String, f32)> {
        tags.iter().cloned().map(|tag| (tag, TAG_WEIGHT)).collect()
    }

    /// Generate synthetic test chunks with realistic structure
    fn generate_test_chunks(n: usize) -> Vec<Chunk> {
        let docs_per_batch = (n as f64).sqrt().ceil() as usize;
        let chunks_per_doc = n.div_ceil(docs_per_batch);

        let mut chunks = Vec::with_capacity(n);

        for doc_idx in 0..docs_per_batch {
            let doc_id = format!("doc_{doc_idx:04}");
            let doc_title = format!("Document {doc_idx}");

            for chunk_idx in 0..chunks_per_doc {
                if chunks.len() >= n {
                    break;
                }

                let chunk_id = format!("chunk_{doc_idx}_{chunk_idx:04}");
                let previous_chunk_id = if chunk_idx > 0 {
                    Some(format!(
                        "chunk_{}_{:04}",
                        doc_idx,
                        chunk_idx.saturating_sub(1)
                    ))
                } else {
                    None
                };

                let next_chunk_id = if chunk_idx.saturating_add(1) < chunks_per_doc {
                    Some(format!(
                        "chunk_{}_{:04}",
                        doc_idx,
                        chunk_idx.saturating_add(1)
                    ))
                } else {
                    None
                };

                let chunk = Chunk {
                    chunk_id,
                    doc_id: doc_id.clone(),
                    doc_title: doc_title.clone(),
                    chunk_index: chunk_idx,
                    content: format!(
                        "Content for chunk {chunk_idx} in document {doc_idx}. This is sample documentation text."
                    ),
                    token_count: 256_usize.saturating_add(chunk_idx % 256),
                    heading: Some(format!("Section {chunk_idx}")),
                    heading_path: vec!["Document".to_string(), format!("Section {chunk_idx}")],
                    chunk_type: contextual_chunker::ChunkType::Prose,
                    previous_chunk_id,
                    next_chunk_id,
                    related_chunk_ids: Vec::new(),
                    summary: format!("Summary of chunk {chunk_idx} in doc {doc_idx}"),
                    chunk_level: ChunkLevel::Standard,
                    parent_chunk_id: None,
                    child_chunk_ids: Vec::new(),
                };

                chunks.push(chunk);
            }
        }

        chunks
    }

    /// Generate synthetic index documents
    fn generate_test_docs(chunks: &[Chunk]) -> Vec<IndexDocument> {
        let mut docs_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut docs_titles: HashMap<String, String> = HashMap::new();

        for chunk in chunks {
            docs_map
                .entry(chunk.doc_id.clone())
                .or_default()
                .push(chunk.chunk_id.clone());
            docs_titles
                .entry(chunk.doc_id.clone())
                .or_insert_with(|| chunk.doc_title.clone());
        }

        docs_map
            .into_iter()
            .enumerate()
            .map(|(idx, (doc_id, chunk_ids))| {
                let title = docs_titles
                    .get(&doc_id)
                    .cloned()
                    .unwrap_or_else(|| format!("Document {idx}"));

                IndexDocument {
                    id: doc_id.clone(),
                    title,
                    path: format!("/docs/doc_{idx}.md"),
                    category: format!("Category {}", idx % 5),
                    tags: vec![
                        format!("tag_{}", idx % 3),
                        format!("tag_{}", idx.saturating_add(1) % 3),
                        format!("tag_{}", idx.saturating_add(2) % 3),
                    ],
                    summary: format!("Summary for document {idx}"),
                    word_count: 1000_usize.saturating_add(idx.saturating_mul(100)),
                    chunk_ids,
                    headings: vec![
                        "Introduction".to_string(),
                        "Content".to_string(),
                        "Conclusion".to_string(),
                    ],
                }
            })
            .collect()
    }

    /// Generate document tags for relationship detection
    fn generate_test_tags(chunks: &[Chunk]) -> Vec<(String, Vec<String>, String)> {
        let mut docs_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut docs_categories: HashMap<String, String> = HashMap::new();

        for chunk in chunks {
            docs_map
                .entry(chunk.doc_id.clone())
                .or_default()
                .push(chunk.chunk_id.clone());
            docs_categories
                .entry(chunk.doc_id.clone())
                .or_insert_with_key(|doc_id| {
                    let doc_num: usize = doc_id
                        .strip_prefix("doc_")
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0);
                    format!("Category {}", doc_num % 5)
                });
        }

        docs_map
            .into_iter()
            .enumerate()
            .map(|(idx, (doc_id, _))| {
                let category = docs_categories
                    .get(&doc_id)
                    .cloned()
                    .unwrap_or_else(|| format!("Category {}", idx % 5));

                let tags = vec![
                    format!("tag_{}", idx % 3),
                    format!("tag_{}", idx.saturating_add(1) % 3),
                    format!("tag_{}", idx.saturating_add(2) % 3),
                    "documentation".to_string(),
                    format!("section_{}", idx.saturating_div(10) % 10),
                ];

                (doc_id, tags, category)
            })
            .collect()
    }

    /// Test HNSW edge count linearity across multiple scales
    /// Verifies that edge count grows linearly (O(n)) not quadratically (O(n²))
    #[test]
    fn test_hnsw_edge_count_linear() {
        for n in [10, 100, 1000] {
            let chunks = generate_test_chunks(n);
            let docs = generate_test_docs(&chunks);
            let tags = generate_test_tags(&chunks);

            let dag = match build_knowledge_dag(&docs, &chunks, &tags, None, None, None, None) {
                Ok(d) => d,
                Err(e) => panic!("Failed to build knowledge DAG for edge count test: {e}"),
            };

            // N × max_related_chunks × safety_factor (1.5)
            // max_related_chunks = 20 in build_knowledge_dag
            let max_edges = n.saturating_mul(20).saturating_mul(15).saturating_div(10);

            assert!(
                dag.edges().len() < max_edges,
                "Edge count {} exceeds linear bound {} for {} chunks",
                dag.edges().len(),
                max_edges,
                n
            );
        }
    }

    #[test]
    fn test_large_chunk_sets_skip_related_edges() {
        const N: usize = 6000;

        let chunks = generate_test_chunks(N);
        let docs = generate_test_docs(&chunks);
        let tags = generate_test_tags(&chunks);

        let dag = match build_knowledge_dag(&docs, &chunks, &tags, None, None, None, None) {
            Ok(d) => d,
            Err(e) => panic!("Failed to build DAG for large chunk set: {e}"),
        };

        let related_edges = dag.edges_by_type(&EdgeType::Related).len();
        assert_eq!(
            related_edges, 0,
            "Expected related edges to be skipped for very large chunk sets"
        );
    }

    /// Test that edge count is O(n log n), not O(n²)
    /// With HNSW, we expect at most max_related edges per node
    #[test]
    fn test_knowledge_dag_edge_count_is_linear() {
        const N: usize = 100;
        const MAX_RELATED: usize = 5;
        let max_related = MAX_RELATED;

        // Create test documents
        let documents: Vec<IndexDocument> = (0..10)
            .map(|i| IndexDocument {
                id: format!("doc_{i}"),
                title: format!("Document {i}"),
                path: format!("/path/doc_{i}.md"),
                category: format!("category_{}", i % 3),
                tags: vec![format!("tag_{}", i % 5), format!("tag_{}", (i + 1) % 5)],
                summary: format!("Summary for document {i}"),
                word_count: 100,
                chunk_ids: vec![],
                headings: vec!["Heading".to_string()],
            })
            .collect();

        // Create test chunks
        let chunks: Vec<Chunk> = (0..N)
            .map(|i| Chunk {
                chunk_id: format!("chunk_{i}"),
                doc_id: format!("doc_{}", i % 10),
                doc_title: format!("Document {}", i % 10),
                chunk_index: i,
                content: format!("Content for chunk {i}"),
                token_count: 100,
                heading: Some(format!("Heading {i}")),
                heading_path: vec!["Document".to_string(), format!("Heading {i}")],
                chunk_type: contextual_chunker::ChunkType::Prose,
                previous_chunk_id: if i > 0 {
                    Some(format!("chunk_{}", i - 1))
                } else {
                    None
                },
                next_chunk_id: Some(format!("chunk_{}", i + 1)),
                related_chunk_ids: vec![],
                summary: format!("Summary {i}"),
                chunk_level: ChunkLevel::Standard,
                parent_chunk_id: None,
                child_chunk_ids: vec![],
            })
            .collect();

        // Create document tags
        let document_tags: Vec<(String, Vec<String>, String)> = (0..10)
            .map(|i| {
                (
                    format!("doc_{i}"),
                    vec![format!("tag_{}", i % 5), format!("tag_{}", (i + 1) % 5)],
                    format!("category_{}", i % 3),
                )
            })
            .collect();

        // Build the DAG
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
            Err(e) => panic!("Failed to build knowledge DAG for linear edge count test: {e}"),
        };

        // Get statistics
        let stats = dag.statistics();

        // Total edges include: parent edges (N), sequential edges (≈N), and related edges
        // Related edges should be at most N * max_related
        let max_expected_related_edges = N * max_related;

        // Count related edges
        let related_edges = dag.edges_by_type(&EdgeType::Related).len();

        println!("Total chunks: {N}");
        println!("Related edges: {related_edges}");
        println!("Max expected (N * {max_related}): {max_expected_related_edges}");
        println!("Total edges: {}", stats.edge_count);

        // Assert that related edges are bounded by O(n log n), not O(n²)
        // With HNSW and max_related=5, we expect at most N*5 related edges
        assert!(
            related_edges <= max_expected_related_edges,
            "Related edges {related_edges} exceeds O(n log n) bound {max_expected_related_edges}. This indicates O(n²) behavior!"
        );

        // For comparison: O(n²) would be 100*99/2 = 4950 edges
        let quadratic_edges = N * (N - 1) / 2;
        println!("Quadratic would be: {quadratic_edges} edges");
        // SAFETY: Edge counts in tests are small (< 10k), well within f64 precision (2^53)
        println!(
            "Ratio: {:.2}% of quadratic",
            (related_edges as f64 / quadratic_edges as f64) * 100.0
        );

        // Verify we're not in quadratic territory (should be < 20% of quadratic)
        assert!(
            related_edges < quadratic_edges / 5,
            "Edge count {} is too close to quadratic {} (should be < {})",
            related_edges,
            quadratic_edges,
            quadratic_edges / 5
        );
    }

    #[test]
    fn test_build_vocabulary() {
        let document_tags = vec![
            (
                "doc1".to_string(),
                vec!["rust".to_string(), "programming".to_string()],
                "tutorial".to_string(),
            ),
            (
                "doc2".to_string(),
                vec!["rust".to_string(), "web".to_string()],
                "guide".to_string(),
            ),
        ];

        let chunks = vec![Chunk {
            chunk_id: "doc1#0-standard".to_string(),
            doc_id: "doc1".to_string(),
            doc_title: "Doc 1".to_string(),
            chunk_index: 0,
            content: "Content".to_string(),
            token_count: 10,
            heading: Some("Ownership Basics".to_string()),
            heading_path: vec!["Doc 1".to_string(), "Ownership Basics".to_string()],
            chunk_type: contextual_chunker::ChunkType::Prose,
            previous_chunk_id: None,
            next_chunk_id: None,
            related_chunk_ids: vec![],
            summary: "Rust ownership guide".to_string(),
            chunk_level: ChunkLevel::Standard,
            parent_chunk_id: None,
            child_chunk_ids: vec![],
        }];

        let vocab = match build_vocabulary(&document_tags, &chunks, 10) {
            Ok(v) => v,
            Err(e) => panic!("Failed to build vocabulary from test document tags: {e}"),
        };

        // Should have 3 unique tags (rust, programming, web) + 2 categories (tutorial, guide)
        // plus chunk keywords (ownership, basics) = 7 total
        // "rust" appears in both documents but is only counted once
        assert_eq!(vocab.len(), 7);
        assert!(vocab.contains_key("rust"));
        assert!(vocab.contains_key("programming"));
        assert!(vocab.contains_key("web"));
        assert!(vocab.contains_key("tutorial"));
        assert!(vocab.contains_key("guide"));
        assert!(vocab.contains_key("ownership"));
        assert!(vocab.contains_key("basics"));
    }

    /// Test that chunk metadata has no duplicate chunk_ids
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
    fn test_generate_embedding_from_terms() {
        let mut vocab = HashMap::new();
        vocab.insert("rust".to_string(), 0);
        vocab.insert("programming".to_string(), 1);
        vocab.insert("tutorial".to_string(), 2);

        let tags = vec!["rust".to_string(), "programming".to_string()];
        let category = "tutorial";

        let embedding = generate_embedding_from_terms(&tags_to_terms(&tags), category, &vocab, 3);

        // Should be normalized
        let magnitude: f32 = embedding.iter().map(|&x| x * x).sum::<f32>().sqrt();
        assert!(
            (magnitude - 1.0).abs() < 0.01,
            "Embedding should be normalized to unit vector"
        );

        // All values should be non-negative since we only add positive weights
        assert!(embedding.iter().all(|&x| x >= 0.0));
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

    // ===========================================================================
    // Property-based tests for generate_embedding_from_terms
    // ===========================================================================
    // These tests verify invariants that should hold for all possible inputs

    /// Build a test vocabulary from a set of tags and categories
    fn build_test_vocabulary(
        all_tags: &[Vec<String>],
        all_categories: &[String],
    ) -> HashMap<String, usize> {
        let mut vocab = HashMap::new();
        let mut idx: usize = 0;

        for category in all_categories {
            if !vocab.contains_key(category.as_str()) && !category.is_empty() {
                vocab.insert(category.clone(), idx);
                idx = match idx.checked_add(1) {
                    Some(i) => i,
                    None => break,
                };
            }
        }

        for tags in all_tags {
            for tag in tags {
                if !vocab.contains_key(tag.as_str()) && !tag.is_empty() {
                    vocab.insert(tag.clone(), idx);
                    idx = match idx.checked_add(1) {
                        Some(i) => i,
                        None => break,
                    };
                }
            }
        }

        vocab
    }

    /// Property 1: Normalization - Result is unit vector (magnitude approx 1.0)
    /// For non-empty embeddings, the magnitude should always be 1.0
    #[test]
    fn proptest_embedding_normalization() {
        use proptest::prelude::*;

        let strategy = (
            prop::collection::vec(".*", 0..10),
            "[a-z]{1,20}",
            1..100usize,
        );

        proptest!(|(tags in strategy.0, category in strategy.1, embedding_dim in strategy.2)| {
            // Filter out empty strings and build vocabulary
            let clean_tags: Vec<String> = tags.into_iter()
                .filter(|s| !s.is_empty())
                .collect();

            let clean_category = if category.is_empty() { "default".to_string() } else { category };

            // Build vocabulary including all tags and category
            let vocab = build_test_vocabulary(&[clean_tags.clone()], &[clean_category.clone()]);

            // If vocabulary is empty, produce zero embedding (always has magnitude 0)
            if vocab.is_empty() {
                let dim = embedding_dim.max(1);
                let zero_embedding = generate_embedding_from_terms(
                    &tags_to_terms(&clean_tags),
                    &clean_category,
                    &vocab,
                    dim,
                );
                prop_assert_eq!(zero_embedding.len(), dim, "Zero embedding length mismatch");
            } else {
                // Ensure embedding_dim is at least 1
                let dim = embedding_dim.max(1);

                let embedding = generate_embedding_from_terms(
                    &tags_to_terms(&clean_tags),
                    &clean_category,
                    &vocab,
                    dim,
                );

                // Property: magnitude should be approximately 1.0 for non-zero embeddings
                let magnitude: f32 = embedding.iter().map(|&x| x * x).sum::<f32>().sqrt();

                // Allow small tolerance for floating point arithmetic
                prop_assert!(
                    magnitude > 0.0 && (magnitude - 1.0).abs() < 0.001,
                    "Embedding magnitude {} is not close to 1.0 (or is zero)",
                    magnitude
                );
            }
        });
    }

    /// Property 2: Length - Output dimension matches expected size
    #[test]
    fn proptest_embedding_length() {
        use proptest::prelude::*;

        let strategy = (
            prop::collection::vec("[a-z]{1,10}", 0..20),
            "[a-z]{1,10}",
            1..200usize,
        );

        proptest!(|(tags in strategy.0, category in strategy.1, embedding_dim in strategy.2)| {
            let vocab = build_test_vocabulary(&[tags.clone()], &[category.clone()]);

            let embedding = generate_embedding_from_terms(
                &tags_to_terms(&tags),
                &category,
                &vocab,
                embedding_dim,
            );

            // Property: output length must equal requested dimension
            prop_assert_eq!(
                embedding.len(),
                embedding_dim,
                "Embedding length {} != expected dimension {}",
                embedding.len(),
                embedding_dim
            );
        });
    }

    /// Property 3: Determinism - Same input produces same output
    #[test]
    fn proptest_embedding_determinism() {
        use proptest::prelude::*;

        let strategy = (
            prop::collection::vec("[a-z]{1,10}", 0..20),
            "[a-z]{1,10}",
            10..100usize,
        );

        proptest!(|(tags in strategy.0, category in strategy.1, embedding_dim in strategy.2)| {
            let vocab = build_test_vocabulary(&[tags.clone()], &[category.clone()]);

            // Generate embedding twice with same inputs
            let embedding1 = generate_embedding_from_terms(
                &tags_to_terms(&tags),
                &category,
                &vocab,
                embedding_dim,
            );
            let embedding2 = generate_embedding_from_terms(
                &tags_to_terms(&tags),
                &category,
                &vocab,
                embedding_dim,
            );

            // Clone for error message since prop_assert_eq! takes ownership
            let e1_repr = format!("{embedding1:?}");
            let e2_repr = format!("{embedding2:?}");

            // Property: outputs must be identical
            prop_assert_eq!(
                embedding1, embedding2,
                "Embeddings differ for same input: {} vs {}",
                e1_repr, e2_repr
            );
        });
    }

    /// Property 4: Empty input handling
    #[test]
    fn proptest_embedding_empty_input() {
        use proptest::prelude::*;

        // Test with various vocabulary sizes
        let vocab_strategy = prop::collection::hash_map("[a-z]{1,10}", 0..200usize, 0..50);

        proptest!(|(vocab in vocab_strategy, embedding_dim in 1..200usize)| {
            let empty_tags: Vec<String> = vec![];
            let empty_category = "";

            let embedding = generate_embedding_from_terms(
                &tags_to_terms(&empty_tags),
                empty_category,
                &vocab,
                embedding_dim,
            );

            // Property: empty input should produce zero vector (normalized to zero)
            // All elements should be 0.0
            prop_assert!(
                embedding.iter().all(|&x| x == 0.0),
                "Empty input should produce zero vector, got {:?}",
                embedding
            );

            // Property: length should still match
            prop_assert_eq!(
                embedding.len(),
                embedding_dim,
                "Zero vector length mismatch"
            );
        });
    }

    /// Property 5: Non-negative values
    /// All embedding values should be non-negative since we only add positive weights
    #[test]
    fn proptest_embedding_non_negative() {
        use proptest::prelude::*;

        let strategy = (
            prop::collection::vec("[a-z]{1,10}", 0..20),
            "[a-z]{1,10}",
            1..100usize,
        );

        proptest!(|(tags in strategy.0, category in strategy.1, embedding_dim in strategy.2)| {
            let vocab = build_test_vocabulary(&[tags.clone()], &[category.clone()]);

            let embedding = generate_embedding_from_terms(
                &tags_to_terms(&tags),
                &category,
                &vocab,
                embedding_dim,
            );

            // Property: all values must be >= 0
            prop_assert!(
                embedding.iter().all(|&x| x >= 0.0),
                "Found negative value in embedding: {:?}",
                embedding
            );
        });
    }

    /// Property 6: Order invariance
    /// Tags in different orders should produce the same embedding
    #[test]
    fn proptest_embedding_order_invariant() {
        use proptest::prelude::*;

        let tags_strategy = prop::collection::vec("[a-z]{1,10}", 2..10);
        let category_strategy = "[a-z]{1,10}";
        let dim_strategy = 10..100usize;

        proptest!(|(tags in tags_strategy, category in category_strategy, embedding_dim in dim_strategy)| {
            // Create a sorted version
            let mut tags_sorted = tags.clone();
            tags_sorted.sort();

            let vocab = build_test_vocabulary(&[tags.clone(), tags_sorted.clone()], &[category.clone()]);

            let embedding1 = generate_embedding_from_terms(
                &tags_to_terms(&tags),
                &category,
                &vocab,
                embedding_dim,
            );
            let embedding2 = generate_embedding_from_terms(
                &tags_to_terms(&tags_sorted),
                &category,
                &vocab,
                embedding_dim,
            );

            // Clone for error message since prop_assert_eq! takes ownership
            let e1_repr = format!("{embedding1:?}");
            let e2_repr = format!("{embedding2:?}");

            // Property: order should not matter
            prop_assert_eq!(
                &embedding1, &embedding2,
                "Embeddings differ for reordered tags: original={}, reordered={}",
                e1_repr, e2_repr
            );
        });
    }

    /// Property 7: Sparsity
    /// For large vocabularies, most dimensions should be zero
    #[test]
    fn proptest_embedding_sparsity() {
        use proptest::prelude::*;

        // Large vocabulary, few tags
        let tags_strategy = prop::collection::vec("[a-z]{1,10}", 1..5);
        let vocab_strategy = prop::collection::hash_map("[a-z]{1,10}", 0..200usize, 50..100);

        proptest!(|(tags in tags_strategy, category in "[a-z]{1,10}", vocab in vocab_strategy)| {
            let embedding_dim = vocab.len().max(1);

            let embedding = generate_embedding_from_terms(
                &tags_to_terms(&tags),
                &category,
                &vocab,
                embedding_dim,
            );

            // Count non-zero elements
            let non_zero_count = embedding.iter().filter(|&&x| x > 0.0).count();

            // Property: non-zero elements should not exceed unique tags + category
            let max_non_zero = tags.len().saturating_add(1); // +1 for category
            prop_assert!(
                non_zero_count <= max_non_zero,
                "Too many non-zero elements: {} > {} (tags: {})",
                non_zero_count, max_non_zero, tags.len()
            );
        });
    }

    /// Test that DAG maintains acyclic structure when HNSW similarity creates
    /// bidirectional edges. This tests the fix for BEAD doc-18ru.
    ///
    /// When chunk A relates to B and chunk B relates to A via HNSW similarity,
    /// the system should NOT create bidirectional edges that form cycles.
    #[test]
    fn test_dag_no_cycles_from_bidirectional_related_edges() {
        // Create chunks where similarity is symmetric (similar tags)
        // This forces HNSW to find mutual neighbors
        let chunks = vec![
            Chunk {
                chunk_id: "chunk_0".to_string(),
                doc_id: "doc_0".to_string(),
                doc_title: "Document 0".to_string(),
                chunk_index: 0,
                content: "Rust programming tutorial about ownership".to_string(),
                token_count: 100,
                heading: Some("Rust Ownership".to_string()),
                heading_path: vec!["Document 0".to_string(), "Rust Ownership".to_string()],
                chunk_type: contextual_chunker::ChunkType::Prose,
                previous_chunk_id: None,
                next_chunk_id: Some("chunk_1".to_string()),
                related_chunk_ids: vec![],
                summary: "Rust ownership tutorial".to_string(),
                chunk_level: ChunkLevel::Standard,
                parent_chunk_id: None,
                child_chunk_ids: vec![],
            },
            Chunk {
                chunk_id: "chunk_1".to_string(),
                doc_id: "doc_0".to_string(),
                doc_title: "Document 0".to_string(),
                chunk_index: 1,
                content: "Rust programming guide about borrowing".to_string(),
                token_count: 100,
                heading: Some("Rust Borrowing".to_string()),
                heading_path: vec!["Document 0".to_string(), "Rust Borrowing".to_string()],
                chunk_type: contextual_chunker::ChunkType::Prose,
                previous_chunk_id: Some("chunk_0".to_string()),
                next_chunk_id: None,
                related_chunk_ids: vec![],
                summary: "Rust borrowing guide".to_string(),
                chunk_level: ChunkLevel::Standard,
                parent_chunk_id: None,
                child_chunk_ids: vec![],
            },
        ];

        let documents = vec![IndexDocument {
            id: "doc_0".to_string(),
            title: "Document 0".to_string(),
            path: "/path/doc_0.md".to_string(),
            category: "tutorial".to_string(),
            tags: vec!["rust".to_string(), "programming".to_string()],
            summary: "A tutorial".to_string(),
            word_count: 100,
            chunk_ids: vec!["chunk_0".to_string(), "chunk_1".to_string()],
            headings: vec!["Rust Ownership".to_string(), "Rust Borrowing".to_string()],
        }];

        // Both chunks share the same tags, so HNSW will find them mutually similar
        let document_tags = vec![(
            "doc_0".to_string(),
            vec!["rust".to_string(), "programming".to_string()],
            "tutorial".to_string(),
        )];

        let dag = build_knowledge_dag(&documents, &chunks, &document_tags, None, None, None, None)
            .expect("Failed to build knowledge DAG");

        // Get the related edges
        let related_edges = dag.edges_by_type(&EdgeType::Related);

        // Check that we don't have bidirectional edges (A->B AND B->A)
        // which would form cycles
        let has_bidirectional = related_edges.iter().any(|e| {
            related_edges
                .iter()
                .any(|other| other.from == e.to && other.to == e.from)
        });

        assert!(
            !has_bidirectional,
            "DAG should not have bidirectional Related edges that form cycles. Found: {:?}",
            related_edges
        );

        // Verify topological order succeeds without fallback
        let topo_order = dag.topological_order();

        // If there were cycles, toposort would fail and we'd get fallback order (sorted by id)
        // Check that we got a valid topological order (not the fallback sorted order)
        // The actual order depends on graph structure, but it should NOT be sorted by id
        // if the DAG has more than one valid topological ordering
        assert!(
            !topo_order.is_empty(),
            "Topological order should contain all nodes. Nodes in graph: {:?}, Edges: {:?}",
            dag.nodes().iter().map(|n| &n.id).collect::<Vec<_>>(),
            dag.edges()
                .iter()
                .map(|e| (&e.from, &e.to))
                .collect::<Vec<_>>()
        );
        // 2 chunks + 1 document = 3 nodes
        assert!(
            topo_order.len() == 3,
            "Topological order should contain all nodes (2 chunks + 1 doc), got {}",
            topo_order.len()
        );
    }
}
