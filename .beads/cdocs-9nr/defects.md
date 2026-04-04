# Black Hat Review: cdocs-9nr

## Bead: Wire startup state open and file diff into `run_index`
## Reviewer: Black Hat (Phase 5.5)
## Date: 2026-04-04

---

# PHASE 1: Contract & Bead Parity

### C-1: StateReadSession creation uses wrong API (MINOR)

**Contract says** (contract.md, line 189):
```rust
fn StateDb::begin_read(&self) -> Result<StateReadSession<'_>, CommitError>
```

**Implementation does** (index.rs, line 122):
```rust
let session = StateReadSession::new(state_db.database())
    .map_err(|e| anyhow::anyhow!("failed to begin state read session: {e}"))?;
```

`StateDb::begin_read()` exists at `commit.rs:705` and returns `Result<StateReadSession<'_>, CommitError>`, wrapping errors as `CommitError::ReadTransaction`. The implementation bypasses this and uses the lower-level `StateReadSession::new()` from `bulk_load.rs:245`, which returns `Result<Self, BulkLoadError>` with `BulkLoadError::StorageError`.

**Impact**: The contract's error taxonomy specifies `CommitError::ReadTransaction` for read transaction failures. The implementation produces `BulkLoadError::StorageError { table: "<begin_read>", message }` instead. Both are correctly converted to `anyhow::Error` at line 123, so behavior is correct. But the structured error path diverges from the contract.

**Recommendation**: Replace line 122-123 with:
```rust
let session = state_db.begin_read()
    .map_err(|e| anyhow::anyhow!("failed to begin state read session: {e}"))?;
```

---

### C-2: `file_diff` dropped inside scope block, violating POST-7 (MINOR)

**Contract says** (contract.md, POST-7, line 72):
> The `FileDiff` is available (in a variable) for downstream use by a future bead.

**Implementation** (index.rs, lines 118-145):
The entire STEP 1.5 is wrapped in a block scope `{ ... }`. The `file_diff` variable is created at line 130-136 and **dropped at line 145** when the scope ends. It is NOT available to downstream steps (STEP 2+).

The QA report acknowledges this: "`file_diff` variable available (in scope block, dropped at end) — PASS". This is a self-contradictory assessment. Either the variable is available or it is dropped. It cannot be both.

**Impact**: The next bead that needs to gate processing on diff status will need to restructure `run_index` to hoist `file_diff` out of the block scope. This is a latent cost, not a correctness bug.

**Recommendation**: Remove the block scope or hoist `file_diff` above the scope block:
```rust
let file_diff = {
    // ... STEP 1.5 ...
    file_diff  // returned from block
};
```

---

### C-3: `compute_config_hash` not called explicitly (OBSERVATION)

**Contract data flow** (contract.md, step 1e, line 221):
```
config_hash = compute_config_hash(config.category_config)
```

**Implementation**: `compute_config_hash` is NOT called as a standalone step in `run_index`. It is called internally by `compute_file_diff` (diff.rs). The QA report marks this as PASS: "PASS (via `compute_file_diff` internally)."

**Verdict**: The contract's data flow diagram is misleading, but the behavior is correct. `compute_config_hash` IS invoked with the correct argument — it's just delegated to `compute_file_diff`. This is a documentation issue, not a code defect.

---

# PHASE 2: Farley Engineering Rigor

### F-1: `run_index` is 207 lines (OBSERVATION — pre-existing)

Lines 76-283. Farley limit: 25 lines per function.

**Mitigation**: This function was 179 lines before this bead. The bead added 28 lines (STEP 1.5 block, lines 117-145). The function is a pipeline orchestrator with 8 sequential steps. Refactoring it into smaller functions is a separate effort outside this bead's scope.

---

### F-2: `file_states_to_stored_hashes` is 16 lines, 2 params (PASS)

Lines 37-52. Under 25-line limit. Under 5-parameter limit. Pure function, no I/O. `#[must_use]` attribute present.

---

### F-3: File is 525 lines total (OBSERVATION — pre-existing)

Architectural drift rule: <300 lines per file. The file is 525 lines (283 production + 242 test). The test module accounts for 46% of the file. Pre-existing structural issue.

---

### F-4: Tests assert behavior, not implementation (PASS)

