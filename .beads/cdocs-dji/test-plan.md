# Test Plan: cdocs-dji — Transform Artifact Cache

## Summary

- **Behaviors identified:** 40
- **Trophy allocation:** 14 unit / 21 integration / 4 e2e / 1 static
- **Proptest invariants:** 5
- **Fuzz targets:** 2
- **Kani harnesses:** 2
- **Error variants covered:** 9/9 (100%)
- **Mutation kill target:** ≥90%

### Changelog (v2 — review fixes)

- **LETHAL-1**: Added B08 — `compute_link_map_fingerprint` returns `Err(LinkMapFingerprintFailed)` on serialization failure. Requires function signature change to `Result<ContentHash, TransformArtifactError>`.
- **LETHAL-2**: Added B20 — `load_cached_artifact` returns `Err(CacheReadFailed)` on genuine storage error. Uses corrupted redb to trigger real error path.
- **LETHAL-3**: Fixed B19 (was B11) and B25 (was B14) — titles now match Then: blocks. B19 honestly tests disabled-cache returning `Ok(None)`. B25 honestly tests disabled-cache silently returning `Ok(())`.
- **MAJOR-1**: Fixed B12 (was B07) — assertion now concrete: `result.as_bytes().len() == 32 && result.as_bytes() != [0u8; 32]`.
- **MAJOR-2**: Fixed B27 (was B15) — error assertion now includes field values: `source_path` matches exactly, `message` is non-empty.
- **MAJOR-3**: Added B31 (empty markdown rejection) and B32 (large markdown handling) for `write_artifact_to_output`.
- **MAJOR-4**: Added B22 (large cached artifact boundary) for `load_cached_artifact`.
- **MAJOR-5**: Resolved by LETHAL-2 fix — B20 catches `CacheReadFailed` branch deletion.
- **MINOR-1**: Split mixed-run into B38 (correct counts) and B39 (correct content per file).
- **MINOR-2**: Fixed B35 (was B23) — Given block now specifies exact pre-population key, artifact, and store call.
- **MINOR-3**: Fixed B19 and B25 — specify exact config field: `CacheConfig { enabled_cache_types: vec![CacheType::Analysis] }` (Transform excluded).
- **MINOR-4**: All "valid link_map" / "valid analyses" replaced with concrete inline preconditions (DAMP).
- **MINOR-5**: Fixed B40 (was B28) — explicitly states DocCache is opened by `run_index` at pipeline start.
- **MINOR-6**: Promoted matrix boundaries to §3 as B05, B06, B07, B15, B16 (full BDD scenarios).
- **MINOR-7**: Added B41 — `transform_all_cached` handles empty analyses slice gracefully.

---

## 1. Behavior Inventory

### Pure Functions (Calc Layer)

| # | Behavior |
|---|----------|
| B01 | `TransformArtifactKey::compute` returns a 32-byte SHA-256 key for any valid `(source_path, content_hash, link_map_fingerprint)` triple |
| B02 | `TransformArtifactKey::compute` is deterministic — identical inputs always produce identical keys (INV-02) |
| B03 | `TransformArtifactKey::compute` produces distinct keys for distinct input triples (POST-05 collision resistance) — 3 sub-tests |
| B04 | `TransformArtifactKey::as_bytes` returns exactly the 32-byte inner slice |
| B05 | `TransformArtifactKey::compute` returns a 32-byte non-zero key for a single-character source path `"a"` (boundary: min valid length) |
| B06 | `TransformArtifactKey::compute` returns a 32-byte non-zero key for a 255-character source path (boundary: max practical length) |
| B07 | `TransformArtifactKey::compute` returns a 32-byte non-zero key for a multi-byte UTF-8 source path like `"日本語/architecture.md"` (boundary: non-ASCII) |
| B08 | `compute_link_map_fingerprint` returns `Err(LinkMapFingerprintFailed)` when serialization fails (error variant coverage) |
| B09 | `compute_link_map_fingerprint` returns deterministic ContentHash for identical link_map contents regardless of HashMap iteration order (P-03) |
| B10 | `compute_link_map_fingerprint` returns distinct ContentHash for different link_map contents |
| B11 | `compute_link_map_fingerprint` returns a ContentHash with `as_bytes().len() == 32 && as_bytes() != [0u8; 32]` for an empty link_map |
| B12 | `TransformArtifact` serde round-trip preserves all fields byte-identically |
| B13 | `TransformArtifact` serde round-trip preserves empty `transformed_markdown` (boundary: min valid) |
| B14 | `TransformArtifact` serde round-trip preserves multi-byte UTF-8 `transformed_markdown` including CJK and emoji (boundary: unicode) |

### I/O Boundary Functions (Actions Layer)

| # | Behavior |
|---|----------|
| B15 | `load_cached_artifact` returns `Ok(Some(artifact))` when cache contains a matching key, and artifact satisfies INV-03 (field-by-field match) |
| B16 | `load_cached_artifact` returns `Ok(None)` when cache has no entry for the computed key |
| B17 | `load_cached_artifact` returns `Ok(None)` when transform cache is disabled via `CacheConfig { enabled_cache_types: vec![CacheType::Analysis] }` (graceful degradation) |
| B18 | `load_cached_artifact` returns `Err(DeserializationFailed)` when cached bytes are corrupt |
| B19 | `load_cached_artifact` returns `Err(CacheReadFailed)` on genuine storage error (corrupted/failed redb transaction) |
| B20 | `load_cached_artifact` returns `Ok(Some(artifact))` with byte-identical content for a large cached artifact (transformed_markdown ≥ 1MB) |
| B21 | `store_artifact` writes artifact to cache so that subsequent `get_transform` returns byte-identical artifact (INV-04) |
| B22 | `store_artifact` returns `Ok(())` when transform cache is disabled via `CacheConfig { enabled_cache_types: vec![CacheType::Analysis] }` (no-op, no panic) |
| B23 | `store_artifact` leaves no partial entry on write failure — `Err(CacheWriteFailed { source_path, message })` with specific field values, and key absent afterward (INV-05) |
| B24 | `write_artifact_to_output` creates file at `docs_dir/{mapping.filename}` with byte-identical content to `artifact.transformed_markdown` |
| B25 | `write_artifact_to_output` returns `Err(MissingIdMapping)` when link_map has no entry for `artifact.source_path` |
| B26 | `write_artifact_to_output` returns `Err(OutputWriteFailed)` when filesystem write fails |
| B27 | `write_artifact_to_output` rejects empty `transformed_markdown` — returns `Err(OutputWriteFailed)` or panics per precondition violation |
| B28 | `write_artifact_to_output` handles large markdown content (≥ 10MB `transformed_markdown`) — writes file with byte-identical content |
| B29 | `write_artifact_to_output` creates `docs_dir` if it does not already exist, then writes file successfully |

### Orchestration (transform_all_cached)

