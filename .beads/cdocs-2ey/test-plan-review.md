bead_id: cdocs-2ey
bead_title: action: wire scrape command to one shared read session and one shutdown commit
phase: state-1.7-test-plan-review
updated_at: 2026-04-04T00:00:00Z
review_round: 2

# Test Plan Review — Plan Inquisition (Mode 1) — Round 2

## VERDICT: APPROVED

---

## Previous Rejection Mandate Verification

All 12 mandates from Round 1 (test-plan-review.md:196-228) verified:

| # | Mandate | Addressed? | Evidence |
|---|---------|-----------|----------|
| 1 | `run_scrape_propagates_read_session_creation_failure` | **YES** | Behavior 43 (test-plan.md:587-601). Asserts Err propagation, BulkLoadError::StorageError wrapping, no silent swallow, no commit_changes call. Mutation checkpoint at line 849. |
| 2 | `run_scrape_propagates_load_scrapes_failure` | **YES** | Behavior 42 (test-plan.md:572-585). Asserts Err propagation, BulkLoadError wrapping, not silently swallowed. Mutation checkpoint at line 850. |
| 3 | `classify_scrape_diff_classifies_all_as_changed_when_all_hashes_differ` | **YES** | Behavior 8 (test-plan.md:196-218). 3 stored entries, 3 scraped pages, all hashes differ. Asserts changed.len()==3, exact URLs, empty new/unchanged. |
| 4 | `classify_scrape_diff_handles_zero_content_hash_boundary` | **YES** | Behavior 9 (test-plan.md:220-236). Stored [0u8;32], scraped with empty markdown (SHA-256("") is non-zero). Asserts changed. |
| 5 | `build_scrape_state_changes_handles_new_pages_only` | **YES** | Behavior 22 (test-plan.md:384-399). new=2, changed=0. Asserts updated_urls.len()==2, both URLs present, new_scrapes.len()==2, specific timestamp. Mutation checkpoint at line 851. |
| 6 | `build_scrape_state_changes_handles_changed_pages_only` | **YES** | Behavior 23 (test-plan.md:401-415). new=0, changed=2. Asserts updated_urls.len()==2, new_scrapes.len()==2. Mutation checkpoint at line 852. |
| 7 | `build_scrape_state_changes_handles_zero_timestamp` | **YES** | Behavior 24 (test-plan.md:417-430). timestamp=0. Asserts last_fetched_secs==0, correct content_hash, new_scrapes.len()==1. Mutation checkpoint at line 853. |
| 8 | Fix Behavior 15: ≥2 pages with distinct markdown, assert each independently | **YES** | Behavior 15 (test-plan.md:354-369). Uses 2 pages ("alpha content here" / "beta content here"). Asserts each page's content_hash == SHA-256 of its own markdown AND that they differ from each other. |
| 9 | Fix Behavior 47: concrete verification mechanism for "reused" | **YES** | Behavior 47 (test-plan.md:632-646). Now specifies "HTTP test server received exactly 0 additional requests (no pages re-fetched)" plus exact table entry counts. Behavior 37 (line 504-518) also fixed with exact composition (2 reused + 1 fresh, 1 updated_url, 1 new_scrape). Mutation checkpoints at lines 854-855. |
| 10 | Fix Behavior 2: assert specific URLs, not just count | **YES** | Behavior 2 (test-plan.md:135-152). Now asserts "result.new contains exactly ["https://a.com/page1", "https://b.com/page2", "https://c.com/page3"]" in addition to len()==3. |
| 11 | Contract alignment for error context strings | **YES** | Contract Alignment Note (test-plan.md:917-934) documents the gap, recommends specific context strings, and explicitly states the test plan approach: test propagation (not wrapping text), tighten assertions once implementation settles. |
| 12 | Resolve open question 4 (load_scrapes hash input) | **YES** | Resolved at test-plan.md:936-938. Documents that hash input is `stored_url_states[url].url_hash` per contract wiring flow (contract.md:111) and load_scrapes signature (contract.md:90). |

**All 12 mandates satisfied. Proceeding to full six-axis re-audit.**

---

## Axis 1 — Contract Parity

**PASS.**

### Error Taxonomy Cross-Reference

