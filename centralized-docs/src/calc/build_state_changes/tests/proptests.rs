//! Proptests for build_state_changes.

use super::*;
use proptest::prelude::*;

// Proptest 1: hash_payload determinism
proptest! {
    #[test]
    fn proptest_hash_payload_is_deterministic(input in prop::collection::vec(any::<u8>(), 0..1024)) {
        let hash1 = hash_payload(&input);
        let hash2 = hash_payload(&input);
        prop_assert_eq!(hash1, hash2, "hash_payload must be deterministic");
    }
}

// Proptest 2: hash_payload injectivity
proptest! {
    #[test]
    fn proptest_hash_payload_is_injective_for_distinct_inputs(
        a in prop::collection::vec(any::<u8>(), 0..1024),
        b in prop::collection::vec(any::<u8>(), 0..1024),
    ) {
        prop_assume!(a != b);
        let hash_a = hash_payload(&a);
        let hash_b = hash_payload(&b);
        prop_assert_ne!(hash_a, hash_b, "distinct inputs must produce distinct hashes");
    }
}

// Proptest 3: build_file_state_raw field preservation
proptest! {
    #[test]
    fn proptest_build_file_state_raw_preserves_all_fields(
        content in any::<[u8; 32]>(),
        config in any::<[u8; 32]>(),
        analysis in any::<[u8; 32]>(),
        transform in any::<[u8; 32]>(),
        chunk in any::<[u8; 32]>(),
        now_secs in any::<u64>(),
    ) {
        let raw = build_file_state_raw(content, config, analysis, transform, chunk, now_secs);
        prop_assert_eq!(raw.content_hash, content);
        prop_assert_eq!(raw.config_hash, config);
        prop_assert_eq!(raw.analysis_hash, analysis);
        prop_assert_eq!(raw.transform_hash, transform);
        prop_assert_eq!(raw.chunk_hash, chunk);
        prop_assert_eq!(raw.last_processed_secs, now_secs);
        prop_assert_eq!(raw.reserved, [0u8; 32]);
    }
}

// Proptest 4: Count invariants (INV-01, INV-02, INV-03)
proptest! {
    #[test]
    fn proptest_count_invariants_hold_for_valid_inputs(
        changed_count in 0usize..5,
        new_count in 0usize..5,
        deleted_count in 0usize..5,
    ) {
        prop_assume!(changed_count + new_count + deleted_count > 0);

        let changed_paths: Vec<String> = (0..changed_count).map(|i| format!("changed/{i}.md")).collect();
        let new_paths: Vec<String> = (0..new_count).map(|i| format!("new/{i}.md")).collect();
        let deleted_paths: Vec<String> = (0..deleted_count).map(|i| format!("deleted/{i}.md")).collect();

        let all_active: Vec<&str> = changed_paths.iter()
            .chain(new_paths.iter())
            .map(std::string::String::as_str)
            .collect();

        let diff = FileDiff {
            unchanged: vec![],
            changed: changed_paths.iter().map(|p| make_discovery_file(p)).collect(),
            new_files: new_paths.iter().map(|p| make_discovery_file(p)).collect(),
            deleted: deleted_paths.clone(),
        };
        let outputs = make_pipeline_outputs_for(&all_active);

        let changes = build_file_state_changes(&diff, &outputs).expect("should succeed");

        let expected = changed_count + new_count;
        prop_assert_eq!(changes.updated_files.len(), expected, "INV-01");
        prop_assert_eq!(changes.deleted_files.len(), deleted_count, "INV-02");
        prop_assert_eq!(changes.new_analyses.len(), expected, "INV-03 analyses");
        prop_assert_eq!(changes.new_transforms.len(), expected, "INV-03 transforms");
        prop_assert_eq!(changes.new_chunks.len(), expected, "INV-03 chunks");
    }
}

