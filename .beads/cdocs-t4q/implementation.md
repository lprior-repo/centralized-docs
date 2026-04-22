---
bead_id: cdocs-t4q
bead_title: "feat(state): Add redb MultimapTableDefinition for source_path -> chunk_ids"
phase: implementation
updated_at: 2026-04-07T16:30:00Z
---

# Implementation Summary

## Changes Made

### 1. `centralized-docs/src/state/mod.rs`

- **New import**: `MultimapTableDefinition` added to redb imports
- **New constant**: `TABLE_NAME_SOURCE_PATH_CHUNKS = "source_path_chunks"`
- **New table definition**: `SOURCE_PATH_CHUNKS_TABLE: MultimapTableDefinition<&str, &[u8]>`
- **New accessor**: `source_path_chunks_table()` returning the multimap definition
- **Updated `initialize_tables`**: Now opens 9 tables (was 8), including the multimap via `write_tx.open_multimap_table()`
- **Updated module docstring**: Reflects 9 tables with multimap documentation
- **Updated existing tests**: All "8 tables" references updated to "9 tables", multimap included in verification

### 2. `centralized-docs/src/state/commit.rs`

- **New import**: `ReadableMultimapTable` added for multimap read operations
- **New import**: `MultimapTable` added for multimap write operations
- **New function `write_source_path_chunks`**: After file state upserts, inserts `source_path -> chunk_hash` into the multimap for every file with a non-zero `chunk_hash`
- **New function `delete_orphaned_chunks`**: When files are deleted, looks up all chunk hashes in the multimap, deletes them from `chunk_outputs`, and removes the multimap entries
- **New function `open_multimap_for_write`**: Helper to open the multimap table in a write transaction
- **Updated `apply_all_writes`**: Added calls to `write_source_path_chunks` (after file upserts) and `delete_orphaned_chunks` (before file state deletions)

## Invariants Maintained

- INV-1: Multimap populated for every upsert with non-zero `chunk_hash`
- INV-2: Orphaned chunks deleted when files are deleted
- INV-3: Zero-hash (`[0u8; 32]`) files do NOT get multimap entries
- INV-4: Table name `"source_path_chunks"` is unique across all 9 tables
- INV-5: All multimap mutations are atomic within the same write transaction
- INV-6: Idempotent initialization via `open_multimap_table`

## Functional Layering

- **Data**: `MultimapTableDefinition`, `TABLE_NAME_SOURCE_PATH_CHUNKS`, `FileStateRaw`
- **Calc**: Determining chunk hashes to insert/delete from validated `StateChanges` data
- **Actions**: `write_source_path_chunks`, `delete_orphaned_chunks`, `open_multimap_for_write`

## Test Results

- 1262 tests pass (9 new + 1253 existing)
- Clippy clean (no new warnings)
- No `unwrap`, `expect`, `panic`, `unsafe` in production code
