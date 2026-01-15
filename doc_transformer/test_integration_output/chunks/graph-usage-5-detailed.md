---
doc_id: graph-usage
chunk_id: graph-usage#5
chunk_level: detailed
chunk_type: code
heading: Output Format
token_count: 392
summary: doc_transformer graph \"sample#0\" --index-dir test_output --reachable. Output includes:
---



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

## Output Format

### Node Information
- **Node ID**: The unique identifier
- **Node Type**: `Document` or `Chunk`
- **Title**: Truncated to 50 characters if longer (with `...`)

### Edge Information
Each edge displays:
- **Direction**: `→` for outgoing, `←` for incoming
- **Target/Source ID**: The other node in the relationship
- **Edge Type**: Parent, Sequential, Related, etc.
- **Weight**: Displayed with 2 decimal precision (e.g., `0.65`)
- **Title**: Title of the connected node (truncated to 40 chars)

### Special Cases
- **No Edges**: Displays "No relationships found"
- **Long Titles**: Automatically truncated with `...` suffix
- **Multiple Edge Types**: All edges between nodes are shown, even if multiple types exist

