# STATE: NEEDS IMPLEMENTATION

**Bead ID:** cdocs-9nr
**Title:** "action: wire startup state open and file diff into `run_index`"
**Status:** NOT IMPLEMENTED
**Date:** 2026-04-03

## Assessment Summary

| Component | Status | Notes |
|-----------|--------|-------|
| `run_index` opens StateDb | MISSING | No StateDb usage in `src/cmd/index.rs` |
| `run_index` creates StateReadSession | MISSING | No session creation |
| `run_index` loads file states | MISSING | No bulk load call |
| `run_index` computes file diff | MISSING | No `compute_file_diff` call |
| `run_index` prints diff telemetry | MISSING | No diff statistics output |
| `tests/run_index_state_diff_tests.rs` | MISSING | Test file does not exist |

## What Needs to Be Built

### 1. Add StateDb and diff imports to `src/cmd/index.rs`

```rust
use crate::state::commit::StateDb;
use crate::diff::compute_file_diff;
```

### 2. Insert between STEP 1 (DISCOVER) and STEP 2 (ANALYZE):

```rust
// STEP 1.5: STATE DIFF (before analysis)
println!("[STEP 1.5] STATE DIFF");
let state_db = StateDb::open(output.join(".state/state.db"))?;
let session = state_db.begin_read()?;
let stored_hashes = session.load_file_states()?;
let current_hashes = compute_file_diff(&files, &analysis_base_path)?;
let diff_result = diff::compute_change_detection(&stored_hashes, &current_hashes);
println!("  Unchanged: {}", diff_result.unchanged.len());
println!("  Changed: {}", diff_result.changed.len());
println!("  New: {}", diff_result.new.len());
println!("  Deleted: {}", diff_result.deleted.len());
drop(session);  // Release read transaction before analysis
println!();
```

### 3. Create test file `tests/run_index_state_diff_tests.rs` with:

- `test_run_index_opens_state_db_and_prints_diff_step_when_prior_state_exists`
- `test_run_index_reuses_one_read_session_for_startup_state_access`
- `test_run_index_failing_state_open_returns_error_before_analysis`
- `test_run_index_file_diff_failure_bubbles_before_pipeline_steps`

## Files to Modify

1. `/home/lewis/src/centralized-docs/centralized-docs/src/cmd/index.rs`
2. Create: `/home/lewis/src/centralized-docs/centralized-docs/tests/run_index_state_diff_tests.rs`

## Contract Reference

Per the bead contract, after implementation:
- `run_index` must open StateDb before analysis
- Create exactly one StateReadSession
- Load file states via `session.load_file_states()`
- Compute diff via `compute_file_diff`
- Print deterministic diff telemetry
- Fail before analysis if state open or diff fails
