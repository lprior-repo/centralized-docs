//! Serde roundtrip tests and error variant tests (B12-B14, error variants).

use super::*;
use crate::cache::ContentHash;

fn content_hash_from(b: &[u8]) -> ContentHash {
    ContentHash::compute(b)
}

#[test]
fn transform_artifact_serde_roundtrip_preserves_all_fields() {
    let artifact = TransformArtifact {
        source_path: "concepts/architecture.md".to_string(),
        content_hash: content_hash_from(b"hello"),
        link_map_fingerprint: content_hash_from(b"world"),
        transformed_markdown: "---\nid: foo\n---\ncontent".to_string(),
    };
    let json = serde_json::to_string(&artifact).expect("serialize");
    let roundtrip: TransformArtifact = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(roundtrip, artifact);
}

#[test]
fn transform_artifact_serde_roundtrip_preserves_empty_markdown() {
    let artifact = TransformArtifact {
        source_path: "a.md".to_string(),
        content_hash: content_hash_from(b"x"),
        link_map_fingerprint: content_hash_from(b"y"),
        transformed_markdown: String::new(),
    };
    let json = serde_json::to_string(&artifact).expect("serialize");
    let roundtrip: TransformArtifact = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(roundtrip, artifact);
}

#[test]
fn transform_artifact_serde_roundtrip_preserves_unicode_markdown() {
    let artifact = TransformArtifact {
        source_path: "\u{65e5}\u{672c}\u{8a9e}/docs.md".to_string(),
        content_hash: content_hash_from(b"x"),
        link_map_fingerprint: content_hash_from(b"y"),
        transformed_markdown:
            "---\nid: \u{30c6}\u{30b9}\u{30c8}\n---\n\u{5185}\u{5bb9} \u{1f389} \u{00e9}moji"
                .to_string(),
    };
    let json = serde_json::to_string(&artifact).expect("serialize");
    let roundtrip: TransformArtifact = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(roundtrip, artifact);
}

#[test]
fn transform_artifact_rejects_corrupt_json() {
    let result = serde_json::from_str::<TransformArtifact>("not valid json{{{");
    assert!(result.is_err());
}

#[test]
fn transform_artifact_rejects_json_with_wrong_types() {
    let result = serde_json::from_str::<TransformArtifact>(
        r#"{"source_path":123,"content_hash":[0;32],"link_map_fingerprint":[0;32],"transformed_markdown":"x"}"#,
    );
    assert!(result.is_err());
}

#[test]
fn transform_artifact_rejects_json_with_missing_fields() {
    let result = serde_json::from_str::<TransformArtifact>(r#"{"source_path":"a.md"}"#);
    assert!(result.is_err());
}

#[test]
fn error_variant_empty_source_path_display() {
    assert!(TransformArtifactError::EmptySourcePath
        .to_string()
        .contains("empty source path"));
}

#[test]
fn error_variant_missing_id_mapping_display() {
    assert!(TransformArtifactError::MissingIdMapping {
        source_path: "orphan.md".to_string()
    }
    .to_string()
    .contains("orphan.md"));
}

#[test]
fn error_variant_link_map_fingerprint_failed_display() {
    assert!(TransformArtifactError::LinkMapFingerprintFailed {
        message: "serialization error".to_string()
    }
    .to_string()
    .contains("serialization error"));
}

#[test]
fn error_variant_cache_read_failed_display() {
    let msg = TransformArtifactError::CacheReadFailed {
        source_path: "a.md".to_string(),
        message: "io error".to_string(),
    }
    .to_string();
    assert!(msg.contains("a.md"));
    assert!(msg.contains("io error"));
}

#[test]
fn error_variant_cache_write_failed_display() {
    let msg = TransformArtifactError::CacheWriteFailed {
        source_path: "b.md".to_string(),
        message: "disk full".to_string(),
    }
    .to_string();
    assert!(msg.contains("b.md"));
    assert!(msg.contains("disk full"));
}

#[test]
fn error_variant_deserialization_failed_display() {
    let msg = TransformArtifactError::DeserializationFailed {
        source_path: "c.md".to_string(),
        message: "corrupt data".to_string(),
    }
    .to_string();
    assert!(msg.contains("c.md"));
    assert!(msg.contains("corrupt data"));
}

#[test]
fn error_variant_file_read_failed_display() {
    let msg = TransformArtifactError::FileReadFailed {
        source_path: "nonexistent.md".to_string(),
        message: "no such file".to_string(),
    }
    .to_string();
    assert!(msg.contains("nonexistent.md"));
    assert!(msg.contains("no such file"));
}

#[test]
fn error_variant_transform_computation_failed_display() {
    let msg = TransformArtifactError::TransformComputationFailed {
        source_path: "bad.md".to_string(),
        message: "parse error".to_string(),
    }
    .to_string();
    assert!(msg.contains("bad.md"));
    assert!(msg.contains("parse error"));
}

#[test]
fn error_variant_output_write_failed_display() {
    let msg = TransformArtifactError::OutputWriteFailed {
        source_path: "a.md".to_string(),
        message: "permission denied".to_string(),
    }
    .to_string();
    assert!(msg.contains("a.md"));
    assert!(msg.contains("permission denied"));
}
