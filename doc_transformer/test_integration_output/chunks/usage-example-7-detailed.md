---
doc_id: usage-example
chunk_id: usage-example#7
chunk_level: detailed
chunk_type: code
heading: Performance Tuning
token_count: 332
summary: Create `Dockerfile`:. ```dockerfile
---

```bash
```

### Docker

Create `Dockerfile`:

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin mcp_server

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/mcp_server /usr/local/bin/
COPY indexed_output /app/indexed_output
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

# Exit with Ctrl+D
```

## Performance Tuning

### Index Warming

Pre-load Tantivy index on startup:

```bash
# Build Tantivy index first
cargo run --bin doc_transformer -- --build-index

# Then start MCP server (index is already warm)
cargo run --bin mcp_server
```

### Release Build

Always use release builds in production:

```bash
cargo build --release --bin mcp_server
./target/release/mcp_server
```

Performance gains:
- ~10x faster search queries
- ~3x lower memory usage
- ~50% faster JSON parsing

