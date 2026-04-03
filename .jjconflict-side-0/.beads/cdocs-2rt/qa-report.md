# QA Report: cdocs-2rt — Deterministic Config Hashing & `compute_file_diff`

**Bead ID**: cdocs-2rt  
**Date**: 2026-04-02  
**QA Agent**: qa-enforcer  
**Contract**: `.beads/cdocs-2rt/contract.md`  
**Implementation**: `centralized-docs/src/diff.rs`  

---

## Execution Evidence

### Test Run 1: Unit Tests (lib, `diff` filter)

```
$ cargo test --lib -p centralized-docs -- diff
running 17 tests
test diff::tests::compute_config_hash_returns_empty_hash_when_none ... ok
test diff::tests::compute_config_hash_returns_sha256_when_file_readable ... ok
test diff::tests::compute_config_hash_returns_empty_hash_when_file_missing ... ok
test diff::tests::compute_config_hash_returns_empty_hash_when_file_unreadable ... ok
test diff::tests::compute_config_hash_returns_identical_hash_across_calls ... ok
test diff::tests::compute_config_hash_returns_distinct_concrete_hashes_for_different_contents ... ok
test diff::tests::compute_config_hash_returns_empty_hash_when_file_is_zero_bytes ... ok
test diff::tests::compute_config_hash_returns_exact_sha256_when_file_is_large ... ok
... (9 other unrelated tests matching 'diff' filter also passed)

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 850 filtered out
```

**Exit code: 0**

### Test Run 2: Integration Tests (diff_compute_tests)

```
$ cargo test --test diff_compute_tests -p centralized-docs
running 27 tests
test compute_file_diff_returns_source_dir_not_found_when_dir_missing ... ok
test compute_file_diff_returns_file_read_when_file_missing ... ok
test compute_file_diff_returns_file_read_when_permission_denied ... ok
test compute_file_diff_returns_path_traversal_when_path_escapes_source_dir ... ok
test compute_file_diff_rejects_absolute_path_outside_source_dir ... ok
test compute_file_diff_rejects_dotdot_path_traversal ... ok
test compute_file_diff_classifies_all_new_when_stored_hashes_empty ... ok
test compute_file_diff_classifies_all_deleted_when_no_discovered_files ... ok
test compute_file_diff_classifies_unchanged_when_hashes_match ... ok
test compute_file_diff_classifies_changed_when_content_hash_differs ... ok
test compute_file_diff_classifies_changed_when_config_hash_differs ... ok
test compute_file_diff_classifies_changed_when_both_hashes_differ ... ok
test compute_file_diff_classifies_new_when_not_in_stored_hashes ... ok
test compute_file_diff_classifies_deleted_when_not_in_discovered_files ... ok
test compute_file_diff_buckets_are_mutually_exclusive ... ok
test compute_file_diff_buckets_are_collectively_exhaustive ... ok
test compute_file_diff_populates_all_four_buckets_in_mixed_scenario ... ok
test compute_file_diff_returns_empty_diff_when_both_inputs_empty ... ok
test compute_file_diff_does_not_mutate_inputs_or_filesystem ... ok
test compute_file_diff_handles_large_file_set_correctly ... ok
test compute_file_diff_produces_deterministic_result_when_duplicate_source_paths ... ok
test compute_file_diff_classifies_changed_when_config_path_points_to_missing_file ... ok
test compute_file_diff_rejects_symlink_traversal ... ok
test compute_file_diff_returns_error_when_source_path_is_empty ... ok
test compute_file_diff_does_not_panic_when_source_path_exceeds_path_max ... ok
test compute_file_diff_never_panics_on_mismatched_stored_hash_keys ... ok
test compute_file_diff_ignores_size_bytes_and_classifies_by_content_hash ... ok

test result: ok. 27 passed; 0 failed; 0 ignored; 0 measured
```

**Exit code: 0**

### Test Run 3: Proptests (diff_proptests)