| # | Behavior |
|---|----------|
| B30 | `transform_all_cached` returns `Err(EmptySourcePath)` when any analysis has an empty `source_path` |
| B31 | `transform_all_cached` returns `Err(MissingIdMapping)` when any analysis has no entry in `link_map` |
| B32 | `transform_all_cached` returns `Err(FileReadFailed)` when source file cannot be read for content hashing |
| B33 | `transform_all_cached` computes fresh transform on cache miss, stores artifact, writes output file |
| B34 | `transform_all_cached` reuses cached artifact on cache hit, writes output file from cache (no re-transform) |
| B35 | `transform_all_cached` returns `Err(TransformComputationFailed)` when fresh transform fails |
| B36 | `transform_all_cached` returns `Err(CacheWriteFailed)` when artifact storage fails |
| B37 | `transform_all_cached` returns `Err(OutputWriteFailed)` when output file write fails |
| B38 | `transform_all_cached` mixed run produces correct `TransformResult` counts reflecting ALL source paths (POST-04) |
| B39 | `transform_all_cached` mixed run produces correct content per file — cached file matches artifact, fresh file starts with `"---\n"` |
| B40 | `transform_all_cached` handles empty `analyses` slice gracefully (returns `Ok(TransformResult { success_count: 0, total_count: 0, error_count: 0 })`) |

### Pipeline Integration (run_index)

| # | Behavior |
|---|----------|
| B41 | `run_index` STEP 4 uses `transform_all_cached` and produces identical output files to the non-cached path for the same inputs |

---

## 2. Trophy Allocation

| Layer | Count | Behaviors | Rationale |
|-------|-------|-----------|-----------|
| **Static** | 1 | Compile-time | `#[non_exhaustive]` on error enum, `#[forbid(unsafe_code)]`, clippy denies |
| **Unit (Calc)** | 14 | B01–B14 | Pure functions: key computation (incl. boundaries), fingerprint (incl. error variant), serde round-trip (incl. boundaries). No I/O. Exhaustive combinatorial coverage. |
| **Integration** | 22 | B15–B29, B33–B37, B40 | Component boundaries with real `DocCache` (in-memory), real filesystem (tempdir). No mocks. Tests state, not interactions. |
| **E2E** | 4 | B30–B32, B38–B39, B41 | Full pipeline scenarios: error propagation through `transform_all_cached`, mixed-run correctness, `run_index` parity. |

**Ratio check:** 14/41 unit (34%) / 22/41 integration (54%) / 4/41 e2e (10%) / 1 static (~2%) — within acceptable bounds. Integration-heavy emphasis correct for I/O-bound artifact caching. Unit layer slightly above target due to promoted boundary tests from matrix (B05–B07, B13–B14).

---

## 3. BDD Scenarios

### B01: `compute` returns 32-byte key for valid inputs

```
Given: source_path = "concepts/architecture.md",
       content_hash = ContentHash::compute(b"hello"),
       link_map_fp = ContentHash::compute(b"world")
When:  TransformArtifactKey::compute(source_path, &content_hash, &link_map_fp)
Then:  result.as_bytes().len() == 32
And:   result.as_bytes() != [0u8; 32]
```

Test: `fn artifact_key_returns_32_byte_key_for_valid_inputs()`

### B02: `compute` is deterministic (INV-02)

```
Given: source_path = "concepts/architecture.md",
       content_hash = ContentHash::compute(b"hello"),
       link_map_fp = ContentHash::compute(b"world")
When:  TransformArtifactKey::compute is called twice with identical inputs
Then:  result1 == result2
```

Test: `fn artifact_key_is_deterministic_for_identical_inputs()`

### B03: `compute` produces distinct keys for distinct triples (POST-05)

```
Given: source_path_a = "a.md", source_path_b = "b.md",
       content_hash = ContentHash::compute(b"same"),
       link_map_fp = ContentHash::compute(b"same")
When:  TransformArtifactKey::compute(source_path_a, &ch, &lfp)
  And: TransformArtifactKey::compute(source_path_b, &ch, &lfp)
Then:  key_a != key_b
```

Test: `fn artifact_key_produces_distinct_keys_for_distinct_source_paths()`

```
Given: source_path = "a.md",
       content_hash_1 = ContentHash::compute(b"content1"),
       content_hash_2 = ContentHash::compute(b"content2"),
       link_map_fp = ContentHash::compute(b"same")
When:  TransformArtifactKey::compute(source_path, &ch_1, &lfp)
  And: TransformArtifactKey::compute(source_path, &ch_2, &lfp)
Then:  key_a != key_b
```

Test: `fn artifact_key_produces_distinct_keys_for_distinct_content_hashes()`

```
Given: source_path = "a.md",
       content_hash = ContentHash::compute(b"same"),
       link_map_fp_1 = ContentHash::compute(b"lmap1"),
       link_map_fp_2 = ContentHash::compute(b"lmap2")
When:  TransformArtifactKey::compute(source_path, &ch, &lfp_1)
  And: TransformArtifactKey::compute(source_path, &ch, &lfp_2)
Then:  key_a != key_b
```

Test: `fn artifact_key_produces_distinct_keys_for_distinct_link_map_fingerprints()`

### B04: `as_bytes` returns inner 32-byte slice

```
Given: a TransformArtifactKey computed from source_path = "a.md",
       content_hash = ContentHash::compute(b"x"),
       link_map_fp = ContentHash::compute(b"y")
When:  key.as_bytes()
Then:  slice.len() == 32
And:   slice == &key.0[..] (inner Vec<u8> contents exactly)
```

Test: `fn artifact_key_as_bytes_returns_32_byte_slice()`

### B05: `compute` handles single-character source path (boundary: min valid)

```
Given: source_path = "a" (1 character),
       content_hash = ContentHash::compute(b"hello"),
       link_map_fp = ContentHash::compute(b"world")
When:  TransformArtifactKey::compute("a", &content_hash, &link_map_fp)
Then:  result.as_bytes().len() == 32
And:   result.as_bytes() != [0u8; 32]
```

Test: `fn artifact_key_returns_32_byte_key_for_single_char_source_path()`

### B06: `compute` handles 255-character source path (boundary: max practical)

```
Given: source_path = "a".repeat(255) (255 characters),
       content_hash = ContentHash::compute(b"hello"),
       link_map_fp = ContentHash::compute(b"world")
When:  TransformArtifactKey::compute(&long_path, &content_hash, &link_map_fp)
Then:  result.as_bytes().len() == 32
And:   result.as_bytes() != [0u8; 32]
```

Test: `fn artifact_key_returns_32_byte_key_for_255_char_source_path()`

### B07: `compute` handles multi-byte UTF-8 source path (boundary: non-ASCII)

```
Given: source_path = "日本語/architecture.md" (multi-byte UTF-8),
       content_hash = ContentHash::compute(b"hello"),
       link_map_fp = ContentHash::compute(b"world")
When:  TransformArtifactKey::compute("日本語/architecture.md", &content_hash, &link_map_fp)
Then:  result.as_bytes().len() == 32
And:   result.as_bytes() != [0u8; 32]
And:   calling compute again with identical inputs produces identical key (determinism holds for UTF-8)
```

Test: `fn artifact_key_returns_32_byte_key_for_multibyte_utf8_source_path()`

### B08: `compute_link_map_fingerprint` returns `Err(LinkMapFingerprintFailed)` on serialization failure

```
Given: compute_link_map_fingerprint is called with a HashMap where the internal
       serde_json::to_string call fails (requires function signature to return
       Result<ContentHash, TransformArtifactError>; test injects serialization
       failure via a test-only mechanism — e.g., a HashMap with entries whose
       combined serialized size exceeds serde_json's internal buffer limit, or
       a test-specific serialization wrapper that returns Err)
When:  compute_link_map_fingerprint returns Err(...)
Then:  Err(TransformArtifactError::LinkMapFingerprintFailed { message: m })
       where m.is_empty() == false
```

