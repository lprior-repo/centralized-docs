//! B07-B13: Field assertion tests for changed/new files.

use super::*;

// B07: content_hash set from PipelineOutputs
#[test]
fn build_changes_sets_content_hash_from_pipeline_outputs() {
    let specific_content_hash: [u8; 32] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32,
    ];
    let path = "docs/a.md";
    let diff = make_diff_with_changed(&[path]);
    let mut outputs = make_pipeline_outputs_for(&[path]);
    outputs
        .content_hashes
        .insert(path.to_string(), specific_content_hash);
    let changes = build_file_state_changes(&diff, &outputs).expect("should succeed");
    assert_eq!(
        changes.updated_files[0].1.content_hash, specific_content_hash,
        "content_hash must match PipelineOutputs::content_hashes"
    );
}

// B08: config_hash set from PipelineOutputs
#[test]
fn build_changes_sets_config_hash_from_pipeline_outputs() {
    let specific_config_hash: [u8; 32] = [3; 32];
    let path = "docs/a.md";
    let diff = make_diff_with_changed(&[path]);
    let mut outputs = make_pipeline_outputs_for(&[path]);
    outputs.config_hash = specific_config_hash;
    let changes = build_file_state_changes(&diff, &outputs).expect("should succeed");
    assert_eq!(
        changes.updated_files[0].1.config_hash, specific_config_hash,
        "config_hash must match PipelineOutputs::config_hash"
    );
}

// B09: last_processed_secs set from PipelineOutputs
#[test]
fn build_changes_sets_last_processed_secs_from_pipeline_outputs() {
    let path = "docs/a.md";
    let diff = make_diff_with_changed(&[path]);
    let mut outputs = make_pipeline_outputs_for(&[path]);
    outputs.now_secs = 1_700_000_000;
    let changes = build_file_state_changes(&diff, &outputs).expect("should succeed");
    assert_eq!(
        changes.updated_files[0].1.last_processed_secs, 1_700_000_000,
        "last_processed_secs must match PipelineOutputs::now_secs"
    );
}

// B10: analysis_hash matches new_analyses key
#[test]
fn build_changes_analysis_hash_matches_new_analyses_key() {
    let path = "docs/a.md";
    let diff = make_diff_with_changed(&[path]);
    let outputs = make_pipeline_outputs_for(&[path]);
    let changes = build_file_state_changes(&diff, &outputs).expect("should succeed");
    let state = &changes.updated_files[0].1;
    let analysis_keys: Vec<[u8; 32]> = changes.new_analyses.iter().map(|(k, _)| *k).collect();
    assert!(analysis_keys.contains(&state.analysis_hash));
}

// B11: transform_hash matches new_transforms key
#[test]
fn build_changes_transform_hash_matches_new_transforms_key() {
    let path = "docs/a.md";
    let diff = make_diff_with_changed(&[path]);
    let outputs = make_pipeline_outputs_for(&[path]);
    let changes = build_file_state_changes(&diff, &outputs).expect("should succeed");
    let state = &changes.updated_files[0].1;
    let transform_keys: Vec<[u8; 32]> = changes.new_transforms.iter().map(|(k, _)| *k).collect();
    assert!(transform_keys.contains(&state.transform_hash));
}

// B12: chunk_hash matches new_chunks key
#[test]
fn build_changes_chunk_hash_matches_new_chunks_key() {
    let path = "docs/a.md";
    let diff = make_diff_with_changed(&[path]);
    let outputs = make_pipeline_outputs_for(&[path]);
    let changes = build_file_state_changes(&diff, &outputs).expect("should succeed");
    let state = &changes.updated_files[0].1;
    let chunk_keys: Vec<[u8; 32]> = changes.new_chunks.iter().map(|(k, _)| *k).collect();
    assert!(chunk_keys.contains(&state.chunk_hash));
}

// B13: reserved is zeroed
#[test]
fn build_changes_zeroesreserved_field_in_file_state_raw() {
    let path = "docs/a.md";
    let diff = make_diff_with_changed(&[path]);
    let outputs = make_pipeline_outputs_for(&[path]);
    let changes = build_file_state_changes(&diff, &outputs).expect("should succeed");
    assert_eq!(changes.updated_files[0].1.reserved, [0u8; 32]);
}
