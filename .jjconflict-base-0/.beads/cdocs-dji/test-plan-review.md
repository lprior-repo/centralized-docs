# Test Plan Review: cdocs-dji — Transform Artifact Cache (v2 Re-review)

```
bead_id: cdocs-dji
reviewed_at: 2026-04-02
reviewer: test-reviewer (Mode 1 — Plan Inquisition, re-review after fixes)
contract: .beads/cdocs-dji/contract.md
test_plan: .beads/cdocs-dji/test-plan.md
prior_review: .beads/cdocs-dji/test-plan-review.md (3 LETHAL · 5 MAJOR · 7 MINOR)
```

---

## VERDICT: APPROVED

0 LETHAL · 0 MAJOR · 4 MINOR

All 15 previously flagged findings (3 LETHAL, 5 MAJOR, 7 MINOR) have been resolved. The
plan is now structurally honest: every title matches its Then: block, every error variant
has an exact assertion, every boundary gap has been filled, and mutation survivability
has been addressed with concrete scenarios.

Four minor documentation inconsistencies remain — none affect test coverage completeness.

---

## Prior Finding Verification

### LETHAL Findings (previously 3 → now 0)

| Prior ID | Finding | Fix Applied | Verified |
|----------|---------|-------------|----------|
| LETHAL-1 | `LinkMapFingerprintFailed` variant: zero scenarios | **B08 added** (test-plan.md:228–244). Then: asserts `Err(TransformArtifactError::LinkMapFingerprintFailed { message: m })` where `m.is_empty() == false`. Implementation note (line 244) commits to changing `compute_link_map_fingerprint` signature to return `Result<ContentHash, TransformArtifactError>`. | ✅ FIXED |
| LETHAL-2 | `CacheReadFailed` variant untested from `load_cached_artifact` | **B19 added** (test-plan.md:399–424). Then: asserts `Err(TransformArtifactError::CacheReadFailed { source_path: "a.md", message: m })` where `m.is_empty() == false`. Implementation note (lines 419–424) recommends corrupting redb file externally. | ✅ FIXED |
| LETHAL-3 | B11/B14 bait-and-switch (title ≠ Then:) | **Renamed + new scenarios added.** Old B11 (titled `Err(CacheReadFailed)`, Then: `Ok(None)`) → now **B17** with honest title "returns `Ok(None)` when transform cache is disabled" (line 366). Old B14 (titled `Err(CacheWriteFailed)`, Then: `Ok(())`) → now **B22** with honest title "returns `Ok(())` when transform cache is disabled" (line 462). Genuine error scenarios now exist as **B19** and **B23**. | ✅ FIXED |

### MAJOR Findings (previously 5 → now 0)

| Prior ID | Finding | Fix Applied | Verified |
|----------|---------|-------------|----------|
| MAJOR-1 | B07 vague assertion ("valid ContentHash") | **B11** (new numbering, test-plan.md:270–279) now asserts concrete: `result.as_bytes().len() == 32 && result.as_bytes() != [0u8; 32]`. | ✅ FIXED |
| MAJOR-2 | B15 incomplete error assertion (no field values) | **B23** (new numbering, test-plan.md:479–496) now asserts `Err(CacheWriteFailed { source_path: "a.md", message: m })` where `m.is_empty() == false`, plus verifies key is absent afterward. | ✅ FIXED |
| MAJOR-3 | `write_artifact_to_output` missing 4 boundaries | **B27** (empty markdown rejection, line 557–579), **B28** (large markdown ≥10MB, line 582–599), **B29** (missing docs_dir creation, line 602–620) added. | ✅ FIXED |
| MAJOR-4 | `load_cached_artifact` missing boundaries | **B20** (large cached artifact ≥1MB, line 426–440) added. | ✅ FIXED |
| MAJOR-5 | Mutation: `CacheReadFailed` branch deletion survives | Resolved by **B19** — same fix as LETHAL-2. | ✅ FIXED |

### MINOR Findings (previously 7 → now 0 from prior, 4 new)

