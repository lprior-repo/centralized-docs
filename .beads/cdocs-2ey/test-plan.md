bead_id: cdocs-2ey
bead_title: action: wire scrape command to one shared read session and one shutdown commit
phase: state-1.5-test-plan
updated_at: 2026-04-03T01:30:00Z

# Test Plan: Wire scrape command to shared read session and shutdown commit

## Summary
- Behaviors identified: 48
- Trophy allocation: 22 unit / 17 integration / 3 e2e / 6 static
- BDD scenarios: 35 (32 unit/integration + 3 e2e)
- Proptest invariants: 6
- Fuzz targets: 3
- Kani harnesses: 3
- Mutation threshold: ≥90% kill rate

## 1. Behavior Inventory

### Calc Layer — Pure Functions (new module: `calc/scrape_diff.rs`)

#### `classify_scrape_diff` (9 behaviors)

1. `classify_scrape_diff` returns `ScrapeDiff { new, changed, unchanged }` when given stored URL states and freshly scraped pages with mixed classifications
2. `classify_scrape_diff` classifies page as New when URL is absent from stored_url_states, asserting specific URL identities
3. `classify_scrape_diff` classifies page as Changed when URL exists but content_hash differs
4. `classify_scrape_diff` classifies page as Unchanged when URL exists and content_hash matches
5. `classify_scrape_diff` handles empty stored_url_states (first run — all pages are New)
6. `classify_scrape_diff` handles empty scraped_pages (no work — all buckets empty)
7. `classify_scrape_diff` returns empty ScrapeDiff when both inputs are empty
8. `classify_scrape_diff` classifies all pages as Changed when every stored content_hash differs from scraped
9. `classify_scrape_diff` correctly classifies when stored entry has content_hash == [0u8; 32] (zero-hash boundary)
10. `classify_scrape_diff` correctly handles partial URL overlap (some stored URLs not scraped, some scraped URLs not stored)
11. `classify_scrape_diff` correctly classifies when stored entry has content_hash == [1u8; 32] (non-zero boundary value)

#### `build_scrape_state_changes` (15 behaviors)

12. `build_scrape_state_changes` returns StateChanges with updated_urls for new and changed pages
13. `build_scrape_state_changes` returns StateChanges with new_scrapes keyed by SHA-256(persisted_bytes)
14. `build_scrape_state_changes` excludes unchanged pages from all output fields
15. `build_scrape_state_changes` sets correct content_hash per page independently (≥2 pages with distinct markdown)
16. `build_scrape_state_changes` produces unique updated_urls keys (INV-5)
17. `build_scrape_state_changes` produces non-zero new_scrapes hash keys (INV-7)
18. `build_scrape_state_changes` ensures every non-zero url_hash has a corresponding new_scrapes entry (INV-6)
19. `build_scrape_state_changes` sets last_fetched_secs from the provided timestamp
20. `build_scrape_state_changes` returns empty StateChanges when ScrapeDiff has only unchanged pages
21. `build_scrape_state_changes` serializes PersistedScrapeResult via rkyv for new_scrapes values
22. `build_scrape_state_changes` handles new-pages-only ScrapeDiff correctly (changed=[], unchanged=[])
23. `build_scrape_state_changes` handles changed-pages-only ScrapeDiff correctly (new=[], unchanged=[])
24. `build_scrape_state_changes` handles timestamp = 0 correctly (all UrlStateRaw.last_fetched_secs == 0)
25. `build_scrape_state_changes` handles timestamp = u64::MAX correctly (no overflow, exact value preserved)
26. `build_scrape_state_changes` handles empty markdown for a page (content_hash == SHA-256 of empty bytes)

### Calc Layer — Pure Functions (existing, consumed by wiring)

27. `hash_payload` returns deterministic SHA-256 for any byte slice
28. `scrape_result_to_persisted` converts ScrapeResult to PersistedScrapeResult with schema_version 1

### Integration — State Wiring in `run_scrape`

29. `run_scrape` opens StateDb at `output.join("state.redb")` on valid path
30. `run_scrape` creates exactly one StateReadSession (INV-1, POST-1)
31. `run_scrape` loads stored URL states at startup via `session.load_url_states()`
32. `run_scrape` loads persisted scrape outputs for unchanged pages via `session.load_scrapes()`
33. `run_scrape` commits StateChanges exactly once at shutdown (POST-2)
34. `run_scrape` drops StateReadSession before calling commit_changes (INV-3)
35. `run_scrape` produces committed StateChanges with correct updated_urls (POST-3)
36. `run_scrape` produces committed StateChanges with correct new_scrapes (POST-3)
37. `run_scrape` reuses unchanged pages from persisted scrape_outputs (POST-4)
38. `run_scrape` produces ScrapeResult combining reused + freshly scraped pages with exact page count (POST-5)
39. `run_scrape` performs zero per-page writes to state.redb (INV-1)

### Integration — Error Propagation

40. `run_scrape` wraps StateDb::open failure with "failed to open state database"
41. `run_scrape` wraps load_url_states failure with "failed to load URL states"
42. `run_scrape` propagates load_scrapes failure with context message (BulkLoadError from contract.md:71)
43. `run_scrape` propagates StateReadSession::new failure (BulkLoadError::StorageError from contract.md:69)
44. `run_scrape` wraps commit_changes failure with "failed to commit scrape state" (POST-7)
45. `run_scrape` returns Err and leaves state.redb unchanged when scrape fails before commit (INV-2, POST-6)

### E2E — Full Command Lifecycle

46. `ctd scrape` succeeds end-to-end on first run (no prior state) and creates state.redb
47. `ctd scrape` succeeds end-to-end on second run and reuses unchanged pages (verified by HTTP request count)
48. `ctd scrape` fails gracefully when state.redb path is unwritable

### Static Analysis

49. `forbid(unsafe_code)` enforced in calc/scrape_diff.rs
50. `clippy::unwrap_used` and `clippy::expect_used` denied in calc/scrape_diff.rs
51. `UrlStateRaw` is exactly 120 bytes (compile-time assert, existing)
52. `StateChanges` struct layout matches commit.rs definition (no field drift)
53. `content_hash == [0u8; 32]` is NOT treated as a sentinel "no hash" value — zero-hash is a valid SHA-256 output
54. Public API signature unchanged: `pub async fn run_scrape(url, output, config) -> Result<()>` (INV-8)

## 2. Trophy Allocation

