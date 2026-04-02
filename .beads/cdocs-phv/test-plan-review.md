---
bead_id: cdocs-phv
reviewer: test-inquisitor
mode: plan-inquisition
date: 2026-04-02
status: APPROVED
revision: 2 (re-audit after fixes)
---

## VERDICT: APPROVED

0 LETHAL + 0 MAJOR + 2 MINOR.

All 12 findings from the prior review (1 LETHAL, 5 MAJOR, 6 MINOR) have been resolved.
Full six-axis re-audit confirms the plan is complete.

---

## Prior Finding Resolution

Every finding from revision 1 was verified against the revised `test-plan.md`.

| Prior Finding | Resolution | Evidence |
|---------------|------------|----------|
| LETHAL-01: `PreconditionViolation` no BDD | **FIXED** — B08 added | test-plan.md:31 (behavior table), :198-207 (BDD scenario), :721 (error variant table), :610 (mutation checkpoint) |
| MAJOR-01: Trophy summary contradicts table | **FIXED** — summary corrected | test-plan.md:6 — "14 unit (48%) / 13 integration (45%) / 2 e2e (7%)" matches per-behavior table count |
| MAJOR-02: B15 `reason` non-specific | **FIXED** — B20 now concrete | test-plan.md:339-340 — `where reason contains the string representation of the output directory path` |
| MAJOR-03: B16 `reason` non-specific | **FIXED** — B21 now concrete | test-plan.md:356-357 — `where reason contains "serialize" or the name of the serialization format` |
| MAJOR-04: `StateDb::new` missing 3 boundaries | **FIXED** — B05, B06, B07 added | test-plan.md:28-30 (behaviors), :168-196 (scenarios), :634-636 (matrix) |
| MAJOR-05: `PreconditionViolation` mutant survives | **FIXED** — B08 kills it | test-plan.md:610 (mutation table entry) |
| MINOR-01: B03 `path` field not asserted | **FIXED** | test-plan.md:152 — `where path matches the input path` |
| MINOR-02: B09 `run_id` not asserted | **FIXED** | test-plan.md:228 — `where run_id matches the batch's run_id` |
| MINOR-03: B11 `run_id` not asserted | **FIXED** | test-plan.md:280 — `where run_id matches the batch's run_id` |
| MINOR-04: Empty path undecided | **FIXED** — decided as accepted | test-plan.md:733-737 — "empty paths ARE accepted" |
| MINOR-05: `set_chunk_count(0)` not tested | **FIXED** — B16 added | test-plan.md:44, :105, :286-294 |
| MINOR-06: B27 Given imprecise | **FIXED** — mechanism specified | test-plan.md:427-431 — exact failure mechanism: malformed frontmatter with unclosed YAML fence |

---

## Axis 1 — Contract Parity

### Public Function Coverage

| # | `pub fn` in contract | BDD Scenarios | Status |
|---|----------------------|---------------|--------|
| 1 | `StateDb::new(output_dir)` | B01, B02, B03, B04, B05, B06, B07, B08 | COVERED |
| 2 | `StateDb::record_file_hash(…)` | B09, B10, B11, B25 | COVERED |
| 3 | `StateDb::set_document_count(count)` | B12, B13 | COVERED |
| 4 | `StateDb::set_chunk_count(count)` | B14, B15, B16 | COVERED |
| 5 | `StateDb::commit_changes()` | B17, B18, B19, B20, B21 | COVERED |
| 6 | `StateDb::is_committed()` | B22, B23 | COVERED |

6/6 public functions have ≥1 BDD scenario. **PASS.**

### Error Variant Coverage

