# Contract Specification

## Metadata

| Field | Value |
|-------|-------|
| bead_id | cdocs-h70 |
| bead_title | data: add fixed-size `FileStateRaw` and `UrlStateRaw` pod types |
| phase | contract |
| status | DRAFT |
| created | 2026-04-02 |
| skill | rust-contract v1.0.0 |

## Context

### Feature

Introduce two fixed-size `#[repr(C)]` Pod structs (`FileStateRaw`, `UrlStateRaw`) in
`src/cache/mod.rs` so that file and URL cache state can be stored as raw fixed-size
byte arrays in redb tables without serde_json serialization overhead. These types
derive `bytemuck::Pod` and `bytemuck::Zeroable`, enabling safe zero-copy byte casting
via bytemuck's checked APIs.

### Domain Terms

- **Pod**: "Plain Old Data" — a type with no padding bytes, no invalid bit patterns,
  and a stable C-compatible memory layout. Required for `bytemuck::cast_bytes` /
  `bytemuck::from_bytes`.
- **FileStateRaw**: Fixed-size byte-layout representing the cached processing state of
  a local file (content hash, size, modification time, status).
- **UrlStateRaw**: Fixed-size byte-layout representing the cached processing state of
  a scraped URL (content hash, HTTP status, content length, last-fetched time, status).
- **Reserved bytes**: Explicit `[u8; N]` fields that pad the struct to a fixed total
  size and reserve space for future schema evolution. These MUST be zero-initialized.
- **redb**: ACID key-value store. Tables defined as `TableDefinition<&[u8], &[u8]>`.
  Pod values can be written directly as byte slices without serialization.

### Source Files Read

1. `centralized-docs/src/lib.rs` — crate root; declares `pub mod cache`; enforces
   `#![forbid(unsafe_code)]`, `#![deny(clippy::unwrap_used)]`.
2. `centralized-docs/src/cache/mod.rs` — existing cache module with `ContentHash`
   newtype (`[u8; 32]`), `DocCache`, `CacheType`, `CacheConfig`, redb table
   definitions, and serde_json-based serialization.
3. `centralized-docs/src/types/mod.rs` — existing newtype patterns (validated
   constructors, `Result<T, Error>`, `thiserror`).
4. `centralized-docs/src/errors/cache.rs` — `CacheError` enum.
5. `centralized-docs/src/errors/mod.rs` — unified `DocTransformerError` hierarchy.
6. `centralized-docs/Cargo.toml` — `redb = "2"` already present; `bytemuck` NOT yet
   a dependency (must be added).

### Assumptions

1. `bytemuck` crate will be added as a dependency (likely `bytemuck = "1"` with
   `derive` feature for `Pod`/`Zeroable` proc macros, or manual unsafe impl if
   `#![forbid(unsafe_code)]` conflicts with bytemuck's derive macros — needs
   verification).
2. The `#![forbid(unsafe_code)]` crate-level attribute applies to `src/lib.rs` and
   all submodules. bytemuck's safe `Pod`/`Zeroable` derives do NOT require the
   consumer to write `unsafe` — the derive macros handle the unsafe impl internally
   in the bytemuck crate. However, if the derive proc-macro emits `unsafe impl` in
   the consumer crate, `#![forbid(unsafe_code)]` will reject it. **Resolution**:
   use `bytemuck::Pod` and `bytemuck::Zeroable` via their `derive` feature; verify
   compilation succeeds under `#![forbid(unsafe_code)]`. If it doesn't, use
   `unsafe_impl_bytemuck_pod` behind a dedicated `cache/pod.rs` file with a local
   `#![allow(unsafe_code)]` override, gated behind a safety comment.
3. Timestamps are stored as `u64` milliseconds since Unix epoch (consistent with
   `chrono::Utc::now().timestamp_millis()`).
4. Content hashes are `[u8; 32]` (SHA-256), matching the existing `ContentHash`
   newtype in `cache/mod.rs`.
5. These types live in `src/cache/mod.rs` alongside `ContentHash` and the existing
   cache infrastructure. A future bead may add conversion methods and redb table
   integration.

