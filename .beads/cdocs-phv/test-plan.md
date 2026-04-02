# Test Plan: cdocs-phv — Commit Index State Once at Shutdown

## Summary

- **Behaviors identified:** 29
- **Trophy allocation:** 14 unit (48%) / 13 integration (45%) / 2 e2e (7%)
  - Deviation from 30/60/5/5 justified: StateDb has significant pure in-memory logic
  (error-path guards, state queries, validation) that naturally falls into unit tests.
- **Static analysis baseline:** clippy (pedantic), cargo-deny, rustc type system —
  enforced in CI, not counted as BDD scenarios.
- **Proptest invariants:** 6
- **Fuzz targets:** 2
- **Kani harnesses:** 3
- **Mutation kill target:** ≥90%

---

## 1. Behavior Inventory

### StateDb Construction

| # | Behavior |
|---|----------|
| B01 | `StateDb::new` returns initialized `StateDb` when output directory exists and is writable |
| B02 | `StateDb::new` returns `StateError::OutputNotAccessible` when output directory does not exist |
| B03 | `StateDb::new` returns `StateError::OutputNotAccessible` when output directory is not writable |
| B04 | `StateDb::new` initializes batch with `committed = false` and empty `StateBatch` fields |
| B05 | `StateDb::new` returns `StateError::OutputNotAccessible` when path is an empty string `""` |
| B06 | `StateDb::new` returns `StateError::OutputNotAccessible` when path points to a regular file (not a directory) |
| B07 | `StateDb::new` returns `StateError::OutputNotAccessible` when path is a dangling symlink |
| B08 | `StateDb::new` returns `StateError::PreconditionViolation` when OutputLock is not held for the output directory |

### Mutation Operations (pre-commit)

| # | Behavior |
|---|----------|
| B09 | `record_file_hash` appends a `FileHashRecord` to the pending batch when state is uncommitted |
| B10 | `record_file_hash` returns `StateError::MutationAfterCommit` when state is already committed |
| B11 | `record_file_hash` returns `StateError::DuplicateFilePath` when the same `relative_path` is recorded twice |
| B12 | `set_document_count` updates batch `document_count` when state is uncommitted |
| B13 | `set_document_count` returns `StateError::MutationAfterCommit` when state is already committed |
| B14 | `set_chunk_count` updates batch `chunk_count` when state is uncommitted |
| B15 | `set_chunk_count` returns `StateError::MutationAfterCommit` when state is already committed |
| B16 | `set_chunk_count` with `count == 0` succeeds (no validation on count value itself; EmptyBatch fires at commit time) |

### Commit Operation

| # | Behavior |
|---|----------|
| B17 | `commit_changes` persists `StateBatch` to output directory and sets `committed = true` |
| B18 | `commit_changes` returns `StateError::AlreadyCommitted` when called a second time |
| B19 | `commit_changes` returns `StateError::EmptyBatch` when no documents were processed (document_count == 0) |
| B20 | `commit_changes` returns `StateError::PersistenceFailed` when I/O write fails |
| B21 | `commit_changes` returns `StateError::SerializationFailed` when batch serialization fails |

### Query Operations

| # | Behavior |
|---|----------|
| B22 | `is_committed` returns `false` on a newly created `StateDb` |
| B23 | `is_committed` returns `true` after `commit_changes` succeeds |

### Drop Behavior

| # | Behavior |
|---|----------|
| B24 | `Drop` does not write any state file when `StateDb` is dropped without committing |

### Batch Consistency

| # | Behavior |
|---|----------|
| B25 | Distinct `relative_path` entries accumulate correctly; batch has N entries after N distinct calls |

### Pipeline Integration

| # | Behavior |
|---|----------|
| B26 | `run_index` commits state exactly once when all pipeline stages succeed |
| B27 | `run_index` writes zero state when any pipeline stage fails (early `?` return) |
| B28 | Committed `StateBatch.document_count` equals number of analyzed documents |
| B29 | Committed `StateBatch.chunk_count` equals `chunks_result.total_chunks` from STEP 5 |

---

## 2. Trophy Allocation

