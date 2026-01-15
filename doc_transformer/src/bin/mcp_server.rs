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
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentIndex {
    pub documents: Vec<IndexDocument>,
    pub chunks: Vec<ChunkMetadata>,
    #[serde(default)]
    pub keywords: HashMap<String, Vec<String>>,
}

// Note: Caching and metrics infrastructure removed in v6.0 cleanup.
// The server loads the index once at startup, making per-request caching unnecessary.
// Metrics can be re-added in v8.0 if monitoring becomes a requirement.

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

/// Tool call parameters for find_related
#[derive(Debug, Deserialize)]
pub struct FindRelatedParams {
    pub chunk_id: String,
    #[serde(default = "default_relationship_type")]
    pub relationship_type: String, // "similar" | "sequential" | "hierarchical"
    #[serde(default = "default_max_depth")]
    pub max_depth: usize,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_relationship_type() -> String {
    "similar".to_string()
}

fn default_max_depth() -> usize {
    2
}

/// Tool call parameters for get_document
#[derive(Debug, Deserialize)]
pub struct GetDocumentParams {
    pub doc_id: String,
    #[serde(default = "default_include_chunks")]
    pub include_chunks: bool,
    #[serde(default)]
    pub chunk_level: Option<String>, // "standard" | "summary" | "detailed"
}

fn default_include_chunks() -> bool {
    true
}

/// Tool call parameters for semantic_search
#[derive(Debug, Deserialize)]
pub struct SemanticSearchParams {
    pub query: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default = "default_threshold")]
    pub threshold: f32,
}

fn default_threshold() -> f32 {
    0.7
}

