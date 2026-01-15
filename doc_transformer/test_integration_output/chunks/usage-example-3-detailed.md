---
doc_id: usage-example
chunk_id: usage-example#3
chunk_level: detailed
chunk_type: code
heading: Integration with AI Assistants
token_count: 738
summary:   \"path\": \"chunks/chunk-001-standard.   \"previous_chunk_id\": null,
---

```

```json
{
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

```bash
echo '{"method":"tools/call","params":{"name":"list_docs","arguments":{}}}' \
  | cargo run --bin mcp_server
```

**Response**:
```json
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

```json
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

```python
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

```javascript
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

