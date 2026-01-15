---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#12
chunk_level: summary
chunk_type: prose
heading: MCP Tools
token_count: 139
summary: take(limit).     Ok(json!({ \"results\": results }))
---

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
