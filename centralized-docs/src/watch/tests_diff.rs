//! Tests for diff computation and snapshot handling.

use super::*;
use crate::scrape::validation::{ScrapeResult, ScrapedPage};
use crate::watch::diff::{resolve_manifest_dir, ManifestResolveError};

fn make_page(url: &str, title: &str, content: &str) -> ScrapedPage {
    ScrapedPage {
        url: url.to_string(),
        markdown: content.to_string(),
        title: title.to_string(),
        links: vec![],
        headers: vec![],
        word_count: content.split_whitespace().count(),
        slug: url.replace('/', "_"),
        filter_status: crate::scrape::validation::PageFilterStatus::Unfiltered,
        elements_removed: 0,
        density_score: 1.0,
    }
}

fn make_result(base: &str, pages: Vec<ScrapedPage>) -> ScrapeResult {
    ScrapeResult {
        total_urls: pages.len(),
        success_count: pages.len(),
        error_count: 0,
        errors: vec![],
        base_url: base.to_string(),
        pages,
    }
}

fn make_snapshot(target: &str, pages: &[(&str, &str, &str)]) -> Snapshot {
    let result = make_result(
        target,
        pages
            .iter()
            .map(|(url, title, content)| make_page(url, title, content))
            .collect(),
    );
    snapshot_from_scrape(target, &result)
}

#[test]
fn snapshot_from_scrape_produces_identical_hashes_for_same_input() {
    let result = make_result(
        "https://example.com",
        vec![
            make_page("https://example.com/a", "Page A", "content a"),
            make_page("https://example.com/b", "Page B", "content b"),
        ],
    );

    let snap1 = snapshot_from_scrape("https://example.com", &result);
    let snap2 = snapshot_from_scrape("https://example.com", &result);

    assert_eq!(
        snap1.pages, snap2.pages,
        "same scrape must produce identical page hashes"
    );
}

#[test]
fn compute_plan_returns_empty_changes_when_content_is_identical() {
    let pages = vec![
        make_page("https://example.com/a", "Page A", "hello"),
        make_page("https://example.com/b", "Page B", "world"),
    ];
    let prev = make_snapshot(
        "https://example.com",
        &[
            ("https://example.com/a", "Page A", "hello"),
            ("https://example.com/b", "Page B", "world"),
        ],
    );
    let current = make_result("https://example.com", pages);

    let plan = compute_plan("https://example.com", &prev, &current);

    assert!(plan.changes.is_empty());
    assert!(plan.summary.is_empty());
    assert_eq!(plan.summary.unchanged, 2);
}

#[test]
fn compute_plan_detects_added_page_when_new_url_appears_in_current() {
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/a", "Page A", "hello")],
    );
    let current = make_result(
        "https://example.com",
        vec![
            make_page("https://example.com/a", "Page A", "hello"),
            make_page("https://example.com/b", "Page B", "new page"),
        ],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    assert_eq!(plan.summary.added, 1);
    assert_eq!(plan.summary.removed, 0);
    assert_eq!(plan.summary.modified, 0);
    assert_eq!(plan.summary.unchanged, 1);
    assert_eq!(plan.changes[0].url, "https://example.com/b");
    assert_eq!(plan.changes[0].kind, ChangeKind::Added);
}

#[test]
fn compute_plan_detects_removed_page_when_url_disappears_from_current() {
    let prev = make_snapshot(
        "https://example.com",
        &[
            ("https://example.com/a", "Page A", "hello"),
            ("https://example.com/b", "Page B", "world"),
        ],
    );
    let current = make_result(
        "https://example.com",
        vec![make_page("https://example.com/a", "Page A", "hello")],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    assert_eq!(plan.summary.added, 0);
    assert_eq!(plan.summary.removed, 1);
    assert_eq!(plan.summary.modified, 0);
    assert_eq!(plan.summary.unchanged, 1);
    assert_eq!(plan.changes[0].url, "https://example.com/b");
    assert_eq!(plan.changes[0].kind, ChangeKind::Removed);
}

#[test]
fn compute_plan_detects_modified_page_when_content_hash_differs() {
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/a", "Page A", "old content")],
    );
    let current = make_result(
        "https://example.com",
        vec![make_page("https://example.com/a", "Page A", "new content")],
    );

    let plan = compute_plan("https://example.com", &prev, &current);

    assert_eq!(plan.summary.added, 0);
    assert_eq!(plan.summary.removed, 0);
    assert_eq!(plan.summary.modified, 1);
    assert_eq!(plan.summary.unchanged, 0);
    assert_eq!(plan.changes[0].kind, ChangeKind::Modified);
    assert!(plan.changes[0].old_hash.is_some());
    assert!(plan.changes[0].new_hash.is_some());
    assert_ne!(plan.changes[0].old_hash, plan.changes[0].new_hash);
}

