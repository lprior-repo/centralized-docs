//! Public chunking API — `chunk()`, `chunk_all()`, and orchestration.
//!
//! Documents are chunked in parallel via `rayon`, then aggregated
//! with zero-mutation functional passes.

use anyhow::Result;
use rayon::prelude::*;

use crate::chunk::{Chunk, ChunkLevel, ChunkingResult};
use crate::document::Document;
use crate::hierarchy::{assign_hierarchy, link_chunks};
use crate::split::create_chunks_at_level;

/// Chunk a single document at a specific hierarchical level.
///
/// # Arguments
///
/// * `document` - The document to chunk
/// * `level` - The hierarchical level (Summary/Standard/Detailed)
///
/// # Returns
///
/// A vector of chunks, one per semantic boundary, with sequential
/// prev/next links.
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

/// Per-document chunking result — produced in parallel, then folded.
struct DocChunks {
    summary: Vec<Chunk>,
    standard: Vec<Chunk>,
    detailed: Vec<Chunk>,
}

/// Chunk a single document at all three hierarchical levels.
fn chunk_document(doc: &Document) -> Result<DocChunks> {
    let mut summary =
        create_chunks_at_level(&doc.id, &doc.title, &doc.content, ChunkLevel::Summary)?;
    let mut standard =
        create_chunks_at_level(&doc.id, &doc.title, &doc.content, ChunkLevel::Standard)?;
    let mut detailed =
        create_chunks_at_level(&doc.id, &doc.title, &doc.content, ChunkLevel::Detailed)?;

    assign_hierarchy(&mut summary, &mut standard, &mut detailed);

    Ok(DocChunks {
        summary,
        standard,
        detailed,
    })
}