| # | Behavior | Layer | Justification |
|---|----------|-------|---------------|
| 1-11 | `classify_scrape_diff` | **Unit** | Pure function: HashMap + Vec in, ScrapeDiff out. No I/O. Exhaustive combinatorial with boundary cases. |
| 12-26 | `build_scrape_state_changes` | **Unit** | Pure function: ScrapeDiff + data in, StateChanges out. No I/O. Boundary cases for timestamp, markdown, and page composition. |
| 27 | `hash_payload` | **Unit** | Already tested in existing codebase; verify no regression. |
| 28 | `scrape_result_to_persisted` | **Unit** | Already tested in persisted.rs; verify no regression. |
| 29-39 | State wiring in `run_scrape` | **Integration** | Touches StateDb (real redb), file system, async runtime. Real deps. |
| 40-45 | Error propagation | **Integration** | Requires real StateDb to trigger commit/validation/read-session failures. |
| 46-48 | Full command lifecycle | **E2E** | Black-box CLI invocation with real filesystem + state database. |
| 49-54 | Static analysis | **Static** | Compile-time enforcement. Free. |

**Ratio**: 22 unit (42%) / 17 integration (33%) / 3 e2e (6%) / 6 static (12%)*
\* Static exceeds 5% target because `forbid(unsafe_code)`, compile-time size assertions, and the zero-hash sentinel invariant are critical safety properties for the 120-byte Pod struct. The effective test-code ratio (unit+integration+e2e) is 42/33/6 — the unit-heavy ratio reflects that 2 new pure functions (`classify_scrape_diff`, `build_scrape_state_changes`) demand exhaustive boundary coverage with 15+ behaviors each. Integration tests still cover all wiring paths including 6 error propagation scenarios.

## 3. BDD Scenarios

### Behavior 1: classify_scrape_diff — mixed classification

```
### Behavior: classify_scrape_diff returns correct ScrapeDiff for mixed input
Given: stored_url_states contains {
        "https://a.com/page1": UrlStateRaw{content_hash: HASH_A},
        "https://a.com/page2": UrlStateRaw{content_hash: HASH_B}
      }
  And scraped_pages contains [
        ScrapedPage{url: "https://a.com/page1", markdown: "old content"},  // content_hash == HASH_A → unchanged
        ScrapedPage{url: "https://a.com/page2", markdown: "new content"},  // content_hash != HASH_B → changed
        ScrapedPage{url: "https://a.com/page3", markdown: "brand new"},    // not in stored → new
      ]
When: classify_scrape_diff(&stored_url_states, &scraped_pages) is called
Then: result.unchanged contains exactly ["https://a.com/page1"]
  And: result.changed contains exactly ["https://a.com/page2"]
  And: result.new contains exactly ["https://a.com/page3"]
```

Test function: `fn classify_scrape_diff_returns_correct_partitions_for_mixed_pages()`

### Behavior 2: classify_scrape_diff — all pages New on first run [FIXED: assert specific URLs]

```
### Behavior: classify_scrape_diff classifies all pages as New when stored states empty
Given: stored_url_states is empty HashMap
  And scraped_pages contains [
        ScrapedPage{url: "https://a.com/page1", markdown: "content 1"},
        ScrapedPage{url: "https://b.com/page2", markdown: "content 2"},
        ScrapedPage{url: "https://c.com/page3", markdown: "content 3"},
      ]
When: classify_scrape_diff(&stored_url_states, &scraped_pages) is called
Then: result.new.len() == 3
  And: result.new contains exactly ["https://a.com/page1", "https://b.com/page2", "https://c.com/page3"]
  And: result.changed.is_empty() == true
  And: result.unchanged.is_empty() == true
```

Test function: `fn classify_scrape_diff_classifies_all_as_new_when_stored_states_empty()`

### Behavior 3: classify_scrape_diff — all Unchanged

```
### Behavior: classify_scrape_diff classifies all pages as Unchanged when hashes match
Given: stored_url_states contains {"https://a.com/p": UrlStateRaw{content_hash: HASH_P}}
  And scraped_pages contains ScrapedPage{url: "https://a.com/p", markdown: content_that_hashes_to_HASH_P}
When: classify_scrape_diff(&stored_url_states, &scraped_pages) is called
Then: result.unchanged.len() == 1
  And: result.new.is_empty() == true
  And: result.changed.is_empty() == true
```

Test function: `fn classify_scrape_diff_classifies_all_as_unchanged_when_hashes_match()`

### Behavior 4: classify_scrape_diff — empty scraped pages

```
### Behavior: classify_scrape_diff returns empty ScrapeDiff when no pages scraped
Given: stored_url_states contains 5 entries
  And scraped_pages is empty Vec
When: classify_scrape_diff(&stored_url_states, &scraped_pages) is called
Then: result.unchanged.is_empty() == true
  And: result.changed.is_empty() == true
  And: result.new.is_empty() == true
```

Test function: `fn classify_scrape_diff_returns_empty_when_no_pages_scraped()`

### Behavior 5: classify_scrape_diff — both inputs empty

```
### Behavior: classify_scrape_diff returns empty ScrapeDiff when both inputs empty
Given: stored_url_states is empty
  And scraped_pages is empty
When: classify_scrape_diff(&stored_url_states, &scraped_pages) is called
Then: result.unchanged.is_empty() == true
  And: result.changed.is_empty() == true
  And: result.new.is_empty() == true
```

Test function: `fn classify_scrape_diff_returns_empty_when_both_inputs_empty()`

### Behavior 8: classify_scrape_diff — all Changed [NEW — fixes MAJOR-2]

```
### Behavior: classify_scrape_diff classifies all pages as Changed when all content hashes differ
Given: stored_url_states contains {
        "https://a.com/p1": UrlStateRaw{content_hash: HASH_OLD_1},
        "https://a.com/p2": UrlStateRaw{content_hash: HASH_OLD_2},
        "https://a.com/p3": UrlStateRaw{content_hash: HASH_OLD_3}
      }
  And scraped_pages contains [
        ScrapedPage{url: "https://a.com/p1", markdown: "completely different 1"},
        ScrapedPage{url: "https://a.com/p2", markdown: "completely different 2"},
        ScrapedPage{url: "https://a.com/p3", markdown: "completely different 3"},
      ]
  And: for each page, SHA-256(markdown) != stored content_hash
When: classify_scrape_diff(&stored_url_states, &scraped_pages) is called
Then: result.changed.len() == 3
  And: result.changed contains exactly ["https://a.com/p1", "https://a.com/p2", "https://a.com/p3"]
  And: result.new.is_empty() == true
  And: result.unchanged.is_empty() == true
```

Test function: `fn classify_scrape_diff_classifies_all_as_changed_when_all_hashes_differ()`