| Error Source (contract.md:62-73) | BDD Scenario | Status |
|---|---|---|
| `CommitError::DatabaseOpen` (StateDb::open) | Behavior 40 (line 548) | COVERED |
| `BulkLoadError::StorageError` (StateReadSession::new) | Behavior 43 (line 587) | COVERED |
| `StateLoadError` (load_url_states) | Behavior 41 (line 560) | COVERED |
| `BulkLoadError` (load_scrapes) | Behavior 42 (line 572) | COVERED |
| `CommitError::*` (commit failure) | Behavior 44 (line 521) | COVERED |
| `anyhow::Error` (pre-commit scrape failure) | Behavior 45 (line 534) | COVERED |

All 6 error sources have BDD scenarios with concrete assertions.

### Function Coverage

| Public Function | BDD Scenarios | Status |
|---|---|---|
| `classify_scrape_diff` (new) | Behaviors 1-5, 8-11 (9 scenarios) | COVERED |
| `build_scrape_state_changes` (new) | Behaviors 12, 14-18, 21-26 (12 scenarios) | COVERED |
| `run_scrape` (modified wiring) | Behaviors 29-30, 34, 37, 39-45 (11 scenarios) | COVERED |
| `hash_payload` (existing, regression) | Behavior 27 | COVERED |
| `scrape_result_to_persisted` (existing, regression) | Behavior 28 | COVERED |

No `pub fn` without a BDD scenario. No error variant without an assertion. No `is_err()` as the sole assertion (Behaviors 42/43 assert propagation with specific error type wrapping).

---

## Axis 2 — Assertion Sharpness

**PASS.**

Every "Then:" clause audited. Results:

- **Concrete URL lists**: Behaviors 1, 2, 8, 9, 10, 11, 22, 23 — assert specific URL strings. PASS.
- **Concrete counts**: Behaviors 2, 3, 8, 12, 22, 23, 24, 25, 26, 29, 37, 47 — assert exact integer values. PASS.
- **Concrete hashes**: Behaviors 9, 11, 15, 24, 25, 26 — assert against `SHA-256(specific_bytes)`. PASS.
- **Concrete error context strings**: Behaviors 40, 41, 44 — assert exact substring match in error chain. PASS.
- **Concrete exit codes**: Behaviors 46, 47, 48 — assert `== 0` or `!= 0`. PASS.
- **Concrete HTTP request count**: Behavior 47 — assert `== 0`. PASS.
- **Error propagation (anyhow)**: Behaviors 42, 43 — assert specific error type in chain (`BulkLoadError`, `BulkLoadError::StorageError`), assert NOT silently swallowed, assert no downstream side effects. Adequate for `anyhow::Error` where exact variant matching requires downcasting. PASS.
- **Reference integrity**: Behavior 18 — iterates `updated_urls` and checks every non-zero `url_hash` has matching `new_scrapes` entry. PASS.
- **Non-zero hash invariant**: Behavior 17 — asserts `*hash != [0u8; 32]` for all entries. PASS.

Zero instances of `is_ok()`, `is_err()` without specifics, `> 0`, or `Some(_)` without inner value.

---

## Axis 3 — Trophy Allocation

**PASS.**

Effective `pub fn` count: 3 (`classify_scrape_diff`, `build_scrape_state_changes`, `run_scrape`).

Test count breakdown:
- BDD scenarios: 35
- Proptest invariants: 6
- Fuzz targets: 3
- Kani harnesses: 3
- Static assertions: 6
- **Total: 53**

Ratio: 53 / 3 = **17.7x** (target ≥5x). PASS.

Proptest coverage for pure functions:
- `classify_scrape_diff`: 2 proptests (partition exclusivity + exhaustiveness, hash equivalence). PASS.
- `build_scrape_state_changes`: 4 proptests (determinism, hash-key correctness, output counts, field emptiness). PASS.
- `hash_payload`: 1 proptest (SHA-256 determinism + collision resistance). PASS.

Fuzz targets: 3 (classify_scrape_diff with arbitrary URLs, build_scrape_state_changes with arbitrary data, UrlStateRaw round-trip). No parser/deserializer introduced. PASS.

Layer distribution: ~22 unit / ~11 integration / 3 e2e / 6 static / 6 proptest / 3 fuzz / 3 Kani. Unit-heavy ratio is justified by 2 new pure functions demanding exhaustive boundary coverage. Integration covers all 6 error propagation paths + 5 wiring paths. E2E covers 3 command lifecycle scenarios. Balanced. PASS.

---

