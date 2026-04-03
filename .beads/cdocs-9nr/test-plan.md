# Test Plan: cdocs-9nr — Wire startup state open and file diff into `run_index`

## Summary

- **Behaviors identified**: 30
- **Trophy allocation**: 10 unit / 18 integration / 2 e2e
- **Proptest invariants**: 4
- **Fuzz targets**: 0 (no new parsers/deserializers in this bead)
- **Kani harnesses**: 2
- **Mutation kill target**: ≥90%

---

## 1. Behavior Inventory

### Pure Functions (Calc Layer)

| # | Behavior | Subject |
|---|----------|---------|
| B1 | `file_states_to_stored_hashes` returns map with identical keys when given non-empty input | conversion |
| B2 | `file_states_to_stored_hashes` returns empty map when given empty input | conversion |
| B3 | `file_states_to_stored_hashes` projects bitwise-identical `content_hash` and `config_hash` from each `FileStateRaw` | conversion |
| B4 | `compute_config_hash` returns `content_hash(b"")` when passed `None` | diff |
| B5 | `compute_config_hash` returns SHA-256 of file bytes when file is readable | diff |
| B6 | `compute_config_hash` returns `content_hash(b"")` when file is missing or unreadable | diff |
| B7 | `compute_config_hash` is deterministic: same input always produces same hash | diff |

### Integration Behaviors (run_index wiring)

| # | Behavior | Subject |
|---|----------|---------|
| B8 | `run_index` opens `StateDb` at `<output>/state.redb` when output is valid | run_index |
| B9 | `run_index` creates `StateReadSession` from `StateDb::begin_read()` successfully | run_index |
| B10 | `run_index` bulk-loads file states via `session.load_file_states()` | run_index |
| B11 | `run_index` receives empty `HashMap` from `load_file_states` on first run (empty DB) | run_index |
| B12 | `run_index` converts `HashMap<String, FileStateRaw>` to `HashMap<String, StoredHashes>` via `file_states_to_stored_hashes` | run_index |
| B13 | `run_index` computes config hash via `compute_config_hash(config.category_config.as_deref())` | run_index |
| B14 | `run_index` calls `compute_file_diff` with discovered files, source dir, config path, and stored hashes | run_index |
| B15 | `run_index` prints diff statistics in format `[DIFF] Unchanged: N  Changed: M  New: K  Deleted: L` | run_index |
| B16 | `run_index` classifies all files as `New` and `deleted` as empty on first run | run_index |
| B17 | `run_index` continues to STEP 2 ANALYZE with all discovered files (no gating) after diff | run_index |
| B18 | `run_index` returns `Err(anyhow)` wrapping `CommitError::DatabaseOpen` when state DB cannot be opened | run_index |
| B19 | `run_index` returns `Err(anyhow)` wrapping `CommitError::ReadTransaction` when begin_read fails | run_index |
| B20 | `run_index` returns `Err(anyhow)` wrapping `CommitError::TableInit` when table initialization fails | run_index |
| B21 | `run_index` returns `Err(anyhow)` wrapping `StateLoadError::MalformedRow` when load_file_states encounters a corrupted row | run_index |
| B22 | `run_index` returns `Err(anyhow)` wrapping `StateLoadError::Utf8KeyError` when a non-UTF-8 key is encountered in state table | run_index |
| B23 | `run_index` returns `Err(anyhow)` wrapping `DiffError::SourceDirNotFound` when source directory does not exist during diff | run_index |
| B24 | `run_index` returns `Err(anyhow)` wrapping `DiffError::FileRead` when a discovered file cannot be read during diff | run_index |
| B25 | `run_index` returns `Err(anyhow)` wrapping `DiffError::PathTraversal` when path traversal is detected during diff | run_index |
| B26 | `run_index` drops `StateReadSession` cleanly on early return (RAII) | run_index |
| B27 | `run_index` does NOT call `commit_changes` (read-only in this bead) | run_index |
| B28 | `run_index` produces correct diff with mixed unchanged/changed/new/deleted files on second run | run_index |

### E2E Behaviors (Full Pipeline)

| # | Behavior | Subject |
|---|----------|---------|
| B29 | Full pipeline end-to-end: `run_index` completes successfully with state diff output for a multi-file project | e2e |
| B30 | Full pipeline end-to-end: `run_index` completes on second run and shows unchanged/changed/deleted correctly | e2e |

---

## 2. Trophy Allocation

| Behavior | Layer | Justification |
|----------|-------|---------------|
| B1 | **Unit** | Pure function, no I/O. Exhaustive combinatorial. |
| B2 | **Unit** | Pure function, empty-input edge case. |
| B3 | **Unit** | Pure function, bitwise field projection assertion. |
| B4 | **Unit** | Pure function, `None` → empty hash. Already tested in diff.rs. |
| B5 | **Unit** | Pure function, reads one file. Already tested in diff.rs. |
| B6 | **Unit** | Pure function, missing file edge case. Already tested in diff.rs. |
| B7 | **Unit** | Pure function, determinism. Already tested in diff.rs. |
| B8 | **Integration** | Opens real redb file on disk. Tests `StateDb::open` with real filesystem. |
| B9 | **Integration** | Opens real read transaction on real database. |
| B10 | **Integration** | Reads from real redb table, verifies bulk-load output. |
| B11 | **Integration** | First-run scenario with empty database (critical business rule). |
| B12 | **Integration** | Tests conversion step embedded in `run_index` with real loaded state. |
| B13 | **Integration** | Tests config hash computation with real config file. |
| B14 | **Integration** | Tests `compute_file_diff` with real files on disk and real stored hashes. |
| B15 | **Integration** | Captures stdout, verifies diff output format. |
| B16 | **Integration** | First-run classification correctness (all New, empty Deleted). |
| B17 | **Integration** | Verifies pipeline continues: analysis runs on all files after diff. |
| B18 | **Integration** | Error propagation: `CommitError::DatabaseOpen` → `anyhow::Error`. |
| B19 | **Integration** | Error propagation: `CommitError::ReadTransaction` → `anyhow::Error`. |
| B20 | **Integration** | Error propagation: `CommitError::TableInit` → `anyhow::Error`. |
| B21 | **Integration** | Error propagation: `StateLoadError::MalformedRow` → `anyhow::Error`. |
| B22 | **Integration** | Error propagation: `StateLoadError::Utf8KeyError` → `anyhow::Error`. |
| B23 | **Integration** | Error propagation: `DiffError::SourceDirNotFound` → `anyhow::Error`. |
| B24 | **Integration** | Error propagation: `DiffError::FileRead` → `anyhow::Error`. |
| B25 | **Integration** | Error propagation: `DiffError::PathTraversal` → `anyhow::Error`. |
| B26 | **Integration** | RAII drop verification — session is cleaned up on error path. |
| B27 | **Integration** | Verify no write transaction is opened (read-only). |
| B28 | **Integration** | Second-run mixed classification with pre-populated state DB. |
| B29 | **E2E** | Full pipeline with real CLI invocation or close-to-CLI entry point. |
| B30 | **E2E** | Incremental re-index with state preserved between runs. |

**Ratio**: 10 unit (33%) / 18 integration (60%) / 2 e2e (7%) — close to target 30/60/5/5.

---

## 3. BDD Scenarios

### Behavior B1: file_states_to_stored_hashes returns map with identical keys

```
fn file_states_to_stored_hashes_returns_map_with_identical_keys_when_input_nonempty()

Given:  a HashMap<String, FileStateRaw> with 3 entries: ("a.rs", state_a), ("b.md", state_b), ("c.txt", state_c)
When:   file_states_to_stored_hashes(&file_states) is called
Then:   result.len() == 3
And:    result contains keys "a.rs", "b.md", "c.txt"
```

### Behavior B2: file_states_to_stored_hashes returns empty map

```
fn file_states_to_stored_hashes_returns_empty_map_when_input_empty()

Given:  an empty HashMap<String, FileStateRaw>
When:   file_states_to_stored_hashes(&empty_map) is called
Then:   result.len() == 0
And:    result == HashMap::new()
```

### Behavior B3: file_states_to_stored_hashes projects bitwise-identical hashes

```
fn file_states_to_stored_hashes_projects_bitwise_identical_content_and_config_hashes()

Given:  a HashMap with one entry where FileStateRaw has content_hash=[0xAA; 32] and config_hash=[0xBB; 32]
When:   file_states_to_stored_hashes(&file_states) is called
Then:   result["key"].content_hash == [0xAA; 32]
And:    result["key"].config_hash == [0xBB; 32]
```

### Behavior B4: compute_config_hash returns empty hash for None

```
fn compute_config_hash_returns_empty_hash_when_none()

Given:  category_config_path is None
When:   compute_config_hash(None) is called
Then:   result == content_hash(b"")
```

> NOTE: Already covered by existing test `compute_config_hash_returns_empty_hash_when_none` in diff.rs.

### Behavior B5: compute_config_hash returns SHA-256 of file bytes

```
fn compute_config_hash_returns_sha256_when_file_readable()

Given:  a temp file containing b"hello world" at path P
When:   compute_config_hash(Some(&P)) is called
Then:   result == content_hash(b"hello world")
```

> NOTE: Already covered by existing test in diff.rs.

### Behavior B6: compute_config_hash returns empty hash for missing file

```
fn compute_config_hash_returns_empty_hash_when_file_missing()

Given:  a path to a nonexistent file
When:   compute_config_hash(Some(&missing_path)) is called
Then:   result == content_hash(b"")
```

> NOTE: Already covered by existing test in diff.rs.

### Behavior B7: compute_config_hash is deterministic

```
fn compute_config_hash_returns_identical_hash_across_calls()

Given:  a temp file with deterministic content
When:   compute_config_hash is called twice with the same path
Then:   hash1 == hash2
```

> NOTE: Already covered by existing test in diff.rs.

### Behavior B8: run_index opens StateDb at output/state.redb

```
fn run_index_opens_state_db_at_output_state_redb_when_output_valid()

Given:  a valid source directory with at least one .md file
And:    a valid writable output directory
When:   run_index(source, output, config) is called
Then:   the file <output>/state.redb exists on disk after the call
And:    run_index returns Ok(())
```

### Behavior B9: run_index creates StateReadSession

```
fn run_index_creates_state_read_session_when_db_open()

Given:  a valid source directory with one .md file
And:    a valid output directory
When:   run_index(source, output, config) is called
Then:   run_index returns Ok(())
And:    the state.redb file has initialized tables (verifiable by opening separately via StateDb::open)
```

### Behavior B10: run_index bulk-loads file states

**Setup helper**: `seed_file_state_rows(db: &Database, rows: &[(&str, FileStateRaw)])` — writes
known `FileStateRaw` entries to the `file_state` table via a redb write transaction.

```
fn run_index_bulk_loads_file_states_from_session()

Given:  a valid source directory with one .md file
And:    an output directory where state.redb already has 2 file_state rows
        (seeded via seed_file_state_rows)
When:   run_index(source, output, config) is called
Then:   run_index returns Ok(())
And:    stdout output includes "[DIFF]"
```

### Behavior B11: run_index receives empty HashMap on first run

```
fn run_index_receives_empty_hashmap_on_first_run_when_db_empty()

Given:  a valid source directory with 3 .md files ("intro.md", "guide.md", "api.md")
And:    a fresh output directory (no existing state.redb)
When:   run_index(source, output, config) is called
Then:   stdout output includes "[DIFF] Unchanged: 0  Changed: 0  New: 3  Deleted: 0"
And:    run_index returns Ok(())
```

### Behavior B12: run_index converts FileStateRaw to StoredHashes

**Setup helper**: `seed_matching_file_state_rows(db: &Database, source_dir: &Path, files: &[&str])`
— writes `FileStateRaw` entries whose `content_hash` and `config_hash` match the SHA-256 of the
actual file contents on disk (computed via `content_hash` + `compute_config_hash`).

```
fn run_index_converts_file_state_raw_to_stored_hashes_correctly()

Given:  a source directory with 2 .md files: "unchanged1.md" and "unchanged2.md"
And:    an output directory with state.redb containing file_state entries for both files
        where content_hash matches their actual on-disk content
        (seeded via seed_matching_file_state_rows)
When:   run_index(source, output, config) is called
Then:   stdout output includes "[DIFF] Unchanged: 2  Changed: 0  New: 0  Deleted: 0"
And:    run_index returns Ok(())
```

### Behavior B13: run_index computes config hash

```
fn run_index_computes_config_hash_and_classifies_unchanged_when_config_same()

Given:  a source directory with 2 .md files
And:    a valid category_config.yaml at path C containing b"categories:\n  - ref"
And:    a first run of run_index completes successfully with config.category_config = Some(C)
When:   run_index(source, output, config) is called a second time with the same config
Then:   stdout includes "[DIFF] Unchanged: 2  Changed: 0  New: 0  Deleted: 0"
And:    run_index returns Ok(())
```

### Behavior B14: run_index calls compute_file_diff with correct arguments

```
fn run_index_calls_compute_file_diff_with_discovered_files_and_source_dir()

Given:  a source directory with files "a.md" and "b.md"
And:    a fresh output directory
When:   run_index(source, output, config) is called
Then:   stdout includes "[DIFF] Unchanged: 0  Changed: 0  New: 2  Deleted: 0"
And:    stdout includes "Found 2 files"
```

### Behavior B15: run_index prints diff statistics in correct format

```
fn run_index_prints_diff_statistics_in_expected_format()

Given:  a source directory with 2 .md files and a fresh output directory
When:   run_index(source, output, config) is called
Then:   stdout contains a line matching the regex: \[DIFF\] Unchanged: \d+  Changed: \d+  New: \d+  Deleted: \d+
```

### Behavior B16: run_index classifies all files as New on first run

```
fn run_index_classifies_all_files_as_new_and_empty_deleted_on_first_run()

Given:  a source directory with files "intro.md" and "guide.md"
And:    a fresh output directory (state.redb will be created empty)
When:   run_index(source, output, config) is called
Then:   stdout contains "[DIFF] Unchanged: 0  Changed: 0  New: 2  Deleted: 0"
```

### Behavior B17: run_index continues pipeline after diff (no gating)

```
fn run_index_continues_pipeline_analyzing_all_files_after_diff()

Given:  a source directory with 3 .md files
And:    a fresh output directory
When:   run_index(source, output, config) is called
Then:   stdout contains "[STEP 2] ANALYZE"
And:    stdout contains "Processed 3 files"
And:    run_index returns Ok(())
```

### Behavior B18: run_index returns Err when StateDb::open fails

**Setup helper**: `make_readonly_output_dir() -> TempDir` — creates a temp directory, creates a
file inside it, then sets the directory permissions to read-only (chmod 0o444) so that redb
cannot create `state.redb` inside it.

```
fn run_index_returns_err_wrapping_commit_error_database_open_when_output_readonly()

Given:  a valid source directory with .md files
And:    an output directory via make_readonly_output_dir() where redb cannot write
When:   run_index(source, output, config) is called
Then:   result is Err and the error message to_string() contains "failed to open state database"
And:    the error message contains the path substring "state.redb"
```

### Behavior B19: run_index returns Err when begin_read fails

**Setup helper**: `create_corrupted_redb_at(path: &Path)` — creates a file at `path` containing
a valid redb header (first 64 bytes from a real `Database::create`) followed by garbage bytes
(truncate or overwrite the page data). This makes `Database::create` succeed (or repair) but
`begin_read()` fail on the corrupted transaction log.

```
fn run_index_returns_err_wrapping_commit_error_read_transaction_when_redb_corrupted()

Given:  a valid source directory with .md files
And:    an output directory where create_corrupted_redb_at(output.join("state.redb")) has been called
When:   run_index(source, output, config) is called
Then:   result is Err and the error message to_string() contains "failed to begin read transaction"
```

### Behavior B20: run_index returns Err when table initialization fails

**Setup helper**: `create_redb_with_locked_tables(path: &Path)` — creates a valid redb database,
initializes tables, then corrupts the table definition area by overwriting bytes at offset
4096..4100 with 0xFF. This causes `Database::create` to succeed on reopen (the file header is
intact) but `initialize_tables` to fail when it tries to write new table definitions.

```
fn run_index_returns_err_wrapping_commit_error_table_init_when_tables_corrupted()

Given:  a valid source directory with .md files
And:    an output directory where create_redb_with_locked_tables(output.join("state.redb"))
        has been called, producing a file that passes Database::create but fails table init
When:   run_index(source, output, config) is called
Then:   result is Err and the error message to_string() contains "failed to initialize tables"
And:    the error message contains a non-empty reason string
```

### Behavior B21: run_index returns Err when load_file_states encounters malformed row

**Setup helper**: `inject_malformed_file_state_row(db: &Database, key: &str, size: usize)`
— opens a write transaction on the database, writes a raw byte value of the given `size`
(not 200 bytes) to the `file_state` table at the given key.

```
fn run_index_returns_err_wrapping_state_load_error_malformed_row_when_row_size_wrong()

Given:  a valid source directory with .md files
And:    a state.redb opened via StateDb::open, where inject_malformed_file_state_row(db, "corrupt.md", 199)
        has inserted a 199-byte value into the file_state table
When:   run_index(source, output, config) is called
Then:   result is Err and the error message to_string() contains "malformed raw state row"
And:    the error message contains "got 199 bytes, expected 200"
```

### Behavior B22: run_index returns Err wrapping StateLoadError::Utf8KeyError

**Setup helper**: `inject_non_utf8_key_into_file_state(db: &Database)` — opens the redb database
with a raw `TableDefinition<&[u8], &[u8]>` using the same table name `"file_state"`, inserts a
key containing the byte sequence `b"\xFF\xFE\x00invalid"` with a valid 200-byte value. This
bypasses the `&str` type enforcement. When `scan_pod_table` reads the table with
`TableDefinition<&str, &[u8]>`, the redb key decoding for `&str` will either produce
`Utf8KeyError` or a `BackendError`. The test asserts the correct error variant is returned.

> **Note**: With redb 2.x, `&str`-typed table definitions enforce UTF-8 at the type level.
> Injecting non-UTF-8 bytes via a `&[u8]`-typed definition targeting the same table name may
> produce either `Utf8KeyError` or `BackendError` depending on redb's internal validation.
> The test MUST assert which variant actually occurs. If redb 2.x raises `TableError` on
> schema mismatch before any read, this test should be updated to verify the error path
> through a direct `StateLoadError::Utf8KeyError` construction test instead (see fallback below).

**Fallback test** (if injection is blocked by redb schema enforcement):

```
fn state_load_error_utf8_key_error_display_output_is_correct()

Given:  a directly constructed StateLoadError::Utf8KeyError { bytes_lossy: "��invalid" }
When:   the error is converted to a String via .to_string()
Then:   the string contains "non-UTF-8 key in state table"
And:    the string contains "��invalid"
```

This fallback verifies the error variant's Display output and its conversion to `anyhow::Error`,
ensuring the error propagation chain is correct even though the code path is currently
unreachable with `&str`-keyed tables.

### Behavior B23: run_index returns Err when compute_file_diff detects SourceDirNotFound

```
fn run_index_returns_err_wrapping_diff_error_source_dir_not_found_when_source_deleted()

Given:  source path is a valid directory with .md files at discovery time
And:    the source directory is deleted after discovery but before diff computation
        (achieved by calling compute_file_diff directly with a nonexistent source_dir
        to avoid TOCTOU flakiness)
When:   compute_file_diff(&files, &nonexistent_dir, None, &stored_hashes) is called
Then:   result is Err(DiffError::SourceDirNotFound(path)) where path matches nonexistent_dir
And:    the error .to_string() contains "source directory does not exist"
```

### Behavior B24: run_index returns Err when compute_file_diff fails with FileRead

```
fn run_index_returns_err_wrapping_diff_error_file_read_when_file_unreadable()

Given:  a source directory with a .md file
And:    that file's permissions are set to 0o000 (chmod) after discovery but before diff
When:   run_index(source, output, config) is called
Then:   result is Err and the error message to_string() contains "failed to read file"
And:    the error message contains the filename of the unreadable file
```

### Behavior B25: run_index returns Err when compute_file_diff detects PathTraversal

```
fn run_index_returns_err_wrapping_diff_error_path_traversal_when_malicious_path()

Given:  a DiscoveryFile with source_path containing "../../etc/passwd"
And:    a valid source directory
When:   compute_file_diff is called (via the internal path through run_index or directly)
Then:   result is Err(DiffError::PathTraversal { path }) where path == "../../etc/passwd"
And:    the error .to_string() contains "path traversal detected"
```

### Behavior B26: run_index drops StateReadSession cleanly on early return

```
fn run_index_drops_state_read_session_cleanly_on_error_path()

Given:  a valid source directory with .md files
And:    a state.redb where inject_malformed_file_state_row has been called to cause load failure
When:   run_index(source, output, config) is called and returns Err
Then:   StateDb::open(output.join("state.redb")) succeeds on a subsequent call
And:    no stale file locks remain (the new begin_read() call succeeds)
```

### Behavior B27: run_index does NOT call commit_changes

```
fn run_index_does_not_call_commit_changes_during_diff_phase()

Given:  a valid source directory with .md files
And:    a fresh output directory
When:   run_index(source, output, config) is called
Then:   state.redb file_state table has 0 rows (nothing was written)
And:    run_index returns Ok(())
```

### Behavior B28: run_index produces correct mixed diff on second run

```
fn run_index_produces_correct_mixed_diff_on_second_run()

Given:  first run: source dir has "unchanged.md" (content: b"same"), "will_change.md" (content: b"original")
And:    first run: run_index completes successfully
And:    between runs: "will_change.md" content is overwritten with b"modified"
And:    a new file "new_file.md" (content: b"brand new") is added
And:    "unchanged.md" is left untouched
When:   run_index(source, output, config) is called a second time
Then:   stdout contains "[DIFF] Unchanged: 1  Changed: 1  New: 1  Deleted: 0"
```

### Behavior B29: Full pipeline end-to-end with state diff output

```
fn e2e_full_pipeline_completes_with_state_diff_output_for_multi_file_project()

Given:  a source directory with 5 .md files (mix of categories)
And:    a valid category_config.yaml
And:    a fresh output directory
When:   run_index(source, output, config) is called
Then:   return is Ok(())
And:    stdout contains "[DIFF]" line
And:    output directory contains INDEX.json, NAVIGATION.md, and state.redb
```

### Behavior B30: Full pipeline end-to-end on second run shows correct diff

```
fn e2e_second_run_shows_correct_unchanged_changed_new_deleted()

Given:  first run completes successfully with 3 files: "keep.md", "modify.md", "remove.md"
And:    between runs: "keep.md" unchanged, "modify.md" content overwritten with b"changed",
        "remove.md" deleted from source, "add.md" (content: b"new file") added
When:   run_index(source, output, config) is called again
Then:   return is Ok(())
And:    stdout contains "[DIFF] Unchanged: 1  Changed: 1  New: 1  Deleted: 1"
```

---

## 4. Proptest Invariants

### Proptest 1: file_states_to_stored_hashes key preservation

```
Invariant: For any HashMap<String, FileStateRaw>, the output HashMap has exactly the same
           set of keys as the input (no keys lost, no keys added).
Strategy:  Generate arbitrary HashMap<String, FileStateRaw> with:
           - keys: proptest string strategy (alphanumeric, 1..50 chars)
           - values: FileStateRaw with arbitrary [u8; 32] fields and u64 timestamp
           - map size: 0..20 entries
Anti-invariant: Never returns a map with different len() than input
```

Test name: `proptest_file_states_to_stored_hashes_preserves_all_keys`

### Proptest 2: file_states_to_stored_hashes bitwise field identity

```
Invariant: For every (key, FileStateRaw) pair in the input, the output
           StoredHashes.content_hash == FileStateRaw.content_hash AND
           StoredHashes.config_hash == FileStateRaw.config_hash (bitwise identical).
Strategy:  Same as above — arbitrary FileStateRaw values with random [u8; 32] fields.
Anti-invariant: Output hashes must never differ from input hashes for any field
```

Test name: `proptest_file_states_to_stored_hashes_bitwise_field_identity`

### Proptest 3: compute_file_diff partition completeness (INV-6)

```
Invariant: For any combination of discovered_files and stored_hashes,
           the union of unchanged ∪ changed ∪ new ∪ deleted == union of
           discovered paths and stored-hash paths, and no path appears in
           more than one bucket.
Strategy:  Generate:
           - discovered_files: Vec<DiscoveryFile> with random source_path strings
           - stored_hashes: HashMap<String, StoredHashes> with random hashes
           - TempDir with actual files for the discovered paths
           - source_dir = tempdir path
Anti-invariant: The four FileDiff sets must always be disjoint and cover the full input universe
```

Test name: `proptest_compute_file_diff_partition_completeness`

### Proptest 4: compute_file_diff determinism (INV-5)

```
Invariant: compute_file_diff called twice with identical arguments always produces
           the same FileDiff.
Strategy:  Generate random discovered_files, source_dir with real files,
           random stored_hashes.
           Call compute_file_diff twice, assert FileDiff equality.
Anti-invariant: Results must never differ for identical inputs
```

Test name: `proptest_compute_file_diff_deterministic`

---

## 5. Fuzz Targets

No new fuzz targets in this bead. The bead introduces no new parsers, deserializers, or
raw-byte interpreters. All functions consume already-typed data structures
(`HashMap<String, FileStateRaw>`, `Vec<DiscoveryFile>`, etc.).

Existing fuzz surfaces already covered by other beads:
- `FileStateRaw::from_bytes` (state/mod.rs) — covered by state module tests
- `compute_file_diff` file I/O — covered by diff module tests

---

## 6. Kani Harnesses

### Kani Harness 1: FileStateRaw to StoredHashes field projection (INV-4)

```
Property: For any FileStateRaw, the conversion to StoredHashes produces
          content_hash and config_hash that are bitwise identical to the
          original FileStateRaw fields.
Bound:    Map size up to 10 entries (Kani search depth limit).
Rationale: INV-4 mandates bitwise identity. While proptest covers random
           inputs, Kani proves it for ALL inputs within the bound — catching
           any accidental byte swap or offset error.
```

### Kani Harness 2: Diff partition disjointness (INV-6)

```
Property: For any FileDiff returned by the partition logic, the intersection
          of any two distinct buckets is empty (no path in multiple buckets).
Bound:    Up to 5 files, 5 stored paths.
Rationale: INV-6 is a critical invariant. A bug here would cause
           misclassification of files, potentially skipping analysis
           or double-processing. Formal proof is warranted.
```

---

## 7. Mutation Testing Checkpoints

**Target: ≥90% mutation kill rate**

### Critical Mutations to Catch

| Mutation | Caught By |
|----------|-----------|
| `file_states_to_stored_hashes` swaps `content_hash` and `config_hash` | B3 proptest: bitwise field identity |
| `file_states_to_stored_hashes` drops a key | B1: key count assertion |
| `file_states_to_stored_hashes` returns empty for non-empty input | B1: len == 3 |
| `compute_file_diff` classifies all as `Unchanged` (skips hash comparison) | B16: first run shows New: 2, not Unchanged: 2 |
| `compute_file_diff` classifies all as `New` (ignores stored_hashes) | B28: second run shows Unchanged: 1 |
| `compute_file_diff` skips `deleted` computation | B28: shows Deleted: 0 when file was deleted |
| `compute_file_diff` uses `\|\|` instead of `&&` for hash comparison | B28: second run shows Changed vs Unchanged correctly |
| `compute_config_hash` returns random hash instead of SHA-256 | B5: exact hash comparison |
| `compute_config_hash` panics on None | B4: returns empty hash for None |
| `run_index` skips `StateDb::open` call | B8: state.redb does not exist |
| `run_index` skips `begin_read` call | B11: no diff output |
| `run_index` skips `load_file_states` call | B28: all files classified as New on second run |
| `run_index` skips `compute_file_diff` call | B15: no `[DIFF]` line in stdout |
| `run_index` skips diff print | B15: no `[DIFF]` line in stdout |
| `run_index` calls `commit_changes` (write) | B27: state.redb has 0 file_state rows |
| `run_index` gates analysis on diff status | B17: all files still analyzed |
| `validate_path_safety` allows `..` traversal | B25: PathTraversal error |
| `validate_path_safety` allows absolute paths | B25: PathTraversal error |
| `classify_file` ignores config_hash in comparison | B28: config change not detected as Changed |
| Error conversion drops context (empty `.map_err()`) | B18-B25: specific error message substring assertions |

### Mutation Kill Verification Tests

