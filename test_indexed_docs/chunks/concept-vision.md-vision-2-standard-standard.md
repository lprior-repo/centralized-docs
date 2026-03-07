---
doc_id: concept/vision.md/vision
chunk_id: concept/vision.md/vision#2-standard
chunk_level: standard
chunk_type: prose
heading: Core Philosophy
token_count: 156
summary: ## Why This Project Exists. Current documentation systems fail AI agents because of context loss, isolation, and search friction
---

# Vision


## Why This Project Exists

Current documentation systems fail AI agents because of context loss, isolation, and search friction. **centralized-docs** solves this through:

- **Contextual Chunking**: Each chunk carries surrounding context.
- **Knowledge Graphs**: DAG-based relationship detection between documents.
- **Semantic Indexing**: Full-text search optimized for AI.
- **Navigation Guides**: Clear mapping for both humans (COMPASS.md) and AI (INDEX.json).

## Core Philosophy

1. **Version Control as Source of Truth**: All docs live in Git.
2. **Search as Exploration**: Find things easily via graphs and indexes.
3. **Portability Above All**: Framework-agnostic JSON and Markdown outputs.
4. **Complete Traceability**: Understand exactly how docs were transformed.
5. **Testable by Default**: High coverage unit tests ensure transformations never fail.
