# Test Plan Review: cdocs-bg3

**Reviewer:** Test Inquisitor (Mode 1 — Plan Inquisition)
**Date:** 2026-04-02
**Revision:** 2 (re-audit after REJECTED revision 1)
**Contract:** `.beads/cdocs-bg3/contract.md` (632 lines)
**Test Plan:** `.beads/cdocs-bg3/test-plan.md` (1290 lines)
**Previous Review:** `.beads/cdocs-bg3/test-plan-review.md` (revision 1 — 11 MAJOR, 7 MINOR)

---

## VERDICT: APPROVED

**0 LETHAL / 2 MAJOR / 2 MINOR**

Thresholds: Any LETHAL = instant REJECT. ≥3 MAJOR = REJECT. ≥5 MINOR = REJECT.
0 + 2 MAJOR (< 3) + 2 MINOR (< 5) = **APPROVED**.

---

## Previous Defect Resolution (ALL 18 findings)

| # | Previous Finding | Resolution | Verified |
|---|---|---|---|
| MAJOR-1 | B26 `table` tautological | Line 482: `table == "file_state"`, line 484: `message.contains("type")` | ✅ |
| MAJOR-2 | DeserializationFailed `type_name` tautological | Line 898: `type_name == "Analysis"` | ✅ |
| MAJOR-3 | StorageError `operation` tautological | Line 943: `operation == "insert"` | ✅ |
| MAJOR-4 | 5 "non-empty" message assertions | Lines 840, 854, 869, 501, 484: all use `.contains()` | ✅ |
| MAJOR-5 | Trophy allocation contradiction | Summary (line 6) and table (lines 106-111): 17/28/1/7=53 | ✅ |
| MAJOR-6 | B11/B12/B13 classification contradiction | Static everywhere, explicit note at lines 113-114 | ✅ |
| MAJOR-7 | INV-11 trailing slash zero coverage | B40 added (lines 687-698) | ✅ |
| MAJOR-8 | B30 Holzmann Rule 2 caveat missing | Lines 553-554: explicit loop prohibition | ✅ |
| MAJOR-9 | B26/B27 non-determinism | B26: type-mismatch strategy with code (lines 472-479). B27: chmod (lines 496-502). Both have Determinism notes. | ✅ |
| MAJOR-10 | SerializationFailed feasibility uncertain | Lines 909-931: honest construction-only with explicit rationale | ✅ |
| MAJOR-11 | StorageError platform-dependence | Lines 946-949: `#[cfg(target_os = "linux")]` gate documented | ✅ |
| MINOR-1 | Source path trailing whitespace | B41 (lines 700-711) | ✅ |
| MINOR-2 | Source path null bytes | B42 (lines 714-725) | ✅ |
| MINOR-3 | Very long source path | B47 (lines 794-805), 500-char path | ✅ |
| MINOR-4 | Partial initialization | B43 (lines 728-743) | ✅ |
| MINOR-5 | URL query/fragment | B45, B46 (lines 764-791) | ✅ |
| MINOR-6 | Proptest 6 N/A inflation | Summary line 8: "5" proptests. Section 4 lists only 1-5 | ✅ |
| MINOR-7 | Shared metadata co-access | B44 (lines 747-760) | ✅ |

**Result: 18/18 previous findings resolved. All mandate items addressed.**

---

## Axis 1 — Contract Parity

### Public Functions → BDD Coverage

| # | Contract Function | Scenario(s) | Verdict |
|---|---|---|---|
| 1 | `fn initialize_tables(db: &Database)` | B24, B25, B26, B27, B28, B29, B43 | PASS |
| 2 | `const fn file_state_table()` | B16 | PASS |
| 3 | `const fn url_state_table()` | B17 | PASS |
| 4 | `const fn analysis_outputs_table()` | B18 | PASS |
| 5 | `const fn transform_outputs_table()` | B19 | PASS |
| 6 | `const fn chunk_outputs_table()` | B20 | PASS |
| 7 | `const fn scrape_outputs_table()` | B21 | PASS |
| 8 | `const fn snapshots_table()` | B22 | PASS |
| 9 | `const fn metadata_table()` | B23 | PASS |

**Result: PASS** — All 9 public functions have ≥1 BDD scenario.

### Error Variant → Scenario Coverage