### Behavior 9: classify_scrape_diff — zero content_hash boundary [NEW — fixes MAJOR-2]

```
### Behavior: classify_scrape_diff correctly classifies zero content_hash boundary
Given: stored_url_states contains {
        "https://a.com/zero": UrlStateRaw{content_hash: [0u8; 32]}
      }
  And scraped_pages contains [
        ScrapedPage{url: "https://a.com/zero", markdown: ""}  // SHA-256("") = e3b0c44298fc1c149afbf4c8996fb924... (non-zero)
      ]
When: classify_scrape_diff(&stored_url_states, &scraped_pages) is called
Then: result.changed contains exactly ["https://a.com/zero"]
  And: result.new.is_empty() == true
  And: result.unchanged.is_empty() == true
```

Test function: `fn classify_scrape_diff_handles_zero_content_hash_boundary()`

### Behavior 10: classify_scrape_diff — partial URL overlap [NEW — fixes MAJOR-2]

```
### Behavior: classify_scrape_diff correctly handles partial URL overlap
Given: stored_url_states contains {
        "https://a.com/p1": UrlStateRaw{content_hash: HASH_P1},
        "https://a.com/p2": UrlStateRaw{content_hash: HASH_P2},
        "https://a.com/p3": UrlStateRaw{content_hash: HASH_P3},
        "https://a.com/p4": UrlStateRaw{content_hash: HASH_P4},
        "https://a.com/p5": UrlStateRaw{content_hash: HASH_P5}
      }
  And scraped_pages contains [
        ScrapedPage{url: "https://a.com/p1", markdown: content_hashing_to_HASH_P1},  // unchanged
        ScrapedPage{url: "https://a.com/p2", markdown: "different content"},          // changed
        ScrapedPage{url: "https://a.com/p6", markdown: "brand new page"},            // new (not in stored)
      ]
When: classify_scrape_diff(&stored_url_states, &scraped_pages) is called
Then: result.unchanged.contains("https://a.com/p1") == true
  And: result.changed.contains("https://a.com/p2") == true
  And: result.new.contains("https://a.com/p6") == true
  And: result.len_all_partitions() == 3
  And: stored URLs p3, p4, p5 do NOT appear in any partition (they were not scraped)
```

Test function: `fn classify_scrape_diff_handles_partial_url_overlap()`

### Behavior 11: classify_scrape_diff — non-zero content_hash boundary [NEW — fixes MAJOR-2]

```
### Behavior: classify_scrape_diff correctly classifies non-zero boundary content_hash
Given: stored_url_states contains {
        "https://a.com/p": UrlStateRaw{content_hash: [1u8; 32]}
      }
  And scraped_pages contains [
        ScrapedPage{url: "https://a.com/p", markdown: content_that_hashes_to_exactly_0101...01_bytes}
      ]
  Where: SHA-256(markdown.as_bytes()) == [1u8; 32]
When: classify_scrape_diff(&stored_url_states, &scraped_pages) is called
Then: result.unchanged contains exactly ["https://a.com/p"]
  And: result.changed.is_empty() == true
  And: result.new.is_empty() == true
```

Test function: `fn classify_scrape_diff_handles_non_zero_content_hash_boundary()`

### Behavior 12: build_scrape_state_changes — new and changed pages

```
### Behavior: build_scrape_state_changes produces updated_urls and new_scrapes for new and changed pages
Given: scrape_diff has new=["https://a.com/new"], changed=["https://a.com/changed"], unchanged=["https://a.com/same"]
  And scraped_pages contains all 3 pages with their markdown
  And timestamp = 1_700_000_000
When: build_scrape_state_changes(&scrape_diff, &scraped_pages, timestamp) is called
Then: changes.updated_urls.len() == 2
  And: changes.updated_urls contains entry for "https://a.com/new"
  And: changes.updated_urls contains entry for "https://a.com/changed"
  And: changes.updated_urls does NOT contain entry for "https://a.com/same"
  And: changes.new_scrapes.len() == 2
  And: changes.new_scrapes keys are all non-zero [u8; 32]
  And: every UrlStateRaw.last_fetched_secs == 1_700_000_000
  And: every UrlStateRaw.status_code == 200
```

Test function: `fn build_scrape_state_changes_produces_entries_for_new_and_changed_pages()`

### Behavior 14: build_scrape_state_changes — unchanged pages excluded

```
### Behavior: build_scrape_state_changes excludes unchanged pages from all outputs
Given: scrape_diff has new=[], changed=[], unchanged=["https://a.com/same"]
When: build_scrape_state_changes(&scrape_diff, &[], timestamp) is called
Then: changes.updated_urls.is_empty() == true
  And: changes.new_scrapes.is_empty() == true
  And: changes.deleted_urls.is_empty() == true
```

Test function: `fn build_scrape_state_changes_excludes_unchanged_pages_from_all_outputs()`

### Behavior 16: build_scrape_state_changes — unique updated_urls keys

```
### Behavior: build_scrape_state_changes produces unique updated_urls keys (INV-5)
Given: scrape_diff has new and changed pages with distinct URLs (no overlap)
When: build_scrape_state_changes(&scrape_diff, &scraped_pages, timestamp) is called
Then: let url_keys: Vec<&str> = changes.updated_urls.iter().map(|(u,_)| u.as_str()).collect();
  url_keys.len() == url_keys.into_iter().collect::<HashSet<_>>().len()
```

Test function: `fn build_scrape_state_changes_produces_unique_updated_url_keys()`

### Behavior 18: build_scrape_state_changes — reference integrity (INV-6)

```
### Behavior: build_scrape_state_changes ensures every non-zero url_hash has matching new_scrapes entry (INV-6)
Given: scrape_diff has 3 new pages and 2 changed pages
When: build_scrape_state_changes(&scrape_diff, &scraped_pages, timestamp) is called
Then: let scrape_keys: HashSet<[u8; 32]> = changes.new_scrapes.iter().map(|(k,_)| *k).collect();
  for (_, state) in &changes.updated_urls:
    if state.url_hash != [0u8; 32]:
      scrape_keys.contains(&state.url_hash) == true
```

Test function: `fn build_scrape_state_changes_maintains_reference_integrity_for_url_hashes()`

### Behavior 17: build_scrape_state_changes — non-zero scrape hash keys (INV-7)

```
### Behavior: build_scrape_state_changes produces non-zero hash keys in new_scrapes (INV-7)
Given: scrape_diff has at least one new or changed page with non-empty markdown
When: build_scrape_state_changes(&scrape_diff, &scraped_pages, timestamp) is called
Then: for (hash, _) in &changes.new_scrapes:
  *hash != [0u8; 32]
```

