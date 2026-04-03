# Test Plan Review: cdocs-9nr (Revision 2)

**Bead**: cdocs-9nr — "Wire startup state open and file diff into `run_index`"
**Reviewer**: Test Inquisitor (Mode 1 — Plan Inquisition)
**Date**: 2026-04-03
**Contract**: `contract.md` (250 lines)
**Test Plan**: `test-plan.md` (791 lines, revised)
**Prior Review**: 7 LETHAL · 4 MAJOR · 5 MINOR — REJECTED

---

## VERDICT: APPROVED

**0 LETHAL · 0 MAJOR · 3 MINOR — all thresholds clear**

---

## Prior Defect Verification

Every defect from the prior rejection has been addressed:

### Prior LETHAL — All 7 FIXED

| Prior ID | Finding | Fix Verification | Status |
|---|---|---|---|
| L-1 | `CommitError::TableInit` — zero test scenarios | B20 added (lines 346–361): full BDD scenario with `create_redb_with_locked_tables` setup helper, concrete Then: `error message contains "failed to initialize tables"` and `non-empty reason string`. Error Variant Coverage table (line 717) maps it. | **FIXED** |
| L-2 | `StateLoadError::Utf8KeyError` — zero test scenarios | B22 added (lines 381–410): dual-path strategy — primary via raw `TableDefinition<&[u8], &[u8]>` injection, fallback via direct construction verifying Display + anyhow conversion. Concrete Then: `string contains "non-UTF-8 key in state table"` and `"��invalid"`. Error Variant Coverage table (line 725) maps it. | **FIXED** |
| L-3 | B18 bare `is_err()` | B18 (lines 314–328): `Then: result is Err and the error message to_string() contains "failed to open state database"` + `And: the error message contains the path substring "state.redb"`. Concrete substrings in Then itself. | **FIXED** |
| L-4 | B19 bare `is_err()` + "or similar" escape hatch | B19 (lines 330–344): `Then: result is Err and the error message to_string() contains "failed to begin read transaction"`. No "or similar". Concrete substring in Then. | **FIXED** |
| L-5 | B20 (MalformedRow) bare `is_err()` | B21 (lines 363–379, renumbered): `Then: result is Err and the error message to_string() contains "malformed raw state row"` + `And: the error message contains "got 199 bytes, expected 200"`. Concrete. | **FIXED** |
| L-6 | B21 (SourceDirNotFound) bare `is_err()` | B23 (lines 412–424, renumbered): `Then: result is Err(DiffError::SourceDirNotFound(path)) where path matches nonexistent_dir` + `And: the error .to_string() contains "source directory does not exist"`. Exact variant + message. | **FIXED** |
| L-7 | B22 (FileRead) bare `is_err()` | B24 (lines 426–436, renumbered): `Then: result is Err and the error message to_string() contains "failed to read file"` + `And: the error message contains the filename of the unreadable file`. Concrete. | **FIXED** |

### Prior MAJOR — All 4 FIXED

| Prior ID | Finding | Fix Verification | Status |
|---|---|---|---|
| M-1 | B12 vague `N > 0` | B12 (line 251): `Then: stdout output includes "[DIFF] Unchanged: 2  Changed: 0  New: 0  Deleted: 0"`. Exact count. | **FIXED** |
| M-2 | B13 non-assertion "verifiable via..." | B13 (line 265): `Then: stdout includes "[DIFF] Unchanged: 2  Changed: 0  New: 0  Deleted: 0"`. Concrete counts from two-run scenario. | **FIXED** |
| M-3 | `file_states_to_stored_hashes` — 3 boundaries unnamed | Combinatorial matrix (lines 669–670): explicit "boundary: max valid" (10,000 entries) and "boundary: overflow" (N/A with rationale). | **FIXED** |
| M-4 | `run_index` wiring — 3 boundaries unnamed | Combinatorial matrix (lines 695–697): explicit "boundary: max valid files" (1,000 .md), "boundary: max stored state" (1,000 rows), "boundary: overflow" (N/A with rationale). | **FIXED** |

### Prior MINOR — All 5 FIXED

