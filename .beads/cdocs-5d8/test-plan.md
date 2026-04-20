---
bead_id: cdocs-5d8
bead_title: "QA: ctd diff/apply require hidden .scrape dir"
phase: p1.5-test-plan
updated_at: 2026-04-20T02:18:35Z
---

# Test Plan: `resolve_manifest_dir` + consumers

## Summary

- **Behaviors identified**: 37
- **Trophy allocation**: 16 unit / 19 integration / 2 e2e
- **Proptest invariants**: 5
- **Fuzz targets**: 3
- **Kani harnesses**: 2
- **Mutation kill target**: ≥90%

This plan covers the new `resolve_manifest_dir` helper, the two modified call sites
(`diff_directories`, `read_manifest`), the producer `write_scraped_pages`, and the
CLI-level `run_diff` / `run_apply` integration paths.

---

## 1. Behavior Inventory

### `resolve_manifest_dir` (NEW — primary target)

| # | Behavior |
|---|----------|
| B1 | `resolve_manifest_dir` returns input path unchanged when `path/manifest.json` exists |
| B2 | `resolve_manifest_dir` returns `path/.scrape` when only `path/.scrape/manifest.json` exists |
| B3 | `resolve_manifest_dir` returns `ManifestResolveError::NotFound` when neither candidate exists |
| B4 | `resolve_manifest_dir` prefers direct match when BOTH `path/manifest.json` AND `path/.scrape/manifest.json` exist |
| B5 | `resolve_manifest_dir` preserves relative paths as relative |
| B6 | `resolve_manifest_dir` preserves absolute paths as absolute |
| B7 | `resolve_manifest_dir` error message contains both candidate paths (`direct` and `nested` fields) |
| B8 | `resolve_manifest_dir` is deterministic — same filesystem state yields same result |
| B9 | `resolve_manifest_dir` performs no filesystem mutation (read-only) |
| B31 | `resolve_manifest_dir` resolves correctly when input path has trailing slash |
| B32 | `resolve_manifest_dir` resolves correctly when directory name contains spaces |
| B33 | `resolve_manifest_dir` resolves correctly when directory name contains unicode characters |
| B34 | `resolve_manifest_dir` resolves correctly when path IS the `.scrape` directory itself |

### `diff_directories` (MODIFIED — now calls `resolve_manifest_dir`)

| # | Behavior |
|---|----------|
| B10 | `diff_directories` resolves `dir_a` and `dir_b` via `resolve_manifest_dir` before reading manifests |
| B11 | `diff_directories` succeeds when both directories are scrape output roots (`.scrape/` subdirectory layout) |
| B12 | `diff_directories` succeeds when both directories are direct manifest directories |
| B13 | `diff_directories` succeeds with mixed layouts (one scrape root, one direct) |
| B14 | `diff_directories` returns error with actionable message when `dir_a` has no manifest anywhere |
| B15 | `diff_directories` returns error when `dir_b` has no manifest anywhere |
| B16 | `diff_directories` returns error when resolved manifest contains invalid JSON |
| B35 | `diff_directories` produces empty plan with all-zero summary when both manifests have empty pages lists |
| B36 | `diff_directories` produces empty plan with unchanged==N when both args are the same directory |
| B37 | `diff_directories` handles large manifests (100+ pages) without error or incorrect summary |

### `read_manifest` (MODIFIED — now calls `resolve_manifest_dir`)

| # | Behavior |
|---|----------|
| B17 | `read_manifest` resolves `scrape_dir` via `resolve_manifest_dir` before reading |
| B18 | `read_manifest` succeeds when given a scrape output root |
| B19 | `read_manifest` succeeds when given a direct manifest directory |
| B20 | `read_manifest` returns error with actionable message when no manifest found |

### `write_scraped_pages` (UNCHANGED — confirmed producer of `.scrape/` layout)

| # | Behavior |
|---|----------|
| B21 | `write_scraped_pages` creates `output_dir/.scrape/` subdirectory |
| B22 | `write_scraped_pages` writes `manifest.json` to `output_dir/.scrape/manifest.json` |
| B23 | `write_scraped_pages` writes page `.md` files into `output_dir/.scrape/` |

### End-to-end: scrape → diff/apply roundtrip

| # | Behavior |
|---|----------|
| B24 | `ctd scrape` output dir works as input to `ctd diff` (same dir, no manual path adjustment) |
| B25 | `ctd scrape` output dir works as input to `ctd apply --scrape-dir` |

### `run_diff` CLI integration

| # | Behavior |
|---|----------|
| B26 | `run_diff` writes report files to output directory when specified |
| B27 | `run_diff` returns error when directory resolution fails |

### `run_apply` CLI integration

| # | Behavior |
|---|----------|
| B28 | `run_apply` returns error when scrape_dir resolution fails |
| B29 | `run_apply` skips commit when plan is empty (no changes) |
| B30 | `run_apply` reads manifest from resolved path and computes correct plan |

---

## 2. Trophy Allocation

### Static Analysis (5%)
- `clippy::pedantic` — catches path manipulation errors
- `clippy::path_ends_with_ext` — ensures `.scrape` literal is used consistently
- `cargo-deny` — dependency auditing (existing CI)

### Unit / Calc Layer (30%)

These test **pure logic** or **single-function behavior** with controlled filesystem state:

| Behavior | Test Function | Justification |
|----------|--------------|---------------|
| B1 | `resolve_manifest_dir_returns_input_path_when_direct_manifest_exists` | Single resolution rule, tempdir |
| B2 | `resolve_manifest_dir_returns_scrape_subdir_when_only_nested_exists` | Single resolution rule, tempdir |
| B3 | `resolve_manifest_dir_returns_not_found_when_neither_exists` | Single error variant, tempdir |
| B4 | `resolve_manifest_dir_prefers_direct_when_both_exist` | Precedence rule, tempdir |
| B5 | `resolve_manifest_dir_preserves_relative_path_form` | Path identity invariant |
| B6 | `resolve_manifest_dir_preserves_absolute_path_form` | Path identity invariant |
| B7 | `resolve_manifest_dir_not_found_error_contains_both_candidate_paths` | Error message completeness (INV5) |
| B8 | `resolve_manifest_dir_is_deterministic` | INV2 — called twice, same result |
| B9 | `resolve_manifest_dir_creates_no_files` | Post5 — no side effects |
| B21 | `write_scraped_pages_creates_scrape_subdir` | Producer layout verification |
| B22 | `write_scraped_pages_puts_manifest_in_scrape_subdir` | Producer layout verification |
| B23 | `write_scraped_pages_puts_md_files_in_scrape_subdir` | Producer layout verification |
| B31 | `resolve_manifest_dir_handles_trailing_slash_in_path` | Boundary: trailing slash in path |
| B32 | `resolve_manifest_dir_handles_spaces_in_directory_name` | Boundary: spaces in path |
| B33 | `resolve_manifest_dir_handles_unicode_in_directory_name` | Boundary: unicode in path |
| B34 | `resolve_manifest_dir_resolves_when_path_is_scrape_dir_itself` | Boundary: path points to .scrape itself |

### Integration (60%)

These test **real filesystem I/O** with `tempdir`, real manifest JSON, and multi-component interactions:

| Behavior | Test Function | Justification |
|----------|--------------|---------------|
| B10 | `diff_directories_resolves_both_dirs_via_helper` | Verifies integration of helper into existing flow |
| B11 | `diff_directories_succeeds_with_scrape_root_layout` | Primary bug fix scenario |
| B12 | `diff_directories_succeeds_with_direct_layout` | Backward compatibility |
| B13 | `diff_directories_succeeds_with_mixed_layouts` | Cross-layout diff |
| B14 | `diff_directories_returns_error_when_dir_a_unresolvable` | Error propagation from helper |
| B15 | `diff_directories_returns_error_when_dir_b_unresolvable` | Error propagation from helper |
| B16 | `diff_directories_returns_error_on_invalid_manifest_json` | Deserialization boundary |
| B17 | `read_manifest_resolves_via_helper` | Integration of helper into read path |
| B18 | `read_manifest_succeeds_with_scrape_root` | Primary bug fix scenario |
| B19 | `read_manifest_succeeds_with_direct_dir` | Backward compatibility |
| B20 | `read_manifest_returns_error_when_unresolvable` | Error propagation |
| B26 | `run_diff_writes_reports_to_output_dir` | CLI-level integration |
| B27 | `run_diff_returns_error_on_resolution_failure` | CLI-level error path |
| B28 | `run_apply_returns_error_on_scrape_dir_resolution_failure` | CLI-level error path |
| B29 | `run_apply_skips_commit_when_plan_empty` | Idempotency guard |
| B30 | `run_apply_reads_manifest_from_resolved_path` | Full apply integration |
| B35 | `diff_directories_with_empty_manifests_produces_empty_plan` | Boundary: both empty pages |
| B36 | `diff_directories_with_same_dir_for_both_args_produces_empty_plan` | Boundary: identical args |
| B37 | `diff_directories_handles_large_manifest_count` | Boundary: 100+ pages |

### E2E (5%)

| Behavior | Test Function | Justification |
|----------|--------------|---------------|
| B24 | `scrape_output_dir_works_as_diff_input` | Full producer→consumer roundtrip |
| B25 | `scrape_output_dir_works_as_apply_input` | Full producer→consumer roundtrip |

---

## 3. BDD Scenarios

### B1: `resolve_manifest_dir` returns input path when direct manifest exists

```
Given: a directory /tmp/test containing manifest.json (valid JSON)
When:  resolve_manifest_dir(Path::new("/tmp/test")) is called
Then:  returns Ok(PathBuf::from("/tmp/test"))

Test: fn resolve_manifest_dir_returns_input_path_when_direct_manifest_exists()
```

### B2: `resolve_manifest_dir` returns `.scrape` subpath when only nested exists

```
Given: a directory /tmp/test containing .scrape/manifest.json (but NOT /tmp/test/manifest.json)
When:  resolve_manifest_dir(Path::new("/tmp/test")) is called
Then:  returns Ok(PathBuf::from("/tmp/test/.scrape"))

Test: fn resolve_manifest_dir_returns_scrape_subdir_when_only_nested_exists()
```

### B3: `resolve_manifest_dir` returns NotFound when neither candidate exists

```
Given: a directory /tmp/empty containing no manifest.json and no .scrape/manifest.json
When:  resolve_manifest_dir(Path::new("/tmp/empty")) is called
Then:  returns Err(ManifestResolveError::NotFound { path: "/tmp/empty",
        scrape_subdir: "/tmp/empty/.scrape", direct: "/tmp/empty/manifest.json",
        nested: "/tmp/empty/.scrape/manifest.json" })

Test: fn resolve_manifest_dir_returns_not_found_when_neither_exists()
```

**Assert all four fields** — `path`, `scrape_subdir`, `direct`, `nested` — not just `is_err()`.

### B4: `resolve_manifest_dir` prefers direct match when both exist

```
Given: a directory /tmp/both containing manifest.json AND .scrape/manifest.json
When:  resolve_manifest_dir(Path::new("/tmp/both")) is called
Then:  returns Ok(PathBuf::from("/tmp/both"))
  AND: result.join("manifest.json") resolves to the direct file, not the nested one

Test: fn resolve_manifest_dir_prefers_direct_when_both_exist()
```

Verify by writing different content to each manifest, reading back the returned path's
manifest, and asserting it matches the **direct** file's content.

### B5: `resolve_manifest_dir` preserves relative path form

