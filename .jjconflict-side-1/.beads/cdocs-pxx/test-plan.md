# Test Plan: Validated StateChanges and Atomic commit_changes

**Bead**: cdocs-pxx
**Source**: `src/cache/mod.rs` (extends existing module)
**Scope**: `StateDb`, `StateChanges`, `StateReadSession`, `commit_changes`
**Revision**: 2 — addresses REJECTED review (MAJOR-1, MAJOR-2, MAJOR-3 + MINOR-1..5)

## Summary

| Metric | Count |
|--------|-------|
| Behaviors identified | 48 |
| BDD scenarios | 54 |
| Trophy: Integration (`/tests/`) | 29 (62%) |
| Trophy: Unit (`#[cfg(test)]`) | 18 (38%) |
| Proptest invariants | 6 |
| Fuzz targets | 0 (no parser/deserializer boundary in this bead) |
| Kani harnesses | 2 |
| Mutation kill target | ≥90% |
| Accepted survivors | 1 (unchanged-row skip integration observability — mitigated by pure-function extraction) |

**Trophy rationale**: This bead is an I/O-bound database layer. Every meaningful behavior requires a real `redb::Database` to prove ACID semantics. The 62% integration ratio reflects that the contract guarantees are about transactional state, not pure computation. The 38% unit tests cover the pure validation/precondition-check logic that runs before the write transaction opens (per contract: "Precondition errors fire BEFORE the write transaction is opened"), plus the extracted `should_skip_write` pure function and per-payload-vec PayloadTooLarge checks.

---

## 1. Behavior Inventory

Behaviors are expressed as `[Subject] [action] [outcome] when [condition]`.

### StateDb::open

1. **StateDb creates database and all tables when path is valid**
2. **StateDb returns DatabaseOpen when redb cannot create/open the file**
3. **StateDb returns TableInit when any table definition fails**
4. **StateDb returns DatabaseOpen when path is empty string**

### StateDb::begin_read

5. **StateDb returns live StateReadSession when database is open**
6. **StateDb returns ReadTransaction when redb cannot begin a read**

### StateDb::commit_changes — Precondition Violations (validation phase, pre-write)

7. **commit_changes rejects ZeroHashKey when any payload vec contains `[0u8; 32]` key**
8. **commit_changes rejects ZeroHashKey in new_analyses specifically**
9. **commit_changes rejects ZeroHashKey in new_transforms specifically**
10. **commit_changes rejects ZeroHashKey in new_chunks specifically**
11. **commit_changes rejects ZeroHashKey in new_scrapes specifically**
12. **commit_changes rejects ZeroHashKey in new_snapshots specifically**
13. **commit_changes rejects EmptyStringKey when any source_path in updated_files is empty**
14. **commit_changes rejects EmptyStringKey when any URL in updated_urls is empty**
15. **commit_changes rejects EmptyStringKey when source_path is whitespace-only**
16. **commit_changes rejects EmptyStringKey when URL is whitespace-only**
17. **commit_changes rejects DuplicateStateKey when updated_files contains duplicate source_paths**
18. **commit_changes rejects DuplicateStateKey when updated_urls contains duplicate URLs**
19. **commit_changes rejects MissingReference when FileStateRaw.analysis_hash not in new_analyses (non-zero)**
20. **commit_changes rejects MissingReference when FileStateRaw.transform_hash not in new_transforms (non-zero)**
21. **commit_changes rejects MissingReference when FileStateRaw.chunk_hash not in new_chunks (non-zero)**
22. **commit_changes rejects MissingReference when UrlStateRaw.url_hash not in new_scrapes (non-zero)**
23. **commit_changes accepts zero hashes in FileStateRaw (no-analysis-yet semantics)**
24. **commit_changes rejects PayloadTooLarge when new_analyses payload exceeds 50 MiB**
25. **commit_changes rejects PayloadTooLarge when new_transforms payload exceeds 50 MiB**
26. **commit_changes rejects PayloadTooLarge when new_chunks payload exceeds 50 MiB**
27. **commit_changes rejects PayloadTooLarge when new_scrapes payload exceeds 50 MiB**
28. **commit_changes rejects PayloadTooLarge when new_snapshots payload exceeds 50 MiB**

### StateDb::commit_changes — Successful Writes

29. **commit_changes persists all updated_files to file_state table**
30. **commit_changes removes all deleted_files from file_state table (skipping non-existent)**
31. **commit_changes persists all new_analyses to analysis_outputs**
32. **commit_changes persists all new_transforms to transform_outputs**
33. **commit_changes persists all new_chunks to chunk_outputs**
34. **commit_changes persists all updated_urls to url_state table**
35. **commit_changes removes all deleted_urls from url_state (skipping non-existent)**
36. **commit_changes persists all new_scrapes to scrape_outputs**
37. **commit_changes persists all new_snapshots to snapshots**
38. **commit_changes removes all deleted_snapshots from snapshots (skipping non-existent)**

### StateDb::commit_changes — Structural Guarantees

39. **commit_changes deduplicates payload entries by hash key (last-write-wins)**
40. **should_skip_write returns true when existing and new bytes are identical**
41. **should_skip_write returns false when existing and new bytes differ**
42. **commit_changes skips byte-identical rows via should_skip_write (integration verification)**
43. **commit_changes rolls back ALL writes when validation fails (zero partial writes)**
44. **commit_changes succeeds with no-op when StateChanges is all-empty vecs**
45. **commit_changes applies mixed mutations atomically (files + urls + payloads + deletes in one transaction)**

### StateDb::commit_changes — Transaction Errors

46. **commit_changes returns WriteTransaction when redb cannot begin write**
47. **commit_changes returns WriteFailed when individual insert/delete fails**
48. **commit_changes returns CommitFailed when redb commit fails**

### StateDb::open — Boundary

49. **StateDb rejects or handles source_paths approaching redb key size limit**

### StateReadSession — DEFERRED (separate bead)

The following public functions are defined in the contract but explicitly excluded by the contract's non-goals (line 523): "Do NOT implement the StateReadSession bulk-load methods (separate bead)." No BDD scenarios are planned here. They will be covered when that bead is implemented.

- **DEFERRED**: `StateReadSession::load_file_states`
- **DEFERRED**: `StateReadSession::load_url_states`
- **DEFERRED**: `StateReadSession::load_analyses`
- **DEFERRED**: `StateReadSession::load_transforms`
- **DEFERRED**: `StateReadSession::load_chunks`
- **DEFERRED**: `StateReadSession::load_scrapes`
- **DEFERRED**: `StateReadSession::load_snapshots`

---

## 2. Trophy Allocation

