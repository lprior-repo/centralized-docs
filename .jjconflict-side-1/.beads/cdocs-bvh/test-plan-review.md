# Test Plan Review: cdocs-bvh — Re-Audit #3 (Final)

## VERDICT: APPROVED

**0 LETHAL · 0 MAJOR · 2 MINOR**

Full six-axis audit completed on the third submission. All 5 findings from Re-Audit #2 (1 LETHAL,
1 MAJOR, 3 MINOR) are verified resolved or carried as acceptable MINOR. The test plan meets every
mandatory threshold.

---

## Previous Defect Verification Matrix (Re-Audit #2 → #3)

| Prev # | Severity | Finding | Fixed? | Evidence |
|--------|----------|---------|--------|----------|
| L1 | LETHAL | Trophy density 4.32x < 5.0x | **YES** | §3 now has 190 BDD fns (grep-verified) + 18 proptest + 8 fuzz + 4 kani = 220 total. 220 / 44 = 5.0x. See Axis 3. |
| M1 | MAJOR | BDD count inflation: claimed 189, actual 160 `fn` declarations | **YES** | `awk` count of `^fn ` in §3 subsections = 190 exactly. Matches claim. Summary table at §10 line 2126 says 190. Consistent. |
| m1 | MINOR | `DeserializationFailed { reason: _ }` wildcard undocumented | **YES** | §11 Open Question 8 (lines 2171-2176): documents rationale — rkyv internal strings are version-coupled, wildcard is intentional. |
| m2 | MINOR | E2E Given blocks (B154-B156) vague — no concrete field values | **YES** | B154 (line 1370): fully inline `TransformResult { success_count: 3, total_count: 5, error_count: 2, errors: vec![...] }`. B155 (line 1377): `where valid_chunk has chunk_id: "doc#0", doc_id: "doc", doc_title: "Doc", content: "text", token_count: 50`. B156 (line 1384): `where valid_page has url: "https://example.com/p1", title: "Page1", slug: "p1", density_score: 0.75f32`. |
| m3 | MINOR | Fixture references without inline values | **CARRIED** | Lines 1174, 1554, 1561, 1568 still reference `valid_persisted_page_change`, `valid_persisted_change_summary`, `valid_persisted_snapshot` without inline expansion. Acceptable for deeply-nested types (ChangePlan has 3 levels of nesting). Holzmann R5 note. See new m2 below. |

**Result: 4 of 5 findings fully resolved. 1 MINOR carried (below 5-MINOR threshold).**

---

## Axis 1 — Contract Parity: PASS

### 1.1 All 44 Public Functions Have ≥1 BDD Scenario

Grep-verified: 44 `fn` declarations in `contract.md` (lines 389-463). Every function has ≥1
corresponding BDD test function in `test-plan.md` §3.

| Function Category | Count | Verified |
|---|---|---|
| Infallible `*_to_persisted` | 22 | ✅ All have ≥1 BDD (B01-B38) |
| Fallible `*_to_runtime` | 22 | ✅ All have ≥1 BDD (B39-B134, B161-B190) |
| **Total** | **44** | **PASS** |

### 1.2 All 8 PersistError Variants Have Explicit Trigger Tests

| Error Variant | Trigger Tests | Status |
|---|---|---|
| `EmptyField { field }` | B44, B45, B50, B51, B55-B57, B65-B66, B78-B81, B94, B103-B104, B107, B111-B112, B119, B124, B128, B130-B134, B161-B170, B174-B175, B187-B190 | ✅ 40+ trigger points |
| `OutOfRange { field, value, min, max }` | B40, B41, B82, B90, B91, B171, B172 | ✅ 7 trigger points |
| `SchemaVersionMismatch { expected, actual }` | B53-B54, B61-B62, B68, B77, B87, B106, B123, B127 | ✅ All 8 versioned types |
| `SerializationFailed { reason }` | B157 (failing Write/buffer injection) | ✅ |
| `DeserializationFailed { reason }` | B149-B152 (truncated, bit-flipped, zeroed, random) | ✅ 4 triggers |
| `UnknownVariant { type_name }` | B158 (invalid discriminant injection) | ✅ |
| `NonFiniteFloat { field, value }` | B98 (NaN), B99 (+Inf), B100 (-Inf) | ✅ 3 triggers |
| `InvalidHashLength { actual_len }` | B159 (corrupted rkyv archived hash) | ✅ |

**8/8 variants directly tested. PASS.**

### 1.3 All 8 Versioned Types Have Schema Rejection Tests

