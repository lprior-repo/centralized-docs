bead_id: cdocs-pi4
bead_title: data: remove LRU backend from `CacheBackendInner` after state migration
phase: state-1.5-test-plan
updated_at: 2026-04-03T00:00:00Z

# Test Plan: Remove LRU Backend from `CacheBackendInner`

## Summary
- Behaviors identified: 32
- Trophy allocation: 10 unit / 17 integration / 3 e2e/static
- Existing tests mapped: 30 (see Section 2a)
- New planned test items: 42 (32 BDD + 5 proptest + 3 fuzz + 2 kani)
- Combined total: 72 test items (30 existing + 42 new)
- Proptest invariants: 5
- Fuzz targets: 3
- Kani harnesses: 2
- Mutation threshold: ≥90% kill rate

## Scope Notes

This bead is a **backend migration cleanup**: remove the `Lru` variant from `CacheBackendInner` and replace `CacheBackend::Memory` internals with redb's `InMemoryBackend`. No public API signatures change. All tests verify **behavioral regression** — that every public behavior works identically after the migration — plus **removal verification** — that LRU artifacts are truly gone.

The critical new behavior is: **in-memory caches no longer have a 10,000-entry capacity limit**. LRU silent eviction is eliminated. In-memory redb is bounded only by process memory.

---

## 1. Behavior Inventory

| # | Behavior |
|---|----------|
| B01 | `DocCache::open` returns initialized cache when `CacheConfig::in_memory()` is used |
| B02 | `DocCache::open` returns initialized cache when `CacheConfig::new(path)` is used |
| B03 | `DocCache::open` creates parent directories when file path has non-existent parent |
| B04 | `DocCache::get` returns `Some(deserialized_value)` when key exists in cache |
| B05 | `DocCache::get` returns `None` when key does not exist in cache |
| B06 | `DocCache::put` stores value and subsequent `get` retrieves identical value |
| B07 | `DocCache::put` returns `Err(CacheError::KeyTooLarge)` when key exceeds 256 bytes |
| B08 | `DocCache::put` returns `Err(CacheError::ValueTooLarge)` when serialized value exceeds 50 MB |
| B09 | `DocCache::put` succeeds when key is exactly 256 bytes |
| B10 | `DocCache::get_or_compute` returns cached value without calling compute when key exists |
| B11 | `DocCache::get_or_compute` calls compute once, caches result, returns it when key missing |
| B12 | `DocCache::get` returns `None` when cache type is disabled (skip behavior) |
| B13 | `DocCache::put` is a no-op (no error) when cache type is disabled |
| B14 | `DocCache::get_snapshot` / `put_snapshot` roundtrips correctly |
| B15 | `DocCache::clear_all` empties all tables — `stats()` returns all zeros |
| B16 | `DocCache::stats` returns accurate per-table entry counts |
| B17 | `DocCache` in-memory backend stores more than 10,000 entries without eviction |
| B18 | `DocCache` in-memory backend drops all data when the `DocCache` is dropped |
| B19 | `DocCache` file backend persists data across open/close cycles |
| B20 | `CacheConfig::disable` / `enable` builder chaining works correctly |
| B21 | `content_hash`, `url_hash`, `path_hash`, `composite_hash` are deterministic and pure |
| B22 | `composite_hash` produces different output when input part order changes |
| B23 | Typed convenience methods (`get_document`, `put_document`, `get_scrape`, etc.) delegate correctly |
| B24 | No code in the crate references `lru`, `LruCache`, `parking_lot`, or `RwLock` after migration |
| B25 | `DocCache::open` returns `Err(CacheError::BackendError)` when redb file is corrupted |
| B26 | `DocCache::put` returns `Err(CacheError::BackendError)` when writing to a read-only directory |
| B27 | `DocCache::get` returns `Err(CacheError::BackendError)` when redb internal state is corrupted |
| B28 | `DocCache::get_or_compute` propagates compute error without caching when compute returns `Err` |
| B29 | `DocCache::put` accepts zero-length (empty) serialized value |
| B30 | `DocCache::clear_all` followed by `put`/`get` roundtrip succeeds (table reinitialization) |
| B31 | `CacheConfig::enable` re-enables a single type without affecting other disabled types |
| B32 | `DocCache::put`/`get` roundtrip works for all 6 cache types individually |

---

## 2. Trophy Allocation

### 2a. Existing Test Coverage (30 tests in `cache/mod.rs`)

