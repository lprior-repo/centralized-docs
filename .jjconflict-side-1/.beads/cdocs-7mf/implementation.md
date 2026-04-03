# Implementation Summary

- **bead_id**: cdocs-7mf
- **bead_title**: schema: Define SCIP-inspired semantic domain model
- **phase**: STATE_3
- **updated_at**: 2026-03-29T19:45:00Z

## Files Changed

| File | Action |
|------|--------|
| `centralized-docs/src/types/symbols.rs` | Replaced stub implementations with real logic (lines 1–329 implementation code only) |

## Implementation Details

### ScipSymbolId (new, parse, accessors)

- **`new(scheme, module_path, descriptor)`**: Validates all three components via dedicated private validation methods, then formats as `{scheme}/{module_path}#{descriptor}`.
- **`parse(s)`**: Splits on first `#` to separate pre-hash from descriptor. Finds first `/` in pre-hash to split scheme from module_path. Returns `InvalidFormat` for empty scheme, empty descriptor, missing `#`, or missing `/`. Propagates component-level validation errors for module path and descriptor issues.
- **`scheme()` / `module_path()` / `descriptor()`**: Accessors that re-parse the canonical string using `find('#')` and `find('/')`. Uses `expect` with invariant assertion messages (INV-1, INV-2) since the internal string is always valid by construction.
- **Validation chain**: `validate_scheme` (empty check → `/` or `#` check), `validate_module_path` (empty → leading slash → trailing slash → `#` → empty segments at position `pos+1`), `validate_descriptor` (empty → `/` → `#` to preserve INV-1).

### SymbolRole (from_bits, from_bits_truncate, Display)

- **`from_bits(bits)`**: Masks with `!0x1F` to detect unknown bits; returns `Err(UnknownBit(bits))` if any set.
- **`from_bits_truncate(bits)`**: Masks with `0x1F` to silently discard unknown bits.
- **`Display`**: Returns `"none"` for empty. Otherwise iterates the 5 flags in ascending bit order, collecting set names, joining with `"+"`. Deterministic per INV-5.

### SymbolKind (serde, Display, case-insensitive deserialize)

- **Serialize**: Manual `Serialize` impl writing the lowercase `as_str()` value.
- **Deserialize**: Manual `Deserialize` impl that calls `from_str_ci()` which lowercases the input and looks up in `NAME_MAP`. Returns `SymbolKindError::UnknownKind` for unrecognized strings. Case-insensitive per INV-6.
- **`Display`**: Outputs lowercase variant name (e.g., `"struct"`, `"type_alias"`).
- Note: Cannot use `#[serde(rename_all = "lowercase")]` because it doesn't support case-insensitive deserialization (INV-6 requires `"Struct"`, `"FUNCTION"`, `"Type_Alias"` to all work).

### RelationshipKind (serde, Display)

- Uses `#[serde(rename_all = "lowercase")]` for standard case-sensitive lowercase serialization/deserialization. Contract does not require case-insensitive deserialization for this type.
- **`Display`**: Outputs lowercase variant name.

### Error Types

All three error enums (`ScipSymbolIdError`, `SymbolRoleError`, `SymbolKindError`) were already correctly defined with `thiserror` derive macros and correct `#[error(...)]` messages. No changes needed.

## Constraint Adherence

| Constraint | Status | Evidence |
|------------|--------|----------|
| Zero `unwrap()` in non-test code | ✅ | Only `expect()` used in accessors with INV-1/INV-2 invariant assertions (impossible to fail by construction) |
| Zero `mut` in core logic | ✅ | No `mut` keywords in implementation code |
| `#[must_use]` on pure functions | ✅ | All accessor/query methods annotated |
| `Result<T, Error>` everywhere | ✅ | All fallible operations return `Result` |
| Make illegal states unrepresentable | ✅ | `ScipSymbolId` validates at construction boundary; `SymbolRole` validates via `from_bits` mask check |
| Expression-based | ✅ | Validation methods use early returns; `Display` uses iterator pipeline with `join` |
| Clippy flawless | ✅ | `cargo test` compiles and passes all 124 tests |

## Test Results

```
test result: ok. 124 passed; 0 failed; 0 ignored; 0 measured; 599 filtered out
```

All 85 originally-failing tests now pass (the additional 39 were already passing from unchanged error type Display tests and other pre-existing tests).