Test: `fn link_map_fingerprint_returns_error_on_serialization_failure()`

**Implementation note:** The contract currently has `compute_link_map_fingerprint` using `.expect()` and returning `ContentHash` directly. This scenario requires changing the function signature to `fn compute_link_map_fingerprint(...) -> Result<ContentHash, TransformArtifactError>` and propagating the serde error with `.map_err(|e| TransformArtifactError::LinkMapFingerprintFailed { message: e.to_string() })`. The test must trigger a genuine `serde_json::Error` — use a HashMap large enough to exceed `serde_json`'s allocation limit, or provide a test-only serialization path that injects failure.

### B09: `compute_link_map_fingerprint` is deterministic across HashMap orderings (P-03)

```
Given: two HashMap<String, IdMapping> with identical entries:
         { "a.md" → IdMapping { id: "gen-arch-001", filename: "ref-general-a.md", .. },
           "b.md" → IdMapping { id: "gen-arch-002", filename: "ref-general-b.md", .. } }
       but inserted in different order (forward vs reverse)
When:  compute_link_map_fingerprint is called on each
Then:  fingerprint1 == fingerprint2
```

Test: `fn link_map_fingerprint_is_deterministic_regardless_of_hashmap_order()`

### B10: `compute_link_map_fingerprint` produces distinct hashes for different contents

```
Given: link_map_1 = { "a.md" → IdMapping { id: "gen-arch-001", filename: "ref-general-a.md" } }
  And: link_map_2 = { "a.md" → IdMapping { id: "gen-arch-999", filename: "ref-general-z.md" } }
When:  compute_link_map_fingerprint is called on each
Then:  fp1 != fp2
```

Test: `fn link_map_fingerprint_produces_distinct_hashes_for_different_contents()`

### B11: `compute_link_map_fingerprint` handles empty link_map

```
Given: empty HashMap<String, IdMapping> (HashMap::new())
When:  compute_link_map_fingerprint(HashMap::new())
Then:  result.as_bytes().len() == 32
And:   result.as_bytes() != [0u8; 32]
```

Test: `fn link_map_fingerprint_returns_nontrivial_hash_for_empty_map()`

### B12: `TransformArtifact` serde round-trip is lossless

```
Given: TransformArtifact {
         source_path: "concepts/architecture.md",
         content_hash: ContentHash::compute(b"hello"),
         link_map_fingerprint: ContentHash::compute(b"world"),
         transformed_markdown: "---\nid: foo\n---\ncontent"
       }
When:  let json = serde_json::to_string(&artifact);
       let roundtrip = serde_json::from_str::<TransformArtifact>(&json)
Then:  roundtrip == Ok(artifact)
And:   roundtrip.unwrap().source_path == "concepts/architecture.md"
And:   roundtrip.unwrap().transformed_markdown == "---\nid: foo\n---\ncontent"
```

Test: `fn transform_artifact_serde_roundtrip_preserves_all_fields()`

### B13: `TransformArtifact` serde round-trip preserves empty `transformed_markdown`

```
Given: TransformArtifact {
         source_path: "a.md",
         content_hash: ContentHash::compute(b"x"),
         link_map_fingerprint: ContentHash::compute(b"y"),
         transformed_markdown: ""
       }
When:  serde_json::to_string then serde_json::from_str::<TransformArtifact>
Then:  deserialized == original (PartialEq)
And:   deserialized.transformed_markdown == ""
```

Test: `fn transform_artifact_serde_roundtrip_preserves_empty_markdown()`

### B14: `TransformArtifact` serde round-trip preserves multi-byte UTF-8 markdown

```
Given: TransformArtifact {
         source_path: "日本語/docs.md",
         content_hash: ContentHash::compute(b"x"),
         link_map_fingerprint: ContentHash::compute(b"y"),
         transformed_markdown: "---\nid: テスト\n---\n内容 🎉 émoji"
       }
When:  serde_json::to_string then serde_json::from_str::<TransformArtifact>
Then:  deserialized == original (PartialEq)
And:   deserialized.transformed_markdown == "---\nid: テスト\n---\n内容 🎉 émoji"
```

Test: `fn transform_artifact_serde_roundtrip_preserves_unicode_markdown()`

---

### B15: `load_cached_artifact` returns `Ok(Some(artifact))` on cache hit

```
Given: DocCache (in-memory redb) with an empty TRANSFORM_TABLE
  And: a TransformArtifact {
         source_path: "a.md",
         content_hash: ContentHash::compute(b"file-bytes"),
         link_map_fingerprint: ContentHash::compute(b"link-map-bytes"),
         transformed_markdown: "---\nid: x\n---\nbody"
       }
  And: the artifact is stored via cache.put_transform::<TransformArtifact>(key, &artifact)
       where key = TransformArtifactKey::compute("a.md", &content_hash, &link_map_fingerprint)
When:  load_cached_artifact(cache, "a.md", &content_hash, &link_map_fingerprint)
Then:  Ok(Some(artifact)) where result.source_path == "a.md"
And:   result.transformed_markdown == "---\nid: x\n---\nbody"
And:   result.content_hash == ContentHash::compute(b"file-bytes")
```

Test: `fn load_cached_artifact_returns_artifact_on_cache_hit()`

### B16: `load_cached_artifact` returns `Ok(None)` on cache miss

```
Given: DocCache (in-memory) with no entries in TRANSFORM_TABLE
  And: source_path = "nonexistent.md",
       content_hash = ContentHash::compute(b"anything"),
       link_map_fingerprint = ContentHash::compute(b"anything")
When:  load_cached_artifact(cache, "nonexistent.md", &ch, &lfp)
Then:  Ok(None)
```

Test: `fn load_cached_artifact_returns_none_on_cache_miss()`

### B17: `load_cached_artifact` returns `Ok(None)` when transform cache is disabled

```
Given: DocCache opened with CacheConfig { enabled_cache_types: vec![CacheType::Analysis] }
       (CacheType::Transform is NOT in the enabled list → transform cache is disabled)
  And: source_path = "a.md",
       content_hash = ContentHash::compute(b"anything"),
       link_map_fingerprint = ContentHash::compute(b"anything")
When:  load_cached_artifact(cache, "a.md", &ch, &lfp)
Then:  Ok(None) (disabled cache gracefully degrades to cache-miss behavior)
```

Test: `fn load_cached_artifact_returns_none_when_transform_cache_disabled()`

### B18: `load_cached_artifact` returns `Err(DeserializationFailed)` on corrupt data

```
Given: DocCache (in-memory) with raw garbage bytes b"NOT VALID JSON{{{" stored
       directly under the computed artifact key via a low-level redb write
       (bypassing typed deserialization)
  And: source_path = "a.md",
       content_hash = ContentHash::compute(b"anything"),
       link_map_fingerprint = ContentHash::compute(b"anything")
When:  load_cached_artifact(cache, "a.md", &ch, &lfp)
Then:  Err(TransformArtifactError::DeserializationFailed {
         source_path: "a.md",
         message: m
       })
       where m.is_empty() == false
```

Test: `fn load_cached_artifact_returns_deserialization_failed_on_corrupt_data()`

### B19: `load_cached_artifact` returns `Err(CacheReadFailed)` on genuine storage error

