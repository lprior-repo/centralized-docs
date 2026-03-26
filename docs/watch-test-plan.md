# Test Plan: watch.rs — Terraform-style Plan/Apply Workflow

## Summary

- **Behaviors identified**: 31
- **Trophy allocation**: 11 unit / 16 integration / 2 e2e / 2 static
- **Proptest invariants**: 5
- **Fuzz targets**: 2
- **Kani harnesses**: 2
- **Existing tests**: 22 unit + 20 integration — **gaps**: 11 behaviors untested or under-tested

---

## 1. Behavior Inventory

Every behavior the system guarantees, enumerated as `[Subject] [action] [outcome] when [condition]`.

| # | Behavior |
|---|----------|
| B01 | `snapshot_from_scrape` produces a snapshot with one entry per page when scrape contains N pages |
| B02 | `snapshot_from_scrape` produces identical hashes when called twice with same content |
| B03 | `snapshot_from_scrape` produces empty snapshot pages when scrape has zero pages |
| B04 | `compute_plan` returns empty changes when previous and current content are identical |
| B05 | `compute_plan` detects added pages when current has URLs absent from previous |
| B06 | `compute_plan` detects removed pages when previous has URLs absent from current |
| B07 | `compute_plan` detects modified pages when same URL has different content hash |
| B08 | `compute_plan` is idempotent — same inputs always produce identical plans |
| B09 | `compute_plan` marks all pages as Added when previous snapshot is empty (first scrape) |
| B10 | `compute_plan` marks all pages as Removed when current scrape is empty (complete removal) |
| B11 | `compute_plan` produces empty plan when both previous and current are empty |
| B12 | `compute_plan` classifies URL change as Added + Removed (not Modified) when content differs by URL |
| B13 | `compute_plan` does NOT detect title-only changes as Modified when content hash is unchanged |
| B14 | `compute_plan` sorts changes by kind then URL when mixed changes exist |
| B15 | `compute_plan` summary conservation: added + modified + unchanged == total_current |
| B16 | `format_plan_markdown` includes all sections when plan has added, removed, and modified changes |
| B17 | `format_plan_markdown` outputs "No changes detected" message when plan is empty |
| B18 | `format_plan_markdown` omits Removed section when no removals exist |
| B19 | `format_plan_json` roundtrips through serialization when given any valid plan |
| B20 | `write_plan_reports` creates change-plan.json and change-plan.md when output dir exists |
| B21 | `write_plan_reports` creates output directory tree when nested path does not exist |
| B22 | `diff_directories` compares manifests and produces plan when both dirs have valid manifest.json |
| B23 | `diff_directories` returns error when one manifest.json is missing |
| B24 | `diff_directories` returns error when one manifest.json contains invalid JSON |
| B25 | `diff_directories` produces empty plan when both manifests are identical |
| B26 | `ChangeSummary::is_empty` returns true when all change counts are zero |
| B27 | `ChangeSummary::is_empty` returns false when any change count is non-zero |
| B28 | `ChangeKind::Display` formats as lowercase snake_case strings |
| B29 | `Snapshot` serialization roundtrip preserves all URLs, hashes, and titles |
| B30 | `Snapshot` handles unicode URLs and titles correctly |
| B31 | `compute_plan` handles empty content strings and whitespace-only content |

---

## 2. Trophy Allocation

| Layer | Count | Rationale |
|-------|-------|-----------|
| **Unit / Calc** | 11 | Pure functions: `snapshot_from_scrape`, `compute_plan`, `format_plan_markdown`, `format_plan_json`, `ChangeSummary::is_empty`, `ChangeKind::Display` — deterministic, no I/O |
| **Integration** | 16 | `diff_directories` (real FS), `write_plan_reports` (real FS), serialization roundtrips, edge cases requiring real data structures |
| **E2E** | 2 | CLI commands `run_watch`, `run_apply`, `run_diff` — full workflow from outside |
| **Static** | 2 | Clippy lint catches, type system enforces `#[must_use]`, `Result` returns |

**Target ratio**: ~55% integration, ~35% unit, ~5% e2e, ~5% static. Justification: This module is calculation-heavy with thin I/O boundaries. The pure calc functions deserve exhaustive combinatorial testing; the I/O boundary needs real filesystem integration tests.

---

