# Test Plan Review: cdocs-0tv

**Reviewer**: Test Inquisitor (Mode 1 — Plan Inquisition)
**Date**: 2026-04-02 (re-review)
**Contract**: `contract.md` (234 lines)
**Test Plan**: `test-plan.md` (631 lines, 24 behaviors)
**Prior Review**: 6 MAJOR, 5 MINOR — all flagged for fix
**Verdict**: **APPROVED**

---

## Prior Finding Disposition

Every finding from the previous review has been addressed. Line-by-line confirmation:

| Prior ID | Finding | Fix | Evidence |
|----------|---------|-----|----------|
| M-1 | `TableOpenFailed` missing from commit write path | B19 added | test-plan.md line 36, lines 339–349: full BDD with `Err(StateError::TableOpenFailed { table: "snapshots", message })` |
| M-2 | `StorageError` missing from commit write path | B20 added | test-plan.md line 37, lines 353–365: full BDD with `Err(StateError::StorageError { operation, message })` |
| M-3 | B01 Then: `bytes.len() > 0` weak assertion | Removed | test-plan.md line 91: now asserts `rkyv::access` succeeds + archived `target_url` equals input + round-trip equality. No `> 0` anywhere. |
| M-4 | B15 Then: "returns an error" without variant | Specified | test-plan.md line 291: `Err(StateError::CommitFailed { message })` where `message` is non-empty. |
| M-5 | Trophy allocation internally inconsistent | Reconciled | test-plan.md line 6: "2 unit / 22 integration / 0 E2E". Line 74: "2 unit (8%), 22 integration (92%)". Trophy table: B01+B02 unit, B03–B24 integration. All three agree. |
| M-6 | No negative test for read-session-still-active | B21 added | test-plan.md line 38, lines 369–381: full BDD with `Err(StateError::WriteTransactionFailed { message })` + state-unchanged verification + retry-after-drop succeeds. |
| m-1 | `load_snapshots` no 10K+ boundary | B23 added | test-plan.md lines 399–411: `map.len() == 10000`, spot-check 10 entries. |
| m-2 | Empty changes no formal BDD | B22 added | test-plan.md lines 385–396: full Given/When/Then, verifies K_EXISTING unchanged. |
| m-3 | `commit_changes` no 10K+ boundary | B24 added | test-plan.md lines 415–427: `map.len() == 10000`, spot-check 10 entries. |
| m-4 | Write-path `StorageError` swap uncaught | B20 added | Mutation table line 560: "remove StorageError mapping → B20 catches". |
| m-5 | Write-path `TableOpenFailed` swap uncaught | B19 added | Mutation table line 559: "remove TableOpenFailed mapping → B19 catches". |

**All 11 prior findings: RESOLVED.**

---

## Axis 1 — Contract Parity

### Functions (3 pub fn)

| Function | BDD Scenarios | Status |
|----------|---------------|--------|
| `serialize_snapshot` | B01, B02 | COVERED |
| `load_snapshots` | B03, B04, B05, B06, B07, B08, B09, B10, B18, B23 | COVERED |
| `commit_changes` (snapshot paths) | B11, B12, B13, B14, B15, B16, B17, B19, B20, B21, B22, B24 | COVERED |

All 3 pub fns have ≥1 BDD scenario. PASS.

### Error Variants (10 in enum)

| Variant | Produced By | Scenario(s) | Exact Assertion | Status |
|---------|-------------|-------------|-----------------|--------|
| `DatabaseOpenFailed` | `StateDb::open` (pre-existing) | — | — | EXEMPT |
| `ReadTransactionFailed` | `begin_read` (pre-existing) | — | — | EXEMPT |
| `WriteTransactionFailed` | `commit_changes` | B16, B21 | `Err(StateError::WriteTransactionFailed { message })` with non-empty message | PASS |
| `TableOpenFailed` | `load_snapshots`, `commit_changes` | B07 (read), B19 (write) | `{ table: "snapshots", message }` with exact table name + non-empty message | PASS |
| `StorageError` | `load_snapshots`, `commit_changes` | B10 (read), B20 (write) | `{ operation: "...", message }` with concrete operation string + non-empty message | PASS |
| `SerializationFailed` | `serialize_snapshot` | B02 | `{ message }` with non-empty message | PASS |
| `DeserializationFailed` | `load_snapshots` | B09 | `{ key_hex, message }` with hex-encoded key match + non-empty message | PASS |
| `ArchiveValidationFailed` | `load_snapshots` | B08 | `{ key_hex, message }` with hex-encoded key match + non-empty message | PASS |
| `CommitFailed` | `commit_changes` | B15, B17 | `{ message }` with non-empty message | PASS |
| `Io` | `StateDb::open` (pre-existing) | — | — | EXEMPT |

