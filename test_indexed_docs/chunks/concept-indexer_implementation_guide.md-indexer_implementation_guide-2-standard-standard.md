---
doc_id: concept/indexer_implementation_guide.md/indexer_implementation_guide
chunk_id: concept/indexer_implementation_guide.md/indexer_implementation_guide#2-standard
chunk_level: standard
chunk_type: prose
heading: Outputs
token_count: 78
summary: # Indexer Implementation Guide. Technical details on `doc_transformer` indexer
---

# Indexer Implementation Guide

Technical details on `doc_transformer` indexer.

## Commands

```bash
# Build
cargo build --release

# Index local docs
./target/release/doc_transformer index ./docs_source --output ./indexed_output
```

## Outputs
- `INDEX.json`: Machine readable graph and index.
- `COMPASS.md`: Human readable navigation.
- `chunks/`: Semantic chunks.
- `docs/`: Standardized markdown.