### Open Questions

1. **bytemuck + forbid(unsafe_code)**: Does bytemuck's `#[derive(Pod, Zeroable)]`
   generate `unsafe impl` blocks that violate `#![forbid(unsafe_code)]`? If yes,
   a localized `#![allow(unsafe_code)]` in a submodule is required. **Action**:
   verify during implementation.
2. **Schema version field**: Should these structs include an explicit `version: u8`
   field for future schema evolution, or rely on reserved bytes for that? This
   contract includes a `version` field.
3. **Endianness**: `#[repr(C)]` does not guarantee endianness. All multi-byte fields
   should use a consistent byte order. This contract assumes native endianness is
   acceptable since redb is single-machine and not cross-platform serialized. If
   cross-platform portability is needed, fields must be stored as big-endian.

---

## Types to Define

### 1. `FileStateRaw`

A fixed-size `#[repr(C)]` struct representing the cached processing state of a local
file. Stored as Pod bytes in redb.

```rust
/// Fixed-size Pod struct for cached file state.
///
/// # Layout (104 bytes total)
///
/// | Offset | Size | Field              | Type      |
/// |--------|------|--------------------|-----------|
/// | 0      | 32   | content_hash       | [u8; 32]  |
/// | 32     | 8    | file_size          | u64       |
/// | 40     | 8    | last_modified_ms   | u64       |
/// | 48     | 1    | version            | u8        |
/// | 49     | 1    | status             | u8        |
/// | 50     | 54   | reserved           | [u8; 54]  |
///
/// # Safety Invariants
///
/// - `reserved` bytes MUST be zero-initialized.
/// - `version` MUST be `1` (current schema version).
/// - `status` is a `FileStateStatus` discriminant (0 = Unknown, 1 = Unchanged,
///   2 = Modified, 3 = Deleted).
/// - Total size: 104 bytes (aligns to 8-byte boundary; no implicit padding).
///
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileStateRaw {
    pub content_hash: [u8; 32],
    pub file_size: u64,
    pub last_modified_ms: u64,
    pub version: u8,
    pub status: u8,
    pub reserved: [u8; 54],
}
```

**Total size: 104 bytes** (32 + 8 + 8 + 1 + 1 + 54 = 104).
104 % 8 = 0 — no alignment violation; no compiler-inserted padding under `#[repr(C)]`.

### 2. `UrlStateRaw`

A fixed-size `#[repr(C)]` struct representing the cached processing state of a scraped
URL. Stored as Pod bytes in redb.

```rust
/// Fixed-size Pod struct for cached URL state.
///
/// # Layout (112 bytes total)
///
/// | Offset | Size | Field              | Type      |
/// |--------|------|--------------------|-----------|
/// | 0      | 32   | content_hash       | [u8; 32]  |
/// | 32     | 2    | http_status        | u16       |
/// | 34     | 2    | _pad1              | [u8; 2]   |
/// | 36     | 8    | content_length     | u64       |
/// | 44     | 8    | last_fetched_ms    | u64       |
/// | 52     | 1    | version            | u8        |
/// | 53     | 1    | status             | u8        |
/// | 54     | 58   | reserved           | [u8; 58]  |
///
/// # Safety Invariants
///
/// - `_pad1` and `reserved` bytes MUST be zero-initialized.
/// - `version` MUST be `1` (current schema version).
/// - `status` is a `UrlStateStatus` discriminant (0 = Unknown, 1 = Fresh,
///   2 = Stale, 3 = Error).
/// - `http_status` is the HTTP response status code (e.g., 200, 404).
/// - Total size: 112 bytes (aligns to 8-byte boundary; no implicit padding).
///
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UrlStateRaw {
    pub content_hash: [u8; 32],
    pub http_status: u16,
    pub _pad1: [u8; 2],
    pub content_length: u64,
    pub last_fetched_ms: u64,
    pub version: u8,
    pub status: u8,
    pub reserved: [u8; 58],
}
```

**Total size: 112 bytes** (32 + 2 + 2 + 8 + 8 + 1 + 1 + 58 = 112).
112 % 8 = 0 — no alignment violation; explicit `_pad1` prevents compiler padding
between `http_status` (u16, align 2) and `content_length` (u64, align 8).

