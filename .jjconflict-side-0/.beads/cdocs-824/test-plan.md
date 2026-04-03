# Test Plan: cdocs-824 — Add zero-copy state dependencies to centralized-docs

## Summary

- **Bead:** cdocs-824 (chore — dependency-only change)
- **Scope:** Add `bytemuck = { version = "1", features = ["derive"] }` to both `[dependencies]` and `[dev-dependencies]` in `centralized-docs/Cargo.toml`. No `.rs` files are modified.
- **Behaviors identified:** 10 (all build-system/manifest level)
- **Trophy allocation:** 0 unit / 7 integration / 1 e2e / 2 static
- **Proptest invariants:** 0 (no pure functions introduced)
- **Fuzz targets:** 0 new (no new parsers/deserializers introduced)
- **Kani harnesses:** 0 (no critical arithmetic or state machines)
- **Mutation kill target:** N/A for manifest-only change; regression suite validates correctness

### Trophy Deviation Justification

This bead deviates from the standard 60/30/5/5 trophy ratio because it introduces **zero runtime behavior**. There are no public functions, no type definitions, no trait implementations, and no control flow. The entire verification surface is at the build-system level: Cargo manifest correctness, dependency graph resolution, and compilation success.

The adapted trophy allocation for manifest-only changes:

```
     [E2E: full test suite regression]        ← 1 — existing tests still pass
  [Integration: cargo check, metadata, tree]  ← 7 — build-system correctness
 [Static: clippy, workspace lints, diff]      ← 2 — compile-time enforcement
```

---

## 1. Behavior Inventory

Every behavior the system guarantees for this bead, expressed as `"[Subject] [action] [outcome] when [condition]"`:

| # | Behavior |
|---|----------|
| B1 | **Cargo manifest resolves** `bytemuck` with `"derive"` feature when `cargo check -p centralized-docs` is invoked |
| B2 | **Cargo manifest resolves** `bytemuck` with `"derive"` feature when `cargo check -p centralized-docs --all-features` is invoked |
| B3 | **Dependency tree includes** `bytemuck` at version `1.x` when `cargo tree -p bytemuck` is queried |
| B4 | **Cargo metadata reports** `bytemuck` in the resolved dependency graph with features `["derive"]` |
| B5 | **Diff to Cargo.toml is** purely additive (no removed or modified lines) when compared to pre-change state |
| B6 | **All pre-existing dependencies remain** unchanged (same versions, same features) when compared to pre-change state |
| B7 | **Workspace lints still enforce** `unsafe_code = "forbid"` when compiling the crate |
| B8 | **bytemuck appears in** both `[dependencies]` and `[dev-dependencies]` sections of `centralized-docs/Cargo.toml` |
| B9 | **rkyv dependency remains** at `{ version = "0.8", features = ["std", "bytecheck"] }` in both `[dependencies]` and `[dev-dependencies]` |
| B10 | **Existing test suite passes** when `cargo test -p centralized-docs` is invoked after the change |

---

## 2. Trophy Allocation

| Behavior | Layer | Justification |
|----------|-------|---------------|
| B1: `cargo check` succeeds | Integration | Validates dependency resolution against real registry, real lockfile, real compiler. Cannot be unit-tested — requires full cargo infrastructure. |
| B2: `cargo check --all-features` succeeds | Integration | Same as B1 but with feature unification. Catches feature-flag conflicts that only appear under `--all-features`. |
| B3: `cargo tree -p bytemuck` resolves | Integration | Queries the real dependency graph. Validates that `bytemuck` is reachable and resolves to a valid version. |
| B4: `cargo metadata` shows correct features | Integration | Programmatic inspection of resolved dependency metadata. Validates the `"derive"` feature is actually enabled. |
| B5: Diff is purely additive | Static (diff analysis) | Mechanical verification that the manifest change contains only additions. Prevents accidental removals or modifications. |
| B6: Pre-existing deps unchanged | Integration | Snapshot comparison of dependency list before and after. Catches accidental version bumps or feature removals. |
| B7: Workspace lints still pass | Static (clippy/lint) | The `unsafe_code = "forbid"` lint is a compile-time gate. Running `cargo check` with workspace lints active verifies the lint still applies. |
| B8: bytemuck in both dep sections | Static (TOML parsing) | Validates manifest structure: `bytemuck` appears in both `[dependencies]` and `[dev-dependencies]`. |
| B9: rkyv unchanged | Integration | Verifies the existing `rkyv` dependency was not accidentally modified during the edit. |
| B10: Existing test suite passes | E2E | Full regression gate — the entire existing test suite must still pass. This is the broadest validation that the new dependency doesn't break anything. |

### Layer Counts

| Layer | Count | Percentage |
|-------|-------|------------|
| Integration | 7 | 70% |
| Static | 2 | 20% |
| E2E | 1 | 10% |
| Unit | 0 | 0% |

**Deviation rationale:** 0% unit tests because there are no pure functions to test. All verification is either build-system integration or static manifest analysis. This is the correct allocation for a dependency-only change.

---

## 3. BDD Scenarios

### Behavior B1: `cargo check` resolves bytemuck with derive feature

```
fn cargo_check_succeeds_when_bytemuck_derive_added()
```

**Given:** `centralized-docs/Cargo.toml` contains `bytemuck = { version = "1", features = ["derive"] }` in `[dependencies]` and `[dev-dependencies]`
**When:** `cargo check -p centralized-docs` is invoked
**Then:** Exit code is 0, and stderr contains no error messages

**Error variant:**
**Given:** `bytemuck` version is incompatible with MSRV 1.94
**When:** `cargo check -p centralized-docs` is invoked
**Then:** Exit code is non-zero, and stderr contains the string `"failed to select a version for the requirement"`

---

### Behavior B2: `cargo check --all-features` resolves

```
fn cargo_check_all_features_succeeds_when_bytemuck_derive_added()
```

**Given:** `centralized-docs/Cargo.toml` contains `bytemuck` with `"derive"` feature
**When:** `cargo check -p centralized-docs --all-features` is invoked
**Then:** Exit code is 0

**Error variant:**
**Given:** The `"derive"` feature conflicts with another feature under unification
**When:** `cargo check -p centralized-docs --all-features` is invoked
**Then:** Exit code is non-zero, and stderr contains a feature conflict message

---

### Behavior B3: `cargo tree` shows bytemuck in dependency graph

```
fn cargo_tree_shows_bytemuck_v1_when_queried()
```

**Given:** `bytemuck` is present in `[dependencies]`
**When:** `cargo tree -p bytemuck --prefix depth` is invoked in the workspace root
**Then:** Stdout contains the string `"bytemuck v1."` (matching any 1.x.y version)

**Error variant:**
**Given:** `bytemuck` was added to `[dev-dependencies]` only, not `[dependencies]`
**When:** `cargo tree -p bytemuck` is invoked (without `--dev`)
**Then:** Exit code is non-zero, and stderr contains `"did not find any packages matching"`

---

### Behavior B4: `cargo metadata` reports correct features for bytemuck

```
fn cargo_metadata_shows_derive_feature_when_bytemuck_resolved()
```

**Given:** `bytemuck = { version = "1", features = ["derive"] }` is in `[dependencies]`
**When:** `cargo metadata --format-version 1 --manifest-path centralized-docs/Cargo.toml` is parsed, and the resolved dependencies for the `centralized-docs` package are inspected
**Then:** The `bytemuck` entry's `features` field contains exactly `["derive"]`

**Error variant:**
**Given:** The `"derive"` feature was misspelled as `"derives"` in Cargo.toml
**When:** `cargo metadata` is parsed for the `bytemuck` entry
**Then:** The `features` field does NOT contain `"derive"` (contains `"derives"` instead)

---

### Behavior B5: Diff to Cargo.toml is purely additive

```
fn manifest_diff_is_purely_additive_when_bytemuck_added()
```

**Given:** The pre-change `Cargo.toml` is stored (or available via git)
**When:** A diff between pre-change and post-change `Cargo.toml` is computed
**Then:** Every removed-line marker (`-`) is absent from the diff; only added lines (`+`) appear for the `bytemuck` entries

**Error variant:**
**Given:** The change accidentally removed the `rkyv` dependency line
**When:** Diff is computed
**Then:** The diff contains a `-` line with `rkyv`, and the test fails

---

### Behavior B6: Pre-existing dependencies remain unchanged

```
fn preexisting_deps_unchanged_when_bytemuck_added()
```