```
Given: DocCache backed by a redb file on disk, where the underlying redb storage
       is corrupted after opening (e.g., truncate the file to 0 bytes while the
       cache handle is still open, or use a redb instance opened on a file in a
       read-only directory that cannot sustain a read transaction)
  And: source_path = "a.md",
       content_hash = ContentHash::compute(b"anything"),
       link_map_fingerprint = ContentHash::compute(b"anything")
When:  load_cached_artifact(cache, "a.md", &ch, &lfp)
Then:  Err(TransformArtifactError::CacheReadFailed {
         source_path: "a.md",
         message: m
       })
       where m.is_empty() == false
```

Test: `fn load_cached_artifact_returns_cache_read_failed_on_storage_error()`

**Implementation note:** Forcing a genuine `CacheReadFailed` requires causing the redb read
transaction to fail. Practical mechanisms: (a) open redb on a file, write an entry, then
truncate/replace the file externally before reading; (b) use a `DocCache` wrapper that
injects a failure into `get_transform`; (c) create a redb on a FUSE filesystem that
returns EIO on read. Option (a) is recommended for CI — write to a tempfile, truncate,
then attempt read on the same handle.

### B20: `load_cached_artifact` handles large cached artifact (≥ 1MB)

```
Given: DocCache (in-memory) containing a stored TransformArtifact where
       transformed_markdown = "x".repeat(1_048_576) (1 MB of content)
  And: source_path = "large.md",
       content_hash = ContentHash::compute(b"large-file"),
       link_map_fingerprint = ContentHash::compute(b"large-lmap")
When:  load_cached_artifact(cache, "large.md", &ch, &lfp)
Then:  Ok(Some(artifact)) where artifact.source_path == "large.md"
And:   artifact.transformed_markdown.len() == 1_048_576
And:   artifact.transformed_markdown == "x".repeat(1_048_576) (byte-identical)
```

Test: `fn load_cached_artifact_handles_large_cached_artifact()`

### B21: `store_artifact` writes and subsequent read returns identical artifact (INV-04)

```
Given: DocCache (in-memory) with empty TRANSFORM_TABLE
  And: TransformArtifact {
         source_path: "a.md",
         content_hash: ContentHash::compute(b"file-bytes"),
         link_map_fingerprint: ContentHash::compute(b"link-map-bytes"),
         transformed_markdown: "---\nid: x\n---\nbody"
       }
  And: link_map_fingerprint = ContentHash::compute(b"link-map-bytes")
When:  store_artifact(cache, &artifact, &link_map_fingerprint)
Then:  Ok(())
And:   subsequent cache.get_transform::<TransformArtifact>(
         TransformArtifactKey::compute("a.md", &content_hash, &link_map_fingerprint)
       ) returns Some(retrieved) where retrieved == artifact (PartialEq, byte-identical)
```

Test: `fn store_artifact_write_then_read_returns_identical_artifact()`

### B22: `store_artifact` returns `Ok(())` when transform cache is disabled

```
Given: DocCache opened with CacheConfig { enabled_cache_types: vec![CacheType::Analysis] }
       (CacheType::Transform is NOT in the enabled list → transform cache is disabled)
  And: TransformArtifact {
         source_path: "a.md",
         content_hash: ContentHash::compute(b"x"),
         link_map_fingerprint: ContentHash::compute(b"y"),
         transformed_markdown: "content"
       }
When:  store_artifact(cache, &artifact, &link_map_fingerprint)
Then:  Ok(()) (disabled cache silently succeeds — confirms no panic, no error)
```

Test: `fn store_artifact_succeeds_silently_when_transform_cache_disabled()`

### B23: `store_artifact` leaves no partial entry on failure (INV-05)

```
Given: DocCache (in-memory) with empty TRANSFORM_TABLE
  And: a TransformArtifact with source_path = "a.md" and
       transformed_markdown content that exceeds redb's maximum value size
       (construct by creating a string of length > redb::Table::max_value_size())
When:  store_artifact(cache, &artifact, &link_map_fingerprint)
Then:  Err(TransformArtifactError::CacheWriteFailed {
         source_path: "a.md",
         message: m
       })
       where m.is_empty() == false
And:   cache.get_transform::<TransformArtifact>(key) returns None
       (no partial write visible)
```

Test: `fn store_artifact_leaves_no_partial_entry_on_failure()`

### B24: `write_artifact_to_output` creates correct file

```
Given: tempdir with docs/ subdirectory
  And: TransformArtifact {
         source_path: "a.md",
         content_hash: ContentHash::compute(b"x"),
         link_map_fingerprint: ContentHash::compute(b"y"),
         transformed_markdown: "---\nid: x\n---\nbody"
       }
  And: link_map = { "a.md" → IdMapping { id: "gen-arch-001", filename: "ref-general-a.md" } }
  And: docs_dir = tempdir.path().join("docs")
When:  write_artifact_to_output(&artifact, &link_map, &docs_dir)
Then:  Ok(())
And:   file exists at docs_dir.join("ref-general-a.md")
And:   std::fs::read_to_string(docs_dir.join("ref-general-a.md")) == Ok("---\nid: x\n---\nbody")
```

Test: `fn write_artifact_to_output_creates_file_with_correct_content()`

### B25: `write_artifact_to_output` returns `Err(MissingIdMapping)`

```
Given: TransformArtifact {
         source_path: "orphan.md",
         content_hash: ContentHash::compute(b"x"),
         link_map_fingerprint: ContentHash::compute(b"y"),
         transformed_markdown: "content"
       }
  And: link_map = HashMap::new() (empty — no mapping for "orphan.md")
  And: docs_dir = tempdir.path().join("docs")
When:  write_artifact_to_output(&artifact, &link_map, &docs_dir)
Then:  Err(TransformArtifactError::MissingIdMapping { source_path: "orphan.md" })
```

Test: `fn write_artifact_to_output_returns_missing_id_mapping_when_no_entry()`

### B26: `write_artifact_to_output` returns `Err(OutputWriteFailed)` on I/O failure

```
Given: tempdir with a read-only parent directory (chmod 0o444 on parent)
  And: docs_dir = read_only_parent.join("docs") (cannot be created)
  And: TransformArtifact {
         source_path: "a.md",
         content_hash: ContentHash::compute(b"x"),
         link_map_fingerprint: ContentHash::compute(b"y"),
         transformed_markdown: "content"
       }
  And: link_map = { "a.md" → IdMapping { id: "gen-arch-001", filename: "ref-general-a.md" } }
When:  write_artifact_to_output(&artifact, &link_map, &docs_dir)
Then:  Err(TransformArtifactError::OutputWriteFailed {
         source_path: "a.md",
         message: m
       })
       where m.is_empty() == false
```

Test: `fn write_artifact_to_output_returns_output_write_failed_on_io_error()`

### B27: `write_artifact_to_output` rejects empty `transformed_markdown`

```
Given: TransformArtifact {
         source_path: "a.md",
         content_hash: ContentHash::compute(b"x"),
         link_map_fingerprint: ContentHash::compute(b"y"),
         transformed_markdown: ""
       }
  And: link_map = { "a.md" → IdMapping { id: "gen-arch-001", filename: "ref-general-a.md" } }
  And: docs_dir = tempdir.path().join("docs")
When:  write_artifact_to_output(&artifact, &link_map, &docs_dir)
Then:  Err(TransformArtifactError::OutputWriteFailed {
         source_path: "a.md",
         message: m
       })
       where m.contains("empty") or m.contains("precondition") == true
       (precondition violation: transformed_markdown must be non-empty)
```