### 3. `FileStateStatus`

Discriminant enum for file cache state.

```rust
/// Status of a cached file entry.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStateStatus {
    Unknown   = 0,
    Unchanged = 1,
    Modified  = 2,
    Deleted   = 3,
}
```

### 4. `UrlStateStatus`

Discriminant enum for URL cache state.

```rust
/// Status of a cached URL entry.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UrlStateStatus {
    Unknown = 0,
    Fresh   = 1,
    Stale   = 2,
    Error   = 3,
}
```

---

## Preconditions

### P-01: bytemuck dependency available

`bytemuck` crate with `derive` feature MUST be listed in `Cargo.toml` `[dependencies]`
before these types can derive `Pod` / `Zeroable`.

### P-02: Crate forbids unsafe_code

The crate-level `#![forbid(unsafe_code)]` in `src/lib.rs` must be respected. If
bytemuck's derive macros emit `unsafe impl` that violates this, a submodule with a
local `#![allow(unsafe_code)]` and a safety comment is required.

### P-03: No compiler-inserted padding

The struct layouts MUST be verified to have zero compiler-inserted padding. This is
ensured by:
1. `#[repr(C)]` attribute on each struct.
2. All fields are fixed-size primitives or byte arrays.
3. Explicit padding/reserved bytes fill any alignment gaps.
4. `std::mem::size_of::<FileStateRaw>() == 104` and `std::mem::size_of::<UrlStateRaw>() == 112`.
5. `std::mem::align_of::<FileStateRaw>() == 8` and `std::mem::align_of::<UrlStateRaw>() == 8`.

### P-04: Reserved bytes zero-initialized

All reserved/padding byte arrays (`reserved`, `_pad1`) MUST be initialized to `[0u8; N]`
in every constructor. This is a Pod safety invariant — undefined bytes in a Pod type
are undefined behavior when cast.

### P-05: Version field initialized to 1

The `version` field on both structs MUST be initialized to `1` in all constructors.
This identifies the current schema version and enables future migration.

### P-06: Status field is valid discriminant

The `status` field MUST always contain a value that is a valid discriminant of the
corresponding status enum (`FileStateStatus` or `UrlStateStatus`). Invalid values
must be rejected at construction time.

---

## Postconditions

### POST-01: Pod trait satisfied

Both `FileStateRaw` and `UrlStateRaw` implement `bytemuck::Pod` and
`bytemuck::Zeroable`. This enables:
- `bytemuck::bytes_of(&raw_state)` — borrow as `&[u8]`
- `bytemuck::from_bytes::<FileStateRaw>(&bytes)` — cast `&[u8]` to `&FileStateRaw`
- `bytemuck::cast_slice::<u8, FileStateRaw>(&bytes)` — cast byte slice

### POST-02: Zeroable default

`bytemuck::Zeroable` implies a valid all-zeros instance exists. Calling
`bytemuck::Zeroable::zeroed()` on either type produces a valid zero-initialized
instance with `reserved` and `_pad1` set to all zeros.

### POST-03: Size is fixed and deterministic

- `std::mem::size_of::<FileStateRaw>() == 104` for all time.
- `std::mem::size_of::<UrlStateRaw>() == 112` for all time.
- No variation across platforms because `#[repr(C)]` specifies a fixed layout and
  all field types have defined sizes.

### POST-04: Conversion to/from byte slices is lossless

For any valid `FileStateRaw` (or `UrlStateRaw`) value `v`:
```
let bytes = bytemuck::bytes_of(&v);
let restored: &FileStateRaw = bytemuck::from_bytes(bytes);
assert_eq!(*restored, v);
```
This round-trip is guaranteed by the Pod contract.

### POST-05: PartialEq equality is byte-level

Since these are `#[repr(C)]` Pod types with no padding (all gaps explicitly filled
with zero-initialized reserved bytes), `PartialEq` comparison is equivalent to a
byte-level comparison. Two instances with identical field values are equal AND have
identical byte representations.

