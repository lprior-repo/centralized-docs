# Test Plan Review: MCP Server with Official rmcp SDK

```
bead_id: cdocs-drj
reviewed_at: 2026-03-28
reviewer: test-inquisitor
mode: 1 — Plan Inquisition
attempt: 3 (re-audit retry 2 after claimed fixes)
status: APPROVED
```

---

## VERDICT: APPROVED

**0 LETHAL | 2 MAJOR | 4 MINOR**

Below all rejection thresholds (LETHAL≥1, MAJOR≥3, MINOR≥5). The plan is cleared for implementation.

| Axis | Verdict | Summary |
|------|---------|---------|
| 1 — Contract Parity | PASS | All 6 pub fns covered. All 7 error variants have exact-assertion scenarios. |
| 2 — Assertion Sharpness | PASS | Zero banned patterns. All Then-clauses concrete. |
| 3 — Trophy Allocation | PASS | 10.7× density. 6 proptests. 3 fuzz targets. Healthy layer ratio. |
| 4 — Boundary Completeness | PASS | All boundaries named per function. |
| 5 — Mutation Survivability | PASS | All 21 identified mutations caught by named tests. |
| 6 — Holzmann Plan Audit | PASS | Rules 2, 4, 5, 7, 8, 9 satisfied. |

---

## Previous Mandate Verification

All 7 mandates from round 2 (retry 1) confirmed resolved. This time the fixes landed in the right place — Section 3 BDD Scenarios, not just Section 1 Behavior Inventory:

| # | Mandate | Status | Evidence |
|---|---------|--------|----------|
| 1 | Line 292 default limit says "exactly 10" | FIXED | Line 292: `"exactly 10 result entries (Given 15 docs, must return exactly 10)"` — no "or fewer" |
| 2 | limit=100 BDD scenario in Section 3 | FIXED | Lines 296-304: full Given/When/Then with test fn `search_docs_accepts_limit_100_as_valid_max`, 120 docs, asserts exactly 100 |
| 3 | spawn_blocking BDD scenario in Section 3 | FIXED | Lines 306-314: full Given/When/Then with test fn `search_docs_does_not_block_async_runtime`, concurrent timeout |
| 4 | CtdMcpServer::new boundary BDD scenarios | FIXED | Lines 199-213: 3 scenarios with test fns — `new_returns_error_for_empty_string_path`, `new_handles_permission_denied_dir`, `new_handles_symlink_directory` |
| 5 | find_chunk_content/find_doc_summary duplicate+empty | FIXED | Lines 576-637: 8 new BDD scenarios covering empty arrays, duplicate ids, empty content/summary |
| 6 | Summary counts updated | FIXED | Line 13: `36 unit / 22 integration / 4 e2e / 2 static` |
| 7 | TestIndex::build_on_disk() renamed | FIXED | Line 993: `build_on_disk() -> (TempDir, PathBuf)` |

The pattern from the previous review — updating Section 1 without touching Section 3 — is **broken**. All mandates now have full BDD scenarios with Given/When/Then blocks and test function names in the right section.

---

## Axis 1 — Contract Parity

### Public Functions → BDD Scenario Coverage

| pub fn | BDD Scenarios | Lines | Status |
|--------|---------------|-------|--------|
| `CtdMcpServer::new()` | 7 scenarios | 157-213 | PASS |
| `info()` | 1 scenario | 219-227 | PASS |
| `search_docs()` | 13 scenarios | 232-350 | PASS |
| `read_chunk()` | 8 scenarios | 356-424 | PASS |
| `get_related_concepts()` | 7 scenarios | 430-489 | PASS |
| `run()` | 2 scenarios | 515-531 | PASS |

**Result: PASS** — All 6 public functions have ≥1 BDD scenario.

### Error Variant → Exact Assertion Coverage

| Error Variant | Scenario Asserting Exact Variant | Status |
|---------------|----------------------------------|--------|
| `IndexNotFound` | Lines 316-323, 400-406, 475-481 (tool handlers) + line 503 (error mapping) | PASS |
| `IndexCorrupted` | Lines 325-332, 408-414, 483-489 (tool handlers) + line 504 (error mapping) | PASS |
| `InvalidInput` | Lines 254-285, 384-398, 459-473 (param validation) + line 505 (error mapping) | PASS |
| `SearchIndexError` | Line 506 (error mapping — constructs variant, asserts code -32603 and message content) | PASS |
| `QueryError` | Line 507 (error mapping — constructs variant, asserts code -32603 and message content) | PASS |
| `IoError` | Lines 169-172 (`new` failure), 529-531 (`run` failure) + line 508 (error mapping) | PASS |
| `Internal` | Line 509 (error mapping — constructs variant, asserts code -32603 and message content) | PASS |

