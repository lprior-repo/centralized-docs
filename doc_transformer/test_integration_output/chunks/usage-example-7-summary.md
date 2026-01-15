---
doc_id: usage-example
chunk_id: usage-example#7
chunk_level: summary
chunk_type: prose
heading: Tool Examples
token_count: 140
summary: ### List All Documents. echo '{\"method\":\"tools/call\",\"params\":{\"name\":\"list_docs\",\"argume
---

{
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
      "tags": ["functional", "advanced", "monads"],
