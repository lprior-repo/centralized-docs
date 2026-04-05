//! Core chunking types — `Chunk`, `ChunkLevel`, `ChunkType`, `ChunkingResult`.
//!
//! Design by Contract:
//! - Invariants: All chunks have non-empty content and valid IDs
//! - Precondition: Token counts must be consistent within ±10%
//! - Postcondition: Parent-child relationships form valid DAG (no cycles)

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

// ---------------------------------------------------------------------------
// Regex statics — compiled once, shared across all modules
// ---------------------------------------------------------------------------

pub(crate) static TABLE_REGEX: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"\|.*\|").ok());

pub(crate) static HEADING_REGEX: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"^(#{1,6})\s+(.+)$").ok());

pub(crate) fn table_regex() -> anyhow::Result<&'static Regex> {
    TABLE_REGEX
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("TABLE regex failed to compile"))
}

pub(crate) fn heading_regex() -> anyhow::Result<&'static Regex> {
    HEADING_REGEX
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("HEADING regex failed to compile"))
}

// ---------------------------------------------------------------------------
// ChunkLevel
// ---------------------------------------------------------------------------

/// Hierarchical chunk level for multi-granularity retrieval.
///
/// Documents can be chunked at three levels simultaneously,
/// with parent-child relationships allowing progressive disclosure:
///
/// - **Summary**: ~128 tokens - High-level overview for quick retrieval
/// - **Standard**: ~512 tokens - Balanced detail for most use cases
/// - **Detailed**: ~1024 tokens - Full context for deep understanding
///
/// # Example
///
/// ```
/// use contextual_chunker::ChunkLevel;
///
/// let level = ChunkLevel::Standard;
/// assert_eq!(level.target_tokens(), 512);
/// assert_eq!(level.as_str(), "standard");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChunkLevel {
    Summary,
    Standard,
    Detailed,
}

impl ChunkLevel {
    /// Target token count for this level.
    #[must_use]
    pub fn target_tokens(&self) -> usize {
        match self {
            ChunkLevel::Summary => 128,
            ChunkLevel::Standard => 512,
            ChunkLevel::Detailed => 1024,
        }
    }

    /// Overlap tokens for text-splitter at this level.
    ///
    /// Single source of truth — used by `create_chunks_at_level` and
    /// `ContextualChunker` factory methods (C8).
    #[must_use]
    pub fn overlap_tokens(&self) -> usize {
        match self {
            ChunkLevel::Summary => 30,
            ChunkLevel::Standard => 100,
            ChunkLevel::Detailed => 200,
        }
    }

    /// String representation (matches serialization format).
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            ChunkLevel::Summary => "summary",
            ChunkLevel::Standard => "standard",
            ChunkLevel::Detailed => "detailed",
        }
    }
}

// ---------------------------------------------------------------------------
// ChunkType
// ---------------------------------------------------------------------------

/// Content type classification for a chunk.
///
/// Makes illegal states unrepresentable: the domain has exactly three
/// valid content types, no string parsing needed after construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChunkType {
    /// Chunk dominated by fenced code blocks (≥5 pairs)
    Code,
    /// Chunk containing a markdown table
    Table,
    /// General prose content
    Prose,
}

impl ChunkType {
    /// Canonical string form for display / serialization compatibility.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            ChunkType::Code => "code",
            ChunkType::Table => "table",
            ChunkType::Prose => "prose",
        }
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

// ---------------------------------------------------------------------------
// Chunk ID generation
// ---------------------------------------------------------------------------

/// Generate a chunk ID: `{doc_id}#{chunk_index}-{level}`
pub(crate) fn generate_chunk_id(doc_id: &str, chunk_index: usize, level: ChunkLevel) -> String {
    format!("{doc_id}#{chunk_index}-{}", level.as_str())
}

// ---------------------------------------------------------------------------
// Chunk
// ---------------------------------------------------------------------------

/// A semantic chunk of a document.
///
/// Chunks preserve document context through:
/// - Hierarchical relationships (parent/child)
/// - Navigation links (previous/next at same level)
/// - Content analysis (type detection, summarization)
///
/// # Chunk ID Format
///
/// `{doc_id}#{index}-{level}` — e.g. `guides-intro#0-summary`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    /// Unique chunk identifier
    pub chunk_id: String,
    /// Original document ID
    pub doc_id: String,
    /// Original document title
    pub doc_title: String,
    /// Index of this chunk within its document (0-based)
    pub chunk_index: usize,
    /// The actual chunk content (markdown)
    pub content: String,
    /// Context prefix from previous section.
    ///
    /// Always `None` within this crate — overlap is handled by text-splitter.
    /// Kept for downstream API compatibility (centralized-docs crate).
    pub context_prefix: Option<String>,
    /// Estimated token count for this chunk
    pub token_count: usize,
    /// The heading that introduces this chunk (if any)
    pub heading: Option<String>,
    /// Full heading path (e.g. `["Guide", "Setup", "Install"]`)
    pub heading_path: Vec<String>,
    /// Content type classification — code-heavy, table-based, or prose
    pub chunk_type: ChunkType,
    /// ID of previous chunk at same level (None for first)
    pub previous_chunk_id: Option<String>,
    /// ID of next chunk at same level (None for last)
    pub next_chunk_id: Option<String>,
    /// Summary of chunk content (extractive, ~200 chars max)
    pub summary: String,
    /// The hierarchical level of this chunk
    pub chunk_level: ChunkLevel,
    /// Parent chunk ID (from higher level)
    pub parent_chunk_id: Option<String>,
    /// Child chunk IDs (at lower level)
    pub child_chunk_ids: Vec<String>,
}

// ---------------------------------------------------------------------------
// ChunkingResult
// ---------------------------------------------------------------------------

/// Result of chunking one or more documents.
///
/// Aggregates all chunks and provides summary statistics.
pub struct ChunkingResult {
    /// All chunks from all input documents
    pub chunks: Vec<Chunk>,
    /// Count of Summary-level chunks
    pub summary_count: usize,
    /// Count of Standard-level chunks
    pub standard_count: usize,
    /// Count of Detailed-level chunks
    pub detailed_count: usize,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    #![allow(clippy::expect_used)]

    use super::*;

    #[test]
    fn test_chunk_level_tokens() {
        assert_eq!(ChunkLevel::Summary.target_tokens(), 128);
        assert_eq!(ChunkLevel::Standard.target_tokens(), 512);
        assert_eq!(ChunkLevel::Detailed.target_tokens(), 1024);
    }

    #[test]
    fn test_chunk_level_overlap() {
        assert_eq!(ChunkLevel::Summary.overlap_tokens(), 30);
        assert_eq!(ChunkLevel::Standard.overlap_tokens(), 100);
        assert_eq!(ChunkLevel::Detailed.overlap_tokens(), 200);
    }

    #[test]
    fn test_chunk_level_str() {
        assert_eq!(ChunkLevel::Summary.as_str(), "summary");
        assert_eq!(ChunkLevel::Standard.as_str(), "standard");
        assert_eq!(ChunkLevel::Detailed.as_str(), "detailed");
    }
}
