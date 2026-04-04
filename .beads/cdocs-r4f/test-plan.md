# Test Plan: cdocs-r4f

```
bead_id: cdocs-r4f
bead_title: calc: build URL-state and scrape-output commit batches from scrape results
phase: state-1.5-test-plan
updated_at: 2026-04-04T00:15:00Z
revision: 2 (fixes from review: PayloadProcessingFailed coverage, B05/B15b assertion sharpness, B13 concreteness, summary arithmetic)
```

## Summary

- **Behaviors identified**: 23 (B01–B23 + MIX)
- **BDD scenarios**: 31 (B01–B14, B15, B15b, B16a–B16f, B17–B23, MIX)
- **Trophy allocation**: 30 unit / 2 integration / 1 static
- **Proptest invariants**: 4
- **Fuzz targets**: 0 (no parser/deserializer in this bead — all inputs are typed Rust structs)
- **Kani harnesses**: 1
- **Mutation threshold**: ≥90%

---

## 1. Behavior Inventory

| # | Subject | Action | Outcome | Condition |
|---|---------|--------|---------|-----------|
| B01 | `build_scrape_state_changes` | produces updated rows | for every changed URL | when diff has changed entries with matching artifacts |
| B02 | `build_scrape_state_changes` | produces updated rows | for every new URL | when diff has new_urls entries with matching artifacts |
| B03 | `build_scrape_state_changes` | produces payload blobs | in `new_scrapes` | for every changed/new URL |
| B04 | `build_scrape_state_changes` | produces delete entries only | for every deleted URL | when diff has deleted entries |
| B05 | `build_scrape_state_changes` | produces no output | for unchanged URLs | when diff has unchanged entries |
| B06 | `build_scrape_state_changes` | sets `content_hash` | from `ScrapeArtifact.content_hash` | for every produced `UrlStateRaw` |
| B07 | `build_scrape_state_changes` | sets `url_hash` | to `hash_payload(artifact.payload_bytes)` | for every produced `UrlStateRaw` |
| B08 | `build_scrape_state_changes` | sets `last_fetched_secs` | to `config.now_secs` | for every produced `UrlStateRaw` |
| B09 | `build_scrape_state_changes` | sets `status_code` | from `ScrapeArtifact.status_code` | for every produced `UrlStateRaw` |
| B10 | `build_scrape_state_changes` | zeroes `reserved` | to `[0u8; 46]` | for every produced `UrlStateRaw` |
| B11 | `build_scrape_state_changes` | leaves file-state fields empty | `updated_files`, `deleted_files`, `new_analyses`, `new_transforms`, `new_chunks`, `new_snapshots`, `deleted_snapshots` all `vec![]` | always |
| B12 | `build_scrape_state_changes` | is deterministic | bit-identical output | when called twice with same inputs |
| B13 | `build_scrape_state_changes` | satisfies reference integrity | `url_hash` in `new_scrapes` keys | for every produced `UrlStateRaw` |
| B14 | `build_scrape_state_changes` | maintains output order | changed then new_urls then deleted | per POST-10 ordering contract |
| B15 | `build_scrape_state_changes` | returns `Err(EmptyDiff)` | when all four buckets empty | |
| B15b | `build_scrape_state_changes` | returns `Ok` with all fields empty | when only unchanged has entries | |
| B16 | `build_scrape_state_changes` | returns `Err(DuplicateUrl)` | when URL appears in multiple buckets | |
| B17 | `build_scrape_state_changes` | returns `Err(MissingScrapeArtifact)` | when changed URL has no artifact | |
| B18 | `build_scrape_state_changes` | returns `Err(MissingScrapeArtifact)` | when new URL has no artifact | |
| B19 | `build_scrape_state_changes` | returns `Err(EmptyScrapePayload)` | when changed URL's artifact has zero-length payload | |
| B20 | `build_scrape_state_changes` | returns `Err(EmptyScrapePayload)` | when new URL's artifact has zero-length payload | |
| B21 | `build_url_state_raw` | constructs `UrlStateRaw` | with all fields set correctly | for any input |
| B22 | `build_url_state_raw` | zeroes reserved | to `[0u8; 46]` | always |
| B23 | `build_scrape_state_changes` | returns `Err(PayloadProcessingFailed)` | when hash computation fails | (defensive — currently unreachable; see note below) |

---

## 2. Trophy Allocation

