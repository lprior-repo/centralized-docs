# Implementation Summary: cdocs-h70

## Status: COMPLETE

## What was implemented

Fixed-size `#[repr(C)]` Pod struct types for zero-copy redb cache state storage, replacing serde_json serialization overhead with bytemuck byte-casting.

## Files Changed/Created

### New crate: `centralized-docs-pod/`

| File | Purpose |
|------|---------|
| `Cargo.toml` | Crate manifest with `bytemuck` (derive + min_const_generics) and `thiserror` |
| `src/lib.rs` | All Pod types, status enums, error types, constructors, validators, tests (~1460 lines) |

### Modified in `centralized-docs/`

| File | Change |
|------|--------|
| `Cargo.toml` | Added `centralized-docs-pod` workspace dependency |
| `src/lib.rs` | Added `pub mod state;` |
| `src/state/mod.rs` | Re-exports from `centralized-docs-pod` |
| `src/errors/mod.rs` | Added `PodState(#[from] PodStateError)` variant to `DocTransformerError` |

### Modified in workspace root

| File | Change |
|------|--------|
| `Cargo.toml` | Added `centralized-docs-pod` to workspace members |

## Types Defined

### `FileStateRaw` (104 bytes, alignment 8)

| Offset | Size | Field | Type |
|--------|------|-------|------|
| 0 | 32 | `content_hash` | `[u8; 32]` |
| 32 | 8 | `file_size` | `u64` |
| 40 | 8 | `last_modified_ms` | `u64` |
| 48 | 1 | `version` | `u8` |
| 49 | 1 | `status` | `u8` |
| 50 | 54 | `reserved` | `[u8; 54]` |

Zero compiler-inserted padding. Verified: `size_of == 104`, `align_of == 8`.

### `UrlStateRaw` (112 bytes, alignment 8)

| Offset | Size | Field | Type |
|--------|------|-------|------|
| 0 | 32 | `content_hash` | `[u8; 32]` |
| 32 | 2 | `http_status` | `u16` |
| 34 | 6 | `_pad1` | `[u8; 6]` |
| 40 | 8 | `content_length` | `u64` |
| 48 | 8 | `last_fetched_ms` | `u64` |
| 56 | 1 | `version` | `u8` |
| 57 | 1 | `status` | `u8` |
| 58 | 54 | `reserved` | `[u8; 54]` |

Zero compiler-inserted padding. Verified: `size_of == 112`, `align_of == 8`.

### `FileStateStatus` (`#[repr(u8)]`)

Variants: `Unknown(0)`, `Unchanged(1)`, `Modified(2)`, `Deleted(3)`.

### `UrlStateStatus` (`#[repr(u8)]`)

Variants: `Unknown(0)`, `Fresh(1)`, `Stale(2)`, `Error(3)`.

### `PodStateError` (`#[non_exhaustive]`)

Variants: `InvalidFileStatus(u8)`, `InvalidUrlStatus(u8)`, `WrongByteSize{type_name, actual, expected}`, `VersionMismatch{type_name, actual, expected}`, `ReservedBytesNonZero{type_name, offset}`.

## Contract Deviations

### Critical Fix: UrlStateRaw `_pad1` size

The contract specified `_pad1: [u8; 2]` at offset 34, with `content_length: u64` at offset 36. This is **physically impossible** with `#[repr(C)]` because `u64` requires 8-byte alignment — offset 36 is not 8-byte aligned (36 % 8 = 4). The compiler would insert 4 hidden padding bytes, breaking the Pod invariant.

**Fix applied**: Changed `_pad1` from `[u8; 2]` to `[u8; 6]`, placing `content_length` at offset 40 (40 % 8 = 0). Adjusted `reserved` from `[u8; 58]` to `[u8; 54]` to maintain total size of 112 bytes. This preserves the total struct size and field order while eliminating all compiler-inserted padding.

### Architecture: Separate crate for Pod types

The contract assumed Pod types could live in `src/cache/mod.rs` with a local `#![allow(unsafe_code)]` override. However, `#![forbid(unsafe_code)]` at the crate level in `src/lib.rs` **cannot be overridden** by module-level `#![allow(unsafe_code)]` — `forbid` is the highest lint level and prevents any downgrade.

**Fix applied**: Created `centralized-docs-pod` as a separate workspace crate. This crate allows `unsafe_code` (needed by bytemuck's derive macros) while the parent crate maintains its strict `#![forbid(unsafe_code)]`. The `state` module in the parent crate re-exports all public types.

### bytemuck `min_const_generics` feature

bytemuck 1.x only implements `Pod`/`Zeroable` for arrays of specific sizes (powers of 2, etc.) by default. Arrays like `[u8; 54]` and `[u8; 6]` are not covered. The `min_const_generics` feature enables const-generic impls for arrays of any size, which is available on stable Rust since 1.51.

## Constraint Adherence

| Constraint | Status | Evidence |
|-----------|--------|----------|
| Zero `mut` in core logic | PASS | No `mut` keyword in any source function |
| Zero `unwrap`/`expect`/`panic` | PASS | All fallible paths return `Result<T, PodStateError>` |
| Make illegal states unrepresentable | PASS | Status enums enforce valid discriminants; `from_bytes_checked` validates all invariants |
| Expression-based | PASS | `map_or`, `find`, pattern matching throughout |
| Clippy flawless | PASS | `cargo clippy -p centralized-docs-pod -- -D warnings` exits 0 |
| No heap allocation | PASS | Both structs are `Copy`, all fields are stack-allocated primitives/arrays |
| Thread-safe by value | PASS | `Copy + Send + Sync` verified by compile-time test |
| Data → Calc → Actions | PASS | All functions are pure Calculations operating on Data types |
| `#![forbid(unsafe_code)]` in main crate | PASS | Main crate has zero unsafe; Pod crate isolated |

## Test Coverage

**92 tests total** (84 unit + 8 proptest):

- **Size & Layout**: 6 tests (sizes, alignments, no-padding proofs)
- **FileStateRaw construction**: 6 tests (field values, version, reserved, zeroed, Pod/Zeroable trait proofs)
- **UrlStateRaw construction**: 6 tests
- **Byte round-trip**: 6 tests (lossless conversion, byte slice lengths, byte-level equality)
- **from_bytes_checked errors (FileStateRaw)**: 11 tests (wrong size, invalid status, version mismatch, nonzero reserved)
- **from_bytes_checked errors (UrlStateRaw)**: 11 tests (wrong size, invalid status, version mismatch, nonzero pad1/reserved)
- **Status enum discriminants**: 12 tests (all 4 variants for each enum + rejection of invalid values + repr(u8) verification)
- **Validate method**: 7 tests (accept/reject status, version, reserved, pad1)
- **status() accessor**: 4 tests
- **Edge cases**: 4 tests (all-zeros, all-0xFF for both types)
- **Error display messages**: 5 tests
- **Trait proofs**: 2 tests (Copy + Send + Sync)
- **Proptests**: 8 tests (round-trip, byte length, discriminant validity, corruption detection)
