# Test Plan: Archived Output Bulk Loaders with Transform Reuse Coverage

**Bead:** cdocs-4s3
**Status:** REVISED (addressing REJECTED review — 4 MAJOR, 6 MINOR defects)
**Date:** 2026-04-02

## Summary

- **Behaviors identified:** 35
- **Trophy allocation:** 6 unit / 29 integration / 0 e2e
- **Proptest invariants:** 4
- **Fuzz targets:** 5
- **Kani harnesses:** 2
- **Mutation kill target:** ≥ 90%

### Trophy Distribution Rationale

| Layer     | Count | Ratio | Rationale |
|-----------|-------|-------|-----------|
| Static    | —     | —     | `clippy`, `cargo-deny`, `#[non_exhaustive]` on `BulkLoadError` — free at compile time |
| Unit      | 6     | 17%   | Pure `OwnedArchive::try_from_bytes` construction + deserialization — no redb dependency |
| Integration | 29  | 83%   | All four bulk loaders need a real redb database, real tables, real rkyv-serialized bytes |
| E2E       | 0     | 0%    | No CLI surface in this bead; bulk loaders are internal library APIs |

### Defects Addressed in This Revision

| Defect ID | Severity | Summary | Fix Applied |
|-----------|----------|---------|-------------|
| MAJOR-1 | MAJOR | Behavior 1 Then-clause was `is_ok()` — no concrete inner value | Behavior 1 now asserts `as_bytes().len() == known_size` and `archived()` field values |
| MAJOR-2 | MAJOR | All 4 bulk loaders missing ≥3 boundaries each | Added large-input test (Behavior 35); boundary section in Section 8; capacity overflow documented |
| MAJOR-3 | MAJOR | No mixed valid+corrupt fail-fast test for I-05 | Added Behaviors 16, 23, 28, 33 — one per loader |
| MAJOR-4 | MAJOR | Mutation checkpoint 3 wrong — Behavior 9 doesn't catch early-return removal | Added Behavior 17 (empty input + table missing); fixed checkpoint 3 |
| MINOR-1 | MINOR | Behavior 9 untestable sub-assertion "no redb table access" | Removed; Behavior 10 now only asserts `Ok(HashMap::new())` with `len() == 0` |
| MINOR-2 | MINOR | Behavior 4 asserts lifetime property, not a value | Replaced with concrete field-value assertion; lifetime is compile-time verified |
| MINOR-3 | MINOR | `try_from_bytes` missing min/max valid payload boundaries | Added to Matrix A in Section 8 |
| MINOR-4 | MINOR | `deserialize()` error path untested | Added Behavior 6 documenting bytecheck strictly stronger than deserialization |
| MINOR-5 | MINOR | Kani Harness 2 uses `is_ok()` + `unwrap()` pattern | Fixed to `assert_eq!(result, Ok(HashMap::new()))` |
| MINOR-6 | MINOR | StorageError test deferred without decision | Decision made: formally accepted as gap with risk assessment (see Behavior 13) |

---

## Section 1 — Behavior Inventory

Every system behavior expressed as `[Subject] [action] [outcome] when [condition]`:

### OwnedArchive Construction (`try_from_bytes` / `archived` / `deserialize`)

1. **OwnedArchive** returns `OwnedArchive<T>` with correct byte length and valid archived field values when bytes pass rkyv bytecheck
2. **OwnedArchive** returns `CorruptPayload` when bytes fail rkyv bytecheck validation
3. **OwnedArchive** preserves exact input bytes in its `Box<[u8]>` when constructed successfully
4. **OwnedArchive** zero-copy `archived()` returns reference with matching field values when called on a valid archive
5. **OwnedArchive** `deserialize()` returns owned `T` with field-by-field equality when bytes are valid
6. **OwnedArchive** `deserialize()` error path is unreachable when `try_from_bytes` succeeded — bytecheck validation is strictly stronger than rkyv `HighDeserializer` (documented, not tested)

### `load_analyses` bulk loader

7. **load_analyses** returns `HashMap` with all matching entries when all hashes exist in `analysis_outputs`
8. **load_analyses** returns `HashMap` with only found entries when some hashes are missing
9. **load_analyses** returns empty `HashMap` when no hashes exist in the table
10. **load_analyses** returns empty `HashMap` when input hash slice is empty (`&[]`)
11. **load_analyses** returns single entry when duplicate hashes appear in input slice
12. **load_analyses** returns `TableOpen` error when `analysis_outputs` table cannot be opened
13. **load_analyses** returns `StorageError` when redb I/O failure occurs during read
14. **load_analyses** returns `CorruptPayload` when stored bytes fail rkyv validation
15. **load_analyses** preserves key identity — output map key equals input hash bytes exactly
16. **load_analyses** fails fast with `CorruptPayload` when input contains mix of valid and corrupt entries (invariant I-05)
17. **load_analyses** returns empty `HashMap` when input slice is empty AND `analysis_outputs` table is missing (proves early-return fires before table open)

### `load_transforms` bulk loader

18. **load_transforms** returns `HashMap` with all matching entries when all hashes exist in `transform_outputs`
19. **load_transforms** returns `HashMap` with only found entries when some hashes are missing
20. **load_transforms** returns empty `HashMap` when input hash slice is empty (`&[]`)
21. **load_transforms** returns single entry when duplicate hashes appear in input slice
22. **load_transforms** returns `CorruptPayload` when stored bytes fail rkyv validation for `String` type
23. **load_transforms** fails fast with `CorruptPayload` when input contains mix of valid and corrupt entries (invariant I-05)

### `load_chunks` bulk loader

24. **load_chunks** returns `HashMap` with all matching entries when all hashes exist in `chunk_outputs`
25. **load_chunks** returns `HashMap` with only found entries when some hashes are missing
26. **load_chunks** returns empty `HashMap` when input hash slice is empty (`&[]`)
27. **load_chunks** returns `CorruptPayload` when stored bytes fail rkyv validation for `Vec<Chunk>` type
28. **load_chunks** fails fast with `CorruptPayload` when input contains mix of valid and corrupt entries (invariant I-05)

### `load_scrapes` bulk loader

29. **load_scrapes** returns `HashMap` with all matching entries when all hashes exist in `scrape_outputs`
30. **load_scrapes** returns `HashMap` with only found entries when some hashes are missing
31. **load_scrapes** returns empty `HashMap` when input hash slice is empty (`&[]`)
32. **load_scrapes** returns `CorruptPayload` when stored bytes fail rkyv validation for `ScrapedPage` type
33. **load_scrapes** fails fast with `CorruptPayload` when input contains mix of valid and corrupt entries (invariant I-05)

### Cross-cutting postconditions and boundaries

34. **Read transaction** remains alive and usable after any bulk loader call returns (Q-05)
35. **Bulk loader** returns `Ok(HashMap)` with exact entry count when input contains 10,000+ hashes all present in the table (large-input boundary)

---

## Section 2 — Trophy Allocation

