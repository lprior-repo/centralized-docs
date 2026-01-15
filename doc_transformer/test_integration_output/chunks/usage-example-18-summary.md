---
doc_id: usage-example
chunk_id: usage-example#18
chunk_level: summary
chunk_type: prose
heading: Production Deployment
token_count: 134
summary: WorkingDirectory=/opt/doc_transformer. Restart=always
---

WorkingDirectory=/opt/doc_transformer
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
