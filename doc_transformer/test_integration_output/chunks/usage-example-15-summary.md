---
doc_id: usage-example
chunk_id: usage-example#15
chunk_level: summary
chunk_type: prose
heading: Error Handling
token_count: 130
summary: forEach((doc) => {. log(`- ${doc
---

  });
}

  results.results.forEach((doc) => {
    console.log(`- ${doc.title} (score: ${doc.score})`);
  });
});
```

## Error Handling

### Invalid Chunk ID

```bash
echo '{"method":"tools/call","params":{"name":"get_chunk","arguments":{"chunk_id":"invalid"}}}' \
  | cargo run --bin mcp_server
```

**Response**:
```json
{
  "error": {
    "code": -32603,
    "message": "chunk not found: invalid"
  }
}
```

### Unknown Method

```bash
echo '{"method":"unknown/method"}' | cargo run --bin mcp_server
```

**Response**:
