# Contract Specification

bead_id: cdocs-13p
bead_title: MAJOR: ctd index exits 0 on corrupt state database
phase: state-1-contract
updated_at: 2026-04-04T12:00:00Z

## Context

- **Feature**: `ctd index` command exit code correctness on corrupt `state.redb`
- **Domain terms**:
  - `state.redb` — redb v2 persistent state database at `<output>/state.redb`
  - `StateDb` — newtype wrapper over `redb::Database` (in `src/state/commit.rs`)
  - `StateReadSession` — scoped read transaction for bulk loading (in `src/state/bulk_load.rs`)
  - `map_error_to_exit_code` — heuristic error-to-exit-code mapper (in `src/sys/error.rs`)
- **Assumptions**:
  - The bug is reproducible: `ctd index` with a corrupt `state.redb` exits 0
  - The error message IS printed to stderr (per bead description)
  - redb v2 `Database::create` on a corrupt file either returns an error OR silently overwrites with a fresh empty database
- **Open questions**:
  - Exact reproduction steps (what corruption pattern triggers the bug?)
  - Does `Database::create` silently overwrite a corrupt file, or does it error?
  - Is the error printed by `eprintln!` inside `run_index`, or by a panic hook?

## Current Behavior (BUG)

The `ctd index` command with a corrupt `state.redb` file:
1. Prints an error message to stderr
2. Exits with code 0 (success)

This causes automation scripts (CI/CD, Makefiles, wrapper scripts) to incorrectly
interpret the failed run as successful.

### Code Path Analysis

The error propagation chain in the current code:

```
index.rs:120  StateDb::open(&state_db_path)
  -> commit.rs:731  Database::create(path)  [redb v2]
  -> On error: map_err -> anyhow!("failed to open state database: {e}") -> ? propagates Err
  -> main.rs:303  map_error_to_exit_code(&err) -> contains "failed to open state database" -> exit 2
```

The code **appears** correct for the `StateDb::open` failure path. The `?` operator
propagates errors, and `map_error_to_exit_code` has `"failed to open state database"`
in `pipeline_error_patterns` (line 36 of `src/sys/error.rs`).

### Suspected Root Cause

The most likely root cause is one of:

1. **`Database::create` silently overwrites corrupt file**: redb v2's `Database::create`
   may detect the file is "not a valid redb database" and create a fresh empty database
   instead of returning an error. This would cause `StateDb::open` to succeed,
   `load_file_states` to return an empty map, and the pipeline to complete normally
   with exit code 0. The "error message" the bead describes may come from redb's
   internal logging (via the `log` crate) rather than from `eprintln!`.

2. **`Database::create` returns error but error string doesn't match patterns**:
   If redb returns an error whose string representation doesn't contain any of the
   patterns in `map_error_to_exit_code`, the function falls through to the default
   case (line 100: `return 2`), which would exit 2 — contradicting the bead.

3. **redb panics on internal corruption**: The `bulk_load.rs:1401` comment states
   "redb detects data corruption via internal page checksums and panics." A panic
   during table iteration would cause a non-zero exit (code 101 by default), but
   the stderr output might look like an error message. This doesn't match "exits 0"
   either.

**Primary hypothesis**: Root cause #1. `Database::create` silently recreates the
database on corruption, the pipeline runs to completion, and exit code is 0.

## Correct Behavior (POST-FIX)

The `ctd index` command with a corrupt `state.redb` file MUST:
1. Print a clear error message to stderr
2. Exit with code 2 (pipeline/internal error)

If `state.redb` does not exist, this is a first run — `Database::create` should
create a fresh database and the pipeline should proceed normally (exit 0).

## Preconditions

- **PRE-1**: `state.redb` file exists at `<output>/state.redb`
- **PRE-2**: `state.redb` contains corrupt data (not a valid redb database)
- **PRE-3**: Output directory is writable
- **PRE-4**: Source directory contains at least one markdown file

## Postconditions

- **POST-1**: On corrupt `state.redb`, the process exits with code 2
- **POST-2**: On corrupt `state.redb`, a human-readable error message is printed to stderr
- **POST-3**: On corrupt `state.redb`, the pipeline does NOT proceed to STEP 2+
- **POST-4**: On missing `state.redb` (first run), the pipeline creates a fresh DB and exits 0
- **POST-5**: On valid `state.redb`, the pipeline runs normally and exits 0

## Invariants

- **INV-1**: Exit code 0 means "pipeline completed successfully" — no errors
- **INV-2**: Exit code 1 means "user input error" — invalid arguments, missing files
- **INV-3**: Exit code 2 means "pipeline/internal error" — corruption, network, IO failures
- **INV-4**: `StateDb::open` on a corrupt file MUST return `Err`, never silently overwrite
- **INV-5**: Any `Err` returned from `run_index` MUST result in a non-zero exit code

