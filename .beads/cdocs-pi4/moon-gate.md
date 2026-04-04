# Moon Gate Report — Bead cdocs-pi4

**Date**: 2026-04-04
**Workspace**: `/home/lewis/src/cdocs-pi4/centralized-docs`
**Run**: Final verification

---

## 1. Clippy (`cargo clippy --lib -- -D warnings`)

**Result: PASS** — 0 warnings, 0 errors.

### Strict Mode (`-D warnings -D clippy::unwrap_used -D clippy::panic -D clippy::expect_used -W clippy::pedantic`)

**Result: PASS** — 0 warnings, 0 errors.

### Previous Failures (Resolved)

| Issue | Status |
|---|---|
| `unexpected cfg condition name: kani` (2 errors) | Fixed — `check-cfg` added to Cargo.toml |
| `pub_underscore_fields` (2 errors) | Fixed — fields renamed |
| `doc_markdown` (~12 errors) | Fixed — backticks added to doc identifiers |
| `single_match` lint | Fixed — converted to `if let` |
| Lifetime elision | Fixed — explicit lifetime elided |

---

## 2. Tests (`cargo test`)

**Result: PASS** — 52 passed, 0 failed, 0 ignored.

### Doc-tests

**Result: PASS** — 5 passed, 2 ignored (pre-existing ignores).

### Previous Failures (Resolved)

| Test | Previous Status |
|---|---|
| `analyze_with_reuse_records_failed_files_for_unanalyzable_input` | Was FAIL — now PASS |
| `analyze_with_reuse_forwards_category_config_path_when_provided` | Was FAIL — now PASS |

---

## 3. Formatting (`cargo fmt --check`)

**Result: PASS** — no formatting issues.

---

## 4. Known Issue: `required_deps` in `manifest_bytemuck_test.rs`

**Verified FIXED** — Line 169 shows `required_deps: [(&str, &str); 3]` containing only `redb`, `sha2`, and `rayon`. The removed entries (`lru`, `parking_lot`) are no longer present, matching the bead's dep removal.

---

## 5. Verdict

| Gate | Status |
|---|---|
| Clippy (`-D warnings`) | **PASS** |
| Clippy (strict pedantic) | **PASS** |
| Tests (52 unit + 5 doc) | **PASS** |
| Formatting | **PASS** |

### **MOON GATE: PASS**
