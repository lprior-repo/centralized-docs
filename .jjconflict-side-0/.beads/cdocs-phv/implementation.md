---
bead_id: cdocs-phv
bead_title: "action: commit index state once at shutdown and assert transaction invariants"
phase: implementation
status: complete
updated_at: 2026-04-02
---

# Implementation Summary: cdocs-phv

## Changed Files

| File | Change |
|------|--------|
| `centralized-docs/src/state.rs` | Replaced all `todo!()` method stubs with full implementations |

## Implemented Types

All types were pre-defined by the test-writer. Implementation added behavior:

- **`RunId`** — Newtype wrapper with `Display`, unique via monotonic counter + wall-clock timestamp
- **`StateBatch`** — Serializable accumulator for pipeline mutations
- **`FileHashRecord`** — Path/hash pair for incremental rebuild detection
- **`StateError`** — 8-variant error taxonomy (`thiserror`-derived, `Clone`, `PartialEq`)
- **`StateDb`** — State machine with `{Uncommitted} -> {Committed}` lifecycle

## Implemented Methods

### `StateDb::new(output_dir: &Path) -> Result<Self, StateError>`
- Validates: empty path, non-directory, non-writable directory
- Generates unique `RunId` via `AtomicU64` counter + `SystemTime`
- Initializes empty `StateBatch` with `committed = false`
- **Contract clauses**: P-01, P-02, POST-02

### `StateDb::record_file_hash(&mut self, path, hash) -> Result<(), StateError>`
- Guards: `ensure_uncommitted()` → `MutationAfterCommit` if committed
- Validates: duplicate path detection via iterator linear scan → `DuplicateFilePath`
- Appends `FileHashRecord` to batch
- **Contract clauses**: INV-02, INV-04

### `StateDb::set_document_count(&mut self, count) -> Result<(), StateError>`
- Guards: `ensure_uncommitted()` → `MutationAfterCommit` if committed
- Last-write-wins semantics (no accumulation)
- **Contract clauses**: INV-02

### `StateDb::set_chunk_count(&mut self, count) -> Result<(), StateError>`
- Same guard and last-write-wins pattern as `set_document_count`
- Accepts `count == 0` (EmptyBatch fires at commit time)
- **Contract clauses**: INV-02

### `StateDb::commit_changes(&mut self) -> Result<(), StateError>`
- **INV-01**: Rejects double-commit with `AlreadyCommitted`
- **POST-04**: Rejects empty batch (`document_count == 0`) with `EmptyBatch`
- **Calculation layer**: `serde_json::to_string_pretty` → `SerializationFailed` on error
- **Action layer**: Atomic write via temp-file (`state-batch.json.tmp`) + `fs::rename` → `PersistenceFailed` on I/O error
- Sets `self.committed = true` only after successful write
- **Contract clauses**: INV-01, POST-01, POST-03

### `StateDb::is_committed(&self) -> bool`
- Pure query, returns `self.committed`

### `StateDb::Drop`
- Intentionally does NOT call `commit_changes` (INV-03)
- No filesystem side effects on drop

### Private Helpers
- `ensure_uncommitted()` — Mutation guard, returns `MutationAfterCommit`
- `effective_run_id()` — Extracts `RunId` from batch with fallback sentinel

## Constraint Adherence

| Constraint | Evidence |
|------------|----------|
| Data -> Calc -> Actions | Serialization is pure calculation; `fs::write`/`fs::rename` isolated in commit action |
| Zero unwrap/expect/panic | All branches use `match` or `map_err`; zero `unwrap*` in non-test code |
| Zero mut (minimised) | `&mut self` required by contract signatures; no `let mut` local variables |
| Make illegal states unrepresentable | `committed: bool` state machine; `Option<StateBatch>` with explicit `None` handling |
| Expression-based | Early returns with `?`; `match` expressions throughout |
| Clippy flawless | `cargo clippy --lib` passes with `-D warnings -D clippy::unwrap_used -W clippy::pedantic` |

## Test Results

```
cargo test --lib -- state
32 passed; 0 failed; 0 ignored

24 unit tests:    ALL PASS
8 proptest cases: ALL PASS
```

## Known Gap: B08 Integration Test

Integration test `state_db_new_returns_precondition_violation_when_output_lock_not_held` expects `StateDb::new` to verify the `.ctd.lock` file exists. However, 12 of 13 integration tests (B01, B17, B20, B21, B24, B26-simulated, B27-simulated, B28, B29, drop-after-commit, etc.) create `StateDb` without a lock file and expect success.

These tests are contradictory — enforcing the lock check makes B01/B17/etc fail; skipping it makes B08 fail. Since all other tests expect success without the lock, the implementation does NOT enforce the lock check. B08 requires a test fix to create a `.ctd.lock` file when testing the lock guard path.

The integration tests also cannot compile currently due to pre-existing errors in the binary targets (`ctd`, `ctd-mcp`), which is noted in the task context as a known issue.

## On-Disk Format

`commit_changes` persists `StateBatch` as pretty-printed JSON to `state-batch.json` in the output directory, using an atomic write (temp file + rename) for crash safety.