| Type | Rejection Test | Concrete actual value |
|---|---|---|
| PersistedAnalysis | B53 (actual: 0), B54 (actual: 2) | ✅ |
| PersistedAnalyzeResult | B61 (actual: 0), B62 (actual: 99) | ✅ |
| PersistedTransformResult | B68 (actual: 5) | ✅ |
| PersistedChunk | B77 (actual: 2) | ✅ |
| PersistedChunksResult | B87 (actual: 3) | ✅ |
| PersistedScrapeResult | B106 (actual: 99) | ✅ |
| PersistedSnapshot | B123 (actual: 5) | ✅ |
| PersistedChangePlan | B127 (actual: 3) | ✅ |

**All 8 covered. PASS.**

---

## Axis 2 — Assertion Sharpness: PASS

Grep of all `Then:` blocks in §3:

| Banned Pattern | Hits | Status |
|---|---|---|
| `is_ok()` | 0 | PASS |
| `is_err()` | 0 | PASS |
| `> 0` in Then context | 0 | PASS |
| Bare `Some(_)` without inner value | 0 | PASS |

All error assertions specify exact variants with concrete field values (e.g.,
`Err(PersistError::OutOfRange { field: "level", value: 7, min: 1, max: 6 })`).

All happy-path assertions specify concrete field values (e.g., `heading.level == 3 AND
heading.text == "Details" AND heading.line == 10`).

**One acceptable wildcard:** §3.13 (B149-B152) `DeserializationFailed { reason: _ }` — documented
with rationale in §11 Q8 (rkyv internal strings are version-coupled). Not a finding.

---

## Axis 3 — Trophy Allocation: PASS

### Density Audit

| Metric | Plan's Claim | Actual (grep-verified) |
|--------|-------------|------------------------|
| Contract `pub fn` count | 44 | 44 ✅ |
| BDD test functions (§3) | 190 | **190** ✅ (awk-verified: 190 `fn` declarations) |
| Proptest invariants (§4) | 18 | 18 ✅ (P1-P18) |
| Fuzz targets (§5) | 8 | 8 ✅ (F1-F8) |
| Kani harnesses (§6) | 4 | 4 ✅ (K1-K4) |
| **Grand total** | **220** | **220** ✅ |
| **Density ratio** | **5.0x** | **5.0x** ✅ |

**BDD fn count verified by section:**

| Section | Title | fns |
|---|---|---|
| §3.1 | Heading to_persisted | 1 |
| §3.2 | Heading to_runtime | 8 |
| §3.3 | LinkKind Conversions | 4 |
| §3.4 | Link and Analysis Family | 20 |
| §3.5 | Transform Family | 9 |
| §3.6 | Chunk Family | 28 |
| §3.7 | Scrape to_persisted | 7 |
| §3.8 | Scrape to_runtime | 21 |
| §3.9 | Watch/Snapshot to_persisted | 11 |
| §3.10 | Watch/Snapshot to_runtime | 25 |
| §3.11 | rkyv Round-Trip | 12 |
| §3.12 | Deterministic Serialization | 2 |
| §3.13 | Corrupted Bytes | 4 |
| §3.14 | Error Variant Triggers | 3 |
| §3.15 | Deterministic Frontmatter | 1 |
| §3.16 | E2E Pipeline | 4 |
| §3.17 | Whitespace Rejection | 10 |
| §3.18 | Extreme Boundaries | 5 |
| §3.19 | Enum Round-Trip | 5 |
| §3.20 | rkyv Determinism (additional) | 4 |
| §3.21 | Additional Edge Cases | 6 |
| **Total** | | **190** |

220 / 44 = 5.0x. Meets the ≥5.0x mandate exactly. **PASS.**

### Proptest Coverage

18 proptest invariants (P1-P18):
- P1-P13: Round-trip invariance for all 13 major conversion groups (heading, link_kind, link,
  analysis, analyze_result, transform_error, chunk, chunks_result, scraped_page, scrape_result,
  snapshot, change_plan, id_mapping)
- P14-P18: Cross-cutting invariants (deterministic serialization, frontmatter sort, heading level
  boundary, density finiteness, identifier emptiness)

All `*_to_runtime` functions with non-trivial input spaces have proptest coverage. Enum-only
functions (2-3 variants) are covered exhaustively by BDD. **PASS.**

### Fuzz Coverage

8 fuzz targets (F1-F8) cover all major rkyv deserialization entry points for complex record types.
Simple leaf types (enums, 2-field structs) reasonably excluded. **PASS.**

### Kani Coverage

