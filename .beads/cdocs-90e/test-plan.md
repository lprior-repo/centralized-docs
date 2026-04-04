bead_id: cdocs-90e
bead_title: action: load archived scrape outputs for unchanged pages and skip downstream stages
phase: state-1.5-test-plan
updated_at: 2026-04-03T13:00:00Z

# Test Plan: Scrape Reuse — Load Archived Scrape Outputs for Unchanged Pages

## Summary
- Behaviors identified: 35 (30 original + 5 new from review mandates)
- Trophy allocation: 8 unit / 9 integration / 1 e2e / 2 static
- Proptest invariants: 5
- Fuzz targets: 2
- Kani harnesses: 2
- Mutation threshold: ≥90% kill rate

## 1. Behavior Inventory

### Classification (Pure Calc)
1. `compute_page_content_hash` returns SHA-256 of markdown content when given any string
2. `compute_page_content_hash` returns all-zeros hash when given empty string
3. `classify_scraped_pages` returns empty partitions when no pages provided
4. `classify_scraped_pages` classifies page as unchanged when content hash matches stored state
5. `classify_scraped_pages` classifies page as changed when content hash mismatches stored state
5b. `classify_scraped_pages` classifies page as changed when stored hash is lexicographically greater but not equal (**mutation trap: catches `>=` vs `==`**)
6. `classify_scraped_pages` classifies page as changed_or_new when url_state missing for URL
7. `classify_scraped_pages` classifies page as changed_or_new when stored url_hash is zero
8. `classify_scraped_pages` produces mutually exclusive and collectively exhaustive partition
8b. `classify_scraped_pages` panics or errors when fresh_pages.len() != fresh_hashes.len() (**mismatched input lengths**)

### Archive Loading (I/O Action)
9. `load_archived_scrape_pages` returns empty maps when no unchanged pages
10. `load_archived_scrape_pages` loads correct archived pages when url_hash matches
10b. `load_archived_scrape_pages` loads pages from multiple different batches when unchanged pages have different url_hashes (**multi-batch load**)
10c. `load_archived_scrape_pages` returns fallback indices when batch contains zero pages (**empty batch**)
11. `load_archived_scrape_pages` returns fallback indices when batch deserialization fails
12. `load_archived_scrape_pages` returns fallback indices when individual page deserialization fails
13. `load_archived_scrape_pages` returns HashMismatch when loaded page hash doesn't match stored hash
14. `load_archived_scrape_pages` returns fallback indices when url_hash has no matching scrape_output
15. `load_archived_scrape_pages` propagates StateLoad error when url_states load fails
16. `load_archived_scrape_pages` propagates BulkLoad error when scrape_outputs load fails

### Merge (Pure Calc)
17. `merge_scrape_pages_in_order` returns fresh_pages when archived_pages is empty
18. `merge_scrape_pages_in_order` returns vec with archived pages substituted at correct indices preserving order

### Entry Point (Integration)
19. `scrape_with_reuse` returns all pages with correct stats when all pages unchanged
19b. `scrape_with_reuse` returns empty result with zero stats when ScrapeResult has zero pages (**empty entry point**)
20. `scrape_with_reuse` returns all fresh pages when all pages changed
21. `scrape_with_reuse` returns mixed result when some pages unchanged some changed
22. `scrape_with_reuse` returns correct ScrapeReuseStats where reused + scraped == total
23. `scrape_with_reuse` propagates StateLoad error from session

### Domain Types
24. `ScrapePageDiff::default` returns empty unchanged and changed_or_new
25. `ScrapeReuseStats::default` returns reused=0, scraped=0

### Error Taxonomy
26. `ScrapeReuseError::StateLoad` variant matches StateLoadError
27. `ScrapeReuseError::BulkLoad` variant matches BulkLoadError
28. `ScrapeReuseError::DeserializationFailed` carries key_hex and message
29. `ScrapeReuseError::HashMismatch` carries url, stored_hex, loaded_hex
30. `ScrapeReuseError::MissingUrlState` carries url

## 2. Trophy Allocation

