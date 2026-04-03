# Test Plan Review — cdocs-824

**Reviewer:** Test Inquisitor (Mode 1 — Plan Inquisition)
**Date:** 2026-04-03
**Bead:** cdocs-824 — "data: add zero-copy state dependencies to centralized-docs crate"
**Inputs:** `contract.md` + `test-plan.md`
**Scope:** Dependency-only `Cargo.toml` change (add `bytemuck` with `"derive"` feature)

---

## VERDICT: APPROVED

**Severity tally:** 0 LETHAL / 1 MAJOR / 4 MINOR → threshold: APPROVED

---

## Axis 1 — Contract Parity: PASS

### Postcondition Coverage

| Postcondition | Covered By | Status |
|---------------|-----------|--------|
| POST-1: `bytemuck` in `[dependencies]` | B1, B3, B4, B8 | ✅ |
| POST-2: `bytemuck` in `[dev-dependencies]` | B8 | ✅ |
| POST-3: `rkyv` unchanged | B9 | ✅ |
| POST-4: All pre-existing deps unchanged | B6 | ✅ |
| POST-5: `cargo check` exits 0 | B1 | ✅ |
| POST-6: `cargo check --all-features` exits 0 | B2 | ✅ |
| POST-7: `bytemuck` in resolved dep tree | B3 | ✅ |
| POST-8: No new `unsafe` code | B7 | ✅ |

### Invariant Coverage

| Invariant | Covered By | Status |
|-----------|-----------|--------|
| INV-1: Additive-only diff | B5 | ✅ |
| INV-2: No `.rs` file changes | **No explicit scenario** | ⚠️ MINOR-3 |
| INV-3: Workspace lint compliance | B7 | ✅ |
| INV-4: Dependency graph acyclicity | Combinatorial matrix (p. 342) | ✅ |
| INV-5: Feature minimality | B4 (exact `["derive"]`) | ✅ |

### Error Taxonomy Coverage

| Error | Scenario | Status |
|-------|----------|--------|
| `CompileError::DependencyResolution` | B1 error variant | ✅ |
| `CompileError::FeatureNotFound` | B4 error variant (partial — misspelling, not version mismatch) | ⚠️ MINOR-4a |
| `CompileError::WorkspaceLintViolation` | B7 (happy path only; dep-only change cannot trigger violation) | ✅ justified |
| `CompileError::ExistingCodeBreakage` | B10 error variant | ✅ |
| `CIError::LockfileConflict` | **No scenario** | ⚠️ MINOR-4b |

**Result:** All postconditions covered. No `pub fn` gap (no functions exist). All error variants have scenarios with concrete assertions. Two MINOR gaps noted.

---

## Axis 2 — Assertion Sharpness: FAIL (1 MAJOR)

Every `Then:` clause was inspected for vagueness.

| Scenario | Then-Assertion | Verdict |
|----------|---------------|---------|
| B1 happy | Exit code 0, stderr no errors | ✅ concrete |
| B1 error | Exit code non-zero, stderr contains `"failed to select a version for the requirement"` | ✅ concrete |
| B2 happy | Exit code 0 | ✅ concrete |
| **B2 error** | **Exit code non-zero, stderr contains "a feature conflict message"** | **❌ MAJOR-1** |
| B3 happy | Stdout contains `"bytemuck v1."` | ✅ concrete |
| B3 error | Exit code non-zero, stderr contains `"did not find any packages matching"` | ✅ concrete |
| B4 happy | Features field contains exactly `["derive"]` | ✅ concrete |
| B4 error | Features field does NOT contain `"derive"` (contains `"derives"`) | ✅ concrete |
| B5 happy | No `-` lines in diff | ✅ concrete |
| B5 error | Diff contains `-` line with `rkyv` | ✅ concrete |
| B6 happy | Every dep matches exact version/feature strings | ✅ concrete |
| B7 happy | Exit code 0 | ✅ concrete |
| B8 happy | Both sections contain `{ version = "1", features = ["derive"] }` | ✅ concrete |
| B8 error | `"bytemuck"` absent from `[dev-dependencies]` | ✅ concrete |
| B9 happy | Value is exactly `{ version = "0.8", features = ["std", "bytecheck"] }` | ✅ concrete |
| B9 error | Features array is `["std"]` not `["std", "bytecheck"]` | ✅ concrete |
| B10 happy | Exit code 0, all tests passing | ✅ concrete |
| B10 error | stderr contains `"duplicate definition"` or `"name conflict"` | ✅ concrete |

