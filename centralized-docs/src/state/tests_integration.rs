//! Integration roundtrip and error display tests.

use super::*;
use redb::Database;
use tempfile::TempDir;

fn open_fresh_db() -> (TempDir, Database) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("state.redb");
    let db = Database::create(&db_path).unwrap();
    initialize_tables(&db).unwrap();
    (temp_dir, db)
}

// =======================================================================
// Roundtrip integration tests
// =======================================================================

#[test]
fn file_state_write_read_roundtrip_through_redb() {
    let (_temp_dir, db) = open_fresh_db();
    let original = FileStateRaw {
        content_hash: [0xFE; 32],
        config_hash: [0xDC; 32],
        analysis_hash: [0xBA; 32],
        transform_hash: [0x98; 32],
        chunk_hash: [0x76; 32],
        last_processed_secs: 999,
        reserved: [0x00; 32],
    };
    let key = "concept/test.md";
    let write_tx = db.begin_write().unwrap();
    {
        let mut t = write_tx.open_table(file_state_table()).unwrap();
        t.insert(key, original.to_bytes().as_slice()).unwrap();
    }
    write_tx.commit().unwrap();
    let guard = db
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
fn url_state_write_read_roundtrip_through_redb() {
    let (_temp_dir, db) = open_fresh_db();
    let original = UrlStateRaw {
        content_hash: [0x11; 32],
        url_hash: [0x22; 32],
        last_fetched_secs: 12345,
        status_code: 200,
        reserved: [0x00; 46],
    };
    let key = "https://docs.example.com/api";
    let write_tx = db.begin_write().unwrap();
    {
        let mut t = write_tx.open_table(url_state_table()).unwrap();
        t.insert(key, original.to_bytes().as_slice()).unwrap();
    }
    write_tx.commit().unwrap();
    let guard = db
        .begin_read()
        .unwrap()
        .open_table(url_state_table())
        .unwrap()
        .get(key)
        .unwrap()
        .unwrap();
    assert_eq!(UrlStateRaw::from_bytes(guard.value()).unwrap(), original);
}

#[test]
fn metadata_table_string_read_write_roundtrip() {
    let (_temp_dir, db) = open_fresh_db();
    let write_tx = db.begin_write().unwrap();
    {
        let mut t = write_tx.open_table(metadata_table()).unwrap();
        t.insert("schema_version", "1").unwrap();
        t.insert("created_by", "ctd").unwrap();
    }
    write_tx.commit().unwrap();
    let table = db
        .begin_read()
        .unwrap()
        .open_table(metadata_table())
        .unwrap();
    assert_eq!(table.get("schema_version").unwrap().unwrap().value(), "1");
    assert_eq!(table.get("created_by").unwrap().unwrap().value(), "ctd");
    assert!(table.get("nonexistent").unwrap().is_none());
}

#[test]
fn hash_keyed_output_table_write_read_roundtrip() {
    let (_temp_dir, db) = open_fresh_db();
    let hash_key: [u8; 32] = [0xAB; 32];
    let value: &[u8] = b"test analysis output bytes";
    let write_tx = db.begin_write().unwrap();
    {
        let mut t = write_tx.open_table(analysis_outputs_table()).unwrap();
        t.insert(hash_key.as_slice(), value).unwrap();
    }
    write_tx.commit().unwrap();
    let guard = db
        .begin_read()
        .unwrap()
        .open_table(analysis_outputs_table())
        .unwrap()
        .get(hash_key.as_slice())
        .unwrap()
        .unwrap();
    assert_eq!(guard.value(), value);
}

// =======================================================================
// Error display tests
// =======================================================================

#[test]
fn state_error_variants_display_correctly() {
    use std::path::PathBuf;
    let errors: Vec<StateError> = vec![
        StateError::OpenFailed {
            path: PathBuf::from("/tmp/test.redb"),
            detail: "permission denied".into(),
        },
        StateError::ReadTransactionFailed {
            message: "mvcc conflict".into(),
        },
        StateError::WriteTransactionFailed {
            message: "already locked".into(),
        },
        StateError::PodSizeMismatch {
            table: "file_state",
            expected: 200,
            actual: 199,
        },
        StateError::PodCastFailed {
            type_name: "FileStateRaw",
            message: "alignment".into(),
        },
        StateError::InvalidArchive {
            type_name: "Analysis",
            message: "bad bytes".into(),
        },
        StateError::DeserializationFailed {
            type_name: "Analysis",
            message: "type mismatch".into(),
        },
        StateError::SerializationFailed {
            type_name: "Analysis",
            message: "oom".into(),
        },
        StateError::TableOpenFailed {
            table: "file_state",
            message: "corrupt".into(),
        },
        StateError::KeyNotFound {
            table: "analysis_outputs",
        },
        StateError::StorageError {
            operation: "get",
            message: "io error".into(),
        },
        StateError::CommitFailed {
            message: "disk full".into(),
        },
        StateError::InvalidHashKeyLength { actual: 16 },
        StateError::InvalidSourcePath {
            reason: "leading /".into(),
        },
        StateError::InvalidUrlKey {
            reason: "no scheme".into(),
        },
    ];
    for err in &errors {
        assert!(!format!("{err}").is_empty());
    }
}

#[test]
fn state_error_write_transaction_failed_exact_fields() {
    let err = StateError::WriteTransactionFailed {
        message: "already locked".into(),
    };
    assert!(
        matches!(&err, StateError::WriteTransactionFailed { message } if message == "already locked")
    );
    let display = format!("{err}");
    assert!(display.contains("already locked"), "{display}");
    assert!(display.contains("write transaction"), "{display}");
}

#[test]
fn state_error_table_open_failed_exact_fields() {
    let err = StateError::TableOpenFailed {
        table: "file_state",
        message: "corrupt".into(),
    };
    assert!(
        matches!(&err, StateError::TableOpenFailed { table: "file_state", message } if message == "corrupt")
    );
    let display = format!("{err}");
    assert!(display.contains("file_state"), "{display}");
    assert!(display.contains("corrupt"), "{display}");
}

#[test]
fn state_error_commit_failed_exact_fields() {
    let err = StateError::CommitFailed {
        message: "disk full".into(),
    };
    assert!(matches!(&err, StateError::CommitFailed { message } if message == "disk full"));
    let display = format!("{err}");
    assert!(display.contains("disk full"), "{display}");
    assert!(display.contains("commit"), "{display}");
}