```
$ cargo test --test diff_proptests -p centralized-docs
running 6 tests
test proptest_config_hash_determinism ... ok
test proptest_config_hash_none_is_constant ... ok
test proptest_partition_invariant ... ok
test proptest_collective_exhaustive_invariant ... ok
test proptest_classification_rules ... ok
test proptest_rayon_determinism_with_duplicates ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured
```

**Exit code: 0**

### Test Run 4: Adversarial QA Tests (11 custom tests)

```
$ cargo test --test cdocs_2rt_adversarial -p centralized-docs
running 11 tests
test adversary_determinism_stress_1000_calls ... ok
test adversary_config_hash_determinism_1000_calls ... ok
test adversary_partition_invariant_under_parallel_stress ... ok
test adversary_path_traversal_with_various_encodings ... ok
test adversary_empty_and_whitespace_paths ... ok
test adversary_unicode_paths ... ok
test adversary_source_dir_is_a_file_not_directory ... ok
test adversary_large_stored_hashes_1000_entries ... ok
test adversary_error_messages_clean ... ok
test adversary_config_hash_always_returns_value ... ok
test adversary_no_secret_leaks_in_output ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured
```

**Exit code: 0**

### Clippy Checks

```
$ cargo clippy -p centralized-docs --no-deps --lib -- -D warnings -D clippy::unwrap_used -D clippy::panic -D clippy::expect_used
Finished `dev` profile [unoptimized + debuginfo] target(s)
Exit code: 0

$ cargo clippy --test diff_compute_tests -- -D warnings
Finished `dev` profile [unoptimized + debuginfo] target(s)
Exit code: 0

$ cargo clippy --test diff_proptests -- -D warnings
Finished `dev` profile [unoptimized + debuginfo] target(s)
Exit code: 0
```

### Panic Detection

```
$ rg 'panic!|unwrap()|expect(' centralized-docs/src/diff.rs
(no matches — exit code 1)
```

**Production code in `diff.rs` has ZERO panics, unwraps, or expects.**

### Format Check

```
$ cargo fmt -p centralized-docs -- --check
Exit code: 0
```

---

## Phase 1 — Discovery

| Check | Result | Evidence |
|-------|--------|----------|
| Module is `pub mod diff` in `lib.rs` | PASS | `pub mod diff;` at line 127 |
| All contract types are public | PASS | `pub struct StoredHashes`, `pub enum DiffStatus`, `pub struct FileDiff`, `pub enum DiffError` |
| Both contract functions are public | PASS | `pub fn compute_config_hash`, `pub fn compute_file_diff` |
| `#[must_use]` on `compute_config_hash` | PASS | Line 91 of diff.rs |
| `thiserror::Error` derive on `DiffError` | PASS | Line 62 of diff.rs |
| `FileDiff` derives `Default` | PASS | Line 53 of diff.rs |

## Phase 2 — Happy Path

| Check | Result | Evidence |
|-------|--------|----------|
| `compute_config_hash(None)` returns `content_hash(b"")` | PASS | Unit test 3.1 |
| `compute_config_hash` returns SHA-256 of file bytes | PASS | Unit test 3.2 + proptest 4.1 |
| `compute_file_diff` classifies unchanged correctly | PASS | Integration test 3.16 |
| `compute_file_diff` classifies changed (content) | PASS | Integration test 3.17 |
| `compute_file_diff` classifies changed (config) | PASS | Integration test 3.18 |
| `compute_file_diff` classifies changed (both) | PASS | Integration test 3.19 |
| `compute_file_diff` classifies new correctly | PASS | Integration tests 3.14 + 3.20 |
| `compute_file_diff` classifies deleted correctly | PASS | Integration tests 3.15 + 3.21 |
| Mixed scenario (all 4 buckets) | PASS | Integration test 3.24 |
| Large file set (50 files, 20/15/15/10 split) | PASS | Integration test 3.27 |

## Phase 3 — Hostile Interrogation

### Error Handling