| # | Existing Test | Behavior(s) Covered | Layer |
|---|---------------|---------------------|-------|
| E01 | `test_cache_basic_roundtrip` | B06 | integration |
| E02 | `test_cache_miss_returns_none` | B05 | integration |
| E03 | `test_cache_struct_value` | B06 | integration |
| E04 | `test_cache_stats` | B16 | integration |
| E05 | `test_get_or_compute_caches_result` | B10, B11 | integration |
| E06 | `test_content_hash_consistency` | B21 | unit |
| E07 | `test_content_hash_different_inputs` | B21 | unit |
| E08 | `test_content_hash_is_newtype` | B21 | unit |
| E09 | `test_clear_all` | B15 | integration |
| E10 | `test_in_memory_cache` | B01 | integration |
| E11 | `test_disabled_cache_skips_operations` | B12, B13 | unit |
| E12 | `test_key_too_large_returns_error` | B07 | unit |
| E13 | `test_value_too_large_returns_error` | B08 | unit |
| E14 | `test_key_at_max_size_succeeds` | B09 | unit |
| E15 | `test_scrape_key_size_validation` | B07 (scrape path) | unit |
| E16 | `test_transform_key_size_validation` | B07 (transform path) | unit |
| E17 | `test_snapshot_roundtrip` | B14 | integration |
| E18 | `test_builder_pattern_disable` | B20 | unit |
| E19 | `test_url_hash_returns_content_hash` | B21 | unit |
| E20 | `test_path_hash_returns_content_hash` | B21 | unit |
| E21 | `test_cache_open_idempotent_single_open_close_cycle` | B19 | integration |
| E22 | `test_cache_open_idempotent_ten_open_cycles` | B19 | integration |
| E23 | `test_cache_open_idempotent_hundred_open_cycles` | B19 | integration |
| E24 | `test_cache_open_idempotent_consecutive_opens_without_close` | B19 | integration |
| E25 | `test_cache_open_idempotent_all_table_types` | B19, B23 | integration |
| E26 | `test_cache_open_idempotent_data_integrity` | B19 | integration |
| E27 | `test_composite_hash_order_matters` | B22 | unit |
| E28 | `test_composite_hash_deterministic` | B22 | unit |
| E29 | `test_analysis_roundtrip` | B23 (analysis path) | integration |
| E30 | `test_chunk_roundtrip` | B23 (chunk path) | integration |

**Gaps in existing coverage** (behaviors with zero existing tests):
- B03 (parent directory creation)
- B17 (capacity >10,000 — the critical regression test)
- B18 (in-memory drops on exit)
- B24 (no LRU references)
- B25, B26, B27 (`CacheError::BackendError` — all three variants)
- B28 (compute error propagation)
- B29 (empty value boundary)
- B30 (clear_all reinitialization)
- B31 (enable without affecting others)
- B32 (all 6 cache types roundtrip)

### 2b. New Planned Test Allocation

| # | Behavior | Layer | Justification |
|---|----------|-------|---------------|
| B01 | open in-memory | integration | Exercises real redb InMemoryBackend creation and table initialization |
| B02 | open file-backed | integration | Exercises real redb file database creation, mkdir, table init |
| B03 | open creates parent dirs | integration | Real filesystem I/O — **no existing test** |
| B04 | get returns cached value | integration | Real redb read transaction + serde deserialization |
| B05 | get returns None on miss | integration | Real redb read path |
| B06 | put/get roundtrip | integration | Real redb write transaction + read back |
| B07 | put rejects oversized key | unit | Pure validation logic (`validate_key_size`) exercised via public API |
| B08 | put rejects oversized value | unit | Pure validation logic (`validate_value_size`) exercised via public API |
| B09 | put accepts max-size key | unit | Boundary value validation |
| B10 | get_or_compute cache hit | integration | Real redb read path |
| B11 | get_or_compute cache miss | integration | Real redb write + read path |
| B12 | get returns None when disabled | unit | `EnabledTypes` bit logic is pure |
| B13 | put no-op when disabled | unit | `EnabledTypes` bit logic is pure |
| B14 | snapshot roundtrip | integration | Real redb write/read via SNAPSHOTS_TABLE |
| B15 | clear_all empties tables | integration | Real redb delete_table + reinit |
| B16 | stats returns accurate counts | integration | Real redb table_len queries |
| B17 | in-memory no capacity limit | integration | Stores >10k entries, verifies no eviction — real redb InMemoryBackend. **Critical regression — no existing test** |
| B18 | in-memory drops on exit | integration | Lifecycle behavior — requires real DocCache drop. **No existing test** |
| B19 | file persists across cycles | integration | Real filesystem persistence |
| B20 | disable/enable chaining | unit | Pure bit manipulation on `EnabledTypes` |
| B21 | hash functions deterministic | unit | Pure SHA-256 computation, no I/O |
| B22 | composite_hash order matters | unit | Pure SHA-256 over concatenated input |
| B23 | typed convenience delegates | integration | Exercises real redb path through delegation |
| B24 | no LRU references remain | e2e/static | Full crate grep for removed symbols. **No existing test** |
| B25 | BackendError on corrupt file | integration | Real redb failure on corrupted database. **No existing test** |
| B26 | BackendError on read-only dir | integration | Real filesystem permission failure. **No existing test** |
| B27 | BackendError on corrupt read | integration | Real redb transaction failure on corrupted data. **No existing test** |
| B28 | get_or_compute propagates error | unit | Error propagation logic — compute failure path. **No existing test** |
| B29 | put accepts empty value | unit | Boundary: zero-length serialized value. **No existing test** |
| B30 | clear_all then roundtrip | integration | Table reinitialization after clear. **No existing test** |
| B31 | enable single type selectively | unit | `EnabledTypes` enable without side effects. **No existing test** |
| B32 | all 6 types roundtrip | integration | Exercises every `table_for_type` mapping. **No existing test** |

