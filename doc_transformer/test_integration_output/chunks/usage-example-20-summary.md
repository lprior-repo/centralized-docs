---
doc_id: usage-example
chunk_id: usage-example#20
chunk_level: summary
chunk_type: prose
heading: Testing
token_count: 133
summary: WORKDIR /app. CMD [\"mcp_server\"]
---



WORKDIR /app
CMD ["mcp_server"]
```

Build and run:
```bash
docker build -t mcp-server .
docker run -i mcp-server
```

## Testing

### Automated Test Suite

```bash
# Run all tests
./test_mcp_server.sh

# Python integration tests
python3 test_mcp_client.py

# Unit tests
cargo test --bin mcp_server
```

### Manual Testing

```bash
# Start interactive session
cargo run --bin mcp_server

# Paste JSON requests line-by-line:
{"method":"tools/list"}
{"method":"tools/call","params":{"name":"search_docs","arguments":{"query":"rust"}}}