Test: `fn write_artifact_to_output_rejects_empty_transformed_markdown()`

**Note:** The contract specifies `transformed_markdown is non-empty` as a precondition. If the implementation asserts/panics on this precondition rather than returning an error, the test should verify `#[should_panic(expected = "non-empty")]` with the same concrete expectation.

### B28: `write_artifact_to_output` handles large markdown content (≥ 10MB)

```
Given: TransformArtifact {
         source_path: "large.md",
         content_hash: ContentHash::compute(b"x"),
         link_map_fingerprint: ContentHash::compute(b"y"),
         transformed_markdown: "x".repeat(10_485_760) (10 MB)
       }
  And: link_map = { "large.md" → IdMapping { id: "gen-arch-001", filename: "ref-general-large.md" } }
  And: docs_dir = tempdir.path().join("docs")
When:  write_artifact_to_output(&artifact, &link_map, &docs_dir)
Then:  Ok(())
And:   file exists at docs_dir.join("ref-general-large.md")
And:   file content.len() == 10_485_760
And:   file content == "x".repeat(10_485_760) (byte-identical)
```

Test: `fn write_artifact_to_output_handles_large_markdown_content()`

### B29: `write_artifact_to_output` creates missing `docs_dir`

```
Given: tempdir WITHOUT a docs/ subdirectory
  And: docs_dir = tempdir.path().join("docs") (does not exist yet)
  And: TransformArtifact {
         source_path: "a.md",
         content_hash: ContentHash::compute(b"x"),
         link_map_fingerprint: ContentHash::compute(b"y"),
         transformed_markdown: "content"
       }
  And: link_map = { "a.md" → IdMapping { id: "gen-arch-001", filename: "ref-general-a.md" } }
When:  write_artifact_to_output(&artifact, &link_map, &docs_dir)
Then:  Ok(())
And:   docs_dir.exists() == true (directory was created)
And:   file exists at docs_dir.join("ref-general-a.md")
And:   file content == "content"
```

Test: `fn write_artifact_to_output_creates_missing_docs_dir()`

---

### B30: `transform_all_cached` returns `Err(EmptySourcePath)` for empty source path

```
Given: analyses = [Analysis { source_path: "", content: Arc::from("..."), ... }]
  And: link_map = HashMap::new()
  And: output_dir = tempdir
  And: DocCache (in-memory)
When:  transform_all_cached(&analyses, &link_map, &output_dir, &cache)
Then:  Err(TransformArtifactError::EmptySourcePath)
```

Test: `fn transform_all_cached_returns_empty_source_path_for_empty_source_path()`

### B31: `transform_all_cached` returns `Err(MissingIdMapping)` when link_map lacks entry

```
Given: analyses = [Analysis { source_path: "a.md", content: Arc::from("# Title"), ... }]
  And: link_map = HashMap::new() (no mapping for "a.md")
  And: output_dir = tempdir
  And: DocCache (in-memory)
When:  transform_all_cached(&analyses, &link_map, &output_dir, &cache)
Then:  Err(TransformArtifactError::MissingIdMapping { source_path: "a.md" })
```

Test: `fn transform_all_cached_returns_missing_id_mapping_when_no_link_map_entry()`

### B32: `transform_all_cached` returns `Err(FileReadFailed)` when source file unreadable

```
Given: analyses = [Analysis { source_path: "nonexistent.md", content: Arc::from("..."), ... }]
  And: link_map = { "nonexistent.md" → IdMapping { id: "gen-arch-001", filename: "ref-general-nonexistent.md" } }
  And: output_dir = tempdir
  And: DocCache (in-memory)
  And: no file at source_path on disk
When:  transform_all_cached(&analyses, &link_map, &output_dir, &cache)
Then:  Err(TransformArtifactError::FileReadFailed {
         source_path: "nonexistent.md",
         message: m
       })
       where m.is_empty() == false
```

Test: `fn transform_all_cached_returns_file_read_failed_when_source_missing()`

### B33: `transform_all_cached` computes fresh transform on cache miss

```
Given: DocCache (in-memory) with empty TRANSFORM_TABLE
  And: analyses = [Analysis { source_path: "a.md", content: Arc::from("# Hello\n\nbody text"), ... }]
  And: link_map = { "a.md" → IdMapping { id: "gen-arch-001", filename: "ref-general-a.md" } }
  And: source file "a.md" exists on disk at the expected source directory
  And: output_dir = tempdir with docs/ subdirectory
When:  transform_all_cached(&analyses, &link_map, &output_dir, &cache)
Then:  Ok(TransformResult { success_count: 1, total_count: 1, error_count: 0 })
And:   output file exists at output_dir/docs/ref-general-a.md
And:   output file content starts with "---\n" (frontmatter present)
And:   output file content contains the transformed body
And:   cache now contains an artifact for "a.md":
         cache.get_transform::<TransformArtifact>(key) returns Some(artifact)
         where artifact.source_path == "a.md"
         and artifact.transformed_markdown == output file content (byte-identical)
```

Test: `fn transform_all_cached_computes_fresh_transform_on_cache_miss()`

### B34: `transform_all_cached` reuses cached artifact on cache hit

```
Given: DocCache (in-memory) pre-populated as follows:
         stored_artifact = TransformArtifact {
           source_path: "a.md",
           content_hash: ContentHash::compute(b"original-file-bytes"),
           link_map_fingerprint: ContentHash::compute(b"serialized-link-map"),
           transformed_markdown: "---\nid: gen-arch-001\n---\ncached body"
         }
         stored via cache.put_transform::<TransformArtifact>(
           TransformArtifactKey::compute("a.md", &stored_artifact.content_hash, &stored_artifact.link_map_fingerprint),
           &stored_artifact
         )
  And: analyses = [Analysis { source_path: "a.md", content: Arc::from("original-file-bytes"), ... }]
  And: link_map = { "a.md" → IdMapping { id: "gen-arch-001", filename: "ref-general-a.md" } }
  And: source file "a.md" on disk with bytes b"original-file-bytes"
  And: output_dir = tempdir with docs/ subdirectory
When:  transform_all_cached(&analyses, &link_map, &output_dir, &cache)
Then:  Ok(TransformResult { success_count: 1, total_count: 1, error_count: 0 })
And:   output file at output_dir/docs/ref-general-a.md contains "---\nid: gen-arch-001\n---\ncached body"
       (byte-identical to stored_artifact.transformed_markdown)
And:   transform computation is NOT invoked for "a.md" (no re-computation)
```

Test: `fn transform_all_cached_reuses_cached_artifact_on_cache_hit()`

### B35: `transform_all_cached` returns `Err(TransformComputationFailed)` when transform fails

```
Given: analyses = [Analysis { source_path: "bad.md", content: Arc::from("malformed content triggering transform error"), ... }]
  And: link_map = { "bad.md" → IdMapping { id: "gen-arch-001", filename: "ref-general-bad.md" } }
  And: source file "bad.md" exists on disk
  And: output_dir = tempdir
  And: DocCache (in-memory)
When:  transform_all_cached(&analyses, &link_map, &output_dir, &cache)
Then:  Err(TransformArtifactError::TransformComputationFailed {
         source_path: "bad.md",
         message: m
       })
       where m.is_empty() == false
```

