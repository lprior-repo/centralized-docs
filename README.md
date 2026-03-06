# Centralized Docs (`doc_transformer`)

A pure Rust CLI tool that transforms raw markdown documentation into AI-optimized, searchable structures. 

It provides tools to parse, index, and search documentation using BM25, semantic chunking, and contextual retrieval patterns.

## Features

- **Semantic Indexing:** Full-text search optimized for AI using BM25.
- **Contextual Chunking:** Chunks documentation while preserving surrounding context for LLM retrieval.
- **`llms.txt` Parsing:** Extracts and structures data according to the `llms.txt` standard.

## Quick Start

### 1. Installation

```bash
cargo install --path .
```

### 2. Basic Usage

```bash
# Search indexed documentation
doc_transformer search "your query"
```

## Documentation

Full documentation is available in the `website/` directory, built with [mdBook](https://rust-lang.github.io/mdBook/).

```bash
# To view the docs locally:
cd website
mdbook serve --open
```

## Architecture

This project is built using Hexagonal Architecture (Ports and Adapters) to isolate domain logic from infrastructure. See the [Architecture Docs](./docs/ARCHITECTURE.md).
