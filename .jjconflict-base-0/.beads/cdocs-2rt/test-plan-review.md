# Test Plan Review: cdocs-2rt — Deterministic Config Hashing & `compute_file_diff`

**STATUS: APPROVED**

---

## Re-Audit Note

This is a re-audit following revision of `test-plan.md`. The previous review (REJECTED)
identified 3 MAJOR findings (M-1, M-2, M-3) and 9 MINOR findings (m-1 through m-9).
All 12 findings have been verified as resolved. See defect tracking below.

---

## Axis 1 — Contract Parity

### Public Functions → BDD Coverage

| `pub fn` | BDD Scenarios | Verdict |
|----------|---------------|---------|
| `compute_config_hash` | 3.1–3.8 (8 scenarios) | PASS |
| `compute_file_diff` | 3.9–3.34 (26 scenarios) | PASS |

### Error Variants → Exact-Variant Scenarios

| Error Variant | Scenario | Asserts Exact Variant? | Verdict |
|---------------|----------|------------------------|---------|
| `DiffError::SourceDirNotFound(String)` | 3.9 | Yes — `Err(DiffError::SourceDirNotFound(path_string))` + message check | PASS |
| `DiffError::FileRead { path, source }` | 3.10, 3.10b | Yes — exact variant + path match + `source.kind()` (NotFound / PermissionDenied) | PASS |
| `DiffError::PathTraversal { path }` | 3.11, 3.12, 3.13, 3.30 | Yes — exact variant + concrete path value | PASS |

**Axis 1 Verdict: PASS — 0 findings.**

---

## Axis 2 — Assertion Sharpness

Every "Then:" in every scenario audited for concrete value specificity.

| Scenario | Assertion | Verdict |
|----------|-----------|---------|
| 3.1 | `content_hash(b"")` — exact | PASS |
| 3.2 | `content_hash(b"hello world")` — exact | PASS |
| 3.3 | `content_hash(b"")` — exact | PASS |
| 3.4 | `content_hash(b"")` — exact | PASS |
| 3.5 | `content_hash(b"deterministic test content")` + identical across calls | PASS |
| 3.6 | `hash_a == content_hash(b"aaa")` AND `hash_b == content_hash(b"bbb")` — concrete | PASS ✅ (was M-1) |
| 3.7 | `content_hash(b"")` — exact | PASS ✅ (was m-2) |
| 3.8 | `content_hash(&vec![b'X'; 1_048_576])` — exact | PASS ✅ (was m-3) |
| 3.9 | `Err(DiffError::SourceDirNotFound(path_string))` + message | PASS |
| 3.10 | `Err(DiffError::FileRead { path, source })` + `source.kind() == NotFound` | PASS |
| 3.10b | `Err(DiffError::FileRead { path, source })` + `source.kind() == PermissionDenied` | PASS |
| 3.11 | `Err(DiffError::PathTraversal { path })` + path `"../../etc/passwd"` | PASS |
| 3.12 | `Err(DiffError::PathTraversal { path })` + path `"/etc/passwd"` | PASS |
| 3.13 | `Err(DiffError::PathTraversal { path })` + path `"../outside.md"` | PASS |
| 3.14 | `new == {"a.md", "b.md"}` + other three buckets empty | PASS |
| 3.15 | `deleted == {"old.md"}` + other three buckets empty | PASS |
| 3.16 | `unchanged == {"same.md"}` + other three buckets empty | PASS |
| 3.17 | `changed == {"edit.md"}` + other three buckets empty | PASS |
| 3.18 | `changed == {"stable.md"}` + other three buckets empty | PASS |
| 3.19 | `changed == {"both.md"}` + other three buckets empty | PASS ✅ (was m-1) |
| 3.20 | `new == {"fresh.md"}` + other three buckets empty | PASS |
| 3.21 | `deleted == {"gone.md", "removed.md"}` + other three buckets empty | PASS |
| 3.22 | 6 pairwise intersection == ∅ | PASS |
| 3.23 | Union == discovered; deleted == stored − discovered | PASS |
| 3.24 | All four buckets with exact contents | PASS |
| 3.25 | All four buckets empty | PASS |
| 3.26 | Pre/post state snapshots (mtime + clone comparison) | PASS |
| 3.27 | Concrete lengths (20/15/15/10) + partition invariant | PASS |
| 3.28 | `Ok(FileDiff)` + single bucket + 10 identical calls | PASS |
| 3.29 | `changed == {"doc.md"}` + all four buckets | PASS ✅ (was m-7) |
| 3.30 | `Err(DiffError::PathTraversal { path })` + path `"link.md"` | PASS ✅ (was m-9) |
| 3.31 | `Err(FileRead { path })` OR `Err(PathTraversal { path })` + path `""` | PASS ✅ (was m-4) |
| 3.32 | `Err(...)` constrained to {FileRead, PathTraversal} — see MINOR-1 | PASS* |
| 3.33 | `Ok(FileDiff)` + exact bucket contents for 3 mismatched keys + 1 real | PASS ✅ (was m-6) |
| 3.34 | `unchanged == {"sized.md"}` + all four buckets | PASS ✅ (was m-8) |

