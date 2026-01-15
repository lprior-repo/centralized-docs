---
doc_id: graph-usage
chunk_id: graph-usage#15
chunk_level: summary
chunk_type: prose
heading: Implementation Details
token_count: 131
summary: Verify edge weights and types for quality assurance:. doc_transformer graph \"troubleshooting#2\"
---


```bash
```

Verify edge weights and types for quality assurance:
```bash
doc_transformer graph "troubleshooting#2"
```

## Implementation Details

### Graph Construction
The knowledge graph is built during the `transform` command:
1. Documents become nodes
2. Chunks become nodes
3. Parent-child edges link documents to their chunks
4. Sequential edges link chunks in reading order
5. Related edges connect semantically similar chunks (Jaccard similarity ≥ 0.3)

### Reachability Calculation
When `--reachable` is used:
