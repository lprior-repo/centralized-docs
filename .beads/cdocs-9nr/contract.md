# Contract Specification: Wire startup state open and file diff into `run_index`

## Context

- **Feature**: cdocs-9nr -- Modify `run_index` to open `StateDb`, create a `StateReadSession`,
  bulk-load file state, compute the current config hash, run `compute_file_diff`, and print
  diff statistics before the analysis step begins.
- **Domain terms**:
  - `StateDb` -- redb database wrapper providing two-transaction architecture (read then write).
  - `StateReadSession` -- scoped read transaction with `load_file_states()` method that returns
    `HashMap<String, FileStateRaw>`.
  - `FileStateRaw` -- 200-byte fixed Pod struct holding content/config/analysis/transform/chunk
    hashes plus timestamp for a single file.
  - `StoredHashes` -- lightweight struct from `diff` module pairing `content_hash` and `config_hash`
    for diff comparison.
  - `compute_config_hash` -- pure function returning `ContentHash` (SHA-256) of the category
    config file. Returns `content_hash(b"")` for `None` or unreadable files.
  - `compute_file_diff` -- pure function classifying discovered files into
    `Unchanged`/`Changed`/`New`/`Deleted` buckets by comparing on-disk hashes against stored state.
  - `FileDiff` (from `diff` module) -- four `HashSet<String>` buckets: unchanged, changed, new, deleted.
  - `IndexConfig` -- CLI configuration containing `category_config: Option<PathBuf>` and other params.
  - `DiscoveryFile` -- discovered file record with `source_path: String` and `size_bytes: u64`.
  - `DiscoverManifest` -- discovery result containing `source_dir: String`.
- **Assumptions**:
  - The state database path is derived from the output directory (e.g., `<output>/state.redb`).
  - First-run (no existing database) is a valid state: `StateDb::open` creates the DB,
    `load_file_states` returns an empty `HashMap`, and all files are classified as `New`.
  - The diff is printed as informational output; it does NOT gate or skip any downstream steps
    in this bead. The pipeline still processes all files (unchanged + changed + new).
  - `run_index` currently uses `anyhow::Result` as its error type. The new state/diff integration
    must convert domain errors (`CommitError`, `DiffError`, `StateLoadError`) into `anyhow::Error`
    via `.map_err()` or the `?` operator with appropriate `From` impls.
  - The `StateReadSession` must be dropped before any future write transaction. In this bead
    we only read; the write (commit) is deferred to a later bead.
  - `compute_file_diff` expects `&HashMap<String, StoredHashes>`, not `&HashMap<String, FileStateRaw>`.
    A conversion step is required.
- **Open questions**:
  1. **Q1: State DB path**: What is the exact path convention for the state database relative
     to `output`? Assumption: `<output>/state.redb`. This must be confirmed or pluggable.
  2. **Q2: Conversion from FileStateRaw to StoredHashes**: `compute_file_diff` expects
     `StoredHashes { content_hash, config_hash }`, but `load_file_states` returns
     `FileStateRaw` which has `content_hash` and `config_hash` fields. The conversion is
     straightforward field projection, but the contract must specify whether this is an
     inline closure or a named function.

## Preconditions

- **PRE-1**: `output` path has been validated by `validate_output_path(output)` (existing behavior).
- **PRE-2**: `source` path exists (existing check: `source.exists()`).
- **PRE-3**: `output` directory is writable (guaranteed by `acquire_output_lock`).
- **PRE-4**: `StateDb::open` is called with a valid, writable parent directory for the redb file.
  If the parent directory does not exist, `StateDb::open` creates it.
- **PRE-5**: `StateReadSession` is created from `StateDb::begin_read()` BEFORE any files are
  discovered (so the read transaction captures a consistent snapshot).
- **PRE-6**: `compute_file_diff` requires `source_dir` to exist (validated by `source_dir.canonicalize()`).
- **PRE-7**: `discovered_files` is non-empty before calling `compute_file_diff` (the existing
  empty-files bail in `run_index` fires first).

## Postconditions

- **POST-1**: `StateDb` is opened successfully at `<output>/state.redb` (or configured path).
- **POST-2**: Exactly one `StateReadSession` is created and held for the duration of the read phase.
- **POST-3**: All file-state rows are bulk-loaded into memory as `HashMap<String, FileStateRaw>`.
  On first run (empty DB), the map is empty.
- **POST-4**: The current config hash is computed via `compute_config_hash(config.category_config.as_deref())`.
- **POST-5**: `compute_file_diff` is called with the discovered files, source directory, config path,
  and the converted `HashMap<String, StoredHashes>`.
