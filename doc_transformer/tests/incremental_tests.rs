use doc_transformer::incremental;
use doc_transformer::discover::DiscoveryFile;
use std::collections::HashMap;
use std::fs;
use tempfile::TempDir;
use std::thread;
use std::time::Duration;

#[test]
fn test_full_incremental_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("source");
    let output_dir = temp_dir.path().join("output");

    fs::create_dir(&source_dir).unwrap();
    fs::create_dir(&output_dir).unwrap();

    // Create initial files
    fs::write(source_dir.join("doc1.md"), "# Document 1\nContent").unwrap();
    fs::write(source_dir.join("doc2.md"), "# Document 2\nContent").unwrap();

    let initial_files = vec![
        DiscoveryFile {
            source_path: "doc1.md".to_string(),
            size_bytes: 100,
        },
        DiscoveryFile {
            source_path: "doc2.md".to_string(),
            size_bytes: 100,
        },
    ];

    // Initial index - no previous state
    let changeset = incremental::determine_changes(&initial_files, None, &source_dir).unwrap();
    assert_eq!(changeset.added.len(), 2);
    assert_eq!(changeset.modified.len(), 0);
    assert_eq!(changeset.unchanged.len(), 0);

    // Save initial state
    let mut doc_ids = HashMap::new();
    doc_ids.insert("doc1.md".to_string(), "doc-1".to_string());
    doc_ids.insert("doc2.md".to_string(), "doc-2".to_string());

    let state = incremental::create_state(&initial_files, &source_dir, &doc_ids).unwrap();
    incremental::save_state(&output_dir, &state).unwrap();

    // Verify state file was created
    assert!(output_dir.join(".index_state.json").exists());

    // Wait to ensure mtime changes
    thread::sleep(Duration::from_millis(1100));

    // Modify one file
    fs::write(source_dir.join("doc1.md"), "# Document 1\nModified content").unwrap();

    // Add a new file
    fs::write(source_dir.join("doc3.md"), "# Document 3\nNew content").unwrap();

    let updated_files = vec![
        DiscoveryFile {
            source_path: "doc1.md".to_string(),
            size_bytes: 120,
        },
        DiscoveryFile {
            source_path: "doc2.md".to_string(),
            size_bytes: 100,
        },
        DiscoveryFile {
            source_path: "doc3.md".to_string(),
            size_bytes: 110,
        },
    ];

    // Load state and detect changes
    let loaded_state = incremental::load_state(&output_dir).unwrap();
    assert!(loaded_state.is_some());

    let changeset = incremental::determine_changes(&updated_files, loaded_state.as_ref(), &source_dir).unwrap();

    // Should detect 1 modified, 1 added, 1 unchanged
    assert_eq!(changeset.modified.len(), 1);
    assert!(changeset.modified.contains(&"doc1.md".to_string()));

    assert_eq!(changeset.added.len(), 1);
    assert!(changeset.added.contains(&"doc3.md".to_string()));

    assert_eq!(changeset.unchanged.len(), 1);
    assert!(changeset.unchanged.contains(&"doc2.md".to_string()));

    assert_eq!(changeset.deleted.len(), 0);
}

#[test]
fn test_incremental_detects_deleted_files() {
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("source");
    let output_dir = temp_dir.path().join("output");

    fs::create_dir(&source_dir).unwrap();
    fs::create_dir(&output_dir).unwrap();

    // Create files
    fs::write(source_dir.join("doc1.md"), "Content 1").unwrap();
    fs::write(source_dir.join("doc2.md"), "Content 2").unwrap();
    fs::write(source_dir.join("doc3.md"), "Content 3").unwrap();

    let initial_files = vec![
        DiscoveryFile {
            source_path: "doc1.md".to_string(),
            size_bytes: 100,
        },
        DiscoveryFile {
            source_path: "doc2.md".to_string(),
            size_bytes: 100,
        },
        DiscoveryFile {
            source_path: "doc3.md".to_string(),
            size_bytes: 100,
        },
    ];

    // Save initial state
    let mut doc_ids = HashMap::new();
    doc_ids.insert("doc1.md".to_string(), "doc-1".to_string());
    doc_ids.insert("doc2.md".to_string(), "doc-2".to_string());
    doc_ids.insert("doc3.md".to_string(), "doc-3".to_string());

    let state = incremental::create_state(&initial_files, &source_dir, &doc_ids).unwrap();
    incremental::save_state(&output_dir, &state).unwrap();

    // Delete one file (simulate by not including it in current files)
    let current_files = vec![
        DiscoveryFile {
            source_path: "doc1.md".to_string(),
            size_bytes: 100,
        },
        DiscoveryFile {
            source_path: "doc3.md".to_string(),
            size_bytes: 100,
        },
    ];

    let loaded_state = incremental::load_state(&output_dir).unwrap();
    let changeset = incremental::determine_changes(&current_files, loaded_state.as_ref(), &source_dir).unwrap();

    assert_eq!(changeset.deleted.len(), 1);
    assert!(changeset.deleted.contains(&"doc2.md".to_string()));
}

