# Test Plan Review: cdocs-4s3 (Re-Audit)

**Reviewer:** Test Inquisitor (Mode 1 — Plan Inquisition)
**Date:** 2026-04-02
**Inputs:** `contract.md` (285 lines), `test-plan.md` (1146 lines, REVISED)
**Previous verdict:** REJECTED (4 MAJOR, 6 MINOR)

---

## VERDICT: APPROVED

0 LETHAL / 0 MAJOR / 2 MINOR — below all rejection thresholds.

---

## Previous Defect Verification

All 10 defects from the prior review have been addressed:

| Defect | Fix Location | Verification |
|--------|-------------|--------------|
| MAJOR-1: Behavior 1 hollow `is_ok()` | test-plan.md L145-158 | Now asserts `as_bytes().len() == L`, `archived().source_path == "test.md"`, `archived().word_count == 42`. **FIXED.** |
| MAJOR-2: Bulk loaders missing boundaries | test-plan.md L676-701 (Behavior 35), L1016-1025 (Matrix G) | Large-input test (10K hashes) added; capacity overflow documented as memory-bounded; one-above-max documented as N/A with reasoning. **FIXED.** |
| MAJOR-3: No fail-fast mixed test | test-plan.md L404-422 (B16), L510-523 (B23), L578-591 (B28), L644-657 (B33) | Four fail-fast tests, one per loader, each with valid h1 + corrupt h2, asserting `Err(CorruptPayload)` with `key_hex == h2`. **FIXED.** |
| MAJOR-4: Mutation checkpoint 3 wrong | test-plan.md L424-441 (Behavior 17), L917 (checkpoint 3) | New test: empty input + missing table → `Ok(HashMap::new())`, not `Err(TableOpen)`. Checkpoint 3 corrected to cite Behavior 17. **FIXED.** |
| MINOR-1: Behavior 9 untestable sub-assertion | test-plan.md L298-312 (Behavior 10) | Untestable "no redb table access" removed. Note added acknowledging implementation detail. **FIXED.** |
| MINOR-2: Behavior 4 asserts lifetime | test-plan.md L192-206 (Behavior 4) | Now asserts concrete field values (`source_path`, `title`, `word_count`). Lifetime noted as compile-time property. **FIXED.** |
| MINOR-3: try_from_bytes min/max boundaries | test-plan.md L956-957 (Matrix A) | Min valid payload (empty String → ~4-8 bytes) and max valid payload (100KB String) added. **FIXED.** |
| MINOR-4: deserialize() error path | test-plan.md L226-251 (Behavior 6), L1100-1116 (Decision 2) | Documented as structurally unreachable with rkyv guarantee cited. Proptests 1-3 provide safety net. **FIXED.** |
| MINOR-5: Kani harness is_ok() pattern | test-plan.md L900-903 | Now uses `assert_eq!(result, Ok(HashMap::new()))`. **FIXED.** |
| MINOR-6: StorageError decision deferred | test-plan.md L1075-1098 (Decision 1) | Formally accepted as gap with risk assessment (LOW), verification strategy (type system + code review), and rationale. **FIXED.** |

---

## Axis 1 — Contract Parity

### Public Function Coverage

| # | `pub fn` (contract.md) | BDD Scenarios (test-plan.md) | Status |
|---|------------------------|------------------------------|--------|
| 1 | `load_analyses` (L172-175) | Behaviors 7–17 (11 scenarios) | COVERED |
| 2 | `load_transforms` (L192-195) | Behaviors 18–23 (6 scenarios) | COVERED |
| 3 | `load_chunks` (L212-215) | Behaviors 24–28 (5 scenarios) | COVERED |
| 4 | `load_scrapes` (L232-235) | Behaviors 29–33 (5 scenarios) | COVERED |
| 5 | `OwnedArchive::try_from_bytes` (L255-259) | Behaviors 1–3 (3 scenarios) | COVERED |
| 6 | `OwnedArchive::archived` (L263) | Behavior 4 (1 scenario) | COVERED |
| 7 | `OwnedArchive::deserialize` (L272) | Behavior 5 (1 scenario) | COVERED |

All 7 public functions have ≥1 BDD scenario. **PASS.**