## 3. BDD Scenarios

### B01: snapshot_from_scrape produces correct page count

```
Given: A ScrapeResult with 3 pages having distinct URLs
When: snapshot_from_scrape("https://example.com", &result) is called
Then: snapshot.pages.len() == 3
And: Every URL in the scrape appears as a key in snapshot.pages
And: snapshot.target_url == "https://example.com"
```

Test: `fn snapshot_from_scrape_produces_correct_page_count()`

---

### B02: snapshot_from_scrape is deterministic

```
Given: A ScrapeResult with pages containing specific markdown content
When: snapshot_from_scrape is called twice with the same inputs
Then: Every page's content_hash is identical across both snapshots
```

Test: `fn snapshot_from_scrape_is_deterministic()`

**Status: COVERED** — `test_snapshot_from_scrape_deterministic` in unit tests, proptest `snapshot_from_same_scrape_is_deterministic`

---

### B03: snapshot_from_scrape produces empty snapshot from empty scrape

```
Given: A ScrapeResult with zero pages (empty pages vec)
When: snapshot_from_scrape is called
Then: snapshot.pages.is_empty() is true
And: snapshot.target_url matches the input
```

Test: `fn snapshot_from_scrape_empty_scrape_produces_empty_snapshot()`

**Status: GAP** — not directly tested (only tested through `compute_plan` with empty current)

---

### B04: compute_plan returns empty changes on identical content

```
Given: Previous snapshot with 2 pages ("a" hash H1, "b" hash H2)
And: Current scrape with same 2 pages (same content → same hashes)
When: compute_plan("https://example.com", &prev, &current) is called
Then: plan.changes is empty
And: plan.summary.is_empty() is true
And: plan.summary.unchanged == 2
And: plan.summary.total_current == plan.summary.total_previous
```

Test: `fn compute_plan_empty_on_identical_content()`

**Status: COVERED** — `test_empty_plan_on_identical_content`

---

### B05: compute_plan detects added pages

```
Given: Previous snapshot with 1 page ("/a")
And: Current scrape with 2 pages ("/a" unchanged, "/b" new)
When: compute_plan is called
Then: plan.summary.added == 1
And: plan.summary.removed == 0
And: plan.summary.modified == 0
And: plan.summary.unchanged == 1
And: The change entry for "/b" has kind == Added, old_hash == None, new_hash == Some(_)
```

Test: `fn compute_plan_detects_added_page()`

**Status: COVERED** — `test_detects_added_page`

---

### B06: compute_plan detects removed pages

```
Given: Previous snapshot with 2 pages ("/a", "/b")
And: Current scrape with 1 page ("/a" only)
When: compute_plan is called
Then: plan.summary.added == 0
And: plan.summary.removed == 1
And: plan.summary.modified == 0
And: plan.summary.unchanged == 1
And: The change entry for "/b" has kind == Removed, old_hash == Some(_), new_hash == None
```

Test: `fn compute_plan_detects_removed_page()`

**Status: COVERED** — `test_detects_removed_page`

---

### B07: compute_plan detects modified pages

```
Given: Previous snapshot with page "/a" having content "old" (hash H1)
And: Current scrape with page "/a" having content "new" (hash H2)
When: compute_plan is called
Then: plan.summary.modified == 1
And: plan.summary.added == 0
And: plan.summary.removed == 0
And: The change entry has kind == Modified
And: old_hash == Some(H1), new_hash == Some(H2), old_hash != new_hash
```

Test: `fn compute_plan_detects_modified_page()`

**Status: COVERED** — `test_detects_modified_page`

---

### B08: compute_plan is idempotent

```
Given: A previous snapshot and a current scrape with mixed changes
When: compute_plan is called twice with identical inputs
Then: Both returned plans have identical changes, summary, and target_url
And: Corresponding PageChange entries match on url, kind, old_hash, new_hash
```

Test: `fn compute_plan_is_idempotent()`

**Status: COVERED** — `test_apply_is_idempotent`, `same_scrape_twice_produces_identical_plans`

---

### B09: first scrape marks all as Added

```
Given: Previous snapshot with 0 pages (empty BTreeMap)
And: Current scrape with 3 pages
When: compute_plan is called
Then: plan.summary.added == 3
And: plan.summary.removed == 0
And: plan.summary.modified == 0
And: plan.summary.unchanged == 0
And: plan.summary.total_previous == 0
And: plan.summary.total_current == 3
And: Every change entry has kind == Added
```

