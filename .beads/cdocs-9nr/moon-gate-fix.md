# Moon Gate Fix — cdocs-9nr

**Date**: 2026-04-03
**Status**: PASS

## Gate 1 — Clippy (24/24 fixed)

### unexpected_cfgs (2 fixed)
- **Cargo.toml**: Added `[lints.rust] unexpected_cfgs = { level = "warn", check-cfg = ['cfg(kani)'] }` to suppress false positives for `#[cfg(kani)]` in:
  - `src/calc/build_state_changes.rs:2268`
  - `src/transform.rs:2050`

### doc_markdown (18 fixed)
**`src/calc/build_state_changes.rs`** (6 fixes):
- Lines 70, 72, 74, 76: Wrapped `source_path` in backticks in `PipelineOutputs` field docs
- Line 119: Wrapped `source_path` in backticks in `DuplicateSourcePath` variant doc
- Line 123: Wrapped `FileDiff` in backticks in `EmptyDiff` variant doc

**`src/transform.rs`** (12 fixes):
- Line 750: Wrapped `link_map` in backticks
- Line 771: Wrapped `link_map` in backticks (after `link_map_fingerprint`)
- Line 815: Wrapped `IdMapping` in backticks
- Lines 820, 828: Wrapped `CacheError` in backticks
- Lines 868, 870: Wrapped `link_map` in backticks in `compute_link_map_fingerprint` doc
- Lines 896, 939: Wrapped `DocCache` in backticks
- Line 1006: Wrapped `source_path`, `content_hash`, `link_map_fingerprint` in backticks

### pub_underscore_fields (2 fixed)
- `src/calc/build_state_changes.rs:38`: Renamed `pub _reserved` to `pub reserved` (Pod padding field)
- `src/calc/build_state_changes.rs:47`: Renamed `pub _placeholder` to `pub placeholder`
- Updated all 15 references to `_reserved` in source and test files

### single_match_else (1 fixed)
- `src/transform.rs:1110`: Refactored `match cached { Some => ..., None => ... }` to `if let Some(artifact) = cached { ... } else { ... }`

### needless_lifetimes (1 fixed)
- `src/state/commit.rs:802`: Changed `impl<'db> StateReadSession<'db>` to `impl StateReadSession<'_>`

## Gate 2 — Tests (1/1 fixed)

### proptest_non_empty_string_key_always_accepted (fixed)
- **File**: `src/state/commit.rs:2824`
- **Root cause**: Original regex `[^\t\n\r\x00-...\x7F ]` excluded ASCII whitespace/control chars but not Unicode whitespace (e.g., `\u{a0}` non-breaking space)
- **Fix**: Changed to raw string `r"[^\p{White_Space}\x00-\x1F\x7F]{1,10}"` which uses the Unicode `White_Space` property to exclude all Unicode whitespace plus remaining ASCII control characters
- **Verification**: Test passes with 100 proptest cases (default)

## Verification

```
$ cargo clippy --lib -- -D warnings 2>&1 | tail -3
    Finished `dev` profile [unoptimized + debuginfo] target/s) in 2.85s

$ cargo nextest run -E "test(~build_state_changes) or test(~commit) or test(~transform_artifact) or test(proptest_non_empty) or test(proptest_empty_string)"
    Summary [  24.430s] 282 tests run: 282 passed, 2850 skipped
```

## Pre-existing Failures (not caused by this fix)

2 tests in `analysis_reuse_tests.rs` fail independently:
- `analyze_with_reuse_forwards_category_config_path_when_provided`
- `analyze_with_reuse_records_failed_files_for_unanalyzable_input`

These are unrelated to the changes made here.

## Files Changed

| File | Change |
|------|--------|
| `Cargo.toml` | Added `check-cfg` lint config for `kani` |
| `src/calc/build_state_changes.rs` | doc_markdown, pub_underscore_fields, field rename |
| `src/state/commit.rs` | needless_lifetimes, proptest regex fix |
| `src/transform.rs` | doc_markdown (12), single_match_else |
| `tests/build_state_changes_integration.rs` | `_reserved` → `reserved` field rename |