**Given:** A snapshot of all dependency names and versions from the pre-change `Cargo.toml`
**When:** The post-change `Cargo.toml` is parsed and dependencies are compared
**Then:** Every dependency that existed before the change has the same version specifier and features, specifically:
  - `rkyv = { version = "0.8", features = ["std", "bytecheck"] }` (both `[dependencies]` and `[dev-dependencies]`)
  - `redb = "2"`
  - `sha2 = "0.10"`
  - `lru = "0.16.3"`
  - `parking_lot = "0.12.5"`
  - `rayon = "1.11.0"`

---

### Behavior B7: Workspace lints still enforce `unsafe_code = "forbid"`

```
fn workspace_lint_forbids_unsafe_when_bytemuck_added()
```

**Given:** The workspace `Cargo.toml` contains `unsafe_code = "forbid"` in `[workspace.lints.rust]`
**When:** `cargo check -p centralized-docs` is invoked with the workspace lints active
**Then:** Exit code is 0 (compilation succeeds, meaning no new `unsafe` code was introduced in the local crate)

**Note:** `bytemuck` uses `unsafe` internally, but workspace lints only apply to the local crate's source, not dependencies. This test confirms that distinction holds.

---

### Behavior B8: bytemuck appears in both dependency sections

```
fn bytemuck_in_both_dep_and_dev_dep_sections_when_manifest_parsed()
```

**Given:** The post-change `centralized-docs/Cargo.toml` is parsed as a TOML document
**When:** Both `[dependencies]` and `[dev-dependencies]` sections are inspected
**Then:** Both sections contain a key `"bytemuck"` with value `{ version = "1", features = ["derive"] }`

**Error variant:**
**Given:** `bytemuck` was added to `[dependencies]` only
**When:** `[dev-dependencies]` is inspected
**Then:** The key `"bytemuck"` is absent from `[dev-dependencies]`, and the test fails

---

### Behavior B9: rkyv dependency remains unchanged

```
fn rkyv_remains_at_v08_with_std_bytecheck_when_bytemuck_added()
```

**Given:** The post-change `Cargo.toml`
**When:** Both `[dependencies]` and `[dev-dependencies]` sections are inspected for the `rkyv` key
**Then:** The value is exactly `{ version = "0.8", features = ["std", "bytecheck"] }` in both sections

**Error variant:**
**Given:** The edit accidentally changed `rkyv` features to `["std"]` (removing `"bytecheck"`)
**When:** `[dependencies]` is inspected for the `rkyv` key
**Then:** The `features` array is `["std"]`, NOT `["std", "bytecheck"]`, and the test fails

---

### Behavior B10: Existing test suite passes

```
fn existing_test_suite_passes_when_bytemuck_added()
```

**Given:** The post-change codebase with `bytemuck` in dependencies
**When:** `cargo test -p centralized-docs` is invoked
**Then:** Exit code is 0, and the test summary shows all tests passing

**Error variant:**
**Given:** Adding `bytemuck` caused a name collision with an existing `derive` macro
**When:** `cargo test -p centralized-docs` is invoked
**Then:** Compilation fails with an error containing `"duplicate definition"` or `"name conflict"`

---

## 4. Proptest Invariants

**No proptest invariants apply to this bead.**

Rationale: Proptest validates properties of pure functions with non-trivial input spaces. This bead introduces no functions, no types, and no runtime behavior. The only "inputs" are the Cargo.toml manifest entries, which have a trivial input space (exactly one valid configuration).

**Future note:** When subsequent beads implement `#[derive(bytemuck::Pod, bytemuck::Zeroable)]` on concrete types, those beads MUST include proptest invariants:
- "Any valid `[u8; N]` cast via `bytemuck::cast_slice` produces a valid `&[T]` where T: Pod"
- "Zeroed bytes always produce a valid `T: Zeroable`"

---

## 5. Fuzz Targets

**No new fuzz targets apply to this bead.**

Rationale: Fuzz targets test parsing, deserialization, and user-input boundaries. This bead introduces no new entry points for untrusted data.

**Existing fuzz targets (unaffected, serve as regression gate):**

