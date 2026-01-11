//! # contextual-chunker
//!
//! Semantic chunking with hierarchical levels for documentation and knowledge bases.
//!
//! This library enables splitting documents into semantically meaningful chunks
//! at multiple hierarchical levels (Summary, Standard, Detailed) with automatic
//! relationship tracking, making it ideal for RAG systems and knowledge bases.
//!
//! ## Key Features
//!
//! - **Semantic Boundaries**: Chunks respect H2 headings (##) in markdown
//! - **Hierarchical Levels**: 3-level hierarchy (128, 512, 1024 tokens)
//! - **Automatic Relationships**: Parent-child links for progressive disclosure
//! - **Navigation Links**: Sequential prev/next pointers at same level
//! - **Content Analysis**: Automatic type detection (code/table/prose)
//! - **Summary Extraction**: Extractive summaries for quick overview
//! - **Deterministic**: Same input always produces same chunks
//! - **Unicode Safe**: No panics on emoji, CJK, or special characters
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
//! ## Chunking Strategy
//!
//! Documents are chunked at three levels simultaneously:
//!
//! 1. **Summary Level** (~128 tokens)
//!    - High-level overview for quick retrieval
//!    - Parent chunks for Standard level
//!
//! 2. **Standard Level** (~512 tokens)
//!    - Balanced detail for most use cases
//!    - Default retrieval level
//!    - Child of Summary, parent of Detailed
//!
//! 3. **Detailed Level** (~1024 tokens)
//!    - Full context for deep understanding
//!    - Leaf chunks in hierarchy
//!
//! ## Chunk Boundaries
//!
//! Chunks respect markdown structure:
//! - H2 headings (##) trigger chunk boundaries
//! - If a section exceeds token limit, split by token count
//! - Previous section's tail included as context
//!
//! ## Example: Multi-Level Retrieval
//!
//! ```
//! use contextual_chunker::{Document, ChunkLevel, chunk_all};
//!
//! let doc = Document::new(
//!     "tutorial".to_string(),
//!     "Tutorial".to_string(),
//!     "## Setup\nInstructions.\n## Testing\nTest cases.".to_string(),
//! );
//!
//! let result = chunk_all(&[doc])?;
//!
//! // Summary chunks: quick lookup
//! let summary_chunks: Vec<_> = result
//!     .chunks
//!     .iter()
//!     .filter(|c| c.chunk_level == ChunkLevel::Summary)
//!     .collect();
//!
//! // Standard chunks: balanced search results
//! let standard_chunks: Vec<_> = result
//!     .chunks
//!     .iter()
//!     .filter(|c| c.chunk_level == ChunkLevel::Standard)
//!     .collect();
//!
//! // Navigate: summary.child_chunk_ids -> standard chunk IDs
//! # Ok::<(), anyhow::Error>(())
//! ```
//!
//! ## Design Principles
//!
//! **Deterministic**: Same input → same chunks (no randomness)
//!
//! **Type-Safe**: Invalid documents rejected at validation, not runtime
//!
//! **Immutable**: Chunks are frozen after creation (no mutations)
//!
//! **Zero-Panic**: All Unicode handled safely; hardcoded regex patterns verified
//!
//! **Minimal Dependencies**: Only standard Rust ecosystem (regex, serde, anyhow)
//!
//! ## Safety & Stability
//!
//! - **Unicode Handling**: Safe on emoji, multibyte, CJK characters
//! - **Token Estimation**: Consistent within ±10% (4 chars ≈ 1 token)
//! - **API Stability**: Chunk structure frozen; no breaking changes in 0.x
//! - **Panic Safety**: No unwrap(), no expect() except hardcoded regex (tested)
//!
//! ## Performance
//!
//! - **Time**: O(n) where n = document content length
//! - **Space**: O(chunks) - stores all chunks in memory
//! - **Token Estimation**: O(content_length) - linear scan
//!
//! Suitable for documents up to 100MB+ with efficient memory management.
//!
//! ## Versioning
//!
//! This crate uses Semantic Versioning:
//! - `0.1.0`: Initial release with core chunking
//! - Future `0.2.0`: Custom separators, token estimation plugins
//! - Future `1.0.0`: Stable public API
//!
//! Breaking changes documented in CHANGELOG.md.
//!
//! ## License
//!
//! MIT - See LICENSE file in repository
//!

pub mod chunk;
pub mod document;

// Re-export public API
pub use chunk::{chunk, chunk_all, Chunk, ChunkLevel, ChunkingResult};
pub use document::Document;