**MAJOR-1** (test-plan.md:107): B2 error variant asserts "stderr contains a feature conflict message" — this is a category description, not a concrete expected string. The test writer has discretion to match any substring, which could be trivially permissive (e.g., matching `"error"`). **Required fix:** specify the exact cargo error substring, e.g., `"feature 'derive' is required by"` or the actual Cargo feature conflict format.

**No `is_ok()` / `is_err()` violations found.** All assertions specify concrete values.

---

## Axis 3 — Trophy Allocation: PASS

| Metric | Value | Target | Verdict |
|--------|-------|--------|---------|
| Public functions introduced | 0 | — | N/A |
| Planned scenarios | 10 (7 integration / 2 static / 1 E2E) | — | — |
| Test-to-function ratio | ∞ (10/0) | ≥ 5× | ✅ trivially satisfied |
| Proptest invariants | 0 | — | ✅ justified (no pure functions) |
| Fuzz targets | 0 | — | ✅ justified (no parsers/deserializers) |
| Unit tests | 0 (0%) | 60% standard | ⚠️ deviation |

**Deviation rationale (accepted):** This bead introduces zero runtime behavior. No functions, no types, no trait implementations. The entire verification surface is at the build-system level. The adapted trophy shape (70% integration / 20% static / 10% E2E / 0% unit) is correct for a manifest-only change. Unit tests would be testing nothing — there is no pure function to exercise.

**Existing fuzz targets noted** (8 listed, regression-gated by B10). Future beads implementing Pod/Archive derives MUST add `fuzz_bytemuck_cast_slice` and `fuzz_rkyv_access` — the plan correctly calls this out.

---

## Axis 4 — Boundary Completeness: PASS (with notes)

This bead has no arithmetic boundaries, no min/max values, no overflow concerns. The "input space" is a single TOML entry. Boundaries that DO exist:

| Boundary | Scenario | Status |
|----------|----------|--------|
| Feature list empty `[]` | Mutation table: "Remove 'derive'" → B4 | ✅ |
| Feature misspelled | B4 error variant | ✅ |
| Missing from one section | B8 error variant | ✅ |
| Wrong section (`build-dependencies`) | Mutation table: row 7 → B8 | ✅ |
| Version changed to `"2"` | Mutation table: row 4 → B3 | ✅ |
| Name misspelled | Mutation table: row 6 → B1 | ✅ |
| Pre-existing dep accidentally removed | B5 + B9 | ✅ |
| rkyv features accidentally changed | B9 error variant | ✅ |

**Not tested but acceptable:**
- `optional = true` added to bytemuck entry → B8 checks exact TOML value, would catch extra key ✅
- Version `"1.0.0"` vs `"1"` → semantically equivalent; B8 checks exact TOML string, which is correct for manifest verification

No boundary gaps severe enough for MAJOR.

---

## Axis 5 — Mutation Survivability: PASS

### Manifest Mutation Kill Analysis

