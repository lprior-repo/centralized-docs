---
bead_id: cdocs-5d8
bead_title: "QA: ctd diff/apply require hidden .scrape dir"
phase: p1.5-test-plan-review
reviewed_at: 2026-04-20T02:25:00Z
verdict: APPROVED
previous_review: 2026-04-19T09:00:00Z (REJECTED — 8 MAJOR)
---

# Test Plan Review (Re-Review): `resolve_manifest_dir` + consumers

## VERDICT: **APPROVED**

**0 LETHAL / 0 MAJOR / 4 MINOR — all thresholds clear**

Previous review REJECTED with 8 MAJOR findings (6 assertion sharpness + 2 boundary completeness).
All 8 MAJOR findings verified as fixed. All 13 MANDATE items addressed. Plan is ready for
implementation.

---

## Previous MANDATE Verification

### Required assertion fixes (6 items) — ALL FIXED

| # | MANDATE Item | Status | Evidence |
|---|-------------|--------|----------|
| 1 | B6: exact concrete PathBuf | **FIXED** | test-plan.md:232 — `Then: returns Ok(PathBuf::from("/tmp/abstest"))`. Line 237 explicitly bans `is_absolute()` and "starts with". |
| 2 | B10: concrete manifest contents + summary | **FIXED** | test-plan.md:278-287 — Given specifies exact pages `[{url: "/alpha", content: "v1"}, {url: "/beta", content: "v1"}]` vs `[{url: "/alpha", content: "v1"}, {url: "/gamma", content: "new"}]`. Then asserts `removed == 1 ("/beta")`, `added == 1 ("/gamma")`, `unchanged == 1 ("/alpha")`, `modified == 0`. |
| 3 | B12: concrete manifest + summary | **FIXED** | test-plan.md:305-315 — Given specifies `[{url: "/x", content: "v1"}, {url: "/y", content: "v1"}]` vs `[{url: "/x", content: "v1"}, {url: "/z", content: "new"}]`. Then asserts `removed == 1 ("/y")`, `added == 1 ("/z")`, `unchanged == 1 ("/x")`, `modified == 0`. |
| 4 | B17: concrete base_url + pages | **FIXED** | test-plan.md:375-382 — Given specifies `base_url: "https://example.com"`, `pages [{url: "/docs", title: "Docs"}]`. Then asserts `result.base_url == "https://example.com"`, `result.pages[0].url == "/docs"`, `result.pages[0].title == "Docs"`. |
| 5 | B24: concrete pages + summary in E2E | **FIXED** | test-plan.md:462-475 — Given specifies exact pages for both dirs. Then asserts `removed == 1 ("/guide")`, `added == 1 ("/api")`, `unchanged == 1 ("/intro")`, `modified == 0`. |
| 6 | B25: concrete ScrapeResult in E2E | **FIXED** | test-plan.md:484-496 — Given specifies exact ScrapeResult. Then asserts `result.base_url`, `result.pages[0].url`, `result.pages[0].title`, `result.pages[1].url`, `result.pages[1].title` — all with concrete values. |

### Required boundary tests (7 items) — ALL ADDED