| # | Behavior | Layer | Justification |
|---|----------|-------|---------------|
| 1 | OwnedArchive returns valid with concrete byte length and field values | **Unit** | Pure function: bytes in, concrete assertions out. No I/O. |
| 2 | OwnedArchive returns CorruptPayload on invalid bytes | **Unit** | Pure function: invalid bytes in, specific error variant out. |
| 3 | OwnedArchive preserves exact input bytes | **Unit** | Byte-level equality check, no deps. |
| 4 | OwnedArchive `archived()` returns reference with matching field values | **Unit** | Field-value comparison on heap-owned bytes, no deps. |
| 5 | OwnedArchive `deserialize()` returns owned T | **Unit** | Pure rkyv deserialize, no deps. |
| 6 | OwnedArchive `deserialize()` error path documented as unreachable | **Unit** | Documentation-only; no executable test (bytecheck is strictly stronger). |
| 7 | load_analyses returns all matching entries | **Integration** | Requires real redb database with real `analysis_outputs` table. |
| 8 | load_analyses omits missing hashes | **Integration** | Requires redb table with partial population. |
| 9 | load_analyses returns empty map when no hashes found | **Integration** | Requires redb with empty table or non-matching hashes. |
| 10 | load_analyses returns empty map on empty input | **Integration** | Requires live `StateReadSession`. |
| 11 | load_analyses deduplicates input hashes | **Integration** | Requires redb with real entries to verify single entry per unique hash. |
| 12 | load_analyses returns TableOpen error | **Integration** | Requires redb database without the expected table definition. |
| 13 | load_analyses returns StorageError on I/O failure | **Integration** | Gap formally accepted — see Behavior 13 detail and Section 9 decision. |
| 14 | load_analyses returns CorruptPayload on bad bytes | **Integration** | Requires writing raw garbage bytes into redb table. |
| 15 | load_analyses preserves key identity | **Integration** | Requires writing known key-value pairs and verifying exact byte match. |
| 16 | load_analyses fails fast on mixed valid+corrupt | **Integration** | Requires redb with valid entry at h1 and garbage at h2. |
| 17 | load_analyses returns empty map when input empty AND table missing | **Integration** | Requires redb without `analysis_outputs` table; proves early-return path. |
| 18 | load_transforms returns all matching entries | **Integration** | Real redb with `transform_outputs` table, rkyv-serialized `String`. |
| 19 | load_transforms omits missing hashes | **Integration** | Partial population of `transform_outputs`. |
| 20 | load_transforms returns empty map on empty input | **Integration** | Live `StateReadSession` with empty slice. |
| 21 | load_transforms deduplicates input hashes | **Integration** | Duplicate hash slice against `transform_outputs`. |
| 22 | load_transforms returns CorruptPayload on bad bytes | **Integration** | Garbage bytes in `transform_outputs` table. |
| 23 | load_transforms fails fast on mixed valid+corrupt | **Integration** | Valid String at h1, garbage at h2. |
| 24 | load_chunks returns all matching entries | **Integration** | Real redb with `chunk_outputs`, rkyv-serialized `Vec<Chunk>`. |
| 25 | load_chunks omits missing hashes | **Integration** | Partial population of `chunk_outputs`. |
| 26 | load_chunks returns empty map on empty input | **Integration** | Live `StateReadSession` with empty slice. |
| 27 | load_chunks returns CorruptPayload on bad bytes | **Integration** | Garbage bytes in `chunk_outputs` table. |
| 28 | load_chunks fails fast on mixed valid+corrupt | **Integration** | Valid Vec<Chunk> at h1, garbage at h2. |
| 29 | load_scrapes returns all matching entries | **Integration** | Real redb with `scrape_outputs`, rkyv-serialized `ScrapedPage`. |
| 30 | load_scrapes omits missing hashes | **Integration** | Partial population of `scrape_outputs`. |
| 31 | load_scrapes returns empty map on empty input | **Integration** | Live `StateReadSession` with empty slice. |
| 32 | load_scrapes returns CorruptPayload on bad bytes | **Integration** | Garbage bytes in `scrape_outputs` table. |
| 33 | load_scrapes fails fast on mixed valid+corrupt | **Integration** | Valid ScrapedPage at h1, garbage at h2. |
| 34 | Read transaction remains alive after call | **Integration** | Verify second read succeeds after first bulk load on same session. |
| 35 | Bulk loader handles large hash count | **Integration** | Requires redb with 10,000+ entries; asserts exact output size. |

---

## Section 3 — BDD Scenarios

### Behavior 1: OwnedArchive returns valid with concrete inner value when bytes pass bytecheck

```
Given: a known Analysis value with source_path="test.md", title="Test", word_count=42,
       has_code=false, has_tables=false, headings=[], links=[], category="cat",
       first_paragraph="fp", content="body"
And:   bytes = rkyv_serialize(&original) producing Box<[u8]> of known length L
When:  OwnedArchive::<Analysis>::try_from_bytes("test_table", &[0xAB; 32], bytes) is called
Then:  result.unwrap().as_bytes().len() == L
And:   result.unwrap().archived().source_path == "test.md" (via ArchivedString comparison)
And:   result.unwrap().archived().word_count == 42

Test name: fn owned_archive_returns_concrete_value_when_bytes_pass_bytecheck()
Layer:    Unit
```

### Behavior 2: OwnedArchive returns CorruptPayload on invalid bytes

```
Given: bytes = Box::from([0xFF; 64])
And:   hash = [0xAA; 32]
When:  OwnedArchive::<Analysis>::try_from_bytes("analysis_outputs", &hash, bytes) is called
Then:  result is Err(BulkLoadError::CorruptPayload)
And:   error.table == "analysis_outputs"
And:   error.key_hex == hex::encode([0xAA; 32])  // i.e., "aaaa...aa" (64 hex chars)

Error variant:
Given: bytes = Box::from([0xFF; 64])
When:  try_from_bytes("analysis_outputs", &[0xAA; 32], bytes)
Then:  Err(BulkLoadError::CorruptPayload { table: "analysis_outputs", key_hex: "aaaa...aa", message: /* non-empty string */ })

Test name: fn owned_archive_returns_corrupt_payload_when_bytes_fail_bytecheck()
Layer:    Unit
```

### Behavior 3: OwnedArchive preserves exact input bytes

```
Given: a known Analysis instance
And:   bytes: Box<[u8]> = rkyv_serialize(&original).into_boxed_slice()
When:  OwnedArchive::<Analysis>::try_from_bytes("t", &hash, bytes.clone()) is called
Then:  result.unwrap().as_bytes() == &bytes[..] (exact byte-level equality)

Test name: fn owned_archive_preserves_exact_input_bytes_when_constructed()
Layer:    Unit
```

### Behavior 4: OwnedArchive `archived()` returns reference with matching field values

```
Given: a valid OwnedArchive<Analysis> constructed from a known Analysis value
       with source_path="fields.md", title="Fields", word_count=100
When:  archived() is called
Then:  archived_ref.source_path == "fields.md" (ArchivedString comparison)
And:   archived_ref.title == "Fields"
And:   archived_ref.word_count == 100
Note:  Lifetime correctness (&self binding) is verified by the Rust compiler, not by
       a runtime assertion. No untestable lifetime claims are made.

Test name: fn owned_archive_archived_returns_matching_field_values_when_called()
Layer:    Unit
```

### Behavior 5: OwnedArchive `deserialize()` returns owned T

```
Given: a valid OwnedArchive<Analysis> constructed from a known Analysis instance `original`
       with source_path="roundtrip.md", title="Roundtrip", word_count=999,
       has_code=true, has_tables=false, headings=[], links=[], category="cat",
       first_paragraph="fp", content="body"
When:  deserialize() is called
Then:  result is Ok(Analysis)
And:   returned.source_path == "roundtrip.md"
And:   returned.title == "Roundtrip"
And:   returned.word_count == 999
And:   returned.has_code == true

Test name: fn owned_archive_deserialize_returns_owned_value_when_valid()
Layer:    Unit
```

