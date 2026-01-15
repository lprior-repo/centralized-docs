---
doc_id: graph-usage
chunk_id: graph-usage#11
chunk_level: summary
chunk_type: prose
heading: Output Format
token_count: 143
summary: ### Missing Graph Data. json exists but doesn't contain graph data:
---

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
