//! Proptest-based property tests for validation and commit logic.

use super::*;
use crate::state::commit::should_skip_write;
use crate::state::commit::validation::{
    check_payload_size, validate_no_duplicate_keys, validate_no_empty_string_keys,
    validate_no_zero_hashes, validate_reference_integrity,
};
use crate::state::commit::MAX_VALUE_SIZE;
use std::collections::HashSet;

#[test]
fn proptest_zero_hash_scan_exhaustive() {
    use proptest::prelude::*;
    proptest!(|(
        hash_a in proptest::array::uniform32(1u8..=255u8),
        hash_b in proptest::array::uniform32(1u8..=255u8),
        inject_zero in 0u8..4,
    )| {
        let mut changes = StateChanges::empty();
        let entries = vec![(hash_a, vec![1]), (hash_b, vec![2])];
        match inject_zero {
            0 => changes.new_analyses = vec![([0u8; 32], vec![0])],
            1 => changes.new_transforms = vec![([0u8; 32], vec![0])],
            2 => changes.new_chunks = vec![([0u8; 32], vec![0])],
            3 => changes.new_scrapes = vec![([0u8; 32], vec![0])],
            _ => {}
        }
        changes.new_analyses = [changes.new_analyses, entries.clone()].concat();
        changes.new_transforms = [changes.new_transforms, entries.clone()].concat();
        changes.new_chunks = [changes.new_chunks, entries.clone()].concat();
        changes.new_scrapes = [changes.new_scrapes, entries.clone()].concat();
        changes.new_snapshots = [changes.new_snapshots, entries.clone()].concat();
        let result = validate_no_zero_hashes(&changes);
        prop_assert!(
            matches!(result, Err(CommitError::ZeroHashKey { .. })),
            "must detect zero hash in vec {inject_zero}"
        );
    });
}

#[test]
fn proptest_duplicate_detection_order_independent() {
    use proptest::prelude::*;
    proptest!(|(keys in proptest::collection::vec(".*", 1..10))| {
        let mut changes = StateChanges::empty();
        changes.updated_files = keys.iter().enumerate().map(|(i, k)| {
            (k.clone(), FileStateRaw { content_hash: [i as u8; 32], ..FileStateRaw::zeroed() })
        }).collect();
        let has_dupes = keys.len() != keys.iter().collect::<HashSet<_>>().len();
        let result = validate_no_duplicate_keys(&changes);
        if has_dupes {
            assert!(matches!(result, Err(CommitError::DuplicateStateKey { .. })));
        } else {
            prop_assert!(result.is_ok());
        }
    });
}

#[test]
fn proptest_reference_integrity_complete() {
    use proptest::prelude::*;
    proptest!(|(
        analysis_hash in proptest::array::uniform32(1u8..=255u8),
        transform_hash in proptest::array::uniform32(1u8..=255u8),
        chunk_hash in proptest::array::uniform32(1u8..=255u8),
        omit_analysis in proptest::bool::ANY,
    )| {
        let mut changes = StateChanges::empty();
        if !omit_analysis { changes.new_analyses = vec![(analysis_hash, vec![1])]; }
        changes.new_transforms = vec![(transform_hash, vec![2])];
        changes.new_chunks = vec![(chunk_hash, vec![3])];
        changes.updated_files = vec![(
            "test.rs".to_string(),
            make_file_state_raw(analysis_hash, transform_hash, chunk_hash),
        )];
        let result = validate_reference_integrity(&changes);
        if omit_analysis {
            assert!(matches!(result, Err(CommitError::MissingReference { field: "analysis_hash", .. })));
        } else {
            prop_assert!(result.is_ok());
        }
    });
}

#[test]
fn proptest_atomicity_mixed_batches() {
    use proptest::prelude::*;
    proptest!(|(
        valid_hash in proptest::array::uniform32(1u8..=255u8),
        valid_bytes in proptest::collection::vec(0u8..=255u8, 0..100),
    )| {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("proptest_atomic.redb");
        let state_db = StateDb::open(&db_path).unwrap();
        let mut setup = StateChanges::empty();
        setup.new_analyses = vec![(valid_hash, valid_bytes.clone())];
        state_db.commit_changes(setup).unwrap();
        let mut invalid = StateChanges::empty();
        invalid.new_analyses = vec![([0u8; 32], vec![99])];
        let err = state_db.commit_changes(invalid);
        assert!(matches!(err, Err(CommitError::ZeroHashKey { .. })));
        let db = state_db.database();
        let stored = read_hash_table(db, analysis_outputs_table(), &valid_hash);
        prop_assert_eq!(stored, Some(valid_bytes));
    });
}

#[test]
fn proptest_should_skip_write_correctness() {
    use proptest::prelude::*;
    proptest!(|(
        a in proptest::collection::vec(0u8..=255u8, 0..256),
        b in proptest::collection::vec(0u8..=255u8, 0..256),
    )| {
        let expected = a == b;
        prop_assert_eq!(should_skip_write(&a, &b), expected);
    });
}

