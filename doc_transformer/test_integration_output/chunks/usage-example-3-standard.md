---
doc_id: usage-example
chunk_id: usage-example#3
chunk_level: standard
chunk_type: prose
heading: Tool Examples
token_count: 141
summary: ### List All Documents.   | cargo run --bin mcp_server
---

{
}
```

### List All Documents

```bash
  | cargo run --bin mcp_server
```

**Response**:
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
    },
    {
      "id": "doc-002",
      "title": "Advanced Functional Programming",
      "category": "concept",
      "tags": ["functional", "advanced", "monads"],
      "word_count": 2000,
      "chunk_count": 1
    }
  ],
  "total": 2
}
```

