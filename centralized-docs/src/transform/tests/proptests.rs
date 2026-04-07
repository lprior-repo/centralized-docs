//! Proptest and Kani verification for transform artifact cache.

use std::collections::HashMap;

use super::*;
use crate::assign::IdMapping;
use crate::cache::{composite_hash, ContentHash};
use proptest::prelude::*;

proptest! {
    #[test]
    fn proptest_artifact_key_determinism(
        source_path in "[a-zA-Z0-9/_.-]{1,100}",
        ch_bytes in any::<[u8; 32]>(),
        lfp_bytes in any::<[u8; 32]>()
    ) {
        let content_hash = ContentHash::from(ch_bytes);
        let link_map_fp = ContentHash::from(lfp_bytes);
        let key1 = TransformArtifactKey::compute(&source_path, &content_hash, &link_map_fp);
        let key2 = TransformArtifactKey::compute(&source_path, &content_hash, &link_map_fp);
        prop_assert_eq!(key1.clone(), key2);
        prop_assert_eq!(key1.as_bytes().len(), 32);
    }
}

proptest! {
    #[test]
    fn proptest_artifact_key_distinct_inputs(
        path_a in "[a-zA-Z0-9]{1,20}",
        path_b in "[a-zA-Z0-9]{1,20}",
        ch_bytes in any::<[u8; 32]>(),
        lfp_bytes in any::<[u8; 32]>()
    ) {
        prop_assume!(path_a != path_b);
        let ch = ContentHash::from(ch_bytes);
        let lfp = ContentHash::from(lfp_bytes);
        let key_a = TransformArtifactKey::compute(&path_a, &ch, &lfp);
        let key_b = TransformArtifactKey::compute(&path_b, &ch, &lfp);
        prop_assert_ne!(key_a, key_b);
    }
}

proptest! {
    #[test]
    fn proptest_link_map_fingerprint_order_independence(
        entries in prop::collection::vec(
            ("[a-zA-Z]{1,10}", any::<(String, String, String, String)>()),
            1..20
        )
    ) {
        let keys: Vec<&String> = entries.iter().map(|(k, _)| k).collect();
        let unique_keys: std::collections::HashSet<&&String> = keys.iter().collect();
        prop_assume!(keys.len() == unique_keys.len(), "keys must be unique");

        let mut map_a = HashMap::new();
        let mut map_b = HashMap::new();
        for (key, (id, filename, subcategory, slug)) in &entries {
            let mapping = IdMapping { id: id.clone(), filename: filename.clone(), subcategory: subcategory.clone(), slug: slug.clone() };
            map_a.insert(key.clone(), mapping.clone());
        }
        for (key, (id, filename, subcategory, slug)) in entries.iter().rev() {
            let mapping = IdMapping { id: id.clone(), filename: filename.clone(), subcategory: subcategory.clone(), slug: slug.clone() };
            map_b.insert(key.clone(), mapping);
        }

        let fp_a = compute_link_map_fingerprint(&map_a);
        let fp_b = compute_link_map_fingerprint(&map_b);
        match (fp_a, fp_b) {
            (Ok(a), Ok(b)) => prop_assert_eq!(a, b),
            (Err(_), Err(_)) => (),
            _ => prop_assert!(false, "One succeeded and one failed"),
        }
    }
}

proptest! {
    #[test]
    fn proptest_artifact_serde_roundtrip(
        source_path in "[a-zA-Z0-9/_.-]{1,50}",
        ch_bytes in any::<[u8; 32]>(),
        lfp_bytes in any::<[u8; 32]>(),
        markdown in ".*{0,500}"
    ) {
        let artifact = TransformArtifact {
            source_path,
            content_hash: ContentHash::from(ch_bytes),
            link_map_fingerprint: ContentHash::from(lfp_bytes),
            transformed_markdown: markdown,
        };
        let json = serde_json::to_string(&artifact).expect("serialization should not fail");
        let roundtrip: TransformArtifact = serde_json::from_str(&json).expect("deserialization should not fail");
        prop_assert_eq!(roundtrip, artifact);
    }
}

proptest! {
    #[test]
    fn proptest_composite_hash_always_32_bytes(
        parts in prop::collection::vec(any::<Vec<u8>>(), 1..10)
    ) {
        let refs: Vec<&[u8]> = parts.iter().map(std::vec::Vec::as_slice).collect();
        let hash = composite_hash(&refs);
        prop_assert_eq!(hash.as_bytes().len(), 32);
    }
}

// Kani harnesses
#[cfg(kani)]
mod kani_verification {
    use super::*;

    #[kani::proof]
    fn verify_artifact_key_output_always_32_bytes() {
        let source_path: &[u8] = kani::any();
        kani::assume(source_path.len() <= 256);
        let ch_bytes: [u8; 32] = kani::any();
        let lfp_bytes: [u8; 32] = kani::any();
        let source_str = std::str::from_utf8(source_path);
        if let Ok(sp) = source_str {
            if !sp.is_empty() {
                let content_hash = ContentHash::from(ch_bytes);
                let link_map_fp = ContentHash::from(lfp_bytes);
                let key = TransformArtifactKey::compute(sp, &content_hash, &link_map_fp);
                assert!(key.as_bytes().len() == 32);
            }
        }
    }

    #[kani::proof]
    fn verify_link_map_fingerprint_no_panic() {
        let map: HashMap<String, IdMapping> = HashMap::new();
        let result = compute_link_map_fingerprint(&map);
        assert!(result.is_ok());
    }
}