Test function: `fn build_scrape_state_changes_produces_non_zero_scrape_hash_keys()`

### Behavior 15: build_scrape_state_changes — correct content_hash per page [FIXED: ≥2 pages — fixes MAJOR-6]

```
### Behavior: build_scrape_state_changes sets content_hash from SHA-256 of each page's own markdown
Given: scrape_diff has new pages "https://a.com/p1" and "https://a.com/p2"
  And scraped_pages contains [
        ScrapedPage{url: "https://a.com/p1", markdown: "alpha content here"},
        ScrapedPage{url: "https://a.com/p2", markdown: "beta content here"},
      ]
When: build_scrape_state_changes(&scrape_diff, &scraped_pages, timestamp) is called
Then: changes.updated_urls["https://a.com/p1"].content_hash == SHA-256(b"alpha content here")
  And: changes.updated_urls["https://a.com/p2"].content_hash == SHA-256(b"beta content here")
  And: changes.updated_urls["https://a.com/p1"].content_hash != changes.updated_urls["https://a.com/p2"].content_hash
```

Test function: `fn build_scrape_state_changes_sets_content_hash_from_sha256_of_each_pages_markdown()`

### Behavior 21: build_scrape_state_changes — persisted bytes match new_scrapes

```
### Behavior: build_scrape_state_changes serializes PersistedScrapeResult for new_scrapes
Given: scrape_diff has 1 new page
When: build_scrape_state_changes(&scrape_diff, &scraped_pages, timestamp) is called
Then: changes.new_scrapes.len() == 1
  And: the value bytes deserialize to a valid PersistedScrapeResult via rkyv
  And: the deserialized PersistedScrapeResult.schema_version == 1
```

Test function: `fn build_scrape_state_changes_serializes_persisted_scrape_result_for_scrapes()`

### Behavior 22: build_scrape_state_changes — new pages only [NEW — fixes MAJOR-3]

```
### Behavior: build_scrape_state_changes handles new-pages-only ScrapeDiff correctly
Given: scrape_diff has new=["https://a.com/new1", "https://a.com/new2"], changed=[], unchanged=[]
  And scraped_pages contains both pages with their markdown
  And timestamp = 1_600_000_000
When: build_scrape_state_changes(&scrape_diff, &scraped_pages, timestamp) is called
Then: changes.updated_urls.len() == 2
  And: changes.updated_urls contains entries for both "https://a.com/new1" and "https://a.com/new2"
  And: changes.new_scrapes.len() == 2
  And: every UrlStateRaw.last_fetched_secs == 1_600_000_000
  And: every UrlStateRaw.status_code == 200
```

Test function: `fn build_scrape_state_changes_handles_new_pages_only()`

### Behavior 23: build_scrape_state_changes — changed pages only [NEW — fixes MAJOR-3]

```
### Behavior: build_scrape_state_changes handles changed-pages-only ScrapeDiff correctly
Given: scrape_diff has new=[], changed=["https://a.com/ch1", "https://a.com/ch2"], unchanged=[]
  And scraped_pages contains both pages with their markdown
  And timestamp = 1_800_000_000
When: build_scrape_state_changes(&scrape_diff, &scraped_pages, timestamp) is called
Then: changes.updated_urls.len() == 2
  And: changes.updated_urls contains entries for both "https://a.com/ch1" and "https://a.com/ch2"
  And: changes.new_scrapes.len() == 2
  And: every UrlStateRaw.last_fetched_secs == 1_800_000_000
```

Test function: `fn build_scrape_state_changes_handles_changed_pages_only()`

### Behavior 24: build_scrape_state_changes — zero timestamp [NEW — fixes MAJOR-3]

```
### Behavior: build_scrape_state_changes handles timestamp = 0 correctly
Given: scrape_diff has new=["https://a.com/p"], changed=[], unchanged=[]
  And scraped_pages contains the page
  And timestamp = 0
When: build_scrape_state_changes(&scrape_diff, &scraped_pages, 0) is called
Then: changes.updated_urls["https://a.com/p"].last_fetched_secs == 0
  And: changes.updated_urls["https://a.com/p"].content_hash == SHA-256(page.markdown.as_bytes())
  And: changes.new_scrapes.len() == 1
```

Test function: `fn build_scrape_state_changes_handles_zero_timestamp()`

### Behavior 25: build_scrape_state_changes — max timestamp [NEW — fixes MAJOR-3]

```
### Behavior: build_scrape_state_changes handles timestamp = u64::MAX without overflow
Given: scrape_diff has new=["https://a.com/p"], changed=[], unchanged=[]
  And scraped_pages contains the page
When: build_scrape_state_changes(&scrape_diff, &scraped_pages, u64::MAX) is called
Then: changes.updated_urls["https://a.com/p"].last_fetched_secs == u64::MAX
  And: changes.updated_urls["https://a.com/p"].content_hash == SHA-256(page.markdown.as_bytes())
  And: changes.new_scrapes.len() == 1
```

Test function: `fn build_scrape_state_changes_handles_max_timestamp()`

### Behavior 26: build_scrape_state_changes — empty markdown [NEW — fixes MINOR-3]

```
### Behavior: build_scrape_state_changes handles page with empty markdown
Given: scrape_diff has new=["https://a.com/empty"], changed=[], unchanged=[]
  And scraped_pages contains ScrapedPage{url: "https://a.com/empty", markdown: ""}
  And timestamp = 1_000_000_000
When: build_scrape_state_changes(&scrape_diff, &scraped_pages, timestamp) is called
Then: changes.updated_urls["https://a.com/empty"].content_hash == SHA-256(b"")  // == e3b0c44298fc1c14...
  And: changes.new_scrapes.len() == 1
  And: the new_scrapes value deserializes to a valid PersistedScrapeResult
```

Test function: `fn build_scrape_state_changes_handles_empty_markdown()`

### Behavior 30: run_scrape — creates and uses one shared read session (INV-1, POST-1)

```
### Behavior: run_scrape creates exactly one StateReadSession for all reads
Given: output directory is valid and empty
  And config has valid URL, query=None, filter=None
  And the scrape will succeed (using test server or mock transport)
When: run_scrape is called
Then: StateDb::open is called exactly once
  And: StateReadSession::new is called exactly once
  And: session.load_url_states() is called exactly once
  And: commit_changes is called exactly once at shutdown
```

Test function: `fn run_scrape_creates_one_shared_read_session_and_one_commit()` — integration