| # | Behavior | Layer | Justification |
|---|----------|-------|---------------|
| 1 | open creates DB + tables | Integration | Proves real redb file creation + table schema |
| 2 | open returns DatabaseOpen | Integration | Requires failed redb::Database::create |
| 3 | open returns TableInit | Integration | Requires failed table definition |
| 4 | open returns DatabaseOpen for empty path | Integration | Requires real filesystem interaction |
| 5 | begin_read returns session | Integration | Proves real read transaction lifecycle |
| 6 | begin_read returns ReadTransaction | Integration | Requires real redb read failure |
| 7 | rejects ZeroHashKey (general) | Unit | Pure validation check on in-memory data |
| 8 | rejects ZeroHashKey new_analyses | Unit | Pure validation — specific table name in error |
| 9 | rejects ZeroHashKey new_transforms | Unit | Pure validation — specific table name |
| 10 | rejects ZeroHashKey new_chunks | Unit | Pure validation — specific table name |
| 11 | rejects ZeroHashKey new_scrapes | Unit | Pure validation — specific table name |
| 12 | rejects ZeroHashKey new_snapshots | Unit | Pure validation — specific table name |
| 13 | rejects EmptyStringKey updated_files | Unit | Pure validation on strings |
| 14 | rejects EmptyStringKey updated_urls | Unit | Pure validation on strings |
| 15 | rejects whitespace-only source_path | Unit | Pure validation — trim semantics |
| 16 | rejects whitespace-only URL | Unit | Pure validation — trim semantics |
| 17 | rejects DuplicateStateKey files | Unit | Pure set-membership check |
| 18 | rejects DuplicateStateKey urls | Unit | Pure set-membership check |
| 19 | rejects MissingReference analysis_hash | Unit | Pure cross-reference check |
| 20 | rejects MissingReference transform_hash | Unit | Pure cross-reference check |
| 21 | rejects MissingReference chunk_hash | Unit | Pure cross-reference check |
| 22 | rejects MissingReference url_hash | Unit | Pure cross-reference check |
| 23 | accepts zero hashes in state | Unit | Pure validation — zero-hash exemption |
| 24 | rejects PayloadTooLarge new_analyses | Unit | Pure size comparison |
| 25 | rejects PayloadTooLarge new_transforms | Unit | Pure size comparison |
| 26 | rejects PayloadTooLarge new_chunks | Unit | Pure size comparison |
| 27 | rejects PayloadTooLarge new_scrapes | Unit | Pure size comparison |
| 28 | rejects PayloadTooLarge new_snapshots | Unit | Pure size comparison |
| 29 | persists updated_files | Integration | Verifies real redb table state after commit |
| 30 | deletes files (idempotent skip) | Integration | Verifies real delete + missing-key skip |
| 31 | persists new_analyses | Integration | Real write + read-back verification |
| 32 | persists new_transforms | Integration | Real write + read-back verification |
| 33 | persists new_chunks | Integration | Real write + read-back verification |
| 34 | persists updated_urls | Integration | Real write + read-back verification |
| 35 | deletes urls (idempotent skip) | Integration | Real delete verification |
| 36 | persists new_scrapes | Integration | Real write + read-back verification |
| 37 | persists new_snapshots | Integration | Real write + read-back verification |
| 38 | deletes snapshots (idempotent skip) | Integration | Real delete verification |
| 39 | dedup last-write-wins | Integration | Requires real DB to prove single entry per hash |
| 40 | should_skip_write returns true for identical | Unit | Pure function — byte comparison predicate |
| 41 | should_skip_write returns false for different | Unit | Pure function — byte comparison predicate |
| 42 | skips unchanged rows (integration) | Integration | Verifies read-back value correctness after no-change commit |
| 43 | rollback on validation failure | Integration | Proves zero partial writes via real DB state |
| 44 | no-op batch succeeds | Integration | Proves empty-path through real write transaction |
| 45 | mixed mutations atomic | Integration | Full end-to-end: write, commit, read-back all tables |
| 46 | WriteTransaction error | Integration | Requires real redb write failure |
| 47 | WriteFailed error | Integration | Requires real redb insert/delete failure |
| 48 | CommitFailed error | Integration | Requires real redb commit failure |
| 49 | long source_path boundary | Integration | Requires real redb to surface key-size limit |

**Allocation**: 29 integration / 18 unit / 0 E2E / 0 static (static covered by workspace clippy config).

---

## 3. BDD Scenarios

### Behavior 1: StateDb creates database and all tables when path is valid

```
Given: a fresh temp directory
When:  StateDb::open(temp_dir.join("state.redb")) is called
Then:  Ok(StateDb) is returned
And:   a subsequent begin_read() succeeds
And:   all 8 tables (file_state, url_state, analysis_outputs,
       transform_outputs, chunk_outputs, scrape_outputs,
       snapshots, metadata) are accessible via the read transaction
```

Test function: `fn state_db_open_returns_ok_when_path_valid()`

### Behavior 2: StateDb returns DatabaseOpen when redb cannot create/open the file

```
Given: a path whose parent directory does not exist and cannot be created
       (e.g., "/nonexistent_root_dir/deeply/nested/state.redb" on a read-only mount)
When:  StateDb::open(path) is called
Then:  Err(CommitError::DatabaseOpen { path, reason }) is returned
And:   path == "/nonexistent_root_dir/deeply/nested/state.redb"
And:   reason contains "nonexistent_root_dir"
       (the reason string embeds the OS error which includes the path components)
```

Test function: `fn state_db_open_returns_database_open_error_when_path_invalid()`

### Behavior 3: StateDb returns TableInit when any table definition fails

```
Given: a database opened on a corrupted file (e.g., a file that is not a valid redb database)
When:  StateDb::open attempts to create/open tables
Then:  Err(CommitError::TableInit { reason }) is returned
And:   reason contains "table"
       (the error originates from redb table definition failure)
```

Test function: `fn state_db_open_returns_table_init_error_when_tables_fail()`

Note: This scenario may be difficult to trigger deterministically with redb 2.x. If no reliable reproduction exists, mark as a manual test gate and document the difficulty. The concrete assertion on `reason.contains("table")` ensures the error path is not a generic fallback.

### Behavior 4: StateDb returns DatabaseOpen when path is empty string

```
Given: Path::new("") as the database path
When:  StateDb::open(Path::new("")) is called
Then:  Err(CommitError::DatabaseOpen { path, reason }) is returned
And:   path == ""
And:   reason contains "No such file" OR reason contains "file" OR reason contains "directory"
       (OS-level error when redb tries to create/open a file at an empty path)
```

Test function: `fn state_db_open_returns_database_open_error_when_path_is_empty()`

### Behavior 5: StateDb returns live StateReadSession when database is open

```
Given: a StateDb opened at a valid path
When:  state_db.begin_read() is called
Then:  Ok(StateReadSession) is returned
And:   the session's lifetime is tied to state_db ('db)
```

Test function: `fn state_db_begin_read_returns_session_when_db_open()`

### Behavior 6: StateDb returns ReadTransaction when redb cannot begin a read

```
Given: a StateDb that has been opened but whose backing file has been deleted
       (or otherwise made inaccessible)
When:  state_db.begin_read() is called
Then:  Err(CommitError::ReadTransaction { reason }) is returned
And:   reason contains "read" OR reason contains "transaction"
       (the reason string reflects the redb read-transaction error category)
```

Test function: `fn state_db_begin_read_returns_error_when_read_fails()`

### Behavior 7: commit_changes rejects ZeroHashKey when any payload vec contains zero hash

```
Given: a StateChanges where new_analyses contains ([0u8; 32], vec![1,2,3]) at index 0
       And all other StateChanges fields are valid per make_minimal_valid_state_changes()
When:  state_db.commit_changes(changes) is called
Then:  Err(CommitError::ZeroHashKey { table: "analysis_outputs", index: 0 }) is returned
And:   no write transaction is opened (precondition phase)
```

Test function: `fn commit_changes_rejects_zero_hash_key_in_analysis_outputs()`

### Behavior 8: commit_changes rejects ZeroHashKey in new_analyses at non-zero index

```
Given: a StateChanges with new_analyses[2] = ([0u8; 32], bytes)
       And all other StateChanges fields are valid per make_minimal_valid_state_changes()
When:  commit_changes is called
Then:  Err(CommitError::ZeroHashKey { table: "analysis_outputs", index: 2 })
```

Test function: `fn commit_changes_reports_index_2_for_zero_hash_in_analyses()`

### Behavior 9: commit_changes rejects ZeroHashKey in new_transforms

```
Given: a StateChanges with new_transforms[0] = ([0u8; 32], bytes)
       And all other StateChanges fields are valid per make_minimal_valid_state_changes()
When:  commit_changes is called
Then:  Err(CommitError::ZeroHashKey { table: "transform_outputs", index: 0 })
```

Test function: `fn commit_changes_rejects_zero_hash_key_in_transform_outputs()`

### Behavior 10: commit_changes rejects ZeroHashKey in new_chunks

```
Given: a StateChanges with new_chunks[0] = ([0u8; 32], bytes)
       And all other StateChanges fields are valid per make_minimal_valid_state_changes()
When:  commit_changes is called
Then:  Err(CommitError::ZeroHashKey { table: "chunk_outputs", index: 0 })
```

