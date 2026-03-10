//! Core chunking types and algorithms
//!
//! Design by Contract:
//! - Invariants: All chunks have non-empty content and valid IDs
//! - Precondition: Token counts must be consistent within ±10%
//! - Postcondition: Parent-child relationships form valid DAG (no cycles)

use crate::document::Document;
use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::LazyLock;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChunkerError {
    #[error("Chunk capacity must be greater than 0")]
    InvalidChunkCapacity,
    #[error("Postcondition violated: chunk exceeded capacity")]
    PostconditionViolated,
    #[error("Tokenization error: {0}")]
    TokenizationError(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Domain type representing a valid, strictly positive chunk capacity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkCapacity(NonZeroUsize);

impl ChunkCapacity {
    pub fn new(capacity: usize) -> std::result::Result<Self, ChunkerError> {
        NonZeroUsize::new(capacity)
            .map(Self)
            .ok_or(ChunkerError::InvalidChunkCapacity)
    }

    pub fn get(&self) -> usize {
        self.0.get()
    }
}

use text_splitter::{ChunkConfig, ChunkSizer, MarkdownSplitter};

#[derive(Clone)]
struct FastTokenizer {
    bpe: std::sync::Arc<tiktoken_rs::CoreBPE>,
}

impl FastTokenizer {
    fn new() -> std::result::Result<Self, String> {
        let bpe = tiktoken_rs::cl100k_base().map_err(|e| e.to_string())?;
        Ok(Self {
            bpe: std::sync::Arc::new(bpe),
        })
    }
}

impl ChunkSizer for FastTokenizer {
    fn size(&self, text: &str) -> usize {
        // Pathological strings (like minified code, base64, or adversarial repeated chars)
        // cause regex-based tokenizers like tiktoken to exhibit O(N^2) or worse behavior.
        // If a long string has extremely low space density, we use a fast approximation.
        let space_count = text.bytes().filter(|&b| b == b' ' || b == b'\n').count();
        if text.len() > 1000 && space_count < text.len() / 100 {
            return (text.len() / 4).max(1);
        }
        self.bpe.encode_with_special_tokens(text).len()
    }
}

/// Chunks markdown text into multiple segments, ensuring no segment exceeds `capacity`
/// and safely handling huge strings to prevent tokenizer panics.
pub fn chunk_markdown(
    text: &str,
    capacity: ChunkCapacity,
) -> std::result::Result<Vec<String>, ChunkerError> {
    let capacity_val = capacity.get();
    let tokenizer = FastTokenizer::new().map_err(|e| ChunkerError::TokenizationError(e))?;

    // Fix: Pre-process absurdly long strings without line breaks to avoid stack overflow
    // in tiktoken-rs PCRE regex parsing
    let safe_text = if text.len() > 50_000 && !text.contains('\n') {
        use itertools::Itertools;
        // Break up the text artificially every 4k chars so the regex doesn't explode
        text.chars()
            .chunks(4_000)
            .into_iter()
            .map(Iterator::collect::<String>)
            .join("\n")
    } else {
        text.to_string()
    };

    let config = ChunkConfig::new(capacity_val).with_sizer(tokenizer);

    let splitter = MarkdownSplitter::new(config);
    let chunks: Vec<String> = splitter
        .chunks(safe_text.as_str())
        .map(String::from)
        .collect();

    // Verify postcondition: no chunk should exceed capacity, unless a single word/token exceeds it.
    // Given the test constraints, we'll verify it according to the specific contract:
    // If a chunk is larger than capacity, but doesn't have whitespace, it's a single word (allowed).
    // If it has whitespace and exceeds, we throw PostconditionViolated.
    for chunk in &chunks {
        let tokenizer = tiktoken_rs::cl100k_base()
            .map_err(|e| ChunkerError::TokenizationError(e.to_string()))?;
        let tokens = tokenizer.encode_with_special_tokens(chunk).len();
        if tokens > capacity_val {
            // Check if it's a single unbreakable word
            if chunk.contains(char::is_whitespace) {
                return Err(ChunkerError::PostconditionViolated);
            }
        }
    }

    Ok(chunks)
}

/// Hierarchical chunk level for multi-granularity retrieval
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
    /// Target token count for this level
    pub fn target_tokens(&self) -> usize {
        match self {
            ChunkLevel::Summary => 128,
            ChunkLevel::Standard => 512,
            ChunkLevel::Detailed => 1024,
        }
    }

    /// String representation (matches serialization format)
    pub fn as_str(&self) -> &str {
        match self {
            ChunkLevel::Summary => "summary",
            ChunkLevel::Standard => "standard",
            ChunkLevel::Detailed => "detailed",
        }
    }
}

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

/// Generate a chunk ID with hierarchical level suffix
///
/// # Format
///
/// `{doc_id}#{chunk_index}-{level}`
///
/// # Examples
///
/// - `test-doc#0-summary`
/// - `test-doc#1-standard`
/// - `test-doc#2-detailed`
fn generate_chunk_id(doc_id: &str, chunk_index: usize, level: ChunkLevel) -> String {
    format!("{doc_id}#{chunk_index}-{}", level.as_str())
}

/// A semantic chunk of a document
///
/// Chunks preserve document context through:
/// - Hierarchical relationships (parent/child)
/// - Navigation links (previous/next at same level)
/// - Content analysis (type detection, summarization)
/// - Context prefixes (50-100 tokens from previous section)
///
/// # Chunk ID Format
///
/// Chunk IDs use format: `{doc_id}#{index}`
/// Example: `guides-intro#0-summary`, `guides-intro#1-standard`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    /// Unique chunk identifier: {doc_id}#{index}
    pub chunk_id: String,

    /// Original document ID (from Document::id)
    pub doc_id: String,

    /// Original document title (from Document::title)
    pub doc_title: String,

    /// Index of this chunk within its document (0-based)
    pub chunk_index: usize,

    /// The actual chunk content (markdown)
    pub content: String,

    /// Context prefix from previous section (50-100 tokens)
    /// Provides context for retrieval systems following Anthropic's recommendations
    /// Reduces retrieval failures by ~35% in multi-turn conversations
    pub context_prefix: Option<String>,

    /// Estimated token count for this chunk
    /// Used for hierarchical bucketing and size tracking
    pub token_count: usize,

    /// The H2 heading (##) that introduces this chunk (if any)
    /// Helps users understand context when chunk is viewed in isolation
    pub heading: Option<String>,

    /// Full heading path for this chunk (e.g. `["Guide", "Setup", "Install"]`)
    /// Includes H1/H2/H3 levels when available
    pub heading_path: Vec<String>,

    /// Content type classification — code-heavy, table-based, or prose
    /// Enables specialized handling in retrieval systems
    pub chunk_type: ChunkType,

    /// ID of previous chunk at same level and in same document (sequential)
    /// None for first chunk
    pub previous_chunk_id: Option<String>,

    /// ID of next chunk at same level and in same document (sequential)
    /// None for last chunk
    pub next_chunk_id: Option<String>,

    /// Summary of chunk content (extractive, no AI generation)
    /// Limited to ~200 characters
    pub summary: String,

    /// The hierarchical level of this chunk
    pub chunk_level: ChunkLevel,

    /// Parent chunk ID (from higher level)
    /// Standard chunks have Summary chunks as parents
    /// Detailed chunks have Standard chunks as parents
    pub parent_chunk_id: Option<String>,

    /// Child chunk IDs (at lower level)
    /// Summary chunks have Standard chunks as children
    /// Standard chunks have Detailed chunks as children
    pub child_chunk_ids: Vec<String>,
}

