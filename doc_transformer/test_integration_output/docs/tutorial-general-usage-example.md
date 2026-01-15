---
id: tutorial/general/usage-example
title: USAGE EXAMPLE
category: tutorial
tags: ["example", "quick", "server", "start", "tutorial"]
---

# MCP Server Usage Example


> **Context**: ```bash cargo run --bin mcp_server



## Quick Start
### 1. Start the MCP Server
```
cargo run --bin mcp_server



```
The server reads JSON-RPC requests from stdin and writes responses to stdout.
### 2. Query Available Tools
```
echo '{"method":"tools/list"}' | cargo run --bin mcp_server



```
**Response**:
```
{
  "tools": [
    {
      "name": "search_docs",
      "description": "Search documentation using full-text search with BM25 ranking",
      "inputSchema": {
        "type": "object",
        "properties": {
          "query": {"type": "string", "description": "Search query"},
          "limit": {"type": "number", "default": 10}
        },
        "required": ["query"]
      }
    },
    {
      "name": "get_chunk",
      "description": "Retrieve a specific chunk by ID with navigation context",
      "inputSchema": {
        "type": "object",
        "properties": {
          "chunk_id": {"type": "string"}
        },
        "required": ["chunk_id"]
      }
    },
    {
      "name": "list_docs",
      "description": "List all available documents with metadata",
      "inputSchema": {"type": "object", "properties": {}}
    }
  ]
}



```
## Tool Examples
### Search Documentation
```
echo '{"method":"tools/call","params":{"name":"search_docs","arguments":{"query":"rust programming","limit":5}}}' \
  | cargo run --bin mcp_server



```
**Response**:
```
{
  "results": [
    {
      "id": "doc-001",
      "title": "Getting Started with Rust",
      "summary": "Learn the basics of Rust programming language",
      "category": "tutorial",
      "score": 1.0,
      "path": "docs/getting-started.md"
    }
  ]
}



```
### Get Specific Chunk
```
echo '{"method":"tools/call","params":{"name":"get_chunk","arguments":{"chunk_id":"chunk-001"}}}' \
  | cargo run --bin mcp_server



```
**Response**:
```
{
  "chunk_id": "chunk-001",
  "doc_id": "doc-001",
  "doc_title": "Getting Started with Rust",
  "heading": "Introduction",
  "chunk_type": "text",
  "token_count": 200,
  "summary": "Introduction to Rust programming",
  "path": "chunks/chunk-001-standard.md",
  "previous_chunk_id": null,
  "next_chunk_id": "chunk-002",
  "related_chunks": [],
  "chunk_level": "standard",
  "parent_chunk_id": null,
  "child_chunk_ids": []
}



```
### List All Documents
```
echo '{"method":"tools/call","params":{"name":"list_docs","arguments":{}}}' \
  | cargo run --bin mcp_server



```
**Response**:
```
{
  "documents": [
    {
      "id": "doc-001",
      "title": "Getting Started with Rust",
      "category": "tutorial",
      "tags": ["rust", "beginner", "tutorial"],
      "word_count": 1500,
      "chunk_count": 2
    },
    {
      "id": "doc-002",
      "title": "Advanced Functional Programming",
      "category": "concept",
      "tags": ["functional", "advanced", "monads"],
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
```
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
```
import json
import subprocess

def query_docs(query: str, limit: int = 10):
    """Search documentation via MCP server."""
    request = {
        "method": "tools/call",
        "params": {
            "name": "search_docs",
            "arguments": {"query": query, "limit": limit}
        }
    }

    proc = subprocess.Popen(
        ["cargo", "run", "--quiet", "--bin", "mcp_server"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        text=True
    )

    stdout, _ = proc.communicate(input=json.dumps(request) + "\n")
    return json.loads(stdout.strip())

# Example usage
results = query_docs("rust functional programming")
for doc in results["results"]:
    print(f"- {doc['title']} (score: {doc['score']})")



```
### Node.js Integration
```
const { spawn } = require('child_process');

async function queryDocs(query, limit = 10) {
  return new Promise((resolve, reject) => {
    const mcp = spawn('cargo', ['run', '--quiet', '--bin', 'mcp_server']);

    const request = JSON.stringify({
      method: 'tools/call',
      params: {
        name: 'search_docs',
        arguments: { query, limit }
      }
    });

    let output = '';
    mcp.stdout.on('data', (data) => {
      output += data.toString();
    });

    mcp.on('close', (code) => {
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
```
echo '{"method":"tools/call","params":{"name":"get_chunk","arguments":{"chunk_id":"invalid"}}}' \
  | cargo run --bin mcp_server



```
**Response**:
```
{
  "error": {
    "code": -32603,
    "message": "chunk not found: invalid"
  }
}



```
### Unknown Method
```
echo '{"method":"unknown/method"}' | cargo run --bin mcp_server



```
**Response**:
```
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
```
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
```
sudo systemctl enable mcp-server
sudo systemctl start mcp-server
sudo journalctl -u mcp-server -f



```
### Docker
Create `Dockerfile`:
```
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
```
docker build -t mcp-server .
docker run -i mcp-server



```
## Testing
### Automated Test Suite
```
# Run all tests
./test_mcp_server.sh

# Python integration tests
python3 test_mcp_client.py

# Unit tests
cargo test --bin mcp_server



```
### Manual Testing
```
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
```
# Build Tantivy index first
cargo run --bin doc_transformer -- --build-index

# Then start MCP server (index is already warm)
cargo run --bin mcp_server



```
### Release Build
Always use release builds in production:
```
cargo build --release --bin mcp_server
./target/release/mcp_server



```
Performance gains:
- ~10x faster search queries
- ~3x lower memory usage
- ~50% faster JSON parsing

## Monitoring
### Request Logging
The server logs to stderr:
```
MCP server started. Loaded 2 documents, 3 chunks



```
Redirect stderr for logging:
```
cargo run --bin mcp_server 2>> /var/log/mcp-server.log



```
### Metrics
Future enhancement: export Prometheus metrics for:
- Request count by tool
- Query latency (p50, p95, p99)
- Error rate
- Index size

**Last Updated**: 2026-01-11
**MCP Version**: 1.0
**Doc Transformer Version**: 0.5.0

## See Also

- [Documentation Index](./COMPASS.md)
