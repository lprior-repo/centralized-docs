# QA Report — cdocs-4s3

**Bead**: cdocs-4s3 — Archived output bulk loaders with transform reuse coverage
**Date**: 2026-04-02
**Contract**: `.beads/cdocs-4s3/contract.md`
**Source**: `centralized-docs/src/state/bulk_load.rs` (507 lines production, 1007 lines tests)
**Integration tests**: `centralized-docs/tests/bulk_load/` (8 files, 34 tests)

---

## Execution Evidence

### Unit Tests (30 tests)

```
$ cargo test --package centralized-docs --lib -- state::bulk_load 2>&1

running 30 tests
test state::bulk_load::tests::session_new_holds_database_reference_when_constructed ... ok
test state::bulk_load::tests::load_file_states_returns_all_rows_when_table_has_valid_entries ... ok
test state::bulk_load::tests::load_file_states_returns_empty_hashmap_when_table_is_empty ... ok
test state::bulk_load::tests::load_file_states_returns_malformed_row_error_when_value_is_one_byte_short ... ok
test state::bulk_load::tests::load_file_states_returns_malformed_row_error_when_value_is_one_byte_over ... ok
test state::bulk_load::tests::load_file_states_returns_malformed_row_error_when_value_is_0_bytes ... ok
test state::bulk_load::tests::load_file_states_aborts_on_first_malformed_row_without_partial_map ... ok
test state::bulk_load::tests::load_file_states_returns_backend_error_when_table_cannot_be_opened ... ok
test state::bulk_load::tests::load_file_states_decoded_values_are_bitwise_identical_to_written_bytes ... ok
test state::bulk_load::tests::load_file_states_uses_borrowed_transaction_without_opening_new_one ... ok
test state::bulk_load::tests::load_file_states_ignores_url_state_table_rows ... ok
test state::bulk_load::tests::load_file_states_preserves_key_strings_exactly ... ok
test state::bulk_load::tests::load_file_states_is_idempotent_across_multiple_calls ... ok
test state::bulk_load::tests::load_file_states_map_size_equals_row_count_for_various_n ... ok
test state::bulk_load::tests::load_url_states_returns_all_rows_when_table_has_valid_entries ... ok
test state::bulk_load::tests::load_url_states_returns_empty_hashmap_when_table_is_empty ... ok
test state::bulk_load::tests::load_url_states_returns_malformed_row_error_when_value_is_one_byte_short ... ok
test state::bulk_load::tests::load_url_states_returns_malformed_row_error_when_value_is_one_byte_over ... ok
test state::bulk_load::tests::load_url_states_returns_malformed_row_error_when_value_is_0_bytes ... ok
test state::bulk_load::tests::load_url_states_returns_malformed_row_error_when_value_is_double_size ... ok
test state::bulk_load::tests::load_url_states_aborts_on_first_malformed_row_without_partial_map ... ok
test state::bulk_load::tests::load_url_states_returns_backend_error_when_table_cannot_be_opened ... ok
test state::bulk_load::tests::load_url_states_decoded_values_are_bitwise_identical_to_written_bytes ... ok
test state::bulk_load::tests::load_url_states_uses_borrowed_transaction_without_opening_new_one ... ok
test state::bulk_load::tests::load_url_states_ignores_file_state_table_rows ... ok
test state::bulk_load::tests::load_url_states_preserves_key_strings_exactly ... ok
test state::bulk_load::tests::load_url_states_is_idempotent_across_multiple_calls ... ok
test state::bulk_load::tests::load_url_states_map_size_equals_row_count_for_various_n ... ok
test state::bulk_load::tests::session_new_returns_storage_error_on_read_failure ... ok
test state::bulk_load::tests::both_loaders_work_independently_on_same_database ... ok

test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 837 filtered out; finished in 0.85s
```

**Exit code: 0**

### Integration Tests (34 tests)

