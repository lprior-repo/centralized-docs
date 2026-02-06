//! doc_transformer - AI-Optimized Documentation Transformation Library
//!
//! A Rust library for transforming raw documentation into AI-friendly knowledge structures.
//! This library implements a 5-phase pipeline that processes markdown files and produces
//! indexed, searchable documentation with semantic chunking and relationship detection.
//!
//! # Pipeline Overview
//!
//! The transformation pipeline consists of five phases:
//!
//! 1. **Discovery** ([`discover`]) - Scan source directories and identify markdown files
//! 2. **Analysis** ([`analyze`]) - Extract metadata, headings, links, and content statistics
//! 3. **Transformation** ([`transform`]) - Convert to canonical format with link rewriting
//! 4. **Indexing** ([`index`]) - Build searchable index with knowledge graph
//! 5. **Output** - Generate chunks, search index, and llms.txt entry point
//!
//! # Key Modules
//!
//! - [`analyze`] - Document analysis and metadata extraction
//! - [`transform`] - Markdown transformation and normalization
//! - [`chunk`] - Semantic document chunking with context
//! - [`index`] - Search index construction with BM25 and HNSW
//! - [`search`] - Full-text search over indexed documentation
//! - [`scrape`] - Web scraping for documentation ingestion
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use doc_transformer::{discover, analyze, index};
//! use std::path::Path;
//!
//! // Phase 1: Discover
//! let (files, _manifest) = discover::discover_files(Path::new("./docs"))?;
//!
//! // Phase 2: Analyze
//! let analyses = analyze::analyze_files(&files, Path::new("./docs"), None)?;
//!
//! // Phases 3-5: Transform, chunk, and index
//! // ... see individual module documentation
//! ```

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

pub mod analyze;
pub mod assign;
pub mod chunk;
pub mod chunking_adapter;
pub mod config;
pub mod discover;
pub mod embeddings;
pub mod errors;
#[cfg(feature = "enhanced")]
pub mod features;
pub mod filter;
pub mod graph;
pub mod highlight;
pub mod index;
pub mod llms;
pub mod scrape;
pub mod search;
pub mod similarity;
pub mod transform;
pub mod types;
pub mod validate;