### Behavior 34: run_scrape — read session dropped before commit (INV-3)

```
### Behavior: run_scrape drops StateReadSession before commit_changes (INV-3)
Given: a successful scrape run
When: run_scrape reaches the commit phase
Then: StateReadSession is dropped before commit_changes is called
  (Verified by: commit_changes would fail if read txn is still held)
```

Test function: `fn run_scrape_drops_read_session_before_commit()` — integration

### Behavior 29: run_scrape — first run creates state.redb with correct data

```
### Behavior: run_scrape creates state.redb on first run and commits all scraped pages
Given: output directory is empty (no prior state.redb)
  And scrape succeeds with 3 pages
When: run_scrape completes successfully
Then: output.join("state.redb") exists
  And: StateDb can be reopened at that path
  And: url_state table contains exactly 3 entries
  And: scrape_outputs table contains exactly 3 entries
```

Test function: `fn run_scrape_creates_state_db_on_first_run_with_all_pages()` — integration

### Behavior 37: run_scrape — second run reuses unchanged pages [FIXED: exact composition — fixes MINOR-5]

```
### Behavior: run_scrape reuses unchanged pages from persisted scrape_outputs on second run
Given: state.redb exists from prior run with 3 URL states and 3 scrape outputs
  And scraped markdown is identical to prior run for pages "p1" and "p2"
  And scraped markdown differs for page "p3"
When: run_scrape is called again with same URL
Then: pages "p1" and "p2" are loaded from scrape_outputs table (not re-fetched from network)
  And: page "p3" is freshly scraped from network
  And: the final ScrapeResult contains exactly 3 pages (2 reused + 1 fresh)
  And: commit contains exactly 1 entry in updated_urls (only "p3")
  And: commit contains exactly 1 entry in new_scrapes (only for "p3")
```

Test function: `fn run_scrape_reuses_unchanged_pages_from_persisted_scrape_outputs()` — integration

### Behavior 44: run_scrape — commit failure propagates (POST-7)

```
### Behavior: run_scrape returns Err when commit_changes fails (POST-7)
Given: scrape succeeds
  But: state.redb is corrupted or locked so commit_changes fails
When: run_scrape reaches the commit phase
Then: run_scrape returns Err
  And: the Err message contains "failed to commit scrape state"
```

Test function: `fn run_scrape_propagates_commit_failure_as_error()` — integration

### Behavior 45: run_scrape — pre-commit failure leaves state intact (INV-2, POST-6)

```
### Behavior: run_scrape leaves state.redb unchanged when scrape fails before commit (INV-2)
Given: state.redb exists with prior data
  And: the scrape itself fails (network error, validation failure)
When: run_scrape returns Err before reaching commit_changes
Then: state.redb can be reopened
  And: url_state table contains the same entries as before the run
  And: scrape_outputs table contains the same entries as before the run
```

Test function: `fn run_scrape_leaves_state_intact_when_scrape_fails_before_commit()` — integration

### Behavior 40: run_scrape — error wrapping for StateDb::open failure

```
### Behavior: run_scrape wraps StateDb::open failure with descriptive context
Given: output directory path points to an unwritable location (e.g., /proc/impossible)
When: run_scrape attempts to open state database
Then: run_scrape returns Err
  And: error chain contains "failed to open state database"
```

Test function: `fn run_scrape_wraps_state_db_open_failure_with_context()` — integration

### Behavior 41: run_scrape — error wrapping for load_url_states failure

```
### Behavior: run_scrape wraps load_url_states failure with descriptive context
Given: state.redb exists but has a malformed url_state row (wrong byte count)
When: run_scrape calls session.load_url_states()
Then: run_scrape returns Err
  And: error chain contains "failed to load URL states"
```

Test function: `fn run_scrape_wraps_load_url_states_failure_with_context()` — integration

### Behavior 42: run_scrape — load_scrapes failure propagates [NEW — fixes LETHAL-2, MAJOR-5]

```
### Behavior: run_scrape propagates load_scrapes failure (BulkLoadError from contract.md:71)
Given: state.redb exists with valid URL states
  And: state.redb has corrupted or missing scrape_outputs table entries
  And: some pages are classified as unchanged (content_hash matches stored)
When: run_scrape calls session.load_scrapes() for unchanged pages
Then: run_scrape returns Err
  And: the error propagates as anyhow::Error wrapping the underlying BulkLoadError
  And: the error is NOT silently swallowed (run_scrape does not proceed without scrape data)
```

Test function: `fn run_scrape_propagates_load_scrapes_failure()` — integration

### Behavior 43: run_scrape — StateReadSession::new failure propagates [NEW — fixes LETHAL-1, MAJOR-4]

```
### Behavior: run_scrape propagates StateReadSession::new failure (BulkLoadError::StorageError from contract.md:69)
Given: output directory is valid and writable
  And: StateDb::open succeeds
  But: StateReadSession::new(state_db.database()) fails (e.g., concurrent write lock, backend error)
When: run_scrape attempts to create the shared read session
Then: run_scrape returns Err
  And: the error propagates as anyhow::Error wrapping the underlying BulkLoadError::StorageError
  And: the error is NOT silently swallowed (run_scrape does not proceed without a read session)
  And: no commit_changes call is made
```

Test function: `fn run_scrape_propagates_read_session_creation_failure()` — integration

### Behavior 39: run_scrape — no per-page writes (INV-1)

```
### Behavior: run_scrape performs zero per-page writes to state.redb (INV-1)
Given: a successful scrape run with 5 pages
When: run_scrape completes
Then: commit_changes was called exactly once
  And: the single StateChanges batch contains all 5 URL state entries
  And: no intermediate write transactions were opened
```

Test function: `fn run_scrape_performs_zero_per_page_writes_to_state_db()` — integration

### Behavior 46: E2E — first run succeeds