| # | Mutation | Killer | Verified |
|---|----------|--------|----------|
| M1 | Remove `"derive"` from features | B4: `cargo metadata` shows empty features | ✅ |
| M2 | Remove `[dev-dependencies]` entry | B8: TOML parse finds missing key | ✅ |
| M3 | Remove `[dependencies]` entry | B3: `cargo tree` fails; B8: missing key | ✅ |
| M4 | Change version to `"2"` | B3: stdout shows `v2.` not `v1.` | ✅ |
| M5 | Remove `rkyv` line | B9: missing key; B1: `cargo check` fails | ✅ |
| M6 | Misspell `bytemuck` as `bytemuc` | B1: cargo check fails (registry lookup) | ✅ |
| M7 | Add to `[build-dependencies]` | B8: absent from `[dependencies]` and `[dev-dependencies]` | ✅ |

**Kill rate: 7/7 = 100%** for manifest mutations.

### Thought Experiment: Unlisted Mutations

| Unlisted Mutation | Caught By | Verdict |
|-------------------|-----------|---------|
| `optional = true` added | B8: TOML value mismatch (extra key) | ✅ |
| Extra feature `"extern_crate_alloc"` | B4: features ≠ exactly `["derive"]` | ✅ |
| Reorder TOML entries | B5: diff shows `-` lines (line moved = removal) | ✅ |
| `version = "1.0.0"` (semantically same) | B8: TOML string `"1.0.0"` ≠ `"1"` | ✅ (false positive acceptable for manifest exactness) |

**No surviving mutants identified.** All planned mutations are killed.

---

## Axis 6 — Holzmann Plan Audit: PASS (1 MINOR)

| Rule | Assessment | Finding |
|------|-----------|---------|
| R1: Keep it Linear | Each scenario: single Given → When → Then. No nesting. | ✅ |
| R2: Bound Every Loop | No loops in any scenario. | ✅ |
| R3: Know What You Own | Tests run cargo commands (OS processes) and parse TOML files. No persistent resources to clean up. | ✅ |
| R4: One Function, One Job | Each scenario tests one behavior. Names are descriptive. | ✅ |
| R5: State Your Assumptions | All Given blocks are explicit **except** B5. | ⚠️ MINOR-2 |
| R6: Never Swallow Errors | No `is_ok()`, `is_err()`, `let _ =`, or `.ok()` in any assertion. | ✅ |
| R7: Narrow Your State | Each scenario is self-contained. No shared mutable state. | ✅ |
| R8: Surface Side Effects | Implementation notes clearly describe cargo commands and file parsing. | ✅ |
| R9: One Layer of Magic | Tests are direct command invocations or TOML parsing. Max 1 helper layer. | ✅ |
| R10: Warnings Are Errors | B7 covers workspace lint enforcement. | ✅ |

**MINOR-2** (test-plan.md:151): B5 Given-clause says "The pre-change Cargo.toml is stored (or available via git)" — the "or" is ambiguous. Holzmann Rule 5 requires explicit, checkable preconditions. The test writer needs a deterministic mechanism: either `git show HEAD:Cargo.toml` (if uncommitted) or a snapshot file. The plan should specify which.

---

## LETHAL FINDINGS

None.

---

## MAJOR FINDINGS (1)

### MAJOR-1: Vague assertion in B2 error variant
- **Location:** test-plan.md:107
- **Axis:** Axis 2 — Assertion Sharpness
- **Detail:** B2 error scenario Then-clause asserts "stderr contains a feature conflict message." This is a category, not a concrete expected value. The test writer could match `"error"` or `"conflict"` and the assertion would be trivially satisfied by any compilation failure.
- **Required fix:** Replace with exact Cargo error substring, e.g., `stderr contains "feature 'derive' is required by"` or equivalent real Cargo output for feature conflicts. If the exact format is unknown at plan time, state: "stderr contains the substring output by `cargo check` when two dependencies require incompatible features of the same crate — exact string to be captured from a reproduction build."

---

## MINOR FINDINGS (4 / 5 threshold)

