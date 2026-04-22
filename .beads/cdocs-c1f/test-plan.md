# Test Plan: redb Builder Durability Tuning with Paranoid Mode

**Bead**: cdocs-c1f
**Contract**: `.beads/cdocs-c1f/contract.md`
**Source**: `centralized-docs/src/state/commit.rs`, `centralized-docs/src/state/mod.rs`

## Summary

- **Behaviors identified**: 28
- **Trophy allocation**: 10 unit / 16 integration / 1 e2e / 1 static
- **Proptest invariants**: 5
- **Fuzz targets**: 0 (no new parsing boundaries — `DurabilityConfig` is a plain enum, `StateDbBuilder` is a config struct)
- **Kani harnesses**: 0 (no critical arithmetic or index bounds introduced)
- **Mutation kill target**: ≥90%

---

## 1. Behavior Inventory

### DurabilityConfig (enum)

| # | Behavior |
|---|----------|
| B1 | `DurabilityConfig` derives `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq` (value type semantics) |
| B2 | `DurabilityConfig::default()` returns `DurabilityConfig::Default` |
| B3 | `DurabilityConfig` is `#[non_exhaustive]` — cannot be exhaustively matched outside the crate |

### StateDbBuilder (builder)

| # | Behavior |
|---|----------|
| B4 | `StateDbBuilder::new()` returns builder with `cache_size = 64 MiB`, `durability = DurabilityConfig::Default` |
| B5 | `StateDbBuilder::default()` equals `StateDbBuilder::new()` |
| B6 | `StateDbBuilder::cache_size(n)` returns builder with updated cache_size |
| B7 | `StateDbBuilder::cache_size(0)` is accepted (redb uses its own default ~1 GiB) |
| B8 | `StateDbBuilder::durability(config)` returns builder with updated durability |
| B9 | `StateDbBuilder::open(self, path)` consumes builder (not `&mut self`) |
| B10 | `StateDbBuilder::open(path)` creates parent directories when absent |
| B11 | `StateDbBuilder::open(path)` returns `StateDb` with configured settings |
| B12 | `StateDbBuilder::open(path)` returns `CommitError::DatabaseOpen` when redb fails |
| B13 | `StateDbBuilder::open(path)` returns `CommitError::TableInit` when table init fails |
| B14 | `StateDbBuilder::open(path)` preserves fallback pattern: `open().or_else(create)` |

### StateDb (modified)

| # | Behavior |
|---|----------|
| B15 | `StateDb::open(path)` returns `StateDb` with `DurabilityConfig::Default` (backward compatible) |
| B16 | `StateDb::open(path)` is exactly equivalent to `StateDbBuilder::new().open(path)` (INV-1) |
| B17 | `StateDb::durability_config()` returns the stored durability configuration |
| B18 | `StateDb::commit_changes` with `DurabilityConfig::Default` does NOT call `set_two_phase_commit` |
| B19 | `StateDb::commit_changes` with `DurabilityConfig::Paranoid` calls `set_two_phase_commit(true)` before writes |
| B20 | `StateDb::commit_changes` with `DurabilityConfig::Paranoid` writes data that survives re-open |
| B21 | `StateDb::commit_changes` precondition validation (all existing preconditions) still enforced when durability is Paranoid |
| B22 | `StateDb::commit_changes` rollback on validation failure still works with Paranoid durability |
| B23 | `StateDb::commit_changes` with `DurabilityConfig::Paranoid` still rejects active read sessions |

### Invariant / Structural

| # | Behavior |
|---|----------|
| B24 | `StateDb` struct layout: `{ db, active_read_sessions, durability_config }` — new field appended |
| B25 | `StateDb::database()` accessor still works after struct change |
| B26 | `StateDb::drop_snapshots_table()` still works after struct change |
| B27 | `StateReadSession` Drop still decrements `active_read_sessions` after struct change |
| B28 | All existing `commit.rs` tests (790–3127) pass unchanged |

---

## 2. Trophy Allocation

