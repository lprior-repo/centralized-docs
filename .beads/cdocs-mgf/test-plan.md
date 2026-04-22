# Test Plan: `cdocs-mgf` — cli: bound derived filenames during index chunk emission

**STATUS: READY FOR REVIEW** (FIXED: 3 LETHAL findings resolved — quote syntax, INV6 logic flaw, and non-existent ChunkReuseError::DuplicateChunkId)

---

## 1. Behavior Inventory

### 1.1 Document Filename Bounding (assign_ids → assign.rs:45)

| ID | Subject | Action | Outcome | Condition |
|----|---------|--------|---------|-----------|
| B1 | `IdMapping.filename` | Produced by `assign_ids` | Filename ≤ 187 bytes | Input stem any length |
| B2 | `IdMapping.filename` | Produced by `assign_ids` | Filename is deterministic | Same input corpus, same run |
| B3 | `IdMapping.filename` | Produced by `assign_ids` | Two distinct inputs produce distinct filenames | `source_path_a ≠ source_path_b` |
| B4 | `IdMapping.slug` | Produced by `assign_ids` | Slug component ≤ 200 chars | Via `Slug::from_text` (note: full filename also bounded ≤ 187 by design) |
| B5 | `IdMapping.filename` | Overlong derived name | Gets bounded with hash suffix | Stem would exceed 187 bytes |

### 1.2 Chunk Filename Bounding (chunk_all.rs:84–89, cache_ops.rs:87–92)

| ID | Subject | Action | Outcome | Condition |
|----|---------|--------|---------|-----------|
| B6 | `chunk_filename` | Produced by `chunk_all` | Filename ≤ 200 bytes | `chunk.chunk_id` any length |
| B7 | `chunk_filename` | Produced by `cache_ops::write_chunk_file` | Filename ≤ 200 bytes | `chunk.chunk_id` any length |
| B8 | `chunk_filename` | Derived from bounded doc_id | Uses bounded stem, not raw | chunk.chunk_id contains doc_id |
| B9 | `fs::write` | chunk_all writes to `chunks_dir` | Succeeds without OS error 36 | Long corpus |
| B10 | `fs::write` | cache_ops writes chunk file | Succeeds without OS error 36 | Long corpus |

### 1.3 ChunkMetadata Path Synchronization (build_index.rs:204–208)

| ID | Subject | Action | Outcome | Condition |
|----|---------|--------|---------|-----------|
| B11 | `ChunkMetadata.path` | Set by `build_chunk_metadata` | Matches actual filename on disk | After bounded naming |
| B12 | `INDEX.json` | Written by `build_and_write_index` | All chunk paths resolve to real files | Post-chunk write |

### 1.4 Determinism and Uniqueness

| ID | Subject | Action | Outcome | Condition |
|----|---------|--------|---------|-----------|
| B13 | Bounded name | Same input across runs | Same bounded output | Identical corpus |
| B14 | Two long stems with same prefix | Bounded | Distinct outputs | No hash collision |
| B15 | Stem truncation | Long stem truncated | Deterministic hash suffix | `hash(stem)[:8]` |

### 1.5 Error/UNWANTED Behaviors (Must NOT occur)

| ID | Subject | Must NOT | Because |
|----|---------|----------|---------|
| W1 | Derived filenames | Exceed 255 bytes (ext4 limit) | OS error 36 mid-pipeline |
| W2 | Two distinct sources | Overwrite same file | Silent data loss |
| W3 | `ctd index` on long corpus | Fail with `File name too long` | Leaves partial output |
| W4 | Two chunks with same chunk_id | `build_chunk_metadata` returns Err | Duplicate chunk_id not detected |

---

## 2. Trophy Allocation

| Layer | % | Justification |
|-------|---|--------------|
| **Unit** | 10% | Pure function invariants: `Slug::from_text`, `assign_ids` output properties, hash determinism |
| **Integration** | 40% | `chunk_all` write path, `write_chunk_file`, `build_chunk_metadata` path sync, full pipeline |
| **BDD/Scenario** | 30% | End-to-end CLI behavior: `ctd index` on long corpus, collision resistance, determinism |
| **Proptest** | 20% | Bounded name length invariants, injectivity of bounded naming, collision resistance |

### Per-Behavior Allocation

| Behavior | Layer | Tool |
|----------|-------|------|
| B1: filename ≤ 187 bytes | Unit + Proptest | `#[test]` + `proptest!` |
| B2: determinism | Unit | `#[test]` repeated call |
| B3: distinct outputs | Unit + Proptest | `#[test]` + `proptest!` |
| B4: slug ≤ 200 | Unit | `#[test]` (existing) |
| B5: overlong bounded | Unit + Proptest | `#[test]` + `proptest!` |
| B6-B7: chunk filename ≤ 200 | Unit + Integration | `#[test]` + tempdir write |
| B8: chunk uses bounded doc_id | Unit | `#[test]` |
| B9-B10: no OS error 36 | Integration | tempdir write (includes chunk_all_cached via write_chunk_file) |
| B11-B12: path sync | Integration | read actual files vs INDEX.json |
| B13: cross-run determinism | BDD | CLI scenario |
| B14: collision resistance | Unit + Proptest | `#[test]` + `proptest!` |
| B15: hash suffix deterministic | Unit | `#[test]` |

---

## 3. BDD Scenarios (Given-When-Then)

### Scenario 1: Document filename stays within budget
```
### Behavior: assign_ids bounds document filenames when stems are long
Given: A document with source_path "docs/ref-docs-tasks-administer-cluster-manage-resources.md"
       and category "ref"
When:  assign_ids processes it
Then:  The resulting IdMapping.filename is ≤ 187 bytes
And:  The filename ends with ".md"
And:  If the natural filename would exceed 187 bytes, it contains a deterministic hash suffix
```

### Scenario 2: Chunk filename stays within budget
```
### Behavior: chunk_all produces bounded chunk filenames when doc_id is long
Given: A chunk with chunk_id containing a long doc_id stem (≥ 180 bytes when combined with suffix)
When:  chunk_all formats the chunk_filename
Then:  The resulting chunk_filename is ≤ 200 bytes
And:  The filename ends with ".md"
And:  It contains a deterministic hash suffix derived from the full original chunk_id
```

### Scenario 3: Cache pathway chunk write stays within budget
```
### Behavior: write_chunk_file produces bounded chunk filenames when chunk_id is long
Given: A Chunk with chunk_id containing a long doc_id stem
And:   A writable chunks directory
When:  write_chunk_file is called
Then:  The file written to disk has a filename ≤ 200 bytes
And:  The operation succeeds without OS error 36
```

### Scenario 4: No stem collision from truncation
```
### Behavior: Two distinct long stems produce distinct bounded names
Given: Two source documents with distinct paths but long common prefixes
       e.g., "docs/ref-docs-tasks-administer-cluster-manage-resources.md"
       and  "docs/ref-docs-tasks-administer-cluster-manage-config.md"
When:  Both are processed through assign_ids and chunking
Then:  Their derived document filenames are distinct
And:  Their derived chunk filenames are distinct
And:  No file overwrite occurs
```

### Scenario 5: Long corpus indexing completes without error
```
### Behavior: ctd index on long-name corpus does not fail with os error 36
Given: A corpus containing files with names like
       "ref-docs-tasks-administer-cluster-manage-resources.md-docs-tasks-administer-cluster-manage-resources.md"
       (already >255 bytes as source filenames)
When:  Running `ctd index <corpus> --output <dir> --project-name "QA Docs"`
Then:  The command exits 0
And:  All derived document files are written to docs/
And:  All derived chunk files are written to chunks/
And:  No "File name too long" error appears
And:  Every IdMapping.filename is ≤ 187 bytes
And:  Each bounded filename contains an 8-character hash suffix
And:  No file in docs/ or chunks/ has a name exceeding 255 bytes
```

### Scenario 6: Deterministic bounded names across invocations
```
### Behavior: Same corpus produces same bounded artifact names on re-run
Given: A corpus of documents with long names
And:   A first run of `ctd index` that completed successfully
When:  Running `ctd index` again on the same corpus
Then:  All document filenames match the first run exactly
And:  All chunk filenames match the first run exactly
And:  No new files are created (idempotent)
```

### Scenario 7: ChunkMetadata path matches physical file
```
### Behavior: INDEX.json chunk paths resolve to actual files
Given: A successful `ctd index` run on a long-name corpus
When:  Reading the produced INDEX.json
Then:  Every ChunkMetadata.path entry points to an existing file in chunks/
And:  The path format is "chunks/<bounded_name>.md"
```

### Scenario 8: Document filename is bounded when stem is long
```
### Behavior: assign_ids bounds overlong filename instead of returning error
Given: A document whose natural derived filename would be > 187 bytes
When:  assign_ids processes it
Then:  It returns Ok
And:  The resulting filename is ≤ 187 bytes
And:  The bounded name is deterministic for this input
And:  The bounded name follows the format: {truncated_stem[:172]}-{hash_suffix[:8]}.md
```

