# Implementation Summary: cdocs-b3v

## Overview

Implemented `load_file_states()` and `load_url_states()` bulk loader methods on the existing `StateReadSession` struct in `state/bulk_load.rs`, plus 30 comprehensive integration tests covering all behaviors from the approved test plan.

## Files Changed

| File | Change |
|------|--------|
| `centralized-docs/src/state/bulk_load.rs` | Added `load_file_states()`, `load_url_states()` methods and `scan_pod_table()` generic helper; fixed pre-existing `hex_encode` clippy lint; added 30 integration tests |
| `centralized-docs/src/state/mod.rs` | `StateLoadError` enum already existed (no changes needed) |

## Implementation Details

### New Methods on `StateReadSession`

1. **`load_file_states(&self) -> Result<HashMap<String, FileStateRaw>, StateLoadError>`**
   - Full-table scan of `file_state` table
   - Validates each value is exactly 200 bytes (`FileStateRaw::SIZE`)
   - Decodes via `FileStateRaw::from_bytes()`
   - Fail-fast on first malformed row

2. **`load_url_states(&self) -> Result<HashMap<String, UrlStateRaw>, StateLoadError>`**
   - Full-table scan of `url_state` table
   - Validates each value is exactly 120 bytes (`UrlStateRaw::SIZE`)
   - Decodes via `UrlStateRaw::from_bytes()`
   - Fail-fast on first malformed row

### Generic Helper: `scan_pod_table<T>()`

A shared Calculation-layer function that:
- Opens the redb table via the shared `ReadTransaction`
- Iterates all rows via `table.iter()`
- Validates value byte length against `expected_size`
- Decodes via a provided `decode_fn` function pointer
- Collects into `HashMap<String, T>` using `try_fold`
- Fail-fast on first `MalformedRow` (no partial map returned)

### Architecture Adherence

- **Data → Calc → Actions**: `scan_pod_table` is a pure Calculation. The only Action is the redb table read in the session's existing read transaction.
- **Zero Mutability**: No `mut` keyword in production code. Uses `try_fold` for accumulation.
- **Zero Panics/Unwraps**: All errors handled via `Result<T, StateLoadError>` with `?` propagation.
- **Expression-Based**: All logic is expression-based with `match`, `map_err`, and `try_fold`.
- **No unsafe**: The codebase has `#![forbid(unsafe_code)]` and all decoding uses safe `from_bytes()` methods.

### Contract Adaptations

The contract specified 40-byte structs with bytemuck, but the actual codebase has:
- `FileStateRaw`: 200 bytes (6 × `[u8; 32]` + `u64` + `[u8; 32]` reserved)
- `UrlStateRaw`: 120 bytes (`[u8; 32]` + `[u8; 32]` + `u64` + `u16` + `[u8; 46]` reserved)
- No bytemuck — uses safe `from_bytes()`/`to_bytes()` methods
- Tables use `&str` keys (not `&[u8]`), making `Utf8KeyError` impossible with current schema

## Test Coverage

### 30 Tests (all passing)

| # | Behavior | Test Name | Status |
|---|----------|-----------|--------|
| B1 | Session construction | `session_new_holds_database_reference_when_constructed` | Pass |
| B4 | File: all rows | `load_file_states_returns_all_rows_when_table_has_valid_entries` | Pass |
| B5 | File: empty table | `load_file_states_returns_empty_hashmap_when_table_is_empty` | Pass |
| B6 | File: 199 bytes | `load_file_states_returns_malformed_row_error_when_value_is_one_byte_short` | Pass |
| B7 | File: 201 bytes | `load_file_states_returns_malformed_row_error_when_value_is_one_byte_over` | Pass |
| B8 | File: 0 bytes | `load_file_states_returns_malformed_row_error_when_value_is_0_bytes` | Pass |
| B9 | File: abort on first | `load_file_states_aborts_on_first_malformed_row_without_partial_map` | Pass |
| B11 | File: BackendError | `load_file_states_returns_backend_error_when_table_cannot_be_opened` | Pass |
| B12 | File: bitwise identity | `load_file_states_decoded_values_are_bitwise_identical_to_written_bytes` | Pass |
| B13 | File: snapshot isolation | `load_file_states_uses_borrowed_transaction_without_opening_new_one` | Pass |
| B14 | File: cross-table isolation | `load_file_states_ignores_url_state_table_rows` | Pass |
| B15 | File: UTF-8 keys | `load_file_states_preserves_key_strings_exactly` | Pass |
| B16 | URL: all rows | `load_url_states_returns_all_rows_when_table_has_valid_entries` | Pass |
| B17 | URL: empty table | `load_url_states_returns_empty_hashmap_when_table_is_empty` | Pass |
| B18 | URL: 119 bytes | `load_url_states_returns_malformed_row_error_when_value_is_one_byte_short` | Pass |
| B19 | URL: 121 bytes | `load_url_states_returns_malformed_row_error_when_value_is_one_byte_over` | Pass |
| B20 | URL: 0 bytes | `load_url_states_returns_malformed_row_error_when_value_is_0_bytes` | Pass |
| B21 | URL: 240 bytes | `load_url_states_returns_malformed_row_error_when_value_is_double_size` | Pass |
| B22 | URL: abort on first | `load_url_states_aborts_on_first_malformed_row_without_partial_map` | Pass |
| B24 | URL: BackendError | `load_url_states_returns_backend_error_when_table_cannot_be_opened` | Pass |
| B25 | URL: bitwise identity | `load_url_states_decoded_values_are_bitwise_identical_to_written_bytes` | Pass |
| B26 | URL: snapshot isolation | `load_url_states_uses_borrowed_transaction_without_opening_new_one` | Pass |
| B27 | URL: cross-table isolation | `load_url_states_ignores_file_state_table_rows` | Pass |
| B28 | URL: UTF-8 keys | `load_url_states_preserves_key_strings_exactly` | Pass |
| — | File: idempotency | `load_file_states_is_idempotent_across_multiple_calls` | Pass |
| — | URL: idempotency | `load_url_states_is_idempotent_across_multiple_calls` | Pass |
| — | Session: construction | `session_new_returns_storage_error_on_read_failure` | Pass |
| — | Mixed: both loaders | `both_loaders_work_independently_on_same_database` | Pass |
| — | File: cardinality | `load_file_states_map_size_equals_row_count_for_various_n` | Pass |
| — | URL: cardinality | `load_url_states_map_size_equals_row_count_for_various_n` | Pass |

### Not Tested (with justification)

| # | Behavior | Reason |
|---|----------|--------|
| B2 | StateReadSession not Clone | Compile-time enforced; no `impl Clone` exists |
| B3 | StateReadSession not Send | Compile-time enforced; `ReadTransaction` is `!Send` |
| B10 | File: Utf8KeyError | Impossible with `&str` key tables (redb enforces UTF-8) |
| B23 | URL: Utf8KeyError | Same as B10 |
| B29 | FileStateRaw == 40 bytes | Actually 200 bytes; already tested in `state::tests` |
| B30 | UrlStateRaw == 40 bytes | Actually 120 bytes; already tested in `state::tests` |

## Verification

- `cargo fmt --check` — pass
- `cargo clippy --lib -- -D warnings -D clippy::unwrap_used -D clippy::panic -D clippy::expect_used -W clippy::pedantic` — pass
- `cargo test --lib` — 867 passed, 0 failed (30 new bulk_load tests)