| Existing Target | File | Must Still Compile |
|-----------------|------|--------------------|
| `transform_artifact_deserialize` | `fuzz/fuzz_targets/transform_artifact_deserialize.rs` | Yes |
| `state_batch_deserialize` | `fuzz/fuzz_targets/state_batch_deserialize.rs` | Yes |
| `record_file_hash_input` | `fuzz/fuzz_targets/record_file_hash_input.rs` | Yes |
| `mcp_stdio_transport` | `fuzz/fuzz_targets/mcp_stdio_transport.rs` | Yes |
| `link_map_fingerprint` | `fuzz/fuzz_targets/link_map_fingerprint.rs` | Yes |
| `fuzz_target_1` | `fuzz/fuzz_targets/fuzz_target_1.rs` | Yes |
| `fuzz_serialize_snapshot` | `fuzz/fuzz_targets/fuzz_serialize_snapshot.rs` | Yes |
| `fuzz_load_snapshots_corrupt_bytes` | `fuzz/fuzz_targets/fuzz_load_snapshots_corrupt_bytes.rs` | Yes |

**Future note:** When future beads implement `bytemuck::from_bytes` or `rkyv::access` call sites on cached state, new fuzz targets MUST be added:
- `fuzz_bytemuck_cast_slice` — fuzz the byte-to-struct cast boundary for redb mmap'd values
- `fuzz_rkyv_access` — fuzz the archived byte validation via `bytecheck`

---

## 6. Kani Harnesses

**No Kani harnesses apply to this bead.**

Rationale: Kani performs bounded model checking on critical arithmetic, state machine completeness, and index/pointer math. This bead introduces no arithmetic, no state machines, and no indexing.

**Future note:** When future beads implement `bytemuck::cast_slice` on mmap'd bytes from redb, a Kani harness MUST verify:
- Property: `size_of::<T>() * slice.len() == byte_slice.len()` for all valid casts
- Bound: slices up to 1MB
- Rationale: Size mismatch in byte-to-struct casts causes undefined behavior; formal verification is warranted for the Pod cast boundary.

---

## 7. Mutation Testing Checkpoints

### Manifest Mutation Testing

Since `cargo-mutants` targets Rust source code (not TOML manifests), traditional mutation testing does not directly apply. However, the **equivalent mutations** for a manifest change are manual fault injections:

| Mutation | Description | Caught By |
|----------|-------------|-----------|
| Remove `"derive"` from features | Change `features = ["derive"]` to `features = []` | B4: `cargo metadata` shows empty features |
| Remove `[dev-dependencies]` entry | Delete `bytemuck` from `[dev-dependencies]` only | B8: TOML parse finds `bytemuck` in `[dependencies]` but not `[dev-dependencies]` |
| Remove `[dependencies]` entry | Delete `bytemuck` from `[dependencies]` only | B3: `cargo tree -p bytemuck` fails without `--dev`; B8: missing from `[dependencies]` |
| Change version to `"2"` | `bytemuck = { version = "2", features = ["derive"] }` | B3: `cargo tree` shows `v2.x` — depends on whether `bytemuck` 2.x exists and is compatible |
| Remove `rkyv` line accidentally | Delete the `rkyv` dependency line | B9: rkyv entry missing from parsed TOML; B1: `cargo check` fails (code uses rkyv) |
| Misspell `bytemuck` as `bytemuc` | `bytemuc = { ... }` | B1: `cargo check` fails — crate name not found in registry |
| Add to wrong section | Place in `[build-dependencies]` instead | B8: absent from `[dependencies]` and `[dev-dependencies]` |

**Target:** All 7 mutations must be caught by the planned scenarios (100% kill rate for manifest mutations).

### Rust Source Mutation Testing

**N/A** — no `.rs` files are modified by this bead.

---

## 8. Combinatorial Coverage Matrix

### Matrix: Manifest Dependency Addition