| Behavior | Layer | Rationale |
|----------|-------|-----------|
| B1: DurabilityConfig derives traits | **Static** | Compile-time trait bound check — `assert_traits::<DurabilityConfig>()` |
| B2: DurabilityConfig::default() | **Unit** | Pure function, no I/O |
| B3: non_exhaustive | **Static** | Compile-time attribute — verified by attempting exhaustive match in doc test |
| B4: StateDbBuilder::new() defaults | **Unit** | Pure constructor, no I/O |
| B5: Default equals new() | **Unit** | Pure comparison |
| B6: cache_size(n) returns builder | **Unit** | Pure method chain, no I/O |
| B7: cache_size(0) accepted | **Unit** | Pure value pass-through |
| B8: durability(config) returns builder | **Unit** | Pure method chain |
| B9: open consumes builder (self) | **Static** | Compile-time: `StateDbBuilder::open` takes `self`, not `&mut self` — enforced by type system |
| B10: open creates parent dirs | **Integration** | Filesystem side effect |
| B11: open returns configured StateDb | **Integration** | redb database creation + table init |
| B12: open returns DatabaseOpen on failure | **Integration** | redb error mapping |
| B13: open returns TableInit on failure | **Integration** | Hard to trigger — documented as code-coverage gap |
| B14: open preserves fallback pattern | **Integration** | Corrupt-file then re-open scenario |
| B15: StateDb::open backward compat | **Integration** | Full `StateDb::open` then verify durability_config |
| B16: StateDb::open == builder new().open() | **Integration** | Compare both paths produce same defaults |
| B17: durability_config() accessor | **Unit** | Pure getter on constructed StateDb |
| B18: commit with Default — no two-phase | **Integration** | Write + read verification via redb |
| B19: commit with Paranoid — two-phase | **Integration** | Write + verify two-phase applied (observable via successful commit + data durability) |
| B20: Paranoid commit survives re-open | **E2E** | Full lifecycle: builder open → commit → close → reopen → verify |
| B21: Preconditions still enforced (Paranoid) | **Integration** | ZeroHashKey, EmptyStringKey, etc. with Paranoid config |
| B22: Rollback still works (Paranoid) | **Integration** | Validation failure with Paranoid — no writes visible |
| B23: Read session rejection (Paranoid) | **Integration** | Active session + Paranoid commit = WriteTransaction error |
| B24: Struct layout | **Static** | Compile-time: field order verified by code review + existing tests pass |
| B25: database() accessor works | **Integration** | Existing test coverage — regression |
| B26: drop_snapshots_table() works | **Integration** | Existing test coverage — regression |
| B27: ReadSession Drop decrements | **Integration** | Existing test coverage — regression |
| B28: All existing tests pass | **Static** | CI gate — `cargo test` green |

**Allocation**: 10 unit, 16 integration, 1 e2e, 1 static (trait bounds), plus 1 static (CI green on existing suite).
Deviation from 60/30/5/5: This is a config/infra feature, heavy on integration because every behavior involves redb. Unit covers pure builder methods. Justified.

---

## 3. BDD Scenarios

### B1: DurabilityConfig satisfies required traits

```
Given: the DurabilityConfig type is defined
When:  the compiler checks trait bounds
Then:  DurabilityConfig: Debug + Clone + Copy + PartialEq + Eq compiles

Test: fn durability_config_satisfies_debug_clone_copy_partial_eq()
```

### B2: DurabilityConfig::default() returns Default variant

```
Given: nothing
When:  DurabilityConfig::default() is called
Then:  result == DurabilityConfig::Default

Test: fn durability_config_default_returns_default_variant()
```

### B4: StateDbBuilder::new() has correct defaults

```
Given: nothing
When:  StateDbBuilder::new() is called
Then:  builder.cache_size == 67_108_864 (64 MiB)
  And: builder.durability == DurabilityConfig::Default

Test: fn state_db_builder_new_has_64mib_cache_and_default_durability()
```

### B5: StateDbBuilder::default() equals new()

```
Given: nothing
When:  StateDbBuilder::default() and StateDbBuilder::new() are called
Then:  both produce builders with identical cache_size and durability

Test: fn state_db_builder_default_equals_new()
```