#[test]
fn compute_plan_produces_identical_empty_plans_for_same_inputs() {
    let prev = make_snapshot(
        "https://example.com",
        &[("https://example.com/a", "Page A", "content")],
    );

    let scrape1 = make_result(
        "https://example.com",
        vec![make_page("https://example.com/a", "Page A", "content")],
    );
    let scrape2 = make_result(
        "https://example.com",
        vec![make_page("https://example.com/a", "Page A", "content")],
    );

    let plan1 = compute_plan("https://example.com", &prev, &scrape1);
    let plan2 = compute_plan("https://example.com", &prev, &scrape2);

    assert!(plan1.changes.is_empty());
    assert!(plan2.changes.is_empty());
}

#[test]
fn compute_plan_marks_all_pages_as_added_when_previous_is_empty() {
    let empty = Snapshot {
        target_url: "https://example.com".to_string(),
        timestamp: chrono::Utc::now(),
        pages: std::collections::BTreeMap::new(),
    };
    let current = make_result(
        "https://example.com",
        vec![
            make_page("https://example.com/a", "Page A", "hello"),
            make_page("https://example.com/b", "Page B", "world"),
        ],
    );

    let plan = compute_plan("https://example.com", &empty, &current);

    assert_eq!(plan.summary.added, 2);
    assert_eq!(plan.summary.removed, 0);
    assert_eq!(plan.summary.unchanged, 0);
}

#[test]
fn compute_plan_marks_all_pages_as_removed_when_current_is_empty() {
    let prev = make_snapshot(
        "https://example.com",
        &[
            ("https://example.com/a", "Page A", "hello"),
            ("https://example.com/b", "Page B", "world"),
        ],
    );
    let current = make_result("https://example.com", vec![]);

    let plan = compute_plan("https://example.com", &prev, &current);

    assert_eq!(plan.summary.removed, 2);
    assert_eq!(plan.summary.added, 0);
    assert_eq!(plan.summary.total_current, 0);
    assert_eq!(plan.summary.total_previous, 2);
}

#[test]
fn snapshot_serialization_roundtrip_preserves_all_page_data() {
    let snapshot = make_snapshot(
        "https://example.com",
        &[
            ("https://example.com/a", "Page A", "hello"),
            ("https://example.com/b", "Page B", "world"),
        ],
    );

    let json = serde_json::to_string(&snapshot).expect("serialize");
    let restored: Snapshot = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(snapshot.target_url, restored.target_url);
    assert_eq!(
        snapshot.pages, restored.pages,
        "all page hashes must survive JSON roundtrip"
    );
}

// ===========================================================================
// Unit tests for resolve_manifest_dir (B1–B9, B31–B34)
// ===========================================================================

/// Helper: write a valid manifest.json into `dir`.
fn write_test_manifest(dir: &std::path::Path) {
    let result = make_result("https://example.com", vec![]);
    let file = std::fs::File::create(dir.join("manifest.json")).expect("create manifest");
    serde_json::to_writer_pretty(file, &result).expect("write manifest");
}

// B1: resolve_manifest_dir returns input path unchanged when path/manifest.json exists
#[test]
fn resolve_manifest_dir_returns_input_path_when_direct_manifest_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    write_test_manifest(dir.path());

    let result = resolve_manifest_dir(dir.path());

    assert_eq!(result, Ok(dir.path().to_path_buf()));
}

// B2: resolve_manifest_dir returns path/.scrape when only path/.scrape/manifest.json exists
#[test]
fn resolve_manifest_dir_returns_scrape_subdir_when_only_nested_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    let scrape_dir = dir.path().join(".scrape");
    std::fs::create_dir_all(&scrape_dir).expect("create .scrape");
    write_test_manifest(&scrape_dir);

    let result = resolve_manifest_dir(dir.path());

    assert_eq!(result, Ok(dir.path().join(".scrape")));
}

// B3: resolve_manifest_dir returns NotFound when neither candidate exists
#[test]
fn resolve_manifest_dir_returns_not_found_when_neither_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    // No manifest anywhere

    let result = resolve_manifest_dir(dir.path());

    let err = result.expect_err("should be NotFound");
    assert_eq!(
        err,
        ManifestResolveError::NotFound {
            path: dir.path().to_path_buf(),
            scrape_subdir: dir.path().join(".scrape"),
            direct: dir.path().join("manifest.json"),
            nested: dir.path().join(".scrape").join("manifest.json"),
        }
    );
}

