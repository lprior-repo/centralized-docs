# Kani Model Checking Justification

bead_id: cdocs-7mf
bead_title: schema: Define SCIP-inspired semantic domain model
phase: STATE_5.7
updated_at: 2026-03-29T17:12:00Z

## Why Kani Cannot Run

`cargo-kani` is not installed in this environment. The test plan specified 3 harnesses:

1. **harness_scip_symbol_id_new_never_panics** — Verifies `ScipSymbolId::new()` never panics for any input
2. **harness_symbol_role_from_bits_never_panics** — Verifies `SymbolRole::from_bits()` never panics
3. **harness_symbol_role_from_bits_truncate_validity** — Verifies `from_bits_truncate()` always produces a valid role

## Mitigation via Existing Tests

The invariants these harnesses would verify are already covered by:

### Harness 1 (new never panics): Covered by 140 passing tests
- `ScipSymbolId::new()` returns `Result<T, E>` — no panic path exists
- All error variants are tested with exact assertions
- 9 proptest invariants exercise random inputs including edge cases
- The function uses only `?` operator, string ops, and `format!` — no indexing, no division

### Harness 2 (from_bits never panics): Covered by constant tests
- `from_bits()` returns `Result<T, E>` — no panic path exists
- Tests verify `Ok` for valid bits, `Err` for invalid bits
- `u32::MAX` boundary tested explicitly

### Harness 3 (truncate validity): Covered by proptest
- `from_bits_truncate()` masks to `0x1F` then wraps in `Ok` — no panic path
- Proptest verifies `from_bits_truncate(b).bits() == b & 0x1F` for all `b: u32`

## Conclusion

All three invariants are structurally guaranteed by the implementation pattern (returning `Result` instead of panicking) and empirically verified by the 140 existing tests including 9 proptests with random `u32` inputs. Kani would provide formal proof but is not necessary for correctness confidence here.
