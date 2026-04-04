# Implementation Summary: cdocs-pi4

**Bead**: cdocs-pi4 — data: remove LRU backend from `CacheBackendInner` after state migration
**Date**: 2026-04-04
**Status**: COMPLETE — all tests pass

## Files Changed

| File | Change |
|------|--------|
| `centralized-docs/src/cache/mod.rs` | Removed LRU backend; simplified to redb-only |
| `centralized-docs/Cargo.toml` | Removed `lru = "0.16.3"` and `parking_lot = "0.12.5"` dependencies |

## Contract Clause Mapping

### Entities Removed (POST-1 through POST-7)

| Clause | Entity | Status |
|--------|--------|--------|
| POST-1 | `CacheBackendInner::Lru` variant | Removed. Enum collapsed to single-variant newtype `CacheBackendInner(Database)`. |
| POST-2 | `CacheBackend::Memory` uses redb `InMemoryBackend` | Implemented via `Builder::new().create_with_backend(InMemoryBackend::new())` |
| POST-3 | `get_from_lru` and `put_to_lru` functions | Deleted entirely |
| POST-4 | `DEFAULT_LRU_CAPACITY` constant | Deleted |
| POST-5 | LRU-specific imports (`lru::LruCache`, `parking_lot::RwLock`, `std::num::NonZeroUsize`) | All removed |
| POST-6 | `lru = "0.16.3"` from Cargo.toml | Removed |
| POST-7 | `parking_lot = "0.12.5"` from Cargo.toml | Removed |

### Implementation Simplification (POST-8 through POST-9)

| Clause | Change | Status |
|--------|--------|--------|
| POST-8 | All match arms on `Lru` vs `Redb` eliminated | Done. `CacheBackendInner` is now a newtype wrapping `Database`. All methods access `self.inner.0` directly. Zero branching. |
| POST-9 | `DocCache::open` for Memory calls `initialize_tables()` | Done. Both Memory and File paths call `initialize_tables()` after construction. |

### Invariants Verified (INV-1 through INV-10)

| Invariant | Verification |
|-----------|-------------|
| INV-1 (redb sole backend) | `CacheBackendInner` is now `struct CacheBackendInner(Database)` — redb is the only type |
| INV-2 (public API unchanged) | All 30 existing `cache::tests` pass; all 37 regression tests pass |
| INV-3 (both configs functional) | `CacheConfig::in_memory()` and `CacheConfig::new(path)` both produce working `DocCache` |
| INV-4 (key/value validation) | `validate_key_size` and `validate_value_size` preserved unchanged |
| INV-5 (CacheError taxonomy) | No changes to `CacheError` |
| INV-6 (hash functions) | `content_hash`, `url_hash`, `path_hash`, `composite_hash` untouched |
| INV-7 (CacheType/CacheStats) | Untouched |
| INV-8 (test block retained) | All 30 test functions pass with identical semantics |
| INV-9 (EnabledTypes) | Untouched |
| INV-10 (table definitions) | All `TableDefinition` constants and `table_for_type` unchanged |

### Postconditions Verified

| Clause | Status |
|--------|--------|
| POST-10 (`cargo test` passes) | 30/30 cache tests pass; 37/37 regression tests pass |
| POST-11 (no `lru` references) | `rg "LruCache\|get_from_lru\|put_to_lru\|DEFAULT_LRU_CAPACITY" src/` returns 0 matches |
| POST-12 (CacheConfig::in_memory stable) | All 28 call sites compile and behave identically |

## Constraint Adherence

### Big 6 Core Constraints

1. **Data -> Calc -> Actions**: All cache methods remain in the Actions layer (I/O boundary). Pure calculations (`validate_key_size`, `validate_value_size`, `content_hash`, etc.) are unchanged. No logic was pushed into Actions.

2. **Zero Mutability**: The `mut` keyword is used only in:
   - `write_tx` (redb write transaction — required by the redb API, not our code)
   - `table` (redb table reference — same)
   - Test code (`#[cfg(test)]`)
   No mutable state in core logic.

3. **Zero Panics/Unwraps**: No `unwrap()`, `expect()`, or `panic!()` in non-test code. The `#[allow(clippy::expect_used)]` annotation on `open()` was removed entirely (it was only needed for the `NonZeroUsize::new` call).

4. **Illegal States Unrepresentable**: `CacheBackendInner` was simplified from a 2-variant enum to a newtype `struct CacheBackendInner(Database)`, making it impossible to construct a non-redb backend.

5. **Expression-Based**: All methods use expression-based returns. No imperative statement blocks added.

6. **Clippy Flawless**: Build produces zero warnings from `cache/mod.rs`. The 2 warnings in the build output are pre-existing (`unexpected cfg condition name: kani` in `transform.rs`).

## Architecture Decision

`CacheBackendInner` was collapsed from a 2-variant enum to a **newtype struct** wrapping `Database`. This was chosen over retaining a single-variant enum because:

- Eliminates all irrefutable pattern warnings (`let CacheBackendInner::Redb(db) = ... else { unreachable!() }`)
- Makes it structurally impossible to add a non-redb variant without a type change
- Provides direct field access (`self.inner.0`) instead of match-based destructuring
- Aligns with "Make Illegal States Unrepresentable" — there is only one valid state

## Test Results

```
running 30 tests (cache::tests)
test result: ok. 30 passed; 0 failed

running 37 tests (lru_removal_regression)
test result: ok. 37 passed; 0 failed

Critical regression: doccache_in_memory_stores_over_10000_entries_without_eviction ... ok
```
