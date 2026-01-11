# BEAD-009: Replace Custom BM25 with Tantivy

**ID**: `centralized-docs-uq2`
**Status**: IN_PROGRESS
**Severity**: P1 (High Priority)
**Epic**: Search & Indexing Enhancement
**Owner**: Architecture Team

---

## Context Block

### Current State
- **File**: `doc_transformer/src/filter.rs` (lines 319-347)
- **Current Code**: Simplified BM25 scoring function (29 lines)
- **Usage Points**:
  1. `main.rs` line 578 - Document search scoring
  2. `scrape.rs` line 464 - Page relevance filtering

### The Smell
- **Manual IDF Calculation**: Hardcoded IDF as `(10.0_f32).ln()` - ignores document frequency
- **No Term Frequency Normalization**: Can't distinguish document lengths
- **No Persistence**: Recalculates scores on every search (no index)
- **No Query Parsing**: Simple whitespace split (no boolean operators, phrases)
- **No Field Boosting**: Can't weight title vs. body differently
- **No Index Corruption Handling**: No recovery if index is corrupted

### Tantivy Benefits
- **Proven Algorithm**: Used by Rust documentation, Meilisearch, tens of thousands of prod systems
- **Better Accuracy**: Real BM25 with proper document frequency and field weights
- **Incremental Updates**: Can add documents without full reindexing
- **Persistence**: Index written to disk, survives restarts
- **Advanced Features**: Phrase queries, boolean operators, field-specific search
- **Battle-Tested**: 30+ years of optimization, handles edge cases

### Dependencies
```toml
tantivy = "0.25"  # Already in Cargo.toml
```

---

## Specification Block

### Design by Contract

#### INDEX SCHEMA (Tantivy Structure)
```rust
Document {
    id: String,          // Unique identifier
    title: String,       // Document title (boosted)
    summary: String,     // Document summary
    content: String,     // Full searchable content (title + summary)
    path: String,        // File path (not searchable)
    category: String,    // Document category (facet)
    word_count: u64,     // Document length (for IDF calculations)
}
```

#### QUERY INTERFACE
```rust
pub fn search(query: &str, index_path: &Path) -> Result<Vec<SearchResult>> {
    // Returns top results sorted by BM25 score
    // Handles query parsing (phrases, boolean operators)
}

pub fn score_document(document: &str, query: &str) -> Result<f32> {
    // Single document scoring (for validation compatibility)
}
```

#### PERSISTENCE MODEL
- Index location: `{output_dir}/.tantivy_index/`
- Index is write-once per scrape/index run
- Index is read-only during searches
- Graceful fallback if index missing (rebuild on demand)

### EARS (Example-Assertion-Rule-Specification)

#### Examples
1. **Simple Query**: `search("rust programming", index_path)` → Results with high score for docs mentioning rust + programming
2. **Phrase Query**: `search('"rust programming"', index_path)` → Exact phrase matches only
3. **Boolean Query**: `search('rust AND systems NOT python', index_path)` → Complex filtering
4. **Field Query**: `search('title:rust', index_path)` → Search in title field only
5. **Empty Query**: `search("", index_path)` → Returns error (as before)
6. **Missing Index**: `search("...", missing_path)` → Builds index if needed, returns results

#### Assertions
- Search results match or exceed current BM25 accuracy (compare with old implementation)
- Performance: Index 1000+ documents in <5 seconds
- Memory: Index is <50MB for typical documentation sets
- Index persistence: Survives process restart
- Error handling: All error paths return meaningful Err(...) types

#### Rules
- Always validate query before indexing
- Always create parent directory if missing
- Always use `.tantivy_index` directory for isolation
- Always handle UTF-8 properly (Tantivy does this)
- Never panic on index corruption (return error instead)

#### Specification
- Tantivy index with 4 fields: title, summary, category, word_count
- Title field gets 2x boost (more important than summary)
- Category field is stored but not indexed (for filtering, not searching)
- Support phrase queries with quotes
- Support boolean operators: AND, OR, NOT, (parentheses)
- Return top 10 results by default (configurable)

### Edge Cases & Error Handling

#### 1. Index Corruption
- **Scenario**: Index directory exists but is incomplete/corrupted
- **Current Behavior**: Would fail to read
- **New Behavior**: Detect corruption, rebuild index from source documents
- **Implementation**: Try to open index, if fails, clear directory and rebuild
- **Test Case**: Manually corrupt index file, call search(), verify rebuild works

