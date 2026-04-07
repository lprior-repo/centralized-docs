#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::field_reassign_with_default
)]
use super::*;
use std::fs;

// === Scenario 3.1: compute_config_hash returns empty hash for None ===
#[test]
fn compute_config_hash_returns_empty_hash_when_none() {
    let result = compute_config_hash(None);
    assert_eq!(
        result,
        content_hash(b""),
        "None input must yield empty hash"
    );
}

// === Scenario 3.2: compute_config_hash returns SHA-256 of file bytes ===
#[test]
fn compute_config_hash_returns_sha256_when_file_readable() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let file_path = dir.path().join("config.yaml");
    fs::write(&file_path, b"hello world").expect("write");

    let result = compute_config_hash(Some(&file_path));
    assert_eq!(
        result,
        content_hash(b"hello world"),
        "must match SHA-256 of file bytes"
    );
}

// === Scenario 3.3: compute_config_hash returns empty hash for missing file ===
#[test]
fn compute_config_hash_returns_empty_hash_when_file_missing() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let missing = dir.path().join("nonexistent.yaml");

    let result = compute_config_hash(Some(&missing));
    assert_eq!(
        result,
        content_hash(b""),
        "missing file must yield empty hash"
    );
}

// === Scenario 3.4: compute_config_hash returns empty hash for unreadable file ===
#[test]
fn compute_config_hash_returns_empty_hash_when_file_unreadable() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let file_path = dir.path().join("secret.yaml");
    fs::write(&file_path, b"secret").expect("write");

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&file_path, PermissionsExt::from_mode(0o000)).expect("chmod");
    }

    let result = compute_config_hash(Some(&file_path));
    assert_eq!(
        result,
        content_hash(b""),
        "unreadable file must yield empty hash"
    );

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&file_path, PermissionsExt::from_mode(0o644));
    }
}

// === Scenario 3.5: compute_config_hash is deterministic ===
#[test]
fn compute_config_hash_returns_identical_hash_across_calls() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let file_path = dir.path().join("deterministic.yaml");
    fs::write(&file_path, b"deterministic test content").expect("write");

    let hash1 = compute_config_hash(Some(&file_path));
    let hash2 = compute_config_hash(Some(&file_path));

    assert_eq!(hash1, hash2, "same input must yield identical hash");
    assert_eq!(hash1, content_hash(b"deterministic test content"));
}

// === Scenario 3.6: compute_config_hash returns distinct hashes ===
#[test]
fn compute_config_hash_returns_distinct_concrete_hashes_for_different_contents() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let file_a = dir.path().join("a.yaml");
    let file_b = dir.path().join("b.yaml");
    fs::write(&file_a, b"aaa").expect("write a");
    fs::write(&file_b, b"bbb").expect("write b");

    let hash_a = compute_config_hash(Some(&file_a));
    let hash_b = compute_config_hash(Some(&file_b));

    assert_eq!(hash_a, content_hash(b"aaa"));
    assert_eq!(hash_b, content_hash(b"bbb"));
    assert_ne!(
        hash_a, hash_b,
        "different content must yield different hashes"
    );
}

// === Scenario 3.7: compute_config_hash returns empty hash for 0-byte file ===
#[test]
fn compute_config_hash_returns_empty_hash_when_file_is_zero_bytes() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let file_path = dir.path().join("empty.yaml");
    fs::write(&file_path, b"").expect("write empty");

    let result = compute_config_hash(Some(&file_path));
    assert_eq!(
        result,
        content_hash(b""),
        "0-byte file must yield empty hash"
    );
}

// === Scenario 3.8: compute_config_hash handles large file ===
#[test]
fn compute_config_hash_returns_exact_sha256_when_file_is_large() {
    let dir = tempfile::TempDir::new().expect("tempdir");
    let file_path = dir.path().join("large.yaml");
    let large_content = vec![b'X'; 1_048_576]; // 1MB
    fs::write(&file_path, &large_content).expect("write large");

    let result = compute_config_hash(Some(&file_path));
    assert_eq!(
        result,
        content_hash(&large_content),
        "large file hash must match"
    );
}