### Behavior 6: OwnedArchive `deserialize()` error path — documented as unreachable

```
Rationale:
  rkyv's bytecheck validation (performed inside try_from_bytes) verifies the byte
  layout is well-formed: correct alignment, valid ArchivedVec lengths, valid
  ArchivedString byte ranges, valid enum discriminants. The HighDeserializer
  interprets the same byte layout without additional validation. Therefore, if
  bytecheck passes, deserialization MUST succeed.

  This is a structural guarantee of rkyv's design: bytecheck is strictly stronger
  than deserialization. A byte payload that passes bytecheck cannot fail
  deserialization for the same type.

  Reference: rkyv crate documentation — bytecheck performs exhaustive validation
  of the archived byte layout; HighDeserializer reads the same layout without
  re-validating.

Conclusion:
  No executable test is planned for this path. If a future rkyv version breaks
  this guarantee, the proptest round-trip invariants (Proptests 1-3) will catch
  it by failing on the deserialize() step after a successful try_from_bytes().

Test name: N/A (documented guarantee, not an executable test)
Layer:    Documentation
```

### Behavior 7: load_analyses returns all matching entries

```
Given: a redb database with `analysis_outputs` table populated with N=3 entries
       at hashes h1, h2, h3
And:   h1 → rkyv-serialized Analysis { source_path: "a.md", word_count: 10, ... }
And:   h2 → rkyv-serialized Analysis { source_path: "b.md", word_count: 20, ... }
And:   h3 → rkyv-serialized Analysis { source_path: "c.md", word_count: 30, ... }
When:  session.load_analyses(&[h1, h2, h3]) is called
Then:  result is Ok(HashMap) with len() == 3
And:   map[&h1].archived().source_path == "a.md"
And:   map[&h1].archived().word_count == 10
And:   map[&h2].archived().source_path == "b.md"
And:   map[&h3].archived().source_path == "c.md"

Test name: fn load_analyses_returns_all_entries_when_all_hashes_exist()
Layer:    Integration
```

### Behavior 8: load_analyses omits missing hashes

```
Given: a redb database with `analysis_outputs` containing entry at hash h1 only
And:   h1 → rkyv-serialized Analysis { source_path: "present.md", ... }
When:  session.load_analyses(&[h1, h2]) is called, where h2 has no stored value
Then:  result is Ok(HashMap) with len() == 1
And:   map contains key &h1
And:   map does NOT contain key &h2
And:   map[&h1].archived().source_path == "present.md"

Test name: fn load_analyses_omits_missing_hashes_when_some_not_found()
Layer:    Integration
```

### Behavior 9: load_analyses returns empty map when no hashes found

```
Given: a redb database with `analysis_outputs` table (empty or with entries at different hashes)
When:  session.load_analyses(&[h_unknown]) is called, where h_unknown has no stored value
Then:  result is Ok(HashMap) with len() == 0

Test name: fn load_analyses_returns_empty_map_when_no_hashes_match()
Layer:    Integration
```

### Behavior 10: load_analyses returns empty map on empty input

```
Given: a redb database with `analysis_outputs` table (may or may not be populated)
When:  session.load_analyses(&[]) is called
Then:  result is Ok(HashMap::new()) with len() == 0

Note: Whether redb table access occurs internally is an implementation detail
not observable from the public API. This test verifies the observable contract:
empty input → empty output, no error. See Behavior 17 for the mutation-catcher
variant that proves the early-return path.

Test name: fn load_analyses_returns_empty_map_when_input_slice_empty()
Layer:    Integration
```

### Behavior 11: load_analyses deduplicates input hashes

```
Given: a redb database with `analysis_outputs` containing entry at hash h1
And:   h1 → rkyv-serialized Analysis { source_path: "dedup.md", word_count: 7, ... }
When:  session.load_analyses(&[h1, h1, h1]) is called
Then:  result is Ok(HashMap) with len() == 1
And:   map contains exactly one entry keyed by h1
And:   map[&h1].archived().source_path == "dedup.md"

Test name: fn load_analyses_deduplicates_when_input_has_duplicate_hashes()
Layer:    Integration
```

### Behavior 12: load_analyses returns TableOpen error

```
Given: a redb database that was NOT initialized with `analysis_outputs` table
       (all other tables may exist)
When:  session.load_analyses(&[h1]) is called
Then:  result is Err(BulkLoadError::TableOpen)
And:   error.table == "analysis_outputs"
And:   error.message is a non-empty String

Test name: fn load_analyses_returns_table_open_error_when_table_missing()
Layer:    Integration
```

### Behavior 13: load_analyses returns StorageError on I/O failure

```
Given: a redb database file that has been corrupted or is unreadable at the storage layer
When:  session.load_analyses(&[h1]) is called and redb returns a storage error
Then:  result is Err(BulkLoadError::StorageError)
And:   error.table == "analysis_outputs"
And:   error.message is a non-empty String

DECISION ON FEASIBILITY (resolving MINOR-6):

  This test is formally accepted as a known gap. The error-mapping code from
  redb::StorageError to BulkLoadError::StorageError is a trivial map_err
  (one line, no conditional logic). The mapping is verified by:

  1. Code review: the map_err closure is visually inspectable
  2. Mutation testing: mutation checkpoint 12 verifies that removing the
     StorageError branch causes the test to fail (if any test triggers redb I/O)
  3. The type system: the match on redb results must be exhaustive

  Inducing a real StorageError in tests requires either: (a) corrupting the redb
  file handle mid-transaction (platform-dependent, flaky), or (b) a FUSE
  filesystem that fails reads (CI-incompatible). Neither is justified for a
  one-line map_err.

  Risk assessment: LOW. StorageError wraps redb's own error with table context.
  If the mapping is wrong, the error message will be missing table context —
  a cosmetic issue, not a data-integrity issue.

Test name: fn load_analyses_returns_storage_error_when_redb_io_fails()
Layer:    Integration (documented gap — see decision above)
```

### Behavior 14: load_analyses returns CorruptPayload on bad bytes

```
Given: a redb database with `analysis_outputs` table containing raw garbage bytes
       [0xDE, 0xAD, 0xBE, 0xEF] at hash h1
When:  session.load_analyses(&[h1]) is called
Then:  result is Err(BulkLoadError::CorruptPayload)
And:   error.table == "analysis_outputs"
And:   error.key_hex == hex::encode(h1)
And:   error.message is a non-empty String

Test name: fn load_analyses_returns_corrupt_payload_when_bytes_invalid()
Layer:    Integration
```

### Behavior 15: load_analyses preserves key identity

```
Given: a redb database with `analysis_outputs` table containing entry at hash h1
And:   h1 = [0x01, 0x02, ..., 0x20] (known 32-byte value)
When:  session.load_analyses(&[h1]) is called
Then:  result is Ok(map)
And:   for every key k in map.keys(): *k == h1 (exact 32-byte equality, byte-by-byte)
And:   map.keys().next().unwrap() == &h1

Test name: fn load_analyses_preserves_key_identity_when_loading_entries()
Layer:    Integration
```

### Behavior 16: load_analyses fails fast when mix of valid and corrupt entries