### POST-06: Constructors produce valid instances

All constructor functions (`new`, `zeroed`) return instances that satisfy:
- `reserved` / `_pad1` are all zeros.
- `version == 1`.
- `status` is a valid `FileStateStatus` or `UrlStateStatus` discriminant.

---

## Invariants

### INV-01: No padding bytes

All bytes between fields are explicitly represented as `_pad1` or `reserved` arrays.
There are no compiler-inserted padding bytes. Verified by:
- `std::mem::size_of` matches the sum of field sizes.
- `#[repr(C)]` guarantees field order.

### INV-02: Alignment is 8

Both structs have alignment 8 (the maximum alignment of their fields, due to `u64`).
This matches redb's typical alignment for value storage and avoids misaligned reads.

### INV-03: Reserved bytes remain zero

After construction, reserved/padding bytes MUST remain zero. Mutation of these bytes
is prohibited by contract — there are no public setter methods for `reserved` or
`_pad1`. Direct field access is possible (struct is public) but violates the contract.

### INV-04: Version is schema identifier

The `version` field acts as a schema tag. It MUST NOT be mutated after construction.
Future schema versions create new structs or extend the reserved bytes.

### INV-05: Thread-safe by value

Both types are `Copy + Send + Sync` (all fields are primitive / byte arrays). No
interior mutability. Safe to share across threads by value or by reference.

### INV-06: No heap allocation

Both types are stack-allocated, fixed-size, `Copy`. No `String`, `Vec`, `Box`, or
other heap-allocated types appear in their fields.

---

## Error Taxonomy

### `PodStateError`

A new error enum for Pod state construction failures. Lives in `src/cache/mod.rs`
(or a dedicated `src/errors/pod_state.rs` if preferred).

```rust
/// Errors that can occur when constructing or validating Pod state types.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PodStateError {
    /// The status byte is not a valid discriminant for the target enum.
    #[error("invalid file state status byte: {0} (valid: 0-3)")]
    InvalidFileStatus(u8),

    /// The status byte is not a valid discriminant for the target enum.
    #[error("invalid URL state status byte: {0} (valid: 0-3)")]
    InvalidUrlStatus(u8),

    /// A byte slice does not match the expected struct size.
    #[error("byte slice has wrong size for {type_name}: got {actual}, expected {expected}")]
    WrongByteSize {
        type_name: &'static str,
        actual: usize,
        expected: usize,
    },

    /// The version field does not match the expected schema version.
    #[error("unsupported schema version for {type_name}: got {actual}, expected {expected}")]
    VersionMismatch {
        type_name: &'static str,
        actual: u8,
        expected: u8,
    },

    /// Reserved bytes contain non-zero values (corruption or uninitialized memory).
    #[error("reserved bytes contain non-zero values in {type_name} at offset {offset}")]
    ReservedBytesNonZero {
        type_name: &'static str,
        offset: usize,
    },
}
```

### Integration with existing error hierarchy

`PodStateError` should be added to `DocTransformerError`:

```rust
// In src/errors/mod.rs
pub enum DocTransformerError {
    // ... existing variants ...
    #[error(transparent)]
    PodState(#[from] PodStateError),
}
```

---

## Contract Signatures

### `FileStateRaw` constructors