| Prior ID | Finding | Fix Applied | Verified |
|----------|---------|-------------|----------|
| MINOR-1 | B24 asserts 5+ properties in one E2E scenario | **Split into B38** (correct counts, line 770–796) **and B39** (correct content per file, line 798–811). | ✅ FIXED |
| MINOR-2 | B23 preconditions not specific (how pre-populated?) | **B34** (new numbering, line 689–714) specifies exact artifact, exact key computation, exact `cache.put_transform` call in Given block. | ✅ FIXED |
| MINOR-3 | B11/B14 disabled-cache config not specific | **B17** and **B22** now specify exact config: `CacheConfig { enabled_cache_types: vec![CacheType::Analysis] }` (CacheType::Transform excluded). | ✅ FIXED |
| MINOR-4 | "valid link_map" / "valid analyses" not self-contained | Changelog confirms "All replaced with concrete inline preconditions (DAMP)." Verified: B15–B41 scenarios specify concrete values. | ✅ FIXED |
| MINOR-5 | B28 didn't state DocCache creation side effect | **B41** (line 835) now explicitly states "DocCache is created and opened by run_index at pipeline start, before STEP 4." | ✅ FIXED |
| MINOR-6 | Matrix boundary entries not in §3 | **Promoted to §3 as B05** (single-char path, line 188–199), **B06** (255-char path, line 201–212), **B07** (multi-byte UTF-8, line 214–226). | ✅ FIXED |
| MINOR-7 | `transform_all_cached` empty analyses slice untested | **B40 added** (line 813–825): returns `Ok(TransformResult { success_count: 0, total_count: 0, error_count: 0 })`. | ✅ FIXED |

---

## Axis 1 — Contract Parity

### Public Functions → BDD Scenario Coverage

| # | Public Function | BDD Scenarios | Status |
|---|----------------|---------------|--------|
| 1 | `TransformArtifactKey::compute` | B01, B02, B03 (3 sub), B04, B05, B06, B07 | PASS |
| 2 | `TransformArtifactKey::as_bytes` | B04 | PASS |
| 3 | `compute_artifact_key` (free fn) | Covered by B01–B07 via method form | PASS* |
| 4 | `compute_link_map_fingerprint` | B08, B09, B10, B11 | PASS |
| 5 | `load_cached_artifact` | B15, B16, B17, B18, B19, B20 | PASS |
| 6 | `store_artifact` | B21, B22, B23 | PASS |
| 7 | `transform_all_cached` | B30–B40 | PASS |
| 8 | `write_artifact_to_output` | B24, B25, B26, B27, B28, B29 | PASS |

*See MINOR-1: `compute_artifact_key` and `TransformArtifactKey::compute` appear to be
the same function (free fn wrapper vs method). If they diverge at implementation, the free
function needs its own scenario.

### Error Variant → Scenario Coverage

| # | Error Variant | Asserting Scenario(s) | Field Assertions | Status |
|---|--------------|----------------------|-----------------|--------|
| 1 | `EmptySourcePath` | B30 | Exact variant (no fields) | PASS |
| 2 | `MissingIdMapping { source_path }` | B25, B31 | `source_path` matches exactly | PASS |
| 3 | `LinkMapFingerprintFailed { message }` | B08 | `message.is_empty() == false` | PASS |
| 4 | `CacheReadFailed { source_path, message }` | B19 | `source_path == "a.md"`, `message.is_empty() == false` | PASS |
| 5 | `CacheWriteFailed { source_path, message }` | B23, B36 | `source_path` matches exactly, `message.is_empty() == false` | PASS |
| 6 | `DeserializationFailed { source_path, message }` | B18 | `source_path == "a.md"`, `message.is_empty() == false` | PASS |
| 7 | `FileReadFailed { source_path, message }` | B32 | `source_path == "nonexistent.md"`, `message.is_empty() == false` | PASS |
| 8 | `TransformComputationFailed { source_path, message }` | B35 | `source_path == "bad.md"`, `message.is_empty() == false` | PASS |
| 9 | `OutputWriteFailed { source_path, message }` | B26, B27, B37 | `source_path` matches exactly, `message.is_empty() == false` | PASS |

**9/9 variants covered with exact assertions. PASS.**

---

## Axis 2 — Assertion Sharpness

Every "Then:" block in every BDD scenario (B01–B41, including B03 sub-tests) was read
verbatim. No banned patterns found.

