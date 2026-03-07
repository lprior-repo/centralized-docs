---
doc_id: concept/indexer.md/indexer
chunk_id: concept/indexer.md/indexer#1-standard
chunk_level: standard
chunk_type: prose
heading: 7-Step Pipeline
token_count: 103
summary: Transforms raw documentation into AI-optimized, searchable knowledge structures using Anthropic's Contextual Retrieval pattern. ## 7-Step Pipeline
---

# Indexer

Transforms raw documentation into AI-optimized, searchable knowledge structures using Anthropic's Contextual Retrieval pattern.

## 7-Step Pipeline

1. **DISCOVER**: Find markdown files.
2. **ANALYZE**: Extract metadata (titles, headings, links).
3. **ASSIGN IDs**: Generate URL-safe slug IDs.
4. **TRANSFORM**: Apply standard formatting and frontmatter.
5. **CHUNK**: Split on `H2` boundaries, prepending context from previous chunks.
6. **INDEX**: Build searchable `INDEX.json` and a navigation guide.
7. **VALIDATE**: Ensure quality standards are met.