```rust
impl FileStateRaw {
    /// Create a new FileStateRaw with the given fields.
    ///
    /// # Preconditions
    /// - `status` is a valid `FileStateStatus` discriminant.
    ///
    /// # Postconditions
    /// - `version == 1`
    /// - `reserved == [0u8; 54]`
    /// - Returned struct is a valid Pod instance.
    pub fn new(
        content_hash: [u8; 32],
        file_size: u64,
        last_modified_ms: u64,
        status: FileStateStatus,
    ) -> Self;

    /// Create a zeroed FileStateRaw (all bytes zero).
    ///
    /// # Postconditions
    /// - `version == 0` (callers must set version explicitly or use `new()`)
    /// - All fields are zero.
    /// - Valid per `bytemuck::Zeroable`.
    fn zeroed() -> Self;

    /// Parse a FileStateRaw from a byte slice, validating invariants.
    ///
    /// # Preconditions
    /// - `bytes.len() == std::mem::size_of::<FileStateRaw>()` (104)
    ///
    /// # Postconditions
    /// - Returns `Ok(FileStateRaw)` if all invariants hold.
    /// - Returns `Err(PodStateError::WrongByteSize)` if slice is wrong size.
    /// - Returns `Err(PodStateError::InvalidFileStatus)` if status byte is invalid.
    /// - Returns `Err(PodStateError::VersionMismatch)` if version is not 1.
    /// - Returns `Err(PodStateError::ReservedBytesNonZero)` if reserved bytes are non-zero.
    pub fn from_bytes_checked(bytes: &[u8]) -> Result<Self, PodStateError>;

    /// Parse a FileStateRaw from a byte slice WITHOUT validation.
    ///
    /// # Safety
    /// - `bytes.len()` MUST equal `std::mem::size_of::<FileStateRaw>()`.
    /// - Caller guarantees the bytes represent a valid instance.
    ///
    /// # Postconditions
    /// - Returns a reference to the Pod-cast bytes (zero-copy).
    pub fn from_bytes_unchecked(bytes: &[u8]) -> &Self;

    /// Borrow this instance as a byte slice (Pod cast).
    ///
    /// # Postconditions
    /// - Returned slice has length `std::mem::size_of::<FileStateRaw>()` (104).
    pub fn as_bytes(&self) -> &[u8];

    /// Get the status as a typed enum.
    ///
    /// # Postconditions
    /// - Returns `Ok(FileStateStatus)` if status byte is valid.
    /// - Returns `Err(PodStateError::InvalidFileStatus)` if status byte is invalid.
    ///   (This should never happen for instances created via `new()`, but may occur
    ///   for instances read from untrusted storage.)
    pub fn status(&self) -> Result<FileStateStatus, PodStateError>;

    /// Validate invariants of this instance.
    ///
    /// # Postconditions
    /// - Returns `Ok(())` if all invariants hold.
    /// - Returns `Err(PodStateError)` with specific violation otherwise.
    pub fn validate(&self) -> Result<(), PodStateError>;
}
```

### `UrlStateRaw` constructors

```rust
impl UrlStateRaw {
    /// Create a new UrlStateRaw with the given fields.
    ///
    /// # Preconditions
    /// - `status` is a valid `UrlStateStatus` discriminant.
    ///
    /// # Postconditions
    /// - `version == 1`
    /// - `_pad1 == [0u8; 2]`
    /// - `reserved == [0u8; 58]`
    /// - Returned struct is a valid Pod instance.
    pub fn new(
        content_hash: [u8; 32],
        http_status: u16,
        content_length: u64,
        last_fetched_ms: u64,
        status: UrlStateStatus,
    ) -> Self;

    /// Create a zeroed UrlStateRaw (all bytes zero).
    ///
    /// # Postconditions
    /// - All fields are zero.
    /// - Valid per `bytemuck::Zeroable`.
    fn zeroed() -> Self;

    /// Parse a UrlStateRaw from a byte slice, validating invariants.
    ///
    /// # Preconditions
    /// - `bytes.len() == std::mem::size_of::<UrlStateRaw>()` (112)
    ///
    /// # Postconditions
    /// - Returns `Ok(UrlStateRaw)` if all invariants hold.
    /// - Returns `Err(PodStateError::WrongByteSize)` if slice is wrong size.
    /// - Returns `Err(PodStateError::InvalidUrlStatus)` if status byte is invalid.
    /// - Returns `Err(PodStateError::VersionMismatch)` if version is not 1.
    /// - Returns `Err(PodStateError::ReservedBytesNonZero)` if padding/reserved bytes are non-zero.
    pub fn from_bytes_checked(bytes: &[u8]) -> Result<Self, PodStateError>;

    /// Parse a UrlStateRaw from a byte slice WITHOUT validation.
    ///
    /// # Safety
    /// - `bytes.len()` MUST equal `std::mem::size_of::<UrlStateRaw>()`.
    /// - Caller guarantees the bytes represent a valid instance.
    ///
    /// # Postconditions
    /// - Returns a reference to the Pod-cast bytes (zero-copy).
    pub fn from_bytes_unchecked(bytes: &[u8]) -> &Self;

    /// Borrow this instance as a byte slice (Pod cast).
    ///
    /// # Postconditions
    /// - Returned slice has length `std::mem::size_of::<UrlStateRaw>()` (112).
    pub fn as_bytes(&self) -> &[u8];

    /// Get the status as a typed enum.
    ///
    /// # Postconditions
    /// - Returns `Ok(UrlStateStatus)` if status byte is valid.
    /// - Returns `Err(PodStateError::InvalidUrlStatus)` if status byte is invalid.
    pub fn status(&self) -> Result<UrlStateStatus, PodStateError>;

    /// Validate invariants of this instance.
    ///
    /// # Postconditions
    /// - Returns `Ok(())` if all invariants hold.
    /// - Returns `Err(PodStateError)` with specific violation otherwise.
    pub fn validate(&self) -> Result<(), PodStateError>;
}
```

