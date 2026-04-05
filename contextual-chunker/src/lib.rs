#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::manual_string_new)]
#![forbid(unsafe_code)]

//! # contextual-chunker
//!
//! Semantic chunking with hierarchical levels for documentation and knowledge bases.
//!
//! This library enables splitting documents into semantically meaningful chunks
//! at multiple hierarchical levels (Summary, Standard, Detailed) with automatic
//! relationship tracking, making it ideal for RAG systems and knowledge bases.
//!
//! ## Quick Start
//!
//! ```
//! use contextual_chunker::{Document, ChunkLevel, chunk_all};
//!
//! let documents = vec![
//!     Document::new(
//!         "guide".to_string(),
//!         "Getting Started".to_string(),
//!         "## Introduction\nWelcome to the guide.\n## Next Steps\nHere's what to do.".to_string(),
//!     ),
//! ];
//!
//! let result = chunk_all(&documents)?;
//! println!("Created {} chunks", result.chunks.len());
//! # Ok::<(), anyhow::Error>(())
//! ```
//!
//! ## License
//!
//! MIT - See LICENSE file in repository

pub mod chunk;
pub(crate) mod chunker;
pub mod document;
pub(crate) mod hierarchy;
pub(crate) mod split;
pub(crate) mod token;

// Re-export public API
pub use chunk::{Chunk, ChunkLevel, ChunkType, ChunkingResult};
pub use chunker::{chunk, chunk_all};
pub use document::Document;

/// Trait for document chunking strategies.
///
/// Kept because downstream tests exercise `dyn Chunker` trait objects (C7).
///
/// # Example
///
/// ```
/// use contextual_chunker::{Chunker, Document, Chunk};
///
/// struct CustomChunker;
///
/// impl Chunker for CustomChunker {
///     fn chunk(&self, doc: &Document) -> anyhow::Result<Vec<Chunk>> {
///         Ok(Vec::new())
///     }
/// }
/// ```
pub trait Chunker {
    /// Chunk a document according to implementation's strategy.
    fn chunk(&self, doc: &Document) -> Result<Vec<Chunk>, anyhow::Error>;
}

/// Contextual chunking with configurable parameters.
///
/// Provides semantic chunking at three hierarchical levels (Summary, Standard, Detailed).
///
/// # Example
///
/// ```
/// use contextual_chunker::{Chunker, ContextualChunker, Document, ChunkLevel};
///
/// let chunker = ContextualChunker::standard();
/// let doc = Document::new(
///     "guide".to_string(),
///     "Guide".to_string(),
///     "## Intro\nContent".to_string(),
/// );
///
/// let chunks = chunker.chunk(&doc).unwrap();
/// ```
pub struct ContextualChunker {
    pub level: ChunkLevel,
    pub context_tokens: usize,
}

impl ContextualChunker {
    /// Create a new ContextualChunker with custom parameters.
    pub fn new(level: ChunkLevel, context_tokens: usize) -> Self {
        Self {
            level,
            context_tokens,
        }
    }

    /// Create a chunker for Summary level (~128 tokens, 30 context tokens).
    ///
    /// Uses [`ChunkLevel::overlap_tokens`] as single source of truth (C8).
    #[must_use]
    pub fn summary() -> Self {
        Self::new(ChunkLevel::Summary, ChunkLevel::Summary.overlap_tokens())
    }

    /// Create a chunker for Standard level (~512 tokens, 100 context tokens).
    #[must_use]
    pub fn standard() -> Self {
        Self::new(ChunkLevel::Standard, ChunkLevel::Standard.overlap_tokens())
    }

    /// Create a chunker for Detailed level (~1024 tokens, 200 context tokens).
    #[must_use]
    pub fn detailed() -> Self {
        Self::new(ChunkLevel::Detailed, ChunkLevel::Detailed.overlap_tokens())
    }
}

impl Chunker for ContextualChunker {
    fn chunk(&self, doc: &Document) -> Result<Vec<Chunk>, anyhow::Error> {
        crate::chunker::chunk(doc, self.level)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    #![allow(clippy::expect_used)]
    #![allow(clippy::assertions_on_constants)]
    #![allow(clippy::semicolon_if_nothing_returned)]
    use super::*;

    #[test]
    fn test_chunker_trait_exists() {
        let chunker = ContextualChunker::standard();
        let doc = Document::new(
            "test".to_string(),
            "Test".to_string(),
            "## Section\nContent".to_string(),
        );

        let chunks = chunker.chunk(&doc);
        assert!(chunks.is_ok());
        let chunks = chunks.unwrap();
        assert!(!chunks.is_empty());
    }

    #[test]
    fn test_contextual_chunker_factory_summary() {
        let chunker = ContextualChunker::summary();
        assert_eq!(chunker.level, ChunkLevel::Summary);
        assert_eq!(chunker.context_tokens, 30);
    }

    #[test]
    fn test_contextual_chunker_factory_standard() {
        let chunker = ContextualChunker::standard();
        assert_eq!(chunker.level, ChunkLevel::Standard);
        assert_eq!(chunker.context_tokens, 100);
    }

    #[test]
    fn test_contextual_chunker_factory_detailed() {
        let chunker = ContextualChunker::detailed();
        assert_eq!(chunker.level, ChunkLevel::Detailed);
        assert_eq!(chunker.context_tokens, 200);
    }

    #[test]
    fn test_contextual_chunker_custom_config() {
        let chunker = ContextualChunker::new(ChunkLevel::Standard, 150);
        assert_eq!(chunker.level, ChunkLevel::Standard);
        assert_eq!(chunker.context_tokens, 150);
    }

    #[test]
    fn test_chunker_produces_correct_level() {
        let doc = Document::new(
            "test".to_string(),
            "Test".to_string(),
            "## Section 1\nContent 1\n## Section 2\nContent 2".to_string(),
        );

        let summary_chunker = ContextualChunker::summary();
        let summary_chunks = summary_chunker
            .chunk(&doc)
            .expect("Failed to chunk summary");
        summary_chunks
            .iter()
            .for_each(|c| assert_eq!(c.chunk_level, ChunkLevel::Summary));

        let standard_chunker = ContextualChunker::standard();
        let standard_chunks = standard_chunker
            .chunk(&doc)
            .expect("Failed to chunk standard");
        standard_chunks
            .iter()
            .for_each(|c| assert_eq!(c.chunk_level, ChunkLevel::Standard));

        let detailed_chunker = ContextualChunker::detailed();
        let detailed_chunks = detailed_chunker
            .chunk(&doc)
            .expect("Failed to chunk detailed");
        detailed_chunks
            .iter()
            .for_each(|c| assert_eq!(c.chunk_level, ChunkLevel::Detailed));
    }

    #[test]
    fn test_backward_compatibility_free_functions() {
        let doc = Document::new(
            "test".to_string(),
            "Test".to_string(),
            "## Section\nContent".to_string(),
        );

        let chunks = chunk(&doc, ChunkLevel::Standard).expect("Failed to chunk");
        assert!(!chunks.is_empty());
    }

    #[test]
    fn test_chunker_trait_object() {
        let chunker: Box<dyn Chunker> = Box::new(ContextualChunker::standard());
        let doc = Document::new(
            "test".to_string(),
            "Test".to_string(),
            "## Section\nContent".to_string(),
        );

        let chunks = chunker.chunk(&doc);
        assert!(chunks.is_ok());
        let chunks = chunks.unwrap();
        assert!(!chunks.is_empty());
    }
}