| Behavior | Layer | Justification |
|----------|-------|---------------|
| B01 | Integration | Creates real filesystem entries; requires tempdir |
| B02 | Integration | Requires non-existent path on real filesystem |
| B03 | Integration | Requires permission manipulation on real filesystem |
| B04 | Unit | Pure state check after construction |
| B05 | Integration | Empty string path triggers filesystem access; requires real FS to confirm |
| B06 | Integration | Requires a real regular file on filesystem to pass as path |
| B07 | Integration | Requires symlink creation on real filesystem |
| B08 | Integration | Requires OutputLock + real filesystem to exercise lock-check path |
| B09 | Unit | Pure mutation of in-memory batch |
| B10 | Unit | Pure error-path, no I/O |
| B11 | Unit | Pure duplicate-detection logic |
| B12 | Unit | Pure mutation of in-memory batch |
| B13 | Unit | Pure error-path, no I/O |
| B14 | Unit | Pure mutation of in-memory batch |
| B15 | Unit | Pure error-path, no I/O |
| B16 | Unit | Pure in-memory validation (count == 0 accepted, no error at set time) |
| B17 | Integration | Writes to real filesystem; verifies file contents |
| B18 | Unit | Pure state machine guard |
| B19 | Unit | Pure validation of batch content |
| B20 | Integration | Must provoke real I/O failure (read-only dir after construction) |
| B21 | Integration | Must provoke real serialization failure |
| B22 | Unit | Pure query, no side effects |
| B23 | Unit | Pure query after state transition |
| B24 | Integration | Must verify no file was written on real filesystem after Drop |
| B25 | Unit | Pure invariant check on in-memory data |
| B26 | E2E | Full pipeline with real source directory and real output |
| B27 | E2E | Full pipeline with deliberate failure injection |
| B28 | Integration | Partial pipeline with real stages through STEP 2 |
| B29 | Integration | Partial pipeline with real stages through STEP 5 |

**Allocation summary:** 14 unit (48%) / 13 integration (45%) / 2 e2e (7%)

---

## 3. BDD Scenarios

### B01: `StateDb::new` succeeds with valid output directory

```
Given:  a writable temporary directory
When:   StateDb::new(temp_dir.path()) is called
Then:   Ok(StateDb) is returned
And:    state_db.is_committed() == false
```

Test: `fn state_db_new_returns_ok_when_output_dir_exists()`

### B02: `StateDb::new` fails when output directory does not exist

```
Given:  a path to a directory that does not exist (e.g., /tmp/nope-xyz-nonexistent)
When:   StateDb::new(nonexistent_path) is called
Then:   Err(StateError::OutputNotAccessible { path }) where path matches the input path
```

Test: `fn state_db_new_returns_output_not_accessible_when_dir_missing()`

### B03: `StateDb::new` fails when output directory is not writable

```
Given:  a temporary directory with mode 0o444 (read-only)
When:   StateDb::new(read_only_dir) is called
Then:   Err(StateError::OutputNotAccessible { path }) where path matches the input path
```

Test: `fn state_db_new_returns_output_not_accessible_when_dir_not_writable()`

### B04: `StateDb::new` initializes empty batch

```
Given:  a writable temporary directory
When:   StateDb::new(temp_dir.path()) is called
Then:   the returned StateDb has is_committed() == false
And:    the internal batch has document_count == 0, chunk_count == 0, file_hashes.is_empty()
```

Test: `fn state_db_new_initializes_empty_batch_and_uncommitted()`

### B05: `StateDb::new` fails when path is empty string

```
Given:  an empty string path ""
When:   StateDb::new(Path::new("")) is called
Then:   Err(StateError::OutputNotAccessible { path: "" })
```

Test: `fn state_db_new_returns_output_not_accessible_when_path_is_empty_string()`

### B06: `StateDb::new` fails when path points to a regular file

```
Given:  a temporary directory containing a regular file "not_a_dir.txt"
When:   StateDb::new(temp_dir.path().join("not_a_dir.txt")) is called
Then:   Err(StateError::OutputNotAccessible { path }) where path ends with "not_a_dir.txt"
```

Test: `fn state_db_new_returns_output_not_accessible_when_path_is_file()`

### B07: `StateDb::new` fails when path is a dangling symlink

```
Given:  a symlink pointing to a non-existent target (e.g., temp_dir/link -> temp_dir/no_such_target)
When:   StateDb::new(symlink_path) is called
Then:   Err(StateError::OutputNotAccessible { path }) where path matches the symlink path
```

Test: `fn state_db_new_returns_output_not_accessible_when_path_is_dangling_symlink()`

### B08: `StateDb::new` returns PreconditionViolation when OutputLock is not held

```
Given:  a writable temporary directory where OutputLock has NOT been acquired
        (no acquire_output_lock call made, no .ctd.lock file exists)
When:   StateDb::new(temp_dir.path()) is called
Then:   Err(StateError::PreconditionViolation { detail })
        where detail contains "output lock not held"
        and detail contains the output directory path
```

Test: `fn state_db_new_returns_precondition_violation_when_output_lock_not_held()`

### B09: `record_file_hash` appends entry

```
Given:  a fresh StateDb in uncommitted state
When:   state_db.record_file_hash("src/guide.md", "sha256:abc123") is called
Then:   Ok(()) is returned
And:    subsequent commit persists a StateBatch where file_hashes contains
        FileHashRecord { relative_path: "src/guide.md", content_hash: "sha256:abc123" }
```

Test: `fn record_file_hash_appends_entry_when_uncommitted()`

### B10: `record_file_hash` rejects after commit

```
Given:  a committed StateDb (commit_changes returned Ok)
When:   state_db.record_file_hash("any.md", "hash") is called
Then:   Err(StateError::MutationAfterCommit { run_id }) where run_id matches the batch's run_id
```

Test: `fn record_file_hash_returns_mutation_after_commit_when_committed()`

### B11: `record_file_hash` rejects duplicate path

```
Given:  a fresh StateDb where record_file_hash("src/guide.md", "hash1") was called successfully
When:   state_db.record_file_hash("src/guide.md", "hash2") is called
Then:   Err(StateError::DuplicateFilePath { path: "src/guide.md" })
```

Test: `fn record_file_hash_returns_duplicate_file_path_when_same_path_twice()`

### B12: `set_document_count` updates count

```
Given:  a fresh StateDb in uncommitted state
When:   state_db.set_document_count(42) is called
Then:   Ok(()) is returned
And:    the committed batch has document_count == 42
```

Test: `fn set_document_count_updates_batch_when_uncommitted()`

### B13: `set_document_count` rejects after commit

```
Given:  a committed StateDb
When:   state_db.set_document_count(10) is called
Then:   Err(StateError::MutationAfterCommit { run_id }) where run_id matches the batch's run_id
```

Test: `fn set_document_count_returns_mutation_after_commit_when_committed()`

### B14: `set_chunk_count` updates count

```
Given:  a fresh StateDb in uncommitted state
When:   state_db.set_chunk_count(128) is called
Then:   Ok(()) is returned
And:    the committed batch has chunk_count == 128
```

Test: `fn set_chunk_count_updates_batch_when_uncommitted()`

### B15: `set_chunk_count` rejects after commit

```
Given:  a committed StateDb
When:   state_db.set_chunk_count(50) is called
Then:   Err(StateError::MutationAfterCommit { run_id }) where run_id matches the batch's run_id
```

Test: `fn set_chunk_count_returns_mutation_after_commit_when_committed()`

### B16: `set_chunk_count` with zero count succeeds at set time

```
Given:  a fresh StateDb in uncommitted state
When:   state_db.set_chunk_count(0) is called
Then:   Ok(()) is returned
And:    the batch's chunk_count == 0
And:    a subsequent commit_changes() with document_count > 0 succeeds
        (chunk_count == 0 is valid — EmptyBatch is determined by document_count only)
```

Test: `fn set_chunk_count_zero_succeeds_and_commit_succeeds_when_documents_exist()`

### B17: `commit_changes` persists and transitions state

```
Given:  a StateDb with at least one document recorded (set_document_count(1), set_chunk_count(1),
        record_file_hash("a.md", "hash"))
When:   state_db.commit_changes() is called
Then:   Ok(()) is returned
And:    state_db.is_committed() == true
And:    a state file exists in the output directory containing the serialized StateBatch
And:    the deserialized file has document_count == 1, chunk_count == 1,
        file_hashes == [FileHashRecord { relative_path: "a.md", content_hash: "hash" }]
```

Test: `fn commit_changes_persists_batch_and_marks_committed()`

### B18: `commit_changes` rejects double commit

```
Given:  a committed StateDb (commit_changes returned Ok)
When:   state_db.commit_changes() is called a second time
Then:   Err(StateError::AlreadyCommitted { run_id }) where run_id matches the batch
```

Test: `fn commit_changes_returns_already_committed_when_called_twice()`

### B19: `commit_changes` rejects empty batch

```
Given:  a fresh StateDb with document_count == 0 and no file_hashes
When:   state_db.commit_changes() is called
Then:   Err(StateError::EmptyBatch { run_id }) where run_id matches the batch
```

Test: `fn commit_changes_returns_empty_batch_when_no_documents()`

### B20: `commit_changes` fails on I/O error

```
Given:  a StateDb whose output_dir has been deleted after construction
        (but after at least one document recorded: set_document_count(1), record_file_hash("a.md", "h"))
When:   state_db.commit_changes() is called
Then:   Err(StateError::PersistenceFailed { run_id, reason })
        where reason contains the string representation of the output directory path
        (because the I/O error from writing to a deleted directory references the path)
```

Test: `fn commit_changes_returns_persistence_failed_when_io_fails()`

### B21: `commit_changes` fails on serialization error

```
Given:  a StateDb where the StateBatch serialization is forced to fail
        (e.g., construct a batch where a FileHashRecord relative_path contains a
        string that triggers a serializer-specific error — for JSON this means testing
        with a custom Serialize impl that returns an error; for the implementation
        bead: inject a non-serializable value via a test-only constructor or
        use #[serde(skip_serializing)] to force a missing-required-field error)
When:   state_db.commit_changes() is called
Then:   Err(StateError::SerializationFailed { reason })
        where reason contains "serialize" or the name of the serialization format
        (e.g., "JSON" or "serde" — the error message from the serializer)
```

Test: `fn commit_changes_returns_serialization_failed_when_serialize_errors()`

### B22: `is_committed` returns false on new StateDb

```
Given:  a freshly created StateDb
When:   state_db.is_committed() is called
Then:   false
```

Test: `fn is_committed_returns_false_when_newly_created()`

### B23: `is_committed` returns true after commit

```
Given:  a committed StateDb
When:   state_db.is_committed() is called
Then:   true
```

Test: `fn is_committed_returns_true_after_commit()`

### B24: Drop does not commit

```
Given:  a fresh StateDb with mutations (record_file_hash, set_document_count) but no commit
When:   state_db goes out of scope (dropped)
Then:   no state file exists in the output directory
And:    no new files appear in the output directory at all
```

Test: `fn drop_does_not_write_state_file_when_uncommitted()`

### B25: Distinct file paths accumulate correctly

```
Given:  a fresh StateDb
When:   record_file_hash("a.md", "h1") is called
And:    record_file_hash("b.md", "h2") is called
And:    record_file_hash("c.md", "h3") is called
Then:   all three calls return Ok(())
And:    after commit, the batch file_hashes has exactly 3 entries:
        [FileHashRecord { relative_path: "a.md", content_hash: "h1" },
         FileHashRecord { relative_path: "b.md", content_hash: "h2" },
         FileHashRecord { relative_path: "c.md", content_hash: "h3" }]
```

Test: `fn batch_accepts_distinct_paths_and_accumulates_correctly()`

### B26: `run_index` commits on full success

```
Given:  a source directory with 3 valid markdown files
        and a writable output directory
When:   run_index(source, output, config) completes successfully
Then:   a state file exists in the output directory
And:    the deserialized StateBatch.document_count == 3
And:    the deserialized StateBatch.file_hashes has exactly 3 entries
```

Test: `fn run_index_commits_state_when_all_stages_succeed()`

### B27: `run_index` writes nothing on failure

```
Given:  a source directory with 1 valid markdown file
        and a writable output directory
        and STEP 4 (TRANSFORM) is configured to fail by providing a source file
        with malformed frontmatter (e.g., unclosed YAML fence "---" without closing "---")
        that causes the transform stage to return Err
When:   run_index(source, output, config) returns Err
Then:   no state file exists in the output directory
And:    no partial state artifacts from this run appear
```

Test: `fn run_index_writes_zero_state_when_pipeline_stage_fails()`

### B28: Committed batch reflects document count

```
Given:  a StateDb populated through a partial pipeline (through STEP 2 ANALYZE)
        that processed 5 documents
When:   commit_changes() is called
Then:   the persisted StateBatch.document_count == 5
```

Test: `fn committed_batch_document_count_equals_analyzed_documents()`

### B29: Committed batch reflects chunk count

```
Given:  a StateDb populated through a partial pipeline (through STEP 5 CHUNK)
        that produced 32 chunks
When:   commit_changes() is called
Then:   the persisted StateBatch.chunk_count == 32
```

Test: `fn committed_batch_chunk_count_equals_chunks_result()`

---

## 4. Proptest Invariants

### PROP-01: `record_file_hash` — idempotent rejection

- **Invariant:** For any two calls to `record_file_hash` with the same `relative_path` (regardless of
  `content_hash`), the second call always returns `Err(StateError::DuplicateFilePath)`.
- **Strategy:** Generate pairs `(relative_path: String, content_hash_a: String, content_hash_b: String)`
  where `relative_path` is any non-empty alphanumeric path string and hashes are random hex strings.
- **Anti-invariant:** Two calls with distinct `relative_path` values always succeed.

### PROP-02: `set_document_count` / `set_chunk_count` — last-write-wins

- **Invariant:** After calling `set_document_count(N)` where N > 0, the committed batch has
  `document_count == N`, regardless of any prior calls with different values. Same for `set_chunk_count`.
- **Strategy:** Generate a `Vec<usize>` of non-zero counts, apply them sequentially, commit,
  read back. The final persisted count must equal the last element.
- **Anti-invariant:** Calling with count == 0 before commit results in `EmptyBatch` if
  document_count is still 0 at commit time.

### PROP-03: Round-trip serialization

- **Invariant:** For any valid `StateBatch` (with `document_count > 0`), serializing then
  deserializing produces an identical `StateBatch`.
- **Strategy:** Generate `StateBatch` with:
  - `run_id`: random alphanumeric string
  - `source_path` / `output_path`: random path strings
  - `document_count`: `1..=1000`
  - `chunk_count`: `0..=10000`
  - `file_hashes`: vec of `FileHashRecord` with distinct `relative_path` strings
  - `created_at_unix_secs`: `0..=u64::MAX`
- **Anti-invariant:** Corrupt bytes (flip bits) produce deserialization failure.

### PROP-04: State machine exhaustiveness

- **Invariant:** The state machine `{Uncommitted} -> {Committed}` has exactly 2 states.
  `is_committed()` always returns `false` before commit and `true` after. No sequence of
  operations can produce `is_committed() == true` then `is_committed() == false`.
- **Strategy:** Generate random sequences of operations
  (`record_file_hash`, `set_document_count`, `set_chunk_count`, `commit_changes`, `is_committed`)
  applied to a fresh `StateDb`. Assert the invariant holds after every operation.
- **Anti-invariant:** No operation sequence can make `committed` revert to `false`.

### PROP-05: Batch non-negativity

- **Invariant:** `document_count` and `chunk_count` are always `>= 0` (usize guarantees this
  in Rust, but the proptest verifies the domain constraint is never violated through the API).
- **Strategy:** Call `set_document_count` and `set_chunk_count` with `any::<usize>()` values.
  Commit and read back. Values must match exactly.

### PROP-06: File hash uniqueness

- **Invariant:** The set of `relative_path` values in `file_hashes` has the same cardinality as
  the `file_hashes` vector after any sequence of successful `record_file_hash` calls.
- **Strategy:** Generate a `Vec<String>` of unique paths, call `record_file_hash` for each with
  random hashes, commit, read back. Verify `file_hashes.len() == file_hashes.iter().map(|h| &h.relative_path).collect::<HashSet<_>>().len()`.
- **Anti-invariant:** Any duplicate path in the input must be rejected with `DuplicateFilePath`.

---

## 5. Fuzz Targets

### FUZZ-01: `StateBatch` deserialization

- **Target function:** The deserialization path for reading a persisted `StateBatch` from disk.
- **Input type:** `arbitrary bytes` — raw bytes fed into the deserializer (e.g., `serde_json::from_slice`).
- **Risk class:** Panic on malformed UTF-8, arithmetic overflow on crafted counts, OOM from
  crafted large `file_hashes` vectors, logic bugs from inconsistent state.