4 Kani harnesses (K1-K4) for safety-critical properties:
- K1: Heading level range completeness (exhaustive u32)
- K2: content_hash bounds (fixed [u8; 32])
- K3: Schema version check exhaustiveness (7 versioned types)
- K4: density_score finiteness (exhaustive f32)

**PASS.**

---

## Axis 4 — Boundary Completeness: PASS

### PersistedHeading (B39-B46, B172)

| Boundary | Test | Status |
|---|---|---|
| Min valid level (1) | B42 | ✅ |
| Max valid level (6) | B43 | ✅ |
| Below min (0) | B40 | ✅ |
| Above max (7) | B41 | ✅ |
| Extreme max (u32::MAX) | B172 | ✅ |
| Empty text | B44 | ✅ |
| Whitespace-only text | B45 | ✅ |
| line == 0 (valid) | B46 | ✅ |

### PersistedHeader (B89-B94, B171)

| Boundary | Test | Status |
|---|---|---|
| Min valid level (1) | B92 | ✅ |
| Max valid level (6) | B93 | ✅ |
| Below min (0) | B90 | ✅ |
| Above max (7) | B91 | ✅ |
| Extreme max (255/u8 max) | B171 | ✅ |
| Empty text | B94 | ✅ |

### PersistedChunk (B76-B85, B164-B165, B174-B175)

| Boundary | Test | Status |
|---|---|---|
| Schema version != 1 | B77 | ✅ |
| chunk_id == "" / whitespace | B78, B164 | ✅ |
| doc_id == "" / whitespace | B79, B165 | ✅ |
| doc_title == "" / whitespace | B80, B175 | ✅ |
| content == "" | B81 | ✅ |
| summary == "" | B174 | ✅ |
| token_count == 0 | B82 | ✅ |
| token_count == 1 (min valid) | B83 | ✅ |
| Empty collections | B84 | ✅ |
| heading == None | B85 | ✅ |

### PersistedScrapedPage (B97-B104, B166-B168, B173)

| Boundary | Test | Status |
|---|---|---|
| density_score NaN | B98 | ✅ |
| density_score +Inf | B99 | ✅ |
| density_score -Inf | B100 | ✅ |
| density_score f32::MAX | B101 | ✅ |
| density_score 0.0 | B102 | ✅ |
| density_score -1.0 (negative finite) | B173 | ✅ |
| url == "" / whitespace | B103, B166 | ✅ |
| slug == "" / whitespace | B104, B167 | ✅ |
| title == "" | B168 | ✅ |

### PersistedIdMapping (B129-B134, B169, B189-B190)

All 5 identifier fields tested for empty + whitespace rejection. ✅

### All Boundaries Covered. PASS.

---

## Axis 5 — Mutation Survivability: PASS

### Mutation Checkpoint Verification

| Mutation | Caught By | Status |
|---|---|---|
| `>` to `>=` in heading level check | B42 (level == 1 now fails if `> 1`) | ✅ |
| Delete `EmptyField` branch for any field | B44, B50, B55-B57, B65-B66, B78-B81, B94, B103-B104, etc. | ✅ |
| Return `Ok(Default::default())` from `*_to_runtime` | Happy-path tests check concrete field values | ✅ |
| Swap function arguments | Field-level assertions catch misplacement | ✅ |
| Delete `schema_version != 1` check | B53-B54, B61-B62, B68, B77, B87, B106, B123, B127 | ✅ |
| Delete `frontmatter.sort_by_key` | B160 (deterministic frontmatter) | ✅ |
| `is_finite()` → `is_normal()` | B102 (0.0 is finite not normal) | ✅ |
| Delete `doc_id` empty check | B79, B165 (whitespace) | ✅ |
| Delete `token_count > 0` check | B82 | ✅ |
| Delete body of `persisted_page_change_to_runtime` | B116-B120 (5 scenarios) | ✅ |
| Delete body of `persisted_change_summary_to_runtime` | B121 (3 of 6 fields + P12 proptest) | ✅ |
| Delete body of `persisted_header_to_runtime` | B89-B94, B171 (7 scenarios) | ✅ |
| `trim().is_empty()` → `is_empty()` | B45, B161-B170, B175, B187-B190 (whitespace tests) | ✅ |
| `level <= 6` → `level <= 7` | B41, B91 (level == 7 rejected) | ✅ |
| `level <= 6` → `level <= u8::MAX` | B171 (level == 255 rejected) | ✅ |
| `level <= 6` → `level <= u32::MAX` | B172 (level == u32::MAX rejected) | ✅ |
| Delete enum variant mapping | B176-B180 (exhaustive round-trips) | ✅ |
| Delete rkyv serialization validation | B149-B152 (corrupted bytes) | ✅ |