```
Given: a relative directory "testdir" containing manifest.json
When:  resolve_manifest_dir(Path::new("testdir")) is called
Then:  returns Ok(PathBuf::from("testdir"))
  AND: result.is_absolute() == false

Test: fn resolve_manifest_dir_preserves_relative_path_form()
```

### B6: `resolve_manifest_dir` preserves absolute path form

```
Given: an absolute directory /tmp/abstest containing manifest.json
When:  resolve_manifest_dir(Path::new("/tmp/abstest")) is called
Then:  returns Ok(PathBuf::from("/tmp/abstest"))

Test: fn resolve_manifest_dir_preserves_absolute_path_form()
```

Assert the **exact** PathBuf value — not `is_absolute()` or "starts with".

### B7: `resolve_manifest_dir` NotFound error contains both candidate paths

```
Given: a directory /tmp/missing with no manifests
When:  resolve_manifest_dir(Path::new("/tmp/missing")) is called
Then:  the error's Display output contains "/tmp/missing"
  AND: contains "/tmp/missing/.scrape"
  AND: contains "Tip:" (actionable guidance)

Test: fn resolve_manifest_dir_not_found_error_contains_both_candidate_paths()
```

Assert the error message string contains both path strings and the tip text.
**Do NOT assert `is_err()` only.**

### B8: `resolve_manifest_dir` is deterministic

```
Given: a directory with .scrape/manifest.json (no direct manifest)
When:  resolve_manifest_dir is called twice with the same path
Then:  both calls return Ok(PathBuf::from("/tmp/det/.scrape"))
  AND: first_result == second_result

Test: fn resolve_manifest_dir_is_deterministic()
```

### B9: `resolve_manifest_dir` creates no files

```
Given: an empty directory /tmp/fresh
When:  resolve_manifest_dir(Path::new("/tmp/fresh")) is called (returns error)
Then:  directory listing of /tmp/fresh is unchanged (no files created)

Test: fn resolve_manifest_dir_creates_no_files()
```

### B10: `diff_directories` resolves both dirs via helper

```
Given: dir_a as scrape root with dir_a/.scrape/manifest.json containing
        pages [{url: "/alpha", content: "v1"}, {url: "/beta", content: "v1"}]
  AND: dir_b as scrape root with dir_b/.scrape/manifest.json containing
        pages [{url: "/alpha", content: "v1"}, {url: "/gamma", content: "new"}]
When:  diff_directories(dir_a, dir_b) is called
Then:  returns Ok(plan) where
        plan.summary.removed == 1  ("/beta")
        plan.summary.added == 1    ("/gamma")
        plan.summary.unchanged == 1 ("/alpha")
        plan.summary.modified == 0

Test: fn diff_directories_resolves_both_dirs_via_helper()
```

### B11: `diff_directories` succeeds with scrape root layout

```
Given: dir_a/.scrape/manifest.json with pages [A, B]
  AND: dir_b/.scrape/manifest.json with pages [A (modified), C]
When:  diff_directories(dir_a.path(), dir_b.path()) is called
Then:  returns Ok(plan) where plan.summary.added == 1, plan.summary.removed == 1, plan.summary.modified == 1

Test: fn diff_directories_succeeds_with_scrape_root_layout()
```

### B12: `diff_directories` succeeds with direct layout (backward compat)

```
Given: dir_a/manifest.json directly (no .scrape/ subdirectory)
        containing pages [{url: "/x", content: "v1"}, {url: "/y", content: "v1"}]
  AND: dir_b/manifest.json directly
        containing pages [{url: "/x", content: "v1"}, {url: "/z", content: "new"}]
When:  diff_directories(dir_a.path(), dir_b.path()) is called
Then:  returns Ok(plan) where
        plan.summary.removed == 1  ("/y")
        plan.summary.added == 1    ("/z")
        plan.summary.unchanged == 1 ("/x")
        plan.summary.modified == 0

Test: fn diff_directories_succeeds_with_direct_layout()
```

This test uses the same setup as the existing `diff_directories_compares_manifests`
test (from `watch_integration_tests.rs` line 180) but is re-verified to confirm
backward compatibility after the change.

### B13: `diff_directories` succeeds with mixed layouts

```
Given: dir_a as scrape root (dir_a/.scrape/manifest.json)
  AND: dir_b as direct layout (dir_b/manifest.json)
  AND: both contain page with same URL but different content
When:  diff_directories(dir_a.path(), dir_b.path()) is called
Then:  returns Ok(plan) with plan.summary.modified == 1

Test: fn diff_directories_succeeds_with_mixed_layouts()
```

### B14: `diff_directories` returns error when dir_a is unresolvable

```
Given: dir_b has valid manifest at dir_b/manifest.json
  AND: dir_a has no manifest.json and no .scrape/manifest.json
When:  diff_directories(dir_a.path(), dir_b.path()) is called
Then:  returns Err where error message contains "No manifest.json found"
  AND: error message contains the dir_a path

Test: fn diff_directories_returns_error_when_dir_a_unresolvable()
```

### B15: `diff_directories` returns error when dir_b is unresolvable

```
Given: dir_a has valid manifest at dir_a/manifest.json
  AND: dir_b has no manifest.json and no .scrape/manifest.json
When:  diff_directories(dir_a.path(), dir_b.path()) is called
Then:  returns Err where error message contains "No manifest.json found"
  AND: error message contains the dir_b path

Test: fn diff_directories_returns_error_when_dir_b_unresolvable()
```

### B16: `diff_directories` returns error on invalid JSON

```
Given: dir_a/.scrape/manifest.json contains valid JSON
  AND: dir_b/.scrape/manifest.json contains "not json"
When:  diff_directories(dir_a.path(), dir_b.path()) is called
Then:  returns Err where error message contains "Invalid manifest"

Test: fn diff_directories_returns_error_on_invalid_json_in_scrape_root()
```

