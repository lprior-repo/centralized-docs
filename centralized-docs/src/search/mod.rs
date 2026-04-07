//! Full-text search using Tantivy
//!
//! Replaces custom BM25 with a proven, production-grade search engine.
//! Handles indexing, querying, and error recovery.
//!
//! ## Design
//!
//! - **Index Location**: `{base_path}/.tantivy_index/`
//! - **Schema**: title (boosted), summary, category, `word_count`
//! - **Query Support**: Simple queries, phrases, boolean operators
//! - **Error Recovery**: Auto-rebuild on corruption
//!
//! ## Example
//!
//! ```no_run
//! use doc_transformer::search;
//! use std::path::Path;
//!
//! let index_path = Path::new("./output/.tantivy_index");
//! let index = search::open_or_create_index(index_path)?;
//! let results = search::search_index(&index, "rust programming", 10)?;
//! # Ok::<(), anyhow::Error>(())
//! ```

use tantivy::schema::{Field, Schema, STORED, TEXT};
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum IndexerError {
    #[error("Directory access failed: {0}")]
    DirectoryAccessFailed(String),
    #[error("Index commit failed: {0}")]
    IndexCommitFailed(String),
    #[error("Invalid document")]
    InvalidDocument,
    #[error("Uncommitted changes")]
    UncommittedChanges,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum SearchError {
    #[error("Empty query")]
    EmptyQuery,
    #[error("Query parse error: {0}")]
    QueryParseError(String),
    #[error("Postcondition violated")]
    PostconditionViolated,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Schema field indices (cached for performance)
pub struct SchemaFields {
    pub id: Field,
    pub title: Field,
    pub summary: Field,
    pub content: Field,
    pub category: Field,
    pub word_count: Field,
    pub path: Field,
}

/// Single search result with score
#[allow(dead_code)] // Exported for library users - not used internally
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub category: String,
    pub score: crate::math_types::Score,
    pub path: String,
}

/// Create Tantivy schema for document indexing
pub(super) fn create_schema() -> (Schema, SchemaFields) {
    #[allow(unused_mut)] // tantivy SchemaBuilder API requires &mut self for add_*_field
    let mut schema_builder = Schema::builder();

    let id = schema_builder.add_text_field("id", TEXT | STORED);
    let title = schema_builder.add_text_field("title", TEXT | STORED);
    let summary = schema_builder.add_text_field("summary", TEXT | STORED);
    let content = schema_builder.add_text_field("content", TEXT);
    let category = schema_builder.add_text_field("category", TEXT | STORED);
    let word_count = schema_builder.add_u64_field("word_count", STORED);
    let path = schema_builder.add_text_field("path", TEXT | STORED);

    let schema = schema_builder.build();
    let fields = SchemaFields {
        id,
        title,
        summary,
        content,
        category,
        word_count,
        path,
    };

    (schema, fields)
}

mod index_ops;
mod indexer;
mod query;

pub use index_ops::{open_existing_index, open_or_create_index, rebuild_index_from_json};
pub use indexer::{index_chunks, index_documents};
pub use query::search_index;

#[cfg(test)]
mod test_rebuild;
#[cfg(test)]
mod test_schema_indexer;
#[cfg(test)]
mod test_search;
#[cfg(test)]
mod test_support;
