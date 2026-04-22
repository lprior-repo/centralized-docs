# Architecture Refactor Report — 5-File Split

## STATUS: REFACTORED

All five files successfully split. Every resulting file is under 300 lines.
`cargo check` and `cargo clippy` pass with zero errors (only pre-existing warnings).

---

## Line Counts: Before → After

| Original File | Before | After | Split Into |
|---|---|---|---|
| `cli/mod.rs` | 377 | 8 | + `commands.rs` (267), `spider_args.rs` (65) |
| `cli/validation.rs` | 314 | 242 | + `validation_hnsw.rs` (82) |
| `cmd/index_state.rs` | 363 | 235 | + `index_state_cleanup.rs` (140) |
| `main.rs` | 322 | 78 | + `dispatch.rs` (256) |
| `mcp/server.rs` | 317 | 255 | + `server_search_index.rs` (78) |

**Max file size: 267 lines** (`commands.rs`) — well under 300.

---

## Refactoring Details

### 1. `cli/mod.rs` (377 → 8) — Extracted Commands enum + shared args
- **`cli/commands.rs`** (267 lines): `Cli` struct + `Commands` enum with `#[command(flatten)]`
  to deduplicate 11 shared web-scraping fields across Scrape/Ingest/Watch.
- **`cli/spider_args.rs`** (65 lines): `SpiderCoreArgs` (7 fields: filter, delay, timeouts,
  retries, redirect, concurrency) + `SpiderCrawlArgs` (4 fields: max_page_bytes,
  max_total_bytes, query, threshold). Cleanly models the shared vs. command-specific split.
- `cli/mod.rs` reduced to module declarations + re-exports.

### 2. `cli/validation.rs` (314 → 242) — Extracted HNSW validators
- **`cli/validation_hnsw.rs`** (82 lines): `validate_max_related_chunks`,
  `validate_max_chunk_keywords`, `validate_hnsw_m`, `validate_hnsw_ef_construction`.
- Re-exported from `validation.rs` via `pub(crate) use hnsw::*`.
- Test modules (`validation_tests_*.rs`) unchanged — they import via `super::*`.

### 3. `cmd/index_state.rs` (363 → 235) — Extracted cleanup logic
- **`cmd/index_state_cleanup.rs`** (140 lines): `cleanup_deleted_outputs`,
  `output_prefix`, `collect_matching_files`.
- Core state-building functions remain: `load_cached_analyses`, `serialize_analysis`,
  `build_state_changes`.

### 4. `main.rs` (322 → 78) — Extracted command dispatch
- **`dispatch.rs`** (256 lines): `dispatch()` (async command router), `handle_error()`,
  `exit_clap()`.
- `main()` is now minimal: parse args → dispatch → handle result.
- Added `mod dispatch;` to module declarations.

### 5. `mcp/server.rs` (317 → 255) — Extracted search index management
- **`mcp/server_search_index.rs`** (78 lines): `ensure_search_index`, `build_new_index`,
  `populate_index`, `write_documents_to_index` as free functions.
- Server calls `search_index::ensure_search_index()` instead of `Self::ensure_search_index()`.

---

## Verification

- `cargo check -p centralized-docs` (lib + bin): **PASS** (zero errors)
- `cargo clippy -p centralized-docs --lib`: **PASS** (zero errors)
- Test compilation: 4 pre-existing errors in `watch/tests_diff.rs` and `watch/tests_format.rs`
  (reference removed `ScrapeResult` type — from other agents' parallel `watch/` refactoring,
  not caused by this refactor).

## DDD Observations

- `SpiderCoreArgs` / `SpiderCrawlArgs` eliminate primitive obsession at the CLI boundary —
  related args are grouped into meaningful types that act as documentation.
- `dispatch.rs` separates the "which command?" decision from the entry point,
  following single-responsibility.
- No `unwrap()`/`expect()`/`panic`/`unsafe` introduced.