Test: `fn transform_all_cached_returns_transform_computation_failed_on_failure()`

### B36: `transform_all_cached` returns `Err(CacheWriteFailed)` when artifact store fails

```
Given: DocCache (in-memory) where put_transform will fail for oversized values
       (artifact with transformed_markdown exceeding redb MAX_VALUE_SIZE)
  And: analyses = [Analysis { source_path: "big.md", content: Arc::from("..."), ... }]
  And: link_map = { "big.md" → IdMapping { id: "gen-arch-001", filename: "ref-general-big.md" } }
  And: source file "big.md" exists on disk
  And: output_dir = tempdir
When:  transform_all_cached(&analyses, &link_map, &output_dir, &cache)
Then:  Err(TransformArtifactError::CacheWriteFailed {
         source_path: "big.md",
         message: m
       })
       where m.is_empty() == false
```

Test: `fn transform_all_cached_returns_cache_write_failed_on_store_failure()`

### B37: `transform_all_cached` returns `Err(OutputWriteFailed)` when file write fails

```
Given: output_dir with read-only docs/ subdirectory (chmod 0o444)
  And: analyses = [Analysis { source_path: "a.md", content: Arc::from("# Title"), ... }]
  And: link_map = { "a.md" → IdMapping { id: "gen-arch-001", filename: "ref-general-a.md" } }
  And: DocCache (in-memory)
When:  transform_all_cached(&analyses, &link_map, &output_dir, &cache)
Then:  Err(TransformArtifactError::OutputWriteFailed {
         source_path: "a.md",
         message: m
       })
       where m.is_empty() == false
```

Test: `fn transform_all_cached_returns_output_write_failed_on_write_error()`

### B38: `transform_all_cached` mixed run produces correct counts (POST-04)

```
Given: DocCache (in-memory) pre-populated with:
         artifact_cached = TransformArtifact {
           source_path: "cached.md",
           content_hash: ContentHash::compute(b"cached-file-bytes"),
           link_map_fingerprint: ContentHash::compute(b"cached-lmap-bytes"),
           transformed_markdown: "---\nid: gen-arch-001\n---\ncached content"
         }
         stored under TransformArtifactKey::compute("cached.md", &content_hash, &link_map_fp)
  And: analyses = [
         Analysis { source_path: "cached.md", content: Arc::from("cached-file-bytes"), ... },
         Analysis { source_path: "fresh.md", content: Arc::from("# Fresh\n\nnew content"), ... }
       ]
  And: link_map = {
         "cached.md" → IdMapping { id: "gen-arch-001", filename: "ref-general-cached.md" },
         "fresh.md" → IdMapping { id: "gen-arch-002", filename: "ref-general-fresh.md" }
       }
  And: source files exist on disk for both "cached.md" and "fresh.md"
  And: output_dir = tempdir with docs/ subdirectory
When:  transform_all_cached(&analyses, &link_map, &output_dir, &cache)
Then:  Ok(TransformResult { success_count: 2, total_count: 2, error_count: 0 })
And:   cache now contains artifacts for both "cached.md" and "fresh.md"
```

Test: `fn transform_all_cached_mixed_run_produces_correct_counts()`

### B39: `transform_all_cached` mixed run produces correct content per file

```
Given: Same setup as B38 — DocCache pre-populated with artifact for "cached.md",
       analyses for "cached.md" and "fresh.md", link_map for both, source files on disk
When:  transform_all_cached(&analyses, &link_map, &output_dir, &cache)
Then:  output file for "cached.md" at docs/ref-general-cached.md == "---\nid: gen-arch-001\n---\ncached content"
       (byte-identical to stored artifact's transformed_markdown)
And:   output file for "fresh.md" at docs/ref-general-fresh.md starts with "---\n"
       (frontmatter from fresh transform)
And:   output file for "fresh.md" contains transformed version of "# Fresh\n\nnew content"
```

Test: `fn transform_all_cached_mixed_run_produces_correct_content_per_file()`

### B40: `transform_all_cached` handles empty analyses slice gracefully

```
Given: analyses = [] (empty slice)
  And: link_map = HashMap::new()
  And: output_dir = tempdir
  And: DocCache (in-memory)
When:  transform_all_cached(&analyses, &link_map, &output_dir, &cache)
Then:  Ok(TransformResult { success_count: 0, total_count: 0, error_count: 0 })
       (no panic, no error — graceful handling of degenerate input)
```

Test: `fn transform_all_cached_handles_empty_analyses_slice_gracefully()`

---

### B41: `run_index` produces identical output with cached transform

```
Given: source directory with 3 markdown files: "a.md", "b.md", "c.md"
  And: output directory A for non-cached run (first invocation)
  And: output directory B for cached run (second invocation)
  And: DocCache is created and opened by run_index at pipeline start, before STEP 4
When:  run_index is called with source dir → output A (first run, populates cache)
  And: run_index is called with source dir → output B (second run, uses cache)
Then:  for each file X in output_a/docs/:
         output_b/docs/X exists and content == output_a/docs/X (byte-identical)
And:   second run completes successfully with Ok(TransformResult { success_count: 3, .. })
```

Test: `fn run_index_produces_identical_output_with_cached_transform()`

---

## 4. Proptest Invariants

### PPT-01: `TransformArtifactKey::compute` determinism

```
Invariant: For any (source_path, content_hash_bytes, link_map_fp_bytes),
           compute(a, b, c) == compute(a, b, c) always.
Strategy:  (any non-empty ASCII string 1..100 chars, any [u8;32], any [u8;32])
Anti:      Empty source_path → precondition violation (should panic or reject)
```

### PPT-02: `TransformArtifactKey::compute` distinct-input distinct-output

```
Invariant: For any two input triples where at least one component differs,
           the resulting 32-byte keys are distinct (SHA-256 collision resistance).
Strategy:  Pairs of triples where exactly one component differs by 1 byte.
Anti:      Identical triples must produce identical keys.
```

### PPT-03: `compute_link_map_fingerprint` order independence

```
Invariant: For any Vec<(String, IdMapping)> of 1..20 entries,
           fingerprint from HashMap built via insertion order A
           == fingerprint from HashMap built via reverse order.
Strategy:  Vec of ( alphanumeric_string(1..20), IdMapping{random fields} ),
           shuffled and reversed.
Anti:      Mutating any IdMapping field must change the fingerprint.
```

### PPT-04: `TransformArtifact` serde round-trip

```
Invariant: serde_json::from_str::<TransformArtifact>(&serde_json::to_string(&artifact).unwrap())
           == Ok(artifact) for any valid TransformArtifact.
Strategy:  source_path: alphanumeric 1..50 chars,
           content_hash: random [u8;32],
           link_map_fingerprint: random [u8;32],
           transformed_markdown: arbitrary UTF-8 string 0..5000 chars.
Anti:      Corrupt JSON bytes should fail deserialization.
```

### PPT-05: `composite_hash` (underlying `TransformArtifactKey::compute`) length

```
Invariant: For any Vec<&[u8]> of 1..10 parts each of length 0..1000,
           ContentHash::as_bytes().len() == 32.
Strategy:  Vec<Vec<u8>> with arbitrary byte content.
Anti:      Empty parts vec (0 parts) — verify behavior is defined.
```