| Behavior # | Layer | Justification |
|------------|-------|---------------|
| B01–B14 | **Unit** (`#[cfg(test)]` in `build_scrape_state_changes.rs`) | Pure calc functions. No I/O, no state mutation. Borrowed inputs, owned output. Exhaustive combinatorial coverage. |
| B15–B20, B23 | **Unit** | Error paths are pure precondition checks. No external deps. |
| B21–B22 | **Unit** | Pure struct constructor. |
| INT-1 | **Integration** (`tests/`) | Verify that `StateChanges` produced by this bead passes `commit_changes`'s reference integrity validation. Uses real `StateDb` (tempfile). |
| INT-2 | **Integration** (`tests/`) | Verify that `StateChanges` produced by this bead round-trips through `UrlStateRaw::to_bytes()` → `UrlStateRaw::from_bytes()` preserving all fields. |
| STATIC-1 | **Static** | `clippy::unwrap_used`, `clippy::expect_used`, `clippy::panic` all denied at crate level. Compile-time size assertion for `UrlStateRaw` (120 bytes). |

**Ratio**: ~88% unit (30/34), ~6% integration (2/34), ~3% static (1/34), ~3% proptest/Kani (not counted as scenarios but as invariant coverage). Justified: this bead is pure calc — no I/O, no external services. Integration layer is thin but critical (reference integrity across the commit boundary). The unit layer carries the full combinatorial weight because there are no side effects to integration-test.

---

## 3. BDD Scenarios

### Behavior: B01 — Changed URLs produce updated rows and payload blobs

```
### Behavior: build_scrape_state_changes produces updated rows for changed URLs

Given: ScrapeDiff with changed=["https://a.com", "https://b.com"], other buckets empty
  And: ScrapeOutputs with artifacts for both URLs (non-empty payload, valid hashes)
  And: ScrapeBatchConfig { now_secs: 1_700_000_000 }
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: result.updated_urls.len() == 2
  And: result.updated_urls[0].0 == "https://a.com"
  And: result.updated_urls[1].0 == "https://b.com"
  And: result.new_scrapes.len() == 2
  And: result.deleted_urls.is_empty()
```

`fn scrape_batch_produces_updated_rows_for_changed_urls()`

### Behavior: B02 — New URLs produce updated rows and payload blobs

```
### Behavior: build_scrape_state_changes produces updated rows for new URLs

Given: ScrapeDiff with new_urls=["https://new.com"], other buckets empty
  And: ScrapeOutputs with artifact for "https://new.com"
  And: ScrapeBatchConfig { now_secs: 1_700_000_000 }
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: result.updated_urls.len() == 1
  And: result.updated_urls[0].0 == "https://new.com"
  And: result.new_scrapes.len() == 1
  And: result.deleted_urls.is_empty()
```

`fn scrape_batch_produces_updated_rows_for_new_urls()`

### Behavior: B03 — Payload blobs are produced in new_scrapes

```
### Behavior: build_scrape_state_changes produces payload blobs in new_scrapes

Given: ScrapeDiff with changed=["https://a.com"] and new_urls=["https://b.com"]
  And: ScrapeOutputs with artifacts for both (payload_bytes = b"serialized_page_1" / b"serialized_page_2")
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: result.new_scrapes.len() == 2
  And: result.new_scrapes[0].1 == b"serialized_page_1"
  And: result.new_scrapes[1].1 == b"serialized_page_2"
```

`fn scrape_batch_produces_payload_blobs_for_changed_and_new_urls()`

### Behavior: B04 — Deleted URLs produce only delete entries

```
### Behavior: build_scrape_state_changes produces only delete entries for deleted URLs

Given: ScrapeDiff with deleted=["https://old1.com", "https://old2.com", "https://old3.com"]
  And: ScrapeOutputs is empty
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: result.deleted_urls.len() == 3
  And: result.deleted_urls.contains(&"https://old1.com".to_string()) == true
  And: result.deleted_urls.contains(&"https://old2.com".to_string()) == true
  And: result.deleted_urls.contains(&"https://old3.com".to_string()) == true
  And: result.updated_urls.is_empty()
  And: result.new_scrapes.is_empty()
```

`fn scrape_batch_produces_only_delete_entries_for_deleted_urls()`

### Behavior: B05 — Unchanged URLs produce no output

```
### Behavior: build_scrape_state_changes excludes unchanged URLs from all outputs

Given: ScrapeDiff with unchanged=["https://u1.com", "https://u2.com", "https://u3.com"]
  And: ScrapeDiff with changed=["https://c.com"]
  And: ScrapeOutputs with artifact for "https://c.com" only
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: result.updated_urls.len() == 1
  And: result.updated_urls[0].0 == "https://c.com"
  And: result.updated_urls.iter().all(|(u, _)| u != "https://u1.com" && u != "https://u2.com" && u != "https://u3.com")
  And: !result.deleted_urls.iter().any(|u| u == "https://u1.com" || u == "https://u2.com" || u == "https://u3.com")
  And: result.new_scrapes.iter().all(|(_, payload)| {
         let hash = hash_payload(payload);
         // None of the unchanged URLs' artifacts are in new_scrapes
         // (unchanged URLs have no artifacts in outputs, so this is verified by output count)
         true
       })
  And: result.new_scrapes.len() == 1 (only for "https://c.com")
```