Test: `fn first_scrape_all_pages_are_added()`

**Status: COVERED** — `test_first_scrape_all_added`, `empty_previous_all_pages_are_added`

---

### B10: complete removal marks all as Removed

```
Given: Previous snapshot with 2 pages
And: Current scrape with 0 pages
When: compute_plan is called
Then: plan.summary.removed == 2
And: plan.summary.added == 0
And: plan.summary.total_current == 0
And: plan.summary.total_previous == 2
And: Every change entry has kind == Removed
```

Test: `fn complete_removal_all_pages_are_removed()`

**Status: COVERED** — `test_complete_removal`, `empty_current_all_pages_are_removed`

---

### B11: both empty produces empty plan

```
Given: Previous snapshot with 0 pages
And: Current scrape with 0 pages
When: compute_plan is called
Then: plan.changes is empty
And: plan.summary.is_empty() is true
And: All summary counters are 0
```

Test: `fn both_empty_produces_empty_plan()`

**Status: COVERED** — `both_empty_produces_empty_plan`

---

### B12: URL change is Added + Removed (not Modified)

```
Given: Previous snapshot with page at "/old-path" with content "same"
And: Current scrape with page at "/new-path" with content "same"
When: compute_plan is called
Then: plan.summary.added == 1 ("/new-path")
And: plan.summary.removed == 1 ("/old-path")
And: plan.summary.modified == 0
```

Test: `fn url_change_produces_added_plus_removed()`

**Status: COVERED** — `page_url_changes_are_added_plus_removed`

---

### B13: title-only change is NOT detected as Modified

```
Given: Previous snapshot with page "/a" titled "Old Title" content "same"
And: Current scrape with page "/a" titled "New Title" content "same"
When: compute_plan is called
Then: plan.summary.modified == 0
And: plan.summary.unchanged == 1
And: plan.changes is empty
```

Test: `fn title_change_only_not_detected_as_modified()`

**Status: COVERED** — `title_change_only_not_detected_as_modified`

---

### B14: changes sorted by kind then URL

```
Given: A plan with added, removed, and modified changes across multiple URLs
When: compute_plan returns the plan
Then: plan.changes is sorted: all Added (by URL), then Removed (by URL), then Modified (by URL)
```

Test: `fn changes_sorted_by_kind_then_url()`

**Status: GAP** — not explicitly tested (sort logic exists in `diff_snapshots` but no test verifies ordering of mixed change kinds)

---

### B15: summary conservation invariant

```
Given: Any previous snapshot and any current scrape
When: compute_plan is called
Then: plan.summary.added + plan.summary.modified + plan.summary.unchanged == plan.summary.total_current
And: plan.summary.total_current == plan.pending_snapshot.pages.len()
```

Test: `fn summary_conservation_holds()`

**Status: PARTIALLY COVERED** — `large_scrape_produces_correct_plan` checks conservation, but not explicitly named as invariant

---

### B16: format_plan_markdown includes all sections

```
Given: A change plan with 1 added, 1 removed, 1 modified page
When: format_plan_markdown(&plan) is called
Then: Output contains "# Documentation Change Plan"
And: Output contains "**Added:** 1"
And: Output contains "**Removed:** 1"
And: Output contains "**Modified:** 1"
And: Output contains "### Added" with "+ `url` — title" format
And: Output contains "### Removed" with "- `url` — title" format
And: Output contains "### Modified" with "~ `url` — title" format
And: Output contains "Run `ctd apply`"
```

Test: `fn markdown_report_contains_all_sections()`

**Status: COVERED** — `test_markdown_report_format`, `markdown_report_shows_all_change_kinds`

---

### B17: format_plan_markdown says "up to date" on empty plan

```
Given: A change plan with zero changes
When: format_plan_markdown(&plan) is called
Then: Output contains "No changes detected. The documentation is up to date."
```

Test: `fn markdown_report_empty_plan_shows_up_to_date()`

**Status: COVERED** — `markdown_report_empty_plan_says_up_to_date`

---

### B18: format_plan_markdown omits Removed section when no removals

