# Test Plan Review: Fixed-Size `FileStateRaw` and `UrlStateRaw` Pod Types

## VERDICT: APPROVED

**Reviewer**: Test Inquisitor (Mode 1 — Plan Inquisition)
**Date**: 2026-04-02 (Re-audit after revision)
**Contract**: `.beads/cdocs-h70/contract.md`
**Test Plan**: `.beads/cdocs-h70/test-plan.md` (REVISED)
**Previous Review**: REJECTED — 2 LETHAL, 2 MAJOR, 9 MINOR

---

## Previous Defect Verification

All 13 previous findings verified as FIXED:

| # | Previous Finding | Status | Evidence in Revised Plan |
|---|-----------------|--------|--------------------------|
| LETHAL-1 | `FileStateRaw::from_bytes_unchecked()` zero scenarios | FIXED | test-plan.md:479–493 (field match) + :517–528 (pointer identity) — 2 full BDD scenarios |
| LETHAL-2 | `UrlStateRaw::from_bytes_unchecked()` zero scenarios | FIXED | test-plan.md:497–513 (field match) + :531–542 (pointer identity) — 2 full BDD scenarios |
| MAJOR-1 | `UrlStateRaw::validate()` missing version rejection | FIXED | test-plan.md:894–902 — `url_state_raw_validate_rejects_wrong_version()` |
| MAJOR-2 | `UrlStateRaw::validate()` missing reserved rejection | FIXED | test-plan.md:918–927 — `url_state_raw_validate_rejects_nonzero_reserved()` |
| MINOR-1 | All-FF test ambiguous on error variant | FIXED | test-plan.md:982 (explicit validation order convention) + :998–1008, :1024–1034 (pinned to `VersionMismatch { actual: 255 }`) |
| MINOR-2 | `UrlStateRaw::new()` missing `content_length=u64::MAX` | FIXED | test-plan.md:327–338 — `url_state_raw_new_stores_max_content_length()` |
| MINOR-3 | `UrlStateRaw::new()` missing `last_fetched_ms=u64::MAX` | FIXED | test-plan.md:340–350 — `url_state_raw_new_stores_max_last_fetched_ms()` |
| MINOR-4 | Vague Given: `file_state_raw_new_sets_version_to_1` | FIXED | test-plan.md:218–220 — concrete `content_hash=[0;32], file_size=4096, ...` |
| MINOR-5 | Vague Given: `file_state_raw_new_zeroes_reserved_bytes` | FIXED | test-plan.md:232–233 — concrete `content_hash=[0xFF;32], file_size=u64::MAX, ...` |
| MINOR-6 | Vague Given: `url_state_raw_new_sets_version_to_1` | FIXED | test-plan.md:303–304 — concrete values |
| MINOR-7 | Vague Given: `url_state_raw_new_zeroes_pad_and_reserved_bytes` | FIXED | test-plan.md:316–317 — concrete values |
| MINOR-8 | Vague Given: `file_state_raw_as_bytes_returns_104_byte_slice` | FIXED | test-plan.md:432 — `new([0xAB;32], 4096, 1700000000000, Modified)` |
| MINOR-9 | Vague Given: `url_state_raw_as_bytes_returns_112_byte_slice` | FIXED | test-plan.md:444 — `new([0xCD;32], 200, 8192, 1700000000000, Fresh)` |

---

## Axis 1 — Contract Parity: PASS

### Inventory

