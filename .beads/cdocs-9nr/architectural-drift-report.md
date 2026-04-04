# Architectural Drift Report: cdocs-9nr

## Bead ID: cdocs-9nr
## Title: "action: wire startup state open and file diff into `run_index`"
## Timestamp: 2026-04-04 (revised)

## STATUS: REFACTORED

## Summary

`cmd/index.rs` was **525 lines** — 75% over the 300-line hard limit. The production
code (283 lines) was fine; the 240-line `#[cfg(test)] mod tests` block pushed it
over. Extracted the test module into a companion file following the existing
`scrape.rs` / `scrape_tests.rs` pattern in this codebase.

## Changes Made

### Split: `cmd/index.rs` → `cmd/index.rs` + `cmd/index_tests.rs`

| File | Before | After |
|------|--------|-------|
| `src/cmd/index.rs` | 525 lines | **288 lines** |
| `src/cmd/index_tests.rs` | (new) | **246 lines** |

**Method**: `include!("index_tests.rs")` at the bottom of `cmd/index.rs`, matching the
exact pattern used by `scrape.rs` → `scrape_tests.rs`. The test file wraps its
contents in `#[cfg(test)] mod tests { ... }` with explicit `use crate::cmd::index::*`
imports (not `use super::*`, since `include!` textually inserts at module scope).

**No module declarations, re-exports, or `mod.rs` changes required.** The `include!`
macro preserves the exact same module structure as the original inline `mod tests`.

## File Size Compliance (post-refactor):

| File | Lines | Status |
|------|-------|--------|
| `src/cmd/index.rs` | 288 | ✅ < 300 |
| `src/cmd/index_tests.rs` | 246 | ✅ < 300 |
| `src/diff.rs` | 407 | ⚠️ Over 300 (pre-existing, not modified by this bead) |

## Scott Wlaschin DDD Compliance:

- ✅ **No primitive obsession**: `run_index` accepts domain types (`&Path`, `&IndexConfig`).
  The pure calc `file_states_to_stored_hashes` converts between domain types
  (`FileStateRaw` → `StoredHashes`) with documented invariants (INV-4).
- ✅ **Types model domain**: `DiffStatus` enum is a mutually-exclusive classification
  making illegal states unrepresentable. `FileDiff` struct has exactly four
  `HashSet<String>` buckets — the union/disjoint invariants are enforced by
  construction in `compute_file_diff`.
- ✅ **Parse at boundary**: `validate_output_path(output)?` and source existence check
  at entry. Error mapping from domain errors (`DiffError`) to `anyhow::Error` at
  the action boundary (lines 120-136).
- ✅ **Explicit workflow**: Pipeline is an explicit sequential state machine:
  STEP 1 (DISCOVER) → STEP 1.5 (STATE+DIFF) → STEP 2 (ANALYZE) → ... → STEP 8.
  Each step is a named function call. No implicit flag/nullable choreography.
- ✅ **Functional core**: `file_states_to_stored_hashes` is a pure `#[must_use]`
  function with zero side effects. `compute_file_diff` in `diff.rs` is similarly
  pure with zero writes.
- ✅ **No Option-as-state**: No `Option` fields encoding workflow lifecycle.

## Pre-existing Issues (NOT modified by this bead):

- `src/diff.rs` (407 lines): Over the 300-line limit. Flagged for a future bead.
- `src/watch.rs`: Pre-existing clippy `arithmetic_side_effects` warnings (7 instances).
- `tests/analysis_reuse_tests.rs`: 2 pre-existing integration test failures unrelated
  to this bead's changes.

## Verification

- `cargo check --tests` — compiles cleanly (only pre-existing warning in bin test)
- `cargo test --bin ctd` — **1072 passed**, 0 failed (includes 7 `cmd::index` tests)
- `cargo clippy --bin ctd` — zero new warnings in modified files
- All 7 extracted tests pass: `file_states_to_stored_hashes_*` and Kani harness

## Conclusion

The sole violation was file length. DDD structure was already sound: pure calculations
with documented invariants, explicit pipeline steps, boundary error mapping, and
no primitive obsession. The extraction preserves identical semantics with zero
behavioral change.
