# Test Plan Review: cdocs-pxx

**Reviewer**: Test Inquisitor (Mode 1 — Plan Inquisition)
**Date**: 2026-04-02 (Revision 2 — re-audit after MAJOR fixes)
**Contract**: `contract.md` (524 lines)
**Test Plan**: `test-plan.md` (1239 lines, Revision 2)
**References**: Holzmann 12 Rules for Tests

---

## VERDICT: APPROVED

---

## Previous Defect Verification

All 8 defects from the REJECTED Revision 1 review have been verified as fixed:

| Defect | Severity | Status | Evidence |
|--------|----------|--------|----------|
| MAJOR-1: Non-concrete `reason` assertions | MAJOR | **FIXED** | Behaviors 2, 3, 6 now use `reason contains "<concrete_substring>"` patterns. See Behaviors 2 (line 197), 3 (line 209), 6 (line 248). |
| MAJOR-2: PayloadTooLarge 1/5 vecs | MAJOR | **FIXED** | Behaviors 24–28 now cover all 5 payload vecs (analysis, transform, chunk, scrape, snapshots) with identical concrete assertions per table. See lines 479–555. |
| MAJOR-3: Unchanged-row skip uncatchable | MAJOR | **FIXED** | `should_skip_write` extracted as pure public function (Behaviors 40–41). Proptest 6 covers all byte patterns. Accepted survivor documented explicitly for integration layer only. See lines 720–766, 969–983. |
| MINOR-1: Empty path for open | MINOR | **FIXED** | Behavior 4 added: `StateDb::open(Path::new(""))` → `Err(DatabaseOpen { path: "", reason: contains "file" })`. See lines 216–227. |
| MINOR-2: Long source_path boundary | MINOR | **FIXED** | Behavior 49 added: 4096-char source_path characterization test. See lines 871–887. |
| MINOR-3: Incomplete BDD preconditions | MINOR | **FIXED** | 23 BDD scenarios now include explicit "And all other StateChanges fields are valid per `make_minimal_valid_state_changes()`" clauses. Verified via grep. |
| MINOR-4: Side-effectful helper name | MINOR | **FIXED** | Renamed to `create_temp_state_db()` at line 1172 with explicit documentation: "Name advertises the side effect: creates filesystem state." |
| MINOR-5: Deferred StateReadSession stubs | MINOR | **FIXED** | Explicit DEFERRED entries for all 7 methods in Behavior Inventory (lines 100–110) + error variant table (line 1161). |

---

## Axis 1 — Contract Parity

### PASS — Error Variant Completeness

Every `CommitError` variant in the contract (12 variants) has a BDD scenario asserting the exact variant with field-level assertions:

| Variant | Scenario(s) | Asserted? |
|---------|-------------|-----------|
| `ZeroHashKey { table, index }` | Behaviors 7–12 | YES — concrete table name + index per vec |
| `EmptyStringKey { table, index }` | Behaviors 13–16 | YES — concrete table name + index, whitespace tested |
| `DuplicateStateKey { table, key }` | Behaviors 17–18 | YES — concrete key value |
| `MissingReference { table, field, hash_hex, payload_table }` | Behaviors 19–22 | YES — concrete field + hash + table names |
| `PayloadTooLarge { table, size, max }` | Behaviors 24–28 | YES — concrete size (52428801) + max (52428800) per vec |
| `DatabaseOpen { path, reason }` | Behaviors 2, 4 | YES — path matched, reason substring asserted |
| `TableInit { reason }` | Behavior 3 | YES — reason contains "table" |
| `ReadTransaction { reason }` | Behavior 6 | YES — reason contains "read" OR "transaction" |
| `WriteTransaction { reason }` | Behavior 46 | YES — reason contains "write" OR "transaction" |
| `WriteFailed { table, reason }` | Behavior 47 | YES — table identified, reason substring asserted |
| `CommitFailed { reason }` | Behavior 48 | YES — reason contains "commit" OR "transaction" |
| `ReadFailed { table, reason }` | DEFERRED | Acknowledged — StateReadSession bead per contract non-goal line 523 |

### PASS — Public Function Coverage

**In-scope functions (3):** All have BDD scenarios:

| Function | Scenarios |
|----------|-----------|
| `StateDb::open` | Behaviors 1–4 (valid path, invalid path, table init failure, empty path) |
| `StateDb::begin_read` | Behaviors 5–6 (success, failure) |
| `StateDb::commit_changes` | Behaviors 7–49 (precondition violations, successful writes, structural guarantees, transaction errors, boundaries) |

