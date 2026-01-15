---
doc_id: graph-usage
chunk_id: graph-usage#7
chunk_level: standard
chunk_type: prose
heading: Use Cases
token_count: 239
summary: - **Edge Type**: Parent, Sequential, Related, etc. - **Weight**: Displayed with 2 decimal precision 
---



- **Edge Type**: Parent, Sequential, Related, etc.
- **Weight**: Displayed with 2 decimal precision (e.g., `0.65`)
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

