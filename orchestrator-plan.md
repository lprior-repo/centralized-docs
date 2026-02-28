# Orchestrator Plan: doc-3o2b - CLI Error Message Format Inconsistency

## Bug Description

**Bead ID**: doc-3o2b  
**Title**: cli: Error message format inconsistency  
**Issue**: Error messages should consistently start with "Error:" prefix but some don't.

## Analysis Summary

After analyzing the codebase, the inconsistency exists in two forms:

1. **Direct `eprintln!` calls** - Some error paths use `eprintln!("Error: ...")` while others don't prefix errors at all
2. **Error vs Warning mixing** - Some places print "Warning:" for what are actually error conditions

### Files with Inconsistent Error Formatting

| File | Line | Current Format | Issue |
|------|------|-----------------|-------|
| `src/analyze.rs` | 115 | `eprintln!("Error: analysis failed: ...")` | Has "Error:" prefix ✓ |
| `src/transform.rs` | 109 | `eprintln!("Error: transform failed: ...")` | Has "Error:" prefix ✓ |
| `src/discover.rs` | 70 | `eprintln!("Warning: Skipping path due to I/O error: {e}")` | Uses "Warning" for error |
| `src/discover.rs` | 127 | `eprintln!("Warning: Skipping empty file ...")` | Uses "Warning" for error |
| `src/index.rs` | 430-431 | `eprintln!("Warning: Failed to build Tantivy index...")` | Uses "Warning" for error |
| `src/index.rs` | 831 | `eprintln!("Warning: HNSW index build failed...")` | Uses "Warning" for error |
| `src/filter.rs` | 650 | `eprintln!("Warning: Skipping path due to I/O error: {e}")` | Uses "Warning" for error |

---

## Atomic Execution Steps

### Phase 1: Identify and Document All Error Message Locations

**Step 1.1**: Search for all error/warning patterns in CLI-facing code

```bash
# Find all eprintln! statements in main CLI modules
grep -rn 'eprintln!' doc_transformer/src/*.rs
```

**Expected Output**: List of all error/warning output statements

**Checkpoint**: All eprintln! statements in CLI modules are cataloged

---

### Phase 2: Fix discover.rs Error Formatting

**Step 2.1**: Fix line 70 in `src/discover.rs`

- **Current**: `eprintln!("Warning: Skipping path due to I/O error: {e}");`
- **Should be**: `eprintln!("Error: Failed to read path: {e}");`
- **Rationale**: This is an error condition (file can't be read), not a warning

**Step 2.2**: Fix line 127 in `src/discover.rs`

- **Current**: `eprintln!("Warning: Skipping empty file {}", path.display());`
- **Should be**: `eprintln!("Error: Cannot index empty file: {}", path.display());`
- **Rationale**: Empty files cause indexing failure, this is an error

---

### Phase 3: Fix index.rs Error Formatting

**Step 3.1**: Fix lines 430-431 in `src/index.rs`

- **Current**: 
  ```rust
  eprintln!("Warning: Failed to build Tantivy index: {e}");
  eprintln!("Search will fall back to INDEX.json, but will be slower");
  ```
- **Should be**: 
  ```rust
  eprintln!("Error: Failed to build Tantivy index: {e}");
  eprintln!("Fallback: Search will use INDEX.json (slower)");
  ```
- **Rationale**: Index build failure is an error condition

**Step 3.2**: Fix line 831 in `src/index.rs`

- **Current**: `eprintln!("Warning: HNSW index build failed ({e}), skipping related chunk edges");`
- **Should be**: `eprintln!("Error: HNSW index build failed ({e}), skipping related chunk edges");`
- **Rationale**: HNSW build failure affects search quality

---

### Phase 4: Fix filter.rs Error Formatting

**Step 4.1**: Fix line 650 in `src/filter.rs`

- **Current**: `eprintln!("Warning: Skipping path due to I/O error: {e}");`
- **Should be**: `eprintln!("Error: Failed to read path: {e}");`
- **Rationale**: File read failure is an error

---

## Deterministic Checkpoints

### Checkpoint 1: Code Analysis Complete
- [ ] All eprintln! statements in CLI modules identified
- [ ] Error vs Warning distinction made for each location
- [ ] List of files requiring changes documented

### Checkpoint 2: discover.rs Fixed
- [ ] Line 70: Changed from "Warning" to "Error" prefix
- [ ] Line 127: Changed from "Warning" to "Error" prefix
- [ ] Code compiles without errors
- [ ] Test: Run index on a directory with permission issues

### Checkpoint 3: index.rs Fixed
- [ ] Lines 430-431: Changed from "Warning" to "Error" prefix
- [ ] Line 831: Changed from "Warning" to "Error" prefix
- [ ] Code compiles without errors
- [ ] Test: Run index with HNSW parameters that cause build failure

### Checkpoint 4: filter.rs Fixed
- [ ] Line 650: Changed from "Warning" to "Error" prefix
- [ ] Code compiles without errors
- [ ] Test: Run search with corrupted index file

### Checkpoint 5: Full Validation
- [ ] All error messages now consistently start with "Error:"
- [ ] All warning messages start with "Warning:"
- [ ] No error conditions are misclassified as warnings
- [ ] moon run :quick passes (fmt + clippy)
- [ ] moon run :test passes

---

## Rollback Criteria

If at any point the changes cause issues:

1. **Compilation Failure**: Run `cargo build` to identify the issue, then revert specific lines
2. **Test Failure**: Run `moon run :test` to identify failing tests, revert if tests fail
3. **Runtime Regression**: Document the specific command that failed and revert the associated change

**Rollback Command**:
```bash
git checkout -- doc_transformer/src/discover.rs doc_transformer/src/index.rs doc_transformer/src/filter.rs
```

---

## Evidence Artifacts

The following must be produced to verify the fix:

1. **Before/After Comparison Table**
   - Document each changed line with before/after code

2. **Test Results**
   - Output of `moon run :quick` showing fmt + clippy pass
   - Output of `moon run :test` showing all tests pass

3. **Error Message Verification**
   - Screenshot or terminal output showing each error type now has consistent "Error:" prefix

4. **Code Diff**
   - Output of `git diff` showing all changes

---

## Implementation Tasks

| Task | File | Lines | Effort |
|------|------|-------|--------|
| Fix discover.rs error formatting | src/discover.rs | 70, 127 | 15min |
| Fix index.rs error formatting | src/index.rs | 430-431, 831 | 15min |
| Fix filter.rs error formatting | src/filter.rs | 650 | 15min |
| Run quality gates | - | - | 5min |
| Verify all errors have "Error:" prefix | - | - | 10min |

**Total Estimated Effort**: 1hr