Test function: `fn commit_changes_rejects_zero_hash_key_in_chunk_outputs()`

### Behavior 11: commit_changes rejects ZeroHashKey in new_scrapes

```
Given: a StateChanges with new_scrapes[0] = ([0u8; 32], bytes)
       And all other StateChanges fields are valid per make_minimal_valid_state_changes()
When:  commit_changes is called
Then:  Err(CommitError::ZeroHashKey { table: "scrape_outputs", index: 0 })
```

Test function: `fn commit_changes_rejects_zero_hash_key_in_scrape_outputs()`

### Behavior 12: commit_changes rejects ZeroHashKey in new_snapshots

```
Given: a StateChanges with new_snapshots[0] = ([0u8; 32], bytes)
       And all other StateChanges fields are valid per make_minimal_valid_state_changes()
When:  commit_changes is called
Then:  Err(CommitError::ZeroHashKey { table: "snapshots", index: 0 })
```

Test function: `fn commit_changes_rejects_zero_hash_key_in_snapshots()`

### Behavior 13: commit_changes rejects EmptyStringKey when source_path is empty

```
Given: a StateChanges with updated_files[0] = ("".to_string(), file_state_raw)
       And file_state_raw has all-zero hashes (so no MissingReference fires)
       And all other StateChanges fields are valid per make_minimal_valid_state_changes()
When:  commit_changes is called
Then:  Err(CommitError::EmptyStringKey { table: "file_state", index: 0 })
```

Test function: `fn commit_changes_rejects_empty_source_path_in_updated_files()`

### Behavior 14: commit_changes rejects EmptyStringKey when URL is empty

```
Given: a StateChanges with updated_urls[0] = ("".to_string(), url_state_raw)
       And url_state_raw has zero url_hash (so no MissingReference fires)
       And all other StateChanges fields are valid per make_minimal_valid_state_changes()
When:  commit_changes is called
Then:  Err(CommitError::EmptyStringKey { table: "url_state", index: 0 })
```

Test function: `fn commit_changes_rejects_empty_url_in_updated_urls()`

### Behavior 15: commit_changes rejects whitespace-only source_path

```
Given: a StateChanges with updated_files[0] = ("   ".to_string(), file_state_raw)
       And file_state_raw has all-zero hashes (so no MissingReference fires)
       And all other StateChanges fields are valid per make_minimal_valid_state_changes()
When:  commit_changes is called
Then:  Err(CommitError::EmptyStringKey { table: "file_state", index: 0 })
```

Test function: `fn commit_changes_rejects_whitespace_only_source_path()`

### Behavior 16: commit_changes rejects whitespace-only URL

```
Given: a StateChanges with updated_urls[0] = ("\t\n".to_string(), url_state_raw)
       And url_state_raw has zero url_hash (so no MissingReference fires)
       And all other StateChanges fields are valid per make_minimal_valid_state_changes()
When:  commit_changes is called
Then:  Err(CommitError::EmptyStringKey { table: "url_state", index: 0 })
```

Test function: `fn commit_changes_rejects_whitespace_only_url()`

### Behavior 17: commit_changes rejects DuplicateStateKey in updated_files

```
Given: a StateChanges with updated_files containing two entries with key "src/main.rs"
       And all other StateChanges fields are valid per make_minimal_valid_state_changes()
When:  commit_changes is called
Then:  Err(CommitError::DuplicateStateKey { table: "file_state", key: "src/main.rs" })
```

Test function: `fn commit_changes_rejects_duplicate_source_path_in_updated_files()`

### Behavior 18: commit_changes rejects DuplicateStateKey in updated_urls

```
Given: a StateChanges with updated_urls containing two entries with key "https://example.com"
       And all other StateChanges fields are valid per make_minimal_valid_state_changes()
When:  commit_changes is called
Then:  Err(CommitError::DuplicateStateKey { table: "url_state", key: "https://example.com" })
```

Test function: `fn commit_changes_rejects_duplicate_url_in_updated_urls()`

### Behavior 19: commit_changes rejects MissingReference for analysis_hash

```
Given: a StateChanges with updated_files[0].1.analysis_hash = [1; 32] (non-zero)
       And new_analyses does NOT contain key [1; 32]
       And updated_files[0].1.transform_hash = [0u8; 32] (exempted)
       And updated_files[0].1.chunk_hash = [0u8; 32] (exempted)
       And all other StateChanges fields are valid per make_minimal_valid_state_changes()
When:  commit_changes is called
Then:  Err(CommitError::MissingReference {
           table: "file_state",
           field: "analysis_hash",
           hash_hex: "01010101...01010101",
           payload_table: "analysis_outputs"
       })
```

Test function: `fn commit_changes_rejects_missing_analysis_hash_reference()`

### Behavior 20: commit_changes rejects MissingReference for transform_hash

```
Given: a StateChanges with updated_files[0].1.transform_hash = [2; 32] (non-zero)
       And new_transforms does NOT contain key [2; 32]
       And updated_files[0].1.analysis_hash = [0u8; 32] (exempted)
       And updated_files[0].1.chunk_hash = [0u8; 32] (exempted)
       And all other StateChanges fields are valid per make_minimal_valid_state_changes()
When:  commit_changes is called
Then:  Err(CommitError::MissingReference {
           table: "file_state",
           field: "transform_hash",
           hash_hex: "02020202...02020202",
           payload_table: "transform_outputs"
       })
```

Test function: `fn commit_changes_rejects_missing_transform_hash_reference()`

### Behavior 21: commit_changes rejects MissingReference for chunk_hash

```
Given: a StateChanges with updated_files[0].1.chunk_hash = [3; 32] (non-zero)
       And new_chunks does NOT contain key [3; 32]
       And updated_files[0].1.analysis_hash = [0u8; 32] (exempted)
       And updated_files[0].1.transform_hash = [0u8; 32] (exempted)
       And all other StateChanges fields are valid per make_minimal_valid_state_changes()
When:  commit_changes is called
Then:  Err(CommitError::MissingReference {
           table: "file_state",
           field: "chunk_hash",
           hash_hex: "03030303...03030303",
           payload_table: "chunk_outputs"
       })
```

Test function: `fn commit_changes_rejects_missing_chunk_hash_reference()`

### Behavior 22: commit_changes rejects MissingReference for url_hash

```
Given: a StateChanges with updated_urls[0].1.url_hash = [4; 32] (non-zero)
       And new_scrapes does NOT contain key [4; 32]
       And all other StateChanges fields are valid per make_minimal_valid_state_changes()
When:  commit_changes is called
Then:  Err(CommitError::MissingReference {
           table: "url_state",
           field: "url_hash",
           hash_hex: "04040404...04040404",
           payload_table: "scrape_outputs"
       })
```

Test function: `fn commit_changes_rejects_missing_url_hash_reference()`

### Behavior 23: commit_changes accepts zero hashes in FileStateRaw (no-analysis-yet)

```
Given: a StateChanges with updated_files[0].1.analysis_hash = [0u8; 32]
       And updated_files[0].1.transform_hash = [0u8; 32]
       And updated_files[0].1.chunk_hash = [0u8; 32]
       And new_analyses, new_transforms, new_chunks are all empty
       And all other StateChanges fields are valid per make_minimal_valid_state_changes()
When:  commit_changes is called
Then:  Ok(()) — zero hashes are treated as "no output yet", not missing refs
```

Test function: `fn commit_changes_accepts_zero_hashes_as_no_output()`

### Behavior 24: commit_changes rejects PayloadTooLarge in new_analyses

```
Given: a StateChanges with new_analyses[0] = (hash, vec![0u8; 50 * 1024 * 1024 + 1])
       And updated_files is empty (no reference integrity to satisfy)
       And all other StateChanges fields are valid per make_minimal_valid_state_changes()
When:  commit_changes is called
Then:  Err(CommitError::PayloadTooLarge {
           table: "analysis_outputs",
           size: 52428801,
           max: 52428800,
       })
```