## Axis 4 — Boundary Completeness

### classify_scrape_diff

| Boundary | Covered? | By |
|---|---|---|
| Empty stored_url_states | YES | Behavior 2 |
| Empty scraped_pages | YES | Behavior 4 |
| Both empty | YES | Behavior 5 |
| Mixed (new + changed + unchanged) | YES | Behavior 1 |
| All unchanged | YES | Behavior 3 |
| All changed | YES | Behavior 8 (NEW) |
| content_hash == [0u8; 32] (zero hash) | YES | Behavior 9 (NEW) |
| content_hash == [1u8; 32] (non-zero boundary) | YES | Behavior 11 (NEW) |
| Partial URL overlap | YES | Behavior 10 (NEW) |

**9/9 boundaries covered. PASS.**

### build_scrape_state_changes

| Boundary | Covered? | By |
|---|---|---|
| Mixed (new + changed + unchanged) | YES | Behavior 12 |
| Unchanged only (empty output) | YES | Behavior 14 |
| New pages only | YES | Behavior 22 (NEW) |
| Changed pages only | YES | Behavior 23 (NEW) |
| Empty ScrapeDiff | YES | Behavior 14 |
| timestamp = 0 | YES | Behavior 24 (NEW) |
| timestamp = u64::MAX | YES | Behavior 25 (NEW) |
| Empty markdown | YES | Behavior 26 (NEW) |
| Content_hash correctness (≥2 pages) | YES | Behavior 15 (FIXED) |
| Unique keys (INV-5) | YES | Behavior 16 |
| Reference integrity (INV-6) | YES | Behavior 18 |
| Non-zero hash keys (INV-7) | YES | Behavior 17 |
| Persisted bytes round-trip | YES | Behavior 21 |

**13/13 boundaries covered. PASS.**

### run_scrape

| Boundary | Covered? | By |
|---|---|---|
| First run (no state) | YES | Behavior 29 |
| Second run (unchanged pages reused) | YES | Behavior 37 |
| StateReadSession::new failure | YES | Behavior 43 (NEW) |
| load_scrapes failure | YES | Behavior 42 (NEW) |
| StateDb::open failure | YES | Behavior 40 |
| load_url_states failure | YES | Behavior 41 |
| Commit failure | YES | Behavior 44 |
| Pre-commit scrape failure (state intact) | YES | Behavior 45 |
| No per-page writes (INV-1) | YES | Behavior 39 |
| Read session drop before commit (INV-3) | YES | Behavior 34 |
| Exactly one read session (POST-1) | YES | Behavior 30 |

**11/11 boundaries covered. PASS.**

---

## Axis 5 — Mutation Survivability

**PASS.**

24 mutation checkpoints documented (test-plan.md:830-855). Mental application of each:

| # | Mutation | Caught By | Verdict |
|---|----------|-----------|---------|
| 1 | `==` to `!=` in content_hash comparison | Behavior 3 (all-unchanged would become all-changed) | CAUGHT |
| 2 | Remove content_hash check (always New) | Behavior 3 (unchanged would be empty) | CAUGHT |
| 3 | Remove content_hash check (always Changed) | Behavior 8 (new would be empty) | CAUGHT |
| 4 | Return wrong URLs with correct count | Behavior 2 (asserts specific URLs) | CAUGHT |
| 5 | Treat [0u8;32] as sentinel | Behavior 9 (zero-hash would be skipped) | CAUGHT |
| 6 | Remove timestamp assignment | Behavior 12 (asserts specific timestamp) | CAUGHT |
| 7 | Skip unchanged exclusion | Behavior 14 (asserts all empty) | CAUGHT |
| 8 | Zero out url_hash | Behavior 17 (asserts non-zero) | CAUGHT |
| 9 | Remove reference integrity check | Behavior 18 (asserts url_hash in new_scrapes) | CAUGHT |
| 10 | Assign wrong page's content_hash | Behavior 15 (≥2 pages, independent assertions) | CAUGHT |
| 11 | Drop commit_changes call | Behavior 30 (asserts called exactly once) | CAUGHT |
| 12 | Remove read session drop before commit | Behavior 34 (commit would fail if txn held) | CAUGHT |
| 13 | Swallow commit error | Behavior 44 (asserts Err propagation) | CAUGHT |
| 14 | Remove error context string | Behavior 40 (asserts "failed to open state database") | CAUGHT |
| 15 | Write per-page instead of batching | Behavior 39 (asserts exactly one commit) | CAUGHT |
| 16 | Allow duplicate URL keys | Behavior 16 (asserts unique count) | CAUGHT |
| 17 | Return empty ScrapeDiff for empty stored | Behavior 2 (asserts new.len()==3) | CAUGHT |
| 18 | Swallow StateReadSession::new error | Behavior 43 (asserts Err + no commit) | CAUGHT |
| 19 | Swallow load_scrapes error | Behavior 42 (asserts Err + not swallowed) | CAUGHT |
| 20 | Return empty for new-only ScrapeDiff | Behavior 22 (asserts updated_urls.len()==2) | CAUGHT |
| 21 | Return empty for changed-only ScrapeDiff | Behavior 23 (asserts updated_urls.len()==2) | CAUGHT |
| 22 | Timestamp uses system clock | Behavior 24 (asserts last_fetched_secs==0) | CAUGHT |
| 23 | Return wrong page count (missing reused) | Behavior 37 (asserts exact 3-page composition) | CAUGHT |
| 24 | Re-fetch unchanged pages from network | Behavior 47 (asserts HTTP requests==0) | CAUGHT |