### Error Variant Coverage

| `BulkLoadError` variant (contract.md L100-126) | Trigger scenario | Asserts exact variant? |
|---|---|---|
| `TableOpen` | Behavior 12 (L329-340) | YES — asserts `table: "analysis_outputs"`, `message: non-empty` |
| `StorageError` | Behavior 13 (L343-373) | YES — asserts `table: "analysis_outputs"`, `message: non-empty` (accepted gap per Decision 1) |
| `CorruptPayload` | Behaviors 2, 14, 16, 22, 23, 27, 28, 32, 33 (9 tests) | YES — asserts `table`, `key_hex`, `message` fields |

All 3 error variants have explicit scenarios with concrete field assertions. **PASS.**

**Axis 1 Verdict: PASS**

---

## Axis 2 — Assertion Sharpness

Every `Then:` clause in every BDD scenario (Section 3, L144-701) was re-inspected.

### Scenarios verified SHARP

| Behavior | Then-clause summary | Verdict |
|----------|-------------------|---------|
| 1 (L147-158) | `as_bytes().len() == L`, `archived().source_path == "test.md"`, `archived().word_count == 42` | SHARP |
| 2 (L163-178) | `Err(CorruptPayload { table: "analysis_outputs", key_hex: hex::encode([0xAA; 32]), message: non-empty })` | SHARP |
| 3 (L181-189) | `as_bytes() == &bytes[..]` byte-level equality | SHARP |
| 4 (L194-206) | `archived_ref.source_path == "fields.md"`, `.title == "Fields"`, `.word_count == 100` | SHARP |
| 5 (L209-224) | `source_path == "roundtrip.md"`, `title == "Roundtrip"`, `word_count == 999`, `has_code == true` | SHARP |
| 7 (L254-269) | `len() == 3`, `map[&h1].archived().source_path == "a.md"`, `.word_count == 10`, etc. | SHARP |
| 8 (L273-284) | `len() == 1`, contains `&h1`, NOT contains `&h2`, `source_path == "present.md"` | SHARP |
| 9 (L288-296) | `len() == 0` | SHARP |
| 10 (L299-312) | `Ok(HashMap::new())`, `len() == 0` | SHARP |
| 11 (L314-326) | `len() == 1`, one entry keyed by h1, `source_path == "dedup.md"` | SHARP |
| 12 (L329-340) | `Err(BulkLoadError::TableOpen)`, `table == "analysis_outputs"`, `message: non-empty` | SHARP |
| 13 (L343-373) | `Err(BulkLoadError::StorageError)`, `table == "analysis_outputs"`, `message: non-empty` | SHARP |
| 14 (L376-388) | `Err(CorruptPayload)`, `table`, `key_hex == hex::encode(h1)`, `message: non-empty` | SHARP |
| 15 (L391-401) | `*k == h1` exact 32-byte equality, `map.keys().next().unwrap() == &h1` | SHARP |
| 16 (L404-422) | `Err(CorruptPayload)`, `table == "analysis_outputs"`, `key_hex == hex::encode(h2)` | SHARP |
| 17 (L424-441) | `Ok(HashMap::new())`, `len() == 0`, NOT `Err(TableOpen)` | SHARP |
| 18-33 | Isomorphic to above patterns with type-appropriate field assertions | SHARP |
| 34 (L660-674) | Both results `Ok(map)`, `len() == 1`, concrete field values for each map | SHARP |
| 35 (L676-701) | `len() == 10_000`, all keys present, `map[&h0].archived().source_path == expected` | SHARP |

**Zero instances of `is_ok()`, `is_err()`, vague `> 0`, or `Some(_)` without concrete inner value.**

**Axis 2 Verdict: PASS**

---

## Axis 3 — Trophy Allocation

### Density Calculation

| Category | Count |
|----------|-------|
| BDD scenarios (executable) | 34 (Behavior 6 is documentation-only) |
| Proptest invariants | 4 |
| Fuzz targets | 5 |
| Kani harnesses | 2 |
| **Total verification points** | **45** |
| Public functions | 7 |
| **Ratio** | **45 / 7 = 6.43×** |

