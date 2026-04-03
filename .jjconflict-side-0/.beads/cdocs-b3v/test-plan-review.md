# Test Plan Review: `cdocs-b3v` — Raw State Bulk Loaders

## VERDICT: APPROVED

**0 LETHAL · 0 MAJOR · 0 MINOR — All axes pass. All previous defects confirmed fixed.**

---

## Previous Defect Resolution

All 12 findings from the prior rejection have been verified as resolved:

| Prev ID  | Severity | Description                                                       | Verified Fix                                                                                       |
|----------|----------|-------------------------------------------------------------------|----------------------------------------------------------------------------------------------------|
| LETHAL-1 | LETHAL   | `load_url_states` has no `BackendError` scenario                  | B24 added (L493-505): exact `operation=="open_table"` + exact message capture                     |
| MAJOR-1  | MAJOR    | `Utf8KeyError` `bytes_lossy` asserts vague "contains"/"non-empty" | B10 L270 + B23 L487: exact `String::from_utf8_lossy(&bytes)` equality                             |
| MAJOR-2  | MAJOR    | `BackendError` `message` asserts only "non-empty"                 | B11 L286-288 + B24 L501-503: exact redb error message capture + equality                          |
| MAJOR-3  | MAJOR    | URL abort test uses `{ .. }` wildcard                             | B22 L468-472: `key`, `actual==20`, `expected==40` — all 3 fields asserted                         |
| MAJOR-4  | MAJOR    | `load_url_states` missing 0/39/41-byte boundary tests             | B18 (39 bytes), B19 (41 bytes), B20 (0 bytes) added                                               |
| MAJOR-5  | MAJOR    | Mutation survivor: `!= 40` → `> 40` in URL path                   | Killed by B18 (39-byte URL test) + M3 checkpoint                                                  |
| MAJOR-6  | MAJOR    | Mutation survivor: BackendError branch deletion in URL path        | Killed by B24 (URL BackendError test) + M13 checkpoint                                            |
| MINOR-1  | MINOR    | B1 "succeeds without error" is `is_ok()` in disguise              | B1 L125: concrete `Ok(map)` with `map.len() == 1`                                                 |
| MINOR-2  | MINOR    | Summary says 14 behaviors; inventory lists 16                     | L8: "Behaviors identified: 30" matches inventory B1-B30                                           |
| MINOR-3  | MINOR    | Summary trophy counts don't match detailed allocation             | L9: "2 static / 2 unit / 26 integration" matches trophy table (L77-108)                           |
| MINOR-4  | MINOR    | Resource cleanup not explicit                                     | L17: explicit statement — all tests use in-memory redb, no tempfile, no cleanup needed            |
| MINOR-5  | MINOR    | `u64::MIN`/`u64::MAX` not named as boundary seeds                | L619, L635-636: proptest strategies list `u64::MIN (0)` and `u64::MAX` as boundary seeds          |

---

## Axis 1 — Contract Parity

**Status: PASS**

### Public function → BDD scenario mapping

| `pub fn` (contract.md)                | BDD Scenarios (test-plan.md)              | Verdict |
|---------------------------------------|-------------------------------------------|---------|
| `StateReadSession::new` (L120)        | B1 (L117-127)                             | ✅ PASS |
| `load_file_states` (L144)             | B4-B15 (L159-358) = 12 scenarios          | ✅ PASS |
| `load_url_states` (L168)             | B16-B28 (L362-577) = 13 scenarios         | ✅ PASS |

### Error variant → exact-assertion mapping

| Variant | `load_file_states` (exact field assertions) | `load_url_states` (exact field assertions) |
|---------|---------------------------------------------|---------------------------------------------|
| `MalformedRow { key, actual, expected }` | B6 (39b), B7 (41b), B8 (0b), B9 (20b mixed) | B18 (39b), B19 (41b), B20 (0b), B21 (80b), B22 (20b mixed) |
| `Utf8KeyError { bytes_lossy }` | B10 — exact `from_utf8_lossy` output | B23 — exact `from_utf8_lossy` output |
| `BackendError { operation, message }` | B11 — `operation=="open_table"`, exact message | B24 — `operation=="open_table"`, exact message |

All 3 `pub fn`s have ≥1 BDD scenario. All 3 error variants have exact-variant assertions for **both** methods. No `is_ok()` or `is_err()` used as terminal assertions.

---

## Axis 2 — Assertion Sharpness

**Status: PASS**

All 30 `Then:` clauses inspected. Findings:

- Zero `is_ok()` / `is_err()` terminal assertions
- Zero `Some(_)` without concrete inner value
- Zero `> 0` or boolean-only assertions
- All `Err` scenarios destructure the variant and assert exact field values
- All `Ok` scenarios assert concrete cardinality + exact struct/byte equality
- `bytes_lossy` fields use exact `String::from_utf8_lossy(&bytes)` equality (B10 L270, B23 L487)
- `BackendError` `message` fields specify exact redb error capture strategy (B11 L286-288, B24 L501-503)
- B5/B17 triple-check empty maps: `len()==0` AND `is_empty()` AND `== HashMap::new()`
- B12/B25 assert byte-for-byte `content_hash` equality AND exact `u64` equality

---

## Axis 3 — Trophy Allocation

**Status: PASS**