Every variant has ≥1 scenario asserting the exact variant by name. The error mapping section (lines 503-509) provides a guaranteed floor of coverage for all 7 variants. Tool-handler-level scenarios provide integration coverage for the most commonly triggered variants (IndexNotFound, IndexCorrupted, InvalidInput, IoError).

**Result: PASS**

---

## Axis 2 — Assertion Sharpness

Full scan of every "Then:" clause across all 60 BDD scenarios:

| Anti-pattern | Count | Status |
|-------------|-------|--------|
| `is_ok()` as sole assertion | 0 | CLEAN |
| `is_err()` as sole assertion | 0 | CLEAN |
| `> 0` without concrete value | 0 | CLEAN |
| `Some(_)` without inner value | 0 | CLEAN |

**Verified concrete assertions (sampling):**

- Line 292: `"exactly 10 result entries"` — exact integer, no qualifiers ✅
- Line 301: `"exactly 100 result entries"` — exact integer ✅
- Line 250: `text equals "No results found."` — exact string ✅
- Line 361: `text equals "This is chunk ABC verbatim."` — exact string ✅
- Line 371: `text equals "Document doc-123:\nSummary:\nSummary of doc 123."` — exact formatted string ✅
- Line 380: `text equals "ID 'nonexistent-xyz' not found in chunks or documents"` — exact string ✅
- Line 422: `text equals "chunk content"` — exact string, disambiguated from doc summary ✅
- Line 455: `text equals "No related concepts found for ID 'orphan-node'"` — exact string ✅
- Line 563: `Some("hello")` — concrete inner value ✅
- Line 571: `None` — exact ✅
- Line 595: `Some("")` — concrete empty string ✅
- Line 587: `Some(first_content) — first match wins` — specifies which value ✅
- Lines 320-321: `error code equals -32603` — exact integer ✅
- Line 340: `score at position N is >= score at position N+1` — concrete ordering property ✅

**Result: PASS** — Zero banned assertion patterns.

---

## Axis 3 — Trophy Allocation

### Density

- Public functions: 6
- Total planned tests: 64 (36 unit / 22 integration / 4 e2e / 2 static)
- Ratio: 64 / 6 = **10.7×** (target ≥5×)

### Proptest Invariant Coverage

| Pure Function | Proptest | Status |
|---------------|----------|--------|
| `find_chunk_content` | Lines 674-688: lookup consistency | PASS |
| `find_doc_summary` | Lines 690-702: lookup consistency | PASS |
| `find_related_edges` | Lines 704-715: bidirectional completeness | PASS |
| `format_search_results` | Lines 718-730: output structure invariants | PASS |
| Error mapping `From` impl | Lines 746-755: message preservation | PASS |
| Idempotent tool calls | Lines 732-743: byte-identical repeated calls | PASS |

### Fuzz Target Coverage

| Parser | Fuzz Target | Status |
|--------|-------------|--------|
| `load_index_json` (JSON) | Lines 762-775: arbitrary bytes, 8 seeds | PASS |
| `search_docs` query (Tantivy) | Lines 777-791: arbitrary &str, 8 seeds | PASS |
| `find_related_edges` (graph) | Lines 793-805: arbitrary (Value, &str), 6 seeds | PASS |

**Result: PASS**

---

## Axis 4 — Boundary Completeness

### CtdMcpServer::new

| Boundary | Scenario | Status |
|----------|----------|--------|
| Valid directory | Line 157 | ✅ |
| Missing directory | Line 166 | ✅ |
| Path is file | Line 176 | ✅ |
| Relative path → absolute | Line 184 | ✅ |
| Empty string path | Line 193 | ✅ |
| Permission-denied | Line 199 | ✅ |
| Symlink to directory | Line 208 | ✅ |

### search_docs

