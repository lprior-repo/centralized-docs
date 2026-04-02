//! Session lifecycle test — Behavior 34.
//!
//! Verifies that the read transaction remains alive and usable after a bulk
//! loader call returns (postcondition Q-05).

use super::common::*;

#[test]
fn read_session_remains_usable_after_bulk_load_call() {
    let (_temp_dir, db) = open_db_with_tables();
    let h1 = hash_from_byte(1);

    insert_analysis(&db, &h1, &sample_analysis("first.md", 10));
    insert_transform(&db, &h1, &sample_transform_result(5, 5));

    let session = create_session(&db);

    // First call: load analyses
    let map1 = session.load_analyses(&[h1]).unwrap();
    assert_eq!(map1.len(), 1);
    // ArchivedString derefs to str; compare via deref
    assert_eq!(
        map1[&h1].archived().unwrap().analyses[0]
            .source_path
            .to_string(),
        "first.md"
    );

    // Second call on SAME session: load transforms
    let map2 = session.load_transforms(&[h1]).unwrap();
    assert_eq!(map2.len(), 1);
    assert_eq!(map2[&h1].archived().unwrap().success_count, 5);
}