| Scenario | Input Class | Expected Output | Layer | Behavior |
|----------|-------------|-----------------|-------|----------|
| Happy path: bytemuck added correctly | Valid manifest with `bytemuck = { version = "1", features = ["derive"] }` in both sections | `cargo check` exit 0; `cargo tree` shows `bytemuck v1.x`; `cargo metadata` shows features `["derive"]` | Integration | B1, B2, B3, B4 |
| Error: version incompatible with MSRV | `bytemuck` version range that requires Rust > 1.94 | `cargo check` exit non-zero; stderr contains `"failed to select a version"` | Integration | B1 |
| Error: feature name misspelled | `features = ["derives"]` | `cargo metadata` shows `["derives"]` NOT `["derive"]` | Integration | B4 |
| Error: missing from dev-deps | `bytemuck` in `[dependencies]` only | TOML parse: `[dev-dependencies]` lacks `bytemuck` | Static | B8 |
| Error: rkyv accidentally modified | `rkyv` features changed to `["std"]` | TOML parse: `rkyv` features ≠ `["std", "bytecheck"]` | Static | B9 |
| Error: diff contains removals | Pre-existing line removed in edit | Diff contains `-` lines; test fails | Static | B5 |
| Boundary: minimal version | `bytemuck = { version = "1", features = ["derive"] }` (latest 1.x) | Resolves to highest compatible 1.x.y | Integration | B3 |
| Invariant: all pre-existing deps unchanged | Pre-change vs post-change dep list | Every pre-existing dep matches exactly | Integration | B6 |
| Regression: full test suite | All existing tests | All pass, exit 0 | E2E | B10 |
| Lint: unsafe_code still forbidden | Workspace lints active | `cargo check` succeeds (no local unsafe added) | Static | B7 |

### Matrix: Dependency Graph Correctness

| Scenario | Query | Expected Result | Layer |
|----------|-------|-----------------|-------|
| bytemuck is reachable | `cargo tree -p bytemuck` | Stdout contains `bytemuck v1.` | Integration |
| bytemuck has derive feature | `cargo metadata` → parse features | `["derive"]` exactly | Integration |
| bytemuck has no circular dep with rkyv | `cargo tree -p bytemuck` and `cargo tree -p rkyv` | Neither tree contains the other | Integration |
| bytemuck is leaf-ish (few transitive deps) | `cargo tree -p bytemuck --depth 1` | `bytemuck` has ≤ 2 transitive deps (`bytemuck_derive` macro crate) | Integration |

---

## Implementation Notes for Test Writer

### Recommended Test Implementation Strategy

Since this is a manifest-only change, the tests should be implemented as a **shell script or integration test** that runs cargo commands and inspects their output. The recommended approach:

1. **B1 + B2 + B7**: Single shell script that runs `cargo check -p centralized-docs` and `cargo check -p centralized-docs --all-features`, asserting exit code 0 for both.

2. **B3 + B4**: Rust integration test using `cargo_metadata` crate (or shell script parsing `cargo metadata --format-version 1` JSON output) to verify `bytemuck` presence and features.

3. **B5 + B6 + B8 + B9**: Rust test that parses `Cargo.toml` using the `toml` crate, checking:
   - `bytemuck` key exists in both `[dependencies]` and `[dev-dependencies]`
   - `bytemuck` has features `["derive"]`
   - `rkyv` is unchanged
   - Diff from git HEAD shows only additions

4. **B10**: Run the existing test suite as a regression gate (`cargo test -p centralized-docs`).

### Pre-existing Dependencies Snapshot (for B6/B9 validation)

The following must be present and unchanged after the edit:

```toml
# [dependencies] — must remain exactly as-is:
rkyv = { version = "0.8", features = ["std", "bytecheck"] }
redb = "2"
sha2 = "0.10"
lru = "0.16.3"
parking_lot = "0.12.5"
rayon = "1.11.0"

# [dev-dependencies] — must remain exactly as-is:
rkyv = { version = "0.8", features = ["std", "bytecheck"] }
redb = "2"
```

---

## Open Questions

None. The contract is unambiguous: add `bytemuck` with `"derive"` feature to both dependency sections, verify compilation and dependency graph correctness.

---

## Exit Criteria Checklist

- [x] Every public API behavior has at least one BDD scenario — (10 behaviors, 10 scenarios; no public API functions exist for this bead)
- [x] Every pure function with multiple inputs has a proptest invariant — (no pure functions introduced)
- [x] Every parsing/deserialization boundary has a fuzz target — (no new parsers/deserializers introduced)
- [x] Every error variant in the Error enum has an explicit test scenario — (5 compile-time error conditions covered in BDD scenarios B1–B10 error variants)
- [x] Mutation threshold target (≥90%) is stated — (100% kill rate for manifest mutations, 7/7 caught)
- [x] No test asserts only `is_ok()` or `is_err()` — (all assertions specify exact values: exit code 0, exact feature names, exact version strings)