### B17: `read_manifest` resolves via helper

```
Given: a directory with .scrape/manifest.json (scrape root layout)
  AND: manifest contains base_url: "https://example.com"
  AND: manifest contains pages [{url: "/docs", title: "Docs"}]
When:  read_manifest(dir.path()) is called
Then:  returns Ok(ScrapeResult) where
        result.base_url == "https://example.com"
  AND: result.pages.len() == 1
  AND: result.pages[0].url == "/docs"
  AND: result.pages[0].title == "Docs"

Test: fn read_manifest_resolves_via_helper()
```

### B18: `read_manifest` succeeds with scrape root

```
Given: output_dir/.scrape/manifest.json exists (as written by ctd scrape)
  AND: manifest contains base_url: "https://example.com"
  AND: manifest contains pages [{url: "/p1", title: "Page 1"}, {url: "/p2", title: "Page 2"}]
When:  read_manifest(output_dir.path()) is called
Then:  returns Ok(result) where
        result.pages.len() == 2
  AND: result.pages[0].url == "/p1"
  AND: result.pages[1].url == "/p2"

Test: fn read_manifest_succeeds_with_scrape_root()
```

### B19: `read_manifest` succeeds with direct directory

```
Given: dir/manifest.json exists directly (no .scrape/ subdirectory)
  AND: manifest contains base_url: "https://example.com"
  AND: manifest contains pages [{url: "/only", title: "Only Page"}]
When:  read_manifest(dir.path()) is called
Then:  returns Ok(result) where
        result.pages.len() == 1
  AND: result.pages[0].url == "/only"
  AND: result.pages[0].title == "Only Page"

Test: fn read_manifest_succeeds_with_direct_dir()
```

### B20: `read_manifest` returns error when unresolvable

```
Given: an empty directory with no manifest.json and no .scrape/manifest.json
When:  read_manifest(empty_dir.path()) is called
Then:  returns Err where error message contains "No manifest.json found"

Test: fn read_manifest_returns_error_when_unresolvable()
```

### B21: `write_scraped_pages` creates `.scrape/` subdirectory

```
Given: an empty output directory
  AND: a ScrapeResult with 1 page
When:  write_scraped_pages(&result, output_dir) is called
Then:  output_dir/.scrape/ exists and is a directory

Test: fn write_scraped_pages_creates_scrape_subdir()
```

### B22: `write_scraped_pages` puts manifest in `.scrape/`

```
Given: a ScrapeResult with pages [A, B]
When:  write_scraped_pages(&result, output_dir) is called
Then:  output_dir/.scrape/manifest.json exists
  AND: deserialize(output_dir/.scrape/manifest.json) yields result with pages.len() == 2

Test: fn write_scraped_pages_puts_manifest_in_scrape_subdir()
```

### B23: `write_scraped_pages` puts `.md` files in `.scrape/`

```
Given: a ScrapeResult with page having slug "getting-started"
When:  write_scraped_pages(&result, output_dir) is called
Then:  output_dir/.scrape/getting-started.md exists
  AND: file content starts with "---\nurl:"

Test: fn write_scraped_pages_puts_md_files_in_scrape_subdir()
```

### B24: Scrape output dir works as `ctd diff` input (E2E)

```
Given: write_scraped_pages writes ScrapeResult with
        pages [{url: "/intro", content: "v1"}, {url: "/guide", content: "v1"}]
        to output_dir_a/.scrape/
  AND: write_scraped_pages writes ScrapeResult with
        pages [{url: "/intro", content: "v1"}, {url: "/api", content: "new"}]
        to output_dir_b/.scrape/
When:  diff_directories(output_dir_a.path(), output_dir_b.path()) is called
Then:  returns Ok(ChangePlan) where
        plan.summary.removed == 1   ("/guide")
        plan.summary.added == 1     ("/api")
        plan.summary.unchanged == 1 ("/intro")
        plan.summary.modified == 0
  AND: the same directory that scrape wrote to can be passed directly to diff
       without the user knowing about .scrape/

Test: fn scrape_output_dir_works_as_diff_input()
```

### B25: Scrape output dir works as `ctd apply` input (E2E)

```
Given: write_scraped_pages writes ScrapeResult with
        base_url: "https://example.com",
        pages [{url: "/intro", title: "Intro"}, {url: "/guide", title: "Guide"}]
        to output_dir/.scrape/
When:  read_manifest(output_dir.path()) is called (as run_apply would)
Then:  returns Ok(ScrapeResult) where
        result.base_url == "https://example.com"
  AND: result.pages.len() == 2
  AND: result.pages[0].url == "/intro"
  AND: result.pages[0].title == "Intro"
  AND: result.pages[1].url == "/guide"
  AND: result.pages[1].title == "Guide"

Test: fn scrape_output_dir_works_as_apply_input()
```

### B26: `run_diff` writes reports to output directory

```
Given: two scrape root directories with valid manifests
  AND: an output directory path
When:  run_diff(dir_a, dir_b, Some(output_dir), OutputFormat::Json) is called
Then:  output_dir/change-plan.json exists
  AND: output_dir/change-plan.md exists

Test: fn run_diff_writes_reports_to_output_dir()
```

### B27: `run_diff` returns error on resolution failure

```
Given: dir_a is valid scrape root, dir_b has no manifest anywhere
When:  run_diff(dir_a, dir_b, None, OutputFormat::Json) is called
Then:  returns Err where error chain contains "No manifest.json found"

Test: fn run_diff_returns_error_on_resolution_failure()
```

### B28: `run_apply` returns error on scrape dir resolution failure

