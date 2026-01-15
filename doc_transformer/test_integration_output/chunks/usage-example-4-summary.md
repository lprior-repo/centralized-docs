---
doc_id: usage-example
chunk_id: usage-example#4
chunk_level: summary
chunk_type: prose
heading: Tool Examples
token_count: 129
summary:       \"name\": \"list_docs\",.       \"inputSchema\": {\"type\": \"object\", \"properties\": {}}
---

        },
      }
    },
    {
      "name": "list_docs",
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
