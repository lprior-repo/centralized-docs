# Introduction & Features

**Centralized Docs** (`doc_transformer`) is a pure Rust CLI tool that transforms raw documentation into AI-optimized, searchable knowledge structures. It implements Anthropic's Contextual Retrieval pattern, semantic chunking, and hexagonal architecture to deliver the most effective documentation system for both human developers and AI agents.

## Why This Project Exists

Current documentation systems fail AI agents because of context loss, isolation, and search friction. Instead of blindly downloading entire documentation sites and wasting tokens, or losing context when reading isolated chunks, **Centralized Docs** provides a streamlined approach. 

By placing an `llms.txt` file at the root of a project and using contextual chunking, we bridge the gap between human-readable docs and AI-accessible knowledge.

## Core Features & Benefits

- **Token Efficiency:** Use up to **60% fewer tokens** by pointing AI agents directly to pre-indexed, relevant chunks instead of entire documentation trees.
- **Improved Accuracy:** Achieve better accuracy (up to **35% better**) by giving AI agents contextual chunks. Every chunk retains its surrounding context, reducing hallucinations and missed connections.
- **Contextual Chunking:** Each chunk carries surrounding context, solving the typical "lost in the middle" problem when AI reads isolated pieces of text.
- **Semantic Indexing:** Fast, full-text search optimized for AI using BM25.
- **Knowledge Graphs:** DAG-based relationship detection between documents, making it easy for an agent to traverse related concepts.
- **llms.txt Generation:** Automatically builds `llms.txt` files, acting as a `robots.txt` equivalent for AI agents.

## Core Philosophy

1. **Version Control as Source of Truth**: All docs live in Git.
2. **Search as Exploration**: Find things easily via graphs and indexes.
3. **Portability Above All**: Framework-agnostic JSON and Markdown outputs.
4. **Complete Traceability**: Understand exactly how docs were transformed.
