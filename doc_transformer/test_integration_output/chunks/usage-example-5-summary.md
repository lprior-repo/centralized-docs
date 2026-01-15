---
doc_id: usage-example
chunk_id: usage-example#5
chunk_level: summary
chunk_type: prose
heading: Tool Examples
token_count: 131
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

### Get Specific Chunk

```bash
echo '{"method":"tools/call","params":{"name":"get_chunk","arguments":{"chunk_id":"chunk-001"}}}' \
  | cargo run --bin mcp_server
```

**Response**:
```json
{
  "chunk_id": "chunk-001",
  "doc_id": "doc-001",
  "doc_title": "Getting Started with Rust",