| Metric | Value | Threshold | Verdict |
|--------|-------|-----------|---------|
| Public functions | 3 | — | — |
| BDD scenarios | 30 (B1-B30) | — | — |
| Proptest invariants | 4 (2 Pod round-trips + 2 cardinality) | — | — |
| Fuzz targets | 2 (file + URL byte parsing) | — | — |
| Kani harnesses | 2 (no-padding proofs) | — | — |
| **Total planned tests** | **38** | — | — |
| **Test/function ratio** | **12.7×** | ≥ 5× | ✅ PASS |

Pure functions with proptest: Pod round-trips for both structs ✅
Parser/deserializer fuzz targets: both file and URL paths ✅
Trophy split: 2 static / 2 unit / 26 integration — justified by redb-bound nature ✅

---

## Axis 4 — Boundary Completeness

**Status: PASS**

### `load_file_states` boundaries (all covered)

Empty (B5) · 1 row (B12) · N rows (proptest) · Value=0b (B8) · Value=39b (B6) · Value=40b (B4,B12) · Value=41b (B7) · Non-UTF-8 key (B10) · Missing table (B11) · Snapshot isolation (B13) · Cross-table (B14) · Multi-byte UTF-8 (B15) · u64::MIN/MAX (proptest L619)

### `load_url_states` boundaries (all covered)

Empty (B17) · 1 row (B25) · N rows (proptest) · Value=0b (B20) · Value=39b (B18) · Value=40b (B16,B25) · Value=41b (B19) · Value=80b (B21) · Non-UTF-8 key (B23) · Missing table (B24) · Snapshot isolation (B26) · Cross-table (B27) · Multi-byte UTF-8 (B28) · u64::MIN/MAX (proptest L635)

Full symmetry between file/URL paths. No function has ≥3 missing boundaries.

---

## Axis 5 — Mutation Survivability

**Status: PASS**

22 mutations mentally applied. 0 survivors.

| Mutation | Caught by |
|----------|-----------|
| `!= 40` → `> 40` (file) | B6 (39 bytes) |
| `!= 40` → `> 40` (URL) | B18 (39 bytes) |
| `!= 40` → `< 40` (file) | B7 (41 bytes) |
| `!= 40` → `< 40` (URL) | B19 (41 bytes) |
| Delete MalformedRow branch (file) | B6 + B4 |
| Delete MalformedRow branch (URL) | B18 + B16 |
| Delete Utf8KeyError branch (file) | B10 |
| Delete Utf8KeyError branch (URL) | B23 |
| Delete BackendError branch (file) | B11 |
| Delete BackendError branch (URL) | B24 |
| Return empty map (file) | B4 (len==3) |
| Return empty map (URL) | B16 (len==3) |
| Swap table targets | B14 + B27 |
| Replace `bytes_lossy` with arbitrary string | B10 + B23 (exact match) |
| Remove early-return on error (file) | B9 (abort test) |
| Remove early-return on error (URL) | B22 (abort test) |
| Swap `actual`/`expected` fields | B6 + B18 |
| Fill `actual` with 0 | B9 + B22 (actual==20) |
| Fill `expected` with 999 | B6 + B18 (expected==40) |
| Return `Ok(Default::default())` | B4 (exact struct values) + B12 (bitwise) |
| `>` to `>=` boundary shift | B6 + B7 + B18 + B19 |
| Delete error branch, propagate raw redb err | B11 + B24 (variant mismatch) |

Mutation kill rate: 22/22 = 100%.

---

## Axis 6 — Holzmann Plan Audit

**Status: PASS**

| Rule | Compliance | Evidence |
|------|------------|----------|
| R1 Linear | ✅ | All scenarios follow Given→When→Then; no nesting |
| R2 Bound loops | ✅ | No loops in test bodies; proptest manages iteration |
| R3 Know resources | ✅ | L17: all tests use in-memory redb, no tempfile, no cleanup |
| R4 One function, one job | ✅ | Each scenario targets one behavior |
| R5 State assumptions | ✅ | Every `Given` block is explicit and self-contained |
| R6 Never swallow errors | ✅ | No `let _ =` or `.ok()` patterns |
| R7 Narrow state | ✅ | Each test creates own redb instance; no shared mutable state |
| R8 Surface side effects | ✅ | Descriptive function names; explicit Given blocks |
| R9 One layer of magic | ✅ | No deep helper chains |
| R10 Warnings are errors | N/A | Plan only, no code |

---

## LETHAL FINDINGS

None.

## MAJOR FINDINGS

None.

## MINOR FINDINGS

None.

---

## Summary Statistics

| Axis | LETHAL | MAJOR | MINOR | Verdict |
|------|--------|-------|-------|---------|
| 1 — Contract Parity | 0 | 0 | 0 | PASS |
| 2 — Assertion Sharpness | 0 | 0 | 0 | PASS |
| 3 — Trophy Allocation | 0 | 0 | 0 | PASS |
| 4 — Boundary Completeness | 0 | 0 | 0 | PASS |
| 5 — Mutation Survivability | 0 | 0 | 0 | PASS |
| 6 — Holzmann Audit | 0 | 0 | 0 | PASS |
| **TOTAL** | **0** | **0** | **0** | **APPROVED** |

Threshold: any LETHAL = REJECTED. ≥3 MAJOR = REJECTED. ≥5 MINOR = REJECTED.
Result: 0 + 0 + 0 = well within all thresholds.

---

## MANDATE

None. This test plan is approved for implementation. All 30 BDD scenarios, 4 proptest invariants, 2 fuzz targets, and 2 Kani harnesses may proceed to the test-writing phase.