| Check | Result |
|-------|--------|
| `is_ok()` as sole assertion | **NONE FOUND** |
| `is_err()` as sole assertion | **NONE FOUND** |
| `> 0` or boolean without concrete value | **NONE FOUND** |
| `Some(_)` without specifying inner value | **NONE FOUND** |
| Vague "valid" or "correct" as assertion | **NONE FOUND** |

All assertions specify exact values: `result.as_bytes().len() == 32`, exact error variants
with field values, exact byte-identical content comparisons, exact `TransformResult` field
values.

**PASS.**

---

## Axis 3 — Trophy Allocation

### Density Audit

| Metric | Value |
|--------|-------|
| Public functions (contract) | 8 |
| BDD test functions | 43 (B01–B41, B03 has 3 sub-tests) |
| Proptest invariants | 5 (PPT-01 through PPT-05) |
| Fuzz targets | 2 |
| Kani harnesses | 2 |
| Static checks | 1 |
| **Total planned test functions** | **53** |
| **Ratio** | **6.6×** (target ≥5×) |

BDD-only ratio: 43/8 = 5.4×. Above threshold even without supplementary tests.

### Pure Function → Proptest Coverage

| Pure Function | Proptest Invariant | Status |
|---------------|-------------------|--------|
| `TransformArtifactKey::compute` | PPT-01 (determinism), PPT-02 (distinct-input) | PASS |
| `compute_link_map_fingerprint` | PPT-03 (order independence) | PASS |
| `TransformArtifact` serde | PPT-04 (round-trip) | PASS |
| `composite_hash` (underlying) | PPT-05 (output length) | PASS |

All pure functions with non-trivial input spaces have proptest invariants. **PASS.**

### Parser/Deserializer → Fuzz Target Coverage

| Target | Fuzz Target | Status |
|--------|-------------|--------|
| `TransformArtifact` JSON deserialization | Fuzz Target 1 (line 903–917) | PASS |
| `compute_link_map_fingerprint` adversarial keys | Fuzz Target 2 (line 919–933) | PASS |

**PASS.**

---

## Axis 4 — Boundary Completeness

### Per-Function Boundary Audit

| Function | Min Valid | Max Valid | Below Min | Above Max | Empty/Zero | Overflow/Edge | Missing |
|----------|-----------|-----------|-----------|-----------|------------|---------------|---------|
| `TransformArtifactKey::compute` | B05 ("a") ✓ | B06 (255 chars) ✓ | B30 (empty → EmptySourcePath) ✓ | N/A (SHA-256 unbounded) | B30 ✓ | B07 (multi-byte UTF-8) ✓ | 0 |
| `compute_link_map_fingerprint` | B11 (empty map) ✓ | PPT-03 (1..20 entries), Fuzz-2 (10K entries) ✓ | N/A | Fuzz-2 ✓ | B11 ✓ | B08 (serialization failure) ✓ | 0 |
| `TransformArtifact` serde | B12 (normal) ✓ | PPT-04 (0..5000 chars) ✓ | — | — | B13 (empty markdown) ✓ | B14 (CJK + emoji) ✓ | 0 |
| `load_cached_artifact` | B15 (cache hit) ✓ | B20 (1MB artifact) ✓ | B16 (empty cache) ✓ | — | B17 (disabled cache) ✓ | B18 (corrupt), B19 (storage error) ✓ | 0 |
| `store_artifact` | B21 (normal) ✓ | B23 (oversized) ✓ | B22 (disabled cache) ✓ | B23 ✓ | — | — | 0 |
| `write_artifact_to_output` | B24 (normal) ✓ | B28 (10MB) ✓ | B27 (empty markdown) ✓ | — | B27 ✓ | B29 (missing docs_dir), B26 (I/O failure) ✓ | 0 |
| `transform_all_cached` | B33 (single analysis) ✓ | B38–B39 (2 analyses mixed) ✓ | B40 (empty slice) ✓ | PPT/Fuzz covers scale | B40 ✓ | B30–B32, B35–B37 (all error paths) ✓ | 0 |

**No function has ≥3 missing boundaries. PASS.**

---

## Axis 5 — Mutation Survivability

### Thought-Experiment Results

Every listed mutation from §7 Mutation Testing Checkpoints (test-plan.md:966–987) was
verified against the asserting scenarios:

| Mutation Target | Caught By | Verified |
|----------------|-----------|----------|
| Delete `EmptySourcePath` branch | B30 | ✅ |
| Delete `MissingIdMapping` branch | B25, B31 | ✅ |
| Delete `LinkMapFingerprintFailed` branch | B08 | ✅ |
| Delete `CacheReadFailed` branch | B19 | ✅ |
| Delete `CacheWriteFailed` branch | B23, B36 | ✅ |
| Delete `DeserializationFailed` branch | B18 | ✅ |
| Delete `FileReadFailed` branch | B32 | ✅ |
| Delete `TransformComputationFailed` branch | B35 | ✅ |
| Delete `OutputWriteFailed` branch | B26, B27, B37 | ✅ |
| `compute`: swap content_hash / link_map_fp args | B03 sub-tests | ✅ |
| `compute_link_map_fingerprint`: remove `sort_by_key` | B09 | ✅ |
| `compute_link_map_fingerprint`: hash only keys | B10 | ✅ |
| `compute_link_map_fingerprint`: replace `.map_err()` with `.expect()` | B08 | ✅ |
| `store_artifact`: skip `put_transform` call | B21 | ✅ |
| `load_cached_artifact`: always return `Ok(None)` | B15 | ✅ |
| `load_cached_artifact`: wrong `source_path` in artifact | B15 (field match) | ✅ |
| `load_cached_artifact`: delete `CacheReadFailed` match arm | B19 | ✅ |
| `write_artifact_to_output`: write empty string | B24 (byte-identical) | ✅ |
| `write_artifact_to_output`: skip `link_map.get` check | B25 | ✅ |
| `transform_all_cached`: skip cache lookup | B34 | ✅ |
| `transform_all_cached`: skip `store_artifact` | B33 (cache populated check) | ✅ |
| `transform_all_cached`: return `success_count = 0` | B38 | ✅ |
| `transform_all_cached`: skip empty source_path validation | B30 | ✅ |
| `TransformArtifact` Serialize: omit a field | B12 | ✅ |
| `TransformArtifact` Deserialize: swap fields | B12 | ✅ |
| `write_artifact_to_output`: skip docs_dir creation | B29 | ✅ |
| `write_artifact_to_output`: ignore empty markdown check | B27 | ✅ |

