# Contract Specification

- **bead_id**: cdocs-7mf
- **bead_title**: schema: Define SCIP-inspired semantic domain model
- **phase**: STATE_1
- **updated_at**: 2026-03-29T12:30:00Z

---

## Context

### Domain Terms

| Term | Definition |
|------|-----------|
| **SCIP** | Source Code Intelligence Protocol — a schema for representing source code symbols as deterministic hierarchical identifiers. |
| **ScipSymbolId** | A deterministic, globally unique string identifier for a code symbol. Format: `<scheme>/<module_path>#<descriptor>`. |
| **scheme** | Language or system prefix (e.g., `rust`, `python`, `go`). |
| **module_path** | Slash-delimited namespace path (e.g., `auth/AuthService`). Must not contain empty segments. |
| **descriptor** | Symbol-specific suffix after `#`. May include `.` for method disambiguation (e.g., `MyStruct.my_method`). |
| **SymbolRole** | Bitmask representing semantic roles a symbol plays: `Definition`, `Read`, `Write`, `Generated`, `Test`. |
| **SymbolKind** | Classification of symbol: `Struct`, `Function`, `Trait`, `Enum`, `Module`, `Constant`, `Method`, `Field`, `Interface`, `TypeAlias`, `Variable`, `Package`. |
| **RelationshipKind** | Directed semantic edge type: `Imports`, `Contains`, `Implements`, `Calls`, `References`, `Overrides`. |

### Assumptions

- The `types/` directory does not yet exist and must be created at `centralized-docs/src/types/`.
- The existing `centralized-docs/src/types.rs` will be converted to `centralized-docs/src/types/mod.rs` to support the sub-module.
- `lib.rs` already declares `pub mod types;` — no change needed there.
- `bitflags` is NOT a dependency and will NOT be added. Bitmask operations use raw `u32` with manual constants.
- All crate-level lints (`#![deny(clippy::unwrap_used)]`, `#![deny(clippy::expect_used)]`) apply.
- `serde`, `thiserror`, and `strum` are available dependencies.
- `derive_more` is available for `Display`, `Deref`, `AsRef` derive macros.

### Open Questions

1. Should `ScipSymbolId::new` accept a pre-formatted string (parse + validate) or accept `(scheme, module_path, descriptor)` as separate parameters? **Decision: Both — `new(scheme, module_path, descriptor)` as the primary constructor (compositional), and `parse(s: &str)` as a fallible parser for deserialization-from-raw-string use cases.**
2. Should `SymbolRole` use `bitflags` crate or hand-rolled `u32` constants? **Decision: Hand-rolled `u32` with `bitflags`-style constants. No new dependency.**
3. Should `SymbolNode::relationships` allow duplicate targets? **Decision: No invariant enforced at the struct level — consumers deduplicate if needed.**

---

## Preconditions

### Module Structure

- [PRE-1] The file `centralized-docs/src/types/symbols.rs` exists and is declared as `pub mod symbols;` in `centralized-docs/src/types/mod.rs`.
- [PRE-2] The crate compiles with zero errors after the change.
- [PRE-3] No new crate dependencies are introduced.

### ScipSymbolId

- [PRE-S1] `scheme` is non-empty and contains no `/` or `#` characters.
- [PRE-S2] `module_path` is non-empty and contains no `#` characters. May contain `/` as segment separator.
- [PRE-S3] `module_path` contains no empty segments (i.e., no `//` and does not start or end with `/`).
- [PRE-S4] `descriptor` is non-empty.
- [PRE-S5] `descriptor` contains no `/` characters.

### SymbolRole

- [PRE-R1] Bit flag values are powers of two: `Definition = 1`, `Read = 2`, `Write = 4`, `Generated = 8`, `Test = 16`.
- [PRE-R2] Combinations are formed via bitwise OR. The zero value (`0`) represents "no roles" and is valid.

### SymbolNode

- [PRE-N1] `scip_id` is a valid `ScipSymbolId` (construction already validated).
- [PRE-N2] `display_name` is non-empty.
- [PRE-N3] `signature` and `documentation` may be `None`.
- [PRE-N4] `relationships` may be empty.

