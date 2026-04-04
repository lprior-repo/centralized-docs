# Implementation Summary: cdocs-2ey

## Bead: action: wire scrape command to one shared read session and one shutdown commit

## Files Changed

| File | Change | Lines |
|------|--------|-------|
| `src/calc/scrape_diff.rs` | Implemented `classify_scrape_diff` and `build_scrape_state_changes` | ~80 lines |
| `src/cmd/scrape.rs` | Wired state operations into `run_scrape` | ~130 lines |
| `src/main.rs` | Added `pub mod calc;` for binary crate access | 1 line |

## Implementation Details

### 1. `classify_scrape_diff` (src/calc/scrape_diff.rs)

**Pure function** — classifies scraped pages against stored URL states.

- Uses `fold` over `scraped_pages` to build three partition buckets
- Computes `SHA-256(markdown)` for each page via `hash_content`
- Compares against `stored_url_states[url].content_hash`
- Classification: New (absent from stored), Unchanged (hash match), Changed (hash differs)
- Zero-hash `[0u8; 32]` is treated as a valid SHA-256 output (not a sentinel)
- Deduplicates URLs in input (first occurrence wins) for proptest robustness

**Constraint adherence**: Pure function, zero I/O, zero `mut`, iterator pipelines, expression-based.

### 2. `build_scrape_state_changes` (src/calc/scrape_diff.rs)

**Pure function** — builds atomic `StateChanges` batch from scrape diff.

- Creates one `PersistedScrapeResult` per active (new+changed) page
- Each page gets its own `new_scrapes` entry keyed by `SHA-256(rkyv_bytes)`
- Sets `url_hash` on each `UrlStateRaw` to reference the corresponding `new_scrapes` key (INV-6 reference integrity)
- `content_hash` = `SHA-256(markdown)` per page
- `last_fetched_secs` = provided timestamp parameter
- `status_code` = 200 for all pages
- Unchanged pages produce zero entries (excluded from all output fields)
- Uses `..StateChanges::empty()` spread for all other fields

**Constraint adherence**: Pure function, zero I/O, zero `mut`, iterator pipelines with `filter_map`, `collect`.

### 3. `run_scrape` wiring (src/cmd/scrape.rs)

**Action function** — wires state database into the scrape command pipeline.

Following the `run_index` pattern:

1. **Post-validation, pre-scrape**: Open `StateDb` at `output.join("state.redb")`, create `StateReadSession`, load stored URL states
2. **Post-scrape**: Classify scraped pages via `classify_scrape_diff` against stored states
3. **Reuse unchanged pages**: Load persisted scrape outputs for unchanged pages via `session.load_scrapes()`, deserialize back to runtime `ScrapedPage`
4. **Build combined result**: Merge reused + fresh pages via `build_combined_scrape_result`
5. **Drop read session** (INV-3): Explicit `drop(session)` before commit
6. **Continue existing pipeline**: Query filter, SPA detection, validation, write to disk
7. **Build state changes**: `build_scrape_state_changes(&scrape_diff, &all_pages, now_secs)`
8. **Single commit**: `state_db.commit_changes(state_changes)` with error context

**Error context strings** (per contract.md:75-78):
- `"failed to open state database: {e}"` — StateDb::open
- `"failed to create read session: {e}"` — StateReadSession::new
- `"failed to load URL states: {e}"` — session.load_url_states
- `"failed to load scrape outputs: {e}"` — session.load_scrapes
- `"failed to commit scrape state: {e}"` — commit_changes

**INV-1**: Zero per-page writes. All writes batched into single `StateChanges`.
**INV-2**: Pre-commit errors abort before `commit_changes`; state unchanged.
**INV-3**: `session` dropped before `commit_changes` (redb constraint).
**INV-4**: Same transaction model as `run_index`.
**INV-8**: Function signature unchanged: `pub async fn run_scrape(url, output, config) -> Result<()>`.

## Test Results

- **34/34** `calc::scrape_diff` unit tests pass (classify + build + proptests)
- **1135/1135** lib tests pass
- **2 pre-existing failures** in `tests/analysis_reuse_tests.rs` (unrelated to this bead)
