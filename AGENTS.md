# kubernetes.io - Agent Instructions

> Documentation scraped from https://kubernetes.io/docs/home/

## Project Overview

This documentation index contains 99 documents organized by category.

### Document Categories

- **ops**: 2 documents
- **tutorial**: 46 documents
- **ref**: 49 documents
- **concept**: 2 documents

## Navigation Guide

When working with this documentation:

1. **Start with llms.txt** - Read this first to understand the structure
2. **Use INDEX.json** - For programmatic lookup of documents and chunks
3. **Follow the DAG** - Use knowledge graph edges to find related content
4. **Chunk navigation** - Each chunk has `previous_chunk_id` and `next_chunk_id`

## File Structure

```
./
├── llms.txt           # AI entry point (read first)
├── llms-full.txt      # Full content for large context models
├── AGENTS.md          # This file - coding instructions
├── INDEX.json         # Machine-readable index + knowledge graph
├── COMPASS.md         # Human-readable navigation
├── docs/              # Transformed documents with frontmatter
└── chunks/            # Semantic chunks with context prefix
```

## Chunk Format

Each chunk file contains:
- YAML frontmatter with `chunk_id`, `doc_id`, `token_count`, navigation pointers
- Context prefix from previous chunk (~50-100 tokens)
- Main content (~170 tokens average)

## INDEX.json Structure

```json
{
  "documents": [...],    // Document metadata
  "chunks": [...],       // Chunk metadata with navigation
  "keywords": {...},     // Term → doc_id lookup
  "graph": {             // Knowledge DAG
    "nodes": [...],      // Documents and chunks
    "edges": [...]       // Relationships (Parent, Sequential, Related)
  }
}
```

## Best Practices

- **Don't guess**: Use INDEX.json to find exact document/chunk IDs
- **Read context**: When reading a chunk, consider reading previous/next chunks
- **Follow relationships**: Use graph edges to find related content
- **Check frontmatter**: Every document has `category`, `tags`, and `summary`
