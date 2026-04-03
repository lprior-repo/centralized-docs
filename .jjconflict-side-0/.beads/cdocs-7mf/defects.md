# Defects Report — Black-Hat Code Review

- **bead_id**: cdocs-7mf
- **bead_title**: schema: Define SCIP-inspired semantic domain model
- **phase**: STATE_5.5
- **updated_at**: 2026-03-29T20:15:00Z
- **reviewer**: test-reviewer (adversarial)
- **review_mode**: Suite Inquisition (Mode 2)

---

## VERDICT: REJECTED

6 LETHAL findings. Rewrite, not patch. The serde bypasses alone invalidate INV-1, INV-2, and INV-4.

---

## Phase 1 — Contract Invariant Audit

### [FAIL] INV-1 / INV-2 — ScipSymbolId serde deserialization bypass

**Severity**: LETHAL
**Location**: `symbols.rs:55`
**Contract references**: INV-1, INV-2, POST-S7

`#[derive(Deserialize)]` on `ScipSymbolId(String)` delegates to `String`'s deserializer, which accepts ANY string. This completely bypasses the `new()` and `parse()` validation boundary. A deserialized `ScipSymbolId` can contain zero `#` separators, multiple `#` separators, empty schemes, empty descriptors — anything.

```
serde_json::from_str::<ScipSymbolId>("\"garbage\"")  → Ok(ScipSymbolId("garbage"))
serde_json::from_str::<ScipSymbolId>("\"a#b#c\"")    → Ok(ScipSymbolId("a#b#c"))
```

The invariants INV-1 ("exactly one `#` separator") and INV-2 ("always matches canonical format") are lies when serde is the construction path. Every `SymbolNode` deserialized from JSON inherits this corruption via its `scip_id` field.

**Fix**: Remove `Deserialize` from the derive list. Implement custom `Deserialize<'de>` that calls `Self::parse()`, mapping `ScipSymbolIdError` to `serde::de::Error::custom`.

---

### [FAIL] INV-4 — SymbolRole serde deserialization bypass

**Severity**: LETHAL
**Location**: `symbols.rs:248`
**Contract references**: INV-4, POST-R6

`#[derive(Deserialize)]` on `SymbolRole(u32)` delegates to `u32`'s deserializer, which accepts any `u32`. This bypasses `from_bits()` mask validation. The defined mask is `0x1F` (bits 0-4), but `serde_json::from_str::<SymbolRole>("99")` succeeds, creating `SymbolRole(99)` with bits 1, 2, 5, and 6 set — three bits outside the defined constants.

```
serde_json::from_str::<SymbolRole>("99")   → Ok(SymbolRole(99))   // bits 1,2,32,64
serde_json::from_str::<SymbolRole>("255")  → Ok(SymbolRole(255))  // 8 undefined bits
```

INV-4 states "No invalid bit patterns can exist outside the defined constants." This is false for deserialized values. The `from_bits()` constructor correctly rejects these, but serde never calls it.

**Fix**: Remove `Deserialize` from the derive list. Implement custom `Deserialize<'de>` that deserializes a `u32` then delegates to `Self::from_bits()`, mapping `SymbolRoleError` to `serde::de::Error::custom`.

---

## Phase 2 — Implementation Correctness

### [FAIL] Accessor `unwrap_or("")` silently masks invalid state

**Severity**: MAJOR
**Location**: `symbols.rs:182`, `symbols.rs:198`, `symbols.rs:207`

`scheme()`, `module_path()`, and `descriptor()` all fall back to `unwrap_or("")` when the internal string doesn't match expected format. The implementation summary claims these use `expect()` with invariant assertions — that is inaccurate. The actual code silently returns empty strings.

This is a defense-in-depth failure: the serde bypass (Phase 1) creates invalid `ScipSymbolId` values, and the accessors silently return garbage instead of making the corruption visible. A consumer calling `.scheme()` on a deserialized `ScipSymbolId("garbage")` gets `""` with no error signal.

