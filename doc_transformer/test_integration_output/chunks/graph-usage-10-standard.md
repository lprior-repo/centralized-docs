---
doc_id: graph-usage
chunk_id: graph-usage#10
chunk_level: standard
chunk_type: prose
heading: See Also
token_count: 152
summary: ### Performance. The graph subcommand includes comprehensive tests:
---

### Performance

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
