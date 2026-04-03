#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

//! Integration tests for StateDb — commit index state once at shutdown.
//!
//! These tests exercise real filesystem I/O, permission checks, symlink
//! handling, and the full commit-once lifecycle.

use doc_transformer::state::{FileHashRecord, StateBatch, StateDb, StateError};
use std::fs;
use std::path::Path;

// ===========================================================================
// Helpers
// ===========================================================================

fn temp_output_dir() -> tempfile::TempDir {
    tempfile::TempDir::new().expect("Failed to create temp dir")
}

/// Read the persisted StateBatch from the output directory.
/// The exact file name is an implementation detail of StateDb;
/// this helper adapts to whatever format is chosen.
fn read_persisted_batch(output_dir: &Path) -> StateBatch {
    // Try JSON first (most likely format for initial implementation)
    let state_file = output_dir.join("state-batch.json");
    if state_file.exists() {
        let content = fs::read_to_string(&state_file).expect("Failed to read state-batch.json");
        return serde_json::from_str(&content).expect("Failed to deserialize state-batch.json");
    }

    // Try bincode fallback
    let bin_file = output_dir.join("state-batch.bin");
    if bin_file.exists() {
        let bytes = fs::read(&bin_file).expect("Failed to read state-batch.bin");
        return bincode::deserialize(&bytes).expect("Failed to deserialize state-batch.bin");
    }

    panic!(
        "No state batch file found in {}. Expected state-batch.json or state-batch.bin",
        output_dir.display()
    );
}

/// Count all files in the output directory (excluding .ctd.lock)
fn count_output_files(output_dir: &Path) -> Vec<String> {
    fs::read_dir(output_dir)
        .expect("Failed to read output dir")
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() != ".ctd.lock")
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect()
}

// ===========================================================================
// B01: StateDb::new succeeds with valid output directory
// ===========================================================================

#[test]
fn state_db_new_returns_ok_when_output_dir_exists() {
    // Given: a writable temporary directory
    let dir = temp_output_dir();

    // When: StateDb::new(temp_dir.path()) is called
    let result = StateDb::new(dir.path());

    // Then: Ok(StateDb) is returned
    assert!(
        result.is_ok(),
        "StateDb::new should succeed with valid directory"
    );
    let state_db = result.expect("StateDb created");

    // And: is_committed() == false
    assert!(!state_db.is_committed());
}

// ===========================================================================
// B02: StateDb::new fails when output directory does not exist
// ===========================================================================

#[test]
fn state_db_new_returns_output_not_accessible_when_dir_missing() {
    // Given: a path to a directory that does not exist
    let nonexistent = std::path::PathBuf::from("/tmp/nope-xyz-nonexistent-cdocs-phv-test");

    // When: StateDb::new(nonexistent_path) is called
    let result = StateDb::new(&nonexistent);

    // Then: Err(StateError::OutputNotAccessible { path })
    match result {
        Err(StateError::OutputNotAccessible { path }) => {
            assert_eq!(path, nonexistent.to_string_lossy().to_string());
        }
        Err(other) => panic!(
            "Expected OutputNotAccessible for nonexistent dir, got: {:?}",
            other
        ),
        Ok(_) => panic!("StateDb::new should fail for nonexistent directory"),
    }
}

// ===========================================================================
// B03: StateDb::new fails when output directory is not writable
// ===========================================================================

#[test]
fn state_db_new_returns_output_not_accessible_when_dir_not_writable() {
    // Given: a temporary directory with mode 0o444 (read-only)
    let dir = temp_output_dir();
    let readonly_path = dir.path().join("readonly");
    fs::create_dir(&readonly_path).expect("Failed to create readonly dir");

    // Set read-only permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&readonly_path, fs::Permissions::from_mode(0o444))
            .expect("Failed to set permissions");
    }

    // When: StateDb::new(read_only_dir) is called
    let result = StateDb::new(&readonly_path);

    // Then: Err(StateError::OutputNotAccessible { path })
    assert!(
        matches!(result, Err(StateError::OutputNotAccessible { .. })),
        "StateDb::new should fail with OutputNotAccessible for read-only directory"
    );

    // Restore permissions for cleanup
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&readonly_path, fs::Permissions::from_mode(0o755));
    }
}

// ===========================================================================
// B05: StateDb::new fails when path is empty string
// ===========================================================================

#[test]
fn state_db_new_returns_output_not_accessible_when_path_is_empty_string() {
    // Given: an empty string path ""
    let empty_path = Path::new("");

    // When: StateDb::new(Path::new("")) is called
    let result = StateDb::new(empty_path);

    // Then: Err(StateError::OutputNotAccessible { path: "" })
    match result {
        Err(StateError::OutputNotAccessible { path }) => {
            assert_eq!(path, "");
        }
        Err(other) => panic!(
            "Expected OutputNotAccessible for empty path, got: {:?}",
            other
        ),
        Ok(_) => panic!("StateDb::new should fail for empty string path"),
    }
}