### SymbolRelationship

- [PRE-SR1] `target` is a valid `ScipSymbolId`.

---

## Postconditions

### Module Structure

- [POST-1] `centralized-docs/src/types/symbols.rs` exists and compiles cleanly.
- [POST-2] `centralized-docs/src/types/mod.rs` exists (converted from existing `types.rs`) and declares `pub mod symbols;`.
- [POST-3] All public types implement `Serialize` and `Deserialize` from `serde`.
- [POST-4] All public types implement `Debug`, `Clone`, `PartialEq`, `Eq`.

### ScipSymbolId

- [POST-S1] `ScipSymbolId::new("rust", "auth/AuthService", "login()")` returns `Ok` with `as_str() == "rust/auth/AuthService#login()"`.
- [POST-S2] `ScipSymbolId::new("rust", "auth/AuthService", "Auth.my_method")` returns `Ok` with `as_str() == "rust/auth/AuthService#Auth.my_method"`.
- [POST-S3] Two `ScipSymbolId`s with identical scheme/module_path/descriptor are `Eq`.
- [POST-S4] `ScipSymbolId` implements `Display`, `Deref<Target = str>`, `AsRef<str>`, `Borrow<str>`, `Hash`, `PartialOrd`, `Ord`.
- [POST-S5] `ScipSymbolId::parse("rust/auth/AuthService#login()")` returns `Ok`.
- [POST-S6] `ScipSymbolId::parse("rust/auth/AuthService#login()")` and `ScipSymbolId::new("rust", "auth/AuthService", "login()")` produce equal values.
- [POST-S7] Serialization produces the canonical string form; deserialization reconstructs an equal value.

### SymbolRole

- [POST-R1] `SymbolRole::DEFINITION.bits() == 1`.
- [POST-R2] `SymbolRole::DEFINITION | SymbolRole::READ` produces a combined role with `bits() == 3`.
- [POST-R3] `Display` for a single role outputs the lowercase name (e.g., `"definition"`).
- [POST-R4] `Display` for combined roles outputs sorted, `+`-delimited names (e.g., `"definition+read"`).
- [POST-R5] `Display` for zero roles outputs `"none"`.
- [POST-R6] Serialization/deserialization round-trips through the `u32` bits value.
- [POST-R7] `SymbolRole::from_bits(3).contains(SymbolRole::DEFINITION)` is `true`.
- [POST-R8] `SymbolRole::empty()` returns a role with `bits() == 0`.

### SymbolKind

- [POST-K1] All 12 variants serialize as lowercase string names (e.g., `"struct"`, `"function"`).
- [POST-K2] Deserialization from a valid lowercase string reconstructs the correct variant.
- [POST-K3] Deserialization from an invalid string returns `Err`.
- [POST-K4] `Display` outputs the lowercase variant name.

### SymbolNode

- [POST-N1] `SymbolNode` is `Serialize`/`Deserialize` with all fields included.
- [POST-N2] `SymbolNode` implements `Hash` (enabling use as graph node keys).

### SymbolRelationship

- [POST-SR1] `SymbolRelationship` is `Serialize`/`Deserialize` with all fields included.
- [POST-SR2] `RelationshipKind` serializes as lowercase string.

### Error Types

- [POST-E1] All error variants implement `Debug`, `Clone`, `PartialEq`, `Eq`.
- [POST-E2] All error variants have human-readable `Display` messages via `thiserror`.

---

## Invariants

- [INV-1] A `ScipSymbolId` always contains exactly one `#` separator between the module path and descriptor.
- [INV-2] A `ScipSymbolId`'s internal string always matches the canonical format `<scheme>/<module_path>#<descriptor>` with no leading/trailing whitespace.
- [INV-3] A `ScipSymbolId`'s module path never contains empty segments (no `//`, no leading/trailing `/`).
- [INV-4] `SymbolRole` bit values are always powers of two (or their combinations). No invalid bit patterns can exist outside the defined constants.
- [INV-5] `SymbolRole::Display` output is deterministic: roles are always sorted by bit value (ascending).
- [INV-6] `SymbolKind` variants serialize/deserialize case-insensitively (accept `"Struct"` or `"struct"`).
- [INV-7] `ScipSymbolId` is `Ord` — ordering is lexicographic on the canonical string.
- [INV-8] Zero `unwrap()` or `expect()` calls in production code within `symbols.rs`.