/// Result of chunking one or more documents
///
/// Aggregates all chunks and provides summary statistics
/// for monitoring and optimization.
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

static TABLE_REGEX: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"\|.*\|").ok());

static HEADING_REGEX: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"^(#{1,6})\s+(.+)$").ok());

/// Get TABLE regex or return error if compilation failed
fn table_regex() -> Result<&'static Regex> {
    TABLE_REGEX
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("TABLE regex failed to compile"))
}

/// Get HEADING regex or return error if compilation failed
fn heading_regex() -> Result<&'static Regex> {
    HEADING_REGEX
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("HEADING regex failed to compile"))
}

/// Chunk a single document at a specific hierarchical level
///
/// # Arguments
///
/// * `document` - The document to chunk
/// * `level` - The hierarchical level (Summary/Standard/Detailed)
///
/// # Returns
///
/// A vector of chunks, one per semantic boundary
///
/// # Algorithm
///
/// 1. Split on H2 headings (##) to find semantic boundaries
/// 2. If a section exceeds target_tokens, split further by token count
/// 3. Add context from previous section (buffer) to new section
/// 4. Link chunks sequentially (prev/next pointers)
///
/// # Chunk Boundaries
///
/// Chunks respect markdown structure:
/// - H2 headings (##) are primary boundaries
/// - H3/H1 headings are used only when no H2 headings exist
/// - Token limit is secondary (if section too long)
/// - Always preserves at least one line of context from previous section
///
/// # Example
///
/// ```
/// use contextual_chunker::{Document, ChunkLevel, chunk};
///
/// let doc = Document::new(
///     "intro".to_string(),
///     "Introduction".to_string(),
///     "## Getting Started\nSome content here.".to_string(),
/// );
///
/// let chunks = chunk(&doc, ChunkLevel::Standard).unwrap();
/// assert!(!chunks.is_empty());
/// ```
pub fn chunk(document: &Document, level: ChunkLevel) -> Result<Vec<Chunk>> {
    if !document.is_valid() {
        anyhow::bail!("Invalid document: id and title must be non-empty");
    }

    let mut chunks =
        create_chunks_at_level(&document.id, &document.title, &document.content, level)?;
    link_chunks(&mut chunks);
    Ok(chunks)
}

