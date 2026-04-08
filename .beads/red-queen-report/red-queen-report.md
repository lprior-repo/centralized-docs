# THE RED QUEEN'S VERDICT

**Target:** centralized-docs v0.6.1 (`ctd` CLI)  
**Binary:** `./target/release/ctd`  
**Champion SHA:** `95868c7e93ea71aa669455cf0947a7c732851504`  
**Date:** 2026-04-07  
**Agent:** Red Queen (Adversarial Evolutionary QA)

---

## CROWN FORFEIT

CRITICAL survivors exist. The champion cannot defend the crown.

---

## EXECUTIVE SUMMARY

**5 generations** of adversarial testing across **13 landscape dimensions**, executing **60+ test commands**. The Queen found **5 confirmed bugs** (3 CRITICAL, 1 MAJOR, 1 MINOR), all filed as bd issues.

The most severe finding is that **`ctd compact` panics (exit 101) on corrupted database files** instead of returning a graceful error. This affects 3 distinct corruption patterns and originates from redb's internal assertions. The `compact_state_db` function lacks `catch_unwind` protection, allowing redb panics to propagate as CLI crashes.

---

## FITNESS LANDSCAPE

| Dimension | Tests | Survivors | Fitness | Status |
|---|---|---|---|---|
| ctd-mcp-validation | 6 | 1 | 0.167 | PROBING |
| compact-edge-cases | 9 | 1 | 0.111 | PROBING |
| compact-resilience | 10 | 2 | 0.200 | PROBING |
| state-corruption | 4 | 1 | 0.250 | PROBING |
| watch-flags | 7 | 0 | 0.000 | COOLING |
| flag-combinations | 8 | 0 | 0.000 | COOLING |
| error-handling | 2 | 0 | 0.000 | COOLING |
| concurrent-access | 2 | 0 | 0.000 | COOLING |
| watch-cmd-integration | 2 | 0 | 0.000 | COOLING |
| scrape-integration | 2 | 0 | 0.000 | COOLING |
| diff-cmd | 3 | 0 | 0.000 | COOLING |
| apply-cmd | 2 | 0 | 0.000 | COOLING |
| edge-cases-final | 4 | 0 | 0.000 | COOLING |

---

## FINDINGS BY SEVERITY

### CRITICAL (P0) — 3 issues

#### [GEN-2-3] ctd compact PANICS on truncated database file
- **Issue:** cdocs-48y
- **Dimension:** compact-resilience
- **Panic:** `redb-2.6.3/src/tree_store/page_store/page_manager.rs:243` — assertion `storage.raw_file_len()? >= header.layout().len()`
- **Exit Code:** 101 (panic)
- **Repro:** Create valid DB, truncate by 100 bytes, run compact
- **Impact:** Any truncated database file (from disk full, crash during write, filesystem corruption) causes a CLI crash instead of an error message

#### [GEN-2-4] ctd compact PANICS on appended garbage data
- **Issue:** cdocs-8vk
- **Dimension:** state-corruption
- **Panic:** `redb-2.6.3/src/tree_store/page_store/page_manager.rs:266` — assertion `left == right` (2674688 vs 2674712)
- **Exit Code:** 101 (panic)
- **Repro:** Create valid DB, append any bytes, run compact
- **Impact:** Filesystem corruption or file concatenation causes a CLI crash

#### [GEN-3-5] ctd compact PANICS on zeroed middle section
- **Issue:** cdocs-cwo
- **Dimension:** compact-resilience
- **Panic:** `redb-2.6.3/src/tree_store/btree.rs:119` — `internal error: entered unreachable code`
- **Exit Code:** 101 (panic)
- **Repro:** Create valid DB, zero out the middle third, run compact
- **Impact:** Corrupted internal pages cause a CLI crash via a DIFFERENT panic path than the other two

**Root Cause (all 3 CRITICALs):** `compact_state_db()` in `state/commit.rs` calls `builder.open(path).or_else(|_| builder.create(path))` without `std::panic::catch_unwind`. redb v2.6.3 has assertion failures and unreachable code that panic on corrupted files instead of returning `Err`.

**Fix:** Wrap the redb open/compact operations in `std::panic::catch_unwind` and convert panics to `CommitError::CompactFailed`. Consider upstream redb bug report for the panics.

---

### MAJOR (P1) — 1 issue

#### [GEN-1-1] ctd-mcp exits 0 for invalid INDEX_DIR
- **Issue:** cdocs-bhg
- **Dimension:** ctd-mcp-validation
- **Exit Code:** 0 (should be non-zero)
- **Repro:** `ctd-mcp /any/directory/without/INDEX.json`
- **Impact:** The MCP server starts and exits 0 even when the INDEX_DIR is completely wrong (empty dir, missing INDEX.json, invalid INDEX.json). Users get zero feedback about misconfiguration. Exit code 0 misleadingly suggests success.
- **Fix:** Validate INDEX_DIR/INDEX.json exists and is parseable before starting the MCP server. Exit non-zero for invalid configuration.

---

### MINOR (P2) — 1 issue

