//! Test modules for chunking_adapter.

use std::collections::HashMap;

use super::*;
use crate::analyze::Analysis;
use crate::assign::IdMapping;

/// Helper: create a minimal Analysis for testing.
pub(crate) fn make_analysis(
    source_path: &str,
    title: &str,
    content: &str,
    category: &str,
) -> Analysis {
    Analysis {
        source_path: source_path.to_string(),
        title: title.to_string(),
        content: content.into(),
        frontmatter: None,
        headings: vec![],
        links: vec![],
        first_paragraph: content.to_string(),
        word_count: content.split_whitespace().count(),
        has_code: false,
        has_tables: false,
        category: category.to_string(),
    }
}

/// Helper: create a test `link_map` entry.
pub(crate) fn make_link_map(analyses: &[Analysis]) -> HashMap<String, IdMapping> {
    analyses
        .iter()
        .map(|a| {
            let slug = a.source_path.split('/').next_back().map_or_else(
                || "untitled".to_string(),
                |s| s.trim_end_matches(".md").to_string(),
            );
            (
                a.source_path.clone(),
                IdMapping {
                    id: format!("{}/{}", a.category, slug),
                    filename: format!("{}-{}.md", a.category, slug),
                    subcategory: "general".to_string(),
                    slug,
                },
            )
        })
        .collect()
}

/// Helper: create a test Chunk.
pub(crate) fn make_test_chunk(
    chunk_id: &str,
    doc_id: &str,
    chunk_index: usize,
    chunk_level: contextual_chunker::ChunkLevel,
    content: &str,
    heading: Option<&str>,
) -> Chunk {
    Chunk {
        chunk_id: chunk_id.to_string(),
        doc_id: doc_id.to_string(),
        doc_title: "Test Doc".to_string(),
        chunk_index,
        content: content.to_string(),
        token_count: content.split_whitespace().count(),
        heading: heading.map(str::to_string),
        heading_path: vec![],
        chunk_type: contextual_chunker::ChunkType::Prose,
        previous_chunk_id: None,
        next_chunk_id: None,
        related_chunk_ids: vec![],
        summary: format!("Summary for {chunk_id}"),
        chunk_level,
        parent_chunk_id: None,
        child_chunk_ids: vec![],
        context_prefix: None,
    }
}

mod cache_partition;
mod cached_boundaries;
mod cached_equiv;
mod cached_order_count;
mod cached_skip_store;
mod conversion;
mod proptests;
mod write_tests;
