//! Property-based tests for diff computation (cdocs-2rt).
//!
//! Covers proptest invariants 4.1–4.6 from the cdocs-2rt test plan.

#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use std::collections::{HashMap, HashSet};
use std::fs;

use proptest::prelude::*;

use doc_transformer::cache::content_hash;
use doc_transformer::diff::{compute_config_hash, compute_file_diff, FileDiff, StoredHashes};
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

fn write_file(dir: &std::path::Path, relative: &str, content: &[u8]) {
    let path = dir.join(relative);
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(path, content).unwrap();
}

/// Pairwise disjoint assertion.
fn assert_pairwise_disjoint(diff: &FileDiff) {
    let buckets: [&HashSet<String>; 4] = [&diff.unchanged, &diff.changed, &diff.new, &diff.deleted];
    for i in 0..4 {
        for j in (i + 1)..4 {
            let intersection: HashSet<&String> = buckets[i].intersection(buckets[j]).collect();
            assert!(
                intersection.is_empty(),
                "buckets[{i}] ∩ buckets[{j}] must be empty"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Proptest 1: compute_config_hash determinism
// ---------------------------------------------------------------------------

proptest! {
    #[test]
    fn proptest_config_hash_determinism(bytes in any::<Vec<u8>>().no_shrink().prop_filter("bounded", |b| b.len() <= 1024)) {
        let dir = tempfile::TempDir::new().unwrap();
        let file_path = dir.path().join("config.yaml");
        fs::write(&file_path, &bytes).unwrap();

        let hash1 = compute_config_hash(Some(&file_path));
        let hash2 = compute_config_hash(Some(&file_path));

        prop_assert_eq!(hash1, hash2);
        prop_assert_eq!(hash1, content_hash(&bytes));
    }
}

// ---------------------------------------------------------------------------
// Proptest 2: compute_config_hash(None) is constant
// ---------------------------------------------------------------------------

proptest! {
    #[test]
    fn proptest_config_hash_none_is_constant(_ in 0..1000i32) {
        let hash = compute_config_hash(None);
        prop_assert_eq!(hash, content_hash(b""));
    }
}

// ---------------------------------------------------------------------------
// Proptest 3: Partition invariant (single-bucket membership)
// ---------------------------------------------------------------------------

proptest! {
    #[test]
    fn proptest_partition_invariant(
        unique_paths in prop::collection::vec("[a-z]{1,5}\\.md", 1..=10usize),
        content_seed in any::<u64>(),
    ) {
        let dir = tempfile::TempDir::new().unwrap();

        let mut files = Vec::new();
        let mut stored = HashMap::new();

        for (idx, path) in unique_paths.iter().enumerate() {
            let content = format!("content_{content_seed}_{idx}");
            write_file(dir.path(), path, content.as_bytes());
            files.push(make_discovery(path, content.len() as u64));

            // Randomly decide stored state
            let decision = (content_seed.wrapping_add(idx as u64)) % 4;
            match decision {
                0 => {
                    // Matching hashes -> should be Unchanged
                    stored.insert(path.clone(), make_stored(content.as_bytes(), b""));
                }
                1 => {
                    // Mismatched content -> should be Changed
                    let old = format!("old_{content_seed}_{idx}");
                    stored.insert(path.clone(), make_stored(old.as_bytes(), b""));
                }
                2 => {
                    // Mismatched config -> should be Changed
                    stored.insert(path.clone(), make_stored(content.as_bytes(), b"old_config"));
                }
                3 => {
                    // Not in stored -> should be New
                }
                _ => {}
            }
        }

        let result = compute_file_diff(&files, dir.path(), None, &stored);
        prop_assert!(result.is_ok(), "diff should succeed");

        let diff = result.unwrap();
        assert_pairwise_disjoint(&diff);
    }
}

// ---------------------------------------------------------------------------
// Proptest 4: Collective exhaustive invariant
// ---------------------------------------------------------------------------

proptest! {
    #[test]
    fn proptest_collective_exhaustive_invariant(
        unique_paths in prop::collection::vec("[a-z]{1,5}\\.md", 1..=10usize),
        content_seed in any::<u64>(),
    ) {
        let dir = tempfile::TempDir::new().unwrap();

        let mut files = Vec::new();
        let mut stored = HashMap::new();

        // Some paths are on-disk
        let on_disk_count = unique_paths.len() / 2 + 1;
        for (idx, path) in unique_paths.iter().enumerate().take(on_disk_count) {
            let content = format!("content_{content_seed}_{idx}");
            write_file(dir.path(), path, content.as_bytes());
            files.push(make_discovery(path, content.len() as u64));

            // Randomly add to stored
            if idx % 2 == 0 {
                stored.insert(path.clone(), make_stored(content.as_bytes(), b""));
            }
        }

        // Remaining paths are stored-only (deleted)
        for (idx, path) in unique_paths.iter().enumerate().skip(on_disk_count) {
            stored.insert(path.clone(), make_stored(format!("stored_{idx}").as_bytes(), b""));
        }

        let result = compute_file_diff(&files, dir.path(), None, &stored);
        prop_assert!(result.is_ok());

        let diff = result.unwrap();
        let discovered_set: HashSet<String> = files.iter().map(|f| f.source_path.clone()).collect();

        // unchanged ∪ changed ∪ new == discovered_set
        let discovered_union: HashSet<String> = diff
            .unchanged
            .union(&diff.changed)
            .cloned()
            .collect::<HashSet<_>>()
            .union(&diff.new)
            .cloned()
            .collect();
        prop_assert_eq!(discovered_union, discovered_set.clone());

        // deleted == stored_keys - discovered_set
        let expected_deleted: HashSet<String> = stored
            .keys()
            .filter(|k| !discovered_set.contains(*k))
            .cloned()
            .collect();
        prop_assert_eq!(diff.deleted, expected_deleted);
    }
}

// ---------------------------------------------------------------------------
// Proptest 5: DiffStatus classification rules
// ---------------------------------------------------------------------------

proptest! {
    #[test]
    fn proptest_classification_rules(
        path in "[a-z]{3,8}\\.md",
        file_content in any::<Vec<u8>>().prop_filter("bounded", |b| b.len() <= 256),
        config_content in any::<Vec<u8>>().prop_filter("bounded", |b| b.len() <= 256),
        classification_seed in 0..4u8,
    ) {
        let dir = tempfile::TempDir::new().unwrap();
        write_file(dir.path(), &path, &file_content);

        let files = vec![make_discovery(&path, file_content.len() as u64)];
        let config_path = dir.path().join("cfg.yaml");
        fs::write(&config_path, &config_content).unwrap();

        let current_config_hash = content_hash(&config_content);
        let current_content_hash = content_hash(&file_content);

        let stored = match classification_seed {
            0 => {
                // Both match -> Unchanged
                let mut m = HashMap::new();
                m.insert(path.clone(), StoredHashes {
                    content_hash: current_content_hash,
                    config_hash: current_config_hash,
                });
                m
            }
            1 => {
                // Content differs -> Changed
                let mut m = HashMap::new();
                m.insert(path.clone(), StoredHashes {
                    content_hash: content_hash(b"different content"),
                    config_hash: current_config_hash,
                });
                m
            }
            2 => {
                // Config differs -> Changed
                let mut m = HashMap::new();
                m.insert(path.clone(), StoredHashes {
                    content_hash: current_content_hash,
                    config_hash: content_hash(b"different config"),
                });
                m
            }
            3 => {
                // Not in stored -> New
                HashMap::new()
            }
            _ => HashMap::new(),
        };

        let result = compute_file_diff(&files, dir.path(), Some(&config_path), &stored);
        prop_assert!(result.is_ok());

        let diff = result.unwrap();
        match classification_seed {
            0 => {
                prop_assert!(diff.unchanged.contains(&path), "expected Unchanged");
                prop_assert!(!diff.changed.contains(&path));
                prop_assert!(!diff.new.contains(&path));
            }
            1 | 2 => {
                prop_assert!(diff.changed.contains(&path), "expected Changed");
                prop_assert!(!diff.unchanged.contains(&path));
                prop_assert!(!diff.new.contains(&path));
            }
            3 => {
                prop_assert!(diff.new.contains(&path), "expected New");
                prop_assert!(!diff.unchanged.contains(&path));
                prop_assert!(!diff.changed.contains(&path));
            }
            _ => {}
        }
        prop_assert!(!diff.deleted.contains(&path), "discovered files are never Deleted");
    }
}

// ---------------------------------------------------------------------------
// Proptest 6: Rayon determinism with duplicate source paths
// ---------------------------------------------------------------------------

proptest! {
    #[test]
    fn proptest_rayon_determinism_with_duplicates(
        unique_names in prop::collection::vec("[a-z]{2,5}\\.md", 2..=5usize),
        content_seed in any::<u64>(),
    ) {
        let dir = tempfile::TempDir::new().unwrap();

        let mut files = Vec::new();
        let mut stored = HashMap::new();

        for (idx, name) in unique_names.iter().enumerate() {
            let content = format!("content_{content_seed}_{idx}");
            write_file(dir.path(), name, content.as_bytes());

            // Each path appears 2 times
            files.push(make_discovery(name, content.len() as u64));
            files.push(make_discovery(name, content.len().saturating_add(1) as u64));

            // Random match/mismatch
            if idx % 2 == 0 {
                stored.insert(name.clone(), make_stored(content.as_bytes(), b""));
            }
        }

        // Run 5 times — all results must be identical
        let results: Vec<FileDiff> = (0..5)
            .map(|_| compute_file_diff(&files, dir.path(), None, &stored))
            .collect::<Result<Vec<_>, _>>()
            .expect("all diff calls should succeed");

        for result in &results {
            assert_pairwise_disjoint(result);
        }

        let first = &results[0];
        for (idx, result) in results.iter().enumerate().skip(1) {
            prop_assert_eq!(
                result,
                first,
                "result at index {} must equal first",
                idx
            );
        }
    }
}