**Additional thought-experiment mutations (not in plan's table):**

| Mutation | Caught By | Status |
|----------|-----------|--------|
| `TransformArtifactKey::compute`: return constant key | B03 (distinct-input → distinct-output) | ✅ |
| `load_cached_artifact`: return artifact with wrong `content_hash` | B15 (field-by-field match includes content_hash) | ✅ |
| `transform_all_cached`: write wrong artifact's content to wrong file | B39 (per-file content check) | ✅ |
| `store_artifact`: store under wrong key | B21 (subsequent read with correct key must return artifact) | ✅ |
| `transform_all_cached`: return `Ok(Default::default())` | B33 (checks concrete `{ success_count: 1, total_count: 1, error_count: 0 }`) | ✅ |

**All deletable branches have catching tests. No surviving mutations identified. PASS.**

---

## Axis 6 — Holzmann Plan Audit

### Rule 2 — Bound Every Loop

No loops in test bodies. Proptest strategies have explicit bounds (`1..100 chars`,
`1..20 entries`, `0..5000 chars`, `1..10 parts`). **PASS.**

### Rule 4 — One Function, One Job

B38 and B39 are properly split (counts vs content). B41 is a single E2E parity check.
No scenario asserts >3 independent properties without clear structural separation.
**PASS.**

### Rule 5 — State Your Assumptions

All scenarios specify concrete Given blocks with exact values: specific `ContentHash::compute(b"...")`,
specific `TransformArtifact { ... }` structs, specific `CacheConfig { ... }` fields.
No unqualified "valid link_map" or "valid analyses" remain. **PASS.**

### Rule 6 — Never Swallow Errors

No `let _ =`, no `.ok()` discard, no bare `unwrap()` as assertion. All scenarios assert
`result == Ok(...)` or `result == Err(ExactVariant { ... })`. **PASS.**

### Rule 7 — No Shared Mutable State

Each scenario describes its own DocCache instance and filesystem setup. No cross-scenario
shared state. **PASS.**

### Rule 8 — Surface Your Side Effects

B41 explicitly states "DocCache is created and opened by run_index at pipeline start,
before STEP 4." Integration scenarios name their side effects: cache pre-population,
file creation, directory setup. **PASS.**

### Rule 9 — One Layer of Magic

No helper abstraction chains described in the plan. Fixture setup is inline per scenario.
**PASS.**

---

## MINOR FINDINGS (4/5 threshold)

### MINOR-1: `compute_artifact_key` delegation assumption
- **Contract ref**: contract.md lines 329–333 (free fn), lines 93–104 (method)
- **Test-plan ref**: Behavior Inventory table, line 33
- **Detail**: The plan states `compute_artifact_key` is "Covered by B01–B04 via method form."
  This assumes the free function is a pure delegation to `TransformArtifactKey::compute`.
  If the implementation diverges (e.g., adds input validation, constructs the key differently),
  the free function has zero direct test coverage.
- **Risk**: Low — likely a delegation wrapper.
- **Action**: Verify at implementation time. If `compute_artifact_key` contains any
  non-trivial logic, add at least one scenario testing it directly.

### MINOR-2: Trophy allocation count mismatch
- **Test-plan ref**: §2 Trophy Allocation (line 100–107)
- **Detail**: §2 claims "14 unit / 22 integration / 4 e2e / 1 static = 41" but:
  - Integration lists B15–B29 (15), B33–B37 (5), B40 (1) = 21, not 22
  - E2E lists B30–B32 (3), B38–B39 (2), B41 (1) = 6, not 4
  - The §8 combinatorial matrix assigns B30–B32 as "integration" layer, which would make
    integration = 24 and E2E = 3 — also not matching §2's claimed 22/4 split
- **Risk**: None to test coverage — all behaviors are present and assigned to some layer.
- **Action**: Reconcile §2 counts with §8 matrix layer assignments. Pick one source of truth.

### MINOR-3: B33 Given block omits exact source file content
- **Test-plan ref**: B33 (line 669–687), line 674
- **Detail**: B33 Given says "source file 'a.md' exists on disk at the expected source
  directory" without specifying file content. Contrast with B34 (line 705) which specifies
  `b"original-file-bytes"`. Holzmann Rule 5: state your assumptions concretely.
- **Risk**: Low — the source content determines the content hash, which determines the
  cache key. A test implementer could use any content, but the plan should specify what
  content to use so the content_hash in the scenario is consistent.
- **Action**: Add file content to B33 Given block, e.g., "source file 'a.md' on disk with
  content matching the Analysis content."

### MINOR-4: Serde anti-pattern matrix entries lack §3 BDD scenarios
- **Test-plan ref**: §8 matrix lines 1025–1027, exit criteria line 1121
- **Detail**: The combinatorial matrix lists three serde anti-pattern tests:
  "corrupt JSON", "wrong types", "missing fields" — all marked "unit" layer but none
  have §3 BDD scenarios. The exit criteria (line 1121) claims "All boundary entries from
  combinatorial matrix have corresponding BDD scenarios in §3." These anti-pattern entries
  contradict that claim.
- **Risk**: Low — these patterns are covered by Fuzz Target 1 corpus seeds (lines 913–916)
  and by PPT-04's anti-case ("Corrupt JSON bytes should fail deserialization").
- **Action**: Either promote these to §3 with full BDD scenarios, or clarify the exit
  criteria to distinguish "boundary entries" from "anti-pattern entries."

---

## Summary Statistics

| Dimension | Status | Detail |
|-----------|--------|--------|
| Contract parity (pub fn → scenario) | **PASS** | 8/8 functions, 9/9 error variants |
| Assertion sharpness | **PASS** | Zero banned patterns. All assertions concrete. |
| Trophy allocation | **PASS** | 6.6× density (53 tests / 8 functions), all proptest/fuzz covered |
| Boundary completeness | **PASS** | No function with ≥3 missing boundaries |
| Mutation survivability | **PASS** | All 19+ mutations caught by named scenarios |
| Holzmann rules | **PASS** | All 9 applicable rules satisfied |

**STATUS: APPROVED**