| # | `StateError` Variant | Producing Scenario | Asserts Exact Variant + Fields? | Status |
|---|----------------------|--------------------|--------------------------------|--------|
| 1 | `AlreadyCommitted { run_id }` | B18 | Yes — `where run_id matches the batch` | PASS |
| 2 | `MutationAfterCommit { run_id }` | B10, B13, B15 | Yes — `where run_id matches the batch's run_id` in all three | PASS |
| 3 | `EmptyBatch { run_id }` | B19 | Yes — `where run_id matches the batch` | PASS |
| 4 | `DuplicateFilePath { path }` | B11 | Yes — `{ path: "src/guide.md" }` concrete | PASS |
| 5 | `PersistenceFailed { run_id, reason }` | B20 | Yes — `reason contains the string representation of the output directory path` | PASS |
| 6 | `OutputNotAccessible { path }` | B02, B03, B05, B06, B07 | Yes — all specify `where path matches/ends with` concrete value | PASS |
| 7 | `SerializationFailed { reason }` | B21 | Yes — `reason contains "serialize" or the name of the serialization format` | PASS |
| 8 | `PreconditionViolation { detail }` | B08 | Yes — `where detail contains "output lock not held"` and `detail contains the output directory path` | PASS |

All 8/8 error variants covered with concrete field assertions. **PASS.**

---

## Axis 2 — Assertion Sharpness

Every "Then:" clause across all 29 scenarios was read and evaluated.

### LETHAL assertions (is_ok / is_err)

Zero instances of `assert!(result.is_ok())` or `assert!(result.is_err())` in the plan. **PASS.**

### MAJOR assertions (vague inner values)

No `> 0`, `Some(_)`, or "non-empty" assertions found. All error fields specify concrete values or containment patterns:
- B20 `reason contains the string representation of the output directory path` — specific, verifiable
- B21 `reason contains "serialize" or the name of the serialization format` — specific, verifiable
- All `run_id` fields use `where run_id matches the batch's run_id` — concrete identity check
- All `path` fields use `where path matches the input` or concrete suffix — concrete

**PASS.**

---

## Axis 3 — Trophy Allocation

### Density Audit

| Metric | Count |
|--------|-------|
| Public functions (contract.md:199-270) | 6 |
| BDD scenarios | 29 |
| Proptest invariants | 6 |
| Fuzz targets | 2 |
| Kani harnesses | 3 |
| **Total planned test artifacts** | **40** |

**40 / 6 = 6.67×** — exceeds ≥5× threshold. **PASS.**

### Trophy Summary Verification

Line 6 claims: "14 unit (48%) / 13 integration (45%) / 2 e2e (7%)"

Manual count from trophy allocation table (lines 88-118):

| Layer | Behaviors | Count |
|-------|-----------|-------|
| Unit | B04, B09, B10, B11, B12, B13, B14, B15, B16, B18, B19, B22, B23, B25 | **14** |
| Integration | B01, B02, B03, B05, B06, B07, B08, B17, B20, B21, B24, B28, B29 | **13** |
| E2E | B26, B27 | **2** |

Summary matches table. **PASS.**

### Pure Function Proptest Coverage

| Pure Function | Non-trivial Input Space? | Proptest | Status |
|---------------|--------------------------|----------|--------|
| `record_file_hash` | Yes — arbitrary `(&str, &str)` | PROP-01, FUZZ-02 | COVERED |
| `set_document_count` | Yes — full `usize` range | PROP-02, PROP-05 | COVERED |
| `set_chunk_count` | Yes — full `usize` range | PROP-02, PROP-05 | COVERED |
| `is_committed` | No — returns `bool` from internal state | N/A | N/A |
| Serialization round-trip | Yes — arbitrary `StateBatch` | PROP-03 | COVERED |

**PASS.**

### Parser/Deserializer Fuzz Coverage

`StateBatch` deserialization is a parser. **FUZZ-01** covers it with arbitrary bytes + 8 corpus seeds including truncated JSON, empty bytes, negative counts, missing fields. **PASS.**

---

## Axis 4 — Boundary Completeness

### `StateDb::new(output_dir: &Path)`

| Boundary | Scenario | Status |
|----------|----------|--------|
| Valid dir | B01 | PASS |
| Non-existent dir | B02 | PASS |
| Read-only dir | B03 | PASS |
| Empty string path `""` | B05 | PASS |
| Path to regular file | B06 | PASS |
| Dangling symlink | B07 | PASS |

All 6 boundary classes covered. **PASS.**

### `record_file_hash(relative_path, content_hash)`

