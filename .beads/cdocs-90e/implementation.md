# Implementation Summary: cdocs-90e

## Changed Files

| File | Change |
|------|--------|
| `src/scrape_reuse.rs` | Implemented 3 stubbed functions + 1 helper |

## Functions Implemented

### 1. `merge_scrape_pages_in_order` (Pure Calculation)
- **Layer**: Calculation
- **Behavior**: Enumerates `fresh_pages` by index; for each position, selects the archived version from `archived_pages` HashMap if present, otherwise keeps the fresh page.
- **POST-1/POST-4**: Output length equals input length; order preserved.
- **No I/O, no errors, no mutation** (uses iterator pipeline with `.enumerate().map().collect()`).

### 2. `load_archived_scrape_pages` (I/O Action)
- **Layer**: Action
- **Behavior**:
  1. Early return for empty `unchanged` list.
  2. Collects `(index, url, url_hash, content_hash)` for each unchanged page, filtering out pages with zero `url_hash` (INV-6: never archived) or missing `url_states` entries.
  3. Deduplicates `url_hash` values and bulk-loads `PersistedScrapeResult` archives via `session.load_scrapes()`.
  4. Intercepts `BulkLoadError::CorruptPayload` and converts to `ScrapeReuseError::DeserializationFailed` per contract error taxonomy.
  5. For each unchanged page, extracts the matching page by URL from its batch, converts via `persisted_scraped_page_to_runtime`, and verifies content hash integrity (INV-8).
  6. Non-fatal failures (missing batch, empty batch, deserialization failure, hash mismatch) result in the page index being added to `fallback_indices`.
- **INV-2**: All DB reads flow through `StateReadSession`.
- **INV-3**: Batch deserialization failure is per-batch fatal (propagated as `DeserializationFailed`).
- **INV-4**: Individual page failures are non-fatal (fallback).

### 3. `scrape_with_reuse` (Entry Point)
- **Layer**: Action (orchestrator)
- **Behavior**:
  1. Early return for empty input (POST-5: 0 + 0 == 0).
  2. Computes SHA-256 content hashes for all pages.
  3. Loads `url_states` from database via `session.load_url_states()`.
  4. Classifies pages via `classify_scraped_pages`.
  5. Loads archived pages via `load_archived_scrape_pages`.
  6. Merges via `merge_scrape_pages_in_order`.
  7. Computes stats: `reused = archived_pages.len()`, `scraped = total - reused`.
- **POST-1**: Every page appears exactly once.
- **POST-5**: `reused + scraped == total_pages`.

### 4. `load_single_page_from_batch` (Private Helper)
- **Layer**: Calculation (pure, returns `Result<ScrapedPage, ()>`)
- **Behavior**: Deserializes an `OwnedArchive<PersistedScrapeResult>`, finds the page matching `expected_url`, converts to runtime `ScrapedPage`, and verifies content hash integrity. Returns `Err(())` for any failure (deserialization, missing page, hash mismatch).

## Constraint Adherence

| Constraint | Status | Evidence |
|-----------|--------|----------|
| Data → Calc → Actions | ✅ | `ScrapePageDiff`, `ScrapeReuseStats`, error types are Data; `classify_scraped_pages`, `merge_scrape_pages_in_order`, `compute_page_content_hash` are pure Calc; `load_archived_scrape_pages`, `scrape_with_reuse` are Actions |
| Zero mutability | ✅ | No `mut` in production code paths; `HashMap::insert` uses `fold`/`collect` patterns; `load_archived_scrape_pages` uses `filter_map` + `collect` |
| Zero panics/unwrap | ✅ | No `unwrap()`/`expect()`/`panic!()` outside `#[cfg(test)]`; `classify_scraped_pages` uses `assert_eq!` for precondition (programmer error, documented in contract) |
| Illegal states unrepresentable | ✅ | `ScrapePageDiff` uses two separate `Vec<usize>` fields (cannot overlap); `ScrapeReuseStats` has only valid counts |
| Expression-based | ✅ | All functions use iterator chains, `match`, `map_err`, `if let` |
| Clippy flawless | ✅ | `cargo clippy -D warnings` produces zero issues for `scrape_reuse.rs` |

## Test Results

- **27 unit tests**: All pass (hash computation, classification, merge, domain types, error Display)
- **15 integration tests**: All pass (DB-backed load, multi-batch, empty batch, corrupt bytes, hash mismatch, missing rows, error propagation, entry point scenarios)
- **Total**: 42/42 passing

## Error Taxonomy Compliance

| Variant | Production Usage |
|---------|-----------------|
| `StateLoad` | Propagated from `session.load_url_states()` via `#[from]` |
| `BulkLoad` | Propagated from `session.load_scrapes()` (non-CorruptPayload variants) via `map_err` |
| `DeserializationFailed` | Converted from `BulkLoadError::CorruptPayload` via `map_err` |
| `HashMismatch` | Handled internally as non-fatal fallback (logged conceptually; contract allows per-page warning) |
| `MissingUrlState` | Handled internally as non-fatal fallback (INV-7 classification) |
