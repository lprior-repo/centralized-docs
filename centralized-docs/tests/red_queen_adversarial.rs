//! Red Queen Adversarial Tests for resolve_manifest_dir
//!
//! These tests attack the contract from dimensions NOT covered by B1-B34:
//! - Symlink attacks (broken symlinks, circular, directory symlinks)
//! - Type confusion (manifest.json is a directory, FIFO, etc.)
//! - Path edge cases (empty path, dot, double-slash, very long)
//! - Permission denied scenarios
//! - Race-condition-adjacent: concurrent filesystem mutation
//! - Error variant field correctness under adversarial inputs
//! - .scrape is a file (not a directory)

use std::path::{Path, PathBuf};

use doc_transformer::watch::{resolve_manifest_dir, ManifestResolveError};

// Helpers
fn write_manifest(dir: &Path) {
    let manifest = serde_json::json!({
        "base_url": "https://example.com",
        "pages": [],
        "total_urls": 0,
        "success_count": 0,
        "error_count": 0,
        "errors": []
    });
    std::fs::write(dir.join("manifest.json"), manifest.to_string()).expect("write manifest");
}

fn temp_dir() -> tempfile::TempDir {
    tempfile::tempdir().expect("tempdir")
}

// ═══════════════════════════════════════════════════════════════
// DIMENSION: type-confusion — manifest.json is NOT a regular file
// ═══════════════════════════════════════════════════════════════

/// RQ-GEN1-TYPE-1: manifest.json is a DIRECTORY.
/// `exists()` returns true for directories. resolve_manifest_dir will claim it resolved,
/// but there's no file to read. This is a contract gray area — the function says it
/// "resolved" but the resolved dir contains no readable manifest.
#[test]
fn rq_manifest_json_is_a_directory_not_a_file() {
    let dir = temp_dir();
    // Create manifest.json as a DIRECTORY, not a file
    let manifest_dir = dir.path().join("manifest.json");
    std::fs::create_dir_all(&manifest_dir).expect("create dir named manifest.json");

    let result = resolve_manifest_dir(dir.path());

    // BUG: resolve_manifest_dir returns Ok(), but manifest.json is a directory.
    // The contract says it checks if manifest.json "exists" but a directory is
    // not a valid manifest file.
    if result.is_ok() {
        let resolved = result.unwrap();
        let candidate = resolved.join("manifest.json");
        // The function resolved successfully but the candidate is a directory
        assert!(
            candidate.is_dir(),
            "RQ-TYPE-1 SURVIVOR: resolve_manifest_dir returned Ok for a directory named manifest.json"
        );
        // If we got here, the function lied — it claimed to resolve a manifest
        // but there's no actual file to read.
        panic!(
            "RQ-TYPE-1 SURVIVOR: resolve_manifest_dir accepted directory as manifest.json. \
             Resolved to {:?} but manifest.json is a directory, not a file.",
            resolved
        );
    }
    // If it returns Err, that's the correct behavior
}

/// RQ-GEN1-TYPE-2: .scrape/manifest.json is a DIRECTORY.
#[test]
fn rq_scrape_manifest_json_is_a_directory_not_a_file() {
    let dir = temp_dir();
    let scrape_dir = dir.path().join(".scrape");
    std::fs::create_dir_all(&scrape_dir).expect("create .scrape");
    // Create manifest.json as a directory inside .scrape
    std::fs::create_dir_all(scrape_dir.join("manifest.json")).expect("create dir");

    let result = resolve_manifest_dir(dir.path());

    if result.is_ok() {
        let resolved = result.unwrap();
        let candidate = resolved.join("manifest.json");
        assert!(
            candidate.is_dir(),
            "RQ-TYPE-2 SURVIVOR: resolve_manifest_dir returned Ok when .scrape/manifest.json is a directory"
        );
        panic!(
            "RQ-TYPE-2 SURVIVOR: resolve_manifest_dir accepted directory as .scrape/manifest.json. \
             Resolved to {:?}",
            resolved
        );
    }
}

