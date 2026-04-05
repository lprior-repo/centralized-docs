//! Internal text splitting and chunk construction logic.
//!
//! Handles the low-level details of splitting markdown with text-splitter,
//! building heading state, and constructing `Chunk` structs.

#![allow(clippy::too_many_arguments)]

use anyhow::Result;
use itertools::Itertools;
use text_splitter::{ChunkConfig, MarkdownSplitter};

use crate::chunk::{generate_chunk_id, Chunk, ChunkLevel};
use crate::hierarchy::{normalize_heading_path, parse_heading, update_heading_stack};
use crate::token::{create_summary, detect_chunk_type, estimate_tokens, FastTokenizer};

// ---------------------------------------------------------------------------
// Heading state — threaded through the splitter via `scan`
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct HeadingState {
    stack: Vec<String>,
    heading_path: Vec<String>,
    current_heading: Option<String>,
}

impl HeadingState {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            heading_path: vec!["Intro".to_string()],
            current_heading: None,
        }
    }

    /// Apply a single line, returning a new state if the line is a heading.
    fn apply_line(self, line: &str) -> Self {
        match parse_heading(line) {
            Some((level, text)) => {
                let mut stack = self.stack;
                update_heading_stack(&mut stack, level, text.clone());
                let heading_path = normalize_heading_path(&stack);
                let current_heading =
                    if level == 2 || (level == 1 && self.current_heading.is_none()) {
                        Some(text)
                    } else {
                        self.current_heading
                    };
                Self {
                    stack,
                    heading_path,
                    current_heading,
                }
            }
            None => self,
        }
    }
}

// ---------------------------------------------------------------------------
// Text pre-processing
// ---------------------------------------------------------------------------

/// Pre-process absurdly long strings without line breaks to avoid
/// stack overflow in tiktoken-rs PCRE regex parsing.
fn safe_text_for_splitter(content: &str) -> String {
    if content.len() > 50_000 && !content.contains('\n') {
        content
            .chars()
            .chunks(4_000)
            .into_iter()
            .map(Iterator::collect::<String>)
            .join("\n")
    } else {
        content.to_string()
    }
}

// ---------------------------------------------------------------------------
// Chunk construction
// ---------------------------------------------------------------------------

/// Build a `Chunk` from splitter output and current heading state.
fn build_chunk_struct(
    doc_id: &str,
    doc_title: &str,
    chunk_index: usize,
    level: ChunkLevel,
    chunk_text: &str,
    current_heading: Option<&String>,
    heading_path: &[String],
) -> Chunk {
    Chunk {
        chunk_id: generate_chunk_id(doc_id, chunk_index, level),
        doc_id: doc_id.to_string(),
        doc_title: doc_title.to_string(),
        chunk_index,
        content: chunk_text.to_string(),
        // context_prefix: always None — overlap is handled by text-splitter.
        // Kept for downstream API compatibility (centralized-docs crate).
        context_prefix: None,
        token_count: estimate_tokens(chunk_text),
        heading: current_heading.cloned(),
        heading_path: heading_path.to_vec(),
        chunk_type: detect_chunk_type(chunk_text),
        previous_chunk_id: chunk_index
            .checked_sub(1)
            .map(|prev| generate_chunk_id(doc_id, prev, level)),
        next_chunk_id: None,
        summary: create_summary(chunk_text),
        chunk_level: level,
        parent_chunk_id: None,
        child_chunk_ids: Vec::new(),
    }
}

/// Build a fallback chunk when the splitter produces no output (empty document).
fn build_empty_chunk(doc_id: &str, doc_title: &str, level: ChunkLevel, content: &str) -> Chunk {
    Chunk {
        chunk_id: generate_chunk_id(doc_id, 0, level),
        doc_id: doc_id.to_string(),
        doc_title: doc_title.to_string(),
        chunk_index: 0,
        content: content.to_string(),
        context_prefix: None,
        token_count: estimate_tokens(content),
        heading: None,
        heading_path: vec!["Intro".to_string()],
        chunk_type: detect_chunk_type(content),
        previous_chunk_id: None,
        next_chunk_id: None,
        summary: create_summary(content),
        chunk_level: level,
        parent_chunk_id: None,
        child_chunk_ids: Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// Core splitting
// ---------------------------------------------------------------------------

/// Create chunks at a specific hierarchical level.
///
/// Uses `ChunkLevel::overlap_tokens()` as the single source of truth for
/// overlap values, eliminating duplication with `ContextualChunker`.
pub(crate) fn create_chunks_at_level(
    doc_id: &str,
    doc_title: &str,
    content: &str,
    level: ChunkLevel,
) -> Result<Vec<Chunk>> {
    let tokenizer = FastTokenizer::new().map_err(|e| anyhow::anyhow!("Tokenizer error: {e}"))?;

    let config = ChunkConfig::new(level.target_tokens())
        .with_sizer(tokenizer)
        .with_overlap(level.overlap_tokens())
        .map_err(|e| anyhow::anyhow!("ChunkConfig error: {e}"))?;

    let splitter = MarkdownSplitter::new(config);
    let safe_text = safe_text_for_splitter(content);

    let chunks: Vec<Chunk> = splitter
        .chunks(safe_text.as_str())
        .enumerate()
        .scan(HeadingState::new(), |state, (chunk_index, chunk_text)| {
            *state = chunk_text
                .lines()
                .fold(state.clone(), HeadingState::apply_line);
            Some((chunk_index, chunk_text, state.clone()))
        })
        .map(|(chunk_index, chunk_text, state)| {
            build_chunk_struct(
                doc_id,
                doc_title,
                chunk_index,
                level,
                chunk_text,
                state.current_heading.as_ref(),
                &state.heading_path,
            )
        })
        .collect();

    if chunks.is_empty() {
        return Ok(vec![build_empty_chunk(doc_id, doc_title, level, content)]);
    }

    Ok(chunks)
}
