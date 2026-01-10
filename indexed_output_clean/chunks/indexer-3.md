---
doc_id: indexer
chunk_id: indexer#3
chunk_type: code
heading: Usage
token_count: 1249
summary: # Indexer: AI-Optimized Documentation Transform. ## Overview
---

# Indexer: AI-Optimized Documentation Transform

## Overview

The **doc_transformer** is a Rust CLI that transforms raw documentation into AI-optimized, searchable knowledge structures. It implements Anthropic's Contextual Retrieval pattern: each chunk includes context prefixes and navigation metadata that enables both semantic search and multi-turn AI conversations.

## Architecture: 7-Step Pipeline

```
RAW DOCS → DISCOVER → ANALYZE → ASSIGN IDs → TRANSFORM → CHUNK → INDEX → VALIDATE
```

### Step 1: DISCOVER
Recursively scan source directory for markdown files (.md, .mdx, .rst, .txt).

**Output:**
- File list with sizes
- Discovery manifest with timestamps

### Step 2: ANALYZE
Extract metadata from each document:
- Title (from H1 or filename)
- Heading hierarchy
- Links (internal + external)
- First paragraph (for summaries)
- Word count
- Category detection (concept/tutorial/ops/ref)
- Presence of code blocks and tables

**Output:**
- Analysis structs with complete metadata
- Category statistics

### Step 3: ASSIGN IDs
Generate unique, URL-safe document IDs based on file paths.

**Output:**
- Slug-based IDs: `docs-tour-basics`, `api-reference`, etc.
- Link map for cross-references

### Step 4: TRANSFORM
Apply standard formatting and frontmatter to each document:
```yaml
---
id: docs-tour-basics
title: Basics of CUE
category: tutorial
tags: [cue, tour, basics]
word_count: 1234
---
```

**Output:**
- Standardized markdown in `docs/` directory
- Consistent heading hierarchy
- Navigation metadata

### Step 5: CHUNK (The Secret Sauce)
**Smart semantic chunking with contextual prefixes:**

1. **Split on H2 boundaries** - Each section (## Heading) becomes a chunk boundary
2. **Prepend context** - Last 50-100 tokens from previous chunk for context
3. **Estimate tokens** - Simple: ~4 chars = 1 token, target ~170 tokens/chunk
4. **Extract navigation** - Each chunk knows:
   - `previous_chunk_id` - Link to prior chunk
   - `next_chunk_id` - Link to next chunk
   - `chunk_index` - Position in document
   - Heading - What section is this?

**Why this works for AI:**
- AI reads chunk → understands context through prefix
- No "I don't have enough context" failures
- Multi-turn conversations stay coherent across chunks
- Reduces RAG retrieval failures by 35% (Anthropic research)

**Example chunk frontmatter:**
```yaml
---
doc_id: docs-tour
chunk_id: docs-tour#0
heading: Basics
token_count: 1850
summary: Introduction to CUE language fundamentals
previous_chunk_id: null
next_chunk_id: docs-tour#1
---
```

**Output:**
- Individual chunk files in `chunks/` directory
- Each chunk is self-contained but linked
- Chunk metadata with navigation

### Step 6: INDEX
Build comprehensive searchable index in `INDEX.json`:

```json
{
  "version": "4.2",
  "stats": {
    "doc_count": 36,
    "chunk_count": 156,
    "avg_chunk_size_tokens": 170
  },
  "documents": [
    {
      "id": "docs-tour",
      "title": "Tour",
      "path": "docs/docs_tour.md",
      "category": "tutorial",
      "tags": ["cue", "tour"],
      "summary": "...",
      "word_count": 1234,
      "chunk_ids": ["docs-tour#0", "docs-tour#1", ...]
    }
  ],
  "chunks": [
    {
      "chunk_id": "docs-tour#0",
      "doc_id": "docs-tour",
      "doc_title": "Tour",
      "heading": "Basics",
      "token_count": 1850,
      "summary": "Introduction...",
      "previous_chunk_id": null,
      "next_chunk_id": "docs-tour#1",
      "path": "chunks/docs-tour-0.md"
    }
  ],
  "keywords": {
    "cue": ["docs-tour", "tutorial"],
    "basics": ["docs-tour"],
    ...
  },
  "navigation": {
    "type": "contextual_retrieval",
    "strategy": "50-100 token context prefix + H2 boundaries",
    "avg_tokens_per_chunk": 170
  }
}
```

**Features:**
- Document index with all metadata
- Chunk index with navigation pointers
- Keyword index for full-text search
- Statistics about document collection
- Navigation strategy documentation

### Step 7: VALIDATE
Verify transformed documents meet quality standards:

- ✅ Single H1 per document
- ✅ Required frontmatter fields
- ✅ Valid heading hierarchy (no skipped levels)
- ✅ Minimum tag count (3+)
- ✅ Context and See Also sections

**Output:**
- Validation report with errors/warnings
- Quality metrics

## Usage

### Build
```bash
cd doc_transformer
cargo build --release
```

### Transform CUE Documentation
```bash
./target/release/doc_transformer ./cue_docs ./indexed_output
```

### Output Structure
```
indexed_output/
├── docs/                           # Transformed source docs
│   ├── docs_introduction.md
│   ├── docs_tour.md
│   └── ...
├── chunks/                         # AI-optimized chunks
│   ├── docs-introduction-0.md      # With frontmatter + navigation
│   ├── docs-tour-0.md
│   ├── docs-tour-1.md
│   └── ...
├── INDEX.json                      # Searchable index
└── COMPASS.md                      # Navigation guide
```