| # | Contract Function | Visibility | BDD Scenario(s) | Status |
|---|-------------------|------------|------------------|--------|
| 1 | `FileStateRaw::new()` | pub | §3.2 (B07) — 3 scenarios | COVERED |
| 2 | `FileStateRaw::zeroed()` | crate | §3.2 (B10) — 1 scenario | COVERED |
| 3 | `FileStateRaw::from_bytes_checked()` | pub | §3.6 (B27–B30) — 13 named tests | COVERED |
| 4 | `FileStateRaw::from_bytes_unchecked()` | pub | §3.5 (B25) — 2 scenarios (field match + pointer identity) | COVERED |
| 5 | `FileStateRaw::as_bytes()` | pub | §3.4 (B21) — 1 scenario | COVERED |
| 6 | `FileStateRaw::status()` | pub | §3.9 — 2 scenarios (valid + invalid) | COVERED |
| 7 | `FileStateRaw::validate()` | pub | §3.8 (B39–B40) — 4 scenarios | COVERED |
| 8 | `UrlStateRaw::new()` | pub | §3.3 (B13) — 5 scenarios | COVERED |
| 9 | `UrlStateRaw::zeroed()` | crate | §3.3 (B16) — 1 scenario | COVERED |
| 10 | `UrlStateRaw::from_bytes_checked()` | pub | §3.6 (B31–B34) — 11 named tests | COVERED |
| 11 | `UrlStateRaw::from_bytes_unchecked()` | pub | §3.5 (B26) — 2 scenarios (field match + pointer identity) | COVERED |
| 12 | `UrlStateRaw::as_bytes()` | pub | §3.4 (B22) — 1 scenario | COVERED |
| 13 | `UrlStateRaw::status()` | pub | §3.9 — 2 scenarios (valid + invalid) | COVERED |
| 14 | `UrlStateRaw::validate()` | pub | §3.8 (B41–B42) — 5 scenarios | COVERED |
| 15 | `FileStateStatus::from_discriminant()` | pub | §3.7 (B35–B36) — 8 named tests | COVERED |
| 16 | `UrlStateStatus::from_discriminant()` | pub | §3.7 (B37–B38) — 8 named tests | COVERED |

**16 contract functions. All have ≥1 BDD scenario.**

### Error Variant Completeness: PASS

All 5 `PodStateError` variants have scenarios asserting the **exact variant** with concrete field values:

| Variant | Scenario(s) |
|---------|-------------|
| `InvalidFileStatus(u8)` | §3.6 (:572–586), §3.7 (:739–751), §3.8 (:836–842), §3.9 (:947–953) |
| `InvalidUrlStatus(u8)` | §3.6 (:655–668), §3.7 (:775–786), §3.8 (:883–890), §3.9 (:968–975) |
| `WrongByteSize { type_name, actual, expected }` | §3.6 (:549–565, :631–648) |
| `VersionMismatch { type_name, actual, expected }` | §3.6 (:589–605, :672–686), §3.8 (:847–855, :894–902), §3.10 (:984–1034) |
| `ReservedBytesNonZero { type_name, offset }` | §3.6 (:609–626, :690–712), §3.8 (:858–866, :906–914, :918–927) |

---

## Axis 2 — Assertion Sharpness: PASS

Every "Then:" clause uses concrete expected values. Audit of all 92 named test functions:

- Concrete integers: `returns 104`, `returns 8`, `== 4096`, `== u64::MAX`
- Exact error variants: `Err(PodStateError::WrongByteSize { type_name: "FileStateRaw", actual: 50, expected: 104 })`
- Exact enum values: `Ok(FileStateStatus::Unknown)`, `Err(PodStateError::InvalidFileStatus(4))`
- Concrete byte arrays: `== [0xAB; 32]`, `== [0u8; 54]`

**Zero instances** of `is_ok()`, `is_err()`, `> 0`, or `Some(_)` in any "Then:" clause.

---

## Axis 3 — Trophy Allocation: PASS

### Density

```
Contract pub fn count:   14
BDD test functions:       92
  §3.1  Size & Layout:          6
  §3.2  FileStateRaw construct: 6
  §3.3  UrlStateRaw construct:  8
  §3.4  Byte round-trip:        6
  §3.5  from_bytes_unchecked:   4
  §3.6  from_bytes_checked:    24
  §3.7  Status discriminants:  14
  §3.8  Validate:               9
  §3.9  status() accessor:      4
  §3.10 All-zero/all-FF:        4
  §3.11 Error display:          5
  §3.12 Trait proofs:           2
  --------------------------------
  Total BDD:                   92

Ratio (BDD/pub fn):      92/14 = 6.6x (target ≥5x) — PASS
```

### Additional Coverage

| Category | Count | Status |
|----------|-------|--------|
| Proptest invariants | 8 | PASS — covers round-trips, discriminant full range, validate corruption |
| Fuzz targets | 4 | PASS — `from_bytes_checked` (×2), `from_discriminant` (×2) |
| Kani harnesses | 5 | PASS — size, alignment, field offsets for both structs |

### Proptest Coverage for Pure Functions

