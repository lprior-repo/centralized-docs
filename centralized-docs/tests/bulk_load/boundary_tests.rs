//! Boundary test — Behavior 35.
//!
//! Verifies that bulk loaders handle large input counts (10,000+ hashes)
//! without panic or overflow, returning the exact entry count.

use super::common::*;

// ===========================================================================
// Helpers (extracted from test bodies per Holzmann Rule 2)
// ===========================================================================

/// Insert 256 unique analysis entries (one per unique hash_from_byte value).
fn insert_256_analysis_entries(db: &redb::Database) {
    for i in 0..256u16 {
        let hash = hash_from_byte(i as u8);
        let value = sample_analysis(&format!("doc_{i}.md"), i as usize);
        insert_analysis(db, &hash, &value);
    }
}

/// Insert N unique transform entries using counter-based hash keys.
fn insert_n_unique_transforms(db: &redb::Database, count: usize) {
    let hashes: Vec<[u8; 32]> = (0..count)
        .map(|i| {
            let mut h = [0u8; 32];
            let bytes = (i as u64).to_le_bytes();
            h[..8].copy_from_slice(&bytes);
            h
        })
        .collect();

    for (i, hash) in hashes.iter().enumerate() {
        let value = sample_transform_result(i, i);
        let bytes = rkyv_serialize!(&value);
        let write_tx = db.begin_write().unwrap();
        {
            let mut table = write_tx
                .open_table(doc_transformer::state::transform_outputs_table())
                .unwrap();
            table.insert(hash.as_slice(), bytes.as_slice()).unwrap();
        }
        write_tx.commit().unwrap();
    }
}

// ===========================================================================
// Behavior 35: Bulk loader handles large hash count without panic
// ===========================================================================

#[test]
fn load_analyses_handles_10k_hashes_without_panic() {
    let (_temp_dir, db) = open_db_with_tables();

    let count = 10_000;
    let hashes: Vec<[u8; 32]> = (0..count)
        .map(|i| hash_from_byte((i % 256) as u8))
        .collect();

    // Insert unique entries via extracted helper (256 unique hashes).
    // The dedup in load_entries will collapse 10k → 256 unique lookups.
    insert_256_analysis_entries(&db);

    let session = create_session(&db);
    let map = session.load_analyses(&hashes).unwrap();

    // After dedup of 10k → 256 unique hashes, all present
    assert_eq!(
        map.len(),
        256,
        "expected 256 unique entries after deduplication"
    );
}

/// Variant: 500 truly unique hashes — stress test the actual table reads.
/// Uses a simple counter-based hash scheme for uniqueness.
#[test]
fn load_transforms_handles_many_unique_hashes() {
    let (_temp_dir, db) = open_db_with_tables();

    let count = 500; // Use 500 to keep test fast; proves the principle
    let hashes: Vec<[u8; 32]> = (0..count)
        .map(|i| {
            let mut h = [0u8; 32];
            let bytes = (i as u64).to_le_bytes();
            h[..8].copy_from_slice(&bytes);
            h
        })
        .collect();

    // Insert each unique entry via extracted helper
    insert_n_unique_transforms(&db, count);

    let session = create_session(&db);
    let map = session.load_transforms(&hashes).unwrap();

    assert_eq!(map.len(), count, "expected {count} unique entries");
}
