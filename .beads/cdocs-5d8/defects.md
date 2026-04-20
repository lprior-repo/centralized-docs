---
bead_id: cdocs-5d8
reviewed_at: 2026-04-20
verdict: APPROVED
reviewer: black-hat-reviewer
---

# Defects: cdocs-5d8 — resolve_manifest_dir

## Summary

5 defects found. None blocking. Core logic is correct, well-tested, and follows all 5 review phases.

## Defects

### DEFECT-P1-1: Behavioral Refinement — `is_file()` vs contract's `exists()`
- **Phase**: 1 (Contract Parity)
- **Severity**: LOW
- **Location**: `centralized-docs/src/watch/diff.rs` lines 45, 47
- **Detail**: Contract (line 72) specifies `std::fs::exists` checks. Implementation uses `Path::is_file()`. This is a behavioral IMPROVEMENT — `is_file()` correctly rejects directories, FIFOs, sockets, and char devices named `manifest.json`, which `exists()` would falsely accept. Red Queen adversarial tests (RQ-TYPE-1, RQ-TYPE-2, RQ-GEN2-TYPE-4, RQ-GEN2-TYPE-5) exploit this exact weakness.
- **Fix**: Update contract Post5 and INV3 to specify `is_file()` instead of `exists()`.

### DEFECT-P1-2: Outdated Docstring on `diff_directories`
- **Phase**: 1 (Contract Parity)
- **Severity**: MEDIUM
- **Location**: `centralized-docs/src/watch/diff.rs` lines 175–181
- **Detail**: Docstring says "Compute a plan by comparing two `.scrape` directories" but the function now transparently handles both scrape output roots (with `.scrape/` subdirectory) and direct manifest directories via `resolve_manifest_dir`. Docstring is misleading about what the function accepts.
- **Contract Reference**: Contract lines 157–167 specify updated docstring.
- **Fix**: Update docstring to mention `resolve_manifest_dir` and both directory layouts.

### DEFECT-P1-3: Missing Docstring on `read_manifest`
- **Phase**: 1 (Contract Parity)
- **Severity**: MEDIUM
- **Location**: `centralized-docs/src/cmd/watch.rs` line 192
- **Detail**: Function has no docstring. Contract (lines 186–195) specifies a full docstring with `# Errors` section describing the `resolve_manifest_dir` integration and failure modes.
- **Fix**: Add docstring per contract specification.

### DEFECT-P1-4: Missing `# Examples` in `resolve_manifest_dir` Docstring
- **Phase**: 1 (Contract Parity)
- **Severity**: LOW
- **Location**: `centralized-docs/src/watch/diff.rs` lines 31–39
- **Detail**: Docstring is missing the `# Examples` section with two `no_run` examples specified in contract lines 141–150.
- **Fix**: Add `# Examples` section per contract.

### DEFECT-P2-1: `diff_directories` Exceeds 25-Line Limit
- **Phase**: 2 (Farley Engineering Rigor)
- **Severity**: LOW
- **Location**: `centralized-docs/src/watch/diff.rs` lines 182–226 (45 lines)
- **Detail**: Function is 45 lines, exceeding the 25-line hard constraint. Only 4 lines (183–186) were added by this bead — the bulk is pre-existing. Should be decomposed in a future cleanup pass (e.g., extract manifest reading into a helper).
- **Fix**: Non-urgent. Create follow-up task for decomposition.

## Recommended Follow-up Tasks

1. Update contract.md to specify `is_file()` instead of `exists()` (resolves DEFECT-P1-1)
2. Fix all three docstring issues (resolves DEFECT-P1-2, P1-3, P1-4)
3. Create decomposition task for `diff_directories` (resolves DEFECT-P2-1)