// B4: resolve_manifest_dir prefers direct match when BOTH exist
#[test]
fn resolve_manifest_dir_prefers_direct_when_both_exist() {
    let dir = tempfile::tempdir().expect("tempdir");

    // Write DIFFERENT content to each manifest to prove which one is found
    let direct_result = make_result(
        "https://direct.example.com",
        vec![make_page(
            "https://direct.example.com/a",
            "Direct Page",
            "direct content",
        )],
    );
    let direct_file =
        std::fs::File::create(dir.path().join("manifest.json")).expect("create direct");
    serde_json::to_writer_pretty(direct_file, &direct_result).expect("write direct");

    let scrape_dir = dir.path().join(".scrape");
    std::fs::create_dir_all(&scrape_dir).expect("create .scrape");
    let nested_result = make_result(
        "https://nested.example.com",
        vec![make_page(
            "https://nested.example.com/b",
            "Nested Page",
            "nested content",
        )],
    );
    let nested_file =
        std::fs::File::create(scrape_dir.join("manifest.json")).expect("create nested");
    serde_json::to_writer_pretty(nested_file, &nested_result).expect("write nested");

    let result = resolve_manifest_dir(dir.path());

    // Direct match takes precedence — returns the parent dir, NOT .scrape
    assert_eq!(result, Ok(dir.path().to_path_buf()));

    // Verify it actually resolves to the DIRECT manifest (not nested)
    let resolved = result.expect("ok");
    let manifest: ScrapeResult = {
        let f = std::fs::File::open(resolved.join("manifest.json")).expect("open");
        serde_json::from_reader(f).expect("parse")
    };
    assert_eq!(manifest.base_url, "https://direct.example.com");
    assert_eq!(manifest.pages.len(), 1);
    assert_eq!(manifest.pages[0].url, "https://direct.example.com/a");
}

// B5: resolve_manifest_dir preserves relative path form
#[test]
fn resolve_manifest_dir_preserves_relative_path_form() {
    // Use a tempdir and canonicalize to find cwd-relative path portably.
    // We create the tempdir, canonicalize both cwd and tempdir, compute the
    // relative path, then verify resolve_manifest_dir returns a relative path.

    let dir = tempfile::tempdir().expect("tempdir");
    write_test_manifest(dir.path());

    // Try to build a relative path from cwd
    let cwd = std::env::current_dir().expect("cwd");
    let canonical_dir = dir.path().canonicalize().expect("canonicalize dir");
    let relative = if let Ok(rel) = canonical_dir.strip_prefix(&cwd) {
        std::path::PathBuf::from(rel)
    } else {
        // If tempdir is outside cwd (e.g. /tmp vs /home), create under cwd instead
        let under_cwd = cwd.join("test_relative_manifest_tmp_b5");
        std::fs::create_dir_all(&under_cwd).expect("create dir");
        write_test_manifest(&under_cwd);
        let r = resolve_manifest_dir(std::path::Path::new("test_relative_manifest_tmp_b5"));
        let resolved = r.expect("should resolve manifest for relative path");
        assert!(
            !resolved.is_absolute(),
            "relative path input must produce relative path output, got: {resolved:?}"
        );
        std::fs::remove_dir_all(&under_cwd).ok();
        return;
    };

    let result = resolve_manifest_dir(&relative);
    let resolved = result.expect("resolve_manifest_dir must succeed for relative path");
    assert!(
        !resolved.is_absolute(),
        "relative path input must produce relative path output, got: {resolved:?}"
    );
}

// B6: resolve_manifest_dir preserves absolute path form
#[test]
fn resolve_manifest_dir_preserves_absolute_path_form() {
    let dir = tempfile::tempdir().expect("tempdir");
    write_test_manifest(dir.path());

    let abs_path = dir.path().canonicalize().expect("canonicalize");
    let result = resolve_manifest_dir(&abs_path);

    assert_eq!(result, Ok(abs_path.clone()));
}

// B7: resolve_manifest_dir NotFound error contains both candidate paths in message
#[test]
fn resolve_manifest_dir_not_found_error_contains_both_candidate_paths() {
    let dir = tempfile::tempdir().expect("tempdir");
    // No manifest anywhere

    let result = resolve_manifest_dir(dir.path());

    let err = result.expect_err("should be NotFound");
    let msg = err.to_string();

    // Must contain the path string
    let path_str = dir.path().to_string_lossy();
    assert!(
        msg.contains(&*path_str),
        "error message must contain path {path_str:?}, got: {msg:?}"
    );

    // Must contain .scrape subdirectory reference
    assert!(
        msg.contains(".scrape"),
        "error message must mention .scrape, got: {msg:?}"
    );

    // Must contain actionable Tip
    assert!(
        msg.contains("Tip:"),
        "error message must contain actionable tip, got: {msg:?}"
    );

    // Must contain both candidate paths
    let direct_str = dir
        .path()
        .join("manifest.json")
        .to_string_lossy()
        .to_string();
    let nested_str = dir
        .path()
        .join(".scrape")
        .join("manifest.json")
        .to_string_lossy()
        .to_string();
    assert!(
        msg.contains(&direct_str),
        "error message must contain direct candidate {direct_str:?}, got: {msg:?}"
    );
    assert!(
        msg.contains(&nested_str),
        "error message must contain nested candidate {nested_str:?}, got: {msg:?}"
    );
}

