//! Integration tests for `compute_file_diff` and related diff operations.
//!
//! Covers scenarios 3.9–3.34 from the cdocs-2rt test plan.

#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use doc_transformer::cache::content_hash;
use doc_transformer::diff::{compute_file_diff, DiffError, FileDiff, StoredHashes};
use doc_transformer::discover::DiscoveryFile;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_discovery(source_path: &str, size_bytes: u64) -> DiscoveryFile {
    DiscoveryFile {
        source_path: source_path.to_string(),
        size_bytes,
    }
}

fn make_stored(content: &[u8], config: &[u8]) -> StoredHashes {
    StoredHashes {
        content_hash: content_hash(content),
        config_hash: content_hash(config),
    }
}

/// Write a file relative to `dir` with the given content bytes.
fn write_file(dir: &Path, relative: &str, content: &[u8]) {
    let path = dir.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent dirs");
    }
    fs::write(path, content).expect("write file");
}

// ---------------------------------------------------------------------------
// Scenario 3.9: SourceDirNotFound
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_returns_source_dir_not_found_when_dir_missing() {
    let missing = Path::new("/nonexistent/dir/xyz_42");
    let result = compute_file_diff(&[], missing, None, &HashMap::new());

    assert!(result.is_err(), "should be Err for missing source_dir");
    let err = result.expect_err("expected error");
    match &err {
        DiffError::SourceDirNotFound(path_str) => {
            assert!(
                path_str.contains("nonexistent"),
                "error message must reference the path: {path_str}"
            );
        }
        other => panic!("expected SourceDirNotFound, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Scenario 3.10: FileRead on missing file
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_returns_file_read_when_file_missing() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let files = vec![make_discovery("ghost.md", 10)];

    let result = compute_file_diff(&files, dir.path(), None, &HashMap::new());

    assert!(result.is_err());
    match result.expect_err("expected error") {
        DiffError::FileRead { path, source } => {
            assert_eq!(path, "ghost.md");
            assert_eq!(source.kind(), std::io::ErrorKind::NotFound);
        }
        other => panic!("expected FileRead, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Scenario 3.10b: FileRead on permission denied
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_returns_file_read_when_permission_denied() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    write_file(dir.path(), "secret.md", b"secret content");

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(
            dir.path().join("secret.md"),
            PermissionsExt::from_mode(0o000),
        )
        .expect("chmod");
    }

    let files = vec![make_discovery("secret.md", 100)];
    let result = compute_file_diff(&files, dir.path(), None, &HashMap::new());

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(
            dir.path().join("secret.md"),
            PermissionsExt::from_mode(0o644),
        );
    }

    assert!(result.is_err());
    match result.expect_err("expected error") {
        DiffError::FileRead { path, source } => {
            assert_eq!(path, "secret.md");
            #[cfg(unix)]
            assert_eq!(source.kind(), std::io::ErrorKind::PermissionDenied);
        }
        other => panic!("expected FileRead, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Scenario 3.11: PathTraversal on dotdot escape
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_returns_path_traversal_when_path_escapes_source_dir() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let files = vec![make_discovery("../../etc/passwd", 0)];

    let result = compute_file_diff(&files, dir.path(), None, &HashMap::new());
    assert!(result.is_err());
    match result.expect_err("expected error") {
        DiffError::PathTraversal { path } => {
            assert_eq!(path, "../../etc/passwd");
        }
        other => panic!("expected PathTraversal, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Scenario 3.12: Reject absolute path outside source dir
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_rejects_absolute_path_outside_source_dir() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let files = vec![make_discovery("/etc/passwd", 0)];

    let result = compute_file_diff(&files, dir.path(), None, &HashMap::new());
    assert!(result.is_err());
    match result.expect_err("expected error") {
        DiffError::PathTraversal { path } => {
            assert_eq!(path, "/etc/passwd");
        }
        other => panic!("expected PathTraversal, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Scenario 3.13: Reject dotdot prefix path
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_rejects_dotdot_path_traversal() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let files = vec![make_discovery("../outside.md", 0)];

    let result = compute_file_diff(&files, dir.path(), None, &HashMap::new());
    assert!(result.is_err());
    match result.expect_err("expected error") {
        DiffError::PathTraversal { path } => {
            assert_eq!(path, "../outside.md");
        }
        other => panic!("expected PathTraversal, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Scenario 3.14: All new when stored_hashes empty
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_classifies_all_new_when_stored_hashes_empty() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    write_file(dir.path(), "a.md", b"alpha");
    write_file(dir.path(), "b.md", b"beta");

    let files = vec![make_discovery("a.md", 5), make_discovery("b.md", 4)];
    let result =
        compute_file_diff(&files, dir.path(), None, &HashMap::new()).expect("diff should succeed");

    assert_eq!(result.new, HashSet::from(["a.md".into(), "b.md".into()]));
    assert!(result.unchanged.is_empty());
    assert!(result.changed.is_empty());
    assert!(result.deleted.is_empty());
}

// ---------------------------------------------------------------------------
// Scenario 3.15: All deleted when no discovered files
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_classifies_all_deleted_when_no_discovered_files() {
    let dir = tempfile::TempDir::new().expect("tempdir");

    let mut stored = HashMap::new();
    stored.insert("old.md".to_string(), make_stored(b"old content", b""));

    let result = compute_file_diff(&[], dir.path(), None, &stored).expect("diff should succeed");

    assert_eq!(result.deleted, HashSet::from(["old.md".into()]));
    assert!(result.unchanged.is_empty());
    assert!(result.changed.is_empty());
    assert!(result.new.is_empty());
}

// ---------------------------------------------------------------------------
// Scenario 3.16: Unchanged when hashes match
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_classifies_unchanged_when_hashes_match() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    write_file(dir.path(), "same.md", b"constant content");

    let files = vec![make_discovery("same.md", 16)];
    let stored = HashMap::from([("same.md".to_string(), make_stored(b"constant content", b""))]);

    let result = compute_file_diff(&files, dir.path(), None, &stored).expect("diff should succeed");

    assert_eq!(result.unchanged, HashSet::from(["same.md".into()]));
    assert!(result.changed.is_empty());
    assert!(result.new.is_empty());
    assert!(result.deleted.is_empty());
}

// ---------------------------------------------------------------------------
// Scenario 3.17: Changed when content hash differs
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_classifies_changed_when_content_hash_differs() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    write_file(dir.path(), "edit.md", b"new content");

    let files = vec![make_discovery("edit.md", 11)];
    let stored = HashMap::from([("edit.md".to_string(), make_stored(b"old content", b""))]);

    let result = compute_file_diff(&files, dir.path(), None, &stored).expect("diff should succeed");

    assert_eq!(result.changed, HashSet::from(["edit.md".into()]));
    assert!(result.unchanged.is_empty());
    assert!(result.new.is_empty());
    assert!(result.deleted.is_empty());
}

// ---------------------------------------------------------------------------
// Scenario 3.18: Changed when config hash differs
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_classifies_changed_when_config_hash_differs() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let source_dir = dir.path();

    write_file(source_dir, "stable.md", b"same bytes");

    // Config file with new content
    let config_path = source_dir.join("category.yaml");
    fs::write(&config_path, b"new config").expect("write config");

    let files = vec![make_discovery("stable.md", 9)];
    let stored = HashMap::from([(
        "stable.md".to_string(),
        StoredHashes {
            content_hash: content_hash(b"same bytes"),
            config_hash: content_hash(b"old config"),
        },
    )]);

    let result = compute_file_diff(&files, source_dir, Some(&config_path), &stored)
        .expect("diff should succeed");

    assert_eq!(result.changed, HashSet::from(["stable.md".into()]));
    assert!(result.unchanged.is_empty());
    assert!(result.new.is_empty());
    assert!(result.deleted.is_empty());
}

// ---------------------------------------------------------------------------
// Scenario 3.19: Changed when both hashes differ
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_classifies_changed_when_both_hashes_differ() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let source_dir = dir.path();

    write_file(source_dir, "both.md", b"new file bytes");

    let config_path = source_dir.join("category.yaml");
    fs::write(&config_path, b"new config bytes").expect("write config");

    let files = vec![make_discovery("both.md", 14)];
    let stored = HashMap::from([(
        "both.md".to_string(),
        StoredHashes {
            content_hash: content_hash(b"old file bytes"),
            config_hash: content_hash(b"old config bytes"),
        },
    )]);

    let result = compute_file_diff(&files, source_dir, Some(&config_path), &stored)
        .expect("diff should succeed");

    assert_eq!(result.changed, HashSet::from(["both.md".into()]));
    assert!(result.unchanged.is_empty());
    assert!(result.new.is_empty());
    assert!(result.deleted.is_empty());
}

// ---------------------------------------------------------------------------
// Scenario 3.20: New when absent from stored
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_classifies_new_when_not_in_stored_hashes() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    write_file(dir.path(), "fresh.md", b"brand new");

    let files = vec![make_discovery("fresh.md", 9)];
    let result =
        compute_file_diff(&files, dir.path(), None, &HashMap::new()).expect("diff should succeed");

    assert_eq!(result.new, HashSet::from(["fresh.md".into()]));
    assert!(result.unchanged.is_empty());
    assert!(result.changed.is_empty());
    assert!(result.deleted.is_empty());
}

// ---------------------------------------------------------------------------
// Scenario 3.21: Deleted when not discovered
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_classifies_deleted_when_not_in_discovered_files() {
    let dir = tempfile::TempDir::new().expect("tempdir");

    let stored = HashMap::from([
        ("gone.md".to_string(), make_stored(b"gone content", b"")),
        (
            "removed.md".to_string(),
            make_stored(b"removed content", b"cfg"),
        ),
    ]);

    let result = compute_file_diff(&[], dir.path(), None, &stored).expect("diff should succeed");

    assert_eq!(
        result.deleted,
        HashSet::from(["gone.md".into(), "removed.md".into()])
    );
    assert!(result.unchanged.is_empty());
    assert!(result.changed.is_empty());
    assert!(result.new.is_empty());
}

// ---------------------------------------------------------------------------
// Scenario 3.22: Partition invariant (mutually exclusive)
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_buckets_are_mutually_exclusive() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    write_file(dir.path(), "a.md", b"content a");
    write_file(dir.path(), "b.md", b"content b");

    let files = vec![make_discovery("a.md", 10), make_discovery("b.md", 10)];
    let stored = HashMap::from([("a.md".to_string(), make_stored(b"content a", b""))]);

    let result = compute_file_diff(&files, dir.path(), None, &stored).expect("diff should succeed");

    // All pairwise intersections must be empty
    assert_pairwise_disjoint(&result);
}

fn assert_pairwise_disjoint(diff: &FileDiff) {
    let buckets: [&HashSet<String>; 4] = [&diff.unchanged, &diff.changed, &diff.new, &diff.deleted];
    for i in 0..4 {
        for j in (i + 1)..4 {
            let intersection: HashSet<&String> = buckets[i].intersection(buckets[j]).collect();
            assert!(
                intersection.is_empty(),
                "buckets[{i}] ∩ buckets[{j}] must be empty, found: {intersection:?}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Scenario 3.23: Collective exhaustive invariant
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_buckets_are_collectively_exhaustive() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    write_file(dir.path(), "a.md", b"content a");
    write_file(dir.path(), "b.md", b"content b changed");

    let files = vec![make_discovery("a.md", 10), make_discovery("b.md", 10)];
    let stored = HashMap::from([
        ("a.md".to_string(), make_stored(b"content a", b"")),
        ("b.md".to_string(), make_stored(b"old b content", b"")),
        ("deleted.md".to_string(), make_stored(b"gone", b"")),
    ]);

    let discovered_set: HashSet<String> = files.iter().map(|f| f.source_path.clone()).collect();
    let result = compute_file_diff(&files, dir.path(), None, &stored).expect("diff should succeed");

    // unchanged ∪ changed ∪ new == discovered_set
    let discovered_union: HashSet<String> = result
        .unchanged
        .union(&result.changed)
        .cloned()
        .collect::<HashSet<_>>()
        .union(&result.new)
        .cloned()
        .collect();
    assert_eq!(
        discovered_union, discovered_set,
        "discovered union invariant"
    );

    // deleted == stored_keys - discovered_set
    let expected_deleted: HashSet<String> = stored
        .keys()
        .filter(|k| !discovered_set.contains(*k))
        .cloned()
        .collect();
    assert_eq!(result.deleted, expected_deleted, "deleted invariant");
}

// ---------------------------------------------------------------------------
// Scenario 3.24: Mixed scenario (all four buckets)
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_populates_all_four_buckets_in_mixed_scenario() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    write_file(dir.path(), "kept.md", b"same");
    write_file(dir.path(), "edited.md", b"changed");
    write_file(dir.path(), "added.md", b"new");

    let files = vec![
        make_discovery("kept.md", 4),
        make_discovery("edited.md", 7),
        make_discovery("added.md", 3),
    ];

    let stored = HashMap::from([
        ("kept.md".to_string(), make_stored(b"same", b"")),
        ("edited.md".to_string(), make_stored(b"original", b"")),
        (
            "removed.md".to_string(),
            make_stored(b"removed content", b""),
        ),
    ]);

    let result = compute_file_diff(&files, dir.path(), None, &stored).expect("diff should succeed");

    assert_eq!(result.unchanged, HashSet::from(["kept.md".into()]));
    assert_eq!(result.changed, HashSet::from(["edited.md".into()]));
    assert_eq!(result.new, HashSet::from(["added.md".into()]));
    assert_eq!(result.deleted, HashSet::from(["removed.md".into()]));
}

// ---------------------------------------------------------------------------
// Scenario 3.25: Both empty produces empty buckets
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_returns_empty_diff_when_both_inputs_empty() {
    let dir = tempfile::TempDir::new().expect("tempdir");

    let result =
        compute_file_diff(&[], dir.path(), None, &HashMap::new()).expect("diff should succeed");

    assert!(result.unchanged.is_empty());
    assert!(result.changed.is_empty());
    assert!(result.new.is_empty());
    assert!(result.deleted.is_empty());
}

// ---------------------------------------------------------------------------
// Scenario 3.26: Does not mutate inputs or disk
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_does_not_mutate_inputs_or_filesystem() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    write_file(dir.path(), "safe.md", b"immutable");

    let files = vec![make_discovery("safe.md", 10)];

    let mut stored = HashMap::new();
    stored.insert("safe.md".to_string(), make_stored(b"immutable", b""));
    let stored_clone = stored.clone();

    // Snapshot mtimes before
    let meta_before = fs::metadata(dir.path().join("safe.md")).expect("metadata before");
    let modified_before = meta_before.modified().expect("mtime before");

    let _result =
        compute_file_diff(&files, dir.path(), None, &stored).expect("diff should succeed");

    // stored_hashes unchanged
    assert_eq!(stored, stored_clone, "stored_hashes must not be mutated");

    // File mtime unchanged
    let meta_after = fs::metadata(dir.path().join("safe.md")).expect("metadata after");
    let modified_after = meta_after.modified().expect("mtime after");
    assert_eq!(
        modified_before, modified_after,
        "file modification time must not change"
    );

    // No new files created
    let entries: Vec<_> = fs::read_dir(dir.path())
        .expect("read_dir")
        .map(|e| e.expect("entry").file_name().to_string_lossy().to_string())
        .collect();
    assert!(
        !entries
            .iter()
            .any(|e| e.contains(".cache") || e.contains(".tmp")),
        "no new files should be created"
    );
}

// ---------------------------------------------------------------------------
// Scenario 3.27: Large file set produces correct partition
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_handles_large_file_set_correctly() {
    let dir = tempfile::TempDir::new().expect("tempdir");

    let mut files = Vec::new();
    let mut stored = HashMap::new();

    // 20 unchanged (indices 0..19)
    for i in 0..20 {
        let name = format!("unchanged_{i:02}.md");
        let content = format!("content_{i}");
        write_file(dir.path(), &name, content.as_bytes());
        files.push(make_discovery(&name, content.len() as u64));
        stored.insert(name, make_stored(content.as_bytes(), b""));
    }

    // 15 changed (indices 0..14)
    for i in 0..15 {
        let name = format!("changed_{i:02}.md");
        let old_content = format!("old_{i}");
        let new_content = format!("new_{i}");
        write_file(dir.path(), &name, new_content.as_bytes());
        files.push(make_discovery(&name, new_content.len() as u64));
        stored.insert(name, make_stored(old_content.as_bytes(), b""));
    }

    // 15 new (indices 0..14)
    for i in 0..15 {
        let name = format!("fresh_{i:02}.md");
        let content = format!("brand_new_{i}");
        write_file(dir.path(), &name, content.as_bytes());
        files.push(make_discovery(&name, content.len() as u64));
    }

    // 10 deleted (in stored but not discovered)
    for i in 0..10 {
        let name = format!("deleted_{i:02}.md");
        stored.insert(name, make_stored(format!("removed_{i}").as_bytes(), b""));
    }

    let result = compute_file_diff(&files, dir.path(), None, &stored).expect("diff should succeed");

    assert_eq!(result.unchanged.len(), 20, "unchanged count");
    assert_eq!(result.changed.len(), 15, "changed count");
    assert_eq!(result.new.len(), 15, "new count");
    assert_eq!(result.deleted.len(), 10, "deleted count");
    assert_pairwise_disjoint(&result);
}

// ---------------------------------------------------------------------------
// Scenario 3.28: Duplicate source_path produces deterministic result
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_produces_deterministic_result_when_duplicate_source_paths() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    write_file(dir.path(), "dup.md", b"content-hash-value");

    let files = vec![make_discovery("dup.md", 100), make_discovery("dup.md", 200)];

    let stored = HashMap::from([(
        "dup.md".to_string(),
        make_stored(b"content-hash-value", b""),
    )]);

    // Run 10 times — all results must be identical
    let first = compute_file_diff(&files, dir.path(), None, &stored).expect("diff 1");
    for iteration in 1..=9 {
        let result = compute_file_diff(&files, dir.path(), None, &stored)
            .unwrap_or_else(|e| panic!("diff {iteration} failed: {e}"));
        assert_eq!(
            result, first,
            "result must be deterministic across iteration {iteration}"
        );
    }

    // "dup.md" appears in exactly one bucket
    let count = first.unchanged.contains("dup.md") as usize
        + first.changed.contains("dup.md") as usize
        + first.new.contains("dup.md") as usize
        + first.deleted.contains("dup.md") as usize;
    assert_eq!(count, 1, "dup.md must appear in exactly one bucket");
    assert!(
        first.unchanged.contains("dup.md"),
        "dup.md should be unchanged (hashes match)"
    );
}

// ---------------------------------------------------------------------------
// Scenario 3.29: Config path nonexistent → Changed classification
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_classifies_changed_when_config_path_points_to_missing_file() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    write_file(dir.path(), "doc.md", b"unchanged content");

    let missing_config = dir.path().join("nonexistent_config.yaml");

    let files = vec![make_discovery("doc.md", 17)];
    let stored = HashMap::from([(
        "doc.md".to_string(),
        StoredHashes {
            content_hash: content_hash(b"unchanged content"),
            config_hash: content_hash(b"real config bytes"),
        },
    )]);

    let result = compute_file_diff(&files, dir.path(), Some(&missing_config), &stored)
        .expect("diff should succeed");

    assert_eq!(result.changed, HashSet::from(["doc.md".into()]));
    assert!(result.unchanged.is_empty());
    assert!(result.new.is_empty());
    assert!(result.deleted.is_empty());
}

// ---------------------------------------------------------------------------
// Scenario 3.30: Symlink traversal rejection (Unix only)
// ---------------------------------------------------------------------------

#[cfg(target_family = "unix")]
#[test]
fn compute_file_diff_rejects_symlink_traversal() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::TempDir::new().expect("tempdir");
    let link_path = dir.path().join("link.md");
    symlink("/etc/passwd", &link_path).expect("create symlink");

    let files = vec![make_discovery("link.md", 0)];
    let result = compute_file_diff(&files, dir.path(), None, &HashMap::new());

    assert!(result.is_err(), "symlink traversal should fail");
    match result.expect_err("expected error") {
        DiffError::PathTraversal { path } => {
            assert_eq!(path, "link.md");
        }
        other => panic!("expected PathTraversal, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Scenario 3.31: Empty source_path boundary
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_returns_error_when_source_path_is_empty() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let files = vec![make_discovery("", 0)];

    let result = compute_file_diff(&files, dir.path(), None, &HashMap::new());

    // Accept either FileRead or PathTraversal — both are valid for empty path
    assert!(result.is_err(), "empty source_path must fail");
    match result.expect_err("expected error") {
        DiffError::FileRead { path, .. } | DiffError::PathTraversal { path } => {
            assert_eq!(path, "");
        }
        other => panic!("expected FileRead or PathTraversal, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Scenario 3.32: Very long source_path (PATH_MAX)
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_does_not_panic_when_source_path_exceeds_path_max() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let long_path = "a".repeat(4096);
    let files = vec![make_discovery(&long_path, 0)];

    let result = compute_file_diff(&files, dir.path(), None, &HashMap::new());

    // Must not panic; must be Err
    assert!(result.is_err(), "PATH_MAX source_path must fail, not panic");
}

// ---------------------------------------------------------------------------
// Scenario 3.33: Mismatched stored hash keys don't panic
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_never_panics_on_mismatched_stored_hash_keys() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    write_file(dir.path(), "real_file.md", b"hello");

    let files = vec![make_discovery("real_file.md", 5)];

    let stored = HashMap::from([
        ("REAL_FILE.MD".to_string(), make_stored(b"hello", b"")),
        ("./real_file.md".to_string(), make_stored(b"hello", b"")),
        (
            "subdir/../real_file.md".to_string(),
            make_stored(b"hello", b""),
        ),
    ]);

    let result = compute_file_diff(&files, dir.path(), None, &stored).expect("diff should succeed");

    // real_file.md is New (not in stored under that exact key)
    assert!(
        result.new.contains("real_file.md"),
        "real_file.md should be New (key mismatch)"
    );
    // All stored keys are Deleted
    assert_eq!(
        result.deleted.len(),
        3,
        "all mismatched keys should be deleted"
    );
    assert!(result.deleted.contains("REAL_FILE.MD"));
    assert!(result.deleted.contains("./real_file.md"));
    assert!(result.deleted.contains("subdir/../real_file.md"));
    assert!(result.unchanged.is_empty());
    assert!(result.changed.is_empty());
}

// ---------------------------------------------------------------------------
// Scenario 3.34: size_bytes = 0 does not affect classification
// ---------------------------------------------------------------------------

#[test]
fn compute_file_diff_ignores_size_bytes_and_classifies_by_content_hash() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    write_file(dir.path(), "sized.md", b"actual content here");

    // size_bytes = 0 is incorrect but should not affect classification
    let files = vec![make_discovery("sized.md", 0)];
    let stored = HashMap::from([(
        "sized.md".to_string(),
        make_stored(b"actual content here", b""),
    )]);

    let result = compute_file_diff(&files, dir.path(), None, &stored).expect("diff should succeed");

    assert_eq!(result.unchanged, HashSet::from(["sized.md".into()]));
    assert!(result.changed.is_empty());
    assert!(result.new.is_empty());
    assert!(result.deleted.is_empty());
}