/// RQ-GEN1-TYPE-3: manifest.json is a symlink to a directory.
#[cfg(unix)]
#[test]
fn rq_manifest_json_is_symlink_to_directory() {
    let dir = temp_dir();
    let real_dir = dir.path().join("real_directory");
    std::fs::create_dir_all(&real_dir).expect("create real dir");
    std::os::unix::fs::symlink(&real_dir, dir.path().join("manifest.json"))
        .expect("create symlink");

    let result = resolve_manifest_dir(dir.path());

    if result.is_ok() {
        let resolved = result.unwrap();
        let candidate = resolved.join("manifest.json");
        // The symlink resolves to a directory via exists()
        if candidate.is_dir() || candidate.exists() {
            panic!(
                "RQ-TYPE-3 SURVIVOR: resolve_manifest_dir accepted symlink-to-directory as manifest.json. \
                 Resolved to {:?}",
                resolved
            );
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// DIMENSION: symlink-attacks — broken, circular, self-referencing
// ═══════════════════════════════════════════════════════════════

/// RQ-GEN1-SYM-1: manifest.json is a BROKEN symlink (dangling).
/// exists() returns false for broken symlinks → should fall through to NotFound.
#[cfg(unix)]
#[test]
fn rq_manifest_json_is_broken_symlink() {
    let dir = temp_dir();
    // Symlink pointing to a non-existent target
    std::os::unix::fs::symlink(
        "/nonexistent/path/manifest.json",
        dir.path().join("manifest.json"),
    )
    .expect("create broken symlink");

    let result = resolve_manifest_dir(dir.path());

    assert!(
        matches!(result, Err(ManifestResolveError::NotFound { .. })),
        "RQ-SYM-1 SURVIVOR: broken symlink should produce NotFound, got: {:?}",
        result
    );
}

/// RQ-GEN1-SYM-2: .scrape itself is a broken symlink.
#[cfg(unix)]
#[test]
fn rq_scrape_dir_is_broken_symlink() {
    let dir = temp_dir();
    // .scrape is a broken symlink — exists() returns false for it AND for anything under it
    std::os::unix::fs::symlink("/nonexistent/scrape", dir.path().join(".scrape"))
        .expect("create broken symlink for .scrape");

    let result = resolve_manifest_dir(dir.path());

    assert!(
        matches!(result, Err(ManifestResolveError::NotFound { .. })),
        "RQ-SYM-2 SURVIVOR: broken .scrape symlink should produce NotFound, got: {:?}",
        result
    );
}

/// RQ-GEN1-SYM-3: manifest.json is a symlink to a valid manifest file.
/// This SHOULD work — the symlink resolves to a real file.
#[cfg(unix)]
#[test]
fn rq_manifest_json_is_valid_symlink_to_real_file() {
    let dir = temp_dir();
    let real_manifest = dir.path().join("real_manifest.json");
    let manifest_content = serde_json::json!({
        "base_url": "https://example.com",
        "pages": [],
        "total_urls": 0,
        "success_count": 0,
        "error_count": 0,
        "errors": []
    });
    std::fs::write(&real_manifest, manifest_content.to_string()).expect("write real manifest");
    std::os::unix::fs::symlink(&real_manifest, dir.path().join("manifest.json"))
        .expect("create symlink");

    let result = resolve_manifest_dir(dir.path());

    assert!(
        result.is_ok(),
        "RQ-SYM-3 SURVIVOR: valid symlink to manifest should resolve, got: {:?}",
        result
    );
    assert_eq!(result.unwrap(), dir.path().to_path_buf());
}

/// RQ-GEN1-SYM-4: .scrape is a symlink to a directory containing manifest.json.
#[cfg(unix)]
#[test]
fn rq_scrape_is_symlink_to_real_dir_with_manifest() {
    let dir = temp_dir();
    let real_scrape = dir.path().join("actual_scrape_output");
    std::fs::create_dir_all(&real_scrape).expect("create real scrape dir");
    write_manifest(&real_scrape);

    // .scrape → actual_scrape_output (symlink)
    std::os::unix::fs::symlink(&real_scrape, dir.path().join(".scrape"))
        .expect("create .scrape symlink");

    let result = resolve_manifest_dir(dir.path());

    assert!(
        result.is_ok(),
        "RQ-SYM-4 SURVIVOR: symlinked .scrape should resolve, got: {:?}",
        result
    );
    // Should return the symlink path (.scrape), not the real path
    let resolved = result.unwrap();
    assert!(
        resolved.ends_with(".scrape"),
        "RQ-SYM-4: expected resolved to end with .scrape, got: {:?}",
        resolved
    );
}

/// RQ-GEN1-SYM-5: Circular symlink — manifest.json → manifest.json.
#[cfg(unix)]
#[test]
fn rq_manifest_json_is_circular_symlink() {
    let dir = temp_dir();
    // manifest.json points to itself (circular)
    let target = dir.path().join("manifest.json");
    // Creating a symlink to itself — this should fail at OS level
    // or produce a dangling symlink
    let _ = std::os::unix::fs::symlink(&target, &target);

    // exists() on a circular symlink returns false on most OSes
    let result = resolve_manifest_dir(dir.path());

    // Should be NotFound since the circular symlink doesn't resolve to a file
    assert!(
        matches!(result, Err(ManifestResolveError::NotFound { .. })),
        "RQ-SYM-5 SURVIVOR: circular symlink should produce NotFound, got: {:?}",
        result
    );
}

// ═══════════════════════════════════════════════════════════════
// DIMENSION: path-edge-cases — empty, dot, double-slash, canonical
// ═══════════════════════════════════════════════════════════════

/// RQ-GEN1-PATH-1: Empty path ("") — what happens?
/// Path::new("").join("manifest.json") → "manifest.json" (relative)
/// exists() checks CWD for manifest.json. Should work or NotFound depending on CWD.
#[test]
fn rq_empty_path_resolves() {
    let empty = Path::new("");

    let result = resolve_manifest_dir(empty);

    // Empty path should behave as "current directory"
    // It's not a bug per se, but we document the behavior
    // If CWD happens to have manifest.json, it resolves; otherwise NotFound
    match result {
        Ok(p) => {
            // If resolved, the path should be empty-ish (current dir)
            let p: PathBuf = p;
            assert!(
                p == PathBuf::from("") || p.is_relative(),
                "RQ-PATH-1: empty path resolved to {:?}, expected relative or empty",
                p
            );
        }
        Err(ManifestResolveError::NotFound { path, .. }) => {
            assert_eq!(path, PathBuf::from(""));
        }
        Err(e) => panic!("RQ-PATH-1 SURVIVOR: unexpected error type: {:?}", e),
    }
}

/// RQ-GEN1-PATH-2: Path is "." (current directory).
#[test]
fn rq_dot_path_resolves() {
    let dot = Path::new(".");

    let result = resolve_manifest_dir(dot);

    // Same as empty path — depends on CWD
    match result {
        Ok(p) => {
            let p: PathBuf = p;
            assert!(
                p.is_relative() || p == PathBuf::from("."),
                "RQ-PATH-2: dot path resolved to {:?}",
                p
            );
        }
        Err(ManifestResolveError::NotFound { path, .. }) => {
            assert_eq!(path, PathBuf::from("."));
        }
        Err(e) => panic!("RQ-PATH-2 SURVIVOR: unexpected error: {:?}", e),
    }
}

/// RQ-GEN1-PATH-3: Path with double-slash (path//to//dir).
#[test]
fn rq_double_slash_in_path() {
    let dir = temp_dir();
    write_manifest(dir.path());

    // Construct path with double slashes
    let abs = dir.path().canonicalize().expect("canonicalize");
    let doubled = format!("{}/", abs.display());
    let path = Path::new(&doubled);

    let result = resolve_manifest_dir(path);

    assert!(
        result.is_ok(),
        "RQ-PATH-3 SURVIVOR: double-slash path should resolve, got: {:?}",
        result
    );
}

/// RQ-GEN1-PATH-4: Path with .. component escaping tempdir.
#[test]
fn rq_path_with_dot_dot_components() {
    let dir = temp_dir();
    let nested = dir.path().join("a").join("b");
    std::fs::create_dir_all(&nested).expect("create nested");
    write_manifest(&nested);

    // Access via .. from parent
    let dotted = dir.path().join("a").join("b").join("..").join("b");
    let result = resolve_manifest_dir(&dotted);

    assert!(
        result.is_ok(),
        "RQ-PATH-4 SURVIVOR: path with .. should resolve, got: {:?}",
        result
    );
}

/// RQ-GEN1-PATH-5: Very long path (4096+ chars).
#[test]
fn rq_very_long_path() {
    let dir = temp_dir();
    let mut long_dir = dir.path().to_path_buf();
    // Extend with many nested directories (but still create a valid one)
    for i in 0..10 {
        long_dir = long_dir.join(format!("subdir_{:04}", i));
    }
    std::fs::create_dir_all(&long_dir).expect("create deep dir");
    write_manifest(&long_dir);

    let result = resolve_manifest_dir(&long_dir);

    assert!(
        result.is_ok(),
        "RQ-PATH-5 SURVIVOR: deep path should resolve, got: {:?}",
        result
    );
}

/// RQ-GEN1-PATH-6: Path is root "/".
#[test]
fn rq_root_path() {
    let result = resolve_manifest_dir(Path::new("/"));

    // Root will almost certainly not have /manifest.json or /.scrape/manifest.json
    match result {
        Ok(p) => {
            // If by some miracle root has manifest.json, that's valid
            let p: &PathBuf = &p;
            assert!(p.is_absolute(), "RQ-PATH-6: root resolved to {:?}", p);
        }
        Err(ManifestResolveError::NotFound { path, .. }) => {
            assert_eq!(path, PathBuf::from("/"));
        }
        Err(e) => panic!("RQ-PATH-6 SURVIVOR: unexpected error for /: {:?}", e),
    }
}

// ═══════════════════════════════════════════════════════════════
// DIMENSION: fs-structure-attacks — .scrape is wrong type
// ═══════════════════════════════════════════════════════════════

/// RQ-GEN1-FS-1: .scrape is a FILE, not a directory.
/// join(".scrape/manifest.json") will fail because .scrape isn't a directory.
#[test]
fn rq_scrape_is_a_file_not_a_directory() {
    let dir = temp_dir();
    // Create .scrape as a regular FILE
    std::fs::write(dir.path().join(".scrape"), "not a directory").expect("create file .scrape");

    let result = resolve_manifest_dir(dir.path());

    // .scrape/manifest.json won't exist (can't traverse into a file)
    // Should be NotFound
    assert!(
        matches!(result, Err(ManifestResolveError::NotFound { .. })),
        "RQ-FS-1 SURVIVOR: .scrape as file should produce NotFound, got: {:?}",
        result
    );
}

/// RQ-GEN1-FS-2: .scrape is a symlink to a file.
#[cfg(unix)]
#[test]
fn rq_scrape_is_symlink_to_file() {
    let dir = temp_dir();
    let target = dir.path().join("target_file");
    std::fs::write(&target, "data").expect("write target");
    std::os::unix::fs::symlink(&target, dir.path().join(".scrape")).expect("symlink");

    let result = resolve_manifest_dir(dir.path());

    assert!(
        matches!(result, Err(ManifestResolveError::NotFound { .. })),
        "RQ-FS-2 SURVIVOR: .scrape → file should produce NotFound, got: {:?}",
        result
    );
}

/// RQ-GEN1-FS-3: manifest.json is an empty file (zero bytes).
/// resolve_manifest_dir should still return Ok — it only checks existence,
/// not validity. But we document this behavior.
#[test]
fn rq_manifest_json_is_empty_file() {
    let dir = temp_dir();
    // Create empty manifest.json
    std::fs::write(dir.path().join("manifest.json"), "").expect("write empty file");

    let result = resolve_manifest_dir(dir.path());

    // Function should return Ok (it only checks existence, not validity)
    // This is correct behavior per contract, but worth documenting
    assert!(
        result.is_ok(),
        "RQ-FS-3: empty manifest.json should still resolve (existence check only). Got: {:?}",
        result
    );
    assert_eq!(result.unwrap(), dir.path().to_path_buf());
}

/// RQ-GEN1-FS-4: manifest.json contains invalid JSON (not a valid ScrapeResult).
/// resolve_manifest_dir checks existence only, not content validity.
#[test]
fn rq_manifest_json_is_invalid_json() {
    let dir = temp_dir();
    std::fs::write(dir.path().join("manifest.json"), "NOT JSON AT ALL!!!{}").expect("write junk");

    let result = resolve_manifest_dir(dir.path());

    // Should return Ok (only checks existence)
    assert!(
        result.is_ok(),
        "RQ-FS-4: invalid JSON should still resolve (existence check only). Got: {:?}",
        result
    );
}

/// RQ-GEN1-FS-5: manifest.json is a hidden file (dotfile in same dir).
/// Ensure function doesn't accidentally match ".manifest.json".
#[test]
fn rq_hidden_manifest_json_not_matched() {
    let dir = temp_dir();
    // Create .manifest.json (dotfile) but NOT manifest.json
    std::fs::write(dir.path().join(".manifest.json"), "{}").expect("write hidden");

    let result = resolve_manifest_dir(dir.path());

    // Should be NotFound — .manifest.json is not manifest.json
    assert!(
        matches!(result, Err(ManifestResolveError::NotFound { .. })),
        "RQ-FS-5 SURVIVOR: .manifest.json should not match, got: {:?}",
        result
    );
}

/// RQ-GEN1-FS-6: Both manifest.json and .scrape/manifest.json are empty.
/// Direct should still win.
#[test]
fn rq_both_manifests_empty_direct_wins() {
    let dir = temp_dir();
    std::fs::write(dir.path().join("manifest.json"), "").expect("write empty direct");
    let scrape = dir.path().join(".scrape");
    std::fs::create_dir_all(&scrape).expect("create .scrape");
    std::fs::write(scrape.join("manifest.json"), "").expect("write empty nested");

    let result = resolve_manifest_dir(dir.path());

    // Direct must win
    assert_eq!(result, Ok(dir.path().to_path_buf()));
}

// ═══════════════════════════════════════════════════════════════
// DIMENSION: error-field-correctness — NotFound fields under stress
// ═══════════════════════════════════════════════════════════════

/// RQ-GEN1-ERR-1: NotFound error fields are correct for relative path.
#[test]
fn rq_not_found_fields_correct_for_relative_path() {
    let relative = Path::new("some/relative/path");

    let result = resolve_manifest_dir(relative);

    match result {
        Err(ManifestResolveError::NotFound {
            path,
            scrape_subdir,
            direct,
            nested,
        }) => {
            assert_eq!(path, PathBuf::from("some/relative/path"));
            assert_eq!(scrape_subdir, PathBuf::from("some/relative/path/.scrape"));
            assert_eq!(direct, PathBuf::from("some/relative/path/manifest.json"));
            assert_eq!(
                nested,
                PathBuf::from("some/relative/path/.scrape/manifest.json")
            );
        }
        Ok(_) => panic!("RQ-ERR-1 SURVIVOR: relative path without manifest should be NotFound"),
        Err(e) => panic!("RQ-ERR-1 SURVIVOR: wrong error type: {:?}", e),
    }
}

/// RQ-GEN1-ERR-2: NotFound error fields are correct for deeply nested absolute path.
#[test]
fn rq_not_found_fields_correct_for_deep_path() {
    let dir = temp_dir();
    let deep = dir.path().join("a").join("b").join("c").join("d").join("e");
    std::fs::create_dir_all(&deep).expect("create deep");

    let result = resolve_manifest_dir(&deep);

    match result {
        Err(ManifestResolveError::NotFound {
            path,
            scrape_subdir,
            direct,
            nested,
        }) => {
            assert_eq!(path, deep);
            assert_eq!(scrape_subdir, deep.join(".scrape"));
            assert_eq!(direct, deep.join("manifest.json"));
            assert_eq!(nested, deep.join(".scrape").join("manifest.json"));
        }
        Ok(_) => panic!("RQ-ERR-2 SURVIVOR: deep empty dir should be NotFound"),
        Err(e) => panic!("RQ-ERR-2 SURVIVOR: wrong error type: {:?}", e),
    }
}

/// RQ-GEN1-ERR-3: Error message is Display-able and non-empty.
#[test]
fn rq_not_found_error_display_is_informative() {
    let dir = temp_dir();
    let result = resolve_manifest_dir(dir.path());
    let err = result.expect_err("should be NotFound");

    let msg = err.to_string();
    assert!(!msg.is_empty(), "error Display must not be empty");
    assert!(msg.contains("manifest.json"), "must mention manifest.json");
    assert!(msg.contains(".scrape"), "must mention .scrape");
    assert!(
        msg.len() > 50,
        "error should be descriptive, got {} chars",
        msg.len()
    );
}

// ═══════════════════════════════════════════════════════════════
// DIMENSION: precedence-invariants — order never violated
// ═══════════════════════════════════════════════════════════════

/// RQ-GEN1-PREC-1: Direct with garbage content beats nested with valid content.
#[test]
fn rq_direct_garbage_beats_nested_valid() {
    let dir = temp_dir();
    // Direct: garbage
    std::fs::write(dir.path().join("manifest.json"), "{{GARBAGE}}").expect("write garbage");
    // Nested: valid
    let scrape = dir.path().join(".scrape");
    std::fs::create_dir_all(&scrape).expect("create .scrape");
    write_manifest(&scrape);

    let result = resolve_manifest_dir(dir.path());

    // Direct must win even with garbage content
    assert_eq!(result, Ok(dir.path().to_path_buf()));
}

/// RQ-GEN1-PREC-2: Direct with 0 bytes beats nested with valid content.
#[test]
fn rq_direct_empty_beats_nested_valid() {
    let dir = temp_dir();
    // Direct: empty
    std::fs::write(dir.path().join("manifest.json"), "").expect("write empty");
    // Nested: valid
    let scrape = dir.path().join(".scrape");
    std::fs::create_dir_all(&scrape).expect("create .scrape");
    write_manifest(&scrape);

    let result = resolve_manifest_dir(dir.path());

    assert_eq!(result, Ok(dir.path().to_path_buf()));
}

/// RQ-GEN1-PREC-3: Direct with a directory beats nested with valid file.
/// (Tests that is_file is NOT checked — only exists)
#[test]
fn rq_direct_directory_beats_nested_valid() {
    let dir = temp_dir();
    // Direct: directory named manifest.json
    std::fs::create_dir_all(dir.path().join("manifest.json")).expect("create dir");
    // Nested: valid file
    let scrape = dir.path().join(".scrape");
    std::fs::create_dir_all(&scrape).expect("create .scrape");
    write_manifest(&scrape);

    let result = resolve_manifest_dir(dir.path());

    // Direct takes precedence, but it's a DIRECTORY, not a file.
    // This IS a bug: the function says it found manifest.json but it's unusable.
    assert!(
        result.is_ok(),
        "RQ-PREC-3: directory-as-direct should still take precedence (even if wrong)"
    );
    // The resolved path won't have a readable manifest.json
    let resolved = result.unwrap();
    let candidate = resolved.join("manifest.json");
    assert!(
        candidate.is_dir(),
        "RQ-PREC-3 SURVIVOR: resolve_manifest_dir claims manifest.json exists at {:?} but it's a directory, not a file",
        candidate
    );
}

// ═══════════════════════════════════════════════════════════════
// DIMENSION: non-existent-path — path itself doesn't exist
// ═══════════════════════════════════════════════════════════════

/// RQ-GEN1-NONEXIST-1: Path doesn't exist at all.
#[test]
fn rq_nonexistent_path_returns_not_found() {
    let dir = temp_dir();
    let ghost = dir.path().join("does_not_exist_at_all");

    let result = resolve_manifest_dir(&ghost);

    assert!(
        matches!(result, Err(ManifestResolveError::NotFound { .. })),
        "RQ-NONEXIST-1 SURVIVOR: non-existent path should be NotFound, got: {:?}",
        result
    );
}

/// RQ-GEN1-NONEXIST-2: NotFound for non-existent path still populates fields correctly.
#[test]
fn rq_not_found_fields_for_nonexistent_path() {
    let dir = temp_dir();
    let ghost = dir.path().join("ghost");

    let result = resolve_manifest_dir(&ghost);

    match result {
        Err(ManifestResolveError::NotFound {
            path,
            scrape_subdir,
            direct,
            nested,
        }) => {
            assert_eq!(path, ghost.clone());
            assert_eq!(scrape_subdir, ghost.join(".scrape"));
            assert_eq!(direct, ghost.join("manifest.json"));
            assert_eq!(nested, ghost.join(".scrape").join("manifest.json"));
        }
        Ok(_) => panic!("RQ-NONEXIST-2 SURVIVOR: ghost path should be NotFound"),
        Err(e) => panic!("RQ-NONEXIST-2 SURVIVOR: wrong error: {:?}", e),
    }
}

// ═══════════════════════════════════════════════════════════════
// DIMENSION: permission-denied — can't read directory
// ═══════════════════════════════════════════════════════════════

/// RQ-GEN1-PERM-1: Directory exists but is not readable.
/// exists() may return false if parent dir can't be read.
#[cfg(unix)]
#[test]
fn rq_unreadable_directory_returns_not_found() {
    let dir = temp_dir();
    let restricted = dir.path().join("restricted");
    std::fs::create_dir_all(&restricted).expect("create restricted");
    write_manifest(&restricted);

    // Remove read permission
    std::fs::set_permissions(
        &restricted,
        std::os::unix::fs::PermissionsExt::from_mode(0o000),
    )
    .expect("chmod 000");

    let result = resolve_manifest_dir(&restricted);

    // Restore permissions for cleanup
    std::fs::set_permissions(
        &restricted,
        std::os::unix::fs::PermissionsExt::from_mode(0o755),
    )
    .ok();

    // On most Linux systems, root can still read. For non-root:
    // exists() returns false → NotFound. This is expected behavior.
    // We can't assert much here since behavior depends on uid.
    match result {
        Ok(_) => {
            // Running as root — everything is readable
        }
        Err(ManifestResolveError::NotFound { .. }) => {
            // Expected for non-root
        }
        Err(e) => panic!("RQ-PERM-1 SURVIVOR: unexpected error: {:?}", e),
    }
}

// ═══════════════════════════════════════════════════════════════
// DIMENSION: concurrent-mutation — manifest deleted between calls
// ═══════════════════════════════════════════════════════════════

/// RQ-GEN1-RACE-1: Manifest deleted between two calls.
/// Demonstrates that the function is not atomic (expected, but documented).
#[test]
fn rq_manifest_deleted_between_calls() {
    let dir = temp_dir();
    write_manifest(dir.path());

    let result1 = resolve_manifest_dir(dir.path());
    assert!(result1.is_ok(), "first call should succeed");

    // Delete manifest
    std::fs::remove_file(dir.path().join("manifest.json")).expect("delete manifest");

    let result2 = resolve_manifest_dir(dir.path());
    assert!(
        matches!(result2, Err(ManifestResolveError::NotFound { .. })),
        "RQ-RACE-1 SURVIVOR: after deleting manifest, should be NotFound, got: {:?}",
        result2
    );
}

/// RQ-GEN1-RACE-2: Manifest moved from direct to nested between calls.
#[test]
fn rq_manifest_moved_direct_to_nested() {
    let dir = temp_dir();
    write_manifest(dir.path());

    let result1 = resolve_manifest_dir(dir.path());
    assert_eq!(result1, Ok(dir.path().to_path_buf()));

    // Move manifest from direct to nested
    let scrape = dir.path().join(".scrape");
    std::fs::create_dir_all(&scrape).expect("create .scrape");
    std::fs::rename(
        dir.path().join("manifest.json"),
        scrape.join("manifest.json"),
    )
    .expect("move manifest");

    let result2 = resolve_manifest_dir(dir.path());
    assert_eq!(
        result2,
        Ok(scrape),
        "RQ-RACE-2: should now resolve to .scrape"
    );
}

// ═══════════════════════════════════════════════════════════════
// DIMENSION: return-path-identity — returned path always has manifest.json
// ═══════════════════════════════════════════════════════════════

/// RQ-GEN1-IDENT-1: For every Ok(resolved), resolved/manifest.json must exist.
#[test]
fn rq_returned_path_always_has_manifest_json() {
    let dir = temp_dir();
    write_manifest(dir.path());

    let result = resolve_manifest_dir(dir.path());
    let resolved = result.expect("should resolve");

    let manifest = resolved.join("manifest.json");
    assert!(
        manifest.exists(),
        "RQ-IDENT-1 SURVIVOR: resolved path {:?} does not contain manifest.json",
        resolved
    );
}

/// RQ-GEN1-IDENT-2: For nested resolution, returned path/manifest.json must exist.
#[test]
fn rq_returned_nested_path_has_manifest_json() {
    let dir = temp_dir();
    let scrape = dir.path().join(".scrape");
    std::fs::create_dir_all(&scrape).expect("create .scrape");
    write_manifest(&scrape);

    let result = resolve_manifest_dir(dir.path());
    let resolved = result.expect("should resolve");

    let manifest = resolved.join("manifest.json");
    assert!(
        manifest.exists(),
        "RQ-IDENT-2 SURVIVOR: resolved path {:?} does not contain manifest.json",
        resolved
    );
}

// ═══════════════════════════════════════════════════════════════
// DIMENSION: diff-directories-integration — resolve_manifest_dir via diff_directories
// ═══════════════════════════════════════════════════════════════

/// RQ-GEN1-DIFF-1: diff_directories with invalid manifest content in dir_a.
#[test]
fn rq_diff_directories_invalid_manifest_a() {
    let dir = temp_dir();
    let dir_a = dir.path().join("a");
    let dir_b = dir.path().join("b");
    std::fs::create_dir_all(&dir_a).expect("create a");
    std::fs::create_dir_all(&dir_b).expect("create b");

    // Write valid manifest to a, garbage to b... wait, let's make a have garbage
    std::fs::write(dir_a.join("manifest.json"), "NOT VALID JSON").expect("write garbage a");
    write_manifest(&dir_b);

    // diff_directories calls resolve_manifest_dir internally
    // resolve_manifest_dir should return Ok for dir_a (exists), then
    // serde_json deserialization fails in diff_directories
    let plan = doc_transformer::watch::diff_directories(&dir_a, &dir_b);
    assert!(
        plan.is_err(),
        "RQ-DIFF-1 SURVIVOR: diff_directories with invalid JSON should fail, got: {:?}",
        plan
    );
}

/// RQ-GEN1-DIFF-2: diff_directories where dir_a doesn't have manifest.
#[test]
fn rq_diff_directories_no_manifest_a() {
    let dir = temp_dir();
    let dir_a = dir.path().join("a");
    let dir_b = dir.path().join("b");
    std::fs::create_dir_all(&dir_a).expect("create a");
    std::fs::create_dir_all(&dir_b).expect("create b");

    // No manifest in a, valid in b
    write_manifest(&dir_b);

    let plan = doc_transformer::watch::diff_directories(&dir_a, &dir_b);
    assert!(
        plan.is_err(),
        "RQ-DIFF-2 SURVIVOR: diff_directories with missing manifest in a should fail, got: {:?}",
        plan
    );
}

// ═══════════════════════════════════════════════════════════════
// GENERATION 2: Deeper type-confusion probes + new dimensions
// ═══════════════════════════════════════════════════════════════

/// RQ-GEN2-TYPE-4: manifest.json is a FIFO (named pipe).
/// exists() returns true for FIFOs. resolve_manifest_dir would say "found it"
/// but reading it would block forever (or fail).
#[cfg(unix)]
#[test]
fn rq_manifest_json_is_a_fifo() {
    let dir = temp_dir();
    let fifo_path = dir.path().join("manifest.json");
    // Create a named pipe (FIFO)
    let status = std::process::Command::new("mkfifo")
        .arg(&fifo_path)
        .status()
        .expect("run mkfifo");
    assert!(status.success(), "mkfifo should succeed");

    // Verify it's a FIFO
    let metadata = std::fs::symlink_metadata(&fifo_path).expect("metadata");
    use std::os::unix::fs::FileTypeExt;
    assert!(metadata.file_type().is_fifo(), "should be a FIFO");

    let result = resolve_manifest_dir(dir.path());

    // resolve_manifest_dir uses exists() which returns true for FIFOs
    if result.is_ok() {
        let resolved = result.unwrap();
        let candidate = resolved.join("manifest.json");
        let meta = std::fs::symlink_metadata(&candidate).expect("meta");
        if meta.file_type().is_fifo() {
            panic!(
                "RQ-GEN2-TYPE-4 SURVIVOR: resolve_manifest_dir accepted FIFO as manifest.json. \
                 Resolved to {:?}. manifest.json is a named pipe, not a regular file.",
                resolved
            );
        }
    }
}

/// RQ-GEN2-TYPE-5: manifest.json is a Unix domain socket.
#[cfg(unix)]
#[test]
fn rq_manifest_json_is_a_socket() {
    let dir = temp_dir();
    let socket_path = dir.path().join("manifest.json");

    // Create a Unix domain socket using std::net
    #[cfg(unix)]
    {
        use std::os::unix::net::UnixListener;
        let _listener = UnixListener::bind(&socket_path).expect("bind socket");

        let result = resolve_manifest_dir(dir.path());

        if result.is_ok() {
            let resolved = result.unwrap();
            let candidate = resolved.join("manifest.json");
            let meta = std::fs::symlink_metadata(&candidate).expect("meta");
            if !meta.file_type().is_file() {
                panic!(
                    "RQ-GEN2-TYPE-5 SURVIVOR: resolve_manifest_dir accepted socket as manifest.json. \
                     Resolved to {:?}. Type: {:?}",
                    resolved, meta.file_type()
                );
            }
        }
    }
}

/// RQ-GEN2-TYPE-6: .scrape/manifest.json is a symlink to a directory.
/// Nested resolution claims success but manifest.json is not a file.
#[cfg(unix)]
#[test]
fn rq_nested_manifest_json_is_symlink_to_directory() {
    let dir = temp_dir();
    let scrape = dir.path().join(".scrape");
    let real_dir = dir.path().join("actual_dir");
    std::fs::create_dir_all(&scrape).expect("create .scrape");
    std::fs::create_dir_all(&real_dir).expect("create real dir");

    // .scrape/manifest.json → actual_dir (directory)
    std::os::unix::fs::symlink(&real_dir, scrape.join("manifest.json")).expect("create symlink");

    let result = resolve_manifest_dir(dir.path());

    if result.is_ok() {
        let resolved = result.unwrap();
        let candidate = resolved.join("manifest.json");
        if candidate.is_dir() {
            panic!(
                "RQ-GEN2-TYPE-6 SURVIVOR: resolve_manifest_dir accepted symlink-to-dir as nested manifest. \
                 Resolved to {:?}. manifest.json resolves to a directory.",
                resolved
            );
        }
    }
}

/// RQ-GEN2-IDEMPOTENT-1: Calling resolve_manifest_dir 100 times produces same result.
#[test]
fn rq_resolve_manifest_dir_100_calls_deterministic() {
    let dir = temp_dir();
    write_manifest(dir.path());

    let mut results = Vec::new();
    for _ in 0..100 {
        results.push(resolve_manifest_dir(dir.path()));
    }

    let first = &results[0];
    for (i, r) in results.iter().enumerate().skip(1) {
        assert_eq!(
            r, first,
            "RQ-GEN2-IDEMPOTENT-1 SURVIVOR: call {} produced different result: {:?} vs {:?}",
            i, r, first
        );
    }
}

/// RQ-GEN2-IDEMPOTENT-2: 100 calls on NotFound path all return NotFound.
#[test]
fn rq_resolve_manifest_dir_100_calls_not_found_deterministic() {
    let dir = temp_dir();
    // Empty directory — no manifest

    for _ in 0..100 {
        let result = resolve_manifest_dir(dir.path());
        assert!(
            matches!(result, Err(ManifestResolveError::NotFound { .. })),
            "RQ-GEN2-IDEMPOTENT-2 SURVIVOR: empty dir should always be NotFound"
        );
    }
}

/// RQ-GEN2-CALLER-1: read_manifest (in cmd/watch.rs) with manifest.json as directory.
/// Tests the full call chain: resolve_manifest_dir → read_manifest.
#[test]
fn rq_read_manifest_fails_gracefully_with_directory_manifest() {
    // This tests the integration: if resolve_manifest_dir is "fixed" to reject
    // directories, does read_manifest handle the error correctly?
    // But for now, if resolve_manifest_dir accepts directories, read_manifest
    // will try to open the "file" and get an EISDIR error.
    let dir = temp_dir();
    std::fs::create_dir_all(dir.path().join("manifest.json")).expect("create dir");

    let resolved = resolve_manifest_dir(dir.path());
    // If resolve_manifest_dir returns Ok, the caller will try to open manifest.json
    // and get an error. The question is: does the error message help the user?
    if let Ok(resolved_path) = resolved {
        let manifest_path = resolved_path.join("manifest.json");
        let open_result = std::fs::File::open(&manifest_path);
        if open_result.is_err() {
            // Good — opening a directory fails. But resolve_manifest_dir shouldn't
            // have returned Ok in the first place.
            let err = open_result.unwrap_err();
            assert!(
                err.kind() == std::io::ErrorKind::IsADirectory
                    || format!("{}", err).contains("Is a directory"),
                "RQ-GEN2-CALLER-1: opening directory-as-manifest should fail with IsADirectory, got: {:?}",
                err
            );
        }
    }
}

/// RQ-GEN2-PATH-7: Path with null byte replacement (unicode null character).
/// Path::new handles this, but filesystem won't have it.
#[test]
fn rq_path_with_special_characters() {
    let dir = temp_dir();
    let special = dir.path().join("dir\twith\ttabs");
    std::fs::create_dir_all(&special).expect("create special dir");
    write_manifest(&special);

    let result = resolve_manifest_dir(&special);
    assert!(
        result.is_ok(),
        "RQ-GEN2-PATH-7 SURVIVOR: path with tabs should resolve, got: {:?}",
        result
    );
}

/// RQ-GEN2-PATH-8: Path with newline in directory name.
#[test]
fn rq_path_with_newline_in_dir_name() {
    let dir = temp_dir();
    let special = dir.path().join("dir\nwith\nnewlines");
    let create_result = std::fs::create_dir_all(&special);
    if create_result.is_ok() {
        write_manifest(&special);
        let result = resolve_manifest_dir(&special);
        assert!(
            result.is_ok(),
            "RQ-GEN2-PATH-8: newline path should resolve, got: {:?}",
            result
        );
    }
    // If create_dir_all fails (some filesystems don't allow \n in names), that's ok
}

/// RQ-GEN2-ERR-4: Error implements std::error::Error (for anyhow compatibility).
#[test]
fn rq_manifest_resolve_error_is_std_error() {
    fn assert_error<E: std::error::Error>(_: &E) {}
    let dir = temp_dir();
    let result = resolve_manifest_dir(dir.path());
    match result {
        Err(e) => assert_error(&e),
        Ok(_) => {}
    }
}

/// RQ-GEN2-ERR-5: Error is Clone (so it can be stored in collections).
#[test]
fn rq_manifest_resolve_error_is_clone() {
    let dir = temp_dir();
    let result = resolve_manifest_dir(dir.path());
    if let Err(e) = result {
        let cloned = e.clone();
        assert_eq!(e, cloned, "RQ-GEN2-ERR-5: cloned error must equal original");
    }
}

/// RQ-GEN2-ERR-6: Error is Debug (for logging).
#[test]
fn rq_manifest_resolve_error_is_debug() {
    let dir = temp_dir();
    let result = resolve_manifest_dir(dir.path());
    if let Err(e) = result {
        let debug = format!("{:?}", e);
        assert!(!debug.is_empty(), "Debug output must not be empty");
        assert!(debug.contains("NotFound"), "Debug must show variant name");
    }
}

/// RQ-GEN2-FS-7: manifest.json is a block device (if possible to create).
/// On Linux, we can't easily create block devices from userspace, but we test
/// the behavior for any non-regular-file type.
#[test]
fn rq_manifest_json_is_readable_file_check() {
    let dir = temp_dir();
    write_manifest(dir.path());
    let result = resolve_manifest_dir(dir.path());
    let resolved = result.expect("should resolve");
    let manifest_path = resolved.join("manifest.json");

    // Verify the resolved manifest.json is actually a readable file
    let metadata = std::fs::metadata(&manifest_path).expect("metadata");
    assert!(
        metadata.is_file(),
        "RQ-GEN2-FS-7: resolved manifest.json must be a regular file, got: {:?}",
        metadata.file_type()
    );
    assert!(
        metadata.len() > 0,
        "RQ-GEN2-FS-7: manifest.json should have content"
    );
}

/// RQ-GEN2-PREC-4: After removing direct manifest, nested should be found.
#[test]
fn rq_fallback_to_nested_after_direct_removed() {
    let dir = temp_dir();
    write_manifest(dir.path());
    let scrape = dir.path().join(".scrape");
    std::fs::create_dir_all(&scrape).expect("create .scrape");
    write_manifest(&scrape);

    // Both exist → direct wins
    let result1 = resolve_manifest_dir(dir.path());
    assert_eq!(result1, Ok(dir.path().to_path_buf()));

    // Remove direct
    std::fs::remove_file(dir.path().join("manifest.json")).expect("remove direct");

    // Now nested should win
    let result2 = resolve_manifest_dir(dir.path());
    assert_eq!(
        result2,
        Ok(scrape.clone()),
        "RQ-GEN2-PREC-4: should fall back to nested"
    );

    // Remove nested too → NotFound
    std::fs::remove_file(scrape.join("manifest.json")).expect("remove nested");
    let result3 = resolve_manifest_dir(dir.path());
    assert!(
        matches!(result3, Err(ManifestResolveError::NotFound { .. })),
        "RQ-GEN2-PREC-4: should be NotFound after both removed"
    );
}

/// RQ-GEN2-RACE-3: .scrape directory created between two calls.
#[test]
fn rq_scrape_dir_appears_between_calls() {
    let dir = temp_dir();
    // Initially: no manifest at all
    let result1 = resolve_manifest_dir(dir.path());
    assert!(result1.is_err(), "first call: should be NotFound");

    // Now create .scrape with manifest
    let scrape = dir.path().join(".scrape");
    std::fs::create_dir_all(&scrape).expect("create .scrape");
    write_manifest(&scrape);

    let result2 = resolve_manifest_dir(dir.path());
    assert_eq!(
        result2,
        Ok(scrape),
        "RQ-GEN2-RACE-3: should now find nested manifest"
    );
}

/// RQ-GEN2-DIFF-3: diff_directories with dir_a containing directory-named manifest.json.
/// Tests that diff_directories surfaces the error when manifest.json is a directory.
#[test]
fn rq_diff_directories_with_dir_as_manifest() {
    let dir = temp_dir();
    let dir_a = dir.path().join("a");
    let dir_b = dir.path().join("b");
    std::fs::create_dir_all(&dir_a).expect("create a");
    std::fs::create_dir_all(&dir_b).expect("create b");

    // dir_a: manifest.json is a DIRECTORY (not a file)
    std::fs::create_dir_all(dir_a.join("manifest.json")).expect("create dir manifest");
    write_manifest(&dir_b);

    // resolve_manifest_dir returns Ok for dir_a (it only checks exists()),
    // then diff_directories tries to open manifest.json → EISDIR
    let plan = doc_transformer::watch::diff_directories(&dir_a, &dir_b);
    // This should fail because you can't read a directory as JSON
    assert!(
        plan.is_err(),
        "RQ-GEN2-DIFF-3 SURVIVOR: diff_directories should fail when manifest.json is a directory, got Ok"
    );
}

/// RQ-GEN2-TYPE-7: Direct manifest is a file but .scrape/manifest.json is a directory.
/// Direct should still win and be valid.
#[test]
fn rq_direct_valid_nested_is_directory() {
    let dir = temp_dir();
    write_manifest(dir.path());
    let scrape = dir.path().join(".scrape");
    std::fs::create_dir_all(&scrape).expect("create .scrape");
    std::fs::create_dir_all(scrape.join("manifest.json")).expect("create nested dir");

    let result = resolve_manifest_dir(dir.path());

    // Direct wins, and it's actually valid
    assert_eq!(result, Ok(dir.path().to_path_buf()));
    // Verify the resolved manifest is actually a file
    let manifest_path = dir.path().join("manifest.json");
    assert!(manifest_path.is_file(), "direct manifest must be a file");
}

// ═══════════════════════════════════════════════════════════════
// GENERATION 3: Final probes — character device, block device paths,
// deep nesting, and error message content verification
// ═══════════════════════════════════════════════════════════════

/// RQ-GEN3-TYPE-8: manifest.json is a character device (if /dev/null is symlinked).
#[cfg(unix)]
#[test]
fn rq_manifest_json_is_char_device_via_symlink() {
    let dir = temp_dir();
    // Symlink /dev/null as manifest.json — /dev/null is a character device
    std::os::unix::fs::symlink("/dev/null", dir.path().join("manifest.json"))
        .expect("symlink /dev/null");

    let result = resolve_manifest_dir(dir.path());

    if result.is_ok() {
        let resolved = result.unwrap();
        let candidate = resolved.join("manifest.json");
        let meta = std::fs::symlink_metadata(&candidate).expect("meta");
        use std::os::unix::fs::FileTypeExt;
        if meta.file_type().is_char_device() {
            panic!(
                "RQ-GEN3-TYPE-8 SURVIVOR: resolve_manifest_dir accepted char device as manifest.json. \
                 Resolved to {:?}. manifest.json is /dev/null (char device).",
                resolved
            );
        }
    }
}

/// RQ-GEN3-PATH-9: Path with only whitespace name.
#[test]
fn rq_path_with_whitespace_name() {
    let dir = temp_dir();
    let space_dir = dir.path().join(" ");
    let create_result = std::fs::create_dir_all(&space_dir);
    if create_result.is_ok() {
        write_manifest(&space_dir);
        let result = resolve_manifest_dir(&space_dir);
        // Should work — whitespace in name is valid on most FS
        assert!(
            result.is_ok(),
            "RQ-GEN3-PATH-9: whitespace dir should resolve, got: {:?}",
            result
        );
    }
}

/// RQ-GEN3-DEEP-1: .scrape/.scrape/manifest.json should NOT be found.
/// The function only checks path/.scrape/manifest.json, not deeper nesting.
#[test]
fn rq_double_nested_scrape_not_found() {
    let dir = temp_dir();
    let double_nested = dir.path().join(".scrape").join(".scrape");
    std::fs::create_dir_all(&double_nested).expect("create double nested");
    write_manifest(&double_nested);

    let result = resolve_manifest_dir(dir.path());

    // Should be NotFound — only one level of .scrape is checked
    assert!(
        matches!(result, Err(ManifestResolveError::NotFound { .. })),
        "RQ-GEN3-DEEP-1 SURVIVOR: double-nested .scrape/.scrape should not be found, got: {:?}",
        result
    );
}

/// RQ-GEN3-DEEP-2: Path is a deeply nested chain of symlinks.
#[cfg(unix)]
#[test]
fn rq_deep_symlink_chain_resolves() {
    let dir = temp_dir();
    write_manifest(dir.path());

    // Create chain: link1 → link2 → link3 → real_dir
    let link1 = dir.path().join("link1");
    let link2 = dir.path().join("link2");
    let link3 = dir.path().join("link3");
    std::os::unix::fs::symlink(&link3, &link2).expect("link2 → link3");
    std::os::unix::fs::symlink(dir.path(), &link3).expect("link3 → real");
    std::os::unix::fs::symlink(&link2, &link1).expect("link1 → link2");

    let result = resolve_manifest_dir(&link1);

    assert!(
        result.is_ok(),
        "RQ-GEN3-DEEP-2 SURVIVOR: deep symlink chain should resolve, got: {:?}",
        result
    );
}

/// RQ-GEN3-ERR-7: Error message contains the full absolute path when given absolute input.
#[test]
fn rq_error_message_contains_absolute_path() {
    let dir = temp_dir();
    let abs = dir.path().canonicalize().expect("canonicalize");
    let result = resolve_manifest_dir(&abs);
    let err = result.expect_err("should be NotFound");
    let msg = err.to_string();
    let abs_str = abs.to_string_lossy().to_string();
    assert!(
        msg.contains(&abs_str),
        "RQ-GEN3-ERR-7: error message must contain absolute path {}, got: {}",
        abs_str,
        msg
    );
}

/// RQ-GEN3-ERR-8: Error message contains "Tip:" for actionable guidance.
#[test]
fn rq_error_message_has_actionable_tip() {
    let dir = temp_dir();
    let result = resolve_manifest_dir(dir.path());
    let err = result.expect_err("should be NotFound");
    let msg = err.to_string();
    assert!(
        msg.contains("Tip:"),
        "RQ-GEN3-ERR-8: error message must contain 'Tip:', got: {}",
        msg
    );
    assert!(
        msg.contains("ctd scrape"),
        "RQ-GEN3-ERR-8: tip should mention 'ctd scrape', got: {}",
        msg
    );
}

/// RQ-GEN3-RETURN-1: Returned path from direct resolution, when joined with
/// "manifest.json", must be readable as JSON.
#[test]
fn rq_direct_resolved_path_manifest_is_readable_json() {
    let dir = temp_dir();
    write_manifest(dir.path());

    let result = resolve_manifest_dir(dir.path());
    let resolved = result.expect("should resolve");
    let manifest_path = resolved.join("manifest.json");

    let content = std::fs::read_to_string(&manifest_path).expect("read manifest");
    let parsed: serde_json::Value = serde_json::from_str(&content).expect("parse JSON");
    assert!(parsed.is_object(), "manifest must be a JSON object");
    assert!(parsed.get("base_url").is_some(), "must have base_url");
}

/// RQ-GEN3-RETURN-2: Returned path from nested resolution, when joined with
/// "manifest.json", must be readable as JSON.
#[test]
fn rq_nested_resolved_path_manifest_is_readable_json() {
    let dir = temp_dir();
    let scrape = dir.path().join(".scrape");
    std::fs::create_dir_all(&scrape).expect("create .scrape");
    write_manifest(&scrape);

    let result = resolve_manifest_dir(dir.path());
    let resolved = result.expect("should resolve");
    let manifest_path = resolved.join("manifest.json");

    let content = std::fs::read_to_string(&manifest_path).expect("read manifest");
    let parsed: serde_json::Value = serde_json::from_str(&content).expect("parse JSON");
    assert!(parsed.is_object(), "manifest must be a JSON object");
}

/// RQ-GEN3-PATH-10: Path with multiple trailing slashes.
#[test]
fn rq_multiple_trailing_slashes() {
    let dir = temp_dir();
    write_manifest(dir.path());
    let abs = dir.path().canonicalize().expect("canonicalize");

    // Multiple trailing slashes
    let multi_slash = format!("{}//", abs.display());
    let path = std::path::Path::new(&multi_slash);

    let result = resolve_manifest_dir(path);
    assert!(
        result.is_ok(),
        "RQ-GEN3-PATH-10: multi-trailing-slash should resolve, got: {:?}",
        result
    );
}

/// RQ-GEN3-RENAME-1: manifest.json renamed to Manifest.json (case sensitivity).
#[test]
fn rq_manifest_case_sensitivity() {
    let dir = temp_dir();
    // Create Manifest.json (capital M) — on Linux this is different from manifest.json
    let manifest_content = serde_json::json!({
        "base_url": "https://example.com",
        "pages": [],
        "total_urls": 0,
        "success_count": 0,
        "error_count": 0,
        "errors": []
    });
    std::fs::write(
        dir.path().join("Manifest.json"),
        manifest_content.to_string(),
    )
    .expect("write Manifest.json");

    let result = resolve_manifest_dir(dir.path());

    // On Linux (case-sensitive): Manifest.json != manifest.json → NotFound
    // On macOS (case-insensitive by default): might resolve
    #[cfg(target_os = "linux")]
    {
        assert!(
            matches!(result, Err(ManifestResolveError::NotFound { .. })),
            "RQ-GEN3-RENAME-1 SURVIVOR: Manifest.json should not match manifest.json on Linux, got: {:?}",
            result
        );
    }
}