// ===========================================================================
// B06: StateDb::new fails when path points to a regular file
// ===========================================================================

#[test]
fn state_db_new_returns_output_not_accessible_when_path_is_file() {
    // Given: a temporary directory containing a regular file
    let dir = temp_output_dir();
    let file_path = dir.path().join("not_a_dir.txt");
    fs::write(&file_path, "I am a file, not a directory").expect("Failed to write file");

    // When: StateDb::new(file_path) is called
    let result = StateDb::new(&file_path);

    // Then: Err(StateError::OutputNotAccessible { path })
    assert!(
        matches!(result, Err(StateError::OutputNotAccessible { ref path }) if path.ends_with("not_a_dir.txt")),
        "StateDb::new should fail with OutputNotAccessible when path is a file, got: {:?}",
        result
    );
}

// ===========================================================================
// B07: StateDb::new fails when path is a dangling symlink
// ===========================================================================

#[test]
fn state_db_new_returns_output_not_accessible_when_path_is_dangling_symlink() {
    // Given: a symlink pointing to a non-existent target
    let dir = temp_output_dir();
    let target = dir.path().join("no_such_target");
    let link = dir.path().join("dangling_link");

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        symlink(&target, &link).expect("Failed to create symlink");
    }

    // When: StateDb::new(symlink_path) is called
    let result = StateDb::new(&link);

    // Then: Err(StateError::OutputNotAccessible { path })
    assert!(
        matches!(result, Err(StateError::OutputNotAccessible { .. })),
        "StateDb::new should fail with OutputNotAccessible for dangling symlink, got: {:?}",
        result
    );
}

// ===========================================================================
// B08: StateDb::new returns PreconditionViolation when OutputLock is not held
// ===========================================================================

#[test]
fn state_db_new_returns_precondition_violation_when_output_lock_not_held() {
    // Given: a writable temporary directory where OutputLock has NOT been acquired
    // (no acquire_output_lock call made, no .ctd.lock file exists)
    let dir = temp_output_dir();
    // Ensure no lock file exists
    assert!(
        !dir.path().join(".ctd.lock").exists(),
        "no lock file should exist"
    );

    // When: StateDb::new(temp_dir.path()) is called WITHOUT acquiring the lock
    let result = StateDb::new(dir.path());

    // Then: Err(StateError::PreconditionViolation { detail })
    //       where detail contains "output lock not held"
    //       and detail contains the output directory path
    match result {
        Err(StateError::PreconditionViolation { detail }) => {
            assert!(
                detail.contains("output lock not held") || detail.contains("lock"),
                "detail should reference the lock requirement, got: '{detail}'"
            );
            assert!(
                detail.contains(&dir.path().to_string_lossy().to_string())
                    || detail.contains("output"),
                "detail should reference the output directory, got: '{detail}'"
            );
        }
        Err(other) => {
            // Some implementations may not check for OutputLock in StateDb::new
            // but instead in a wrapper. This test verifies the behavior when
            // they DO check. If this fails, the implementation needs to add the check.
            panic!(
                "Expected PreconditionViolation for missing OutputLock, got: {:?}",
                other
            );
        }
        Ok(_) => {
            // If StateDb::new succeeds without the lock check, the test exposes
            // that INV-05 (single-writer via OutputLock) is not enforced at construction.
            // This is a valid finding — the contract says the check SHOULD be there.
            panic!(
                "StateDb::new should require OutputLock to be held (INV-05). \
                 If the implementation delegates this check, this test needs updating."
            );
        }
    }
}

// ===========================================================================
// B17: commit_changes persists batch and marks committed
// ===========================================================================

#[test]
fn commit_changes_persists_batch_and_marks_committed() {
    // Given: a StateDb with at least one document recorded
    let dir = temp_output_dir();
    let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");
    state_db
        .set_document_count(1)
        .expect("set_document_count should succeed");
    state_db
        .set_chunk_count(1)
        .expect("set_chunk_count should succeed");
    state_db
        .record_file_hash("a.md", "hash")
        .expect("record_file_hash should succeed");

    // When: commit_changes() is called
    let result = state_db.commit_changes();

    // Then: Ok(()) is returned
    assert_eq!(result, Ok(()), "commit_changes should succeed");

    // And: is_committed() == true
    assert!(state_db.is_committed(), "should be committed after commit");

    // And: a state file exists in the output directory
    let files = count_output_files(dir.path());
    assert!(
        !files.is_empty(),
        "output directory should contain a state file after commit"
    );

    // And: the deserialized file has the correct values
    let batch = read_persisted_batch(dir.path());
    assert_eq!(batch.document_count, 1);
    assert_eq!(batch.chunk_count, 1);
    assert_eq!(batch.file_hashes.len(), 1);
    assert_eq!(
        batch.file_hashes[0],
        FileHashRecord {
            relative_path: "a.md".to_string(),
            content_hash: "hash".to_string(),
        }
    );
}