```
Given: a StateDb at cache_path, a URL, and a scrape_dir with no manifest
When:  run_apply(url, cache_path, nonexistent_scrape_dir, ConfirmMode::AutoConfirm) is called
Then:  returns Err where error message contains "No manifest.json found"

Note: This requires a real StateDb. Use tempdir for cache_path.

Test: fn run_apply_returns_error_on_scrape_dir_resolution_failure()
```

### B29: `run_apply` skips commit when plan is empty

```
Given: a StateDb with an existing snapshot for url
  AND: a scrape_dir with the same content as the stored snapshot
When:  run_apply(url, cache_path, scrape_dir, ConfirmMode::AutoConfirm) is called
Then:  returns Ok(()) and no new snapshot is committed

Test: fn run_apply_skips_commit_when_plan_empty()
```

### B30: `run_apply` reads manifest from resolved path

```
Given: a scrape root directory (with .scrape/manifest.json)
        containing pages [{url: "/api", title: "API"}, {url: "/guide", title: "Guide"}]
  AND: a StateDb with empty snapshot for url
When:  run_apply(url, cache_path, scrape_root, ConfirmMode::AutoConfirm) is called
Then:  returns Ok(())
  AND: loading the snapshot for url shows 2 pages
  AND: snapshot contains page with url == "/api"
  AND: snapshot contains page with url == "/guide"

Test: fn run_apply_reads_manifest_from_resolved_path()
```

### B31: `resolve_manifest_dir` handles trailing slash in path

```
Given: a directory "/tmp/output" containing .scrape/manifest.json
  AND: no /tmp/output/manifest.json exists
When:  resolve_manifest_dir(Path::new("/tmp/output/")) is called
        (note: trailing slash in input)
Then:  returns Ok(PathBuf::from("/tmp/output/.scrape"))

Test: fn resolve_manifest_dir_handles_trailing_slash_in_path()
```

This verifies that `Path::join` handles trailing slashes correctly. If the
implementation uses string concatenation instead of `Path::join`, this test
catches it (e.g., producing `"/tmp/output//.scrape"` or similar).

### B32: `resolve_manifest_dir` handles spaces in directory name

```
Given: a directory "/tmp/my output" (contains space) containing .scrape/manifest.json
  AND: no "/tmp/my output/manifest.json" exists
When:  resolve_manifest_dir(Path::new("/tmp/my output")) is called
Then:  returns Ok(PathBuf::from("/tmp/my output/.scrape"))

Test: fn resolve_manifest_dir_handles_spaces_in_directory_name()
```

### B33: `resolve_manifest_dir` handles unicode in directory name

```
Given: a directory "/tmp/ドキュメント" containing .scrape/manifest.json
  AND: no "/tmp/ドキュメント/manifest.json" exists
When:  resolve_manifest_dir(Path::new("/tmp/ドキュメント")) is called
Then:  returns Ok(PathBuf::from("/tmp/ドキュメント/.scrape"))

Test: fn resolve_manifest_dir_handles_unicode_in_directory_name()
```

### B34: `resolve_manifest_dir` resolves when path IS the `.scrape` dir itself

```
Given: a directory "/tmp/output/.scrape" containing manifest.json directly
        (i.e., /tmp/output/.scrape/manifest.json exists)
When:  resolve_manifest_dir(Path::new("/tmp/output/.scrape")) is called
Then:  returns Ok(PathBuf::from("/tmp/output/.scrape"))
  AND: result.join("manifest.json") == PathBuf::from("/tmp/output/.scrape/manifest.json")

Test: fn resolve_manifest_dir_resolves_when_path_is_scrape_dir_itself()
```

This tests a real user error scenario: the user passes the `.scrape` directory
itself instead of the parent. The direct check (`path/manifest.json`) should
find `manifest.json` and return the input path unchanged.

### B35: `diff_directories` with empty manifests produces empty plan

```
Given: dir_a/.scrape/manifest.json containing {"pages": []}
  AND: dir_b/.scrape/manifest.json containing {"pages": []}
When:  diff_directories(dir_a.path(), dir_b.path()) is called
Then:  returns Ok(plan) where
        plan.summary.added == 0
        plan.summary.removed == 0
        plan.summary.modified == 0
        plan.summary.unchanged == 0
  AND: plan.summary.is_empty() == true

Test: fn diff_directories_with_empty_manifests_produces_empty_plan()
```

### B36: `diff_directories` with same dir for both args produces empty plan

```
Given: a directory with .scrape/manifest.json containing
        pages [{url: "/a", content: "x"}, {url: "/b", content: "y"}]
When:  diff_directories(dir.path(), dir.path()) is called
        (same directory for both arguments)
Then:  returns Ok(plan) where
        plan.summary.added == 0
        plan.summary.removed == 0
        plan.summary.modified == 0
        plan.summary.unchanged == 2

Test: fn diff_directories_with_same_dir_for_both_args_produces_empty_plan()
```

### B37: `diff_directories` handles large manifest count

```
Given: dir_a/.scrape/manifest.json with 100 pages
        (urls "/page-0" through "/page-99", all with content "v1")
  AND: dir_b/.scrape/manifest.json with 100 pages
        (urls "/page-50" through "/page-99" with content "v1",
         plus urls "/new-0" through "/new-49" with content "new")
When:  diff_directories(dir_a.path(), dir_b.path()) is called
Then:  returns Ok(plan) where
        plan.summary.removed == 50   (pages 0–49 from dir_a)
        plan.summary.unchanged == 50 (pages 50–99 shared)
        plan.summary.added == 50     (new-0 through new-49)
        plan.summary.modified == 0

Test: fn diff_directories_handles_large_manifest_count()
```

---

## 4. Proptest Invariants

### Proptest 1: `resolve_manifest_dir` — determinism

```
Invariant: For any valid tempdir layout, calling resolve_manifest_dir twice
           with the same path yields identical results (both Ok with same PathBuf,
           or both Err with same variant).
Strategy:  Generate random directory names (alphanumeric, 1-20 chars).
           For each, randomly choose: direct only / nested only / both / neither.
Anti-invariant: Non-determinism would manifest as different results on successive calls.
```