### 2c. Ratio Summary

**Trophy target**: 22 public functions × 5 = **110 tests** (guideline per Testing Trophy).

| Category | Count | Notes |
|----------|-------|-------|
| Existing tests (cache/mod.rs) | 30 | All pass pre-migration; must pass post-migration |
| New BDD scenarios | 32 | Fill gaps identified above |
| New proptest invariants | 5 | Pure function property checks |
| New fuzz targets | 3 | Parser/boundary fuzzing |
| New Kani harnesses | 2 | Formal verification of critical bounds |
| **Combined total** | **72** | |

**Ratio**: 72 / 22 = **3.3x**

**Justification for deviation from 5x guideline**:

This is a **deletion bead** (NG-6: the code change itself does not add tests). The 30 existing tests provide baseline behavioral coverage for 15 of the 32 behaviors. The new plan adds 42 items focused exclusively on:
1. **Regression gaps** — 12 behaviors with zero existing tests (B03, B17, B18, B24, B25–B32)
2. **Error variant coverage** — 3 `CacheError::BackendError` scenarios that had zero coverage
3. **Boundary completeness** — empty value (B29), reinitialization (B30)
4. **Formal verification** — Kani harnesses for security boundaries

The remaining gap to 110 is accounted for by the fact that many public functions are thin delegation wrappers (e.g., `get_document` delegates to `get`; `get_scrape` delegates to `get`). Testing each delegate independently adds test count but not behavioral coverage. The existing idempotency stress tests (E21–E26) exercise multiple behaviors per test (5 tests covering B19 alone). Expanding these into 25+ granular tests would inflate count without improving coverage.

The plan achieves **100% public API coverage**, **100% error variant coverage**, and targets **≥90% mutation kill rate**.

---

## 3. BDD Scenarios

### Behavior B01: DocCache::open returns initialized cache when CacheConfig::in_memory()
```
Given: no pre-existing state
When:  DocCache::open(CacheConfig::in_memory()) is called
Then:  returns Ok(DocCache) — a valid, usable cache instance
And:   cache.put_document(b"k", &"v") returns Ok(())
And:   cache.get_document(b"k") returns Ok(Some("v".to_string()))
```
`fn doccache_open_returns_usable_cache_when_in_memory_config()`

### Behavior B02: DocCache::open returns initialized cache when CacheConfig::new(path)
```
Given: a temporary directory path
When:  DocCache::open(CacheConfig::new(path)) is called
Then:  returns Ok(DocCache) — a valid, usable cache instance
And:   a redb file exists at the given path
```
`fn doccache_open_returns_usable_cache_when_file_config()`

### Behavior B03: DocCache::open creates parent directories for file path
```
Given: a file path whose parent directory does not exist (e.g., /tmp/nonexistent/deep/cache.redb)
When:  DocCache::open(CacheConfig::new(path)) is called
Then:  returns Ok(DocCache)
And:   the parent directory exists on the filesystem (verified via std::fs::metadata)
```
`fn doccache_open_creates_parent_directories_when_path_missing()`

### Behavior B04: DocCache::get returns deserialized value when key exists
```
Given: an open DocCache (in-memory) with a stored value at key b"existing"
When:  cache.get::<String>(CacheType::Document, b"existing") is called
Then:  returns Ok(Some("stored_value".to_string()))
```
`fn doccache_get_returns_value_when_key_exists()`

### Behavior B05: DocCache::get returns None when key does not exist
```
Given: an open DocCache (in-memory) with no entries
When:  cache.get::<String>(CacheType::Document, b"nonexistent") is called
Then:  returns Ok(None)
```
`fn doccache_get_returns_none_when_key_missing()`

### Behavior B06: DocCache::put/get roundtrip preserves value
```
Given: an open DocCache (in-memory)
When:  cache.put_document(b"key", &TestData { name: "test".into(), count: 42 }) then
       cache.get_document::<TestData>(b"key")
Then:  returns Ok(Some(TestData { name: "test".into(), count: 42 }))
```
`fn doccache_put_then_get_returns_identical_struct()`

### Behavior B07: DocCache::put rejects oversized key
```
Given: an open DocCache (in-memory)
When:  cache.put_document(&vec![0u8; 257], &"value") is called
Then:  returns Err(CacheError::KeyTooLarge { size: 257, max: 256 })
```
`fn doccache_put_returns_key_too_large_when_key_exceeds_256_bytes()`

### Behavior B08: DocCache::put rejects oversized value
```
Given: an open DocCache (in-memory)
When:  cache.put_document(b"key", &"x".repeat(50 * 1024 * 1024 + 1)) is called
Then:  returns Err(CacheError::ValueTooLarge { size: 52428801, max: 52428800 })
```
`fn doccache_put_returns_value_too_large_when_value_exceeds_50mb()`