- **Corpus seeds:**
  - A valid serialized `StateBatch` (happy path)
  - Truncated JSON (`{"run_id": "abc"`)
  - Empty bytes (`[]`)
  - Very large `file_hashes` array
  - Negative counts represented as strings (`"document_count": "-1"`)
  - `document_count` as string instead of number
  - Missing required fields
  - Extra unknown fields

### FUZZ-02: `record_file_hash` input validation

- **Target function:** `StateDb::record_file_hash(&mut self, relative_path: &str, content_hash: &str)`
- **Input type:** `arbitrary (String, String)` — two arbitrary UTF-8 strings.
- **Risk class:** Panic on empty strings, NUL bytes, extremely long strings, Unicode edge cases.
  Paths with `../` traversal could be a security concern.
- **Corpus seeds:**
  - Empty string for `relative_path` (accepted — no path validation beyond duplicates)
  - Empty string for `content_hash` (accepted — no hash validation)
  - Path with `../` traversal (`../../etc/passwd`)
  - Very long path (64KB+)
  - Path with Unicode characters (`日本語/ガイド.md`)
  - Path with special chars (`path with spaces/and#hashes.md`)

---

## 6. Kani Harnesses

### KANI-01: State machine transition completeness

- **Property:** The `committed` field transitions from `false` to `true` at most once.
  After `commit_changes` returns `Ok(())`, no operation can set `committed` back to `false`.
- **Bound:** Single `StateDb` instance, up to 5 operations.
- **Rationale:** This is the core correctness invariant (INV-01). A state machine bug here
  could cause double-commits or lost writes. Formal verification is warranted because
  proptest may not explore all interleavings of operations.

### KANI-02: `document_count` / `chunk_count` arithmetic safety

- **Property:** No arithmetic overflow in `set_document_count(count)` or `set_chunk_count(count)`
  for any `usize` input. The stored value must exactly equal the input value.
- **Bound:** `count` in range `0..=usize::MAX`.
- **Rationale:** While Rust's usize doesn't overflow on assignment, any intermediate
  arithmetic (addition during batch aggregation) must be verified safe.

### KANI-03: `file_hashes` vector capacity bounds

- **Property:** After `N` successful `record_file_hash` calls with distinct paths,
  `file_hashes.len() == N`. No off-by-one errors, no duplicate entries.
- **Bound:** `N` in range `0..=100`.
- **Rationale:** Index bounds errors in batch bookkeeping could cause silent data loss
  or panics. Bounded model checking can exhaustively prove correctness for small N.

---

## 7. Mutation Testing Checkpoints

**Target:** ≥90% mutation kill rate via `cargo-mutants`.

### Critical Mutations and Their Catching Tests

| Mutation | Caught By |
|----------|-----------|
| Remove `committed = true` in `commit_changes` | B23: `is_committed_returns_true_after_commit` — asserts `true` |
| Remove `AlreadyCommitted` guard in `commit_changes` | B18: `commit_changes_returns_already_committed_when_called_twice` — asserts exact error variant |
| Remove `MutationAfterCommit` guard in `record_file_hash` | B10: `record_file_hash_returns_mutation_after_commit_when_committed` — asserts exact variant + run_id |
| Remove `MutationAfterCommit` guard in `set_document_count` | B13: `set_document_count_returns_mutation_after_commit_when_committed` — asserts exact variant + run_id |
| Remove `MutationAfterCommit` guard in `set_chunk_count` | B15: `set_chunk_count_returns_mutation_after_commit_when_committed` — asserts exact variant + run_id |
| Remove `DuplicateFilePath` check in `record_file_hash` | B11: `record_file_hash_returns_duplicate_file_path_when_same_path_twice` |
| Remove `EmptyBatch` check in `commit_changes` | B19: `commit_changes_returns_empty_batch_when_no_documents` |
| Remove file write in `commit_changes` | B17: `commit_changes_persists_batch_and_marks_committed` — reads file back |
| Remove state file write from `commit_changes` (make it a no-op) | B17: verifies file content after commit |
| Change `is_committed` to always return `true` | B22: `is_committed_returns_false_when_newly_created` |
| Change `is_committed` to always return `false` | B23: `is_committed_returns_true_after_commit` |
| Remove `Drop` guard (make Drop call commit) | B24: `drop_does_not_write_state_file_when_uncommitted` — checks no file created |
| Remove document_count assignment in `set_document_count` | B12: `set_document_count_updates_batch_when_uncommitted` — reads back via commit |
| Remove chunk_count assignment in `set_chunk_count` | B14: `set_chunk_count_updates_batch_when_uncommitted` |
| Remove `OutputNotAccessible` check in `new` (missing dir) | B02: `state_db_new_returns_output_not_accessible_when_dir_missing` |
| Remove `OutputNotAccessible` check in `new` (empty string) | B05: `state_db_new_returns_output_not_accessible_when_path_is_empty_string` |
| Remove `OutputNotAccessible` check in `new` (file-as-path) | B06: `state_db_new_returns_output_not_accessible_when_path_is_file` |
| Remove `OutputNotAccessible` check in `new` (dangling symlink) | B07: `state_db_new_returns_output_not_accessible_when_path_is_dangling_symlink` |
| Remove `PreconditionViolation` check in `new` (no OutputLock) | B08: `state_db_new_returns_precondition_violation_when_output_lock_not_held` — asserts exact variant + detail |
| Flip condition in pipeline commit guard (commit on failure) | B27: `run_index_writes_zero_state_when_pipeline_stage_fails` |

