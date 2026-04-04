# Moon Gate Report: cdocs-9nr

bead_id: cdocs-9nr
bead_title: action: wire startup state open and file diff into `run_index`
phase: state-4-moon-gate (re-run after architectural refactoring)
updated_at: 2026-04-04T06:01:00Z

## Status: PASS

---

## Gate 1 — Clippy (`cargo clippy --lib -- -D warnings`)

**Result: PASS**

```
Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.04s
```

Zero warnings, zero errors.

---

## Gate 2 — Tests (`cargo nextest run`)

**Result: PASS**

```
Summary [ 30.918s] 3082 tests run: 3082 passed, 50 skipped
```

3082 passed, 0 failed, 50 skipped. All 3082 runnable tests pass.

---

## Pre-existing Test Bugs Fixed

Two tests in `analysis_reuse_tests.rs` were failing on BOTH `main` and the
cdocs-9nr branch (pre-existing bugs, not regressions from refactoring):

### Fix 1: `analyze_with_reuse_forwards_category_config_path_when_provided`

**Root cause:** Test wrote a TOML-formatted categories config missing the
required `default_category` field. `CategoryConfig::load_from_file` (YAML
parser) correctly rejected it, causing `analyze_files` to fail during
re-analysis of guide.md with a config load error instead of the expected
config-hash-mismatch re-analysis.

**Fix:** Replaced invalid TOML config with valid YAML containing
`default_category: "docs"` and proper `rules` structure.

**File:** `centralized-docs/tests/analysis_reuse_tests.rs:990-994`

### Fix 2: `analyze_with_reuse_records_failed_files_for_unanalyzable_input`

**Root cause:** Test used NUL bytes (`\x00`) as "binary content", but NUL bytes
are valid UTF-8. `fs::read_to_string` succeeded and the file was analyzed
normally (2 analyses instead of expected 1 analysis + 1 failed file).

**Fix:** Replaced NUL bytes with actual invalid UTF-8 sequence (`\xFF\xFE`),
which causes `fs::read_to_string` to fail and the file to be recorded as a
`FailedFile`.

**File:** `centralized-docs/tests/analysis_reuse_tests.rs:895-898`

---

## Auto-fixes Applied

1. Fixed invalid category config YAML in test (line 990-994)
2. Fixed invalid-UTF-8 test data to use actual invalid UTF-8 bytes (line 895-898)

---

## Beads Filed

None — all issues resolved inline.

---

## VERDICT: PASS
