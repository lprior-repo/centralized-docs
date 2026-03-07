---
id: concept/indexer.md/indexer
title: Indexer
category: concept
tags: ["7-step", "concept", "graph", "indexer", "knowledge"]
---

# Indexer



 > 
 > **Context**: Transforms raw documentation into AI-optimized, searchable knowledge structures using Anthropic's Contextual Retrieval pattern.



Transforms raw documentation into AI-optimized, searchable knowledge structures using Anthropic’s Contextual Retrieval pattern.

## 7-Step Pipeline

1. **DISCOVER**: Find markdown files.
1. **ANALYZE**: Extract metadata (titles, headings, links).
1. **ASSIGN IDs**: Generate URL-safe slug IDs.
1. **TRANSFORM**: Apply standard formatting and frontmatter.
1. **CHUNK**: Split on `H2` boundaries, prepending context from previous chunks.
1. **INDEX**: Build searchable `INDEX.json` and a navigation guide.
1. **VALIDATE**: Ensure quality standards are met.

## Knowledge Graph

Builds a Directed Acyclic Graph (DAG) for navigating document relationships using `petgraph`.
## See Also

- [Documentation Index](./COMPASS.md)