```
Given: A change plan with only additions (no removals, no modifications)
When: format_plan_markdown(&plan) is called
Then: Output does NOT contain "### Removed"
And: Output does NOT contain "### Modified"
And: Output contains "### Added"
```

Test: `fn markdown_report_omits_empty_sections()`

**Status: GAP** — not explicitly tested

---

### B19: format_plan_json roundtrips

```
Given: A valid ChangePlan with mixed changes
When: format_plan_json(&plan) is called and result is deserialized
Then: Deserialized plan has same target_url, changes count, and summary
And: format_plan_json returns Ok(String)
```

Test: `fn json_report_roundtrips()`

**Status: COVERED** — `test_json_report_format`, `json_report_roundtrips`

---

### B20: write_plan_reports creates both files

```
Given: A valid change plan and an existing output directory
When: write_plan_reports(&plan, &dir) is called
Then: dir/change-plan.json exists and deserializes to valid ChangePlan
And: dir/change-plan.md exists and contains "# Documentation Change Plan"
And: Returns Ok(())
```

Test: `fn write_plan_reports_creates_both_files()`

**Status: COVERED** — `test_write_plan_reports_creates_files`, `write_plan_reports_creates_both_files`

---

### B21: write_plan_reports creates nested output directory

```
Given: A valid change plan and a deeply nested path that does not exist
When: write_plan_reports(&plan, &nested_path) is called
Then: The full directory tree is created
And: Both change-plan.json and change-plan.md exist in the nested path
And: Returns Ok(())
```

Test: `fn write_plan_reports_creates_nested_directories()`

**Status: COVERED** — `write_plan_reports_creates_output_dir_if_missing`

---

### B22: diff_directories compares two valid manifests

```
Given: dir_a/manifest.json has 2 pages ("/a", "/b")
And: dir_b/manifest.json has "/a" (modified content) and "/c" (new)
When: diff_directories(&dir_a, &dir_b) is called
Then: Returns Ok(ChangePlan)
And: plan.summary.added == 1 ("/c")
And: plan.summary.removed == 1 ("/b")
And: plan.summary.modified == 1 ("/a")
And: plan.target_url contains " → " separator
```

Test: `fn diff_directories_compares_manifests()`

**Status: COVERED** — `diff_directories_compares_manifests`

---

### B23: diff_directories returns error on missing manifest

```
Given: dir_a has no manifest.json
When: diff_directories(&dir_a, &dir_b) is called
Then: Returns Err containing "Cannot read" and the path to dir_a/manifest.json
```

Error variant:
```
Given: dir_a has manifest.json, dir_b has no manifest.json
When: diff_directories(&dir_a, &dir_b) is called
Then: Returns Err containing "Cannot read" and the path to dir_b/manifest.json
```

Test: `fn diff_directories_missing_manifest_returns_error()`, `fn diff_directories_missing_manifest_b_returns_error()`

**Status: PARTIALLY COVERED** — only dir_b missing is tested; dir_a missing is not

---

### B24: diff_directories returns error on invalid JSON

```
Given: dir_a/manifest.json contains "not json"
And: dir_b/manifest.json is valid
When: diff_directories(&dir_a, &dir_b) is called
Then: Returns Err containing "Invalid manifest" and the path to dir_a
```

Error variant:
```
Given: dir_a/manifest.json is valid
And: dir_b/manifest.json contains "not json"
When: diff_directories(&dir_a, &dir_b) is called
Then: Returns Err containing "Invalid manifest" and the path to dir_b
```

Test: `fn diff_directories_invalid_manifest_a_returns_error()`, `fn diff_directories_invalid_manifest_b_returns_error()`

**Status: PARTIALLY COVERED** — only dir_b invalid is tested

---

### B25: diff_directories produces empty plan on identical manifests

```
Given: dir_a/manifest.json and dir_b/manifest.json contain identical ScrapeResult
When: diff_directories(&dir_a, &dir_b) is called
Then: Returns Ok(ChangePlan) with empty changes
And: plan.summary.is_empty() is true
```

Test: `fn diff_identical_directories_produces_empty_plan()`

**Status: COVERED** — `diff_identical_directories_produces_empty_plan`

---

### B26: ChangeSummary::is_empty returns true when all zero

```
Given: ChangeSummary { added: 0, removed: 0, modified: 0, unchanged: 5, ... }
When: is_empty() is called
Then: Returns true
```