| # | StateError Variant | Scenario(s) | Asserts Exact Variant? | Concrete Fields? |
|---|---|---|---|---|
| 1 | `OpenFailed` | Dedicated (line 835) | YES | `path: exact dir`, `source.contains("open")` |
| 2 | `ReadTransactionFailed` | Dedicated (line 849) | YES | `message.contains("transaction")` |
| 3 | `WriteTransactionFailed` | Dedicated (line 863) | YES | `message.contains("write")` |
| 4 | `PodSizeMismatch` | B36, B37 | YES | `table`, `expected`, `actual` all concrete |
| 5 | `PodCastFailed` | Dedicated (line 876) | YES | `type_name == "FileStateRaw"`, `message.contains("size")` |
| 6 | `InvalidArchive` | B38 | YES | `type_name == "Analysis"`, `message.len() > 0` (see MAJOR-1) |
| 7 | `DeserializationFailed` | Dedicated (line 891) | YES | `type_name == "Analysis"` |
| 8 | `SerializationFailed` | Dedicated (line 909) | YES | Construction-only: variant match + display format |
| 9 | `TableOpenFailed` | B26 | YES | `table == "file_state"`, `message.contains("type")` |
| 10 | `KeyNotFound` | B39 | YES | `table: "analysis_outputs"` |
| 11 | `StorageError` | Dedicated (line 934) | YES | `operation == "insert"`, `message.len() > 0` (see MAJOR-2) |
| 12 | `CommitFailed` | B27 | YES | `message.contains("write") or contains("permission")` |
| 13 | `InvalidHashKeyLength` | B32 | YES | `actual: 16` |
| 14 | `InvalidSourcePath` | B33, B34, B41, B42 | YES | Substring assertions per case |
| 15 | `InvalidUrlKey` | B35, B40 | YES | Substring assertions per case |

**Result: PASS** — All 15 variants have scenarios asserting exact variant with concrete structural fields.
No `is_ok()` or `is_err()` patterns found anywhere in the plan.

---

## Axis 2 — Assertion Sharpness

### Scanned: All 47 BDD scenarios + 7 error variant scenarios (54 total)

Every `Then:` and `And:` block examined for:

- `is_ok()` / `is_err()` → 0 instances found ✅
- `> 0` or `len() > 0` without concrete value → 2 instances found ⚠️
- `Some(_)` without inner value → 0 instances found ✅
- Vague "non-empty" descriptions → 0 instances found ✅

**MAJOR-1: B38 InvalidArchive `message.len() > 0` (test-plan.md:666)**

```
And:   message.len() > 0
```

The `> 0` pattern. The structural assertion `type_name == "Analysis"` is concrete and kills
the important mutation. The message comes from rkyv's bytecheck library — its exact content
is unpredictable across rkyv versions. However, per Axis 2 rules, `> 0` assertions are
classified as MAJOR regardless of context. A mutation returning `"x"` for the message
would survive this specific assertion (though the type_name assertion would still pass).

Severity: **MAJOR** (per Axis 2 rules). Impact is mitigated by the concrete type_name assertion.

**MAJOR-2: StorageError `message.len() > 0` (test-plan.md:944)**

```
And:   message.len() > 0
```

Same `> 0` pattern. The structural assertion `operation == "insert"` is concrete. The message
comes from the OS I/O error (`std::io::Error::to_string()`) — genuinely unpredictable. Same
mitigation: the important field has a concrete assertion.

Severity: **MAJOR** (per Axis 2 rules). Impact is mitigated by the concrete operation assertion.

---

## Axis 3 — Trophy Allocation

### Density Ratio

| Metric | Value |
|--------|-------|
| Public functions (contract) | 9 |
| Static tests (B01-B04, B11-B13) | 7 |
| Unit tests (B05-B10, B14-B23, PodCastFailed) | 17 |
| Integration tests (B24-B29, B31-B47, 5 error variants) | 28 |
| E2E tests (B30) | 1 |
| **Total test functions** | **53** |
| **Density ratio** | **53 / 9 = 5.89×** |

**Result: PASS** — 5.89× exceeds 5× threshold.

### Allocation Consistency

Summary line 6: `17 unit / 28 integration / 1 e2e / 7 static = 53`
Trophy table (lines 106-111): Static=7, Unit=17, Integration=28, E2E=1, Total=53
Ratio breakdown (line 112): 32% / 53% / 2% / 13% — math verified correct.

**Result: PASS** — Numbers are fully reconciled.

### Proptest & Fuzz

