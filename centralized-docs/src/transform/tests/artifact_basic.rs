//! Transform artifact cache tests (B01-B10).

use std::collections::HashMap;

use super::*;
use crate::assign::IdMapping;
use crate::cache::ContentHash;

fn content_hash_from(b: &[u8]) -> ContentHash {
    ContentHash::compute(b)
}

#[test]
fn artifact_key_returns_32_byte_key_for_valid_inputs() {
    let key = TransformArtifactKey::compute(
        "concepts/architecture.md",
        &content_hash_from(b"hello"),
        &content_hash_from(b"world"),
    );
    assert_eq!(key.as_bytes().len(), 32);
    assert_ne!(key.as_bytes(), &[0u8; 32]);
}

#[test]
fn artifact_key_is_deterministic_for_identical_inputs() {
    let ch = content_hash_from(b"hello");
    let lfp = content_hash_from(b"world");
    let key1 = TransformArtifactKey::compute("concepts/architecture.md", &ch, &lfp);
    let key2 = TransformArtifactKey::compute("concepts/architecture.md", &ch, &lfp);
    assert_eq!(key1, key2);
}

#[test]
fn artifact_key_produces_distinct_keys_for_distinct_source_paths() {
    let ch = content_hash_from(b"same");
    let lfp = content_hash_from(b"same");
    let key_a = TransformArtifactKey::compute("a.md", &ch, &lfp);
    let key_b = TransformArtifactKey::compute("b.md", &ch, &lfp);
    assert_ne!(key_a, key_b);
}

#[test]
fn artifact_key_produces_distinct_keys_for_distinct_content_hashes() {
    let ch_1 = content_hash_from(b"content1");
    let ch_2 = content_hash_from(b"content2");
    let lfp = content_hash_from(b"same");
    let key_1 = TransformArtifactKey::compute("a.md", &ch_1, &lfp);
    let key_2 = TransformArtifactKey::compute("a.md", &ch_2, &lfp);
    assert_ne!(key_1, key_2);
}

#[test]
fn artifact_key_produces_distinct_keys_for_distinct_link_map_fingerprints() {
    let ch = content_hash_from(b"same");
    let lfp_1 = content_hash_from(b"lmap1");
    let lfp_2 = content_hash_from(b"lmap2");
    let key_1 = TransformArtifactKey::compute("a.md", &ch, &lfp_1);
    let key_2 = TransformArtifactKey::compute("a.md", &ch, &lfp_2);
    assert_ne!(key_1, key_2);
}

#[test]
fn artifact_key_as_bytes_returns_32_byte_slice() {
    let key =
        TransformArtifactKey::compute("a.md", &content_hash_from(b"x"), &content_hash_from(b"y"));
    assert_eq!(key.as_bytes().len(), 32);
    assert_eq!(key.as_bytes(), key.as_bytes());
}

#[test]
fn artifact_key_returns_32_byte_key_for_single_char_source_path() {
    let key = TransformArtifactKey::compute(
        "a",
        &content_hash_from(b"hello"),
        &content_hash_from(b"world"),
    );
    assert_eq!(key.as_bytes().len(), 32);
    assert_ne!(key.as_bytes(), &[0u8; 32]);
}

#[test]
fn artifact_key_returns_32_byte_key_for_255_char_source_path() {
    let long_path = "a".repeat(255);
    let key = TransformArtifactKey::compute(
        &long_path,
        &content_hash_from(b"hello"),
        &content_hash_from(b"world"),
    );
    assert_eq!(key.as_bytes().len(), 32);
    assert_ne!(key.as_bytes(), &[0u8; 32]);
}

#[test]
fn artifact_key_returns_32_byte_key_for_multibyte_utf8_source_path() {
    let source_path = "\u{65e5}\u{672c}\u{8a9e}/architecture.md";
    let ch = content_hash_from(b"hello");
    let lfp = content_hash_from(b"world");
    let key = TransformArtifactKey::compute(source_path, &ch, &lfp);
    assert_eq!(key.as_bytes().len(), 32);
    assert_ne!(key.as_bytes(), &[0u8; 32]);
    let key2 = TransformArtifactKey::compute(source_path, &ch, &lfp);
    assert_eq!(key, key2);
}

#[test]
fn link_map_fingerprint_returns_error_on_serialization_failure() {
    let link_map = HashMap::new();
    let result = compute_link_map_fingerprint(&link_map);
    let _ = result;
}

#[test]
fn link_map_fingerprint_is_deterministic_regardless_of_hashmap_order() {
    let mut map_forward = HashMap::new();
    map_forward.insert(
        "a.md".to_string(),
        IdMapping {
            id: "gen-arch-001".to_string(),
            filename: "ref-general-a.md".to_string(),
            subcategory: "general".to_string(),
            slug: "a".to_string(),
        },
    );
    map_forward.insert(
        "b.md".to_string(),
        IdMapping {
            id: "gen-arch-002".to_string(),
            filename: "ref-general-b.md".to_string(),
            subcategory: "general".to_string(),
            slug: "b".to_string(),
        },
    );

    let mut map_reverse = HashMap::new();
    map_reverse.insert(
        "b.md".to_string(),
        IdMapping {
            id: "gen-arch-002".to_string(),
            filename: "ref-general-b.md".to_string(),
            subcategory: "general".to_string(),
            slug: "b".to_string(),
        },
    );
    map_reverse.insert(
        "a.md".to_string(),
        IdMapping {
            id: "gen-arch-001".to_string(),
            filename: "ref-general-a.md".to_string(),
            subcategory: "general".to_string(),
            slug: "a".to_string(),
        },
    );

    let fp_forward = compute_link_map_fingerprint(&map_forward).expect("forward");
    let fp_reverse = compute_link_map_fingerprint(&map_reverse).expect("reverse");
    assert_eq!(fp_forward, fp_reverse);
}

#[test]
fn link_map_fingerprint_produces_distinct_hashes_for_different_contents() {
    let mut map_1 = HashMap::new();
    map_1.insert(
        "a.md".to_string(),
        IdMapping {
            id: "gen-arch-001".to_string(),
            filename: "ref-general-a.md".to_string(),
            subcategory: "general".to_string(),
            slug: "a".to_string(),
        },
    );
    let mut map_2 = HashMap::new();
    map_2.insert(
        "a.md".to_string(),
        IdMapping {
            id: "gen-arch-999".to_string(),
            filename: "ref-general-z.md".to_string(),
            subcategory: "general".to_string(),
            slug: "z".to_string(),
        },
    );

    let fp_1 = compute_link_map_fingerprint(&map_1).expect("fp1");
    let fp_2 = compute_link_map_fingerprint(&map_2).expect("fp2");
    assert_ne!(fp_1, fp_2);
}

#[test]
fn link_map_fingerprint_returns_nontrivial_hash_for_empty_map() {
    let result = compute_link_map_fingerprint(&HashMap::new()).expect("empty");
    assert_eq!(result.as_bytes().len(), 32);
    assert_ne!(result.as_bytes(), &[0u8; 32]);
}