- **POST-6**: Diff statistics are printed to stdout in the format:
  ```
  [DIFF] Unchanged: N  Changed: M  New: K  Deleted: L
  ```
- **POST-7**: The `FileDiff` is available (in a variable) for downstream use by a future bead,
  even though this bead does not yet gate processing on it.
- **POST-8**: The `StateReadSession` remains alive (not dropped) until after all reads are done.
  (In this bead, the session is held until the function ends or the scope ends.)
- **POST-9**: No files are skipped or gated based on diff status. The full pipeline
  (analyze, assign, transform, chunk, validate, index) still processes all discovered files.
- **POST-10**: On error (state DB open failure, read failure, diff computation failure),
  the function returns `Err(anyhow::Error)` with a descriptive message. The `StateReadSession`
  and `StateDb` are dropped cleanly (RAII).

## Invariants

- **INV-1 (Session lifetime)**: The `StateReadSession` borrows from `StateDb`. The `StateDb`
  must outlive the session. Both are stack-allocated in `run_index` in the correct drop order.
- **INV-2 (Single read transaction)**: At most one `StateReadSession` exists at a time.
  Multiple calls to `begin_read()` are not allowed concurrently.
- **INV-3 (No writes in this bead)**: The `StateDb::commit_changes` method is NOT called
  in this bead. The state DB is opened for read-only purposes.
- **INV-4 (Hash conversion correctness)**: The conversion from `FileStateRaw` to `StoredHashes`
  must project `content_hash` and `config_hash` fields without mutation. The resulting
  `StoredHashes` values must be bitwise-identical to the corresponding `FileStateRaw` fields.
- **INV-5 (Deterministic diff)**: For the same set of discovered files, source directory contents,
  and stored state, `compute_file_diff` always produces the same `FileDiff` partition.
- **INV-6 (Diff partition completeness)**: The union of `unchanged + changed + new + deleted`
  equals the union of all discovered paths and all stored-hash paths. No file appears in
  multiple buckets.
- **INV-7 (Error propagation)**: Every fallible operation returns `Result<_, Error>`. No
  `unwrap()`, `expect()`, or `panic!()` in the new code paths. All errors propagate via `?`
  with contextual `.map_err()` where the source error type is not `anyhow::Error`.
- **INV-8 (First-run correctness)**: When the state DB has no `file_state` table entries,
  all discovered files are classified as `New` and `deleted` is empty.

## Error Taxonomy

The new code paths in `run_index` can fail at three integration points. Each maps to
`anyhow::Error` via contextual wrapping:

### 1. State Database Errors (from `CommitError`)

| Variant | When | Recovery |
|---------|------|----------|
| `CommitError::DatabaseOpen { path, reason }` | redb cannot create/open the file | Fatal: return Err |
| `CommitError::TableInit { reason }` | redb table creation fails | Fatal: return Err |
| `CommitError::ReadTransaction { reason }` | redb cannot begin a read transaction | Fatal: return Err |

### 2. Bulk Load Errors (from `StateLoadError`)

| Variant | When | Recovery |
|---------|------|----------|
| `StateLoadError::MalformedRow { key, actual, expected }` | A row in `file_state` has wrong byte count | Fatal: return Err |
| `StateLoadError::Utf8KeyError { bytes_lossy }` | Non-UTF-8 key in state table | Fatal: return Err |
| `StateLoadError::BackendError { operation, message }` | redb storage failure | Fatal: return Err |

### 3. Diff Computation Errors (from `DiffError`)

| Variant | When | Recovery |
|---------|------|----------|
| `DiffError::SourceDirNotFound(path)` | Source dir does not exist | Fatal: return Err |
| `DiffError::FileRead { path, source }` | A discovered file cannot be read | Fatal: return Err |
| `DiffError::PathTraversal { path }` | A path escapes source directory | Fatal: return Err |

### Error Conversion Strategy

All three error types (`CommitError`, `StateLoadError`, `DiffError`) must be convertible
to `anyhow::Error`. Options:

- **Option A (Recommended)**: Add `impl From<CommitError> for anyhow::Error` and
  `impl From<StateLoadError> for anyhow::Error` and `impl From<DiffError> for anyhow::Error`
  using `.map_err()` at call sites (explicit, no orphan issues since `anyhow::Error` is foreign).
- **Option B**: Use `.context("...")` at each call site.

The contract mandates **no `.unwrap()` / `.expect()` / `panic!()`** anywhere in the new code.

## Contract Signatures

### Modified function signature (unchanged, but internal flow changes)