```
proptest! {
    #[test]
    fn resolve_manifest_dir_is_deterministic_for_any_layout(
        name in "[a-zA-Z0-9_]{1,20}",
        layout in 0u8..4, // 0=direct, 1=nested, 2=both, 3=neither
    )
}
```

### Proptest 2: `resolve_manifest_dir` — path identity (INV3)

```
Invariant: When resolve_manifest_dir returns Ok(path), then
           path.join("manifest.json") exists on disk.
Strategy:  Same as above — random layouts with guaranteed at least one manifest.
Anti-invariant: Returning a path where manifest.json doesn't exist violates INV3/INV4.
```

### Proptest 3: `write_scraped_pages` → `resolve_manifest_dir` roundtrip

```
Invariant: For any ScrapeResult with 0-50 pages (random URLs, titles, content),
           write_scraped_pages followed by resolve_manifest_dir on the output_dir
           always returns Ok(output_dir.join(".scrape")).
Strategy:  Generate ScrapeResult with proptest arbitrary pages.
Anti-invariant: If write_scraped_pages changes its layout, this proptest catches it.
```

### Proptest 4: `diff_directories` — summary conservation via resolved paths

```
Invariant: For any two ScrapeResults written via write_scraped_pages to separate
           output directories, diff_directories on those output roots always satisfies:
           plan.summary.added + plan.summary.modified + plan.summary.unchanged == plan.summary.total_current
Strategy:  Generate two vectors of pages (0-20 each) with overlapping and unique URLs.
Anti-invariant: Conservation law violation indicates a counting bug.
```

### Proptest 5: `resolve_manifest_dir` — direct always wins over nested

```
Invariant: When both path/manifest.json and path/.scrape/manifest.json exist,
           resolve_manifest_dir always returns the input path (not the .scrape subpath).
Strategy:  Generate random directory names, write both files, verify precedence.
Anti-invariant: Returning the nested path would break the "least surprise" contract.
```

---

## 5. Fuzz Targets

### Fuzz Target 1: `resolve_manifest_dir` — arbitrary path input

```
Function: resolve_manifest_dir
Input type: &Path (arbitrary UTF-8 string)
Risk: Path traversal, panic on special characters, unexpected symlink behavior
Corpus seeds:
  - "" (empty string)
  - "." (current dir)
  - ".." (parent dir)
  - "/proc/self/fd/0" (symlink to special file)
  - "dir/with\0null" (null byte in path)
  - "/very/deep/nested/path/that/does/not/exist"
  - "." repeated 1000 times (path component length stress)
  - "/tmp/output/" (trailing slash)
  - "/tmp/my output/" (spaces)
  - "/tmp/ドキュメント" (unicode)
  - "/tmp/output/.scrape" (path IS .scrape)
```

**Implementation note**: Use `arbitrary::Arbitrary` to generate `String` inputs,
convert to `Path`, call `resolve_manifest_dir`. Assert: never panics. Result is
either `Ok(PathBuf)` or `Err(ManifestResolveError::NotFound)`.

### Fuzz Target 2: `serde_json::from_reader<ScrapeResult>` — manifest deserialization

```
Function: ScrapeResult deserialization (called by read_manifest and diff_directories)
Input type: arbitrary bytes (file content of manifest.json)
Risk: Panic on malformed JSON, OOM on deeply nested structures, logic errors on
      unexpected field values
Corpus seeds:
  - "{}" (empty object)
  - "{\"pages\":[]}" (minimal valid)
  - "null"
  - "[1,2,3]" (array instead of object)
  - "{\"pages\":[{\"url\":\"" + "A"*100000 + "\"}]}" (long string)
  - Binary bytes: [0x00, 0xFF, 0xFE, 0x80]
```

Already partially covered by existing proptest `manifest_deserialization_never_panics_on_random_bytes`
in `watch_integration_tests.rs`, but should be promoted to a `cargo-fuzz` target for
longer runtimes and coverage-guided exploration.

### Fuzz Target 3: `serde_json::from_reader<ChangePlan>` — plan deserialization

```
Function: ChangePlan deserialization (called by report readers)
Input type: arbitrary bytes
Risk: Same as Fuzz Target 2
Corpus seeds:
  - "{}"
  - "{\"changes\":[],\"summary\":{\"added\":0,\"removed\":0,\"modified\":0,\"unchanged\":0}}"
  - Random bytes [0u8..255]
```

Already partially covered by existing `change_plan_deserialization_never_panics_on_random_bytes`.

---

## 6. Kani Harnesses

### Kani Harness 1: Path join correctness for `resolve_manifest_dir`

```
Property: For any input path P (bounded to 64 chars, alphanumeric + '/' + '.' + '-'),
          if path.join("manifest.json").exists() returns true,
          then resolve_manifest_dir returns Ok(P.as_ref().to_path_buf()).
          If not, and path.join(".scrape").join("manifest.json").exists() returns true,
          then resolve_manifest_dir returns Ok(path.join(".scrape")).
          Otherwise returns Err(NotFound).

Bound: Path string length ≤ 64, alphanumeric characters only.
Rationale: Path construction is the core logic. Kani can exhaustively verify all
           path strings within the bound, proving no off-by-one or join errors
           exist in the resolution logic.
```

### Kani Harness 2: Error variant field completeness

```
Property: Every ManifestResolveError::NotFound variant always has exactly 4 fields
          populated (path, scrape_subdir, direct, nested), and:
          - scrape_subdir == path.join(".scrape")
          - direct == path.join("manifest.json")
          - nested == path.join(".scrape").join("manifest.json")

Bound: Path length ≤ 32.
Rationale: INV5 guarantees error message completeness. Kani verifies the construction
           logic always produces correct field values — no field is ever accidentally
           left as an empty PathBuf or wrong join order.
```

