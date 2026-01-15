---
doc_id: usage-example
chunk_id: usage-example#11
chunk_level: summary
chunk_type: prose
heading: Integration with AI Assistants
token_count: 129
summary:         \"params\": {.     proc = subprocess
---


        "params": {
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