| Boundary | Scenario | Status |
|----------|----------|--------|
| Empty query | Line 254 | ✅ |
| Whitespace query | Line 263 | ✅ |
| Single char query | Matrix line 889 | ✅ |
| limit = 0 | Line 273 | ✅ |
| limit = 1 | Matrix line 885 | ✅ |
| limit = 10 (default) | Line 287 | ✅ |
| limit = 100 (max) | Line 296 | ✅ |
| limit = 101 (one above max) | Line 280 | ✅ |
| INDEX.json missing | Line 316 | ✅ |
| INDEX.json corrupted | Line 325 | ✅ |

### read_chunk

| Boundary | Scenario | Status |
|----------|----------|--------|
| chunk_id match | Line 356 | ✅ |
| doc_id match | Line 366 | ✅ |
| No match | Line 375 | ✅ |
| Shared id (chunk wins) | Line 416 | ✅ |
| Empty id | Line 384 | ✅ |
| Whitespace id | Line 393 | ✅ |
| INDEX.json missing | Line 400 | ✅ |
| INDEX.json corrupted | Line 408 | ✅ |

### get_related_concepts

| Boundary | Scenario | Status |
|----------|----------|--------|
| id as `from` | Line 430 | ✅ |
| id as `to` (inbound) | Line 440 | ✅ |
| No edges | Line 451 | ✅ |
| Empty id | Line 459 | ✅ |
| Whitespace id | Line 467 | ✅ |
| INDEX.json missing | Line 475 | ✅ |
| INDEX.json corrupted | Line 483 | ✅ |

### Internal Pure Functions

All 15 internal helper test scenarios explicitly test: found, not found, empty collection, duplicate ids, empty content/summary, missing keys. See lines 537-668.

**Result: PASS**

---

## Axis 5 — Mutation Survivability

All mutations from the previous review's lethal findings are now killed:

| Mutation | Catching Test | Killed? |
|----------|---------------|---------|
| `limit <= 100` → `limit < 100` | `search_docs_accepts_limit_100_as_valid_max` (line 304) — limit=100 must succeed | ✅ KILLED |
| `default_limit()` 10 → 5 | `search_docs_defaults_limit_to_10` (line 294) — Given 15 docs, expects exactly 10 | ✅ KILLED |
| Remove `spawn_blocking` | `search_docs_does_not_block_async_runtime` (line 314) — concurrent timeout must complete | ✅ KILLED |

Full mutation table verification (21 mutations from lines 837-860):

| # | Mutation | Catching Test | Status |
|---|----------|---------------|--------|
| 1 | Remove query.trim().is_empty() | `search_docs_returns_invalid_input_when_query_whitespace` | ✅ |
| 2 | Remove limit > 0 check | `search_docs_returns_invalid_input_when_limit_zero` | ✅ |
| 3 | limit <= 100 → limit < 100 | `search_docs_accepts_limit_100_as_valid_max` | ✅ |
| 4 | Remove IndexNotFound check | `search_docs_returns_index_not_found_when_json_missing` | ✅ |
| 5 | Swap chunk/doc order | `read_chunk_prefers_chunk_match_over_doc_match` | ✅ |
| 6 | Remove `to` edge matching | `get_related_returns_inbound_edges_when_id_is_target` | ✅ |
| 7 | Remove `from` edge matching | `get_related_returns_from_edges_when_id_is_source` | ✅ |
| 8 | Change error code -32603 | All 7 `error_map_*` tests | ✅ |
| 9 | Remove "No results found." | `search_docs_returns_no_results_message_when_no_match` | ✅ |
| 10 | Remove "not found" branch | `read_chunk_returns_not_found_when_id_matches_nothing` | ✅ |
| 11 | Remove "No related concepts" | `get_related_returns_empty_message_when_no_edges` | ✅ |
| 12 | Change default 10 → anything | `search_docs_defaults_limit_to_10` | ✅ |
| 13 | Remove canonicalization | `new_canonicalizes_relative_path` | ✅ |
| 14 | Skip empty id in read_chunk | `read_chunk_returns_invalid_input_when_id_empty` | ✅ |
| 15 | Skip empty id in get_related | `get_related_returns_invalid_input_when_id_empty` | ✅ |
| 16 | Remove "inbound" label | `get_related_returns_inbound_edges_when_id_is_target` | ✅ |
| 17 | Flip sort to ascending | `search_docs_results_sorted_by_score_descending` | ✅ |
| 18 | Remove spawn_blocking | `search_docs_does_not_block_async_runtime` | ✅ |
| 19 | Remove null graph.edges handling | `find_related_edges_returns_empty_when_graph_missing` | ✅ |
| 20 | Flip new dir existence check | `new_returns_io_error_when_dir_missing` | ✅ |
| 21 | Return last instead of first (chunks) | `find_chunk_content_returns_first_match_for_duplicate_ids` | ✅ |

