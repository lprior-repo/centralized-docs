---
doc_id: usage-example
chunk_id: usage-example#17
chunk_level: summary
chunk_type: prose
heading: Production Deployment
token_count: 130
summary: **Response**:.     \"code\": -32603,
---

```bash
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