| # | Required Test | Status | Evidence |
|---|--------------|--------|----------|
| 7 | `resolve_manifest_dir_handles_trailing_slash_in_path` | **ADDED** | B31 (test-plan.md:560-574). Given: `"/tmp/output/"` with `.scrape/manifest.json`. Then: `Ok(PathBuf::from("/tmp/output/.scrape"))`. |
| 8 | `resolve_manifest_dir_handles_spaces_in_directory_name` | **ADDED** | B32 (test-plan.md:576-585). Given: `"/tmp/my output"`. Then: `Ok(PathBuf::from("/tmp/my output/.scrape"))`. |
| 9 | `resolve_manifest_dir_handles_unicode_in_directory_name` | **ADDED** | B33 (test-plan.md:587-596). Given: `"/tmp/ドキュメント"`. Then: `Ok(PathBuf::from("/tmp/ドキュメント/.scrape"))`. |
| 10 | `resolve_manifest_dir_resolves_when_path_is_scrape_dir_itself` | **ADDED** | B34 (test-plan.md:598-612). Given: `"/tmp/output/.scrape"` with `manifest.json` directly. Then: `Ok(PathBuf::from("/tmp/output/.scrape"))`. |
| 11 | `diff_directories_with_empty_manifests_produces_empty_plan` | **ADDED** | B35 (test-plan.md:614-628). Given: `{"pages":[]}` both sides. Then: `added == 0, removed == 0, modified == 0, unchanged == 0`, `plan.summary.is_empty() == true`. |
| 12 | `diff_directories_with_same_dir_for_both_args_produces_empty_plan` | **ADDED** | B36 (test-plan.md:630-644). Given: same dir for both args. Then: `unchanged == 2, added/removed/modified == 0`. |
| 13 | `diff_directories_handles_large_manifest_count` | **ADDED** | B37 (test-plan.md:646-662). Given: 100 pages each (50 overlap). Then: `removed == 50, unchanged == 50, added == 50, modified == 0`. |

**Result: 13/13 MANDATE items addressed.**

---

## Axis 1 — Contract Parity: **PASS**

### Public functions vs BDD scenarios

| Function | Visibility | Source | Scenarios | Status |
|----------|-----------|--------|-----------|--------|
| `resolve_manifest_dir` | `pub(crate) fn` | contract.md:151 | B1–B9, B31–B34 (13) | PASS |
| `diff_directories` | `pub fn` | contract.md:167 | B10–B16, B35–B37 (9) | PASS |
| `read_manifest` | `fn` (private) | contract.md:195 | B17–B20 (4) | PASS |
| `write_scraped_pages` | existing | contract.md:210 | B21–B23 (3) | PASS |
| `run_diff` / `run_apply` | CLI layer | contract.md:218-231 | B26–B30 (5) | PASS |

Every function in the contract has ≥1 BDD scenario. No gaps.

### Error variant coverage

`ManifestResolveError::NotFound` — sole variant (contract.md:91-108).

- B3 (test-plan.md:190-200): Asserts **all four fields** — `path`, `scrape_subdir`, `direct`, `nested` — with exact concrete values. Explicit prohibition of `is_err()` only.
- B7 (test-plan.md:239-252): Asserts Display string contains both candidate paths, `.scrape` substring, and `"Tip:"` guidance.

No `is_err()` or `is_ok()` anywhere in any scenario. Exact variant asserted.

---

## Axis 2 — Assertion Sharpness: **PASS**

Every "Then:" block in all 37 scenarios audited. Results:

| Scenario | Then Content | Sharp? | Notes |
|----------|-------------|--------|-------|
| B1 | `Ok(PathBuf::from("/tmp/test"))` | YES | Exact PathBuf |
| B2 | `Ok(PathBuf::from("/tmp/test/.scrape"))` | YES | Exact PathBuf |
| B3 | `Err(NotFound { path: "/tmp/empty", scrape_subdir: "/tmp/empty/.scrape", direct: "/tmp/empty/manifest.json", nested: "/tmp/empty/.scrape/manifest.json" })` | YES | All 4 fields, exact values |
| B4 | `Ok(PathBuf::from("/tmp/both"))` + content verification of resolved manifest | YES | Exact value + readback |
| B5 | `Ok(PathBuf::from("testdir"))` + `result.is_absolute() == false` | YES | Exact value + boolean check |
| B6 | `Ok(PathBuf::from("/tmp/abstest"))` | YES | **Fixed from previous** — was `is_absolute()` + "starts with", now exact PathBuf |
| B7 | Display contains `"/tmp/missing"`, `"/tmp/missing/.scrape"`, `"Tip:"` | YES | 3 concrete string assertions |
| B8 | Both calls return `Ok(PathBuf::from("/tmp/det/.scrape"))` + equality | YES | Exact value + determinism |
| B9 | Directory listing unchanged | YES | No-filesystem-mutation check |
| B10 | `removed == 1 ("/beta")`, `added == 1 ("/gamma")`, `unchanged == 1 ("/alpha")`, `modified == 0` | YES | **Fixed from previous** — was "correct diff", now concrete counts with URL labels |
| B11 | `added == 1, removed == 1, modified == 1` | YES | Concrete counts |
| B12 | `removed == 1 ("/y")`, `added == 1 ("/z")`, `unchanged == 1 ("/x")`, `modified == 0` | YES | **Fixed from previous** — was "existing behavior preserved" |
| B13 | `modified == 1` | YES | Concrete count |
| B14 | Err contains `"No manifest.json found"` + dir_a path | YES | Concrete string |
| B15 | Err contains `"No manifest.json found"` + dir_b path | YES | Concrete string |
| B16 | Err contains `"Invalid manifest"` | YES | Concrete string |
| B17 | `base_url == "https://example.com"`, `pages[0].url == "/docs"`, `pages[0].title == "Docs"` | YES | **Fixed from previous** — was "correct base_url and pages" |
| B18 | `pages.len() == 2`, `pages[0].url == "/p1"`, `pages[1].url == "/p2"` | YES | **Fixed from previous MINOR** — now checks page URLs, not just count |
| B19 | `pages[0].url == "/only"`, `pages[0].title == "Only Page"` | YES | **Fixed from previous MINOR** — now checks page content |
| B20 | Err contains `"No manifest.json found"` | YES | Concrete string |
| B21 | `output_dir/.scrape/ exists and is a directory` | YES | Concrete path + type |
| B22 | `.scrape/manifest.json` exists + deserialize yields `pages.len() == 2` | YES | Concrete + roundtrip |
| B23 | `.scrape/getting-started.md` exists + content starts with `"---\nurl:"` | YES | Concrete path + content prefix |
| B24 | `removed == 1 ("/guide")`, `added == 1 ("/api")`, `unchanged == 1 ("/intro")`, `modified == 0` | YES | **Fixed from previous** — was narrative, now concrete |
| B25 | `base_url == "https://example.com"`, `pages[0].url == "/intro"`, `pages[0].title == "Intro"`, `pages[1].url == "/guide"`, `pages[1].title == "Guide"` | YES | **Fixed from previous** — was narrative, now concrete |
| B26 | `change-plan.json` exists, `change-plan.md` exists | YES | Concrete paths |
| B27 | Err chain contains `"No manifest.json found"` | YES | Concrete string |
| B28 | Err contains `"No manifest.json found"` | YES | Concrete string |
| B29 | `Ok(())` + no new snapshot committed | YES | Concrete + side-effect check |
| B30 | 2 pages, `url == "/api"`, `url == "/guide"` | YES | **Fixed from previous MINOR** — exact page assertions |
| B31 | `Ok(PathBuf::from("/tmp/output/.scrape"))` | YES | Exact PathBuf |
| B32 | `Ok(PathBuf::from("/tmp/my output/.scrape"))` | YES | Exact PathBuf |
| B33 | `Ok(PathBuf::from("/tmp/ドキュメント/.scrape"))` | YES | Exact PathBuf |
| B34 | `Ok(PathBuf::from("/tmp/output/.scrape"))` + `result.join("manifest.json") == ...` | YES | Exact value + readback |
| B35 | `added == 0, removed == 0, modified == 0, unchanged == 0`, `is_empty() == true` | YES | All 4 counts + boolean |
| B36 | `unchanged == 2, added/removed/modified == 0` | YES | Concrete counts |
| B37 | `removed == 50, unchanged == 50, added == 50, modified == 0` | YES | Concrete counts at scale |

**0 vague assertions.** Every Then block specifies concrete expected values. All 6 previous
MAJOR findings in this axis are verified fixed.

---

## Axis 3 — Trophy Allocation: **PASS**

### Density audit

