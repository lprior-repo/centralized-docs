---
doc_id: graph-usage
chunk_id: graph-usage#7
chunk_level: detailed
chunk_type: prose
heading: Implementation Details
token_count: 412
summary: - **Title**: Title of the connected node (truncated to 40 chars). ### Special Cases
---

- **Title**: Title of the connected node (truncated to 40 chars)

### Special Cases
- **No Edges**: Displays "No relationships found"
- **Long Titles**: Automatically truncated with `...` suffix
- **Multiple Edge Types**: All edges between nodes are shown, even if multiple types exist

## Use Cases

### 1. Understanding Document Structure
Explore how a document breaks down into chunks:
```bash
doc_transformer graph "my-document"
```

### 2. Finding Related Content
See what other chunks are semantically similar:
```bash
doc_transformer graph "authentication#0"
```

### 3. Navigation Planning
Determine reachability for navigation features:
```bash
doc_transformer graph "index" --reachable
```

### 4. Debugging Relationships
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
- Performs depth-first search (DFS) from the starting node
- Counts all nodes reachable via outgoing edges
- Excludes the starting node from the count
- Uses transitive closure to find indirect relationships

### Performance
- Graph loaded from INDEX.json (one-time cost)
- Edge filtering performed in-memory
- Reachability uses efficient DFS with visited set
- Suitable for graphs with thousands of nodes

