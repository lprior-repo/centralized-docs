---
bead_id: cdocs-5d8
bead_title: "QA: ctd diff/apply require hidden .scrape dir"
phase: p2-implementation
updated_at: 2026-04-20T02:30:00Z
---

# Implementation Summary: `resolve_manifest_dir`

## Status: GREEN — All 29 target tests pass

## What Changed

### 1. `centralized-docs/src/watch/diff.rs`

**`resolve_manifest_dir` — implemented from `todo!()` stub** (lines 40–55)

Pure calculation function that resolves a user-supplied path to the directory containing `manifest.json`:
- Checks `path/manifest.json` first (direct match, takes precedence per contract Open Question 3)
- Falls back to `path/.scrape/manifest.json` (nested match, what `ctd scrape` produces)
- Returns `ManifestResolveError::NotFound` with all 4 diagnostic fields when neither exists
- Uses `Path::join` throughout (handles trailing slashes, spaces, unicode correctly)
- Expression-based `if/else` chain (clippy `match_bool` compliant)

**`diff_directories` — updated to use `resolve_manifest_dir`** (lines 168–172)

Before reading manifests, both `dir_a` and `dir_b` are now resolved via `resolve_manifest_dir`. The resolved paths are used instead of raw paths for manifest reading.

### 2. `centralized-docs/src/cmd/watch.rs`

**`read_manifest` — updated to use `resolve_manifest_dir`** (line 192–198)

Before joining `manifest.json`, the `scrape_dir` is resolved via `resolve_manifest_dir`. The resolved path is used for manifest reading. This fixes both `run_apply` and transitively any caller of `read_manifest`.

**Import updated** — added `resolve_manifest_dir` to the `use crate::watch::{...}` import.

## Constraint Adherence

### Big 6

| Constraint | Evidence |
|---|---|
| **Data → Calc → Actions** | `resolve_manifest_dir` is a pure calculation (only `exists()` checks, no I/O mutation). `diff_directories` and `read_manifest` are the Actions boundary. |
| **Zero Mutability** | No `mut` keyword in any changed function. All paths constructed via `Path::join` (immutable). |
| **Zero Panics/Unwraps** | No `unwrap()`, `expect()`, or `panic!()` in non-test code. Error propagated via `Result`. |
| **Illegal States Unrepresentable** | `ManifestResolveError::NotFound` is a typed enum with 4 mandatory `PathBuf` fields — impossible to construct without all diagnostic paths. |
| **Expression-Based** | `resolve_manifest_dir` is a single `if/else if/else` expression returning `Result`. |
| **Clippy Flawless** | `cargo clippy --lib` exits 0 under `-D warnings -D clippy::unwrap_used -D clippy::panic -D clippy::expect_used -W clippy::pedantic`. |

### Postconditions Verified

| Postcondition | Test Coverage |
|---|---|
| Post1: Direct match returns input unchanged | B1, B6, B34 |
| Post2: Nested match returns `path/.scrape` | B2, B8, B31, B32, B33 |
| Post3: Neither → NotFound with 4 fields | B3 |
| Post4: Path form preserved (relative/absolute) | B5, B6 |
| Post5: No side effects | B9 |
| Direct precedence over nested | B4, P5 proptest |
| Determinism | B8, P1 proptest |
| Path identity (INV3) | P2 proptest |
| `diff_directories` integration | B10–B16, B35–B37 |
| `write_scraped_pages` roundtrip | P3 proptest |
| Summary conservation | P4 proptest |

## Error Taxonomy

`ManifestResolveError::NotFound` contains:
- `path`: user-supplied path
- `scrape_subdir`: `path/.scrape`
- `direct`: `path/manifest.json`
- `nested`: `path/.scrape/manifest.json`

Display format includes all paths plus actionable `Tip:` guidance.

## Test Results

```
29 tests run: 29 passed, 3657 skipped  (resolve_manifest + related)
12 tests run: 12 passed                 (diff_directories integration)
3505 tests run: 3497 passed, 8 failed   (full package — 8 failures are pre-existing playwright/E2E browser tests)
```

All 8 failures are pre-existing playwright browser tests requiring a running Chromium instance — completely unrelated to this bead.