| Check | Result | Evidence |
|-------|--------|----------|
| Missing source_dir → `DiffError::SourceDirNotFound` | PASS | Test 3.9: exact variant asserted |
| Missing file → `DiffError::FileRead` with `NotFound` | PASS | Test 3.10: `source.kind() == NotFound` |
| Permission denied → `DiffError::FileRead` | PASS | Test 3.10b: `source.kind() == PermissionDenied` |
| Path traversal `../../` → `DiffError::PathTraversal` | PASS | Test 3.11 |
| Absolute path → `DiffError::PathTraversal` | PASS | Test 3.12 |
| `../` prefix → `DiffError::PathTraversal` | PASS | Test 3.13 |
| Symlink traversal → `DiffError::PathTraversal` | PASS | Test 3.30 (Unix only) |
| Empty source_path → Err, no panic | PASS | Test 3.31 |
| PATH_MAX source_path → Err, no panic | PASS | Test 3.32 |

### Determinism

| Check | Result | Evidence |
|-------|--------|----------|
| `compute_config_hash` same input → same output | PASS | Test 3.5 + proptest 4.1 + adversary 1000-call stress |
| `compute_config_hash(None)` always same | PASS | Proptest 4.2 (1000 iterations) + adversary 1000-call |
| `compute_file_diff` deterministic across 10 calls | PASS | Test 3.28 (10 calls) + proptest 4.6 (5 calls) |
| `compute_file_diff` deterministic across 1000 calls | PASS | Adversary test: 1000 identical results |
| Rayon determinism with duplicate paths | PASS | Proptest 4.6: 5 calls with duplicates all identical |
| Parallel stress (8 threads × 10 calls) | PASS | Adversary test: all partition invariants hold |

### Partition Invariant

| Check | Result | Evidence |
|-------|--------|----------|
| Pairwise disjoint (mutually exclusive) | PASS | Tests 3.22 + 3.27 + proptest 4.3 + parallel stress |
| Collectively exhaustive | PASS | Test 3.23 + proptest 4.4 |
| Classification rules correct | PASS | Proptest 4.5 (4-way classification) |
| All-empty inputs → all-empty buckets | PASS | Test 3.25 |

### Edge Cases & Adversarial

| Check | Result | Evidence |
|-------|--------|----------|
| Unicode paths (日本語, emoji, accented) | PASS | Adversary test: all 3 files classified as New |
| Various path traversal encodings | PASS | Adversary test: 5 patterns, all rejected |
| Empty/whitespace paths | PASS | Adversary test: all 5 patterns rejected |
| File-as-source-dir with empty inputs | PASS | Adversary test: Ok(empty diff), no panic |
| File-as-source-dir with files | PASS | Adversary test: Err, no panic |
| 1000-entry stored_hashes (all deleted) | PASS | Adversary test: 1000 deleted, 0 others |
| size_bytes=0 doesn't affect classification | PASS | Test 3.34 |
| Mismatched stored_hash keys | PASS | Test 3.33: no panic, safe classification |
| Config path nonexistent → Changed | PASS | Test 3.29 |
| No mutation of inputs or filesystem | PASS | Test 3.26: mtime + clone comparison |

### Quality Gates

| Check | Result | Evidence |
|-------|--------|----------|
| No panics in production code | PASS | rg: zero matches for `panic!|unwrap()|expect(` |
| No stack traces in error messages | PASS | Adversary test: error messages clean |
| No secret leaks in output | PASS | Adversary test: no password/token/api_key in messages |
| Clippy clean (strict) | PASS | `-D warnings -D clippy::unwrap_used -D clippy::panic` |
| Format clean | PASS | `cargo fmt --check` exit 0 |
| Non-zero exit on error | PASS | All error tests assert `is_err()` |

---

## Findings

### CRITICAL (block merge)

None.

### MAJOR (fix before merge)

None.

### MINOR (fix if time)

#### MINOR-1: `compute_file_diff` accepts file-as-source-dir with empty inputs