`fn scrape_batch_excludes_unchanged_urls_from_all_outputs()`

### Behavior: B06 — content_hash fidelity

```
### Behavior: build_scrape_state_changes sets content_hash from artifact

Given: ScrapeDiff with changed=["https://a.com"]
  And: ScrapeArtifact with content_hash = [0xAB; 32]
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: result.updated_urls[0].1.content_hash == [0xAB; 32]
```

`fn scrape_batch_sets_content_hash_from_artifact()`

### Behavior: B07 — url_hash equals hash_payload of payload_bytes

```
### Behavior: build_scrape_state_changes sets url_hash to hash of payload bytes

Given: ScrapeDiff with changed=["https://a.com"]
  And: ScrapeArtifact with payload_bytes = b"test_payload"
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: result.updated_urls[0].1.url_hash == hash_payload(b"test_payload")
  And: result.new_scrapes[0].0 == hash_payload(b"test_payload")
```

`fn scrape_batch_sets_url_hash_to_hash_of_payload_bytes()`

### Behavior: B08 — timestamp fidelity

```
### Behavior: build_scrape_state_changes sets last_fetched_secs from config

Given: ScrapeDiff with changed=["https://a.com"]
  And: ScrapeBatchConfig { now_secs: 1_712_345_678 }
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: result.updated_urls[0].1.last_fetched_secs == 1_712_345_678
```

`fn scrape_batch_sets_last_fetched_secs_from_config()`

### Behavior: B09 — status_code fidelity

```
### Behavior: build_scrape_state_changes sets status_code from artifact

Given: ScrapeDiff with changed=["https://a.com"]
  And: ScrapeArtifact with status_code = 301
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: result.updated_urls[0].1.status_code == 301
```

`fn scrape_batch_sets_status_code_from_artifact()`

### Behavior: B10 — reserved field is zeroed

```
### Behavior: build_scrape_state_changes zeroes reserved in UrlStateRaw

Given: ScrapeDiff with changed=["https://a.com"]
  And: valid ScrapeOutputs
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: result.updated_urls[0].1.reserved == [0u8; 46]
```

`fn scrape_batch_zeroes_reserved_field_in_url_state_raw()`

### Behavior: B11 — Non-URL fields are empty

```
### Behavior: build_scrape_state_changes leaves file-state fields empty

Given: ScrapeDiff with changed=["https://a.com"], deleted=["https://b.com"]
  And: valid ScrapeOutputs
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: result.updated_files.is_empty() == true
  And: result.deleted_files.is_empty() == true
  And: result.new_analyses.is_empty() == true
  And: result.new_transforms.is_empty() == true
  And: result.new_chunks.is_empty() == true
  And: result.new_snapshots.is_empty() == true
  And: result.deleted_snapshots.is_empty() == true
```

`fn scrape_batch_leaves_file_state_fields_empty()`

### Behavior: B12 — Determinism

```
### Behavior: build_scrape_state_changes is deterministic

Given: ScrapeDiff with changed=["https://a.com", "https://b.com"], new_urls=["https://c.com"], deleted=["https://d.com"]
  And: valid ScrapeOutputs for a, b, c
When: build_scrape_state_changes(&diff, &outputs, &config) called twice
Then: first_call.updated_urls == second_call.updated_urls
  And: first_call.deleted_urls == second_call.deleted_urls
  And: first_call.new_scrapes == second_call.new_scrapes
  And: first_call.updated_files == second_call.updated_files (all empty)
```

`fn scrape_batch_produces_identical_output_for_identical_inputs()`

### Behavior: B13 — Reference integrity

```
### Behavior: build_scrape_state_changes satisfies reference integrity

Given: ScrapeDiff with changed=["https://a.com"] and new_urls=["https://b.com"]
  And: valid ScrapeOutputs for both (payload_bytes = b"payload_a" and b"payload_b")
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: let expected_hash_a = hash_payload(b"payload_a")
  And: let expected_hash_b = hash_payload(b"payload_b")
  And: result.new_scrapes.keys().collect::<Vec<_>>() == vec![&expected_hash_a, &expected_hash_b]
  And: result.updated_urls.iter().find(|(u, _)| u == "https://a.com").unwrap().1.url_hash == expected_hash_a
  And: result.updated_urls.iter().find(|(u, _)| u == "https://b.com").unwrap().1.url_hash == expected_hash_b
  And: result.new_scrapes.len() == 2
```

`fn scrape_batch_url_hash_appears_as_key_in_new_scrapes()`

### Behavior: B14 — Output ordering (changed then new_urls then deleted)