| Pure Function | Input Space | Proptest |
|---------------|-------------|----------|
| `FileStateRaw` round-trip | All valid field combos | §4 — PASS |
| `UrlStateRaw` round-trip | All valid field combos | §4 — PASS |
| `FileStateStatus::from_discriminant` | Full u8 range (0..=255) | §4 — PASS |
| `UrlStateStatus::from_discriminant` | Full u8 range (0..=255) | §4 — PASS |
| `FileStateRaw::validate` | All invalid status bytes (4..=255) | §4 — PASS |
| `UrlStateRaw::validate` | All invalid status bytes (4..=255) | §4 — PASS |

### Fuzz Targets for Parsers/Deserializers

| Deserializer | Fuzz Target |
|--------------|-------------|
| `FileStateRaw::from_bytes_checked` | §5 — `file_state_raw_from_bytes` — PASS |
| `UrlStateRaw::from_bytes_checked` | §5 — `url_state_raw_from_bytes` — PASS |
| `FileStateStatus::from_discriminant` | §5 — PASS |
| `UrlStateStatus::from_discriminant` | §5 — PASS |

### Integration/Unit Ratio

Plan acknowledges atypical ratio (pure data types, zero I/O). "Integration" = crossing bytemuck API boundary. Rationale is sound for this bead type.

---

## Axis 4 — Boundary Completeness: PASS

### `FileStateRaw::new()`

| Boundary | Covered | Evidence |
|----------|---------|----------|
| Min valid (all zeros, Unknown) | ✓ | test-plan.md:200–211 |
| Max valid (u64::MAX, 0xFF hash, Deleted) | ✓ | test-plan.md:230–238 |
| All 4 status variants | ✓ | proptest §4 (full cross-product) |
| Reserved always zero | ✓ | test-plan.md:230–238 |
| Version always 1 | ✓ | test-plan.md:218–224 |

### `UrlStateRaw::new()`

| Boundary | Covered | Evidence |
|----------|---------|----------|
| Min valid (all zeros, Unknown) | ✓ | test-plan.md:283–296 |
| Max http_status (u16::MAX) | ✓ | test-plan.md:316–322 |
| Max content_length (u64::MAX) | ✓ | test-plan.md:327–338 |
| Max last_fetched_ms (u64::MAX) | ✓ | test-plan.md:340–350 |
| All 4 status variants | ✓ | proptest §4 |
| Pad/reserved always zero | ✓ | test-plan.md:314–322 |
| Version always 1 | ✓ | test-plan.md:300–308 |

### `FileStateRaw::from_bytes_checked()`

| Boundary | Covered | Evidence |
|----------|---------|----------|
| Empty slice (0 bytes) | ✓ | test-plan.md:558 |
| One byte short (103) | ✓ | test-plan.md:559 |
| One byte over (105) | ✓ | test-plan.md:560 |
| Invalid status: 4, 128, 255 | ✓ | test-plan.md:572–586 |
| Version: 0, 2, 255 | ✓ | test-plan.md:589–605 |
| Reserved: first byte, last byte, all-FF | ✓ | test-plan.md:609–626 |
| All-zero 104 bytes | ✓ | test-plan.md:984–994 |
| All-FF 104 bytes | ✓ | test-plan.md:998–1008 |

### `UrlStateRaw::from_bytes_checked()`

| Boundary | Covered | Evidence |
|----------|---------|----------|
| Empty, one-short (111), one-over (113) | ✓ | test-plan.md:631–648 |
| Invalid status: 5, 200, 255 | ✓ | test-plan.md:655–668 |
| Version: 0, 99 | ✓ | test-plan.md:672–686 |
| _pad1 non-zero | ✓ | test-plan.md:690–698 |
| Reserved non-zero (offset 54) | ✓ | test-plan.md:700–712 |
| All-zero 112 bytes | ✓ | test-plan.md:1012–1020 |
| All-FF 112 bytes | ✓ | test-plan.md:1024–1034 |

### `FileStateRaw::validate()`

| Boundary | Covered | Evidence |
|----------|---------|----------|
| Valid instance → Ok(()) | ✓ | test-plan.md:825–831 |
| Corrupted status (99) | ✓ | test-plan.md:836–842 |
| Wrong version (0) | ✓ | test-plan.md:847–855 |
| Non-zero reserved | ✓ | test-plan.md:858–866 |

### `UrlStateRaw::validate()`

