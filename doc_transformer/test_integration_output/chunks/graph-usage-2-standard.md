---
doc_id: graph-usage
chunk_id: graph-usage#2
chunk_level: standard
chunk_type: prose
heading: Edge Types
token_count: 178
summary: # Graph Subcommand - Knowledge DAG Traversal. The `graph` subcommand allows you to explore the knowl
---

# Graph Subcommand - Knowledge DAG Traversal

The `graph` subcommand allows you to explore the knowledge graph relationships between documents and chunks in your documentation.

## Overview

The knowledge graph is a Directed Acyclic Graph (DAG) that represents:
- **Documents** as nodes
- **Chunks** (semantic sections) as nodes
- **Relationships** as weighted edges between nodes

## Edge Types

- **Parent**: Document contains chunk (weight: 1.0)
- **Sequential**: Next chunk in document order (weight: 1.0)
- **Related**: Semantically similar content (weight: 0.3-1.0, based on Jaccard similarity)
- **References**: Explicit cross-references in documentation
- **ReferencedBy**: Backlinks from other documents