### Mutation Kill Strategy

1. Every `if !self.committed` / `if self.committed` branch must be tested from both sides.
2. Every `Err(StateError::*)` return must have a test that asserts the exact variant.
3. Every field assignment (`self.batch.document_count = count`) must be verified by committing
   and reading back the persisted value.
4. The `Drop` impl must be tested by verifying the filesystem side-effect is absent.
5. The `PreconditionViolation` branch must be exercised by constructing StateDb without
   the OutputLock, ensuring the deletion mutant is killed.

---

## 8. Combinatorial Coverage Matrix

### StateDb Construction

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| valid dir | existing, writable path | `Ok(StateDb { committed: false })` | integration |
| missing dir | non-existent path | `Err(OutputNotAccessible { path })` where path matches input | integration |
| read-only dir | existing, read-only path | `Err(OutputNotAccessible { path })` where path matches input | integration |
| empty string | `Path::new("")` | `Err(OutputNotAccessible { path: "" })` | integration |
| file as path | path to regular file | `Err(OutputNotAccessible { path })` where path ends with filename | integration |
| dangling symlink | symlink to non-existent target | `Err(OutputNotAccessible { path })` where path matches symlink | integration |
| no OutputLock | valid dir, no lock file | `Err(PreconditionViolation { detail })` where detail contains "output lock not held" | integration |

### `record_file_hash`

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| happy path | unique path, any hash | `Ok(())`, batch contains entry with exact path and hash | unit |
| duplicate path | same path twice | `Err(DuplicateFilePath { path: "src/guide.md" })` | unit |
| after commit | any valid input | `Err(MutationAfterCommit { run_id })` where run_id matches batch | unit |
| empty path | `""` | `Ok(())` — accepted, no path validation beyond duplicates | unit |
| path with traversal | `"../etc/passwd"` | `Ok(())` — accepted, no path sanitization in contract | unit |

### `set_document_count`

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| happy path | any `usize > 0` | `Ok(())`, batch reflects count exactly | unit |
| count == 1 (minimum meaningful) | `1` | `Ok(())`, batch reflects count == 1 (covered by B17 which uses count == 1) | unit |
| zero count | `0` | `Ok(())`, but commit fails with `EmptyBatch { run_id }` | unit |
| after commit | any `usize` | `Err(MutationAfterCommit { run_id })` where run_id matches batch | unit |
| usize::MAX | `usize::MAX` | `Ok(())`, batch reflects count | unit |

### `set_chunk_count`

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| happy path | any `usize > 0` | `Ok(())`, batch reflects count exactly | unit |
| count == 0 | `0` | `Ok(())`, batch reflects chunk_count == 0; commit succeeds if document_count > 0 | unit |
| after commit | any `usize` | `Err(MutationAfterCommit { run_id })` where run_id matches batch | unit |
| usize::MAX | `usize::MAX` | `Ok(())`, batch reflects count | unit |