```
### Behavior: ctd scrape succeeds end-to-end on first run
Given: a fresh temporary output directory
  And a local HTTP test server serving 2 pages
When: `ctd scrape http://localhost:<port> <output_dir>` is invoked
Then: exit code == 0
  And: state.redb exists at <output_dir>/state.redb
  And: url_state table has 2 entries
  And: scrape_outputs table has 2 entries
  And: .scrape/*.md files exist on disk
```

Test function: `fn e2e_scrape_first_run_creates_state_and_output_files()` — e2e

### Behavior 47: E2E — second run reuses unchanged pages [FIXED: concrete verification — fixes MAJOR-1]

```
### Behavior: ctd scrape on second run correctly reuses unchanged pages
Given: output directory with state.redb from a prior successful scrape of 2 pages
  And the test server serves identical content for both pages
  And the test server tracks HTTP request count
When: `ctd scrape http://localhost:<port> <output_dir>` is invoked again
Then: exit code == 0
  And: the HTTP test server received exactly 0 additional requests (no pages re-fetched)
  And: the scrape_outputs table still has exactly 2 entries (unchanged)
  And: url_state table still has exactly 2 entries (no new updates committed)
```

Test function: `fn e2e_scrape_second_run_reuses_unchanged_pages()` — e2e

### Behavior 48: E2E — graceful failure on unwritable output

```
### Behavior: ctd scrape fails gracefully when output path is unwritable
Given: output path points to a non-existent root (e.g., /proc/fake/state.redb)
When: `ctd scrape http://localhost:<port> <output>` is invoked
Then: exit code != 0
  And: stderr contains "failed to open state database" or similar
```

Test function: `fn e2e_scrape_fails_gracefully_on_unwritable_path()` — e2e

## 4. Proptest Invariants

### Proptest 1: classify_scrape_diff — partition correctness

```
### Proptest: classify_scrape_diff partitions are mutually exclusive and collectively exhaustive
Invariant: For any (stored_url_states, scraped_pages):
  - Every scraped page URL appears in exactly one of {new, changed, unchanged}
  - No URL appears in more than one partition
  - Union of all partition URL sets == set of all scraped page URLs
Strategy:
  - stored_url_states: HashMap<String, UrlStateRaw> with 0..20 entries
    keys: "https://example.com/page-{n}"
    values: UrlStateRaw with random content_hash
  - scraped_pages: Vec<ScrapedPage> with 0..20 entries
    URLs: mix of URLs present and absent from stored_url_states
    markdown: random ASCII strings 1..500 bytes
Anti-invariant: A page URL appearing in two partitions simultaneously should never happen
```

### Proptest 2: classify_scrape_diff — unchanged iff content_hash matches

```
### Proptest: unchanged classification is equivalent to content_hash equality
Invariant: For any scraped page with URL U:
  U is in result.unchanged iff stored_url_states[U].content_hash == SHA-256(page.markdown.as_bytes())
Strategy:
  - Generate random markdown, compute its hash, store as UrlStateRaw
  - For "match" cases: use the same markdown
  - For "mismatch" cases: use different markdown
  - Verify classification matches expected
```

### Proptest 3: build_scrape_state_changes — determinism

```
### Proptest: build_scrape_state_changes is deterministic for identical inputs
Invariant: build_scrape_state_changes(diff, pages, ts) == build_scrape_state_changes(diff.clone(), pages.clone(), ts)
Strategy:
  - ScrapeDiff with 0..10 entries per bucket
  - ScrapedPages with random markdown 1..1000 bytes
  - timestamp: any u64
```

### Proptest 4: build_scrape_state_changes — url_hash == SHA-256 of persisted_bytes

```
### Proptest: every new_scrapes key equals SHA-256 of its value bytes
Invariant: For all (hash, bytes) in result.new_scrapes: hash == SHA-256(bytes)
Strategy:
  - ScrapeDiff with 1..10 new/changed pages
  - Random markdown content
```

### Proptest 5: build_scrape_state_changes — output field counts match input

```
### Proptest: output vec lengths equal number of new + changed pages
Invariant:
  - result.updated_urls.len() == scrape_diff.new.len() + scrape_diff.changed.len()
  - result.new_scrapes.len() == scrape_diff.new.len() + scrape_diff.changed.len()
  - result.deleted_urls.len() == 0 (scrape never deletes)
  - result.updated_files.len() == 0 (scrape never touches file state)
Strategy:
  - ScrapeDiff with arbitrary new/changed/unchanged counts
```

### Proptest 6: hash_payload — SHA-256 determinism

```
### Proptest: hash_payload is deterministic and collision-resistant for small inputs
Invariant: hash_payload(bytes) == hash_payload(bytes) for all byte slices
  And: hash_payload(a) != hash_payload(b) when a != b (for inputs < 32 bytes)
Strategy: proptest::collection::vec(any::<u8>(), 0..256)
Anti-invariant: hash_payload([0u8; N]) should never equal hash_payload([1u8; N]) for N > 0
```

## 5. Fuzz Targets

### Fuzz Target 1: classify_scrape_diff with arbitrary URL strings

```
### Fuzz Target: classify_scrape_diff
Input type: (Vec<(String, [u8; 120])>, Vec<(String, Vec<u8>)>)
  - First element: raw (url_string, url_state_raw_bytes) pairs — stored states
  - Second element: raw (url_string, markdown_bytes) pairs — scraped pages
Risk: Panic on empty strings, non-UTF-8, zero-length markdown, extremely long URLs
  Incorrect classification when content_hash computation overflows or panics
Corpus seeds:
  - Empty stored states, single page
  - Matching content hash (unchanged)
  - Mismatching content hash (changed)
  - URL not in stored (new)
  - Empty markdown (edge case for SHA-256)
  - Very long URL (4096 chars)
  - URL with special characters (spaces, unicode, control chars)
  - content_hash == [0u8; 32] with non-matching page (zero-hash boundary)
```

### Fuzz Target 2: build_scrape_state_changes with arbitrary ScrapeDiff + pages

```
### Fuzz Target: build_scrape_state_changes
Input type: (Vec<(String, Vec<u8>)>, u64)
  - Vec of (url, markdown_bytes) for new/changed pages
  - u64 timestamp
Risk: Panic on empty markdown, OOM on very large markdown, rkyv serialization failure
Corpus seeds:
  - Single page with 1-byte markdown
  - Single page with 100KB markdown
  - 100 pages with random content
  - Empty page list (should return empty StateChanges)
  - Page with all-zero markdown
  - Page with empty markdown (SHA-256 of b"" edge case)
  - timestamp = 0
  - timestamp = u64::MAX
```

### Fuzz Target 3: UrlStateRaw round-trip through bytes

```
### Fuzz Target: UrlStateRaw::from_bytes / to_bytes round-trip
Input type: [u8; 120]
Risk: from_bytes panics on malformed input, to_bytes produces wrong output
Corpus seeds:
  - All zeros
  - All 0xFF
  - Valid state with realistic hash values
  - Single non-zero byte at each offset
  - Max values for u64/u16 fields
```

## 6. Kani Harnesses

### Kani Harness 1: UrlStateRaw byte layout correctness

```
### Kani Harness: url_state_raw_byte_layout_is_correct
Property: For any [u8; 120] input, from_bytes returns Ok iff the input is exactly 120 bytes,
  and to_bytes followed by from_bytes returns the original struct.
Bound: Exhaustive for all 120-byte inputs (infeasible — use representative field combinations)
Rationale: UrlStateRaw is a #[repr(C)] Pod struct shared with redb. Layout correctness is
  a data integrity invariant. A single byte offset error corrupts all state reads.
  Verification via kani::proof for specific field boundary values (offset 0, 32, 64, 72, 74).
```

### Kani Harness 2: classify_scrape_diff — no page URL is lost

```
### Kani Harness: classify_scrape_diff_preserves_all_scraped_urls
Property: For any stored_url_states and scraped_pages,
  let result = classify_scrape_diff(&stored, &pages);
  result.new.union(result.changed).union(result.unchanged) == set_of_all_scraped_urls
Bound: stored_url_states 0..3 entries, scraped_pages 0..3 entries
Rationale: INV-1 requires all pages to be accounted for. A lost page is a silent data
  corruption bug. Kani can exhaustively prove this for small input sizes.
```

### Kani Harness 3: build_scrape_state_changes — updated_urls count matches new+changed

```
### Kani Harness: build_scrape_state_changes_output_count_matches_input
Property: result.updated_urls.len() == scrape_diff.new.len() + scrape_diff.changed.len()
Bound: ScrapeDiff with 0..3 entries per bucket
Rationale: Every new or changed page must produce exactly one UrlStateRaw entry.
  A count mismatch indicates a page was silently dropped or duplicated.
```

## 7. Mutation Testing Checkpoints

| Mutation | Caught By | Scenario |
|----------|-----------|----------|
| Change `==` to `!=` in content_hash comparison | `classify_scrape_diff_classifies_all_as_unchanged_when_hashes_match` | Behavior 3 |
| Remove content_hash check (always New) | `classify_scrape_diff_classifies_all_as_unchanged_when_hashes_match` | Behavior 3 |
| Remove content_hash check (always Changed) | `classify_scrape_diff_classifies_all_as_changed_when_all_hashes_differ` | Behavior 8 |
| Return wrong URLs with correct count | `classify_scrape_diff_classifies_all_as_new_when_stored_states_empty` | Behavior 2 (asserts specific URLs) |
| Treat [0u8; 32] as sentinel "no hash" | `classify_scrape_diff_handles_zero_content_hash_boundary` | Behavior 9 |
| Remove timestamp assignment | `build_scrape_state_changes_produces_entries_for_new_and_changed_pages` | Behavior 12 |
| Skip unchanged exclusion | `build_scrape_state_changes_excludes_unchanged_pages_from_all_outputs` | Behavior 14 |
| Zero out url_hash | `build_scrape_state_changes_produces_non_zero_scrape_hash_keys` | Behavior 17 |
| Remove reference integrity check | `build_scrape_state_changes_maintains_reference_integrity_for_url_hashes` | Behavior 18 |
| Assign wrong page's content_hash (cross-page swap) | `build_scrape_state_changes_sets_content_hash_from_sha256_of_each_pages_markdown` | Behavior 15 (≥2 pages) |
| Drop commit_changes call | `run_scrape_creates_one_shared_read_session_and_one_commit` | Behavior 30 |
| Remove read session drop before commit | `run_scrape_drops_read_session_before_commit` | Behavior 34 |
| Swallow commit error | `run_scrape_propagates_commit_failure_as_error` | Behavior 44 |
| Remove "failed to open state database" context | `run_scrape_wraps_state_db_open_failure_with_context` | Behavior 40 |
| Write per-page instead of batching | `run_scrape_performs_zero_per_page_writes_to_state_db` | Behavior 39 |
| Allow duplicate URL keys | `build_scrape_state_changes_produces_unique_updated_url_keys` | Behavior 16 |
| Return empty ScrapeDiff instead of New for empty stored | `classify_scrape_diff_classifies_all_as_new_when_stored_states_empty` | Behavior 2 |
| **Swallow StateReadSession::new error** (proceed with empty state) | `run_scrape_propagates_read_session_creation_failure` | Behavior 43 **[NEW]** |
| **Swallow load_scrapes error** (proceed without persisted data) | `run_scrape_propagates_load_scrapes_failure` | Behavior 42 **[NEW]** |
| **build_scrape_state_changes returns empty for new-only** | `build_scrape_state_changes_handles_new_pages_only` | Behavior 22 **[NEW]** |
| **build_scrape_state_changes returns empty for changed-only** | `build_scrape_state_changes_handles_changed_pages_only` | Behavior 23 **[NEW]** |
| **Timestamp always uses system clock instead of parameter** | `build_scrape_state_changes_handles_zero_timestamp` | Behavior 24 **[NEW]** |
| **Return ScrapeResult with wrong page count** (missing reused pages) | `run_scrape_reuses_unchanged_pages_from_persisted_scrape_outputs` | Behavior 37 (asserts exact count) **[FIXED]** |
| **Re-fetch unchanged pages from network** (skip reuse) | `e2e_scrape_second_run_reuses_unchanged_pages` | Behavior 47 (HTTP request count == 0) **[FIXED]** |

**Mutation threshold: ≥90% kill rate.** The 24 planned checkpoints cover all critical branches including error propagation paths (StateReadSession::new, load_scrapes), cross-page hash correctness (≥2 pages), boundary values (zero timestamp, max timestamp, empty markdown), and exact output composition. Any mutation that changes a comparison operator, removes a validation, swallows an error, or produces wrong page identities will be caught.

## 8. Combinatorial Coverage Matrix

### classify_scrape_diff

| Scenario | Input: stored_url_states | Input: scraped_pages | Expected Output | BDD Scenario | Layer |
|----------|--------------------------|----------------------|-----------------|-------------|-------|
| happy: mixed | 2 entries | 3 pages (1 match, 1 differ, 1 new) | new=1, changed=1, unchanged=1 | Behavior 1 | unit |
| first run: empty stored | empty | 3 pages | new=3, changed=0, unchanged=0 | Behavior 2 | unit |
| no change | 3 entries (all match) | 3 pages (same content) | new=0, changed=0, unchanged=3 | Behavior 3 | unit |
| **all changed** | 3 entries | 3 pages (all different) | new=0, changed=3, unchanged=0 | **Behavior 8** | unit |
| empty pages | 3 entries | empty | new=0, changed=0, unchanged=0 | Behavior 4 | unit |
| both empty | empty | empty | new=0, changed=0, unchanged=0 | Behavior 5 | unit |
| **partial URL overlap** | 5 entries | 3 pages (2 overlap, 1 new) | new=1, changed/unchanged per hash | **Behavior 10** | unit |
| **content_hash boundary: zero hash** | entry with [0;32] | page with non-matching content | changed=1 | **Behavior 9** | unit |
| **content_hash boundary: non-zero** | entry with [1;32] | page with matching content | unchanged=1 | **Behavior 11** | unit |

### build_scrape_state_changes

| Scenario | Input: ScrapeDiff | Expected updated_urls | Expected new_scrapes | BDD Scenario | Layer |
|----------|-------------------|-----------------------|----------------------|-------------|-------|
| mixed | new=1, changed=1, unchanged=2 | 2 entries | 2 entries | Behavior 12 | unit |
| unchanged only | new=0, changed=0, unchanged=3 | 0 entries | 0 entries | Behavior 14 | unit |
| **new pages only** | new=2, changed=0 | 2 entries | 2 entries | **Behavior 22** | unit |
| **changed pages only** | new=0, changed=2 | 2 entries | 2 entries | **Behavior 23** | unit |
| empty | all empty | 0 entries | 0 entries | Behavior 14 | unit |
| **timestamp = 0** | new=1 | last_fetched_secs == 0 | 1 entry | **Behavior 24** | unit |
| **timestamp = u64::MAX** | new=1 | last_fetched_secs == MAX | 1 entry | **Behavior 25** | unit |
| **empty markdown** | new=1, markdown="" | content_hash == SHA-256(b"") | 1 entry | **Behavior 26** | unit |
| content_hash correctness (≥2 pages) | new=2 | each hash == SHA-256(own md) | N/A | Behavior 15 | unit |
| unique keys | new=3, changed=2 | 5 unique keys | N/A | Behavior 16 | unit |
| reference integrity | new=3, changed=2 | all url_hashes in new_scrapes | N/A | Behavior 18 | unit |
| non-zero hash keys | new=1 | N/A | all keys != [0;32] | Behavior 17 | unit |
| persisted bytes | new=1 | N/A | valid rkyv PersistedScrapeResult | Behavior 21 | unit |

### run_scrape integration

| Scenario | State | Condition | Expected | BDD Scenario | Layer |
|----------|-------|-----------|----------|-------------|-------|
| first run success | no state.redb | 3 pages scraped | state.redb created, 3 url_state, 3 scrape_outputs | Behavior 29 | integration |
| second run unchanged | prior state | 3 pages identical | reused pages, 0 updated_urls, exact 3-page ScrapeResult | Behavior 37 | integration |
| second run mixed | prior state | 2 unchanged, 1 changed | 2 reused + 1 fresh, 1 updated_url, 1 new_scrape | Behavior 37 | integration |
| **StateReadSession::new failure** | valid path | read session creation fails | Err propagates BulkLoadError::StorageError | **Behavior 43** | integration |
| **load_scrapes failure** | valid state | scrape_outputs corrupted | Err propagates BulkLoadError | **Behavior 42** | integration |
| state db open failure | unwritable path | StateDb::open fails | Err with "failed to open state database" | Behavior 40 | integration |
| load_url_states failure | malformed row | load_url_states fails | Err with "failed to load URL states" | Behavior 41 | integration |
| commit failure | valid state | commit_changes fails | Err with "failed to commit scrape state" | Behavior 44 | integration |
| scrape failure before commit | valid state | network error | Err, state unchanged | Behavior 45 | integration |
| no per-page writes | valid state | 5 pages | commit_changes called exactly once | Behavior 39 | integration |
| read session drop order | valid state | successful run | session dropped before commit | Behavior 34 | integration |

### E2E

| Scenario | Setup | Expected | BDD Scenario | Layer |
|----------|-------|----------|-------------|-------|
| first run | fresh dir, 2-page server | exit 0, state.redb, 2 entries | Behavior 46 | e2e |
| **second run reuse** | prior state, identical content, **request-counting server** | exit 0, **HTTP requests == 0** | **Behavior 47** | e2e |
| unwritable path | /proc/fake | exit != 0, stderr contains error | Behavior 48 | e2e |

## Contract Alignment Note

### Error context string gap (contract.md:75-78)

The contract specifies context strings for three error paths:
- `"failed to open state database: {e}"` — StateDb::open
- `"failed to load URL states: {e}"` — load_url_states
- `"failed to commit scrape state: {e}"` — commit_changes

**Two error paths lack specified context strings:**
1. `StateReadSession::new` failure (contract.md:69) — no context string specified
2. `load_scrapes()` failure (contract.md:71) — no context string specified

**Recommendation:** The implementation SHOULD wrap these with consistent context strings:
- `"failed to create read session: {e}"` — for StateReadSession::new
- `"failed to load scrape outputs: {e}"` — for load_scrapes

**Test plan approach:** Behaviors 42 and 43 test that errors **propagate** (not silently swallowed). If the implementation adds context strings, the tests can be tightened to assert those specific strings. The critical invariant is propagation, not the exact wrapping text. The test-writer should update assertions once the implementation settles on context strings.

### load_scrapes hash input (Open question 4 — RESOLVED)

**Resolved:** Per the contract wiring flow (contract.md:111) and the `load_scrapes` signature (contract.md:90), the hash input for `session.load_scrapes(hashes)` is `stored_url_states[url].url_hash` for each unchanged page. The `url_hash` field in `UrlStateRaw` is the key used in the `scrape_outputs` table, stored during the prior commit via `build_scrape_state_changes`. This is the only consistent source — it links the current run's unchanged page classification back to the prior run's persisted scrape output.

## Open Questions

1. **Test server for integration tests**: The `run_scrape` integration tests (Behaviors 30-39) require a working HTTP server. Options:
   - Use `wiremock` or `mockito` crate for in-process HTTP mocking
   - Use `httptest` crate for lightweight test server
   - Extract the scrape-site call behind a trait and inject a fake
   Recommendation: `httptest` (real HTTP, minimal setup, no trait extraction needed)

2. **calc/scrape_diff.rs module location**: The contract references this module but it does not yet exist. The test plan assumes it will be created at `src/calc/scrape_diff.rs` with `pub mod scrape_diff;` in `src/calc/mod.rs`. Confirm this path.

3. **ScrapeDiff type definition**: The contract mentions classification (unchanged/changed/new) but does not specify the exact struct. This plan assumes:
   ```rust
   pub struct ScrapeDiff {
       pub new: Vec<String>,
       pub changed: Vec<String>,
       pub unchanged: Vec<String>,
   }
   ```
   If the implementation uses HashSet or includes the ScrapedPage directly, adjust the test scenarios accordingly.