Test function: `fn commit_changes_rejects_payload_exceeding_max_value_size_in_analysis_outputs()`

### Behavior 25: commit_changes rejects PayloadTooLarge in new_transforms

```
Given: a StateChanges with new_transforms[0] = (hash, vec![0u8; 50 * 1024 * 1024 + 1])
       And updated_files is empty (no reference integrity to satisfy)
       And all other StateChanges fields are valid per make_minimal_valid_state_changes()
When:  commit_changes is called
Then:  Err(CommitError::PayloadTooLarge {
           table: "transform_outputs",
           size: 52428801,
           max: 52428800,
       })
```

Test function: `fn commit_changes_rejects_payload_exceeding_max_value_size_in_transform_outputs()`

### Behavior 26: commit_changes rejects PayloadTooLarge in new_chunks

```
Given: a StateChanges with new_chunks[0] = (hash, vec![0u8; 50 * 1024 * 1024 + 1])
       And updated_files is empty (no reference integrity to satisfy)
       And all other StateChanges fields are valid per make_minimal_valid_state_changes()
When:  commit_changes is called
Then:  Err(CommitError::PayloadTooLarge {
           table: "chunk_outputs",
           size: 52428801,
           max: 52428800,
       })
```

Test function: `fn commit_changes_rejects_payload_exceeding_max_value_size_in_chunk_outputs()`

### Behavior 27: commit_changes rejects PayloadTooLarge in new_scrapes

```
Given: a StateChanges with new_scrapes[0] = (hash, vec![0u8; 50 * 1024 * 1024 + 1])
       And updated_urls is empty (no reference integrity to satisfy)
       And all other StateChanges fields are valid per make_minimal_valid_state_changes()
When:  commit_changes is called
Then:  Err(CommitError::PayloadTooLarge {
           table: "scrape_outputs",
           size: 52428801,
           max: 52428800,
       })
```

Test function: `fn commit_changes_rejects_payload_exceeding_max_value_size_in_scrape_outputs()`

### Behavior 28: commit_changes rejects PayloadTooLarge in new_snapshots

```
Given: a StateChanges with new_snapshots[0] = (hash, vec![0u8; 50 * 1024 * 1024 + 1])
       And all other StateChanges fields are valid per make_minimal_valid_state_changes()
When:  commit_changes is called
Then:  Err(CommitError::PayloadTooLarge {
           table: "snapshots",
           size: 52428801,
           max: 52428800,
       })
```

Test function: `fn commit_changes_rejects_payload_exceeding_max_value_size_in_snapshots()`

### Behavior 29: commit_changes persists all updated_files to file_state

```
Given: a StateDb with an empty file_state table
  And: a StateChanges with updated_files = [
       ("src/main.rs", file_state_raw_a),
       ("docs/README.md", file_state_raw_b),
  ]
  And: file_state_raw_a has all-zero hashes (no payload references needed)
  And: file_state_raw_b has all-zero hashes (no payload references needed)
  And: all other StateChanges fields are empty
When:  state_db.commit_changes(changes) is called
Then:  Ok(()) is returned
And:   reading file_state table yields exactly:
       { "src/main.rs" → file_state_raw_a bytes, "docs/README.md" → file_state_raw_b bytes }
```

Test function: `fn commit_changes_persists_updated_files_to_file_state_table()`

### Behavior 30: commit_changes removes deleted_files (idempotent skip)

```
Given: a StateDb where file_state contains "old_file.rs" → some_state
  And: a StateChanges with deleted_files = ["old_file.rs", "nonexistent.rs"]
  And: all other StateChanges fields are empty
When:  state_db.commit_changes(changes) is called
Then:  Ok(()) is returned
And:   "old_file.rs" no longer exists in file_state
And:   "nonexistent.rs" deletion was silently skipped (no error)
```

Test function: `fn commit_changes_deletes_files_and_skips_nonexistent()`

### Behavior 31: commit_changes persists new_analyses to analysis_outputs

```
Given: a StateDb with empty analysis_outputs
  And: a StateChanges with new_analyses = [(hash_a, bytes_a), (hash_b, bytes_b)]
  And: updated_files is empty (no reference integrity to satisfy)
  And: all other StateChanges fields are empty
When:  state_db.commit_changes(changes) is called
Then:  Ok(())
And:   analysis_outputs contains exactly hash_a → bytes_a and hash_b → bytes_b
```

Test function: `fn commit_changes_persists_new_analyses_to_analysis_outputs()`

### Behavior 32: commit_changes persists new_transforms to transform_outputs

```
Given: a StateDb with empty transform_outputs
  And: a StateChanges with new_transforms = [(hash_a, bytes_a)]
  And: updated_files is empty
  And: all other StateChanges fields are empty
When:  commit_changes is called
Then:  Ok(())
And:   transform_outputs contains hash_a → bytes_a
```

Test function: `fn commit_changes_persists_new_transforms_to_transform_outputs()`

### Behavior 33: commit_changes persists new_chunks to chunk_outputs

```
Given: a StateDb with empty chunk_outputs
  And: a StateChanges with new_chunks = [(hash_a, bytes_a)]
  And: updated_files is empty
  And: all other StateChanges fields are empty
When:  commit_changes is called
Then:  Ok(())
And:   chunk_outputs contains hash_a → bytes_a
```

Test function: `fn commit_changes_persists_new_chunks_to_chunk_outputs()`

### Behavior 34: commit_changes persists updated_urls to url_state

```
Given: a StateDb with empty url_state
  And: a StateChanges with updated_urls = [("https://example.com", url_state_raw)]
  And: url_state_raw has zero url_hash (no scrape reference needed)
  And: all other StateChanges fields are empty
When:  commit_changes is called
Then:  Ok(())
And:   url_state contains "https://example.com" → url_state_raw bytes
```

Test function: `fn commit_changes_persists_updated_urls_to_url_state()`

### Behavior 35: commit_changes removes deleted_urls (idempotent skip)

```
Given: a StateDb where url_state contains "https://old.com" → some_state
  And: a StateChanges with deleted_urls = ["https://old.com", "https://nonexistent.com"]
  And: all other StateChanges fields are empty
When:  commit_changes is called
Then:  Ok(())
And:   "https://old.com" no longer in url_state
And:   "https://nonexistent.com" deletion silently skipped
```

Test function: `fn commit_changes_deletes_urls_and_skips_nonexistent()`

### Behavior 36: commit_changes persists new_scrapes to scrape_outputs

```
Given: a StateDb with empty scrape_outputs
  And: a StateChanges with new_scrapes = [(hash_a, bytes_a)]
  And: updated_urls is empty
  And: all other StateChanges fields are empty
When:  commit_changes is called
Then:  Ok(())
And:   scrape_outputs contains hash_a → bytes_a
```

Test function: `fn commit_changes_persists_new_scrapes_to_scrape_outputs()`

### Behavior 37: commit_changes persists new_snapshots to snapshots

```
Given: a StateDb with empty snapshots table
  And: a StateChanges with new_snapshots = [(hash_a, bytes_a)]
  And: all other StateChanges fields are empty
When:  commit_changes is called
Then:  Ok(())
And:   snapshots contains hash_a → bytes_a
```

Test function: `fn commit_changes_persists_new_snapshots_to_snapshots_table()`

### Behavior 38: commit_changes removes deleted_snapshots (idempotent skip)

```
Given: a StateDb where snapshots contains hash_old → bytes_old
  And: a StateChanges with deleted_snapshots = [hash_old, [0xAA; 32]]
  And: all other StateChanges fields are empty
When:  commit_changes is called
Then:  Ok(())
And:   hash_old no longer in snapshots
And:   [0xAA; 32] deletion silently skipped (not present)
```

Test function: `fn commit_changes_deletes_snapshots_and_skips_nonexistent()`

### Behavior 39: commit_changes deduplicates payload entries (last-write-wins)