// B8: resolve_manifest_dir is deterministic — same filesystem state, same result
#[test]
fn resolve_manifest_dir_is_deterministic() {
    let dir = tempfile::tempdir().expect("tempdir");
    let scrape_dir = dir.path().join(".scrape");
    std::fs::create_dir_all(&scrape_dir).expect("create .scrape");
    write_test_manifest(&scrape_dir);

    let result1 = resolve_manifest_dir(dir.path());
    let result2 = resolve_manifest_dir(dir.path());

    assert_eq!(result1, result2);
    assert_eq!(result1, Ok(dir.path().join(".scrape")));
}

// B9: resolve_manifest_dir creates no files (read-only)
#[test]
fn resolve_manifest_dir_creates_no_files() {
    let dir = tempfile::tempdir().expect("tempdir");
    // Empty directory — resolution will fail

    let entries_before: Vec<_> = std::fs::read_dir(dir.path())
        .expect("read dir")
        .map(|e| e.expect("entry").file_name())
        .collect();

    let result = resolve_manifest_dir(dir.path());
    assert!(
        matches!(result, Err(ManifestResolveError::NotFound { .. })),
        "resolve_manifest_dir should fail with NotFound for empty directory"
    );

    let entries_after: Vec<_> = std::fs::read_dir(dir.path())
        .expect("read dir")
        .map(|e| e.expect("entry").file_name())
        .collect();

    assert_eq!(
        entries_before, entries_after,
        "resolve_manifest_dir must not create any files"
    );
}

// B31: resolve_manifest_dir handles trailing slash in path
#[test]
fn resolve_manifest_dir_handles_trailing_slash_in_path() {
    let dir = tempfile::tempdir().expect("tempdir");
    let scrape_dir = dir.path().join(".scrape");
    std::fs::create_dir_all(&scrape_dir).expect("create .scrape");
    write_test_manifest(&scrape_dir);

    // Add trailing slash to path
    let path_with_slash = std::path::PathBuf::from(format!("{}/", dir.path().display()));

    let result = resolve_manifest_dir(&path_with_slash);

    // Should resolve to .scrape regardless of trailing slash
    let expected = dir.path().join(".scrape");
    assert_eq!(result, Ok(expected));
}

// B32: resolve_manifest_dir handles spaces in directory name
#[test]
fn resolve_manifest_dir_handles_spaces_in_directory_name() {
    let dir = tempfile::tempdir().expect("tempdir");
    let spaced_dir = dir.path().join("my output");
    std::fs::create_dir_all(&spaced_dir).expect("create spaced dir");
    let scrape_dir = spaced_dir.join(".scrape");
    std::fs::create_dir_all(&scrape_dir).expect("create .scrape");
    write_test_manifest(&scrape_dir);

    let result = resolve_manifest_dir(&spaced_dir);

    assert_eq!(result, Ok(spaced_dir.join(".scrape")));
}

// B33: resolve_manifest_dir handles unicode in directory name
#[test]
fn resolve_manifest_dir_handles_unicode_in_directory_name() {
    let dir = tempfile::tempdir().expect("tempdir");
    let unicode_dir = dir.path().join("ドキュメント");
    std::fs::create_dir_all(&unicode_dir).expect("create unicode dir");
    let scrape_dir = unicode_dir.join(".scrape");
    std::fs::create_dir_all(&scrape_dir).expect("create .scrape");
    write_test_manifest(&scrape_dir);

    let result = resolve_manifest_dir(&unicode_dir);

    assert_eq!(result, Ok(unicode_dir.join(".scrape")));
}

// B34: resolve_manifest_dir resolves when path IS the .scrape directory itself
#[test]
fn resolve_manifest_dir_resolves_when_path_is_scrape_dir_itself() {
    let dir = tempfile::tempdir().expect("tempdir");
    let scrape_dir = dir.path().join(".scrape");
    std::fs::create_dir_all(&scrape_dir).expect("create .scrape");
    write_test_manifest(&scrape_dir);

    // Pass the .scrape directory ITSELF as the path
    let result = resolve_manifest_dir(&scrape_dir);

    // Should find manifest.json directly in .scrape/ and return the input path unchanged
    assert_eq!(result, Ok(scrape_dir.clone()));
    // Verify the joined manifest path is correct
    assert_eq!(
        result.expect("ok").join("manifest.json"),
        scrape_dir.join("manifest.json")
    );
}
