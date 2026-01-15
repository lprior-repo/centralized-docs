---
doc_id: graph-usage
chunk_id: graph-usage#8
chunk_level: summary
chunk_type: prose
heading: Usage
token_count: 95
summary: doc_transformer graph \"sample#0\" --index-dir test_output --reachable. Output includes:
---



```


```bash
doc_transformer graph "sample#0" --index-dir test_output --reachable
```

Output includes:
```
Reachable: 12 nodes
```

This shows how many nodes can be reached by following edges from the starting node.

#### 4. Explore with Chunk ID containing #

```bash
doc_transformer graph "doc-id#0" --index-dir output
```

Chunk IDs with `#` characters are properly handled.