**Deferred functions (7):** All StateReadSession methods have explicit DEFERRED entries in Behavior Inventory (lines 104–110), with rationale citing contract non-goals line 523. Error variant `ReadFailed` is also deferred in section 8c (line 1161).

---

## Axis 2 — Assertion Sharpness

### PASS — No banned assertions

Every "Then:" block uses concrete values:
- `Ok(StateDb)`, `Ok(StateReadSession)`, `Ok(())` — concrete success types
- `Err(CommitError::ExactVariant { field: concrete_value })` — exact variant + fields
- Exact database read-back: `"src/main.rs" → file_state_raw_a bytes` — concrete key-value pairs
- Exact counts: `analysis_outputs.len() == 2` — concrete cardinality
- `should_skip_write` returns `true` / `false` — concrete boolean

No `is_ok()`, no `is_err()`, no bare `> 0`, no unbound `Some(_)`.

All `reason` fields in error assertions use concrete `contains` patterns:
- Behavior 2: `reason contains "nonexistent_root_dir"` (line 197)
- Behavior 3: `reason contains "table"` (line 209)
- Behavior 4: `reason contains "No such file" OR reason contains "file" OR reason contains "directory"` (line 224)
- Behavior 6: `reason contains "read" OR reason contains "transaction"` (line 248)
- Behavior 46: `reason contains "write" OR reason contains "transaction"` (line 836)
- Behavior 47: `reason contains "insert" OR reason contains "write" OR reason contains "I/O"` (line 851)
- Behavior 48: `reason contains "commit" OR reason contains "transaction"` (line 865)

---

## Axis 3 — Trophy Allocation

### PASS — Density ratio

| Metric | Count |
|--------|-------|
| In-scope `pub fn` | 3 |
| Total test items (BDD + proptest + Kani) | 57 (49 BDD + 6 proptest + 2 Kani) |
| Density ratio | 19.0x (target ≥ 5x) |

Even counting all 10 declared `pub fn` (3 in-scope + 7 deferred): 57 / 10 = 5.7x — above threshold.

### PASS — Proptest invariants for pure functions

All pure functions with non-trivial input spaces have proptest invariants:

| Pure Function | Proptest |
|---------------|----------|
| Zero-hash scan | Proptest 1 |
| Duplicate detection | Proptest 2 |
| Reference integrity | Proptest 3 |
| Dedup (last-write-wins) | Proptest 4 |
| Atomicity guarantee | Proptest 5 |
| `should_skip_write` byte comparison | Proptest 6 |

All 6 invariants specify: function under test, invariant, strategy, anti-invariant.

### PASS — No fuzz targets justified

No parser/deserializer boundary in this bead. All inputs are strongly-typed Rust structs. Fuzz exemption is justified.

### PASS — Trophy ratio rationale

25 integration / 24 unit (per section 2 trophy table, 49 total). Integration-heavy is correct for an I/O-bound database layer where every meaningful behavior requires a real `redb::Database` to prove ACID semantics. Unit tests cover the pure validation phase.

---

## Axis 4 — Boundary Completeness

### PASS — commit_changes boundaries

| Boundary | Tested? | Where |
|----------|---------|-------|
| Zero hash `[0u8; 32]` in all 5 payload vecs | YES | Behaviors 7–12 |
| Non-zero hash (valid) | YES | Behavior 23 |
| Empty string `""` | YES | Behaviors 13, 14 |
| Whitespace-only `"   "` `"\t\n"` | YES | Behaviors 15, 16 |
| Duplicate key | YES | Behaviors 17, 18 |
| Empty batch (all vecs empty) | YES | Behavior 44 |
| Single entry per vec | YES | Behaviors 29–38 |
| Multiple entries + mixed | YES | Behavior 45 |
| Payload exactly at MAX (50 MiB) | YES | Matrix row line 1109 |
| Payload MAX + 1 (50 MiB + 1) in all 5 vecs | YES | Behaviors 24–28 |
| Payload 0 bytes | YES | Matrix row line 1115 |
| Non-existent delete target | YES | Behaviors 30, 35, 38 |
| Empty path for open | YES | Behavior 4 |
| Long source_path (4096 chars) | YES | Behavior 49 |
| should_skip_write: empty slices, different lengths | YES | Matrix rows lines 1116–1120 |

### PASS — open/begin_read boundaries

| Boundary | Tested? | Where |
|----------|---------|-------|
| Valid path | YES | Behavior 1 |
| Invalid parent directory | YES | Behavior 2 |
| Empty path | YES | Behavior 4 |
| Corrupted file / table init failure | YES | Behavior 3 |
| Read session success | YES | Behavior 5 |
| Read session failure | YES | Behavior 6 |

