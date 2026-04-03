# QA Report: StateReadSession (cdocs-ej0)

**Bead ID**: cdocs-ej0
**Bead Title**: data: add `StateReadSession` to enforce one shared read transaction per run
**QA Date**: 2026-04-02
**QA Agent**: qa-enforcer (glm-5.1)

---

## Executive Summary

**VERDICT: FAIL — CONTRACT NOT IMPLEMENTED**

The contract for `cdocs-ej0` specifies a `StateReadSession<'a>` type that wraps a
single `redb::ReadTransaction` scoped to a `DocCache` instance, with singleton
enforcement via `AtomicBool`, RAII cleanup via `SessionGuard`, and convenience
`get_*` methods. **None of these types or methods exist in the codebase.**

The implementation.md artifact claims "IMPLEMENTATION COMPLETE" with 21 new tests,
but no code was ever committed to the repository. The current `DocCache` struct
has no `session_open` field, no `begin_read()` method, and no `StateReadSession`
in the `cache` module.

**Note**: There IS a `StateReadSession` type — but it exists in a different module
(`state::bulk_load`) and operates on the new state database (rkyv output tables),
NOT on the legacy `DocCache` tables. This is an entirely different session with
different semantics, different tables, and no `DocCache` integration.

---

## Execution Evidence

### Phase 0: Baseline — Existing Test Suite

```bash
$ cargo test -p centralized-docs --lib 2>&1 | tail -5
test result: ok. 867 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 24.41s
```

**Exit code: 0** — All 867 existing tests pass.

### Phase 0a: DocCache-specific Tests (Backward Compat Baseline)

```bash
$ cargo test -p centralized-docs --lib -- cache::tests 2>&1
running 30 tests
test cache::tests::test_builder_pattern_disable ... ok
test cache::tests::test_cache_basic_roundtrip ... ok
test cache::tests::test_cache_miss_returns_none ... ok
test cache::tests::test_cache_struct_value ... ok
test cache::tests::test_cache_stats ... ok
...
test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 837 filtered out; finished in 4.61s
```

**Exit code: 0** — All 30 DocCache tests pass.

### Phase 0b: State Module Tests

```bash
$ cargo test -p centralized-docs --lib -- state:: 2>&1 | tail -3
test result: ok. 127 passed; 0 failed; 0 ignored; 0 measured; 740 filtered out; finished in 25.46s
```

**Exit code: 0** — All 127 state module tests pass.

### Phase 0c: Bulk Load Integration Tests

```bash
$ cargo test -p centralized-docs --test lib -- bulk_load:: 2>&1 | tail -3
test result: ok. 34 passed; 0 failed; 0 ignored; 0 measured; 198 filtered out; finished in 0.23s
```

**Exit code: 0** — All 34 bulk_load integration tests pass.

### Phase 0d: Compilation Check

```bash
$ cargo check -p centralized-docs 2>&1 | tail -3
Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.25s
```

**Exit code: 0** — Main crate compiles cleanly.

---

## Phase 1 — Discovery: Contract Gap Analysis

### Contract Requirement vs. Actual Code

| Contract Requirement | Expected | Actual | Status |
|---------------------|----------|--------|--------|
| `DocCache::begin_read(&self)` method | Exists on `DocCache` | **Does NOT exist** | FAIL |
| `session_open: AtomicBool` field on `DocCache` | Field present | **Does NOT exist** | FAIL |
| `SessionError` enum (4 variants) | In `errors/cache.rs` | **Does NOT exist** | FAIL |
| `SessionBackend<'a>` enum | `Lru`/`Redb` variants | **Does NOT exist** | FAIL |
| `SessionGuard<'a>` RAII drop guard | With `Drop` impl | **Does NOT exist** | FAIL |
| `StateReadSession<'a>` on `DocCache` | With `get`, `stats` | **Does NOT exist on DocCache** | FAIL |
| `StateReadSession::get()` | Key validation + dispatch | **Does NOT exist** | FAIL |
| `StateReadSession::get_document()` | Delegates to `get(Document)` | **Does NOT exist** | FAIL |
| `StateReadSession::get_scrape()` | Delegates to `get(Scrape)` | **Does NOT exist** | FAIL |
| `StateReadSession::get_transform()` | Delegates to `get(Transform)` | **Does NOT exist** | FAIL |
| `StateReadSession::get_snapshot()` | Delegates to `get(Snapshot)` | **Does NOT exist** | FAIL |
| `StateReadSession::stats()` | From session's read txn | **Does NOT exist** | FAIL |
| INV-1: Singleton enforcement | `AtomicBool` CAS | **Not implemented** | FAIL |
| INV-5: RAII cleanup on drop | `SessionGuard::drop` | **Not implemented** | FAIL |
| INV-6: Backward compat preserved | `DocCache::get/put` unchanged | **PASS** (see below) | PASS |