### B6: StateDbBuilder::cache_size(n) sets cache

```
Given: a StateDbBuilder
When:  .cache_size(128 * 1024 * 1024) is called
Then:  returned builder.cache_size == 134_217_728

Test: fn state_db_builder_cache_size_returns_updated_builder()
```

### B7: StateDbBuilder::cache_size(0) accepted

```
Given: a StateDbBuilder
When:  .cache_size(0) is called
Then:  returned builder.cache_size == 0 (redb will use its default)

Test: fn state_db_builder_cache_size_zero_is_accepted()
```

### B8: StateDbBuilder::durability(config) sets durability

```
Given: a StateDbBuilder
When:  .durability(DurabilityConfig::Paranoid) is called
Then:  returned builder.durability == DurabilityConfig::Paranoid

Test: fn state_db_builder_durability_returns_updated_builder()
```

### B10: StateDbBuilder::open creates parent directories

```
Given: a path like /tmp/does_not_exist_xxx/sub/state.redb where parent dirs don't exist
When:  StateDbBuilder::new().open(path) is called
Then:  parent directories are created
  And: result is Ok(StateDb)

Test: fn state_db_builder_open_creates_parent_directories_when_absent()
```

### B11: StateDbBuilder::open returns configured StateDb

```
Given: a temp directory and valid path
When:  StateDbBuilder::new()
         .cache_size(32 * 1024 * 1024)
         .durability(DurabilityConfig::Paranoid)
         .open(path)
Then:  result is Ok(StateDb)
  And: state_db.durability_config() == DurabilityConfig::Paranoid

Test: fn state_db_builder_open_returns_state_db_with_configured_durability()
```

### B12: StateDbBuilder::open returns DatabaseOpen on failure

```
Given: an invalid path like "/nonexistent_root_xyz_cdocs/deeply/nested/state.redb"
When:  StateDbBuilder::new().open(path)
Then:  Err(CommitError::DatabaseOpen { path, .. })
  And: error message contains the path

Test: fn state_db_builder_open_returns_database_open_when_path_invalid()
```

### B14: StateDbBuilder::open preserves fallback pattern

```
Given: a file at path containing garbage bytes (corrupt redb file)
When:  StateDbBuilder::new().open(path)
Then:  Err(CommitError::DatabaseOpen { .. }) — both open and create fail on corrupt file

Test: fn state_db_builder_open_preserves_fallback_open_then_create_pattern()
```

### B15: StateDb::open backward compatible

```
Given: a temp directory and valid path
When:  StateDb::open(path) is called
Then:  result is Ok(StateDb)
  And: state_db.durability_config() == DurabilityConfig::Default

Test: fn state_db_open_returns_default_durability_for_backward_compat()
```

### B16: StateDb::open equivalent to builder new().open()

```
Given: two temp directories with valid paths
When:  StateDb::open(path_a) and StateDbBuilder::new().open(path_b)
Then:  both succeed
  And: both return StateDb with durability_config() == DurabilityConfig::Default

Test: fn state_db_open_is_equivalent_to_builder_new_open()
```

### B17: durability_config() accessor

```
Given: a StateDb opened with DurabilityConfig::Paranoid
When:  state_db.durability_config() is called
Then:  result == DurabilityConfig::Paranoid

Test: fn state_db_durability_config_returns_configured_value()
```

### B18: commit_changes with Default does not use two-phase

```
Given: a StateDb opened with DurabilityConfig::Default
When:  commit_changes is called with valid StateChanges
Then:  commit succeeds
  And: data is readable from the database

Note: Two-phase commit is an internal implementation detail. The observable
behavior is that data is committed and readable. The absence of two-phase
is verified by the fact that Default mode has always worked (existing tests).

Test: fn commit_changes_with_default_durability_commits_successfully()
```

### B19: commit_changes with Paranoid uses two-phase commit

```
Given: a StateDb opened with DurabilityConfig::Paranoid
When:  commit_changes is called with valid StateChanges containing one analysis entry
Then:  commit succeeds (Ok(()))
  And: analysis entry is readable from analysis_outputs table

Test: fn commit_changes_with_paranoid_durability_commits_and_data_readable()
```