---

## Axis 5 — Mutation Survivability

### Mutation analysis — all critical mutations caught

| Mutation | Caught? | By Test |
|----------|---------|---------|
| Remove zero-hash check per vec (5 vecs) | YES | Behaviors 7–12 (one per vec) |
| `trim().is_empty()` → `is_empty()` | YES | Behaviors 15, 16 |
| Remove duplicate check files/urls | YES | Behaviors 17, 18 |
| Remove ref integrity per hash type (4 types) | YES | Behaviors 19–22 |
| Reject zero hashes (remove exemption) | YES | Behavior 23 |
| `>` → `>=` in size check | YES | Behaviors 24–28 (exactly MAX+1) + matrix (exactly MAX passes) |
| Remove PayloadTooLarge check per vec (5 vecs) | YES | Behaviors 24–28 (one per vec) |
| Skip file_state write loop | YES | Behavior 29 |
| Skip deleted_files loop | YES | Behavior 30 |
| Skip any table write in mixed batch | YES | Behavior 45 (checks all 7 tables) |
| Remove dedup (store all entries) | YES | Behavior 39 (checks exact count == 2) |
| Remove idempotent delete (error on missing) | YES | Behaviors 30, 35, 38 |
| Remove rollback on validation fail | YES | Behavior 43 (checks DB state unchanged) |
| Remove should_skip_write (always true) | YES | Behavior 41 (asserts false for differing bytes) |
| Remove should_skip_write (always false) | YES | Behavior 40 (asserts true for identical bytes) |
| Skip empty-batch early return | YES | Behavior 44 |
| Swap error variant names | YES | Every scenario checks exact variant + fields |

### Accepted Survivor — documented

| Mutation | Layer | Caught by | Justification |
|----------|-------|-----------|---------------|
| Remove unchanged-row skip (always rewrite) | Integration (Behavior 42) | Unit layer (Behaviors 40–41) | Integration test cannot distinguish "wrote identical bytes" from "skipped write" by observing DB state. Mutation IS caught at unit layer by `should_skip_write` pure-function tests. Documented transparently in plan lines 758–766. |

This is a well-managed accepted survivor. The mutation is killed at the unit layer; the integration layer provides a correctness backstop.

---

## Axis 6 — Holzmann Plan Audit

### PASS — Rule 1 (Linear)

All BDD scenarios follow straight-line Given → When → Then. No nested conditionals.

### PASS — Rule 2 (Bound Every Loop)

No loops described in any test body. Proptest strategies describe data generation (framework-managed), not test-body loops.

### PASS — Rule 3 (Know What You Own)

Section 9 specifies `create_temp_state_db() -> (StateDb, TempDir)`. TempDir keeps directory alive. Section 9 explicitly states: "Every test creates its OWN StateDb instance (no shared database)." Resource cleanup is automatic via `tempfile::TempDir`.

### PASS — Rule 4 (One Function, One Job)

Each behavior maps to exactly one test function. Test names describe the single behavior.

### PASS — Rule 5 (State Your Assumptions)

All validation BDD scenarios now include explicit "And all other StateChanges fields are valid per `make_minimal_valid_state_changes()`" clauses (23 occurrences verified). Integration scenarios specify exact pre-existing database state.

### PASS — Rule 6 (Never Swallow Errors)

No code to review — plan does not describe `let _ =` or `.ok()` patterns.

### PASS — Rule 7 (Narrow Your State)

Section 9: "No test depends on execution order. No test modifies global state. Every test uses `tempfile::TempDir` for automatic cleanup."

### PASS — Rule 8 (Surface Your Side Effects)

Helper renamed to `create_temp_state_db()` — name explicitly advertises filesystem side effect.

### PASS — Rule 9 (One Layer of Magic)

All helpers are at most 1 call deep from the test body.

### N/A — Rule 10 (Warnings Are Errors)

No code to lint in plan review.

---

## Severity Tally

| Severity | Count | Threshold | Triggers Rejection? |
|----------|-------|-----------|---------------------|
| LETHAL | 0 | Any 1 | No |
| MAJOR | 0 | ≥ 3 | No |
| MINOR | 3 | ≥ 5 | No |

0 LETHAL + 0 MAJOR + 3 MINOR = **APPROVED**

---

## LETHAL FINDINGS

None.

---

## MAJOR FINDINGS (0)

None. All 3 previous MAJOR findings verified as fixed.

---

## MINOR FINDINGS (3/5 threshold)