All 7 producible variants have scenarios asserting the exact variant. 3 pre-existing variants (`DatabaseOpenFailed`, `ReadTransactionFailed`, `Io`) are correctly exempted — they belong to `StateDb::open` and `begin_read`, which are out of scope for this bead. PASS.

---

## Axis 2 — Assertion Sharpness

Every "Then:" clause in all 24 BDD scenarios was inspected. Summary:

| Scenario | Then: Form | Sharp? |
|----------|-----------|--------|
| B01 | `Ok(bytes)` + `rkyv::access` succeeds + archived `target_url` == input + round-trip equality | YES |
| B02 | `Err(StateError::SerializationFailed { message })` non-empty | YES |
| B03 | `Ok(map)` + `map.len() == 1` + `contains_key(&K1)` + deserialize equals original | YES |
| B04 | `Ok(map)` + `map.is_empty()` is true | YES |
| B05 | `Ok(map)` + `map.is_empty()` is true + zero table access | YES |
| B06 | `archived()` valid + `deserialize()` equals original (after txn dropped) | YES |
| B07 | `Err(StateError::TableOpenFailed { table: "snapshots", message })` table exact + message non-empty | YES |
| B08 | `Err(StateError::ArchiveValidationFailed { key_hex, message })` key_hex matches + message non-empty | YES |
| B09 | `Err(StateError::DeserializationFailed { key_hex, message })` key_hex matches + message non-empty | YES |
| B10 | `Err(StateError::StorageError { operation: "load_snapshots", message })` operation exact + message non-empty | YES |
| B11 | `Ok(())` + new session load verifies persisted value | YES |
| B12 | `Ok(())` + new session load verifies key absent | YES |
| B13 | `Ok(())` + new session load verifies key absent (delete wins) | YES |
| B14 | `Ok(())` + new session load verifies bytes_v2 value (last wins) | YES |
| B15 | `Err(StateError::CommitFailed { message })` + K_EXISTING unchanged + K_NEW absent | YES |
| B16 | `Err(StateError::WriteTransactionFailed { message })` non-empty | YES |
| B17 | `Err(StateError::CommitFailed { message })` non-empty | YES |
| B18 | `Ok(map)` + `map.len() == 2` + contains_key K1, K2 true, K3 false | YES |
| B19 | `Err(StateError::TableOpenFailed { table: "snapshots", message })` table exact + message non-empty | YES |
| B20 | `Err(StateError::StorageError { operation, message })` operation matches commit path + message non-empty | YES |
| B21 | `Err(StateError::WriteTransactionFailed { message })` + state unchanged + retry after drop succeeds | YES |
| B22 | `Ok(())` + K_EXISTING unchanged (load verifies len==1, deserialize matches) | YES |
| B23 | `Ok(map)` + `map.len() == 10000` + spot-check 10 entries | YES |
| B24 | `Ok(())` + new session `map.len() == 10000` + spot-check 10 entries | YES |

No `is_ok()`, `is_err()`, bare `> 0`, or `Some(_)` assertions found. Every Then clause specifies a concrete value or exact error variant with field-level checks. PASS.

---

## Axis 3 — Trophy Allocation

### Density

| Metric | Count |
|--------|-------|
| BDD scenarios | 24 |
| Proptest invariants | 3 |
| Fuzz targets | 2 |
| Kani harnesses | 1 |
| **Total test functions** | **30** |
| Public functions | 3 |
| **Ratio** | **30 / 3 = 10.0×** |

Target: ≥5×. Achieved: 10.0×. PASS.

### Trophy Consistency Check

Three locations must agree:

1. **Summary** (line 6): "2 unit / 22 integration / 0 E2E / 0 static"
2. **Ratios** (line 74): "2 unit (8%), 22 integration (92%), 0 static (0%), 0 E2E (0%)"
3. **Trophy table** (lines 47–72): B01+B02 = unit, B03–B24 = integration → 2 unit / 22 integration

All three agree. PASS. (Prior M-5 resolved.)

### Proptest Coverage

- `serialize_snapshot` is a pure function with non-trivial input space (variable-length strings, BTreeMap, timestamps). Proptest 1 (round-trip), Proptest 2 (determinism) cover it. PASS.
- `load_snapshots` round-trip through commit+load: Proptest 3 covers it with 0–20 entries. PASS.

### Fuzz Coverage

- `load_snapshots` deserializes untrusted bytes from redb. Fuzz Target 1 (corrupt bytes) with 7 corpus seeds. PASS.
- `serialize_snapshot` is a serializer taking a typed `&Snapshot`. Fuzz Target 2 validates the assumption that rkyv serialization is infallible for valid types. PASS.

