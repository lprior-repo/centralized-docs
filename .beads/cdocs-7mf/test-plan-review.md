# Test Plan Review: SCIP-inspired Semantic Domain Model

- **bead_id**: cdocs-7mf
- **bead_title**: schema: Define SCIP-inspired semantic domain model
- **phase**: STATE_1.7_RETRY_2
- **updated_at**: 2026-03-29T22:15:00Z
- **reviewer**: Test Inquisitor (Mode 1 — Plan Inquisition)
- **audited_files**:
  - `contract.md` (369 lines)
  - `test-plan.md` (1772 lines)
- **prior_review**: STATE_1.7_RETRY_1 — REJECTED (0 LETHAL, 0 MAJOR, 5 MINOR)

---

## VERDICT: APPROVED

0 LETHAL + 0 MAJOR + 1 MINOR.

All 5 MINOR findings from RETRY_1 have been addressed. One residual documentation count error remains — the summary's unit test count (82) does not correspond to any defensible enumeration. The underlying test coverage, assertion sharpness, boundary completeness, and mutation resistance are all excellent. This is a paperwork error, not a coverage gap.

---

## Prior Defect Verification (5/5 addressed from RETRY_1)

| ID | Finding | Status | Evidence |
|----|---------|--------|----------|
| m1 | Summary trophy allocation counts stale | **PARTIALLY FIXED** | Integration count correctly updated to 19. Unit count changed from 76 to 82 — but 82 does not match any defensible count (see fresh finding f1). Line 240 Totals also says "82 unit". |
| m2 | SymbolRelationship header "(4 behaviors)" | **FIXED** | test-plan.md:115 — now says "(5 behaviors)". Summary line 10 says "95 behaviors" = 37+27+8+6+5+8+4 = 95. ✓ |
| m3 | Section 3.2 header "5 error variants" | **FIXED** | test-plan.md:280 — now says "2 error variants". Matches contract: EmptyScheme, InvalidScheme. ✓ |
| m4 | Section 3.3 header "6 error variants" | **FIXED** | test-plan.md:344 — now says "5 error variants". Matches contract: EmptyModulePath, EmptyModuleSegment, LeadingSlash, TrailingSlash, HashInModulePath. ✓ |
| m5 | Section 3.4 header "4 error variants" | **FIXED** | test-plan.md:418 — now says "2 error variants". Matches contract: EmptyDescriptor, SlashInDescriptor. ✓ |

---

## Fresh 6-Axis Audit

### Axis 1 — Contract Parity

#### pub fn Coverage

Contract defines **15 public functions**:

| # | Function | BDD Scenario? | Location |
|---|----------|--------------|----------|
| 1 | `ScipSymbolId::new` | Yes (14 scenarios) | test-plan.md:246-476 |
| 2 | `ScipSymbolId::parse` | Yes (9 scenarios) | test-plan.md:480-561 |
| 3 | `ScipSymbolId::as_str` | Yes | test-plan.md:581-589 |
| 4 | `ScipSymbolId::scheme` | Yes | test-plan.md:591-599 |
| 5 | `ScipSymbolId::module_path` | Yes | test-plan.md:601-609 |
| 6 | `ScipSymbolId::descriptor` | Yes | test-plan.md:611-619 |
| 7 | `ScipSymbolId::into_string` | Yes | test-plan.md:621-629 |
| 8 | `SymbolRole::from_bits` | Yes (5 scenarios) | test-plan.md:754-803 |
| 9 | `SymbolRole::from_bits_truncate` | Yes (2 scenarios) | test-plan.md:807-829 |
| 10 | `SymbolRole::empty` | Yes | test-plan.md:730-749 |
| 11 | `SymbolRole::bits` | Yes | test-plan.md:835-843 |
| 12 | `SymbolRole::contains` | Yes (3 scenarios) | test-plan.md:845-873 |
| 13 | `SymbolRole::is_empty` | Yes (2 scenarios) | test-plan.md:875-883 |
| 14 | `SymbolRole::union` | Yes | test-plan.md:889-899 |
| 15 | `SymbolRole::intersection` | Yes | test-plan.md:901-911 |

Plus trait impls: Display, Deref, AsRef, Borrow, Hash, PartialOrd, Ord, BitOr, BitAnd, BitOrAssign, BitAndAssign — all have BDD scenarios.

**All 15 pub fn covered.**

#### Error Variant Coverage

Contract defines **12 error variants**:

| Variant | Asserted Exactly? | Location |
|---------|-------------------|----------|
| `EmptyScheme` | Yes | test-plan.md:287, 297, 307 |
| `InvalidScheme(s)` where s=="/" | Yes | test-plan.md:317 |
| `InvalidScheme(s)` where s=="#" | Yes | test-plan.md:327 |
| `InvalidScheme(s)` wildcard | Yes | test-plan.md:337 |
| `EmptyModulePath` | Yes | test-plan.md:351, 361, 371 |
| `EmptyModuleSegment(pos)` where pos==5 | Yes | test-plan.md:381, 547 |
| `LeadingSlash` | Yes | test-plan.md:391 |
| `TrailingSlash` | Yes | test-plan.md:401 |
| `HashInModulePath` | Yes | test-plan.md:411 |
| `EmptyDescriptor` | Yes | test-plan.md:425, 435, 445 |
| `SlashInDescriptor` | Yes | test-plan.md:455, 557 |
| `InvalidFormat(s)` with concrete values | Yes | test-plan.md:497, 507, 517, 527, 537 |
| `UnknownBit(bit)` where bit==32 | Yes | test-plan.md:790 |
| `UnknownBit(bit)` where bit==33 | Yes | test-plan.md:800 |
| `UnknownKind(s)` where s=="unknown_kind" | Yes | test-plan.md:1037 |
| `UnknownKind(s)` where s=="" | Yes | test-plan.md:1047 |

**All error variants have scenarios asserting the exact variant with concrete payloads.**

[PASS] Contract parity: all pub fn and all error variants covered.

---

### Axis 2 — Assertion Sharpness

Every `Then:` clause inspected across all 21 BDD sections. **Zero** `is_ok()`, **zero** `is_err()`, **zero** `> 0`, **zero** `Some(_)` without inner value.

All `Ok(id)` results specify `id.as_str() == "exact_string"`. All `Err(ExactVariant)` specify the variant name and payload. All boolean assertions specify the expected value. All numeric assertions specify exact values. All string assertions specify exact strings.

One deliberate ambiguity at test-plan.md:465 — `Err(ScipSymbolIdError::InvalidScheme(_)) or implementation rejects to preserve INV-1` — documented with rationale at lines 472-476. This is an accurate reflection of contract ambiguity between PRE-S5 and INV-1, not a sharpness defect.

[PASS] Assertion sharpness: no banned patterns, no vague assertions.

---

### Axis 3 — Trophy Allocation

#### Density

- **Public functions:** 15
- **BDD test functions:** 109 (88 `Test:` lines + 21 bullet `fn` entries)
- **Minimum ratio:** 109 / 15 = **7.27x** (target >=5x)

Even with the most conservative count (76 unit behaviors from trophy table): 76 / 15 = **5.07x**.

[PASS] Density: well above 5x threshold.

#### Proptest Invariants

9 proptests covering all pure functions with non-trivial input spaces:
1. ScipSymbolId new→parse round-trip
2. ScipSymbolId single `#` invariant (INV-1)
3. ScipSymbolId no-empty-segments invariant (INV-3) — uses `if let` not `unwrap()`
4. SymbolRole from_bits→bits round-trip
5. SymbolRole algebraic laws (commutativity, identity, absorption, bounded)
6. SymbolRole contains/bits consistency
7. SymbolRole Display determinism and sort order
8. SymbolNode serde round-trip (all fields)
9. SymbolKind serde round-trip (all variants)

[PASS] All pure functions with non-trivial input spaces have proptest invariants.

#### Fuzz Targets

4 fuzz targets covering all parsers and deserializers:
1. `ScipSymbolId::parse` — 17 corpus seeds
2. serde deserialize `ScipSymbolId` — 7 corpus seeds
3. serde deserialize `SymbolRole` — 9 corpus seeds
4. serde deserialize `SymbolKind` — 8 corpus seeds

[PASS] All parsers and deserializers have fuzz targets.

#### Kani Harnesses

3 Kani harnesses for formal verification:
1. `ScipSymbolId::new` never panics (any String inputs)
2. `SymbolRole::from_bits` never panics (any u32)
3. `SymbolRole::from_bits_truncate` result always valid

[PASS]

---

### Axis 4 — Boundary Completeness

All boundaries for all functions explicitly named and tested:

- **ScipSymbolId::new scheme**: valid, empty, whitespace (spaces/tabs/newlines), `/`, `#`, multiple invalid, unicode, hyphens — **complete**
- **ScipSymbolId::new module_path**: valid single/multi-segment, empty, whitespace, double/triple/only slashes, leading/trailing, `#` — **complete**
- **ScipSymbolId::new descriptor**: valid, dots, parens, empty, whitespace (spaces/mixed), `/`, `#`, unicode — **complete**
- **ScipSymbolId::parse**: valid, no `#`, empty, just `#`, empty scheme, empty descriptor, no `/`, multiple `#`, whitespace, propagated errors — **complete**
- **SymbolRole::from_bits**: 0, each flag (1,2,4,8,16), combinations (3,31), unknown (32,33), u32::MAX — **complete**
- **SymbolRole::from_bits_truncate**: 0, 31, 33, 64, 255, u32::MAX — **complete**
- **SymbolKind**: all 12 variants, unknown, empty, case-insensitive — **complete**
- **RelationshipKind**: all 6 variants, unknown — **complete**
- **SymbolRelationship/SymbolNode**: Eq, Hash, Hash/Eq consistency, serialize, deserialize, round-trip (all/None/empty/mixed) — **complete**

[PASS] Boundary completeness: all boundaries for all functions explicitly covered.

---

### Axis 5 — Mutation Survivability

29 critical mutations listed in Section 7 (lines 1566-1596), each with a named killer test. All 29 verified against BDD scenarios — every mutation has a test that would fail if the mutation were applied.

Additional thought-experiment mutations checked:
- Change `union` to AND: caught by `result.bits() == 3`
- Change `intersection` to OR: caught by `result.bits() == 2`
- Change `empty()` to non-zero: caught by `empty().bits() == 0`
- Remove Hash derive from SymbolNode: compile failure
- Change `as_str` to wrong format: accessor tests + proptest round-trip

No surviving mutations identified.

[PASS] Mutation survivability: all 29 critical mutations killed.

---

### Axis 6 — Holzmann Plan Audit

| Rule | Status | Evidence |
|------|--------|----------|
| Rule 1 (Keep it Linear) | PASS | All BDD scenarios: straight-line Given→When→Then. No nested conditionals. |
| Rule 2 (Bound Every Loop) | PASS | No loops in any BDD scenario body. Proptests use strategy generators. |
| Rule 5 (State Assumptions) | PASS | All scenarios have explicit `Given:` blocks with concrete values. |
| Rule 6 (Never Swallow Errors) | PASS | `unwrap()` only in Given blocks for known-valid setup. Proptest 3 uses `if let`. Zero `let _ =` or `.ok();`. |
| Rule 7 (Narrow Your State) | PASS | Each test creates its own state. No shared mutable state. |
| Rule 8 (Surface Side Effects) | PASS | Pure domain model — zero I/O, zero network, zero filesystem. |

[PASS] Holzmann plan audit: all rules satisfied.

---

## LETHAL FINDINGS

None.

---

## MAJOR FINDINGS (0)

None.

---

## MINOR FINDINGS (1/5 threshold — NOT EXCEEDED)

### f1 — Summary unit test count "82" is incorrect
**File:** test-plan.md:11, test-plan.md:240
**Detail:** Summary states "82 unit / 19 integration / 2 static". The integration count (19) and static count (2) are correct. The unit count of 82 does not correspond to any defensible enumeration:
- Trophy table Unit **rows**: 71
- Trophy table Unit **behaviors** (expanding ranges): 76
- Actual Unit **test functions** (expanding multi-test behaviors 59 and 92): 89
- No arithmetic combination of these produces 82.

The correct count depends on what "unit" measures:
- If counting **behaviors**: 76 unit / 19 integration / 2 static = 97
- If counting **test functions**: 89 unit / 19 integration / 2 static = 110

Either is defensible; 82 is not. This was partially fixed from RETRY_1 (integration correctly updated from 13 to 19) but the unit count was incorrectly adjusted.
**Impact:** Summary misrepresents the test distribution. Does not affect coverage — all tests exist in the BDD sections regardless of the summary count.

---

## MANDATE

The test plan is **APPROVED**. The single MINOR finding (f1) is a documentation count error with no coverage implications. Fix at discretion:

1. **Update summary line 11 and totals line 240** — Change "82 unit" to either "76 unit" (behavior count) or "89 unit" (test function count). Recommend "76 unit" to align with the behavior inventory's "95 behaviors" total (76 + 19 = 95).

No re-review required. This plan is ready for implementation.
