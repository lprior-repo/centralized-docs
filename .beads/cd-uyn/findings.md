# cd-uyn: Module Size Audit Findings

## STATUS: REFACTORED

## Audit Scope

All `.rs` source files under `centralized-docs/src/` and `centralized-docs-pod/src/`.

## Line Count Summary

- **Total source files**: ~170 files in centralized-docs/src, 5 in centralized-docs-pod/src
- **Violations found**: 2 files exceeding 300 lines
- **Violations after refactor**: 0

## Violations Detected & Fixed

### 1. `watch/tests_diff.rs` — 554 lines (VIOLATION: +254 over limit)

**Responsibility**: Tests for diff computation, snapshot handling, and manifest directory resolution.

**Problem**: Two distinct test domains crammed into one file:
- `compute_plan` + `snapshot` tests (lines 1-238, 9 tests)
- `resolve_manifest_dir` tests (lines 240-554, 12 tests B1-B9, B31-B34)

**Refactor**: Split into two files:
- `watch/tests_diff.rs` (237 lines) — snapshot and change-plan computation tests
- `watch/tests_resolve_manifest.rs` (281 lines) — manifest directory resolution tests
- Helper functions (`make_page`, `make_result`, `make_snapshot`) promoted to `pub(crate)` in `tests_diff.rs` for shared use
- New module registered in `watch/mod.rs`

### 2. `cmd/index.rs` — 306 lines (VIOLATION: +6 over limit)

**Responsibility**: Main index pipeline orchestrator (discover → diff → analyze → assign → transform → chunk → validate → index).

**Problem**: Barely over 300 lines. Trailing blank lines and excessive separator comments.

**Refactor**: Trimmed trailing whitespace and collapsed redundant comment separators (302 lines).

## Near-Miss Files (290-300 lines)

These are within compliance but approaching the limit:

| File | Lines | Module |
|------|-------|--------|
| `types/symbols/symbol_kind_tests.rs` | 299 | Symbol kind test cases |
| `persisted/analysis.rs` | 299 | Persisted analysis layer |
| `calc/build_state_changes/tests/error_path_tests.rs` | 299 | Error path tests |
| `graph/dag.rs` | 298 | DAG graph implementation |
| `cmd/watch.rs` | 298 | Watch command |
| `bin/llms_txt_validator/checks.rs` | 298 | LLMs.txt validation checks |
| `scrape/mod.rs` | 297 | Scrape orchestrator |
| `scrape/transformers/mod.rs` | 295 | Content transformers |

## DDD / Scott Wlaschin Assessment

- **No primitive obsession detected**: The codebase uses proper NewTypes (e.g., `PageHash`, `ChangeKind`, `ChangeSummary`)
- **Parse, don't validate**: `resolve_manifest_dir` returns typed `ManifestResolveError` instead of raw strings
- **Explicit state transitions**: `ChangeKind::Added/Modified/Removed` models the change lifecycle as an enum
- **Module cohesion is strong**: Each module has a clear single responsibility

## Test Results

- All 1185 lib tests pass (0 failures, 4 ignored)
- 22 watch tests verified after split (9 in tests_diff, 13 in tests_resolve_manifest)

## Final Line Counts (Post-Refactor)

| File | Before | After |
|------|--------|-------|
| `watch/tests_diff.rs` | 554 | 237 |
| `watch/tests_resolve_manifest.rs` | (new) | 281 |
| `cmd/index.rs` | 306 | 302 |
