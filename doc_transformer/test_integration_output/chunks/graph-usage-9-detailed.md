---
doc_id: graph-usage
chunk_id: graph-usage#9
chunk_level: detailed
chunk_type: prose
heading: See Also
token_count: 257
summary: When `--reachable` is used:. - Performs depth-first search (DFS) from the starting node
---


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

## See Also

- `transform` command - Builds the knowledge graph
- `search` command - Search documents and chunks by content
- INDEX.json schema - Graph data structure reference