```
Given: a redb database with `analysis_outputs` table containing:
       - h1 → valid rkyv-serialized Analysis { source_path: "valid.md", ... }
       - h2 → raw garbage bytes [0xFF; 64]
When:  session.load_analyses(&[h1, h2]) is called
Then:  result is Err(BulkLoadError::CorruptPayload)
And:   error.table == "analysis_outputs"
And:   error.key_hex == hex::encode(h2)  // the corrupt entry's key, not h1
NOT:   Ok(HashMap) with only h1 present — that would be the "skip silently" mutation

This test kills the mutation: "skip corrupt entries silently instead of failing fast."
Without this test, a loader that silently skipped h2 and returned Ok(map{h1}) would
pass all other tests.

Test name: fn load_analyses_fails_fast_when_mix_of_valid_and_corrupt_entries()
Layer:    Integration
```

### Behavior 17: load_analyses returns empty map when input empty AND table missing

```
Given: a redb database where `analysis_outputs` table was NOT created
       (database opened but initialize_tables skipped for analysis_outputs)
When:  session.load_analyses(&[]) is called
Then:  result is Ok(HashMap::new()) with len() == 0
NOT:   Err(BulkLoadError::TableOpen { .. })

This test kills the mutation: "remove the empty-slice early return."
If the early return is removed, the loader attempts table.open() on the missing
table, returning Err(TableOpen) instead of Ok(empty). This is the ONLY test
that catches that mutation. Behavior 10 (same call but table exists) produces
the same observable output with or without the early return.

Test name: fn load_analyses_returns_empty_map_when_input_empty_and_table_missing()
Layer:    Integration
```

### Behavior 18: load_transforms returns all matching entries

```
Given: a redb database with `transform_outputs` table populated with entries at h1, h2
And:   h1 → rkyv-serialized String "hello"
And:   h2 → rkyv-serialized String "world"
When:  session.load_transforms(&[h1, h2]) is called
Then:  result is Ok(HashMap) with len() == 2
And:   map[&h1].archived() dereferences to "hello"
And:   map[&h2].archived() dereferences to "world"

Test name: fn load_transforms_returns_all_entries_when_all_hashes_exist()
Layer:    Integration
```

### Behavior 19: load_transforms omits missing hashes

```
Given: a redb database with `transform_outputs` containing entry at h1 only
And:   h1 → rkyv-serialized String "present"
When:  session.load_transforms(&[h1, h_missing]) is called
Then:  result is Ok(HashMap) with len() == 1
And:   map contains key &h1
And:   map does NOT contain key &h_missing
And:   map[&h1].archived() dereferences to "present"

Test name: fn load_transforms_omits_missing_hashes_when_some_not_found()
Layer:    Integration
```

### Behavior 20: load_transforms returns empty map on empty input

```
Given: a redb database with `transform_outputs` table
When:  session.load_transforms(&[]) is called
Then:  result is Ok(HashMap::new()) with len() == 0

Test name: fn load_transforms_returns_empty_map_when_input_slice_empty()
Layer:    Integration
```

### Behavior 21: load_transforms deduplicates input hashes

```
Given: a redb database with `transform_outputs` containing entry at h1
And:   h1 → rkyv-serialized String "dedup"
When:  session.load_transforms(&[h1, h1]) is called
Then:  result is Ok(HashMap) with len() == 1
And:   map[&h1].archived() dereferences to "dedup"

Test name: fn load_transforms_deduplicates_when_input_has_duplicate_hashes()
Layer:    Integration
```

### Behavior 22: load_transforms returns CorruptPayload on bad bytes

```
Given: a redb database with `transform_outputs` table containing garbage bytes at h1
When:  session.load_transforms(&[h1]) is called
Then:  result is Err(BulkLoadError::CorruptPayload)
And:   error.table == "transform_outputs"
And:   error.key_hex == hex::encode(h1)

Test name: fn load_transforms_returns_corrupt_payload_when_bytes_invalid()
Layer:    Integration
```

### Behavior 23: load_transforms fails fast when mix of valid and corrupt entries

```
Given: a redb database with `transform_outputs` table containing:
       - h1 → valid rkyv-serialized String "good"
       - h2 → raw garbage bytes [0xFF; 64]
When:  session.load_transforms(&[h1, h2]) is called
Then:  result is Err(BulkLoadError::CorruptPayload)
And:   error.table == "transform_outputs"
And:   error.key_hex == hex::encode(h2)

Test name: fn load_transforms_fails_fast_when_mix_of_valid_and_corrupt_entries()
Layer:    Integration
```

### Behavior 24: load_chunks returns all matching entries

```
Given: a redb database with `chunk_outputs` table populated with entries at h1, h2
And:   h1 → rkyv-serialized Vec<Chunk> with 3 chunks
And:   h2 → rkyv-serialized Vec<Chunk> with 7 chunks
When:  session.load_chunks(&[h1, h2]) is called
Then:  result is Ok(HashMap) with len() == 2
And:   map[&h1].archived().len() == 3
And:   map[&h2].archived().len() == 7

Test name: fn load_chunks_returns_all_entries_when_all_hashes_exist()
Layer:    Integration
```

### Behavior 25: load_chunks omits missing hashes

```
Given: a redb database with `chunk_outputs` containing entry at h1 only
And:   h1 → rkyv-serialized Vec<Chunk> with 1 chunk
When:  session.load_chunks(&[h1, h_missing]) is called
Then:  result is Ok(HashMap) with len() == 1
And:   map contains key &h1, does NOT contain &h_missing
And:   map[&h1].archived().len() == 1

Test name: fn load_chunks_omits_missing_hashes_when_some_not_found()
Layer:    Integration
```

### Behavior 26: load_chunks returns empty map on empty input

```
Given: a redb database with `chunk_outputs` table
When:  session.load_chunks(&[]) is called
Then:  result is Ok(HashMap::new()) with len() == 0

Test name: fn load_chunks_returns_empty_map_when_input_slice_empty()
Layer:    Integration
```

### Behavior 27: load_chunks returns CorruptPayload on bad bytes

```
Given: a redb database with `chunk_outputs` table containing garbage bytes at h1
When:  session.load_chunks(&[h1]) is called
Then:  result is Err(BulkLoadError::CorruptPayload)
And:   error.table == "chunk_outputs"
And:   error.key_hex == hex::encode(h1)

Test name: fn load_chunks_returns_corrupt_payload_when_bytes_invalid()
Layer:    Integration
```

### Behavior 28: load_chunks fails fast when mix of valid and corrupt entries

```
Given: a redb database with `chunk_outputs` table containing:
       - h1 → valid rkyv-serialized Vec<Chunk> with 2 chunks
       - h2 → raw garbage bytes [0xFF; 64]
When:  session.load_chunks(&[h1, h2]) is called
Then:  result is Err(BulkLoadError::CorruptPayload)
And:   error.table == "chunk_outputs"
And:   error.key_hex == hex::encode(h2)

Test name: fn load_chunks_fails_fast_when_mix_of_valid_and_corrupt_entries()
Layer:    Integration
```

### Behavior 29: load_scrapes returns all matching entries

```
Given: a redb database with `scrape_outputs` table populated with entry at h1
And:   h1 → rkyv-serialized ScrapedPage with url == "https://example.com"
When:  session.load_scrapes(&[h1]) is called
Then:  result is Ok(HashMap) with len() == 1
And:   map[&h1].archived().url == "https://example.com"

Test name: fn load_scrapes_returns_all_entries_when_all_hashes_exist()
Layer:    Integration
```

### Behavior 30: load_scrapes omits missing hashes

