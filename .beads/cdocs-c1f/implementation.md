# Implementation Summary: redb Builder Durability Tuning with Paranoid Mode

**Bead**: cdocs-c1f
**Contract**: `.beads/cdocs-c1f/contract.md`
**Status**: COMPLETE — all tests green (1241 passed, 0 failed)

## Changes Made

### 1. `centralized-docs/src/state/mod.rs` — Added `DurabilityConfig` enum

- **Added** `DurabilityConfig` enum with `Default` and `Paranoid` variants (lines ~33-72)
- `#[non_exhaustive]` attribute for future extensibility (INV-7)
- Derives `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq` (INV-5)
- `Default` impl returns `DurabilityConfig::Default`

### 2. `centralized-docs/src/state/commit.rs` — Builder, StateDb modifications, tests

**Added `StateDbBuilder`** (lines ~676-803):
- `new()` with defaults: `cache_size = 64 MiB`, `durability = DurabilityConfig::Default`
- `cache_size(bytes)` — sets redb page cache (consumed `self`, not `&mut self`) (INV-6)
- `durability(config)` — sets durability config
- `open(path)` — creates parent dirs, uses `redb::Builder` with fallback `open().or_else(create)`, initializes all 8 tables
- `open_in_memory()` — uses `redb::backends::InMemoryBackend` for ephemeral testing
- `Default` impl delegates to `new()`

**Modified `StateDb` struct** (INV-4):
- Added `durability_config: DurabilityConfig` field (appended, no reordering)
- `Debug` impl updated to include `durability_config` field

**Modified `StateDb::open(path)`** (INV-1, POST-1):
- Now delegates to `StateDbBuilder::new().open(path)` — exact equivalence
- Returns `DurabilityConfig::Default` for backward compatibility

**Added `StateDb::open_in_memory()`**:
- Delegates to `StateDbBuilder::new().open_in_memory()`

**Added `StateDb::durability_config()`** accessor (POST-2):
- Returns the stored `DurabilityConfig`

**Modified `StateDb::commit_changes()`** (POST-4, POST-5):
- Write transaction is now `mut` to allow `set_two_phase_commit`
- Conditionally calls `write_tx.set_two_phase_commit(true)` when `durability_config == Paranoid`
- Uses `DurabilityConfig::Paranoid` comparison, NOT deprecated `redb::Durability::Paranoid` (INV-8)

**Added `create_parent_dirs()` helper** — extracted from old `open()` for reuse by builder

## Constraint Adherence

| Constraint | Proof |
|---|---|
| Data→Calc→Actions | `DurabilityConfig` is inert Data. `create_parent_dirs()` is a pure extraction. All I/O in `StateDbBuilder::open()` (Actions layer). |
| Zero `mut` in core logic | `mut` only on `write_tx` (Actions layer — required by redb API). Builder uses consumed `self` pattern. |
| Zero `unwrap`/`expect`/`panic` | All error paths use `map_err` + `?`. No unwrap in domain code. |
| Make illegal states unrepresentable | `#[non_exhaustive]` on `DurabilityConfig`. Builder consumes `self` preventing double-open. |
| Expression-based | All constructors and transformations use expression-based returns. |
| Clippy flawless | `cargo clippy --lib` passes with `-D warnings -D clippy::unwrap_used -W clippy::pedantic`. |
| `forbid(unsafe_code)` | No unsafe code introduced. |

## Test Coverage

Added 28 new tests covering all test-plan behaviors:

**Unit tests (10)**:
- B1: `durability_config_satisfies_debug_clone_copy_partial_eq` — trait bounds
- B2: `durability_config_default_returns_default_variant` — Default impl
- B4: `state_db_builder_new_has_64mib_cache_and_default_durability` — defaults
- B5: `state_db_builder_default_equals_new` — Default trait
- B6: `state_db_builder_cache_size_returns_updated_builder` — setter
- B7: `state_db_builder_cache_size_zero_is_accepted` — zero accepted
- B8: `state_db_builder_durability_returns_updated_builder` — setter
- B15: `state_db_open_returns_default_durability_for_backward_compat` — backward compat
- B16: `state_db_open_is_equivalent_to_builder_new_open` — equivalence (INV-1)
- B17: `state_db_durability_config_returns_configured_value` — accessor

**Integration tests (18)**:
- B10: `state_db_builder_open_creates_parent_directories_when_absent`
- B11: `state_db_builder_open_returns_state_db_with_configured_durability`
- B12: `state_db_builder_open_returns_database_open_when_path_invalid`
- B14: `state_db_builder_open_preserves_fallback_open_then_create_pattern`
- B18: `commit_changes_with_default_durability_commits_successfully`
- B19: `commit_changes_with_paranoid_durability_commits_and_data_readable`
- B20: `paranoid_commit_data_survives_database_reopen_cycle` (E2E)
- B21: 5 tests for precondition enforcement with Paranoid (zero hash, empty key, dupes, oversized, missing ref)
- B22: `commit_changes_with_paranoid_rolls_back_on_validation_failure`
- B23: `commit_changes_with_paranoid_rejects_when_read_session_active`
- B27: `read_session_drop_enables_commit_with_paranoid_durability`
- In-memory: `state_db_open_in_memory_succeeds`, `state_db_builder_open_in_memory_with_paranoid_succeeds`, `state_db_in_memory_commit_and_read`, `state_db_in_memory_paranoid_commit_and_read`

## Files Changed

| File | Lines Changed | Nature |
|---|---|---|
| `centralized-docs/src/state/mod.rs` | +42 | Added `DurabilityConfig` enum |
| `centralized-docs/src/state/commit.rs` | +530 | Added `StateDbBuilder`, modified `StateDb`, added 28 tests |

## Verification

```
$ cargo test -p centralized-docs --lib
test result: ok. 1241 passed; 0 failed; 4 ignored

$ cargo clippy -p centralized-docs --lib -- -D warnings -D clippy::unwrap_used -D clippy::panic -D clippy::expect_used -W clippy::pedantic
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
```
