# centralized-docs

A pure Rust CLI tool for transforming raw documentation into AI-optimized, searchable knowledge structures.

## Overview

**centralized-docs** transforms markdown documentation into a semantic knowledge graph with:
- 📑 Automatic metadata extraction (titles, headings, categories, tags)
- 🔗 Knowledge Graph DAG (directed acyclic graph) with document relationships
- 📝 Semantic chunking with contextual prefixes (AI-optimized)
- 🔍 Full-text indexing with keyword search
- 🧭 Navigation guide (COMPASS.md) for document discovery
- ✅ Automated validation and quality checking

## Quick Start

### Build
```bash
cd doc_transformer
cargo build --release
```

### Transform Documentation
```bash
./target/release/doc_transformer ./source_docs ./output_index
```

### Output Structure
```
output_index/
├── docs/                    # Transformed source documents
├── chunks/                  # AI-optimized semantic chunks
├── INDEX.json              # Complete searchable index
└── COMPASS.md              # Navigation guide
```

## Architecture

### 7-Step Pipeline

1. **DISCOVER** - Scan directories for markdown files
2. **ANALYZE** - Extract metadata (titles, headings, categories)
3. **ASSIGN IDs** - Generate hierarchical document IDs
4. **TRANSFORM** - Apply standard formatting and frontmatter
5. **CHUNK** - Semantic splitting with context prefixes (~170 tokens/chunk)
6. **INDEX** - Build searchable index (INDEX.json)
7. **VALIDATE** - Quality checks and validation

### Key Features

**Knowledge Graph (DAG)**
- Automatic relationship detection
- Jaccard similarity scoring
- Topological ordering
- Semantic navigation

**Contextual Retrieval**
- Each chunk includes 50-100 token context prefix
- Natural multi-turn AI conversations
- 35% fewer retrieval failures (Anthropic research)

**Full-Text Search**
- Keyword indexing
- Category and tag filtering
- Complete chunk navigation

## Example

```bash
# Transform CUE documentation (36 files)
./target/release/doc_transformer ./cue_docs ./indexed_output

# Output
# ======================================================================
# DOC_TRANSFORMER v4.3 (Knowledge DAG)
# ======================================================================
# [STEP 1] DISCOVER: Found 36 files
# [STEP 2] ANALYZE: Processed 36 files
# [STEP 3] ASSIGN IDs: Generated 36 IDs
# [STEP 4] TRANSFORM: 36/36 files (0 errors)
# [STEP 5] CHUNK: Generated 156 chunks
# [STEP 6] INDEX: Created COMPASS.md and INDEX.json
# [STEP 7] VALIDATE: 36/36 files passed
# ======================================================================
# COMPLETE
```

## For AI Agents

Load the generated INDEX.json to:
1. Search by keyword
2. Get document metadata and chunk list
3. Retrieve individual chunks with context
4. Navigate related documents via Knowledge Graph

See `docs/INDEXER.md` for complete integration guide.

## Testing

```bash
# Run all tests
cargo test --all-features

# Run specific tests
cargo test graph::tests::

# With output
cargo test -- --nocapture
```

## Project Structure

```
centralized-docs/
├── doc_transformer/              # Rust transformer binary
│   ├── src/
│   │   ├── main.rs              # Entry point
│   │   ├── discover.rs          # File discovery
│   │   ├── analyze.rs           # Metadata extraction
│   │   ├── assign.rs            # ID generation
│   │   ├── transform.rs         # Document transformation
│   │   ├── chunk.rs             # Semantic chunking
│   │   ├── graph.rs             # Knowledge DAG
│   │   ├── index.rs             # Indexing
│   │   └── validate.rs          # Validation
│   └── Cargo.toml
├── cue_docs/                     # Example: CUE documentation (36 files)
├── docs/
│   └── INDEXER.md               # Complete documentation
├── CLAUDE.md                     # AI development rules
└── README.md                     # This file
```

## Dependencies

- **petgraph** - Graph data structures (Knowledge DAG)
- **serde** / **serde_json** - Serialization
- **regex** - Pattern matching
- **walkdir** - Directory traversal
- **chrono** - Timestamps
- **clap** - CLI parsing
- **tokio** - Async runtime
- **anyhow** - Error handling

## Version

**v4.3** - Knowledge DAG with Anthropic Contextual Retrieval pattern

## License

MIT