**Result: PASS** — 21/21 mutations killed (100%).

---

## Axis 6 — Holzmann Plan Audit

| Rule | Assessment | Status |
|------|-----------|--------|
| R1 — Linear | Each test follows single Given→When→Then flow. No nested conditionals. | PASS |
| R2 — Bound Every Loop | No loops in test bodies. Proptest strategies bounded (0..20). | PASS |
| R4 — One Function, One Job | Each test name describes one behavior. Bodies ≤20 lines (in plan). | PASS |
| R5 — State Your Assumptions | Every scenario has explicit `Given:` with preconditions. TestIndex builder names side effects (`build_on_disk`, `build_with_tantivy`). | PASS |
| R7 — Narrow Your State | Per-test TempDir. No shared mutable state. `flavor = "current_thread"` specified. | PASS |
| R8 — Surface Side Effects | `build_on_disk()` and `build_with_tantivy()` clearly advertise I/O. | PASS |
| R9 — One Layer of Magic | TestIndex builder is single-layer. No helper chains >1 deep. | PASS |

**Result: PASS**

---

## MAJOR FINDINGS (2 — threshold ≥3 for rejection)

### MAJOR-1: Missing integration BDD scenarios for SearchIndexError and QueryError production paths

**Behaviors:** 17, 18 (inventory lines 63-64)
**Trophy table:** Claims integration tests (lines 121-122)
**Section 3 BDD scenarios:** NONE

The behavior inventory correctly identifies:
- Line 63: `search_docs returns Err(CtdMcpError::SearchIndexError) when Tantivy index cannot be opened or rebuilt`
- Line 64: `search_docs returns Err(CtdMcpError::QueryError) when Tantivy cannot parse the query`

The trophy table (lines 121-122) maps both to "integration" layer. But Section 3.3 (lines 230-351) has 13 BDD scenarios for search_docs, and NONE of them cover SearchIndexError or QueryError. After the IndexCorrupted scenario (line 332), the next scenario jumps to "results sorted by score" (line 334). There is no heading for "Behavior: search_docs SearchIndexError" or "Behavior: search_docs QueryError."

**What exists:** Unit-level error mapping tests (section 3.6, lines 506-507) that construct each variant directly and assert `code == -32603` and message content. These verify the `From<CtdMcpError>` conversion is correct in isolation.

**What's missing:** Integration-level Given/When/Then scenarios that trigger a real Tantivy failure and verify the error propagates correctly through the tool handler. Without these, a bug in the error-mapping path between `open_or_rebuild_search_index` → tool handler → McpError would go undetected. Example missing scenario:

```
Given: a CtdMcpServer whose index_dir contains valid INDEX.json
And:   the .tantivy_index/ directory contains corrupted/unreadable files
When:  search_docs(SearchDocsParams { query: "test", limit: 10 }) is called
Then:  Err(McpError) where message matches "Search index error"
And:   error code equals -32603
```

**Severity rationale:** The error mapping unit tests satisfy the letter of Axis 1 (every variant has an exact-assertion scenario). But the integration production path is untested. MAJOR, not LETHAL, because the variant IS tested in isolation — just not through the full handler chain.

### MAJOR-2: Phantom IoError behaviors for read_chunk and get_related_concepts

**Behaviors:** 28 (line 77), 36 (line 88)
**Trophy table:** Claims integration tests (lines 132, 140)
**Section 3 BDD scenarios:** NONE

The inventory lists:
- Line 77: `read_chunk returns Err(CtdMcpError::IoError) when INDEX.json file read fails`
- Line 88: `get_related_concepts returns Err(CtdMcpError::IoError) when INDEX.json file read fails`

Neither has a BDD scenario in sections 3.4 or 3.5.

**Architectural concern:** The contract's `load_index_json` spec (contract.md lines 330-332) only documents two error paths:
```
# Errors
- CtdMcpError::IndexNotFound if file missing
- CtdMcpError::IndexCorrupted if JSON invalid
```

Both `read_chunk` and `get_related_concepts` call `load_index_json` as their sole I/O operation. The contract documents no path from `load_index_json` to `IoError`. The error mapping rules table (contract.md lines 156-167) lists `std::io::Error during file reads → IoError` but this is a general rule, not specific to `load_index_json`.