### Behavior B09: DocCache::put accepts key at exactly MAX_KEY_SIZE
```
Given: an open DocCache (in-memory)
When:  cache.put_document(&vec![0u8; 256], &"value") is called
Then:  returns Ok(())
And:   cache.get_document::<String>(&vec![0u8; 256]) returns Ok(Some("value".to_string()))
```
`fn doccache_put_succeeds_when_key_is_exactly_256_bytes()`

### Behavior B10: DocCache::get_or_compute returns cached value on hit
```
Given: an open DocCache (in-memory) with value "cached" at key b"k"
When:  cache.get_or_compute(CacheType::Document, b"k", || { panic!("should not call") })
Then:  returns Ok("cached".to_string())
And:   compute closure was never invoked
```
`fn doccache_get_or_compute_returns_cached_value_without_calling_compute()`

### Behavior B11: DocCache::get_or_compute computes, caches, returns on miss
```
Given: an open DocCache (in-memory) with no entry at key b"k"
When:  cache.get_or_compute(CacheType::Document, b"k", || Ok("computed".to_string()))
Then:  returns Ok("computed".to_string())
And:   a subsequent get_or_compute with a different compute returns Ok("computed".to_string()) — not the new compute result
```
`fn doccache_get_or_compute_calls_compute_once_and_caches_result()`

### Behavior B12: DocCache::get returns None when cache type disabled
```
Given: an open DocCache with CacheType::Document disabled
When:  cache.get::<String>(CacheType::Document, b"any_key") is called
Then:  returns Ok(None) — even if data was previously stored in that type
```
`fn doccache_get_returns_none_when_cache_type_disabled()`

### Behavior B13: DocCache::put is no-op when cache type disabled
```
Given: an open DocCache with CacheType::Document disabled
When:  cache.put_document(b"key", &"value") is called
Then:  returns Ok(())
And:   cache.get_document::<String>(b"key") returns Ok(None) — value was not stored
```
`fn doccache_put_is_noop_when_cache_type_disabled()`

### Behavior B14: DocCache::get_snapshot / put_snapshot roundtrip
```
Given: an open DocCache (in-memory)
When:  cache.put_snapshot(b"snap_key", &SnapshotData { url: "https://x.com".into(), count: 7 })
       then cache.get_snapshot::<SnapshotData>(b"snap_key")
Then:  returns Ok(Some(SnapshotData { url: "https://x.com".into(), count: 7 }))
```
`fn doccache_snapshot_put_then_get_returns_identical_value()`

### Behavior B15: DocCache::clear_all empties all tables
```
Given: an open DocCache (in-memory) with entries in Document and Scrape tables
When:  cache.clear_all() is called
Then:  returns Ok(())
And:   cache.stats() returns Ok(CacheStats {
         document_entries: 0,
         scrape_entries: 0,
         transform_entries: 0,
         snapshot_entries: 0,
         analysis_entries: 0,
         chunk_entries: 0,
       })
```
`fn doccache_clear_all_empties_all_tables()`

### Behavior B16: DocCache::stats returns accurate per-table counts
```
Given: an open DocCache (in-memory) with 3 document entries and 1 scrape entry
When:  cache.stats() is called
Then:  returns Ok(CacheStats {
         document_entries: 3,
         scrape_entries: 1,
         transform_entries: 0,
         snapshot_entries: 0,
         analysis_entries: 0,
         chunk_entries: 0,
       })
```
`fn doccache_stats_returns_accurate_per_table_counts()`

### Behavior B17: In-memory backend stores >10,000 entries without eviction
```
Given: an open DocCache (in-memory)
When:  10_001 entries are written via put_document with unique keys b"key_00000" through b"key_10000"
Then:  cache.stats().document_entries == 10_001
And:   cache.get_document::<String>(b"key_00000") returns Ok(Some("value_00000".to_string())) — first entry still readable
```

**Holzmann Rule 2 compliance**: The 10,001-entry write uses a bounded `for i in 0..=10_000` loop with a fixed ceiling of 10,001 iterations. This is a data-generation loop, not a control-flow loop — each iteration is a deterministic, independent cache write with no conditional exit. The iteration count is a fixed constant derived from `DEFAULT_LRU_CAPACITY + 1`, not a runtime value. Exception documented: bounded to exactly 10,001 iterations.

`fn doccache_in_memory_stores_over_10000_entries_without_eviction()`

**This is the critical regression test.** Under the old LRU backend, writing 10,001 entries would silently evict the first entry. Under redb InMemoryBackend, all entries persist.

### Behavior B18: In-memory backend drops all data on DocCache drop
```
Given: an open DocCache (in-memory) with cache.put_document(b"ephemeral", &"temp_data") stored
When:  the DocCache is dropped and a new DocCache::open(CacheConfig::in_memory()) is created
Then:  new_cache.stats() returns Ok(CacheStats {
         document_entries: 0,
         scrape_entries: 0,
         transform_entries: 0,
         snapshot_entries: 0,
         analysis_entries: 0,
         chunk_entries: 0,
       })
And:   new_cache.get_document::<String>(b"ephemeral") returns Ok(None)
```
`fn doccache_in_memory_drops_all_data_on_exit()`

