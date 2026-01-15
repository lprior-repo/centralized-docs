---
doc_id: usage-example
chunk_id: usage-example#2
chunk_level: standard
chunk_type: code
heading: Tool Examples
token_count: 512
summary:       \"description\": \"Retrieve a specific chunk by ID with navigation context\",.       \"inputSc
---

    {
      "description": "Retrieve a specific chunk by ID with navigation context",
      "inputSchema": {
        "type": "object",
        "properties": {
          "chunk_id": {"type": "string"}
        },
        "required": ["chunk_id"]
      }
    },
    {
      "name": "list_docs",
      "description": "List all available documents with metadata",
      "inputSchema": {"type": "object", "properties": {}}
    }
  ]
}
```

## Tool Examples

### Search Documentation

```bash
echo '{"method":"tools/call","params":{"name":"search_docs","arguments":{"query":"rust programming","limit":5}}}' \
  | cargo run --bin mcp_server
```

**Response**:
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

### List All Documents

```bash
echo '{"method":"tools/call","params":{"name":"list_docs","arguments":{}}}' \
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
