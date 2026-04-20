//! B15-B24: Error path tests + additional edge cases.

use super::*;

// B15: Missing analysis for changed file
#[test]
fn build_changes_returns_missing_analysis_when_changed_file_has_no_analysis() {
    let path = "docs/missing_analysis.md";
    let diff = make_diff_with_changed(&[path]);
    let mut outputs = make_pipeline_outputs_for(&[path]);
    outputs.analyses.remove(path);
    let err = build_file_state_changes(&diff, &outputs).expect_err("should fail");
    match err {
        BatchBuildError::MissingAnalysis { path: p } => assert_eq!(p, "docs/missing_analysis.md"),
        other => panic!("expected MissingAnalysis, got {other:?}"),
    }
}

// B16: Missing transform for changed file
#[test]
fn build_changes_returns_missing_transform_when_changed_file_has_no_transform() {
    let path = "docs/no_transform.md";
    let diff = make_diff_with_changed(&[path]);
    let mut outputs = make_pipeline_outputs_for(&[path]);
    outputs.transforms.remove(path);
    match build_file_state_changes(&diff, &outputs) {
        Err(BatchBuildError::MissingTransform { path: p }) => assert_eq!(p, "docs/no_transform.md"),
        other => panic!("expected MissingTransform, got {other:?}"),
    }
}

// B17: Missing chunk for changed file
#[test]
fn build_changes_returns_missing_chunk_when_changed_file_has_no_chunk() {
    let path = "docs/no_chunk.md";
    let diff = make_diff_with_changed(&[path]);
    let mut outputs = make_pipeline_outputs_for(&[path]);
    outputs.chunks.remove(path);
    match build_file_state_changes(&diff, &outputs) {
        Err(BatchBuildError::MissingChunk { path: p }) => assert_eq!(p, "docs/no_chunk.md"),
        other => panic!("expected MissingChunk, got {other:?}"),
    }
}

// B18: Missing content hash for changed file
#[test]
fn build_changes_returns_missing_content_hash_when_changed_file_has_no_hash() {
    let path = "docs/no_hash.md";
    let diff = make_diff_with_changed(&[path]);
    let mut outputs = make_pipeline_outputs_for(&[path]);
    outputs.content_hashes.remove(path);
    match build_file_state_changes(&diff, &outputs) {
        Err(BatchBuildError::MissingContentHash { path: p }) => assert_eq!(p, "docs/no_hash.md"),
        other => panic!("expected MissingContentHash, got {other:?}"),
    }
}

// B19-B22: Missing artifacts for new files
#[test]
fn build_changes_returns_missing_analysis_when_new_file_has_no_analysis() {
    let path = "brand_new.md";
    let diff = make_diff_with_new(&[path]);
    let mut outputs = make_pipeline_outputs_for(&[path]);
    outputs.analyses.remove(path);
    match build_file_state_changes(&diff, &outputs) {
        Err(BatchBuildError::MissingAnalysis { path: p }) => assert_eq!(p, "brand_new.md"),
        other => panic!("expected MissingAnalysis, got {other:?}"),
    }
}

#[test]
fn build_changes_returns_missing_transform_when_new_file_has_no_transform() {
    let path = "brand_new.md";
    let diff = make_diff_with_new(&[path]);
    let mut outputs = make_pipeline_outputs_for(&[path]);
    outputs.transforms.remove(path);
    match build_file_state_changes(&diff, &outputs) {
        Err(BatchBuildError::MissingTransform { path: p }) => assert_eq!(p, "brand_new.md"),
        other => panic!("expected MissingTransform, got {other:?}"),
    }
}

#[test]
fn build_changes_returns_missing_chunk_when_new_file_has_no_chunk() {
    let path = "brand_new.md";
    let diff = make_diff_with_new(&[path]);
    let mut outputs = make_pipeline_outputs_for(&[path]);
    outputs.chunks.remove(path);
    match build_file_state_changes(&diff, &outputs) {
        Err(BatchBuildError::MissingChunk { path: p }) => assert_eq!(p, "brand_new.md"),
        other => panic!("expected MissingChunk, got {other:?}"),
    }
}

