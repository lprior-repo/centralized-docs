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

## Tuning
- **Tokens:** ~4 chars = 1 token (adjustable).
- **Chunk sizes:** Summary (128), Standard (512), Detailed (1024).
- **Relationships:** Configurable Jaccard similarity threshold for the DAG.