1. **`swap content_hash/config_hash`**: `proptest_file_states_to_stored_hashes_bitwise_field_identity` — catches immediately.
2. **`skip hash comparison (always Unchanged)`**: `run_index_classifies_all_files_as_new_and_empty_deleted_on_first_run` — expects New: 2, not Unchanged: 2.
3. **`skip deleted computation`**: `run_index_produces_correct_mixed_diff_on_second_run` — expects Deleted: 1.
4. **`skip entire diff step`**: `run_index_prints_diff_statistics_in_expected_format` — expects `[DIFF]` line in stdout.
5. **`gate analysis on diff`**: `run_index_continues_pipeline_analyzing_all_files_after_diff` — expects all files processed.
6. **`drop CommitError::TableInit error context`**: B20 asserts error message contains "failed to initialize tables".
7. **`drop StateLoadError::Utf8KeyError error context`**: B22 asserts error message contains "non-UTF-8 key".
8. **`change DatabaseOpen message`**: B18 asserts error message contains "failed to open state database".

---

## 8. Combinatorial Coverage Matrix

### Unit Tests: `file_states_to_stored_hashes`

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| happy path | HashMap with 3 entries | HashMap with same 3 keys, correct hashes | unit |
| empty input | HashMap::new() | HashMap::new() (len == 0) | unit |
| single entry | HashMap with 1 entry | HashMap with 1 entry, bitwise matching hashes | unit |
| large input | HashMap with 100 entries | HashMap with 100 entries, all keys present | unit |
| hash identity | arbitrary FileStateRaw | StoredHashes with matching content_hash and config_hash | unit |
| **boundary: max valid** | HashMap with 10,000 entries | HashMap with 10,000 entries, all keys preserved | unit |
| **boundary: overflow** | HashMap near `isize::MAX` entries | Result is a HashMap of identical size (documented as N/A for practical purposes — Rust allocates on heap, OOM before logic error. Not a testable boundary.) | unit |
| invariant: keys preserved | any valid HashMap | output.keys() == input.keys() | proptest |
| invariant: bitwise hashes | any valid HashMap | all hashes match per entry | proptest |

### Integration Tests: `run_index` state/diff wiring

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| first run, 3 files | fresh output, 3 .md files | `[DIFF] Unchanged: 0  Changed: 0  New: 3  Deleted: 0` | integration |
| second run, mixed | modified files between runs | `[DIFF] Unchanged: 1  Changed: 1  New: 1  Deleted: 0` | integration |
| empty source | source with 0 .md files | bail before diff (existing behavior) | integration |
| state.db open failure | read-only output dir | Err, message contains "failed to open state database" | integration |
| table init failure | corrupted redb tables | Err, message contains "failed to initialize tables" | integration |
| read transaction failure | corrupted redb txn log | Err, message contains "failed to begin read transaction" | integration |
| malformed row in state | 199-byte value in file_state | Err, message contains "malformed raw state row" and "got 199 bytes, expected 200" | integration |
| non-UTF-8 key in state | non-UTF-8 bytes in key column | Err, message contains "non-UTF-8 key in state table" (or documented as unreachable with current schema) | integration |
| source dir missing | deleted source | Err, message contains "source directory does not exist" | integration |
| file not readable | chmod 0o000 file | Err, message contains "failed to read file" | integration |
| path traversal | malicious source_path | Err(DiffError::PathTraversal { path }) with path == traversal string | integration |
| no config | category_config = None | diff uses empty hash for config | integration |
| with config | category_config = Some(path) | diff uses SHA-256 of config bytes | integration |
| session dropped on error | error-inducing state | state.redb openable afterward | integration |
| no writes | any valid run | state.redb file_state has 0 rows | integration |
| pipeline continues | any valid run | STEP 2 ANALYZE runs on all files | integration |
| config change between runs | config modified | files classified as Changed | integration |
| **boundary: max valid files** | source with 1,000 .md files | `[DIFF]` line with correct counts, pipeline completes | integration |
| **boundary: max stored state** | state DB with 1,000 pre-existing rows | `[DIFF]` line shows correct Unchanged/Deleted counts | integration |
| **boundary: overflow** | Files exceeding filesystem limits | Documented as N/A — filesystem limits caught by OS errors before logic boundary. No testable code-level overflow. | integration |

### E2E Tests

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| full first run | 5 .md files, config.yaml | INDEX.json + NAVIGATION.md + state.redb + `[DIFF]` line | e2e |
| full second run | modified source | `[DIFF] Unchanged: 1  Changed: 1  New: 1  Deleted: 1` + complete pipeline output | e2e |

---

## Error Variant Coverage

Every error variant mentioned in the contract must have at least one planned test scenario:

### CommitError Variants

| Variant | Test Scenario |
|---------|---------------|
| `CommitError::DatabaseOpen { path, reason }` | B18: run_index with read-only output directory — asserts error message contains `"failed to open state database"` |
| `CommitError::TableInit { reason }` | B20: run_index with corrupted redb table definitions — asserts error message contains `"failed to initialize tables"` and a non-empty reason string |
| `CommitError::ReadTransaction { reason }` | B19: run_index with corrupted redb transaction log — asserts error message contains `"failed to begin read transaction"` |

