//! Test modules for index.

use std::collections::HashMap;

use super::*;
use crate::analyze::{Analysis, Heading};
use crate::assign::IdMapping;
use crate::chunking_adapter::{Chunk, ChunksResult};
use contextual_chunker::ChunkLevel;
use std::sync::Arc;

pub(crate) fn make_heading(level: u32, text: &str) -> Heading {
    Heading {
        level,
        text: text.to_string(),
        line: 0,
    }
}

pub(crate) fn make_analysis(
    source_path: &str,
    title: &str,
    category: &str,
    headings: Vec<Heading>,
    first_paragraph: &str,
    word_count: usize,
) -> Analysis {
    Analysis {
        source_path: source_path.to_string(),
        title: title.to_string(),
        frontmatter: None,
        headings,
        links: vec![],
        first_paragraph: first_paragraph.to_string(),
        word_count,
        has_code: false,
        has_tables: false,
        category: category.to_string(),
        content: Arc::from(format!(
            "{first_paragraph} Additional content for testing purposes."
        )),
    }
}

pub(crate) fn make_chunk(
    chunk_id: &str,
    doc_id: &str,
    doc_title: &str,
    content: &str,
    heading: Option<&str>,
    level: ChunkLevel,
) -> Chunk {
    Chunk {
        chunk_id: chunk_id.to_string(),
        doc_id: doc_id.to_string(),
        doc_title: doc_title.to_string(),
        chunk_index: 0,
        content: content.to_string(),
        token_count: content.split_whitespace().count(),
        heading: heading.map(String::from),
        heading_path: vec![],
        chunk_type: contextual_chunker::ChunkType::Prose,
        previous_chunk_id: None,
        next_chunk_id: None,
        related_chunk_ids: vec![],
        summary: content.to_string(),
        chunk_level: level,
        parent_chunk_id: None,
        child_chunk_ids: vec![],
        context_prefix: None,
    }
}

pub(crate) fn make_link_map(mappings: Vec<(&str, &str, &str, &str)>) -> HashMap<String, IdMapping> {
    mappings
        .into_iter()
        .map(|(source_path, id, filename, subcategory)| {
            (
                source_path.to_string(),
                IdMapping {
                    id: id.to_string(),
                    filename: filename.to_string(),
                    subcategory: subcategory.to_string(),
                    slug: filename.to_string(),
                },
            )
        })
        .collect()
}

pub(crate) fn make_empty_chunks_result() -> ChunksResult {
    ChunksResult {
        total_chunks: 0,
        document_count: 0,
        chunks_metadata: vec![],
        summary_chunks: 0,
        standard_chunks: 0,
        detailed_chunks: 0,
    }
}

mod assembly_nav;
mod build_chunk_metadata;
mod build_document_index;
mod dag_analytics;
mod serialization_pipeline;