Test: `fn change_summary_is_empty_when_all_zero()`

**Status: COVERED** — tested via assertions in multiple tests, but no dedicated unit test

---

### B27: ChangeSummary::is_empty returns false when any non-zero

```
Given: ChangeSummary { added: 1, removed: 0, modified: 0, ... }
When: is_empty() is called
Then: Returns false
```

Error variants (one per change kind):
```
Given: ChangeSummary { added: 0, removed: 1, modified: 0, ... }  → false
Given: ChangeSummary { added: 0, removed: 0, modified: 1, ... }  → false
Given: ChangeSummary { added: 1, removed: 1, modified: 1, ... }  → false
```

Test: `fn change_summary_not_empty_when_added()`, `fn change_summary_not_empty_when_removed()`, `fn change_summary_not_empty_when_modified()`

**Status: GAP** — no dedicated test for `is_empty() == false`

---

### B28: ChangeKind::Display formats correctly

```
Given: Each ChangeKind variant
When: to_string() is called (via Display impl)
Then:
  Added.to_string() == "added"
  Removed.to_string() == "removed"
  Modified.to_string() == "modified"
```

Test: `fn change_kind_display_formats_lowercase()`

**Status: GAP** — not tested anywhere

---

### B29: Snapshot serialization roundtrip preserves data

```
Given: A snapshot with multiple pages having distinct URLs, hashes, and titles
When: serde_json::to_string then serde_json::from_str
Then: restored.target_url == original.target_url
And: restored.pages.len() == original.pages.len()
And: For each URL: content_hash, title, url all match
```

Test: `fn snapshot_serialization_roundtrip()`

**Status: COVERED** — `test_snapshot_serialization_roundtrip`, `snapshot_json_roundtrip_preserves_all_hashes`

---

### B30: Unicode URLs and titles are handled correctly

```
Given: Pages with CJK characters in URLs and titles, emoji in content
When: compute_plan is called
Then: Changes reference exact unicode strings (no corruption)
And: Modified detection works correctly for unicode content
```

Test: `fn handles_unicode_urls_and_titles()`, `fn handles_emoji_in_content()`

**Status: COVERED** — `handles_unicode_urls_and_titles`, `handles_emoji_in_content`

---

### B31: Empty and whitespace content is hashable

```
Given: Previous has page with empty content ""
When: compute_plan detects transition to "now has content"
Then: summary.modified == 1 (empty → non-empty detected)
```

```
Given: Both previous and current have page with whitespace-only "   "
When: compute_plan is called
Then: plan.changes is empty (identical whitespace → identical hash)
```

Test: `fn empty_content_to_content_is_modified()`, `fn whitespace_content_unchanged()`

**Status: COVERED** — `pages_with_empty_content_are_handled`, `page_with_only_whitespace_is_hashable`

---

## 4. Proptest Invariants

### Proptest 1: Hash determinism (snapshot_from_scrape)

```
Invariant: For any content string, snapshot_from_scrape produces identical hashes
           when called multiple times with the same input.
Strategy: "[a-zA-Z0-9 ]{0,1000}" — printable ASCII with spaces
Anti-invariant: None (determinism holds for ALL inputs)
```

**Status: COVERED** — `same_content_always_produces_same_hash`

---

### Proptest 2: Hash collision resistance

```
Invariant: For any two distinct content strings, the content_hash differs.
Strategy: Two independent "[a-zA-Z]{1,100}" strings where a != b
Anti-invariant: Identical strings should produce same hash (covered by Proptest 1)
```

**Status: COVERED** — `different_content_produces_different_hash`

**Note**: This proptest verifies xxh3_128 collision resistance for short ASCII strings. For production, extend to Unicode and longer inputs.

---

### Proptest 3: Snapshot determinism with arbitrary URL sets

```
Invariant: For any set of URLs, snapshot_from_scrape produces the same snapshot
           when called twice (deterministic ordering via BTreeMap).
Strategy: prop::collection::vec("[a-z]{1,20}", 1..50) — 1-50 lowercase URL slugs
Anti-invariant: None
```

**Status: COVERED** — `snapshot_from_same_scrape_is_deterministic`

---

### Proptest 4: compute_plan idempotency