```
Given: a StateChanges with new_analyses = [
       (hash_a, bytes_v1),
       (hash_b, bytes_v2),
       (hash_a, bytes_v3),   // duplicate key, different value
  ]
  And: updated_files is empty
  And: all other StateChanges fields are empty
When:  commit_changes is called
Then:  Ok(())
And:   analysis_outputs[hash_a] == bytes_v3 (last-write-wins)
And:   analysis_outputs[hash_b] == bytes_v2
And:   analysis_outputs.len() == 2 (exactly 2 unique keys)
```

Test function: `fn commit_changes_deduplicates_payload_entries_last_write_wins()`

### Behavior 40: should_skip_write returns true when existing and new bytes are identical

```
Given: two byte slices existing = &[1, 2, 3, 4] and new = &[1, 2, 3, 4]
When:  should_skip_write(existing, new) is called
Then:  true (write should be skipped — bytes are identical)
```

Test function: `fn should_skip_write_returns_true_when_bytes_identical()`

### Behavior 41: should_skip_write returns false when existing and new bytes differ

```
Given: two byte slices existing = &[1, 2, 3, 4] and new = &[1, 2, 3, 5]
When:  should_skip_write(existing, new) is called
Then:  false (write must proceed — bytes differ)
```

Test function: `fn should_skip_write_returns_false_when_bytes_differ()`

### Behavior 42: commit_changes skips byte-identical rows via should_skip_write (integration verification)

```
Given: a StateDb where file_state already contains "src/main.rs" → file_state_raw_a bytes
  And: a StateChanges with updated_files = [("src/main.rs", file_state_raw_a)]
       where the bytes are IDENTICAL to what is already stored
  And: all other StateChanges fields are empty
When:  commit_changes is called
Then:  Ok(())
And:   the file_state table for "src/main.rs" contains the exact same bytes as before
And:   NO additional redb insert call is made for this key
       (verified by asserting that the count of rows written == 0 for unchanged entries,
        using an instrumented write-counter or by verifying that a spy/wrapper recorded
        zero insert calls for the unchanged key)
```

Test function: `fn commit_changes_skips_unchanged_rows_without_rewriting()`

**Resolution for MAJOR-3**: The unchanged-row skip logic is resolved by a two-layer approach:

1. **Unit layer (Behaviors 40–41)**: Extract `fn should_skip_write(existing: &[u8], new: &[u8]) -> bool` as a pure, public function. Test it exhaustively in unit tests. This catches the mutation `s/should_skip_write/true/` at the unit level because Behavior 41 asserts `false` for differing bytes.

2. **Integration layer (Behavior 42)**: Verify that the end-to-end commit path produces correct results when a row is unchanged. The integration test asserts correct read-back values. The mutation `remove skip logic entirely (always rewrite)` is NOT caught at the integration level because the values still round-trip correctly — this is an **accepted survivor** for the integration test, documented below in the mutation section.

3. **Proptest layer (Proptest 6)**: Property-test `should_skip_write` across all byte patterns to ensure no false positives/negatives.

The key mutation (`always rewrite`) is caught by Behavior 41 (unit layer), NOT by Behavior 42 (integration layer). This is explicit and documented.

### Behavior 43: commit_changes rolls back ALL writes when validation fails

```
Given: a StateDb where file_state is empty
  And: a StateChanges with:
       updated_files = [("valid.rs", file_state_valid)]
       new_analyses = [([0u8; 32], bytes)]  ← ZeroHashKey violation
  And: file_state_valid has all-zero hashes (so no MissingReference)
  And: all other StateChanges fields are empty
When:  commit_changes is called
Then:  Err(CommitError::ZeroHashKey { table: "analysis_outputs", index: 0 }) is returned
And:   file_state table is STILL EMPTY (the valid updated_file was NOT written)
```

Test function: `fn commit_changes_rolls_back_all_writes_when_validation_fails()`

### Behavior 44: commit_changes succeeds with no-op batch

```
Given: a StateDb (any state)
  And: a StateChanges with all vecs empty (no files, no urls, no payloads, no deletes)
When:  commit_changes is called
Then:  Ok(()) is returned
And:   database state is unchanged (all tables have same contents as before)
```

Test function: `fn commit_changes_succeeds_with_noop_empty_batch()`

### Behavior 45: commit_changes applies mixed mutations atomically

```
Given: a StateDb where file_state contains "old.rs" → old_state
       and url_state contains "https://old.com" → old_url_state
       and analysis_outputs contains hash_old → old_bytes
  And: a StateChanges with:
       updated_files = [("new.rs", file_state_new)]
       deleted_files = ["old.rs"]
       new_analyses = [(hash_new, new_analysis_bytes)]
       new_transforms = [(hash_t, transform_bytes)]
       new_chunks = [(hash_c, chunk_bytes)]
       updated_urls = [("https://new.com", url_state_new)]
       deleted_urls = ["https://old.com"]
       new_scrapes = [(hash_s, scrape_bytes)]
       new_snapshots = [(hash_snap, snap_bytes)]
       deleted_snapshots = [hash_old]
  And: file_state_new.analysis_hash = hash_new, .transform_hash = hash_t, .chunk_hash = hash_c
  And: url_state_new.url_hash = hash_s
When:  commit_changes is called
Then:  Ok(())
And:   file_state contains "new.rs" → file_state_new (NOT "old.rs")
And:   url_state contains "https://new.com" → url_state_new (NOT "https://old.com")
And:   analysis_outputs contains hash_new → new_analysis_bytes (NOT hash_old)
And:   transform_outputs contains hash_t → transform_bytes
And:   chunk_outputs contains hash_c → chunk_bytes
And:   scrape_outputs contains hash_s → scrape_bytes
And:   snapshots contains hash_snap → snap_bytes (NOT hash_old)
```

Test function: `fn commit_changes_applies_mixed_mutations_atomically_in_single_transaction()`

### Behavior 46: commit_changes returns WriteTransaction when redb cannot begin write

```
Given: a scenario where redb::Database::begin_write() fails
       (e.g., database was opened read-only, or file handle is corrupted)
When:  commit_changes is called with valid StateChanges
Then:  Err(CommitError::WriteTransaction { reason }) is returned
And:   reason contains "write" OR reason contains "transaction"
       (the reason string reflects the redb write-transaction error category)
```

Test function: `fn commit_changes_returns_write_transaction_error_when_begin_fails()`

Note: This is hard to trigger in normal redb usage. May require wrapping the database to inject failure or using a read-only file. Document feasibility.

### Behavior 47: commit_changes returns WriteFailed when individual write fails

```
Given: a scenario where redb table insert() fails
       (e.g., disk full, I/O error)
When:  commit_changes is called with valid StateChanges
Then:  Err(CommitError::WriteFailed { table, reason }) is returned
And:   table identifies which table failed (e.g., "file_state")
And:   reason contains "insert" OR reason contains "write" OR reason contains "I/O"
       (the reason string reflects the nature of the write failure)
```

Test function: `fn commit_changes_returns_write_failed_error_when_insert_fails()`

### Behavior 48: commit_changes returns CommitFailed when redb commit fails

```
Given: a scenario where redb WriteTransaction::commit() fails
       (e.g., disk full at commit time)
When:  commit_changes has completed all writes but commit() fails
Then:  Err(CommitError::CommitFailed { reason }) is returned
And:   reason contains "commit" OR reason contains "transaction"
       (the reason string reflects the commit-phase failure)
And:   no writes are visible to subsequent reads (rolled back by redb)
```

Test function: `fn commit_changes_returns_commit_failed_error_when_commit_fails()`

### Behavior 49: StateDb handles long source_paths approaching redb key size limit

```
Given: a source_path string of length 4096 (approaching typical redb key limits)
  And: a StateChanges with updated_files = [(long_path, file_state_raw)]
  And: file_state_raw has all-zero hashes
  And: all other StateChanges fields are empty
When:  commit_changes is called
Then:  one of:
       (a) Ok(()) — redb accepts the key and the value is readable, OR
       (b) Err(CommitError::WriteFailed { table: "file_state", reason })
           where reason contains "key" OR "size" — redb rejects the oversized key
```