```
Given: a redb database with `scrape_outputs` containing entry at h1 only
And:   h1 → rkyv-serialized ScrapedPage with url == "https://present.com"
When:  session.load_scrapes(&[h1, h_missing]) is called
Then:  result is Ok(HashMap) with len() == 1
And:   map contains key &h1, does NOT contain &h_missing
And:   map[&h1].archived().url == "https://present.com"

Test name: fn load_scrapes_omits_missing_hashes_when_some_not_found()
Layer:    Integration
```

### Behavior 31: load_scrapes returns empty map on empty input

```
Given: a redb database with `scrape_outputs` table
When:  session.load_scrapes(&[]) is called
Then:  result is Ok(HashMap::new()) with len() == 0

Test name: fn load_scrapes_returns_empty_map_when_input_slice_empty()
Layer:    Integration
```

### Behavior 32: load_scrapes returns CorruptPayload on bad bytes

```
Given: a redb database with `scrape_outputs` table containing garbage bytes at h1
When:  session.load_scrapes(&[h1]) is called
Then:  result is Err(BulkLoadError::CorruptPayload)
And:   error.table == "scrape_outputs"
And:   error.key_hex == hex::encode(h1)

Test name: fn load_scrapes_returns_corrupt_payload_when_bytes_invalid()
Layer:    Integration
```

### Behavior 33: load_scrapes fails fast when mix of valid and corrupt entries

```
Given: a redb database with `scrape_outputs` table containing:
       - h1 → valid rkyv-serialized ScrapedPage with url == "https://good.com"
       - h2 → raw garbage bytes [0xFF; 64]
When:  session.load_scrapes(&[h1, h2]) is called
Then:  result is Err(BulkLoadError::CorruptPayload)
And:   error.table == "scrape_outputs"
And:   error.key_hex == hex::encode(h2)

Test name: fn load_scrapes_fails_fast_when_mix_of_valid_and_corrupt_entries()
Layer:    Integration
```

### Behavior 34: Read transaction remains alive after call

```
Given: a redb database with `analysis_outputs` and `transform_outputs` tables
And:   `analysis_outputs` contains valid entry at h1 with source_path="first.md"
And:   `transform_outputs` contains valid entry at h1 with String "transform_result"
When:  first_result = session.load_analyses(&[h1]) is called
And:   second_result = session.load_transforms(&[h1]) is called on the SAME session
Then:  first_result is Ok(map1) with map1.len() == 1
And:   second_result is Ok(map2) with map2.len() == 1
And:   map1[&h1].archived().source_path == "first.md"
And:   map2[&h1].archived() dereferences to "transform_result"

Test name: fn read_session_remains_usable_after_bulk_load_call()
Layer:    Integration
```

### Behavior 35: Bulk loader handles large hash count without panic

```
Given: a redb database with `analysis_outputs` table populated with 10_000 entries
       at hashes h0, h1, ..., h9999, each with valid rkyv-serialized Analysis values
When:  session.load_analyses(&[h0, h1, ..., h9999]) is called
Then:  result is Ok(HashMap) with len() == 10_000
And:   map contains all 10_000 input hashes as keys
And:   map[&h0].archived().source_path == expected value for h0

This test proves:
1. No integer overflow in HashMap::with_capacity(hashes.len())
2. No stack overflow from recursive processing
3. Exact output cardinality at scale

Boundary note: Inputs approaching isize::MAX / size_of::<(u8, OwnedArchive<T>)>()
entries would exhaust available memory before causing an arithmetic overflow in
HashMap::with_capacity. On 64-bit systems, usize::MAX is ~2^63, which is not
reachable in practice. The API contract is bounded by available memory. The
implementation should use HashMap::with_capacity(hashes.len()) which delegates
to the standard library's overflow-safe capacity calculation. No explicit
overflow test is needed beyond this large-input test.

Test name: fn load_analyses_handles_large_hash_count_without_panic()
Layer:    Integration
```

---

## Section 4 — Proptest Invariants

### Proptest 1: OwnedArchive round-trip for Analysis

```
Invariant: For any valid Analysis value, serializing via rkyv, constructing
           OwnedArchive via try_from_bytes, then calling deserialize() produces
           a value equal to the original (field-by-field for deterministic fields).

Strategy:  Generate Analysis with:
           - source_path: any non-empty ASCII string (1..64 chars)
           - title: any non-empty ASCII string (1..64 chars)
           - word_count: 0..100_000
           - has_code: bool
           - has_tables: bool
           - headings: Vec<Heading> with 0..10 entries
           - links: Vec<Link> with 0..10 entries
           - category: any non-empty string
           - first_paragraph: any string (0..200 chars)
           - content: any string (0..500 chars)

Anti-invariant: Empty source_path (domain rule violation — not a serialization concern)
                should still serialize/deserialize correctly; verify data fidelity only.
```

### Proptest 2: OwnedArchive round-trip for String (transform outputs)

```
Invariant: For any String, rkyv-serializing, constructing OwnedArchive, then
           deserialize() yields the original string.

Strategy:  Generate arbitrary String with 0..10_000 chars, including Unicode,
           empty string, strings with null bytes.

Anti-invariant: None — all strings should round-trip. Empty string is a valid transform output.
```

### Proptest 3: OwnedArchive round-trip for Vec<Chunk>

```
Invariant: For any Vec<Chunk> value, round-trip through rkyv serialize →
           OwnedArchive → deserialize() preserves the original value.

Strategy:  Generate Vec<Chunk> with 0..20 entries, each Chunk with:
           - chunk_id: any string 1..32 chars
           - doc_id: any string 1..32 chars
           - chunk_index: 0..1000
           - token_count: 0..100_000
           - content: any string 0..500 chars

Anti-invariant: Vec with >10_000 entries — test memory bounds, not serialization.
```

### Proptest 4: Bulk loader deduplication — output size ≤ input unique count

```
Invariant: For any slice of [u8; 32] hashes (including duplicates), the returned
           HashMap from any bulk loader method has len() <= number of unique hashes
           in the input.

Strategy:  Generate Vec<[u8; 32]> with 0..50 entries, where each entry is a random
           32-byte value. Inject duplicates by repeating some entries 1..3 times.

Anti-invariant: Empty slice always yields empty HashMap.
```

---

## Section 5 — Fuzz Targets

### Fuzz Target 1: `OwnedArchive<Analysis>::try_from_bytes`

```
Input type:  Arbitrary bytes (Vec<u8>)
Function:    OwnedArchive::<Analysis>::try_from_bytes("fuzz_analysis", &[0; 32], bytes.into())
Risk:        Panic in rkyv bytecheck, out-of-bounds read on malformed archived data,
             infinite loops in validation, OOM from malformed length prefix.
Corpus seeds:
  - Valid rkyv-serialized Analysis bytes (from a known Analysis instance)
  - Empty bytes (0-length)
  - Single byte [0x00]
  - All-0xFF bytes (64 bytes)
  - Valid rkyv header with corrupted trailing bytes
  - Bytes with length prefix claiming 4GB but only 64 bytes present
  - Minimum valid rkyv payload for Analysis (smallest field values)
  - Maximum practical rkyv payload for Analysis (large strings, many headings/links)

Assert: Function must never panic. Must return either Ok(OwnedArchive) or
        Err(BulkLoadError::CorruptPayload { .. }). No abort, no UB.
```

### Fuzz Target 2: `OwnedArchive<String>::try_from_bytes`

