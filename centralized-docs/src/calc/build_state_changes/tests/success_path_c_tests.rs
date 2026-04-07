//! B30-B38: Pure function tests (hash_payload, serialize_and_hash, build_file_state_raw).

use super::*;

// B30: SHA-256 known test vector
#[test]
fn hash_payload_returns_sha256_of_input_bytes() {
    let input = b"hello world";
    let expected: [u8; 32] = [
        0xb9, 0x4d, 0x27, 0xb9, 0x93, 0x4d, 0x3e, 0x08, 0xa5, 0x2e, 0x52, 0xd7, 0xda, 0x7d, 0xab,
        0xfa, 0xc4, 0x84, 0xef, 0xe3, 0x7a, 0x53, 0x80, 0xee, 0x90, 0x88, 0xf7, 0xac, 0xe2, 0xef,
        0xcd, 0xe9,
    ];
    assert_eq!(hash_payload(input), expected);
}

// B31: Determinism
#[test]
fn hash_payload_produces_identical_output_for_identical_input() {
    let input = b"determinism test data";
    assert_eq!(hash_payload(input), hash_payload(input));
}

// B32: Non-zero for non-empty input
#[test]
fn hash_payload_returns_non_zero_hash_for_non_empty_input() {
    assert_ne!(hash_payload(b"any non-empty data"), [0u8; 32]);
}

// B33: serialize_and_hash returns hash and bytes
#[test]
fn serialize_and_hash_returns_hash_and_bytes_for_valid_input() {
    let (hash, bytes) = serialize_and_hash("test transform content", "test/path.md").expect("ok");
    assert!(!bytes.is_empty());
    assert_eq!(hash, hash_payload(&bytes));
}

// B33b: Empty string
#[test]
fn serialize_and_hash_handles_empty_string_without_panic() {
    let (hash, bytes) = serialize_and_hash("", "test.md").expect("ok");
    assert!(!bytes.is_empty());
    assert_eq!(hash, hash_payload(&bytes));
}

// B33c: Large value (64KB)
#[test]
fn serialize_and_hash_handles_large_value_without_panic() {
    let large_value = "a".repeat(65536);
    let (hash, bytes) = serialize_and_hash(&large_value, "large.md").expect("ok");
    assert!(bytes.len() >= 65536);
    assert_eq!(hash, hash_payload(&bytes));
}

// B33d: Empty path string
#[test]
fn serialize_and_hash_handles_empty_path_string_without_panic() {
    let (hash, bytes) = serialize_and_hash("some content", "").expect("ok");
    assert_eq!(hash, hash_payload(&bytes));
}

// B35: build_file_state_raw sets all hash fields
#[test]
fn build_file_state_raw_sets_all_hash_fields_to_provided_values() {
    let raw = build_file_state_raw(
        make_hash(1),
        make_hash(2),
        make_hash(3),
        make_hash(4),
        make_hash(5),
        1_700_000_000,
    );
    assert_eq!(raw.content_hash, make_hash(1));
    assert_eq!(raw.config_hash, make_hash(2));
    assert_eq!(raw.analysis_hash, make_hash(3));
    assert_eq!(raw.transform_hash, make_hash(4));
    assert_eq!(raw.chunk_hash, make_hash(5));
}

// B36: last_processed_secs
#[test]
fn build_file_state_raw_sets_last_processed_secs() {
    let raw = build_file_state_raw(
        make_hash(0),
        make_hash(0),
        make_hash(0),
        make_hash(0),
        make_hash(0),
        1_700_000_000,
    );
    assert_eq!(raw.last_processed_secs, 1_700_000_000);
}

// B37: Reserved zeroed
#[test]
fn build_file_state_raw_zeroesreserved_field() {
    let raw = build_file_state_raw(
        make_hash(0xFF),
        make_hash(0xFF),
        make_hash(0xFF),
        make_hash(0xFF),
        make_hash(0xFF),
        999,
    );
    assert_eq!(raw.reserved, [0u8; 32]);
}

// B38: Struct size
#[test]
fn build_file_state_raw_produces_200_byte_struct() {
    let raw = build_file_state_raw(
        make_hash(0),
        make_hash(0),
        make_hash(0),
        make_hash(0),
        make_hash(0),
        0,
    );
    assert_eq!(std::mem::size_of_val(&raw), 200);
}

// Boundary: all zeros
#[test]
fn build_file_state_raw_handles_all_zero_hashes() {
    let raw = build_file_state_raw([0u8; 32], [0u8; 32], [0u8; 32], [0u8; 32], [0u8; 32], 0);
    assert_eq!(raw.content_hash, [0u8; 32]);
    assert_eq!(raw.last_processed_secs, 0);
    assert_eq!(raw.reserved, [0u8; 32]);
}

