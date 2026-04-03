#![allow(clippy::pedantic)]
#![allow(clippy::nursery)]
#![allow(clippy::complexity)]
//! `ctd` — AI-Optimized Documentation Transformation Library
//!
//! Transforms raw documentation (local markdown files or live websites) into
//! AI-friendly knowledge structures: semantic chunks, a searchable Tantivy index,
//! a knowledge DAG for related-content discovery, and a `llms.txt` entry point.
//!
//! # Five-Phase Pipeline
//!
//! ```text
//! ┌─────────────┐    ┌──────────┐    ┌─────────────┐    ┌───────────┐    ┌────────┐
//! │  1. discover │───▶│ 2. analyze│───▶│ 3. transform │───▶│ 4. chunk  │───▶│5. index│
//! └─────────────┘    └──────────┘    └─────────────┘    └───────────┘    └────────┘
//! ```
//!
//! 1. **[`discover`]** — Walk source directories; return [`discover::DiscoveryFile`] list
//! 2. **[`analyze`]** — Extract titles, headings, [`analyze::Link`]s, categories,
//!    word counts from each file
//! 3. **[`transform`]** — Rewrite links, enforce heading structure, inject frontmatter
//! 4. **[`chunking_adapter`]** — Delegate to `contextual-chunker` for hierarchical
//!    `Summary / Standard / Detailed` chunks
//! 5. **[`index`]** — Build Tantivy full-text index + HNSW similarity graph;
//!    emit `llms.txt` via [`llms`]
//!
//! # Domain Types
//!
//! Validated newtypes live in [`types`] — pass through the CLI boundary to guarantee
//! constraints are checked exactly once:
//!
//! - [`types::Slug`], [`types::Title`], [`types::Category`], [`types::FilePath`]
//! - [`types::MaxRelatedChunks`], [`types::HnswM`], [`types::HnswEfConstruction`]
//!
//! Scrape-phase behaviour is expressed as explicit enums rather than `bool` flags:
//!
//! - [`scrape::SitemapStrategy`] — sitemap vs. crawl-only
//! - [`scrape::RobotsPolicy`] — respect or ignore robots.txt
//! - [`scrape::FilteringMode`] — apply content-density filtering or store raw markdown
//! - [`scrape::RetryStrategy`] — exponential backoff vs. fixed delay
//! - [`scrape::StealthMode`] — randomised browser headers vs. plain user-agent
//!
//! # Key Modules
//!
//! | Module | Role |
//! |--------|------|
//! | [`analyze`] | Metadata extraction and auto-categorisation |
//! | [`transform`] | AST-level markdown normalisation |
//! | [`chunk`] | Re-export of `contextual-chunker` public API |
//! | [`chunking_adapter`] | Bridge between pipeline types and chunker types |
//! | [`index`] | Tantivy + HNSW index construction |
//! | [`search`] | Full-text query execution |
//! | [`scrape`] | Spider-based web scraping with retry/backoff |
//! | [`filter`] | HTML pruning and BM25 relevance filtering |
//! | [`graph`] | Knowledge DAG for related-document edges |
//! | [`llms`] | `llms.txt` generation |
//! | [`types`] | Validated domain newtypes |
//! | [`errors`] | Structured error taxonomy |
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use doc_transformer::{discover, analyze, transform, index};
//! use std::path::Path;
//!
//! let src = Path::new("./docs");
//! let out = Path::new("./output");
//!
//! let (files, _manifest) = discover::discover_files(src)?;
//! let analyses = analyze::analyze_files(&files, src, None)?;
//! let (analyses, link_map) = doc_transformer::assign::assign_ids(analyses);
//! transform::transform_all(&analyses, &link_map, out)?;
//! index::index_documents(out, out, None)?;
//! ```

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::float_cmp,
        clippy::field_reassign_with_default,
        clippy::too_many_arguments,
        clippy::match_same_arms
    )
)]
// Allow pedantic format_push_string warnings - common in imperative shell code
#![allow(clippy::format_push_string)]
// Allow pedantic clone_on_copy warnings - sometimes intentional for explicitness
#![allow(clippy::clone_on_copy)]
// Allow pedantic unnecessary_wraps - sometimes Result is part of API even if always Ok
#![allow(clippy::unnecessary_wraps)]
// Allow items_after_statements - common in initialization code
#![allow(clippy::items_after_statements)]
// Allow manual_let_else - match is sometimes clearer
#![allow(clippy::manual_let_else)]
// Allow missing_errors_doc - not all Result functions need full error docs
#![allow(clippy::missing_errors_doc)]
// Allow cast_precision_loss - intentional for BM25 scoring
#![allow(clippy::cast_precision_loss)]
// Allow ptr_arg - HashMap by value is sometimes clearer
#![allow(clippy::ptr_arg)]
// Allow implicit_hasher - HashMap is commonly used
#![allow(clippy::implicit_hasher)]
// Allow cast_possible_truncation - validated conversions
#![allow(clippy::cast_possible_truncation)]
// Allow needless_borrow - sometimes clearer
#![allow(clippy::needless_borrow)]
// Allow map_unwrap_or - functional style sometimes uses this
#![allow(clippy::map_unwrap_or)]
// Allow unnecessary_valu - sometimes needed for API compatibility
#![allow(clippy::unnecessary_literal_unwrap)]
// Allow used_underscore_binding - sometimes needed
#![allow(clippy::used_underscore_binding)]

pub mod analyze;
pub mod assign;
pub mod cache;
pub mod calc;
pub mod chunk;
pub mod chunking_adapter;
pub mod config;
pub mod diff;
pub mod discover;
pub mod embeddings;
pub mod errors;

// Re-export commonly used types for library users
pub use config::{CategoryConfig, GraphConfig};
pub use discover::DiscoverConfig;
#[cfg(feature = "enhanced")]
pub mod features;
pub mod filter;
pub mod graph;
pub mod highlight;
pub mod index;
pub mod llms;
pub mod math_types;
pub mod mcp;
pub mod persisted;
pub mod scrape;
pub mod search;
pub mod state;
pub mod transform;
pub mod types;
pub mod validate;
pub mod watch;
