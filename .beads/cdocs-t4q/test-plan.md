---
bead_id: cdocs-t4q
bead_title: "feat(state): Add redb MultimapTableDefinition for source_path -> chunk_ids"
phase: test-plan
updated_at: 2026-04-07T16:16:00Z
---

# Test Plan: source_path_chunks Multimap

## Testing Trophy Allocation

- **Unit (70%)**: Pure calc functions, table definitions, multimap CRUD operations
- **Integration (20%)**: Full commit_changes pipeline with multimap interactions
- **Property (10%)**: Round-trip invariants, atomicity under random mutations

## Unit Tests

### T1: Table definition name is unique (mod.rs)
```gherkin
Given all 9 table definitions (8 existing + source_path_chunks)
When collecting all names into a HashSet
Then exactly 9 unique names exist
And "source_path_chunks" is among them
```

### T2: source_path_chunks_table accessor returns correct definition (mod.rs)
```gherkin
Given source_path_chunks_table()
When calling .name()
Then result is "source_path_chunks"
```

### T3: initialize_tables creates all 9 tables including multimap (mod.rs)
```gherkin
Given a fresh redb database
When initialize_tables is called
Then opening source_path_chunks_table on a read transaction succeeds
```

### T4: initialize_tables is idempotent with multimap (mod.rs)
```gherkin
Given a database with existing multimap data
When initialize_tables is called again
Then all existing data persists in the multimap
```

### T5: Multimap survives database reopen (mod.rs)
```gherkin
Given a database with multimap entries
When the database is closed and reopened
Then the multimap entries are still accessible
```

### T6: Upsert with non-zero chunk_hash populates multimap (commit.rs)
```gherkin
Given an empty database
And updated_files contains ("src/main.rs", FileStateRaw { chunk_hash: [0xAA; 32], ... })
And new_chunks contains ([0xAA; 32], some_bytes)
When commit_changes is called
Then source_path_chunks multimap contains "src/main.rs" -> [0xAA; 32]
```

### T7: Upsert with zero chunk_hash does NOT populate multimap (commit.rs)
```gherkin
Given an empty database
And updated_files contains ("src/main.rs", FileStateRaw { chunk_hash: [0u8; 32], ... })
When commit_changes is called
Then source_path_chunks multimap has NO entries for "src/main.rs"
```

### T8: Delete file removes orphaned chunks and multimap entries (commit.rs)
```gherkin
Given a database where "src/main.rs" has chunk_hash [0xAA; 32]
And chunk_outputs contains [0xAA; 32] -> data
And source_path_chunks contains "src/main.rs" -> [0xAA; 32]
When commit_changes with deleted_files = ["src/main.rs"]
Then file_state row for "src/main.rs" is removed
And chunk_outputs row [0xAA; 32] is removed
And source_path_chunks entries for "src/main.rs" are removed
```

### T9: Delete nonexistent file does not error (commit.rs)
```gherkin
Given an empty database
When commit_changes with deleted_files = ["nonexistent.rs"]
Then commit succeeds (Ok)
And source_path_chunks multimap has no entries
```

### T10: Re-upsert updates multimap to new chunk_hash (commit.rs)
```gherkin
Given a database where "src/main.rs" has chunk_hash [0xAA; 32]
When commit_changes with updated_files = [("src/main.rs", state with chunk_hash [0xBB; 32])]
And new_chunks contains [0xBB; 32]
Then source_path_chunks multimap contains "src/main.rs" -> [0xBB; 32]
```

## Integration Tests

### T11: Full lifecycle — insert, update, delete (commit.rs)
```gherkin
Given an empty database
When inserting file "a.rs" with chunk_hash [1u8; 32]
Then inserting file "b.rs" with chunk_hash [2u8; 32]
Then deleting file "a.rs"
Then file "b.rs" still has multimap entry
And file "a.rs" has no multimap entry
And chunk [1u8; 32] is removed from chunk_outputs
And chunk [2u8; 32] still exists in chunk_outputs
```

### T12: Atomicity — failed validation leaves multimap unchanged (commit.rs)
```gherkin
Given a database with no multimap entries
When commit_changes with valid updated_files but zero hash in new_analyses
Then commit fails with ZeroHashKey
And source_path_chunks multimap has NO entries
```

### T13: Mixed mutations include multimap operations (commit.rs)
```gherkin
Given a pre-populated database
When commit_changes with updated_files, deleted_files, new_chunks, etc.
Then all multimap mutations are correct alongside all other state changes
```

## Property Tests

### T14: Multimap reflects chunk_hashes of all committed files
```gherkin
For any valid StateChanges batch committed successfully:
- For every (path, state) in updated_files where chunk_hash != [0u8; 32]:
  multimap.get(path) should contain chunk_hash
- For every path in deleted_files:
  multimap.get(path) should be empty
```

### T15: Orphaned chunks are never left after delete
```gherkin
For any sequence of commits:
- After deleting file X, if chunk_outputs has no remaining multimap reference to hash H, H is removed
```

## BDD Scenarios

### Scenario: New file with chunks
- Given empty database
- When committing ("guide.md", FileStateRaw with chunk_hash=[0xAB; 32]) and new_chunks=[0xAB; 32]
- Then multimap["guide.md"] contains [0xAB; 32]

### Scenario: File deletion cascades to chunks
- Given database with file "old.md" having chunk_hash=[0xCD; 32] in both chunk_outputs and multimap
- When committing deleted_files=["old.md"]
- Then chunk_outputs[0xCD; 32] is gone
- And multimap["old.md"] is empty

### Scenario: Zero-hash file has no multimap entry
- Given empty database
- When committing ("pending.md", FileStateRaw with chunk_hash=[0u8; 32])
- Then multimap has no entry for "pending.md"

### Scenario: Multiple files can share same chunk hash
- Given empty database
- When committing two files both with chunk_hash=[0xAA; 32]
- Then multimap has "file1.rs" -> [0xAA; 32] and "file2.rs" -> [0xAA; 32]
- When deleting "file1.rs"
- Then chunk_outputs[0xAA; 32] still exists (file2.rs still references it)

## Mutation Testing Checkpoints

- M1: Remove multimap insert in write_source_path_chunks → T6 fails
- M2: Remove chunk deletion in delete_orphaned_chunks → T8 fails
- M3: Remove multimap cleanup in delete_orphaned_chunks → T8 fails (multimap entry lingers)
- M4: Skip zero-hash check → T7 fails (zero hash inserted into multimap)
- M5: Remove multimap from initialize_tables → T3 fails

## Holzmann Rules Compliance

- Every test has a clear pass/fail criterion (assertion-based)
- No test depends on execution order
- Tests clean up their own resources (TempDir)
- No hardcoded paths or timing dependencies