/// Tool call parameters for search_by_category
#[derive(Debug, Deserialize)]
pub struct SearchByCategoryParams {
    pub category: String,
    pub query: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

/// Tool call parameters for search_by_tags
#[derive(Debug, Deserialize)]
pub struct SearchByTagsParams {
    pub tags: Vec<String>,
    #[serde(default = "default_match_mode")]
    pub match_mode: String, // "all" | "any"
    pub query: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_match_mode() -> String {
    "any".to_string()
}

/// Tool call parameters for get_navigation
#[derive(Debug, Deserialize)]
pub struct GetNavigationParams {
    #[serde(default = "default_format")]
    pub format: String, // "hierarchical" | "flat"
}

fn default_format() -> String {
    "hierarchical".to_string()
}

/// Tool call parameters for explain_chunk
#[derive(Debug, Deserialize)]
pub struct ExplainChunkParams {
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

/// Load index with caching (avoids repeated INDEX.json reads)
fn load_index_with_cache(index_path: &Path, cache: &IndexCache) -> Result<DocumentIndex, McpError> {
    let path_str = index_path.to_string_lossy().to_string();

    // Try to get from cache first
    {
        let cache_read = cache.read().map_err(|e| McpError::IoError(format!("cache lock error: {}", e)))?;

        if let Some(cached) = cache_read.get(&path_str) {
            if cached.is_fresh() {
                return Ok(cached.index.clone());
            }
        }
    }

    // Cache miss or stale - load fresh index
    let index = load_index(index_path)?;

    // Update cache
    {
        let mut cache_write = cache.write().map_err(|e| McpError::IoError(format!("cache lock error: {}", e)))?;

        cache_write.insert(
            path_str,
            CachedIndex {
                index: index.clone(),
                loaded_at: SystemTime::now(),
                index_path: index_path.to_path_buf(),
            },
        );
    }

    Ok(index)
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

/// Find related chunks by traversing relationships
fn find_related(
    chunk_id: &str,
    relationship_type: &str,
    max_depth: usize,
    limit: usize,
    chunks: &[ChunkMetadata],
) -> Result<Value, McpError> {
    // Find the starting chunk
    let _start_chunk = chunks
        .iter()
        .find(|c| c.chunk_id == chunk_id)
        .ok_or_else(|| McpError::ChunkNotFound(chunk_id.to_string()))?;

    let mut relationships = Vec::new();
    let mut visited = std::collections::HashSet::new();
    visited.insert(chunk_id.to_string());

    // Helper function to traverse relationships
    fn traverse(
        current_id: &str,
        depth: usize,
        max_depth: usize,
        relationship_type: &str,
        chunks: &[ChunkMetadata],
        visited: &mut std::collections::HashSet<String>,
        relationships: &mut Vec<Value>,
        limit: usize,
    ) {
        if depth >= max_depth || relationships.len() >= limit {
            return;
        }

        if let Some(current) = chunks.iter().find(|c| c.chunk_id == current_id) {
            let next_ids: Vec<String> = match relationship_type {
                "sequential" => {
                    current.next_chunk_id.clone().into_iter().collect()
                }
                "hierarchical" => {
                    current.child_chunk_ids.clone()
                }
                "similar" | _ => {
                    current.related_chunks.iter().map(|r| r.chunk_id.clone()).collect()
                }
            };

            for next_id in next_ids {
                if !visited.contains(&next_id) && relationships.len() < limit {
                    visited.insert(next_id.clone());

                    if let Some(next_chunk) = chunks.iter().find(|c| c.chunk_id == next_id) {
                        relationships.push(json!({
                            "chunk_id": next_chunk.chunk_id,
                            "relationship": relationship_type,
                            "score": 1.0, // Default score
                            "path": vec![current_id, &next_id],
                            "doc_title": next_chunk.doc_title,
                            "heading": next_chunk.heading
                        }));

                        traverse(
                            &next_id,
                            depth + 1,
                            max_depth,
                            relationship_type,
                            chunks,
                            visited,
                            relationships,
                            limit,
                        );
                    }
                }
            }
        }
    }

    traverse(
        chunk_id,
        0,
        max_depth,
        relationship_type,
        chunks,
        &mut visited,
        &mut relationships,
        limit,
    );

    Ok(json!({ "relationships": relationships }))
}

/// Get full document with optional chunks
fn get_document(
    doc_id: &str,
    include_chunks: bool,
    chunk_level: Option<&str>,
    documents: &[IndexDocument],
    chunks: &[ChunkMetadata],
) -> Result<Value, McpError> {
    let doc = documents
        .iter()
        .find(|d| d.id == doc_id)
        .ok_or_else(|| McpError::ChunkNotFound(format!("document not found: {}", doc_id)))?;

    let mut result = json!({
        "doc_id": doc.id,
        "title": doc.title,
        "category": doc.category,
        "tags": doc.tags,
        "word_count": doc.word_count,
        "path": doc.path
    });

    if include_chunks {
        let doc_chunks: Vec<Value> = chunks
            .iter()
            .filter(|c| c.doc_id == doc_id)
            .filter(|c| {
                if let Some(level) = chunk_level {
                    c.chunk_level == level
                } else {
                    true
                }
            })
            .map(|c| {
                json!({
                    "chunk_id": c.chunk_id,
                    "heading": c.heading,
                    "content": "",  // Content not stored in metadata
                    "token_count": c.token_count,
                    "chunk_level": c.chunk_level
                })
            })
            .collect();

        result["chunks"] = json!(doc_chunks);
    }

    Ok(result)
}

/// Semantic search (stub for now - requires vector embeddings)
fn semantic_search(
    query: &str,
    limit: usize,
    _threshold: f32,
    chunks: &[ChunkMetadata],
) -> Result<Value, McpError> {
    // For now, fallback to simple text matching on summaries
    // TODO: Implement true vector-based semantic search in v8.0
    let results: Vec<Value> = chunks
        .iter()
        .filter(|c| {
            let query_lower = query.to_lowercase();
            c.summary.to_lowercase().contains(&query_lower)
                || c.heading.as_ref().map_or(false, |h| h.to_lowercase().contains(&query_lower))
        })
        .take(limit)
        .map(|c| {
            json!({
                "chunk_id": c.chunk_id,
                "doc_title": c.doc_title,
                "heading": c.heading,
                "similarity_score": 0.85, // Stub score
                "excerpt": c.summary
            })
        })
        .collect();

    Ok(json!({ "results": results }))
}

/// Search documents by category
fn search_by_category(
    category: &str,
    query: &str,
    limit: usize,
    documents: &[IndexDocument],
) -> Result<Value, McpError> {
    let query_lower = query.to_lowercase();
    let results: Vec<Value> = documents
        .iter()
        .filter(|doc| doc.category == category)
        .filter(|doc| {
            doc.title.to_lowercase().contains(&query_lower)
                || doc.summary.to_lowercase().contains(&query_lower)
        })
        .take(limit)
        .map(|doc| {
            json!({
                "doc_id": doc.id,
                "title": doc.title,
                "category": doc.category,
                "score": 0.9
            })
        })
        .collect();

    Ok(json!({ "results": results }))
}

/// Search documents by tags
fn search_by_tags(
    tags: &[String],
    match_mode: &str,
    query: &str,
    limit: usize,
    documents: &[IndexDocument],
) -> Result<Value, McpError> {
    let query_lower = query.to_lowercase();
    let results: Vec<Value> = documents
        .iter()
        .filter(|doc| {
            let matches = if match_mode == "all" {
                tags.iter().all(|t| doc.tags.contains(t))
            } else {
                tags.iter().any(|t| doc.tags.contains(t))
            };
            matches
        })
        .filter(|doc| {
            doc.title.to_lowercase().contains(&query_lower)
                || doc.summary.to_lowercase().contains(&query_lower)
        })
        .take(limit)
        .map(|doc| {
            json!({
                "doc_id": doc.id,
                "title": doc.title,
                "tags": doc.tags,
                "score": 0.88
            })
        })
        .collect();

    Ok(json!({ "results": results }))
}

/// Get navigation structure
fn get_navigation(format: &str, documents: &[IndexDocument]) -> Value {
    if format == "flat" {
        json!({
            "documents": documents.iter().map(|d| json!({
                "id": d.id,
                "title": d.title,
                "path": d.path
            })).collect::<Vec<_>>()
        })
    } else {
        // Hierarchical: group by category
        let mut sections: std::collections::HashMap<String, Vec<Value>> =
            std::collections::HashMap::new();

        for doc in documents {
            sections
                .entry(doc.category.clone())
                .or_insert_with(Vec::new)
                .push(json!({
                    "id": doc.id,
                    "title": doc.title,
                    "path": doc.path
                }));
        }

        let sections_list: Vec<Value> = sections
            .into_iter()
            .map(|(name, docs)| {
                json!({
                    "name": name,
                    "documents": docs
                })
            })
            .collect();

        json!({ "sections": sections_list })
    }
}

/// Explain a chunk with full context trail
fn explain_chunk(chunk_id: &str, chunks: &[ChunkMetadata]) -> Result<Value, McpError> {
    let chunk = chunks
        .iter()
        .find(|c| c.chunk_id == chunk_id)
        .ok_or_else(|| McpError::ChunkNotFound(chunk_id.to_string()))?;

    // Build context trail by traversing previous_chunk_id backwards
    let mut context_trail = Vec::new();
    let mut current_id = chunk.previous_chunk_id.as_ref();
    let mut visited = std::collections::HashSet::new();

    while let Some(prev_id) = current_id {
        if visited.contains(prev_id) {
            break; // Prevent cycles
        }
        visited.insert(prev_id.clone());

        if let Some(prev_chunk) = chunks.iter().find(|c| &c.chunk_id == prev_id) {
            context_trail.push(json!({
                "chunk_id": prev_chunk.chunk_id,
                "heading": prev_chunk.heading,
                "excerpt": truncate_summary(&prev_chunk.summary, 100)
            }));

            current_id = prev_chunk.previous_chunk_id.as_ref();
        } else {
            break;
        }
    }

    // Reverse to show chronological order (first -> current)
    context_trail.reverse();

    // Collect next chunks (sequential and children)
    let mut next_chunks = Vec::new();
    if let Some(next_id) = &chunk.next_chunk_id {
        next_chunks.push(next_id.clone());
    }
    next_chunks.extend(chunk.child_chunk_ids.iter().cloned());

    // Collect related chunks
    let related_chunks: Vec<String> = chunk
        .related_chunks
        .iter()
        .map(|r| r.chunk_id.clone())
        .collect();

    Ok(json!({
        "chunk_id": chunk.chunk_id,
        "doc_id": chunk.doc_id,
        "doc_title": chunk.doc_title,
        "heading": chunk.heading,
        "chunk_type": chunk.chunk_type,
        "token_count": chunk.token_count,
        "summary": chunk.summary,
        "chunk_level": chunk.chunk_level,
        "context_trail": context_trail,
        "next_chunks": next_chunks,
        "related_chunks": related_chunks
    }))
}

/// Truncate summary to max_chars characters
fn truncate_summary(summary: &str, max_chars: usize) -> String {
    if summary.len() <= max_chars {
        summary.to_string()
    } else {
        format!("{}...", &summary[..max_chars])
    }
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
            },
            {
                "name": "find_related",
                "description": "Navigate knowledge DAG to find related chunks",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "chunk_id": {
                            "type": "string",
                            "description": "Starting chunk identifier"
                        },
                        "relationship_type": {
                            "type": "string",
                            "description": "Type of relationship (similar | sequential | hierarchical)",
                            "default": "similar"
                        },
                        "max_depth": {
                            "type": "number",
                            "description": "Maximum traversal depth (default: 2)",
                            "default": 2
                        },
                        "limit": {
                            "type": "number",
                            "description": "Maximum number of results (default: 10)",
                            "default": 10
                        }
                    },
                    "required": ["chunk_id"]
                }
            },
            {
                "name": "get_document",
                "description": "Retrieve full document with optional chunks",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "doc_id": {
                            "type": "string",
                            "description": "Document identifier"
                        },
                        "include_chunks": {
                            "type": "boolean",
                            "description": "Include document chunks (default: true)",
                            "default": true
                        },
                        "chunk_level": {
                            "type": "string",
                            "description": "Filter by chunk level (standard | summary | detailed)"
                        }
                    },
                    "required": ["doc_id"]
                }
            },
            {
                "name": "semantic_search",
                "description": "Vector-based semantic search (stub - full implementation in v8.0)",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "Natural language query"
                        },
                        "limit": {
                            "type": "number",
                            "description": "Maximum number of results (default: 10)",
                            "default": 10
                        },
                        "threshold": {
                            "type": "number",
                            "description": "Minimum similarity threshold (default: 0.7)",
                            "default": 0.7
                        }
                    },
                    "required": ["query"]
                }
            },
            {
                "name": "search_by_category",
                "description": "Search documents filtered by category",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "category": {
                            "type": "string",
                            "description": "Document category to filter by"
                        },
                        "query": {
                            "type": "string",
                            "description": "Search query"
                        },
                        "limit": {
                            "type": "number",
                            "description": "Maximum number of results (default: 10)",
                            "default": 10
                        }
                    },
                    "required": ["category", "query"]
                }
            },
            {
                "name": "search_by_tags",
                "description": "Search documents filtered by tags",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "tags": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Tags to filter by"
                        },
                        "match_mode": {
                            "type": "string",
                            "description": "Match mode (all | any, default: any)",
                            "default": "any"
                        },
                        "query": {
                            "type": "string",
                            "description": "Search query"
                        },
                        "limit": {
                            "type": "number",
                            "description": "Maximum number of results (default: 10)",
                            "default": 10
                        }
                    },
                    "required": ["tags", "query"]
                }
            },
            {
                "name": "get_navigation",
                "description": "Get documentation navigation structure",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "format": {
                            "type": "string",
                            "description": "Output format (hierarchical | flat, default: hierarchical)",
                            "default": "hierarchical"
                        }
                    }
                }
            },
            {
                "name": "explain_chunk",
                "description": "Return chunk with full context trail showing navigation path",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "chunk_id": {
                            "type": "string",
                            "description": "Chunk ID to explain"
                        }
                    },
                    "required": ["chunk_id"]
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
                "find_related" => {
                    let params: FindRelatedParams = serde_json::from_value(arguments.clone())
                        .map_err(|e| McpError::InvalidRequest(format!("invalid find_related params: {}", e)))?;

                    find_related(
                        &params.chunk_id,
                        &params.relationship_type,
                        params.max_depth,
                        params.limit,
                        &index.chunks,
                    )
                }
                "get_document" => {
                    let params: GetDocumentParams = serde_json::from_value(arguments.clone())
                        .map_err(|e| McpError::InvalidRequest(format!("invalid get_document params: {}", e)))?;

                    get_document(
                        &params.doc_id,
                        params.include_chunks,
                        params.chunk_level.as_deref(),
                        &index.documents,
                        &index.chunks,
                    )
                }
                "semantic_search" => {
                    let params: SemanticSearchParams = serde_json::from_value(arguments.clone())
                        .map_err(|e| McpError::InvalidRequest(format!("invalid semantic_search params: {}", e)))?;

                    semantic_search(&params.query, params.limit, params.threshold, &index.chunks)
                }
                "search_by_category" => {
                    let params: SearchByCategoryParams = serde_json::from_value(arguments.clone())
                        .map_err(|e| McpError::InvalidRequest(format!("invalid search_by_category params: {}", e)))?;

                    search_by_category(&params.category, &params.query, params.limit, &index.documents)
                }
                "search_by_tags" => {
                    let params: SearchByTagsParams = serde_json::from_value(arguments.clone())
                        .map_err(|e| McpError::InvalidRequest(format!("invalid search_by_tags params: {}", e)))?;

                    search_by_tags(&params.tags, &params.match_mode, &params.query, params.limit, &index.documents)
                }
                "get_navigation" => {
                    let params: GetNavigationParams = serde_json::from_value(arguments.clone())
                        .map_err(|e| McpError::InvalidRequest(format!("invalid get_navigation params: {}", e)))?;

                    Ok(get_navigation(&params.format, &index.documents))
                }
                "explain_chunk" => {
                    let params: ExplainChunkParams = serde_json::from_value(arguments.clone())
                        .map_err(|e| McpError::InvalidRequest(format!("invalid explain_chunk params: {}", e)))?;

                    explain_chunk(&params.chunk_id, &index.chunks)
                }
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
    // Determine index directory: INDEX_DIR env var > current directory
    let index_dir_str = std::env::var("INDEX_DIR").unwrap_or_else(|_| ".".to_string());
    let index_dir = Path::new(&index_dir_str);
    let index_path = index_dir.join("INDEX.json");

    // Load index once at startup
    let index = load_index(&index_path)?;

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
        // v6.0: Added 7 new tools (find_related, get_document, semantic_search,
        // search_by_category, search_by_tags, get_navigation, explain_chunk)
        // Total: 3 (v5.0) + 7 (v6.0) = 10 tools
        assert_eq!(tools_array.map(Vec::len), Some(10));
    }

    #[test]
    fn test_list_all_documents_empty() {
        let docs: Vec<IndexDocument> = vec![];
        let result = list_all_documents(&docs);

        assert_eq!(result["total"], 0);
        assert_eq!(result["documents"].as_array().map(Vec::len), Some(0));
    }
}
