//! Persisted types and conversions for the Chunk pipeline phase.

use super::error::{require_non_empty, require_schema_v1, PersistError};
use crate::chunking_adapter::{Chunk, ChunksResult};
use contextual_chunker::{ChunkLevel, ChunkType};

// ---------------------------------------------------------------------------
// Persisted Record Types — Chunk Family
// ---------------------------------------------------------------------------

/// Persisted chunk type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum PersistedChunkType {
    /// Code-dominated chunk.
    Code,
    /// Table-containing chunk.
    Table,
    /// General prose chunk.
    Prose,
}

/// Persisted chunk level hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum PersistedChunkLevel {
    /// High-level overview (~128 tokens).
    Summary,
    /// Balanced detail (~512 tokens).
    Standard,
    /// Full context (~1024 tokens).
    Detailed,
}

/// Persisted extended chunk with knowledge graph relationships.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedChunk {
    /// Schema version (always 1).
    pub schema_version: u32,
    /// Chunk identifier (format: "{`doc_id`}#{`index`}").
    pub chunk_id: String,
    /// Parent document identifier.
    pub doc_id: String,
    /// Parent document title.
    pub doc_title: String,
    /// Index of this chunk within the document.
    pub chunk_index: usize,
    /// Chunk text content (non-empty).
    pub content: String,
    /// Estimated token count (> 0).
    pub token_count: usize,
    /// Optional heading this chunk falls under.
    pub heading: Option<String>,
    /// Full heading path from root.
    pub heading_path: Vec<String>,
    /// Content type classification.
    pub chunk_type: PersistedChunkType,
    /// Previous chunk in document sequence.
    pub previous_chunk_id: Option<String>,
    /// Next chunk in document sequence.
    pub next_chunk_id: Option<String>,
    /// Related chunks via knowledge graph.
    pub related_chunk_ids: Vec<String>,
    /// Extractive summary of chunk content.
    pub summary: String,
    /// Hierarchical level.
    pub chunk_level: PersistedChunkLevel,
    /// Parent chunk in hierarchy (Summary → Standard).
    pub parent_chunk_id: Option<String>,
    /// Child chunks in hierarchy (Standard → Detailed).
    pub child_chunk_ids: Vec<String>,
    /// Context preserved from previous chunk.
    pub context_prefix: Option<String>,
}

/// Persisted batch chunking result.
#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PersistedChunksResult {
    /// Schema version (always 1).
    pub schema_version: u32,
    /// Total chunks created across all documents.
    pub total_chunks: usize,
    /// Number of documents chunked.
    pub document_count: usize,
    /// Metadata for each chunk.
    pub chunks_metadata: Vec<PersistedChunk>,
    /// Count of summary-level chunks.
    pub summary_chunks: usize,
    /// Count of standard-level chunks.
    pub standard_chunks: usize,
    /// Count of detailed-level chunks.
    pub detailed_chunks: usize,
}

// ===========================================================================
// Conversions: Runtime → Persisted (Infallible)
// ===========================================================================

/// Convert a runtime [`ChunkType`] to its persisted form.
#[must_use]
pub fn chunk_type_to_persisted(t: &ChunkType) -> PersistedChunkType {
    match t {
        ChunkType::Code => PersistedChunkType::Code,
        ChunkType::Table => PersistedChunkType::Table,
        ChunkType::Prose => PersistedChunkType::Prose,
    }
}

/// Convert a runtime [`ChunkLevel`] to its persisted form.
#[must_use]
pub fn chunk_level_to_persisted(l: &ChunkLevel) -> PersistedChunkLevel {
    match l {
        ChunkLevel::Summary => PersistedChunkLevel::Summary,
        ChunkLevel::Standard => PersistedChunkLevel::Standard,
        ChunkLevel::Detailed => PersistedChunkLevel::Detailed,
    }
}

/// Convert a runtime ctd [`Chunk`] to its persisted form.
#[must_use]
pub fn chunk_to_persisted(c: &Chunk) -> PersistedChunk {
    PersistedChunk {
        schema_version: 1,
        chunk_id: c.chunk_id.clone(),
        doc_id: c.doc_id.clone(),
        doc_title: c.doc_title.clone(),
        chunk_index: c.chunk_index,
        content: c.content.clone(),
        token_count: c.token_count,
        heading: c.heading.clone(),
        heading_path: c.heading_path.clone(),
        chunk_type: chunk_type_to_persisted(&c.chunk_type),
        previous_chunk_id: c.previous_chunk_id.clone(),
        next_chunk_id: c.next_chunk_id.clone(),
        related_chunk_ids: c.related_chunk_ids.clone(),
        summary: c.summary.clone(),
        chunk_level: chunk_level_to_persisted(&c.chunk_level),
        parent_chunk_id: c.parent_chunk_id.clone(),
        child_chunk_ids: c.child_chunk_ids.clone(),
        context_prefix: c.context_prefix.clone(),
    }
}

