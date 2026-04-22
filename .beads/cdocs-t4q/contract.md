---
bead_id: cdocs-t4q
bead_title: "feat(state): Add redb MultimapTableDefinition for source_path -> chunk_ids"
phase: contract
updated_at: 2026-04-07T16:15:00Z
---

# Contract: source_path -> chunk_ids Multimap

## Problem

When a file is deleted via `StateChanges::deleted_files`, its `FileStateRaw` row is removed
from the `file_state` table. However, the corresponding chunk records in `chunk_outputs`
(hash-keyed) are **orphaned** — they persist indefinitely because there is no reverse index
from `source_path` to the chunk hashes that belong to it.

Currently, the only link is `FileStateRaw.chunk_hash` (a single 32-byte hash). Once the
`file_state` row is deleted, this link is lost, and the chunk data becomes unreachable garbage.

## Solution

Add a `redb::MultimapTableDefinition` table named `"source_path_chunks"` that maps:

```
source_path: &str  ->  chunk_hash: &[u8]   (one-to-many)
```

This enables O(1) "delete all chunks for file X" by:
1. `multimap.get(source_path)` → iterator of chunk hashes
2. Delete each chunk hash from `chunk_outputs`
3. `multimap.remove_all(source_path)` → clean up the multimap itself

## Data Types

### New Table Definition

```rust
const SOURCE_PATH_CHUNKS_TABLE: MultimapTableDefinition<&str, &[u8]> =
    MultimapTableDefinition::new("source_path_chunks");
```

- **Key**: `&str` — the source file path (same key space as `file_state` table)
- **Value**: `&[u8]` — a 32-byte chunk hash (same format as `chunk_outputs` keys)

### New Table Name Constant

```rust
pub const TABLE_NAME_SOURCE_PATH_CHUNKS: &str = "source_path_chunks";
```

### New Accessor Function

```rust
pub const fn source_path_chunks_table()
    -> MultimapTableDefinition<'static, &'static str, &'static [u8]>
```

## Invariants

### INV-1: Multimap Consistency on Upsert
For every `(source_path, FileStateRaw)` in `updated_files` where `chunk_hash != [0u8; 32]`,
the multimap MUST contain the entry `source_path -> chunk_hash` after commit.

### INV-2: Multimap Cleanup on Delete
For every `source_path` in `deleted_files`, after commit:
- All entries `source_path -> chunk_hash` MUST be removed from the multimap
- Each referenced `chunk_hash` MUST be removed from `chunk_outputs`
- The `file_state` row MUST be removed (existing behavior preserved)

### INV-3: Zero-Hash Exclusion
When `FileStateRaw.chunk_hash == [0u8; 32]`, NO entry is inserted into the multimap.
The zero hash means "no chunks yet" and is not a valid chunk_outputs key.

### INV-4: Table Name Uniqueness
`"source_path_chunks"` is distinct from all 8 existing table names:
`file_state`, `url_state`, `analysis_outputs`, `transform_outputs`, `chunk_outputs`,
`scrape_outputs`, `snapshots`, `metadata`.

### INV-5: Atomicity
All multimap mutations occur within the same write transaction as all other state changes.
On validation failure, NO multimap writes are applied (transaction rolled back).

### INV-6: Idempotent Initialization
`initialize_tables` MUST call `open_multimap_table` for the new table, creating it if absent,
succeeding silently if present. Existing data must not be destroyed.

## Preconditions (enforced by commit_changes)

All existing preconditions are preserved unchanged:
- P1: No zero hash keys in payload vecs
- P2: No empty/whitespace string keys
- P3: No duplicate string keys
- P4: Reference integrity
- P6: Payload size limits

No new preconditions are needed — the multimap is populated from existing validated data.

## Postconditions (on Ok)

All existing postconditions are preserved, plus:
- PC-NEW-1: Multimap reflects the current chunk_hash for every upserted file
- PC-NEW-2: Orphaned chunks are deleted from `chunk_outputs` for every deleted file
- PC-NEW-3: Multimap entries for deleted files are fully removed

## Postconditions (on Err)

- NO writes applied (existing behavior). Multimap is unchanged.

## Error Taxonomy

No new error variants needed. Multimap operations use existing `CommitError::WriteFailed`:

```rust
CommitError::WriteFailed {
    table: "source_path_chunks",
    reason: String,
}
```

## Files Modified

1. **`centralized-docs/src/state/mod.rs`**:
   - Add `use redb::MultimapTableDefinition` import
   - Add `TABLE_NAME_SOURCE_PATH_CHUNKS` constant
   - Add `SOURCE_PATH_CHUNKS_TABLE` const definition
   - Add `source_path_chunks_table()` accessor function
   - Update `initialize_tables` to call `write_tx.open_multimap_table(SOURCE_PATH_CHUNKS_TABLE)`
   - Update module docstring to reflect 9 tables (was 8)

2. **`centralized-docs/src/state/commit.rs`**:
   - Add `source_path_chunks_table` to imports from `super`
   - Update `apply_all_writes` to:
     a. After `write_file_states`, insert multimap entries for non-zero chunk hashes
     b. After `delete_entries` for files, look up multimap and delete orphaned chunks
   - Add helper functions:
     - `write_source_path_chunks` — insert multimap entries
     - `delete_orphaned_chunks` — look up and delete chunks + multimap cleanup

## Non-Goals

- This bead does NOT change `StateChanges` struct (no new fields)
- This bead does NOT add `deleted_chunks` to `StateChanges`
- This bead does NOT modify the read session / bulk-load API
- This bead does NOT add migration logic for existing databases
  (multimap will be populated incrementally on next file upsert)

## Functional Layering (Data → Calc → Actions)

- **Data**: `MultimapTableDefinition`, table name constant, `FileStateRaw`
- **Calc**: Pure functions — determining which chunk hashes to insert/delete from the multimap
  can be computed from `StateChanges` data alone
- **Actions**: `write_source_path_chunks`, `delete_orphaned_chunks` — I/O operations
  confined to the redb write transaction