| Prior ID | Finding | Fix Verification | Status |
|---|---|---|---|
| m-1 | Cleanup strategy not stated | Lines 752–758: explicit "Cleanup Strategy" section describing TempDir RAII semantics, including permission restoration behavior. | **FIXED** |
| m-2 | B10 setup mechanism unspecified | B10 (line 211): `seed_file_state_rows(db: &Database, rows: &[(&str, FileStateRaw)])` — named helper with signature and purpose. | **FIXED** |
| m-3 | B19 "corrupted state.redb" mechanism unspecified | B19 (lines 331–335): `create_corrupted_redb_at(path: &Path)` — named helper with detailed corruption strategy description. | **FIXED** |
| m-4 | B20 "row != 200 bytes" mechanism unspecified | B21 (lines 365–369): `inject_malformed_file_state_row(db: &Database, key: &str, size: usize)` — named helper with detailed description. | **FIXED** |
| m-5 | Side-effectful helpers unnamed | Lines 740–751: "Setup Helpers (named, side-effect-advertising)" table — 7 helpers listed with names, purposes, and consuming behaviors. | **FIXED** |

---

## Fresh Six-Axis Inquisition

### Axis 1 — Contract Parity

**[PASS]**

| Contract Function | BDD Scenarios | Status |
|---|---|---|
| `pub fn run_index` | B8–B28 (21 scenarios) | PASS |
| `fn file_states_to_stored_hashes` | B1–B3 (3 scenarios) + Proptest 1–2 + Kani 1 | PASS |
| `StateDb::open` (external) | B8, B18, B20 | PASS |
| `StateDb::begin_read` (external) | B9, B19 | PASS |
| `StateReadSession::load_file_states` (external) | B10, B11, B21, B22 | PASS |
| `compute_config_hash` (external, infallible) | B4–B7 (noted pre-existing) | PASS |
| `compute_file_diff` (external) | B14, B23–B25, Proptest 3–4, Kani 2 | PASS |

**Error variant coverage:**

| Error Variant | Test Scenario | Assertion | Status |
|---|---|---|---|
| `CommitError::DatabaseOpen` | B18 | `contains "failed to open state database"` + path substring | PASS |
| `CommitError::TableInit` | B20 | `contains "failed to initialize tables"` + non-empty reason | PASS |
| `CommitError::ReadTransaction` | B19 | `contains "failed to begin read transaction"` | PASS |
| `StateLoadError::MalformedRow` | B21 | `contains "malformed raw state row"` + `"got 199 bytes, expected 200"` | PASS |
| `StateLoadError::Utf8KeyError` | B22 | `contains "non-UTF-8 key in state table"` + `"��invalid"` | PASS |
| `StateLoadError::BackendError` | Coverage table (line 726) | Described: `BackendError { operation: "open_table" }` on uninitiated DB | PASS* |
| `DiffError::SourceDirNotFound` | B23 | `Err(DiffError::SourceDirNotFound(path))` + `contains "source directory does not exist"` | PASS |
| `DiffError::FileRead` | B24 | `contains "failed to read file"` + filename | PASS |
| `DiffError::PathTraversal` | B25 | `Err(DiffError::PathTraversal { path })` + `contains "path traversal detected"` | PASS |

*See MINOR-2: `BackendError` is described in the coverage table but lacks a formal B-number.

### Axis 2 — Assertion Sharpness

**[PASS]**

Every "Then:" clause in all 30 BDD scenarios audited:

- Zero bare `is_ok()` or `is_err()` assertions.
- Zero vague `> 0`, `Some(_)`, or "or similar" escape hatches.
- All error scenarios specify concrete error message substrings within the "Then:" clause itself (not deferred to "And:").
- All success scenarios specify concrete values: exact counts (`Unchanged: 2`), exact file existence checks (`state.redb exists on disk`), exact output format (`[DIFF] Unchanged: N  Changed: M  New: K  Deleted: L`).
- B15 is a format-validation test using regex — appropriate for its purpose (format, not values). Values are covered by B11, B12, B14, B16, B28, B30.

### Axis 3 — Trophy Allocation

**[PASS]**

