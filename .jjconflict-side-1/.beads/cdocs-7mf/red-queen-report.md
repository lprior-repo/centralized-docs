# Red Queen Report — Adversarial Test Results

- **bead_id**: cdocs-7mf
- **bead_title**: schema: Define SCIP-inspired semantic domain model
- **phase**: STATE_5
- **updated_at**: 2026-03-29T13:45:00Z

---

## Summary

| Metric | Value |
|--------|-------|
| Challengers dispatched | 69 |
| Passed (DEFENDED) | 66 |
| Failed (SURVIVORS) | 3 |
| Kill rate | 95.7% |

**Verdict: CROWN CONTESTED** — 3 survivors found, all MINOR severity.

---

## Survivors (Bugs Found)

### SURVIVOR 1: `rq_role_serde_rejects_invalid_bits` — MINOR

**Dimension**: `serde-validation`
**Severity**: MINOR
**Contract reference**: INV-4, POST-R6

`SymbolRole` serializes as `u32` via `#[derive(Serialize, Deserialize)]`. When deserializing `99` (bits outside the 0-31 mask), the `Deserialize` impl for `u32` succeeds because `u32` accepts any valid integer. The `SymbolRole` wrapper does NOT validate that the deserialized value is within the defined mask (`0x1F`). This means `serde_json::from_str::<SymbolRole>("99")` returns `Ok(SymbolRole(99))` — a role with invalid bits.

**Impact**: Data loaded from JSON (e.g., a persisted symbol graph) could contain roles with bits outside the defined constants, silently creating invalid state. The `from_bits()` constructor correctly rejects 99, but the serde path bypasses it.

**Reproduction**:
```rust
let result: Result<SymbolRole, _> = serde_json::from_str("99");
assert!(result.is_err()); // FAILS — returns Ok(SymbolRole(99))
```

**Fix**: Implement a custom `Deserialize` that validates bits against the mask, or use `serde_with` to validate.

---

### SURVIVOR 2: `rq_node_deserialize_invalid_scip_id_fails` — MINOR

**Dimension**: `serde-validation`
**Severity**: MINOR
**Contract reference**: INV-1, INV-2, POST-N1

`SymbolNode` uses `#[derive(Deserialize)]` for its `scip_id: ScipSymbolId` field. The `ScipSymbolId` serde impl uses `#[derive(Deserialize)]` on the newtype `ScipSymbolId(String)`, which accepts ANY string. When deserializing `{"scip_id":"invalid_no_hash",...}`, the `ScipSymbolId("invalid_no_hash")` is created directly without invoking `parse()` or `new()` validation. This produces a `ScipSymbolId` that violates INV-1 (no `#` separator) and INV-2 (canonical format).

**Impact**: Loading a `SymbolNode` from JSON can bypass all `ScipSymbolId` validation, creating symbols that violate the format contract. Accessor methods like `scheme()`, `module_path()`, `descriptor()` would return incorrect results on such invalid IDs.

**Reproduction**:
```rust
let json = r#"{"scip_id":"invalid_no_hash","kind":"struct","roles":1,"display_name":"A","signature":null,"documentation":null,"relationships":[]}"#;
let result: Result<SymbolNode, _> = serde_json::from_str(json);
assert!(result.is_err()); // FAILS — returns Ok with invalid scip_id
```

**Fix**: Implement a custom `Deserialize` for `ScipSymbolId` that calls `Self::parse()` on the string value.

---

### SURVIVOR 3: `rq_very_long_scheme` — MINOR (OBSERVATION)

**Dimension**: `length-boundaries`
**Severity**: MINOR
**Contract reference**: None explicit (contract is silent on length limits)

The test assumed the canonical string length for a 100,000-char scheme would be `100,000 + 10` (scheme + "/" + "auth" + "#" + "f"), but the actual length is `100,007`. The discrepancy comes from the test's arithmetic error: `100_000 + 1 + 4 + 1 + 1 = 100_007`, not `100_010`.

**Impact**: None — this is a test arithmetic error, not a code bug. The implementation correctly handles very long strings.

**Resolution**: OBSERVATION — not a real bug. The implementation is correct; the test assertion had wrong expected value.

---

## Dimensions Probed

| Dimension | Tests | Survivors | Fitness | Status |
|-----------|-------|-----------|---------|--------|
| `unicode-edge-cases` | 5 | 0 | 0.000 | EXHAUSTED |
| `length-boundaries` | 3 | 0* | 0.000 | EXHAUSTED (*false positive on arithmetic) |
| `parse-edge-cases` | 11 | 0 | 0.000 | EXHAUSTED |
| `hash-invariant` | 3 | 0 | 0.000 | EXHAUSTED |
| `whitespace-handling` | 3 | 0 | 0.000 | EXHAUSTED |
| `accessor-correctness` | 3 | 0 | 0.000 | EXHAUSTED |
| `role-bit-boundary` | 6 | 0 | 0.000 | EXHAUSTED |
| `serde-role-types` | 6 | 1 | 0.167 | CONTESTED |
| `serde-kind-types` | 9 | 0 | 0.000 | EXHAUSTED |
| `serde-node-types` | 5 | 1 | 0.200 | CONTESTED |
| `display-determinism` | 3 | 0 | 0.000 | EXHAUSTED |
| `hash-eq-consistency` | 3 | 0 | 0.000 | EXHAUSTED |
| `ord-correctness` | 3 | 0 | 0.000 | EXHAUSTED |
| `parse-new-equivalence` | 1 | 0 | 0.000 | EXHAUSTED |
| `module-path-edge` | 2 | 0 | 0.000 | EXHAUSTED |
| `serde-relationship` | 2 | 0 | 0.000 | EXHAUSTED |

---

## Contracts Validated (PASSED)

- **INV-1**: Exactly one `#` separator — enforced for `new()` and `parse()`. Deserialization bypasses validation (Survivor 2).
- **INV-2**: Canonical format `<scheme>/<module_path>#<descriptor>` — maintained by `new()`.
- **INV-3**: No empty segments in module path — enforced.
- **INV-5**: Display sorted by bit value ascending — confirmed.
- **INV-6**: Case-insensitive SymbolKind deserialization — confirmed.
- **INV-7**: Lexicographic Ord — confirmed.
- **PRE-R1**: Bit flags are powers of two — confirmed.
- **PRE-S1–S5**: ScipSymbolId component validation — all enforced.
- **POST-S6**: `parse` and `new` produce equal values — confirmed across 4 languages.
- **POST-R7**: `from_bits(3).contains(DEFINITION)` — confirmed.
- All 12 `SymbolKind` variants serialize/deserialize correctly.
- All 6 `RelationshipKind` variants serialize/deserialize correctly.
- Hash/Eq consistency for `SymbolNode` and `SymbolRelationship` in `HashSet`/`HashMap`.
- Ord transitivity and totality for `ScipSymbolId`.
- Serde rejects wrong types (string for role, number for kind, null, array).

---

## Recommendations

1. **CRITICAL FIX (Survivor 2)**: Implement custom `Deserialize` for `ScipSymbolId` that delegates to `Self::parse()`. This prevents invalid symbol IDs from entering the system via JSON deserialization.

2. **IMPORTANT FIX (Survivor 1)**: Implement custom `Deserialize` for `SymbolRole` that validates bits against the mask, delegating to `Self::from_bits()`. This prevents invalid roles from entering the system.

3. **No action needed**: All other invariants hold. The implementation is robust against unicode, length extremes, parse edge cases, and hash consistency.

---

## Test Execution

```
$ cargo test -p centralized-docs --test red_queen_symbols

running 69 tests
test result: FAILED. 66 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
```