#### 2. Query Failures
- **Scenario**: User enters invalid boolean query syntax
- **Current Behavior**: Hardcoded split() succeeds on anything
- **New Behavior**: Tantivy parser validates syntax, returns meaningful error
- **Implementation**: Match on parse error, provide hint to user
- **Test Case**: Try queries like `AND`, `(unclosed`, `rust OR`, verify error messages

#### 3. Schema Mismatches
- **Scenario**: Index was built with old schema (e.g., missing field), current code expects new schema
- **Current Behavior**: N/A (no index before)
- **New Behavior**: Version check in index metadata, reject if incompatible
- **Implementation**: Store schema version in index, check on load
- **Test Case**: Create index with v1 schema, try to open as v2, verify error + rebuild

#### 4. Missing/Empty Index
- **Scenario**: INDEX.json exists but no .tantivy_index directory
- **Current Behavior**: Can still search (scores everything against query in-memory)
- **New Behavior**: Same result, but use Tantivy index if available, fallback to memory if not
- **Implementation**: Try Tantivy first, fallback to manual scoring
- **Test Case**: Run search before indexing, verify it works (fallback path)

#### 5. Very Large Documents
- **Scenario**: Document has 1M+ tokens (very long markdown file)
- **Current Behavior**: BM25 calculation still works
- **New Behavior**: Tantivy handles efficiently, but may consume significant RAM
- **Implementation**: No special handling needed (Tantivy is designed for this)
- **Test Case**: Create 10MB document, index it, search for terms, verify performance <100ms

#### 6. Special Characters & UTF-8
- **Scenario**: Document contains emojis, non-ASCII languages, special symbols
- **Current Behavior**: Handled correctly by split() and string operations
- **New Behavior**: Tantivy tokenizer normalizes these (is this breaking change?)
- **Implementation**: Use UTF-8 tokenizer, document behavior change in migration guide
- **Test Case**: Index documents in Chinese, Arabic, with emojis, verify search works

#### 7. Performance Regression
- **Scenario**: Tantivy indexing is slower than old approach
- **Current Behavior**: No indexing before (in-memory only)
- **New Behavior**: First index build might take longer
- **Implementation**: Benchmark first build (should be <5 seconds for 1000 docs)
- **Test Case**: Index 1000 docs, measure time, assert <5 seconds

#### 8. Concurrent Access
- **Scenario**: Multiple processes try to write to index simultaneously
- **Current Behavior**: Not possible (in-memory)
- **New Behavior**: Second writer waits or fails with lock error
- **Implementation**: Tantivy handles this with file locks
- **Test Case**: Two threads try to write, verify only one succeeds

---

## Implementation Guidance

### Step 1: Create Tantivy Module (`src/search.rs`)

```rust
//! Full-text search using Tantivy
//!
//! Replaces custom BM25 with a proven, production-grade search engine.
//! Handles indexing, querying, and error recovery.

use anyhow::{anyhow, Result};
use std::path::Path;
use tantivy::{Document, Index, IndexWriter, Query, ReloadPolicy, Schema, Term};

/// Initialize Tantivy schema
pub fn create_schema() -> Schema {
    // Define fields: title (text + stored, boosted)
    // summary (text + stored), category (text + stored), word_count (u64)
}

/// Build or open Tantivy index
pub fn open_or_create_index(index_path: &Path) -> Result<Index> {
    // If index exists and valid, open it
    // If index corrupted, rebuild
    // If missing, create new
}

/// Index a batch of documents
pub fn index_documents(
    index: &Index,
    documents: Vec<IndexDocument>,
) -> Result<()> {
    // Create writer, add documents, commit
}

/// Search the index
pub fn search_index(
    index: &Index,
    query_str: &str,
    limit: usize,
) -> Result<Vec<SearchResult>> {
    // Parse query, execute search, return results
}

/// Fallback: score single document (for backward compatibility)
pub fn score_single_document(
    document: &str,
    query: &str,
    avg_doc_length: f32,
) -> f32 {
    // Uses simplified BM25 (current implementation)
    // For when Tantivy index is unavailable
}
```

### Step 2: Update Module Exports (`src/lib.rs`)

```rust
pub mod search;  // NEW: Tantivy-based full-text search
```

### Step 3: Update `src/filter.rs`

**Old Function** (lines 319-347):
```rust
pub fn bm25_score(document: &str, query: &str, avg_doc_length: f32) -> f32 {
    // Current simplified implementation
}
```