### `commit_changes`

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| happy path | populated batch | `Ok(())`, file exists with exact content, `is_committed() == true` | integration |
| empty batch | `document_count == 0` | `Err(EmptyBatch { run_id })` where run_id matches batch | unit |
| double commit | already committed | `Err(AlreadyCommitted { run_id })` where run_id matches batch | unit |
| I/O failure | output dir removed after construction | `Err(PersistenceFailed { run_id, reason })` where reason contains output dir path | integration |
| serialization failure | non-serializable batch data | `Err(SerializationFailed { reason })` where reason contains "serialize" or format name | integration |

### `is_committed`

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| pre-commit | fresh StateDb | `false` | unit |
| post-commit | committed StateDb | `true` | unit |

### `Drop`

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| uncommitted drop | mutated but not committed | no state file in output dir | integration |
| committed drop | already committed | state file remains intact | integration |

### Pipeline Integration (`run_index`)

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| full success | valid source dir with 3 .md files | state file with document_count == 3 | e2e |
| stage failure at STEP 4 | source with malformed frontmatter causing transform Err | no state file | e2e |
| empty source | dir with no .md files | `Err(...)` from discover, no state file | e2e |

---

## 9. Error Variant Exhaustiveness

Every variant in `StateError` must have at least one test that produces it and asserts its fields:

| Error Variant | Producing Scenario | Test |
|---------------|--------------------|------|
| `AlreadyCommitted { run_id }` | Call `commit_changes()` twice | B18 |
| `MutationAfterCommit { run_id }` | Call `record_file_hash` after commit | B10 |
| `MutationAfterCommit { run_id }` | Call `set_document_count` after commit | B13 |
| `MutationAfterCommit { run_id }` | Call `set_chunk_count` after commit | B15 |
| `EmptyBatch { run_id }` | Call `commit_changes()` with `document_count == 0` | B19 |
| `DuplicateFilePath { path }` | Call `record_file_hash` with same path twice | B11 |
| `PersistenceFailed { run_id, reason }` | Delete output dir before commit | B20 |
| `OutputNotAccessible { path }` | Pass non-existent dir to `new` | B02 |
| `OutputNotAccessible { path }` | Pass read-only dir to `new` | B03 |
| `OutputNotAccessible { path }` | Pass empty string to `new` | B05 |
| `OutputNotAccessible { path }` | Pass file path to `new` | B06 |
| `OutputNotAccessible { path }` | Pass dangling symlink to `new` | B07 |
| `SerializationFailed { reason }` | Force serialization to fail | B21 |
| `PreconditionViolation { detail }` | Call `StateDb::new` without OutputLock held | B08 |

**All 8 error variants covered.** 14 producing scenarios across all variants.

---

## 10. Open Questions

1. **On-disk format:** The contract is storage-backend-agnostic. Tests that verify persistence
   (B17, B24, B26, B27) need a helper function `read_state_batch(output_dir) -> StateBatch`
   that couples to the chosen format. This should be defined in the implementation bead.

2. **Empty `relative_path` / `content_hash`:** The contract specifies `DuplicateFilePath` for
   duplicates but does not define validation for empty strings. **Decision:** empty paths ARE
   accepted — `record_file_hash("", "hash")` returns `Ok(())`. The only validation is duplicate
   detection, not content validation. B16 (formerly a gap) confirms the set_chunk_count(0)
   analogous case. The fuzz target FUZZ-02 covers empty strings in its corpus.

3. **Thread safety:** The contract assumes single-threaded access within a single `run_index`
   call. If `StateDb` needs to be `Send`/`Sync`, additional tests for concurrent access
   patterns are warranted.

4. **`run_id` generation:** `RunId(String)` is constructed in `StateDb::new`. The format is
   unspecified. Tests should verify uniqueness (across two `StateDb::new` calls in the same
   test) but should NOT assert a specific format.

5. **B21 serialization failure injection:** The exact mechanism depends on the chosen
   serialization format. For JSON (serde_json), the test can construct a `StateBatch` where
   a custom `Serialize` impl returns an error, or use a test-only wrapper. The implementation
   bead must expose a test hook or the test must use a `#[cfg(test)]` constructor that injects
   a failing batch.