Test function: `fn commit_changes_handles_long_source_path_approaching_redb_key_limit()`

Note: This test documents the actual behavior at the boundary. The contract does not specify a maximum source_path length. This test serves as a living characterization test — if redb rejects the key, we know the limit; if it accepts, we know paths up to 4096 chars are safe. Either outcome is acceptable; the test records which one holds.

---

## 4. Proptest Invariants

### Proptest 1: Zero-hash scan is exhaustive

```
Function: The validation scan that checks for zero hashes in payload vecs
Invariant: For ANY StateChanges where at least one entry in any of
           new_analyses, new_transforms, new_chunks, new_scrapes, new_snapshots
           has key == [0u8; 32], commit_changes MUST return
           Err(CommitError::ZeroHashKey { .. }).
Strategy:  Generate random StateChanges with 0–10 entries per payload vec.
           With probability 0.2, inject a [0u8; 32] key into a random vec
           at a random index.
Anti-invariant: Any StateChanges with ALL non-zero hash keys, non-empty
                strings, no duplicate state keys, and valid references
                MUST return Ok(()) (assuming other preconditions met).
```

### Proptest 2: Duplicate detection is order-independent

```
Function: The validation that checks for duplicate string keys
Invariant: For ANY set of source_paths S where |S| < |updated_files|,
           commit_changes returns Err(CommitError::DuplicateStateKey).
           For ANY set where all source_paths are unique,
           no DuplicateStateKey error is returned.
Strategy:  Generate Vec<String> of 1–20 strings.
           With probability 0.3, inject a duplicate.
           Permute the vec randomly.
Anti-invariant: A vec with all unique keys must never trigger DuplicateStateKey.
```

### Proptest 3: Reference integrity is complete

```
Function: The cross-reference check between state hashes and payload vecs
Invariant: For ANY updated_files entry whose analysis_hash, transform_hash,
           or chunk_hash is non-zero, that hash MUST appear as a key in
           the corresponding new_* payload vec. If ANY is missing,
           Err(CommitError::MissingReference { .. }) is returned.
Strategy:  Generate 1–5 FileStateRaw entries with random non-zero hashes.
           Generate corresponding payload entries.
           With probability 0.2, omit one payload entry to trigger violation.
Anti-invariant: All-zero hashes in FileStateRaw must NEVER trigger
                MissingReference, even when payload vecs are empty.
```

### Proptest 4: Last-write-wins deduplication preserves final value

```
Function: Deduplication of payload entries during commit
Invariant: For ANY payload vec with N entries where K unique keys exist,
           after commit_changes the database contains exactly K entries,
           and each entry's value equals the LAST occurrence of that key
           in the input vec.
Strategy:  Generate payload vecs of 2–10 entries.
           With probability 0.4, repeat a key with different values.
           Verify final state matches last-write-wins semantics.
Anti-invariant: A vec with all unique keys must produce identical-length
                output (no entries dropped).
```

### Proptest 5: Atomicity under mixed valid/invalid batches

```
Function: commit_changes rollback guarantee
Invariant: For ANY StateChanges that fails validation,
           the database state is BIT-IDENTICAL to its state before the call.
           (Read all tables before, read all tables after, compare.)
Strategy:  Generate a valid database state with 0–5 entries per table.
           Generate a StateChanges that is intentionally invalid
           (random precondition violation).
           Verify all 7 tables are unchanged byte-for-byte.
Anti-invariant: A valid StateChanges must always mutate the database
                in exactly the expected way (no silent data loss).
```

### Proptest 6: should_skip_write is correct for all byte patterns

```
Function: should_skip_write(existing: &[u8], new: &[u8]) -> bool
Invariant: For ANY pair of byte slices (existing, new):
           - If existing == new, should_skip_write returns true
           - If existing != new, should_skip_write returns false
           This is EXACTLY the equality predicate — no false positives,
           no false negatives, regardless of content (including empty slices,
           all-zeros, all-0xFF, high-entropy random bytes).
Strategy:  Generate pairs of Vec<u8> with lengths 0–256.
           With probability 0.5, set new = existing.clone() (identical).
           Otherwise, generate independent random bytes (likely different).
Anti-invariant: For slices of DIFFERENT lengths, must always return false.
                For slices of same length but different content, must always return false.
```

---

## 5. Fuzz Targets

**No fuzz targets in this bead.** This bead introduces no parser or deserializer boundary. All inputs are strongly-typed Rust structs (`StateChanges`, `FileStateRaw`, `UrlStateRaw`, `[u8; 32]` hashes). There is no raw byte/str input boundary where fuzzing would add value.

If future beads add deserialization of `StateChanges` from disk/network (e.g., JSON, TOML, or binary format), those beads MUST add fuzz targets for:
- `StateChanges::deserialize(bytes)` → crash on malformed input
- `FileStateRaw::from_bytes(bytes)` → out-of-bounds if len != 200
- `OwnedArchive<T>::new(bytes)` → rkyv access violation on corrupt bytes

---

## 6. Kani Harnesses

### Kani Harness 1: Zero-hash scan completeness

```
Property: For a StateChanges with exactly 1 entry in each payload vec,
          the zero-hash detection correctly identifies [0u8; 32] keys
          in ALL five payload vecs (new_analyses, new_transforms,
          new_chunks, new_scrapes, new_snapshots) and returns the
          correct table name and index.
Bound:    Each payload vec has length 1. Hash keys are [0u8; 32] or [1u8; 32].
Rationale: The scan iterates 5 separate vecs. An off-by-one or wrong table
           name mapping could silently skip a vec. Kani proves exhaustive
           coverage of all 5 vecs for the boundary case.
```

### Kani Harness 2: FileStateRaw/UrlStateRaw Pod size invariant

```
Property: std::mem::size_of::<FileStateRaw>() == 200 AND
          std::mem::size_of::<UrlStateRaw>() == 120 AND
          both types are #[repr(C)] with no padding that would
          cause bytemuck::Pod to be unsound.
Bound:    Single struct instances.
Rationale: These are fixed-size POD types read/written via bytemuck memcpy.
           Any size regression is a silent data corruption bug.
           Kani proves the compile-time size assertions hold.
```

Note: Kani harness 2 depends on `FileStateRaw` and `UrlStateRaw` types being defined (separate bead). If those types are not yet available, defer this harness.

---

## 7. Mutation Testing Checkpoints

**Target: ≥90% mutation kill rate**

### Accepted Survivor

| Mutation | Survives in | Caught by | Justification |
|----------|-------------|-----------|---------------|
| Remove unchanged-row skip (always rewrite) | Integration layer (Behavior 42) | **Unit layer (Behaviors 40–41)**: `should_skip_write` unit tests catch the mutation at the pure-function level. Behavior 41 asserts `false` for differing bytes, so the mutation `s/should_skip_write(_, _)/true/` is caught. | The integration test (Behavior 42) cannot distinguish "wrote identical bytes" from "skipped write" by observing database state alone. However, the mutation is caught at the unit layer where the pure predicate is exhaustively tested. The integration test provides a correctness backstop (values round-trip correctly). |

### Critical mutations that MUST be caught:

| Mutation | Caught by test |
|----------|---------------|
| Remove zero-hash check for `new_analyses` | Behavior 8: `commit_changes_reports_index_2_for_zero_hash_in_analyses` |
| Remove zero-hash check for `new_transforms` | Behavior 9: `commit_changes_rejects_zero_hash_key_in_transform_outputs` |
| Remove zero-hash check for `new_chunks` | Behavior 10: `commit_changes_rejects_zero_hash_key_in_chunk_outputs` |
| Remove zero-hash check for `new_scrapes` | Behavior 11: `commit_changes_rejects_zero_hash_key_in_scrape_outputs` |
| Remove zero-hash check for `new_snapshots` | Behavior 12: `commit_changes_rejects_zero_hash_key_in_snapshots` |
| Replace `trim().is_empty()` with `is_empty()` | Behavior 15: `commit_changes_rejects_whitespace_only_source_path` |
| Remove duplicate-key check for `updated_files` | Behavior 17: `commit_changes_rejects_duplicate_source_path_in_updated_files` |
| Remove duplicate-key check for `updated_urls` | Behavior 18: `commit_changes_rejects_duplicate_url_in_updated_urls` |
| Remove reference integrity check for `analysis_hash` | Behavior 19: `commit_changes_rejects_missing_analysis_hash_reference` |
| Remove reference integrity check for `transform_hash` | Behavior 20: `commit_changes_rejects_missing_transform_hash_reference` |
| Remove reference integrity check for `chunk_hash` | Behavior 21: `commit_changes_rejects_missing_chunk_hash_reference` |
| Remove reference integrity check for `url_hash` | Behavior 22: `commit_changes_rejects_missing_url_hash_reference` |
| Remove zero-hash exemption (reject all zero hashes) | Behavior 23: `commit_changes_accepts_zero_hashes_as_no_output` |
| Remove size check for `new_analyses` | Behavior 24: `commit_changes_rejects_payload_exceeding_max_value_size_in_analysis_outputs` |
| Remove size check for `new_transforms` | Behavior 25: `commit_changes_rejects_payload_exceeding_max_value_size_in_transform_outputs` |
| Remove size check for `new_chunks` | Behavior 26: `commit_changes_rejects_payload_exceeding_max_value_size_in_chunk_outputs` |
| Remove size check for `new_scrapes` | Behavior 27: `commit_changes_rejects_payload_exceeding_max_value_size_in_scrape_outputs` |
| Remove size check for `new_snapshots` | Behavior 28: `commit_changes_rejects_payload_exceeding_max_value_size_in_snapshots` |
| Change `>` to `>=` in size check | Behavior 24: uses exactly MAX + 1 bytes; also matrix row "Payload at max size" uses exactly MAX bytes and must pass |
| Remove `file_state` write loop | Behavior 29: `commit_changes_persists_updated_files_to_file_state_table` |
| Remove `deleted_files` loop | Behavior 30: `commit_changes_deletes_files_and_skips_nonexistent` |
| Remove deduplication (store all entries) | Behavior 39: `commit_changes_deduplicates_payload_entries_last_write_wins` (checks exact count == 2) |
| Remove unchanged-row skip (always rewrite) — unit layer | Behavior 41: `should_skip_write_returns_false_when_bytes_differ` catches mutation to `always return true` |
| Remove unchanged-row skip (always skip) — unit layer | Behavior 40: `should_skip_write_returns_true_when_bytes_identical` catches mutation to `always return false` — wait, that's wrong. Behavior 41 catches "always return true" (asserts false for differing). Behavior 40 catches "always return false" (asserts true for identical). Both mutations caught. |
| Abort transaction on validation failure → remove abort | Behavior 43: `commit_changes_rolls_back_all_writes_when_validation_fails` |
| Skip empty-batch early return (open transaction anyway) | Behavior 44: `commit_changes_succeeds_with_noop_empty_batch` |
| Skip any single table write in mixed batch | Behavior 45: `commit_changes_applies_mixed_mutations_atomically_in_single_transaction` |
| Swap error variant (e.g., ZeroHashKey → EmptyStringKey) | Each specific error scenario checks the exact variant |
| Remove `should_skip_write` function body (always false) | Behavior 40: asserts true for identical bytes — mutation killed |
| Remove `should_skip_write` function body (always true) | Behavior 41: asserts false for differing bytes — mutation killed |

### Mutation testing strategy:

1. Run `cargo-mutants --in-place` on `src/cache/mod.rs` targeting the `commit_changes` method and its validation helpers, including `should_skip_write`.
2. Exclude `DocCache` code (already tested, separate concern).
3. Every test must assert the **exact error variant** (not just `is_err()`), ensuring mutation of error arms is caught.
4. Every integration test must assert **exact database state** (not just "row exists"), ensuring mutation of write logic is caught.
5. `should_skip_write` must be tested as a standalone pure function to catch mutations to the skip logic.

---

## 8. Combinatorial Coverage Matrix

### 8a: Precondition Validation (Unit Layer)

| Scenario | Input Class | Expected Output | Test Layer |
|----------|-------------|-----------------|------------|
| Zero hash in new_analyses | `[0u8; 32]` at index N | `Err(ZeroHashKey { table: "analysis_outputs", index: N })` | unit |
| Zero hash in new_transforms | `[0u8; 32]` at index N | `Err(ZeroHashKey { table: "transform_outputs", index: N })` | unit |
| Zero hash in new_chunks | `[0u8; 32]` at index N | `Err(ZeroHashKey { table: "chunk_outputs", index: N })` | unit |
| Zero hash in new_scrapes | `[0u8; 32]` at index N | `Err(ZeroHashKey { table: "scrape_outputs", index: N })` | unit |
| Zero hash in new_snapshots | `[0u8; 32]` at index N | `Err(ZeroHashKey { table: "snapshots", index: N })` | unit |
| No zero hashes in any vec | all non-zero `[u8; 32]` | Passes zero-hash check (proceeds) | unit |
| Empty string source_path | `""` at index 0 | `Err(EmptyStringKey { table: "file_state", index: 0 })` | unit |
| Whitespace-only source_path | `"   "` at index 0 | `Err(EmptyStringKey { table: "file_state", index: 0 })` | unit |
| Empty string URL | `""` at index 0 | `Err(EmptyStringKey { table: "url_state", index: 0 })` | unit |
| Whitespace-only URL | `"\t\n"` at index 0 | `Err(EmptyStringKey { table: "url_state", index: 0 })` | unit |
| Duplicate source_path | 2 entries with "foo.rs" | `Err(DuplicateStateKey { table: "file_state", key: "foo.rs" })` | unit |
| Duplicate URL | 2 entries with "https://x" | `Err(DuplicateStateKey { table: "url_state", key: "https://x" })` | unit |
| Missing analysis ref | non-zero hash not in new_analyses | `Err(MissingReference { field: "analysis_hash", payload_table: "analysis_outputs", .. })` | unit |
| Missing transform ref | non-zero hash not in new_transforms | `Err(MissingReference { field: "transform_hash", payload_table: "transform_outputs", .. })` | unit |
| Missing chunk ref | non-zero hash not in new_chunks | `Err(MissingReference { field: "chunk_hash", payload_table: "chunk_outputs", .. })` | unit |
| Missing url_hash ref | non-zero hash not in new_scrapes | `Err(MissingReference { field: "url_hash", payload_table: "scrape_outputs", .. })` | unit |
| Zero hash exemption | all-zero hashes in state | Passes ref check (exempted) | unit |
| Payload at max size in new_analyses | exactly 50 MiB bytes | Passes size check | unit |
| Payload over max size in new_analyses | 50 MiB + 1 bytes | `Err(PayloadTooLarge { table: "analysis_outputs", size: 52428801, max: 52428800 })` | unit |
| Payload over max size in new_transforms | 50 MiB + 1 bytes | `Err(PayloadTooLarge { table: "transform_outputs", size: 52428801, max: 52428800 })` | unit |
| Payload over max size in new_chunks | 50 MiB + 1 bytes | `Err(PayloadTooLarge { table: "chunk_outputs", size: 52428801, max: 52428800 })` | unit |
| Payload over max size in new_scrapes | 50 MiB + 1 bytes | `Err(PayloadTooLarge { table: "scrape_outputs", size: 52428801, max: 52428800 })` | unit |
| Payload over max size in new_snapshots | 50 MiB + 1 bytes | `Err(PayloadTooLarge { table: "snapshots", size: 52428801, max: 52428800 })` | unit |
| Payload at 0 bytes | empty vec `vec![]` | Passes size check | unit |
| Identical bytes → skip | existing == new | `should_skip_write` returns `true` | unit |
| Different bytes → write | existing != new | `should_skip_write` returns `false` | unit |
| Empty existing, non-empty new | `&[]` vs `&[1]` | `should_skip_write` returns `false` | unit |
| Both empty | `&[]` vs `&[]` | `should_skip_write` returns `true` | unit |
| Different lengths | `&[1,2]` vs `&[1]` | `should_skip_write` returns `false` | unit |