### Behavior B19: File backend persists data across open/close cycles
```
Given: a file-backed DocCache with cache.put_document(b"persistent", &"persistence_test_value") stored
When:  the DocCache is dropped and a new DocCache::open(CacheConfig::new(same_path)) is created
Then:  reopened_cache.get_document::<String>(b"persistent") returns Ok(Some("persistence_test_value".to_string()))
```
`fn doccache_file_backend_persists_across_open_close_cycles()`

### Behavior B20: CacheConfig disable/enable builder chaining
```
Given: let mut config = CacheConfig::in_memory()
When:  config.disable(CacheType::Document).disable(CacheType::Scrape)
Then:  config.is_enabled(CacheType::Document) == false
And:   config.is_enabled(CacheType::Scrape) == false
And:   config.is_enabled(CacheType::Transform) == true
And:   config.is_enabled(CacheType::Snapshot) == true
And:   config.is_enabled(CacheType::Analysis) == true
And:   config.is_enabled(CacheType::Chunk) == true

When:  config.enable(CacheType::Document)
Then:  config.is_enabled(CacheType::Document) == true
And:   config.is_enabled(CacheType::Scrape) == false  — unchanged by enable(Document)
And:   config.is_enabled(CacheType::Transform) == true — unchanged
```
`fn cacheconfig_disable_enable_builder_chaining_works()`

### Behavior B21: Hash functions are deterministic and pure
```
Given: the same input bytes
When:  content_hash(b"test") is called twice
Then:  both calls return identical ContentHash values
And:   ContentHash::as_bytes() returns exactly 32 bytes
```
`fn content_hash_returns_same_value_for_same_input()`

```
Given: a URL string "https://example.com"
When:  url_hash("https://example.com") is called
Then:  assert_eq!(url_hash("https://example.com"), content_hash(b"https://example.com"))
```
`fn url_hash_equals_content_hash_of_url_bytes()`

```
Given: a filesystem path "/foo/bar.md"
When:  path_hash(Path::new("/foo/bar.md")) is called
Then:  assert_eq!(path_hash(Path::new("/foo/bar.md")), content_hash("/foo/bar.md".as_bytes()))
```
`fn path_hash_equals_content_hash_of_path_bytes()`

### Behavior B22: composite_hash is order-sensitive
```
Given: parts [b"hello", b"world"] vs [b"world", b"hello"]
When:  composite_hash is called on each
Then:  assert_ne!(composite_hash(&[b"hello", b"world"]), composite_hash(&[b"world", b"hello"]))
```
`fn composite_hash_produces_different_output_when_part_order_changes()`

### Behavior B23: Typed convenience methods delegate correctly
```
Given: an open DocCache (in-memory)
When:  cache.put_scrape(b"url_key", &"html_data") is called
       then cache.get_scrape::<String>(b"url_key")
Then:  returns Ok(Some("html_data".to_string()))
```
`fn doccache_scrape_put_then_get_returns_identical_value()`

```
Given: an open DocCache (in-memory)
When:  cache.put_transform(b"tx_key", &"tx_data") is called
       then cache.get_transform::<String>(b"tx_key")
Then:  returns Ok(Some("tx_data".to_string()))
```
`fn doccache_transform_put_then_get_returns_identical_value()`

### Behavior B24: No LRU references remain in the crate
```
Given: the crate source code after migration
When:  `cargo build 2>&1 | grep -i lru` is run
Then:  output is empty — no production or test code references `lru` crate
And:   `cargo build 2>&1 | grep -i parking_lot` returns empty
And:   `rg "LruCache|get_from_lru|put_to_lru|DEFAULT_LRU_CAPACITY" src/` returns no matches
```
`fn no_lru_references_remain_in_crate_source()`

### Behavior B25: DocCache::open returns BackendError when redb file is corrupted
```
Given: a file path at which a valid redb database was previously created and then closed
       and the file at that path has been overwritten with 64 bytes of 0xFF garbage
When:  DocCache::open(CacheConfig::new(corrupted_path)) is called
Then:  returns Err(CacheError::BackendError { operation: _, message: _ })
And:   the error matches CacheError::BackendError variant (match on variant, not is_err())
```

**Implementation note**: Use `std::fs::write(path, &[0xFFu8; 64])` to corrupt the file after normal cache creation and drop. The `Database::create` call should fail because redb cannot initialize a valid database atop corrupted data.

`fn doccache_open_returns_backend_error_when_redb_file_corrupted()`

### Behavior B26: DocCache::put returns BackendError when writing to read-only directory
```
Given: a read-only directory (created with std::fs::create_dir_all then chmod 0o555)
       and a file path inside that directory
When:  DocCache::open(CacheConfig::new(read_only_path)) is called
       then cache.put_document(b"key", &"value") is called
Then:  the open or put call returns Err(CacheError::BackendError { operation: _, message: _ })
And:   the error matches CacheError::BackendError variant (match on variant, not is_err())
```