| Boundary | Scenario | Status |
|----------|----------|--------|
| Unique path (happy path) | B09 | PASS |
| Duplicate path | B11 | PASS |
| After commit | B10 | PASS |
| Empty `relative_path` `""` | Combinatorial matrix:646 + Open Q §2: accepted | PASS |
| Path traversal `../` | FUZZ-02 corpus + matrix:647 | PASS |
| Very long string (64KB+) | FUZZ-02 corpus | PASS |
| Unicode | FUZZ-02 corpus | PASS |

**PASS.**

### `set_document_count(count: usize)`

| Boundary | Scenario | Status |
|----------|----------|--------|
| count > 0 | B12 (42) | PASS |
| count == 1 (minimum meaningful) | B17 uses count == 1; matrix:654 | PASS |
| count == 0 | B19 (EmptyBatch fires at commit) | PASS |
| usize::MAX | matrix:657 | PASS |
| After commit | B13 | PASS |

**PASS.**

### `set_chunk_count(count: usize)`

| Boundary | Scenario | Status |
|----------|----------|--------|
| count > 0 | B14 (128) | PASS |
| count == 0 | B16 (explicitly tested, commit succeeds if docs > 0) | PASS |
| usize::MAX | matrix:666 | PASS |
| After commit | B15 | PASS |

**PASS.**

### `commit_changes()`

| Boundary | Scenario | Status |
|----------|----------|--------|
| Populated batch | B17 | PASS |
| Empty batch (document_count == 0) | B19 | PASS |
| Double commit | B18 | PASS |
| I/O failure | B20 | PASS |
| Serialization failure | B21 | PASS |

**PASS.**

### `is_committed()`

Pre/post commit: B22, B23. Trivial function. **PASS.**

### `Drop`

Uncommitted drop: B24. **PASS.**

---

## Axis 5 — Mutation Survivability (Thought Experiment)

20 mutations applied mentally. All mapped to catching tests in the plan's Section 7.

| # | Mutation | Catching Test | Caught? |
|---|----------|---------------|---------|
| 1 | Remove `committed = true` in `commit_changes` | B23 asserts `true` | YES |
| 2 | Remove `AlreadyCommitted` guard | B18 asserts exact variant | YES |
| 3 | Remove `MutationAfterCommit` in `record_file_hash` | B10 asserts exact variant + run_id | YES |
| 4 | Remove `MutationAfterCommit` in `set_document_count` | B13 asserts exact variant + run_id | YES |
| 5 | Remove `MutationAfterCommit` in `set_chunk_count` | B15 asserts exact variant + run_id | YES |
| 6 | Remove `DuplicateFilePath` check | B11 asserts exact variant + concrete path | YES |
| 7 | Remove `EmptyBatch` check | B19 asserts exact variant | YES |
| 8 | Remove file write in `commit_changes` | B17 reads file back + verifies content | YES |
| 9 | Make commit a no-op (return Ok without writing) | B17 verifies file existence + content | YES |
| 10 | `is_committed` always returns `true` | B22 asserts `false` | YES |
| 11 | `is_committed` always returns `false` | B23 asserts `true` | YES |
| 12 | Make `Drop` call `commit_changes` | B24 checks no file created | YES |
| 13 | Remove document_count assignment | B12 reads back via commit | YES |
| 14 | Remove chunk_count assignment | B14 reads back via commit | YES |
| 15 | Remove `OutputNotAccessible` check (missing dir) | B02 | YES |
| 16 | Remove `OutputNotAccessible` check (empty string) | B05 | YES |
| 17 | Remove `OutputNotAccessible` check (file-as-path) | B06 | YES |
| 18 | Remove `OutputNotAccessible` check (dangling symlink) | B07 | YES |
| 19 | Remove `PreconditionViolation` check | B08 asserts exact variant + detail | YES |
| 20 | Flip pipeline commit guard (commit on failure) | B27 checks no state file | YES |

**Kill rate: 20/20 = 100%.** **PASS.**

---

## Axis 6 — Holzmann Plan Audit

### Rule 1 — Keep it Linear

All 29 scenarios follow Given → When → Then. No nested conditionals. **PASS.**

### Rule 2 — Bound Every Loop

No loops in any scenario. Proptest strategies declare explicit ranges (e.g., `1..=1000`, `0..=100`). **PASS.**

