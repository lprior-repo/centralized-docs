# Contract Specification — cdocs-824

## Context

- **Feature:** Add zero-copy state dependencies (`bytemuck` with derive, confirm `rkyv` with `bytecheck`) to the `centralized-docs` crate's `Cargo.toml`.
- **Bead:** cdocs-824 (chore, priority 1)
- **Crate path:** `centralized-docs/Cargo.toml` within the workspace at `/home/lewis/src/cdocs-824/`
- **Scope:** Dependency manifest changes ONLY. No production code, no trait implementations, no type definitions.

### Domain Terms

| Term | Definition |
|------|-----------|
| `bytemuck` | Crate providing safe zero-copy transmute between byte slices and plain-old-data (Pod) types. |
| `Pod` | "Plain old data" — a marker trait (`bytemuck::Pod`) indicating a type has no padding, no invalid bit-patterns, and is byte-compatible. Derivable via `#[derive(bytemuck::Pod, bytemuck::Zeroable)]`. |
| `NoCell` | Marker trait (`bytemuck::NoCell`) indicating a type contains no `UnsafeCell`, making it safe to cast from `&[u8]`. Derivable via `#[derive(bytemuck::NoCell)]`. |
| `rkyv` | Zero-copy deserialization framework (v0.8). Archives Rust types into a byte-compatible layout that can be read without copying. |
| `bytecheck` | Validation crate bundled with rkyv for verifying archived bytes before zero-copy access. |
| `redb` | ACID key-value store using mmap'd files — the primary consumer of `bytemuck` for safe reinterpretation of mmap'd value bytes. |
| `mmap` | Memory-mapped file I/O — redb uses this internally; `bytemuck` provides safe casts from the resulting `&[u8]`. |

### Current State (Pre-Change)

- `rkyv = { version = "0.8", features = ["std", "bytecheck"] }` is present in both `[dependencies]` (line 97) and `[dev-dependencies]` (line 118).
- `bytemuck` is NOT present anywhere in the dependency graph.
- Cache-related dependencies are present: `redb = "2"`, `sha2 = "0.10"`, `lru = "0.16.3"`, `parking_lot = "0.12.5"`, `rayon = "1.11.0"`.

### Assumptions

1. `bytemuck` v1.x (latest stable) is the correct version range — no specific pin needed unless CI reveals incompatibility.
2. The `derive` feature of `bytemuck` is sufficient for future Pod/NoCell/Zeroable derives. No `extern_crate_alloc` or `min_const_generics` features needed at this stage.
3. `rkyv` 0.8 with `std` + `bytecheck` is the correct configuration — no additional rkyv features (e.g., `validation`, `size_16`) are needed now.
4. This change is dependency-only. No code will implement `Pod`, `NoCell`, or `Archive` derives yet.

### Open Questions

None. The scope is a single manifest edit with a compile verification gate.

---

## Preconditions

- [PRE-1] The workspace must compile successfully before the change (`cargo check -p centralized-docs` exits 0).
- [PRE-2] `rkyv` v0.8 with features `["std", "bytecheck"]` must already be present in `[dependencies]`.
- [PRE-3] The existing cache-related dependencies must all be present in `[dependencies]`: `redb`, `sha2`, `lru`, `parking_lot`, `rayon`.
- [PRE-4] The workspace lint `unsafe_code = "forbid"` is active — all zero-copy safety must come from `bytemuck`'s safe API, not from hand-written `unsafe` blocks in this crate.

## Postconditions

- [POST-1] `bytemuck` with feature `"derive"` is added to `[dependencies]` in `centralized-docs/Cargo.toml`.
- [POST-2] `bytemuck` with feature `"derive"` is added to `[dev-dependencies]` in `centralized-docs/Cargo.toml` (parity with rkyv's dev-dep presence for test/proptest usage).
- [POST-3] `rkyv = { version = "0.8", features = ["std", "bytecheck"] }` remains unchanged in both `[dependencies]` and `[dev-dependencies]`.
- [POST-4] All pre-existing dependencies remain unchanged (no removals, no version bumps, no feature downgrades).
- [POST-5] `cargo check -p centralized-docs` exits 0 after the change (compilation succeeds).
- [POST-6] `cargo check -p centralized-docs --all-features` exits 0 after the change.
- [POST-7] `bytemuck` appears in the resolved dependency tree: `cargo tree -p bytemuck` returns a valid tree rooted at `bytemuck v1.x`.
- [POST-8] No new `unsafe` code is introduced in the crate itself (workspace lint `unsafe_code = "forbid"` enforces this at compile time).

## Invariants

- [INV-1] **Additive-only change:** The diff to `Cargo.toml` must be purely additive lines. No lines are removed or modified (except adding new dependency entries).
- [INV-2] **No code changes:** No `.rs` files are modified. The change touches only `Cargo.toml`.
- [INV-3] **Workspace lint compliance:** The crate continues to compile under `unsafe_code = "forbid"`. The `bytemuck` crate uses `unsafe` internally, but this is acceptable because the lint applies to the local crate's source, not its dependencies.
- [INV-4] **Dependency graph acyclicity:** `bytemuck` has no dependency on `rkyv` and vice versa — they are independent zero-copy pipelines. Adding both must not introduce circular dependencies.
- [INV-5] **Feature minimality:** Only the `"derive"` feature is enabled for `bytemuck`. No speculative features are added.

## Error Taxonomy

Since this is a manifest-only change with no runtime behavior, the "errors" are compile-time or CI failures:

| Error | Condition | Resolution |
|-------|-----------|------------|
| `CompileError::DependencyResolution` | `cargo check` fails because `bytemuck` version conflicts with the MSRV or another dependency. | Pin `bytemuck` to a compatible version or adjust MSRV. |
| `CompileError::FeatureNotFound` | `bytemuck` version doesn't support the `"derive"` feature. | Verify the feature name in `bytemuck` docs; adjust version. |
| `CompileError::WorkspaceLintViolation` | Adding `bytemuck` somehow triggers a workspace lint (extremely unlikely for a dep-only change). | Investigate the specific lint and resolve. |
| `CompileError::ExistingCodeBreakage` | Adding `bytemuck` causes a name collision or trait conflict in existing code. | Namespace with `bytemuck = { ... }` alias or resolve collision. |
| `CIError::LockfileConflict` | `Cargo.lock` merge conflict after dependency addition. | Regenerate lockfile with `cargo update -p bytemuck`. |

## Contract Signatures

This bead introduces no function signatures. It is a dependency manifest change.

The contract is expressed in terms of **build-system operations**, not Rust function calls:

```toml
# Addition to [dependencies] in centralized-docs/Cargo.toml:
bytemuck = { version = "1", features = ["derive"] }

# Addition to [dev-dependencies] in centralized-docs/Cargo.toml:
bytemuck = { version = "1", features = ["derive"] }
```

**Verification operations (must all return exit code 0):**

```
cargo check -p centralized-docs
cargo check -p centralized-docs --all-features
cargo tree -p bytemuck
```

## Non-goals

- Implementing `Pod`, `Zeroable`, or `NoCell` derives on any existing types.
- Implementing `rkyv::Archive` derives on any existing types.
- Writing `bytemuck::cast_slice` or `bytemuck::from_bytes` call sites.
- Changing redb access patterns to use `bytemuck` casts.
- Adding `rkyv` serialization/deserialization code paths.
- Performance benchmarking (future bead).
- Updating the workspace `Cargo.toml` with `bytemuck` as a shared workspace dependency (can be done later if other crates need it).
