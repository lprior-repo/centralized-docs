//! OwnedArchive construction and access tests.

use super::*;
use crate::persisted::PersistedAnalyzeResult;

// =======================================================================
// Construction failure tests
// =======================================================================

#[test]
fn owned_archive_try_from_bytes_returns_corrupt_payload_for_garbage() {
    let garbage: Box<[u8]> =
        vec![0xDE, 0xAD, 0xBE, 0xEF, 0xFF, 0xFF, 0xFF, 0xFF].into_boxed_slice();
    let key: [u8; 32] = [0x42; 32];
    let result =
        OwnedArchive::<PersistedAnalyzeResult>::try_from_bytes("analysis_outputs", &key, garbage);
    let err = result.unwrap_err();
    assert!(
        matches!(
            &err,
            BulkLoadError::CorruptPayload {
                table: "analysis_outputs",
                key_hex,
                message: _,
            } if *key_hex == hex_encode(&key)
        ),
        "expected CorruptPayload with exact table and key_hex, got {err:?}"
    );
}

#[test]
fn owned_archive_try_from_bytes_returns_corrupt_payload_for_empty_bytes() {
    let empty: Box<[u8]> = Box::new([]);
    let key: [u8; 32] = [0x00; 32];
    let result =
        OwnedArchive::<PersistedAnalyzeResult>::try_from_bytes("transform_outputs", &key, empty);
    let err = result.unwrap_err();
    assert!(
        matches!(
            &err,
            BulkLoadError::CorruptPayload {
                table: "transform_outputs",
                ..
            }
        ),
        "expected CorruptPayload for empty bytes, got {err:?}"
    );
}

#[test]
fn owned_archive_try_from_bytes_returns_corrupt_payload_for_truncated_rkyv() {
    let truncated: Box<[u8]> = vec![0u8].into_boxed_slice();
    let key: [u8; 32] = [0xFF; 32];
    use crate::persisted::PersistedTransformResult;
    let result = OwnedArchive::<PersistedTransformResult>::try_from_bytes(
        "transform_outputs",
        &key,
        truncated,
    );
    let err = result.unwrap_err();
    assert!(
        matches!(
            &err,
            BulkLoadError::CorruptPayload {
                table: "transform_outputs",
                key_hex,
                ..
            } if *key_hex == hex_encode(&key)
        ),
        "expected CorruptPayload for truncated bytes, got {err:?}"
    );
}

// =======================================================================
// Construction success and access tests
// =======================================================================

#[test]
fn owned_archive_as_bytes_returns_exact_input_bytes_when_valid() {
    use crate::persisted::PersistedTransformResult;
    let original = PersistedTransformResult {
        schema_version: 1,
        success_count: 42,
        total_count: 50,
        error_count: 8,
        errors: vec![],
    };
    let rkyv_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&original)
        .expect("serialization should succeed")
        .to_vec()
        .into_boxed_slice();
    let rkyv_len = rkyv_bytes.len();
    let key: [u8; 32] = [0x11; 32];

    let archive = OwnedArchive::<PersistedTransformResult>::try_from_bytes(
        "transform_outputs",
        &key,
        rkyv_bytes.clone(),
    )
    .expect("valid rkyv bytes should construct archive");

    let retrieved_bytes = archive.as_bytes();
    assert_eq!(retrieved_bytes.len(), rkyv_len);
    assert_eq!(retrieved_bytes, rkyv_bytes.as_ref());
}

#[test]
fn owned_archive_archived_returns_valid_reference_when_constructed_from_valid_bytes() {
    use crate::persisted::PersistedTransformResult;
    let original = PersistedTransformResult {
        schema_version: 1,
        success_count: 10,
        total_count: 10,
        error_count: 0,
        errors: vec![],
    };
    let rkyv_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&original)
        .expect("serialization should succeed")
        .to_vec()
        .into_boxed_slice();
    let key: [u8; 32] = [0x22; 32];

    let archive = OwnedArchive::<PersistedTransformResult>::try_from_bytes(
        "transform_outputs",
        &key,
        rkyv_bytes,
    )
    .expect("valid rkyv bytes should construct archive");

    let archived = archive
        .archived()
        .expect("archived() should succeed on valid bytes");
    assert_eq!(archived.success_count, 10);
    assert_eq!(archived.total_count, 10);
    assert_eq!(archived.error_count, 0);
}