---

## 5. Fuzz Targets

### Fuzz Target 1: `TransformArtifact` deserialization

```
Function:    serde_json::from_str::<TransformArtifact>(input)
Input type:  &str (arbitrary bytes as UTF-8)
Risk:        Panic in serde_json on malformed input, OOM on huge strings,
             logic error if deserialized artifact has inconsistent fields.
Corpus seeds:
  - valid artifact JSON: {"source_path":"a.md","content_hash":[0;32],"link_map_fingerprint":[0;32],"transformed_markdown":"---\n---\nbody"}
  - empty string
  - truncated JSON: {"source_path"
  - extra fields: {"source_path":"a.md","unknown":true}
  - wrong types: {"source_path":123}
  - unicode source_path: {"source_path":"日本語.md","content_hash":[0;32],"link_map_fingerprint":[0;32],"transformed_markdown":"内容"}
```

### Fuzz Target 2: `compute_link_map_fingerprint` with adversarial keys

```
Function:    compute_link_map_fingerprint(input_map)
Input type:  HashMap<String, IdMapping> — fuzz via JSON deserialization
Risk:        Panic on empty keys (precondition violation),
             non-determinism if sort is locale-dependent,
             OOM on huge maps.
Corpus seeds:
  - empty map: {}
  - single entry with unicode key: {"日本語.md": {...}}
  - map with 10000 entries
  - map with key containing NUL bytes
  - map with keys that differ only in case
```

---

## 6. Kani Harnesses

### Kani-01: `TransformArtifactKey::compute` output is always 32 bytes

```
Property: For all valid (source_path: &str, content_hash: [u8;32], link_map_fp: [u8;32]),
          result.as_bytes().len() == 32.
Bound:    source_path length ≤ 256 bytes (MAX_KEY_SIZE constraint).
Rationale: SHA-256 output is always 32 bytes by definition, but this formally
          proves the composite_hash wrapper does not truncate or extend.
```

### Kani-02: `compute_link_map_fingerprint` never panics for valid inputs

```
Property: For all HashMap<String, IdMapping> with non-empty keys and ≤ 256 entries,
          compute_link_map_fingerprint returns Result::Ok(ContentHash) without panic
          (after signature change to return Result).
Bound:    Map size ≤ 256, key length ≤ 100, IdMapping fields ≤ 100 chars each.
Rationale: The function previously used .expect() internally — Kani proves the
          Result-returning version has no remaining panic path for valid inputs.
```

---

## 7. Mutation Testing Checkpoints

**Threshold: ≥90% mutation kill rate**

| Mutation Target | Caught By Test |
|----------------|----------------|
| `composite_hash`: remove one `hasher.update(part)` | `artifact_key_produces_distinct_keys_for_distinct_source_paths` |
| `TransformArtifactKey::compute`: swap content_hash and link_map_fp arguments | `artifact_key_produces_distinct_keys_for_distinct_content_hashes` + `distinct_link_map_fingerprints` |
| `compute_link_map_fingerprint`: remove `sort_by_key` line | `link_map_fingerprint_is_deterministic_regardless_of_hashmap_order` |
| `compute_link_map_fingerprint`: hash only keys, not values | `link_map_fingerprint_produces_distinct_hashes_for_different_contents` |
| `compute_link_map_fingerprint`: replace `.map_err(...)` with `.expect(...)` | `link_map_fingerprint_returns_error_on_serialization_failure` (B08) |
| `store_artifact`: skip `cache.put_transform` call | `store_artifact_write_then_read_returns_identical_artifact` |
| `load_cached_artifact`: always return `Ok(None)` | `load_cached_artifact_returns_artifact_on_cache_hit` |
| `load_cached_artifact`: return artifact with wrong source_path | `load_cached_artifact_returns_artifact_on_cache_hit` (asserts source_path match) |
| `load_cached_artifact`: delete `CacheReadFailed` match arm | `load_cached_artifact_returns_cache_read_failed_on_storage_error` (B19) |
| `write_artifact_to_output`: write empty string instead of markdown | `write_artifact_to_output_creates_file_with_correct_content` (asserts byte-identical) |
| `write_artifact_to_output`: skip `link_map.get` check | `write_artifact_to_output_returns_missing_id_mapping_when_no_entry` |
| `transform_all_cached`: skip cache lookup (always compute fresh) | `transform_all_cached_reuses_cached_artifact_on_cache_hit` |
| `transform_all_cached`: skip `store_artifact` call | `transform_all_cached_computes_fresh_transform_on_cache_miss` (verifies cache populated) |
| `transform_all_cached`: return success_count = 0 | `transform_all_cached_mixed_run_produces_correct_counts` |
| `transform_all_cached`: skip empty source_path validation | `transform_all_cached_returns_empty_source_path_for_empty_source_path` |
| `TransformArtifact` Serialize: omit a field | `transform_artifact_serde_roundtrip_preserves_all_fields` |
| `TransformArtifact` Deserialize: swap fields on read | `transform_artifact_serde_roundtrip_preserves_all_fields` |
| `write_artifact_to_output`: skip docs_dir creation | `write_artifact_to_output_creates_missing_docs_dir` (B29) |
| `write_artifact_to_output`: ignore empty markdown check | `write_artifact_to_output_rejects_empty_transformed_markdown` (B27) |

---

## 8. Combinatorial Coverage Matrix

### Unit Tests: `TransformArtifactKey::compute`

| Scenario | Input Class | Expected Output | Test Layer |
|----------|-------------|-----------------|------------|
| happy path | valid (path, ch, lfp) | 32-byte non-zero key | unit |
| determinism | identical inputs called twice | keys are equal | unit |
| distinct source_path | "a.md" vs "b.md", same ch+lfp | keys differ | unit |
| distinct content_hash | same path, different ch | keys differ | unit |
| distinct link_map_fp | same path+ch, different lfp | keys differ | unit |
| boundary: single-char path | "a", valid ch, valid lfp | 32-byte non-zero key | unit |
| boundary: long path | 255-char path | 32-byte non-zero key | unit |
| boundary: multi-byte UTF-8 | "日本語/architecture.md" | 32-byte non-zero key | unit |
| anti: empty path | "", valid ch, valid lfp | Err(EmptySourcePath) / panic | unit |

### Unit Tests: `compute_link_map_fingerprint`

| Scenario | Input Class | Expected Output | Test Layer |
|----------|-------------|-----------------|------------|
| happy path | non-empty map with 3 entries | Ok(ContentHash) where as_bytes().len() == 32 | unit |
| determinism | same entries, different insertion order | Ok(identical ContentHash) | unit |
| distinct contents | map with different IdMapping fields | Ok(different ContentHash) | unit |
| boundary: empty map | HashMap::new() | Ok(ContentHash) where as_bytes().len() == 32 && != [0u8;32] | unit |
| boundary: single entry | map with 1 entry | Ok(ContentHash) with len == 32 | unit |
| boundary: large map | 100 entries | Ok(ContentHash) with len == 32 | unit |
| error: serialization failure | data that triggers serde_json error | Err(LinkMapFingerprintFailed { message: non-empty }) | unit |

### Unit Tests: `TransformArtifact` serde

