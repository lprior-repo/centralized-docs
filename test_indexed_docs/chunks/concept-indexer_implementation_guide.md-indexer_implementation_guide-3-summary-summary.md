---
doc_id: concept/indexer_implementation_guide.md/indexer_implementation_guide
chunk_id: concept/indexer_implementation_guide.md/indexer_implementation_guide#3-summary
chunk_level: summary
chunk_type: prose
heading: Tuning
token_count: 73
summary: json`: Machine readable graph and index. md`: Human readable navigation
---






- `INDEX.json`: Machine readable graph and index.
- `COMPASS.md`: Human readable navigation.
- `chunks/`: Semantic chunks.
- `docs/`: Standardized markdown.

## Tuning
- **Tokens:** ~4 chars = 1 token (adjustable).
- **Chunk sizes:** Summary (128), Standard (512), Detailed (1024).
- **Relationships:** Configurable Jaccard similarity threshold for the DAG.