- **Public functions**: 1 (`run_index`)
- **Total planned tests**: 30 behaviors + 4 proptest + 2 Kani = **36**
- **Density ratio**: 36 / 1 = 36× (target ≥5×) ✓
- **Pure function proptest**: `file_states_to_stored_hashes` (Proptest 1–2) ✓
- **`compute_file_diff` proptest**: Proptest 3–4 (partition completeness + determinism) ✓
- **Fuzz targets**: None — justified (no new parsers/deserializers in this bead) ✓
- **Trophy ratio**: 10 unit / 18 integration / 2 e2e = 33%/60%/7% — within target ✓

### Axis 4 — Boundary Completeness

**[PASS with 0 MAJOR, 0 MINOR]**

**`file_states_to_stored_hashes`:**

| Boundary | Named? | Location |
|---|---|---|
| Minimum valid (empty HashMap) | ✓ | B2, combinatorial matrix |
| Maximum valid (10,000 entries) | ✓ | Combinatorial matrix line 669 |
| One-below-minimum | N/A | Negative cardinality impossible |
| One-above-maximum | N/A | No domain upper bound; 10,000 is an arbitrary stress size, not a limit. `HashMap` grows until OOM. |
| Empty / zero | ✓ | B2 |
| Overflow / underflow | ✓ (N/A with rationale) | Combinatorial matrix line 670: "Rust allocates on heap, OOM before logic error" |

**`run_index` state/diff wiring:**

| Boundary | Named? | Location |
|---|---|---|
| Minimum valid (1 file) | ✓ | B8 |
| Maximum valid (1,000 files, 1,000 stored rows) | ✓ | Combinatorial matrix lines 695–696 |
| One-below-minimum (0 files) | ✓ | Combinatorial matrix: "bail before diff" |
| One-above-maximum | N/A | No domain upper bound; filesystem limits catch overflow before logic errors. |
| Empty DB (first run) | ✓ | B11, B16 |
| Overflow | ✓ (N/A with rationale) | Combinatorial matrix line 697 |

Both functions have no natural domain maximum. The "one-above-max" concept doesn't apply when the max is an arbitrary test size, not a constraint. The stress tests at 1,000–10,000 entries verify no degradation at scale. N/A rationale is honest and sufficient.

### Axis 5 — Mutation Survivability

**[PASS]**

The plan identifies 20 mutations with explicit catching tests (lines 616–655). Mental verification of the most dangerous:

| Mutation | Caught By | Verified |
|---|---|---|
| Swap `content_hash`/`config_hash` | B3 + Proptest 2 | ✓ |
| Drop a key in conversion | B1 + Proptest 1 | ✓ |
| Return empty for non-empty input | B1 (len == 3) | ✓ |
| Classify all as Unchanged | B16 (expects New: 2) | ✓ |
| Classify all as New (ignore stored) | B28 (expects Unchanged: 1) | ✓ |
| Skip deleted computation | B30 (expects Deleted: 1) — see MINOR-1 | ✓ |
| `||` instead of `&&` for hash comparison | B28 (content-only change would be Unchanged with `||`) | ✓ |
| `run_index` skips `StateDb::open` | B8 (state.redb must exist) | ✓ |
| `run_index` skips `begin_read` | B11 (no diff output) | ✓ |
| `run_index` calls `commit_changes` | B27 (0 rows in file_state) | ✓ |
| `run_index` gates analysis on diff | B17 (all files analyzed) | ✓ |
| Error conversion drops context | B18–B25 (concrete message substrings) | ✓ |

All mutations are caught. See MINOR-1 for a documentation error in the mapping table.

### Axis 6 — Holzmann Plan Audit

**[PASS]**

