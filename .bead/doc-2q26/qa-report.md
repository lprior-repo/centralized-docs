---
bead_id: doc-2q26
bead_title: quality: enforce clippy warning budget for release
phase: P5
updated_at: 2026-02-28T15:45:00Z
---

# QA Report

## Scope Executed

- Contract compliance against `.beads/doc-2q26/contract-spec.md`
- Protocol gates from `.beads/doc-2q26/evaluation-protocol.md`
- Runtime release-gate exit code behavior (0, 3, 4)
- Baseline file existence/integer validation

## Evidence (Executed Commands)

1) Required Moon gates

- Command: `moon run :fmt`
  - Exit: `0`
  - Output (key): `Tasks: 3 completed (3 cached)`
- Command: `moon run :clippy`
  - Exit: `0`
  - Output (key): includes warnings, e.g. `warning: 'release-gate' (bin "release-gate" test) generated 6 warnings` (see full log line 146 in `/home/lewis/.local/share/opencode/tool-output/tool_ca4cdcca20018a3G9V1uV0wNZW`)
- Command: `moon run :check`
  - Exit: `0`
  - Output (key): `Finished 'dev' profile ...` with warnings in workspace
- Command: `moon run :test`
  - Exit: `0`
  - Output (key): all suites completed, no failing tests
- Command: `moon run :build`
  - Exit: `0`
  - Output (key): release builds complete

2) Runtime release gate (normal)

- Command: `./target/release/release-gate`
  - Exit: `0`
  - Output (key): `✅ Moon CI passed`, `✅ Warning budget check PASSED - 0 warnings (baseline: 100)`, `✅ Release gate PASSED - all checks passed`
  - Log: `/home/lewis/.local/share/opencode/tool-output/tool_ca4ce2772001SxMTbNlFaepfrA`

3) Runtime release gate (budget exceeded -> exit 4)

- Setup used for deterministic repro:
  - Created temporary shim `PATH="$PWD/.qa-bin:$PATH"` that intercepts only `cargo clippy --workspace -- -D warnings` and returns one warning + exit 101
  - Baseline set to `0`
- Command: `PATH="$PWD/.qa-bin:$PATH" ./target/release/release-gate`
  - Exit: `4`
  - Output (key): `❌ Warning budget EXCEEDED - 1 warnings (baseline: 0, +1)` and `❌ Release gate FAILED - Warning budget exceeded release`
  - Log: `/home/lewis/.local/share/opencode/tool-output/tool_ca4cf21f50019qJ8V5hDJAYkNl`

4) Runtime release gate (invalid baseline -> exit 3)

- Setup: `.clippy-baseline` set to `not-a-number`
- Command: `./target/release/release-gate`
  - Exit: `3`
  - Output (key): `❌ Release gate BUDGET CHECK FAILED: invalid baseline content: 'not-a-number' - expected non-negative integer`
  - Log: `/home/lewis/.local/share/opencode/tool-output/tool_ca4cf8798001caG8L7qALq0rHB`

5) Baseline file validation

- Command: `python` check for file + integer parse
  - Exit: `0`
  - Output: `exists=true`, `first_line='8'`, `valid_integer=True`

6) Warning-budget focused tests

- Command: `moon run :test -- --package release-gate -- warning_budget`
  - Exit: `0`
  - Output (key): `running 28 tests` in `warning_budget::tests`, all `ok`

## Validation Verdicts

1. Contract Compliance: **FAIL**

- Expected: signatures and contract shape from `contract-spec.md`.
- Actual:
  - Contract specifies `load_baseline`, implementation exposes `read_baseline` (`release-gate/src/warning_budget.rs:148`).
  - Contract specifies `run_clippy_and_count_warnings`, implementation exposes `run_clippy_count` (`release-gate/src/warning_budget.rs:159`).
  - Contract specifies `execute_budget_check`, implementation has no such public function (grep found none).
  - Contract uses `is_passed()`, implementation exposes `passed()` (`release-gate/src/warning_budget.rs:115`).
- Passes: Domain types and error variants are present (`WarningCount`, `BudgetCheckResult`, `BudgetError` variants).

2. Test Coverage: **PASS (with concern)**

- Expected: happy path, error path, edge cases, determinism.
- Actual: `warning_budget` test module executes 28 tests covering parse success/fail, edge values, comparison logic, deterministic comparison loop.
- Determinism evidence: `test_invariant_deterministic_comparison` repeatedly asserts same outcome.
- Concern: clippy warns inside release-gate tests; quality signal is weaker than intended.

3. Runtime Behavior: **PASS**

- Exit code `0` verified for passing case.
- Exit code `4` verified for warning budget exceeded (baseline 0 + simulated warning > 0).
- Exit code `3` verified for invalid baseline precondition failure.

4. Baseline File: **PASS**

- `.clippy-baseline` exists at repo root.
- Content validated as integer (`8`) at time of final check.

## Issues Found

### MAJOR-1: Contract/API drift from agreed spec

- Expected: function names and API from contract.
- Actual: naming/signature mismatches (`load_baseline` vs `read_baseline`, missing `execute_budget_check`, `is_passed` vs `passed`).
- Repro:
  1. Open contract `.beads/doc-2q26/contract-spec.md`
  2. Compare to `release-gate/src/warning_budget.rs`

### MAJOR-2: `moon run :release-gate` task is broken in this workspace

- Command: `moon run :release-gate`
- Exit: overall `1` (task failure)
- Output: `❌ Release gate ERROR: Failed to open beads issues file: No such file or directory (os error 2)`
- Expected: run once from repo root.
- Actual: task executes per project, so cwd does not contain `.beads/issues.jsonl`.

### MAJOR-3: Clippy gate does not satisfy warning-budget policy intent

- Command: `moon run :clippy`
- Exit: `0`
- Actual output includes multiple warnings, including `release-gate` test warnings.
- Expected (protocol): no warning regressions / release-gate warning budget intent enforced.

### MINOR-1: CI fail-fast behavior in contract not fully enforced

- Contract states CI fail-fast before budget check.
- Implementation always computes budget after CI run and returns `ci_passed: bool` (`release-gate/src/gate.rs:63-68`), so exit-2 path in `main` is effectively harder to reach via typed error path.

## Final QA Gate

- Must-pass protocol commands executed: **YES**
- Exit code 4 behavior validated: **YES**
- Critical issues: **NONE**
- Merge readiness for this bead: **NO** (blocked by MAJOR-1/2/3)