// Boundary: max values
#[test]
fn build_file_state_raw_handles_max_values() {
    let max_hash = [0xFFu8; 32];
    let raw = build_file_state_raw(max_hash, max_hash, max_hash, max_hash, max_hash, u64::MAX);
    assert_eq!(raw.content_hash, max_hash);
    assert_eq!(raw.last_processed_secs, u64::MAX);
    assert_eq!(raw.reserved, [0u8; 32]);
}

// hash_payload boundary: empty input
#[test]
fn hash_payload_handles_empty_bytes() {
    let expected: [u8; 32] = [
        0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9,
        0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52,
        0xb8, 0x55,
    ];
    assert_eq!(hash_payload(b""), expected);
}

// hash_payload boundary: single byte
#[test]
fn hash_payload_handles_single_byte() {
    let expected: [u8; 32] = [
        0xca, 0x97, 0x81, 0x12, 0xca, 0x1b, 0xbd, 0xca, 0xfa, 0xc2, 0x31, 0xb3, 0x9a, 0x23, 0xdc,
        0x4d, 0xa7, 0x86, 0xef, 0xf8, 0x14, 0x7c, 0x4e, 0x72, 0xb9, 0x80, 0x77, 0x85, 0xaf, 0xee,
        0x48, 0xbb,
    ];
    assert_eq!(hash_payload(b"a"), expected);
}

// hash_payload boundary: large input (1MB)
#[test]
fn hash_payload_handles_large_input() {
    let large_input = vec![0u8; 1_048_576];
    assert_ne!(hash_payload(&large_input), [0u8; 32]);
}

// Ignored tests for serialization failure (serde_json can't fail for these types)
#[test]
#[ignore = "requires rkyv serialization to trigger failure; serde_json cannot fail for Analysis"]
fn build_changes_returns_analysis_serialization_failed_on_rkyv_error() {
    let path = "docs/fail.md";
    let diff = make_diff_with_changed(&[path]);
    let outputs = make_pipeline_outputs_for(&[path]);
    match build_file_state_changes(&diff, &outputs) {
        Err(BatchBuildError::AnalysisSerializationFailed { path: p, reason }) => {
            assert_eq!(p, "docs/fail.md");
            assert!(!reason.is_empty());
        }
        other => panic!("expected AnalysisSerializationFailed, got {other:?}"),
    }
}

#[test]
#[ignore = "requires rkyv serialization to trigger failure"]
fn build_changes_returns_transform_serialization_failed_on_rkyv_error() {
    let path = "docs/fail_transform.md";
    let diff = make_diff_with_changed(&[path]);
    let outputs = make_pipeline_outputs_for(&[path]);
    match build_file_state_changes(&diff, &outputs) {
        Err(BatchBuildError::TransformSerializationFailed { path: p, reason }) => {
            assert_eq!(p, "docs/fail_transform.md");
            assert!(!reason.is_empty());
        }
        other => panic!("expected TransformSerializationFailed, got {other:?}"),
    }
}

#[test]
#[ignore = "requires rkyv serialization to trigger failure"]
fn build_changes_returns_chunk_serialization_failed_on_rkyv_error() {
    let path = "docs/fail_chunks.md";
    let diff = make_diff_with_changed(&[path]);
    let outputs = make_pipeline_outputs_for(&[path]);
    match build_file_state_changes(&diff, &outputs) {
        Err(BatchBuildError::ChunkSerializationFailed { path: p, reason }) => {
            assert_eq!(p, "docs/fail_chunks.md");
            assert!(!reason.is_empty());
        }
        other => panic!("expected ChunkSerializationFailed, got {other:?}"),
    }
}

#[test]
#[ignore = "requires rkyv serialization to trigger failure"]
fn serialize_and_hash_includes_path_and_reason_in_error_when_serialization_fails() {
    match serialize_and_hash("value", "docs/fail.md") {
        Err(BatchBuildError::AnalysisSerializationFailed { path, reason }) => {
            assert_eq!(path, "docs/fail.md");
            assert!(!reason.is_empty());
        }
        Err(BatchBuildError::TransformSerializationFailed { path, reason }) => {
            assert_eq!(path, "docs/fail.md");
            assert!(!reason.is_empty());
        }
        Err(BatchBuildError::ChunkSerializationFailed { path, reason }) => {
            assert_eq!(path, "docs/fail.md");
            assert!(!reason.is_empty());
        }
        Ok((hash, bytes)) => {
            assert_eq!(hash, hash_payload(&bytes));
            panic!("should fail but succeeded with {} bytes", bytes.len());
        }
        Err(other) => panic!("wrong error variant: {other:?}"),
    }
}
