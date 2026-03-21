# QA Report: redb Cache Implementation

**Module:** `centralized-docs/src/cache/mod.rs`
**Date:** 2026-03-21
**Reviewer:** qa-enforcer (actual execution, no hallucination)
**Verdict:** QA_FAIL

---

## Test Results

### 1. Compilation Check
**Command:** `cargo check -p centralized-docs --lib`
**Exit Code:** 0
**Result:** PASS (with warnings)

**Output:**
```
warning: unused import: `Deserialize`
  --> centralized-docs/src/cache/mod.rs:19:35
   |
19 | use serde::{de::DeserializeOwned, Deserialize, Serialize};
   |                                   ^^^^^^^^^^^
```

**Finding:** Unused `Deserialize` import at mod.rs:19. Non-blocking but should be removed.

---

### 2. All Lib Tests
**Command:** `cargo test -p centralized-docs --lib`
**Exit Code:** 0
**Result:** PASS

**Output:** 273 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.35s

---

### 3. Cache-Specific Tests
**Command:** `cargo test -p centralized-docs --lib -- cache`
**Exit Code:** 0
**Result:** PASS

**Output:** 15 passed; 0 failed; 0 ignored; 0 measured; 258 filtered out; finished in 0.34s

**Tests executed:**
- `test_cache_basic_roundtrip` ... ok
- `test_cache_miss_returns_none` ... ok
- `test_cache_struct_value` ... ok
- `test_cache_stats` ... ok
- `test_get_or_compute_caches_result` ... ok
- `test_content_hash_consistency` ... ok
- `test_content_hash_different_inputs` ... ok
- `test_clear_all` ... ok
- `test_in_memory_cache` ... ok
- `test_disabled_cache_skips_operations` ... ok
- `test_key_too_large_returns_error` ... ok
- `test_value_too_large_returns_error` ... ok
- `test_key_at_max_size_succeeds` ... ok
- `test_scrape_key_size_validation` ... ok
- `test_transform_key_size_validation` ... ok

---

### 4. Clippy (lib only)
**Command:** `cargo clippy -p centralized-docs --lib -- -W clippy::all`
**Exit Code:** 0
**Result:** PASS (warnings in cache module)

**Cache-module-specific warnings (6 total):**

| Line | Severity | Lint | Detail |
|------|----------|------|--------|
| 19 | warn | `unused_imports` | `Deserialize` imported but never used |
| 14 | warn | `doc_markdown` | `DoS` needs backticks |
| 39 | warn | `doc_markdown` | `InMemoryBackend` needs backticks |
| 356 | warn | `must_use_candidate` | `content_hash` should be `#[must_use]` |
| 366 | warn | `must_use_candidate` | `url_hash` should be `#[must_use]` |
| 370 | warn | `must_use_candidate` | `path_hash` should be `#[must_use]` |

**Non-cache warnings exist in other modules** (discover.rs, filter.rs, index.rs, scrape/, search.rs, transform.rs) but are out of scope.

---

### 5. Verify SHA-256 (not DefaultHasher)
**Command:** `grep -r "DefaultHasher" centralized-docs/src/cache/`
**Exit Code:** 1 (no matches)
**Result:** PASS

No DefaultHasher found. SHA-256 is used via `sha2::Sha256` at mod.rs:357-360.

---

### 6. Verify No unwrap/expect in Production Code
**Command:** `grep -n "unwrap()\|expect(" centralized-docs/src/cache/mod.rs | grep -v "#\[cfg(test)\]" | grep -v "fn test_"`
**Exit Code:** 1 (no matches)
**Result:** PASS

Zero unwrap/expect in production code. All public methods return `Result`.

---

### 7. Line Count
**Command:** `wc -l centralized-docs/src/cache/mod.rs`
**Exit Code:** 0
**Output:** `626 centralized-docs/src/cache/mod.rs`
**Result:** **FAIL**

626 lines exceeds the 300-line file limit (AGENTS.md architectural-drift rule).
Note: ~250 lines (lines 374-626) are test code in `#[cfg(test)] mod tests`. Production code is ~374 lines, still over the 300-line limit.

---

### 8. Verify CacheBackend Enum
**Command:** `grep -n "enum CacheBackend" centralized-docs/src/cache/mod.rs`
**Exit Code:** 0
**Output:** `38:pub enum CacheBackend {`
**Result:** PASS

Enum exists with `Memory` and `File(PathBuf)` variants, marked `#[non_exhaustive]`.

---

### 9. Verify Error Propagation (no silent swallowing)
**Command:** `grep -n 'return Ok(None)' centralized-docs/src/cache/mod.rs`
**Exit Code:** 0
**Output:**
```
136:            return Ok(None);
158:            return Ok(None);
180:            return Ok(None);
289:        return Ok(None);
```
**Result:** PASS

All 4 instances are **intentional**, not silent error swallowing:
- Lines 136, 158, 180: Feature-flag early returns (cache disabled for specific type)
- Line 289: Cache miss (key not found) - standard cache behavior

---

### 10. Verify Size Limits
**Command:** `grep -n "MAX_KEY_SIZE\|MAX_VALUE_SIZE" centralized-docs/src/cache/mod.rs`
**Exit Code:** 0
**Output:**
```
24:const MAX_KEY_SIZE: usize = 256;
28:const MAX_VALUE_SIZE: usize = 10 * 1024 * 1024;
```
**Result:** PASS

Limits defined and enforced via `validate_key_size()` (line 312) and `validate_value_size()` (line 324).

---

## Summary Table

| # | Check | Exit Code | Verdict |
|---|-------|-----------|---------|
| 1 | Compilation | 0 | PASS (1 warning: unused import) |
| 2 | All lib tests (273) | 0 | PASS |
| 3 | Cache tests (15) | 0 | PASS |
| 4 | Clippy | 0 | PASS (6 cache warnings) |
| 5 | No DefaultHasher | 1 | PASS |
| 6 | No unwrap/expect in prod | 1 | PASS |
| 7 | Line count <= 300 | 0 | **FAIL (626 lines)** |
| 8 | CacheBackend enum exists | 0 | PASS |
| 9 | Error propagation | 0 | PASS |
| 10 | Size limits | 0 | PASS |

---

## Findings

### CRITICAL
(none)

### MAJOR
1. **[MAJOR-001] File exceeds 300-line limit** - `mod.rs` is 626 lines (production: ~374 lines, tests: ~252 lines). Must be split per AGENTS.md architectural-drift rule.

### MINOR
2. **[MINOR-001] Unused import** - `Deserialize` at mod.rs:19 is never used.
3. **[MINOR-002] Missing `#[must_use]`** - `content_hash`, `url_hash`, `path_hash` should all be `#[must_use]` (clippy:must_use_candidate).
4. **[MINOR-003] Doc markdown** - `DoS` and `InMemoryBackend` need backticks in doc comments.

---

## Final Verdict

**QA_FAIL: mod.rs is 626 lines, exceeding the 300-line limit (MAJOR-001).**

All functional checks pass: compiles, all 273 tests pass (15 cache-specific), SHA-256 used, zero unwrap/expect in prod, CacheBackend enum exists, error propagation correct, size limits enforced. The single blocking issue is file length.