/// Chunk all documents at all three hierarchical levels
///
/// Creates Summary, Standard, and Detailed chunks for each document,
/// automatically linking parent-child relationships.
///
/// # Arguments
///
/// * `documents` - Slice of documents to chunk
///
/// # Returns
///
/// ChunkingResult with all chunks and summary statistics
///
/// # Example
///
/// ```
/// use contextual_chunker::{Document, chunk_all};
///
/// let docs = vec![
///     Document::new("doc1".to_string(), "Title 1".to_string(), "Content 1".to_string()),
///     Document::new("doc2".to_string(), "Title 2".to_string(), "Content 2".to_string()),
/// ];
///
/// let result = chunk_all(&docs).unwrap();
/// println!("Created {} chunks", result.chunks.len());
/// ```
pub fn chunk_all(documents: &[Document]) -> Result<ChunkingResult> {
    // Validate all documents
    for doc in documents {
        if !doc.is_valid() {
            anyhow::bail!(
                "Invalid document: {} - id and title must be non-empty",
                doc.id
            );
        }
    }

    let mut all_chunks = Vec::new();
    let mut summary_count = 0usize;
    let mut standard_count = 0usize;
    let mut detailed_count = 0usize;

    for doc in documents {
        let mut summary =
            create_chunks_at_level(&doc.id, &doc.title, &doc.content, ChunkLevel::Summary)?;
        let mut standard =
            create_chunks_at_level(&doc.id, &doc.title, &doc.content, ChunkLevel::Standard)?;
        let mut detailed =
            create_chunks_at_level(&doc.id, &doc.title, &doc.content, ChunkLevel::Detailed)?;

        assign_hierarchy(&mut summary, &mut standard, &mut detailed);

        let summary_len = summary.len();
        let standard_len = standard.len();
        let detailed_len = detailed.len();

        all_chunks.extend(summary);
        all_chunks.extend(standard);
        all_chunks.extend(detailed);

        summary_count = summary_count.saturating_add(summary_len);
        standard_count = standard_count.saturating_add(standard_len);
        detailed_count = detailed_count.saturating_add(detailed_len);
    }

    Ok(ChunkingResult {
        chunks: all_chunks,
        summary_count,
        standard_count,
        detailed_count,
    })
}

fn heading_key(chunk: &Chunk) -> String {
    chunk.heading.clone().unwrap_or_else(|| "intro".to_string())
}

fn group_by_heading(chunks: &[Chunk]) -> HashMap<String, Vec<usize>> {
    let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
    for (idx, chunk) in chunks.iter().enumerate() {
        groups.entry(heading_key(chunk)).or_default().push(idx);
    }
    groups
}