## Error Taxonomy

| Error Variant | Source | Exit Code | Message Pattern |
|---|---|---|---|
| `CommitError::DatabaseOpen` | `StateDb::open` → `Database::create` | 2 | `"failed to open state database at {path}: {reason}"` |
| `CommitError::TableInit` | `StateDb::open` → `initialize_tables` | 2 | `"failed to initialize tables: {reason}"` |
| `BulkLoadError::StorageError` | `StateReadSession::new` → `begin_read` | 2 | `"failed to begin state read session: {message}"` |
| `StateLoadError::BackendError` | `load_file_states` → table iteration | 2 | `"failed to load file states: {message}"` |
| `StateLoadError::MalformedRow` | `load_file_states` → row decode | 2 | `"failed to load file states: malformed row ..."` |

## Fix Location

### Primary Fix: `src/state/commit.rs` — `StateDb::open()` (line 721-744)

**Change**: Replace `Database::create(path)` with `Database::open(path)`.

```rust
// BEFORE (line 731):
let db = Database::create(path).map_err(|e| CommitError::DatabaseOpen { ... })?;

// AFTER:
let db = Database::open(path).map_err(|e| CommitError::DatabaseOpen { ... })?;
```

**Rationale**: `Database::create` is designed for initial database creation. On an
existing corrupt file, it may silently overwrite with a fresh empty database.
`Database::open` is designed for opening existing databases and will return an
error if the file is not a valid redb database.

**Impact**: After this change, `ctd index` on a corrupt `state.redb` will:
1. `StateDb::open` returns `Err(CommitError::DatabaseOpen { ... })`
2. `index.rs:120-121` wraps it: `anyhow!("failed to open state database: {e}")`
3. `?` propagates `Err` to `main.rs`
4. `map_error_to_exit_code` matches `"failed to open state database"` → exit 2
5. `eprintln!("Error: {err}")` prints the error to stderr
6. `process::exit(2)` terminates with correct exit code

### Secondary Concern: First-run path

After switching to `Database::open`, a fresh output directory (no `state.redb`)
will fail because `Database::open` requires an existing database. The fix must
handle the first-run case:

**Option A** (Recommended): Try `Database::open` first, fall back to `Database::create`:

```rust
let db = Database::open(path)
    .or_else(|_| Database::create(path))
    .map_err(|e| CommitError::DatabaseOpen {
        path: path.display().to_string(),
        reason: e.to_string(),
    })?;
```

**Option B**: Check if file exists before choosing open vs create:

```rust
let db = if path.exists() {
    Database::open(path).map_err(|e| CommitError::DatabaseOpen { ... })?
} else {
    Database::create(path).map_err(|e| CommitError::DatabaseOpen { ... })?
};
```

Option A is preferred because it handles the edge case where `path.exists()` is
true but the file is empty (redb treats empty files as "create new").

### Tertiary Fix: `src/sys/error.rs` — `map_error_to_exit_code` (line 11-101)

Verify that all error strings from the `StateDb`/`StateReadSession`/`load_file_states`
chain match existing patterns in `map_error_to_exit_code`:

- `"failed to open state database"` — already in `pipeline_error_patterns` (line 36)
- `"failed to begin state read session"` — NOT in any pattern. Falls through to default `return 2` (line 100). **Correct but fragile.**
- `"failed to load file states"` — NOT in any pattern. Falls through to default `return 2`. **Correct but fragile.**

**Recommendation**: Add these patterns to `pipeline_error_patterns` for explicitness:

```rust
let pipeline_error_patterns = [
    // ... existing patterns ...
    "failed to begin state read session",
    "failed to load file states",
];
```

### Affected Files

| File | Change |
|---|---|
| `src/state/commit.rs` | `StateDb::open`: use `Database::open` with fallback to `Database::create` |
| `src/sys/error.rs` | Add missing state-related patterns to `pipeline_error_patterns` |
| `src/state/commit.rs` (tests) | Add test: `StateDb::open returns error on corrupt database` |
| `src/sys/error.rs` (tests) | Add tests: `map_error_to_exit_code` for state read/load errors |

## Non-goals

- This fix does NOT address redb panics on internal page corruption (that's a
  separate concern handled by redb's internal integrity checks)
- This fix does NOT change the `Database::create` usage in tests or other modules
- This fix does NOT add database repair/recovery logic
- This fix does NOT change the exit code semantics (0/1/2 mapping remains unchanged)
