//! Core types for the index module.

use contextual_chunker::ChunkType;
use serde::{Deserialize, Serialize};
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
    pub related_chunks: Vec<RelatedChunk>,
    pub chunk_level: contextual_chunker::ChunkLevel,
    pub parent_chunk_id: Option<String>,
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
pub struct DocumentIndexResult {
    pub documents: Vec<IndexDocument>,
    pub keywords: std::collections::HashMap<String, Vec<String>>,
    pub document_tags: Vec<(String, Vec<String>, String)>,
}
