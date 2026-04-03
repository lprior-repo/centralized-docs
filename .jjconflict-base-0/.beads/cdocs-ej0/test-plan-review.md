# Test Plan Review: StateReadSession (cdocs-ej0) — Re-Audit #2

## STATUS: APPROVED

---

## Audit Methodology

Mode 1 — Plan Inquisition. Full re-audit from Axis 1. No compilation, no execution.
Contract (`contract.md`) cross-referenced against revised test plan (`test-plan.md`) across six axes.
All 16 findings from the previous rejection verified against the revised plan. Fresh findings cataloged.

---

## Previous Defect Verification

All 16 findings from the previous rejection (test-plan-review.md v1) were checked:

| # | Previous ID | Description | Resolution | Status |
|---|-------------|-------------|------------|--------|
| 1 | L-TROPHY-1 | Density 4.625x (37/8) — below 5x | 6 new BDD scenarios (B25-B30) added. Total now 43/8 = 5.375x. | **FIXED** |
| 2 | M-SHARP-1 | B04 "new, distinct session" — no concrete assertion | B04 Now: clause now asserts `session.get() returns Ok(Some("survivor"))`. Concrete value. | **FIXED** |
| 3 | M-SHARP-2 | B19 "baseline" undefined | B19 now specifies: 10 cycles, fd_count_before/after captured via `/proc/self/fd`, delta == 0. | **FIXED** |
| 4 | M-BOUND-1 | `get()` missing 3 boundaries (empty key, >300 bytes, all-disabled) | B28 (empty key → Ok(None)), B29 (10000-byte key → KeyTooLarge), B30 (all-disabled → Ok(None)). | **FIXED** |
| 5 | M-TROPHY-2 | Trophy allocation numbers don't match behavior counts | Revised to 14 unit / 16 integration / 1 E2E. Verified: counts match listed behaviors. | **FIXED** |
| 6 | m-sharp-1 | B01 "can be called" — not an assertion | Removed. B01 now asserts: `session.get() returns Ok(None)` + `enabled mask all 6 bits set`. | **FIXED** |
| 7 | m-sharp-2 | B02 "reads from same LRU" — behavioral | B02 now asserts: both `DocCache::get()` and `session.get()` return `Ok(Some("lru_val"))`. | **FIXED** |
| 8 | m-sharp-3 | B09 "no I/O performed" — unverifiable | B09 now: `returns Ok(None) — even when data exists for that CacheType (verified in B24)`. | **FIXED** |
| 9 | m-sharp-4 | B18 "functionally independent" — vague | Removed. B18 now: `begin_read() succeeds on the next call, returning Ok(StateReadSession)`. | **FIXED** |
| 10 | m-bound-1 | No BDD for empty key (0 bytes) | B28 added: `session.get(CacheType::Document, b"") returns Ok(None)`. | **FIXED** |
| 11 | m-bound-2 | No BDD for 10000-byte key; P1 caps at 300 | B29 added; P1 strategy expanded to 0..10001. | **FIXED** |
| 12 | m-bound-3 | No BDD for all CacheTypes disabled | B30 added: queries all 4 types under empty enabled mask, all return Ok(None). | **FIXED** |
| 13 | m-bound-4 | No BDD for empty tables stats | B25 added: `CacheStats { all zeros }` with concrete field values. | **FIXED** |
| 14 | m-mut-1 | BackendError message assertion missing from B05 BDD | B05 Now includes: `And: message.len() > 0 — non-empty error description`. | **FIXED** |
| 15 | m-mut-2 | Field reorder weakly caught (B19 assertion too vague) | B19 now concrete: `fd_count_after == fd_count_before (delta == 0)`. | **FIXED** |
| 16 | m-holz-1 | B01/B05/B12 Givens unspecified | B01: `initialize_tables() has been called, creating DOCUMENT_TABLE, SCRAPE_TABLE, TRANSFORM_TABLE, and SNAPSHOT_TABLE`. B05/B12: `DocCache::test_invalidate_backend()` method specified inline. | **FIXED** |

**All 16 previous findings: RESOLVED.**

---

## Contract Function Inventory

Eight (8) public functions in `contract.md`:

| # | Function | Signature | Lines |
|---|----------|-----------|-------|
| 1 | `DocCache::begin_read` | `(&self) -> Result<StateReadSession<'_>, SessionError>` | contract.md:148 |
| 2 | `StateReadSession::get` | `(&self, CacheType, &[u8]) -> Result<Option<V>, SessionError>` | contract.md:170-174 |
| 3 | `StateReadSession::get_document` | `(&self, &[u8]) -> Result<Option<V>, SessionError>` | contract.md:192 |
| 4 | `StateReadSession::get_scrape` | `(&self, &[u8]) -> Result<Option<V>, SessionError>` | contract.md:200 |
| 5 | `StateReadSession::get_transform` | `(&self, &[u8]) -> Result<Option<V>, SessionError>` | contract.md:208 |
| 6 | `StateReadSession::get_snapshot` | `(&self, &[u8]) -> Result<Option<V>, SessionError>` | contract.md:216 |
| 7 | `StateReadSession::stats` | `(&self) -> Result<CacheStats, SessionError>` | contract.md:225 |
| 8 | `StateReadSession::drop` | `(implicit Drop trait impl)` | contract.md:238-241 |

Four (4) error variants in `SessionError` (contract.md:389-415):

| # | Variant | Fields |
|---|---------|--------|
| E1 | `AlreadyOpen` | (none) |
| E2 | `KeyTooLarge` | `{ size: usize, max: usize }` |
| E3 | `DeserializationError` | `{ message: String }` |
| E4 | `BackendError` | `{ operation: &'static str, message: String }` |

---

## Axis 1 — Contract Parity

**Verdict: PASS**

### Function → BDD Scenario Mapping

| Function | BDD Scenarios | Status |
|----------|---------------|--------|
| `begin_read` | B01, B02, B03, B04, B05 | PASS — 5 scenarios covering both backends, singleton violation, post-drop recovery, backend failure |
| `get` | B06, B07, B08, B09, B10 (×2), B11, B12, B28, B29, B30 | PASS — 11 tests covering both backends, missing key, disabled type, boundary, empty key, oversized key, all-disabled, corrupt data, backend failure |
| `get_document` | B13, B26 | PASS — happy path + missing key error path |
| `get_scrape` | B14, B27 | PASS — happy path + key-too-large through delegation |
| `get_transform` | B15 | PASS — happy path delegation verified |
| `get_snapshot` | B16 | PASS — happy path delegation verified |
| `stats` | B17, B25 | PASS — populated tables + empty tables |
| `drop` | B18, B19 | PASS — flag reset + fd leak detection |

Every `pub fn` has ≥1 BDD scenario. No missing functions.

### Error Variant → BDD Scenario Mapping

| Variant | Scenario(s) | Asserts Exact Variant? | Status |
|---------|-------------|------------------------|--------|
| `AlreadyOpen` | B03 | Yes: `matches!(err, SessionError::AlreadyOpen)` | PASS |
| `KeyTooLarge { size, max }` | B10, B27, B29 | Yes: `size == 257, max == 256` / `size == 10000, max == 256` | PASS |
| `DeserializationError { message }` | B11 | Yes: variant matched + `message.len() > 0` | PASS |
| `BackendError { operation, message }` | B05, B12 | Yes: `operation == "begin_read"` / `operation is &'static str` + `message.len() > 0` | PASS |

Every error variant has a scenario asserting the exact variant with field values. No `is_err()` or `is_ok()` anywhere in the plan. PASS.

---

## Axis 2 — Assertion Sharpness

**Verdict: PASS — 3 MINOR (below threshold)**

Every `Then:` clause inspected. All assertions are concrete values or exact variant matches.

### MINOR FINDINGS (3)

**m-sharp-1: B05 `message.len() > 0` (test-plan.md:146)**

```
And: message.len() > 0 — non-empty error description
```

`> 0` assertion on the `message` field. The exact message content is unknowable — it comes
from redb's internal error reporting. The variant (`BackendError`) and `operation` field
(`"begin_read"`) are concretely asserted. The `> 0` catches the relevant mutation (empty
message field). Acceptable for external-library error messages, but technically a `> 0`.

**m-sharp-2: B11 `message.len() > 0` (test-plan.md:231-232)**

```
And: message.len() > 0
```

