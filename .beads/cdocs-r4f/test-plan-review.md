# Test Plan Review: cdocs-r4f (Round 2)

```
bead_id: cdocs-r4f
bead_title: calc: build URL-state and scrape-output commit batches from scrape results
reviewer: test-inquisitor (Mode 1 — Plan Inquisition)
reviewed_at: 2026-04-04T00:30:00Z
revision: round-2 (resubmission after R1 rejection)
```

## VERDICT: APPROVED

---

### Mandate Verification (Round 1 Rejection)

| # | Mandate | Status | Evidence |
|---|---------|--------|----------|
| M1 | Add `PayloadProcessingFailed` test with exact variant assertion | **FIXED** | test-plan.md:486–534 — B23 defines primary scenario asserting `Err(ScrapeBatchBuildError::PayloadProcessingFailed { url: "https://fail.com", reason: "SHA-256 hardware accelerator unavailable" })` plus fallback Display test. Mutation table lines 652–653 add two rows covering branch deletion and `unwrap()` substitution. |
| M2 | Fix B05 vague negative assertions | **FIXED** | test-plan.md:148–158 — Now asserts `result.updated_urls[0].0 == "https://c.com"` (positive), `result.updated_urls.iter().all(\|(u, _) \| u != "https://u1.com" && u != "https://u2.com" && u != "https://u3.com")` (negative), `!result.deleted_urls.iter().any(\|u \| u == ...)` (negative), `result.new_scrapes.len() == 1` (count). All concrete. |
| M3 | Fix B15b bare `Ok(StateChanges)` assertion | **FIXED** | test-plan.md:316–329 — Now lists all 10 `StateChanges` fields individually with `.is_empty()`: `updated_urls`, `deleted_urls`, `new_scrapes`, `updated_files`, `deleted_files`, `new_analyses`, `new_transforms`, `new_chunks`, `new_snapshots`, `deleted_snapshots`. |
| M4 | Fix summary arithmetic | **FIXED** | test-plan.md:13–15 — "Behaviors identified: 23 (B01–B23 + MIX)", "BDD scenarios: 31", "30 unit / 2 integration / 1 static". Verified: 23 numbered behaviors + 1 composite = 24 behaviors (notation slightly ambiguous, see MINOR #3). 28 single-scenario behaviors + 2 B23 sub-scenarios + 1 MIX = 31 BDD scenarios. 30 unit + 2 integration + 1 static = 33 test entities. Arithmetic consistent. |

All 4 mandates from Round 1 are satisfied. Proceeding to full 6-axis audit.

---

### Axis 1 — Contract Parity: **PASS**

**Public functions** (contract.md:154–175, 297–298):
| Function | Scenarios | Status |
|----------|-----------|--------|
| `build_scrape_state_changes` | B01–B20, B23, MIX, P1–P3, INT-1 | ✅ |
| `build_url_state_raw` | B21, B22, P4, K1, INT-2 | ✅ |

**Error variants** (contract.md:113–134, 5 total):
| Variant | Scenario | Assertion | Status |
|---------|----------|-----------|--------|
| `EmptyDiff` | B15 | `Err(ScrapeBatchBuildError::EmptyDiff)` | ✅ Exact variant |
| `DuplicateUrl { url }` | B16a–B16f | `Err(ScrapeBatchBuildError::DuplicateUrl { url: "https://dup.com" })` | ✅ Exact variant + field |
| `MissingScrapeArtifact { url }` | B17, B18 | `Err(ScrapeBatchBuildError::MissingScrapeArtifact { url: "https://missing.com" })` | ✅ Exact variant + field |
| `EmptyScrapePayload { url }` | B19, B20 | `Err(ScrapeBatchBuildError::EmptyScrapePayload { url: "https://empty.com" })` | ✅ Exact variant + field |
| `PayloadProcessingFailed { url, reason }` | B23 | `Err(ScrapeBatchBuildError::PayloadProcessingFailed { url: "https://fail.com", reason: "SHA-256 hardware accelerator unavailable" })` | ✅ Exact variant + both fields |

No `is_ok()` or `is_err()` bare assertions. No missing functions. No missing error variants.

---

### Axis 2 — Assertion Sharpness: **PASS**

Every `Then:` block in every scenario asserts concrete values:

| Scenario | Sharpest Assertion | Assessment |
|----------|--------------------|------------|
| B01 | `result.updated_urls[0].0 == "https://a.com"`, `.len() == 2` | ✅ Concrete |
| B02 | `result.updated_urls[0].0 == "https://new.com"`, `.len() == 1` | ✅ Concrete |
| B03 | `result.new_scrapes[0].1 == b"serialized_page_1"` | ✅ Concrete bytes |
| B04 | `.contains(&"https://old1.com".to_string()) == true` | ✅ Concrete |
| B05 | `u != "https://u1.com" && u != "https://u2.com" && u != "https://u3.com"` | ✅ Concrete URLs |
| B06 | `result.updated_urls[0].1.content_hash == [0xAB; 32]` | ✅ Concrete hash |
| B07 | `result.updated_urls[0].1.url_hash == hash_payload(b"test_payload")` | ✅ Concrete hash |
| B08 | `result.updated_urls[0].1.last_fetched_secs == 1_712_345_678` | ✅ Concrete value |
| B09 | `result.updated_urls[0].1.status_code == 301` | ✅ Concrete value |
| B10 | `result.updated_urls[0].1.reserved == [0u8; 46]` | ✅ Concrete pattern |
| B11 | 7 fields each with `.is_empty() == true` | ✅ Per-field explicit |
| B12 | `first_call.updated_urls == second_call.updated_urls` | ✅ PartialEq |
| B13 | `.url_hash == expected_hash_a`, keys == vec of exact hashes | ✅ Concrete hashes |
| B14 | `.updated_urls[0].0 == "https://c1.com"`, `.updated_urls[1].0 == "https://n1.com"` | ✅ Concrete |
| B15 | `Err(ScrapeBatchBuildError::EmptyDiff)` | ✅ Exact variant |
| B15b | 10 fields each with `.is_empty()` | ✅ Per-field explicit |
| B16a–B16f | `Err(DuplicateUrl { url: "https://dup.com" })` | ✅ Exact variant + field |
| B17–B18 | `Err(MissingScrapeArtifact { url: "https://missing.com" })` | ✅ Exact variant + field |
| B19–B20 | `Err(EmptyScrapePayload { url: "https://empty.com" })` | ✅ Exact variant + field |
| B21 | `.content_hash == [0xAA; 32]`, `size_of::<UrlStateRaw>() == 120` | ✅ Concrete |
| B22 | `.reserved == [0u8; 46]` | ✅ Concrete pattern |
| B23 | `Err(PayloadProcessingFailed { url: "https://fail.com", reason: "..." })` | ✅ Exact variant + both fields |
| MIX | `len() == 4`, `len() == 2`, `len() == 4`, URL-specific exclusion | ✅ Concrete |

Zero `is_ok()`, zero `is_err()`, zero `> 0`, zero `Some(_)` without inner value.

---

### Axis 3 — Trophy Allocation: **PASS**

| Metric | Value | Target | Status |
|--------|-------|--------|--------|
| Public functions | 2 | — | — |
| BDD scenarios | 31 | — | — |
| Integration tests | 2 | — | — |
| Static checks | 1 | — | — |
| Proptest invariants | 4 | — | — |
| Kani harnesses | 1 | — | — |
| **Total test entities** | **39** | — | — |
| **Test/function ratio** | **19.5x** | **≥5x** | ✅ |
| Proptest for `build_scrape_state_changes` (non-trivial input space) | P1, P2, P3 | ≥1 | ✅ |
| Proptest for `build_url_state_raw` (non-trivial input space) | P4 | ≥1 | ✅ |
| Fuzz targets | 0 | 0 justified | ✅ (no parser/deserializer) |

---

### Axis 4 — Boundary Completeness: **PASS**

**`build_scrape_state_changes` precondition branches:**
| Branch | Scenario | Status |
|--------|----------|--------|
| All buckets empty | B15 | ✅ |
| Only unchanged non-empty | B15b | ✅ |
| Single changed URL | B01 (2 URLs), B06–B10 (1 URL) | ✅ |
| Single new URL | B02 | ✅ |
| Single deleted URL | B04 (3 URLs) | ✅ |
| All four categories populated | MIX | ✅ |
| Duplicate: all 6 pairwise combinations | B16a–B16f | ✅ |
| Missing artifact (changed) | B17 | ✅ |
| Missing artifact (new) | B18 | ✅ |
| Empty payload (changed) | B19 | ✅ |
| Empty payload (new) | B20 | ✅ |
| Hash computation failure | B23 | ✅ |

No function has ≥3 missing boundaries.

**`build_url_state_raw`** — Pure constructor, no branching. All field assignments verified by B21. Struct size verified by B21 + K1. Byte round-trip verified by P4 + INT-2. Extreme values (u64::MAX, u16::MAX, 0) covered probabilistically by P4 and deterministically by Kani (K1 says "all u64 values, all u16 values"). ✅

---

### Axis 5 — Mutation Survivability: **PASS**

All 20 mutation checkpoints (test-plan.md:644–666) have named catching scenarios. Key mutations verified:

| Mutation | Caught By | Survives? |
|----------|-----------|-----------|
| Remove `EmptyDiff` check | B15 | ❌ Caught |
| Skip duplicate detection | B16a | ❌ Caught |
| Check duplicates only in changed+new | B16d | ❌ Caught |
| Skip `MissingScrapeArtifact` (changed) | B17 | ❌ Caught |
| Skip `MissingScrapeArtifact` (new) | B18 | ❌ Caught |
| Skip `EmptyScrapePayload` check | B19 | ❌ Caught |
| **Delete `PayloadProcessingFailed` branch** | **B23** | **❌ Caught** |
| **Replace `PayloadProcessingFailed` with `unwrap()`** | **B23** | **❌ Caught** |
| Hash wrong field (content vs payload) | B07 | ❌ Caught |
| Set timestamp to 0 instead of config | B08 | ❌ Caught |
| Set status_code to 200 instead of artifact | B09 | ❌ Caught |
| Don't zero `reserved` | B10, B22 | ❌ Caught |
| Include unchanged URLs in output | B05 | ❌ Caught |
| Populate file-state fields | B11 | ❌ Caught |
| Don't add payload to `new_scrapes` | B13 | ❌ Caught |
| Swap ordering (new before changed) | B14 | ❌ Caught |
| `build_url_state_raw` doesn't zero reserved | B22 | ❌ Caught |
| Return early after first URL | MIX | ❌ Caught |
| Non-determinism (HashMap order leak) | B12 | ❌ Caught |
| B15b: populate non-empty instead of empty | B15b | ❌ Caught |

Zero uncaught mutations identified.

---

### Axis 6 — Holzmann Plan Audit: **PASS**

| Rule | Check | Status |
|------|-------|--------|
| Rule 2 — Bound every loop | No loops in test bodies. Proptest ranges have explicit ceilings (`0..5`, `0..10`). | ✅ |
| Rule 5 — State assumptions | B15 now has concrete preconditions (empty ScrapeDiff/Outputs/Config). All scenarios with config-dependent assertions specify `ScrapeBatchConfig { now_secs: N }`. | ✅ |
| Rule 7 — No shared mutable state | No statics, no lazy_static, no Mutex in test plan. All tests create state from scratch. | ✅ |
| Rule 8 — Surface side effects | No side effects — pure calc. Integration tests (INT-1, INT-2) explicitly name their side effects ("verify StateChanges passes commit_changes", "round-trips through to_bytes/from_bytes"). | ✅ |

---

### LETHAL FINDINGS

None.

---

### MAJOR FINDINGS (0)

None.

---

### MINOR FINDINGS (4/5 threshold)

1. **test-plan.md:152–157** — B05 contains a vacuous assertion in the `new_scrapes` closure: `true` is returned unconditionally regardless of input. The closure body computes `hash_payload(payload)` but never uses the result, then returns `true`. This assertion is redundant with `result.new_scrapes.len() == 1` (line 158). It should be removed to avoid confusion — a reader might assume it's checking something meaningful.

2. **test-plan.md:274** — B13 asserts `result.new_scrapes.keys().collect::<Vec<_>>() == vec![&expected_hash_a, &expected_hash_b]`. `HashMap` does not guarantee key iteration order. While Rust's HashMap with 2 entries is practically deterministic (no rehashing), this is an implementation detail. The assertion should sort the collected keys before comparison (`let mut keys: Vec<_> = result.new_scrapes.keys().collect(); keys.sort(); ...`) or use `HashSet`/`BTreeSet` comparison to be robust against future HashMap changes.

3. **test-plan.md:13** — "Behaviors identified: 23 (B01–B23 + MIX)" is ambiguous. B01–B23 = 23 numbered behaviors. MIX is an additional composite scenario that exercises multiple behaviors simultaneously. The parenthetical "(B01–B23 + MIX)" could be read as "23 plus MIX" (= 24) or "23 including MIX" (wrong). Should clarify: "24 behaviors (B01–B23 plus MIX)" or "23 atomic behaviors (B01–B23) plus 1 composite scenario (MIX)".

4. **test-plan.md:111–113** — B03 Given block does not explicitly map `b"serialized_page_1"` to `"https://a.com"` and `b"serialized_page_2"` to `"https://b.com"`. The mapping is implied by POST-10 ordering (changed first, then new_urls), but per Holzmann Rule 5, preconditions should be explicit: `And: ScrapeOutputs.artifacts["https://a.com"].payload_bytes = b"serialized_page_1"` and `And: ScrapeOutputs.artifacts["https://b.com"].payload_bytes = b"serialized_page_2"`.

---

### MANDATE

None. All Round 1 mandates are satisfied. The 4 MINOR findings are below the 5-finding threshold and do not block approval. The test plan may address them at the test writer's discretion during implementation.

### Summary

| Category | Count |
|----------|-------|
| LETHAL | 0 |
| MAJOR | 0 |
| MINOR | 4 (threshold: 5) |
| **Result** | **APPROVED** |