```
Invariant: For any (previous, current) pair, calling compute_plan twice
           produces plans with identical changes and summary.
Strategy: Generate random page sets for previous (0..20 pages) and current (0..20 pages)
          with random content strings.
Anti-invariant: None (idempotency is unconditional)
```

**Status: GAP** — tested manually but not as proptest

---

### Proptest 5: Summary conservation

```
Invariant: For any (previous, current) pair,
           plan.summary.added + plan.summary.modified + plan.summary.unchanged
           == plan.summary.total_current
           == plan.pending_snapshot.pages.len()
Strategy: Same as Proptest 4
Anti-invariant: None
```

**Status: GAP** — partially tested in `large_scrape_produces_correct_plan` but not as proptest

---

### Proptest 6: Change consistency (Added/Removed/Modified field rules)

```
Invariant: In any plan:
  - Added entries always have old_hash == None, new_hash == Some(_)
  - Removed entries always have old_hash == Some(_), new_hash == None
  - Modified entries always have old_hash != None, new_hash != None, old_hash != new_hash
Strategy: Same as Proptest 4
Anti-invariant: None
```

**Status: GAP** — tested manually but not as proptest

---

## 5. Fuzz Targets

### Fuzz Target 1: ScrapeResult deserialization (diff_directories / read_manifest boundary)

```
Input type: bytes (raw file content)
Risk: Panics on malformed JSON, OOM on deeply nested structures,
      logic errors on unexpected field types
Corpus seeds:
  - Valid ScrapeResult with 3 pages
  - Empty JSON object {}
  - JSON array instead of object
  - Deeply nested structure (1000 levels)
  - Very long string values (1MB+ url field)
  - Missing required fields
  - Null values for non-nullable fields
  - Negative numbers for usize fields
  - Unicode edge cases (surrogate pairs, null bytes)
```

**Rationale**: `diff_directories` and `read_manifest` deserialize untrusted JSON from disk. A corrupted or malicious manifest.json must not panic.

---

### Fuzz Target 2: Snapshot deserialization (load_snapshot from cache)

```
Input type: bytes (cached snapshot data)
Risk: Panics on corrupted cache, type confusion on u128 deserialization,
      BTreeMap invariant violations
Corpus seeds:
  - Valid Snapshot JSON
  - Truncated JSON (incomplete bytes)
  - Snapshot with duplicate URL keys (BTreeMap should deduplicate)
  - content_hash as string instead of number
  - Extremely large pages map (stress BTreeMap)
```

**Rationale**: Cached snapshots may be corrupted by disk errors, partial writes, or version mismatches. The deserialization must be robust.

---

## 6. Kani Harnesses

### Kani Harness 1: Summary conservation proof

```
Property: For ALL possible Snapshot pairs (previous, current),
          compute_plan produces a plan where:
          added + modified + unchanged == current.pages.len()
Bound: 5 pages per snapshot (10 pages total — bounded model checking)
Rationale: This arithmetic invariant must NEVER be violated. Property testing
           can sample but Kani proves it exhaustively within the bound. A bug
           here would cause incorrect exit codes in CLI and misleading reports.
```

### Kani Harness 2: ChangeKind field consistency

```
Property: For ALL possible PageChange vectors produced by diff_snapshots,
          every Added entry has old_hash == None,
          every Removed entry has new_hash == None,
          every Modified entry has old_hash != new_hash.
Bound: 5 changes per vector
Rationale: Field consistency is critical for downstream consumers (format_plan_markdown,
           JSON output). An inconsistent PageChange would produce misleading reports.
```

---

## 7. Mutation Testing Checkpoints

Target: **≥90% mutation kill rate**

### Critical mutations that MUST be caught

