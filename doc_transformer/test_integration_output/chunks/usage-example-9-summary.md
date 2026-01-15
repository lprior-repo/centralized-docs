---
doc_id: usage-example
chunk_id: usage-example#9
chunk_level: summary
chunk_type: prose
heading: Integration with AI Assistants
token_count: 129
summary:       \"category\": \"concept\",.       \"word_count\": 2000,
---

{
    {
    },
    {
      "category": "concept",
      "word_count": 2000,
      "chunk_count": 1
    }
  ],
  "total": 2
}
```

## Integration with AI Assistants

### Claude Desktop Configuration

Add to `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "doc_transformer": {
      "command": "/path/to/doc_transformer/target/release/mcp_server",
      "args": [],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

### Python Integration

```python
