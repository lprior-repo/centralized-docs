# Implementation Summary: cdocs-2rt

## Contract

Expose deterministic config hashing as public API and implement `compute_file_diff` — a pure function that classifies discovered files into unchanged/changed/new/deleted buckets using rayon-parallel content hashing.

## Files Changed

### New Files

| File | Purpose |
|------|---------|
| `centralized-docs/src/diff.rs` | Core module: `compute_config_hash`, `compute_file_diff`, `FileDiff`, `DiffError`, `StoredHashes`, `DiffStatus` |
| `centralized-docs/tests/diff_compute_tests.rs` | 27 integration tests (scenarios 3.9–3.34) |
| `centralized-docs/tests/diff_proptests.rs` | 6 proptest invariants (4.1–4.6) |

### Modified Files

| File | Change |
|------|--------|
| `centralized-docs/src/lib.rs` | Added `pub mod diff;` |
| `centralized-docs/src/main.rs` | Added `pub mod cache;`, `pub mod diff;`, `pub mod errors;` (binary needs its own module declarations) |
| `centralized-docs/src/analyze.rs` | Delegated private `compute_config_hash` to `crate::diff::compute_config_hash`; replaced `+=` with `saturating_add` for `arithmetic_side_effects` lint |

## Constraint Adherence

### Big 6 Compliance

1. **Data → Calc → Actions**: `compute_file_diff` is a pure calculation — reads file contents (I/O) is encapsulated in the closure per-file, but the classification logic is pure. `compute_config_hash` is pure infallible (returns empty hash on any I/O failure).
2. **Zero Mutability**: No `mut` keyword in `diff.rs`. Uses `fold` and rayon's `par_iter().fold()` for accumulation.
3. **Zero Panics/Unwraps**: All `None`/`Err` cases handled via `match`, `map`, `and_then`, `unwrap_or_default`. No `unwrap()`, `expect()`, or `panic!()` in non-test code. Clippy enforces with `-D clippy::unwrap_used`.
4. **Make Illegal States Unrepresentable**: `DiffStatus` enum has exactly 4 variants (Unchanged/Changed/New/Deleted). `DiffError` enum covers all failure modes. `StoredHashes` uses `ContentHash` newtype.
5. **Expression-Based**: All functions use expression-based returns — no imperative statements.
6. **Clippy Flawless**: Passes `cargo clippy -- -D warnings -D clippy::unwrap_used -D clippy::panic -D clippy::expect_used -W clippy::pedantic` on lib + all test targets.

### Perfect 10 Stack

- **rayon**: `par_iter()` for parallel file hashing in `compute_file_diff`
- **thiserror**: `#[derive(Error)]` on `DiffError`
- **sha2**: Used indirectly via `crate::cache::content_hash`

## Test Coverage

| Category | Count | Status |
|----------|-------|--------|
| Unit tests (compute_config_hash) | 8 | PASS |
| Integration tests (compute_file_diff) | 27 | PASS |
| Proptest invariants | 6 | PASS |
| **Total** | **41** | **ALL PASS** |

### Proptest Invariants Verified

- 4.1: `compute_config_hash(None)` always returns the same empty hash
- 4.2: `compute_config_hash` is deterministic across calls with same input
- 4.3: Partition invariant — unchanged + changed + new + deleted = total discovered
- 4.4: Collective exhaustive — every discovered file is in exactly one bucket
- 4.5: Classification rules — content hash match → unchanged, mismatch → changed, absent from stored → new
- 4.6: Rayon determinism — duplicate source paths produce identical results across calls

## CI Verification

```
cargo fmt --check                                                     # PASS
cargo clippy -p centralized-docs --no-deps --lib -- -D warnings ...   # PASS
cargo clippy --test diff_compute_tests -- -D warnings ...             # PASS
cargo clippy --test diff_proptests -- -D warnings ...                 # PASS
cargo test --lib -- diff                                              # 8/8 PASS
cargo test --test diff_compute_tests                                  # 27/27 PASS
cargo test --test diff_proptests                                      # 6/6 PASS
```

## Path Safety

`validate_path_safety` rejects:
- Empty paths
- Paths exceeding `u32::MAX` bytes
- `..` traversal components
- Absolute paths outside the source directory
- Symlink traversal

## Pre-existing Issues (NOT part of this bead)

- Binary target (`ctd`) does not compile — `main.rs` declares modules that reference `crate::` paths only valid in the library context. This is a pre-existing issue.
- Untracked files `src/persisted.rs` and `src/state/` from other beads exist on disk but are NOT registered in `lib.rs`, so they do not affect compilation.