| Mutation | Location | Catching Test |
|----------|----------|---------------|
| `== 0` → `!= 0` in `is_empty()` | `watch.rs:118` | `change_summary_is_empty_when_all_zero` |
| `&&` → `||` in `is_empty()` | `watch.rs:119` | `change_summary_not_empty_when_added`, `..._removed`, `..._modified` |
| `Added` → `Removed` in match arm | `watch.rs:63` | `change_kind_display_formats_lowercase` |
| Remove `None` arm in `diff_snapshots` first loop | `watch.rs:299` | `compute_plan_detects_added_page` |
| Remove `Some(_)` unchanged arm | `watch.rs:296` | `compute_plan_empty_on_identical_content` |
| `!=` → `==` in hash comparison | `watch.rs:287` | `compute_plan_detects_modified_page` |
| Remove second loop (removed check) | `watch.rs:311-322` | `compute_plan_detects_removed_page` |
| Remove `sort_by` call | `watch.rs:324-329` | `changes_sorted_by_kind_then_url` (GAP) |
| `plan.changes.is_empty()` early return in markdown | `watch.rs:148-151` | `markdown_report_empty_plan_shows_up_to_date` |
| Remove `### Added` section in markdown | `watch.rs:171-177` | `markdown_report_contains_all_sections` |
| `create_dir_all` → no-op | `watch.rs:400` | `write_plan_reports_creates_nested_directories` |
| `format_plan_json` returns empty string | `watch.rs:207` | `json_report_roundtrips` |
| Summary count calculation off-by-one | `watch.rs:260` | `large_scrape_produces_correct_plan`, `summary_conservation_holds` (GAP) |
| `process::exit(0)` → `process::exit(1)` when empty | `cmd/watch.rs:62` | E2E test (GAP) |
| `process::exit(1)` → `process::exit(0)` when changes | `cmd/watch.rs:62` | E2E test (GAP) |

### Mutation gap analysis

3 mutations are currently **NOT caught** by existing tests:
1. **sort_by removal** — no test verifies change ordering
2. **summary conservation** — no proptest covers this
3. **CLI exit codes** — no E2E test verifies exit status

---

## 8. Combinatorial Coverage Matrix

### Group A: compute_plan (pure calc — unit layer)