#[test]
fn build_changes_returns_missing_content_hash_when_new_file_has_no_hash() {
    let path = "brand_new.md";
    let diff = make_diff_with_new(&[path]);
    let mut outputs = make_pipeline_outputs_for(&[path]);
    outputs.content_hashes.remove(path);
    match build_file_state_changes(&diff, &outputs) {
        Err(BatchBuildError::MissingContentHash { path: p }) => assert_eq!(p, "brand_new.md"),
        other => panic!("expected MissingContentHash, got {other:?}"),
    }
}

// B23: Duplicate source path — all cross-category combinations
#[test]
fn build_changes_returns_duplicate_when_path_in_changed_and_new() {
    let dup = "docs/dup.md";
    let diff = FileDiff {
        unchanged: vec![],
        changed: vec![make_discovery_file(dup)],
        new_files: vec![make_discovery_file(dup)],
        deleted: vec![],
    };
    let outputs = make_pipeline_outputs_for(&[dup]);
    match build_file_state_changes(&diff, &outputs) {
        Err(BatchBuildError::DuplicateSourcePath { path: p }) => assert_eq!(p, "docs/dup.md"),
        other => panic!("expected DuplicateSourcePath, got {other:?}"),
    }
}

#[test]
fn build_changes_returns_duplicate_when_path_in_unchanged_and_changed() {
    let dup = "docs/stale.md";
    let diff = FileDiff {
        unchanged: vec![make_unchanged_entry(dup)],
        changed: vec![make_discovery_file(dup)],
        new_files: vec![],
        deleted: vec![],
    };
    match build_file_state_changes(&diff, &make_pipeline_outputs_for(&[dup])) {
        Err(BatchBuildError::DuplicateSourcePath { path: p }) => assert_eq!(p, "docs/stale.md"),
        other => panic!("expected DuplicateSourcePath, got {other:?}"),
    }
}

#[test]
fn build_changes_returns_duplicate_when_path_in_unchanged_and_new() {
    let dup = "docs/existing.md";
    let diff = FileDiff {
        unchanged: vec![make_unchanged_entry(dup)],
        changed: vec![],
        new_files: vec![make_discovery_file(dup)],
        deleted: vec![],
    };
    match build_file_state_changes(&diff, &make_pipeline_outputs_for(&[dup])) {
        Err(BatchBuildError::DuplicateSourcePath { path: p }) => assert_eq!(p, "docs/existing.md"),
        other => panic!("expected DuplicateSourcePath, got {other:?}"),
    }
}

#[test]
fn build_changes_returns_duplicate_when_path_in_unchanged_and_deleted() {
    let dup = "docs/ghost.md";
    let diff = FileDiff {
        unchanged: vec![make_unchanged_entry(dup)],
        changed: vec![],
        new_files: vec![],
        deleted: vec![dup.to_string()],
    };
    let outputs = PipelineOutputs {
        analyses: HashMap::new(),
        transforms: HashMap::new(),
        chunks: HashMap::new(),
        content_hashes: HashMap::new(),
        config_hash: make_hash(2),
        now_secs: 1_700_000_000,
    };
    match build_file_state_changes(&diff, &outputs) {
        Err(BatchBuildError::DuplicateSourcePath { path: p }) => assert_eq!(p, "docs/ghost.md"),
        other => panic!("expected DuplicateSourcePath, got {other:?}"),
    }
}

#[test]
fn build_changes_returns_duplicate_when_path_in_changed_and_deleted() {
    let dup = "docs/contradiction.md";
    let diff = FileDiff {
        unchanged: vec![],
        changed: vec![make_discovery_file(dup)],
        new_files: vec![],
        deleted: vec![dup.to_string()],
    };
    match build_file_state_changes(&diff, &make_pipeline_outputs_for(&[dup])) {
        Err(BatchBuildError::DuplicateSourcePath { path: p }) => {
            assert_eq!(p, "docs/contradiction.md");
        }
        other => panic!("expected DuplicateSourcePath, got {other:?}"),
    }
}