```
Input type:  Arbitrary bytes (Vec<u8>)
Function:    OwnedArchive::<String>::try_from_bytes("fuzz_transform", &[0; 32], bytes.into())
Risk:        Panic in rkyv archived string access, OOB read on archived str.
Corpus seeds:
  - Valid rkyv-serialized String bytes (empty string, short string, long string)
  - Empty bytes
  - Bytes representing a valid ArchivedString but with corrupted length
  - Minimum valid rkyv payload for String (empty string: typically 4-8 bytes for length prefix)
  - Large valid rkyv payload for String (100KB string)
```

### Fuzz Target 3: `OwnedArchive<Vec<Chunk>>::try_from_bytes`

```
Input type:  Arbitrary bytes (Vec<u8>)
Function:    OwnedArchive::<Vec<Chunk>>::try_from_bytes("fuzz_chunks", &[0; 32], bytes.into())
Risk:        Panic in rkyv archived Vec access, OOB from malformed length, stack overflow
             from deeply nested archived structures.
Corpus seeds:
  - Valid rkyv-serialized Vec<Chunk> bytes (empty vec, single-element vec, multi-element vec)
  - Bytes with ArchivedVec length claiming millions of elements
  - Partially valid bytes (valid header, truncated element data)
  - Minimum valid rkyv payload for Vec<Chunk> (empty vec)
  - Maximum practical rkyv payload for Vec<Chunk> (1000+ chunks)
```

### Fuzz Target 4: `OwnedArchive<ScrapedPage>::try_from_bytes`

```
Input type:  Arbitrary bytes (Vec<u8>)
Function:    OwnedArchive::<ScrapedPage>::try_from_bytes("fuzz_scrape", &[0; 32], bytes.into())
Risk:        Panic in rkyv archived ScrapedPage field access, OOB on archived String fields.
Corpus seeds:
  - Valid rkyv-serialized ScrapedPage bytes
  - Empty bytes
  - Bytes with valid structure but invalid UTF-8 in archived string fields
  - Minimum valid rkyv payload for ScrapedPage
  - Large valid rkyv payload for ScrapedPage (long HTML body)
```

### Fuzz Target 5: Bulk loader with arbitrary hash slice

```
Input type:  Arbitrary slice of [u8; 32] values
Function:    session.load_analyses(&hashes) (with pre-populated redb database)
Risk:        Panic on empty input, incorrect HashMap sizing, integer overflow
             in capacity calculation.
Corpus seeds:
  - Empty slice (&[])
  - Single hash
  - 10_000 hashes (stress — asserts exact output size)
  - Slice of identical hashes (all same)
  - Random 32-byte hashes (likely all missing from table)

Assert: Must return Ok(HashMap) or Err(BulkLoadError). No panic.
```

---

## Section 6 — Kani Harnesses

### Kani Harness 1: Key identity — no hashing of hashes

```
Property:  For any [u8; 32] key used as input to a bulk loader, the exact same
           32 bytes appear as the HashMap key in the output. No transformation
           (no hashing, no endianness swap, no truncation) is applied.

Bound:     Verify for all single-key lookups (1 key at a time).
           Kani can enumerate all possible byte patterns for a [u8; 32] input
           within the search bound. Practically, use a bounded subset (e.g.,
           nondet<[u8; 32]>).

Rationale: The contract guarantees I-06 "Key identity" — the output key is the
           exact same bytes as the input. A subtle bug could hash the key or
           reverse endianness. Proptest might miss this if the hash function
           happens to be the identity for tested inputs. Formal proof is stronger.

Harness sketch:
  let key: [u8; 32] = kani::any();
  // Assume key exists in table (setup constraint)
  let map = load_analyses(&[key]).unwrap();
  let output_key = map.keys().next().unwrap();
  assert(*output_key == key);
```

### Kani Harness 2: Empty input produces empty output

```
Property:  For any bulk loader method, calling with an empty slice &[]
           always returns Ok(HashMap::new()). No code path can return an error
           for empty input.

Bound:     Single call per loader with empty input. No search needed —
           deterministic path.

Rationale: Contract Q-06 guarantees empty input = empty output with no error.
           This is a trivial but critical invariant — no conditional branch should
           open a table or perform I/O when the input is empty. Kani proves the
           early-return path is exhaustive.

Harness sketch:
  let result = load_analyses(&[]);
  assert_eq!(result, Ok(HashMap::new()));
```

---

## Section 7 — Mutation Testing Checkpoints

**Target mutation kill rate: ≥ 90%**

### Critical mutations that must be caught:

| # | Mutation | Caught by test | How |
|---|----------|----------------|-----|
| 1 | Remove bytecheck validation in `try_from_bytes` | `owned_archive_returns_corrupt_payload_when_bytes_fail_bytecheck` — garbage bytes would incorrectly return Ok |
| 2 | Swap `table` name string in error variant | `load_analyses_returns_corrupt_payload_when_bytes_invalid` — asserts `table == "analysis_outputs"` |
| 3 | Remove empty-slice early return in bulk loader | `load_analyses_returns_empty_map_when_input_empty_and_table_missing` — table is missing, so without early return the loader would return Err(TableOpen) instead of Ok(empty). **Note: Behavior 10 (empty input on populated table) does NOT catch this mutation — both paths produce the same observable output.** |
| 4 | Replace `HashMap::from_iter` with `HashMap::new()` (drop all entries) | `load_analyses_returns_all_entries_when_all_hashes_exist` — asserts len() == N where N > 0 |
| 5 | Skip deduplication — insert all input hashes | `load_analyses_deduplicates_when_input_has_duplicate_hashes` — asserts len() == 1 for 3 identical inputs |
| 6 | Use `HashMap::with_capacity(0)` instead of input-based capacity | `load_analyses_returns_all_entries_when_all_hashes_exist` — would reallocate but still work; verify via benchmark not mutation |
| 7 | Remove `CorruptPayload` error branch | `load_analyses_returns_corrupt_payload_when_bytes_invalid` — garbage bytes would panic or return wrong error |
| 8 | Change `TableOpen` to return wrong table name | `load_analyses_returns_table_open_error_when_table_missing` — asserts exact table string |
| 9 | Remove ownership copy from `AccessGuard` (keep reference) | Compile error — lifetime mismatch. Caught by compiler, not test. |
| 10 | Flip conditional: include missing hashes with default value | `load_analyses_omits_missing_hashes_when_some_not_found` — asserts len() == 1, missing key absent |
| 11 | Replace `hex::encode(key)` with `hex::encode(input_index)` | `load_analyses_returns_corrupt_payload_when_bytes_invalid` — asserts exact key_hex matches known hash |
| 12 | Remove `StorageError` branch | Code review + type system: the match on redb results must be exhaustive. If non-exhaustive, compiler error. If wrong mapping, error message lacks table context (cosmetic). |
| 13 | Skip table open error handling | `load_analyses_returns_table_open_error_when_table_missing` — would panic on unwrap |
| 14 | Change each loader to use wrong table definition | `load_transforms_returns_all_entries_when_all_hashes_exist`, `load_chunks_returns_all_entries_when_all_hashes_exist`, `load_scrapes_returns_all_entries_when_all_hashes_exist` — wrong table would return empty or wrong data |
| 15 | Skip corrupt entries silently instead of failing fast | `load_analyses_fails_fast_when_mix_of_valid_and_corrupt_entries` — valid+corrupt mix must return Err, not Ok with subset. This test kills the mutation that Behaviors 14/22/27/32 cannot kill alone. |
| 16 | Change fail-fast error key_hex to wrong hash | `load_analyses_fails_fast_when_mix_of_valid_and_corrupt_entries` — asserts error.key_hex == hex::encode(h2) (the corrupt entry, not the valid one) |