---

## Error Taxonomy

### ScipSymbolIdError

| Variant | Trigger Condition | Error Message Pattern |
|---------|------------------|-----------------------|
| `EmptyScheme` | `scheme` is empty or whitespace-only after trim | `"SCIP scheme cannot be empty"` |
| `InvalidScheme(s: String)` | `scheme` contains `/` or `#` | `"SCIP scheme contains invalid character: {s}"` |
| `EmptyModulePath` | `module_path` is empty or whitespace-only | `"SCIP module path cannot be empty"` |
| `EmptyModuleSegment(pos: usize)` | `module_path` contains an empty segment at position `pos` (e.g., `auth//service`) | `"SCIP module path contains empty segment at position {pos}"` |
| `LeadingSlash` | `module_path` starts with `/` | `"SCIP module path must not start with '/'"` |
| `TrailingSlash` | `module_path` ends with `/` | `"SCIP module path must not end with '/'"` |
| `HashInModulePath` | `module_path` contains `#` | `"SCIP module path must not contain '#'"` |
| `EmptyDescriptor` | `descriptor` is empty or whitespace-only | `"SCIP descriptor cannot be empty"` |
| `SlashInDescriptor` | `descriptor` contains `/` | `"SCIP descriptor must not contain '/'"` |
| `InvalidFormat(s: String)` | `parse()` input does not match `<scheme>/<path>#<descriptor>` pattern | `"Invalid SCIP symbol format: {s}"` |

### SymbolRoleError

| Variant | Trigger Condition | Error Message Pattern |
|---------|------------------|-----------------------|
| `UnknownBit(bit: u32)` | `from_bits()` called with bits outside the defined mask | `"Unknown SymbolRole bit: {bit}"` |

### SymbolKindError

| Variant | Trigger Condition | Error Message Pattern |
|---------|------------------|-----------------------|
| `UnknownKind(s: String)` | Deserialization receives an unrecognized string | `"Unknown SymbolKind: {s}"` |

---

## Contract Signatures

### ScipSymbolId

```rust
/// Deterministic SCIP-format symbol identifier.
///
/// Format: `<scheme>/<module_path>#<descriptor>`
/// Example: `rust/auth/AuthService#login()`
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ScipSymbolId(String);
```

```rust
impl ScipSymbolId {
    /// Construct from validated components. Returns `Err(ScipSymbolIdError)` on violation.
    pub fn new(
        scheme: impl Into<String>,
        module_path: impl Into<String>,
        descriptor: impl Into<String>,
    ) -> Result<Self, ScipSymbolIdError>;

    /// Parse a pre-formatted SCIP string. Returns `Err(ScipSymbolIdError)` if format invalid.
    pub fn parse(s: &str) -> Result<Self, ScipSymbolIdError>;

    /// The canonical string representation.
    #[must_use]
    pub fn as_str(&self) -> &str;

    /// Extract the scheme portion.
    #[must_use]
    pub fn scheme(&self) -> &str;

    /// Extract the module path portion.
    #[must_use]
    pub fn module_path(&self) -> &str;

    /// Extract the descriptor portion.
    #[must_use]
    pub fn descriptor(&self) -> &str;