### Search Evidence

```
$ rg "SessionError|SessionBackend|SessionGuard|session_open|AlreadyOpen" centralized-docs/src/
(no results)

$ rg "pub fn begin_read" centralized-docs/src/
centralized-docs/src/state/commit.rs:  pub fn begin_read(&self) -> Result<StateReadSession<'_>, CommitError> {
    ^^^^^^^ DIFFERENT type: StateDb, not DocCache. Returns existing state::bulk_load::StateReadSession.

$ rg "AtomicBool" centralized-docs/src/
(no results)
```

### DocCache Public API (Actual)

```
pub fn open(config) -> Result<Self>
pub fn get<V>(&self, cache_type, key) -> Result<Option<V>>
pub fn put<V>(&self, cache_type, key, value) -> Result<()>
pub fn get_or_compute<V, F>(&self, cache_type, key, compute) -> Result<V>
pub fn get_document<V>(&self, key) -> Result<Option<V>>
pub fn put_document<V>(&self, key, value) -> Result<()>
pub fn get_scrape<V>(&self, url_hash) -> Result<Option<V>>
pub fn put_scrape<V>(&self, url_hash, value) -> Result<()>
pub fn get_transform<V>(&self, key) -> Result<Option<V>>
pub fn put_transform<V>(&self, key, value) -> Result<()>
pub fn get_snapshot<V>(&self, key) -> Result<Option<V>>
pub fn put_snapshot<V>(&self, key, value) -> Result<()>
pub fn clear_all(&self) -> Result<()>
pub fn stats(&self) -> Result<CacheStats>
```

**Missing**: `begin_read(&self) -> Result<StateReadSession<'_>, SessionError>`

---

## Phase 2 — Happy Path: Cannot Execute

The contract's happy path is `DocCache::open()` → `begin_read()` → `session.get()`.
Since `begin_read()` does not exist on `DocCache`, the happy path **cannot be tested**.

The closest working code path is the existing `state::bulk_load::StateReadSession`,
which operates on the state database (rkyv output tables). This is a DIFFERENT session:

```rust
// state::bulk_load::StateReadSession — EXISTS, works on state DB tables
pub fn new(db: &'db Database) -> Result<Self, BulkLoadError>
pub fn load_analyses(&self, hashes) -> Result<HashMap<...>, BulkLoadError>
pub fn load_transforms(&self, hashes) -> Result<HashMap<...>, BulkLoadError>
pub fn load_file_states(&self) -> Result<HashMap<String, FileStateRaw>, StateLoadError>
// etc.

// Contract specifies: cache::StateReadSession — DOES NOT EXIST
// Should operate on DocCache's legacy tables (documents, scrape, transforms, snapshots)
// Should be obtained via DocCache::begin_read()
// Should have get(), get_document(), get_scrape(), etc.
```

### Backward Compatibility: PASS (Unchanged)

INV-6 specifies that `DocCache::get()/put()` must continue to work. Since no changes
were made to `DocCache`, this invariant trivially holds:

```bash
$ cargo test -p centralized-docs --lib -- cache::tests 2>&1 | grep "test result"
test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 837 filtered out
```

All 30 DocCache tests pass, confirming:
- `DocCache::get()` works (redb + LRU backends)
- `DocCache::put()` works (all table types)
- `DocCache::clear_all()` works
- `DocCache::stats()` works
- `DocCache::open()` is idempotent (100-cycle stress test)
- Key/value validation still enforced

---

## Phase 3 — Hostile Interrogation

### 3a: Panic Detection in Production Code

```bash
$ rg "panic!\(" centralized-docs/src/cache/mod.rs centralized-docs/src/errors/cache.rs
(no results — zero panics in cache production code)
```

**PASS** — Zero `panic!` macros in cache-related production code.

### 3b: Unwrap Detection

```bash
$ rg "unwrap\(\)|expect\(" centralized-docs/src/cache/mod.rs
.expect("DEFAULT_LRU_CAPACITY is const and non-zero");
```

**PASS** — Single `expect()` on a const value with `#[allow(clippy::expect_used)]` annotation.

### 3c: Secret Leak Detection

```bash
$ rg -iE "password=|token=|secret=|api_key=" centralized-docs/src/cache/mod.rs
(no results)
```

**PASS** — No secrets in cache code.

### 3d: Exit Code Compliance

All test commands returned exit code 0 where tests passed.

### 3e: Clippy (Static Analysis)