6.43× ≥ 5× target. **PASS.**

### Trophy Distribution

| Layer | Count | Ratio | Verdict |
|-------|-------|-------|---------|
| Unit | 6 | 17% | Reasonable — pure OwnedArchive construction, no redb |
| Integration | 29 | 83% | Correct — all bulk loaders need real redb + real rkyv |
| E2E | 0 | 0% | Correct — no CLI surface |

### Proptest / Fuzz Coverage

| Pure function | Non-trivial input? | Proptest | Fuzz |
|---------------|-------------------|----------|------|
| `try_from_bytes` | Yes (arbitrary bytes) | Proptests 1-3 (round-trip) | Fuzz 1-4 |
| `deserialize` | Yes (rkyv deserialization) | Proptests 1-3 (round-trip) | Implicit via Fuzz 1-4 |
| Bulk loader dedup | Yes (arbitrary hash slices) | Proptest 4 | Fuzz 5 |

All pure functions with non-trivial input spaces have proptest invariants. All parsers/deserializers have fuzz targets. **PASS.**

**Axis 3 Verdict: PASS**

---

## Axis 4 — Boundary Completeness

### `OwnedArchive::try_from_bytes` boundaries

| Boundary | Named? | Where |
|----------|--------|-------|
| Min valid payload (smallest rkyv value) | YES | Matrix A L956 |
| Max valid payload (100KB) | YES | Matrix A L957 |
| Empty bytes (0 length) | YES | Matrix A L952 |
| Garbage bytes | YES | Matrix A L953 |
| Truncated valid bytes | YES | Matrix A L954 |
| Valid header + corrupted body | YES | Matrix A L955 |
| Overflow (4GB length prefix) | YES | Fuzz 1 corpus seed L553 |

All 7 boundaries present. **PASS.**

### Bulk loader boundaries (load_analyses representative, per L959 isomorphic claim)

| Boundary | Named? | Where |
|----------|--------|-------|
| Minimum: empty slice `&[]` | YES | Behavior 10, Matrix G L1020 |
| Single hash | YES | Behaviors 12-15 |
| Multiple hashes | YES | Behavior 7 |
| All duplicates | YES | Behavior 11 |
| No matching hashes | YES | Behavior 9 |
| Large input (10,000) | YES | Behavior 35, Matrix G L1023 |
| Capacity overflow | DOCUMENTED | Matrix G L1024 — memory-bounded, std HashMap delegates |
| One-above-max | DOCUMENTED | Matrix G L1025 — N/A, boundary is system memory |

**PASS.** The capacity overflow and one-above-max boundaries are addressed with explicit documented decisions explaining why they are not numerically bounded (memory is the constraint, not a discrete limit). The justification at L1024 is technically sound: `HashMap::with_capacity(hashes.len())` takes `usize`, and a slice of `[u8; 32]` cannot exceed `isize::MAX / 32` entries before the system OOMs.

**Axis 4 Verdict: PASS**

---

## Axis 5 — Mutation Survivability (Thought Experiment)

### Mutation checkpoint verification (Section 7, L907-938)

All 16 checkpoints re-verified against revised behaviors:

| # | Mutation | Caught by | Correct? |
|---|----------|-----------|----------|
| 1 | Remove bytecheck validation | Behavior 2 | ✓ |
| 2 | Swap table name string in error | Behavior 14 | ✓ asserts `table == "analysis_outputs"` |
| 3 | Remove empty-slice early return | **Behavior 17** | ✓ **FIXED** — was wrong (cited Behavior 9 before) |
| 4 | Return empty HashMap always | Behavior 7 | ✓ asserts `len() == 3` |
| 5 | Skip dedup | Behavior 11 | ✓ asserts `len() == 1` for 3 identical inputs |
| 6 | HashMap::with_capacity(0) | Behavior 7 | Acknowledged as benchmark concern |
| 7 | Remove CorruptPayload branch | Behavior 14 | ✓ garbage would panic or wrong error |
| 8 | Wrong table name in TableOpen | Behavior 12 | ✓ asserts exact string |
| 9 | Remove ownership copy | Compile error | ✓ lifetime mismatch |
| 10 | Include missing hashes with default | Behavior 8 | ✓ asserts `len() == 1`, missing key absent |
| 11 | Wrong key_hex in CorruptPayload | Behavior 14 | ✓ asserts exact hex match |
| 12 | Remove StorageError branch | Type system | Accepted gap per Decision 1 |
| 13 | Skip table open error handling | Behavior 12 | ✓ would panic |
| 14 | Wrong table def per loader | Behaviors 18, 24, 29 | ✓ wrong data or empty |
| 15 | **Skip corrupt silently** | **Behavior 16** | ✓ **NEW** — valid+corrupt mix must return Err |
| 16 | Wrong key_hex in fail-fast | Behavior 16 | ✓ asserts `key_hex == hex::encode(h2)` |