```
$ cargo test --package centralized-docs --test lib -- bulk_load 2>&1

running 34 tests
test bulk_load::owned_archive_tests::owned_archive_returns_concrete_value_when_bytes_pass_bytecheck ... ok
test bulk_load::owned_archive_tests::owned_archive_returns_corrupt_payload_when_bytes_fail_bytecheck ... ok
test bulk_load::owned_archive_tests::owned_archive_preserves_exact_input_bytes_when_constructed ... ok
test bulk_load::owned_archive_tests::owned_archive_archived_returns_matching_field_values_when_called ... ok
test bulk_load::owned_archive_tests::owned_archive_deserialize_returns_owned_value_when_valid ... ok
test bulk_load::load_analyses_tests::load_analyses_returns_all_entries_when_all_hashes_exist ... ok
test bulk_load::load_analyses_tests::load_analyses_omits_missing_hashes_when_some_not_found ... ok
test bulk_load::load_analyses_tests::load_analyses_returns_empty_map_when_no_hashes_match ... ok
test bulk_load::load_analyses_tests::load_analyses_returns_empty_map_when_input_slice_empty ... ok
test bulk_load::load_analyses_tests::load_analyses_deduplicates_when_input_has_duplicate_hashes ... ok
test bulk_load::load_analyses_tests::load_analyses_returns_table_open_error_when_table_missing ... ok
test bulk_load::load_analyses_tests::load_analyses_returns_corrupt_payload_when_bytes_invalid ... ok
test bulk_load::load_analyses_tests::load_analyses_preserves_key_identity_when_loading_entries ... ok
test bulk_load::load_analyses_tests::load_analyses_fails_fast_when_mix_of_valid_and_corrupt_entries ... ok
test bulk_load::load_analyses_tests::load_analyses_returns_empty_map_when_input_empty_and_table_missing ... ok
test bulk_load::load_transforms_tests::load_transforms_returns_all_entries_when_all_hashes_exist ... ok
test bulk_load::load_transforms_tests::load_transforms_omits_missing_hashes_when_some_not_found ... ok
test bulk_load::load_transforms_tests::load_transforms_returns_empty_map_when_input_slice_empty ... ok
test bulk_load::load_transforms_tests::load_transforms_deduplicates_when_input_has_duplicate_hashes ... ok
test bulk_load::load_transforms_tests::load_transforms_returns_corrupt_payload_when_bytes_invalid ... ok
test bulk_load::load_transforms_tests::load_transforms_fails_fast_when_mix_of_valid_and_corrupt_entries ... ok
test bulk_load::load_chunks_tests::load_chunks_returns_all_entries_when_all_hashes_exist ... ok
test bulk_load::load_chunks_tests::load_chunks_omits_missing_hashes_when_some_not_found ... ok
test bulk_load::load_chunks_tests::load_chunks_returns_empty_map_when_input_slice_empty ... ok
test bulk_load::load_chunks_tests::load_chunks_returns_corrupt_payload_when_bytes_invalid ... ok
test bulk_load::load_chunks_tests::load_chunks_fails_fast_when_mix_of_valid_and_corrupt_entries ... ok
test bulk_load::load_scrapes_tests::load_scrapes_returns_all_entries_when_all_hashes_exist ... ok
test bulk_load::load_scrapes_tests::load_scrapes_omits_missing_hashes_when_some_not_found ... ok
test bulk_load::load_scrapes_tests::load_scrapes_returns_empty_map_when_input_slice_empty ... ok
test bulk_load::load_scrapes_tests::load_scrapes_returns_corrupt_payload_when_bytes_invalid ... ok
test bulk_load::load_scrapes_tests::load_scrapes_fails_fast_when_mix_of_valid_and_corrupt_entries ... ok
test bulk_load::session_lifecycle_tests::read_session_remains_usable_after_bulk_load_call ... ok
test bulk_load::boundary_tests::load_analyses_handles_10k_hashes_without_panic ... ok
test bulk_load::boundary_tests::load_transforms_handles_many_unique_hashes ... ok

test result: ok. 34 passed; 0 failed; 0 ignored; 0 measured; 198 filtered out; finished in 0.75s
```

**Exit code: 0**

### Clippy (zero warnings)

```
$ cargo clippy --package centralized-docs --lib -- -D warnings 2>&1
    Checking centralized-docs v0.6.1
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.85s
```

**Exit code: 0**

### Panic/unwrap audit (production code only)

```
$ rg -n "panic!|todo!|unimplemented!|unwrap\(\)|expect\(" centralized-docs/src/state/bulk_load.rs
(no matches in production code — all matches are inside #[cfg(test)] mod tests {})
```

### Unsafe code audit

```
$ rg -n "unsafe " centralized-docs/src/state/bulk_load.rs
(no output — zero unsafe code)
```

### Serde/bincode invariant (I-02)

```
$ rg -n "serde_json|bincode" centralized-docs/src/state/bulk_load.rs
(exit code 1 — no matches)
$ rg -n "serde_json|bincode" centralized-docs/src/state/mod.rs
(exit code 1 — no matches)
```

---

## Phase 1 — Discovery