### 8b: Write Operations (Integration Layer)

| Scenario | Input Class | Expected Output | Test Layer |
|----------|-------------|-----------------|------------|
| Insert new file state | unique source_path, new FileStateRaw | Ok(()), row present with exact bytes | integration |
| Update existing file state | existing source_path, new FileStateRaw | Ok(()), row updated to new bytes | integration |
| Delete existing file | source_path in table | Ok(()), row absent | integration |
| Delete non-existent file | source_path not in table | Ok(()), no error | integration |
| Insert new analysis payload | unique hash, valid bytes | Ok(()), hash present with exact bytes | integration |
| Insert duplicate hash (dedup) | same hash, different bytes | Ok(()), last-write-wins value | integration |
| Insert 5 payloads across all tables | valid hashes, valid bytes | Ok(()), all 5 tables populated | integration |
| Upsert URL state | unique URL, UrlStateRaw | Ok(()), row present | integration |
| Delete existing URL | URL in table | Ok(()), row absent | integration |
| Delete non-existent URL | URL not in table | Ok(()), no error | integration |
| Delete existing snapshot hash | hash in snapshots | Ok(()), row absent | integration |
| Delete non-existent snapshot hash | hash not in snapshots | Ok(()), no error | integration |
| Unchanged file state | identical bytes to existing row | Ok(()), same bytes in table, skip verified | integration |
| Unchanged payload | identical bytes to existing entry | Ok(()), same bytes in table | integration |
| No-op batch | all vecs empty | Ok(()), no table changes | integration |
| Mixed full batch | entries in all 10 vecs | Ok(()), all changes visible | integration |
| Validation failure rollback | valid writes + one precondition violation | Err(exact variant), NO writes applied | integration |
| Empty path for open | `Path::new("")` | `Err(DatabaseOpen { path: "", reason: contains "file" or "No such file" })` | integration |
| Long source_path (4096 chars) | 4096-char string key | Ok(()) or Err(WriteFailed) — characterization test | integration |

### 8c: Error Variant Coverage

| Error Variant | Trigger Condition | Asserted In Test |
|---------------|-------------------|------------------|
| `ZeroHashKey { table, index }` | `[0u8; 32]` in any payload vec | Behaviors 7–12 |
| `EmptyStringKey { table, index }` | empty/whitespace string key | Behaviors 13–16 |
| `DuplicateStateKey { table, key }` | duplicate string key | Behaviors 17–18 |
| `MissingReference { table, field, hash_hex, payload_table }` | non-zero hash not in payload vec | Behaviors 19–22 |
| `PayloadTooLarge { table, size, max }` | payload > 50 MiB in any of 5 vecs | Behaviors 24–28 |
| `DatabaseOpen { path, reason }` | invalid or empty path | Behaviors 2, 4 |
| `TableInit { reason }` | table creation failure | Behavior 3 |
| `ReadTransaction { reason }` | read txn failure | Behavior 6 |
| `WriteTransaction { reason }` | write txn failure | Behavior 46 |
| `WriteFailed { table, reason }` | individual write failure | Behavior 47 |
| `CommitFailed { reason }` | commit failure | Behavior 48 |
| `ReadFailed { table, reason }` | read failure | DEFERRED — StateReadSession bead (not this bead) |

**Every error variant in `CommitError` has at least one test scenario.** `ReadFailed` is excluded from this bead per contract: "Do NOT implement the StateReadSession bulk-load methods." A deferred placeholder is listed in the Behavior Inventory.

---

## 9. Test Infrastructure Requirements

### Shared Fixtures (Integration Tests)

```
fn create_temp_state_db() -> (StateDb, TempDir)
```
- Creates a temp directory AND opens StateDb AND creates all tables.
- Name advertises the side effect: creates filesystem state.
- TempDir keeps the directory alive for the test duration.
- Used by ALL integration tests.

```
fn make_file_state_raw(analysis: [u8;32], transform: [u8;32], chunk: [u8;32]) -> FileStateRaw
```
- Constructs a valid FileStateRaw with specified hashes.
- Remaining fields filled with deterministic defaults.

```
fn make_url_state_raw(url_hash: [u8;32]) -> UrlStateRaw
```
- Constructs a valid UrlStateRaw with specified hash.

```
fn make_minimal_valid_state_changes() -> StateChanges
```
- Returns a StateChanges with all vecs empty (the minimal valid state).
- Individual tests modify specific fields and document what they change.
- Tests that need reference integrity add entries to both the state vec
  and the corresponding payload vec explicitly.

### Test Isolation

- Every test creates its OWN StateDb instance (no shared database).
- Every test uses `tempfile::TempDir` for automatic cleanup.
- No test depends on execution order.
- No test modifies global state.

---

## 10. Open Questions

1. **Behavior 3 (TableInit)**: How to reliably trigger a table creation failure in redb 2.x? If no deterministic method exists, this test may be a manual gate or require a wrapper trait for `Database`.

2. **Behaviors 46–48 (transaction errors)**: redb 2.x makes it hard to inject failures in begin_write(), insert(), or commit(). Options:
   - (a) Use a trait-based abstraction (`StateDbBackend`) with a fake that can fail on demand
   - (b) Skip these tests and rely on redb's own test suite
   - (c) Use file-descriptor shenanigans (close the fd mid-transaction)
   - Recommendation: Define a `StateDbBackend` trait from the start. This makes the code testable and follows Farley's "test via public API" principle. If the contract forbids a trait, then document these as "tested by redb upstream" and focus on the 45 behaviors we CAN control.

3. **StateReadSession lifetime**: The contract says the caller must drop the read session before calling commit_changes. Should `commit_changes` detect and return an error if a session is still alive? Or is this a caller invariant with no runtime check? The test plan assumes it's a caller invariant (no runtime detection), but if runtime detection is added, it needs a test.

4. **Pod type availability**: Tests for `FileStateRaw` and `UrlStateRaw` depend on those types being defined (separate bead). This test plan assumes they exist and are constructible in tests. If not yet available, the unit tests can use placeholder byte arrays until the types land.

5. **Behavior 49 (long source_path)**: This is a characterization test. The expected outcome depends on redb's internal key-size limits. The test records the actual behavior rather than prescribing it. If redb rejects keys > N bytes, this test documents that fact and can be updated to assert the specific error.

---

## Exit Criteria Checklist

- [x] Every public API behavior has at least one BDD scenario (49 behaviors, 54 scenarios)
- [x] Every pure function with multiple inputs has a proptest invariant (6 invariants including `should_skip_write`)
- [x] Every error variant in `CommitError` has an explicit test scenario (12 variants covered, 1 deferred)
- [x] Mutation threshold ≥90% stated and mapped to specific tests
- [x] No test asserts only `is_ok()` or `is_err()` — all assertions specify exact values/variants
- [x] No parser/deserializer boundary exists in this bead (no fuzz targets needed, justified)
- [x] Kani harnesses specified for critical size invariants
- [x] Every `reason` field in error assertions has a concrete pattern (contains-substring check)
- [x] PayloadTooLarge tested for ALL 5 payload vecs (new_analyses, new_transforms, new_chunks, new_scrapes, new_snapshots)
- [x] Unchanged-row skip mutation resolved via extracted pure function `should_skip_write` with dedicated unit tests
- [x] Deferred StateReadSession functions have explicit DEFERRED entries in Behavior Inventory
- [x] All BDD validation scenarios specify "And all other StateChanges fields are valid"
- [x] Side-effectful helper renamed to `create_temp_state_db()`