All 16 checkpoints now correctly mapped. The two previously wrong checkpoints (3 and the absent 15) are fixed.

### Additional mutation probe — "skip corrupt silently" per-loader

Applying the same mutation to load_transforms, load_chunks, load_scrapes:
- load_transforms: Behavior 23 catches it (valid String at h1, garbage at h2) ✓
- load_chunks: Behavior 28 catches it (valid Vec<Chunk> at h1, garbage at h2) ✓
- load_scrapes: Behavior 33 catches it (valid ScrapedPage at h1, garbage at h2) ✓

The fail-fast mutation is caught for all 4 loaders. **PASS.**

### Remaining per-loader gap analysis

The plan tests the full matrix (13 scenarios) for load_analyses (Matrix B) and reduced matrices for the other 3 loaders, explicitly relying on the isomorphic claim (L959). Two specific per-loader scenarios are absent:

1. **TableOpen error for load_transforms/chunks/scrapes**: Only Behavior 12 tests TableOpen (for load_analyses). The mutation "remove TableOpen error handling from load_transforms" would only be caught if the implementation uses a shared generic helper. If each loader has independent table-open code, the mutation survives.

2. **Deduplication for load_chunks/scrapes**: Only Behavior 11 (load_analyses) and Behavior 21 (load_transforms) explicitly test deduplication with concrete BDD assertions. load_chunks and load_scrapes rely on Proptest 4 (generic deduplication invariant) and the isomorphic assumption.

These are noted as MINOR findings below. The isomorphic claim is structurally reasonable — the contract shows identical function signatures differing only in table name and return type — and the shared patterns are thoroughly tested on the representative loader.

**Axis 5 Verdict: PASS (with 2 MINOR notes)**

---

## Axis 6 — Holzmann Plan Audit

| Rule | Assessment | Finding |
|------|-----------|---------|
| Rule 2 — Bound Every Loop | No loops in any planned test body | PASS |
| Rule 3 — Know What You Own | `tempfile::TempDir` specified (Section 9, L1035) | PASS |
| Rule 4 — One Function, One Job | Integration tests will need redb setup; acceptable for integration layer | PASS |
| Rule 5 — State Your Assumptions | Given blocks are explicit; fixture helpers described in Section 9 | PASS |
| Rule 6 — Never Swallow Errors | No `let _ =` or `.ok()` in any planned assertion | PASS |
| Rule 7 — Narrow Your State | Per-test TempDir databases; no shared mutable state | PASS |
| Rule 8 — Surface Your Side Effects | Fixture helpers descriptively named (L1032-1043): `insert_valid`, `insert_garbage`, `create_session` | PASS |
| Rule 9 — One Layer of Magic | Test → fixture helper → redb. One level of indirection | PASS |

**Axis 6 Verdict: PASS**

---

## Cumulative Finding Summary

### LETHAL FINDINGS: 0

### MAJOR FINDINGS: 0

### MINOR FINDINGS: 2

