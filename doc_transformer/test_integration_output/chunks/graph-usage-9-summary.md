---
doc_id: graph-usage
chunk_id: graph-usage#9
chunk_level: summary
chunk_type: prose
heading: Error Handling
token_count: 132
summary: doc_transformer graph \"doc-id#0\" --index-dir output. Chunk IDs with `#` characters are properly ha
---



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