| Metric | Value |
|--------|-------|
| Contract `pub fn` count | 2 (`resolve_manifest_dir`, `diff_directories`) |
| Unit tests | 16 (B1–B9, B21–B23, B31–B34) |
| Unit density | 16 / 2 = **8.0×** (target ≥5×) |
| Integration tests | 19 (B10–B20, B26–B30, B35–B37) |
| E2E tests | 2 (B24–B25) |
| Proptest invariants | 5 (P1–P5) |
| Fuzz targets | 3 (F1–F3) |
| Kani harnesses | 2 (K1–K2) |
| **Total** | **47** (37 behavioral + 5 proptest + 3 fuzz + 2 kani) |
| **Overall density** | 47 / 2 = **23.5×** |

Well above every threshold.

### Proptest for non-trivial input space

`resolve_manifest_dir` has filesystem-dependent behavior but the resolution logic itself
is pure (given filesystem state). 3 proptests target it specifically:

- P1: Determinism (any layout, call twice → same result)
- P2: Path identity (INV3 — returned path always has manifest.json)
- P5: Precedence (direct always wins over nested)

`diff_directories` has 1 proptest:
- P4: Summary conservation (added + modified + unchanged == total_current)

Roundtrip:
- P3: write_scraped_pages → resolve_manifest_dir always resolves correctly

### Fuzz targets

F1: Arbitrary path input (11 corpus seeds including empty, null byte, unicode, special dirs).
F2: ScrapeResult deserialization (7 corpus seeds).
F3: ChangePlan deserialization (3 corpus seeds).

### Trophy shape

16 unit / 19 integration / 2 e2e = 43% / 51% / 5%. Integration-heavy, but this feature
IS an integration bug (producer writes one layout, consumer expects another). The unit
layer still covers the core resolution logic at 8× density. Justified.

---

## Axis 4 — Boundary Completeness: **PASS**

### `resolve_manifest_dir` boundaries

| Boundary | Scenario | Concrete? | Status |
|----------|----------|-----------|--------|
| Direct only (happy path) | B1 | `Ok(PathBuf::from("/tmp/test"))` | PASS |
| Nested only (happy path) | B2 | `Ok(PathBuf::from("/tmp/test/.scrape"))` | PASS |
| Neither (error) | B3 | `Err(NotFound { 4 exact fields })` | PASS |
| Both (precedence) | B4 | `Ok(PathBuf::from("/tmp/both"))` | PASS |
| Relative path | B5 | `Ok(PathBuf::from("testdir"))` | PASS |
| Absolute path | B6 | `Ok(PathBuf::from("/tmp/abstest"))` | PASS |
| Trailing slash | B31 | `Ok(PathBuf::from("/tmp/output/.scrape"))` | PASS |
| Spaces in name | B32 | `Ok(PathBuf::from("/tmp/my output/.scrape"))` | PASS |
| Unicode in name | B33 | `Ok(PathBuf::from("/tmp/ドキュメント/.scrape"))` | PASS |
| Path IS `.scrape` | B34 | `Ok(PathBuf::from("/tmp/output/.scrape"))` | PASS |
| Nonexistent path | B3 (variant) | `Err(NotFound)` | PASS |
| Long path | Fuzz F1 | Never panics | PASS |
| Empty path | Fuzz F1 | Never panics | PASS |
| Path with null bytes | Fuzz F1 | Never panics | PASS |

All previous 4 missing boundaries (trailing slash, spaces, unicode, path=.scrape) are now
addressed with concrete BDD scenarios B31–B34.

### `diff_directories` boundaries

| Boundary | Scenario | Concrete? | Status |
|----------|----------|-----------|--------|
| Both scrape roots | B10, B11 | Exact summary counts | PASS |
| Both direct layout | B12 | Exact summary counts | PASS |
| Mixed layouts (a=scrape, b=direct) | B13 | `modified == 1` | PASS |
| dir_a unresolvable | B14 | Err string assertion | PASS |
| dir_b unresolvable | B15 | Err string assertion | PASS |
| Invalid JSON | B16 | Err string assertion | PASS |
| Empty manifests | B35 | All zeros + `is_empty()` | PASS |
| Same dir both args | B36 | `unchanged == 2, others == 0` | PASS |
| Large manifests (100 pages) | B37 | `removed == 50, unchanged == 50, added == 50` | PASS |

