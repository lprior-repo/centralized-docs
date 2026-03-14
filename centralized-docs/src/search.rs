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
            let title = doc["title"].as_str().unwrap_or("");
            let summary = doc["summary"].as_str().unwrap_or("");
            let path = doc["path"].as_str().unwrap_or("");
            let category = doc["category"].as_str().unwrap_or("");
            let word_count = doc["word_count"].as_u64().unwrap_or(0) as usize;
            let tags: Vec<String> = doc["tags"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();
            let chunk_ids: Vec<String> = doc["chunk_ids"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();
            let headings: Vec<String> = doc["headings"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();
            let content = doc["content"].as_str().unwrap_or("").to_string();

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

    let mut chunks = Vec::with_capacity(chunks_val.len());
    for chunk in chunks_val {
        let Some(chunk_id) = chunk["chunk_id"].as_str() else {
            continue;
        };
        let doc_id = chunk["doc_id"].as_str().unwrap_or("");
        let doc_title = chunk["doc_title"].as_str().unwrap_or("");
        let summary = chunk["summary"].as_str().unwrap_or("");
        let token_count = chunk["token_count"].as_u64().unwrap_or(0) as usize;
        let heading = chunk["heading"].as_str().map(String::from);

        // Reconstruct chunk level
        let level_str = chunk["chunk_level"].as_str().unwrap_or("standard");
        let chunk_level = match level_str {
            "summary" => contextual_chunker::ChunkLevel::Summary,
            "detailed" => contextual_chunker::ChunkLevel::Detailed,
            _ => contextual_chunker::ChunkLevel::Standard,
        };

        // Read the content from the file system
        let chunk_filename = format!("{}-{}.md", chunk_id.replace(['/', '#'], "-"), level_str);
        let chunk_path = index_path.join("chunks").join(&chunk_filename);

        let raw_content = fs::read_to_string(&chunk_path).map_err(|e| {
            anyhow::anyhow!("Failed to read chunk file {}: {}", chunk_path.display(), e)
        })?;

        // Remove frontmatter if present (robust parsing that handles \r\n and closing tags)
        let content = if raw_content.starts_with("---\n") || raw_content.starts_with("---\r\n") {
            let mut lines = raw_content.lines();
            lines.next(); // skip opening ---

            let mut found_end = false;
            while let Some(line) = lines.next() {
                if line.trim_end() == "---" {
                    found_end = true;
                    break;
                }
            }

            if found_end {
                lines.collect::<Vec<_>>().join("\n")
            } else {
                raw_content
            }
        } else {
            raw_content
        };

        chunks.push(crate::chunking_adapter::Chunk {
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
        });
    }

    let index_dir = index_path.join(".tantivy_index");
    fs::create_dir_all(&index_dir)?;
    let (schema, _fields) = create_schema();
    let index = Index::create_in_dir(&index_dir, schema)
        .map_err(|e| anyhow!("Failed to create index: {e}"))?;

    if !chunks.is_empty() {
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
    let mut query_parser = QueryParser::for_index(index, vec![fields.title, fields.content]);
    query_parser.set_field_boost(fields.title, 3.0); // Boost title matches significantly

    let query = query_parser
        .parse_query(&escaped_query)
        .map_err(|e| SearchError::QueryParseError(format!("Invalid query: {e}")))?;

    // Execute search and get top results
    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(limit))
        .map_err(|e| SearchError::Other(anyhow::anyhow!(e)))?;

    // Extract stored fields from results
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
                    .unwrap_or_else(|| "unknown".to_string());

                let title = retrieved_doc
                    .get_first(fields.title)
                    .map(tantivy::schema::OwnedValue::from)
                    .and_then(|v| v.as_ref().as_str().map(std::string::ToString::to_string))
                    .unwrap_or_else(|| "Untitled".to_string());

                let summary = retrieved_doc
                    .get_first(fields.summary)
                    .map(tantivy::schema::OwnedValue::from)
                    .and_then(|v| v.as_ref().as_str().map(std::string::ToString::to_string))
                    .unwrap_or_else(|| "No summary available".to_string());

                let category = retrieved_doc
                    .get_first(fields.category)
                    .map(tantivy::schema::OwnedValue::from)
                    .and_then(|v| v.as_ref().as_str().map(std::string::ToString::to_string))
                    .unwrap_or_else(|| "uncategorized".to_string());

                let _word_count = retrieved_doc
                    .get_first(fields.word_count)
                    .map(tantivy::schema::OwnedValue::from)
                    .and_then(|v| v.as_ref().as_u64())
                    .unwrap_or(0);

                let path = retrieved_doc
                    .get_first(fields.path)
                    .map(tantivy::schema::OwnedValue::from)
                    .and_then(|v| v.as_ref().as_str().map(std::string::ToString::to_string))
                    .unwrap_or_else(|| format!("docs/{}.md", id.replace('/', "-")));

                let score = crate::math_types::Score::try_new(tantivy_score)
                    .unwrap_or_else(|_| crate::math_types::Score::zero());

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
mod tests {
    use super::*;
    use tempfile::TempDir;

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

        // Create index
        let _index1 = open_or_create_index(index_path)?;

        // Verify we can open the same index again
        let _index2 = open_or_create_index(index_path)?;

        // Both should refer to the same files
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
}
