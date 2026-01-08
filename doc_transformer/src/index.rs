use crate::analyze::Analysis;
use crate::assign::IdMapping;
use crate::chunk::ChunksResult;
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

    // Build document index
    for analysis in analyses {
        if let Some(mapping) = link_map.get(&analysis.source_path) {
            let tags = extract_tags(analysis);

            // Build keywords from headings
            for heading in &analysis.headings {
                for word in heading.text.split_whitespace() {
                    let word_lower = word.to_lowercase();
                    if word_lower.len() > 3 && !is_stopword(&word_lower) {
                        keywords
                            .entry(word_lower)
                            .or_insert_with(Vec::new)
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
            path: format!("chunks/{}.md", chunk.chunk_id.replace('/', "-").replace('#', "-")),
        });
    }

    let index = json!({
        "version": "4.2",
        "generated": chrono::Utc::now().to_rfc3339(),
        "stats": {
            "doc_count": documents.len(),
            "chunk_count": chunks_result.total_chunks,
            "avg_chunk_size_tokens": if chunks_result.total_chunks > 0 {
                chunks_result.chunks_metadata.iter().map(|c| c.token_count).sum::<usize>() / chunks_result.total_chunks
            } else {
                0
            }
        },
        "documents": documents,
        "chunks": chunks_metadata,
        "keywords": keywords,
        "navigation": {
            "type": "contextual_retrieval",
            "strategy": "50-100 token context prefix + H2 boundaries",
            "avg_tokens_per_chunk": 170
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

    for analysis in analyses {
        if let Some(mapping) = link_map.get(&analysis.source_path) {
            let tags = extract_tags(analysis);
            by_category
                .entry(analysis.category.clone())
                .or_insert_with(Vec::new)
                .push((analysis.title.clone(), mapping.filename.clone(), tags));
        }
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