| Check | Result |
|-------|--------|
| Pure functions (FileStateRaw/UrlStateRaw) have proptest? | PASS — Proptests 1-5 cover round-trip, byte layout, zeroed fixed-point |
| Parser/deserializer fuzz target? | PASS — Fuzz Target 1 covers Pod byte interpretation |
| Fuzz Target 2 (rkyv) deferred honestly? | PASS — rkyv derives out of scope per contract Non-goal #1 |
| Proptest count accurate? | PASS — 5 listed, 5 real, no N/A inflations |

---

## Axis 4 — Boundary Completeness

### Per-Function Boundary Audit

| Function | min valid | max valid | one-below | one-above | empty/zero | overflow |
|----------|-----------|-----------|-----------|-----------|------------|----------|
| `initialize_tables` | empty db (B24) | init'd db (B25) | N/A | N/A | empty db ✅ | partial init (B43) ✅ |
| Accessor fns (8) | N/A (no input) | N/A | N/A | N/A | N/A | N/A |
| FileStateRaw size | 200B (B01) | — | 199B (B36) | 201B (matrix:1248) | zeroed (B05) | — |
| UrlStateRaw size | 120B (B02) | — | 119B (matrix:1250) | 121B (B37) | zeroed (B06) | — |
| Hash key len | 32B (B32+) | — | 0B (matrix:1225), 16B (B32) | 33B (matrix:1227) | 0B ✅ | — |
| Source path | valid (B47) | 500 chars (B47) | "/" abs (B33) | — | "" (matrix:1231) | null (B42), whitespace (B41) |
| URL key | valid (B45, B46) | — | no scheme (B35) | — | "" (matrix:1237) | trailing slash (B40) |

### Missing Boundaries

**MINOR-1: Root URL trailing-slash exception untested (contract.md:384)**

INV-11 states: *"MUST NOT contain trailing slashes (except for root `/`)"*. B40 tests
rejection of `"https://example.com/api/"` (non-root trailing slash). No test verifies
the POSITIVE case: `"https://example.com/"` (root path = trailing slash exception) is
accepted. The contract explicitly defines this exception — it must be tested as a valid
boundary case.

**MINOR-2: Combinatorial matrix entries lack B## traceability (test-plan.md:1222-1251)**

Six boundary entries in the combinatorial coverage matrix have no B## scenario numbers:

| Matrix Entry | Line | Has B##? |
|---|---|---|
| Hash key 0 bytes | 1225 | No |
| Hash key 33 bytes | 1227 | No |
| Source path empty `""` | 1231 | No |
| URL empty `""` | 1237 | No |
| file_state 201 bytes | 1248 | No |
| url_state 119 bytes | 1250 | No |

These are specified with concrete expected outputs but have no explicit test function
traceability. A tester implementing from the plan could overlook them. Each should either:
(a) be assigned a B## number, or (b) be explicitly grouped under an existing scenario
with a note that the test function covers multiple matrix entries.

---

## Axis 5 — Mutation Survivability

### Section 7 Verification (27 claimed mutations)

All 27 mutations in the plan's table (lines 1115-1145) verified against their listed killers:

| Mutation Category | Count | Killed By | Verified |
|---|---|---|---|
| Table name change | 3 | B09/B10 + accessor tests | ✅ |
| Pod struct layout change | 3 | B01/B02 + Kani + Proptest 3 | ✅ |
| initialize_tables skip | 3 | B24, B28, B43 | ✅ |
| Size validation off-by-one | 2 | B36, B37 | ✅ |
| Hash key check off-by-one | 1 | B32 + combinatorial 33B | ✅ |
| Remove validation check | 6 | B33, B34, B35, B40, B41, B42 | ✅ |
| Error field wrong value | 4 | B26, DeserializationFailed, StorageError, message checks | ✅ |
| Error message trivialized | 1 | B26/B27/OpenFailed/ReadTxFailed/WriteTxFailed substring checks | ✅ |
| Metadata type/name change | 2 | B13, B15, B23 | ✅ |
| Name uniqueness skip | 1 | B09 | ✅ |
| Reject valid input | 3 | B45, B46, B47 | ✅ |

### Surviving Mutations (2)

| Mutation | Survives At | Reason | Impact |
|---|---|---|---|
| Return `"x"` for InvalidArchive message | B38 (line 666) | `message.len() > 0` passes for `"x"` | Low — type_name concrete |
| Return `"x"` for StorageError message | StorageError (line 944) | `message.len() > 0` passes for `"x"` | Low — operation concrete |