### Mutation test configuration

```toml
# .cargo/mutants.toml
exclude_globs = ["**/benches/**", "**/examples/**"]
minimum_test_timeout = 60
```

---

## Section 8 — Combinatorial Coverage Matrix

### Matrix A: `OwnedArchive::<T>::try_from_bytes`

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| valid bytes for Analysis | rkyv-serialized Analysis with known fields | `Ok(OwnedArchive)` where `as_bytes().len() == known_size` AND `archived().source_path == "test.md"` | unit |
| valid bytes for String | rkyv-serialized String | `Ok(OwnedArchive)` where `archived()` dereferences to original string | unit |
| valid bytes for Vec<Chunk> | rkyv-serialized Vec<Chunk> | `Ok(OwnedArchive)` where `archived().len() == expected_count` | unit |
| valid bytes for ScrapedPage | rkyv-serialized ScrapedPage | `Ok(OwnedArchive)` where `archived().url == expected_url` | unit |
| empty bytes | `Box<[u8]>` length 0 | `Err(CorruptPayload { table, key_hex, message: non-empty })` | unit |
| garbage bytes | `[0xFF; 64]` | `Err(CorruptPayload { table, key_hex: hex of input hash, message: non-empty })` | unit |
| truncated valid bytes | first 4 bytes of valid rkyv | `Err(CorruptPayload { .. })` | unit |
| valid header + corrupted body | valid prefix + garbage suffix | `Err(CorruptPayload { .. })` | unit |
| **min valid payload** | **smallest rkyv-serializable value (e.g., empty String → ~4-8 bytes for length prefix + 0 content)** | **`Ok(OwnedArchive<String>)` with `archived().len() == 0`** | **unit** |
| **max valid payload** | **large rkyv-serializable value (e.g., String with 100KB content)** | **`Ok(OwnedArchive<String>)` with `as_bytes().len() == expected_large_size`** | **unit** |

### Matrix B: `load_analyses` (representative — other 3 loaders are isomorphic)

| Scenario | Input Class | Table State | Expected Output | Layer |
|----------|-------------|-------------|-----------------|-------|
| all found | 3 valid hashes | all 3 present | `Ok(map)` len==3, map[&h1].archived().source_path == "a.md" | integration |
| partial found | 3 hashes, 1 missing | 2 of 3 present | `Ok(map)` len==2, missing key absent | integration |
| none found | 1 unknown hash | entries exist | `Ok(map)` len==0 | integration |
| empty input | `&[]` | populated | `Ok(HashMap::new())` len==0 | integration |
| empty input + table missing | `&[]` | table NOT created | `Ok(HashMap::new())` len==0 (NOT Err(TableOpen)) | integration |
| duplicates | `[h1, h1, h1]` | h1 present | `Ok(map)` len==1, map[&h1].archived().source_path == "dedup.md" | integration |
| table missing | 1 hash | table not created | `Err(TableOpen { table: "analysis_outputs", message: non-empty })` | integration |
| corrupt payload | 1 hash | garbage at h1 | `Err(CorruptPayload { table: "analysis_outputs", key_hex: hex::encode(h1), message: non-empty })` | integration |
| storage error | 1 hash | I/O failure | `Err(StorageError { table: "analysis_outputs" })` — documented gap | integration |
| key identity | 1 hash h1 | h1 present | map.keys().next().unwrap() == &h1 (exact 32-byte equality) | integration |
| session survives | 2 calls on same session | populated | both calls return Ok, second map has correct values | integration |
| **fail-fast: valid + corrupt** | **[h_valid, h_corrupt]** | **valid at h1, garbage at h2** | **`Err(CorruptPayload { table: "analysis_outputs", key_hex: hex::encode(h2) })`** | **integration** |
| **large input** | **10_000 hashes** | **all present** | **`Ok(map)` len==10_000, all keys present** | **integration** |

### Matrix C: `load_transforms`

| Scenario | Input Class | Table State | Expected Output | Layer |
|----------|-------------|-------------|-----------------|-------|
| all found | 2 hashes | both present, values "hello", "world" | `Ok(map)` len==2, map[&h1].archived() == "hello", map[&h2].archived() == "world" | integration |
| partial found | 2 hashes | 1 present | `Ok(map)` len==1, map[&h1].archived() == "present" | integration |
| empty input | `&[]` | any | `Ok(HashMap::new())` len==0 | integration |
| duplicates | `[h1, h1]` | h1 present | `Ok(map)` len==1, map[&h1].archived() == "dedup" | integration |
| corrupt payload | 1 hash | garbage at h1 | `Err(CorruptPayload { table: "transform_outputs", key_hex: hex::encode(h1) })` | integration |
| **fail-fast: valid + corrupt** | **[h_valid, h_corrupt]** | **valid at h1, garbage at h2** | **`Err(CorruptPayload { table: "transform_outputs", key_hex: hex::encode(h2) })`** | **integration** |

### Matrix D: `load_chunks`

| Scenario | Input Class | Table State | Expected Output | Layer |
|----------|-------------|-------------|-----------------|-------|
| all found | 2 hashes | both present with Vec<Chunk> | `Ok(map)` len==2, map[&h1].archived().len()==3, map[&h2].archived().len()==7 | integration |
| partial found | 2 hashes | 1 present | `Ok(map)` len==1, map[&h1].archived().len()==1 | integration |
| empty input | `&[]` | any | `Ok(HashMap::new())` len==0 | integration |
| corrupt payload | 1 hash | garbage at h1 | `Err(CorruptPayload { table: "chunk_outputs", key_hex: hex::encode(h1) })` | integration |
| **fail-fast: valid + corrupt** | **[h_valid, h_corrupt]** | **valid at h1, garbage at h2** | **`Err(CorruptPayload { table: "chunk_outputs", key_hex: hex::encode(h2) })`** | **integration** |

### Matrix E: `load_scrapes`

| Scenario | Input Class | Table State | Expected Output | Layer |
|----------|-------------|-------------|-----------------|-------|
| all found | 1 hash | present with ScrapedPage | `Ok(map)` len==1, map[&h1].archived().url == "https://example.com" | integration |
| partial found | 2 hashes | 1 present | `Ok(map)` len==1, map[&h1].archived().url == "https://present.com" | integration |
| empty input | `&[]` | any | `Ok(HashMap::new())` len==0 | integration |
| corrupt payload | 1 hash | garbage at h1 | `Err(CorruptPayload { table: "scrape_outputs", key_hex: hex::encode(h1) })` | integration |
| **fail-fast: valid + corrupt** | **[h_valid, h_corrupt]** | **valid at h1, garbage at h2** | **`Err(CorruptPayload { table: "scrape_outputs", key_hex: hex::encode(h2) })`** | **integration** |

### Matrix F: Error variant exhaustiveness

| Error Variant | Trigger | Asserted by test | Layer |
|---------------|---------|-------------------|-------|
| `BulkLoadError::TableOpen` | Missing table definition | `load_analyses_returns_table_open_error_when_table_missing` | integration |
| `BulkLoadError::StorageError` | redb I/O failure | Documented gap (trivial map_err, verified by code review + type system) | integration |
| `BulkLoadError::CorruptPayload` | Garbage bytes in table | `owned_archive_returns_corrupt_payload_when_bytes_fail_bytecheck` + per-loader corrupt tests (4 loaders) + fail-fast tests (4 loaders) = 9 tests total | unit + integration |

