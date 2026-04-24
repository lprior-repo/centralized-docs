//! Tests for resolve_manifest_dir (B1-B9, B31-B34).

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

fn write_test_manifest(dir: &std::path::Path) {
    let result = make_result("https://example.com", vec![]);
    let file = std::fs::File::create(dir.join("manifest.json")).expect("create manifest");
    serde_json::to_writer_pretty(file, &result).expect("write manifest");
}

#[test]
fn resolve_manifest_dir_returns_input_path_when_direct_manifest_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    write_test_manifest(dir.path());

    let result = resolve_manifest_dir(dir.path());

    assert_eq!(result, Ok(dir.path().to_path_buf()));
}

#[test]
fn resolve_manifest_dir_returns_scrape_subdir_when_only_nested_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    let scrape_dir = dir.path().join(".scrape");
    std::fs::create_dir_all(&scrape_dir).expect("create .scrape");
    write_test_manifest(&scrape_dir);

    let result = resolve_manifest_dir(dir.path());

    assert_eq!(result, Ok(dir.path().join(".scrape")));
}

#[test]
fn resolve_manifest_dir_returns_not_found_when_neither_exists() {
    let dir = tempfile::tempdir().expect("tempdir");

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

#[test]
fn resolve_manifest_dir_prefers_direct_when_both_exist() {
    let dir = tempfile::tempdir().expect("tempdir");

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

    assert_eq!(result, Ok(dir.path().to_path_buf()));

    let resolved = result.expect("ok");
    let manifest: ScrapeResult = {
        let f = std::fs::File::open(resolved.join("manifest.json")).expect("open");
        serde_json::from_reader(f).expect("parse")
    };
    assert_eq!(manifest.base_url, "https://direct.example.com");
    assert_eq!(manifest.pages.len(), 1);
    assert_eq!(manifest.pages[0].url, "https://direct.example.com/a");
}

#[test]
fn resolve_manifest_dir_preserves_relative_path_form() {
    let dir = tempfile::tempdir().expect("tempdir");
    write_test_manifest(dir.path());

    let cwd = std::env::current_dir().expect("cwd");
    let canonical_dir = dir.path().canonicalize().expect("canonicalize dir");
    let relative = if let Ok(rel) = canonical_dir.strip_prefix(&cwd) {
        std::path::PathBuf::from(rel)
    } else {
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

#[test]
fn resolve_manifest_dir_preserves_absolute_path_form() {
    let dir = tempfile::tempdir().expect("tempdir");
    write_test_manifest(dir.path());

    let abs_path = dir.path().canonicalize().expect("canonicalize");
    let result = resolve_manifest_dir(&abs_path);

    assert_eq!(result, Ok(abs_path.clone()));
}

#[test]
fn resolve_manifest_dir_not_found_error_contains_both_candidate_paths() {
    let dir = tempfile::tempdir().expect("tempdir");
    let err = resolve_manifest_dir(dir.path()).expect_err("should be NotFound");
    let msg = err.to_string();
    let path_str = dir.path().to_string_lossy();
    assert!(msg.contains(&*path_str), "must contain path, got: {msg:?}");
    assert!(msg.contains(".scrape"), "must mention .scrape, got: {msg:?}");
    assert!(msg.contains("Tip:"), "must contain actionable tip, got: {msg:?}");
    let direct = dir.path().join("manifest.json").to_string_lossy().to_string();
    let nested = dir.path().join(".scrape").join("manifest.json").to_string_lossy().to_string();
    assert!(msg.contains(&direct), "must contain direct candidate {direct:?}");
    assert!(msg.contains(&nested), "must contain nested candidate {nested:?}");
}

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

#[test]
fn resolve_manifest_dir_creates_no_files() {
    let dir = tempfile::tempdir().expect("tempdir");

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

#[test]
fn resolve_manifest_dir_handles_trailing_slash_in_path() {
    let dir = tempfile::tempdir().expect("tempdir");
    let scrape_dir = dir.path().join(".scrape");
    std::fs::create_dir_all(&scrape_dir).expect("create .scrape");
    write_test_manifest(&scrape_dir);

    let path_with_slash = std::path::PathBuf::from(format!("{}/", dir.path().display()));

    let result = resolve_manifest_dir(&path_with_slash);

    let expected = dir.path().join(".scrape");
    assert_eq!(result, Ok(expected));
}

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

#[test]
fn resolve_manifest_dir_resolves_when_path_is_scrape_dir_itself() {
    let dir = tempfile::tempdir().expect("tempdir");
    let scrape_dir = dir.path().join(".scrape");
    std::fs::create_dir_all(&scrape_dir).expect("create .scrape");
    write_test_manifest(&scrape_dir);

    let result = resolve_manifest_dir(&scrape_dir);

    assert_eq!(result, Ok(scrape_dir.clone()));
    assert_eq!(
        result.expect("ok").join("manifest.json"),
        scrape_dir.join("manifest.json")
    );
}