Both survivors are cosmetic (error message quality), not logic errors. The structural
fields (`type_name`, `operation`) have concrete assertions that catch the meaningful mutations.

**Kill rate: 27/29 ≈ 93%. Above 90% threshold.**

**Result: PASS**

---

## Axis 6 — Holzmann Plan Audit

| Rule | Assessment | Verdict |
|------|-----------|---------|
| Rule 1 (Linear) | All scenarios Given/When/Then, single flow | PASS |
| Rule 2 (Bound Loops) | B30 explicit loop-prohibition caveat (lines 553-554) | PASS |
| Rule 3 (Own Resources) | All integration tests use `tempfile::TempDir`, self-cleaning | PASS |
| Rule 4 (One Job) | Each scenario tests one behavior, clear test names | PASS |
| Rule 5 (State Assumptions) | All scenarios have explicit Given blocks with preconditions | PASS |
| Rule 6 (No Swallow) | No `let _ =` or `.ok()` patterns in plan | PASS |
| Rule 7 (Narrow State) | Each integration test creates own database. No shared state. | PASS |
| Rule 8 (Surface Effects) | Test function names describe behavior clearly | PASS |
| Rule 9 (One Magic Layer) | No deep helper chains described | PASS |
| Rule 10 (Warnings = Errors) | Deferred to Mode 2 (implementation time) | PASS (deferred) |

**Result: PASS**

---

## MAJOR FINDINGS (2/3 threshold)

| # | Location | Finding |
|---|----------|---------|
| MAJOR-1 | test-plan.md:666 | B38 InvalidArchive `message.len() > 0` — `> 0` pattern. Structural field (`type_name == "Analysis"`) is concrete. Message comes from rkyv bytecheck (external, unpredictable). Mutation returning `"x"` for message survives. Mitigated by concrete type_name. |
| MAJOR-2 | test-plan.md:944 | StorageError `message.len() > 0` — `> 0` pattern. Structural field (`operation == "insert"`) is concrete. Message comes from OS I/O error (external, unpredictable). Mutation returning `"x"` for message survives. Mitigated by concrete operation. |

**Assessment:** Both MAJORs are on external-message diagnostic fields where the important structural assertions are concrete. The surviving mutations are cosmetic (error message content), not logic-altering. These do NOT represent test effectiveness gaps for the contract's behavioral guarantees.

---

## MINOR FINDINGS (2/5 threshold)

| # | Location | Finding |
|---|----------|---------|
| MINOR-1 | contract.md:384 → test-plan.md (absent) | Root URL trailing-slash exception untested as positive case. INV-11: "MUST NOT contain trailing slashes (except for root `/`)." B40 tests rejection. No test verifies `"https://example.com/"` IS accepted. |
| MINOR-2 | test-plan.md:1222-1251 | 6 combinatorial matrix entries lack B## scenario numbers (hash key 0B, 33B; source path empty; URL empty; file_state 201B; url_state 119B). No traceability to test functions. |

---

## VERDICT: APPROVED

**0 LETHAL / 2 MAJOR (< 3 threshold) / 2 MINOR (< 5 threshold)**

The revised test plan resolves all 18 findings from the previous rejection. The plan now
provides:

- **Complete contract parity:** All 9 public functions and all 15 error variants covered
- **Sharp assertions:** 52/54 scenarios use concrete values; 2 use `> 0` on external diagnostic fields
- **Adequate density:** 53 tests / 9 functions = 5.89×
- **Comprehensive proptest + fuzz:** 5 proptests, 1 in-scope fuzz target
- **Mutation kill rate ≥ 90%:** 27/29 = 93%
- **Holzmann compliance:** All 10 applicable rules satisfied

The 2 remaining MAJOR findings are on external error message fields where the structural
assertions are already concrete. The 2 MINOR findings are traceability gaps, not coverage gaps.
None cross the rejection thresholds.

**This plan is approved for implementation.**

### Recommendations (non-blocking)

1. For B38 and StorageError: if rkyv/OS error messages contain any predictable substring
   (e.g., "invalid", "error"), upgrade `message.len() > 0` to `message.contains(...)`.
   If no predictable substring exists across versions, document this decision inline.

2. Assign B## numbers to the 6 unnumbered combinatorial matrix entries, or add a note
   under each existing scenario listing which additional matrix entries it covers.

3. Add a positive test for `"https://example.com/"` (root URL with trailing slash = INV-11
   exception case) alongside the existing B40 rejection test.