### Integration/Unit Ratio

92% integration, 8% unit. Justification is sound: this is a data layer backed by redb. The only pure function (`serialize_snapshot`) is tested at unit level. All `load_snapshots` and `commit_changes` behaviors require a real redb database to validate. No CLI surface → no E2E. PASS.

---

## Axis 4 — Boundary Completeness

### `serialize_snapshot`

| Boundary | Covered | Where |
|----------|---------|-------|
| Minimum valid (empty pages) | YES | Combinatorial table line 573 |
| Maximum valid (100+ pages) | YES | Combinatorial table line 574 |
| Empty target_url | YES | Proptest anti-invariant line 442 |
| Overflow / panic | YES | Kani harness line 527 |
| Pathological failure | YES | B02 |
| Non-ASCII target_url | YES | Fuzz Target 2 corpus seed |
| Zero timestamp | YES | Fuzz Target 2 corpus seed |

PASS — all boundaries explicitly named.

### `load_snapshots`

| Boundary | Covered | Where |
|----------|---------|-------|
| Empty hashes `&[]` | YES | B05 |
| Single hash | YES | B03 |
| Multiple hashes (partial match) | YES | B18 |
| All-zeros key `[0u8; 32]` | YES | Combinatorial table line 588 |
| All-0xFF key `[0xFF; 32]` | YES | Combinatorial table line 589 |
| 10,000+ hashes (scale) | YES | B23 |

PASS — all boundaries explicitly named.

### `commit_changes` (snapshot paths)

| Boundary | Covered | Where |
|----------|---------|-------|
| Empty changes (both fields empty) | YES | B22 |
| Single insert | YES | B11 |
| Single delete | YES | B12 |
| Delete + insert same key | YES | B13 |
| Duplicate keys in new_snapshots | YES | B14 |
| 10,000+ new entries (scale) | YES | B24 |
| Read session still active (negative) | YES | B21 |

PASS — all boundaries explicitly named.

---

## Axis 5 — Mutation Survivability

The plan lists 19 mutation checkpoints in Section 7 (lines 542–562). Each was cross-referenced against a killing test:

| # | Mutation Target | Killed By | Assessment |
|---|----------------|-----------|------------|
| 1 | `serialize_snapshot` returns empty vec | B01 (rkyv::access + field equality) | CAUGHT |
| 2 | Swap `SerializationFailed` ↔ `StorageError` | B02 (exact variant) | CAUGHT |
| 3 | Remove early-return for empty hashes | B05 (empty input must not touch table) | CAUGHT |
| 4 | Return error for missing key instead of omit | B04 (expects Ok(empty)) | CAUGHT |
| 5 | Return borrowed bytes instead of owned | B06 (access after txn drop) | CAUGHT |
| 6 | Swap `ArchiveValidationFailed` → `DeserializationFailed` | B08 (exact variant) | CAUGHT |
| 7 | Swap `DeserializationFailed` → `ArchiveValidationFailed` | B09 (exact variant) | CAUGHT |
| 8 | Skip one hash in iteration | B03 (count == 1), B18 (count == 2) | CAUGHT |
| 9 | Skip `key_hex` formatting in error | B08, B09 (key_hex field check) | CAUGHT |
| 10 | Swap insert/delete order (insert after delete) | B13 (delete must win) | CAUGHT |
| 11 | First-wins instead of last-wins | B14 (last entry asserted) | CAUGHT |
| 12 | Skip delete entirely | B12 (key must be absent) | CAUGHT |
| 13 | Skip insert entirely | B11 (key must be present) | CAUGHT |
| 14 | Return `Ok(())` early before commit | B15 (state unchanged) + B11 | CAUGHT |
| 15 | Swap `CommitFailed` ↔ `WriteTransactionFailed` | B17 (exact variant) | CAUGHT |
| 16 | Remove `TableOpenFailed` mapping for write path | B19 (exact variant) | CAUGHT |
| 17 | Remove `StorageError` mapping for write path | B20 (exact variant) | CAUGHT |
| 18 | Skip read-session-active check | B21 (must fail when session active) | CAUGHT |
| 19 | Mutate table when changes are empty | B22 (existing state unchanged) | CAUGHT |

**Kill rate: 19/19 = 100%.** Exceeds 90% target. PASS.

Additional thought-experiment mutations beyond the plan's explicit list:

| Unlisted Mutation | Caught By | Assessment |
|-------------------|-----------|------------|
| `load_snapshots` returns all requested keys with Default values | B03 (deserialize checks exact value) | CAUGHT |
| `load_snapshots` returns extra entries not requested | B03 (`map.len() == 1`) | CAUGHT |
| `commit_changes` writes corrupt/truncated bytes | B11 (load back + deserialize verification) | CAUGHT |
| `commit_changes` applies only inserts, silently ignores deletes | B12 (key must be absent), B13 (delete wins) | CAUGHT |
| `commit_changes` applies only deletes, silently ignores inserts | B11 (key must be present) | CAUGHT |
| `serialize_snapshot` non-deterministic output | Proptest 2 (identical bytes on repeat call) | CAUGHT |
| `load_snapshots` panics on corrupt bytes | Fuzz Target 1 (must return Err, not panic) | CAUGHT |

No uncaught mutations identified. PASS.

---

## Axis 6 — Holzmann Plan Audit

| Rule | Assessment | Finding |
|------|-----------|---------|
| Rule 1 — Linear | Every BDD scenario is Given→When→Then, no nesting | PASS |
| Rule 2 — Bounded loops | No loops in any scenario body | PASS |
| Rule 3 — Resource ownership | Each integration test creates own StateDb (tempdir-backed). B06 explicitly drops session. | PASS |
| Rule 4 — One job per test | Each scenario tests one behavior. B15 tests ACID + verifies two keys — borderline but coherent (both keys prove the same ACID property). | PASS |
| Rule 5 — Explicit preconditions | Every scenario has a Given block with explicit state description. No implied setup. | PASS |
| Rule 6 — No swallowed errors | All scenarios assert on return values. No `let _ =` or `.ok()` patterns in plan. | PASS |
| Rule 7 — No shared state | Each scenario creates its own StateDb instance. No shared mutable state. | PASS |
| Rule 8 — Named side effects | B03: "write it to the snapshots table". B07: "delete_table". B15: "corrupting the database handle". Side effects visible in names. | PASS |
| Rule 9 — One layer of magic | Scenarios describe direct API interactions. No deep fixture chains. | PASS |

Holzmann audit: **PASS** — no violations.

---

## Summary of Findings

### LETHAL (0)

None.

### MAJOR (0)

None. All 6 prior MAJOR findings resolved.

### MINOR (1)

| ID | Location | Finding |
|----|----------|--------|
| m-1 | test-plan.md lines 409, 425 | B23 and B24 Then clauses say "A random sample of 10 entries." The word "random" implies non-deterministic test behavior (e.g., `rand::thread_rng()`), which violates the Holzmann spirit of reproducibility. At implementation time, the test writer should use **deterministic indices** — e.g., first (0), last (9999), and 8 evenly-spaced indices (999, 1999, …, 8999). This ensures the test is reproducible and always checks the same entries. |

**Advisory** (not counted): B20 (line 363) specifies `operation` matching "the commit-path operation string (e.g., `"commit_snapshot_insert"`)". The exact string is not pinned in the plan because it depends on the implementation's choice of operation name. This is an inherited contract gap — the contract itself does not specify the exact operation string for the write path. The test writer must hardcode the concrete string at implementation time rather than reading it from the implementation (which would make the assertion tautological).

---

## Severity Tally

- **LETHAL**: 0
- **MAJOR**: 0 (threshold ≥3 → pass)
- **MINOR**: 1 (threshold ≥5 → pass)

## VERDICT: **APPROVED**

0 LETHAL + 0 MAJOR + 1 MINOR = APPROVED.

All prior findings resolved. The plan covers all 3 pub fns with ≥1 BDD scenario each, all 7 producible error variants with exact-variant assertions, concrete Then clauses throughout, 10× test density, proptest for pure functions, fuzz for deserializers, all boundaries explicitly named, 100% mutation kill rate on listed checkpoints, and clean Holzmann audit.

---

## Advisory for Implementation

1. **B23/B24**: Replace "random sample" with deterministic index selection. Recommended: `[0, 999, 1999, 2999, 3999, 4999, 5999, 6999, 7999, 8999, 9999]` (11 indices evenly spaced across 10K entries).

2. **B02**: If `rkyv::to_bytes` is infallible for all valid `Snapshot` inputs, document `SerializationFailed` as "not practically reachable — covered by type system" rather than fabricating an impossible test scenario.

3. **B10, B17, B20**: Each has notes acknowledging potential difficulty in deterministically triggering the error path. If no deterministic trigger exists, document the error mapping code as correct but the path unreachable. Do NOT skip writing the test — write it with the concrete assertion and document the trigger limitation.

4. **B20 `operation` field**: Pin the exact operation string at implementation time. Do NOT read it from the implementation code in the test (tautological). Assert a hardcoded string literal.

5. **B21 error variant**: Verify that redb returns a contention error that maps to `WriteTransactionFailed` when a read session is active. If redb returns a different error type, update the Then clause to match the actual variant.