All previous 3 missing boundaries (empty manifests, same dir, large manifests) are now
addressed with B35–B37.

### `read_manifest` boundaries

| Boundary | Scenario | Concrete? | Status |
|----------|----------|-----------|--------|
| Scrape root | B18 | `pages[0].url == "/p1"`, `pages[1].url == "/p2"` | PASS |
| Direct dir | B19 | `pages[0].url == "/only"`, `pages[0].title == "Only Page"` | PASS |
| Unresolvable | B20 | Err string assertion | PASS |
| Concrete data | B17 | `base_url`, `pages[0].url`, `pages[0].title` | PASS |

### `write_scraped_pages` boundaries

| Boundary | Scenario | Concrete? | Status |
|----------|----------|-----------|--------|
| Creates .scrape/ subdir | B21 | `exists()` + `is_dir()` | PASS |
| Writes manifest | B22 | Deserializes to `pages.len() == 2` | PASS |
| Writes .md files | B23 | `exists()` + content starts with `"---\nurl:"` | PASS |

---

## Axis 5 — Mutation Survivability: **PASS**

Mental mutation analysis against every scenario in the plan. The plan's own mutation table
(test-plan.md:830-865) lists 17 mutations with catching tests — verified correct.

### Previously surviving mutants — ALL NOW CAUGHT

| Mutation | Previously survived? | Now caught by | Evidence |
|----------|---------------------|---------------|----------|
| `diff_directories` returns empty ChangePlan for scrape roots | YES (B10 was "correct diff") | B10 | test-plan.md:283-287 — `removed == 1, added == 1, unchanged == 1, modified == 0` |
| `diff_directories` returns empty ChangePlan for direct layout | YES (B12 was "behavior preserved") | B12 | test-plan.md:312-315 — `removed == 1, added == 1, unchanged == 1, modified == 0` |
| `read_manifest` returns `ScrapeResult::default()` | YES (B17 was "correct values") | B17 | test-plan.md:378-382 — `base_url == "https://example.com"`, `pages[0].url == "/docs"` |
| Full scrape→diff returns empty ChangePlan | YES (B24 was narrative) | B24 | test-plan.md:471-475 — `removed == 1, added == 1, unchanged == 1` |
| Full scrape→apply reads wrong manifest | YES (B25 was narrative) | B25 | test-plan.md:491-496 — 6 concrete field assertions |
| `resolve_manifest_dir` returns child path for absolute input | YES (B6 was "starts with") | B6 | test-plan.md:232 — `Ok(PathBuf::from("/tmp/abstest"))` exact value |

### Additional mutation coverage verification

| Mutation | Caught by | How |
|----------|-----------|-----|
| Skip first `exists()` check | B1 | Direct-only dir returns wrong path |
| Skip second `exists()` check | B2 | Scrape root fails to resolve |
| Swap precedence order | B4 | Both-exist case returns nested path |
| `join(".scrape")` → `join("scrape")` | B2, B21 | Hidden dir lookup fails |
| Error returns without field population | B7 | String assertions on all 4 paths |
| String concat instead of `Path::join` | B31, B32, B33 | Trailing slash / spaces / unicode breaks |
| Revert diff_directories to old join | B11 | Scrape root layout fails |
| Revert read_manifest to old join | B18 | Scrape root fails |
| Swap dir_a / dir_b resolution | B14 vs B15 | Error message names wrong directory |
| Off-by-one in summary counting | B37 | `removed == 50` (exact, not `≥ 49`) |
| Return `ScrapeResult::default()` in read path | B17, B25 | Concrete field assertions |
| Return empty ChangePlan | B10, B12, B24 | Concrete summary counts |

Estimated kill rate: ≥95% (17/17 planned mutations caught, plus all 6 previously surviving
mutants now caught by fixed assertions).

---

## Axis 6 — Holzmann Plan Audit: **PASS**