#[test]
fn test_incremental_handles_file_touch_without_content_change() {
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("source");
    let output_dir = temp_dir.path().join("output");

    fs::create_dir(&source_dir).unwrap();
    fs::create_dir(&output_dir).unwrap();

    // Create file
    let file_path = source_dir.join("doc.md");
    fs::write(&file_path, "Content").unwrap();

    let files = vec![
        DiscoveryFile {
            source_path: "doc.md".to_string(),
            size_bytes: 100,
        },
    ];

    // Save initial state
    let hash = incremental::calculate_content_hash(&source_dir, "doc.md").unwrap();
    let mut doc_ids = HashMap::new();
    doc_ids.insert("doc.md".to_string(), "doc-1".to_string());

    let state = incremental::create_state(&files, &source_dir, &doc_ids).unwrap();
    incremental::save_state(&output_dir, &state).unwrap();

    // Wait to ensure mtime changes
    thread::sleep(Duration::from_millis(1100));

    // Touch the file (update mtime but not content)
    fs::write(&file_path, "Content").unwrap(); // Same content

    let new_hash = incremental::calculate_content_hash(&source_dir, "doc.md").unwrap();
    assert_eq!(hash, new_hash); // Content hash should be same

    let loaded_state = incremental::load_state(&output_dir).unwrap();
    let changeset = incremental::determine_changes(&files, loaded_state.as_ref(), &source_dir).unwrap();

    // Should be unchanged because content hash matches
    assert_eq!(changeset.unchanged.len(), 1);
    assert_eq!(changeset.modified.len(), 0);
}

#[test]
fn test_incremental_force_mode_ignores_state() {
    // This test verifies the intended behavior: when --force is used,
    // the state should be ignored (tested at the CLI level in main.rs)

    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path();

    // Create a state file
    let state = incremental::IncrementalState {
        last_run: chrono::Utc::now(),
        files: HashMap::new(),
    };
    incremental::save_state(output_dir, &state).unwrap();

    // Verify state exists
    let loaded = incremental::load_state(output_dir).unwrap();
    assert!(loaded.is_some());

    // In force mode, main.rs should not call load_state or determine_changes
    // This is a structural test showing the state exists but would be ignored
}

#[test]
fn test_changeset_files_to_process_excludes_unchanged() {
    let mut changeset = incremental::ChangeSet::new();

    changeset.added.push("new1.md".to_string());
    changeset.added.push("new2.md".to_string());
    changeset.modified.push("modified1.md".to_string());
    changeset.unchanged.push("unchanged1.md".to_string());
    changeset.unchanged.push("unchanged2.md".to_string());
    changeset.deleted.push("deleted1.md".to_string());

    let to_process = changeset.files_to_process();

    assert_eq!(to_process.len(), 3); // 2 added + 1 modified
    assert!(to_process.contains(&"new1.md".to_string()));
    assert!(to_process.contains(&"new2.md".to_string()));
    assert!(to_process.contains(&"modified1.md".to_string()));
    assert!(!to_process.contains(&"unchanged1.md".to_string()));
    assert!(!to_process.contains(&"unchanged2.md".to_string()));
    assert!(!to_process.contains(&"deleted1.md".to_string()));

    assert_eq!(changeset.total_to_process(), 3);
}

