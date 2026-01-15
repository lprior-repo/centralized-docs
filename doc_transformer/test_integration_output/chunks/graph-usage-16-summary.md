---
doc_id: graph-usage
chunk_id: graph-usage#16
chunk_level: summary
chunk_type: prose
heading: Implementation Details
token_count: 131
summary:  Sequential edges link chunks in reading order. ### Reachability Calculation
---

```bash
```


4. Sequential edges link chunks in reading order

### Reachability Calculation
When `--reachable` is used:
- Performs depth-first search (DFS) from the starting node
- Counts all nodes reachable via outgoing edges
- Excludes the starting node from the count
- Uses transitive closure to find indirect relationships

### Performance
- Graph loaded from INDEX.json (one-time cost)
- Edge filtering performed in-memory
- Reachability uses efficient DFS with visited set
- Suitable for graphs with thousands of nodes
