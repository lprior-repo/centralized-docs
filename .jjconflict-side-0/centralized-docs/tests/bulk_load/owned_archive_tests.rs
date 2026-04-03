//! Unit tests for `OwnedArchive<T>` construction, byte preservation, archived access, and deserialization.
//!
//! Covers Behaviors 1–5 from the test plan.

use super::common::*;
use doc_transformer::persisted::{PersistedAnalysis, PersistedAnalyzeResult};
use doc_transformer::state::bulk_load::{BulkLoadError, OwnedArchive};

// ===========================================================================
// Behavior 1: OwnedArchive returns valid with concrete byte length and field values
// ===========================================================================

#[test]
fn owned_archive_returns_concrete_value_when_bytes_pass_bytecheck() {
    let original = sample_analysis("test.md", 42);
    let bytes = rkyv_serialize!(&original);
    let byte_len = bytes.len();
    let hash = [0xAB_u8; 32];

    let archive = OwnedArchive::<PersistedAnalyzeResult>::try_from_bytes(
        "test_table",
        &hash,
        bytes.clone().into_boxed_slice(),
    )
    .unwrap();

    assert_eq!(archive.as_bytes().len(), byte_len);
    let archived = archive.archived().unwrap();
    assert_eq!(archived.analyses.len(), 1);
    assert_eq!(archived.analyses[0].source_path.as_ref(), "test.md");
    assert_eq!(archived.analyses[0].word_count, 42);
}

// ===========================================================================
// Behavior 2: OwnedArchive returns CorruptPayload on invalid bytes
// ===========================================================================

#[test]
fn owned_archive_returns_corrupt_payload_when_bytes_fail_bytecheck() {
    let garbage: Box<[u8]> = Box::from([0xFF_u8; 64]);
    let hash = [0xAA_u8; 32];

    let result =
        OwnedArchive::<PersistedAnalyzeResult>::try_from_bytes("analysis_outputs", &hash, garbage);

    let err = result.unwrap_err();
    assert!(
        matches!(err, BulkLoadError::CorruptPayload { table, ref key_hex, .. } if table == "analysis_outputs"
            && key_hex == &hex_encode_32(&hash))
    );
    assert!(!err.to_string().is_empty());
}

// ===========================================================================
// Behavior 3: OwnedArchive preserves exact input bytes
// ===========================================================================

#[test]
fn owned_archive_preserves_exact_input_bytes_when_constructed() {
    let original = sample_analysis("bytes.md", 10);
    let bytes = rkyv_serialize!(&original);
    let hash = [0x01_u8; 32];

    let archive = OwnedArchive::<PersistedAnalyzeResult>::try_from_bytes(
        "t",
        &hash,
        bytes.clone().into_boxed_slice(),
    )
    .unwrap();

    assert_eq!(archive.as_bytes(), bytes.as_slice());
}

// ===========================================================================
// Behavior 4: OwnedArchive `archived()` returns reference with matching field values
// ===========================================================================

#[test]
fn owned_archive_archived_returns_matching_field_values_when_called() {
    let original = sample_analysis("fields.md", 100);
    let bytes = rkyv_serialize!(&original);
    let hash = [0x02_u8; 32];

    let archive = OwnedArchive::<PersistedAnalyzeResult>::try_from_bytes(
        "t",
        &hash,
        bytes.into_boxed_slice(),
    )
    .unwrap();

    let archived = archive.archived().unwrap();
    assert_eq!(archived.analyses[0].source_path.as_ref(), "fields.md");
    assert_eq!(archived.analyses[0].word_count, 100);
}

// ===========================================================================
// Behavior 5: OwnedArchive `deserialize()` returns owned T
// ===========================================================================

#[test]
fn owned_archive_deserialize_returns_owned_value_when_valid() {
    let original = PersistedAnalyzeResult {
        schema_version: 1,
        analyses: vec![PersistedAnalysis {
            schema_version: 1,
            source_path: "roundtrip.md".to_string(),
            title: "Roundtrip".to_string(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: "fp".to_string(),
            word_count: 999,
            has_code: true,
            has_tables: false,
            category: "cat".to_string(),
            content: "body".to_string(),
        }],
        failed_files: vec![],
        total_discovered: 1,
    };

    let bytes = rkyv_serialize!(&original);
    let hash = [0x03_u8; 32];

    let archive = OwnedArchive::<PersistedAnalyzeResult>::try_from_bytes(
        "t",
        &hash,
        bytes.into_boxed_slice(),
    )
    .unwrap();

    let deserialized = archive.deserialize().unwrap();
    assert_eq!(deserialized.analyses[0].source_path, "roundtrip.md");
    assert_eq!(deserialized.analyses[0].word_count, 999);
    assert_eq!(deserialized.analyses[0].has_code, true);
}