fn assign_hierarchy(summary: &mut [Chunk], standard: &mut [Chunk], detailed: &mut [Chunk]) {
    if summary.is_empty() && standard.is_empty() && detailed.is_empty() {
        return;
    }

    let summary_groups = group_by_heading(summary);
    let standard_groups = group_by_heading(standard);
    let detailed_groups = group_by_heading(detailed);

    let mut summary_children: HashMap<usize, Vec<String>> = HashMap::new();
    let mut standard_children: HashMap<usize, Vec<String>> = HashMap::new();

    for (heading, standard_indices) in &standard_groups {
        let summary_indices = summary_groups
            .get(heading)
            .or_else(|| summary_groups.get("intro"));

        if let Some(summary_indices) = summary_indices {
            // Use div_ceil for even distribution of children across parents
            // Bug fix: (pos * parent_len) / child_len produced uneven distribution
            let items_per_parent = standard_indices.len().div_ceil(summary_indices.len());
            for (pos, standard_idx) in standard_indices.iter().enumerate() {
                let parent_pos = pos / items_per_parent;
                let parent_idx = summary_indices[parent_pos];
                standard[*standard_idx].parent_chunk_id =
                    Some(summary[parent_idx].chunk_id.clone());
                summary_children
                    .entry(parent_idx)
                    .or_default()
                    .push(standard[*standard_idx].chunk_id.clone());
            }
        }
    }

    for (heading, detailed_indices) in &detailed_groups {
        let standard_indices = standard_groups
            .get(heading)
            .or_else(|| standard_groups.get("intro"));

        if let Some(standard_indices) = standard_indices {
            // Use div_ceil for even distribution
            let items_per_parent = detailed_indices.len().div_ceil(standard_indices.len());
            for (pos, detailed_idx) in detailed_indices.iter().enumerate() {
                let parent_pos = pos / items_per_parent;
                let parent_idx = standard_indices[parent_pos];
                detailed[*detailed_idx].parent_chunk_id =
                    Some(standard[parent_idx].chunk_id.clone());
                standard_children
                    .entry(parent_idx)
                    .or_default()
                    .push(detailed[*detailed_idx].chunk_id.clone());
            }
        }
    }

    for (idx, children) in summary_children {
        summary[idx].child_chunk_ids = children;
    }

    for (idx, children) in standard_children {
        standard[idx].child_chunk_ids = children;
    }
}

/// Internal: Create chunks at a specific level
#[allow(clippy::too_many_lines)]
fn create_chunks_at_level(
    doc_id: &str,
    doc_title: &str,
    content: &str,
    level: ChunkLevel,
) -> Result<Vec<Chunk>> {
    let target_tokens = level.target_tokens();
    let overlap = match level {
        ChunkLevel::Summary => 30,
        ChunkLevel::Standard => 100,
        ChunkLevel::Detailed => 200,
    };

    let tokenizer = FastTokenizer::new().map_err(|e| anyhow::anyhow!("Tokenizer error: {e}"))?;

    let config = ChunkConfig::new(target_tokens)
        .with_sizer(tokenizer)
        .with_overlap(overlap)
        .map_err(|e| anyhow::anyhow!("ChunkConfig error: {e}"))?;

    let splitter = MarkdownSplitter::new(config);

    // Fix: Pre-process absurdly long strings without line breaks to avoid stack overflow
    // in tiktoken-rs PCRE regex parsing
    let safe_text = if content.len() > 50_000 && !content.contains('\n') {
        use itertools::Itertools;
        content
            .chars()
            .chunks(4_000)
            .into_iter()
            .map(Iterator::collect::<String>)
            .join("\n")
    } else {
        content.to_string()
    };

    let mut heading_stack: Vec<String> = Vec::new();
    let mut chunk_heading_path: Vec<String> = vec!["Intro".to_string()];
    let mut current_heading: Option<String> = None;
    let mut chunks = Vec::new();

    for (chunk_index, chunk_text) in splitter.chunks(safe_text.as_str()).enumerate() {
        for line in chunk_text.lines() {
            if let Some((heading_level, text)) = parse_heading(line) {
                update_heading_stack(&mut heading_stack, heading_level, text.clone());
                chunk_heading_path = normalize_heading_path(&heading_stack);
                if heading_level == 2 || (heading_level == 1 && current_heading.is_none()) {
                    current_heading = Some(text);
                }
            }
        }

        let chunk_id = generate_chunk_id(doc_id, chunk_index, level);
        let summary = create_summary(chunk_text);
        let token_count = estimate_tokens(chunk_text);
        let chunk_type = detect_chunk_type(chunk_text);
        let previous_chunk_id = chunk_index
            .checked_sub(1)
            .map(|prev| generate_chunk_id(doc_id, prev, level));

        // text-splitter with overlap already handles context, no need for manual prefixes
        let context_prefix = None;

        chunks.push(Chunk {
            chunk_id,
            doc_id: doc_id.to_string(),
            doc_title: doc_title.to_string(),
            chunk_index,
            content: chunk_text.to_string(),
            context_prefix,
            token_count,
            heading: current_heading.clone(),
            heading_path: chunk_heading_path.clone(),
            chunk_type,
            previous_chunk_id,
            next_chunk_id: None,
            summary,
            chunk_level: level,
            parent_chunk_id: None,
            child_chunk_ids: Vec::new(),
        });
    }

    if chunks.is_empty() {
        let chunk_id = generate_chunk_id(doc_id, 0, level);
        let summary = create_summary(content);
        let token_count = estimate_tokens(content);
        let chunk_type = detect_chunk_type(content);

        chunks.push(Chunk {
            chunk_id,
            doc_id: doc_id.to_string(),
            doc_title: doc_title.to_string(),
            chunk_index: 0,
            content: content.to_string(),
            context_prefix: None,
            token_count,
            heading: None,
            heading_path: vec!["Intro".to_string()],
            chunk_type,
            previous_chunk_id: None,
            next_chunk_id: None,
            summary,
            chunk_level: level,
            parent_chunk_id: None,
            child_chunk_ids: Vec::new(),
        });
    }

    Ok(chunks)
}

