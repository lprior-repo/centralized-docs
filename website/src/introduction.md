# Introduction & Features

**Centralized Docs** (`ctd`) is a pure Rust CLI tool that transforms raw documentation and static sites into AI-optimized, searchable knowledge structures. 

## The Origin Problem

This project stemmed from the frustration of trying to get AI agents to understand modern software libraries. The standard workflows suck:
- Blindly copy/pasting massive chunks of documentation into prompts.
- Wasting time manually curating custom "skills" or context files for every new library.
- Watching AI agents hallucinate because they downloaded a massive, unstructured documentation site and lost the context in the noise.

Instead of fighting the docs, **Centralized Docs** acts as a bridge. You can scrape any static documentation site or read local markdown files, and the CLI will output an `llms.txt` entry point alongside a DAG (Directed Acyclic Graph) of the knowledge. 

You simply point your AI at the `llms.txt`, and it can intelligently traverse the DAG and search the index rather than blindly reading everything.

## Core Features & Benefits

- **Scrapes Static Sites:** Point the CLI at a documentation URL, and it will crawl the site, extract the content, and convert it to clean markdown.
- **Contextual Chunking:** Each chunk carries surrounding context, solving the typical "lost in the middle" problem when AI reads isolated pieces of text.
- **Knowledge Graphs (DAG):** Detects and maps relationships between documents, making it easy for an agent to traverse related concepts systematically.
- **llms.txt Generation:** Automatically builds `llms.txt` files, acting as a `robots.txt` equivalent for AI agents.
- **Semantic Indexing:** Fast, full-text search optimized for AI using BM25, allowing agents to query exactly what they need.
- **MCP Server:** Built-in Model Context Protocol server for native AI agent integration (Claude Desktop, Claude Code, etc.).
- **Incremental Indexing:** State database tracks file changes, only re-processing what changed on re-index.

## Core Philosophy

1. **Version Control as Source of Truth**: All generated docs can live in Git.
2. **Search as Exploration**: Find things easily via graphs and indexes.
3. **Portability Above All**: Framework-agnostic JSON and Markdown outputs.
4. **Complete Traceability**: Understand exactly how docs were transformed.