| # | Behavior | Layer | Justification |
|---|----------|-------|---------------|
| 1 | `compute_page_content_hash` hash output | **unit** | Pure function, deterministic, no I/O. Exhaustive boundary testing. |
| 2 | `compute_page_content_hash` empty input | **unit** | Pure function, boundary value. |
| 3 | `classify_scraped_pages` empty input | **unit** | Pure function, degenerate case. |
| 4 | `classify_scraped_pages` unchanged match | **unit** | Pure function, core happy path. |
| 5 | `classify_scraped_pages` hash mismatch | **unit** | Pure function, error classification path. |
| 5b | `classify_scraped_pages` lexicographic trap | **unit** | Pure function, mutation survival trap for `>=` vs `==`. |
| 6 | `classify_scraped_pages` missing url_state | **unit** | Pure function, missing key path. |
| 7 | `classify_scraped_pages` zero url_hash | **unit** | Pure function, sentinel value path. |
| 8 | `classify_scraped_pages` MCE partition | **unit** | Pure function, invariant verification. |
| 8b | `classify_scraped_pages` mismatched lengths | **unit** | Pure function, precondition violation — must panic, assert, or error. |
| 9 | `load_archived_scrape_pages` empty input | **integration** | Touches redb via `StateReadSession` — needs real DB. |
| 10 | `load_archived_scrape_pages` successful load | **integration** | Real DB read + rkyv deserialization. |
| 10b | `load_archived_scrape_pages` multi-batch | **integration** | Real DB with two different batch keys — exercises batch-keyed architecture (INV-4, assumption #4). |
| 10c | `load_archived_scrape_pages` empty batch | **integration** | Real DB with valid but empty PersistedScrapeResult — boundary case. |
| 11 | `load_archived_scrape_pages` batch deser fail | **integration** | Real DB with corrupt archive bytes. |
| 12 | `load_archived_scrape_pages` page deser fail | **integration** | Real DB with valid batch but invalid page fields. |
| 13 | `load_archived_scrape_pages` hash mismatch | **integration** | Real DB + hash integrity verification. |
| 14 | `load_archived_scrape_pages` missing scrape_output | **integration** | Real DB with url_state but no corresponding scrape_output row. |
| 15 | `load_archived_scrape_pages` StateLoad error | **integration** | DB without url_state table (missing tables). |
| 16 | `load_archived_scrape_pages` BulkLoad error | **integration** | DB without scrape_outputs table. |
| 17 | `merge_scrape_pages_in_order` empty archived | **unit** | Pure function, identity case. |
| 18 | `merge_scrape_pages_in_order` substitution + order | **unit** | Pure function, core behavior. |
| 19 | `scrape_with_reuse` all unchanged | **integration** | Full pipeline with real DB. |
| 19b | `scrape_with_reuse` empty ScrapeResult | **integration** | Full pipeline with 0 pages — degenerate but valid input. |
| 20 | `scrape_with_reuse` all changed | **integration** | Full pipeline, fresh-only path. |
| 21 | `scrape_with_reuse` mixed | **integration** | Full pipeline, partitioned path. |
| 22 | `scrape_with_reuse` stats invariant | **integration** | POST-5 verification. |
| 23 | `scrape_with_reuse` StateLoad error | **integration** | Error propagation through entry point. |
| 24 | `ScrapePageDiff::default` | **unit** | Data type default. |
| 25 | `ScrapeReuseStats::default` | **unit** | Data type default. |
| 26-30 | Error variant Display messages | **unit** | Verify error formatting strings. |

**Rationale for ratio deviation**: This bead has a higher proportion of pure calculation functions (classification, hash computation, merge) compared to the 60% integration target. The I/O boundary is concentrated in `load_archived_scrape_pages` and `scrape_with_reuse`. The pure functions warrant exhaustive unit testing. Final ratio: ~50% unit, ~43% integration, ~7% static.

## 3. BDD Scenarios

### Behavior 1: `compute_page_content_hash` — hash of non-empty markdown

**Test function:** `fn compute_page_content_hash_returns_sha256_when_given_nonempty_markdown()`

```
Given: markdown string "# Hello\n\nWorld"
When:  compute_page_content_hash(markdown) is called
Then:  result == SHA-256("# Hello\n\nWorld") (32 bytes)
And:   result != [0u8; 32]
```

### Behavior 2: `compute_page_content_hash` — empty string

**Test function:** `fn compute_page_content_hash_returns_sha256_of_empty_when_given_empty_string()`

```
Given: markdown string ""
When:  compute_page_content_hash("") is called
Then:  result == SHA-256("") (32 bytes, which is e3b0c44298fc...)
And:   result != [0u8; 32]
```

### Behavior 3: `classify_scraped_pages` — empty input

**Test function:** `fn classify_scraped_pages_returns_empty_partitions_when_no_pages_provided()`

```
Given: fresh_pages = [], fresh_hashes = [], url_states = empty HashMap
When:  classify_scraped_pages(fresh_pages, fresh_hashes, url_states) is called
Then:  result.unchanged == []
And:   result.changed_or_new == []
```

### Behavior 4: `classify_scraped_pages` — unchanged match

**Test function:** `fn classify_scraped_pages_classifies_unchanged_when_hash_matches_stored()`

```
Given: fresh_pages = [ScrapedPage { url: "https://a.com", markdown: "hello", ... }]
And:   fresh_hashes = [[HASH_OF("hello")]]
And:   url_states = { "https://a.com" => UrlStateRaw { content_hash: HASH_OF("hello"), url_hash: [1; 32], ... } }
When:  classify_scraped_pages(fresh_pages, fresh_hashes, url_states) is called
Then:  result.unchanged == [0]
And:   result.changed_or_new == []
```

### Behavior 5: `classify_scraped_pages` — hash mismatch

**Test function:** `fn classify_scraped_pages_classifies_changed_when_hash_mismatches_stored()`

```
Given: fresh_pages = [ScrapedPage { url: "https://a.com", markdown: "new content", ... }]
And:   fresh_hashes = [[HASH_OF("new content")]]
And:   url_states = { "https://a.com" => UrlStateRaw { content_hash: HASH_OF("old content"), url_hash: [1; 32], ... } }
When:  classify_scraped_pages(fresh_pages, fresh_hashes, url_states) is called
Then:  result.unchanged == []
And:   result.changed_or_new == [0]
```

### Behavior 5b: `classify_scraped_pages` — lexicographic hash trap (`>=` vs `==` mutation)

**Test function:** `fn classify_scraped_pages_classifies_changed_when_stored_hash_is_lexicographically_greater()`

```
Given: fresh_pages = [ScrapedPage { url: "https://trap.com", markdown: "aaa", ... }]
And:   fresh_hashes = [[HASH_OF("aaa")]]  (which is some hash H_fresh)
And:   url_states = { "https://trap.com" => UrlStateRaw { content_hash: [0xFF; 32], url_hash: [1; 32], ... } }
       where [0xFF; 32] is lexicographically greater than H_fresh AND not equal to H_fresh
When:  classify_scraped_pages(fresh_pages, fresh_hashes, url_states) is called
Then:  result.unchanged == []
And:   result.changed_or_new == [0]
Note:  This test DETERMINISTICALLY catches the mutation where `==` is replaced
       with `>=` for hash comparison. If the code uses `>=`, then stored_hash
       [0xFF; 32] >= H_fresh would be true, incorrectly classifying the page
       as unchanged. Only `==` (exact match) correctly classifies it as changed.
```

### Behavior 6: `classify_scraped_pages` — missing url_state

**Test function:** `fn classify_scraped_pages_classifies_changed_or_new_when_url_state_missing()`

```
Given: fresh_pages = [ScrapedPage { url: "https://new.com", ... }]
And:   fresh_hashes = [[SOME_HASH]]
And:   url_states = {} (empty — no entry for this URL)
When:  classify_scraped_pages(fresh_pages, fresh_hashes, url_states) is called
Then:  result.unchanged == []
And:   result.changed_or_new == [0]
```

### Behavior 7: `classify_scraped_pages` — zero url_hash

**Test function:** `fn classify_scraped_pages_classifies_changed_or_new_when_url_hash_is_zero()`

```
Given: fresh_pages = [ScrapedPage { url: "https://a.com", markdown: "hello", ... }]
And:   fresh_hashes = [[HASH_OF("hello")]]
And:   url_states = { "https://a.com" => UrlStateRaw { content_hash: HASH_OF("hello"), url_hash: [0u8; 32], ... } }
When:  classify_scraped_pages(fresh_pages, fresh_hashes, url_states) is called
Then:  result.unchanged == []
And:   result.changed_or_new == [0]
And:   INV-6 verified: zero hash means "never archived"
```

### Behavior 8: `classify_scraped_pages` — MCE partition

**Test function:** `fn classify_scraped_pages_produces_mutually_exclusive_collectively_exhaustive_partition()`

```
Given: fresh_pages has N pages
And:   fresh_hashes has N hashes (one per page)
And:   url_states has entries for some subset of URLs
When:  classify_scraped_pages(fresh_pages, fresh_hashes, url_states) is called
Then:  result.unchanged.len() + result.changed_or_new.len() == N
And:   unchanged ∩ changed_or_new == ∅ (no index appears in both)
And:   every index 0..N-1 appears in exactly one partition
```

### Behavior 8b: `classify_scraped_pages` — mismatched input lengths

**Test function:** `fn classify_scraped_pages_panics_or_errors_when_input_lengths_mismatch()`

```
Given: fresh_pages = [page_A, page_B]  (length 2)
And:   fresh_hashes = [[HASH_A]]  (length 1 — one fewer than pages)
And:   url_states = empty HashMap
When:  classify_scraped_pages(fresh_pages, fresh_hashes, url_states) is called
Then:  the function panics with a message containing "length" OR
       returns a result where the implementation documents the truncation behavior
Note:  The contract signature does not return Result, so panic is the expected
       behavior for violated preconditions (consistent with INV-1 / deny(panic)
       being for production paths — this IS a programmer error path).
       The test must verify that the function does NOT silently truncate
       or produce a corrupt partition with incorrect index mapping.
```

### Behavior 9: `load_archived_scrape_pages` — empty unchanged

**Test function:** `fn load_archived_scrape_pages_returns_empty_when_no_unchanged_pages()`

```
Given: page_diff = ScrapePageDiff { unchanged: [], changed_or_new: [...] }
When:  load_archived_scrape_pages(page_diff, fresh_pages, url_states, session) is called
Then:  result == (empty HashMap, empty Vec)
```

### Behavior 10: `load_archived_scrape_pages` — successful load

**Test function:** `fn load_archived_scrape_pages_loads_correct_pages_when_url_hash_matches()`

```
Given: redb with scrape_outputs table containing valid PersistedScrapeResult at key [1; 32]
And:   url_states = { "https://a.com" => UrlStateRaw { content_hash: [H; 32], url_hash: [1; 32], ... } }
And:   page_diff = ScrapePageDiff { unchanged: [0], changed_or_new: [] }
And:   fresh_pages = [ScrapedPage { url: "https://a.com", ... }]
When:  load_archived_scrape_pages(page_diff, fresh_pages, url_states, session) is called
Then:  result.0 == { 0 => ScrapedPage { url: "https://a.com", ... } } (archived version)
And:   result.1 == [] (no fallback indices)
And:   POST-6: loaded page's url matches fresh page's url
```

### Behavior 10b: `load_archived_scrape_pages` — multi-batch load (different url_hashes)

**Test function:** `fn load_archived_scrape_pages_loads_pages_from_different_batches_when_url_hashes_differ()`

```
Given: redb with scrape_outputs table containing:
       - valid PersistedScrapeResult (batch_A) at key [0xAA; 32] with page for "https://a.com"
       - valid PersistedScrapeResult (batch_B) at key [0xBB; 32] with page for "https://b.com"
And:   url_states = {
         "https://a.com" => UrlStateRaw { content_hash: [H_A; 32], url_hash: [0xAA; 32], ... },
         "https://b.com" => UrlStateRaw { content_hash: [H_B; 32], url_hash: [0xBB; 32], ... }
       }
And:   page_diff = ScrapePageDiff { unchanged: [0, 1], changed_or_new: [] }
And:   fresh_pages = [ScrapedPage { url: "https://a.com" }, ScrapedPage { url: "https://b.com" }]
When:  load_archived_scrape_pages(page_diff, fresh_pages, url_states, session) is called
Then:  result.0.len() == 2
And:   result.0[0].url == "https://a.com" (loaded from batch_A at key [0xAA; 32])
And:   result.0[1].url == "https://b.com" (loaded from batch_B at key [0xBB; 32])
And:   result.1 == [] (no fallback indices)
Note:  This exercises INV-4 and assumption #4 — batch-keyed architecture where
       different pages may belong to different scrape batches.
```

### Behavior 10c: `load_archived_scrape_pages` — empty batch (0 pages in PersistedScrapeResult)

**Test function:** `fn load_archived_scrape_pages_returns_fallback_when_batch_contains_zero_pages()`

```
Given: redb with scrape_outputs table containing a valid PersistedScrapeResult at key [1; 32]
       where the PersistedScrapeResult has pages = [] (empty vec, 0 pages)
And:   url_states = { "https://a.com" => UrlStateRaw { content_hash: [H; 32], url_hash: [1; 32], ... } }
And:   page_diff = ScrapePageDiff { unchanged: [0], changed_or_new: [] }
And:   fresh_pages = [ScrapedPage { url: "https://a.com", ... }]
When:  load_archived_scrape_pages(page_diff, fresh_pages, url_states, session) is called
Then:  result.0 == {} (no successfully loaded pages — batch was empty)
And:   result.1 == [0] (fallback to fresh for the page whose batch was empty)
Note:  A valid but empty batch means the page was never actually archived.
       The page must fall back to fresh processing.
```

### Behavior 11: `load_archived_scrape_pages` — batch deserialization failure

**Test function:** `fn load_archived_scrape_pages_returns_fallback_when_batch_deserialization_fails()`

```
Given: redb with scrape_outputs table containing corrupt bytes [0xFF; 128] at key [0xDE, 0xAD, 0xBE, 0xEF; 8]
And:   url_states = { "https://a.com" => UrlStateRaw { content_hash: [H; 32], url_hash: [0xDE, 0xAD, 0xBE, 0xEF; 8], ... } }
And:   page_diff = ScrapePageDiff { unchanged: [0], changed_or_new: [] }
When:  load_archived_scrape_pages(page_diff, fresh_pages, url_states, session) is called
Then:  result is Err(ScrapeReuseError::DeserializationFailed) where:
       - key_hex == "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"
       - message is a non-empty string (the concrete rkyv::rancor::Error::to_string()
         for corrupt bytes [0xFF; 128])
       - message.len() > 5 (rejects trivial placeholders)
       The test MUST assert both key_hex and message — not just the variant.
```

### Behavior 12: `load_archived_scrape_pages` — individual page deserialization failure (non-fatal)

**Test function:** `fn load_archived_scrape_pages_falls_back_when_individual_page_deserialization_fails()`

```
Given: redb with valid PersistedScrapeResult batch containing one page with NaN density_score
And:   url_states with matching url_hash
And:   page_diff = ScrapePageDiff { unchanged: [0], changed_or_new: [] }
When:  load_archived_scrape_pages(page_diff, fresh_pages, url_states, session) is called
Then:  result.0 == {} (no successfully loaded pages)
And:   result.1 == [0] (fallback to fresh for the failed page)
And:   INV-4 verified: individual page failure is non-fatal
```

### Behavior 13: `load_archived_scrape_pages` — hash mismatch

**Test function:** `fn load_archived_scrape_pages_returns_hash_mismatch_when_loaded_hash_differs()`

```
Given: redb with valid PersistedScrapeResult batch where page content differs from stored hash
And:   url_states = { "https://a.com" => UrlStateRaw { content_hash: [STORED_H; 32], url_hash: [1; 32], ... } }
And:   archived page has markdown producing hash [DIFFERENT_H; 32]
And:   page_diff = ScrapePageDiff { unchanged: [0], changed_or_new: [] }
When:  load_archived_scrape_pages(page_diff, fresh_pages, url_states, session) is called
Then:  result.0 == {} (no loaded pages)
And:   result.1 == [0] (fallback to fresh)
And:   Warning logged about HashMismatch for "https://a.com"
```

### Behavior 14: `load_archived_scrape_pages` — missing scrape_output row

**Test function:** `fn load_archived_scrape_pages_returns_fallback_when_scrape_output_missing()`

```
Given: redb with no entry in scrape_outputs for url_hash [1; 32]
And:   url_states = { "https://a.com" => UrlStateRaw { content_hash: [H; 32], url_hash: [1; 32], ... } }
And:   page_diff = ScrapePageDiff { unchanged: [0], changed_or_new: [] }
When:  load_archived_scrape_pages(page_diff, fresh_pages, url_states, session) is called
Then:  result.0 == {} (no loaded pages)
And:   result.1 == [0] (fallback to fresh)
```

### Behavior 15: `load_archived_scrape_pages` — StateLoad error propagation

**Test function:** `fn load_archived_scrape_pages_propagates_state_load_error_when_url_state_table_missing()`

```
Given: redb WITHOUT url_state table (tables not initialized)
When:  session.load_url_states() is called
Then:  result is Err(StateLoadError::BackendError) where:
       - operation == "open_table" (exact match — this field is a &'static str constant)
       - message is a non-empty string (the concrete redb::StorageError::to_string()
         for the missing table — version-dependent)
       - message.len() > 3 (rejects empty or trivial placeholders)
       The test MUST assert both operation and message — not just the variant.
And:   scrape_with_reuse propagates as Err(ScrapeReuseError::StateLoad(inner))
       where inner.operation == "open_table" and inner.message is non-empty
```

### Behavior 16: `load_archived_scrape_pages` — BulkLoad error propagation

**Test function:** `fn load_archived_scrape_pages_propagates_bulk_load_error_when_scrape_outputs_table_missing()`

```
Given: redb WITHOUT scrape_outputs table
And:   url_states loaded successfully (url_state table exists)
When:  load_archived_scrape_pages attempts to load from scrape_outputs
Then:  result is Err(ScrapeReuseError::BulkLoad(BulkLoadError::TableOpen)) where:
       - table == "scrape_outputs" (exact match — this field is a &'static str constant)
       - message is a non-empty string (the concrete redb::StorageError::to_string()
         for the missing table — version-dependent)
       - message.len() > 3 (rejects empty or trivial placeholders)
       The test MUST assert both table and message — not just the variant.
```

### Behavior 17: `merge_scrape_pages_in_order` — empty archived

**Test function:** `fn merge_scrape_pages_in_order_returns_fresh_pages_when_archived_is_empty()`

```
Given: fresh_pages = [page_A, page_B, page_C]
And:   archived_pages = {} (empty HashMap)
When:  merge_scrape_pages_in_order(fresh_pages, archived_pages) is called
Then:  result.len() == 3
And:   result[0].url == page_A.url
And:   result[1].url == page_B.url
And:   result[2].url == page_C.url
```

### Behavior 18: `merge_scrape_pages_in_order` — substitution preserving order

**Test function:** `fn merge_scrape_pages_in_order_substitutes_archived_at_correct_indices_preserving_order()`

```
Given: fresh_pages = [page_A, page_B, page_C]
And:   archived_pages = { 0 => archived_A, 2 => archived_C }
When:  merge_scrape_pages_in_order(fresh_pages, archived_pages) is called
Then:  result.len() == 3
And:   result[0] == archived_A (not fresh page_A)
And:   result[1] == page_B (fresh, not in archived)
And:   result[2] == archived_C (not fresh page_C)
And:   POST-4: order matches input order
```

### Behavior 19: `scrape_with_reuse` — all unchanged

**Test function:** `fn scrape_with_reuse_returns_all_reused_pages_when_all_unchanged()`

```
Given: redb with valid scrape_outputs for all page hashes
And:   fresh_result with 3 pages, all matching stored hashes
When:  scrape_with_reuse(fresh_result, session) is called
Then:  result.0.pages.len() == 3
And:   result.1 == ScrapeReuseStats { reused: 3, scraped: 0 }
And:   POST-1: every page appears exactly once
And:   POST-2: unchanged pages loaded from archive
```

### Behavior 19b: `scrape_with_reuse` — empty ScrapeResult (zero pages)

**Test function:** `fn scrape_with_reuse_returns_empty_result_with_zero_stats_when_no_pages()`

```
Given: redb with any state (tables exist and are readable)
And:   fresh_result = ScrapeResult { pages: [], ... } (zero pages)
When:  scrape_with_reuse(fresh_result, session) is called
Then:  result.0.pages == [] (empty pages in returned ScrapeResult)
And:   result.1 == ScrapeReuseStats { reused: 0, scraped: 0 }
And:   POST-5 verified: 0 + 0 == 0
```

### Behavior 20: `scrape_with_reuse` — all changed

**Test function:** `fn scrape_with_reuse_returns_all_fresh_pages_when_all_changed()`

```
Given: redb with no matching hashes (all content changed)
And:   fresh_result with 2 pages
When:  scrape_with_reuse(fresh_result, session) is called
Then:  result.0.pages.len() == 2
And:   result.1 == ScrapeReuseStats { reused: 0, scraped: 2 }
And:   POST-3: all pages from fresh scrape
```

### Behavior 21: `scrape_with_reuse` — mixed

**Test function:** `fn scrape_with_reuse_returns_mixed_result_when_some_unchanged_some_changed()`

```
Given: redb with valid scrape_outputs for 1 of 3 pages
And:   fresh_result with 3 pages: page_0 (unchanged), page_1 (changed), page_2 (new)
When:  scrape_with_reuse(fresh_result, session) is called
Then:  result.0.pages.len() == 3
And:   result.1.reused == 1
And:   result.1.scraped == 2
And:   POST-4: output order matches input order
```

### Behavior 22: `scrape_with_reuse` — stats invariant

**Test function:** `fn scrape_with_reuse_stats_reused_plus_scraped_equals_total_pages()`

```
Given: any fresh_result with N pages
And:   any redb state (any combination of matches/non-matches)
When:  scrape_with_reuse(fresh_result, session) is called
Then:  stats.reused + stats.scraped == N
And:   POST-5 verified
```

### Behavior 23: `scrape_with_reuse` — StateLoad error propagation

**Test function:** `fn scrape_with_reuse_propagates_state_load_error_from_session()`

```
Given: redb WITHOUT url_state table
When:  scrape_with_reuse(fresh_result, session) is called
Then:  result is Err(ScrapeReuseError::StateLoad(StateLoadError::BackendError)) where:
       - inner.operation == "open_table" (exact match)
       - inner.message is a non-empty string (concrete redb error for missing table)
       - inner.message.len() > 3 (rejects empty or trivial placeholders)
       The test MUST assert both inner.operation and inner.message — not just the
       outer ScrapeReuseError::StateLoad variant wrapper.
```

### Behaviors 24-25: Default trait implementations

**Test function:** `fn scrape_page_diff_default_returns_empty_partitions()`

```
Given: ScrapePageDiff::default()
Then:  diff.unchanged == []
And:   diff.changed_or_new == []
```

**Test function:** `fn scrape_reuse_stats_default_returns_zero_counts()`

```
Given: ScrapeReuseStats::default()
Then:  stats.reused == 0
And:   stats.scraped == 0
```

### Behaviors 26-30: Error variant Display messages

**Test function:** `fn scrape_reuse_error_state_load_displays_correctly()`
```
Given: ScrapeReuseError::StateLoad(StateLoadError::BackendError { operation: "open_table", message: "table missing".into() })
When:  format!("{error}")
Then:  message contains "failed to load url states" AND "table missing"
```

**Test function:** `fn scrape_reuse_error_bulk_load_displays_correctly()`
```
Given: ScrapeReuseError::BulkLoad(BulkLoadError::TableOpen { table: "scrape_outputs", message: "not found".into() })
When:  format!("{error}")
Then:  message contains "failed to load archived scrape outputs" AND "not found"
```

**Test function:** `fn scrape_reuse_error_deserialization_failed_displays_key_hex_and_message()`
```
Given: ScrapeReuseError::DeserializationFailed { key_hex: "deadbeef".into(), message: "invalid archive".into() }
When:  format!("{error}")
Then:  message contains "deadbeef" AND "invalid archive"
```

**Test function:** `fn scrape_reuse_error_hash_mismatch_displays_url_and_hashes()`
```
Given: ScrapeReuseError::HashMismatch { url: "https://a.com".into(), stored_hex: "aa".into(), loaded_hex: "bb".into() }
When:  format!("{error}")
Then:  message contains "https://a.com" AND "aa" AND "bb"
```

**Test function:** `fn scrape_reuse_error_missing_url_state_displays_url()`
```
Given: ScrapeReuseError::MissingUrlState { url: "https://missing.com".into() }
When:  format!("{error}")
Then:  message contains "https://missing.com" AND "missing url_state"
```

## 4. Proptest Invariants

### Proptest: `compute_page_content_hash` determinism

```
Invariant:  compute_page_content_hash(s) == compute_page_content_hash(s) for all s
Strategy:   any valid UTF-8 string (0..10_000 chars)
Anti-invariant: no anti-invariant (always holds)
```

### Proptest: `compute_page_content_hash` — different inputs → different hashes (probabilistic)

```
Invariant:  For any two distinct non-empty strings s1 != s2,
            compute_page_content_hash(s1) != compute_page_content_hash(s2)
            (holds with overwhelming probability for SHA-256)
Strategy:   two distinct non-empty strings
Anti-invariant: same hash for different inputs (SHA-256 collision — astronomically unlikely)
```

### Proptest: `classify_scraped_pages` — partition completeness

```
Invariant:  diff.unchanged.len() + diff.changed_or_new.len() == fresh_pages.len()
            for all inputs (including empty)
Strategy:   Vec<ScrapedPage> (0..100 pages) with random markdown content,
            corresponding fresh_hashes, random url_states HashMap
Anti-invariant: none — always holds by construction
```

### Proptest: `classify_scraped_pages` — partition mutual exclusivity

```
Invariant:  unchanged ∩ changed_or_new == ∅
            No index appears in both partitions
Strategy:   Vec<ScrapedPage> (0..100 pages), random url_states
Anti-invariant: none — always holds by construction
```

### Proptest: `merge_scrape_pages_in_order` — length and order preservation

```
Invariant:  merged.len() == fresh_pages.len()
            AND merged[i].url == (archived_pages.get(&i).unwrap_or(&fresh_pages[i])).url
Strategy:   Vec<ScrapedPage> (1..50 pages), random HashMap<usize, ScrapedPage> subset
Anti-invariant: none — always holds
```

## 5. Fuzz Targets

### Fuzz Target: `compute_page_content_hash`

```
Input type:    arbitrary &[u8] (reinterpreted as UTF-8 string, lossy)
Risk:          Panic on extremely long input (unbounded allocation in SHA-256)
               → Should accept any byte sequence without panic
Corpus seeds:  [b"", b"# Title\n", b"\x00\x00\x00", b"hello world",
               10KB random markdown, 100KB random bytes, UTF-8 edge cases,
               markdown with null bytes, markdown with only newlines]
```

### Fuzz Target: `classify_scraped_pages` via rkyv deserialization of PersistedScrapeResult

```
Input type:    arbitrary &[u8] (fed to rkyv::from_bytes::<PersistedScrapeResult>)
Risk:          Panic, OOM, or logic error on malformed archived bytes
               → Must never panic; malformed input produces Err or fallback
Corpus seeds:  [valid serialized PersistedScrapeResult with 1 page,
               valid with 0 pages, valid with empty markdown,
               truncated bytes (partial archive), all-zeros (32 bytes),
               corrupted schema_version field, NaN density_score bytes]
```

## 6. Kani Harnesses

### Kani Harness: `classify_scraped_pages` partition completeness

```
Property:     For any inputs (with bounded sizes), unchanged.len() + changed_or_new.len() == N
Bound:        fresh_pages.len() <= 10, url_states.len() <= 10
Rationale:    This is a critical invariant (INV-5 / POST-1). A bug here means
              pages are silently dropped or duplicated. Proptest catches most
              cases, but Kani proves it for ALL inputs within the bound.
```

### Kani Harness: `merge_scrape_pages_in_order` length preservation

```
Property:     merged.len() == fresh_pages.len() for all valid inputs
Bound:        fresh_pages.len() <= 20, archived_pages.len() <= 20
Rationale:    A length mismatch would mean a page was silently dropped or
              duplicated. This is POST-1/POST-4 — the most critical guarantee.
```

## 7. Mutation Testing Checkpoints

### Critical mutations to catch:

| Source function | Mutation | Caught by test scenario |
|-----------------|----------|------------------------|
| `compute_page_content_hash` | Change SHA-256 to identity (return all-zeros) | `compute_page_content_hash_returns_sha256_when_given_nonempty_markdown` |
| `classify_scraped_pages` | Skip zero-url_hash check (treat as unchanged) | `classify_scraped_pages_classifies_changed_or_new_when_url_hash_is_zero` |
| `classify_scraped_pages` | Swap unchanged/changed_or_new buckets | `classify_scraped_pages_classifies_unchanged_when_hash_matches_stored` |
| `classify_scraped_pages` | Skip missing-url_state check | `classify_scraped_pages_classifies_changed_or_new_when_url_state_missing` |
| `classify_scraped_pages` | **Use `>=` instead of `==` for hash comparison** | **`classify_scraped_pages_classifies_changed_when_stored_hash_is_lexicographically_greater` (Behavior 5b — NEW)** |
| `classify_scraped_pages` | Put index in BOTH partitions | `classify_scraped_pages_produces_mutually_exclusive_collectively_exhaustive_partition` |
| `classify_scraped_pages` | Put index in NEITHER partition | `classify_scraped_pages_produces_mutually_exclusive_collectively_exhaustive_partition` |
| `classify_scraped_pages` | Silently truncate mismatched input lengths | `classify_scraped_pages_panics_or_errors_when_input_lengths_mismatch` (Behavior 8b — NEW) |
| `merge_scrape_pages_in_order` | Always use fresh page (ignore archived) | `merge_scrape_pages_in_order_substitutes_archived_at_correct_indices_preserving_order` |
| `merge_scrape_pages_in_order` | Swap merge priority (archived first → fresh first) | `merge_scrape_pages_in_order_substitutes_archived_at_correct_indices_preserving_order` |
| `merge_scrape_pages_in_order` | Reverse order | `merge_scrape_pages_in_order_substitutes_archived_at_correct_indices_preserving_order` |
| `scrape_with_reuse` | Don't compute hashes (pass empty) | `scrape_with_reuse_returns_all_reused_pages_when_all_unchanged` |
| `scrape_with_reuse` | Swap reused/scraped stats | `scrape_with_reuse_returns_all_reused_pages_when_all_unchanged` (asserts reused:3, scraped:0) |
| `scrape_with_reuse` | Drops pages on empty input | `scrape_with_reuse_returns_empty_result_with_zero_stats_when_no_pages` (Behavior 19b — NEW) |
| `scrape_with_reuse` | Returns wrong stats on empty input | `scrape_with_reuse_returns_empty_result_with_zero_stats_when_no_pages` (Behavior 19b — NEW) |
| `load_archived_scrape_pages` | Skip hash verification | `load_archived_scrape_pages_returns_hash_mismatch_when_loaded_hash_differs` |
| `load_archived_scrape_pages` | Make batch deser failure fatal for entire operation | `load_archived_scrape_pages_falls_back_when_individual_page_deserialization_fails` |
| `load_archived_scrape_pages` | Ignores multi-batch (only loads first batch) | `load_archived_scrape_pages_loads_pages_from_different_batches_when_url_hashes_differ` (Behavior 10b — NEW) |
| `load_archived_scrape_pages` | Treats empty batch as success (no fallback) | `load_archived_scrape_pages_returns_fallback_when_batch_contains_zero_pages` (Behavior 10c — NEW) |
| `ScrapeReuseError::HashMismatch` | Wrong field in Display impl | `scrape_reuse_error_hash_mismatch_displays_url_and_hashes` |
| `ScrapeReuseError::DeserializationFailed` | Missing key_hex in Display | `scrape_reuse_error_deserialization_failed_displays_key_hex_and_message` |
| `ScrapeReuseError::StateLoad` | Missing operation in propagated error | `scrape_with_reuse_propagates_state_load_error_from_session` (asserts inner.operation == "open_table") |
| `ScrapeReuseError::BulkLoad` | Missing table in propagated error | `load_archived_scrape_pages_propagates_bulk_load_error_when_scrape_outputs_table_missing` (asserts inner.table == "scrape_outputs") |

**Mutation threshold: ≥90% kill rate.**
The proptest invariants provide additional coverage against arithmetic/logic mutations that targeted unit tests might miss. The new Behavior 5b deterministically catches the `>=` vs `==` mutation that was previously only caught probabilistically.

## 8. Combinatorial Coverage Matrix

### `classify_scraped_pages`

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| empty pages | 0 pages | unchanged=[], changed_or_new=[] | unit |
| single unchanged | 1 page, hash matches, url_hash non-zero | unchanged=[0], changed_or_new=[] | unit |
| single changed (hash mismatch) | 1 page, hash differs | unchanged=[], changed_or_new=[0] | unit |
| **single changed (lexicographic trap)** | 1 page, stored=[0xFF;32] > fresh_hash | **unchanged=[], changed_or_new=[0]** | **unit** |
| single new (missing state) | 1 page, no url_states entry | unchanged=[], changed_or_new=[0] | unit |
| single zero url_hash | 1 page, url_hash=[0;32] | unchanged=[], changed_or_new=[0] | unit |
| **mismatched lengths** | pages.len()=2, hashes.len()=1 | **panic with "length" message** | **unit** |
| multiple mixed | 3 pages: match, mismatch, missing | unchanged=[0], changed_or_new=[1,2] | unit |
| all unchanged | N pages, all match | unchanged=[0..N], changed_or_new=[] | unit |
| all changed | N pages, all mismatch | unchanged=[], changed_or_new=[0..N] | unit |
| partition MCE | any valid input | len(unchanged)+len(changed_or_new)==N | proptest |

### `load_archived_scrape_pages`

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| no unchanged pages | unchanged=[] | Ok({}, []) | integration |
| successful load | valid archive in DB | Ok({idx=>page}, []) | integration |
| **multi-batch load** | 2 pages, 2 different url_hashes | **Ok({0=>page_A, 1=>page_B}, [])** | **integration** |
| **empty batch** | valid archive with 0 pages | **Ok({}, [idx])** | **integration** |
| corrupt batch | invalid rkyv bytes | Err(DeserializationFailed { key_hex: "deadbeef...", message: non_empty }) | integration |
| page-level failure | valid batch, bad page fields | Ok({}, [idx]) | integration |
| hash mismatch | valid batch, hash differs | Ok({}, [idx]) | integration |
| missing scrape_output | no row for url_hash | Ok({}, [idx]) | integration |
| missing url_state table | DB without table | Err(StateLoad(BackendError { operation: "open_table", message: non_empty })) | integration |
| missing scrape_outputs table | DB without table | Err(BulkLoad(TableOpen { table: "scrape_outputs", message: non_empty })) | integration |

### `merge_scrape_pages_in_order`

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| empty archived | archived_pages={} | fresh_pages unchanged | unit |
| full archived | archived_pages={0,1,2} | all from archive | unit |
| partial archived | archived_pages={0,2} | [archived_0, fresh_1, archived_2] | unit |
| single page | 1 page, no archive | [fresh_0] | unit |
| length invariant | any valid input | merged.len() == fresh.len() | proptest |

### `scrape_with_reuse` (entry point)

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| all unchanged | all hashes match | (result, stats{reused=N, scraped=0}) | integration |
| **empty ScrapeResult** | 0 pages | **(result{pages=[]}, stats{reused=0, scraped=0})** | **integration** |
| all changed | no hashes match | (result, stats{reused=0, scraped=N}) | integration |
| mixed | partial match | (result, stats{reused=M, scraped=K}) | integration |
| stats invariant | any valid input | reused+scraped == N | integration |
| DB error | missing tables | Err(StateLoad(BackendError { operation: "open_table", message: non_empty })) | integration |

### `ScrapeReuseError` variants

| Scenario | Variant | Expected Display contains | Layer |
|----------|---------|--------------------------|-------|
| state load fail | StateLoad | "failed to load url states" | unit |
| bulk load fail | BulkLoad | "failed to load archived scrape outputs" | unit |
| deser fail | DeserializationFailed | key_hex + message | unit |
| hash mismatch | HashMismatch | url + stored_hex + loaded_hex | unit |
| missing state | MissingUrlState | url + "missing url_state" | unit |

## 9. Static Analysis Gates

| Lint | Enforcement |
|------|-------------|
| `deny(unwrap_used)` | Module-level `#![deny]` directive |
| `deny(expect_used)` | Module-level `#![deny]` directive |
| `deny(panic)` | Module-level `#![deny]` directive |
| `forbid(unsafe_code)` | Module-level `#![forbid]` directive |
| `clippy::pedantic` | Module-level `#![allow]` with selective denies |
| `cargo-deny` | No new advisories or unsound dependencies |

## 10. Integration Test Infrastructure

All integration tests require a real redb database. Use the same pattern as `bulk_load.rs` tests:

```rust
fn fresh_db() -> (TempDir, Database) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.redb");
    let db = Database::create(&db_path).unwrap();
    initialize_tables(&db).unwrap();
    (temp_dir, db)
}
```

Helper functions needed:
- `write_url_rows(db, &[(url, UrlStateRaw)])` — already exists in `bulk_load/tests`
- `write_scrape_output(db, &[[u8; 32]], &PersistedScrapeResult)` — NEW, must serialize with rkyv
- `make_scraped_page(url, markdown) -> ScrapedPage` — test factory
- `make_persisted_scrape_result(pages) -> PersistedScrapeResult` — test factory
- `open_db_without_table(table_name: &str) -> (TempDir, Database)` — already exists in `bulk_load/tests` (for missing-table scenarios)

## 11. Open Questions

None. The contract is fully specified with no ambiguities. All types, error variants, invariants, and pre/postconditions are documented.

## Exit Criteria Checklist

- [x] Every public API behavior has a BDD scenario (35 behaviors, 35 test functions)
- [x] Every Error variant has a test scenario (5 variants, 5 Display tests)
- [x] Every pure function with multiple inputs has a proptest invariant (5 invariants)
- [x] Every parsing/deserialization boundary has a fuzz target (2 targets)
- [x] Kani harnesses for critical invariants (2 harnesses)
- [x] Mutation threshold (≥90%) is stated with specific mutation-to-test mappings
- [x] No planned assertion is just `is_ok()` or `is_err()` — all assert exact values or exact error variants
- [x] No ellipsis (`...`) in any error variant assertion — all fields have concrete checks
- [x] `>=` vs `==` mutation on hash comparison is deterministically caught (Behavior 5b)
- [x] Mismatched input lengths scenario specified (Behavior 8b)
- [x] Multi-batch load scenario specified (Behavior 10b)
- [x] Empty batch scenario specified (Behavior 10c)
- [x] Empty ScrapeResult entry point scenario specified (Behavior 19b)