**All standard mutations caught. §7 table lists 28 specific mutations, all with catching tests. PASS.**

---

## Axis 6 — Holzmann Plan Audit: PASS

| Rule | Status | Notes |
|---|---|---|
| R1 — Keep it linear | PASS | All 190 BDD scenarios follow Given/When/Then. No nesting. |
| R2 — Bound loops | PASS | No loops planned in test bodies. |
| R3 — Know what you own | PASS | No file I/O, no resources. Pure conversion + in-memory rkyv. |
| R4 — One function, one job | PASS | Each BDD scenario tests one behavior. |
| R5 — State assumptions | PASS* | E2E tests (B154-B156) now have concrete field values. B153 uses `valid_analysis` without expansion. Integration tests with deeply-nested types use fixture references. See m2 below. |
| R6 — Never swallow errors | PASS | No `let _ =` or `.ok()` planned. |
| R7 — Narrow state | PASS | No shared mutable state. Each test constructs its own inputs. |
| R8 — Surface side effects | PASS | No I/O side effects. |
| R9 — One layer of magic | PASS | Direct function calls. No helper chains deeper than 1 level. |
| R10 — Warnings are errors | N/A | No implementation yet. Enforce at Tier 1 (Mode 2). |

**PASS.**

---

## Full Finding Summary

### LETHAL FINDINGS (0)

None.

### MAJOR FINDINGS (0)

None.

### MINOR FINDINGS (2/5 threshold)

| # | Axis | Finding | Location |
|---|------|---------|----------|
| m1 | 6 | Fixture references without inline values. Integration tests for deeply-nested types (ChangePlan at lines 1049, 1174; ChunksResult at 1554; ScrapeResult at 1561; ChangePlan at 1568) reference `valid_persisted_page_change`, `valid_persisted_change_summary`, `valid_persisted_snapshot`, `valid_chunk`, `valid_page` without expanding all fields. Acceptable for 3+ levels of nesting — inline expansion would make Given blocks unreadable. Holzmann R5 note only. | test-plan.md:1049, 1174, 1554, 1561, 1568 |
| m2 | 3 | §2 Trophy Layer Breakdown table: Static row claims count 12, but the listed behaviors (B02-B03, B12-B17, B22-B23, B29-B31) total 13. Cosmetic only — the §10 Density Calculation table (the authoritative count) correctly states 190 BDD + 30 non-BDD = 220 total. Grand total and density ratio unaffected. | test-plan.md:21-25 vs 333 |

**0 LETHAL + 0 MAJOR + 2 MINOR = APPROVED**

---

## Cumulative Audit History

| Audit | LETHAL | MAJOR | MINOR | Verdict |
|-------|--------|-------|-------|---------|
| #1 (initial) | 15 | 9 | 8 | REJECTED |
| #2 | 1 | 1 | 3 | REJECTED |
| #3 (this) | 0 | 0 | 2 | **APPROVED** |

Total findings resolved across all audits: 15 + 9 + 8 + 1 + 1 + 3 = 37. All LETHAL and MAJOR
findings resolved. 2 remaining MINOR findings are cosmetic/acceptable and well below the 5-MINOR
rejection threshold.

---

## Implementation Notes

The test plan is approved. The following are advisory notes for the implementation phase, not
blocking findings:

1. **§11 Open Questions are well-documented.** Questions 1-3 (SerializationFailed, UnknownVariant,
   InvalidHashLength trigger approaches) and 4-7 (DateTime precision, external types, feature flag,
   line == 0) should be resolved at implementation time. The documented approaches are sound.

2. **E2E test B153** (line 1363) references `valid_analysis` without expanding fields. While B154-B156
   now have inline values, B153 lags. Consider adding a `where valid_analysis has ...` clause for
   consistency, but this is not blocking.

3. **`persisted_change_summary_to_runtime`** has only 1 BDD scenario (B121) checking 3 of 6 fields.
   The function is trivially correct (6 Copy usize fields, no validation logic), and P12 provides
   proptest round-trip coverage. No gap, but the plan could note this explicitly.

4. **Tier 1 (Mode 2) will enforce:** `cargo clippy --tests -- -D warnings`, `cargo nextest run
   --retries 2`, ordering probe, insta staleness (if applicable). All Holzmann rules should be
   verified against actual test code at that stage.

---

**STATUS: APPROVED. Proceed to implementation.**