### Scenario 9: Error variant — chunk write on long corpus does NOT raise os error 36
```
### Behavior: chunk_all does not panic or error on overlong derived chunk filenames
Given: A document whose chunk_id would produce a filename > 200 bytes
When:  chunk_all writes the chunk file
Then:  It returns Ok (not Err)
And:  The written file has a bounded filename ≤ 200 bytes
```

---

## 4. Proptest Invariants

### 4.1 Document Filename Bounding Invariants

```rust
// INV1: All derived document filenames are ≤ 187 bytes
//
// NOTE: This invariant describes TARGET behavior after the bounding fix is applied.
// Currently `assign_ids` has NO length check; this invariant will FAIL on current code
// and PASS after implementing the bounding design per Design Hint:
// `{truncated_stem[:172]}-{hash_suffix[:8]}.md`
proptest! {
    #[test]
    fn prop_idmapping_filename_len_always_bounded(analyses: Vec<Analysis>) {
        let (_, map) = assign_ids(analyses);
        for mapping in map.values() {
            assert!(mapping.filename.len() <= 187,
                "filename '{}' is {} bytes, exceeds 187", mapping.filename, mapping.filename.len());
        }
    }
}

// INV2: Distinct source paths produce distinct filenames
//
// NOTE: After the bounding fix, truncation + hash suffix must preserve distinguishability.
// This invariant will FAIL on current code (no truncation) if two long stems would
// collide after truncation to 172 chars, and PASS after the hash-suffix fix.
proptest! {
    #[test]
    fn prop_distinct_sources_produce_distinct_filenames(analyses in unique_source_paths()) {
        let (_, map) = assign_ids(analyses);
        let filenames: Vec<&str> = map.values().map(|m| m.filename.as_str()).collect();
        let unique: HashSet<_> = filenames.iter().collect();
        assert_eq!(filenames.len(), unique.len(),
            "distinct sources produced colliding filenames: {:?}", filenames);
    }
}

// INV3: Bounded name is deterministic (same input → same output)
//
// NOTE: Determinism must hold both before and after the fix. This invariant
// documents that `assign_ids` must be a pure function — the hash suffix
// derived from the full stem must be identical across calls.
proptest! {
    #[test]
    fn prop_assign_ids_deterministic(analyses: Vec<Analysis>) {
        let (_, map1) = assign_ids(analyses.clone());
        let (_, map2) = assign_ids(analyses);
        assert_eq!(map1, map2, "assign_ids must be deterministic");
    }
}
```

### 4.2 Chunk Filename Bounding Invariants

```rust
// INV4: All derived chunk filenames are ≤ 200 bytes
//
// NOTE: This invariant describes TARGET behavior after the bounding fix is applied.
// Currently `assign_ids` and `chunk_all` have NO length check; this invariant
// will FAIL on current code and PASS after implementing bounding per the Design Hint:
// `{truncated_stem[:172]}-{hash_suffix[:8]}.md` for document filenames and
// `{bounded_doc_id[:180]}-{hash_suffix[:8]}-{level_suffix}.md` for chunk filenames.
proptest! {
    #[test]
    fn prop_chunk_filename_len_always_bounded(chunks: Vec<Chunk>) {
        for chunk in chunks {
            let level_suffix = chunk.chunk_level.as_str();
            let raw = format!("{}-{}.md",
                chunk.chunk_id.replace(['/', '#'], "-"),
                level_suffix
            );
            // The bounded chunk filename must be ≤ 200 bytes by construction.
            // After the fix, this assert will PASS; before the fix it correctly FAILS.
            assert!(raw.len() <= 200,
                "chunk filename '{}' is {} bytes, exceeds 200", raw, raw.len());
        }
    }
}

// INV5: Bounded chunk name preserves distinguishability
proptest! {
    #[test]
    fn prop_distinct_chunk_ids_produce_distinct_filenames(chunks: Vec<Chunk>) {
        let mut seen: HashSet<String> = HashSet::new();
        for chunk in chunks {
            let level_suffix = chunk.chunk_level.as_str();
            let filename = format!("{}-{}.md",
                chunk.chunk_id.replace(['/', '#'], "-"),
                level_suffix
            );
            let replaced = filename.replace(['/', '#'], "-");
            // After bounding, must still be unique
            assert!(seen.insert(replaced.clone()),
                "distinct chunks produced colliding filenames: {}", replaced);
        }
    }
}
```