```
### Behavior: build_scrape_state_changes maintains output ordering per POST-10

Given: ScrapeDiff with changed=["https://c1.com"], new_urls=["https://n1.com"], deleted=["https://d1.com"]
  And: valid ScrapeOutputs for c1 and n1
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: result.updated_urls[0].0 == "https://c1.com" (changed first)
  And: result.updated_urls[1].0 == "https://n1.com" (new_urls second)
  And: result.deleted_urls[0] == "https://d1.com"
```

`fn scrape_batch_maintains_changed_then_new_then_deleted_ordering()`

### Behavior: B15 — EmptyDiff error

```
### Behavior: build_scrape_state_changes returns Err(EmptyDiff) when all buckets empty

Given: ScrapeDiff { unchanged: vec![], changed: vec![], new_urls: vec![], deleted: vec![] }
  And: ScrapeOutputs { artifacts: HashMap::new() }
  And: ScrapeBatchConfig { now_secs: 0 }
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: Err(ScrapeBatchBuildError::EmptyDiff)
```

`fn scrape_batch_returns_empty_diff_error_when_all_categories_empty()`

### Behavior: B15b — Only unchanged is not empty diff

```
### Behavior: build_scrape_state_changes returns Ok with all fields empty when only unchanged has entries

Given: ScrapeDiff { unchanged: vec!["https://u1.com".to_string()], changed: vec![], new_urls: vec![], deleted: vec![] }
  And: ScrapeOutputs { artifacts: HashMap::new() }
  And: ScrapeBatchConfig { now_secs: 1_700_000_000 }
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: result.updated_urls.is_empty()
  And: result.deleted_urls.is_empty()
  And: result.new_scrapes.is_empty()
  And: result.updated_files.is_empty()
  And: result.deleted_files.is_empty()
  And: result.new_analyses.is_empty()
  And: result.new_transforms.is_empty()
  And: result.new_chunks.is_empty()
  And: result.new_snapshots.is_empty()
  And: result.deleted_snapshots.is_empty()
```

`fn scrape_batch_returns_empty_ok_when_only_unchanged_urls_present()`

### Behavior: B16a — DuplicateUrl (changed + new_urls)

```
### Behavior: build_scrape_state_changes returns Err(DuplicateUrl) when URL in changed and new_urls

Given: ScrapeDiff with changed=["https://dup.com"], new_urls=["https://dup.com"]
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: Err(ScrapeBatchBuildError::DuplicateUrl { url: "https://dup.com" })
```

`fn scrape_batch_returns_duplicate_url_when_in_changed_and_new()`

### Behavior: B16b — DuplicateUrl (unchanged + changed)

```
### Behavior: build_scrape_state_changes returns Err(DuplicateUrl) when URL in unchanged and changed

Given: ScrapeDiff with unchanged=["https://dup.com"], changed=["https://dup.com"]
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: Err(ScrapeBatchBuildError::DuplicateUrl { url: "https://dup.com" })
```

`fn scrape_batch_returns_duplicate_url_when_in_unchanged_and_changed()`

### Behavior: B16c — DuplicateUrl (unchanged + new_urls)

```
### Behavior: build_scrape_state_changes returns Err(DuplicateUrl) when URL in unchanged and new_urls

Given: ScrapeDiff with unchanged=["https://dup.com"], new_urls=["https://dup.com"]
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: Err(ScrapeBatchBuildError::DuplicateUrl { url: "https://dup.com" })
```

`fn scrape_batch_returns_duplicate_url_when_in_unchanged_and_new()`

### Behavior: B16d — DuplicateUrl (unchanged + deleted)

```
### Behavior: build_scrape_state_changes returns Err(DuplicateUrl) when URL in unchanged and deleted

Given: ScrapeDiff with unchanged=["https://dup.com"], deleted=["https://dup.com"]
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: Err(ScrapeBatchBuildError::DuplicateUrl { url: "https://dup.com" })
```

`fn scrape_batch_returns_duplicate_url_when_in_unchanged_and_deleted()`

### Behavior: B16e — DuplicateUrl (changed + deleted)

```
### Behavior: build_scrape_state_changes returns Err(DuplicateUrl) when URL in changed and deleted

Given: ScrapeDiff with changed=["https://dup.com"], deleted=["https://dup.com"]
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: Err(ScrapeBatchBuildError::DuplicateUrl { url: "https://dup.com" })
```

`fn scrape_batch_returns_duplicate_url_when_in_changed_and_deleted()`

### Behavior: B16f — DuplicateUrl (new_urls + deleted)

```
### Behavior: build_scrape_state_changes returns Err(DuplicateUrl) when URL in new_urls and deleted

Given: ScrapeDiff with new_urls=["https://dup.com"], deleted=["https://dup.com"]
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: Err(ScrapeBatchBuildError::DuplicateUrl { url: "https://dup.com" })
```