**Fix**: Replace `unwrap_or("")` with `expect("INV-1/INV-2 violated: internal string format is corrupt")` in all three accessors. This makes invalid state loud. Alternatively, return `Option<&str>` — but the contract specifies `&str` return types, so `expect` is the correct choice.

---

### [FAIL] Descriptor `#` validation returns wrong error variant

**Severity**: MAJOR
**Location**: `symbols.rs:103-104`

When `validate_descriptor` detects a `#` in the descriptor, it returns `Err(ScipSymbolIdError::InvalidScheme("#".to_string()))`. The error says "SCIP scheme contains invalid character" but the trigger is in the descriptor. This misleads debugging — a consumer seeing `InvalidScheme` will investigate the scheme component, which is correct.

**Fix**: Either add a dedicated `HashInDescriptor` variant to `ScipSymbolIdError`, or return `InvalidFormat(s.to_string())` with a clear message. The current variant assignment is semantically wrong.

---

## Phase 3 — Error Taxonomy & Test Sharpness

### [FAIL] `assert!(result.is_err())` — banned assertion pattern

**Severity**: LETHAL
**Location**: `symbols.rs:682-685`

```rust
assert!(
    result.is_err(),
    "descriptor containing '#' must be rejected to preserve INV-1"
);
```

`is_err()` does not assert the error variant. If the code changed to return a different error (or `Ok(())` with different validation logic), this test would still pass or fail for the wrong reason. The test must assert `Err(ScipSymbolIdError::InvalidScheme("#".to_string()))` (or whatever the correct variant should be after fixing the Phase 2 defect above).

---

### [FAIL] `assert!(result.is_err())` — banned assertion pattern

**Severity**: LETHAL
**Location**: `symbols.rs:1347-1350`

```rust
assert!(
    result.is_err(),
    "unknown variant should produce deserialization error"
);
```

Same violation. Does not assert the exact error. Must assert the error message contains the expected content at minimum, or better, check the serde error kind.

---

### [FAIL] Loop in test body — Holzmann Rule 2

**Severity**: LETHAL
**Location**: `symbols.rs:1307-1311`

```rust
fn symbol_kind_round_trips_through_json_for_each_variant() {
    let variants = [ /* ... */ ];
    for kind in variants {           // ← loop in test body
        let json = serde_json::to_string(&kind).unwrap();
        let back: SymbolKind = serde_json::from_str(&json).unwrap();
        assert_eq!(back, kind, "round-trip failed for {kind:?}");
    }
}
```

A loop in a test body is a program with hidden logic. If the second variant fails, the first variant's `to_string`/`from_str` has already mutated shared state (not applicable here, but the pattern is banned regardless). Use `rstest` cartesian product or split into 12 individual tests.

---

### [FAIL] Loop in test body — Holzmann Rule 2

**Severity**: LETHAL
**Location**: `symbols.rs:1363-1367`

Same pattern in `relationship_kind_round_trips_through_json_for_each_variant`. Split into individual tests or use `rstest`.

---

### [WARN] `expect_err` + `contains` — imprecise error assertion

**Severity**: MINOR
**Location**: `symbols.rs:1256-1261`, `symbols.rs:1267-1273`

```rust
let err = result.expect_err("should fail for unknown kind");
let msg = err.to_string();
assert!(msg.contains("unknown_kind"), ...);
```

These are serde errors, not domain errors, so exact variant matching is impractical. However, the `contains` check should be strengthened to assert the full expected prefix `"Unknown SymbolKind: "` to reduce false positives.

---

## Phase 4 — Test Suite Structural Assessment

### [PASS] Density audit

- Public functions: 17 (`new`, `parse`, `as_str`, `scheme`, `module_path`, `descriptor`, `into_string`, `from_bits`, `from_bits_truncate`, `empty`, `bits`, `contains`, `is_empty`, `union`, `intersection`, `as_str` [SymbolKind], `from_str_ci`)
- Test count: 124 (85 unit + 9 proptest + 30 error display)
- Ratio: 7.3x — exceeds 5x target

### [PASS] Error variant completeness

