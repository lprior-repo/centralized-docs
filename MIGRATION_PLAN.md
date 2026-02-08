# Centralized Docs Library Migration Plan

**Date**: 2026-02-07  
**Status**: Analysis Complete  
**Target**: Migration from deprecated crates to modern alternatives

---

## Executive Summary

This document outlines the comprehensive migration plan for the centralized-docs project, covering:
- tiktoken-rs 0.5 -> tiktoken 0.6+ migration
- pulldown-cmark verification (already integrated)
- tantivy verification (already integrated)

**Key Findings**:
- Spider 2.39 is acceptable and required (no changes needed)
- pulldown-cmark 0.13 is correctly integrated in doc_transformer
- tantivy 0.25 is correctly integrated in doc_transformer
- tiktoken-rs 0.5 in contextual-chunker needs migration to tiktoken 1.0+
- tiktoken-rs 0.5 also declared in doc_transformer (can be removed after migration)

---

## Current State Analysis

### Project Structure

```
centralized-docs/
├── Cargo.toml (workspace root)
├── contextual-chunker/         # Chunking library
│   ├── Cargo.toml
│   └── src/
│       ├── chunk.rs            # Token estimation uses tiktoken-rs 0.5
│       ├── document.rs
│       └── lib.rs
├── doc_transformer/            # Document transformation
│   ├── Cargo.toml
│   └── src/
│       ├── transform.rs        # Uses pulldown-cmark 0.13
│       ├── search.rs           # Uses tantivy 0.25
│       ├── analyze.rs
│       └── lib.rs
└── llms-txt-parser/            # llms.txt parser
    ├── Cargo.toml
    └── src/
```

### Dependency Inventory

#### contextual-chunker/Cargo.toml
```toml
tiktoken-rs = "0.5"  # Line 20 - NEEDS MIGRATION
```

#### doc_transformer/Cargo.toml
```toml
tiktoken-rs = "0.5"  # Line 42 - NEEDS MIGRATION (can remove after)
pulldown-cmark = { version = "0.13", default-features = false, features = ["html"] }  # Line 64 - OK
tantivy = "0.25"  # Line 61 - OK
```

### Code Usage Analysis

#### tiktoken-rs Usage (contextual-chunker/src/chunk.rs:647-655)

```rust
/// Estimate token count using tiktoken cl100k_base tokenizer
/// Falls back to character approximation if tokenizer unavailable
fn estimate_tokens(text: &str) -> usize {
    tiktoken_rs::cl100k_base()
        .ok()
        .map_or_else(|| (text.len() / 4).max(1), |bpe| {
            bpe.encode_with_special_tokens(text).len()
        })
}
```

**Current Behavior**:
- Calls `tiktoken_rs::cl100k_base()` to get tokenizer
- Returns `Option<BPE>` which is unwrapped with `.ok()`
- Falls back to 4 chars ≈ 1 token if unavailable
- Uses `encode_with_special_tokens()` method

#### pulldown-cmark Usage (doc_transformer/src/transform.rs:52, 192-197)

```rust
use pulldown_cmark::{CowStr, Event, Options, Parser, Tag, TagEnd};

/// Parse markdown using pulldown-cmark with full CommonMark + GFM support
fn parse_markdown(content: &str) -> Vec<Event<'_>> {
    let options = Options::all();
    let parser = Parser::new_ext(content, options);
    parser.collect()
}
```

**Current Behavior**:
- Imports core types: `CowStr, Event, Options, Parser, Tag, TagEnd`
- Creates parser with all options enabled
- Collects events into Vec for processing

#### tantivy Usage (doc_transformer/src/search.rs:29-33)

```rust
use tantivy::collector::TopDocs;
use tantivy::doc;
use tantivy::query::QueryParser;
use tantivy::schema::{Field, Schema, Value, STORED, TEXT};
use tantivy::Index;
```