/// Convert a runtime [`ChunksResult`] to its persisted form.
#[must_use]
pub fn chunks_result_to_persisted(r: &ChunksResult) -> PersistedChunksResult {
    PersistedChunksResult {
        schema_version: 1,
        total_chunks: r.total_chunks,
        document_count: r.document_count,
        chunks_metadata: r.chunks_metadata.iter().map(chunk_to_persisted).collect(),
        summary_chunks: r.summary_chunks,
        standard_chunks: r.standard_chunks,
        detailed_chunks: r.detailed_chunks,
    }
}

// ===========================================================================
// Conversions: Persisted → Runtime (Fallible)
// ===========================================================================

/// Convert a persisted chunk type back to runtime form (1:1 mapping, always succeeds).
pub fn persisted_chunk_type_to_runtime(p: PersistedChunkType) -> Result<ChunkType, PersistError> {
    match p {
        PersistedChunkType::Code => Ok(ChunkType::Code),
        PersistedChunkType::Table => Ok(ChunkType::Table),
        PersistedChunkType::Prose => Ok(ChunkType::Prose),
    }
}

/// Convert a persisted chunk level back to runtime form (1:1 mapping, always succeeds).
pub fn persisted_chunk_level_to_runtime(
    p: PersistedChunkLevel,
) -> Result<ChunkLevel, PersistError> {
    match p {
        PersistedChunkLevel::Summary => Ok(ChunkLevel::Summary),
        PersistedChunkLevel::Standard => Ok(ChunkLevel::Standard),
        PersistedChunkLevel::Detailed => Ok(ChunkLevel::Detailed),
    }
}

/// Convert a persisted chunk back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::SchemaVersionMismatch`] if `schema_version` != 1.
/// Returns [`PersistError::EmptyField`] if `chunk_id`, `doc_id`, or content is empty.
/// Returns [`PersistError::OutOfRange`] if `token_count` == 0.
pub fn persisted_chunk_to_runtime(p: &PersistedChunk) -> Result<Chunk, PersistError> {
    require_schema_v1(p.schema_version)?;
    require_non_empty(&p.chunk_id, "chunk_id")?;
    require_non_empty(&p.doc_id, "doc_id")?;
    require_non_empty(&p.content, "content")?;
    if p.token_count == 0 {
        return Err(PersistError::OutOfRange {
            field: "token_count".to_string(),
            value: 0,
            min: 1,
            max: i64::MAX,
        });
    }

    Ok(Chunk {
        chunk_id: p.chunk_id.clone(),
        doc_id: p.doc_id.clone(),
        doc_title: p.doc_title.clone(),
        chunk_index: p.chunk_index,
        content: p.content.clone(),
        token_count: p.token_count,
        heading: p.heading.clone(),
        heading_path: p.heading_path.clone(),
        chunk_type: persisted_chunk_type_to_runtime(p.chunk_type)?,
        previous_chunk_id: p.previous_chunk_id.clone(),
        next_chunk_id: p.next_chunk_id.clone(),
        related_chunk_ids: p.related_chunk_ids.clone(),
        summary: p.summary.clone(),
        chunk_level: persisted_chunk_level_to_runtime(p.chunk_level)?,
        parent_chunk_id: p.parent_chunk_id.clone(),
        child_chunk_ids: p.child_chunk_ids.clone(),
        context_prefix: p.context_prefix.clone(),
    })
}

/// Convert a persisted chunks result back to runtime form.
///
/// # Errors
///
/// Returns [`PersistError::SchemaVersionMismatch`] if `schema_version` != 1.
/// Propagates any errors from nested chunk conversions.
pub fn persisted_chunks_result_to_runtime(
    p: &PersistedChunksResult,
) -> Result<ChunksResult, PersistError> {
    require_schema_v1(p.schema_version)?;
    let chunks_metadata = p
        .chunks_metadata
        .iter()
        .map(persisted_chunk_to_runtime)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ChunksResult {
        total_chunks: p.total_chunks,
        document_count: p.document_count,
        chunks_metadata,
        summary_chunks: p.summary_chunks,
        standard_chunks: p.standard_chunks,
        detailed_chunks: p.detailed_chunks,
    })
}