| Scenario | Previous Pages | Current Pages | Expected Summary | Layer |
|----------|---------------|---------------|------------------|-------|
| identical content | [a,b] | [a,b] (same hashes) | added=0, removed=0, modified=0, unchanged=2 | unit |
| single added | [a] | [a,b] | added=1, removed=0, modified=0, unchanged=1 | unit |
| single removed | [a,b] | [a] | added=0, removed=1, modified=0, unchanged=1 | unit |
| single modified | [a] (hash H1) | [a] (hash H2) | added=0, removed=0, modified=1, unchanged=0 | unit |
| first scrape (empty prev) | [] | [a,b,c] | added=3, removed=0 | unit |
| complete removal | [a,b] | [] | added=0, removed=2 | unit |
| both empty | [] | [] | all 0, changes empty | unit |
| url change | [old] | [new] | added=1, removed=1, modified=0 | unit |
| title change only | [a: "Old"] (same hash) | [a: "New"] (same hash) | modified=0, unchanged=1 | unit |
| mixed changes | [a,b,c] | [a',d,c] | added=1, removed=1, modified=1, unchanged=1 | unit |
| large scale | 100 pages | 95 pages (mixed) | conservation holds | integration |
| invariant: conservation | any | any | added+modified+unchanged == total_current | proptest |
| invariant: idempotency | any | any | plan₁ == plan₂ | proptest |

### Group B: format_plan_markdown (pure calc — unit layer)

| Scenario | Plan State | Expected Output | Layer |
|----------|-----------|-----------------|-------|
| empty plan | no changes | "No changes detected. The documentation is up to date." | unit |
| all three kinds | 1A + 1R + 1M | Contains "### Added", "### Removed", "### Modified" | unit |
| only additions | 2A, 0R, 0M | Contains "### Added", NOT "### Removed", NOT "### Modified" | unit |
| only removals | 0A, 1R, 0M | Contains "### Removed", NOT "### Added" | unit |
| correct format | 1A | "+ `url` — title" format | unit |
| footer | any plan | Contains "Run `ctd apply`" | unit |

### Group C: format_plan_json (pure calc — unit layer)

| Scenario | Plan State | Expected Output | Layer |
|----------|-----------|-----------------|-------|
| roundtrip | mixed changes | serialize → deserialize matches | unit |
| empty plan | no changes | valid JSON, changes: [] | unit |
| unicode content | unicode URLs | JSON contains correct unicode strings | integration |

### Group D: diff_directories (I/O boundary — integration layer)

| Scenario | dir_a | dir_b | Expected Outcome | Layer |
|----------|-------|-------|------------------|-------|
| valid mixed changes | manifest with [a,b] | manifest with [a',c] | Ok(plan): 1A, 1R, 1M | integration |
| identical | manifest [a] | manifest [a] | Ok(plan): empty changes | integration |
| missing manifest in dir_a | no manifest | valid manifest | Err("Cannot read", path_a) | integration |
| missing manifest in dir_b | valid manifest | no manifest | Err("Cannot read", path_b) | integration |
| invalid JSON in dir_a | "not json" | valid manifest | Err("Invalid manifest", path_a) | integration |
| invalid JSON in dir_b | valid manifest | "not json" | Err("Invalid manifest", path_b) | integration |

### Group E: write_plan_reports (I/O boundary — integration layer)

| Scenario | Output Dir | Expected Outcome | Layer |
|----------|-----------|------------------|-------|
| dir exists | existing dir | Ok(()), both files exist | integration |
| nested dir missing | deeply/nested/output | Ok(()), dirs created, files exist | integration |
| read-only dir (GAP) | read-only dir | Err(I/O permission error) | integration |

### Group F: ChangeSummary::is_empty (pure predicate — unit layer)

| Scenario | added | removed | modified | Expected | Layer |
|----------|-------|---------|----------|----------|-------|
| all zero | 0 | 0 | 0 | true | unit |
| added non-zero | 1 | 0 | 0 | false | unit |
| removed non-zero | 0 | 1 | 0 | false | unit |
| modified non-zero | 0 | 0 | 1 | false | unit |
| all non-zero | 1 | 1 | 1 | false | unit |

### Group G: ChangeKind::Display (pure formatting — unit layer)

| Scenario | Variant | Expected String | Layer |
|----------|---------|-----------------|-------|
| Added | ChangeKind::Added | "added" | unit |
| Removed | ChangeKind::Removed | "removed" | unit |
| Modified | ChangeKind::Modified | "modified" | unit |

---

## Open Questions

1. **`run_watch` / `run_apply` / `run_diff` E2E testing**: These CLI handlers call `process::exit()`. Testing them requires either:
   - Refactoring to return exit codes instead of calling `process::exit()` directly (preferred — enables unit testing)
   - Spawning subprocesses in integration tests (slow, fragile)
   - Using `std::panic::catch_unwind` around `process::exit` (hacky, platform-dependent)
   
   **Recommendation**: Extract a `run_watch_inner() -> Result<ExitCode>` that the public `run_watch` wraps. Test the inner function.

2. **`diff_snapshots` is private**: The sorting behavior (B14) cannot be tested directly. It is tested indirectly through `compute_plan`, but a dedicated ordering test would be cleaner. Consider either making it `pub(crate)` or testing ordering through the public API.

3. **`build_scrape_config` in cmd/watch.rs**: This pure function has no tests. It should have at least one test verifying that filter, delay, timeout, retry, and concurrency parameters are correctly mapped to ScrapeConfig fields.

4. **Error message format in `diff_directories`**: The error messages include filesystem paths. Tests should verify the path appears in the error message but not assert the exact format (to avoid brittleness across platforms).

5. **`ChangeKind` serde rename**: `#[serde(rename_all = "snake_case")]` means JSON serialization uses "added"/"removed"/"modified". No test verifies the JSON field names — only the Display impl. Add a serde-specific test.

---

## Gap Summary

| Category | Gap | Severity |
|----------|-----|----------|
| B14 — changes sorted by kind then URL | No test verifies ordering | Medium |
| B26/B27 — ChangeSummary::is_empty dedicated tests | Only tested inline | Low |
| B28 — ChangeKind::Display | No test at all | Low |
| B18 — markdown omits empty sections | No test for omission | Low |
| B23 — dir_a missing manifest | Only dir_b case tested | Medium |
| B24 — invalid manifest in dir_a | Only dir_b case tested | Medium |
| Proptest 4 — compute_plan idempotency | Manual test only | Medium |
| Proptest 5 — summary conservation | Manual test only | High |
| Proptest 6 — change consistency | Manual test only | Medium |
| Fuzz targets | None exist | High |
| Kani harnesses | None exist | Medium |
| CLI exit codes | No E2E test | High |
| Read-only directory error path | No test | Low |
| build_scrape_config | No test | Low |
| ChangeKind serde field names | No test | Low |