#[test]
fn build_changes_returns_duplicate_when_path_in_new_and_deleted() {
    let dup = "docs/impossible.md";
    let diff = FileDiff {
        unchanged: vec![],
        changed: vec![],
        new_files: vec![make_discovery_file(dup)],
        deleted: vec![dup.to_string()],
    };
    match build_file_state_changes(&diff, &make_pipeline_outputs_for(&[dup])) {
        Err(BatchBuildError::DuplicateSourcePath { path: p }) => {
            assert_eq!(p, "docs/impossible.md");
        }
        other => panic!("expected DuplicateSourcePath, got {other:?}"),
    }
}

// B24: Empty diff
#[test]
fn build_changes_returns_empty_diff_error_when_all_categories_empty() {
    let diff = FileDiff {
        unchanged: vec![],
        changed: vec![],
        new_files: vec![],
        deleted: vec![],
    };
    let outputs = PipelineOutputs {
        analyses: HashMap::new(),
        transforms: HashMap::new(),
        chunks: HashMap::new(),
        content_hashes: HashMap::new(),
        config_hash: make_hash(2),
        now_secs: 1_700_000_000,
    };
    match build_file_state_changes(&diff, &outputs) {
        Err(BatchBuildError::EmptyDiff) => {}
        other => panic!("expected EmptyDiff, got {other:?}"),
    }
}

// B24b: Only unchanged files returns Ok(empty)
#[test]
fn build_changes_returns_empty_ok_when_diff_has_only_unchanged_files() {
    let diff = FileDiff {
        unchanged: vec![
            make_unchanged_entry("docs/u1.md"),
            make_unchanged_entry("docs/u2.md"),
        ],
        changed: vec![],
        new_files: vec![],
        deleted: vec![],
    };
    let outputs = PipelineOutputs {
        analyses: HashMap::new(),
        transforms: HashMap::new(),
        chunks: HashMap::new(),
        content_hashes: HashMap::new(),
        config_hash: make_hash(2),
        now_secs: 1_700_000_000,
    };
    let changes =
        build_file_state_changes(&diff, &outputs).expect("unchanged-only should return Ok");
    assert!(changes.updated_files.is_empty());
    assert!(changes.deleted_files.is_empty());
    assert!(changes.new_analyses.is_empty());
}

// B15b: Multiple artifacts missing — first detected wins
#[test]
fn build_changes_reports_first_missing_artifact_when_multiple_missing() {
    let path = "docs/multi_missing.md";
    let diff = make_diff_with_changed(&[path]);
    let mut outputs = make_pipeline_outputs_for(&[path]);
    outputs.analyses.remove(path);
    outputs.transforms.remove(path);
    match build_file_state_changes(&diff, &outputs) {
        Err(BatchBuildError::MissingAnalysis { path: p }) => assert_eq!(p, "docs/multi_missing.md"),
        other => panic!("expected MissingAnalysis (first missing wins), got {other:?}"),
    }
}

// B15c: Empty-string source_path
#[test]
fn build_changes_handles_empty_source_path_without_panic() {
    let path = "";
    let diff = make_diff_with_changed(&[path]);
    let outputs = make_pipeline_outputs_for(&[path]);
    let changes =
        build_file_state_changes(&diff, &outputs).expect("empty-string path should succeed");
    assert_eq!(changes.updated_files[0].0, "");
}

// B27b: Empty transform content
#[test]
fn build_changes_handles_empty_transform_content_without_error() {
    let path = "docs/empty_content.md";
    let diff = make_diff_with_changed(&[path]);
    let mut outputs = make_pipeline_outputs_for(&[path]);
    outputs.transforms.insert(path.to_string(), String::new());
    let changes =
        build_file_state_changes(&diff, &outputs).expect("empty transform should succeed");
    assert_eq!(changes.updated_files.len(), 1);
    assert_eq!(changes.new_transforms.len(), 1);
    assert!(!changes.new_transforms[0].1.is_empty());
    assert_ne!(changes.updated_files[0].1.transform_hash, [0u8; 32]);
}
