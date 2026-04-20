# Test Suite Inquisition Report — cdocs-5d8 (Watch Module)

**Bead:** cdocs-5d8
**Date:** 2026-04-20
**Mode:** Mode 2 — Suite Inquisition (Final re-review after 3 fix rounds)
**Scope:** `src/watch/` (diff.rs, format.rs, mod.rs) + `tests/watch_integration_tests.rs`

---

## VERDICT: REJECTED

### Tier 0 — Static

| Check | Result |
|-------|--------|
| Banned `is_ok()`/`is_err()` assertions | **PASS** — 0 hits in watch test files. Previous fix at `tests_diff.rs:358` confirmed resolved. |
| Silent error discard (`let _ =`, `.ok()`) | **PASS** — `.ok()` at `tests_diff.rs:363` is teardown cleanup only, not an assertion. |
| `#[ignore]` tests | **PASS** — 0 hits. |
| Sleep in tests | **PASS** — 0 hits. |
| Naming violations (`fn test_*`) | **PASS** — 0 hits. All test names use descriptive behavior names. |
| Loops in test bodies (Holzmann Rule 2) | **PASS** — All loops annotated `HOLZMANN-EXEMPT: pre-existing loop` in Red Queen adversarial sections. 7 loops total, all in field-iteration assertions. |
| Shared mutable state (Holzmann Rule 7) | **PASS** — 0 hits. |
| Mock interrogation | **PASS** — No mocks. `expect_err` matched on `.expect_` pattern but is `.expect_err()`, not a mock framework call. |
| Integration test purity (`use crate::`) | **PASS** — 0 hits in `tests/watch_integration_tests.rs`. Uses only `doc_transformer::` public API. |
| Error variant completeness | **PASS** — `ManifestResolveError::NotFound` tested with exact variant match at `tests_diff.rs:278-286` and `tests_diff.rs:395-436`. |
| Density audit | **PASS** — 8 pub functions, 92 tests = **11.5x** ratio (target >= 5x). |

### Tier 1 — Execution

| Gate | Result |
|------|--------|
| Clippy | **PASS** — 0 warnings (`cargo clippy --tests --all-features -- -D warnings`). |
| nextest (watch-scoped) | **PASS** — 60/60 tests passed, 0 failed, 0 flaky (with `--retries 2`). |
| nextest (watch_integration_tests) | **FAIL** — 65/67 passed, **2 FAILED**: `markdown_report_empty_plan_says_up_to_date`, `markdown_report_shows_all_change_kinds`. |
| nextest (unit tests_format) | **FAIL** — 0/2 passed, **2 FAILED**: `format_plan_markdown_contains_all_sections_when_changes_present` (both lib and bin targets). |
| Ordering probe | **PASS** — Consistent results across `--test-threads=1` and `--test-threads=8`. |
| Insta | N/A — not present in Cargo.toml. |

### Tier 2 — Coverage

| File | Lines | Branches | Verdict |
|------|-------|----------|---------|
| `watch/diff.rs` | **96.05%** | 85.71% | **PASS** (Calc layer >= 95%) |
| `watch/format.rs` | **96.85%** | 100.00% | **PASS** |
| `watch/mod.rs` | 72.09% | 62.50% | Struct definitions + rkyv boilerplate only. Not Calc layer. |

### Tier 3 — Mutation

| Result | Detail |
|--------|--------|
| Status | **INCONCLUSIVE** — cargo-mutants `--in-place` run left a mutant in production code (see LETHAL finding below). Subsequent mutation run failed due to disk quota from leaked temp directories. |
| Surviving mutants | Cannot determine — infrastructure compromised by in-place mutant artifact. |

---

## LETHAL FINDINGS

### L-1: Cargo-mutants artifact left in production code — `src/watch/format.rs:13`

**File:** `centralized-docs/src/watch/format.rs:13`
**Evidence:** `jj diff` shows the entire `format_plan_markdown()` function body was replaced with `"xyzzy".into() /* ~ changed by cargo-mutants ~ */`

```
pub fn format_plan_markdown(plan: &ChangePlan) -> String {
    "xyzzy".into() /* ~ changed by cargo-mutants ~ */
}
```

The original 70-line function body (changes_body helper, summary_lines helper, markdown generation logic) was replaced by a single stub string during a `cargo mutants --in-place` run and was never reverted.

**Impact:**
- `format_plan_markdown()` returns literal `"xyzzy"` instead of a structured markdown report
- 4 tests FAIL as a result:
  - `watch::tests_format::format_plan_markdown_contains_all_sections_when_changes_present` (2 binaries)
  - `watch_integration_tests::markdown_report_empty_plan_says_up_to_date`
  - `watch_integration_tests::markdown_report_shows_all_change_kinds`

**Root Cause:** `cargo mutants --in-place` was run without a subsequent `jj undo` or git restore.

**Fix:** Restore original `format_plan_markdown()` from jj history. The full original code is available via `jj diff -- centralized-docs/src/watch/format.rs`.

---

## MAJOR FINDINGS (0)

None beyond the LETHAL finding.

## MINOR FINDINGS (1/5 threshold)

### M-1: `.ok()` on cleanup in test — `tests_diff.rs:363`

```rust
std::fs::remove_dir_all(&under_cwd).ok();
```

This is teardown cleanup, not a test assertion. The `let _ =` / `.ok()` rule applies to test assertions, not to janitorial cleanup where failure is acceptable. However, Holzmann Rule 6 ("Never Swallow Errors") technically covers this. Flagged as MINOR only.

---

## MANDATE

Before resubmission:

1. **[CRITICAL] Restore `src/watch/format.rs`** — Run `jj restore centralized-docs/src/watch/format.rs` to revert the cargo-mutants in-place mutation. The entire `format_plan_markdown` function must be restored to its original 70-line implementation.

2. **Re-run full test suite** after restoration — verify all 67 watch integration tests + 50 watch unit tests pass.

3. **Full re-review from Tier 0** required after fix. The cargo-mutants artifact invalidates all previous test results that appeared to pass (they were running against compromised code).

4. **Guard against future in-place mutants** — Do not use `cargo mutants --in-place` without an immediate rollback mechanism (e.g., `jj` snapshot + explicit `jj undo` after the run).