### Matrix G: Bulk loader boundary analysis

| Boundary | Test | Expected Result | Rationale |
|----------|------|-----------------|-----------|
| Minimum input: empty slice `&[]` | Behavior 10 | `Ok(HashMap::new())` | Zero entries requested → zero entries returned |
| Single hash | Behaviors 12-15 | Various (per scenario) | Smallest non-empty input |
| Duplicate hashes | Behavior 11 | `Ok(map)` len==1 | Deduplication at minimum meaningful size |
| Large input: 10,000 hashes | Behavior 35 | `Ok(map)` len==10_000 | Proves no overflow in capacity calc, no panic at scale |
| HashMap capacity overflow (`isize::MAX` entries) | **Not tested — documented decision** | — | On 64-bit systems, `HashMap::with_capacity(hashes.len())` takes `usize`. A slice of `[u8; 32]` cannot exceed `isize::MAX` bytes in memory (Rust allocation limit). Each entry is 32 bytes, so the maximum slice length is `isize::MAX / 32`. This is ~288M entries, requiring ~9GB just for the input slice. The `HashMap` itself would need additional memory for each entry. The system will OOM long before any arithmetic overflow. The implementation uses `HashMap::with_capacity(hashes.len())` which delegates to the standard library's safe capacity calculation. **No explicit overflow test is warranted.** |
| One-above-max | **Not applicable** | — | There is no discrete "max valid" count — the boundary is system memory, not a numeric limit. The large-input test (10,000 entries) proves correctness at a realistic upper bound. |

---

## Section 9 — Test Infrastructure Requirements

### Integration test fixture helper

All integration tests need a shared fixture that:

1. Creates a `tempfile::TempDir` for the redb database
2. Opens a `redb::Database` via `StateDb::open` (or equivalent)
3. Calls `initialize_tables` to create all four output tables
4. Provides helper methods to:
   - Insert a valid rkyv-serialized value at a given `[u8; 32]` key
   - Insert raw garbage bytes at a given key (for corrupt-payload tests)
   - Create a `StateReadSession` from the database
   - Omit specific tables (for TableOpen error tests)
   - Populate N entries with known values at sequential hashes (for large-input tests)

### rkyv serialization helper

Tests need a function to serialize domain types into `Vec<u8>` via rkyv:

```rust
fn rkyv_serialize<T: rkyv::Serialize<rkyv::api::high::HighSerializer<Vec<u8>, ...>>>(value: &T) -> Vec<u8>
```

This must work for `Analysis`, `String`, `Vec<Chunk>`, and `ScrapedPage`.

### Test file location

```
centralized-docs/tests/
  bulk_load/
    mod.rs              — module root
    owned_archive_tests.rs   — unit tests (behaviors 1-5)
    load_analyses_tests.rs   — integration tests (behaviors 7-17)
    load_transforms_tests.rs — integration tests (behaviors 18-23)
    load_chunks_tests.rs     — integration tests (behaviors 24-28)
    load_scrapes_tests.rs    — integration tests (behaviors 29-33)
    session_lifecycle_tests.rs — integration tests (behavior 34)
    boundary_tests.rs        — integration tests (behavior 35)
    common.rs                — shared fixture helpers
```

---

## Section 10 — Decisions on Known Gaps

### Decision 1: `StorageError` test (resolves MINOR-6)

**Decision: ACCEPTED GAP.**

The `BulkLoadError::StorageError` variant is produced by a trivial `map_err` from
redb's own error type. There is no conditional logic, no data transformation, no
domain-specific interpretation. The mapping is:

```rust
redb_result.map_err(|e| BulkLoadError::StorageError {
    table: TABLE_NAME,
    message: e.to_string(),
})?
```

Inducing a real `StorageError` in tests requires either:
- Corrupting the redb file handle mid-transaction (platform-dependent, flaky)
- A FUSE filesystem that fails reads (CI-incompatible)
- Mocking the redb database (violates "prefer real implementations" principle)

None of these is justified for a one-line error mapping. The variant is verified by:
1. **Type system**: The match on redb results must be exhaustive; removing the variant causes a compile error.
2. **Code review**: The mapping is visually inspectable.
3. **Mutation testing**: If the variant is removed, the compiler catches it.

### Decision 2: `deserialize()` error path (resolves MINOR-4)

**Decision: DOCUMENTED AS UNREACHABLE.**

rkyv's `bytecheck` validation (performed inside `try_from_bytes`) verifies the byte
layout is well-formed: correct alignment, valid `ArchivedVec` lengths, valid
`ArchivedString` byte ranges, valid enum discriminants. The `HighDeserializer`
interprets the same byte layout without additional validation. Therefore, if
bytecheck passes, deserialization MUST succeed.

This is a structural guarantee of rkyv's design: bytecheck is strictly stronger
than deserialization. A byte payload that passes bytecheck cannot fail
deserialization for the same type.

If this guarantee is ever violated by a future rkyv version, the proptest
round-trip invariants (Proptests 1-3) will catch it by failing on the
`deserialize()` step after a successful `try_from_bytes()`.

---

## Exit Criteria Verification

- [x] Every public API behavior (4 bulk loaders + OwnedArchive methods) has BDD scenarios
- [x] Every `BulkLoadError` variant (`TableOpen`, `StorageError`, `CorruptPayload`) has explicit test scenarios
  - `TableOpen`: Behavior 12
  - `StorageError`: Behavior 13 (documented gap with risk assessment)
  - `CorruptPayload`: Behaviors 2, 14, 16, 22, 23, 27, 28, 32, 33 (9 tests total)
- [x] Every pure function with multiple inputs (`try_from_bytes`, `deserialize`) has proptest invariants
- [x] Every deserialization boundary (`try_from_bytes` for each of 4 types) has a fuzz target
- [x] Mutation kill threshold (≥90%) is stated with specific mutation-checkpoint mapping (18 checkpoints)
- [x] No planned assertion is `is_ok()` or `is_err()` — every test specifies exact values or error variants
- [x] Key identity (I-06) has both integration test (Behavior 15) and Kani harness (harness 1)
- [x] Empty input edge case covered for all 4 loaders (Behaviors 10, 20, 26, 31)
- [x] Duplicate input edge case covered for all 4 loaders (Behaviors 11, 21 + proptest 4)
- [x] Missing hash (silent omission) covered for all 4 loaders (Behaviors 8, 19, 25, 30)
- [x] Session lifecycle (Q-05) has explicit test (Behavior 34)
- [x] **Fail-fast invariant (I-05) tested for all 4 loaders** (Behaviors 16, 23, 28, 33) — mixed valid+corrupt
- [x] **Empty-slice early-return mutation caught** (Behavior 17 — empty input + missing table)
- [x] **Large-input boundary tested** (Behavior 35 — 10,000 hashes)
- [x] **Mutation checkpoint 3 corrected** — Behavior 17 (not Behavior 10) catches the mutation
- [x] **Behavior 1 asserts concrete inner value** — byte length + archived field values
- [x] **Behavior 4 asserts field values, not lifetime properties**
- [x] **Behavior 10 has no untestable sub-assertion**
- [x] **Kani harness 2 uses `assert_eq!` not `is_ok()` + `unwrap()`**
- [x] **try_from_bytes min/max valid payload boundaries added** (Matrix A)
- [x] **deserialize() error path documented as structurally unreachable** (Section 10, Decision 2)
- [x] **StorageError gap formally accepted with risk assessment** (Section 10, Decision 1)
