---
doc_id: graph-usage
chunk_id: graph-usage#13
chunk_level: summary
chunk_type: prose
heading: Use Cases
token_count: 130
summary: ### Special Cases. - **Multiple Edge Types**: All edges between nodes are shown, even if multiple ty
---




### Special Cases
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