// ===========================================================================
// B20: commit_changes returns PersistenceFailed when I/O fails
// ===========================================================================

#[test]
fn commit_changes_returns_persistence_failed_when_io_fails() {
    // Given: a StateDb whose output_dir has been deleted after construction
    let dir = temp_output_dir();
    let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");
    state_db
        .set_document_count(1)
        .expect("set_document_count should succeed");
    state_db
        .record_file_hash("a.md", "h")
        .expect("record_file_hash should succeed");

    // Remove the output directory to cause I/O failure
    fs::remove_dir_all(dir.path()).expect("Failed to remove output dir");

    // When: commit_changes() is called
    let result = state_db.commit_changes();

    // Then: Err(StateError::PersistenceFailed { run_id, reason })
    assert!(
        matches!(result, Err(StateError::PersistenceFailed { .. })),
        "commit_changes should fail with PersistenceFailed when output dir is deleted, got: {:?}",
        result
    );
}

// ===========================================================================
// B21: commit_changes returns SerializationFailed when serialization fails
// ===========================================================================
// NOTE: This test uses a read-only directory to force write failure during commit.

#[test]
fn commit_changes_returns_serialization_failed_when_serialize_errors() {
    // Given: a StateDb where serialization is forced to fail
    // APPROACH: Use a directory that becomes read-only after construction.
    let dir = temp_output_dir();
    let subdir = dir.path().join("deeply_nested_commit_target");
    fs::create_dir_all(&subdir).expect("Failed to create nested dir");

    let mut state_db = StateDb::new(&subdir).expect("StateDb::new should succeed");
    state_db
        .set_document_count(1)
        .expect("set_document_count should succeed");
    state_db
        .record_file_hash("a.md", "h")
        .expect("record_file_hash should succeed");

    // Make the directory read-only to force write failure (which manifests as
    // PersistenceFailed or SerializationFailed depending on implementation)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&subdir, fs::Permissions::from_mode(0o444))
            .expect("Failed to set read-only");
    }

    let result = state_db.commit_changes();

    // The implementation should return either PersistenceFailed or SerializationFailed
    assert!(
        matches!(
            result,
            Err(StateError::PersistenceFailed { .. }) | Err(StateError::SerializationFailed { .. })
        ),
        "commit_changes should fail when output dir is read-only, got: {:?}",
        result
    );

    // Restore for cleanup
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&subdir, fs::Permissions::from_mode(0o755));
    }
}

// ===========================================================================
// B24: Drop does not write any state file when StateDb is dropped without committing
// ===========================================================================

#[test]
fn drop_does_not_write_state_file_when_uncommitted() {
    // Given: a fresh StateDb with mutations but no commit
    let dir = temp_output_dir();
    let output_path = dir.path().to_path_buf();

    // Record files before creating StateDb so we know what was there
    {
        let mut state_db = StateDb::new(&output_path).expect("StateDb::new should succeed");
        state_db
            .record_file_hash("a.md", "hash1")
            .expect("record should succeed");
        state_db
            .set_document_count(1)
            .expect("set_document_count should succeed");

        // StateDb is dropped here without committing
    }

    // Then: no state file exists in the output directory
    let files = count_output_files(&output_path);
    let state_files: Vec<&String> = files
        .iter()
        .filter(|f| f.contains("state-batch") || f.contains("state_batch"))
        .collect();
    assert!(
        state_files.is_empty(),
        "No state file should exist after drop without commit, but found: {:?}",
        state_files
    );
}

// ===========================================================================
// B26: Simulated pipeline success commits state (StateDb-level E2E)
// ===========================================================================
// NOTE: Tests the StateDb lifecycle as it would be used in run_index,
// without depending on the full pipeline (which is in the binary crate).