### B20: Paranoid commit data survives database re-open (E2E)

```
Given: a StateDb opened with DurabilityConfig::Paranoid at path P
When:  commit_changes writes a file_state entry with key "test.md"
  And: StateDb is dropped
  And: StateDb is reopened at path P
Then:  file_state entry for "test.md" is still present with correct bytes

Test: fn paranoid_commit_data_survives_database_reopen_cycle()
```

### B21: Preconditions enforced with Paranoid durability

```
Given: a StateDb opened with DurabilityConfig::Paranoid
When:  commit_changes is called with a zero hash key in new_analyses
Then:  Err(CommitError::ZeroHashKey { table: "analysis_outputs", index: 0 })

Error variant:
Given: Paranoid StateDb
When:  commit_changes with empty string key in updated_files
Then:  Err(CommitError::EmptyStringKey { table: "file_state", index: 0 })

Error variant:
Given: Paranoid StateDb
When:  commit_changes with duplicate source_path keys
Then:  Err(CommitError::DuplicateStateKey { table: "file_state", key: "dup.md" })

Error variant:
Given: Paranoid StateDb
When:  commit_changes with payload exceeding MAX_VALUE_SIZE
Then:  Err(CommitError::PayloadTooLarge { table: "analysis_outputs", size: 50*1024*1024+1, max: 50*1024*1024 })

Error variant:
Given: Paranoid StateDb
When:  commit_changes with missing reference (non-zero hash in FileStateRaw with no matching payload)
Then:  Err(CommitError::MissingReference { table: "file_state", field: "analysis_hash", payload_table: "analysis_outputs", .. })

Test: fn commit_changes_with_paranoid_rejects_zero_hash_key()
Test: fn commit_changes_with_paranoid_rejects_empty_string_key()
Test: fn commit_changes_with_paranoid_rejects_duplicate_keys()
Test: fn commit_changes_with_paranoid_rejects_oversized_payload()
Test: fn commit_changes_with_paranoid_rejects_missing_reference()
```

### B22: Rollback on validation failure with Paranoid

```
Given: a StateDb opened with DurabilityConfig::Paranoid
When:  commit_changes includes both a valid file entry AND a zero hash payload
Then:  Err(CommitError::ZeroHashKey { .. })
  And: the valid file entry is NOT present in the database

Test: fn commit_changes_with_paranoid_rolls_back_on_validation_failure()
```

### B23: Read session rejection with Paranoid

```
Given: a StateDb opened with DurabilityConfig::Paranoid
  And: an active StateReadSession (not yet dropped)
When:  commit_changes is called with valid StateChanges
Then:  Err(CommitError::WriteTransaction { reason })
  And: reason contains "read session"

Test: fn commit_changes_with_paranoid_rejects_when_read_session_active()
```

### B27: ReadSession Drop still decrements counter

```
Given: a StateDb with DurabilityConfig::Paranoid
When:  begin_read() creates a session
  And: the session is dropped
Then:  commit_changes succeeds (counter back to 0)

Test: fn read_session_drop_enables_commit_with_paranoid_durability()
```

---

## 4. Proptest Invariants

### Proptest: DurabilityConfig round-trip through assignment

```
Invariant: For any DurabilityConfig value, assigning it to a variable and
comparing with == yields true. Copy semantics hold: let a = config; let b = a;
assert_eq!(a, b).
Strategy: any::<DurabilityConfig>() — proptest enum with 2 variants
Anti-invariant: N/A (enum with no invalid states)
```

### Proptest: StateDbBuilder chain preserves all settings

```
Invariant: builder.cache_size(X).durability(Y).cache_size(Z) has cache_size == Z
and durability == Y (last-write-wins per field).
Strategy: any::<usize> × any::<DurabilityConfig> × any::<usize>
Anti-invariant: N/A (no invalid builder states exist)
```

### Proptest: should_skip_write with Paranoid — unchanged rows still skipped