**Axis 2 Verdict: PASS — 0 MAJOR, 1 MINOR.**

### MINOR-1: Scenario 3.32 Imprecise Wording (test-plan.md:603-606)

The Then: clause says "the exact variant does not matter (FileRead, PathTraversal, or
OS error)." The parenthetical constrains the acceptable variants, which prevents bare
`is_err()` — the intent is correct. However, the "does not matter" qualifier and the
"result is Err (not Ok with incorrect classification)" line are imprecise. A literalist
test implementation could satisfy this with `assert!(result.is_err())`.

**Recommendation**: Replace the "does not matter" language with an explicit match:
```rust
assert!(matches!(result, Err(DiffError::FileRead { .. }) | Err(DiffError::PathTraversal { .. })));
```
This explicitly excludes `SourceDirNotFound` (which would be semantically wrong since
source_dir exists). Not blocking — the parenthetical makes intent clear — but tighten
during implementation.

---

## Axis 3 — Trophy Allocation

### Density Audit

| Metric | Count |
|--------|-------|
| Public functions | 2 |
| BDD scenarios | 34 |
| Proptest invariants | 6 |
| Fuzz targets | 2 |
| Kani harnesses | 3 |
| **Total planned tests** | **45** |
| **Ratio** | **22.5× (target ≥5×)** |

### Pure Function Proptest Coverage

| Pure/Near-Pure Function | Proptest | Verdict |
|--------------------------|----------|---------|
| `compute_config_hash` | Proptest 1 (any::<Vec<u8>>), Proptest 2 (None constant) | PASS |
| Partition invariant (mathematical property) | Proptest 3, 4, 5 | PASS |
| Rayon determinism | Proptest 6 | PASS |

### Fuzz Target Coverage

| Risk Surface | Fuzz Target | Verdict |
|--------------|-------------|---------|
| Source path strings (traversal, panic) | Fuzz 1 with 10 corpus seeds | PASS |
| File content hashing (SHA-256, OOM) | Fuzz 2 with 6 corpus seeds | PASS |

### Integration/Unit Ratio

22 integration / 10 unit = 69% / 31%. Justified: `compute_file_diff` is I/O-bound by design.

**Axis 3 Verdict: PASS — 0 findings.**

---

## Axis 4 — Boundary Completeness

### `compute_config_hash` Boundaries

| Boundary | Specified? | Scenario |
|----------|-----------|----------|
| None input | ✅ | 3.1 |
| Valid readable file | ✅ | 3.2 |
| Non-existent file | ✅ | 3.3 |
| Unreadable file (0o000) | ✅ | 3.4 |
| Determinism | ✅ | 3.5 |
| Different contents | ✅ | 3.6 |
| Empty file (0 bytes) | ✅ | 3.7 |
| Large file (1MB+) | ✅ | 3.8 |

**0 missing boundaries.**

### `compute_file_diff` Boundaries

