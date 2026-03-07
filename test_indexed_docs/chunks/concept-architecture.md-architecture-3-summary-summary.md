---
doc_id: concept/architecture.md/architecture
chunk_id: concept/architecture.md/architecture#3-summary
chunk_level: summary
chunk_type: prose
heading: The Pipeline
token_count: 96
summary: ## Core Principles.  **Adapters Layer**: File I/O (`discover
---



## Core Principles


## Layers

4. **Adapters Layer**: File I/O (`discover.rs`), JSON serialization (`index.rs`), regex (`analyze.rs`).

## The Pipeline

1. **Discover**: Find markdown files.
2. **Analyze**: Extract metadata.
3. **Assign**: Generate IDs.
4. **Transform**: Apply standard formatting.
5. **Chunk**: Semantic splitting.
6. **Index**: Create `INDEX.json` and `COMPASS.md`.
7. **Validate**: Run quality checks.
