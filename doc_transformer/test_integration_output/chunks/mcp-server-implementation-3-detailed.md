---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#3
chunk_level: detailed
chunk_type: table
heading: Implementation Details
token_count: 989
summary: **Status**: ✅ CLOSED. ## Executive Summary
---



**Status**: ✅ CLOSED

---

## Executive Summary

Successfully implemented a production-ready MCP (Model Context Protocol) server for AI-powered documentation queries. The server exposes three tools via JSON-RPC over stdio:

1. **search_docs** - Full-text search with BM25 ranking (Tantivy + fallback)
2. **get_chunk** - Retrieve specific chunks with navigation context
3. **list_docs** - List all documents with metadata

The implementation follows strict **Functional Rust** principles:
- ✅ Zero panics (no `.unwrap()`, `.expect()`, `panic!()`)
- ✅ Railway-Oriented Programming with `Result<T, E>`
- ✅ Semantic error types using `thiserror`
- ✅ Functional Core, Imperative Shell architecture
- ✅ Immutability by default
- ✅ Iterator combinators over imperative loops

---

## Implementation Details

### File Structure

```
src/bin/mcp_server.rs          # MCP server implementation (400+ lines)
indexed_output/INDEX.json      # Test index with 2 docs, 3 chunks
test_mcp_server.sh            # Bash test script
test_mcp_client.py            # Python integration test client
```

### Architecture

```
┌─────────────────────────────────────┐
│   IMPERATIVE SHELL (I/O Layer)      │
│  - stdio JSON-RPC communication     │
│  - File loading (INDEX.json)        │
│  - Error formatting                 │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│   FUNCTIONAL CORE (Pure Logic)      │
│  - load_index()                     │
│  - search_documents()               │
│  - find_chunk()                     │
│  - list_all_documents()             │
│  - handle_request()                 │
│  ALL return Result<T, McpError>     │
└─────────────────────────────────────┘
```

### Error Handling (Railway-Oriented Programming)

All errors are typed and semantic using `thiserror`:

```rust
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
```

### Search Implementation (Tantivy + Fallback)

The search function gracefully handles multiple failure scenarios:

1. **Tantivy index exists and has results** → Use Tantivy BM25 search
2. **Tantivy index missing/corrupted** → Fallback to simple text matching
3. **Tantivy returns 0 results** → Fallback to simple text matching

This ensures search **never fails** even when the Tantivy index is unavailable.

```rust
fn search_documents(
    index_dir: &Path,
    query: &str,
    limit: usize,
    fallback_docs: &[IndexDocument],
) -> Result<Value, McpError> {
    let tantivy_results = search::open_or_create_index(index_dir)
        .ok()
        .and_then(|idx| search::search_index(&idx, query, limit).ok())
        .filter(|results| !results.is_empty());

    let results = tantivy_results
        .map(|search_results| { /* Tantivy path */ })
        .unwrap_or_else(|| {
            // Fallback: simple text matching
            fallback_docs.iter()
                .filter(|doc| /* case-insensitive search */)
                .take(limit)
                .collect()
        });

    Ok(json!({ "results": results }))
}
```

---

