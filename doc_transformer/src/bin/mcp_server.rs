#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

//! MCP (Model Context Protocol) server for AI documentation queries
//!
//! Exposes documentation search via JSON-RPC over stdio following the MCP specification.
//!
//! ## Tools Provided
//!
//! - `search_docs`: Search documentation using Tantivy full-text search
//! - `get_chunk`: Retrieve a specific chunk by ID with context
//! - `list_docs`: List all available documents with metadata
//!
//! ## Architecture
//!
//! - **Functional Core**: Pure domain logic (search, validation)
//! - **Imperative Shell**: I/O operations (stdio, file reading)
//! - **Error Handling**: Railway-oriented programming with Result combinators

use anyhow::Result;
use doc_transformer::index::{ChunkMetadata, IndexDocument};
use doc_transformer::search;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::path::Path;
use thiserror::Error;

// ============================================================================
// DOMAIN TYPES (Functional Core)
// ============================================================================

/// MCP protocol errors (semantic, enumerable)
#[derive(Debug, Error)]
pub enum McpError {
    #[error("INDEX.json not found at path: {0}")]
    IndexNotFound(String),

    #[error("failed to parse INDEX.json: {0}")]
    InvalidIndex(String),

    #[error("unknown method: {0}")]
    UnknownMethod(String),

    #[error("invalid request: {0}")]
    InvalidRequest(String),

    #[error("search error: {0}")]
    SearchError(String),

    #[error("chunk not found: {0}")]
    ChunkNotFound(String),

    #[error("I/O error: {0}")]
    IoError(String),

    #[error("JSON serialization error: {0}")]
    JsonError(String),
}

/// Document index loaded from INDEX.json
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentIndex {
    pub documents: Vec<IndexDocument>,
    pub chunks: Vec<ChunkMetadata>,
    #[serde(default)]
    pub keywords: HashMap<String, Vec<String>>,
}

/// MCP JSON-RPC request
#[derive(Debug, Deserialize)]
pub struct McpRequest {
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

/// Tool call parameters for search_docs
#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub query: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    10
}

/// Tool call parameters for get_chunk
#[derive(Debug, Deserialize)]
pub struct GetChunkParams {
    pub chunk_id: String,
}

// ============================================================================
// FUNCTIONAL CORE (Pure Logic)
// ============================================================================

/// Load and parse INDEX.json from disk
fn load_index(index_path: &Path) -> Result<DocumentIndex, McpError> {
    std::fs::read_to_string(index_path)
        .map_err(|e| McpError::IndexNotFound(format!("{}: {}", index_path.display(), e)))
        .and_then(|content| {
            serde_json::from_str::<Value>(&content)
                .map_err(|e| McpError::InvalidIndex(e.to_string()))
        })
        .and_then(|index_json| {
            // Extract documents, chunks, keywords from the index structure
            let documents = index_json["documents"]
                .as_array()
                .ok_or_else(|| McpError::InvalidIndex("missing 'documents' field".to_string()))?
                .iter()
                .map(|v| serde_json::from_value(v.clone()))
                .collect::<Result<Vec<IndexDocument>, _>>()
                .map_err(|e| McpError::InvalidIndex(format!("invalid document: {}", e)))?;

            let chunks = index_json["chunks"]
                .as_array()
                .ok_or_else(|| McpError::InvalidIndex("missing 'chunks' field".to_string()))?
                .iter()
                .map(|v| serde_json::from_value(v.clone()))
                .collect::<Result<Vec<ChunkMetadata>, _>>()
                .map_err(|e| McpError::InvalidIndex(format!("invalid chunk: {}", e)))?;

            let keywords = index_json
                .get("keywords")
                .and_then(|v| serde_json::from_value(v.clone()).ok())
                .unwrap_or_default();

            Ok(DocumentIndex {
                documents,
                chunks,
                keywords,
            })
        })
}

