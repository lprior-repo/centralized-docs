# Implementation Summary

```
bead_id: cdocs-r4f
title: calc: build URL-state and scrape-output commit batches from scrape results
phase: state-3-implementation
date: 2026-04-04
status: complete
```

## Changed Files

| File | Change |
|------|--------|
| `src/calc/build_scrape_state_changes.rs` | Replaced stub implementations with production code; added 2 doc-comment backtick fixes |

## Implementation

### `build_url_state_raw`

Pure constructor that builds a `UrlStateRaw` (120-byte `#[repr(C)]` Pod struct) from
individual field values. `reserved` is zeroed per INV-8. All four input fields are set
directly — no computation, no I/O.

### `build_scrape_state_changes`

Pure calculation function mirroring `build_file_state_changes` but for the URL-state
domain. Pipeline:

1. **PRE-1 (EmptyDiff)**: Returns `Err(EmptyDiff)` when all four `ScrapeDiff` buckets
   are empty.
2. **PRE-2 (DuplicateUrl)**: Collects all URLs from all four buckets, deduplicates via
   `HashSet`, returns `Err(DuplicateUrl)` with the first offending URL if any duplicate
   is found. Uses the same O(n²) first-duplicate pattern as `check_no_duplicates` in
   `build_state_changes.rs`.
3. **Processing**: Chains `diff.changed` then `diff.new_urls` (preserving POST-10
   ordering). For each URL, `process_single_url` validates:
   - **PRE-3 (MissingScrapeArtifact)**: Artifact exists in `ScrapeOutputs`
   - **PRE-4 (EmptyScrapePayload)**: `payload_bytes` is non-empty
   Then computes `url_hash = hash_payload(&artifact.payload_bytes)` (infallible SHA-256)
   and builds the `UrlStateRaw` row.
4. **Assembly**: Builds `updated_urls`, `new_scrapes`, and `deleted_urls` vectors.
   All file-state fields (`updated_files`, `deleted_files`, `new_analyses`, etc.) are
   left empty per POST-9.

### `process_single_url` (internal helper)

Validates a single URL's artifact and produces a `ScrapeEntry` (url, state, url_hash,
payload_bytes). Returns appropriate `ScrapeBatchBuildError` variants on precondition
violations.

### `check_no_duplicate_urls` (internal helper)

Mirrors `check_no_duplicates` from `build_state_changes.rs`. Collects all URLs from
all four `ScrapeDiff` categories, checks `HashSet` size matches total count, returns
first duplicate found.

## Constraint Adherence

| Constraint | Evidence |
|------------|----------|
| **Data → Calc → Actions** | All functions are pure calculations. No I/O, no state mutation, no logging. Input references borrowed; output owned. |
| **Zero mutability** | No `mut` keyword in non-test code. All accumulation via `collect()`, `map()`, `chain()`. |
| **Zero panics/unwrap** | No `.unwrap()`, `.expect()`, or `panic!()` in production code. All fallible operations use `Result` propagation via `?` or `ok_or_else`. |
| **Make illegal states unrepresentable** | `ScrapeDiff` partitions enforce mutual exclusivity via runtime validation (PRE-2). `ScrapeBatchBuildError` exhaustively covers all precondition violations. |
| **Expression-based** | All logic is expression-based (early returns on error, iterator chains for processing). |
| **Determinism (POST-10)** | Output order: `changed` URLs first, then `new_urls`, then `deleted`. No `HashMap` iteration order leak. |

## Contract Clause Mapping

| Clause | Implementation |
|--------|---------------|
| PRE-1 (diff non-empty) | Line: `if diff.unchanged.is_empty() && ...` → `Err(EmptyDiff)` |
| PRE-2 (no duplicates) | `check_no_duplicate_urls()` with `HashSet` dedup |
| PRE-3 (artifacts exist) | `outputs.artifacts.get(url).ok_or_else(...)` in `process_single_url` |
| PRE-4 (non-empty payload) | `artifact.payload_bytes.is_empty()` check in `process_single_url` |
| POST-1 (unchanged → no output) | `unchanged` URLs are never iterated — only `changed` + `new_urls` chained |
| POST-2/3 (changed/new → rows + payloads) | `process_single_url` produces both `UrlStateRaw` and `(hash, bytes)` |
| POST-4 (deleted → delete only) | `deleted_urls: diff.deleted.clone()` with no corresponding payload |
| POST-5 (reference integrity) | `url_hash = hash_payload(&payload_bytes)` used as both FK and `new_scrapes` key |
| POST-6 (content_hash fidelity) | `build_url_state_raw(artifact.content_hash, ...)` |
| POST-7 (timestamp fidelity) | `build_url_state_raw(..., config.now_secs, ...)` |
| POST-8 (status_code fidelity) | `build_url_state_raw(..., artifact.status_code)` |
| POST-9 (file fields empty) | All 7 file-state fields set to `vec![]` |
| POST-10 (determinism) | Iterator chain: `changed.iter().chain(new_urls.iter())` preserves input order |
| INV-8 (reserved zeroed) | `reserved: [0u8; 46]` in `build_url_state_raw` |

## Test Results

```
49 passed; 0 failed; 0 ignored
```

All unit tests (B01–B23 + MIX + edge cases + error display) and all 4 proptests (P1–P4) pass.
Pre-existing failures in `analysis_reuse_tests.rs` (2 tests) are unrelated to this bead.