| Rule | Assessment |
|---|---|
| R1 (Linear) | All 30 scenarios follow single Given→When→Then flow. No nested conditionals. |
| R2 (Bound loops) | No loops planned in test bodies. Proptest strategies generate data, not iterate. |
| R3 (Own resources) | Lines 752–758: explicit TempDir RAII cleanup strategy. Permission restoration documented. |
| R4 (One job) | Each scenario tests one behavior. Names describe the single assertion. |
| R5 (State assumptions) | All Given blocks state preconditions explicitly. Setup helpers named with signatures. |
| R6 (No swallow) | N/A for plan mode — no implementation to audit. |
| R7 (Narrow state) | No shared mutable state. Per-test TempDir instances. |
| R8 (Surface effects) | Lines 740–751: 7 setup helpers with side-effect-advertising names (`seed_file_state_rows`, `create_corrupted_redb_at`, `inject_malformed_file_state_row`, etc.). |
| R9 (One magic layer) | Test → helper → redb API. Max depth 2. Plan suggests extracting `compute_initial_diff` (line 775). |
| R10 (Warnings = errors) | N/A for plan mode. |

---

## MINOR FINDINGS (3/5 threshold — does NOT trigger rejection)

### MINOR-1: Mutation table B28/B30 misattribution

**Location**: test-plan.md line 629
**Finding**: The mutation table says `compute_file_diff skips deleted computation` is caught by B28 (`shows Deleted: 0 when file was deleted`). This is incorrect — B28 has no deleted files (all 3 original files remain: one unchanged, one changed, one new added). B28 correctly expects `Deleted: 0` because nothing was deleted, not because deleted computation was skipped.

The mutation IS caught — by B30 (line 512), which deletes `remove.md` and expects `Deleted: 1`. If deleted computation is skipped, B30 would fail (seeing Deleted: 0 instead of 1).

**Impact**: Documentation error only. Mutation coverage is not affected. B30 catches the mutant. The table should read "B30: shows Deleted: 1 when file was deleted".

### MINOR-2: `StateLoadError::BackendError` lacks formal BDD scenario

**Location**: test-plan.md line 726
**Finding**: The Error Variant Coverage table describes a test for `StateLoadError::BackendError`: "StateReadSession::new on uninitiated database, `load_file_states()` returns `BackendError { operation: "open_table" }`." This is a concrete test description with an exact variant assertion. However, it is not included in the Behavior Inventory (B1–B30) and has no formal BDD scenario in Section 3.

**Impact**: The test is described with sufficient specificity to implement. A developer reading the plan would know exactly what to write. The structural gap is that it's embedded as a note in the coverage table rather than standing as B31 with its own Given/When/Then.

### MINOR-3: B25 "When" clause tests compute_file_diff directly, not through run_index

**Location**: test-plan.md line 445
**Finding**: B25's "When" clause says "compute_file_diff is called (via the internal path through run_index or directly)." The "or" is ambiguous. The primary test path appears to call `compute_file_diff` directly (not through the `run_index` integration), meaning the integration wiring for `PathTraversal` through the full `run_index` call path is not explicitly tested. The variant IS tested with exact assertion (`Err(DiffError::PathTraversal { path })`), so this is a level-of-testing gap, not a coverage gap.

**Impact**: If `run_index` adds a sanitization step before calling `compute_file_diff` that swallows `PathTraversal`, B25 would still pass (it bypasses `run_index`). This is unlikely given the contract specifies `compute_file_diff` is the source of truth for path validation, but the ambiguity remains.

---

## LETHAL FINDINGS

None.

## MAJOR FINDINGS

None.

## MINOR FINDINGS (3/5 threshold)

1. test-plan.md:629 — Mutation table misattributes "skip deleted" to B28 instead of B30.
2. test-plan.md:726 — `StateLoadError::BackendError` described in coverage table but lacks formal B-number/BDD scenario.
3. test-plan.md:445 — B25 "When" clause ambiguous about test level (direct vs. through run_index).

## MANDATE

No mandatory changes required. The 3 MINOR findings are advisory:

1. **Recommended**: Fix mutation table line 629 to reference B30 instead of B28.
2. **Recommended**: Promote `StateLoadError::BackendError` test from coverage-table note (line 726) to a formal B31 BDD scenario with Given/When/Then.
3. **Recommended**: Clarify B25 "When" to either "compute_file_diff is called directly with malicious path" (if testing at diff level) or split into two scenarios (one direct, one through run_index).

None of these block approval.

---

**STATUS: APPROVED**