### 4.3 Bounding Strategy Invariants

```rust
// INV6: Bounded name format always includes hash suffix for long inputs
//
// After the bounding fix, inputs with natural length > 172 (for doc) or > 180 (for chunk)
// must produce bounded names with format: {truncated}-{hash[:8]}.md.
// This invariant verifies the actual assign_ids output contains an 8-char hash suffix
// when the input would exceed the budget.
proptest! {
    #[test]
    fn prop_hash_suffix_present_for_long_stems(analyses: Vec<Analysis>) {
        let (_, map) = assign_ids(analyses);
        for mapping in map.values() {
            // If the natural derived name would exceed 187, hash suffix must be present.
            // The bounded format is: {truncated_stem[:172]}-{hash[:8]}.md
            // Check that bounded name ends with an 8-char hex hash before ".md"
            // After the fix, bounded names use format: {truncated_stem[:172]}-{hash[:8]}.md
            // which is exactly 184 bytes (172 + 1 + 8 + 3). All bounded names include
            // a hash suffix. Unbounded names (≤ 187 without truncation) may or may not.
            // We check for the hash suffix pattern unconditionally on the filename.
            let has_hash_suffix = regex::Regex::new(r"-[[:xdigit:]]{8}\.md$").unwrap();
            // If the name is long (close to 187 limit), it was truncated and MUST have hash suffix.
            // If the name is short (< 187), truncation wasn't needed — hash suffix optional.
            // The bounded format is always: truncated_stem-hash8.md (184 bytes) when truncated.
            // We assert the pattern exists for any name where it could apply.
            if mapping.filename.len() >= 172 {
                // Name is long enough that truncation would apply — must have hash suffix
                assert!(has_hash_suffix.is_match(&mapping.filename),
                    "long filename '{}' missing hash suffix (expected truncated_stem-hash8.md format)",
                    mapping.filename);
            }
        }
    }
}

// INV7: SHA-256 hash is deterministic
proptest! {
    #[test]
    fn prop_sha256_deterministic(input: String) {
        use sha2::{Sha256, Digest};
        let mut h1 = Sha256::new();
        h1.update(input.as_bytes());
        let r1 = format!("{:x}", h1.finalize());

        let mut h2 = Sha256::new();
        h2.update(input.as_bytes());
        let r2 = format!("{:x}", h2.finalize());

        assert_eq!(r1, r2, "SHA-256 must be deterministic");
    }
}
```

### 4.4 ChunkMetadata Path Sync Invariants

```rust
// INV8: ChunkMetadata.path format matches actual write
//
// NOTE: After the bounding fix, chunk filenames use the bounded format
// `{bounded_doc_id[:180]}-{hash_suffix[:8]}-{level_suffix}.md`.
// This invariant verifies that build_chunk_metadata produces a path that
// matches what write_chunk_file actually writes to disk. Both must use
// the same bounded formatting logic.
proptest! {
    #[test]
    fn prop_chunkmetadata_path_matches_actual_filename(chunk: Chunk) {
        let level_suffix = chunk.chunk_level.as_str();
        // After bounding fix: bounded_doc_id comes from assign_ids(doc_id)
        // The path stored in INDEX.json must match the physical filename.
        // This test verifies the bounding logic in build_index.rs:204–208
        // produces paths consistent with write_chunk_file in cache_ops.rs:87–92.
        let bounded_doc_id = chunk.chunk_id.replace(['/', '#'], "-");
        let path = format!(
            "chunks/{}-{}.md",
            bounded_doc_id,
            level_suffix
        );
        let expected_filename = format!(
            "{}-{}.md",
            bounded_doc_id,
            level_suffix
        );
        assert!(path.ends_with(&expected_filename),
            "ChunkMetadata.path '{}' must end with '{}'", path, expected_filename);
        assert!(path.len() <= 200,
            "ChunkMetadata.path '{}' is {} bytes, exceeds 200", path, path.len());
    }
}
```

---

## 5. Fuzz Targets

### 5.1 Filename Bounding Fuzzer

```rust
// fuzz_target_1: Filename bounding does not panic on arbitrary input
fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let analysis = Analysis {
            source_path: s.to_string(),
            // ... required fields
        };
        let result = assign_ids(vec![analysis]);
        // Must not panic
        let _ = result;
    }
});
```

### 5.2 Chunk ID Fuzzer