#[test]
fn test_incremental_state_persistence() {
    let temp_dir = TempDir::new().unwrap();

    let mut files = HashMap::new();
    files.insert(
        "test1.md".to_string(),
        incremental::FileState {
            mtime: 1000,
            content_hash: "hash1".to_string(),
            doc_id: "doc-1".to_string(),
        },
    );
    files.insert(
        "test2.md".to_string(),
        incremental::FileState {
            mtime: 2000,
            content_hash: "hash2".to_string(),
            doc_id: "doc-2".to_string(),
        },
    );

    let state = incremental::IncrementalState {
        last_run: chrono::Utc::now(),
        files,
    };

    // Save and load
    incremental::save_state(temp_dir.path(), &state).unwrap();
    let loaded = incremental::load_state(temp_dir.path()).unwrap();

    assert!(loaded.is_some());
    let loaded_state = loaded.unwrap();

    assert_eq!(loaded_state.files.len(), 2);

    let file1 = loaded_state.files.get("test1.md").unwrap();
    assert_eq!(file1.mtime, 1000);
    assert_eq!(file1.content_hash, "hash1");
    assert_eq!(file1.doc_id, "doc-1");

    let file2 = loaded_state.files.get("test2.md").unwrap();
    assert_eq!(file2.mtime, 2000);
    assert_eq!(file2.content_hash, "hash2");
    assert_eq!(file2.doc_id, "doc-2");
}

#[test]
fn test_content_hash_consistency() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(temp_dir.path().join("file.md"), "test content").unwrap();

    let hash1 = incremental::calculate_content_hash(temp_dir.path(), "file.md").unwrap();
    let hash2 = incremental::calculate_content_hash(temp_dir.path(), "file.md").unwrap();

    // Same file should produce same hash
    assert_eq!(hash1, hash2);

    // Different content should produce different hash
    fs::write(temp_dir.path().join("file.md"), "different content").unwrap();
    let hash3 = incremental::calculate_content_hash(temp_dir.path(), "file.md").unwrap();

    assert_ne!(hash1, hash3);
}

#[test]
fn test_mtime_extraction() {
    let temp_dir = TempDir::new().unwrap();

    fs::write(temp_dir.path().join("file.md"), "content").unwrap();

    let mtime1 = incremental::get_mtime(temp_dir.path(), "file.md").unwrap();

    // mtime should be a reasonable Unix timestamp
    assert!(mtime1 > 1_600_000_000); // After Sept 2020
    assert!(mtime1 < 2_000_000_000); // Before May 2033

    // Wait and modify
    thread::sleep(Duration::from_millis(1100));
    fs::write(temp_dir.path().join("file.md"), "new content").unwrap();

    let mtime2 = incremental::get_mtime(temp_dir.path(), "file.md").unwrap();

    // mtime should have increased
    assert!(mtime2 > mtime1);
}

#[test]
fn test_empty_changeset() {
    let changeset = incremental::ChangeSet::new();

    assert_eq!(changeset.added.len(), 0);
    assert_eq!(changeset.modified.len(), 0);
    assert_eq!(changeset.unchanged.len(), 0);
    assert_eq!(changeset.deleted.len(), 0);
    assert_eq!(changeset.total_to_process(), 0);
    assert_eq!(changeset.files_to_process().len(), 0);
}

#[test]
fn test_incremental_with_no_changes() {
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("source");
    let output_dir = temp_dir.path().join("output");

    fs::create_dir(&source_dir).unwrap();
    fs::create_dir(&output_dir).unwrap();

    // Create file
    fs::write(source_dir.join("doc.md"), "Content").unwrap();

    let files = vec![
        DiscoveryFile {
            source_path: "doc.md".to_string(),
            size_bytes: 100,
        },
    ];

    // Save state
    let mut doc_ids = HashMap::new();
    doc_ids.insert("doc.md".to_string(), "doc-1".to_string());

    let state = incremental::create_state(&files, &source_dir, &doc_ids).unwrap();
    incremental::save_state(&output_dir, &state).unwrap();

    // Check again without any changes
    let loaded_state = incremental::load_state(&output_dir).unwrap();
    let changeset = incremental::determine_changes(&files, loaded_state.as_ref(), &source_dir).unwrap();

    // Nothing should have changed
    assert_eq!(changeset.added.len(), 0);
    assert_eq!(changeset.modified.len(), 0);
    assert_eq!(changeset.unchanged.len(), 1);
    assert_eq!(changeset.deleted.len(), 0);
    assert_eq!(changeset.total_to_process(), 0);
}