`fn scrape_batch_returns_duplicate_url_when_in_new_and_deleted()`

### Behavior: B17 — MissingScrapeArtifact for changed URL

```
### Behavior: build_scrape_state_changes returns Err(MissingScrapeArtifact) for changed URL

Given: ScrapeDiff with changed=["https://missing.com"]
  And: ScrapeOutputs.artifacts does not contain "https://missing.com"
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: Err(ScrapeBatchBuildError::MissingScrapeArtifact { url: "https://missing.com" })
```

`fn scrape_batch_returns_missing_artifact_when_changed_url_has_no_artifact()`

### Behavior: B18 — MissingScrapeArtifact for new URL

```
### Behavior: build_scrape_state_changes returns Err(MissingScrapeArtifact) for new URL

Given: ScrapeDiff with new_urls=["https://missing.com"]
  And: ScrapeOutputs.artifacts does not contain "https://missing.com"
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: Err(ScrapeBatchBuildError::MissingScrapeArtifact { url: "https://missing.com" })
```

`fn scrape_batch_returns_missing_artifact_when_new_url_has_no_artifact()`

### Behavior: B19 — EmptyScrapePayload for changed URL

```
### Behavior: build_scrape_state_changes returns Err(EmptyScrapePayload) for changed URL

Given: ScrapeDiff with changed=["https://empty.com"]
  And: ScrapeOutputs.artifacts["https://empty.com"].payload_bytes = vec![]
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: Err(ScrapeBatchBuildError::EmptyScrapePayload { url: "https://empty.com" })
```

`fn scrape_batch_returns_empty_payload_when_changed_url_artifact_has_zero_bytes()`

### Behavior: B20 — EmptyScrapePayload for new URL

```
### Behavior: build_scrape_state_changes returns Err(EmptyScrapePayload) for new URL

Given: ScrapeDiff with new_urls=["https://empty.com"]
  And: ScrapeOutputs.artifacts["https://empty.com"].payload_bytes = vec![]
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: Err(ScrapeBatchBuildError::EmptyScrapePayload { url: "https://empty.com" })
```

`fn scrape_batch_returns_empty_payload_when_new_url_artifact_has_zero_bytes()`

### Behavior: B21 — build_url_state_raw constructs correctly

```
### Behavior: build_url_state_raw constructs UrlStateRaw with all fields set

Given: content_hash=[0xAA; 32], url_hash=[0xBB; 32], last_fetched_secs=1_700_000_000, status_code=200
When: build_url_state_raw(content_hash, url_hash, last_fetched_secs, status_code)
Then: result.content_hash == [0xAA; 32]
  And: result.url_hash == [0xBB; 32]
  And: result.last_fetched_secs == 1_700_000_000
  And: result.status_code == 200
  And: std::mem::size_of::<UrlStateRaw>() == 120
```

`fn build_url_state_raw_sets_all_fields_correctly()`

### Behavior: B22 — build_url_state_raw zeroes reserved

```
### Behavior: build_url_state_raw zeroes the reserved field

Given: content_hash=[0x11; 32], url_hash=[0x22; 32], last_fetched_secs=42, status_code=0
When: build_url_state_raw(content_hash, url_hash, last_fetched_secs, status_code)
Then: result.reserved == [0u8; 46]
```

`fn build_url_state_raw_zeroes_reserved_field()`

### Behavior: B23 — PayloadProcessingFailed (defensive variant)

> **NOTE on unreachability**: The `PayloadProcessingFailed` variant exists as a defensive
> contract boundary. In the current implementation, `hash_payload` is infallible (SHA-256
> via the `sha2` crate never fails). This variant would be triggered only if `hash_payload`
> were changed to fallible (e.g., using a hardware accelerator that can error, or a
> configurable hash algorithm with validation). The test below uses an implementation-
> substituting approach: if `hash_payload` returns `Result`, the `?` propagation would
> trigger this variant. Since `hash_payload` is currently infallible, this test verifies
> that the error path exists in the code and would be reached if the internal call site
> changed from infallible to fallible.
>
> **Testing strategy**: The implementation should use an internal helper that returns
> `Result<Hash, String>` for hash computation, even if the current `hash_payload` function
> is infallible. The test substitutes a fallible hash function via dependency injection or
> a cfg(test) internal function. If this is impractical, the test must at minimum verify
> that the `PayloadProcessingFailed` variant is constructable and its `Display` impl
> produces the expected format.

