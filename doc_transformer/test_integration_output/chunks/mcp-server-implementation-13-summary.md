---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#13
chunk_level: summary
chunk_type: prose
heading: MCP Tools
token_count: 128
summary:   \"results\": [.       \"title\": \"Getting Started with Rust\",
---

  "results": [
    {
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