### Rule 3 — Know What You Own

Filesystem-dependent tests (B01, B02, B03, B05, B06, B07, B08, B17, B20, B21, B24, B26, B27, B28, B29) reference temporary directories. `tempfile::tempdir()` is self-cleaning. **PASS.**

### Rule 4 — One Function, One Job

Each scenario tests one logical behavior. Prior concern about B20 (acceptance + rejection) has been resolved — B25 tests accumulation, B11 tests duplicate rejection, B09 tests happy path. **PASS.**

### Rule 5 — State Your Assumptions

All Given blocks state preconditions explicitly. B27 now specifies exact failure mechanism: "STEP 4 (TRANSFORM) is configured to fail by providing a source file with malformed frontmatter (e.g., unclosed YAML fence '---' without closing '---')." **PASS.**

### Rule 6 — Never Swallow Errors

No `let _ =` or `.ok()` patterns in any scenario. Every fallible call's result is asserted. **PASS.**

### Rule 7 — Narrow Your State

No shared mutable state. Each scenario constructs its own `StateDb`. **PASS.**

### Rule 8 — Surface Your Side Effects

Filesystem operations explicitly named in Given blocks. "a writable temporary directory," "a temporary directory with mode 0o444," "a symlink pointing to a non-existent target" — all self-documenting. **PASS.**

### Rule 9 — One Layer of Magic

No deep helper chains. Each scenario is self-contained. **PASS.**

### Rule 10 — Warnings Are Errors

N/A for plan review (no code yet). **PASS.**

---

## LETHAL FINDINGS (0)

None.

---

## MAJOR FINDINGS (0)

None.

---

## MINOR FINDINGS (2 / 5 threshold)

### MINOR-01: Combinatorial matrix "committed drop" scenario has no formal BDD entry

- **Location:** test-plan.md:690 — matrix lists `committed drop | already committed | state file remains intact | integration`
- **Defect:** The combinatorial coverage matrix identifies a scenario where a committed `StateDb` is dropped and the test verifies the state file persists. No B## entry exists for this. If a developer implements `Drop` to delete the state file on drop, B17 would pass (file read before Drop runs) but the file would disappear after the value drops.
- **Mitigation:** B26 (E2E) effectively covers this — `run_index` commits, then `StateDb` drops on function return, and the test reads the output directory after `run_index` returns. The mutant IS caught. The gap is formal: the combinatorial matrix identifies a test case without a dedicated BDD scenario.

### MINOR-02: Combinatorial matrix "empty source" scenario has no formal BDD entry

- **Location:** test-plan.md:698 — matrix lists `empty source | dir with no .md files | Err(...) from discover, no state file | e2e`
- **Defect:** This tests a failure path through STEP 1 (DISCOVER) — before any state is accumulated — which is a different code path than B27's STEP 4 failure (after partial state accumulation). B27 proves "failure → no state" for one failure point. An empty source tests an earlier failure point.
- **Mitigation:** B27's invariant ("any pipeline stage returns Err → no state committed") is logically comprehensive. The empty source case is a distinct failure mode but is covered by the same invariant.

---

## SEVERITY SUMMARY

| Severity | Count | Threshold | Exceeded? |
|----------|-------|-----------|-----------|
| LETHAL | 0 | Any → REJECT | No |
| MAJOR | 0 | ≥3 → REJECT | No |
| MINOR | 2 | ≥5 → REJECT | No |

**0 LETHAL + 0 MAJOR + 2 MINOR = APPROVED.**

---

## MANDATE

No mandatory fixes required. The plan passes all six axes.

### Recommended (does not block APPROVED)

1. Consider adding a formal BDD scenario for "committed drop — file persists after Drop" to eliminate the MINOR-01 gap. This can be a simple test: commit, explicitly `drop(state_db)`, verify file still exists.

2. Consider adding a formal BDD scenario for "empty source directory — STEP 1 failure propagates without state commit" to test the early-failure code path explicitly.

3. B21 serialization failure injection mechanism is implementation-dependent (Open Questions §5). The implementation bead must resolve this with a concrete test hook or test-only constructor before tests are written.
