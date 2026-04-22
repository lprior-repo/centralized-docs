# Implementation Summary: cdocs-v0h

## Bead: feat(state): Add Database::compact() for state.redb garbage collection

### Problem
state.redb grows to 2.6MB for 12KB of docs (217x overhead) with no calls to `Database::compact()` anywhere.

### Solution

#### 1. Free function `compact_state_db(path)` in `commit.rs`
- **Why free function?** `Database::compact()` takes `&mut self`, but `StateDb` wraps `Database` with `&self` methods. Rather than introducing interior mutability (Mutex) or unsafe code, we open the database exclusively in a standalone function.
- **Signature**: `pub fn compact_state_db(path: &Path) -> Result<bool, CommitError>`
- Opens the database, calls `db.compact()`, returns `true` if compaction performed, `false` if already compact.
- Added `CommitError::CompactFailed { path, reason }` variant for compaction-specific errors.

#### 2. CLI subcommand `ctd compact <STATE_DB_PATH>`
- Added `Compact` variant to `Commands` enum in `cli/mod.rs`
- Created `cmd/compact.rs` with `run_compact()` handler
- Wires into `main.rs` dispatch — reports result to stderr
- Listed in `ctd --help` output

#### 3. Auto-compact warning after `commit_changes`
- Added `should_suggest_compaction(file_size, logical_data_size)` pure calculation
- Threshold: `COMPACTION_THRESHOLD_RATIO = 10.0` (warns when file >10x logical data size)
- `log_compaction_suggestion()` action function runs after every successful commit
- Reads file metadata + iterates all tables to compute logical size
- Logs `tracing::warn!` with actionable message suggesting `ctd compact <path>`
- Added `db_path: Option<PathBuf>` field to `StateDb` to support this check
- Silently ignores errors (advisory, not critical)
- Skipped entirely for in-memory databases

#### 4. Tests (6 new tests in `commit.rs` test module)
- `test_compact_on_empty_db_succeeds` — compact succeeds on empty DB
- `test_compact_on_fresh_db_is_noop` — compact succeeds + data survives
- `test_compact_after_deletes_preserves_remaining_data` — insert, delete, compact, verify data integrity
- `should_suggest_compaction_returns_true_when_ratio_exceeded` — pure function unit test
- `should_suggest_compaction_returns_false_when_ratio_ok` — pure function unit test
- `should_suggest_compaction_returns_false_for_zero_sizes` — edge cases
- `commit_error_compact_failed_display_contains_path_and_reason` — error variant display
- `state_db_db_path_returns_path_for_on_disk_database` — accessor test
- `state_db_db_path_returns_none_for_in_memory_database` — accessor test

### Constraint Adherence

| Constraint | Status |
|---|---|
| Data→Calc→Actions layering | `should_suggest_compaction` is pure; `compact_state_db` and `log_compaction_suggestion` are Actions |
| Zero mut in domain code | No `mut` in core logic; `compact_state_db` uses `mut db` at the I/O boundary only |
| Zero unwrap/panic/expect | None in domain code; all error paths return `Result` or silently handled |
| forbid(unsafe_code) | No unsafe code added |
| Expression-based | Used `let Ok(read_tx) = ... else { return }` per clippy |
| Clippy clean | All new code passes clippy pedantic (remaining warning is pre-existing in scrape/http.rs) |

### Files Changed

| File | Change |
|---|---|
| `centralized-docs/src/state/commit.rs` | Added `compact_state_db()`, `should_suggest_compaction()`, `log_compaction_suggestion()`, `COMPACTION_THRESHOLD_RATIO`, `CommitError::CompactFailed`, `StateDb.db_path` field, `StateDb.db_path()` accessor, 9 new tests |
| `centralized-docs/src/cli/mod.rs` | Added `Compact` variant to `Commands` enum |
| `centralized-docs/src/cmd/compact.rs` | New file: `run_compact()` handler |
| `centralized-docs/src/cmd/mod.rs` | Added `pub mod compact` |
| `centralized-docs/src/main.rs` | Added `Commands::Compact` dispatch arm |

### Verification

```
cargo test -p centralized-docs --lib  → 1250 passed, 0 failed
cargo clippy -p centralized-docs --release  → 0 new warnings
cargo build --release -p centralized-docs  → SUCCESS
```
