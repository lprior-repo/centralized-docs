//! Proptest verification for chunking adapter.

use super::*;
use crate::cache::composite_hash;

#[test]
fn proptest_compute_chunker_config_hash_idempotent() {
    use proptest::prelude::*;
    proptest!(|(max_bytes in any::<u64>())| {
        let h1 = compute_chunker_config_hash(max_bytes);
        let h2 = compute_chunker_config_hash(max_bytes);
        assert_eq!(h1, h2, "idempotent for max_bytes={max_bytes}");
    });
}

#[test]
fn proptest_compute_chunker_config_hash_injective() {
    use proptest::prelude::*;
    proptest!(|(a in any::<u64>(), b in any::<u64>())| {
        if a != b {
            assert_ne!(
                compute_chunker_config_hash(a),
                compute_chunker_config_hash(b),
                "different inputs should produce different hashes: {a} vs {b}"
            );
        }
    });
}

#[test]
fn proptest_chunk_cache_key_equivalence_with_composite_hash() {
    use proptest::prelude::*;
    proptest!(|(path in ".*", content in ".*")| {
        let config_hash = compute_chunker_config_hash(1024);
        let key = chunk_cache_key(&path, &content, &config_hash);
        let expected = composite_hash(&[
            path.as_bytes(),
            content.as_bytes(),
            config_hash.as_bytes(),
        ]);
        assert_eq!(key, expected, "chunk_cache_key should match composite_hash for path={path}, content={content}");
    });
}