| Boundary | Specified? | Scenario |
|----------|-----------|----------|
| Empty discovered_files | ✅ | 3.15 |
| Empty stored_hashes | ✅ | 3.14 |
| Both empty | ✅ | 3.25 |
| Large file set (50) | ✅ | 3.27 |
| Source dir missing | ✅ | 3.9 |
| File missing on disk | ✅ | 3.10 |
| Permission denied | ✅ | 3.10b |
| Path traversal (`../../`) | ✅ | 3.11 |
| Path traversal (absolute) | ✅ | 3.12 |
| Path traversal (`../` prefix) | ✅ | 3.13 |
| Duplicate source_path | ✅ | 3.28 |
| Empty source_path (`""`) | ✅ | 3.31 |
| Very long source_path (PATH_MAX) | ✅ | 3.32 |
| Stored hash key format mismatch | ✅ | 3.33 |
| Config path nonexistent → Changed | ✅ | 3.29 |
| size_bytes = 0 isolation | ✅ | 3.34 |
| Symlink-based traversal | ✅ | 3.30 |

**0 missing boundaries.** All 9 previous gaps resolved.

**Axis 4 Verdict: PASS — 0 findings.**

---

## Axis 5 — Mutation Survivability

22 mutations applied. All caught.

| # | Mutation | Caught By | Status |
|---|----------|-----------|--------|
| 1 | Remove `None => content_hash(b"")` branch | 3.1 | ✅ KILLED |
| 2 | Replace fallback with `content_hash(b"x")` | 3.3 | ✅ KILLED |
| 3 | Remove `SourceDirNotFound` check | 3.9 | ✅ KILLED |
| 4 | Remove `PathTraversal` canonicalization | 3.11 + 3.12 + 3.13 + 3.30 | ✅ KILLED |
| 5 | Swap Unchanged ↔ Changed | 3.16 + 3.17 | ✅ KILLED |
| 6 | Remove New branch | 3.14 + 3.20 | ✅ KILLED |
| 7 | Remove Deleted bucket | 3.15 + 3.21 | ✅ KILLED |
| 8 | Change `&&` to `||` in Unchanged condition | 3.18 | ✅ KILLED |
| 9 | Return `Ok(Default::default())` | 3.14 | ✅ KILLED |
| 10 | Flip `stored_hashes.get()` always None | 3.16 | ✅ KILLED |
| 11 | Flip `stored_hashes.get()` always Some | 3.20 | ✅ KILLED |
| 12 | Remove content hash computation | 3.16 | ✅ KILLED |
| 13 | Remove config hash computation | 3.18 | ✅ KILLED |
| 14 | Early return after first file | 3.27 (50 files) | ✅ KILLED |
| 15 | Remove FileRead propagation | 3.10 | ✅ KILLED |
| 16 | Return hash of PATH not BYTES | 3.2 + 3.6 | ✅ KILLED |
| 17 | Config nonexistent → use old hash | 3.29 | ✅ KILLED |
| 18 | Symlink traversal not detected | 3.30 | ✅ KILLED |
| 19 | size_bytes influences classification | 3.34 | ✅ KILLED |
| 20 | Duplicate: nondeterministic via rayon | 3.28 (10 calls) + Proptest 6 | ✅ KILLED |
| 21 | Key format mismatch causes panic | 3.33 | ✅ KILLED |
| 22 | Empty source_path succeeds as Ok | 3.31 | ✅ KILLED |

**Kill rate: 22/22 = 100%.** Target ≥90% exceeded.

### Acceptable Survivors (non-behavioral)

- Variable renames, logging text changes, rayon→sequential swap — all cosmetic.

**Axis 5 Verdict: PASS — 0 findings.**

---

## Axis 6 — Holzmann Plan Audit

| Rule | Assessment | Verdict |
|------|-----------|---------|
| 1 — Keep it Linear | All scenarios: single Given→When→Then. No nested conditionals. | PASS |
| 2 — Bound Every Loop | No loops in any scenario body. Proptest ranges bounded. | PASS |
| 3 — Know What You Own | All scenarios use `tempfile::tempdir()`. Self-cleaning. | PASS |
| 4 — One Function, One Job | Each scenario tests exactly one behavior. | PASS |
| 5 — State Your Assumptions | Every scenario has explicit Given block with concrete values. | PASS |
| 6 — Never Swallow Errors | No `let _ =`, no `.ok()` in any scenario. All Results matched. | PASS |
| 7 — Narrow Your State | Each scenario creates own tempdir. No shared mutable state. | PASS |
| 8 — Surface Your Side Effects | File creation explicit in Given blocks. No hidden helpers. | PASS |
| 9 — One Layer of Magic | No helper chains. Scenarios self-contained. | PASS |
| 10 — Warnings Are Errors | Clippy gate specified in static layer. | PASS |