**Implementation note**: The BackendError may trigger at `DocCache::open` (if `Database::create` fails) or at `put` (if `begin_write` or `commit` fails). The test must assert the variant regardless of which operation triggers it. Clean up by restoring directory permissions in a Drop guard.

`fn doccache_put_returns_backend_error_when_directory_read_only()`

### Behavior B27: DocCache::get returns BackendError when redb internal state is corrupted
```
Given: a file-backed DocCache with data stored at key b"corrupt_me"
       and the DocCache is dropped
       and the redb file has been partially corrupted (e.g., zeroing bytes at offset 4096)
When:  DocCache::open(CacheConfig::new(same_path)) succeeds (if create overwrites corruption)
       then cache.get_document::<String>(b"corrupt_me") is called
Then:  returns Err(CacheError::BackendError { operation: "begin_read" | "open_table", message: _ })
```

**Implementation note**: This scenario depends on redb's `Database::create` behavior. If `create` overwrites the corrupted file, this test may need adjustment to use `Database::open` instead. If `create` succeeds but the file retains corrupted pages, the read transaction will fail. The test must handle both cases: if open fails, assert BackendError on open; if open succeeds, assert BackendError on read. The key assertion is `matches!(result, Err(CacheError::BackendError { .. }))`.

`fn doccache_get_returns_backend_error_when_redb_data_corrupted()`

### Behavior B28: DocCache::get_or_compute propagates compute error without caching
```
Given: an open DocCache (in-memory) with no entry at key b"err_key"
When:  cache.get_or_compute(CacheType::Document, b"err_key", || Err(anyhow::anyhow!("compute failed"))) is called
Then:  returns Err(...) — the compute error propagates unchanged
And:   cache.get_document::<String>(b"err_key") returns Ok(None) — nothing was cached
```
`fn doccache_get_or_compute_propagates_compute_error_without_caching()`

### Behavior B29: DocCache::put accepts zero-length (empty) serialized value
```
Given: an open DocCache (in-memory)
When:  cache.put_document(b"empty_key", &"") is called
Then:  returns Ok(())
And:   cache.get_document::<String>(b"empty_key") returns Ok(Some("".to_string()))
```
`fn doccache_put_succeeds_when_value_is_empty_string()`

### Behavior B30: DocCache::clear_all followed by put/get roundtrip succeeds
```
Given: an open DocCache (in-memory) with 5 document entries stored
When:  cache.clear_all() is called
       then cache.put_document(b"after_clear", &"new_value") is called
       then cache.get_document::<String>(b"after_clear") is called
Then:  clear_all returns Ok(())
And:   put returns Ok(())
And:   get returns Ok(Some("new_value".to_string()))
And:   cache.stats().document_entries == 1
```
`fn doccache_clear_all_then_put_get_roundtrip_succeeds()`

### Behavior B31: CacheConfig::enable re-enables a single type without affecting others
```
Given: let mut config = CacheConfig::in_memory()
       config.disable(CacheType::Document).disable(CacheType::Scrape).disable(CacheType::Transform)
When:  config.enable(CacheType::Scrape)
Then:  config.is_enabled(CacheType::Scrape) == true
And:   config.is_enabled(CacheType::Document) == false — still disabled
And:   config.is_enabled(CacheType::Transform) == false — still disabled
And:   config.is_enabled(CacheType::Snapshot) == true — was never disabled
```
`fn cacheconfig_enable_single_type_without_affecting_others()`

### Behavior B32: DocCache::put/get roundtrip works for all 6 cache types individually
```
Given: an open DocCache (in-memory)
When:  For each (put_fn, get_fn, cache_type) in [
         (put_document, get_document, CacheType::Document),
         (put_scrape, get_scrape, CacheType::Scrape),
         (put_transform, get_transform, CacheType::Transform),
       ]:
       put_fn(b"type_key", &"type_value") then get_fn::<String>(b"type_key")
       assert_eq!(result, Ok(Some("type_value".to_string())))

       And for Snapshot, Analysis, Chunk via cache.put(CacheType::X, key, &value):
       cache.put(CacheType::Snapshot, b"snap", &"snap_val") → cache.get::<String>(CacheType::Snapshot, b"snap") == Ok(Some("snap_val".to_string()))
       cache.put(CacheType::Analysis, b"analysis", &"analysis_val") → cache.get::<String>(CacheType::Analysis, b"analysis") == Ok(Some("analysis_val".to_string()))
       cache.put(CacheType::Chunk, b"chunk", &"chunk_val") → cache.get::<String>(CacheType::Chunk, b"chunk") == Ok(Some("chunk_val".to_string()))
Then:  all 6 cache types return the stored value correctly
And:   cache.stats() returns CacheStats {
         document_entries: 1,
         scrape_entries: 1,
         transform_entries: 1,
         snapshot_entries: 1,
         analysis_entries: 1,
         chunk_entries: 1,
       }
```
`fn doccache_roundtrip_works_for_all_six_cache_types()`

---

## 4. Proptest Invariants

### Proptest: content_hash determinism
```
Invariant:  content_hash(x) == content_hash(x) for all x: &[u8]
Strategy:   proptest::collection::vec(any::<u8>(), 0..1024)
Anti-invariant: none — always holds
```

