use crate::analyze::Analysis;
use crate::assign::IdMapping;
use crate::chunk::ChunksResult;
use crate::graph::{EdgeType, GraphEdge, GraphNode, KnowledgeDAG, NodeType, RelationshipDetector};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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
}

pub fn build_and_write_index(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping>,
    chunks_result: &ChunksResult,
    output_dir: &Path,
) -> Result<()> {
    let mut documents = Vec::new();
    let mut chunks_metadata = Vec::new();
    let mut keywords: HashMap<String, Vec<String>> = HashMap::new();
    let mut document_chunk_tags: Vec<(String, Vec<String>, String)> = Vec::new();
    let mut skipped_count = 0;

    // Build document index
    for analysis in analyses {
        match link_map.get(&analysis.source_path) {
            Some(mapping) => {
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
            None => {
                skipped_count += 1;
                eprintln!("WARNING: INDEX: No ID mapping for {}", analysis.source_path);
            }
        }
    }

    if skipped_count > 0 {
        eprintln!("WARNING: INDEX: {} documents skipped (no ID mapping)", skipped_count);
    }

    // Build chunk metadata for semantic navigation
    for chunk in &chunks_result.chunks_metadata {
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
            path: format!("chunks/{}.md", chunk.chunk_id.replace(['/', '#'], "-")),
        });
    }

    // Build knowledge graph (DAG)
    let dag = build_knowledge_dag(&documents, &chunks_result.chunks_metadata, &document_chunk_tags);
    let dag_stats = dag.statistics();

    let index = json!({
        "version": "4.3",
        "generated": chrono::Utc::now().to_rfc3339(),
        "stats": {
            "doc_count": documents.len(),
            "chunk_count": chunks_result.total_chunks,
            "avg_chunk_size_tokens": if chunks_result.total_chunks > 0 {
                chunks_result.chunks_metadata.iter().map(|c| c.token_count).sum::<usize>() / chunks_result.total_chunks
            } else {
                0
            },
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

    Ok(())
}

pub fn build_and_write_compass(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping>,
    output_dir: &Path,
) -> Result<()> {
    let mut by_category: HashMap<String, Vec<(String, String, Vec<String>)>> = HashMap::new();
    let mut skipped_count = 0;

    for analysis in analyses {
        match link_map.get(&analysis.source_path) {
            Some(mapping) => {
                let tags = extract_tags(analysis);
                by_category
                    .entry(analysis.category.clone())
                    .or_default()
                    .push((analysis.title.clone(), mapping.filename.clone(), tags));
            }
            None => {
                skipped_count += 1;
                eprintln!("WARNING: COMPASS: No ID mapping for {}", analysis.source_path);
            }
        }
    }

    if skipped_count > 0 {
        eprintln!("WARNING: COMPASS: {} documents skipped (no ID mapping)", skipped_count);
    }

    let mut compass = format!(
        "---\nid: meta/navigation/compass\ntitle: Documentation Compass\ngenerated: {}\n---\n\n",
        chrono::Utc::now().to_rfc3339()
    );

    compass.push_str(&format!("# Documentation Compass\n\n> **{} documents**\n\n", analyses.len()));

    // By category
    for category in &["tutorial", "concept", "ref", "ops", "meta"] {
        if let Some(docs) = by_category.get(*category) {
            compass.push_str(&format!("## {}\n\n", category.to_uppercase()));
            for (title, filename, tags) in docs.iter().take(5) {
                let tag_str = tags.iter().take(2).map(|t| format!("`{}`", t)).collect::<Vec<_>>().join(" ");
                compass.push_str(&format!("- [{}](./docs/{}) {}\n", title, filename, tag_str));
            }
            compass.push('\n');
        }
    }

    let compass_file = output_dir.join("COMPASS.md");
    fs::write(compass_file, compass)?;

    Ok(())
}

fn extract_tags(analysis: &Analysis) -> Vec<String> {
    let mut tags = vec![analysis.category.clone()];

    for heading in analysis.headings.iter().take(3) {
        for word in heading.text.split_whitespace() {
            if word.len() > 4 && !is_stopword(&word.to_lowercase()) {
                tags.push(word.to_lowercase());
            }
        }
    }

    tags.sort();
    tags.dedup();
    tags.truncate(5);
    tags
}

fn is_stopword(word: &str) -> bool {
    matches!(
        word,
        "this" | "that" | "these" | "those" | "about" | "guide" | "the" | "and" | "or" | "for"
    )
}

/// Build a knowledge graph DAG from documents and chunks
fn build_knowledge_dag(
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
            title: format!("{} - {}", chunk.doc_title, chunk.heading.as_ref().unwrap_or(&"Intro".to_string())),
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

    // Detect and add related chunk edges (Jaccard similarity)
    let detector = RelationshipDetector::new(0.3);

    // Pre-build HashMap for O(1) lookups instead of O(n) linear scans
    let doc_metadata: HashMap<&str, (&[String], &str)> = document_tags
        .iter()
        .map(|(id, tags, cat)| (id.as_str(), (tags.as_slice(), cat.as_str())))
        .collect();

    for (i, chunk) in chunks.iter().enumerate() {
        let (chunk_tags, chunk_category) = doc_metadata
            .get(chunk.doc_id.as_str())
            .map(|(tags, cat)| (*tags, *cat))
            .unwrap_or((&[], ""));

        let all_chunks_metadata: Vec<(String, Vec<String>, String)> = chunks
            .iter()
            .enumerate()
            .filter(|(j, _)| j != &i)
            .map(|(_, c)| {
                let (tags, category) = doc_metadata
                    .get(c.doc_id.as_str())
                    .map(|(t, cat)| (t.to_vec(), cat.to_string()))
                    .unwrap_or_else(|| (Vec::new(), String::new()));

                (c.chunk_id.clone(), tags, category)
            })
            .collect();

        let related = detector.detect_relationships(&chunk.chunk_id, chunk_tags, chunk_category, &all_chunks_metadata);

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

    dag
}
