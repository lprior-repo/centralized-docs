---
doc_id: usage-example
chunk_id: usage-example#5
chunk_level: detailed
chunk_type: code
heading: Production Deployment
token_count: 446
summary:         resolve(JSON. parse(output));
---


        resolve(JSON.parse(output));
      } else {
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

## Production Deployment

### Systemd Service

Create `/etc/systemd/system/mcp-server.service`:

```ini
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