**Additional mutations verified via proptest/Kani:**

| Mutation | Caught By |
|----------|-----------|
| Page URL in two partitions simultaneously | Proptest 1 (partition exclusivity) |
| Page URL lost (not in any partition) | Proptest 1 (collective exhaustiveness) + Kani Harness 2 |
| Wrong hash for new_scrapes key | Proptest 4 (hash == SHA-256 of value bytes) |
| Wrong status_code (not 200) | Behavior 12 (asserts status_code==200) |
| Non-deterministic output | Proptest 3 (determinism invariant) |
| updated_files not empty | Proptest 5 (asserts len()==0) |
| UrlStateRaw byte layout corruption | Fuzz Target 3 + Kani Harness 1 |

**All critical mutations covered. 24 explicit checkpoints + 7 proptest/Kani invariants. PASS.**

---

## Axis 6 — Holzmann Plan Audit

**PASS.**

| Rule | Status | Evidence |
|------|--------|----------|
| Rule 1 (Linear flow) | PASS | All 35 BDD scenarios follow Given→When→Then with no nested conditionals in test bodies. |
| Rule 2 (Bound loops) | PASS | Zero loops in test scenarios. All iteration via rstest cartesian products or proptest strategies with explicit bounds (0..20, 0..3). |
| Rule 4 (One test, one job) | PASS | Each BDD scenario tests one behavior. Test function names describe the single assertion. |
| Rule 5 (State assumptions) | PASS | Every scenario has explicit `Given` block with concrete preconditions (exact URLs, exact timestamps, exact hash values, specific state configurations). |
| Rule 7 (Narrow state) | PASS | Integration tests create fresh tempdir per test. No shared mutable state between tests. |
| Rule 8 (Surface side effects) | PASS | Side effects explicitly named: "test server or mock transport" (line 467), "state.redb exists from prior run" (line 508), "unwritable location" (line 552), "corrupted or missing scrape_outputs table" (line 577). |

---

## Coverage Matrix Integrity

**PASS.** (Was MINOR-1 in Round 1.)

All entries in the combinatorial coverage matrix (test-plan.md:859-915) now have corresponding BDD scenarios. No phantom entries. Matrix accurately reflects the BDD section.

---

## Defect Summary

### LETHAL: 0

### MAJOR: 0

### MINOR: 2 (threshold: ≥5 for rejection)

1. **MINOR-1** — Behavior 10 (test-plan.md:258) references `result.len_all_partitions() == 3` — this method may not exist on `ScrapeDiff`. The struct uses `Vec<String>` fields, so the test would need `result.new.len() + result.changed.len() + result.unchanged.len() == 3`. Notational issue only; the test-writer will implement correctly. Non-blocking.

2. **MINOR-2** — Behavior 48 (test-plan.md:655) asserts `stderr contains "failed to open state database" or similar` — the "or similar" qualifier is slightly weak for an E2E test. However, E2E tests with exact string matching on stderr are fragile across platforms and shell environments. Acceptable for E2E scope. Non-blocking.

---

## Mandate Fulfillment

All 12 mandates from Round 1 have been satisfied. No new mandates required.

The plan is ready for test implementation (State 2).