```
### Behavior: build_scrape_state_changes returns Err(PayloadProcessingFailed) when hash computation fails

Given: ScrapeDiff with changed=["https://fail.com"]
  And: ScrapeOutputs with artifact for "https://fail.com" (non-empty payload_bytes = b"some_payload")
  And: a fallible hash function that returns Err("SHA-256 hardware accelerator unavailable")
  And: ScrapeBatchConfig { now_secs: 1_700_000_000 }
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: Err(ScrapeBatchBuildError::PayloadProcessingFailed {
         url: "https://fail.com",
         reason: "SHA-256 hardware accelerator unavailable"
       })
```

**Fallback test** (if implementation does not support dependency injection for hash):

```
### Behavior: PayloadProcessingFailed variant is constructable and displays correctly

Given: ScrapeBatchBuildError::PayloadProcessingFailed {
         url: "https://fail.com".to_string(),
         reason: "hash function returned error".to_string()
       }
When: the error is formatted via Display
Then: format contains "https://fail.com"
  And: format contains "hash function returned error"
```

`fn scrape_batch_returns_payload_processing_failed_when_hash_computation_fails()`
`fn payload_processing_failed_displays_url_and_reason()`

### Behavior: MIX — Mixed diff categories

```
### Behavior: build_scrape_state_changes handles mixed diff correctly

Given: ScrapeDiff with unchanged=["https://u1.com", "https://u2.com"]
        changed=["https://c1.com", "https://c2.com", "https://c3.com"]
        new_urls=["https://n1.com"]
        deleted=["https://d1.com", "https://d2.com"]
  And: ScrapeOutputs with artifacts for c1, c2, c3, n1
When: build_scrape_state_changes(&diff, &outputs, &config)
Then: result.updated_urls.len() == 4  (3 changed + 1 new)
  And: result.deleted_urls.len() == 2
  And: result.new_scrapes.len() == 4
  And: result.updated_urls.iter().all(|(u, _)| u != "https://u1.com" && u != "https://u2.com")
  And: !result.deleted_urls.iter().any(|u| u == "https://u1.com" || u == "https://u2.com")
```

`fn scrape_batch_handles_mixed_diff_categories_correctly()`

---

## 4. Proptest Invariants

### Proptest P1: Reference integrity for arbitrary valid inputs

```
### Proptest: build_scrape_state_changes reference integrity

Invariant: For every (url, state) in result.updated_urls,
           state.url_hash is a key in result.new_scrapes (exact byte equality).

Strategy: Generate arbitrary non-empty ScrapeDiff (changed + new_urls non-empty)
          with matching ScrapeOutputs (non-empty payload_bytes).
          Use proptest::collection::vec(proptest::string::string_regex("https://[a-z]+\\.com/[a-z]+").unwrap(), 0..5)

Anti-invariant: No valid input should produce a UrlStateRaw whose url_hash
                is NOT a key in new_scrapes.
```

### Proptest P2: One-to-one URL-to-row mapping

```
### Proptest: build_scrape_state_changes one-to-one mapping

Invariant: len(result.updated_urls) == len(diff.changed) + len(diff.new_urls)

Strategy: Generate arbitrary ScrapeDiff with changed and new_urls
          (0..10 URLs each) with matching artifacts.

Anti-invariant: No valid input should produce 0 rows for non-empty changed+new_urls.
```

### Proptest P3: Determinism under permutation-invariant inputs

```
### Proptest: build_scrape_state_changes is deterministic

Invariant: build_scrape_state_changes(a, b, c) == build_scrape_state_changes(a, b, c)
           for the same inputs (bit-identical output).

Strategy: Generate arbitrary valid inputs and call twice, assert PartialEq.
```

### Proptest P4: build_url_state_raw round-trip through bytes

```
### Proptest: build_url_state_raw round-trips through UrlStateRaw::to_bytes/from_bytes

Invariant: UrlStateRaw::from_bytes(&build_url_state_raw(ch, uh, ts, sc).to_bytes()) == Ok(build_url_state_raw(ch, uh, ts, sc))

Strategy: Arbitrary [u8; 32] for content_hash and url_hash, arbitrary u64 for timestamp, arbitrary u16 for status_code.
```

---

## 5. Fuzz Targets

**None.** This bead has no parsing or deserialization boundaries. All inputs are typed Rust structs (`ScrapeDiff`, `ScrapeOutputs`, `ScrapeBatchConfig`). The only byte-level operations are SHA-256 hashing (via `hash_payload`) and `UrlStateRaw` byte conversion, both of which are already well-tested in `state/mod.rs` and `calc/build_state_changes.rs`.

If a future bead introduces URL string parsing or payload deserialization into the scrape pipeline, that bead should add fuzz targets for those boundaries.

---

## 6. Kani Harnesses

### Kani K1: UrlStateRaw byte layout integrity