### MINOR-1: Behavior 45 incorrect expected outcome for analysis_outputs (Axis 5)

- `test-plan.md:812` — `deleted_snapshots = [hash_old]` deletes from `snapshots` table, NOT from `analysis_outputs`
- `test-plan.md:819` — Then clause claims `analysis_outputs contains hash_new → new_analysis_bytes (NOT hash_old)` — the `(NOT hash_old)` is **incorrect**
- After commit, `analysis_outputs` should contain BOTH `hash_old → old_bytes` (unchanged, no delete targets this table) AND `hash_new → new_analysis_bytes`
- If implemented as written, the test would FAIL because hash_old is still present
- Correct assertion: `analysis_outputs contains hash_new → new_analysis_bytes AND hash_old → old_bytes (unchanged)`
- This also means the test currently fails to verify an important invariant: pre-existing unrelated payload entries survive the commit

### MINOR-2: No integration test for "update existing key with different value" upsert path (Axis 4)

- The contract specifies upsert semantics: "Keys not already in the DB are inserts; keys already present are updates" (line 63–64)
- The combinatorial matrix (line 1127) lists "Update existing file state | existing source_path, new FileStateRaw | Ok(()), row updated to new bytes" but no BDD scenario implements it
- Existing tests cover: insert into empty table (Behavior 29), unchanged bytes skip (Behavior 42), mixed insert+delete (Behavior 45)
- Missing: pre-populate file_state with key K → state_v1, commit updated_files with K → state_v2 (different bytes), verify K → state_v2 after commit
- Mutation "skip insert for existing keys" partially survives: new keys are tested (Behavior 29), unchanged keys are tested (Behavior 42), but the update-existing-with-new-value path has no integration witness

### MINOR-3: Summary table counts stale from previous revision (Axis 3)

- `test-plan.md:13` — Summary claims "BDD scenarios: 54" but section 3 has exactly 49 BDD scenario blocks
- `test-plan.md:14` — Summary claims "Integration: 29 (62%), Unit: 18 (38%)" but section 2 trophy table contains 25 integration + 24 unit = 49 entries
- 29 + 18 = 47 ≠ 49 (actual trophy entries)
- 54 ≠ 49 (actual BDD scenario blocks)
- Numbers appear stale from a pre-revision draft. Section content is correct; summary table is not.

---

## MANDATE

No mandatory changes required for approval. The 3 MINOR findings should be addressed before implementation:

1. **Fix Behavior 45 analysis_outputs assertion**: Change `(NOT hash_old)` to `(AND hash_old → old_bytes unchanged)` to reflect correct expected state. This also strengthens the test by verifying the "unrelated entries survive" invariant.

2. **Consider adding an update-existing-key integration test**: Either add a dedicated BDD scenario for "update existing file with different bytes" or verify this path through an extension to Behavior 45 (pre-populate with "new.rs" → state_v1, then update to state_v2 in the same commit).

3. **Update summary table**: Align counts with actual section content (49 BDD scenarios, 25 integration, 24 unit).

These are non-blocking. The plan is APPROVED as-is.

---

## Positive Observations

The revised plan demonstrates exceptional quality across all axes:

- **All 3 MAJOR defects from Revision 1 are thoroughly fixed.** Each fix goes beyond the minimum: PayloadTooLarge now covers all 5 vecs with identical assertion precision; `should_skip_write` is extracted with unit tests AND proptest AND transparent documentation of the accepted survivor; `reason` fields use concrete `contains` patterns throughout.

- **Error variant coverage is exhaustive.** All 12 `CommitError` variants have exact-variant assertions with concrete field values. This is rare.

- **Proptest invariants are comprehensive.** 6 invariants covering every pure function with non-trivial input space, each with strategy, anti-invariant, and property specification.

- **Mutation checkpoint mapping is rigorous.** 26+ mutations explicitly mapped to catching tests. The single accepted survivor is transparently documented with three-layer mitigation (unit + proptest + integration backstop).

- **Precondition specification is now complete.** 23 BDD scenarios explicitly state "And all other StateChanges fields are valid per `make_minimal_valid_state_changes()`", eliminating the ordering-dependency risk from Revision 1.

- **Test infrastructure is clean.** `create_temp_state_db()` advertises side effects. Per-test databases. No shared state. `make_minimal_valid_state_changes()` provides a documented baseline.

- **The `should_skip_write` extraction is a textbook example** of the "extract pure function for testability" pattern. The three-layer approach (unit → proptest → integration) with explicit documentation of which layer catches which mutation is exemplary.