```
Invariant: For any byte arrays existing and new where existing == new,
commit_changes with DurabilityConfig::Paranoid does not rewrite the row.
(Verified by committing identical data twice and checking the second commit
does not corrupt data.)
Strategy: any::<Vec<u8>> (arbitrary payload bytes)
Anti-invariant: Different bytes should always be written
```

### Proptest: Paranoid commit + reopen — data integrity

```
Invariant: For any valid StateChanges (generated via proptest strategies),
data committed with DurabilityConfig::Paranoid is exactly equal to data
read after reopen.
Strategy: valid FileStateRaw (all fields random), valid hash keys (non-zero)
Anti-invariant: Zero hash keys always rejected regardless of durability
```

### Proptest: Mixed Default/Paranoid databases coexist on different paths

```
Invariant: Two databases opened on different paths with different durability
configs can be written to independently. Data in DB_A (Default) is unaffected
by writes to DB_B (Paranoid) and vice versa.
Strategy: two temp paths × valid StateChanges × both durability configs
Anti-invariant: Same-path concurrent access is not tested (redb file lock)
```

---

## 5. Fuzz Targets

No new fuzz targets required. This contract introduces:
- `DurabilityConfig`: a two-variant enum with no parsing boundary
- `StateDbBuilder`: a plain struct constructor with no deserialization
- `StateDb` modifications: internal field addition and conditional method call

No parser, deserializer, or raw-byte boundary is introduced. The existing fuzz target for `ArchivedRaw::try_from_bytes` (G20/B95 in commit.rs) continues to cover the rkyv deserialization boundary.

---

## 6. Kani Harnesses

No Kani harnesses required. This contract introduces:
- No critical arithmetic (no new index math, no overflow-prone calculations)
- No new state machine transitions (durability is a config, not a state)
- No pointer/index manipulation

The `cache_size` field is `usize` and is passed directly to redb without arithmetic.

---

## 7. Mutation Testing Checkpoints

**Threshold: ≥90% mutation kill rate**

### Critical mutations to catch:

| Mutation | Caught by test |
|----------|---------------|
| `DurabilityConfig::default()` returns `Paranoid` instead of `Default` | `durability_config_default_returns_default_variant()` |
| `StateDbBuilder::new()` cache_size set to wrong value | `state_db_builder_new_has_64mib_cache_and_default_durability()` |
| `StateDbBuilder::cache_size()` doesn't update field (no-op) | `state_db_builder_cache_size_returns_updated_builder()` |
| `StateDbBuilder::durability()` doesn't update field (no-op) | `state_db_builder_durability_returns_updated_builder()` |
| `StateDbBuilder::open()` doesn't create parent dirs | `state_db_builder_open_creates_parent_directories_when_absent()` |
| `StateDbBuilder::open()` doesn't call initialize_tables | Regression: existing `all_8_tables_survive_database_reopen()` test |
| `StateDb::open()` doesn't store DurabilityConfig | `state_db_open_returns_default_durability_for_backward_compat()` |
| `durability_config()` returns wrong variant | `state_db_durability_config_returns_configured_value()` |
| `commit_changes` with Paranoid doesn't call `set_two_phase_commit(true)` | `commit_changes_with_paranoid_durability_commits_and_data_readable()` — mutation would change observable behavior (if redb treats the absence differently under crash) |
| `commit_changes` with Paranoid calls `set_two_phase_commit(false)` | Same test — negated boolean |
| Validation skipped when durability is Paranoid | `commit_changes_with_paranoid_rejects_zero_hash_key()` and all B21 tests |
| Fallback pattern removed (`open` without `or_else create`) | `state_db_builder_open_preserves_fallback_open_then_create_pattern()` |
| `active_read_sessions` check removed for Paranoid | `commit_changes_with_paranoid_rejects_when_read_session_active()` |

### Mutations NOT caught (acceptable gap):

- `set_two_phase_commit(true)` vs `set_two_phase_commit(false)`: Both produce the same test outcome (successful commit) under normal operation. The difference is only observable during OS crash. This is an acceptable gap — redb's own test suite covers two-phase commit semantics.

---

## 8. Combinatorial Coverage Matrix

