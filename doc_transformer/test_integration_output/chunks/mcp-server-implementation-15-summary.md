---
doc_id: mcp-server-implementation
chunk_id: mcp-server-implementation#15
chunk_level: summary
chunk_type: prose
heading: MCP Tools
token_count: 109
summary:   \"child_chunk_ids\": []. ### Tool 3: `list_docs`
---

```json
{
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

