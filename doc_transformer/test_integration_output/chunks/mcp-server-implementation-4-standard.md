---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#4
chunk_level: standard
chunk_type: table
heading: Implementation Details
token_count: 478
summary: │  ALL return Result<T, McpError>     │. ### Error Handling (Railway-Oriented Programming)
---


```
               ▼
│  ALL return Result<T, McpError>     │
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