**Two interpretations:**

1. **`load_index_json` differentiates error kinds** (e.g., `NotFound → IndexNotFound`, `PermissionDenied → IoError`) — If so, the contract's error spec is incomplete and two BDD scenarios are missing.
2. **`IoError` is unreachable through these tools** — If so, behaviors 28/36 and trophy rows 28/36 are phantom entries inflating the test count.

Either way, the plan is internally inconsistent. The trophy table claims integration tests that don't exist, and the behaviors may be unimplementable as described.

**Resolution needed:** Either add BDD scenarios with a documented trigger for `IoError`, or remove these phantom behaviors from the inventory and trophy table.

---

## MINOR FINDINGS (4 — threshold ≥5 for rejection)

### MINOR-1: Duplicate behavior numbering (line 61-62)

Both lines use number `16`:
```
16. search_docs returns Err(CtdMcpError::IndexNotFound) when INDEX.json is missing
16. search_docs returns Err(CtdMcpError::IndexCorrupted) when INDEX.json is malformed JSON
```
Should be 16 and 17 (with subsequent numbers shifted). This creates ambiguity when cross-referencing inventory numbers to trophy table rows.

### MINOR-2: Near-duplicate behaviors 14a and 15

- Line 58 (14a): `accepts limit=100 as valid maximum and returns up to 100 results`
- Line 60 (15): `accepts limit=100 as valid maximum and returns exactly 100 results when ≥100 docs match`

Only one BDD scenario exists (lines 296-304) covering the "exactly 100" case, which subsumes the weaker "up to 100" claim. These should be merged or 14a should be removed to avoid confusion.

### MINOR-3: Graph edge edge-cases not in BDD scenarios

Two edge cases appear only in fuzz target seeds (lines 800-801), not in concrete BDD scenarios:
- Self-referencing edges (from == to == id) — fuzz seed line 801
- Edges with missing/null fields — fuzz seed line 800

The proptest for `find_related_edges` (lines 704-715) generates arbitrary edges which may include these cases, and the fuzz target explicitly seeds them. A concrete BDD scenario for each would make the expected behavior explicit (e.g., does a self-referencing edge produce two output lines — one outbound, one inbound?).

### MINOR-4: Test count inconsistency between header and trophy table

- Line 13 (summary header): `36 unit / 22 integration / 4 e2e / 2 static` = 64 total
- Line 148 (trophy table total): `14 unit / 22 integration / 4 e2e / 2 static` = 42 total

The header's "36 unit" presumably includes 15 internal helper tests + 7 error mapping tests + ~14 param validation tests that aren't individually listed in the trophy table. The trophy table counts only the direct behavior-to-layer mappings. Both numbers are defensible but the inconsistency is confusing for anyone trying to verify the total.

---

## Summary Scorecard

```
Axis 1 — Contract Parity:      PASS  (all pub fns + all error variants covered)
Axis 2 — Assertion Sharpness:  PASS  (zero banned patterns, all assertions concrete)
Axis 3 — Trophy Allocation:    PASS  (10.7× density, 6 proptests, 3 fuzz targets)
Axis 4 — Boundary Completeness:PASS  (all boundaries named per function)
Axis 5 — Mutation Survivability:PASS  (21/21 mutations killed = 100%)
Axis 6 — Holzmann Plan Audit:  PASS  (R2, R4, R5, R7, R8, R9 satisfied)
```

---

## MANDATE

**Status: APPROVED for implementation.** The 2 MAJOR and 4 MINOR findings are non-blocking but should be resolved during implementation:

1. **MAJOR-1 (SearchIndexError/QueryError integration scenarios):** Write two integration tests during implementation that trigger real Tantivy failures. The trophy table already claims these tests exist — the implementation should deliver them.

2. **MAJOR-2 (IoError phantom behaviors):** During implementation, either:
   - If `load_index_json` differentiates `PermissionDenied` from `NotFound`: update the contract's error spec to document this, and write the integration tests.
   - If `IoError` is unreachable through read_chunk/get_related: remove behaviors 28/36 and trophy rows 28/36 from the plan.

3. **MINOR findings (1-4):** Address in a plan revision pass — fix duplicate numbering, merge near-duplicate behaviors, add graph edge BDD scenarios, reconcile count inconsistency.