```rust
/// Run the index command (main pipeline) with state-aware diff computation.
///
/// New internal flow (inserted between STEP 1 DISCOVER and STEP 2 ANALYZE):
///   1a. Open StateDb at `<output>/state.redb`
///   1b. Begin StateReadSession
///   1c. Bulk load file states: session.load_file_states()
///   1d. Convert HashMap<String, FileStateRaw> to HashMap<String, StoredHashes>
///   1e. Compute config hash: compute_config_hash(config.category_config.as_deref())
///   1f. Compute file diff: compute_file_diff(&files, source_dir, config_path, &stored_hashes)
///   1g. Print diff statistics
///
/// The pipeline continues unchanged: all files are still analyzed, transformed, etc.
/// The diff is informational only in this bead.
pub fn run_index(source: &Path, output: &Path, config: &IndexConfig) -> Result<()>
```

### New internal helper (conversion)

```rust
/// Convert loaded file state rows to the StoredHashes format expected by compute_file_diff.
///
/// Pure function: projects content_hash and config_hash from each FileStateRaw.
/// The resulting map has exactly the same keys as the input map.
///
/// # Invariants
///
/// - INV-4: StoredHashes.content_hash == FileStateRaw.content_hash (bitwise identical)
/// - INV-4: StoredHashes.config_hash == FileStateRaw.config_hash (bitwise identical)
/// - Output map len() == input map len()
/// - Output map keys == input map keys (byte-identical Strings)
fn file_states_to_stored_hashes(
    file_states: &HashMap<String, FileStateRaw>,
) -> HashMap<String, StoredHashes>
```

### External function calls (existing, called in new order)

```rust
// State database operations (from state::commit)
fn StateDb::open(path: &Path) -> Result<StateDb, CommitError>
fn StateDb::begin_read(&self) -> Result<StateReadSession<'_>, CommitError>

// Bulk load (from state::bulk_load, method on StateReadSession)
fn StateReadSession::load_file_states(&self) -> Result<HashMap<String, FileStateRaw>, StateLoadError>

// Diff computation (from diff module)
fn compute_config_hash(category_config_path: Option<&Path>) -> ContentHash  // infallible
fn compute_file_diff(
    discovered_files: &[DiscoveryFile],
    source_dir: &Path,
    category_config_path: Option<&Path>,
    stored_hashes: &HashMap<String, StoredHashes>,
) -> Result<FileDiff, DiffError>
```

## Data Flow (New Steps in run_index)

```
After STEP 1 (DISCOVER):
  files: Vec<DiscoveryFile>       -- discovered on disk
  discover_manifest.source_dir    -- canonical source path

  ┌──────────────────────────────────────────────────────────────┐
  │ STEP 1.5: STATE + DIFF (NEW)                                 │
  │                                                              │
  │  state_db_path = output.join("state.redb")                   │
  │  state_db = StateDb::open(&state_db_path)?                   │
  │  session = state_db.begin_read()?                            │
  │  file_states = session.load_file_states()?                   │
  │  stored_hashes = file_states_to_stored_hashes(&file_states)  │
  │  config_hash = compute_config_hash(config.category_config)   │
  │  file_diff = compute_file_diff(                              │
  │      &files,                                                 │
  │      &PathBuf::from(&discover_manifest.source_dir),          │
  │      config.category_config.as_deref(),                      │
  │      &stored_hashes,                                         │
  │  )?                                                          │
  │                                                              │
  │  println!(                                                   │
  │      "[DIFF] Unchanged: {}  Changed: {}  New: {}  Deleted: {}", │
  │      file_diff.unchanged.len(),                              │
  │      file_diff.changed.len(),                                │
  │      file_diff.new.len(),                                    │
  │      file_diff.deleted.len(),                                │
  │  );                                                          │
  └──────────────────────────────────────────────────────────────┘

Continue to STEP 2 (ANALYZE) -- unchanged, processes ALL files.
```

## Non-goals

- Do NOT gate or skip pipeline steps based on diff status. All files are still fully processed.
- Do NOT call `StateDb::commit_changes`. Write-back is a separate bead.
- Do NOT modify the `StateDb`, `StateReadSession`, `compute_file_diff`, or `compute_config_hash`
  APIs. They are consumed as-is.
- Do NOT change the return type of `run_index` (remains `anyhow::Result<()>`).
- Do NOT add new error types. Existing domain errors (`CommitError`, `StateLoadError`, `DiffError`)
  are converted to `anyhow::Error` at the integration boundary.
- Do NOT introduce async. `run_index` remains synchronous.
- Do NOT modify the output format or file structure of the index pipeline.
