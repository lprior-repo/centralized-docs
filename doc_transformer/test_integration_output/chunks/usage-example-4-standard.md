---
doc_id: usage-example
chunk_id: usage-example#4
chunk_level: standard
chunk_type: prose
heading: Integration with AI Assistants
token_count: 517
summary:       \"title\": \"Getting Started with Rust\",.       \"category\": \"tutorial\",
---


```json
{
    {
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