All tests assert WHAT (key preservation, bitwise identity, partition correctness, error messages) not HOW (internal iterator mechanics). Integration tests exercise the full STEP 1.5 pipeline. Proptests verify invariants with random fuzzing.

---

# PHASE 3: NASA-Level Functional Rust (The Big 6)

### N-1: No illegal states possible (PASS)

The pure function `file_states_to_stored_hashes` has no conditional logic — it's a simple projection. No enum-based state machines in the new code. `FileDiff` is consumed as-is from the `diff` module.

---

### N-2: Parse, Don't Validate (PASS)

The conversion from `FileStateRaw` to `StoredHashes` at lines 40-51 is a pure field projection with zero validation. Data flows from the redb boundary (already parsed by `StateReadSession::load_file_states`) through the projection into the diff function.

---

### N-3: No boolean parameters (PASS)

No boolean parameters in the new code.

---

### N-4: `pub` visibility on `file_states_to_stored_hashes` (MINOR)

Line 37: `pub fn file_states_to_stored_hashes(...)`. This function is only called from `run_index` in the same module (line 127). No external callers found. Should be `pub(crate)` or `pub(super)` to minimize API surface.

---

# PHASE 4: Ruthless Simplicity & DDD (Scott Wlaschin)

### S-1: Zero panic vectors in production code (PASS)

`rg "unwrap\(\)|expect(" src/cmd/index.rs` returns exactly 1 match at line 520, inside `#[cfg(kani)]` block — never compiled in production.

---

### S-2: Zero `let mut` in production code (PASS)

All 14 `let mut` occurrences are in the `#[cfg(test)]` module (lines 322+). Production code (lines 1-283) has zero unnecessary mutability.

---

### S-3: No Option-based state machines (PASS)

The STEP 1.5 block has no Option-based branching for state transitions.

---

### S-4: Duplicated test helper (OBSERVATION)

`file_states_to_stored_hashes` is replicated at `tests/run_index_state_diff_tests.rs:43-58`. This is a structural artifact: `cmd/index.rs` is binary-only code, so integration tests cannot import from it. The duplication is identical logic, verified by both test suites passing.

---

# PHASE 5: The Bitter Truth (Velocity & Legibility)

### B-1: Code is boring and obvious (PASS)

The `file_states_to_stored_hashes` function is 12 lines of pure map/collect. The STEP 1.5 block reads top-to-bottom like a recipe. No cleverness, no abstractions, no generic handlers. A junior developer could understand this in 30 seconds.

---

### B-2: No YAGNI violations (PASS)

No code built for "future use". The `file_diff` variable would be YAGNI if the contract didn't explicitly require it to be available downstream (POST-7). The block scope that drops it is arguably correct for this bead's scope.

---

### B-3: Sniff test (PASS)

The code does not look like it was written by someone trying to prove how smart they are. It looks like it was written by someone who wanted to wire two existing modules together with minimal friction and maximal test coverage.

---

# Summary

| Phase | CRITICAL | MAJOR | MINOR | OBSERVATION |
|-------|----------|-------|-------|-------------|
| 1: Contract Parity | 0 | 0 | 2 | 1 |
| 2: Farley Rigor | 0 | 0 | 0 | 3 |
| 3: NASA Rust | 0 | 0 | 1 | 0 |
| 4: Simplicity/DDD | 0 | 0 | 0 | 1 |
| 5: Bitter Truth | 0 | 0 | 0 | 0 |
| **Total** | **0** | **0** | **3** | **5** |

### Defects (actionable)

| ID | Severity | Finding | Action |
|----|----------|---------|--------|
| C-1 | MINOR | `StateReadSession::new()` used instead of `StateDb::begin_read()` | Replace at line 122 |
| C-2 | MINOR | `file_diff` dropped in block scope, violating POST-7 | Hoist out of scope or return from block |
| N-4 | MINOR | `file_states_to_stored_hashes` is unnecessarily `pub` | Change to `pub(crate)` |

### Observations (non-blocking, pre-existing)

| ID | Finding |
|----|---------|
| C-3 | `compute_config_hash` delegated to `compute_file_diff` (correct but not per contract data flow diagram) |
| F-1 | `run_index` is 207 lines (pre-existing, not this bead) |
| F-3 | File is 525 lines (pre-existing, test module is 46% of file) |
| S-4 | Test helper duplicated in integration tests (structural artifact of binary-only `cmd` module) |
