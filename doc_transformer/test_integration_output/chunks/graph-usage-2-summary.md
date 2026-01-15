---
doc_id: graph-usage
chunk_id: graph-usage#2
chunk_level: summary
chunk_type: prose
heading: Edge Types
token_count: 113
summary: - **Documents** as nodes. - **Chunks** (semantic sections) as nodes
---


- **Documents** as nodes
- **Chunks** (semantic sections) as nodes
- **Relationships** as weighted edges between nodes

## Edge Types

- **Parent**: Document contains chunk (weight: 1.0)
- **Sequential**: Next chunk in document order (weight: 1.0)
- **Related**: Semantically similar content (weight: 0.3-1.0, based on Jaccard similarity)
- **References**: Explicit cross-references in documentation
- **ReferencedBy**: Backlinks from other documents