### Proptest: content_hash output size
```
Invariant:  content_hash(x).as_bytes().len() == 32 for all x: &[u8]
Strategy:   proptest::collection::vec(any::<u8>(), 0..4096)
Anti-invariant: none — always holds
```

### Proptest: composite_hash order sensitivity
```
Invariant:  For any non-empty parts with at least 2 elements where parts[0] != parts[1],
            composite_hash(&[parts[0], parts[1]]) != composite_hash(&[parts[1], parts[0]])
Strategy:   Two non-equal byte vectors of size 1..256
Anti-invariant: parts with identical elements — composite_hash([a, a]) == composite_hash([a, a])
```

### Proptest: put/get roundtrip for arbitrary serializable values
```
Invariant:  After put(key, value), get(key) returns Some(value) for any V: Serialize + DeserializeOwned + PartialEq
Strategy:   key = vec(any::<u8>(), 1..256), value = any::<String>() or custom strategy for struct types
Anti-invariant: key > 256 bytes → Err(CacheError::KeyTooLarge)
```

### Proptest: EnabledTypes disable/enable is idempotent
```
Invariant:  After disable(X).disable(X), X remains disabled
            After enable(X).enable(X), X remains enabled
Strategy:   any::<CacheType> (6 variants), repeated disable/enable 1..10 times
Anti-invariant: none — always holds
```

---

## 5. Fuzz Targets

### Fuzz Target: cache key deserialization roundtrip
```
Input type:  arbitrary &[u8] (keys fed to DocCache::get)
Risk:        serde_json::from_slice may panic on malformed data if stored by a compromised writer.
             However, since redb stores raw bytes and deserialization happens on read, malformed
             data should return Err (not panic).
Corpus seeds: empty slice, valid JSON string, truncated UTF-8, 256-byte key, 257-byte key
```

### Fuzz Target: validate_key_size boundary
```
Input type:  Vec<u8> of length 0..512
Risk:        Off-by-one in the 256-byte boundary check could accept oversized keys
Corpus seeds: vec![0u8; 0], vec![0u8; 255], vec![0u8; 256], vec![0u8; 257], vec![0u8; 512]
```

### Fuzz Target: BackendError resilience on arbitrary file corruption
```
Input type:  Vec<u8> of length 0..8192 (written as file content to a .redb path)
Risk:        redb may panic, OOM, or produce incorrect results when reading a corrupted
             database file. The cache must return CacheError::BackendError, never panic.
Corpus seeds: empty file, 64 bytes of 0xFF, 64 bytes of 0x00, valid redb header + garbage body,
             single byte, maximum redb page size (4096 bytes of 0xFF)
Implementation: Create a valid redb cache, drop it, overwrite the file with fuzz input,
                attempt DocCache::open, assert no panic and Err(CacheError::BackendError { .. }).
```

---

## 6. Kani Harnesses

### Kani Harness: validate_key_size rejects oversized keys
```
Property:   For all key: &[u8], validate_key_size(key) returns Err(CacheError::KeyTooLarge)
            if and only if key.len() > 256
Bound:      key length 0..300 (covers the boundary at 256)
Rationale:  This is a security boundary — oversized keys must never reach redb.
            Proptest can miss exact boundary; Kani proves it for all lengths in range.
```

### Kani Harness: validate_value_size rejects oversized values
```
Property:   For all bytes: &[u8], validate_value_size(bytes) returns Err(CacheError::ValueTooLarge)
            if and only if bytes.len() > 50 * 1024 * 1024
Bound:      value length 0..50MB + 1 (use abstract interpretation — may need to constrain to 0..1MB for tractability)
Rationale:  Prevents unbounded memory allocation in redb. Critical invariant.
```

---

## 7. Mutation Testing Checkpoints

### Critical Mutations to Catch

| Mutation | Which Test Catches It | Rationale |
|----------|----------------------|-----------|
| `validate_key_size` boundary changed from `>` to `>=` | B09 (`put succeeds at exactly 256 bytes`) | Off-by-one: 256-byte keys would be rejected |
| `validate_key_size` boundary changed from `>` to `==` | B07 (`put rejects at 257 bytes`) | Keys > 257 would pass validation |
| `EnabledTypes::is_enabled` bit test `!=` → `==` | B12 (`get returns None when disabled`) | Disabled types would appear enabled |
| `EnabledTypes::disable` `&=` → `\|=` | B20 (`disable/enable chaining` — asserts `is_enabled == false`) | Disabling would actually enable; concrete `== false` assertions catch this |
| `clear_all` skips delete_table calls | B15 (`clear_all empties all tables`) | Data would persist after clear |
| `get_or_compute` always calls compute | B10 (`cache hit skips compute`) | Cache hit optimization broken |
| `DocCache::open` skips `initialize_tables()` | B01 (`open returns usable cache`) | Tables wouldn't exist for first write |
| `read_cached` returns Ok(None) instead of deserializing | B04 (`get returns cached value`) | Cache would appear empty |
| `write_cached` skips `validate_key_size` | B07 (`put rejects oversized key`) | Validation bypass |
| `stats` returns hardcoded zeros | B16 (`stats returns accurate counts`) | Stats would always report empty |
| `InMemoryBackend` replaced with file backend in open() | B18 (`in-memory drops data on exit`) | In-memory would persist to disk |
| `put_to_lru` / `get_from_lru` still called | B24 (`no LRU references`) | Compile error — dead code |
| `get_or_compute` caches value even when compute returns Err | B28 (`compute error propagates without caching`) | Poisoned cache entry |
| `clear_all` fails to reinitialize tables | B30 (`clear_all then roundtrip succeeds`) | Post-clear writes would fail |
| `enable` affects types other than the target | B31 (`enable single type without affecting others`) | Side effect in enable |
| `table_for_type` returns wrong table for a CacheType | B32 (`all 6 types roundtrip`) | Cross-table contamination |

