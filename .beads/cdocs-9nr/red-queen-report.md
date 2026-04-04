# Red Queen Report: cdocs-9nr

## Bead ID: cdocs-9nr
## Title: "Wire startup state open and file diff into run_index"
## Timestamp: 2026-04-04
## Red Queen Session: drq-session

---

## Executive Summary

Adversarial testing of the `run_index` function (ctd index command) identified **1 potential defect** related to error handling with corrupt state databases. The state-diff feature computes correctly but state persistence across runs was not verified as the feature is informational-only in this bead's scope.

---

## Test Commands Executed

| # | Command | Expected | Actual | Status |
|---|---------|----------|--------|--------|
| 1 | `ctd index [empty_dir] --output [out]` | exit 1 + helpful msg | exit 1 + "No markdown files..." | PASS |
| 2 | `ctd index /nonexistent --output [out]` | exit 2 | exit 2 | PASS |
| 3 | `ctd index [1_file] --output [out]` (first run) | New: 1 | New: 1 | PASS |
| 4 | `ctd index [1_file] --output [out]` (second run) | Unchanged: 1 | New: 1 | NOTE* |
| 5 | `ctd index [1_file_modified] --output [out]` | Changed: 1 | New: 1 | NOTE* |
| 6 | `ctd index [empty_after_delete] --output [out]` | exit 1 | exit 1 | PASS |
| 7 | `ctd index [subdir_files] --output [out]` | Normal | Normal | PASS |
| 8 | `ctd index [src] --output [out] --hnsw-m 0` | exit 2 | exit 2 | PASS |
| 9 | `ctd index [src] --output [out] --hnsw-ef-construction 10` | exit 2 | exit 2 | PASS |
| 10a | `ctd index [src] --output [out]` (fresh state) | Creates state.db | 3.6MB state.db | PASS |
| 10b | `ctd index [src] --output [out]` with corrupt state.redb | exit != 0 | exit 0 + error | **FAIL** |
| 11 | `ctd index [1000_files] --output [out]` | Completes | Completes 0s | PASS |
| 12a | `ctd index [src] --output [out]` | Normal | Normal | PASS |
| 12b | `ctd index [src] --output [out]` (state.db removed) | Normal | Normal | PASS |

**NOTE***: Tests 4 and 5 show "New: 1" on repeated runs because this bead does not implement state WRITING (only reading). The diff is informational. Per bead contract: "The diff is informational only in this bead."

---

## Defects Found

### DEFECT-1: Corrupt State Database Exits with Code 0

| Field | Value |
|-------|-------|
| **Severity** | MAJOR |
| **Dimension** | error-handling |
| **Command** | `ctd index [source] --output [output]` with corrupted `output/state.redb` |
| **Expected Exit** | Non-zero (error) |
| **Actual Exit** | 0 |
| **Stdout/Stderr** | `Error: failed to open state database: failed to open state database at .../state.redb: I/O error: invalid data` |

**Root Cause Analysis**:
The error IS propagated correctly (printed to stderr), but the exit code is 0. This suggests the error handling in `run_index` may be wrapping errors in a way that `map_error_to_exit_code` returns 0, or there's an early return with `Ok(())` somewhere.

**Verification**:
```bash
# Corrupt the state database
echo "corrupt_data" > output/state.redb
# Run index
ctd index source --output output
# Error is printed but exit code is 0
```

---

## Landscape Summary

| Dimension | Tests | Survivors | Fitness |
|-----------|-------|-----------|---------|
| error-handling | 4 | 1 | 0.25 |
| state-diff | 4 | 0 | 0.00 |
| cli-validation | 3 | 0 | 0.00 |
| performance | 1 | 0 | 0.00 |

**Crown Status**: CONTESTED (1 MAJOR survivor in error-handling)

---

## Done When (Permanent Regressions)

The following commands are now locked as permanent regression gates:

```bash
# DEFECT-1 verification (when fixed):
# ctd index with-corrupt-state-db should exit non-zero
```

---

## Recommendations

1. **Fix DEFECT-1**: Investigate why corrupt state database produces exit code 0 despite printing error message. The `map_error_to_exit_code` function or the error wrapping in `run_index` should be corrected.

2. **State Persistence**: The diff showing "New: 1" on repeated runs is by design (informational-only in this bead). A future bead should wire state WRITING to enable true incremental indexing.

---

## Verdict

**CROWN CONTESTED** - 1 MAJOR defect found in error-handling dimension.

The `run_index` function correctly opens state database, computes diff, and handles most error cases. However, the corrupt state database case produces a false success exit code, which could cause automation scripts to misinterpret results.
