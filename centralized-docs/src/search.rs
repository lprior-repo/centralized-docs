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

use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;
use tantivy::collector::TopDocs;
use tantivy::doc;
use tantivy::query::QueryParser;
use tantivy::schema::{Field, Schema, Value, STORED, TEXT};
use tantivy::Index;
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
///
/// Schema includes:
/// - `id`: String identifier (stored, not indexed)
/// - `title`: Text field (important for ranking)
/// - `summary`: Text field (stored, indexed)
/// - `content`: Combined searchable content (title + summary, indexed but not stored)
/// - `category`: Text field for filtering (stored, not indexed)
/// - `word_count`: U64 field for relevance calculation (stored, not indexed)
/// - `path`: Stored file path for accurate output
///
/// # Returns
///
/// Tantivy Schema with field definitions
fn create_schema() -> (Schema, SchemaFields) {
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

/// Rebuild the Tantivy index from INDEX.json data.
///
/// This is used for recovery when the Tantivy index is corrupted.
/// It reads document data from INDEX.json and re-indexes all documents.
#[allow(clippy::too_many_lines)]
pub fn rebuild_index_from_json(index_path: &Path) -> Result<Index> {
    let index_json_path = index_path.join("INDEX.json");

    let index_content = fs::read_to_string(&index_json_path)
        .map_err(|e| anyhow!("Failed to read INDEX.json: {e}"))?;

    let index_value: serde_json::Value = serde_json::from_str(&index_content)
        .map_err(|e| anyhow!("Failed to parse INDEX.json: {e}"))?;

    let documents = index_value["documents"]
        .as_array()
        .ok_or_else(|| anyhow!("Invalid INDEX.json: missing documents array"))?;

    let docs: Vec<crate::index::IndexDocument> = documents
        .iter()
        .filter_map(|doc| {
            let id = doc["id"].as_str()?;
            let title = doc["title"].as_str().map_or("", |s| s);
            let summary = doc["summary"].as_str().map_or("", |s| s);
            let path = doc["path"].as_str().map_or("", |s| s);
            let category = doc["category"].as_str().map_or("", |s| s);
            let word_count = doc["word_count"].as_u64().map_or(0, |v| v) as usize;
            let tags: Vec<String> = doc["tags"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .map_or_else(Vec::new, std::convert::identity);
            let chunk_ids: Vec<String> = doc["chunk_ids"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .map_or_else(Vec::new, std::convert::identity);
            let headings: Vec<String> = doc["headings"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .map_or_else(Vec::new, std::convert::identity);
            let content = doc["content"].as_str().map_or("", |s| s).into();

            Some(crate::index::IndexDocument {
                id: id.to_string(),
                title: title.to_string(),
                path: path.to_string(),
                category: category.to_string(),
                tags,
                summary: summary.to_string(),
                word_count,
                chunk_ids,
                headings,
                content,
            })
        })
        .collect();

    let chunks_val = index_value["chunks"]
        .as_array()
        .ok_or_else(|| anyhow!("Invalid INDEX.json: missing chunks array"))?;

    let chunks: Vec<crate::chunking_adapter::Chunk> = chunks_val
        .iter()
        .filter_map(|chunk| {
            let chunk_id = chunk["chunk_id"].as_str()?;
            let doc_id = chunk["doc_id"].as_str().map_or("", |s| s);
            let doc_title = chunk["doc_title"].as_str().map_or("", |s| s);
            let summary = chunk["summary"].as_str().map_or("", |s| s);
            let token_count = chunk["token_count"].as_u64().map_or(0, |v| v) as usize;
            let heading = chunk["heading"].as_str().map(String::from);

            let level_str = chunk["chunk_level"].as_str().map_or("standard", |s| s);
            let chunk_level = match level_str {
                "summary" => contextual_chunker::ChunkLevel::Summary,
                "detailed" => contextual_chunker::ChunkLevel::Detailed,
                _ => contextual_chunker::ChunkLevel::Standard,
            };

            let chunk_filename = format!("{}-{}.md", chunk_id.replace(['/', '#'], "-"), level_str);
            let chunk_path = index_path.join("chunks").join(&chunk_filename);

            let raw_content = fs::read_to_string(&chunk_path)
                .map_err(|e| {
                    anyhow::anyhow!("Failed to read chunk file {}: {}", chunk_path.display(), e)
                })
                .ok()?;

            let content = if raw_content.starts_with("---\n") || raw_content.starts_with("---\r\n")
            {
                let remaining = raw_content
                    .lines()
                    .skip(1)
                    .skip_while(|line| line.trim_end() != "---")
                    .skip(1)
                    .collect::<Vec<_>>()
                    .join("\n");
                if remaining.is_empty() {
                    raw_content
                } else {
                    remaining
                }
            } else {
                raw_content
            };

            Some(crate::chunking_adapter::Chunk {
                chunk_id: chunk_id.to_string(),
                doc_id: doc_id.to_string(),
                doc_title: doc_title.to_string(),
                chunk_index: 0,
                content,
                token_count,
                heading,
                heading_path: vec![],
                chunk_type: contextual_chunker::ChunkType::Prose,
                previous_chunk_id: None,
                next_chunk_id: None,
                related_chunk_ids: vec![],
                summary: summary.to_string(),
                chunk_level,
                parent_chunk_id: None,
                child_chunk_ids: vec![],
                context_prefix: None,
            })
        })
        .collect();

    let index_dir = index_path.join(".tantivy_index");
    fs::create_dir_all(&index_dir)?;
    let (schema, _fields) = create_schema();
    let index = Index::create_in_dir(&index_dir, schema)
        .map_err(|e| anyhow!("Failed to create index: {e}"))?;

    if !chunks.is_empty() {
        #[allow(unused_mut)] // tantivy IndexWriter API requires &mut self
        let mut writer = index
            .writer(50_000_000)
            .map_err(|e| anyhow!("Failed to create writer: {e}"))?;
        index_chunks(&mut writer, &docs, &chunks)?;
        writer
            .commit()
            .map_err(|e| anyhow!("Failed to commit: {e}"))?;
    }

    Ok(index)
}

pub fn open_or_create_index(index_path: &Path) -> Result<Index> {
    let index_dir = index_path.join(".tantivy_index");

    // Try to open existing index
    if index_dir.exists() {
        if index_dir.is_file() {
            fs::remove_file(&index_dir)?;
        } else {
            match Index::open_in_dir(&index_dir) {
                Ok(index) => return Ok(index),
                Err(e) => {
                    // Index is corrupted, try to rebuild from INDEX.json
                    eprintln!("Warning: Failed to open index: {e}");
                    if index_path.join("INDEX.json").exists() {
                        eprintln!("Attempting to rebuild index from INDEX.json...");
                        match rebuild_index_from_json(index_path) {
                            Ok(index) => {
                                eprintln!("Successfully rebuilt index from INDEX.json");
                                return Ok(index);
                            }
                            Err(rebuild_err) => {
                                eprintln!(
                                    "Warning: Failed to rebuild index from INDEX.json: {rebuild_err}"
                                );
                            }
                        }
                    }
                    // Fall back to creating empty index
                    fs::remove_dir_all(&index_dir).ok();
                }
            }
        }
    }

    // Create new index
    fs::create_dir_all(&index_dir)?;
    let (schema, _fields) = create_schema();
    Index::create_in_dir(&index_dir, schema).map_err(|e| anyhow!("Failed to create index: {e}"))
}

/// Open Tantivy index if it already exists.
///
/// Returns Ok(None) when:
/// - No index directory is present
/// - Index directory is corrupted (recovers by removing)
///
/// This allows the search to fall back to INDEX.json when the Tantivy index
/// is unavailable or corrupted.
#[allow(dead_code)]
pub fn open_existing_index(index_path: &Path) -> Result<Option<Index>> {
    let index_dir = index_path.join(".tantivy_index");

    if !index_dir.exists() {
        return Ok(None);
    }

    // Handle case where path exists but is a file (not a directory)
    if index_dir.is_file() {
        fs::remove_file(&index_dir).ok();
        return Ok(None);
    }

    // Try to open existing index
    if let Ok(index) = Index::open_in_dir(&index_dir) {
        Ok(Some(index))
    } else {
        // Index is corrupted, remove it and return None to trigger fallback
        // This allows search to use INDEX.json instead
        fs::remove_dir_all(&index_dir).ok();
        Ok(None)
    }
}

/// Index a batch of documents into Tantivy (used by tests)
#[allow(dead_code)]
pub fn index_documents(
    writer: &mut tantivy::IndexWriter,
    documents: &[crate::index::IndexDocument],
) -> std::result::Result<(), IndexerError> {
    let (_schema, fields) = create_schema();

    // Add each document
    documents
        .iter()
        .try_for_each(|doc| -> std::result::Result<(), IndexerError> {
            let tags_str = doc.tags.join(" ");
            let headings_str = doc.headings.join(" ");
            let searchable_content = format!(
                "{} {} {} {} {} {}",
                doc.title, doc.summary, doc.path, tags_str, headings_str, doc.content
            );

            // Use tantivy::doc! macro to build document
            let tantivy_doc = doc!(
                fields.id => doc.id.as_str(),
                fields.title => doc.title.as_str(),
                fields.summary => doc.summary.as_str(),
                fields.content => searchable_content.as_str(),
                fields.category => doc.category.as_str(),
                fields.word_count => doc.word_count as u64,
                fields.path => doc.path.as_str(),
            );

            writer
                .add_document(tantivy_doc)
                .map_err(|e| IndexerError::IndexCommitFailed(e.to_string()))?;
            Ok(())
        })?;

    Ok(())
}

/// Index a batch of chunks into Tantivy
///
/// ## Behavior
///
/// - Adds all chunks
/// - Does NOT commit transaction (caller is responsible)
///
/// ## Error Handling
///
/// Returns error if write fails.
///
/// # Arguments
///
/// * `writer` - Mutable reference to Tantivy `IndexWriter`
/// * `documents` - Original documents to resolve categories/paths
/// * `chunks` - Chunks to index
///
/// # Returns
///
/// Success, error if any operation fails
pub fn index_chunks(
    writer: &mut tantivy::IndexWriter,
    documents: &[crate::index::IndexDocument],
    chunks: &[crate::chunking_adapter::Chunk],
) -> std::result::Result<(), IndexerError> {
    let (_schema, fields) = create_schema();

    // Map doc_id to doc for fast lookup of category and path
    let doc_map: std::collections::HashMap<_, _> =
        documents.iter().map(|d| (d.id.as_str(), d)).collect();

    // Add each chunk
    chunks
        .iter()
        .try_for_each(|chunk| -> std::result::Result<(), IndexerError> {
            let doc = doc_map.get(chunk.doc_id.as_str());
            let category = doc.map_or("uncategorized", |d| d.category.as_str());

            // Build the path based on how chunks are saved: "chunks/xxx-summary.md"
            let level_suffix = chunk.chunk_level.as_str();
            let chunk_filename = format!(
                "chunks/{}-{}.md",
                chunk.chunk_id.replace(['/', '#'], "-"),
                level_suffix
            );

            let title = if let Some(h) = &chunk.heading {
                format!("{} - {}", chunk.doc_title, h)
            } else {
                chunk.doc_title.clone()
            };

            // Use tantivy::doc! macro to build document
            let tantivy_doc = doc!(
                fields.id => chunk.chunk_id.as_str(),
                fields.title => title.as_str(),
                fields.summary => chunk.summary.as_str(),
                fields.content => chunk.content.as_str(),
                fields.category => category,
                fields.word_count => chunk.token_count as u64,
                fields.path => chunk_filename.as_str(),
            );

            writer
                .add_document(tantivy_doc)
                .map_err(|e| IndexerError::IndexCommitFailed(e.to_string()))?;
            Ok(())
        })?;

    Ok(())
}

/// Escape wildcard characters that would create unintended wildcard queries.
///
/// Only escapes `*` and `?` which would match arbitrary characters.
/// Other special characters (quotes, parentheses, etc.) are left unescaped
/// so that invalid queries still produce helpful error messages.
fn escape_tantivy_query(query: &str) -> String {
    query.chars().fold(
        String::with_capacity(query.len().saturating_mul(2)),
        |mut escaped, ch| {
            if matches!(ch, '*' | '?') {
                escaped.push('\\');
            }
            escaped.push(ch);
            escaped
        },
    )
}

/// Search the Tantivy index
///
/// ## Query Syntax
///
/// - Simple: `rust programming` → Any document with both terms
/// - Phrase: `"rust programming"` → Exact phrase match
/// - Boolean: `rust AND systems` → Both terms required
/// - Negation: `rust NOT python` → rust without python
/// - Operators: `(rust OR systems) AND NOT python`
///
/// ## Behavior
///
/// - Parses query using Tantivy's default `QueryParser`
/// - Executes against content field (searchable combination)
/// - Returns top N results sorted by BM25 score (highest first)
/// - Returns empty Vec if no matches
///
/// ## Error Handling
///
/// Returns error if query is invalid (syntax error).
/// Empty query returns error.
///
/// # Arguments
///
/// * `index` - Tantivy index to search
/// * `query_str` - Query string (supports phrase and boolean operators)
/// * `limit` - Maximum number of results to return
///
/// # Returns
///
/// Vector of `SearchResult` sorted by relevance (highest score first)
#[allow(dead_code)] // Exported for library users - not used internally
pub fn search_index(
    index: &Index,
    query_str: &str,
    limit: usize,
) -> std::result::Result<Vec<SearchResult>, SearchError> {
    let (_schema, fields) = create_schema();

    // Validate query using centralized validation
    let query_str = crate::validate::validate_query(query_str)
        .map_err(|e| SearchError::QueryParseError(e.to_string()))?;

    // Validate limit to prevent Tantivy panic (must be > 0)
    let limit = crate::validate::validate_limit(&limit.to_string())
        .map_err(|e| SearchError::QueryParseError(e.to_string()))?;

    // Escape special characters that have meaning in Tantivy query syntax
    // This prevents wildcard queries and other unintended query parsing
    let escaped_query = escape_tantivy_query(query_str);

    // Get reader for searching
    let reader = index
        .reader()
        .map_err(|e| SearchError::Other(anyhow::anyhow!(e)))?;
    let searcher = reader.searcher();

    // Parse query
    // Search across title and content. We could add boosts, but simply including title
    // helps find relevant structural matches.
    #[allow(unused_mut)] // tantivy QueryParser::set_field_boost requires &mut self
    let mut query_parser = QueryParser::for_index(index, vec![fields.title, fields.content]);
    query_parser.set_field_boost(fields.title, 3.0); // Boost title matches significantly

    let query = query_parser.parse_query(&escaped_query).map_err(|_e| {
        SearchError::QueryParseError("Search query contains unsupported syntax.".to_string())
    })?;

    // Execute search and get top results
    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(limit))
        .map_err(|e| SearchError::Other(anyhow::anyhow!(e)))?;

    // Extract stored fields from results
    #[allow(unused_mut)] // Vec::sort_by_key requires &mut self — no functional alternative in std
    let mut results: Vec<SearchResult> = top_docs
        .into_iter()
        .map(
            |(tantivy_score, doc_address)| -> std::result::Result<Option<SearchResult>, SearchError> {
                let retrieved_doc: tantivy::TantivyDocument = searcher.doc(doc_address).map_err(|e| SearchError::Other(anyhow::anyhow!(e)))?;

                // Extract fields (safely with defaults)
                // Tantivy 0.25: Convert CompactDocValue -> OwnedValue -> extract
                let id = retrieved_doc
                    .get_first(fields.id)
                    .map(tantivy::schema::OwnedValue::from)
                    .and_then(|v| v.as_ref().as_str().map(std::string::ToString::to_string))
                    .map_or_else(|| "unknown".to_string(), std::convert::identity);

                let title = retrieved_doc
                    .get_first(fields.title)
                    .map(tantivy::schema::OwnedValue::from)
                    .and_then(|v| v.as_ref().as_str().map(std::string::ToString::to_string))
                    .map_or_else(|| "Untitled".to_string(), std::convert::identity);

                let summary = retrieved_doc
                    .get_first(fields.summary)
                    .map(tantivy::schema::OwnedValue::from)
                    .and_then(|v| v.as_ref().as_str().map(std::string::ToString::to_string))
                    .map_or_else(|| "No summary available".to_string(), std::convert::identity);

                let category = retrieved_doc
                    .get_first(fields.category)
                    .map(tantivy::schema::OwnedValue::from)
                    .and_then(|v| v.as_ref().as_str().map(std::string::ToString::to_string))
                    .map_or_else(|| "uncategorized".to_string(), std::convert::identity);

                let _word_count = retrieved_doc
                    .get_first(fields.word_count)
                    .map(tantivy::schema::OwnedValue::from)
                    .and_then(|v| v.as_ref().as_u64())
                    .map_or(0, |v| v);

                let path = retrieved_doc
                    .get_first(fields.path)
                    .map(tantivy::schema::OwnedValue::from)
                    .and_then(|v| v.as_ref().as_str().map(std::string::ToString::to_string))
                    .map_or_else(|| format!("docs/{}.md", id.replace('/', "-")), std::convert::identity);

                let score = crate::math_types::Score::try_new(tantivy_score)
                    .map_or_else(|_| crate::math_types::Score::zero(), std::convert::identity);

                Ok(Some(SearchResult {
                    id,
                    title,
                    summary,
                    category,
                    score,
                    path,
                }))
            },
        )
        .filter_map(Result::transpose)
        .collect::<std::result::Result<Vec<_>, SearchError>>()?;

    results.sort_by_key(|b| std::cmp::Reverse(b.score));

    Ok(results)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn make_index_document(
        id: &str,
        title: &str,
        summary: &str,
        content: &str,
        category: &str,
    ) -> crate::index::IndexDocument {
        crate::index::IndexDocument {
            id: id.to_string(),
            title: title.to_string(),
            path: format!("docs/{id}.md"),
            category: category.to_string(),
            tags: vec![],
            summary: summary.to_string(),
            word_count: content.split_whitespace().count(),
            chunk_ids: vec![],
            headings: vec![],
            content: content.into(),
        }
    }

    fn make_chunk(
        chunk_id: &str,
        doc_id: &str,
        doc_title: &str,
        content: &str,
        heading: Option<&str>,
    ) -> crate::chunking_adapter::Chunk {
        crate::chunking_adapter::Chunk {
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
            chunk_level: contextual_chunker::ChunkLevel::Standard,
            parent_chunk_id: None,
            child_chunk_ids: vec![],
            context_prefix: None,
        }
    }

    fn create_test_index_with_docs(docs: &[crate::index::IndexDocument]) -> (TempDir, Index) {
        let dir = TempDir::new().unwrap();
        let index = open_or_create_index(dir.path()).unwrap();
        let mut writer = index.writer(50_000_000).unwrap();
        index_documents(&mut writer, docs).unwrap();
        writer.commit().unwrap();
        (dir, index)
    }

    #[test]
    fn test_open_or_create_index_new() -> Result<()> {
        let dir = TempDir::new()?;
        let index_path = dir.path();

        let _index = open_or_create_index(index_path)?;
        assert!(index_path.join(".tantivy_index").exists());

        Ok(())
    }

    #[test]
    fn test_open_or_create_index_existing() -> Result<()> {
        let dir = TempDir::new()?;
        let index_path = dir.path();

        let _index1 = open_or_create_index(index_path)?;
        let _index2 = open_or_create_index(index_path)?;

        assert!(index_path.join(".tantivy_index").exists());

        Ok(())
    }

    #[test]
    fn test_open_or_create_index_recovers_from_file_path() -> Result<()> {
        let dir = TempDir::new()?;
        let index_path = dir.path();
        let index_dir = index_path.join(".tantivy_index");

        fs::write(&index_dir, "not a directory")?;

        let _index = open_or_create_index(index_path)?;

        assert!(index_dir.exists());
        assert!(index_dir.is_dir());

        Ok(())
    }

    #[test]
    fn test_create_schema() {
        let (schema, fields) = create_schema();

        assert!(schema.get_field("id").is_ok());
        assert!(schema.get_field("title").is_ok());
        assert!(schema.get_field("summary").is_ok());
        assert!(schema.get_field("content").is_ok());
        assert!(schema.get_field("category").is_ok());
        assert!(schema.get_field("word_count").is_ok());
        assert!(schema.get_field("path").is_ok());

        assert_eq!(fields.id, schema.get_field("id").unwrap());
        assert_eq!(fields.title, schema.get_field("title").unwrap());
        assert_eq!(fields.summary, schema.get_field("summary").unwrap());
        assert_eq!(fields.content, schema.get_field("content").unwrap());
        assert_eq!(fields.category, schema.get_field("category").unwrap());
        assert_eq!(fields.word_count, schema.get_field("word_count").unwrap());
        assert_eq!(fields.path, schema.get_field("path").unwrap());
    }

    #[test]
    fn test_schema_fields_struct_fields() {
        let (_, fields) = create_schema();
        let _ = SchemaFields {
            id: fields.id,
            title: fields.title,
            summary: fields.summary,
            content: fields.content,
            category: fields.category,
            word_count: fields.word_count,
            path: fields.path,
        };
    }

    #[test]
    fn test_index_documents_single() {
        let docs = vec![make_index_document(
            "doc1",
            "Rust Programming",
            "A guide to Rust",
            "Rust is a systems programming language",
            "tutorial",
        )];
        let (_dir, index) = create_test_index_with_docs(&docs);
        let results = search_index(&index, "Rust", 10).unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_index_documents_multiple() {
        let docs = vec![
            make_index_document(
                "doc1",
                "Rust Basics",
                "Intro to Rust",
                "Rust basics tutorial",
                "tutorial",
            ),
            make_index_document(
                "doc2",
                "Python Guide",
                "Python basics",
                "Python programming guide",
                "tutorial",
            ),
            make_index_document(
                "doc3",
                "API Reference",
                "HTTP API",
                "REST API endpoints",
                "ref",
            ),
        ];
        let (_dir, index) = create_test_index_with_docs(&docs);
        let results = search_index(&index, "Rust", 10).unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_index_documents_empty() {
        let docs: Vec<crate::index::IndexDocument> = vec![];
        let (_dir, index) = create_test_index_with_docs(&docs);
        let results = search_index(&index, "nonexistent", 10).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_index_documents_with_tags_and_headings() {
        let mut doc = make_index_document(
            "doc1",
            "Advanced Rust",
            "Advanced patterns",
            "Pattern matching and traits in Rust",
            "concept",
        );
        doc.tags = vec!["rust".to_string(), "patterns".to_string()];
        doc.headings = vec!["Introduction".to_string(), "Patterns".to_string()];
        let (_dir, index) = create_test_index_with_docs(&[doc]);
        let results = search_index(&index, "Advanced", 10).unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_index_chunks_basic() {
        let docs = vec![make_index_document(
            "doc1",
            "Test Document",
            "Test summary",
            "Test content about programming",
            "concept",
        )];
        let chunks = vec![
            make_chunk(
                "doc1#0-standard",
                "doc1",
                "Test Document",
                "First chunk content about Rust",
                Some("Introduction"),
            ),
            make_chunk(
                "doc1#1-standard",
                "doc1",
                "Test Document",
                "Second chunk about programming",
                Some("Details"),
            ),
        ];

        let dir = TempDir::new().unwrap();
        let index = open_or_create_index(dir.path()).unwrap();
        let mut writer = index.writer(50_000_000).unwrap();
        index_chunks(&mut writer, &docs, &chunks).unwrap();
        writer.commit().unwrap();

        let results = search_index(&index, "Rust", 10).unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_index_chunks_empty_docs_and_chunks() {
        let docs: Vec<crate::index::IndexDocument> = vec![];
        let chunks: Vec<crate::chunking_adapter::Chunk> = vec![];

        let dir = TempDir::new().unwrap();
        let index = open_or_create_index(dir.path()).unwrap();
        let mut writer = index.writer(50_000_000).unwrap();
        index_chunks(&mut writer, &docs, &chunks).unwrap();
        writer.commit().unwrap();
    }

    #[test]
    fn test_index_chunks_doc_not_in_doc_map() {
        let chunks = vec![make_chunk(
            "orphan#0-standard",
            "orphan_doc",
            "Orphan Doc",
            "Orphan chunk content",
            None,
        )];

        let dir = TempDir::new().unwrap();
        let index = open_or_create_index(dir.path()).unwrap();
        let mut writer = index.writer(50_000_000).unwrap();
        index_chunks(&mut writer, &[], &chunks).unwrap();
        writer.commit().unwrap();
    }

    #[test]
    fn test_index_chunks_no_heading() {
        let docs = vec![make_index_document(
            "doc1",
            "No Heading Doc",
            "Summary",
            "Content",
            "concept",
        )];
        let chunks = vec![make_chunk(
            "doc1#0-standard",
            "doc1",
            "No Heading Doc",
            "Chunk without heading",
            None,
        )];

        let dir = TempDir::new().unwrap();
        let index = open_or_create_index(dir.path()).unwrap();
        let mut writer = index.writer(50_000_000).unwrap();
        index_chunks(&mut writer, &docs, &chunks).unwrap();
        writer.commit().unwrap();
    }

    #[test]
    fn test_index_chunks_summary_level() {
        let docs = vec![make_index_document(
            "doc1",
            "Summary Doc",
            "Summary",
            "Content",
            "tutorial",
        )];
        let mut chunk = make_chunk("doc1#0", "doc1", "Summary Doc", "Summary content", None);
        chunk.chunk_level = contextual_chunker::ChunkLevel::Summary;

        let dir = TempDir::new().unwrap();
        let index = open_or_create_index(dir.path()).unwrap();
        let mut writer = index.writer(50_000_000).unwrap();
        index_chunks(&mut writer, &docs, &[chunk]).unwrap();
        writer.commit().unwrap();
    }

    #[test]
    fn test_search_index_basic_query() {
        let docs = vec![
            make_index_document(
                "doc1",
                "Rust Programming",
                "Learn Rust",
                "Rust is a systems programming language focused on safety",
                "tutorial",
            ),
            make_index_document(
                "doc2",
                "Python Guide",
                "Learn Python",
                "Python is a high-level scripting language",
                "tutorial",
            ),
        ];
        let (_dir, index) = create_test_index_with_docs(&docs);

        let results = search_index(&index, "Rust", 10).unwrap();
        assert!(!results.is_empty());
        assert!(
            results.iter().any(|r| r.id == "doc1"),
            "doc1 should be in search results"
        );
    }

    #[test]
    fn test_search_index_returns_empty_for_no_match() {
        let docs = vec![make_index_document(
            "doc1",
            "Rust Programming",
            "Learn Rust",
            "Rust content",
            "tutorial",
        )];
        let (_dir, index) = create_test_index_with_docs(&docs);

        let results = search_index(&index, "xyznonexistent", 10).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_index_limit() {
        let docs = vec![
            make_index_document(
                "doc1",
                "Alpha Document",
                "First",
                "alpha content here",
                "concept",
            ),
            make_index_document(
                "doc2",
                "Beta Document",
                "Second",
                "beta content here",
                "concept",
            ),
            make_index_document(
                "doc3",
                "Gamma Document",
                "Third",
                "gamma content here",
                "concept",
            ),
        ];
        let (_dir, index) = create_test_index_with_docs(&docs);

        let results = search_index(&index, "content", 2).unwrap();
        assert!(results.len() <= 2);
    }

    #[test]
    fn test_search_index_empty_query_returns_error() {
        let docs = vec![make_index_document(
            "doc1", "Test", "Test", "content", "concept",
        )];
        let (_dir, index) = create_test_index_with_docs(&docs);

        let result = search_index(&index, "", 10);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, SearchError::QueryParseError(_)),
            "Expected QueryParseError, got: {err:?}"
        );
    }

    #[test]
    fn test_search_index_whitespace_only_query_returns_error() {
        let docs = vec![make_index_document(
            "doc1", "Test", "Test", "content", "concept",
        )];
        let (_dir, index) = create_test_index_with_docs(&docs);

        let result = search_index(&index, "   ", 10);
        assert!(result.is_err());
    }

    #[test]
    fn test_search_index_null_bytes_returns_error() {
        let docs = vec![make_index_document(
            "doc1", "Test", "Test", "content", "concept",
        )];
        let (_dir, index) = create_test_index_with_docs(&docs);

        let result = search_index(&index, "test\0query", 10);
        assert!(result.is_err());
    }

    #[test]
    fn test_search_index_results_sorted_by_score() {
        let docs = vec![
            make_index_document(
                "doc1",
                "Rust Programming Language",
                "All about Rust",
                "Rust programming language systems",
                "tutorial",
            ),
            make_index_document(
                "doc2",
                "Some Document",
                "Unrelated",
                " mentions rust once briefly",
                "concept",
            ),
        ];
        let (_dir, index) = create_test_index_with_docs(&docs);

        let results = search_index(&index, "Rust programming", 10).unwrap();
        assert!(results.len() >= 2);
        for i in 0..results.len().saturating_sub(1) {
            assert!(
                results[i].score >= results[i + 1].score,
                "Results should be sorted by descending score"
            );
        }
    }

    #[test]
    fn test_search_result_fields() {
        let docs = vec![make_index_document(
            "doc1",
            "Test Title",
            "Test Summary",
            "Test content",
            "ref",
        )];
        let (_dir, index) = create_test_index_with_docs(&docs);

        let results = search_index(&index, "Test", 10).unwrap();
        assert_eq!(results.len(), 1);
        let r = &results[0];
        assert_eq!(r.id, "doc1");
        assert_eq!(r.title, "Test Title");
        assert_eq!(r.summary, "Test Summary");
        assert_eq!(r.category, "ref");
        assert!(r.path.contains("doc1"));
    }

    #[test]
    fn test_search_index_with_chunks() {
        let docs = vec![make_index_document(
            "doc1",
            "Test Doc",
            "Summary",
            "Original content",
            "tutorial",
        )];
        let chunks = vec![
            make_chunk(
                "doc1#0-standard",
                "doc1",
                "Test Doc",
                "Chunk about rust programming patterns",
                Some("Rust Patterns"),
            ),
            make_chunk(
                "doc1#1-standard",
                "doc1",
                "Test Doc",
                "Chunk about python scripting",
                Some("Python Scripts"),
            ),
        ];

        let dir = TempDir::new().unwrap();
        let index = open_or_create_index(dir.path()).unwrap();
        let mut writer = index.writer(50_000_000).unwrap();
        index_documents(&mut writer, &docs).unwrap();
        index_chunks(&mut writer, &docs, &chunks).unwrap();
        writer.commit().unwrap();

        let results = search_index(&index, "rust patterns", 10).unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_escape_tantivy_query_no_special_chars() {
        assert_eq!(escape_tantivy_query("hello world"), "hello world");
    }

    #[test]
    fn test_escape_tantivy_query_wildcard_star() {
        assert_eq!(escape_tantivy_query("test*"), "test\\*");
    }

    #[test]
    fn test_escape_tantivy_query_wildcard_question() {
        assert_eq!(escape_tantivy_query("test?"), "test\\?");
    }

    #[test]
    fn test_escape_tantivy_query_multiple_special() {
        assert_eq!(escape_tantivy_query("a*b?c*d"), "a\\*b\\?c\\*d");
    }

    #[test]
    fn test_escape_tantivy_query_empty() {
        assert_eq!(escape_tantivy_query(""), "");
    }

    #[test]
    fn test_open_existing_index_no_dir() {
        let dir = TempDir::new().unwrap();
        let result = open_existing_index(dir.path()).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_open_existing_index_valid() {
        let dir = TempDir::new().unwrap();
        let _index = open_or_create_index(dir.path()).unwrap();

        let result = open_existing_index(dir.path()).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_open_existing_index_file_instead_of_dir() {
        let dir = TempDir::new().unwrap();
        let index_file = dir.path().join(".tantivy_index");
        fs::write(&index_file, "not a directory").unwrap();

        let result = open_existing_index(dir.path()).unwrap();
        assert!(result.is_none());
        assert!(!index_file.exists(), "File should have been removed");
    }

    #[test]
    fn test_open_existing_index_corrupted() {
        let dir = TempDir::new().unwrap();
        let index_dir = dir.path().join(".tantivy_index");
        fs::create_dir_all(&index_dir).unwrap();
        fs::write(index_dir.join("managed.json"), "corrupted garbage data").unwrap();

        let result = open_existing_index(dir.path()).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_rebuild_index_from_json_missing_file() {
        let dir = TempDir::new().unwrap();
        let result = rebuild_index_from_json(dir.path());
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to read INDEX.json"));
    }

    #[test]
    fn test_rebuild_index_from_json_invalid_json() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("INDEX.json"), "not valid json").unwrap();

        let result = rebuild_index_from_json(dir.path());
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to parse INDEX.json"));
    }

    #[test]
    fn test_rebuild_index_from_json_missing_documents() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("INDEX.json"), r#"{"chunks": []}"#).unwrap();

        let result = rebuild_index_from_json(dir.path());
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("missing documents array"));
    }

    #[test]
    fn test_rebuild_index_from_json_missing_chunks() {
        let dir = TempDir::new().unwrap();
        fs::write(
            dir.path().join("INDEX.json"),
            r#"{"documents": [{"id": "doc1", "title": "Test"}]}"#,
        )
        .unwrap();

        let result = rebuild_index_from_json(dir.path());
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("missing chunks array"));
    }

    #[test]
    fn test_rebuild_index_from_json_empty_arrays() {
        let dir = TempDir::new().unwrap();
        fs::write(
            dir.path().join("INDEX.json"),
            r#"{"documents": [], "chunks": []}"#,
        )
        .unwrap();

        let index = rebuild_index_from_json(dir.path()).unwrap();
        assert!(dir.path().join(".tantivy_index").exists());

        let reader = index.reader().unwrap();
        let searcher = reader.searcher();
        assert_eq!(searcher.num_docs(), 0);
    }

    #[test]
    fn test_rebuild_index_from_json_with_documents_and_chunks() {
        let dir = TempDir::new().unwrap();

        let chunks_dir = dir.path().join("chunks");
        fs::create_dir_all(&chunks_dir).unwrap();
        fs::write(
            chunks_dir.join("doc1-0-standard.md"),
            "Chunk content about Rust programming",
        )
        .unwrap();

        let index_json = r#"{
            "documents": [
                {
                    "id": "doc1",
                    "title": "Rust Guide",
                    "summary": "Learn Rust",
                    "path": "docs/rust.md",
                    "category": "tutorial",
                    "tags": ["rust"],
                    "word_count": 100,
                    "chunk_ids": ["doc1#0"],
                    "headings": ["Introduction"],
                    "content": "Rust programming language"
                }
            ],
            "chunks": [
                {
                    "chunk_id": "doc1#0",
                    "doc_id": "doc1",
                    "doc_title": "Rust Guide",
                    "summary": "Chunk summary",
                    "token_count": 50,
                    "heading": "Introduction",
                    "chunk_level": "standard"
                }
            ]
        }"#;

        fs::write(dir.path().join("INDEX.json"), index_json).unwrap();

        let index = rebuild_index_from_json(dir.path()).unwrap();
        let reader = index.reader().unwrap();
        let searcher = reader.searcher();
        assert_eq!(searcher.num_docs(), 1);
    }

    #[test]
    fn test_rebuild_index_from_json_chunk_with_frontmatter() {
        let dir = TempDir::new().unwrap();

        let chunks_dir = dir.path().join("chunks");
        fs::create_dir_all(&chunks_dir).unwrap();
        let chunk_content = "---\ntitle: Chunk Title\n---\nActual chunk content here";
        fs::write(chunks_dir.join("doc1-0-standard.md"), chunk_content).unwrap();

        let index_json = r#"{
            "documents": [
                {
                    "id": "doc1",
                    "title": "Doc",
                    "summary": "Summary",
                    "path": "docs/doc.md",
                    "category": "concept",
                    "tags": [],
                    "word_count": 50,
                    "chunk_ids": ["doc1#0"],
                    "headings": [],
                    "content": "Content"
                }
            ],
            "chunks": [
                {
                    "chunk_id": "doc1#0",
                    "doc_id": "doc1",
                    "doc_title": "Doc",
                    "summary": "Chunk summary",
                    "token_count": 30,
                    "heading": null,
                    "chunk_level": "standard"
                }
            ]
        }"#;

        fs::write(dir.path().join("INDEX.json"), index_json).unwrap();

        let index = rebuild_index_from_json(dir.path()).unwrap();
        let reader = index.reader().unwrap();
        let searcher = reader.searcher();
        assert_eq!(searcher.num_docs(), 1);
    }

    #[test]
    fn test_rebuild_index_from_json_chunk_file_missing() {
        let dir = TempDir::new().unwrap();

        let index_json = r#"{
            "documents": [
                {
                    "id": "doc1",
                    "title": "Doc",
                    "summary": "Summary",
                    "path": "docs/doc.md",
                    "category": "concept",
                    "tags": [],
                    "word_count": 50,
                    "chunk_ids": ["doc1#0"],
                    "headings": [],
                    "content": "Content"
                }
            ],
            "chunks": [
                {
                    "chunk_id": "doc1#0",
                    "doc_id": "doc1",
                    "doc_title": "Doc",
                    "summary": "Chunk summary",
                    "token_count": 30,
                    "heading": null,
                    "chunk_level": "standard"
                }
            ]
        }"#;

        fs::write(dir.path().join("INDEX.json"), index_json).unwrap();

        let index = rebuild_index_from_json(dir.path()).unwrap();
        let reader = index.reader().unwrap();
        let searcher = reader.searcher();
        assert_eq!(
            searcher.num_docs(),
            0,
            "Missing chunk files should be skipped"
        );
    }

    #[test]
    fn test_rebuild_index_from_json_summary_chunk_level() {
        let dir = TempDir::new().unwrap();

        let chunks_dir = dir.path().join("chunks");
        fs::create_dir_all(&chunks_dir).unwrap();
        fs::write(
            chunks_dir.join("doc1-0-summary.md"),
            "Summary chunk content",
        )
        .unwrap();

        let index_json = r#"{
            "documents": [],
            "chunks": [
                {
                    "chunk_id": "doc1#0",
                    "doc_id": "doc1",
                    "doc_title": "Doc",
                    "summary": "Sum",
                    "token_count": 20,
                    "heading": null,
                    "chunk_level": "summary"
                }
            ]
        }"#;

        fs::write(dir.path().join("INDEX.json"), index_json).unwrap();

        let index = rebuild_index_from_json(dir.path()).unwrap();
        let reader = index.reader().unwrap();
        let searcher = reader.searcher();
        assert_eq!(searcher.num_docs(), 1);
    }

    #[test]
    fn test_rebuild_index_from_json_detailed_chunk_level() {
        let dir = TempDir::new().unwrap();

        let chunks_dir = dir.path().join("chunks");
        fs::create_dir_all(&chunks_dir).unwrap();
        fs::write(
            chunks_dir.join("doc1-0-detailed.md"),
            "Detailed chunk content",
        )
        .unwrap();

        let index_json = r#"{
            "documents": [],
            "chunks": [
                {
                    "chunk_id": "doc1#0",
                    "doc_id": "doc1",
                    "doc_title": "Doc",
                    "summary": "Det",
                    "token_count": 20,
                    "heading": null,
                    "chunk_level": "detailed"
                }
            ]
        }"#;

        fs::write(dir.path().join("INDEX.json"), index_json).unwrap();

        let index = rebuild_index_from_json(dir.path()).unwrap();
        let reader = index.reader().unwrap();
        let searcher = reader.searcher();
        assert_eq!(searcher.num_docs(), 1);
    }

    #[test]
    fn test_rebuild_index_from_json_doc_with_minimal_fields() {
        let dir = TempDir::new().unwrap();

        let chunks_dir = dir.path().join("chunks");
        fs::create_dir_all(&chunks_dir).unwrap();
        fs::write(chunks_dir.join("id-standard.md"), "chunk text").unwrap();

        let index_json = r#"{
            "documents": [
                {"id": "minimal"}
            ],
            "chunks": [
                {
                    "chunk_id": "id",
                    "doc_id": "",
                    "doc_title": "",
                    "summary": "",
                    "token_count": 0,
                    "heading": null,
                    "chunk_level": "standard"
                }
            ]
        }"#;

        fs::write(dir.path().join("INDEX.json"), index_json).unwrap();

        let index = rebuild_index_from_json(dir.path()).unwrap();
        let reader = index.reader().unwrap();
        let searcher = reader.searcher();
        assert_eq!(searcher.num_docs(), 1);
    }

    #[test]
    fn test_open_or_create_index_recovers_from_corrupted_index_with_json() {
        let dir = TempDir::new().unwrap();
        let index_dir = dir.path().join(".tantivy_index");
        fs::create_dir_all(&index_dir).unwrap();
        fs::write(index_dir.join("managed.json"), "corrupted").unwrap();

        fs::write(
            dir.path().join("INDEX.json"),
            r#"{"documents": [], "chunks": []}"#,
        )
        .unwrap();

        let _index = open_or_create_index(dir.path()).unwrap();
        assert!(index_dir.exists());
        assert!(index_dir.is_dir());
    }

    #[test]
    fn test_open_or_create_index_corrupted_no_json_falls_back() {
        let dir = TempDir::new().unwrap();
        let index_dir = dir.path().join(".tantivy_index");
        fs::create_dir_all(&index_dir).unwrap();
        fs::write(index_dir.join("managed.json"), "corrupted").unwrap();

        let _index = open_or_create_index(dir.path()).unwrap();
        assert!(index_dir.is_dir());
    }

    #[test]
    fn test_search_error_types() {
        let _ = IndexerError::DirectoryAccessFailed("test".to_string());
        let _ = IndexerError::IndexCommitFailed("test".to_string());
        let _ = IndexerError::InvalidDocument;
        let _ = IndexerError::UncommittedChanges;
        let _ = IndexerError::Other(anyhow!("wrapped"));

        let _ = SearchError::EmptyQuery;
        let _ = SearchError::QueryParseError("bad".to_string());
        let _ = SearchError::PostconditionViolated;
        let _ = SearchError::Other(anyhow!("wrapped"));
    }

    #[test]
    fn test_search_result_debug_clone() {
        let result = SearchResult {
            id: "test-id".to_string(),
            title: "Test Title".to_string(),
            summary: "Test Summary".to_string(),
            category: "tutorial".to_string(),
            score: crate::math_types::Score::zero(),
            path: "docs/test.md".to_string(),
        };

        let _cloned = result.clone();
        let debug = format!("{result:?}");
        assert!(debug.contains("test-id"));
    }

    #[test]
    fn test_chunk_with_special_chars_in_id() {
        let docs = vec![make_index_document(
            "doc/a#1",
            "Special ID",
            "Summary",
            "content",
            "concept",
        )];
        let mut chunk = make_chunk(
            "doc/a#1-0-standard",
            "doc/a#1",
            "Special ID",
            "chunk content",
            None,
        );
        chunk.chunk_id = "doc/a#1".to_string();

        let dir = TempDir::new().unwrap();
        let index = open_or_create_index(dir.path()).unwrap();
        let mut writer = index.writer(50_000_000).unwrap();
        index_chunks(&mut writer, &docs, &[chunk]).unwrap();
        writer.commit().unwrap();
    }
}
