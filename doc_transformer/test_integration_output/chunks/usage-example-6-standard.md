---
doc_id: usage-example
chunk_id: usage-example#6
chunk_level: standard
chunk_type: prose
heading: Error Handling
token_count: 230
summary:       if (code === 0) {.         resolve(JSON
---


      if (code === 0) {
        resolve(JSON.parse(output));
      } else {
        reject(new Error(`MCP server exited with code ${code}`));
      }
    });

    mcp.stdin.write(request + '\n');
    mcp.stdin.end();
  });
}

// Example usage
queryDocs('rust functional programming').then((results) => {
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
```json
{
  "error": {
    "code": -32603,
    "message": "unknown method: unknown/method"
  }
}
```

