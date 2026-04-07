//! Uncached chunking entry point.

use super::types::{
    analysis_to_document, convert_chunking_result, create_dir_with_context, escape_frontmatter,
    ChunksResult,
};
use crate::analyze::Analysis;
use crate::assign::IdMapping;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Chunk all analyses using `contextual-chunker`
///
/// This is the main entry point for chunking in `ctd`.
/// It converts Analysis types to Documents, calls `contextual-chunker`,
/// converts the results back to `ctd` types, and writes
/// chunk files to disk.
///
/// # Errors
///
/// Returns an error if:
/// - The chunks directory cannot be created
/// - Chunking fails in contextual-chunker
/// - Writing chunk files fails
/// - A document exceeds `max_document_bytes`
#[allow(clippy::implicit_hasher)]
pub fn chunk_all(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping>,
    output_dir: &Path,
    max_document_bytes: u64,
) -> Result<ChunksResult> {
    // Create chunks directory
    let chunks_dir = output_dir.join("chunks");
    create_dir_with_context(&chunks_dir, "chunks")?;

    // Calculate warning threshold (50% of max)
    let warning_threshold = max_document_bytes / 2;

    // Check document sizes and warn/fail for oversized content
    analyses.iter().try_for_each(|analysis| {
        let content_size = analysis.content.len() as u64;
        if content_size > max_document_bytes {
            anyhow::bail!(
                "Document '{}' ({} bytes) exceeds maximum document size limit ({} bytes). \
                 Please split the document or increase --max-document-bytes.",
                analysis.source_path,
                content_size,
                max_document_bytes
            );
        }
        if content_size > warning_threshold {
            eprintln!(
                "Warning: Large document '{}' ({} bytes) may take significant time to chunk. \
                 Consider splitting documents larger than {} bytes for better performance.",
                analysis.source_path, content_size, warning_threshold
            );
        }
        Ok(())
    })?;

    // Convert analyses to documents
    let documents: Vec<_> = analyses
        .iter()
        .map(|a| analysis_to_document(a, link_map))
        .collect();

    // Call contextual-chunker
    let result = contextual_chunker::chunk_all(&documents)?;

    // Convert result back to ctd types
    let chunks_result = convert_chunking_result(result, analyses.len());

    // Write chunks to disk
    chunks_result.chunks_metadata.iter().try_for_each(|chunk| {
        let level_suffix = match chunk.chunk_level {
            contextual_chunker::ChunkLevel::Summary => "summary",
            contextual_chunker::ChunkLevel::Standard => "standard",
            contextual_chunker::ChunkLevel::Detailed => "detailed",
        };

        let chunk_filename = format!(
            "{}-{}.md",
            chunk.chunk_id.replace(['/', '#'], "-"),
            level_suffix
        );
        let chunk_file = chunks_dir.join(&chunk_filename);

        let frontmatter = format!(
            "---\ndoc_id: {}\nchunk_id: {}\nchunk_level: {}\nchunk_type: {}\nheading: {}\ntoken_count: {}\nsummary: {}\n---\n",
            chunk.doc_id,
            chunk.chunk_id,
            level_suffix,
            chunk.chunk_type,
            chunk.heading.as_ref().map_or(&"Introduction".to_string(), |v| v),
            chunk.token_count,
            escape_frontmatter(&chunk.summary)
        );

        let content = format!("{}\n{}", frontmatter, chunk.content);
        fs::write(chunk_file, content)
    })?;

    Ok(chunks_result)
}