| ID | Axis | Location | Finding |
|----|------|----------|---------|
| MINOR-7 | 5 (Mutation) | test-plan.md Matrices C/D/E (L977-1006) | `load_transforms`, `load_chunks`, `load_scrapes` lack explicit TableOpen error tests. Only `load_analyses` has Behavior 12 testing the TableOpen path. The mutation "remove TableOpen error handling from load_transforms" survives if each loader has independent table-open code. Mitigated by: isomorphic claim, mutation checkpoint 14 (wrong table def caught by all-found tests), and the `#[non_exhaustive]` enum making compile-time removal impossible. Recommendation: either add per-loader TableOpen tests or add an explicit note in Section 9 stating "implementation MUST use shared generic bulk_load helper; per-loader TableOpen testing is redundant by design." |
| MINOR-8 | 5 (Mutation) | test-plan.md Behaviors 24-28, 29-33 | `load_chunks` and `load_scrapes` lack explicit deduplication BDD scenarios. Only `load_analyses` (Behavior 11) and `load_transforms` (Behavior 21) test dedup with concrete assertions. Proptest 4 covers the generic invariant (`len() ≤ unique count`) for all loaders, which is the stronger property. The explicit BDD tests serve as readable examples and mutation killers. Low risk — dedup logic is shared or trivially identical. |

### Severity Assessment

| Severity | Count | Threshold | Result |
|----------|-------|-----------|--------|
| LETHAL | 0 | ≥1 = REJECT | — |
| MAJOR | 0 | ≥3 = REJECT | — |
| MINOR | 2 | ≥5 = REJECT | **NOT BREACHED** |

**0 LETHAL + 0 MAJOR + 2 MINOR = APPROVED.**

---

## Advisory Notes (non-blocking)

These observations do not affect the verdict but are worth considering during implementation:

1. **`as_bytes()` not in contract**: Behaviors 1, 3 and Matrix A reference `OwnedArchive::as_bytes()` which is not declared in the `OwnedArchive` contract (contract.md L244-274 shows only `try_from_bytes`, `archived()`, `deserialize()`). The implementation will need to add `pub fn as_bytes(&self) -> &[u8]` or the test writer must replace these assertions with contract-compliant alternatives (e.g., compare via `archived()` field values only). Trivial fix either way.

2. **Proptest 4 scope**: Proptest 4 (L758-769) says "any bulk loader method" but will need to test at least one concrete loader. The plan should specify which loader(s) the proptest exercises, or use a parameterized approach testing all four.

3. **Behavior 35 setup complexity**: Populating 10,000 entries in a redb table for the large-input test requires a helper with a loop. The loop is in the *fixture setup*, not the test body, so it doesn't violate Holzmann Rule 2. Worth noting explicitly in the test implementation to avoid a false positive during code review.

---

## What the Plan Gets Right

This revised plan demonstrates genuine engagement with the prior review:

- **Every MAJOR finding was addressed with a concrete, correct fix.** Not papered over — the fail-fast tests (Behaviors 16, 23, 28, 33) are exactly the right shape: valid + corrupt mix, asserting the corrupt key's hex in the error. The early-return mutation catcher (Behavior 17) is clever and correct — it's the only test that distinguishes "skip table open" from "open table and find nothing."

- **Error variant exhaustiveness is genuinely complete.** Every `BulkLoadError` variant has a scenario with concrete field matching. The StorageError gap (Decision 1) is handled with professional rigor — risk assessment, verification strategy, and clear rationale for why inducing the error is CI-incompatible.

- **The `deserialize()` unreachable path** (Decision 2) is documented with structural reasoning, not hand-waving. The rkyv guarantee is cited, and the proptest safety net is identified.

- **Mutation checkpoint table** is now accurate. All 16 checkpoints correctly map mutations to catching tests. The previously wrong checkpoint 3 and absent checkpoint 15 are both fixed.

- **Proptest and fuzz coverage** are well-scoped. Fuzz corpus seeds target real rkyv failure modes (length prefix claiming 4GB, valid header + corrupted body). Kani Harness 1 (key identity) addresses the subtle "hash of hash" bug class.

- **Boundary documentation** is now thorough. Matrix G explicitly addresses each boundary with a test reference or documented decision. The capacity overflow reasoning (L1024) is technically sound.

---

## MANDATE

**None.** The plan meets all quality thresholds for APPROVED status.

The two MINOR findings (per-loader TableOpen and deduplication gaps) are advisory. They represent a tradeoff between exhaustive per-loader coverage and the isomorphic assumption. The plan author should consider adding the explicit shared-implementation note recommended in MINOR-7, but this is not required for approval.