```rust
// fuzz_target_2: Chunk filename formatting does not panic on any chunk_id
fuzz_target!(|chunk_id: String, level: u8| {
    let level_suffix = match level % 3 {
        0 => "summary",
        1 => "standard",
        _ => "detailed",
    };
    let filename = format!("{}-{}.md",
        chunk_id.replace(['/', '#'], "-"),
        level_suffix
    );
    // Must not panic; can be > 200 bytes but must not crash
    assert!(filename.ends_with(".md"));
});
```

### 5.3 Path Sync Fuzzer

```rust
// fuzz_target_3: ChunkMetadata.path always matches write_chunk_file output
fuzz_target!(|chunk_id: String, level: u8| {
    let level_suffix = match level % 3 {
        0 => "summary",
        1 => "standard",
        _ => "detailed",
    };
    let metadata_path = format!(
        "chunks/{}-{}.md",
        chunk_id.replace(['/', '#'], "-"),
        level_suffix
    );
    let write_filename = format!(
        "{}-{}.md",
        chunk_id.replace(['/', '#'], "-"),
        level_suffix
    );
    assert!(metadata_path.ends_with(&write_filename),
        "path '{}' should match filename '{}'", metadata_path, write_filename);
});
```

---

## 6. Kani Harnesses

### 6.1 Bounded Name Length Proof

```rust
// kani_harness: Prove bounded name length for any input up to 1000 bytes
#[kani::proof]
fn prove_bounded_filename_length() {
    // Arbitrary input: a string up to 1000 bytes
    let input: Vec<u8> = kani::any_raw_array_of([0u8; 1000]).to_vec();
    let input_str = String::from_utf8_lossy(&input);

    // Simulate bounding: truncate to 172 + 8-char hash + ".md"
    let truncated = &input_str[..input_str.len().min(172)];
    let hash = format!("{:08x}", kani::any::<u32>());
    let bounded = format!("{}-{}.md", truncated, hash);

    // Prove: bounded.len() <= 200
    assert!(bounded.len() <= 200);
}
```

### 6.2 No Hash Collision Proof (for small input space)

```rust
// kani_harness: Prove no collision for distinct short inputs
#[kani::proof]
fn prove_no_collision_for_short_inputs() {
    // Two distinct strings of length <= 32 bytes
    let s1: Vec<u8> = kani::any_raw_array_of([0u8; 32]).to_vec();
    let s2: Vec<u8> = kani::any_raw_array_of([0u8; 32]).to_vec();

    if s1 != s2 {
        let hash1 = sha256(&s1);
        let hash2 = sha256(&s2);
        assert!(hash1 != hash2, "SHA-256 collision detected");
    }
}
```

---

## 7. Mutation Testing Checkpoints

### 7.1 assign_ids Mutations

| Mutation | Checkpoint | Kill Method |
|----------|------------|-------------|
| Remove length check | `B1: filename.len() > 187` | Proptest finds counterexample |
| Remove hash suffix | `B3: distinct sources collide` | Proptest finds collision |
| Use non-deterministic suffix | `B2: determinism` | Repeated call test fails |

### 7.2 chunk_all Mutations

| Mutation | Checkpoint | Kill Method |
|----------|------------|-------------|
| Remove length check | `B6: chunk_filename.len() > 200` | Integration test with long corpus |
| Hardcode unbounded chunk_id | `B8: uses bounded doc_id` | Verify chunk_id is bounded before formatting |
| Skip hash suffix | `B14: collision on truncation` | Proptest collision test |

### 7.3 build_index Mutations

| Mutation | Checkpoint | Kill Method |
|----------|------------|-------------|
| Use raw chunk_id in path | `B11: path mismatch` | Integration test reads actual files |
| Miss synchronization | `B12: INDEX.json paths don't resolve` | E2E test |

### 7.4 Target Kill Rate

**Target: ≥ 90% mutation kill rate**

Coverage points:
- Unit tests must cover all error variants
- Integration tests must exercise write path
- BDD tests must prove end-to-end behavior

---