**Current Behavior**:
- Uses standard tantivy 0.25 APIs
- Implements full-text search with BM25 scoring
- Custom schema for document indexing

---

## Migration Plan: tiktoken-rs → tiktoken

### Step 1: Update Dependencies

#### contextual-chunker/Cargo.toml

```toml
# BEFORE
tiktoken-rs = "0.5"

# AFTER
tiktoken = "1.0"
```

#### doc_transformer/Cargo.toml

```toml
# BEFORE
tiktoken-rs = "0.5"

# AFTER
# Remove this line entirely after migration is complete
# tiktoken is now a transitive dependency via contextual-chunker
```

**Rationale**: 
- `contextual-chunker` is a dependency of `doc_transformer`, so updating the former will propagate to the latter
- Once `contextual-chunker` uses tiktoken, `doc_transformer` can remove its direct tiktoken-rs dependency

---

### Step 2: Update Import Statements

#### contextual-chunker/src/chunk.rs

```rust
// BEFORE
use tiktoken_rs::{BPE, cl100k_base};

// AFTER
use tiktoken::{BPE, cl100k_base};
```

---

### Step 3: Update Function Implementation

#### contextual-chunker/src/chunk.rs:647-655

```rust
// BEFORE
fn estimate_tokens(text: &str) -> usize {
    tiktoken_rs::cl100k_base()
        .ok()
        .map_or_else(|| (text.len() / 4).max(1), |bpe| {
            bpe.encode_with_special_tokens(text).len()
        })
}

// AFTER
fn estimate_tokens(text: &str) -> usize {
    cl100k_base()
        .ok()
        .map_or_else(|| (text.len() / 4).max(1), |bpe| {
            bpe.encode_with_special_tokens(text, &[]).len()
        })
}
```

**Changes**:
1. `tiktoken_rs::cl100k_base()` → `cl100k_base()` (direct import)
2. `encode_with_special_tokens(text)` → `encode_with_special_tokens(text, &[])` (added second parameter)

**API Change**: tiktoken 1.0 requires a second parameter for special tokens. Passing `&[]` maintains the same behavior as the old API.

---

### Step 4: Update llms-txt-parser/Cargo.toml (Optional)

If llms-txt-parser needs token estimation in the future:

```toml
[dependencies]
# Add if you need token estimation
tiktoken = "1.0"
```

---

## Verification Steps

### Test tiktoken Migration

```bash
# Run type checks
moon run :check

# Run tests
moon run :test -- contextual-chunker

# Test token estimation specifically
moon run :test -- estimate_tokens
```

### Verify pulldown-cmark

```bash
# Run full pipeline to test markdown parsing
moon run :test -- doc_transformer

# Check for any pulldown-cmark related warnings
moon run :quick
```

### Verify tantivy

```bash
# Test search functionality
moon run :test -- search

# Verify index operations
moon run :test -- index

# Run full integration tests
moon run :ci
```

---

## Before/After Code Examples

### Example 1: Token Estimation

**Before**:
```rust
use tiktoken_rs::cl100k_base;

fn estimate_tokens(text: &str) -> usize {
    cl100k_base()
        .ok()
        .map_or_else(|| (text.len() / 4).max(1), |bpe| {
            bpe.encode_with_special_tokens(text).len()
        })
}

// Usage
let count = estimate_tokens("Hello, world!"); // Returns token count
```

**After**:
```rust
use tiktoken::cl100k_base;

fn estimate_tokens(text: &str) -> usize {
    cl100k_base()
        .ok()
        .map_or_else(|| (text.len() / 4).max(1), |bpe| {
            bpe.encode_with_special_tokens(text, &[]).len()
        })
}

// Usage
let count = estimate_tokens("Hello, world!"); // Returns token count (same result)
```

### Example 2: Markdown Parsing (No Changes Needed)

**Current (already working)**:
```rust
use pulldown_cmark::{Event, Options, Parser};

fn parse_markdown(content: &str) -> Vec<Event<'_>> {
    let options = Options::all();
    let parser = Parser::new_ext(content, options);
    parser.collect()
}
```

