# Moon Gate Report — cdocs-2ey

**Date**: 2026-04-04
**State**: 4 (Moon Gate)
**Workspace**: `/home/lewis/src/cdocs-2ey/centralized-docs`

---

## Gate 1: `cargo clippy --lib -- -D warnings`

**Result: PASS** — zero warnings, zero errors.

### Previous Issues (all resolved)

| # | Issue | Resolution |
|---|-------|------------|
| 1 | 20 rustdoc missing-backtick errors | Fixed in prior session |
| 2 | 2 `kani` cfg warnings | Fixed in prior session |
| 3 | 1 unused import `PersistedScrapeResult` | Fixed in prior session |
| 4 | 2 dead_code public fields | Fixed in prior session |

---

## Gate 2: `cargo test`

**Result: PASS** — 247 passed, 0 failed, 2 ignored (doc-tests).

### Fixes Applied This Session

1. **`PersistedScrapeResult` not found in scope** (compile error)
   - File: `centralized-docs/src/calc/scrape_diff.rs`
   - Lines: 738, 874
   - Fix: Added `use crate::persisted::PersistedScrapeResult;` to the test module imports (line 225).
   - Root cause: Test code referenced `PersistedScrapeResult` without importing it. The non-test code used the fully-qualified `crate::persisted::PersistedScrapeResult`, but the test module's `use super::*;` did not bring it in since it wasn't used in the parent module's scope.

2. **`run_scrape_creates_one_shared_read_session_and_one_commit`** — redb lock contention
   - File: `centralized-docs/tests/scrape_state_wiring.rs:173`
   - Fix: Added `drop(db);` before opening `db2` on the same file path.
   - Root cause: redb does not allow opening a second `Database` handle on the same file while the first is still alive. The test opened `db`, committed, then tried to open `db2` to verify — but `db` was still in scope.

3. **`run_scrape_leaves_state_intact_when_scrape_fails_before_commit`** — redb lock contention
   - File: `centralized-docs/tests/scrape_state_wiring.rs:459`
   - Fix: Added `drop(db);` before opening `db2` on the same file path.
   - Root cause: Same pattern — second `StateDb::open` on same file while first handle was live.

### Changed Files

| File | Change |
|------|--------|
| `src/calc/scrape_diff.rs` | Added `use crate::persisted::PersistedScrapeResult;` in test module |
| `tests/scrape_state_wiring.rs` | Added `drop(db);` before second `open_state_db` call in 2 tests |

---

## Summary

| Gate | Status | Details |
|------|--------|---------|
| Clippy | **PASS** | 0 warnings, 0 errors |
| Tests | **PASS** | 247/247 passed, 2 doc-tests ignored |

---

## VERDICT: **MOON GATE: PASS**
