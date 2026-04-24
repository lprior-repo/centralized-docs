# ARCH-DRIFT: Batch 4 Findings

**Date**: 2026-04-24
**Status**: REFACTORED (1 file split)

## 1. Line Count Violations (>300 lines, src/ non-test)

### Refactored

| File | Before | After | Action |
|------|--------|-------|--------|
| `centralized-docs/src/sys/error.rs` | 340 | 103 | Split tests to `sys/error_tests.rs` (208 lines) |

### Borderline (noted, no action)

| File | Lines | Notes |
|------|-------|-------|
| `centralized-docs/src/cmd/index.rs` | 306 | Production code is 298 lines; 6 lines are `#[cfg(test)]` path attr. Tests already extracted to `index_tests.rs`. |
| `centralized-docs-pod/src/tests.rs` | 1003 | Test-only file already split from lib.rs. Not production code. |

### Clean (<300 lines, near boundary)

- `diff.rs`: 300 (exact boundary)
- `graph/dag.rs`: 298
- `cmd/watch.rs`: 298
- `persisted/analysis.rs`: 299

## 2. DDD Audit (Scott Wlaschin)

### Summary

| Category | High | Medium | Low | Total |
|----------|------|--------|-----|-------|
| Primitive obsession | 7 | 16 | 4 | 27 |
| Parse-don't-validate | 2 | 7 | 2 | 11 |
| State machine | 0 | 1 | 4 | 5 |
| **Total** | **9** | **24** | **10** | **43** |

### Key Pattern: Newtypes Exist But Are Not Used

The codebase has well-designed newtypes in `src/types/` (`ProjectName`, `Title`, `Category`, `FilePath`, `Slug`, `DocumentId`, `ChunkId`, `MaxRelatedChunks`, `HnswM`, `HnswEfConstruction`, `ConnectTimeoutSecs`, `RequestTimeoutSecs`, `ContentHash`), but the central domain objects overwhelmingly use raw primitives:

**HIGH severity - Newtypes available but unused:**
- `Analysis` (central domain object): uses raw `String` for `source_path`, `title`, `category` instead of `FilePath`, `Title`, `Category`
- `IndexConfig`: raw `String`/`usize` instead of `ProjectName`, `MaxRelatedChunks`, `HnswM`
- `ScrapeCommandConfig`: raw `u64`/`f32` instead of existing `ConnectTimeoutSecs`, `RequestTimeoutSecs`
- `ScrapeConfig`: defines `ConnectTimeoutSecs`/`RequestTimeoutSecs` newtypes in the same file but uses raw `u64` for its own fields
- `CategoryConfig`: raw `String` for `default_category` despite `Category` newtype existing with validation

**HIGH severity - Parse-don't-validate:**
- `cli/validation.rs`: All validators return raw primitives (`f32`, `u64`, `u32`, `usize`) instead of newtypes, losing type safety after clap boundary
- `CategoryConfig::load_from_file`: Validates then constructs with same raw `String` fields

### No Production unwrap/expect Panics

The codebase uses `#![deny(clippy::unwrap_used)]` and `#![deny(clippy::expect_used)]` in the `state/` module. All production unwrap instances found are `unwrap_or`, `unwrap_or_default`, or `unwrap_or_else` (safe defaults).

### State Machine Concerns: Minimal

No complex state machines found. `DiffStatus` classification is the closest pattern.

## 3. Recommended Priority for Future Batches

1. **Adopt newtypes in central domain objects** (`Analysis`, `ScrapeConfig`, `IndexConfig`) — highest impact, eliminates most validation duplication
2. **Return newtypes from CLI validators** instead of raw primitives
3. **MCP layer**: Store `ValidQuery`/`ValidLimit`/`ValidId` directly in param structs instead of unwrapping
4. **Consider `EdgeWeight(f32)` and `HeadingLevel(u8)` newtypes** for bounded numeric invariants