---

## Testing Commands Reference

### Quick Validation (Development)
```bash
# Fast type check (cached)
moon run :check

# Fast check with linting
moon run :quick

# Auto-fix formatting
moon run :fmt-fix
```

### Full Pipeline Validation
```bash
# Build release
moon run :build

# Run all tests
moon run :test

# Full CI pipeline
moon run :ci

# Quality gates
moon run :quality
```

### Targeted Testing
```bash
# Test specific crate
moon run :test -- contextual-chunker

# Test with specific filter
moon run :test -- estimate

# Run benchmarks
moon run :test -- --bench

# Test search functionality
moon run :test -- search_index
```

---

## Estimated Timeline

| Task | Estimate | Status |
|------|----------|--------|
| Update Cargo.toml dependencies | 5 min | Ready |
| Update import statements | 5 min | Ready |
| Update function implementation | 10 min | Ready |
| Run type checks | 10 min | Ready |
| Run tests | 20 min | Ready |
| Run full CI pipeline | 30 min | Ready |
| **Total** | **~80 min** | **Ready** |

---

## Risk Assessment

### Low Risk ✅
- **Dependency Update**: Simple string replacement in Cargo.toml
- **Import Changes**: Direct 1:1 mapping (tiktoken_rs → tiktoken)
- **Functionality**: API is backward compatible (with minor parameter change)

### Mitigation Strategies
1. **Test thoroughly**: Run full test suite before and after
2. **CI validation**: Ensure all quality gates pass
3. **Rollback plan**: Keep tiktoken-rs branch in git history

### Breaking Changes to Watch
- `encode_with_special_tokens()` now requires a second parameter for special tokens
- **Solution**: Pass `&[]` to maintain current behavior

---

## Post-Migration Cleanup

### Remove Old Dependencies

After migration and verification, remove tiktoken-rs from doc_transformer:

```toml
# Remove this line from doc_transformer/Cargo.toml
tiktoken-rs = "0.5"
```

### Verify No Breaking Changes

```bash
# Check for unused dependencies
cargo tree -i tiktoken-rs  # Should show no direct dependencies

# Check for any remaining tiktoken_rs references
grep -r "tiktoken_rs" src/

# Should return no results after migration
```

---

## Rollback Plan

If issues arise:

```bash
# Revert dependencies
git checkout Cargo.toml

# Revert source changes
git checkout src/chunk.rs

# Clean and rebuild
moon run :clean
moon run :build
```

---

## Notes

### Why tiktoken over tiktoken-rs?

1. **Active maintenance**: tiktoken is the maintained successor
2. **Performance improvements**: Optimized tokenization algorithms
3. **Better API**: Cleaner function signatures
4. **Ecosystem**: Used by major Rust LLM projects

### Spider 2.39 Decision

**No upgrade needed**. The project comment explains:
> Using 2.39 to avoid cache_mem compilation issues in 2.40+

This is a conscious decision to maintain stability over features.

### pulldown-cmark 0.13 Status

**Already current**. Version 0.13 is the latest stable release with:
- Full CommonMark support
- GFM extensions (tables, task lists, etc.)
- Stable API

### tantivy 0.25 Status

**Already current**. Version 0.25 is the latest stable release with:
- Production-grade BM25 implementation
- Full-text search capabilities
- Schema flexibility

---

## References

- tiktoken crate docs: https://docs.rs/tiktoken
- tiktoken-rs crate: https://crates.io/crates/tiktoken-rs
- pulldown-cmark docs: https://docs.rs/pulldown-cmark
- tantivy docs: https://docs.rs/tantivy

---

## Sign-off

**Migration Plan Created**: 2026-02-07  
**Analysis by**: opencode  
**Ready for implementation**: Yes  
**Tested**: N/A (plan only)  
**Status**: Ready for execution
