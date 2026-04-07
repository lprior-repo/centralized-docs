//! B01-B06: Changed, new, deleted, unchanged file tests.

use super::*;

// B01: Changed files produce updated rows
#[test]
fn build_changes_produces_updated_rows_for_changed_files() {
    let paths = ["docs/a.md", "docs/b.md"];
    let diff = make_diff_with_changed(&paths);
    let outputs = make_pipeline_outputs_for(&paths);
    let changes = build_file_state_changes(&diff, &outputs).expect("should succeed");
    assert_eq!(changes.updated_files.len(), 2);
    assert_eq!(changes.updated_files[0].0, "docs/a.md");
    assert_eq!(changes.updated_files[1].0, "docs/b.md");
    assert!(changes.deleted_files.is_empty());
}

// B02: New files produce updated rows
#[test]
fn build_changes_produces_updated_rows_for_new_files() {
    let paths = ["docs/new1.md", "docs/new2.md"];
    let diff = make_diff_with_new(&paths);
    let outputs = make_pipeline_outputs_for(&paths);
    let changes = build_file_state_changes(&diff, &outputs).expect("should succeed");
    assert_eq!(changes.updated_files.len(), 2);
    assert_eq!(changes.updated_files[0].0, "docs/new1.md");
    assert_eq!(changes.updated_files[1].0, "docs/new2.md");
}

// B03: Changed files produce payload blobs
#[test]
fn build_changes_produces_payload_blobs_for_changed_files() {
    let paths = ["docs/a.md", "docs/b.md"];
    let diff = make_diff_with_changed(&paths);
    let outputs = make_pipeline_outputs_for(&paths);
    let changes = build_file_state_changes(&diff, &outputs).expect("should succeed");
    assert_eq!(changes.new_analyses.len(), 2);
    assert_eq!(changes.new_transforms.len(), 2);
    assert_eq!(changes.new_chunks.len(), 2);
}

// B04: New files produce payload blobs
#[test]
fn build_changes_produces_payload_blobs_for_new_files() {
    let paths = ["docs/new1.md", "docs/new2.md"];
    let diff = make_diff_with_new(&paths);
    let outputs = make_pipeline_outputs_for(&paths);
    let changes = build_file_state_changes(&diff, &outputs).expect("should succeed");
    assert_eq!(changes.new_analyses.len(), 2);
    assert_eq!(changes.new_transforms.len(), 2);
    assert_eq!(changes.new_chunks.len(), 2);
}

// B05: Deleted files produce only delete entries
#[test]
fn build_changes_produces_only_delete_entries_for_deleted_files() {
    let paths = ["docs/old1.md", "docs/old2.md", "docs/old3.md"];
    let diff = make_diff_with_deleted(&paths);
    let outputs = PipelineOutputs {
        analyses: HashMap::new(),
        transforms: HashMap::new(),
        chunks: HashMap::new(),
        content_hashes: HashMap::new(),
        config_hash: make_hash(2),
        now_secs: 1_700_000_000,
    };
    let changes = build_file_state_changes(&diff, &outputs).expect("should succeed");
    assert_eq!(changes.deleted_files.len(), 3);
    assert!(changes.deleted_files.contains(&"docs/old1.md".to_string()));
    assert!(changes.deleted_files.contains(&"docs/old2.md".to_string()));
    assert!(changes.deleted_files.contains(&"docs/old3.md".to_string()));
    assert!(changes.updated_files.is_empty());
    assert!(changes.new_analyses.is_empty());
    assert!(changes.new_transforms.is_empty());
    assert!(changes.new_chunks.is_empty());
}