**Axis 6 Verdict: PASS — 0 findings.**

---

## Verdict Summary

| Axis | Verdict | LETHAL | MAJOR | MINOR |
|------|---------|--------|-------|-------|
| 1. Contract Parity | PASS | 0 | 0 | 0 |
| 2. Assertion Sharpness | PASS | 0 | 0 | 1 |
| 3. Trophy Allocation | PASS | 0 | 0 | 0 |
| 4. Boundary Completeness | PASS | 0 | 0 | 0 |
| 5. Mutation Survivability | PASS | 0 | 0 | 0 |
| 6. Holzmann Rules | PASS | 0 | 0 | 0 |
| **TOTAL** | | **0** | **0** | **1** |

---

## Previous Defect Resolution

All 12 findings from the previous review are verified resolved:

| Previous Finding | Description | Resolution | Verified |
|-----------------|-------------|------------|----------|
| M-1 | Scenario 3.6: relational `!=` without concrete values | Scenario 3.6 now asserts `hash_a == content_hash(b"aaa")` AND `hash_b == content_hash(b"bbb")` | ✅ test-plan.md:171-173 |
| M-2 | 7 missing boundaries on `compute_file_diff` | Scenarios 3.28, 3.29, 3.30, 3.31, 3.32, 3.33, 3.34 added | ✅ test-plan.md:497-654 |
| M-3 | Duplicate source_path mutation survivor | Scenario 3.28 + Proptest 6 verify determinism and no-panic | ✅ test-plan.md:512-534, 738-759 |
| m-1 | Scenario 3.15: missing bucket assertions | Scenario 3.19 now asserts all four buckets | ✅ test-plan.md:378-385 |
| m-2 | Empty file (0 bytes) — no BDD scenario | Scenario 3.7 added | ✅ test-plan.md:179-188 |
| m-3 | Large file (1MB+) — no BDD scenario | Scenario 3.8 added | ✅ test-plan.md:190-199 |
| m-4 | Empty source_path — no scenario | Scenario 3.31 added | ✅ test-plan.md:576-592 |
| m-5 | Very long source_path — no scenario | Scenario 3.32 added | ✅ test-plan.md:594-608 |
| m-6 | Stored hash key mismatch — no scenario | Scenario 3.33 added | ✅ test-plan.md:610-628 |
| m-7 | Config path nonexistent → Changed — no scenario | Scenario 3.29 added | ✅ test-plan.md:536-558 |
| m-8 | size_bytes = 0 — no scenario | Scenario 3.34 added | ✅ test-plan.md:630-654 |
| m-9 | Symlink traversal — no scenario | Scenario 3.30 added | ✅ test-plan.md:560-574 |

---

## LETHAL FINDINGS

None.

## MAJOR FINDINGS

None.

## MINOR FINDINGS (1 — threshold ≥5 for rejection)

- **MINOR-1** (test-plan.md:603-606): Scenario 3.32 Then: clause says "the exact variant
  does not matter." The parenthetical constrains to `{FileRead, PathTraversal}`, which
  prevents bare `is_err()`, but the "does not matter" language is imprecise and could
  lead to a lazy test implementation. Recommend replacing with explicit match assertion
  during implementation. Not blocking.

---

## Severity Assessment

| Severity | Count | Threshold | Triggers Rejection? |
|----------|-------|-----------|---------------------|
| LETHAL | 0 | ≥1 | No |
| MAJOR | 0 | ≥3 | No |
| MINOR | 1 | ≥5 | No |

**0 LETHAL + 0 MAJOR + 1 MINOR → APPROVED.**

---

## NOTE FOR IMPLEMENTATION

1. **Scenario 3.32**: Tighten "the exact variant does not matter" to explicit
   `matches!(result, Err(DiffError::FileRead { .. }) | Err(DiffError::PathTraversal { .. }))`.
   This excludes `SourceDirNotFound` which would be semantically incorrect.

2. **Scenario 3.31**: Per Open Questions (line 994-998), pin down whether empty
   source_path returns `FileRead` or `PathTraversal` during implementation.

---

*Inquisition complete. All previous defects verified resolved. Plan is clean.*
