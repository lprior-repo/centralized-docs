# Centralized Docs (`ctd`)

A pure Rust CLI tool that transforms raw markdown documentation into AI-optimized, searchable structures.

Introduction & Features
Centralized Docs (ctd) is a pure Rust CLI tool that transforms raw documentation and static sites into AI-optimized, searchable knowledge structures.

The Origin Problem
This project stemmed from the frustration of trying to get AI agents to understand modern software libraries. The standard workflows suck:

* Blindly copy/pasting massive chunks of documentation into prompts.
* Wasting time manually curating custom “skills” or context files for every new library.
* Watching AI agents hallucinate because they downloaded a massive, unstructured documentation site and lost the context in the noise.
* Instead of fighting the docs, Centralized Docs acts as a bridge. You can scrape any static documentation site or read local markdown files, and the CLI will output an llms.txt entry point alongside a DAG (Directed Acyclic Graph) of the knowledge.

You simply point your AI at the llms.txt, and it can intelligently traverse the DAG and search the index rather than blindly reading everything.

Core Features & Benefits
Scrapes Static Sites: Point the CLI at a documentation URL, and it will crawl the site, extract the content, and convert it to clean markdown.
Contextual Chunking: Each chunk carries surrounding context, solving the typical “lost in the middle” problem when AI reads isolated pieces of text.
Knowledge Graphs (DAG): Detects and maps relationships between documents, making it easy for an agent to traverse related concepts systematically.
llms.txt Generation: Automatically builds llms.txt files, acting as a robots.txt equivalent for AI agents.
Semantic Indexing: Fast, full-text search optimized for AI using BM25, allowing agents to query exactly what they need.

- ## Documentation Site

The published docs site lives at `https://lprior-repo.github.io/centralized-docs/`.

## Quick Start

### 1. Installation

**Release binary (Linux x86_64 and macOS Apple Silicon):**
```bash
curl -sSL https://raw.githubusercontent.com/lprior-repo/centralized-docs/main/scripts/install.sh | bash
```

## Documentation Site

The published docs site lives at `https://lprior-repo.github.io/centralized-docs/`.

**From source:**
```bash
cargo install --path centralized-docs
```

### 2. Basic Usage

```bash
# Index a directory of markdown files
ctd index ./docs --output ./output

# Search the generated index
ctd search "your query" --index-dir ./output

# Validate generated metadata
llms_txt_validator ./output/llms.txt
```
