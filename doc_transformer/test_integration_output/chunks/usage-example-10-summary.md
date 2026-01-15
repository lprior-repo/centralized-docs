---
doc_id: usage-example
chunk_id: usage-example#10
chunk_level: summary
chunk_type: prose
heading: Integration with AI Assistants
token_count: 130
summary:       \"args\": [],.         \"RUST_LOG\": \"info\"
---


```json
{
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