    /// Convert to owned String.
    #[must_use]
    pub fn into_string(self) -> String;
}
```

Plus trait impls: `Display`, `Deref<Target = str>`, `AsRef<str>`, `Borrow<str>`.

### SymbolRole

```rust
/// Bitmask of semantic roles a symbol can play.
///
/// Uses hand-rolled u32 constants (no bitflags crate dependency).
/// Individual flags are powers of two. Combine via `|`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SymbolRole(u32);
```

```rust
impl SymbolRole {
    pub const DEFINITION: Self = SymbolRole(1);
    pub const READ:         Self = SymbolRole(2);
    pub const WRITE:        Self = SymbolRole(4);
    pub const GENERATED:    Self = SymbolRole(8);
    pub const TEST:         Self = SymbolRole(16);
    pub const ALL:          Self = SymbolRole(31); // 1|2|4|8|16

    /// Create from raw bits. Returns `Err` if any bit is outside the defined mask.
    pub fn from_bits(bits: u32) -> Result<Self, SymbolRoleError>;

    /// Create from raw bits, silently ignoring unknown bits.
    #[must_use]
    pub fn from_bits_truncate(bits: u32) -> Self;

    /// Zero-role (no flags set).
    #[must_use]
    pub const fn empty() -> Self;

    /// Raw bitmask value.
    #[must_use]
    pub const fn bits(self) -> u32;

    /// Check if a specific flag is set.
    #[must_use]
    pub fn contains(self, other: Self) -> bool;

    /// Check if no flags are set.
    #[must_use]
    pub fn is_empty(self) -> bool;

    /// Bitwise OR union.
    #[must_use]
    pub const fn union(self, other: Self) -> Self;

    /// Bitwise AND intersection.
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self;
}
```

Plus trait impls: `Display` (sorted `+`-delimited), `BitOr`, `BitAnd`, `BitOrAssign`, `BitAndAssign`.

### SymbolKind

```rust
/// Classification of a code symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SymbolKind {
    Struct,
    Function,
    Trait,
    Enum,
    Module,
    Constant,
    Method,
    Field,
    Interface,
    TypeAlias,
    Variable,
    Package,
}
```

Plus trait impls: `Display` (lowercase variant name).

### RelationshipKind

```rust
/// Directed semantic relationship between symbols.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RelationshipKind {
    Imports,
    Contains,
    Implements,
    Calls,
    References,
    Overrides,
}
```

Plus trait impls: `Display` (lowercase variant name).

### SymbolRelationship

```rust
/// A directed semantic edge from one symbol to another.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SymbolRelationship {
    pub target: ScipSymbolId,
    pub kind: RelationshipKind,
}
```

### SymbolNode

```rust
/// A semantic symbol with identity, kind, roles, and relationships.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SymbolNode {
    pub scip_id: ScipSymbolId,
    pub kind: SymbolKind,
    pub roles: SymbolRole,
    pub display_name: String,
    pub signature: Option<String>,
    pub documentation: Option<String>,
    pub relationships: Vec<SymbolRelationship>,
}
```

---

## Non-goals

- [NON-1] This contract does NOT cover graph-level operations (building symbol graphs, traversal, querying). That is a separate bead.
- [NON-2] This contract does NOT define a builder pattern for `SymbolNode`. Direct struct construction is sufficient.
- [NON-3] This contract does NOT define a `SymbolDatabase` or persistence layer.
- [NON-4] This contract does NOT define code parsing/extractor logic (e.g., Rust AST -> SymbolNode). That is a separate bead.
- [NON-5] This contract does NOT add the `bitflags` crate dependency.
- [NON-6] This contract does NOT define fuzzy matching or similarity for `ScipSymbolId`.
- [NON-7] This contract does NOT modify any existing types in `types.rs` (now `types/mod.rs`) other than adding the `pub mod symbols;` declaration.

---

## Acceptance Criteria Mapping

| # | Acceptance Test | Contract Reference |
|---|----------------|--------------------|
| 1 | SCIP ID generation creates correct strings like `rust/auth/AuthService#login()` | POST-S1, POST-S5, POST-S6 |
| 2 | SymbolRole correctly formats via bitmask representation | POST-R3, POST-R4, POST-R5 |
| 3 | Fails gracefully on empty module names | PRE-S2, ScipSymbolIdError::EmptyModulePath |
| 4 | Rejects invalid syntax formatting | ScipSymbolIdError::InvalidFormat |