### Status enum conversions

```rust
impl FileStateStatus {
    /// Convert from a raw u8 discriminant.
    ///
    /// # Preconditions
    /// - `value` is in range 0..=3.
    ///
    /// # Postconditions
    /// - Returns `Ok(FileStateStatus)` for valid discriminants.
    /// - Returns `Err(PodStateError::InvalidFileStatus)` for invalid values.
    pub fn from_discriminant(value: u8) -> Result<Self, PodStateError>;
}

impl UrlStateStatus {
    /// Convert from a raw u8 discriminant.
    ///
    /// # Preconditions
    /// - `value` is in range 0..=3.
    ///
    /// # Postconditions
    /// - Returns `Ok(UrlStateStatus)` for valid discriminants.
    /// - Returns `Err(PodStateError::InvalidUrlStatus)` for invalid values.
    pub fn from_discriminant(value: u8) -> Result<Self, PodStateError>;
}
```

---

## Non-goals

1. **redb table integration** — This contract defines the types only. Wiring them
   into redb table definitions and replacing existing serde_json serialization is
   a separate bead.
2. **Conversion from/to higher-level domain types** — `FileStateRaw` <-> domain
   `FileState` conversion is a future bead.
3. **Serialization/deserialization via serde** — Optional via feature flag; not
   required for the Pod use case. Pod types exist to _avoid_ serde overhead.
4. **Schema migration** — Future versions of these structs may change layout.
   Migration logic is out of scope for this bead.
5. **Endianness conversion** — Cross-platform byte-order handling is not required
   for single-machine redb storage.

---

## Anti-Hallucination Notes

- **Files verified as read**: `centralized-docs/src/lib.rs` (148 lines),
  `centralized-docs/src/cache/mod.rs` (1261 lines),
  `centralized-docs/src/types/mod.rs` (1079 lines),
  `centralized-docs/src/errors/cache.rs` (20 lines),
  `centralized-docs/src/errors/mod.rs` (133 lines),
  `centralized-docs/Cargo.toml` (212 lines).
- **No existing `FileStateRaw` or `UrlStateRaw` types** found in the codebase
  (grep confirmed zero matches for `FileStateRaw`, `UrlStateRaw`, `bytemuck`,
  `Pod`, `Zeroable`).
- **`bytemuck` is NOT a current dependency** — Cargo.toml does not list it.
  Implementation MUST add it.
- **`#![forbid(unsafe_code)]` is active** in `src/lib.rs` line 80. This is a hard
  constraint on implementation approach.
- **Existing `ContentHash` newtype** at `cache/mod.rs:67` wraps `[u8; 32]` and
  provides the closest analogue to the patterns used here.
- **Current redb storage uses serde_json** (variable-length `Vec<u8>` values).
  These Pod types will eventually replace that for state entries, but this bead
  only defines the types.
- **Status enums with values 0-3** are novel to this bead — no existing status
  enums in the codebase match this pattern. The `#[repr(u8)]` approach is consistent
  with the project's "types as docs" principle from the cache module header.