### Threshold
**≥90% mutation kill rate** — measured via `cargo-mutants`. All 16 critical mutations above must be caught. The remaining mutations (comment changes, unused variable introductions) are caught by clippy.

---

## 8. Combinatorial Coverage Matrix

### DocCache::put (validation boundaries)

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| happy path: normal key + value | key=16 bytes, value=String | Ok(()) | integration |
| key at boundary: exactly 256 | key=256 bytes | Ok(()) | unit |
| key over boundary: 257 | key=257 bytes | Err(CacheError::KeyTooLarge { size: 257, max: 256 }) | unit |
| key over boundary: 10,000 | key=10000 bytes | Err(CacheError::KeyTooLarge { size: 10000, max: 256 }) | unit |
| value at boundary: exactly 50MB | value=50MB string | Ok(()) | unit |
| value over boundary: 50MB+1 | value=50MB+1 string | Err(CacheError::ValueTooLarge { size: 52428801, max: 52428800 }) | unit |
| empty key | key=0 bytes | Ok(()) | unit |
| empty value | value="" (0 bytes serialized) | Ok(()) | unit |
| disabled cache type | CacheType::Document disabled | Ok(()) — no-op | unit |

### DocCache::get (read paths)

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| cache hit | key exists in table | Ok(Some(value)) | integration |
| cache miss | key not in table | Ok(None) | integration |
| disabled type | CacheType disabled | Ok(None) — skip | unit |
| corrupted database | redb file has garbage bytes | Err(CacheError::BackendError { .. }) | integration |

### DocCache::open (backend construction)

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| in-memory backend | CacheBackend::Memory | Ok(DocCache) — uses redb InMemoryBackend | integration |
| file backend (new) | CacheBackend::File(new_path) | Ok(DocCache) — creates file + tables | integration |
| file backend (existing) | CacheBackend::File(existing_path) | Ok(DocCache) — opens existing DB | integration |
| nested directory | File(path/to/deep/nested/cache.redb) | Ok(DocCache) — creates intermediate dirs | integration |
| corrupted file | File with garbage bytes at path | Err(CacheError::BackendError { .. }) | integration |
| read-only directory | File inside chmod 0o555 dir | Err(CacheError::BackendError { .. }) | integration |

### DocCache capacity (LRU migration critical path)

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| 10,000 entries (old LRU max) | 10_000 unique puts | stats.document_entries == 10_000, first entry still readable | integration |
| 10,001 entries (exceeds old max) | 10_001 unique puts | stats.document_entries == 10_001, first entry still readable | integration |
| 50,000 entries (stress) | 50_000 unique puts | stats.document_entries == 50_000 | integration |

### Hash functions

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| empty input | b"" | 32-byte ContentHash (deterministic) | unit |
| same input twice | b"test" | Both calls return equal ContentHash | unit |
| different inputs | b"a" vs b"b" | Two different ContentHash values | unit |
| composite order matters | [b"a", b"b"] vs [b"b", b"a"] | Two different ContentHash values | unit |
| composite same order | [b"a", b"b"] twice | Same ContentHash both times | unit |

### CacheError variants

| Scenario | Trigger | Expected Output | Layer |
|----------|---------|-----------------|-------|
| KeyTooLarge | key=257 bytes | Err(CacheError::KeyTooLarge { size: 257, max: 256 }) | unit |
| ValueTooLarge | value=50MB+1 | Err(CacheError::ValueTooLarge { size: 52428801, max: 52428800 }) | unit |
| BackendError (open) | corrupted .redb file | Err(CacheError::BackendError { operation: _, message: _ }) | integration |
| BackendError (write) | read-only directory | Err(CacheError::BackendError { operation: _, message: _ }) | integration |
| BackendError (read) | corrupted redb data pages | Err(CacheError::BackendError { operation: _, message: _ }) | integration |

---

## Open Questions

None. The contract fully specifies the migration scope. All preconditions have been verified against the codebase. The error taxonomy is unchanged. No new public APIs are introduced. BackendError scenarios (B25–B27) may require implementation-dependent adjustments based on redb's `Database::create` vs `Database::open` behavior, but the invariant (return `CacheError::BackendError`, never panic) is firm.