**New Function**:
```rust
pub fn bm25_score(document: &str, query: &str, avg_doc_length: f32) -> f32 {
    // DEPRECATED: Use crate::search::score_single_document instead
    // Kept for backward compatibility
    crate::search::score_single_document(document, query, avg_doc_length)
}
```

### Step 4: Update `src/scrape.rs`

**Old Usage** (line 464):
```rust
let score = bm25_score(&page.markdown, query, avg_doc_length);
```

**New Usage**:
```rust
// Try Tantivy index first, fall back to simple scoring
let score = match crate::search::score_single_document(&page.markdown, query, avg_doc_length) {
    Ok(s) => s,
    Err(_) => filter::bm25_score(&page.markdown, query, avg_doc_length),
};
```

### Step 5: Update `src/main.rs`

**Old run_search** (lines 508-620):
```rust
fn run_search(query: &str, index_dir: &Path, limit: usize, use_color: bool) -> Result<()> {
    // Current: Loads INDEX.json, manually scores all documents
    // New: Uses Tantivy index, much faster
}
```

**New run_search**:
```rust
fn run_search(query: &str, index_dir: &Path, limit: usize, use_color: bool) -> Result<()> {
    // 1. Try Tantivy index first
    // 2. If available, use it (much faster)
    // 3. If missing, fall back to INDEX.json + manual scoring
    // 4. Display results with scores and paths
}
```

### Step 6: Create Index During `run_index()`

**In `run_index()` function**:
```rust
// After creating INDEX.json, also build Tantivy index
let tantivy_index = crate::search::open_or_create_index(output)?;
crate::search::index_documents(&tantivy_index, &documents)?;
```

### Step 7: Add Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_create_schema() { /* Verify schema fields */ }

    #[test]
    fn test_index_and_search() { /* Index docs, search, verify results */ }

    #[test]
    fn test_phrase_queries() { /* "exact phrase" matching */ }

    #[test]
    fn test_boolean_queries() { /* AND, OR, NOT operators */ }

    #[test]
    fn test_field_queries() { /* title:rust searches only title */ }

    #[test]
    fn test_index_corruption_recovery() { /* Corrupt index, verify rebuild */ }

    #[test]
    fn test_missing_index_fallback() { /* Verify fallback to manual scoring */ }

    #[test]
    fn test_special_characters() { /* UTF-8, emojis, non-ASCII */ }

    #[test]
    fn test_large_document() { /* 1MB+ document indexing */ }

    #[test]
    fn test_concurrent_access() { /* Multiple writers */ }
}
```

### Step 8: Benchmark Comparison

```bash
# Before (current BM25 implementation)
time cargo run --release -- search "rust programming" --index-dir .

# After (Tantivy implementation)
time cargo run --release -- search "rust programming" --index-dir .

# Compare: Tantivy should be 10x faster for large indexes
```

---

## Verification Checklist

- [ ] **Compilation**: `cargo build --release` succeeds with no warnings
- [ ] **Tests**: `cargo test` passes (24 existing + 10 new tests)
- [ ] **Search Quality**: Tantivy results match or exceed old BM25 ranking
- [ ] **Performance**: Index build <5s for 1000 docs, search <100ms
- [ ] **Error Handling**: All edge cases return meaningful Err, no panics
- [ ] **Backward Compatibility**: `bm25_score()` still works (delegates to Tantivy)
- [ ] **Documentation**: Updated comments, added examples
- [ ] **Integration**: All three usage points (search, scrape, validate) updated

---

## Success Criteria

1. ✅ Custom BM25 code replaced with Tantivy
2. ✅ No panic attacks (all errors handled gracefully)
3. ✅ Search quality equal or better than before
4. ✅ Performance improved (faster searches)
5. ✅ All existing tests pass
6. ✅ All new edge case tests pass
7. ✅ Index corruption handled (auto-rebuild)
8. ✅ Query failures handled (meaningful errors)
9. ✅ Schema mismatches handled (version checks)
10. ✅ Bead closed with completion summary

---

## Time Estimate

- Step 1-2: Create module + exports: **30 min**
- Step 3-5: Update integration points: **45 min**
- Step 6-7: Indexing + tests: **60 min**
- Step 8: Benchmarking: **30 min**
- Total: **~2.5 hours**

---

## Rollback Plan

If Tantivy integration causes issues:
1. Keep old `bm25_score()` function
2. Add feature flag: `[features] tantivy_search = ["tantivy"]`
3. Disable Tantivy: `--no-default-features`
4. Falls back to in-memory scoring (current behavior)

---

**Created**: 2026-01-11
**Next**: Execute Step 1 - Create Tantivy module
