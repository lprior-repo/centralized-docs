---
doc_id: usage-example
chunk_id: usage-example#6
chunk_level: summary
chunk_type: prose
heading: Tool Examples
token_count: 145
summary: **Response**:.   \"chunk_id\": \"chunk-001\",
---


```bash
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
