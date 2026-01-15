---
doc_id: usage-example
chunk_id: usage-example#16
chunk_level: summary
chunk_type: prose
heading: Error Handling
token_count: 56
summary: ### Unknown Method. echo '{\"method\":\"unknown/method\"}' | cargo run --bin mcp_server
---

  }
}
```

### Unknown Method

```bash
echo '{"method":"unknown/method"}' | cargo run --bin mcp_server
```

**Response**:
```json
{
  "error": {
    "code": -32603,
    "message": "unknown method: unknown/method"
  }
}
```

