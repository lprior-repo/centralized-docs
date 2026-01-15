---
doc_id: usage-example
chunk_id: usage-example#6
chunk_level: detailed
chunk_type: prose
heading: Testing
token_count: 322
summary: Description=MCP Documentation Server. After=network
---

[Unit]
Description=MCP Documentation Server
After=network.target

[Service]
Type=simple
User=mcp
WorkingDirectory=/opt/doc_transformer
ExecStart=/opt/doc_transformer/target/release/mcp_server
Restart=always
RestartSec=10
StandardInput=socket
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

Start the service:
```bash
sudo systemctl enable mcp-server
sudo systemctl start mcp-server
sudo journalctl -u mcp-server -f
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

