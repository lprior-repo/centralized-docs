//! Core types for the chunking adapter.
//!
//! Contains [`Chunk`] and [`ChunksResult`] extended types plus conversion
//! helpers between `ctd` and `contextual-chunker` representations.

use crate::analyze::Analysis;
use crate::assign::IdMapping;
use crate::types::Slug;
use anyhow::Result;
use contextual_chunker::{self, ChunkType, Document};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

/// Create directory with improved error context for permission issues
pub(crate) fn create_dir_with_context(path: &Path, context: &str) -> Result<()> {
    fs::create_dir_all(path).map_err(|e| {
        if e.kind() == io::ErrorKind::PermissionDenied {
            anyhow::anyhow!(
                "Permission denied: cannot create {} directory '{}'\n  \
                 Hint: Check directory permissions or run with appropriate access",
                context,
                path.display()
            )
        } else {
            anyhow::anyhow!(
                "Failed to create {} directory '{}': {}",
                context,
                path.display(),
                e
            )
        }
    })
}

/// Extended chunk type for `ctd` with knowledge graph relationships
///
/// This extends `contextual_chunker::Chunk` with `ctd`-specific fields
/// like `related_chunk_ids` for the knowledge graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub chunk_id: String,
    pub doc_id: String,
    pub doc_title: String,
    pub chunk_index: usize,
    pub content: String,
    pub token_count: usize,
    pub heading: Option<String>,
    pub heading_path: Vec<String>,
    pub chunk_type: ChunkType,
    pub previous_chunk_id: Option<String>,
    pub next_chunk_id: Option<String>,
    pub related_chunk_ids: Vec<String>,
    pub summary: String,
    pub chunk_level: contextual_chunker::ChunkLevel,
    pub parent_chunk_id: Option<String>,
    pub child_chunk_ids: Vec<String>,
    /// Context preserved from previous chunk for continuity
    pub context_prefix: Option<String>,
}

/// Extended chunking result for `ctd`
#[derive(Debug)]
pub struct ChunksResult {
    pub total_chunks: usize,
    pub document_count: usize,
    pub chunks_metadata: Vec<Chunk>,
    pub summary_chunks: usize,
    pub standard_chunks: usize,
    pub detailed_chunks: usize,
}

/// Convert Analysis to `contextual_chunker::Document`
///
/// Maps `ctd`'s Analysis type to the simpler Document type
/// used by `contextual-chunker`. Uses `link_map` to get the assigned doc ID,
/// falling back to a deterministic slugified ID if missing.
pub(crate) fn analysis_to_document(
    analysis: &Analysis,
    link_map: &HashMap<String, IdMapping>,
) -> Document {
    let doc_id = link_map
        .get(&analysis.source_path)
        .map_or_else(|| fallback_doc_id(analysis), |m| m.id.clone());

    let title = if analysis.title.is_empty() {
        "Untitled".to_string()
    } else {
        analysis.title.clone()
    };

    Document::new(doc_id, title, analysis.content.to_string())
}

pub(crate) fn fallback_doc_id(analysis: &Analysis) -> String {
    let parts: Vec<&str> = analysis.source_path.split('/').collect();
    let subcategory = parts
        .get(parts.len().saturating_sub(2))
        .map_or_else(|| "general".to_string(), |s| s.to_lowercase());
    let filename_stem = Path::new(&analysis.source_path)
        .file_stem()
        .filter(|s| !s.is_empty())
        .map_or_else(
            || "untitled".to_string(),
            |s| s.to_string_lossy().to_string(),
        );
    let slug = slugify(&filename_stem);

    format!("{}/{}/{}", analysis.category, subcategory, slug)
}

pub(crate) fn slugify(text: &str) -> String {
    let slug = text
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == ' ' {
                c
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-");
    Slug::from_text(&slug).into_string()
}

/// Convert `contextual_chunker::Chunk` to `doc_transformer::Chunk`
///
/// Creates extended chunk with empty `related_chunk_ids` (filled later by graph analysis)
pub(crate) fn convert_chunk(chunk: contextual_chunker::Chunk) -> Chunk {
    Chunk {
        chunk_id: chunk.chunk_id,
        doc_id: chunk.doc_id,
        doc_title: chunk.doc_title,
        chunk_index: chunk.chunk_index,
        content: chunk.content,
        token_count: chunk.token_count,
        heading: chunk.heading,
        heading_path: chunk.heading_path,
        chunk_type: chunk.chunk_type,
        previous_chunk_id: chunk.previous_chunk_id,
        next_chunk_id: chunk.next_chunk_id,
        related_chunk_ids: Vec::new(), // Populated later by knowledge graph
        summary: chunk.summary,
        chunk_level: chunk.chunk_level,
        parent_chunk_id: chunk.parent_chunk_id,
        child_chunk_ids: chunk.child_chunk_ids,
        context_prefix: chunk.context_prefix,
    }
}

/// Convert `contextual_chunker::ChunkingResult` to `doc_transformer::ChunksResult`
pub(crate) fn convert_chunking_result(
    result: contextual_chunker::ChunkingResult,
    document_count: usize,
) -> ChunksResult {
    let chunks_metadata = result.chunks.into_iter().map(convert_chunk).collect();

    ChunksResult {
        total_chunks: result
            .summary_count
            .saturating_add(result.standard_count)
            .saturating_add(result.detailed_count),
        document_count,
        chunks_metadata,
        summary_chunks: result.summary_count,
        standard_chunks: result.standard_count,
        detailed_chunks: result.detailed_count,
    }
}

/// Escape frontmatter values
pub(crate) fn escape_frontmatter(s: &str) -> String {
    s.replace('\n', " ").replace('\"', "\\\"")
}
