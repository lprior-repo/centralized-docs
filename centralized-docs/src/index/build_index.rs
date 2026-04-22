//! Build document index and chunk metadata from analyses and link mapping.

use super::index_assembly::{
    assemble_index_json, build_tantivy_index, compute_graph_analytics, write_index_file,
};
use super::navigation::extract_tags;
use super::types::{ChunkMetadata, DocumentIndexResult, IndexDocument, RelatedChunk};
use crate::analyze::Analysis;
use crate::assign::IdMapping;
use crate::chunking_adapter::{Chunk, ChunksResult};
use crate::graph::KnowledgeDAG;
use crate::types::bounded_chunk_name;
use crate::types::is_stopword;
use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Build and write the search index to disk.
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
    let doc_index = build_document_index(analyses, link_map, chunks_result);
    let dag = super::knowledge_dag::build_knowledge_dag(
        &doc_index.documents,
        &chunks_result.chunks_metadata,
        &doc_index.document_tags,
        max_related_chunks,
        hnsw_m,
        hnsw_ef_construction,
        max_chunk_keywords,
    )?;
    let chunks_metadata = build_chunk_metadata(&chunks_result.chunks_metadata, &dag)?;
    let analytics = compute_graph_analytics(&dag, &doc_index.documents);
    let index_json = assemble_index_json(
        &doc_index.documents,
        &chunks_metadata,
        &doc_index.keywords,
        &dag,
        &analytics,
        chunks_result.total_chunks,
        project_name,
    );
    write_index_file(output_dir, &index_json)?;
    build_tantivy_index(
        output_dir,
        &doc_index.documents,
        &chunks_result.chunks_metadata,
    )?;
    Ok(())
}

/// Build document index from analyses and link mapping.
#[allow(clippy::implicit_hasher)]
pub fn build_document_index<S: std::hash::BuildHasher>(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping, S>,
    chunks_result: &ChunksResult,
) -> DocumentIndexResult {
    let doc_chunks_index: HashMap<&str, Vec<String>> = chunks_result
        .chunks_metadata
        .iter()
        .map(|chunk| (chunk.doc_id.as_str(), chunk.chunk_id.clone()))
        .into_group_map();

    let analyses_with_mapping: Vec<_> = analyses
        .iter()
        .filter_map(|a| link_map.get(&a.source_path).map(|m| (a, m)))
        .collect();

    let documents: Vec<IndexDocument> = analyses_with_mapping
        .iter()
        .map(|(analysis, mapping)| {
            let tags = extract_tags(analysis);
            let chunk_ids = doc_chunks_index
                .get(mapping.id.as_str())
                .cloned()
                .unwrap_or_default();
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
        .map(|(a, m)| (m.id.clone(), extract_tags(a), a.category.clone()))
        .collect();

    let keywords: HashMap<String, Vec<String>> = analyses_with_mapping
        .iter()
        .flat_map(|(analysis, mapping)| {
            analysis.headings.iter().flat_map(move |heading| {
                heading.text.split_whitespace().filter_map(move |word| {
                    let wl = word.to_lowercase();
                    if wl.len() > 3 && !is_stopword(&wl) {
                        Some((wl, mapping.id.clone()))
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
pub fn build_chunk_metadata(chunks: &[Chunk], dag: &KnowledgeDAG) -> Result<Vec<ChunkMetadata>> {
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
                .filter(|h| h.as_str() != "Intro")
                .map(|h| slugify_heading(h));
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
                    "chunks/{}",
                    bounded_chunk_name(
                        &chunk.chunk_id.replace(['/', '#'], "-"),
                        chunk.chunk_level.as_str()
                    )
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

pub(crate) fn slugify_heading(text: &str) -> String {
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