#[test]
fn owned_archive_archived_returns_consistent_results_on_repeated_calls() {
    use crate::persisted::PersistedTransformResult;
    let original = PersistedTransformResult {
        schema_version: 1,
        success_count: 7,
        total_count: 7,
        error_count: 0,
        errors: vec![],
    };
    let rkyv_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&original)
        .expect("serialization should succeed")
        .to_vec()
        .into_boxed_slice();
    let key: [u8; 32] = [0x77; 32];
    let archive = OwnedArchive::<PersistedTransformResult>::try_from_bytes(
        "transform_outputs",
        &key,
        rkyv_bytes,
    )
    .expect("valid rkyv bytes should construct archive");
    let first = archive.archived().expect("first call should succeed");
    let second = archive.archived().expect("second call should succeed");
    assert_eq!(first.success_count, second.success_count);
    assert_eq!(first.total_count, second.total_count);
}

#[test]
fn owned_archive_as_bytes_matches_serialize_output() {
    use crate::persisted::PersistedTransformResult;
    let original = PersistedTransformResult {
        schema_version: 1,
        success_count: 3,
        total_count: 5,
        error_count: 2,
        errors: vec![],
    };
    let rkyv_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&original).expect("ok");
    let boxed: Box<[u8]> = rkyv_bytes.to_vec().into_boxed_slice();
    let key: [u8; 32] = [0x88; 32];
    let archive =
        OwnedArchive::<PersistedTransformResult>::try_from_bytes("transform_outputs", &key, boxed)
            .expect("valid rkyv bytes should construct archive");
    assert_eq!(archive.as_bytes(), rkyv_bytes.as_slice());
}

#[test]
fn corrupt_payload_key_hex_matches_hex_encode_for_known_key() {
    let key: [u8; 32] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32,
        0x10, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54,
        0x32, 0x10,
    ];
    let garbage: Box<[u8]> = vec![0xDE, 0xAD].into_boxed_slice();
    let err =
        OwnedArchive::<PersistedAnalyzeResult>::try_from_bytes("analysis_outputs", &key, garbage)
            .unwrap_err();
    let expected_hex = hex_encode(&key);
    if let BulkLoadError::CorruptPayload { key_hex, .. } = &err {
        assert_eq!(
            key_hex, &expected_hex,
            "key_hex must match hex_encode of the input key"
        );
    } else {
        panic!("expected CorruptPayload, got {err:?}");
    }
}

// =======================================================================
// Proptest: OwnedArchive transform roundtrip
// =======================================================================

#[test]
fn proptest_owned_archive_transform_roundtrip_preserves_data() {
    use crate::persisted::PersistedTransformResult;
    use proptest::prelude::*;
    proptest!(
        |(success_count in 0usize..100_000, total_count in 0usize..100_000, error_count in 0usize..100_000)| {
        let original = PersistedTransformResult {
            schema_version: 1,
            success_count,
            total_count,
            error_count,
            errors: vec![],
        };
        let rkyv_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&original)
            .expect("serialization should succeed")
            .to_vec()
            .into_boxed_slice();
        let key: [u8; 32] = [0xAA; 32];

        let archive = OwnedArchive::<PersistedTransformResult>::try_from_bytes(
            "transform_outputs",
            &key,
            rkyv_bytes,
        )
        .expect("valid rkyv bytes should construct archive");

        let deserialized = archive
            .deserialize()
            .expect("deserialize should succeed");
        prop_assert_eq!(deserialized.success_count, success_count);
        prop_assert_eq!(deserialized.total_count, total_count);
        prop_assert_eq!(deserialized.error_count, error_count);
    });
}