---

## 7. Mutation Testing Checkpoints

**Threshold: ≥90% mutation kill rate**

### Critical mutations that MUST be caught:

| Mutation Location | Mutated Code | Caught By Test |
|-------------------|-------------|----------------|
| `resolve_manifest_dir` — first `exists()` check removed | Skips direct check, always goes to nested | B1 (direct-only dir returns wrong path) |
| `resolve_manifest_dir` — second `exists()` check removed | Skips nested check, always errors | B2 (scrape root dir fails to resolve) |
| `resolve_manifest_dir` — precedence swapped (nested checked first) | Returns `.scrape` even when direct exists | B4 (both-exist returns wrong path) |
| `resolve_manifest_dir` — `join(".scrape")` → `join("scrape")` | Looks for non-hidden directory | B2 (scrape root with hidden `.scrape/` fails) |
| `resolve_manifest_dir` — error returns without fields | Empty NotFound error | B7 (error message field assertions) |
| `resolve_manifest_dir` — returns `.scrape` even on error | Resolved path has no manifest | B3 (unresolvable dir incorrectly returns Ok) |
| `resolve_manifest_dir` — string concat instead of Path::join | Path separator issues | B31 (trailing slash), B32 (spaces), B33 (unicode) |
| `diff_directories` — reverts to old `dir.join("manifest.json")` | No longer resolves `.scrape/` | B11 (scrape root layout fails) |
| `read_manifest` — reverts to old `scrape_dir.join("manifest.json")` | No longer resolves `.scrape/` | B18 (scrape root layout fails) |
| `diff_directories` — swaps dir_a/dir_b resolution | Wrong resolution order | B14 vs B15 (specific dir error messages) |
| `diff_directories` — removes error mapping from helper | anyhow wraps wrong type | B14 (error message content changes) |
| `diff_directories` — returns empty ChangePlan for scrape roots | No diff computed | B10 (concrete summary: removed==1, added==1, unchanged==1) |
| `diff_directories` — returns empty ChangePlan for direct layout | No diff computed | B12 (concrete summary: removed==1, added==1, unchanged==1) |
| `read_manifest` — returns `ScrapeResult::default()` | Wrong data | B17 (concrete: base_url=="https://example.com", pages[0].url=="/docs") |
| E2E diff — returns empty ChangePlan | No diff computed | B24 (concrete: removed==1, added==1, unchanged==1) |
| E2E apply — returns `ScrapeResult::default()` | Wrong data | B25 (concrete: pages[0].url=="/intro", pages[1].url=="/guide") |
| `write_scraped_pages` — `join(".scrape")` → `join("scrape")` | Writes to non-hidden dir | B24 (scrape→diff roundtrip breaks) |
| `diff_directories` — off-by-one in summary counting | Wrong counts | B37 (100-page diff with exact removed==50, unchanged==50, added==50) |

### Mutation kill strategy:

1. **Every branch in `resolve_manifest_dir`** must have at least 2 tests (one for each
   branch outcome) — covered by B1/B2/B3/B4.
2. **Every `?` operator** that propagates errors from the helper must have a test that
   triggers it — covered by B14/B15/B20/B27/B28.
3. **The string literal `.scrape`** appears in both the producer and consumer — tests
   B21-B25 form a roundtrip that catches any change to this string.
4. **Every vague assertion** from the original plan now has exact concrete values —
   B6 asserts exact PathBuf, B10/B12 assert exact summary counts, B17/B25 assert exact
   page fields. No mutation can produce an empty/wrong value and survive.
5. **Boundary tests** B31-B34 catch path-encoding mutations (string concat vs Path::join),
   and B35-B37 catch edge cases in diff logic (empty manifests, same dir, large counts).

---

## 8. Combinatorial Coverage Matrix

### Matrix A: `resolve_manifest_dir`

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| direct only | path/manifest.json exists | Ok(path) | unit (B1) |
| nested only | path/.scrape/manifest.json exists | Ok(path/.scrape) | unit (B2) |
| neither | no manifests | Err(NotFound { path, scrape_subdir, direct, nested }) | unit (B3) |
| both exist | both files present | Ok(path) — direct wins | unit (B4) |
| relative path | "relativedir" with manifest | Ok("relativedir") | unit (B5) |
| absolute path | "/tmp/abstest" with manifest | Ok(PathBuf::from("/tmp/abstest")) | unit (B6) |
| error fields | empty dir | Err with all 4 paths populated | unit (B7) |
| determinism | any valid layout | identical on 2nd call | unit (B8) |
| no mutation | empty dir (error case) | directory unchanged | unit (B9) |
| nonexistent dir | "/nonexistent" | Err(NotFound) | unit (B3 variant) |
| boundary: path = "." | cwd/manifest.json exists | Ok(".") | unit |
| boundary: path = ".." | parent/manifest.json exists | Ok("..") | unit |
| boundary: trailing slash | "/tmp/output/" with .scrape/manifest.json | Ok(PathBuf::from("/tmp/output/.scrape")) | unit (B31) |
| boundary: spaces | "/tmp/my output" with .scrape/manifest.json | Ok(PathBuf::from("/tmp/my output/.scrape")) | unit (B32) |
| boundary: unicode | "/tmp/ドキュメント" with .scrape/manifest.json | Ok(PathBuf::from("/tmp/ドキュメント/.scrape")) | unit (B33) |
| boundary: path IS .scrape | "/tmp/output/.scrape" with manifest.json | Ok(PathBuf::from("/tmp/output/.scrape")) | unit (B34) |