/// Search documents using Tantivy index (fallback to simple search on error)
fn search_documents(
    index_dir: &Path,
    query: &str,
    limit: usize,
    fallback_docs: &[IndexDocument],
) -> Result<Value, McpError> {
    // Validate query using centralized validation
    let query = doc_transformer::validate::validate_query(query)
        .map_err(|e| McpError::SearchError(e.to_string()))?;

    // Try Tantivy search first
    let tantivy_results = search::open_or_create_index(index_dir)
        .ok()
        .and_then(|idx| search::search_index(&idx, query, limit).ok())
        .filter(|results| !results.is_empty());

    let results = tantivy_results
        .map(|search_results| {
            search_results
                .into_iter()
                .map(|r| {
                    json!({
                        "id": r.id,
                        "title": r.title,
                        "summary": r.summary,
                        "category": r.category,
                        "score": r.score,
                        "path": r.path
                    })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| {
            // Fallback: simple text matching on documents
            fallback_docs
                .iter()
                .filter(|doc| {
                    let query_lower = query.to_lowercase();
                    doc.title.to_lowercase().contains(&query_lower)
                        || doc.summary.to_lowercase().contains(&query_lower)
                        || doc.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
                })
                .take(limit)
                .map(|doc| {
                    json!({
                        "id": doc.id,
                        "title": doc.title,
                        "summary": doc.summary,
                        "category": doc.category,
                        "score": 1.0,
                        "path": doc.path
                    })
                })
                .collect::<Vec<_>>()
        });

    Ok(json!({ "results": results }))
}

/// Find a chunk by ID with navigation context
fn find_chunk(chunk_id: &str, chunks: &[ChunkMetadata]) -> Result<Value, McpError> {
    chunks
        .iter()
        .find(|c| c.chunk_id == chunk_id)
        .map(|chunk| {
            json!({
                "chunk_id": chunk.chunk_id,
                "doc_id": chunk.doc_id,
                "doc_title": chunk.doc_title,
                "heading": chunk.heading,
                "chunk_type": chunk.chunk_type,
                "token_count": chunk.token_count,
                "summary": chunk.summary,
                "path": chunk.path,
                "previous_chunk_id": chunk.previous_chunk_id,
                "next_chunk_id": chunk.next_chunk_id,
                "related_chunks": chunk.related_chunks,
                "chunk_level": chunk.chunk_level,
                "parent_chunk_id": chunk.parent_chunk_id,
                "child_chunk_ids": chunk.child_chunk_ids
            })
        })
        .ok_or_else(|| McpError::ChunkNotFound(chunk_id.to_string()))
}

/// List all documents with basic metadata
fn list_all_documents(documents: &[IndexDocument]) -> Value {
    let doc_list = documents
        .iter()
        .map(|doc| {
            json!({
                "id": doc.id,
                "title": doc.title,
                "category": doc.category,
                "tags": doc.tags,
                "word_count": doc.word_count,
                "chunk_count": doc.chunk_ids.len()
            })
        })
        .collect::<Vec<_>>();

    json!({ "documents": doc_list, "total": documents.len() })
}

/// Generate MCP tools/list response
fn generate_tools_list() -> Value {
    json!({
        "tools": [
            {
                "name": "search_docs",
                "description": "Search documentation using full-text search with BM25 ranking",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "Search query (supports phrases and boolean operators)"
                        },
                        "limit": {
                            "type": "number",
                            "description": "Maximum number of results (default: 10)",
                            "default": 10
                        }
                    },
                    "required": ["query"]
                }
            },
            {
                "name": "get_chunk",
                "description": "Retrieve a specific chunk by ID with navigation context",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "chunk_id": {
                            "type": "string",
                            "description": "Chunk identifier"
                        }
                    },
                    "required": ["chunk_id"]
                }
            },
            {
                "name": "list_docs",
                "description": "List all available documents with metadata",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            }
        ]
    })
}

