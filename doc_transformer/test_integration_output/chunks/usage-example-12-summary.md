---
doc_id: usage-example
chunk_id: usage-example#12
chunk_level: summary
chunk_type: prose
heading: Integration with AI Assistants
token_count: 129
summary: # Example usage.     print(f\"- {doc['title']} (score: {doc['score']})\")
---

    )


# Example usage
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