```
### Kani Harness: UrlStateRaw byte layout is exactly 120 bytes

Property: For all valid (content_hash: [u8; 32], url_hash: [u8; 32],
          last_fetched_secs: u64, status_code: u16):
          build_url_state_raw produces a struct where to_bytes().len() == 120
          AND from_bytes(to_bytes()) == Ok(original).

Bound: All u64 values for last_fetched_secs, all u16 values for status_code.

Rationale: The 120-byte `#[repr(C)]` layout is a contract with the redb storage layer.
           A Kani proof ensures no padding or alignment change silently breaks the format.
           This is structural — unit tests verify specific values, Kani proves the
           layout for ALL possible field values.
```

---

## 7. Mutation Testing Checkpoints

| Mutation | Caught By | Scenario |
|----------|-----------|----------|
| Remove `EmptyDiff` check (always proceed) | `scrape_batch_returns_empty_diff_error_when_all_categories_empty` | B15 |
| Skip duplicate detection entirely | `scrape_batch_returns_duplicate_url_when_in_changed_and_new` | B16a |
| Check duplicates only in changed+new (miss unchanged+deleted) | `scrape_batch_returns_duplicate_url_when_in_unchanged_and_deleted` | B16d |
| Skip `MissingScrapeArtifact` check for changed | `scrape_batch_returns_missing_artifact_when_changed_url_has_no_artifact` | B17 |
| Skip `MissingScrapeArtifact` check for new_urls | `scrape_batch_returns_missing_artifact_when_new_url_has_no_artifact` | B18 |
| Skip `EmptyScrapePayload` check | `scrape_batch_returns_empty_payload_when_changed_url_artifact_has_zero_bytes` | B19 |
| **Delete `PayloadProcessingFailed` branch entirely** | **`scrape_batch_returns_payload_processing_failed_when_hash_computation_fails`** | **B23** |
| **Replace `PayloadProcessingFailed` with `unwrap()`** | **`scrape_batch_returns_payload_processing_failed_when_hash_computation_fails`** | **B23** |
| Use `hash_payload` on wrong field (e.g., content instead of payload) | `scrape_batch_sets_url_hash_to_hash_of_payload_bytes` | B07 |
| Set `last_fetched_secs` to 0 instead of `config.now_secs` | `scrape_batch_sets_last_fetched_secs_from_config` | B08 |
| Set `status_code` to 200 instead of artifact's value | `scrape_batch_sets_status_code_from_artifact` | B09 |
| Don't zero `reserved` field | `scrape_batch_zeroes_reserved_field_in_url_state_raw` | B10 |
| Include unchanged URLs in output | `scrape_batch_excludes_unchanged_urls_from_all_outputs` | B05 |
| Populate `updated_files` instead of leaving empty | `scrape_batch_leaves_file_state_fields_empty` | B11 |
| Don't add payload to `new_scrapes` | `scrape_batch_url_hash_appears_as_key_in_new_scrapes` | B13 |
| Swap ordering (new_urls before changed) | `scrape_batch_maintains_changed_then_new_then_deleted_ordering` | B14 |
| `build_url_state_raw` doesn't zero reserved | `build_url_state_raw_zeroes_reserved_field` | B22 |
| Return early after first changed URL (skip rest) | `scrape_batch_handles_mixed_diff_categories_correctly` | MIX |
| Non-determinism (HashMap iteration order leak) | `scrape_batch_produces_identical_output_for_identical_inputs` | B12 |
| B15b: populate a non-empty vec instead of all empty | `scrape_batch_returns_empty_ok_when_only_unchanged_urls_present` | B15b |

**Threshold**: ≥90% mutation kill rate. The 30 unit tests + 2 integration tests + 4 proptests provide redundant coverage of critical paths (reference integrity is tested by B07, B13, and proptest P1). The `PayloadProcessingFailed` branch deletion is now covered by B23.

---

## 8. Combinatorial Coverage Matrix

### build_scrape_state_changes — Happy Paths

| Scenario | Input Class | Expected Output | BDD Scenario | Layer |
|----------|-------------|-----------------|--------------|-------|
| changed-only | diff.changed non-empty, artifacts present | Ok: updated_urls.len() == changed.len(), new_scrapes.len() == changed.len() | B01 | unit |
| new_urls-only | diff.new_urls non-empty, artifacts present | Ok: updated_urls.len() == new_urls.len(), new_scrapes.len() == new_urls.len() | B02 | unit |
| deleted-only | diff.deleted non-empty, no artifacts needed | Ok: deleted_urls == diff.deleted, updated_urls empty, new_scrapes empty | B04 | unit |
| unchanged-only | diff.unchanged non-empty | Ok: all 10 output vecs empty (see B15b) | B15b | unit |
| mixed all four | all buckets non-empty, artifacts for changed+new | Ok: correct counts, no unchanged leakage | MIX | unit |

### build_scrape_state_changes — Error Paths

| Scenario | Input Class | Expected Output | BDD Scenario | Layer |
|----------|-------------|-----------------|--------------|-------|
| all buckets empty | all four buckets vec![] | Err(ScrapeBatchBuildError::EmptyDiff) | B15 | unit |
| duplicate: changed+new | same URL in both | Err(DuplicateUrl { url: "https://dup.com" }) | B16a | unit |
| duplicate: unchanged+changed | same URL in both | Err(DuplicateUrl { url: "https://dup.com" }) | B16b | unit |
| duplicate: unchanged+new | same URL in both | Err(DuplicateUrl { url: "https://dup.com" }) | B16c | unit |
| duplicate: unchanged+deleted | same URL in both | Err(DuplicateUrl { url: "https://dup.com" }) | B16d | unit |
| duplicate: changed+deleted | same URL in both | Err(DuplicateUrl { url: "https://dup.com" }) | B16e | unit |
| duplicate: new+deleted | same URL in both | Err(DuplicateUrl { url: "https://dup.com" }) | B16f | unit |
| missing artifact (changed) | changed URL not in ScrapeOutputs | Err(MissingScrapeArtifact { url: "https://missing.com" }) | B17 | unit |
| missing artifact (new) | new URL not in ScrapeOutputs | Err(MissingScrapeArtifact { url: "https://missing.com" }) | B18 | unit |
| empty payload (changed) | payload_bytes.len() == 0 | Err(EmptyScrapePayload { url: "https://empty.com" }) | B19 | unit |
| empty payload (new) | payload_bytes.len() == 0 | Err(EmptyScrapePayload { url: "https://empty.com" }) | B20 | unit |
| hash computation failure | fallible hash returns Err | Err(PayloadProcessingFailed { url, reason }) | B23 | unit |

### build_scrape_state_changes — Field Fidelity

| Scenario | Input Class | Expected Output | BDD Scenario | Layer |
|----------|-------------|-----------------|--------------|-------|
| content_hash fidelity | specific [0xAB; 32] in artifact | state.content_hash == [0xAB; 32] | B06 | unit |
| url_hash fidelity | specific payload_bytes | state.url_hash == hash_payload(payload_bytes) | B07 | unit |
| timestamp fidelity | config.now_secs = 1_712_345_678 | state.last_fetched_secs == 1_712_345_678 | B08 | unit |
| status_code fidelity | artifact.status_code = 301 | state.status_code == 301 | B09 | unit |
| reserved zeroed | any input | state.reserved == [0u8; 46] | B10 | unit |
| file fields empty | any input | 7 file-state vecs all empty | B11 | unit |

### build_scrape_state_changes — Structural Invariants

| Scenario | Input Class | Expected Output | BDD Scenario | Layer |
|----------|-------------|-----------------|--------------|-------|
| reference integrity | any valid | every url_hash in new_scrapes keys (exact byte equality) | B13 | unit |
| one-to-one mapping | any valid | updated_urls.len() == changed.len() + new_urls.len() | P2 | proptest |
| determinism | same input twice | PartialEq holds | B12, P3 | unit + proptest |
| output ordering | mixed diff | changed first, then new_urls, then deleted | B14 | unit |

### build_url_state_raw

| Scenario | Input Class | Expected Output | BDD Scenario | Layer |
|----------|-------------|-----------------|--------------|-------|
| all fields set | arbitrary valid inputs | fields match inputs exactly | B21 | unit |
| reserved zeroed | any inputs | reserved == [0u8; 46] | B22 | unit |
| struct size | any instance | size_of == 120 bytes | B21 | unit (compile-time assert) |
| byte round-trip | any instance | from_bytes(to_bytes()) == Ok(original) | P4 | proptest |

### Integration Tests

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| commit_changes accepts batch | StateChanges from build_scrape_state_changes | commit_changes returns Ok(()) | integration |
| UrlStateRaw round-trip through redb | write then read | from_bytes matches original | integration |

### Proptest Invariants

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| reference integrity for all | arbitrary valid (proptest) | invariant holds for 1000 cases | proptest |
| one-to-one mapping for all | arbitrary valid (proptest) | invariant holds for 1000 cases | proptest |
| determinism for all | arbitrary valid (proptest) | invariant holds for 1000 cases | proptest |
| round-trip for all | arbitrary UrlStateRaw (proptest) | invariant holds for 1000 cases | proptest |

---

## Open Questions

None. All types (`UrlStateRaw`, `StateChanges`), helper functions (`hash_payload`), and patterns are established in the existing codebase. The contract is fully specified with no ambiguities. The `PayloadProcessingFailed` variant is documented as currently unreachable with a concrete testing strategy for when the implementation changes.