| Boundary | Covered | Evidence |
|----------|---------|----------|
| Valid instance → Ok(()) | ✓ | test-plan.md:870–878 |
| Corrupted status (99) | ✓ | test-plan.md:883–890 |
| Wrong version (0) | ✓ | test-plan.md:894–902 |
| Non-zero _pad1 | ✓ | test-plan.md:906–914 |
| Non-zero reserved (offset 54) | ✓ | test-plan.md:918–927 |

### `from_bytes_unchecked()` (both structs)

| Boundary | Covered | Evidence |
|----------|---------|----------|
| Valid bytes → field match | ✓ | test-plan.md:479–493, :497–513 |
| Pointer identity preserved | ✓ | test-plan.md:517–528, :531–542 |

### `from_discriminant()` (both enums)

| Boundary | Covered | Evidence |
|----------|---------|----------|
| All 4 valid values (0–3) | ✓ | test-plan.md:717–733, :754–770 |
| Invalid: 4, 255 | ✓ | test-plan.md:739–751, :775–786 |
| Exhaustive u8 range | ✓ | proptest §4 |

---

## Axis 5 — Mutation Survivability: PASS

### 29 Explicit Mutations — All Killed

| # | Mutation | Caught By |
|---|----------|-----------|
| 1 | `FileStateRaw::new()` sets `version=0` | `file_state_raw_new_sets_version_to_1` |
| 2 | `FileStateRaw::new()` sets `version=2` | `file_state_raw_new_sets_version_to_1` |
| 3 | `FileStateRaw::new()` doesn't zero `reserved` | `file_state_raw_new_zeroes_reserved_bytes` |
| 4 | `UrlStateRaw::new()` doesn't zero `_pad1` | `url_state_raw_new_zeroes_pad_and_reserved_bytes` |
| 5 | `UrlStateRaw::new()` doesn't zero `reserved` | `url_state_raw_new_zeroes_pad_and_reserved_bytes` |
| 6 | Remove size check from `from_bytes_checked` | wrong-size tests (§3.6) |
| 7 | Remove version check from `from_bytes_checked` | version rejection tests (§3.6) |
| 8 | Remove status check from `from_bytes_checked` | invalid status tests (§3.6) |
| 9 | Remove reserved check from `from_bytes_checked` | nonzero reserved tests (§3.6) |
| 10 | Remove `_pad1` check (Url) | `url_state_raw_from_bytes_checked_rejects_nonzero_pad1_bytes` |
| 11 | `from_discriminant` accepts value 4 | `from_discriminant_4` tests |
| 12 | `from_discriminant` accepts value 255 | `from_discriminant_255` tests |
| 13 | Swap `Unchanged`/`Modified` discriminants | per-variant discriminant tests |
| 14 | `as_bytes()` returns wrong length | explicit length tests (§3.4) |
| 15 | WrongByteSize reports wrong `expected` | `pod_state_error_wrong_byte_size_displays_correct_message` |
| 16 | VersionMismatch reports wrong `actual` | `pod_state_error_version_mismatch_displays_correct_message` |
| 17 | Delete `FileStateRaw::from_bytes_unchecked()` body | `file_state_raw_from_bytes_unchecked_returns_reference_to_same_bytes` |
| 18 | `from_bytes_unchecked` returns wrong reference (File) | `file_state_raw_from_bytes_unchecked_preserves_pointer_identity` |
| 19 | Delete `UrlStateRaw::from_bytes_unchecked()` body | `url_state_raw_from_bytes_unchecked_returns_reference_to_same_bytes` |
| 20 | `from_bytes_unchecked` returns wrong reference (Url) | `url_state_raw_from_bytes_unchecked_preserves_pointer_identity` |
| 21 | Remove version check in `UrlStateRaw::validate()` | `url_state_raw_validate_rejects_wrong_version` |
| 22 | Remove reserved check in `UrlStateRaw::validate()` | `url_state_raw_validate_rejects_nonzero_reserved` |
| 23 | Remove status check in `UrlStateRaw::validate()` | `url_state_raw_validate_rejects_corrupted_status` |
| 24 | Remove `_pad1` check in `UrlStateRaw::validate()` | `url_state_raw_validate_rejects_nonzero_pad1` |
| 25 | Remove version check in `FileStateRaw::validate()` | `file_state_raw_validate_rejects_wrong_version` |
| 26 | Remove status check in `FileStateRaw::validate()` | `file_state_raw_validate_rejects_corrupted_status` |
| 27 | Remove reserved check in `FileStateRaw::validate()` | `file_state_raw_validate_rejects_nonzero_reserved` |
| 28 | Swap `content_length`/`last_fetched_ms` in `UrlStateRaw::new()` | `url_state_raw_new_returns_correct_field_values` (different concrete values) |
| 29 | Truncate `content_length` on construction | `url_state_raw_new_stores_max_content_length` |