// Proptest 5: Hash consistency (INV-04, INV-05)
proptest! {
    #[test]
    fn proptest_hash_consistency_invariants_hold(
        changed_count in 1usize..5,
        new_count in 0usize..3,
    ) {
        let changed_paths: Vec<String> = (0..changed_count).map(|i| format!("c/{i}.md")).collect();
        let new_paths: Vec<String> = (0..new_count).map(|i| format!("n/{i}.md")).collect();
        let all_active: Vec<&str> = changed_paths.iter().chain(new_paths.iter()).map(String::as_str).collect();

        let diff = FileDiff {
            unchanged: vec![],
            changed: changed_paths.iter().map(|p| make_discovery_file(p)).collect(),
            new_files: new_paths.iter().map(|p| make_discovery_file(p)).collect(),
            deleted: vec![],
        };
        let outputs = make_pipeline_outputs_for(&all_active);
        let changes = build_file_state_changes(&diff, &outputs).expect("ok");

        for (_path, state) in &changes.updated_files {
            let a_keys: Vec<[u8; 32]> = changes.new_analyses.iter().map(|(k, _)| *k).collect();
            let t_keys: Vec<[u8; 32]> = changes.new_transforms.iter().map(|(k, _)| *k).collect();
            let c_keys: Vec<[u8; 32]> = changes.new_chunks.iter().map(|(k, _)| *k).collect();
            prop_assert!(a_keys.contains(&state.analysis_hash), "INV-04 analysis");
            prop_assert!(t_keys.contains(&state.transform_hash), "INV-04 transform");
            prop_assert!(c_keys.contains(&state.chunk_hash), "INV-04 chunk");
        }

        let a_refs: Vec<[u8; 32]> = changes.updated_files.iter().map(|(_, s)| s.analysis_hash).collect();
        for (key, _) in &changes.new_analyses {
            prop_assert_eq!(a_refs.iter().filter(|h| *h == key).count(), 1, "INV-05");
        }
    }
}

// Proptest 6: Determinism
proptest! {
    #[test]
    fn proptest_build_changes_is_deterministic(file_count in 1usize..5) {
        let paths: Vec<String> = (0..file_count).map(|i| format!("det/{i}.md")).collect();
        let path_refs: Vec<&str> = paths.iter().map(String::as_str).collect();
        let diff = make_diff_with_changed(&path_refs);
        let outputs = make_pipeline_outputs_for(&path_refs);
        let c1 = build_file_state_changes(&diff, &outputs).expect("ok");
        let c2 = build_file_state_changes(&diff, &outputs).expect("ok");
        prop_assert_eq!(c1.updated_files, c2.updated_files);
        prop_assert_eq!(c1.new_analyses, c2.new_analyses);
        prop_assert_eq!(c1.new_transforms, c2.new_transforms);
        prop_assert_eq!(c1.new_chunks, c2.new_chunks);
    }
}

// Proptest 7: Unchanged exclusion (INV-06)
proptest! {
    #[test]
    fn proptest_unchanged_files_never_appear_in_output(
        unchanged_count in 0usize..5,
        changed_count in 1usize..5,
    ) {
        let uc_paths: Vec<String> = (0..unchanged_count).map(|i| format!("unchanged/{i}.md")).collect();
        let ch_paths: Vec<String> = (0..changed_count).map(|i| format!("changed/{i}.md")).collect();
        let active_refs: Vec<&str> = ch_paths.iter().map(String::as_str).collect();

        let diff = FileDiff {
            unchanged: uc_paths.iter().map(|p| make_unchanged_entry(p)).collect(),
            changed: ch_paths.iter().map(|p| make_discovery_file(p)).collect(),
            new_files: vec![],
            deleted: vec![],
        };
        let outputs = make_pipeline_outputs_for(&active_refs);
        let changes = build_file_state_changes(&diff, &outputs).expect("ok");

        let updated: Vec<&str> = changes.updated_files.iter().map(|(p, _)| p.as_str()).collect();
        for uc in &uc_paths {
            prop_assert!(!updated.contains(&uc.as_str()), "INV-06: {uc}");
        }
    }
}

// Proptest 8: serialize_and_hash hash integrity
proptest! {
    #[test]
    fn proptest_serialize_and_hash_hash_matches_sha256_of_bytes(input in ".*") {
        let (hash, bytes) = serialize_and_hash(&input, "proptest.md").expect("ok");
        prop_assert_eq!(hash, hash_payload(&bytes));
    }
}
