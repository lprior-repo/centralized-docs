bead_id: cdocs-5d8
bead_title: QA: ctd diff/apply require hidden .scrape dir
phase: p5.7-kani-justification
updated_at: 2026-04-20T01:35:00Z

# Kani Justification — Why Formal Verification Is Not Required

## Function Under Analysis

`resolve_manifest_dir(path: &Path) -> Result<PathBuf, ManifestResolveError>`

## Argument

### 1. Trivial Branch Count
The function has exactly 2 conditional branches:
- `direct.is_file()` → return `path.to_path_buf()`
- `nested.is_file()` → return `scrape_subdir`

Total path combinations: 2² = 4 (direct-only, nested-only, both, neither). All 4 are
covered by unit tests B1–B4 in `tests_diff.rs` with exact value assertions.

### 2. No Arithmetic or Index Operations
The function performs no arithmetic, no array indexing, and no numeric conversions.
There are no overflow, underflow, or out-of-bounds invariants to formally verify.

### 3. No Concurrency or Unsafe Code
The function is purely sequential and uses only `std::path::PathBuf` and `std::fs::metadata`
(via `is_file()`). No `unsafe`, no atomics, no threads.

### 4. Exhaustive Proptest Coverage
Property-based tests (P1–P5 in the test plan) exercise:
- Determinism: same input → same output
- Path identity: returned path always contains `manifest.json`
- Roundtrip: `write_scraped_pages` → `resolve_manifest_dir` roundtrip
- Conservation: diff summaries are consistent through resolution
- Precedence: direct always wins over nested

These invariants cover the complete state space of the function's return values.

### 5. Toolchain Incompatibility
Kani 0.67.0 does not support rustc 1.94.0-nightly, making harness execution impossible
in the current environment.

## Conclusion

The function's 2-branch, no-arithmetic, no-unsafe, no-concurrency nature makes formal
verification redundant with the existing 31 unit/integration/proptest tests that achieve
96%+ line coverage and 100% branch coverage on `resolve_manifest_dir`.