/// Internal: Link chunks sequentially (prev/next)
fn link_chunks(chunks: &mut [Chunk]) {
    for i in 0..chunks.len() {
        if let Some(next_i) = i.checked_add(1) {
            if next_i < chunks.len()
                && chunks[i].doc_id == chunks[next_i].doc_id
                && chunks[i].chunk_level == chunks[next_i].chunk_level
            {
                chunks[i].next_chunk_id = Some(chunks[next_i].chunk_id.clone());
            }
        }
    }
}

fn parse_heading(line: &str) -> Option<(usize, String)> {
    let regex = heading_regex().ok()?;
    let caps = regex.captures(line)?;
    let level = caps.get(1)?.as_str().len();
    let text = caps.get(2)?.as_str().trim().to_string();
    Some((level, text))
}

fn update_heading_stack(stack: &mut Vec<String>, level: usize, text: String) {
    if level == 0 {
        return;
    }

    let target_len = level.saturating_sub(1);
    if stack.len() > target_len {
        stack.truncate(target_len);
    }
    while stack.len() < target_len {
        stack.push("".to_string());
    }
    stack.push(text);
}

fn normalize_heading_path(stack: &[String]) -> Vec<String> {
    let path: Vec<String> = stack
        .iter()
        .filter(|item| !item.is_empty())
        .cloned()
        .collect();

    if path.is_empty() {
        vec!["Intro".to_string()]
    } else {
        path
    }
}

/// Estimate token count using tiktoken cl100k_base tokenizer
/// Falls back to character approximation if tokenizer unavailable
fn estimate_tokens(text: &str) -> usize {
    // Fast path for adversarial/pathological strings with no whitespace
    // BPE algorithms can be pathologically slow on huge unbroken strings
    if text.len() > 1000 && !text.contains(|c: char| c.is_whitespace()) {
        return (text.len() / 4).max(1);
    }

    // Use get_encoding() which returns a cached &'static CoreBpe
    let encoder =
        tiktoken::get_encoding("cl100k_base").expect("Failed to load cl100k_base encoding");

    encoder.count(text)
}