### MINOR-1: INV-2 not explicitly verified
- **Location:** contract.md:62 (INV-2) vs test-plan.md
- **Detail:** INV-2 states "No `.rs` files are modified." No scenario explicitly verifies this. B5 checks the diff to `Cargo.toml` is additive but does not assert the absence of `.rs` file changes in the diff. B10 (existing tests pass) provides indirect coverage but would not catch additive safe changes to `.rs` files.
- **Recommendation:** B5 should be extended to assert: "The diff contains no `.rs` file paths" or "All changed file paths in the diff end with `Cargo.toml`."

### MINOR-2: B5 precondition mechanism ambiguous
- **Location:** test-plan.md:151
- **Detail:** B5 Given-clause says "The pre-change Cargo.toml is stored (or available via git)." The "or" violates Holzmann Rule 5 — preconditions must be explicit and deterministic. The test writer needs one specified mechanism.
- **Recommendation:** State explicitly: "The pre-change Cargo.toml content is obtained via `git show HEAD:path/to/Cargo.toml`" (if uncommitted change) or "a snapshot file at `tests/fixtures/Cargo.toml.pre-change`."

### MINOR-3: CompileError::FeatureNotFound coverage gap
- **Location:** contract.md:74 vs test-plan.md:138-142
- **Detail:** The contract's Error Taxonomy lists `CompileError::FeatureNotFound` ("bytemuck version doesn't support the 'derive' feature"). B4's error variant tests a MISSPELLED feature name, not a version that lacks the feature. These are different failure modes.
- **Mitigation:** Extremely unlikely for `bytemuck` v1 (derive has existed since early versions). The gap is real but low-risk. If the test writer can construct this scenario (pinning to a bytemuck version that predates derive), it should be added.

### MINOR-4: CIError::LockfileConflict has no scenario
- **Location:** contract.md:77 vs test-plan.md
- **Detail:** The contract lists `CIError::LockfileConflict` in the Error Taxonomy. No scenario in the test plan addresses this condition. This is a CI/merge workflow concern, not a unit-test concern, but it is listed in the contract as a known error.
- **Mitigation:** Lockfile conflicts are detected by `git` and CI pipelines, not by Rust tests. The plan's exit criteria checklist item "Every error variant in the Error enum has an explicit test scenario" is technically inaccurate — this variant has none. Recommend adding a note: "CIError::LockfileConflict is out of scope for the test plan; it is a CI workflow concern addressed by `cargo update -p bytemuck`."

---

## MANDATE

**STATUS: APPROVED.** No resubmission required.

Before test implementation, the test writer SHOULD address:
1. **MAJOR-1:** Specify the exact Cargo error substring for B2's feature conflict scenario.
2. **MINOR-1:** Extend B5 to assert no `.rs` files in the diff.
3. **MINOR-2:** Specify the deterministic mechanism for obtaining pre-change Cargo.toml in B5.

These are recommendations, not blockers. The plan is sound for a dependency-only bead.

---

## Appendix: Grounding Verification

The following contract assumptions were verified against the actual workspace:

| Assumption | Verified | Source |
|-----------|----------|--------|
| `rkyv` v0.8 in `[dependencies]` | ✅ | centralized-docs/Cargo.toml:97 |
| `rkyv` v0.8 in `[dev-dependencies]` | ✅ | centralized-docs/Cargo.toml:118 |
| `bytemuck` NOT present | ✅ | Absent from entire Cargo.toml |
| `redb = "2"` present | ✅ | centralized-docs/Cargo.toml:89 |
| `sha2 = "0.10"` present | ✅ | centralized-docs/Cargo.toml:92 |
| `lru = "0.16.3"` present | ✅ | centralized-docs/Cargo.toml:93 |
| `parking_lot = "0.12.5"` present | ✅ | centralized-docs/Cargo.toml:94 |
| `rayon = "1.11.0"` present | ✅ | centralized-docs/Cargo.toml:86 |
| `unsafe_code = "forbid"` at workspace level | ✅ | Cargo.toml:15 (workspace root) |
