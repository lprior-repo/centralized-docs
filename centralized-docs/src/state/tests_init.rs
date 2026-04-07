//! Init tests for state database.

use super::*;
use redb::{
    Database, MultimapTableHandle, ReadableMultimapTable, ReadableTableMetadata, TableHandle,
};
use tempfile::TempDir;

fn open_fresh_db() -> (TempDir, Database) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("state.redb");
    let db = Database::create(&db_path).unwrap();
    initialize_tables(&db).unwrap();
    (temp_dir, db)
}

#[test]
fn initialize_tables_creates_all_8_tables_on_fresh_db() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("fresh.redb");
    let db = Database::create(&db_path).unwrap();
    initialize_tables(&db).unwrap();
    let read_tx = db.begin_read().unwrap();
    read_tx.open_table(file_state_table()).unwrap();
    read_tx.open_table(url_state_table()).unwrap();
    read_tx.open_table(analysis_outputs_table()).unwrap();
    read_tx.open_table(transform_outputs_table()).unwrap();
    read_tx.open_table(chunk_outputs_table()).unwrap();
    read_tx.open_table(scrape_outputs_table()).unwrap();
    read_tx.open_table(snapshots_table()).unwrap();
    read_tx.open_table(metadata_table()).unwrap();
    read_tx
        .open_multimap_table(source_path_chunks_table())
        .unwrap();
}

#[test]
fn initialize_tables_is_idempotent_on_second_call() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("idempotent.redb");
    let db = Database::create(&db_path).unwrap();
    initialize_tables(&db).unwrap();
    {
        let write_tx = db.begin_write().unwrap();
        {
            let mut table = write_tx.open_table(file_state_table()).unwrap();
            let state = FileStateRaw::zeroed();
            table
                .insert("test/key.md", state.to_bytes().as_slice())
                .unwrap();
        }
        write_tx.commit().unwrap();
    }
    initialize_tables(&db).unwrap();
    let read_tx = db.begin_read().unwrap();
    let table = read_tx.open_table(file_state_table()).unwrap();
    let guard = table.get("test/key.md").unwrap();
    assert!(
        guard.is_some(),
        "data should persist across idempotent init"
    );
}

#[test]
fn all_8_tables_survive_database_reopen() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("reopen.redb");
    {
        let db = Database::create(&db_path).unwrap();
        initialize_tables(&db).unwrap();
    }
    let db = Database::create(&db_path).unwrap();
    let read_tx = db.begin_read().unwrap();
    for def in [file_state_table(), url_state_table()] {
        let table = read_tx.open_table(def).unwrap();
        assert_eq!(
            table.len().unwrap(),
            0,
            "table '{}' should be empty",
            def.name()
        );
    }
    for def in [
        analysis_outputs_table(),
        transform_outputs_table(),
        chunk_outputs_table(),
        scrape_outputs_table(),
        snapshots_table(),
    ] {
        let table = read_tx.open_table(def).unwrap();
        assert_eq!(
            table.len().unwrap(),
            0,
            "table '{}' should be empty",
            def.name()
        );
    }
    assert_eq!(
        read_tx.open_table(metadata_table()).unwrap().len().unwrap(),
        0
    );
    assert_eq!(
        read_tx
            .open_multimap_table(source_path_chunks_table())
            .unwrap()
            .len()
            .unwrap(),
        0
    );
}

#[test]
fn written_data_survives_across_reopen_cycle() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("persist.redb");
    let key = "concept/general/test.md";
    let original = FileStateRaw {
        content_hash: [0xAB; 32],
        config_hash: [0xCD; 32],
        analysis_hash: [0xEF; 32],
        transform_hash: [0x01; 32],
        chunk_hash: [0x23; 32],
        last_processed_secs: 1_700_000_000,
        reserved: [0x00; 32],
    };
    {
        let db = Database::create(&db_path).unwrap();
        initialize_tables(&db).unwrap();
        let write_tx = db.begin_write().unwrap();
        {
            let mut t = write_tx.open_table(file_state_table()).unwrap();
            t.insert(key, original.to_bytes().as_slice()).unwrap();
        }
        write_tx.commit().unwrap();
    }
    let guard = Database::create(&db_path)
        .unwrap()
        .begin_read()
        .unwrap()
        .open_table(file_state_table())
        .unwrap()
        .get(key)
        .unwrap()
        .unwrap();
    assert_eq!(FileStateRaw::from_bytes(guard.value()).unwrap(), original);
}

#[test]
fn data_survives_ten_sequential_open_write_close_cycles() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("ten_cycles.redb");
    for cycle in 0..10u8 {
        let db = Database::create(&db_path).unwrap();
        initialize_tables(&db).unwrap();
        let key = format!("cycle_{cycle}.md");
        let state = FileStateRaw {
            content_hash: [cycle; 32],
            config_hash: [cycle.saturating_add(1); 32],
            analysis_hash: [cycle.saturating_add(2); 32],
            transform_hash: [cycle.saturating_add(3); 32],
            chunk_hash: [cycle.saturating_add(4); 32],
            last_processed_secs: u64::from(cycle),
            reserved: [0u8; 32],
        };
        let write_tx = db.begin_write().unwrap();
        {
            let mut t = write_tx.open_table(file_state_table()).unwrap();
            t.insert(key.as_str(), state.to_bytes().as_slice()).unwrap();
        }
        write_tx.commit().unwrap();
    }
    let db = Database::create(&db_path).unwrap();
    let table = db
        .begin_read()
        .unwrap()
        .open_table(file_state_table())
        .unwrap();
    for cycle in 0..10u8 {
        let key = format!("cycle_{cycle}.md");
        let restored =
            FileStateRaw::from_bytes(table.get(key.as_str()).unwrap().unwrap().value()).unwrap();
        assert_eq!(restored.content_hash, [cycle; 32], "cycle {cycle}");
    }
}

#[test]
fn new_state_tables_coexist_with_legacy_doc_cache_tables() {
    use redb::TableDefinition;
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("coexist.redb");
    {
        use crate::cache::{CacheConfig, DocCache};
        let config = CacheConfig::new(&db_path);
        let cache = DocCache::open(config).unwrap();
        cache.put_document(b"legacy_key", &"legacy_value").unwrap();
    }
    let db = Database::create(&db_path).unwrap();
    initialize_tables(&db).unwrap();
    let read_tx = db.begin_read().unwrap();
    let legacy_def: TableDefinition<&[u8], &[u8]> = TableDefinition::new("documents");
    assert!(read_tx
        .open_table(legacy_def)
        .unwrap()
        .get(b"legacy_key" as &[u8])
        .unwrap()
        .is_some());
    read_tx.open_table(file_state_table()).unwrap();
    read_tx.open_table(metadata_table()).unwrap();
}

#[test]
fn missing_key_returns_none_from_redb() {
    let (_temp_dir, db) = open_fresh_db();
    let table = db
        .begin_read()
        .unwrap()
        .open_table(analysis_outputs_table())
        .unwrap();
    assert!(table.get([1u8; 32].as_slice()).unwrap().is_none());
}
