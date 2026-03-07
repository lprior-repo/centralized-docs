---
id: concept/vision.md/vision
title: Vision
category: concept
tags: ["concept", "exists", "philosophy", "project", "vision"]
---

# Vision



 > 
 > **Context**: \**centralized-docs** is a pure Rust CLI tool that transforms raw documentation into AI-optimized, searchable knowledge structures. It implements Anthr



**centralized-docs** is a pure Rust CLI tool that transforms raw documentation into AI-optimized, searchable knowledge structures. It implements Anthropic’s Contextual Retrieval pattern, semantic chunking, and hexagonal architecture to deliver the most effective documentation system for both human developers and AI agents.

## Why This Project Exists

Current documentation systems fail AI agents because of context loss, isolation, and search friction. **centralized-docs** solves this through:

* **Contextual Chunking**: Each chunk carries surrounding context.
* **Knowledge Graphs**: DAG-based relationship detection between documents.
* **Semantic Indexing**: Full-text search optimized for AI.
* **Navigation Guides**: Clear mapping for both humans (COMPASS.md) and AI (INDEX.json).

## Core Philosophy

1. **Version Control as Source of Truth**: All docs live in Git.
1. **Search as Exploration**: Find things easily via graphs and indexes.
1. **Portability Above All**: Framework-agnostic JSON and Markdown outputs.
1. **Complete Traceability**: Understand exactly how docs were transformed.
1. **Testable by Default**: High coverage unit tests ensure transformations never fail.
## See Also

- [Documentation Index](./COMPASS.md)