### Matrix B: `diff_directories` (post-modification)

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| both scrape roots | dir_a/.scrape/manifest + dir_b/.scrape/manifest | Ok(ChangePlan) with concrete summary | integration (B10, B11) |
| both direct | dir_a/manifest + dir_b/manifest | Ok(ChangePlan) with removed==1, added==1, unchanged==1 | integration (B12) |
| mixed: a=scrape, b=direct | dir_a/.scrape/manifest + dir_b/manifest | Ok(ChangePlan) with modified==1 | integration (B13) |
| mixed: a=direct, b=scrape | dir_a/manifest + dir_b/.scrape/manifest | Ok(ChangePlan) | integration (B13) |
| dir_a unresolvable | dir_a has no manifest | Err("No manifest.json found") | integration (B14) |
| dir_b unresolvable | dir_b has no manifest | Err("No manifest.json found") | integration (B15) |
| invalid JSON in dir_b | dir_b/.scrape/manifest = "not json" | Err("Invalid manifest") | integration (B16) |
| dir_a nonexistent path | "/nonexistent" | Err | integration (B14 variant) |
| boundary: both empty | {"pages":[]} vs {"pages":[]} | Ok(plan) with summary all zeros, is_empty()==true | integration (B35) |
| boundary: same dir both args | dir == dir | Ok(plan) with unchanged==N, added/removed/modified==0 | integration (B36) |
| boundary: large manifests | 100 pages vs 100 pages (50 overlap) | Ok(plan) with removed==50, unchanged==50, added==50 | integration (B37) |

### Matrix C: `read_manifest` (post-modification)

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| scrape root | output_dir/.scrape/manifest.json | Ok(ScrapeResult) with pages[0].url=="/p1", pages[1].url=="/p2" | integration (B18) |
| direct dir | dir/manifest.json | Ok(ScrapeResult) with pages[0].url=="/only" | integration (B19) |
| unresolvable | empty dir | Err("No manifest.json found") | integration (B20) |
| invalid JSON | .scrape/manifest.json = "not json" | Err("Invalid manifest") | integration |
| empty manifest | .scrape/manifest.json = {} | Err or Ok with defaults | integration |

### Matrix D: `write_scraped_pages` → `resolve_manifest_dir` roundtrip

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| 1 page | ScrapeResult with 1 page | resolve on output_dir → Ok(output_dir/.scrape) | e2e (B24) |
| 0 pages | empty ScrapeResult | resolve on output_dir → Ok(output_dir/.scrape) | e2e |
| 50 pages | ScrapeResult with 50 pages | resolve on output_dir → Ok(output_dir/.scrape) | e2e |
| slug collisions | 2 pages with same slug | resolve succeeds, files have suffixes | integration |

### Matrix E: Summary conservation invariant (proptest)

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| any valid scrape roots | random pages 0-20 | added + modified + unchanged == total_current | proptest (P4) |
| empty both | 0 pages each | all zeros, is_empty() == true | proptest |
| identical content | same pages in both | added=0, removed=0, modified=0 | proptest |

---

## 9. Existing Test Coverage Assessment

The following tests in `watch_integration_tests.rs` and `src/watch/tests_diff.rs`
already cover the **unchanged** pure Calc layer functions. They do NOT need to be
duplicated, but the following existing tests will **break** when the code changes
and must be **updated** to use scrape root layout:

### Tests that need updating (currently use direct layout):

| Existing Test | File | Line | Change Required |
|--------------|------|------|-----------------|
| `diff_directories_compares_manifests` | `tests/watch_integration_tests.rs` | 180 | Also add variant with `.scrape/` layout |
| `diff_identical_directories_produces_empty_plan` | `tests/watch_integration_tests.rs` | 215 | Add scrape-root variant |
| `diff_missing_manifest_returns_error` | `tests/watch_integration_tests.rs` | 234 | Verify error message from new helper |
| `diff_invalid_manifest_returns_error` | `tests/watch_integration_tests.rs` | 246 | Add scrape-root variant |
| `diff_returns_error_when_dir_a_missing` | `tests/watch_integration_tests.rs` | 952 | Verify error includes resolution info |
| `diff_with_one_empty_manifest_and_one_large` | `tests/watch_integration_tests.rs` | 1540 | Add scrape-root variant |

### Tests that remain valid unchanged:

All unit tests for `compute_plan`, `snapshot_from_scrape`, `format_plan_markdown`,
`format_plan_json`, `write_plan_reports`, `ChangeSummary::is_empty`, and proptest
invariants — these test pure Calc functions unaffected by the path resolution change.

---

## 10. Open Questions

1. **`read_manifest` visibility**: Currently `fn` (private to `cmd/watch.rs`). Tests
   for B17-B20 may need to go through `run_apply` (integration) or `read_manifest`
   must be made `pub(crate)` for direct testing. **Recommendation**: Make it
   `pub(crate)` so the integration tests in `/tests/` can call it.

2. **StateDb dependency for B28-B30**: `run_apply` requires a real `StateDb`. The
   integration tests need to create a tempdir and open a StateDb. If StateDb
   initialization is expensive, consider a shared test fixture. Verify that the
   existing test infrastructure in `tests/` has access to `state::commit::StateDb`.

3. **Async test infrastructure**: `run_apply` is `async fn`. Tests B28-B30 require
   a tokio runtime. The existing `tests/` directory should use `#[tokio::test]`.

4. **ManifestResolveError display format**: The contract specifies an exact error
   message format with "Tip:" guidance. Tests in B7 assert string fragments.
   If the exact format changes during implementation, these tests need updating.
   Consider providing a `ManifestResolveError::candidate_paths()` accessor method
   so tests can assert fields programmatically rather than parsing error strings.

5. **Cross-platform path separators**: Tests B5/B6 assume Unix-style paths. On
   Windows, `.scrape` uses backslashes. The contract says "symlinks are followed
   by `std::fs::exists`" — verify this holds on the target platform (Linux CI).
