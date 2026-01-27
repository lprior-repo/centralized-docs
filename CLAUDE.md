# CLAUDE.md - Project Instructions for Claude Code

## Critical Rules

### ALWAYS Use Moon - NEVER Use Cargo Directly
**ABSOLUTE RULE: ALL build operations MUST go through Moon. NEVER use raw cargo commands.**

```bash
# ✅ CORRECT - Always use Moon
moon run :quick      # Format + lint check (parallel, cached)
moon run :clippy     # Lint with strict clippy rules
moon run :fmt        # Check formatting
moon run :fmt-fix    # Auto-fix formatting
moon run :test       # Run all tests (workspace-wide)
moon run :test-doc   # Run documentation tests
moon run :build      # Release build (cached)
moon run :check      # Fast type check
moon run :ci         # Full pipeline (all checks in parallel)
moon run :quality    # All quality gates
moon run :install    # Install binaries to ~/.local/bin

# ❌ WRONG - NEVER do this
cargo fmt            # NO
cargo clippy         # NO
cargo test           # NO
cargo build          # NO
cargo check          # NO
cargo clean          # NO - use `moon run :clean` instead
```

**Why Moon?**
- Persistent caching across sessions (runs in ms when cached)
- Parallel task execution
- Workspace-wide operations (all crates together)
- Dependency-aware rebuilds

### NEVER Touch Clippy/Lint Configuration
**ABSOLUTE RULE: DO NOT MODIFY clippy or linting configuration files. EVER.**

This includes but is not limited to:
- `.clippy.toml`
- `clippy.toml`
- Any `#![allow(...)]` or `#![deny(...)]` attributes in `lib.rs` or `main.rs`
- Clippy-related sections in `Cargo.toml`
- Any lint configuration in `.moon` or build scripts

If clippy reports warnings or errors, fix the **code**, not the lint rules.
The user has explicitly configured these rules. Do not second-guess them.

### Code Quality: Pure Functional Rust
**Zero panics, zero unwraps, explicit errors:**
- `unwrap()` and `expect()` are **forbidden** in production code
- `panic!`, `todo!`, `unimplemented!` are **forbidden**
- All errors must use `Result<T, Error>` with proper propagation
- Use functional patterns: `map`, `and_then`, `?` operator
- For arithmetic: use `saturating_add`, `saturating_sub`, `checked_*` methods

**Allowed in test code only:**
- `unwrap()` in test assertions (with justification in comment)
- `panic!()` in test setup (with comment explaining why)
- Use `#[expect(clippy::expect_used)]` for hardcoded regex patterns

### Project Structure
```
centralized-docs/
├── doc_transformer/       # Main library (indexing, search, MCP server)
├── contextual-chunker/    # Semantic document chunking
├── llms-txt-parser/       # llms.txt file format parsing
└── Cargo.toml             # Workspace configuration
```

### Key Architectural Decisions
- **HNSW Index**: O(log n) similarity search with cosine distance
- **Tantivy**: Full-text search with BM25 ranking
- **Knowledge Graph**: DAG-based document relationships (petgraph)
- **MCP Server**: Model Context Protocol for AI agent integration
- **Semantic Chunking**: Context-aware token-based splits

### Dependencies
- `hnsw_rs`: Approximate nearest neighbor search
- `tantivy`: Full-text search engine
- `petgraph`: Graph algorithms for document relationships
- `pulldown-cmark`: CommonMark parsing with AST
- `spider`: Web scraping with content extraction

### Quality Gates
All code must pass (using Moon):
1. `moon run :fmt` - Code formatting check
2. `moon run :clippy` - Zero warnings (strict mode)
3. `moon run :test` - All tests pass
4. `moon run :test-doc` - Documentation tests pass

Or simply run: `moon run :ci` for everything in parallel.
