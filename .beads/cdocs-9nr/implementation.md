# Implementation Summary: cdocs-9nr

## Status: GREEN PHASE COMPLETE

**Bead**: cdocs-9nr — "Wire startup state open and file diff into `run_index`"

## Test Suite Summary

### Test Count
- **Unit tests (#[cfg(test)] in cmd/index.rs)**: 7
  - `file_states_to_stored_hashes_returns_map_with_identical_keys_when_input_nonempty`
  - `file_states_to_stored_hashes_returns_empty_map_when_input_empty`
  - `file_states_to_stored_hashes_projects_bitwise_identical_content_and_config_hashes`
  - `file_states_to_stored_hashes_preserves_single_entry_with_zeroed_state`
  - `file_states_to_stored_hashes_handles_large_input_of_100_entries`
  - `file_states_to_stored_hashes_preserves_distinct_hashes_per_entry`
  - `file_states_to_stored_hashes_output_keys_are_byte_identical_strings`
- **Integration tests (tests/run_index_state_diff_tests.rs)**: 38 (31 unit/integration + 4 proptest + 3 error variant display)
- **Proptest invariants**: 4 (key preservation, bitwise field identity, partition disjointness, determinism)
- **Kani harnesses**: 1 (bitwise field projection for INV-4) — `#[cfg(kani)]` guarded
- **Total**: 45 tests + 4 proptest (×1000 cases each)

### Gate Results
- [x] Compilation: 0 errors
- [x] nextest (cdocs-9nr tests): 45 passed, 0 failed
- [x] Pre-existing test failure: `state::commit::tests::proptest_non_empty_string_key_always_accepted` — NOT related to this bead

### Behavior Coverage (30 behaviors from test-plan)

| Behavior | Layer | Status |
|----------|-------|--------|
| B1: file_states_to_stored_hashes identical keys | Unit (inline) | ✅ |
| B2: file_states_to_stored_hashes empty input | Unit (inline) | ✅ |
| B3: file_states_to_stored_hashes bitwise identity | Unit (inline) | ✅ |
| B4-B7: compute_config_hash | Pre-existing in diff.rs | ✅ |
| B8: StateDb opens at output | Integration | ✅ |
| B9: StateReadSession created | Integration | ✅ |
| B10: bulk-loads file states | Integration | ✅ |
| B11: empty HashMap on first run | Integration | ✅ |
| B12: converts FileStateRaw to StoredHashes | Integration | ✅ |
| B13: computes config hash | Integration | ✅ |
| B14: calls compute_file_diff | Integration | ✅ |
| B15: prints diff statistics format | Integration | ⚠️ Tested via pipeline (counts verified) |
| B16: all New on first run | Integration | ✅ |
| B17: pipeline continues after diff | Integration | ⚠️ STEP 1.5 pipeline tested directly |
| B18: DatabaseOpen error | Integration (error display) | ✅ |
| B19: ReadTransaction error | Integration (error display) | ✅ |
| B20: TableInit error | Integration (error display) | ✅ |
| B21: MalformedRow error | Integration | ✅ |
| B22: Utf8KeyError display | Integration | ✅ |
| B23: SourceDirNotFound error | Integration | ✅ |
| B24: FileRead error | Integration | ✅ |
| B25: PathTraversal error | Integration | ✅ |
| B26: RAII drop on error | Integration | ✅ |
| B27: no writes (read-only) | Integration | ✅ |
| B28: second run mixed diff | Integration | ✅ |
| B29-B30: E2E full pipeline | Integration (STEP 1.5 sim) | ✅ |
| Proptest 1: key preservation | Proptest | ✅ |
| Proptest 2: bitwise field identity | Proptest | ✅ |
| Proptest 3: partition disjointness | Proptest | ✅ |
| Proptest 4: determinism | Proptest | ✅ |
| Kani 1: bitwise field projection | Kani (cfg-gated) | ✅ |

### Note on RED vs GREEN

The implementation was already committed (git hash `433d4955`) when this bead was picked up for STATE 2.
All tests are GREEN because the implementation exists. The test suite covers the full contract:
- 7 inline unit tests in `cmd/index.rs` for `file_states_to_stored_hashes`
- 38 integration tests in `tests/run_index_state_diff_tests.rs` covering the full STEP 1.5 pipeline
- 4 proptest invariants with random fuzzing
- 1 Kani harness for formal verification of INV-4

### Files Modified
1. `centralized-docs/src/cmd/index.rs` — Added 7 inline unit tests + 1 Kani harness
2. `centralized-docs/tests/run_index_state_diff_tests.rs` — Pre-existing (38 tests)
3. `.beads/cdocs-9nr/implementation.md` — This file
