// Tests for cmd::index — extracted from index.rs for file-length compliance (<300 lines).
//
// This file is `include!`-ed from index.rs. It follows the same pattern as
// scrape_tests.rs: `#[cfg(test)] mod tests { ... }` with explicit crate paths.

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]
mod tests {
    use crate::cache::ContentHash;
    use crate::cmd::index::file_states_to_stored_hashes;
    use crate::state::FileStateRaw;
    use std::collections::HashMap;

    // -----------------------------------------------------------------------
    // Test Helpers
    // -----------------------------------------------------------------------

    /// Helper: create a FileStateRaw with specific content_hash and config_hash.
    fn file_state_with_hashes(content_hash: [u8; 32], config_hash: [u8; 32]) -> FileStateRaw {
        FileStateRaw {
            content_hash,
            config_hash,
            analysis_hash: [0u8; 32],
            transform_hash: [0u8; 32],
            chunk_hash: [0u8; 32],
            last_processed_secs: 0,
            reserved: [0u8; 32],
        }
    }

    // ===================================================================
    // B1: file_states_to_stored_hashes returns map with identical keys
    // ===================================================================

    #[test]
    fn file_states_to_stored_hashes_returns_map_with_identical_keys_when_input_nonempty() {
        // Given: a HashMap with 3 entries
        let mut file_states = HashMap::new();
        file_states.insert("a.rs".to_string(), FileStateRaw::zeroed());
        file_states.insert("b.md".to_string(), FileStateRaw::zeroed());
        file_states.insert("c.txt".to_string(), FileStateRaw::zeroed());

        // When
        let result = file_states_to_stored_hashes(&file_states);

        // Then: same keys, same count
        assert_eq!(result.len(), 3, "output must have same number of entries");
        assert!(result.contains_key("a.rs"), "must contain key 'a.rs'");
        assert!(result.contains_key("b.md"), "must contain key 'b.md'");
        assert!(result.contains_key("c.txt"), "must contain key 'c.txt'");
    }

    // ===================================================================
    // B2: file_states_to_stored_hashes returns empty map
    // ===================================================================

    #[test]
    fn file_states_to_stored_hashes_returns_empty_map_when_input_empty() {
        // Given
        let file_states = HashMap::<String, FileStateRaw>::new();

        // When
        let result = file_states_to_stored_hashes(&file_states);

        // Then
        assert_eq!(result.len(), 0, "empty input must produce empty output");
        assert_eq!(result, HashMap::new());
    }

    // ===================================================================
    // B3: file_states_to_stored_hashes projects bitwise-identical hashes
    // ===================================================================

    #[test]
    fn file_states_to_stored_hashes_projects_bitwise_identical_content_and_config_hashes() {
        // Given: one entry with known hashes
        let mut file_states = HashMap::new();
        file_states.insert(
            "key".to_string(),
            file_state_with_hashes([0xAA; 32], [0xBB; 32]),
        );

        // When
        let result = file_states_to_stored_hashes(&file_states);

        // Then: content_hash and config_hash are bitwise identical
        let stored = &result["key"];
        assert_eq!(
            stored.content_hash,
            ContentHash::from([0xAA; 32]),
            "content_hash must be bitwise identical"
        );
        assert_eq!(
            stored.config_hash,
            ContentHash::from([0xBB; 32]),
            "config_hash must be bitwise identical"
        );
    }

    // ===================================================================
    // Single entry boundary
    // ===================================================================

    #[test]
    fn file_states_to_stored_hashes_preserves_single_entry_with_zeroed_state() {
        // Given
        let mut file_states = HashMap::new();
        file_states.insert("single.md".to_string(), FileStateRaw::zeroed());

        // When
        let result = file_states_to_stored_hashes(&file_states);

        // Then
        assert_eq!(result.len(), 1);
        assert!(result.contains_key("single.md"));
        assert_eq!(
            result["single.md"].content_hash,
            ContentHash::from([0u8; 32])
        );
        assert_eq!(
            result["single.md"].config_hash,
            ContentHash::from([0u8; 32])
        );
    }

    // ===================================================================
    // Large input boundary (100 entries)
    // ===================================================================

