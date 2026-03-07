---
doc_id: concept/vision.md/vision
chunk_id: concept/vision.md/vision#1-detailed
chunk_level: detailed
chunk_type: prose
heading: Why This Project Exists
token_count: 125
summary: **centralized-docs** is a pure Rust CLI tool that transforms raw documentation into AI-optimized, searchable knowledge structures.  It implements Anthropic's Contextual Retrieval pattern, semantic ...
---

# Vision

**centralized-docs** is a pure Rust CLI tool that transforms raw documentation into AI-optimized, searchable knowledge structures. It implements Anthropic's Contextual Retrieval pattern, semantic chunking, and hexagonal architecture to deliver the most effective documentation system for both human developers and AI agents.

## Why This Project Exists

Current documentation systems fail AI agents because of context loss, isolation, and search friction. **centralized-docs** solves this through:

- **Contextual Chunking**: Each chunk carries surrounding context.
- **Knowledge Graphs**: DAG-based relationship detection between documents.
- **Semantic Indexing**: Full-text search optimized for AI.
- **Navigation Guides**: Clear mapping for both humans (COMPASS.md) and AI (INDEX.json).