/// Route MCP request to appropriate handler
fn handle_request(req: &McpRequest, index: &DocumentIndex, index_dir: &Path) -> Result<Value, McpError> {
    match req.method.as_str() {
        "tools/list" => Ok(generate_tools_list()),
        "tools/call" => {
            let tool_name = req.params["name"]
                .as_str()
                .ok_or_else(|| McpError::InvalidRequest("missing tool name".to_string()))?;

            let arguments = &req.params["arguments"];

            match tool_name {
                "search_docs" => {
                    let params: SearchParams = serde_json::from_value(arguments.clone())
                        .map_err(|e| McpError::InvalidRequest(format!("invalid search params: {}", e)))?;

                    search_documents(index_dir, &params.query, params.limit, &index.documents)
                }
                "get_chunk" => {
                    let params: GetChunkParams = serde_json::from_value(arguments.clone())
                        .map_err(|e| McpError::InvalidRequest(format!("invalid get_chunk params: {}", e)))?;

                    find_chunk(&params.chunk_id, &index.chunks)
                }
                "list_docs" => Ok(list_all_documents(&index.documents)),
                _ => Err(McpError::UnknownMethod(tool_name.to_string())),
            }
        }
        method => Err(McpError::UnknownMethod(method.to_string())),
    }
}

/// Format error as JSON-RPC error response
fn format_error(error: McpError) -> Value {
    json!({
        "error": {
            "code": -32603,
            "message": error.to_string()
        }
    })
}

// ============================================================================
// IMPERATIVE SHELL (Side Effects)
// ============================================================================

/// Main MCP server loop (stdio JSON-RPC)
fn run_server() -> Result<(), McpError> {
    let index_path = Path::new("indexed_output/INDEX.json");
    let index_dir = Path::new("indexed_output");

    // Load index once at startup
    let index = load_index(index_path)?;

    eprintln!("MCP server started. Loaded {} documents, {} chunks",
             index.documents.len(),
             index.chunks.len());

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    // Process requests line-by-line
    for line in stdin.lock().lines() {
        let line = line.map_err(|e| McpError::IoError(e.to_string()))?;

        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // Parse request
        let request: McpRequest = serde_json::from_str(&line)
            .map_err(|e| McpError::InvalidRequest(format!("invalid JSON: {}", e)))?;

        // Handle request (functional core)
        let response = handle_request(&request, &index, index_dir)
            .unwrap_or_else(|e| format_error(e));

        // Write response
        serde_json::to_writer(&mut stdout, &response)
            .map_err(|e| McpError::JsonError(e.to_string()))?;

        stdout
            .write_all(b"\n")
            .map_err(|e| McpError::IoError(e.to_string()))?;

        stdout
            .flush()
            .map_err(|e| McpError::IoError(e.to_string()))?;
    }

    Ok(())
}

fn main() {
    if let Err(e) = run_server() {
        eprintln!("MCP server error: {}", e);
        std::process::exit(1);
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_limit() {
        assert_eq!(default_limit(), 10);
    }

    #[test]
    fn test_format_error() {
        let error = McpError::UnknownMethod("test_method".to_string());
        let formatted = format_error(error);

        assert_eq!(formatted["error"]["code"], -32603);
        assert!(formatted["error"]["message"]
            .as_str()
            .map_or(false, |s| s.contains("unknown method")));
    }

    #[test]
    fn test_generate_tools_list() {
        let tools = generate_tools_list();
        let tools_array = tools["tools"].as_array();

        assert!(tools_array.is_some());
        assert_eq!(tools_array.map(Vec::len), Some(3));
    }

    #[test]
    fn test_list_all_documents_empty() {
        let docs: Vec<IndexDocument> = vec![];
        let result = list_all_documents(&docs);

        assert_eq!(result["total"], 0);
        assert_eq!(result["documents"].as_array().map(Vec::len), Some(0));
    }
}