// B06: Unchanged files are absent from all outputs
#[test]
fn build_changes_excludes_unchanged_files_from_all_outputs() {
    let unchanged_paths: Vec<&str> = vec![
        "docs/u1.md",
        "docs/u2.md",
        "docs/u3.md",
        "docs/u4.md",
        "docs/u5.md",
    ];
    let changed_path = "docs/changed.md";
    let diff = FileDiff {
        unchanged: unchanged_paths
            .iter()
            .map(|p| make_unchanged_entry(p))
            .collect(),
        changed: vec![make_discovery_file(changed_path)],
        new_files: vec![],
        deleted: vec![],
    };
    let outputs = make_pipeline_outputs_for(&[changed_path]);
    let changes = build_file_state_changes(&diff, &outputs).expect("should succeed");
    assert_eq!(changes.updated_files.len(), 1);
    let updated_paths: Vec<&str> = changes
        .updated_files
        .iter()
        .map(|(p, _)| p.as_str())
        .collect();
    for uc_path in &unchanged_paths {
        assert!(
            !updated_paths.contains(uc_path),
            "unchanged {uc_path} must not appear"
        );
    }
}

// B14: URL state fields are empty
#[test]
fn build_changes_produces_empty_url_state_fields() {
    let path = "docs/a.md";
    let diff = make_diff_with_changed(&[path]);
    let outputs = make_pipeline_outputs_for(&[path]);
    let changes = build_file_state_changes(&diff, &outputs).expect("should succeed");
    assert!(changes.updated_urls.is_empty());
    assert!(changes.deleted_urls.is_empty());
    assert!(changes.new_scrapes.is_empty());
    assert!(changes.new_snapshots.is_empty());
    assert!(changes.deleted_snapshots.is_empty());
}

// B25: Determinism
#[test]
fn build_changes_produces_identical_output_for_identical_inputs() {
    let paths = ["docs/a.md", "docs/b.md", "docs/c.md"];
    let diff1 = make_diff_with_changed(&paths);
    let outputs1 = make_pipeline_outputs_for(&paths);
    let diff2 = diff1.clone();
    let outputs2 = outputs1.clone();
    let changes1 = build_file_state_changes(&diff1, &outputs1).expect("ok");
    let changes2 = build_file_state_changes(&diff2, &outputs2).expect("ok");
    assert_eq!(changes1.updated_files, changes2.updated_files);
    assert_eq!(changes1.new_analyses, changes2.new_analyses);
    assert_eq!(changes1.new_transforms, changes2.new_transforms);
    assert_eq!(changes1.new_chunks, changes2.new_chunks);
}

// B26: Non-zero hashes for all payloads
#[test]
fn build_changes_produces_non_zero_hashes_for_all_payloads() {
    let path = "docs/a.md";
    let diff = make_diff_with_changed(&[path]);
    let outputs = make_pipeline_outputs_for(&[path]);
    let changes = build_file_state_changes(&diff, &outputs).expect("should succeed");
    let state = &changes.updated_files[0].1;
    assert_ne!(state.analysis_hash, [0u8; 32]);
    assert_ne!(state.transform_hash, [0u8; 32]);
    assert_ne!(state.chunk_hash, [0u8; 32]);
}

// Mixed scenario
#[test]
fn build_changes_handles_mixed_diff_categories_correctly() {
    let active_paths = ["docs/c1.md", "docs/c2.md", "docs/c3.md", "docs/n1.md"];
    let diff = FileDiff {
        unchanged: vec![
            make_unchanged_entry("docs/u1.md"),
            make_unchanged_entry("docs/u2.md"),
        ],
        changed: vec![
            make_discovery_file("docs/c1.md"),
            make_discovery_file("docs/c2.md"),
            make_discovery_file("docs/c3.md"),
        ],
        new_files: vec![make_discovery_file("docs/n1.md")],
        deleted: vec!["docs/d1.md".to_string(), "docs/d2.md".to_string()],
    };
    let outputs = make_pipeline_outputs_for(&active_paths);
    let changes = build_file_state_changes(&diff, &outputs).expect("should succeed");
    assert_eq!(
        changes.updated_files.len(),
        4,
        "3 changed + 1 new = 4 updated"
    );
    assert_eq!(changes.deleted_files.len(), 2);
    assert_eq!(changes.new_analyses.len(), 4);
    let updated_paths: Vec<&str> = changes
        .updated_files
        .iter()
        .map(|(p, _)| p.as_str())
        .collect();
    assert!(!updated_paths.contains(&"docs/u1.md"));
    assert!(!updated_paths.contains(&"docs/u2.md"));
}