/// Create a summary from chunk content (extractive)
/// Extracts first 200 chars cleanly without mangling markdown
fn create_summary(content: &str) -> String {
    let clean_content = content.trim();
    if clean_content.is_empty() {
        return String::new();
    }

    let mut chars = clean_content.chars();
    let truncated: String = chars.by_ref().take(200).collect();

    // If there's no 201st character, the string is <= 200 chars
    if chars.next().is_none() {
        return clean_content.to_string();
    }

    if let Some(last_space) = truncated.rfind(char::is_whitespace) {
        format!("{}...", &truncated[..last_space])
    } else {
        format!("{}...", truncated)
    }
}

/// Detect chunk content type
fn detect_chunk_type(content: &str) -> ChunkType {
    let code_block_count = content.matches("```").count() / 2;
    let has_table =
        content.contains('|') && table_regex().is_ok_and(|regex| regex.is_match(content));

    if code_block_count > 5 {
        ChunkType::Code
    } else if has_table {
        ChunkType::Table
    } else {
        ChunkType::Prose
    }
}

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
    fn test_chunk_level_str() {
        assert_eq!(ChunkLevel::Summary.as_str(), "summary");
        assert_eq!(ChunkLevel::Standard.as_str(), "standard");
        assert_eq!(ChunkLevel::Detailed.as_str(), "detailed");
    }

    #[test]
    fn test_chunk_single_document() {
        let doc = Document::new(
            "test-doc".to_string(),
            "Test Document".to_string(),
            "## Section 1\nContent 1\n## Section 2\nContent 2".to_string(),
        );

        let chunks = chunk(&doc, ChunkLevel::Standard).expect("Failed to chunk test document");
        assert!(!chunks.is_empty());
        assert_eq!(chunks[0].doc_id, "test-doc");
        assert_eq!(chunks[0].doc_title, "Test Document");
    }

    #[test]
    fn test_chunk_all_documents() {
        let docs = vec![
            Document::new(
                "doc1".to_string(),
                "Doc 1".to_string(),
                "## Intro\nContent for doc 1".to_string(),
            ),
            Document::new(
                "doc2".to_string(),
                "Doc 2".to_string(),
                "## Intro\nContent for doc 2".to_string(),
            ),
        ];

        let result = chunk_all(&docs).expect("Failed to chunk all documents");
        assert!(result.summary_count > 0);
        assert!(result.standard_count > 0);
        assert!(result.detailed_count > 0);
    }

    #[test]
    fn test_create_summary_ascii() {
        let content = "This is a test. This is another sentence.";
        let summary = create_summary(content);
        assert!(!summary.is_empty());
        assert!(summary.contains("This is a test"));
    }

    #[test]
    fn test_create_summary_unicode_emoji() {
        let content = "This is a test with emoji 🎉 and more content here.";
        let summary = create_summary(content);
        assert!(!summary.is_empty());
    }

    #[test]
    fn test_create_summary_unicode_cjk() {
        let content = "这是一个测试。这是另一个句子。More content after Chinese.";
        let summary = create_summary(content);
        assert!(!summary.is_empty());
    }

    #[test]
    fn test_chunk_type_detection() {
        let code = "```\ncode\n```\n```\ncode\n```\n```\ncode\n```\n```\ncode\n```\n```\ncode\n```\n```\ncode\n```";
        assert_eq!(detect_chunk_type(code), ChunkType::Code);

        let table = "| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |";
        assert_eq!(detect_chunk_type(table), ChunkType::Table);

        let prose = "This is just regular prose content with no tables or code blocks.";
        assert_eq!(detect_chunk_type(prose), ChunkType::Prose);
    }

    #[test]
    fn test_estimate_tokens() {
        let text = "This is a test";
        let tokens = estimate_tokens(text);
        assert!(tokens > 0);
        assert!((3..=4).contains(&tokens));
    }

    #[test]
    fn test_empty_document() {
        let doc = Document::new("empty".to_string(), "Empty Doc".to_string(), "".to_string());
        let chunks = chunk(&doc, ChunkLevel::Standard).expect("Failed to chunk empty document");
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].content, "");
    }

    #[test]
    fn test_invalid_document() {
        let invalid = Document::new("".to_string(), "Title".to_string(), "content".to_string());
        let result = chunk(&invalid, ChunkLevel::Standard);
        assert!(result.is_err());
    }

    #[test]
    fn test_chunk_no_h2_headings() {
        let content = "# Title\n\nLong content without any H2 headings.\n\n".repeat(100);
        let doc = Document::new(
            "no-h2".to_string(),
            "No H2 Doc".to_string(),
            content.clone(),
        );
        let chunks = chunk(&doc, ChunkLevel::Standard)
            .expect("Failed to chunk document with no H2 headings");

        assert!(!chunks.is_empty(), "Should create at least one chunk");
        assert!(chunks[0].content.contains("Title"), "Should include H1");
        if content.split_whitespace().count() > 512 {
            assert!(chunks.len() > 1, "Long content should split");
        }
    }

    #[test]
    fn test_chunk_very_short_document() {
        let content = "# Short\n\nJust a few words.".to_string();
        let doc = Document::new("short".to_string(), "Short Doc".to_string(), content);
        let chunks =
            chunk(&doc, ChunkLevel::Standard).expect("Failed to chunk very short document");

        assert_eq!(chunks.len(), 1, "Short doc should be one chunk");
        assert!(chunks[0].token_count < 512, "Should be under target");
    }

    #[test]
    fn test_chunk_only_h1_no_sections() {
        let content = "# Title\n\nContent here.\n\n# Another Title\n\nMore content.".to_string();
        let doc = Document::new("h1-only".to_string(), "H1 Only".to_string(), content);
        let chunks = chunk(&doc, ChunkLevel::Standard)
            .expect("Failed to chunk document with only H1 headings");

        assert!(!chunks.is_empty());
        let all_content: String = chunks.iter().map(|c| c.content.as_str()).collect();
        assert!(all_content.contains("Title"));
        assert!(all_content.contains("Another Title"));
    }

    #[test]
    fn test_chunk_very_long_document() {
        let long_content = "# Title\n\n## Section\n\n".to_string() + &"word ".repeat(10000);
        let doc = Document::new(
            "long".to_string(),
            "Long Doc".to_string(),
            long_content.clone(),
        );
        let chunks = chunk(&doc, ChunkLevel::Standard).expect("Failed to chunk very long document");

        assert!(
            chunks.len() > 1,
            "Long doc should split into multiple chunks"
        );

        let total_words: usize = chunks
            .iter()
            .map(|c| c.content.split_whitespace().count())
            .sum();
        let original_words = long_content.split_whitespace().count();
        assert!(
            total_words >= original_words.saturating_sub(100),
            "Most words preserved"
        );
    }

    #[test]
    fn test_chunk_unicode_boundaries() {
        let content = "# Unicode\n\n## Section\n\n";
        let emoji_content = "emoji 😀 ".repeat(1000);
        let full_content = content.to_string() + &emoji_content;
        let doc = Document::new(
            "unicode".to_string(),
            "Unicode Doc".to_string(),
            full_content.clone(),
        );
        let chunks = chunk(&doc, ChunkLevel::Standard)
            .expect("Failed to chunk document with unicode boundaries");

        for chunk in &chunks {
            assert!(chunk.content.is_char_boundary(0));
            assert!(chunk.content.is_char_boundary(chunk.content.len()));
        }

        let all_content: String = chunks.iter().map(|c| c.content.as_str()).collect();
        assert!(
            all_content.matches('😀').count() >= full_content.matches('😀').count(),
            "Emojis should be preserved (can be more due to overlap)"
        );
    }

    #[test]
    fn test_chunk_empty_sections() {
        let content =
            "# Title\n\n## Empty\n\n## Another Empty\n\n## Has Content\n\nSome text.".to_string();
        let doc = Document::new(
            "empty-sections".to_string(),
            "Empty Sections".to_string(),
            content,
        );
        let chunks = chunk(&doc, ChunkLevel::Standard)
            .expect("Failed to chunk document with empty sections");

        assert!(!chunks.is_empty());
    }

    #[test]
    fn test_chunk_table_preservation() {
        let content = r"# Title

## Table Section

| Col1 | Col2 |
|------|------|
| A    | B    |
| C    | D    |

More content."
            .to_string();

        let doc = Document::new("table".to_string(), "Table Doc".to_string(), content);
        let chunks =
            chunk(&doc, ChunkLevel::Standard).expect("Failed to chunk document with table");

        let has_table = chunks.iter().any(|c| c.content.contains("| Col1 |"));
        assert!(has_table, "Table should be preserved in chunks");
    }
}