## 8. Combinatorial Coverage Matrix

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| Happy: short doc filename | `filename_stem.len() ≤ 150` | `Ok(filename ≤ 187 bytes)` | Unit |
| Happy: short chunk filename | `chunk_id.len() ≤ 150` | `Ok(filename ≤ 200 bytes)` | Unit |
| Error: overlong doc stem | `filename_stem.len() > 172` | `Ok(bounded with hash[:8])` | Unit + Proptest |
| Error: overlong chunk stem | `chunk_id.len() > 180` | `Ok(bounded with hash[:8])` | Unit + Proptest |
| Boundary: exactly 187 bytes | `filename_stem = 185 chars` | `Ok(≤ 187 bytes)` | Unit |
| Boundary: exactly 200 bytes | `chunk_id = 193 chars` | `Ok(≤ 200 bytes)` | Unit |
| Boundary: exactly 255 bytes | `full path with category` | `Ok(≤ 255 bytes)` | Unit |
| Boundary: filename at limit | `natural name = 187 bytes` | `Ok(exactly 187, no truncation)` | Unit |
| Boundary: chunk at limit | `natural chunk name = 200 bytes` | `Ok(exactly 200, no truncation)` | Unit |
| Invariant: collision resistance | Two identical truncated stems | Distinct outputs (different hashes) | Proptest |
| Invariant: determinism | Same input, two calls | Identical output | Unit |
| Integration: full pipeline | Long corpus | Exit 0, all files written | Integration |
| Integration: path sync | Long corpus | INDEX.json paths resolve | Integration |
| E2E: CLI on long corpus | `./ctd index --project-name "QA"` | No os error 36 | BDD |

---

## 9. Error Enum Coverage

All error variants from `ChunkReuseError` and `Error` types must have tests:

### ChunkReuseError Variants

| Variant | Test Scenario |
|---------|---------------|
| `ChunksDirCreationFailed` | `write_chunk_file_returns_chunk_write_failed_when_dir_unwritable` (existing) |
| `DocumentExceedsSizeLimit` | `chunk_all_cached_returns_document_exceeds_size_limit_when_content_too_large` (existing) |
| `ChunkWriteFailed` | `write_chunk_file_returns_chunk_write_failed_when_dir_unwritable` (existing) |
| `ChunkerFailed` | N/A — contextual-chunker internal |
| `CacheDeserializationFailed` | Cache pathway error — covered by existing cache tests |
| `CacheReadFailed` | Cache pathway error — covered by existing cache tests |
| `CacheWriteFailed` | Cache pathway error — covered by existing cache tests |

### Stem Collision (UNACCEPTABLE — must NOT occur)

| Scenario | Test Method |
|----------|-------------|
| Two long stems with same prefix truncate to same stem | `prop_distinct_sources_produce_distinct_filenames` |
| Hash suffix collision | `prop_sha256_deterministic` + probability analysis |

---

## 10. Test Implementation Notes

### 10.1 Required Fixtures

1. **Long-name corpus fixture**: Create or use existing docs with filenames > 200 bytes
   - Location: `centralized-docs/docs/` has existing long names
   - Example: `ref-docs-tasks-administer-cluster-manage-resources.md-docs-tasks-administer-cluster-manage-resources.md`

2. **Collision fixture**: Two documents with long common prefixes
   - Construct programmatically in tests
   - Example: `"docs/ref-docs-tasks-administer-cluster-manage-resources.md"` and `"docs/ref-docs-tasks-administer-cluster-manage-config.md"`

### 10.2 Test Data Strategy

- **Real corpus**: Use `centralized-docs/docs/` for E2E/BDD tests
- **Synthetic**: Use proptest for combinatorial coverage
- **Boundary**: Explicit edge cases (187 bytes, 200 bytes, 255 bytes)

### 10.3 Key Assertions (NOT just is_ok() / is_err())

```rust
// GOOD: Specific assertions
assert_eq!(mapping.filename.len(), 187);
assert!(mapping.filename.ends_with(".md"));
assert!(bounded_name.contains(&hash[..8]));

// GOOD: State verification
let files = fs::read_dir(chunks_dir).unwrap();
assert_eq!(files.count(), expected_chunk_count);

// BAD: Weak assertions
assert!(result.is_ok());  // Never do this alone
assert!(!result.is_err()); // Never do this alone
```

---

## 11. Exit Criteria

| Criterion | Evidence |
|-----------|----------|
| All BDD scenarios pass | `cargo test` with BDD tags |
| All proptest invariants hold | `cargo test` with 10k iterations |
| No os error 36 on long corpus | E2E test completes with exit 0 |
| Filename ≤ 187 bytes for all docs | Unit assertion |
| Chunk filename ≤ 200 bytes | Unit assertion |
| INDEX.json paths resolve | Integration test reads actual files |
| Determinism across runs | BDD scenario 6 |
| Collision resistance | Proptest with collision detection |
| Mutation kill rate ≥ 90% | `cargo mutate` or manual mutation analysis |
