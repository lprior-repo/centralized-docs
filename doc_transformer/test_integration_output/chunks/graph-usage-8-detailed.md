---
doc_id: graph-usage
chunk_id: graph-usage#8
chunk_level: detailed
chunk_type: prose
heading: Testing
token_count: 312
summary: ## Implementation Details. ### Graph Construction
---

```bash
```

```bash
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
- Performs depth-first search (DFS) from the starting node
- Counts all nodes reachable via outgoing edges
- Excludes the starting node from the count
- Uses transitive closure to find indirect relationships

### Performance
- Graph loaded from INDEX.json (one-time cost)
- Edge filtering performed in-memory
- Reachability uses efficient DFS with visited set
- Suitable for graphs with thousands of nodes

## Testing

The graph subcommand includes comprehensive tests:
- ✅ Finding node edges
- ✅ Node not found errors
- ✅ Graph command with valid nodes
- ✅ Chunk IDs with `#` characters
- ✅ Nodes with no edges
- ✅ Reachable nodes calculation
- ✅ Missing graph data handling
- ✅ Title truncation
- ✅ Edge weight precision
- ✅ Multiple edge types between nodes

Run tests:
```bash
cargo test graph
```