#[test]
fn proptest_empty_string_key_boundary_detection() {
    use proptest::prelude::*;
    proptest!(|(whitespace in "([ \t\n\r]{0,20})")| {
        let mut changes = StateChanges::empty();
        changes.updated_files = vec![(whitespace.clone(), FileStateRaw::zeroed())];
        let result = validate_no_empty_string_keys(&changes);
        prop_assert!(
            matches!(result, Err(CommitError::EmptyStringKey { table: "file_state", index: 0 })),
            "whitespace-only key '{}' should be rejected",
            whitespace.escape_unicode()
        );
    });
}

#[test]
fn proptest_non_empty_string_key_always_accepted() {
    use proptest::prelude::*;
    proptest!(|(key in r"[^\p{White_Space}\x00-\x1F\x7F]{1,10}")| {
        let mut changes = StateChanges::empty();
        changes.updated_files = vec![(key, FileStateRaw::zeroed())];
        let result = validate_no_empty_string_keys(&changes);
        prop_assert!(result.is_ok(), "non-whitespace key should be accepted");
    });
}

#[test]
fn proptest_validate_hash_key_classifies_by_length() {
    use crate::state::validate_hash_key;
    use proptest::prelude::*;
    proptest!(|(bytes in proptest::collection::vec(any::<u8>(), 0..64))| {
        let result = validate_hash_key(&bytes);
        if bytes.len() == 32 {
            prop_assert!(result.is_ok(), "32-byte key should be valid");
        } else {
            prop_assert!(
                matches!(result, Err(crate::state::StateError::InvalidHashKeyLength { actual }) if actual == bytes.len()),
                "non-32-byte key (len={}) should return InvalidHashKeyLength",
                bytes.len()
            );
        }
    });
}

#[test]
fn proptest_validate_source_path_rejects_invalid_patterns() {
    use crate::state::validate_source_path;
    use proptest::prelude::*;
    proptest!(|(s in ".*{0,50}")| {
        let result = validate_source_path(&s);
        let is_invalid = s.is_empty()
            || s.as_bytes().first() == Some(&b'/')
            || s.split('/').any(|c| c == "..");
        if is_invalid {
            prop_assert!(result.is_err(), "path '{}' should be rejected", s.escape_unicode());
        } else {
            prop_assert!(result.is_ok(), "valid path '{}' should be accepted", s.escape_unicode());
        }
    });
}

#[test]
fn proptest_validate_url_key_rejects_invalid_patterns() {
    use crate::state::validate_url_key;
    use proptest::prelude::*;
    proptest!(|(s in ".*{0,100}")| {
        let result = validate_url_key(&s);
        let is_invalid = s.is_empty() || !s.contains("://");
        if is_invalid {
            prop_assert!(result.is_err(), "URL '{}' should be rejected", s.escape_unicode());
        } else {
            prop_assert!(result.is_ok(), "URL with scheme '{}' should be accepted", s.escape_unicode());
        }
    });
}

#[test]
fn proptest_payload_size_boundary() {
    use proptest::prelude::*;
    proptest!(|(
        sizes in proptest::collection::vec(0usize..MAX_VALUE_SIZE + 2, 0..5),
    )| {
        let mut entries: Vec<([u8; 32], Vec<u8>)> = Vec::new();
        for (i, size) in sizes.iter().enumerate() {
            let mut hash = [0u8; 32];
            hash[0] = u8::try_from(i).unwrap_or(u8::MAX);
            entries.push((hash, vec![0u8; *size]));
        }
        let result = check_payload_size(&entries, "analysis_outputs");
        let has_oversized = entries.iter().any(|(_, v)| v.len() > MAX_VALUE_SIZE);
        if has_oversized {
            prop_assert!(
                matches!(result, Err(CommitError::PayloadTooLarge { table: "analysis_outputs", .. })),
                "oversized payload should be rejected"
            );
        } else {
            prop_assert!(result.is_ok(), "all valid sizes should be accepted");
        }
    });
}

#[test]
fn owned_archive_try_from_bytes_never_panics_on_arbitrary_bytes() {
    use crate::persisted::PersistedAnalyzeResult;
    use crate::state::bulk_load::{BulkLoadError, OwnedArchive};
    let seeds: &[&[u8]] = &[
        &[0xFF, 0xFF, 0xFF, 0xFF],
        &[],
        &[0u8; 64],
        &[0xFFu8; 256],
        &[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    ];
    for seed in seeds {
        let bytes: Box<[u8]> = seed.to_vec().into_boxed_slice();
        let key: [u8; 32] = [0x42; 32];
        let result =
            OwnedArchive::<PersistedAnalyzeResult>::try_from_bytes("analysis_outputs", &key, bytes);
        match result {
            Ok(_) | Err(BulkLoadError::CorruptPayload { .. }) => {}
            Err(other) => panic!("unexpected error variant: {other:?}"),
        }
    }
}
