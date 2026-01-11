use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use crate::discover::DiscoveryFile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementalState {
    /// Timestamp of last full/incremental index
    pub last_run: DateTime<Utc>,
    /// Per-file state: path -> FileState
    pub files: HashMap<String, FileState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileState {
    pub mtime: i64,
    pub content_hash: String,
    pub doc_id: String,
}

#[derive(Debug, Clone)]
pub struct ChangeSet {
    pub added: Vec<String>,
    pub modified: Vec<String>,
    pub unchanged: Vec<String>,
    pub deleted: Vec<String>,
}

impl ChangeSet {
    pub fn new() -> Self {
        Self {
            added: Vec::new(),
            modified: Vec::new(),
            unchanged: Vec::new(),
            deleted: Vec::new(),
        }
    }

    pub fn files_to_process(&self) -> Vec<String> {
        let mut result = Vec::new();
        result.extend(self.added.clone());
        result.extend(self.modified.clone());
        result
    }

    pub fn total_to_process(&self) -> usize {
        self.added.len() + self.modified.len()
    }
}

/// Load incremental state from output directory
pub fn load_state(output_dir: &Path) -> Result<Option<IncrementalState>> {
    let state_file = output_dir.join(".index_state.json");

    if !state_file.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&state_file)?;
    let state: IncrementalState = serde_json::from_str(&content)?;

    Ok(Some(state))
}

/// Save incremental state to output directory
pub fn save_state(output_dir: &Path, state: &IncrementalState) -> Result<()> {
    let state_file = output_dir.join(".index_state.json");
    let content = serde_json::to_string_pretty(&state)?;
    fs::write(&state_file, content)?;

    Ok(())
}

/// Calculate SHA256 hash of file content
pub fn calculate_content_hash(source_dir: &Path, relative_path: &str) -> Result<String> {
    let full_path = source_dir.join(relative_path);
    let content = fs::read(&full_path)?;

    let mut hasher = Sha256::new();
    hasher.update(&content);
    let result = hasher.finalize();

    Ok(format!("{:x}", result))
}

/// Get file modification time as Unix timestamp
pub fn get_mtime(source_dir: &Path, relative_path: &str) -> Result<i64> {
    let full_path = source_dir.join(relative_path);
    let metadata = fs::metadata(&full_path)?;
    let mtime = metadata.modified()?;
    let duration = mtime.duration_since(std::time::UNIX_EPOCH)?;

    Ok(duration.as_secs() as i64)
}

/// Determine which files have changed since last index
pub fn determine_changes(
    source_files: &[DiscoveryFile],
    previous_state: Option<&IncrementalState>,
    source_dir: &Path,
) -> Result<ChangeSet> {
    let mut changeset = ChangeSet::new();

    // If no previous state, all files are new
    let Some(prev_state) = previous_state else {
        changeset.added = source_files.iter()
            .map(|f| f.source_path.clone())
            .collect();
        return Ok(changeset);
    };

    // Build current file set for comparison
    let current_files: HashMap<String, &DiscoveryFile> = source_files
        .iter()
        .map(|f| (f.source_path.clone(), f))
        .collect();

    // Check each current file
    for file in source_files {
        let path = &file.source_path;

        match prev_state.files.get(path) {
            None => {
                // New file not in previous state
                changeset.added.push(path.clone());
            }
            Some(prev_file_state) => {
                // File exists in previous state - check if modified
                let current_mtime = get_mtime(source_dir, path)?;

                if current_mtime > prev_file_state.mtime {
                    // mtime changed - check content hash to be sure
                    let current_hash = calculate_content_hash(source_dir, path)?;

                    if current_hash != prev_file_state.content_hash {
                        // Content actually changed
                        changeset.modified.push(path.clone());
                    } else {
                        // File touched but content unchanged
                        changeset.unchanged.push(path.clone());
                    }
                } else {
                    // mtime same or older - file unchanged
                    changeset.unchanged.push(path.clone());
                }
            }
        }
    }

    // Check for deleted files
    for prev_path in prev_state.files.keys() {
        if !current_files.contains_key(prev_path) {
            changeset.deleted.push(prev_path.clone());
        }
    }

    Ok(changeset)
}