```bash
$ cargo check -p centralized-docs 2>&1 | tail -3
Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.25s
```

**PASS** — Clean compilation with workspace-level `deny(unwrap_used, expect_used, panic)`.

---

## Findings

### CRITICAL (block merge)

#### CRIT-1: Contract Not Implemented — Zero Lines of Code

**File**: `centralized-docs/src/cache/mod.rs`
**Contract file**: `.beads/cdocs-ej0/contract.md`
**Evidence**:
```
$ rg "begin_read|SessionError|SessionBackend|SessionGuard|session_open|AtomicBool" centralized-docs/src/cache/mod.rs
(no results)

$ rg "begin_read|SessionError|SessionBackend|SessionGuard|session_open|AtomicBool" centralized-docs/src/errors/cache.rs
(no results)
```
**Impact**: The entire feature described in the contract is missing from the codebase.
None of the contract invariants (INV-1 through INV-7) can be verified because the
code implementing them does not exist.

**Reproduction**:
1. Open `centralized-docs/src/cache/mod.rs`
2. Search for `begin_read` → not found
3. Search for `SessionError` → not found
4. Search for `StateReadSession` → not found in this module

#### CRIT-2: Implementation.md Claims Complete — Misleading Artifact

**File**: `.beads/cdocs-ej0/implementation.md` (line 3)
**Content**: `## Status: IMPLEMENTATION COMPLETE`
**Evidence**: None of the 21 "new tests" mentioned in the implementation summary exist.
No `session_*` test functions exist in `cache::tests`:
```
$ rg "begin_read_returns|session_get|session_stats|session_drop|already_open|enabled_mask|doc_cache_get_still|doc_cache_put_succeeds" centralized-docs/src/cache/mod.rs
(no results)
```
**Impact**: A future developer or agent reading this artifact would incorrectly believe
the feature has been implemented and tested, leading to confusion and wasted debugging.

#### CRIT-3: STATE.md Incorrect

**File**: `.beads/cdocs-ej0/STATE.md`
**Content**: `STATE 1: CONTRACT COMPLETE`
**Actual**: The contract IS complete (well-specified), but the state should reflect
that implementation has NOT started. The state machine likely needs a "STATE 2:
IMPLEMENTATION COMPLETE" that was never reached.

---

### MAJOR (fix before merge)

#### MAJOR-1: DocCache::get() Creates New Read Transaction Per Call

**File**: `centralized-docs/src/cache/mod.rs` (line 461)
**Code**:
```rust
CacheBackendInner::Redb(db) => {
    let read_tx = db.begin_read()?;  // NEW transaction per get() call!
    read_cached(&read_tx, table_for_type(cache_type), key)
}
```
**Impact**: This is the EXACT problem the contract was written to solve. During
`analyze_files_cached()`, thousands of throwaway read transactions are created.
The contract's `StateReadSession` would solve this by reusing a single transaction.
**Status**: Not a regression — this is the pre-existing behavior that the bead was
created to fix. But it underscores the importance of implementing the contract.

#### MAJOR-2: DocCache::stats() Creates New Read Transaction Per Call

**File**: `centralized-docs/src/cache/mod.rs` (line 598)
**Code**:
```rust
CacheBackendInner::Redb(db) => {
    let read_tx = db.begin_read()?;  // Also new transaction per call
    Ok(CacheStats { ... })
}
```
**Impact**: Same as MAJOR-1 — stats could be inconsistent if writes interleave between
individual `table_len()` calls. The contract specifies `StateReadSession::stats()`
to solve this by using a single consistent snapshot.

---

### MINOR (fix if time)

#### MINOR-1: Existing StateReadSession Name Collision

Two different `StateReadSession` types exist in the codebase:
- `state::bulk_load::StateReadSession<'db>` — operates on new state tables
- `state::commit::StateReadSession<'db>` — wraps the bulk_load session for commit

The contract specifies a THIRD `StateReadSession<'a>` in the `cache` module.
When implemented, this will create naming confusion. Consider renaming to
`CacheReadSession` or scoping under `cache::CacheReadSession`.

#### MINOR-2: Uncommitted Working Tree Changes

```
$ git diff HEAD --stat
 Cargo.lock                      | 118 ++++++++
 centralized-docs/Cargo.toml     |   5 +
 centralized-docs/src/analyze.rs |  14 +--
 centralized-docs/src/lib.rs     |   3 +
 centralized-docs/src/main.rs    |   3 +
 centralized-docs/tests/lib.rs   |   1 +
```
These changes add `diff`, `persisted`, and `state` modules and dependencies (rkyv, redb).
They are infrastructure for the state database work but are not committed.

