---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#4
chunk_level: detailed
chunk_type: code
heading: MCP Tools
token_count: 593
summary: This ensures search **never fails** even when the Tantivy index is unavailable. fn search_documents(
---

```




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

## MCP Tools

### Tool 1: `search_docs`

**Description**: Search documentation using full-text search with BM25 ranking

**Input Schema**:
```json
{
  "query": "rust programming",    // required
  "limit": 10                     // optional, default: 10
}
```

**Output**:
```json
{
  "results": [
    {
      "id": "doc-001",
      "title": "Getting Started with Rust",
      "summary": "Learn the basics of Rust programming language",
      "category": "tutorial",
      "score": 1.0,
      "path": "docs/getting-started.md"
    }
  ]
}
```

### Tool 2: `get_chunk`

**Description**: Retrieve a specific chunk by ID with navigation context

**Input Schema**:
```json
{
  "chunk_id": "chunk-001"
}
```

**Output**:
```json
{
  "chunk_id": "chunk-001",
  "doc_id": "doc-001",
  "doc_title": "Getting Started with Rust",
  "heading": "Introduction",
  "chunk_type": "text",
  "token_count": 200,
  "summary": "Introduction to Rust programming",
  "path": "chunks/chunk-001-standard.md",
  "previous_chunk_id": null,
  "next_chunk_id": "chunk-002",
  "related_chunks": [],
  "chunk_level": "standard",
  "parent_chunk_id": null,
  "child_chunk_ids": []
}
```

### Tool 3: `list_docs`

**Description**: List all available documents with metadata

**Input Schema**:
```json
{}
```

**Output**:
```json
{
  "documents": [
    {
      "id": "doc-001",
      "title": "Getting Started with Rust",
      "category": "tutorial",
      "tags": ["rust", "beginner", "tutorial"],
      "word_count": 1500,
      "chunk_count": 2
    }
  ],
  "total": 2
}
```

---