Same pattern as m-sharp-1. `DeserializationError::message` content depends on `serde_json`'s
error formatting. Variant and non-empty check are the only verifiable properties.

**m-sharp-3: B12 `message.len() > 0` (test-plan.md:247-248)**

```
And: message.len() > 0
```

Same pattern as m-sharp-1. `BackendError::message` from redb I/O failure.

**Note on m-sharp-1/2/3**: These three `> 0` assertions are on error messages from external
libraries (redb, serde_json). The exact content is not deterministic from the test's
perspective. Each assertion catches the named mutation ("BackendError message field is
empty"). The variant AND other struct fields are concretely asserted. Flagged as MINOR per
strict rule interpretation, but they are justified in context.

### All Other Then: Clauses — Verified Concrete

| Scenario | Then: Value | Concrete? |
|----------|------------|-----------|
| B01 | `Ok(None)` + enabled mask all 6 bits set | ✅ |
| B02 | `Ok(Some("lru_val"))` from both paths | ✅ |
| B03 | `Err(SessionError::AlreadyOpen)` exact variant | ✅ |
| B04 | `Ok(StateReadSession)` + `Ok(Some("survivor"))` | ✅ |
| B06 | `Ok(Some("stored_value"))` | ✅ |
| B07 | `Ok(Some("scrape_result"))` | ✅ |
| B08 | `Ok(None)` | ✅ |
| B09 | `Ok(None)` | ✅ |
| B10 | `Err(KeyTooLarge { size: 257, max: 256 })` | ✅ |
| B10 boundary | `Ok(None)` or `Ok(Some(..))` for 256-byte key | ✅ |
| B13 | `Ok(Some("doc_val"))` | ✅ |
| B14 | `Ok(Some("scrape_val"))` | ✅ |
| B15 | `Ok(Some("transform_val"))` | ✅ |
| B16 | `Ok(Some("snap_val"))` | ✅ |
| B17 | `Ok(CacheStats { document_entries: 3, scrape_entries: 2, transform_entries: 1, snapshot_entries: 0, analysis_entries: 0, chunk_entries: 0 })` | ✅ |
| B18 | `begin_read() succeeds → Ok(StateReadSession)` | ✅ |
| B19 | `fd_count_after == fd_count_before (delta == 0)` | ✅ |
| B20 | `Ok(Some("original"))` | ✅ |
| B21 | `Ok(None)` | ✅ |
| B22 | `Ok(Some("compat_val"))` | ✅ |
| B23 | `Ok(())` + `Ok(Some("written_during_session"))` | ✅ |
| B24 | `Ok(Some("doc"))` + `Ok(None)` | ✅ |
| B25 | `Ok(CacheStats { all fields: 0 })` | ✅ |
| B26 | `Ok(None)` | ✅ |
| B27 | `Err(KeyTooLarge { size: 257, max: 256 })` | ✅ |
| B28 | `Ok(None)` | ✅ |
| B29 | `Err(KeyTooLarge { size: 10000, max: 256 })` | ✅ |
| B30 | `Ok(None)` × 4 types | ✅ |

No `is_ok()`, `is_err()`, or unqualified `Some(_)` found. All other assertions are concrete values.

---

## Axis 3 — Trophy Allocation

**Verdict: PASS**

### Density Audit

```
Public functions:             8
Total planned test count:    43
  - BDD named tests:         31  (B01-B30, B10 has 2 tests)
  - Proptest invariants:      5  (P1-P5)
  - Fuzz targets:             1  (F1)
  - Kani harnesses:           2  (K1-K2)
  - Static checks:            4
Ratio: 43 / 8 = 5.375x ≥ 5.0x     PASS
```

### Trophy Table Verification

| Layer | Claimed | Actual (counted from Behaviors column) | Match? |
|-------|---------|----------------------------------------|--------|
| Static | 4 | 4 (INV-4 checks, Send+Sync, non_exhaustive, drop order) | ✅ |
| Unit | 14 | B08(1), B09(1), B10(2), B13(1), B14(1), B15(1), B16(1), B25(1), B26(1), B27(1), B28(1), B29(1), B30(1) = 14 | ✅ |
| Integration | 16 | B01(1), B02(1), B03(1), B04(1), B05(1), B06(1), B07(1), B11(1), B12(1), B17(1), B18(1), B20(1), B21(1), B22(1), B23(1), B24(1) = 16 | ✅ |
| E2E | 1 | B19(1) = 1 | ✅ |

Numbers internally consistent. Summary line `43 tests / 8 fns = 5.375x` matches counted totals.

### Proptest and Fuzz Coverage

| Function | Non-trivial input space? | Proptest? | Fuzz? | Status |
|----------|--------------------------|-----------|-------|--------|
| `get` key validation | Yes | P1 (0..10001) | — | PASS |
| `get` deserialization | Yes | — | F1 (arbitrary bytes) | PASS |
| `get` round-trip | Yes | P2 (arbitrary V) | — | PASS |
| Enabled mask logic | Yes | P3 (u8 bitmask) | — | PASS |
| Open/drop cycle | Moderate | P4 (1..=50) | — | PASS |
| Snapshot consistency | Moderate | P5 (random key/value) | — | PASS |

No pure functions missing proptest. No parser/deserializer missing fuzz. PASS.

### Trophy Ratio Check

Unit: 14, Integration: 16, E2E: 1 → ~40% unit / ~46% integration / ~3% E2E.

The heavier integration weighting is justified: the core value of this feature is correct
interaction between `DocCache`, `SessionGuard`, `AtomicBool`, and redb's `ReadTransaction`.
Singleton enforcement and snapshot isolation are inherently cross-component.

---

## Axis 4 — Boundary Completeness

**Verdict: PASS**

### `DocCache::begin_read()` — All Boundaries Explicit

| Boundary | Scenario | Status |
|----------|----------|--------|
| No session active (redb) | B01 | ✅ |
| No session active (LRU) | B02 | ✅ |
| Session already active | B03 | ✅ |
| Session just dropped | B04 | ✅ |
| Backend failure | B05 | ✅ |
| Rapid open/drop/open cycle | P4 (1..50) | ✅ |
| Empty DocCache (no data) | Implicit in B01 | ✅ |

### `StateReadSession::get()` — All Boundaries Explicit

| Boundary | Scenario | Status |
|----------|----------|--------|
| Key exists, type enabled (redb) | B06 | ✅ |
| Key exists, type enabled (LRU) | B07 | ✅ |
| Key missing | B08 | ✅ |
| Type disabled | B09 | ✅ |
| Key exactly 256 bytes | B10 boundary | ✅ |
| Key 257 bytes (one-above-max) | B10 | ✅ |
| Key 0 bytes (empty) | B28 | ✅ |
| Key 10000 bytes (far overflow) | B29 | ✅ |
| All CacheTypes disabled | B30 | ✅ |
| Corrupt stored bytes | B11 | ✅ |
| Backend I/O failure | B12 | ✅ |
| Concurrent write (snapshot) | B20 | ✅ |
| Write after session open | B21 | ✅ |

All 6 standard boundary classes (min valid, max valid, one-below-min, one-above-max, empty/zero, overflow) explicitly named. PASS.

### `StateReadSession::stats()` — All Boundaries Explicit

| Boundary | Scenario | Status |
|----------|----------|--------|
| Populated tables (mixed counts) | B17 | ✅ |
| Empty tables (all zeros) | B25 | ✅ |
| Consistent snapshot | B17 + B21 | ✅ |

### Convenience Methods — All Boundaries Explicit

| Method | Happy Path | Error Path | Status |
|--------|------------|------------|--------|
| `get_document` | B13 | B26 (missing key) | ✅ |
| `get_scrape` | B14 | B27 (key too large) | ✅ |
| `get_transform` | B15 | Covered via delegation to `get()` | ✅ |
| `get_snapshot` | B16 | Covered via delegation to `get()` | ✅ |

Delegation methods `get_transform` and `get_snapshot` have only happy-path BDD tests. Their
error paths delegate to `get()`, which is comprehensively tested (B08-B12, B28-B30). The
delegation mechanism is proven by B26 (missing key through `get_document`) and B27 (key
validation through `get_scrape`). Additional error-path tests on each convenience method
would test the language, not the logic. PASS.

---

## Axis 5 — Mutation Survivability (Thought Experiment)

**Verdict: PASS**

All 15 mutations from the plan's table (test-plan.md:629-645) traced to catching tests:

| # | Mutation | Catching Test | Survives? |
|---|----------|---------------|-----------|
| 1 | AtomicBool store → no-op | B03 — second begin_read incorrectly succeeds | No ✅ |
| 2 | `store(false)` → `store(true)` | B04 — next begin_read fails | No ✅ |
| 3 | session_open check removed | B03 — returns Ok instead of Err | No ✅ |
| 4 | `>` → `>=` in key check | B10 boundary (256-byte key) | No ✅ |
| 5 | `>` offset by +1 | B10 (257-byte key) | No ✅ |
| 6 | Disabled type proceeds with I/O | B24 — returns data for disabled type | No ✅ |
| 7 | Enabled mask not copied | B24 — disabled type returns data | No ✅ |
| 8 | Convenience method wrong CacheType | B13-B16 — wrong table queried | No ✅ |
| 9 | SessionGuard drop removed | B18 — flag never clears | No ✅ |
| 10 | `_guard` field reordered | B19 — fd leak detected (now concrete delta==0) | No ✅ |
| 11 | Stats creates new read_tx | B17 + B21 — stats sees newer data than session.get() | No ✅ |
| 12 | BackendError message empty | B05 — message.len() > 0 assertion | No ✅ |
| 13 | Key validation skipped in convenience | B27 — 257-byte key passes through | No ✅ |
| 14 | Stats hardcoded/1 for empty | B25 — returns non-zero | No ✅ |
| 15 | get returns Err(KeyTooLarge) for 0-length key | B28 — returns Err instead of Ok(None) | No ✅ |

### Additional Mutations Checked (not in plan's table)

| Mutation | Catching Test | Survives? |
|----------|---------------|-----------|
| `get()` always returns `Ok(None)` | B06, B07 — assert `Some("exact_value")` | No ✅ |
| `begin_read()` returns session with wrong enabled mask | B24 — disabled type returns data | No ✅ |
| `get()` for disabled type returns `Err(...)` | B09, B30 — assert `Ok(None)` | No ✅ |
| `stats()` double-counts entries | B17 — exact count expected | No ✅ |
| `get()` ignores CacheType parameter | B06 vs B07 — different tables, different values | No ✅ |

No uncaught mutations identified. PASS.

---

## Axis 6 — Holzmann Plan Audit

**Verdict: PASS**

| Rule | Status | Evidence |
|------|--------|----------|
| Rule 1 — Keep it Linear | PASS | All BDD scenarios follow Given/When/Then. No nested conditionals. |
| Rule 2 — Bound Every Loop | PASS | B19 loop bounded at 10 iterations. Proptests use bounded strategies. |
| Rule 3 — Know What You Own | PASS | Side effects explicit: B11 "injected via raw redb write transaction", B19 "/proc/self/fd". |
| Rule 4 — One Function, One Job | PASS | Each scenario tests one behavior. Names describe the one thing proven. |
| Rule 5 — State Your Assumptions | PASS | All Givens specify setup procedures inline. B01 names `initialize_tables()` + 4 tables. B05/B12 name `test_invalidate_backend()`. |
| Rule 6 — Never Swallow Errors | PASS | No `let _ =` or `.ok()` in plan. All error paths asserted. |
| Rule 7 — Narrow Your State | PASS | Per-test DocCache instances. No shared mutable state. |
| Rule 8 — Surface Your Side Effects | PASS | B11: "bypassing public API" is explicit. B19: fd counting via /proc. |
| Rule 9 — One Layer of Magic | PASS | No deep helper chains. Each scenario is self-contained. |
| Rule 10 — Warnings Are Errors | PASS | Static analysis section includes `clippy::all` with deny. |

### Advisory for Mode 2 (Implementation Phase)

**B19 loop in test body**: The plan specifies "a StateReadSession is created and dropped
10 times in a loop." When implemented, this will be a loop in a test body, which is
**LETHAL** under Holzmann Rule 2 (Mode 2, Tier 0). The implementation MUST either:

1. Unroll the 10 iterations explicitly (verbose but compliant), or
2. Extract the create/drop cycle into a proptest with N in 1..=10, or
3. Use `rstest` parameterized cases for different cycle counts

The plan is acceptable because it specifies a bounded ceiling (10). The implementation
must not contain a raw `for` or `while` loop inside a `#[test]` function.

---

## Finding Summary

### LETHAL FINDINGS (0)

None.

### MAJOR FINDINGS (0)

None.

### MINOR FINDINGS (3)

| ID | File:Line | Description |
|----|-----------|-------------|
| m-sharp-1 | test-plan.md:146 | B05 `message.len() > 0` on BackendError — `> 0` assertion. Justified: external library error message, variant + operation concretely asserted, mutation caught. |
| m-sharp-2 | test-plan.md:231-232 | B11 `message.len() > 0` on DeserializationError — `> 0` assertion. Justified: serde_json error content non-deterministic, variant concretely asserted. |
| m-sharp-3 | test-plan.md:247-248 | B12 `message.len() > 0` on BackendError — `> 0` assertion. Justified: same rationale as m-sharp-1. |

---

## Severity Assessment

| Severity | Count | Threshold | Triggers REJECTED? |
|----------|-------|-----------|---------------------|
| LETHAL | 0 | ≥ 1 | No |
| MAJOR | 0 | ≥ 3 | No |
| MINOR | 3 | ≥ 5 | No |

**Verdict formula:** 0 LETHAL + < 3 MAJOR + < 5 MINOR = **APPROVED**.

---

## VERDICT

```
## VERDICT: APPROVED

### Axis 1 — Contract Parity
[PASS] All 8 pub fns have ≥1 BDD scenario
[PASS] All 4 Error variants have exact-variant assertions with field values
[PASS] No is_ok() or is_err() assertions found

### Axis 2 — Assertion Sharpness
[PASS] 30/33 Then: clauses use concrete values or exact variant matches
[MINOR] 3 instances of message.len() > 0 (B05, B11, B12) — justified for external library errors

### Axis 3 — Trophy Allocation
[PASS] Density: 43 tests / 8 fns = 5.375x (≥5x target)
[PASS] Trophy numbers internally consistent (14 unit / 16 integration / 1 E2E / 4 static)
[PASS] All pure functions with non-trivial input space have proptest (P1-P5)
[PASS] Deserializer has fuzz target (F1)
[PASS] AtomicBool ordering has Kani harness (K1)
[PASS] EnabledTypes bitmask has Kani harness (K2)

### Axis 4 — Boundary Completeness
[PASS] begin_read: 6/6 boundaries explicit (both backends, active session, dropped, failure, cycling)
[PASS] get: 13/13 boundaries explicit (both backends, missing, disabled, 0/256/257/10000 key, all-disabled, corrupt, failure, concurrent)
[PASS] stats: 3/3 boundaries explicit (populated, empty, snapshot-consistent)
[PASS] Convenience methods: happy + error paths covered via B13-B16, B26-B27

### Axis 5 — Mutation Survivability
[PASS] All 15 planned mutations have named catching tests
[PASS] All 5 additional mutations checked have catching tests
[PASS] No surviving mutants identified

### Axis 6 — Holzmann Rules
[PASS] Rules 1-10: no violations
[ADVISORY] B19 loop must be unrolled or extracted for Mode 2 compliance (Holzmann Rule 2)

### LETHAL: 0
### MAJOR: 0
### MINOR: 3 (all justified, below 5 threshold)
### STATUS: APPROVED
```

---

## Implementation Advisories

These are not blocking findings but MUST be addressed during Mode 2:

1. **B19 loop unrolling**: Replace the planned "10 times in a loop" with 10 explicit
   create/drop statements, or restructure as a proptest. A `for` loop inside a `#[test]`
   is LETHAL under Mode 2 Tier 0.

2. **`test_invalidate_backend()` scope**: B05 and B12 depend on a test-only method. This
   should be gated behind `#[cfg(test)]` and documented as a test-only escape hatch. The
   method must NOT appear in production builds.

3. **B19 platform gate**: The fd-count test is Linux-specific (`/proc/self/fd`). The
   `#[cfg(target_os = "linux")]` gate must be present in implementation. Non-Linux CI
   runners will skip this test — ensure this is documented in CI configuration.

4. **Proptest P1 strategy**: Range 0..10001 generates keys up to 10KB. Ensure the
   proptest is not run with a very high `max_shrink_iters` that causes timeouts on large
   key generation.