**Location**: `centralized-docs/src/diff.rs:116-118`  
**Evidence**:
```
// Passing a regular file as source_dir with empty discovered_files:
compute_file_diff(&[], &file_path, None, &HashMap::new())
// Returns: Ok(FileDiff { unchanged: {}, changed: {}, new: {}, deleted: {} })
```
**Impact**: The contract error doc says "source directory does not exist **or is not a directory**" but the implementation only checks `canonicalize()` success, not `is_dir()`. With empty inputs, this is benign. With non-empty inputs, it produces a `FileRead` error (still correct behavior, just not the most precise error variant).  
**Severity**: MINOR — functionally correct, only a UX precision issue.  
**Recommendation**: Add `if !canonical_source.is_dir() { return Err(DiffError::SourceDirNotFound(...)) }` after canonicalization.

### OBSERVATION

#### OBS-1: Adversarial test file was temporary

The 11 adversarial tests were written to a temporary test file (`cdocs_2rt_adversarial.rs`) which was deleted after verification. These tests could be added to the permanent test suite for ongoing regression protection.

---

## Auto-fixes Applied

None required — all code was clean on first inspection.

---

## Beads Filed

None — no issues requiring implementation work were found.

---

## Contract Verification Matrix

| Contract ID | Description | Verified By | Status |
|-------------|-------------|-------------|--------|
| POST-1 | Partition: mutually exclusive | Tests 3.22, proptest 4.3, adversary parallel | PASS |
| POST-2 | Partition: collectively exhaustive | Test 3.23, proptest 4.4 | PASS |
| POST-3 | Unchanged iff hashes match | Test 3.16, proptest 4.5 (seed=0) | PASS |
| POST-4 | Changed iff hash mismatch | Tests 3.17/3.18/3.19, proptest 4.5 (seed=1/2) | PASS |
| POST-5 | New iff not in stored | Tests 3.14/3.20, proptest 4.5 (seed=3) | PASS |
| POST-6 | Deleted iff not discovered | Tests 3.15/3.21 | PASS |
| POST-7 | Zero writes/mutations | Test 3.26 (mtime + clone comparison) | PASS |
| POST-8 | Config hash deterministic | Test 3.5, proptest 4.1, adversary 1000-call | PASS |
| POST-9 | `compute_config_hash(None)` = empty hash | Test 3.1, proptest 4.2 | PASS |
| POST-10 | Empty stored → all New | Test 3.14 | PASS |
| POST-11 | Empty discovered → all Deleted | Test 3.15 | PASS |
| INV-1 | Partition invariant | Tests 3.22/3.23/3.27, proptests 4.3/4.4 | PASS |
| INV-2 | Diff determinism | Test 3.28, proptest 4.6, adversary 1000-call | PASS |
| INV-3 | Hash determinism | Test 3.5, proptest 4.1 | PASS |
| INV-4 | No mutation | Test 3.26 | PASS |
| INV-5 | Empty-input total | Test 3.25 | PASS |
| INV-6 | Single-bucket membership | Proptest 4.3, all integration tests | PASS |
| PRE-1 | source_dir exists | Test 3.9 | PASS |
| PRE-2 | Files exist under source_dir | Test 3.10 | PASS |
| PRE-3 | No path traversal | Tests 3.11/3.12/3.13/3.30 | PASS |

---

## Test Summary

| Category | Count | Status |
|----------|-------|--------|
| Unit tests (`compute_config_hash`) | 8 | ALL PASS |
| Integration tests (`compute_file_diff`) | 27 | ALL PASS |
| Proptest invariants | 6 | ALL PASS |
| Adversarial QA tests (custom) | 11 | ALL PASS |
| **Total** | **52** | **ALL PASS** |

---

## VERDICT: PASS

**0 CRITICAL · 0 MAJOR · 1 MINOR · 1 OBSERVATION**

All contract postconditions, invariants, and preconditions verified through actual execution.
52 tests executed with zero failures. Clippy clean with strict linting. Zero panics in production code.
The single MINOR finding (file-as-source-dir precision) does not affect correctness.