#### [GEN-1-2] ctd compact silently creates new DB on 0-byte file
- **Issue:** cdocs-crt
- **Dimension:** compact-edge-cases
- **Exit Code:** 0 (misleading "Compaction completed successfully")
- **Repro:** `touch /tmp/test.redb && ctd compact /tmp/test.redb`
- **Impact:** The `compact_state_db` function's `open().or_else(|_| create())` fallback silently creates a new database for empty/nonexistent files. The user sees "compaction completed" for what is actually a database creation.
- **Fix:** Check if the file exists and has content before proceeding. Return an error for 0-byte files instead of creating a new DB.

---

## FEATURES TESTED & RESULTS

### 1. ctd compact (New subcommand)
| Test | Result |
|---|---|
| Missing argument | PASS (exit 1) |
| Nonexistent path | PASS (exit 2, error message) |
| Corrupt file | PASS (exit 2, "invalid data") |
| Read-only file | PASS (exit 2, "Permission denied") |
| Directory instead of file | PASS (exit 2, "Is a directory") |
| Special characters in path | PASS (creates and compacts) |
| Unicode path | PASS (creates and compacts) |
| Double compact (idempotent) | PASS ("already compact") |
| Named pipe | PASS (exit 2, "Invalid argument") |
| /dev/null | PASS (exit 2, "Invalid argument") |
| 0-byte file | **FAIL** (silently creates new DB, exit 0) |
| Truncated valid DB | **PANIC** (exit 101) |
| Appended garbage | **PANIC** (exit 101) |
| Zeroed middle section | **PANIC** (exit 101) |
| Header corruption | PASS (exit 2, "invalid data") |
| All-FF file | PASS (exit 2, "invalid data") |
| Bit-flipped file | PASS (no crash, exit 0) |
| NUL-filled file | PASS (exit 2, "invalid data") |
| Concurrent access | PASS (lock error, exit 2) |

### 2. Watch --connect-timeout-secs (New flag)
| Test | Result |
|---|---|
| Value 0 | PASS (validation error, exit 2) |
| Value 1 (minimum) | PASS (connect timeout fires) |
| Value 60 (maximum) | PASS (accepted) |
| Value 61 | PASS (validation error, exit 2) |
| Negative value | PASS (validation error, exit 2) |
| Float value | PASS (validation error, "must be integer") |
| Text value | PASS (validation error, "must be integer") |
| Very large value (999999) | PASS (validation error, "at most 60") |
| All flags at maximum | PASS (no validation errors) |
| All flags at minimum | PASS (no validation errors) |

### 3. ctd-mcp --help (Fixed flag handling)
| Test | Result |
|---|---|
| --help | PASS (shows usage) |
| --version | PASS (shows version) |
| No arguments | PASS (shows usage, exit 1) |
| Nonexistent directory | PASS (I/O error, exit 1) |
| File instead of directory | PASS ("not a directory", exit 1) |
| Empty string | PASS ("No such file", exit 1) |
| Invalid INDEX.json | **FAIL** (exits 0, no validation) |
| Extra positional args | **FAIL** (silently ignored, exits 0) |
| Dangling symlink | PASS ("No such file", exit 1) |

### 4. Durability Tuning / In-memory StateDb
- These features are API-only (no CLI surface). Validated via code review.
- `StateDbBuilder` with configurable `cache_size` and `durability` is well-tested in unit tests.
- `StateDb::open_in_memory()` is tested in the existing test suite.
- No CLI-exercisable bugs found.

---

## VALIDATION RATCHET

```
15 checks registered
10 PASS (contract checks + correct error handling)
5 FAIL (the 5 bugs above — these WILL pass once bugs are fixed)
```

The ratchet ensures that when these bugs are fixed, the regression checks will pass. Any future change that re-introduces these issues will be caught.

---

## OBSERVATIONS (No deterministic verification possible)

1. **Compact on bit-flipped DB passes silently:** Changing a single byte at offset 100 in a valid DB doesn't cause any error. This is because redb doesn't validate all pages during `open()` — only the header and layout. The corrupted data may never be read during compact.

2. **Compact on mid-file corrupted DB passes silently:** Writing 100 bytes of `\xff` in the middle of a valid DB file doesn't cause an error. Same reason as above.

3. **Stale lock files:** After `ctd watch` fails with a connect timeout, the `.cache/ctd_cache.redb` lock file may persist briefly, causing subsequent commands to fail with "Database already open."

---

## RECOMMENDATIONS

1. **Wrap `compact_state_db` in `catch_unwind`** — This is the most critical fix. All 3 CRITICAL panics are prevented by catching redb's panics and converting them to `CommitError::CompactFailed`.

2. **Validate INDEX_DIR in ctd-mcp** — Add early validation of the index directory before starting the MCP server. Exit non-zero for invalid paths.

3. **Fix compact's create-fallback** — The `open().or_else(|_| create())` pattern silently creates databases. For `compact`, this should only open existing databases, not create new ones.

4. **Consider upstream redb bug report** — The panics in `page_manager.rs` and `btree.rs` are arguably redb bugs. A corrupted database should return `Err`, not panic.