| Rule | Status | Notes |
|------|--------|-------|
| Rule 1 (Linear) | PASS | All scenarios are Given/When/Then. No nested conditionals. |
| Rule 2 (Bound loops) | PASS | No loops in any scenario body. Proptest uses strategies. |
| Rule 3 (Resource ownership) | PASS | All scenarios use tempdir for filesystem isolation. |
| Rule 4 (One function, one job) | PASS | Each scenario tests one behavior. |
| Rule 5 (State assumptions explicitly) | PASS | B10, B12, B24, B25 Given blocks now specify exact manifest contents. **All 4 previous MINOR findings fixed.** |
| Rule 6 (Never swallow errors) | PASS | No `let _ =` or `.ok()` in any scenario. |
| Rule 7 (Narrow state) | PASS | No shared mutable state. Each scenario has its own tempdir. |
| Rule 8 (Surface side effects) | PASS | File creation explicitly described in Given blocks. |
| Rule 9 (One layer of magic) | PASS | No deep helper chains. Open Q #2 (StateDb fixture) noted but not decided — acceptable at plan stage. |
| Rule 10 (Warnings are errors) | PASS | Static analysis section includes `clippy::pedantic`. |

---

## Summary

| Axis | Verdict | Findings |
|------|---------|----------|
| 1. Contract Parity | **PASS** | All pub fns covered, error variant exact |
| 2. Assertion Sharpness | **PASS** | All 37 scenarios have concrete Then values. 6 previous MAJOR findings fixed. |
| 3. Trophy Allocation | **PASS** | 8× unit density, 5 proptests, 3 fuzz targets, 2 Kani harnesses |
| 4. Boundary Completeness | **PASS** | All 7 previously missing boundaries now have BDD scenarios B31–B37 |
| 5. Mutation Survivability | **PASS** | Estimated ≥95% kill rate. All 6 previously surviving mutants caught. |
| 6. Holzmann Rules | **PASS** | All 4 previous MINOR findings (vague Given blocks) fixed. |

---

## LETHAL FINDINGS

None.

## MAJOR FINDINGS

None.

## MINOR FINDINGS (4/5 threshold — under threshold)

1. **test-plan.md:295-298 (B11 Given/Then)** — Uses shorthand `[A, B]` and `[A (modified), C]`
   for page definitions while all other integration scenarios (B10, B12, B13) use explicit
   `{url: "...", content: "..."}` notation. The Then labels summary counts with parenthetical
   URL annotations but the Given doesn't map A/B/C to concrete URLs. Holzmann Rule 5:
   state assumptions explicitly. The Then constraints uniquely determine the semantics,
   making this a consistency issue rather than a correctness gap.

2. **test-plan.md:925 (Matrix D)** — Slug collision edge case (`2 pages with same slug`)
   listed in combinatorial matrix but has no BDD scenario. write_scraped_pages is unchanged
   code, so this is beyond scope, but the roundtrip B24/B25 depends on correct write output.
   Covered implicitly by proptest P3 (random pages) but not explicitly named.

3. **test-plan.md:885-886 (Matrix A)** — Boundary cases `path = "."` and `path = ".."`
   listed in combinatorial matrix with expected outputs but have no BDD scenario numbers.
   Fuzz target F1 includes both as corpus seeds (asserting "never panics") and B5 covers
   general relative path handling. However, the specific filesystem semantics of `.` (cwd)
   and `..` (parent) are not behaviorally tested with concrete assertions.

4. **test-plan.md:970-973 (Open Q #2)** — StateDb fixture approach for tests B28–B30
   remains undecided. Holzmann Rule 9 concern: if the fixture involves deep helper chains,
   the tests become hard to debug. Acceptable at plan stage but must be resolved during
   implementation.

## MANDATE

None. Plan is approved for implementation.

### Recommended (not blocking)
- B11: Align Given notation with B10/B12 style (explicit `{url, content}` dicts) for
  consistency across all diff_directories scenarios.
- B28–B30 implementation: Resolve Open Q #2 before writing tests. Prefer inline tempdir
  setup over shared fixtures (Holzmann Rule 9).
- Consider promoting Matrix A `.` / `..` cases into named unit tests if the implementation
  has any special handling for them.