/// Chunk all documents at all three hierarchical levels in parallel.
///
/// Documents are independent — no shared mutable state — making this
/// embarrassingly parallel. `rayon::par_iter` distributes work across
/// all available cores with zero coordination overhead.
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
    // Validate all documents first (fail-fast, sequential).
    documents
        .iter()
        .find(|doc| !doc.is_valid())
        .map_or(Ok(()), |doc| {
            anyhow::bail!(
                "Invalid document: {} - id and title must be non-empty",
                doc.id
            )
        })?;

    // Parallel chunking: each document is independent.
    let doc_results: Vec<DocChunks> = documents
        .par_iter()
        .map(chunk_document)
        .collect::<Result<Vec<_>>>()?;

    // Functional aggregation — three separate passes, each O(n) with zero mutation.
    let summary_count: usize = doc_results.iter().map(|dc| dc.summary.len()).sum();
    let standard_count: usize = doc_results.iter().map(|dc| dc.standard.len()).sum();
    let detailed_count: usize = doc_results.iter().map(|dc| dc.detailed.len()).sum();

    let all_chunks: Vec<Chunk> = doc_results
        .into_iter()
        .flat_map(|dc| dc.summary.into_iter().chain(dc.standard).chain(dc.detailed))
        .collect();

    Ok(ChunkingResult {
        chunks: all_chunks,
        summary_count,
        standard_count,
        detailed_count,
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    #![allow(clippy::expect_used)]
    use super::*;

    #[test]
    fn test_chunk_single_document() {
        let doc = Document::new(
            "test-doc".to_string(),
            "Test Document".to_string(),
            "## Section 1\nContent 1\n## Section 2\nContent 2".to_string(),
        );
        let chunks = chunk(&doc, ChunkLevel::Standard).expect("Failed to chunk");
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
                "## Intro\nContent 1".to_string(),
            ),
            Document::new(
                "doc2".to_string(),
                "Doc 2".to_string(),
                "## Intro\nContent 2".to_string(),
            ),
        ];
        let result = chunk_all(&docs).expect("Failed to chunk all");
        assert!(result.summary_count > 0);
        assert!(result.standard_count > 0);
        assert!(result.detailed_count > 0);
    }

    #[test]
    fn test_empty_document() {
        let doc = Document::new("empty".to_string(), "Empty Doc".to_string(), "".to_string());
        let chunks = chunk(&doc, ChunkLevel::Standard).expect("Failed to chunk empty");
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].content, "");
    }

    #[test]
    fn test_invalid_document() {
        let invalid = Document::new("".to_string(), "Title".to_string(), "content".to_string());
        assert!(chunk(&invalid, ChunkLevel::Standard).is_err());
    }

    #[test]
    fn test_chunk_no_h2_headings() {
        let content = "# Title\n\nLong content without any H2 headings.\n\n".repeat(100);
        let doc = Document::new("no-h2".to_string(), "No H2".to_string(), content.clone());
        let chunks = chunk(&doc, ChunkLevel::Standard).expect("Failed to chunk");
        assert!(!chunks.is_empty());
        assert!(chunks[0].content.contains("Title"));
        if content.split_whitespace().count() > 512 {
            assert!(chunks.len() > 1);
        }
    }

    #[test]
    fn test_chunk_very_short_document() {
        let doc = Document::new(
            "short".to_string(),
            "Short".to_string(),
            "# Short\n\nJust a few words.".to_string(),
        );
        let chunks = chunk(&doc, ChunkLevel::Standard).expect("Failed to chunk");
        assert_eq!(chunks.len(), 1);
        assert!(chunks[0].token_count < 512);
    }

    #[test]
    fn test_chunk_only_h1_no_sections() {
        let doc = Document::new(
            "h1".to_string(),
            "H1".to_string(),
            "# Title\n\nContent.\n\n# Another\n\nMore.".to_string(),
        );
        let chunks = chunk(&doc, ChunkLevel::Standard).expect("Failed to chunk");
        assert!(!chunks.is_empty());
        let all: String = chunks.iter().map(|c| c.content.as_str()).collect();
        assert!(all.contains("Title") && all.contains("Another"));
    }

    #[test]
    fn test_chunk_very_long_document() {
        let long = "# Title\n\n## Section\n\n".to_string() + &"word ".repeat(10_000);
        let doc = Document::new("long".to_string(), "Long".to_string(), long.clone());
        let chunks = chunk(&doc, ChunkLevel::Standard).expect("Failed to chunk");
        assert!(chunks.len() > 1);
        let total: usize = chunks
            .iter()
            .map(|c| c.content.split_whitespace().count())
            .sum();
        assert!(total >= long.split_whitespace().count().saturating_sub(100));
    }

    #[test]
    fn test_chunk_unicode_boundaries() {
        let content = "# U\n\n## S\n\n".to_string() + &"😀 ".repeat(1000);
        let doc = Document::new("uni".to_string(), "U".to_string(), content.clone());
        let chunks = chunk(&doc, ChunkLevel::Standard).expect("Failed to chunk");
        for c in &chunks {
            assert!(c.content.is_char_boundary(0));
            assert!(c.content.is_char_boundary(c.content.len()));
        }
        let all: String = chunks.iter().map(|c| c.content.as_str()).collect();
        assert!(all.matches('😀').count() >= content.matches('😀').count());
    }

    #[test]
    fn test_chunk_empty_sections() {
        let doc = Document::new(
            "es".to_string(),
            "ES".to_string(),
            "# T\n\n## E\n\n## E2\n\n## C\n\nText.".to_string(),
        );
        assert!(!chunk(&doc, ChunkLevel::Standard)
            .expect("Failed")
            .is_empty());
    }

    #[test]
    fn test_chunk_table_preservation() {
        let content = "# T\n\n## TS\n\n| A | B |\n|---|---|\n| 1 | 2 |\n\nMore.".to_string();
        let doc = Document::new("tbl".to_string(), "T".to_string(), content);
        let chunks = chunk(&doc, ChunkLevel::Standard).expect("Failed to chunk");
        assert!(chunks.iter().any(|c| c.content.contains("| A |")));
    }
}