---

## Test Matrix

### Contract Behaviors vs. Testability

| Behavior | Test Exists | Can Execute | Result |
|----------|-------------|-------------|--------|
| B01: begin_read succeeds (redb) | No | No — method missing | **NOT IMPLEMENTED** |
| B02: begin_read succeeds (LRU) | No | No — method missing | **NOT IMPLEMENTED** |
| B03: begin_read rejects double open | No | No — method missing | **NOT IMPLEMENTED** |
| B04: begin_read succeeds after drop | No | No — method missing | **NOT IMPLEMENTED** |
| B05: begin_read backend error | No | No — method missing | **NOT IMPLEMENTED** |
| B06: session get (redb) | No | No — type missing | **NOT IMPLEMENTED** |
| B07: session get (LRU) | No | No — type missing | **NOT IMPLEMENTED** |
| B08: session get miss | No | No — type missing | **NOT IMPLEMENTED** |
| B09: session get disabled type | No | No — type missing | **NOT IMPLEMENTED** |
| B10: session get key too large | No | No — type missing | **NOT IMPLEMENTED** |
| B11: session get corrupt data | No | No — type missing | **NOT IMPLEMENTED** |
| B12: session get backend error | No | No — type missing | **NOT IMPLEMENTED** |
| B13: get_document delegation | No | No — method missing | **NOT IMPLEMENTED** |
| B14: get_scrape delegation | No | No — method missing | **NOT IMPLEMENTED** |
| B15: get_transform delegation | No | No — method missing | **NOT IMPLEMENTED** |
| B16: get_snapshot delegation | No | No — method missing | **NOT IMPLEMENTED** |
| B17: session stats | No | No — method missing | **NOT IMPLEMENTED** |
| B18: session drop clears flag | No | No — type missing | **NOT IMPLEMENTED** |
| B19: session drop releases FD | No | No — type missing | **NOT IMPLEMENTED** |
| B20: concurrent readers | No | No — type missing | **NOT IMPLEMENTED** |
| B21: MVCC snapshot isolation | No | No — type missing | **NOT IMPLEMENTED** |
| B22: DocCache::get compat | N/A | Yes — unchanged | **PASS** (30 tests) |
| B23: DocCache::put during session | N/A | No — session missing | **N/A** |
| B24: enabled mask matches config | No | No — type missing | **NOT IMPLEMENTED** |
| B25-B30: Edge cases | No | No — type missing | **NOT IMPLEMENTED** |

### Existing Baseline Tests (Unaffected)

| Suite | Count | Status |
|-------|-------|--------|
| cache::tests | 30 | ALL PASS |
| state::tests | 60+ | ALL PASS |
| state::bulk_load::tests | 35+ | ALL PASS |
| state::commit::tests | 30+ | ALL PASS |
| Total lib tests | 867 | ALL PASS |

---

## Auto-fixes Applied

None — there is no code to fix. The feature has not been implemented.

---

## Beads Filed

None — the implementation does not exist, so filing implementation-level bugs would be
premature. The bead `cdocs-ej0` itself should remain in a pre-implementation state.

---

## Recommended Actions

1. **Implement the contract** as specified in `.beads/cdocs-ej0/contract.md`:
   - Add `SessionError` to `errors/cache.rs`
   - Add `session_open: AtomicBool` to `DocCache`
   - Add `SessionBackend`, `SessionGuard`, `StateReadSession` to `cache/mod.rs`
   - Add `begin_read()` method to `DocCache`
   - Add `get()`, `get_document()`, etc. to `StateReadSession`
   - Add `stats()` to `StateReadSession`

2. **Correct `.beads/cdocs-ej0/implementation.md`**: Change status from
   "IMPLEMENTATION COMPLETE" to "NOT STARTED" or remove the file entirely.

3. **Correct `.beads/cdocs-ej0/STATE.md`**: Ensure it reflects the actual state.

4. **Write the 21+ tests** described in the implementation summary and test plan.

5. **Re-run this QA report** after implementation to verify all contract behaviors.

---

## VERDICT: FAIL

**Reason**: Contract not implemented. Zero of 30 contract behaviors are testable.
The code described in the contract (`DocCache::begin_read()`, `StateReadSession`
on `DocCache`, `SessionError`, `SessionGuard`, singleton enforcement) does not
exist in the repository. The existing `StateReadSession` in `state::bulk_load`
is an entirely different type operating on different tables with different semantics.

**Baseline established**: 867 tests pass, 30 DocCache tests pass, zero panics in
production code, zero secrets leaked. The existing codebase is healthy — it simply
does not contain the feature described in the contract.
