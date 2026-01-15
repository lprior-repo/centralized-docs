---
doc_id: graph-usage
chunk_id: graph-usage#4
chunk_level: detailed
chunk_type: code
heading: Error Handling
token_count: 320
summary:      Sample - Basic Concepts.   → sample#3 [Related, weight: 1
---

     Sample - Basic Concepts
  → sample#3 [Related, weight: 1.00]
     Sample - Writing Your First Program

Incoming Edges (3):
  ← sample#1 [Related, weight: 1.00]
     Sample - Installation
  ← sample#2 [Related, weight: 1.00]
     Sample - Basic Concepts
  ← sample#3 [Related, weight: 1.00]
     Sample - Writing Your First Program

======================================================================
```

#### 3. Show Reachable Nodes

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

## Error Handling

### Node Not Found
```bash
$ doc_transformer graph "nonexistent" --index-dir test_output
Error: Node not found: nonexistent
```

### Missing INDEX.json
```bash
$ doc_transformer graph "sample#0" --index-dir /invalid
Error: INDEX.json not found at: /invalid/INDEX.json
Please run the transform command first.
```

### Missing Graph Data
If INDEX.json exists but doesn't contain graph data:
```
Error: INDEX.json missing graph data
```

