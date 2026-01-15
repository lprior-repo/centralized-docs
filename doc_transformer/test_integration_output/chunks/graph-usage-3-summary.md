---
doc_id: graph-usage
chunk_id: graph-usage#3
chunk_level: summary
chunk_type: prose
heading: Usage
token_count: 140
summary: - **References**: Explicit cross-references in documentation. - **ReferencedBy**: Backlinks from oth
---



- **References**: Explicit cross-references in documentation
- **ReferencedBy**: Backlinks from other documents

## Usage

### Basic Command

```bash
doc_transformer graph <NODE_ID> [OPTIONS]
```

### Options

- `<NODE_ID>`: The ID of the node to explore (required)
  - Document IDs: e.g., `"tutorial/general/getting-started"`
  - Chunk IDs: e.g., `"getting-started#0"`, `"doc-id#1"`
  
- `-i, --index-dir <DIR>`: Directory containing INDEX.json (default: current directory)

- `--reachable`: Show count of nodes reachable from this node (transitive closure)
