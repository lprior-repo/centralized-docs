---
doc_id: graph-usage
chunk_id: graph-usage#18
chunk_level: summary
chunk_type: prose
heading: Testing
token_count: 132
summary: ### Performance. - Reachability uses efficient DFS with visited set
---




### Performance
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