All 10 `ScipSymbolIdError` variants have tests asserting exact variant equality. `SymbolRoleError::UnknownBit` has exact assertion. `SymbolKindError::UnknownKind` has message-contains assertion (acceptable for serde-wrapped errors).

### [PASS] Holzmann Rule 7 — No shared mutable state

No `static mut`, `lazy_static`, or `Mutex<RwLock>` in test code.

### [PASS] Holzmann Rule 1 — Linear flow

All test bodies follow Given/When/Then. No nested conditionals in test logic.

### [WARN] Missing `// Given` comments

**Severity**: MINOR
**Location**: `symbols.rs:928-934`, `symbols.rs:964-967`, `symbols.rs:980-985`

Several tests skip explicit `// Given` blocks, violating Holzmann Rule 5 (state your assumptions).

---

## Phase 5 — Serde Boundary Analysis (Red Queen Corroboration)

The Red Queen found 2 real defects (1 false positive on test arithmetic). This review independently confirms both:

| Red Queen Survivor | This Review | Agreement |
|---|---|---|
| SURVIVOR 1: SymbolRole serde accepts any u32 | Phase 1, INV-4 | Confirmed LETHAL |
| SURVIVOR 2: ScipSymbolId serde accepts any string | Phase 1, INV-1/INV-2 | Confirmed LETHAL |
| SURVIVOR 3: Long scheme arithmetic | N/A | False positive, not a code bug |

Additionally, this review identifies 4 further LETHAL findings the Red Queen did not flag (banned test patterns, accessor fallbacks).

---

## Finding Summary

| # | Severity | ID | Location | Summary |
|---|----------|----|----------|---------|
| 1 | LETHAL | D-01 | `symbols.rs:55` | ScipSymbolId `Deserialize` bypasses all validation |
| 2 | LETHAL | D-02 | `symbols.rs:248` | SymbolRole `Deserialize` bypasses mask validation |
| 3 | LETHAL | D-03 | `symbols.rs:682` | `assert!(is_err())` — banned assertion pattern |
| 4 | LETHAL | D-04 | `symbols.rs:1347` | `assert!(is_err())` — banned assertion pattern |
| 5 | LETHAL | D-05 | `symbols.rs:1307` | Loop in test body (Holzmann Rule 2) |
| 6 | LETHAL | D-06 | `symbols.rs:1363` | Loop in test body (Holzmann Rule 2) |
| 7 | MAJOR | D-07 | `symbols.rs:182,198,207` | `unwrap_or("")` silently masks invalid state |
| 8 | MAJOR | D-08 | `symbols.rs:103-104` | Descriptor `#` returns wrong error variant |
| 9 | MINOR | D-09 | `symbols.rs:1256,1268` | Imprecise error message assertion |
| 10 | MINOR | D-10 | `symbols.rs:928,964,980` | Missing `// Given` comments |

**Totals**: 6 LETHAL / 2 MAJOR / 2 MINOR

---

## MANDATE

Before resubmission, ALL of the following must exist:

1. **Custom `Deserialize` for `ScipSymbolId`** — must delegate to `Self::parse()`, rejecting invalid strings. Add test: `serde_json::from_str::<ScipSymbolId>("\"garbage\"")` returns `Err`.
2. **Custom `Deserialize` for `SymbolRole`** — must delegate to `Self::from_bits()`, rejecting invalid bits. Add test: `serde_json::from_str::<SymbolRole>("99")` returns `Err`.
3. **Replace `unwrap_or("")` with `expect`** in `scheme()`, `module_path()`, `descriptor()` — invariant violation must be loud.
4. **Fix descriptor `#` error variant** — return correct error type, not `InvalidScheme`.
5. **Replace `assert!(is_err())`** at lines 682 and 1347 with exact error variant assertions.
6. **Split loop-tests** at lines 1307 and 1363 into individual `#[test]` functions or `rstest` cases.
7. **Add serde rejection tests** — explicit tests proving that deserialization rejects invalid ScipSymbolId strings, invalid SymbolRole bit patterns, and invalid SymbolNode payloads.

Resubmit for full re-review from Phase 1.

---

STATUS: REJECTED
