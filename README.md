# Doc Transformer

**Transform raw documentation into AI-optimized knowledge structures**

Doc Transformer is a high-performance Rust CLI tool that converts unstructured documentation into semantically chunked, graph-connected, and AI-ready knowledge bases. It analyzes, categorizes, chunks, and indexes documentation with validation and incremental processing support.

## Features

- **Smart Discovery**: Automatically finds and categorizes markdown documentation
- **Semantic Analysis**: Classifies docs by type (tutorial, reference, concept, ops, meta)
- **Intelligent Chunking**: Breaks documents into ~170 token semantic chunks with contextual prefixes
- **Knowledge Graph**: Builds a DAG (Directed Acyclic Graph) of document relationships
- **Full-Text Search**: BM25-based search across documents and chunks
- **Incremental Processing**: Only processes changed files (with `--incremental` flag)
- **Validation Pipeline**: Comprehensive validation of output quality and link integrity
- **Graph Exploration**: Query and explore knowledge graph relationships

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [CLI Commands](#cli-commands)
  - [Transform Command](#transform-command)
  - [Search Command](#search-command)
  - [Graph Command](#graph-command)
- [Configuration](#configuration)
- [Architecture](#architecture)
- [Output Structure](#output-structure)
- [Development](#development)
- [Testing](#testing)
- [Troubleshooting](#troubleshooting)
- [License](#license)

## Installation

### Prerequisites

- **Rust 1.70+** (Install from [rustup.rs](https://rustup.rs/))
- **Cargo** (included with Rust)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/centralized-docs.git
cd centralized-docs/doc_transformer

# Build the project
cargo build --release

# Binary will be at target/release/doc_transformer
./target/release/doc_transformer --help
```

### Install Locally

```bash
# Install to ~/.cargo/bin
cargo install --path .

# Now available as 'doc-transformer'
doc-transformer --help
```

## Quick Start

### 1. Transform Documentation

Transform a directory of markdown files into an AI-optimized knowledge base:

```bash
doc-transformer transform ./docs ./output
```

This will:
- Discover all markdown files in `./docs`
- Analyze and categorize each document
- Generate semantic chunks (~170 tokens each)
- Build a knowledge graph with relationships
- Create searchable index (INDEX.json)
- Generate navigation guide (COMPASS.md)
- Validate all outputs

### 2. Search Documents

Search across your indexed documentation:

```bash
# Search documents
doc-transformer search "installation" --index-dir ./output --limit 5

# Search within chunks (more granular)
doc-transformer search "rust ownership" --index-dir ./output --chunks --limit 10
```

### 3. Explore Knowledge Graph

Explore relationships in the knowledge graph:

```bash
# Show direct connections
doc-transformer graph tutorial-general-getting-started --index-dir ./output

# Show all reachable nodes (transitive closure)
doc-transformer graph tutorial-general-getting-started --index-dir ./output --reachable
```

### 4. Incremental Updates

Process only changed files (faster for large doc sets):

```bash
# First run - full index
doc-transformer transform ./docs ./output --incremental

# Subsequent runs - only process changes
doc-transformer transform ./docs ./output --incremental

# Force full re-index
doc-transformer transform ./docs ./output --force
```

## CLI Commands

### Transform Command

Transform raw documentation into AI-optimized knowledge structures.

```bash
doc-transformer transform <SOURCE_DIR> <OUTPUT_DIR> [OPTIONS]
```

**Arguments:**
- `<SOURCE_DIR>` - Directory containing markdown documentation
- `<OUTPUT_DIR>` - Directory for transformed output

**Options:**
- `--incremental` - Only process changed files (saves state for future runs)
- `--force` - Force full re-index, ignoring incremental state
- `--verbose` - Show detailed validation output and warnings

**Examples:**

```bash
# Basic transformation
doc-transformer transform ./raw_docs ./indexed_docs

# Incremental mode (only process changes)
doc-transformer transform ./raw_docs ./indexed_docs --incremental

# Force full re-index with verbose output
doc-transformer transform ./raw_docs ./indexed_docs --force --verbose
```

**Pipeline Steps:**

1. **DISCOVER** - Find all markdown files
2. **ANALYZE** - Extract metadata, classify by type
3. **ASSIGN** - Generate unique document IDs
4. **TRANSFORM** - Convert to standardized format
5. **CHUNK** - Create semantic chunks with context
6. **INDEX** - Build searchable index and knowledge graph
7. **VALIDATE** - Check output quality and standards
8. **VALIDATE LINKS** - Verify internal link integrity

### Search Command

Search documents or chunks using BM25 full-text search.

```bash
doc-transformer search <QUERY> --index-dir <DIR> [OPTIONS]
```

**Arguments:**
- `<QUERY>` - Search query string

**Options:**
- `--index-dir, -i <DIR>` - Directory containing INDEX.json (required)
- `--limit, -n <NUMBER>` - Maximum results to return (default: 10)
- `--chunks` - Search within chunks instead of documents

**Examples:**

```bash
# Search documents
doc-transformer search "getting started" --index-dir ./output

# Search chunks with more results
doc-transformer search "ownership borrowing" --index-dir ./output --chunks -n 20

# Search with custom limit
doc-transformer search "installation" -i ./output -n 5
```

**Output Format:**

Document search shows:
- Document title and ID
- Category (tutorial, ref, concept, ops, meta)
- BM25 relevance score
- Summary excerpt

Chunk search shows:
- Chunk ID and level
- Parent document title
- Section heading (if applicable)
- Content snippet
- BM25 relevance score

### Graph Command

Explore knowledge graph relationships between documents and chunks.

```bash
doc-transformer graph <NODE_ID> --index-dir <DIR> [OPTIONS]
```

**Arguments:**
- `<NODE_ID>` - Document ID or chunk ID to explore

**Options:**
- `--index-dir, -i <DIR>` - Directory containing INDEX.json (default: current directory)
- `--reachable` - Show count of all reachable nodes (transitive closure)

**Examples:**

```bash
# Show direct connections
doc-transformer graph tutorial-general-rust-basics --index-dir ./output

# Show reachability analysis
doc-transformer graph concept-general-ownership --index-dir ./output --reachable
```

**Edge Types:**

- **References** - Document A links to Document B
- **PartOf** - Chunk belongs to Document
- **Follows** - Sequential relationship (next/previous)
- **RelatesTo** - Semantic relationship

**Output Shows:**
- Node type (Document or Chunk)
- Node title
- Outgoing edges (what this node references)
- Incoming edges (what references this node)
- Reachable node count (with `--reachable`)

## Configuration

Doc Transformer uses command-line arguments for configuration. No environment variables or config files are required.

### Directory Structure

**Input (Source Directory):**
```
docs/
├── getting-started.md
├── concepts/
│   ├── ownership.md
│   └── borrowing.md
├── tutorials/
│   └── first-program.md
└── reference/
    └── api-docs.md
```

**Output (Indexed Directory):**
```
output/
├── INDEX.json              # Master index with graph data
├── COMPASS.md             # Human-readable navigation guide
├── docs/                  # Transformed documents
│   ├── tutorial-general-getting-started.md
│   ├── concept-general-ownership.md
│   └── ref-general-api-docs.md
└── chunks/                # Semantic chunks
    ├── getting-started-0.md
    ├── getting-started-1.md
    ├── ownership-0.md
    └── ...
```

## Architecture

Doc Transformer follows a functional pipeline architecture with these core modules:

### Pipeline Stages

1. **discover** - File system traversal and markdown detection
2. **analyze** - Content analysis, metadata extraction, classification
3. **assign** - ID generation and link mapping
4. **transform** - Document standardization and frontmatter injection
5. **chunk** - Semantic chunking with contextual prefixes
6. **graph** - Knowledge graph construction (DAG)
7. **index** - Search index building (BM25)
8. **validate** - Quality checks and link validation
9. **search** - BM25-based full-text search
10. **incremental** - Change detection and state management

### Document Classification

Documents are automatically classified into categories:

- **tutorial** - Step-by-step guides (contains "tutorial", "getting started", "step")
- **reference** - API docs and specifications (contains "api", "reference", "spec")
- **concept** - Explanatory content (contains "concept", "overview", "architecture")
- **ops** - Operational guides (contains "deploy", "install", "configure")
- **meta** - Project metadata (README, contributing guides)

### Chunking Strategy

- **Target Size**: ~170 tokens per chunk
- **Context Preservation**: Each chunk includes document title and section heading
- **Semantic Boundaries**: Respects markdown heading structure
- **Overlap**: Minimal overlap to maintain context

### Knowledge Graph

- **Node Types**: Documents and Chunks
- **Edge Types**: References, PartOf, Follows, RelatesTo
- **Graph Structure**: Directed Acyclic Graph (DAG)
- **Validation**: Cycle detection, orphan detection

## Output Structure

### INDEX.json

The master index contains:

```json
{
  "version": "4.3",
  "generated_at": "2026-01-11T12:00:00Z",
  "total_documents": 42,
  "total_chunks": 156,
  "documents": [
    {
      "id": "tutorial-general-getting-started",
      "title": "Getting Started",
      "source_path": "docs/getting-started.md",
      "output_path": "output/docs/tutorial-general-getting-started.md",
      "category": "tutorial",
      "summary": "Quick start guide...",
      "word_count": 450,
      "chunks": ["getting-started-0", "getting-started-1"]
    }
  ],
  "chunks": [
    {
      "id": "getting-started-0",
      "document_id": "tutorial-general-getting-started",
      "chunk_number": 0,
      "heading": "Installation",
      "content": "...",
      "token_count": 168
    }
  ],
  "graph": {
    "nodes": [...],
    "edges": [...]
  }
}
```

### COMPASS.md

Human-readable navigation guide with:
- Document index by category
- Chunk statistics
- Graph topology summary
- Link health status

## Development

### Project Structure

```
doc_transformer/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── discover.rs      # File discovery
│   ├── analyze.rs       # Content analysis
│   ├── assign.rs        # ID assignment
│   ├── transform.rs     # Document transformation
│   ├── chunk.rs         # Semantic chunking
│   ├── graph.rs         # Knowledge graph
│   ├── index.rs         # Index building
│   ├── validate.rs      # Validation pipeline
│   ├── search.rs        # Search implementation
│   └── incremental.rs   # Incremental processing
├── tests/               # Integration tests
├── Cargo.toml          # Dependencies
└── Cargo.lock          # Locked dependencies
```

### Building

```bash
# Development build (faster, with debug symbols)
cargo build

# Release build (optimized, smaller binary)
cargo build --release

# Check without building
cargo check
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_discover

# Run integration tests only
cargo test --test '*'
```

### Code Formatting

```bash
# Format code
cargo fmt

# Check formatting without changes
cargo fmt -- --check
```

### Linting

```bash
# Run Clippy linter
cargo clippy

# Clippy with all features
cargo clippy --all-features --all-targets
```

## Testing

### Test Coverage

The project includes comprehensive tests:

- **Unit tests** - In each module (`src/*.rs`)
- **Integration tests** - In `tests/` directory
- **Test fixtures** - Sample docs in `test_docs/`
- **Test output** - Validated output in `test_output/`

### Running Integration Tests

```bash
# Run all tests with verbose output
cargo test -- --nocapture

# Test specific module
cargo test discover_tests

# Test the full transform pipeline
cargo test --test validate_tests
```

### Test Data

Sample test documents are in `doc_transformer/test_docs/`:
- `sample.md` - Basic markdown document
- Uses real-world patterns for validation

## Troubleshooting

### Problem: "INDEX.json not found"

**Symptom:**
```
INDEX.json not found at: ./output/INDEX.json
Please run the transform command first.
```

**Solution:**
Run the transform command before search or graph commands:
```bash
doc-transformer transform ./docs ./output
```

### Problem: "No markdown files found"

**Symptom:**
```
DISCOVER: Found 0 files
```

**Solution:**
- Verify source directory exists and contains `.md` files
- Check file permissions (must be readable)
- Ensure files have `.md` extension (case-sensitive)

### Problem: High validation errors

**Symptom:**
```
VALIDATE: 12/42 files passed (30 errors, 8 warnings)
```

**Solution:**
Run with `--verbose` to see specific issues:
```bash
doc-transformer transform ./docs ./output --verbose
```

Common validation issues:
- **Empty documents** - Add meaningful content
- **Missing titles** - Ensure H1 heading exists
- **Broken links** - Fix or remove invalid links
- **Invalid frontmatter** - Check YAML syntax

### Problem: Incremental mode not detecting changes

**Symptom:**
```
Unchanged: 42 files (skipped)
```

**Solution:**
- Ensure file timestamps are correct
- Use `--force` to bypass incremental state
- Delete `.incremental_state.json` in output directory

### Problem: Binary too large

**Symptom:**
Release binary is larger than expected

**Solution:**
```bash
# Strip debug symbols
strip target/release/doc_transformer

# Or build with minimal size
cargo build --release --config profile.release.opt-level='z'
```

### Problem: Out of memory during chunking

**Symptom:**
Process crashes during CHUNK step

**Solution:**
- Process documents in smaller batches
- Reduce document size (split large files)
- Increase system memory
- Check for extremely large documents

### Problem: Search returns no results

**Symptom:**
```
No matching documents found.
```

**Solution:**
- Verify INDEX.json exists and is valid JSON
- Check query spelling and terms
- Try broader search terms
- Increase `--limit` parameter
- Use `--chunks` for more granular search

### Problem: Graph command shows no edges

**Symptom:**
```
No relationships found
```

**Solution:**
- Documents may not have internal links
- Add markdown links between documents: `[text](../other-doc.md)`
- Check if node ID is correct (case-sensitive)
- Some documents naturally have no connections

## Version

Current version: **4.3** (Knowledge DAG)

Features in this version:
- Incremental processing with state management
- BM25 full-text search
- Knowledge graph exploration
- Enhanced validation pipeline
- Link integrity checking
- Semantic chunking with context

## License

MIT License - See LICENSE file for details

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Write tests for new features
4. Ensure all tests pass: `cargo test`
5. Format code: `cargo fmt`
6. Run linter: `cargo clippy`
7. Submit a pull request

## Support

For issues, questions, or contributions:
- Open an issue on GitHub
- Check existing issues for solutions
- Include error messages and `--verbose` output

---

**Built with Rust** - Fast, safe, and reliable documentation transformation.
