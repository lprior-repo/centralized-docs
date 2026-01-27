use crate::analyze::Analysis;
use crate::assign::IdMapping;
use crate::chunk::ChunksResult;
use crate::graph::{EdgeType, GraphEdge, GraphNode, KnowledgeDAG, NodeType, RelationshipDetector};
use crate::search;
use crate::similarity::{build_index, query_neighbors};
use anyhow::Result;
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkMetadata {
    pub chunk_id: String,
    pub doc_id: String,
    pub doc_title: String,
    pub heading: Option<String>,
    pub chunk_type: String,
    pub token_count: usize,
    pub summary: String,
    pub previous_chunk_id: Option<String>,
    pub next_chunk_id: Option<String>,
    pub path: String,
    /// Related chunks with similarity scores (populated from knowledge DAG)
    pub related_chunks: Vec<RelatedChunk>,
    /// Hierarchical chunk level (summary/standard/detailed)
    pub chunk_level: String,
    /// Parent chunk ID (for hierarchical navigation)
    pub parent_chunk_id: Option<String>,
    /// Child chunk IDs (for hierarchical navigation)
    pub child_chunk_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedChunk {
    pub chunk_id: String,
    pub similarity: f32,
}

pub fn build_and_write_index(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping>,
    chunks_result: &ChunksResult,
    output_dir: &Path,
    project_name: &str,
) -> Result<()> {
    let mut documents = Vec::new();
    let mut chunks_metadata = Vec::new();
    let mut keywords: HashMap<String, Vec<String>> = HashMap::new();
    let mut document_chunk_tags: Vec<(String, Vec<String>, String)> = Vec::new();

    // Build document index
    for analysis in analyses {
        if let Some(mapping) = link_map.get(&analysis.source_path) {
            let tags = extract_tags(analysis);
            document_chunk_tags.push((mapping.id.clone(), tags.clone(), analysis.category.clone()));

            // Build keywords from headings
            for heading in &analysis.headings {
                for word in heading.text.split_whitespace() {
                    let word_lower = word.to_lowercase();
                    if word_lower.len() > 3 && !is_stopword(&word_lower) {
                        keywords
                            .entry(word_lower)
                            .or_default()
                            .push(mapping.id.clone());
                    }
                }
            }

            // Get chunk IDs for this document
            let chunk_ids: Vec<String> = chunks_result
                .chunks_metadata
                .iter()
                .filter(|c| c.doc_id == mapping.id)
                .map(|c| c.chunk_id.clone())
                .collect();

            documents.push(IndexDocument {
                id: mapping.id.clone(),
                title: analysis.title.clone(),
                path: format!("docs/{}", mapping.filename),
                category: analysis.category.clone(),
                tags,
                summary: analysis.first_paragraph.clone(),
                word_count: analysis.word_count,
                chunk_ids,
            });
        }
    }

    // Build knowledge graph (DAG) first so we can use it for related chunks
    let dag = build_knowledge_dag(
        &documents,
        &chunks_result.chunks_metadata,
        &document_chunk_tags,
    );
    let dag_stats = dag.statistics();

    // Build chunk metadata for semantic navigation (with related chunks from DAG)
    for chunk in &chunks_result.chunks_metadata {
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

        chunks_metadata.push(ChunkMetadata {
            chunk_id: chunk.chunk_id.clone(),
            doc_id: chunk.doc_id.clone(),
            doc_title: chunk.doc_title.clone(),
            heading: chunk.heading.clone(),
            chunk_type: chunk.chunk_type.clone(),
            token_count: chunk.token_count,
            summary: chunk.summary.clone(),
            previous_chunk_id: chunk.previous_chunk_id.clone(),
            next_chunk_id: chunk.next_chunk_id.clone(),
            path: format!(
                "chunks/{}-{}.md",
                chunk.chunk_id.replace(['/', '#'], "-"),
                chunk.chunk_level.as_str()
            ),
            related_chunks,
            chunk_level: chunk.chunk_level.as_str().to_string(),
            parent_chunk_id: chunk.parent_chunk_id.clone(),
            child_chunk_ids: chunk.child_chunk_ids.clone(),
        });
    }

    // Compute topological order for traversal
    let topo_order = dag.topological_order();

    // Compute reachability from each document node (transitive closure)
    let mut reachability: HashMap<String, Vec<String>> = HashMap::new();
    let mut node_importance: HashMap<String, f32> = HashMap::new();
    for doc in &documents {
        let reachable = dag.reachable_from(&doc.id);
        let mut reachable_list: Vec<String> = reachable
            .into_iter()
            .filter(|id| id != &doc.id) // Exclude self
            .collect();
        reachable_list.sort();
        reachability.insert(doc.id.clone(), reachable_list);

        // Compute node importance (sum of outgoing edge weights)
        node_importance.insert(doc.id.clone(), dag.node_importance(&doc.id));
    }

    let timestamp = chrono::Utc::now().to_rfc3339();
    let index = json!({
        "version": "5.0",
        "project": project_name,
        "updated": timestamp,
        "generated": timestamp,
        "stats": {
            "doc_count": documents.len(),
            "chunk_count": chunks_result.total_chunks,
            "avg_chunk_size_tokens": chunks_result.chunks_metadata.iter()
                .map(|c| c.token_count)
                .sum::<usize>()
                .checked_div(chunks_result.total_chunks)
                .unwrap_or(0),
            "graph": {
                "node_count": dag_stats.node_count,
                "edge_count": dag_stats.edge_count,
                "sequential_edges": dag_stats.sequential_edges,
                "related_edges": dag_stats.related_edges,
                "reference_edges": dag_stats.reference_edges
            }
        },
        "documents": documents,
        "chunks": chunks_metadata,
        "keywords": keywords,
        "graph": {
            "nodes": dag.nodes(),
            "edges": dag.edges(),
            "topological_order": topo_order,
            "reachability": reachability,
            "node_importance": node_importance,
            "statistics": dag_stats
        },
        "navigation": {
            "type": "contextual_retrieval_with_dag",
            "strategy": "50-100 token context prefix + H2 boundaries + knowledge DAG with semantic similarity",
            "avg_tokens_per_chunk": 170,
            "graph_enabled": true,
            "similarity_metric": "jaccard_on_tags_and_category",
            "min_similarity_threshold": 0.3
        }
    });

    let index_file = output_dir.join("INDEX.json");
    fs::write(index_file, serde_json::to_string_pretty(&index)?)?;

    // Build Tantivy index for faster searching
    // This is optional - if it fails, we can still search via INDEX.json
    if let Err(e) = search::open_or_create_index(output_dir)
        .and_then(|index| search::index_documents(&index, documents))
    {
        eprintln!("Warning: Failed to build Tantivy index: {e}");
        eprintln!("Search will fall back to INDEX.json, but will be slower");
    }

    Ok(())
}

pub fn build_and_write_compass(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping>,
    output_dir: &Path,
) -> Result<()> {
    let mut by_category: HashMap<String, Vec<(String, String, Vec<String>)>> = HashMap::new();

    for analysis in analyses {
        if let Some(mapping) = link_map.get(&analysis.source_path) {
            let tags = extract_tags(analysis);
            by_category
                .entry(analysis.category.clone())
                .or_default()
                .push((analysis.title.clone(), mapping.filename.clone(), tags));
        }
    }

    let mut compass = format!(
        "---\nid: meta/navigation/compass\ntitle: Documentation Compass\ngenerated: {}\n---\n\n",
        chrono::Utc::now().to_rfc3339()
    );

    compass.push_str(&format!(
        "# Documentation Compass\n\n> **{} documents**\n\n",
        analyses.len()
    ));

    // By category
    for category in &["tutorial", "concept", "ref", "ops", "meta"] {
        if let Some(docs) = by_category.get(*category) {
            compass.push_str(&format!("## {}\n\n", category.to_uppercase()));
            for (title, filename, tags) in docs.iter().take(5) {
                let tag_str = tags
                    .iter()
                    .take(2)
                    .map(|t| format!("`{t}`"))
                    .collect::<Vec<_>>()
                    .join(" ");
                compass.push_str(&format!("- [{title}](./docs/{filename}) {tag_str}\n"));
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
                .map(|word| word.to_lowercase()),
        )
        .sorted()
        .dedup()
        .take(5)
        .collect()
}

/// Stopwords to filter out from tags
const STOPWORDS: [&str; 10] = [
    "this", "that", "these", "those", "about", "guide", "the", "and", "or", "for",
];

fn is_stopword(word: &str) -> bool {
    STOPWORDS.contains(&word)
}

/// Generate a simple embedding vector from tags and category.
/// Uses a bag-of-words approach with a fixed vocabulary built from all unique words.
/// Returns a sparse embedding where each dimension represents a word's presence.
fn generate_embedding_from_tags(
    tags: &[String],
    category: &str,
    vocabulary: &HashMap<String, usize>,
    embedding_dim: usize,
) -> Vec<f32> {
    let mut embedding = vec![0.0; embedding_dim];

    // Add tag contributions
    for tag in tags {
        if let Some(&idx) = vocabulary.get(tag) {
            if idx < embedding_dim {
                embedding[idx] = 1.0;
            }
        }
    }

    // Add category contribution (weighted higher)
    if let Some(&idx) = vocabulary.get(category) {
        if idx < embedding_dim {
            embedding[idx] = 2.0;
        }
    }

    // Normalize to unit vector for cosine similarity
    let magnitude: f32 = embedding.iter().map(|&x| x * x).sum::<f32>().sqrt();
    if magnitude > 0.0 {
        embedding.iter_mut().for_each(|x| *x /= magnitude);
    }

    embedding
}

/// Build vocabulary from all tags and categories
fn build_vocabulary(document_tags: &[(String, Vec<String>, String)]) -> HashMap<String, usize> {
    let mut vocab = HashMap::new();
    let mut idx: usize = 0;

    for (_, tags, category) in document_tags {
        // Add category to vocabulary
        if !vocab.contains_key(category) && !category.is_empty() {
            vocab.insert(category.clone(), idx);
            idx = idx.saturating_add(1);
        }

        // Add tags to vocabulary
        for tag in tags {
            if !vocab.contains_key(tag) && !tag.is_empty() {
                vocab.insert(tag.clone(), idx);
                idx = idx.saturating_add(1);
            }
        }
    }

    vocab
}

/// Build a knowledge graph DAG from documents and chunks
pub fn build_knowledge_dag(
    documents: &[IndexDocument],
    chunks: &[crate::chunk::Chunk],
    document_tags: &[(String, Vec<String>, String)],
) -> KnowledgeDAG {
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
    const MAX_RELATED_CHUNKS: usize = 5;
    const SIMILARITY_THRESHOLD: f32 = 0.3;

    if !chunks.is_empty() {
        // Build vocabulary from all tags and categories
        let vocabulary = build_vocabulary(document_tags);
        let embedding_dim = vocabulary.len().max(1); // At least 1 dimension

        // Generate embeddings for all chunks
        let embeddings: Vec<Vec<f32>> = chunks
            .iter()
            .map(|chunk| {
                let tags = document_tags
                    .iter()
                    .find(|(id, _, _)| id == &chunk.doc_id)
                    .map(|(_, tags, _)| tags.clone())
                    .unwrap_or_default();

                let category = document_tags
                    .iter()
                    .find(|(id, _, _)| id == &chunk.doc_id)
                    .map(|(_, _, cat)| cat.clone())
                    .unwrap_or_default();

                generate_embedding_from_tags(&tags, &category, &vocabulary, embedding_dim)
            })
            .collect();

        // Build HNSW index for O(log n) nearest neighbor search
        match build_index(&embeddings) {
            Ok(index) => {
                // Query top-k neighbors for each chunk
                for (i, chunk) in chunks.iter().enumerate() {
                    let chunk_tags = document_tags
                        .iter()
                        .find(|(id, _, _)| id == &chunk.doc_id)
                        .map(|(_, tags, _)| tags.clone())
                        .unwrap_or_default();

                    let chunk_category = document_tags
                        .iter()
                        .find(|(id, _, _)| id == &chunk.doc_id)
                        .map(|(_, _, cat)| cat.clone())
                        .unwrap_or_default();

                    let query_embedding =
                        generate_embedding_from_tags(&chunk_tags, &chunk_category, &vocabulary, embedding_dim);

                    // Query HNSW for top-k neighbors (k+1 to account for self)
                    if let Ok(neighbors) = query_neighbors(&index, &query_embedding, MAX_RELATED_CHUNKS + 1) {
                        let mut added_edges: usize = 0;
                        for (neighbor_idx, similarity) in neighbors {
                            // Skip self-edges and low-similarity matches
                            if neighbor_idx != i && similarity >= SIMILARITY_THRESHOLD && added_edges < MAX_RELATED_CHUNKS {
                                let edge = GraphEdge {
                                    from: chunk.chunk_id.clone(),
                                    to: chunks[neighbor_idx].chunk_id.clone(),
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
            Err(_) => {
                // Fallback to Jaccard similarity if HNSW fails (e.g., empty embeddings)
                let detector = RelationshipDetector::new(SIMILARITY_THRESHOLD);
                for chunk in chunks {
                    let chunk_tags = document_tags
                        .iter()
                        .find(|(id, _, _)| id == &chunk.doc_id)
                        .map(|(_, tags, _)| tags.clone())
                        .unwrap_or_default();

                    let chunk_category = document_tags
                        .iter()
                        .find(|(id, _, _)| id == &chunk.doc_id)
                        .map(|(_, _, cat)| cat.clone())
                        .unwrap_or_default();

                    let all_chunks_metadata: Vec<(String, Vec<String>, String)> = chunks
                        .iter()
                        .filter(|c| c.chunk_id != chunk.chunk_id)
                        .map(|c| {
                            let tags = document_tags
                                .iter()
                                .find(|(id, _, _)| id == &c.doc_id)
                                .map(|(_, t, _)| t.clone())
                                .unwrap_or_default();

                            let category = document_tags
                                .iter()
                                .find(|(id, _, _)| id == &c.doc_id)
                                .map(|(_, _, cat)| cat.clone())
                                .unwrap_or_default();

                            (c.chunk_id.clone(), tags, category)
                        })
                        .collect();

                    let related = detector.detect_relationships(
                        &chunk.chunk_id,
                        &chunk_tags,
                        &chunk_category,
                        &all_chunks_metadata,
                    );

                    for (related_id, weight) in related {
                        let edge = GraphEdge {
                            from: chunk.chunk_id.clone(),
                            to: related_id,
                            edge_type: EdgeType::Related,
                            weight,
                        };
                        dag.add_edge(edge);
                    }
                }
            }
        }
    }

    dag
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk::{Chunk, ChunkLevel};

    /// Test that edge count is O(n log n), not O(n²)
    /// With HNSW, we expect at most MAX_RELATED_CHUNKS edges per node
    #[test]
    fn test_knowledge_dag_edge_count_is_linear() {
        const N: usize = 100;
        const MAX_RELATED_CHUNKS: usize = 5;

        // Create test documents
        let documents: Vec<IndexDocument> = (0..10)
            .map(|i| IndexDocument {
                id: format!("doc_{i}"),
                title: format!("Document {i}"),
                path: format!("/path/doc_{i}.md"),
                category: format!("category_{}", i % 3), // 3 categories
                tags: vec![format!("tag_{}", i % 5), format!("tag_{}", (i + 1) % 5)], // 5 tags
                summary: format!("Summary for document {i}"),
                word_count: 100,
                chunk_ids: vec![],
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
                chunk_type: "standard".to_string(),
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
        let dag = build_knowledge_dag(&documents, &chunks, &document_tags);

        // Get statistics
        let stats = dag.statistics();

        // Total edges include: parent edges (N), sequential edges (≈N), and related edges
        // Related edges should be at most N * MAX_RELATED_CHUNKS
        let max_expected_related_edges = N * MAX_RELATED_CHUNKS;

        // Count related edges
        let related_edges = dag.edges_by_type(&EdgeType::Related).len();

        println!("Total chunks: {N}");
        println!("Related edges: {related_edges}");
        println!(
            "Max expected (N * {MAX_RELATED_CHUNKS}): {max_expected_related_edges}"
        );
        println!("Total edges: {}", stats.edge_count);

        // Assert that related edges are bounded by O(n log n), not O(n²)
        // With HNSW and MAX_RELATED_CHUNKS=5, we expect at most N*5 related edges
        assert!(
            related_edges <= max_expected_related_edges,
            "Related edges {related_edges} exceeds O(n log n) bound {max_expected_related_edges}. This indicates O(n²) behavior!"
        );

        // For comparison: O(n²) would be 100*99/2 = 4950 edges
        let quadratic_edges = N * (N - 1) / 2;
        println!("Quadratic would be: {quadratic_edges} edges");
        // SAFETY: Edge counts in tests are small (< 10k), well within f64 precision (2^53)
        println!("Ratio: {:.2}% of quadratic", (related_edges as f64 / quadratic_edges as f64) * 100.0);

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

        let vocab = build_vocabulary(&document_tags);

        // Should have 3 unique tags (rust, programming, web) + 2 categories (tutorial, guide) = 5 total
        // "rust" appears in both documents but is only counted once
        assert_eq!(vocab.len(), 5);
        assert!(vocab.contains_key("rust"));
        assert!(vocab.contains_key("programming"));
        assert!(vocab.contains_key("web"));
        assert!(vocab.contains_key("tutorial"));
        assert!(vocab.contains_key("guide"));
    }

    #[test]
    fn test_generate_embedding_from_tags() {
        let mut vocab = HashMap::new();
        vocab.insert("rust".to_string(), 0);
        vocab.insert("programming".to_string(), 1);
        vocab.insert("tutorial".to_string(), 2);

        let tags = vec!["rust".to_string(), "programming".to_string()];
        let category = "tutorial";

        let embedding = generate_embedding_from_tags(&tags, category, &vocab, 3);

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

        let dag = build_knowledge_dag(&documents, &chunks, &document_tags);

        let stats = dag.statistics();
        assert_eq!(stats.node_count, 0);
        assert_eq!(stats.edge_count, 0);
    }
}