    #[test]
    fn file_states_to_stored_hashes_handles_large_input_of_100_entries() {
        // Given: 100 entries
        let mut file_states = HashMap::new();
        for i in 0..100u8 {
            let mut hash = [0u8; 32];
            hash[0] = i;
            file_states.insert(format!("file_{i}.md"), file_state_with_hashes(hash, hash));
        }

        // When
        let result = file_states_to_stored_hashes(&file_states);

        // Then: all 100 entries preserved
        assert_eq!(result.len(), 100);
        for i in 0..100u8 {
            let key = format!("file_{i}.md");
            assert!(result.contains_key(&key), "must contain key: {key}");
        }
    }

    // ===================================================================
    // Distinct hashes per entry
    // ===================================================================

    #[test]
    fn file_states_to_stored_hashes_preserves_distinct_hashes_per_entry() {
        // Given: 3 entries with distinct hashes
        let mut hash_a = [0u8; 32];
        hash_a[0] = 0x0A;
        let mut hash_b = [0u8; 32];
        hash_b[0] = 0x0B;
        let mut hash_c = [0u8; 32];
        hash_c[0] = 0x0C;
        let mut config_a = [0u8; 32];
        config_a[1] = 0xCA;
        let mut config_b = [0u8; 32];
        config_b[1] = 0xCB;
        let mut config_c = [0u8; 32];
        config_c[1] = 0xCC;

        let mut file_states = HashMap::new();
        file_states.insert("a.md".to_string(), file_state_with_hashes(hash_a, config_a));
        file_states.insert("b.md".to_string(), file_state_with_hashes(hash_b, config_b));
        file_states.insert("c.md".to_string(), file_state_with_hashes(hash_c, config_c));

        // When
        let result = file_states_to_stored_hashes(&file_states);

        // Then: each entry has its own distinct hashes
        assert_eq!(result["a.md"].content_hash, ContentHash::from(hash_a));
        assert_eq!(result["a.md"].config_hash, ContentHash::from(config_a));
        assert_eq!(result["b.md"].content_hash, ContentHash::from(hash_b));
        assert_eq!(result["b.md"].config_hash, ContentHash::from(config_b));
        assert_eq!(result["c.md"].content_hash, ContentHash::from(hash_c));
        assert_eq!(result["c.md"].config_hash, ContentHash::from(config_c));
    }

    // ===================================================================
    // Keys with special characters (Unicode, spaces, dashes)
    // ===================================================================

    #[test]
    fn file_states_to_stored_hashes_output_keys_are_byte_identical_strings() {
        // Given: keys with special characters
        let mut file_states = HashMap::new();
        file_states.insert(
            "path/with spaces/and-dashes.md".to_string(),
            FileStateRaw::zeroed(),
        );
        file_states.insert("src/üñíçödé.md".to_string(), FileStateRaw::zeroed());

        // When
        let result = file_states_to_stored_hashes(&file_states);

        // Then
        assert!(result.contains_key("path/with spaces/and-dashes.md"));
        assert!(result.contains_key("src/üñíçödé.md"));
    }

    // ===================================================================
    // Kani Harness 1: FileStateRaw to StoredHashes field projection (INV-4)
    // ===================================================================

    #[cfg(kani)]
    mod verification {
        use super::*;

        /// Kani proof: For any FileStateRaw, the conversion produces
        /// content_hash and config_hash that are bitwise identical to
        /// the original FileStateRaw fields. (INV-4)
        ///
        /// Bound: Map size up to 10 entries (Kani search depth limit).
        #[kani::proof]
        fn verify_file_states_to_stored_hashes_bitwise_identity() {
            let content_hash: [u8; 32] = kani::any();
            let config_hash: [u8; 32] = kani::any();
            let key: String = kani::any();

            let file_states_raw = file_state_with_hashes(content_hash, config_hash);
            let mut file_states = HashMap::new();
            file_states.insert(key, file_states_raw);

            let result = file_states_to_stored_hashes(&file_states);
            assert_eq!(result.len(), 1);

            let entry = result.values().next().unwrap();
            assert_eq!(entry.content_hash, ContentHash::from(content_hash));
            assert_eq!(entry.config_hash, ContentHash::from(config_hash));
        }
    }
}