**[PASS]** Contract file exists and is well-structured (285 lines)
**[PASS]** All four bulk loader methods present: `load_analyses`, `load_transforms`, `load_chunks`, `load_scrapes`
**[PASS]** Error taxonomy matches contract: `TableOpen`, `StorageError`, `CorruptPayload`
**[PASS]** `#[non_exhaustive]` on `BulkLoadError` enum
**[PASS]** `OwnedArchive<T>` provides `try_from_bytes`, `as_bytes`, `archived`, `deserialize`
**[PASS]** `StateReadSession` wraps `ReadTransaction` with RAII semantics

## Phase 2 — Happy Path

### Q-01: Returned HashMap contains exactly hashes with matching stored values

**[PASS]** Verified for all four loaders:
- `load_analyses_returns_all_entries_when_all_hashes_exist` — 3 hashes, 3 results
- `load_transforms_returns_all_entries_when_all_hashes_exist` — 2 hashes, 2 results
- `load_chunks_returns_all_entries_when_all_hashes_exist` — 2 hashes, 2 results
- `load_scrapes_returns_all_entries_when_all_hashes_exist` — 1 hash, 1 result

### Q-02: Each value is a valid OwnedArchive whose bytes passed rkyv bytecheck

**[PASS]** `OwnedArchive::try_from_bytes` validates via `rkyv::access` before storing.
Test `owned_archive_returns_concrete_value_when_bytes_pass_bytecheck` verifies field access works.

### Q-03: OwnedArchive bytes are fully independent of redb AccessGuard lifetime

**[PASS]** Code at line 422: `let bytes: Box<[u8]> = access_guard.value().to_vec().into_boxed_slice();`
Bytes are copied into heap-owned `Box<[u8]>` before the guard is dropped.

### Q-04: All reads in single shared ReadTransaction

**[PASS]** `load_entries` receives `&ReadTransaction` from `self.read_txn`, never opens new transactions.
`session_lifecycle_tests::read_session_remains_usable_after_bulk_load_call` proves session survives multiple calls.

### Q-05: Read transaction remains alive and usable after call returns

**[PASS]** Test `read_session_remains_usable_after_bulk_load_call` calls `load_analyses` then `load_transforms` on the same session.

### Q-06: Missing hashes silently omitted

**[PASS]** Verified for all four loaders:
- `load_analyses_omits_missing_hashes_when_some_not_found`
- `load_transforms_omits_missing_hashes_when_some_not_found`
- `load_chunks_omits_missing_hashes_when_some_not_found`
- `load_scrapes_omits_missing_hashes_when_some_not_found`

### Q-07: Duplicate hashes produce single entry

**[PASS]** Verified for analyses and transforms:
- `load_analyses_deduplicates_when_input_has_duplicate_hashes` — 3 copies of same hash → 1 result
- `load_transforms_deduplicates_when_input_has_duplicate_hashes` — 2 copies → 1 result

## Phase 3 — Hostile Interrogation

### CorruptPayload on invalid rkyv bytes

**[PASS]** All four loaders tested with garbage bytes (`[0xDE, 0xAD, 0xBE, 0xEF, 0xFF, 0xFF, 0xFF, 0xFF]`):
- `load_analyses_returns_corrupt_payload_when_bytes_invalid`
- `load_transforms_returns_corrupt_payload_when_bytes_invalid`
- `load_chunks_returns_corrupt_payload_when_bytes_invalid`
- `load_scrapes_returns_corrupt_payload_when_bytes_invalid`

Each test verifies: error variant is `CorruptPayload`, table name is correct, `key_hex` matches, message is non-empty.

### Fail-fast on corruption (I-05): mix of valid + corrupt

**[PASS]** All four loaders tested with one valid + one corrupt entry:
- `load_analyses_fails_fast_when_mix_of_valid_and_corrupt_entries`
- `load_transforms_fails_fast_when_mix_of_valid_and_corrupt_entries`
- `load_chunks_fails_fast_when_mix_of_valid_and_corrupt_entries`
- `load_scrapes_fails_fast_when_mix_of_valid_and_corrupt_entries`

Each test confirms: error is `CorruptPayload` for the corrupt key, no partial result.

### Empty input returns empty HashMap

**[PASS]** All four loaders:
- `load_analyses_returns_empty_map_when_input_slice_empty`
- `load_transforms_returns_empty_map_when_input_slice_empty`
- `load_chunks_returns_empty_map_when_input_slice_empty`
- `load_scrapes_returns_empty_map_when_input_slice_empty`

### Empty input + missing table → still Ok(empty map)

**[PASS]** `load_analyses_returns_empty_map_when_input_empty_and_table_missing`
Code path: empty hash slice → early return at line 396 before `open_table` is called.

### TableOpen error when table missing

**[PASS]** `load_analyses_returns_table_open_error_when_table_missing`
Database created without `analysis_outputs` table → `BulkLoadError::TableOpen`.