### StateLoadError Variants

| Variant | Test Scenario |
|---------|---------------|
| `StateLoadError::MalformedRow { key, actual, expected }` | B21: run_index with 199-byte value in file_state — asserts error message contains `"malformed raw state row"` and `"got 199 bytes, expected 200"` |
| `StateLoadError::Utf8KeyError { bytes_lossy }` | B22: inject non-UTF-8 key into file_state table via raw `TableDefinition<&[u8], &[u8]>` and assert error message contains `"non-UTF-8 key in state table"`. Fallback: directly construct `StateLoadError::Utf8KeyError` and verify Display + anyhow conversion. |
| `StateLoadError::BackendError { operation, message }` | B21 setup also validates BackendError path when table is missing. Test: `StateReadSession::new` on uninitiated database, `load_file_states()` returns `BackendError { operation: "open_table" }`. |

### DiffError Variants

| Variant | Test Scenario |
|---------|---------------|
| `DiffError::SourceDirNotFound(path)` | B23: compute_file_diff with nonexistent source_dir — asserts `Err(DiffError::SourceDirNotFound(path))` and message contains `"source directory does not exist"` |
| `DiffError::FileRead { path, source }` | B24: run_index with chmod 0o000 file — asserts error message contains `"failed to read file"` and the filename |
| `DiffError::PathTraversal { path }` | B25: compute_file_diff with `"../../etc/passwd"` path — asserts `Err(DiffError::PathTraversal { path })` and message contains `"path traversal detected"` |

---

## Test Infrastructure

### Setup Helpers (named, side-effect-advertising)

| Helper Name | Purpose | Used By |
|-------------|---------|---------|
| `seed_file_state_rows(db, rows)` | Write known FileStateRaw entries to file_state table via redb write transaction | B10, B12 |
| `seed_matching_file_state_rows(db, source_dir, files)` | Write FileStateRaw entries whose hashes match actual file contents on disk | B12 |
| `make_readonly_output_dir()` | Create TempDir with chmod 0o444 (redb cannot write) | B18 |
| `create_corrupted_redb_at(path)` | Create a file that looks like redb but has corrupted transaction log | B19 |
| `create_redb_with_locked_tables(path)` | Create valid redb, then corrupt table definition area at offset 4096 | B20 |
| `inject_malformed_file_state_row(db, key, size)` | Insert raw bytes (not 200) into file_state table | B21, B26 |
| `inject_non_utf8_key_into_file_state(db)` | Insert non-UTF-8 key via raw `TableDefinition<&[u8], &[u8]>` | B22 |

### Cleanup Strategy

All integration tests use `tempfile::TempDir` which provides RAII-based cleanup. The `TempDir`
is held in a binding (`let _temp_dir = TempDir::new()`) that is dropped at test function exit,
removing all files including `state.redb`. No manual cleanup required. File permission changes
(e.g., `make_readonly_output_dir`) are restored by `TempDir`'s drop, which uses `rm -rf`
semantics that bypass permission checks on the directory itself.

---

## Open Questions

1. **Q: How to trigger `CommitError::ReadTransaction` deterministically?**
   Creating a corrupted redb file that causes `begin_read()` to fail is non-trivial.
   The `create_corrupted_redb_at` helper attempts this by corrupting the transaction log
   area of a valid redb file. If redb 2.x's crash recovery repairs the damage transparently,
   the test may need to use a different corruption strategy (e.g., truncating the file to
   just the header). If no reliable method exists, this scenario will be tested via direct
   construction of `CommitError::ReadTransaction` with error message verification.

2. **Q: Should `run_index` integration tests use the full `run_index` function or
   extract the state/diff wiring into a testable helper?**
   Recommendation: Extract the new STEP 1.5 logic into a helper function
   (e.g., `compute_initial_diff`) that can be tested independently with controlled
   inputs. The integration test for `run_index` then only verifies the wiring.

3. **Q: How to capture stdout for B15 (diff statistics format)?**
   Use a helper that redirects stdout to a buffer, or refactor the print to accept
   a `Write` trait object for testability. If neither is possible, use the `assert_cmd`
   crate or process output capture in E2E tests.

4. **Q: Can redb 2.x `&str`-keyed tables be opened with `&[u8]` keys to inject non-UTF-8?**
   If redb enforces type compatibility at the schema level, the `inject_non_utf8_key_into_file_state`
   helper will fail with `TableError`. In that case, B22 falls back to a direct-construction
   test of `StateLoadError::Utf8KeyError` verifying Display and anyhow conversion. The test
   plan covers both paths.

5. **Q: Race condition in B23 (source deleted between discovery and diff)?**
   This is inherently a TOCTOU issue. The test calls `compute_file_diff` directly with a
   non-existent `source_dir` to avoid flaky timing-dependent tests.