/// Create a new incremental state from processed files
pub fn create_state(
    source_files: &[DiscoveryFile],
    source_dir: &Path,
    doc_ids: &HashMap<String, String>, // path -> doc_id
) -> Result<IncrementalState> {
    let mut files = HashMap::new();

    for file in source_files {
        let path = &file.source_path;
        let mtime = get_mtime(source_dir, path)?;
        let content_hash = calculate_content_hash(source_dir, path)?;
        let doc_id = doc_ids.get(path).cloned().unwrap_or_else(|| "unknown".to_string());

        files.insert(
            path.clone(),
            FileState {
                mtime,
                content_hash,
                doc_id,
            },
        );
    }

    Ok(IncrementalState {
        last_run: Utc::now(),
        files,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_load_state_when_file_missing() {
        let temp_dir = TempDir::new().unwrap();
        let result = load_state(temp_dir.path()).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_save_and_load_state() {
        let temp_dir = TempDir::new().unwrap();

        let mut files = HashMap::new();
        files.insert(
            "test.md".to_string(),
            FileState {
                mtime: 1000,
                content_hash: "abc123".to_string(),
                doc_id: "doc-1".to_string(),
            },
        );

        let state = IncrementalState {
            last_run: Utc::now(),
            files,
        };

        save_state(temp_dir.path(), &state).unwrap();
        let loaded = load_state(temp_dir.path()).unwrap();

        assert!(loaded.is_some());
        let loaded_state = loaded.unwrap();
        assert_eq!(loaded_state.files.len(), 1);
        assert!(loaded_state.files.contains_key("test.md"));
    }

    #[test]
    fn test_calculate_content_hash() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.md");
        fs::write(&file_path, "test content").unwrap();

        let hash1 = calculate_content_hash(temp_dir.path(), "test.md").unwrap();
        let hash2 = calculate_content_hash(temp_dir.path(), "test.md").unwrap();

        // Same content should give same hash
        assert_eq!(hash1, hash2);

        // Change content
        fs::write(&file_path, "different content").unwrap();
        let hash3 = calculate_content_hash(temp_dir.path(), "test.md").unwrap();

        // Different content should give different hash
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_determine_changes_no_previous_state() {
        let temp_dir = TempDir::new().unwrap();

        let files = vec![
            DiscoveryFile {
                source_path: "file1.md".to_string(),
                size_bytes: 100,
            },
            DiscoveryFile {
                source_path: "file2.md".to_string(),
                size_bytes: 200,
            },
        ];

        let changeset = determine_changes(&files, None, temp_dir.path()).unwrap();

        assert_eq!(changeset.added.len(), 2);
        assert_eq!(changeset.modified.len(), 0);
        assert_eq!(changeset.unchanged.len(), 0);
        assert_eq!(changeset.deleted.len(), 0);
    }

    #[test]
    fn test_determine_changes_detects_new_file() {
        let temp_dir = TempDir::new().unwrap();

        // Create existing file
        fs::write(temp_dir.path().join("existing.md"), "content").unwrap();

        let mut prev_files = HashMap::new();
        prev_files.insert(
            "existing.md".to_string(),
            FileState {
                mtime: 1000,
                content_hash: "hash".to_string(),
                doc_id: "doc-1".to_string(),
            },
        );

        let prev_state = IncrementalState {
            last_run: Utc::now(),
            files: prev_files,
        };

        // Now we have existing + new file
        fs::write(temp_dir.path().join("new.md"), "new content").unwrap();

        let current_files = vec![
            DiscoveryFile {
                source_path: "existing.md".to_string(),
                size_bytes: 100,
            },
            DiscoveryFile {
                source_path: "new.md".to_string(),
                size_bytes: 150,
            },
        ];

        let changeset = determine_changes(&current_files, Some(&prev_state), temp_dir.path()).unwrap();

        assert_eq!(changeset.added.len(), 1);
        assert!(changeset.added.contains(&"new.md".to_string()));
    }

    #[test]
    fn test_determine_changes_detects_modified_file() {
        let temp_dir = TempDir::new().unwrap();

        // Create file with initial content
        let file_path = temp_dir.path().join("test.md");
        fs::write(&file_path, "initial content").unwrap();

        let initial_mtime = get_mtime(temp_dir.path(), "test.md").unwrap();
        let initial_hash = calculate_content_hash(temp_dir.path(), "test.md").unwrap();

        let mut prev_files = HashMap::new();
        prev_files.insert(
            "test.md".to_string(),
            FileState {
                mtime: initial_mtime,
                content_hash: initial_hash,
                doc_id: "doc-1".to_string(),
            },
        );

        let prev_state = IncrementalState {
            last_run: Utc::now(),
            files: prev_files,
        };

        // Wait to ensure mtime changes (some file systems have 1-second granularity)
        thread::sleep(Duration::from_millis(1100));

        // Modify the file
        fs::write(&file_path, "modified content").unwrap();

        let current_files = vec![
            DiscoveryFile {
                source_path: "test.md".to_string(),
                size_bytes: 100,
            },
        ];

        let changeset = determine_changes(&current_files, Some(&prev_state), temp_dir.path()).unwrap();

        assert_eq!(changeset.modified.len(), 1);
        assert!(changeset.modified.contains(&"test.md".to_string()));
    }

    #[test]
    fn test_determine_changes_detects_deleted_file() {
        let temp_dir = TempDir::new().unwrap();

        let mut prev_files = HashMap::new();
        prev_files.insert(
            "deleted.md".to_string(),
            FileState {
                mtime: 1000,
                content_hash: "hash".to_string(),
                doc_id: "doc-1".to_string(),
            },
        );

        let prev_state = IncrementalState {
            last_run: Utc::now(),
            files: prev_files,
        };

        // Current files is empty (file was deleted)
        let current_files = vec![];

        let changeset = determine_changes(&current_files, Some(&prev_state), temp_dir.path()).unwrap();

        assert_eq!(changeset.deleted.len(), 1);
        assert!(changeset.deleted.contains(&"deleted.md".to_string()));
    }

    #[test]
    fn test_determine_changes_touched_but_unchanged() {
        let temp_dir = TempDir::new().unwrap();

        // Create file
        let file_path = temp_dir.path().join("test.md");
        fs::write(&file_path, "content").unwrap();

        let initial_hash = calculate_content_hash(temp_dir.path(), "test.md").unwrap();

        // Old mtime (pretend it was indexed before)
        let mut prev_files = HashMap::new();
        prev_files.insert(
            "test.md".to_string(),
            FileState {
                mtime: 1000, // Old timestamp
                content_hash: initial_hash.clone(),
                doc_id: "doc-1".to_string(),
            },
        );

        let prev_state = IncrementalState {
            last_run: Utc::now(),
            files: prev_files,
        };

        // File now has newer mtime but same content hash
        let current_files = vec![
            DiscoveryFile {
                source_path: "test.md".to_string(),
                size_bytes: 100,
            },
        ];

        let changeset = determine_changes(&current_files, Some(&prev_state), temp_dir.path()).unwrap();

        // Should be in unchanged because content hash matches despite mtime difference
        assert_eq!(changeset.modified.len(), 0);
        assert_eq!(changeset.unchanged.len(), 1);
    }

    #[test]
    fn test_changeset_files_to_process() {
        let mut changeset = ChangeSet::new();
        changeset.added.push("new.md".to_string());
        changeset.modified.push("modified.md".to_string());
        changeset.unchanged.push("unchanged.md".to_string());

        let to_process = changeset.files_to_process();

        assert_eq!(to_process.len(), 2);
        assert!(to_process.contains(&"new.md".to_string()));
        assert!(to_process.contains(&"modified.md".to_string()));
        assert!(!to_process.contains(&"unchanged.md".to_string()));
    }

    #[test]
    fn test_create_state() {
        let temp_dir = TempDir::new().unwrap();

        // Create test files
        fs::write(temp_dir.path().join("file1.md"), "content1").unwrap();
        fs::write(temp_dir.path().join("file2.md"), "content2").unwrap();

        let files = vec![
            DiscoveryFile {
                source_path: "file1.md".to_string(),
                size_bytes: 100,
            },
            DiscoveryFile {
                source_path: "file2.md".to_string(),
                size_bytes: 200,
            },
        ];

        let mut doc_ids = HashMap::new();
        doc_ids.insert("file1.md".to_string(), "doc-1".to_string());
        doc_ids.insert("file2.md".to_string(), "doc-2".to_string());

        let state = create_state(&files, temp_dir.path(), &doc_ids).unwrap();

        assert_eq!(state.files.len(), 2);
        assert!(state.files.contains_key("file1.md"));
        assert!(state.files.contains_key("file2.md"));
        assert_eq!(state.files.get("file1.md").unwrap().doc_id, "doc-1");
        assert_eq!(state.files.get("file2.md").unwrap().doc_id, "doc-2");
    }
}