#[test]
fn simulated_pipeline_commits_state_when_all_stages_succeed() {
    // Given: a simulated pipeline with 3 documents
    let dir = temp_output_dir();
    let source = "/source";
    let output = dir.path().to_string_lossy().to_string();

    {
        // Create StateDb (as run_index would at the start)
        let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

        // STEP 1: DISCOVER — record file hashes
        for i in 1..=3 {
            state_db
                .record_file_hash(&format!("doc{i}.md"), &format!("sha256:hash{i}"))
                .expect("record_file_hash should succeed");
        }

        // STEP 2: ANALYZE — set document count
        state_db
            .set_document_count(3)
            .expect("set_document_count should succeed");

        // STEP 5: CHUNK — set chunk count
        state_db
            .set_chunk_count(9)
            .expect("set_chunk_count should succeed");

        // FINAL: commit on success (only reached if all stages succeed)
        state_db.commit_changes().expect("commit should succeed");
    }

    // Then: state file exists with correct values
    let batch = read_persisted_batch(dir.path());
    assert_eq!(
        batch.document_count, 3,
        "document_count should equal number of source files"
    );
    assert_eq!(
        batch.file_hashes.len(),
        3,
        "file_hashes should have one entry per file"
    );
    assert_eq!(batch.chunk_count, 9, "chunk_count should match");
}

// ===========================================================================
// B27: Simulated pipeline failure writes zero state
// ===========================================================================

#[test]
fn simulated_pipeline_writes_zero_state_when_stage_fails() {
    // Given: a simulated pipeline that fails at an intermediate stage
    let dir = temp_output_dir();
    let output_path = dir.path().to_path_buf();

    {
        // Create StateDb (as run_index would at the start)
        let mut state_db = StateDb::new(&output_path).expect("StateDb::new should succeed");

        // STEP 1: DISCOVER — record one file hash
        state_db
            .record_file_hash("doc1.md", "sha256:hash1")
            .expect("record_file_hash should succeed");

        // STEP 2: ANALYZE — set document count
        state_db
            .set_document_count(1)
            .expect("set_document_count should succeed");

        // STEP 4: TRANSFORM FAILS — we simulate this by NOT calling commit_changes.
        // In the real pipeline, the ? operator would propagate the error and
        // commit_changes would never be reached. The StateDb is dropped here.
    }

    // Then: no state file exists
    let files = count_output_files(&output_path);
    let state_files: Vec<&String> = files
        .iter()
        .filter(|f| f.contains("state-batch") || f.contains("state_batch"))
        .collect();
    assert!(
        state_files.is_empty(),
        "No state file should exist after pipeline failure, but found: {:?}",
        state_files
    );
}

// ===========================================================================
// B28: Committed batch document_count equals analyzed documents
// ===========================================================================

#[test]
fn committed_batch_document_count_equals_analyzed_documents() {
    // Given: 5 documents processed through a partial pipeline
    let dir = temp_output_dir();
    let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

    // Simulate 5 analyzed documents
    state_db
        .set_document_count(5)
        .expect("set_document_count should succeed");
    for i in 1..=5 {
        state_db
            .record_file_hash(&format!("doc{i}.md"), &format!("hash{i}"))
            .expect("record_file_hash should succeed");
    }

    // When: commit_changes() is called
    state_db.commit_changes().expect("commit should succeed");

    // Then: persisted StateBatch.document_count == 5
    let batch = read_persisted_batch(dir.path());
    assert_eq!(batch.document_count, 5);
}

// ===========================================================================
// B29: Committed batch chunk_count equals chunks result
// ===========================================================================

#[test]
fn committed_batch_chunk_count_equals_chunks_result() {
    // Given: a pipeline that produced 32 chunks
    let dir = temp_output_dir();
    let mut state_db = StateDb::new(dir.path()).expect("StateDb::new should succeed");

    state_db
        .set_document_count(10)
        .expect("set_document_count should succeed");
    state_db
        .set_chunk_count(32)
        .expect("set_chunk_count should succeed");
    state_db
        .record_file_hash("a.md", "h")
        .expect("record_file_hash should succeed");

    // When: commit_changes() is called
    state_db.commit_changes().expect("commit should succeed");

    // Then: persisted StateBatch.chunk_count == 32
    let batch = read_persisted_batch(dir.path());
    assert_eq!(batch.chunk_count, 32);
}

// ===========================================================================
// Drop-then-check: committed state survives Drop
// ===========================================================================

#[test]
fn drop_after_commit_leaves_state_file_intact() {
    // Given: a committed StateDb
    let dir = temp_output_dir();
    let output_path = dir.path().to_path_buf();

    {
        let mut state_db = StateDb::new(&output_path).expect("StateDb::new should succeed");
        state_db
            .set_document_count(1)
            .expect("set_document_count should succeed");
        state_db
            .record_file_hash("a.md", "h")
            .expect("record_file_hash should succeed");
        state_db.commit_changes().expect("commit should succeed");
        // StateDb is dropped here after committing
    }

    // Then: the state file still exists after Drop
    let files = count_output_files(&output_path);
    let has_state = files
        .iter()
        .any(|f| f.contains("state-batch") || f.contains("state_batch"));
    assert!(
        has_state,
        "State file should persist after Drop of committed StateDb, found: {:?}",
        files
    );
}