### Key identity preserved (I-06)

**[PASS]** `load_analyses_preserves_key_identity_when_loading_entries`
Uses non-trivial key `[0x01, 0x02, ..., 0x20]`, verifies `map.keys().next() == hash`.

### Large input boundary (10,000+ hashes)

**[PASS]** `load_analyses_handles_10k_hashes_without_panic` — 10k hashes with 256 unique, all resolved.
`load_transforms_handles_many_unique_hashes` — 500 truly unique hashes, all resolved.

---

## Invariant Verification

| ID  | Invariant | Status | Evidence |
|-----|-----------|--------|----------|
| I-01 | Transaction scope | **PASS** | `load_entries` receives `&ReadTransaction`, never opens write txn |
| I-02 | No serde/bincode | **PASS** | `rg serde_json\|bincode` in bulk_load.rs and state/mod.rs: zero matches |
| I-03 | Ownership transfer | **PASS** | Line 422: `.to_vec().into_boxed_slice()` copies out of AccessGuard |
| I-04 | Deterministic | **PASS** | Idempotency tests verify same results across multiple calls |
| I-05 | Fail-fast on corruption | **PASS** | `try_fold` propagates first error; 4 tests verify mixed valid+corrupt |
| I-06 | Key identity | **PASS** | `*hash` used as HashMap key; explicit test with non-trivial key bytes |

---

## Contract Deviation Analysis

### Deviation 1: `archived()` return type (MINOR)

**Contract says:**
```rust
pub fn archived(&self) -> &T::Archived;
```

**Actual code:**
```rust
pub fn archived(&self) -> Result<&T::Archived, BulkLoadError>
```

**Assessment**: The code is **strictly safer** than the contract. Because the codebase uses `forbid(unsafe_code)`, you cannot simply cast bytes to `&T::Archived` without re-validation. The code re-runs `rkyv::access` and returns a `Result`, which is documented as "theoretically unreachable after successful `try_from_bytes`". This is a soundness improvement that the contract should be updated to reflect.

**Severity**: MINOR — contract should be updated to match the safer `Result` return type.

### Deviation 2: Domain type names (OBSERVATION)

**Contract uses simplified names**: `Analysis`, `Vec<Chunk>`, `ScrapedPage`, `String`

**Actual code uses**: `PersistedAnalyzeResult`, `PersistedChunksResult`, `PersistedScrapeResult`, `PersistedTransformResult`

**Assessment**: The code uses the actual domain types from `persisted.rs`. The contract used simplified names for readability. The types are structurally identical — the persisted wrappers are the correct domain types for the output tables.

**Severity**: OBSERVATION — documentation clarity issue only.

---

## Findings

### CRITICAL (block merge)
None.

### MAJOR (fix before merge)
None.

### MINOR (fix if time)

#### MINOR-1: `archived()` return type differs from contract

- **File**: `centralized-docs/src/state/bulk_load.rs:175`
- **Contract**: `pub fn archived(&self) -> &T::Archived`
- **Actual**: `pub fn archived(&self) -> Result<&T::Archived, BulkLoadError>`
- **Impact**: Callers must handle `Result`, which is more work but safer
- **Recommendation**: Update contract to reflect the actual safer signature

### OBSERVATION

#### OBS-1: Type name simplification in contract

- **Contract uses**: `Analysis`, `Vec<Chunk>`, `ScrapedPage`, `String`
- **Code uses**: `PersistedAnalyzeResult`, `PersistedChunksResult`, `PersistedScrapeResult`, `PersistedTransformResult`
- **Impact**: None — just a documentation naming difference

---

## Auto-fixes Applied
None required — all tests pass, no issues found.

## Beads Filed
None — no issues requiring implementation work.

---

## Summary Statistics

| Metric | Value |
|--------|-------|
| Production code lines | 507 |
| Unit test lines | ~1007 |
| Integration test lines | ~816 |
| Unit tests | 30 |
| Integration tests | 34 |
| Total tests | 64 |
| Tests passed | 64 |
| Tests failed | 0 |
| Panics detected | 0 |
| Unsafe code blocks | 0 |
| Clippy warnings | 0 |
| Contract deviations | 1 minor, 1 observation |
| Critical findings | 0 |
| Major findings | 0 |

## VERDICT: **PASS**

All 64 tests pass with zero failures, zero panics, zero clippy warnings, and zero unsafe code. The implementation fully satisfies the contract's postconditions (Q-01 through Q-07), invariants (I-01 through I-06), and error taxonomy. The sole contract deviation (`archived()` returning `Result` instead of bare reference) is a soundness improvement due to `forbid(unsafe_code)` — the contract should be updated, not the code.