**Kill rate: 29/29 = 100%.** Additional mutations from thought experiment:

- Reorder validation checks → killed by all-FF tests pinned to `VersionMismatch` (§3.10, explicit ordering convention at line 982)
- Swap `file_size`/`last_modified_ms` in `FileStateRaw::new()` → killed by `file_state_raw_new_returns_correct_field_values` (values 4096 vs 1700000000000)
- `status()` returns hardcoded variant → killed by specific discriminant test + invalid discriminant test
- Delete entire `validate()` body (return `Ok(())`) → killed by all rejection tests for both structs

---

## Axis 6 — Holzmann Plan Audit: PASS

### Rule 1 (Linear): PASS
All 92 scenarios follow Given → When → Then. No nested conditionals.

### Rule 2 (Bound loops): PASS
No loops in any test body. Each discriminant value tested via individual named function.

### Rule 3 (Own resources): PASS
No I/O, no filesystem, no network. Pure data types. Zero resource management needed.

### Rule 4 (One function, one job): PASS
Each test proves exactly one behavior. Test names describe the single assertion.

### Rule 5 (State assumptions): PASS
All Given blocks now specify concrete preconditions:
- Construction tests: explicit field values (e.g., `content_hash=[0xFF;32], file_size=u64::MAX, ...`)
- Validation tests: explicit corruption (e.g., `version == 0`, `reserved[0] == 1`)
- Type-property tests: explicit type references (e.g., `FileStateRaw implements Zeroable`)
- No instances of "any valid inputs" or "any instance" remain.

### Rule 6 (Never swallow): PASS
No `let _ =` or `.ok()` patterns. `unwrap()` in When blocks is setup-only, not the assertion.

### Rule 7 (Narrow state): PASS
No shared mutable state. Each test constructs its own instances.

### Rule 8 (Surface side effects): PASS
No side effects in any test.

### Rule 9 (One layer of magic): PASS
No helper abstraction chains. Each test is self-contained.

### Rule 10 (Warnings as errors): N/A
No implementation code yet. Applies in Mode 2.

---

## Severity Summary

| Severity | Count | Threshold | Triggered? |
|----------|-------|-----------|------------|
| LETHAL | 0 | Any single | NO |
| MAJOR | 0 | ≥ 3 | NO |
| MINOR | 0 | ≥ 5 | NO |

**Result: 0 LETHAL + 0 MAJOR + 0 MINOR = APPROVED**

---

## LETHAL FINDINGS

None.

## MAJOR FINDINGS (0)

None.

## MINOR FINDINGS (0/5 threshold)

None.

---

## Documentation Notes (non-blocking)

1. **Behavior inventory count**: Section 1 summary says "44 behaviors identified" but the numbered inventory runs B01–B43 (43 entries). The actual BDD section (§3) contains 92 named test functions. The discrepancy is cosmetic — Section 3 is the authoritative specification and is complete.

2. **Trophy allocation counts**: Section 2 table says "33 unit / 10 integration / 2 static". Actual named test functions in Section 3 exceed these counts due to boundary expansions listed under "Additional boundaries" / "Test names:" sub-blocks. Not a coverage gap — the boundary expansions are fully specified.

---

## MANDATE

None. The test plan is complete and approved for implementation.

### Quality Summary

- **Contract parity**: 16/16 pub functions covered. 5/5 error variants tested with exact assertions.
- **Assertion sharpness**: 92/92 scenarios use concrete expected values. Zero `is_ok()`/`is_err()`.
- **Density**: 92 BDD tests / 14 pub fn = 6.6x (exceeds 5x threshold).
- **Boundary coverage**: All critical boundaries explicitly named per function.
- **Mutation kill rate**: 100% (29/29 identified mutations killed).
- **Holzmann compliance**: All 10 applicable rules satisfied.