| Scenario | Input Class | Expected Output | Test Layer |
|----------|-------------|-----------------|------------|
| happy path | valid artifact with non-empty markdown | round-trip equal (PartialEq) | unit |
| boundary: empty markdown | transformed_markdown = "" | round-trip equal (PartialEq) | unit |
| boundary: unicode markdown | transformed_markdown with CJK + emoji | round-trip equal (PartialEq) | unit |
| anti: corrupt JSON | truncated/invalid bytes | Err(serde_json::Error) | unit |
| anti: wrong types | JSON with number where string expected | Err(serde_json::Error) | unit |
| anti: missing fields | JSON missing source_path | Err(serde_json::Error) | unit |

### Integration Tests: `load_cached_artifact`

| Scenario | Input Class | Expected Output | Test Layer |
|----------|-------------|-----------------|------------|
| cache hit | artifact stored under matching key | Ok(Some(artifact)) with matching fields | integration |
| cache miss | no entry for key | Ok(None) | integration |
| corrupt data | garbage bytes under key | Err(DeserializationFailed { source_path: "a.md", message: non-empty }) | integration |
| disabled cache | CacheConfig { enabled_cache_types: vec![CacheType::Analysis] } | Ok(None) | integration |
| storage error | corrupted redb / failed read transaction | Err(CacheReadFailed { source_path: "a.md", message: non-empty }) | integration |
| boundary: large artifact | artifact with 1MB transformed_markdown | Ok(Some(artifact)) with byte-identical 1MB content | integration |

### Integration Tests: `store_artifact`

| Scenario | Input Class | Expected Output | Test Layer |
|----------|-------------|-----------------|------------|
| happy path | valid artifact | Ok(()) + retrievable | integration |
| write-then-read | store then get | byte-identical artifact | integration |
| disabled cache | CacheConfig { enabled_cache_types: vec![CacheType::Analysis] } | Ok(()) (no-op) | integration |
| oversized value | artifact exceeding redb MAX_VALUE_SIZE | Err(CacheWriteFailed { source_path: "a.md", message: non-empty }) + key absent | integration |

### Integration Tests: `write_artifact_to_output`

| Scenario | Input Class | Expected Output | Test Layer |
|----------|-------------|-----------------|------------|
| happy path | valid artifact + link_map | file written with exact content | integration |
| missing mapping | source_path not in link_map | Err(MissingIdMapping { source_path: "orphan.md" }) | integration |
| I/O failure | read-only output dir | Err(OutputWriteFailed { source_path: "a.md", message: non-empty }) | integration |
| boundary: empty markdown | transformed_markdown = "" | Err(OutputWriteFailed { source_path: "a.md", message: contains "empty"/"precondition" }) or panic | integration |
| boundary: large markdown | 10MB transformed_markdown | Ok(()) + file with byte-identical 10MB content | integration |
| boundary: missing docs_dir | docs_dir does not exist | Ok(()) + directory created + file written | integration |

### Integration Tests: `transform_all_cached`

| Scenario | Input Class | Expected Output | Test Layer |
|----------|-------------|-----------------|------------|
| all cache miss | empty cache, 2 analyses | Ok(TransformResult{s:2,t:2,e:0}) + 2 output files | integration |
| all cache hit | pre-populated cache | Ok(TransformResult{s:2,t:2,e:0}) + cached content | integration |
| mixed run counts | 1 cached, 1 fresh | Ok(TransformResult{s:2,t:2,e:0}) | e2e |
| mixed run content | 1 cached, 1 fresh | cached file == artifact, fresh file starts with "---\n" | e2e |
| empty source_path | analysis with source_path="" | Err(EmptySourcePath) | integration |
| missing link_map entry | analysis not in link_map | Err(MissingIdMapping { source_path }) | integration |
| source file missing | nonexistent source file | Err(FileReadFailed { source_path, message: non-empty }) | integration |
| transform fails | malformed content | Err(TransformComputationFailed { source_path, message: non-empty }) | integration |
| cache write fails | oversized artifact | Err(CacheWriteFailed { source_path, message: non-empty }) | integration |
| output write fails | read-only output dir | Err(OutputWriteFailed { source_path, message: non-empty }) | integration |
| empty analyses | analyses = [] | Ok(TransformResult{s:0,t:0,e:0}) | integration |

### E2E Tests: `run_index` with caching

| Scenario | Input Class | Expected Output | Test Layer |
|----------|-------------|-----------------|------------|
| full pipeline | 3 source files | all output files present + correct | e2e |
| idempotent second run | same source, cached | identical output to first run (byte-identical per file) | e2e |

---

## Error Variant Coverage Matrix

| # | Error Variant | Asserting Scenario(s) | Assertion Specificity |
|---|--------------|----------------------|----------------------|
| 1 | `EmptySourcePath` | B30 | Exact variant (no fields) |
| 2 | `MissingIdMapping { source_path }` | B25, B31 | `source_path` matches exactly |
| 3 | `LinkMapFingerprintFailed { message }` | B08 | `message.is_empty() == false` |
| 4 | `CacheReadFailed { source_path, message }` | B19 | `source_path == "a.md"`, `message.is_empty() == false` |
| 5 | `CacheWriteFailed { source_path, message }` | B23, B36 | `source_path` matches exactly, `message.is_empty() == false` |
| 6 | `DeserializationFailed { source_path, message }` | B18 | `source_path == "a.md"`, `message.is_empty() == false` |
| 7 | `FileReadFailed { source_path, message }` | B32 | `source_path == "nonexistent.md"`, `message.is_empty() == false` |
| 8 | `TransformComputationFailed { source_path, message }` | B35 | `source_path == "bad.md"`, `message.is_empty() == false` |
| 9 | `OutputWriteFailed { source_path, message }` | B26, B27, B37 | `source_path` matches exactly, `message.is_empty() == false` |

---

## Open Questions

1. **Transform computation failure simulation**: The contract references `TransformComputationFailed` but the existing `transform_file` returns `anyhow::Error`. The new code must wrap this properly. Tests should confirm the error variant mapping is correct.

2. **Broken-link warnings on cache hit**: Contract explicitly excludes broken links from the artifact. Integration tests should verify that no broken-link warnings are printed during a cache hit (stdout/stderr inspection).

3. **RESOLVED — `LinkMapFingerprintFailed` variant**: Previously flagged as "currently unreachable." This plan now requires `compute_link_map_fingerprint` to return `Result<ContentHash, TransformArtifactError>` (propagating serde errors instead of `.expect()`). B08 provides full BDD coverage. The `.expect()` in the contract must be replaced with proper error propagation.

4. **Thread safety of `transform_all_cached`**: The contract uses `rayon::par_iter` in the existing code. The new `transform_all_cached` may need to be sequential (since cache lookups are sequential per source). Clarify whether parallelism is preserved.

---

**Exit Criteria Checklist:**
- [x] Every public API behavior (B01–B41) has a BDD scenario
- [x] Every error variant (9/9) has an explicit test scenario with specific field assertions
- [x] Every pure function has a proptest invariant (5 invariants)
- [x] Every deserializer has a fuzz target (2 targets)
- [x] Mutation threshold (≥90%) is stated
- [x] No test asserts only `is_ok()` or `is_err()` — all assertions specify exact values, exact error variants, and exact field values
- [x] No bait-and-switch scenarios — all titles match their Then: blocks
- [x] All boundary entries from combinatorial matrix have corresponding BDD scenarios in §3
- [x] Every scenario specifies concrete preconditions (no unqualified "valid" references)