### DurabilityConfig unit tests

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| default | none | `DurabilityConfig::Default` | unit |
| Debug format | `DurabilityConfig::Default` | contains "Default" | unit |
| Debug format | `DurabilityConfig::Paranoid` | contains "Paranoid" | unit |
| Clone | `DurabilityConfig::Paranoid` | equal to original | unit |
| Copy | `DurabilityConfig::Default` | equal to original after copy | unit |
| PartialEq eq | `Default == Default` | true | unit |
| PartialEq neq | `Default != Paranoid` | true | unit |

### StateDbBuilder unit tests

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| new() defaults | none | cache_size=67108864, durability=Default | unit |
| default() equals new() | none | all fields equal | unit |
| cache_size(n) | valid usize | updated cache_size | unit |
| cache_size(0) | zero | cache_size=0 accepted | unit |
| durability(Paranoid) | Paranoid variant | updated durability | unit |
| method chaining | cache_size then durability | both updated | unit |

### StateDbBuilder integration tests

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| open valid path | temp dir + valid path | Ok(StateDb) with configured durability | integration |
| open creates parent dirs | nested nonexistent path | Ok(StateDb) + dirs exist | integration |
| open invalid path | `/nonexistent_root/...` | Err(CommitError::DatabaseOpen { .. }) | integration |
| open corrupt file | garbage bytes at path | Err(CommitError::DatabaseOpen { .. }) | integration |
| open + commit (Default) | valid StateChanges | Ok(()) + data readable | integration |
| open + commit (Paranoid) | valid StateChanges | Ok(()) + data readable | integration |

### StateDb integration tests

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| open (legacy) backward compat | valid path | Ok + durability_config() == Default | integration |
| open == builder new().open() | two valid paths | identical durability_config | integration |
| commit zero hash (Paranoid) | ZeroHashKey | Err(CommitError::ZeroHashKey { .. }) | integration |
| commit empty key (Paranoid) | EmptyStringKey | Err(CommitError::EmptyStringKey { .. }) | integration |
| commit duplicate key (Paranoid) | DuplicateStateKey | Err(CommitError::DuplicateStateKey { .. }) | integration |
| commit oversized (Paranoid) | PayloadTooLarge | Err(CommitError::PayloadTooLarge { .. }) | integration |
| commit missing ref (Paranoid) | MissingReference | Err(CommitError::MissingReference { .. }) | integration |
| commit with active session (Paranoid) | active session | Err(CommitError::WriteTransaction { .. }) | integration |
| rollback on failure (Paranoid) | partial valid + invalid | Err + no writes visible | integration |
| session drop enables commit (Paranoid) | session dropped then commit | Ok(()) | integration |

### E2E test

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| Paranoid commit survives reopen | write → close → reopen | data intact with exact bytes | e2e |

### Proptest

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| Copy semantics | any DurabilityConfig | a == b after copy | proptest |
| Builder last-write-wins | any (usize, DurabilityConfig, usize) | last cache_size wins | proptest |
| Unchanged rows skipped (Paranoid) | any identical bytes | data unchanged | proptest |
| Data integrity after reopen (Paranoid) | any valid StateChanges | stored == original | proptest |
| Independent databases | two paths, both configs | no cross-contamination | proptest |

---

## Open Questions

1. **How to verify `set_two_phase_commit(true)` was actually called?** — redb's `WriteTransaction` doesn't expose this state. The observable behavior is identical to Default mode under normal operation. Verification relies on: (a) code review of the conditional, (b) mutation testing catching the inverted branch, (c) the fact that redb's two-phase commit produces an extra fsync which is only observable during crash recovery. If stronger verification is needed, consider a seam/dependency inversion in a future bead.

2. **TableInit error variant coverage** — Hard to trigger with redb 2.x on a healthy system. Same gap exists in existing tests. Documented, not blocked.

3. **`cache_size` observability** — redb doesn't expose the effective cache size after construction. We verify the builder stores the value correctly via unit tests but cannot verify redb actually applied it at runtime. This is acceptable: redb's own tests cover `Builder::set_cache_size`.
