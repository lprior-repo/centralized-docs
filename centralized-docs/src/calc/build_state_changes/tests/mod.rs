//! Shared test helpers for `build_state_changes` tests.

use super::*;
use std::collections::HashMap;
use std::sync::Arc;

pub(super) mod error_path_tests;
pub(super) mod proptests;
pub(super) mod success_path_a_tests;
pub(super) mod success_path_b_tests;
pub(super) mod success_path_c_tests;

// -----------------------------------------------------------------------
// Test Helpers
// -----------------------------------------------------------------------

pub(super) fn make_discovery_file(path: &str) -> crate::discover::DiscoveryFile {
    crate::discover::DiscoveryFile {
        source_path: path.to_string(),
        size_bytes: 100,
    }
}

pub(super) fn make_analysis(path: &str) -> crate::analyze::Analysis {
    crate::analyze::Analysis {
        source_path: path.to_string(),
        title: format!("Title for {path}"),
        frontmatter: None,
        headings: vec![],
        links: vec![],
        first_paragraph: String::new(),
        word_count: 10,
        has_code: false,
        has_tables: false,
        category: "concept".to_string(),
        content: Arc::from("test content"),
    }
}

pub(super) fn make_chunk(doc_id: &str, index: usize) -> crate::chunking_adapter::Chunk {
    use contextual_chunker::ChunkType;
    crate::chunking_adapter::Chunk {
        chunk_id: format!("{doc_id}#{index}"),
        doc_id: doc_id.to_string(),
        doc_title: format!("Title for {doc_id}"),
        chunk_index: index,
        content: format!("Chunk {index} content"),
        token_count: 50,
        heading: None,
        heading_path: vec![],
        chunk_type: ChunkType::Prose,
        previous_chunk_id: None,
        next_chunk_id: None,
        related_chunk_ids: vec![],
        summary: format!("Summary for chunk {index}"),
        chunk_level: contextual_chunker::ChunkLevel::Standard,
        parent_chunk_id: None,
        child_chunk_ids: vec![],
        context_prefix: None,
    }
}

pub(super) fn make_hash(value: u8) -> [u8; 32] {
    [value; 32]
}

pub(super) fn make_pipeline_outputs_for(paths: &[&str]) -> PipelineOutputs {
    let mut analyses = HashMap::new();
    let mut transforms = HashMap::new();
    let mut chunks = HashMap::new();
    let mut content_hashes = HashMap::new();

    for &path in paths {
        analyses.insert(path.to_string(), make_analysis(path));
        transforms.insert(path.to_string(), format!("transformed {path}"));
        chunks.insert(path.to_string(), vec![make_chunk(path, 0)]);
        content_hashes.insert(path.to_string(), make_hash(1));
    }

    PipelineOutputs {
        analyses,
        transforms,
        chunks,
        content_hashes,
        config_hash: make_hash(2),
        now_secs: 1_700_000_000,
    }
}

pub(super) fn make_diff_with_changed(paths: &[&str]) -> FileDiff {
    FileDiff {
        unchanged: vec![],
        changed: paths.iter().map(|p| make_discovery_file(p)).collect(),
        new_files: vec![],
        deleted: vec![],
    }
}

pub(super) fn make_diff_with_new(paths: &[&str]) -> FileDiff {
    FileDiff {
        unchanged: vec![],
        changed: vec![],
        new_files: paths.iter().map(|p| make_discovery_file(p)).collect(),
        deleted: vec![],
    }
}

pub(super) fn make_diff_with_deleted(paths: &[&str]) -> FileDiff {
    FileDiff {
        unchanged: vec![],
        changed: vec![],
        new_files: vec![],
        deleted: paths.iter().map(std::string::ToString::to_string).collect(),
    }
}

pub(super) fn make_unchanged_entry(path: &str) -> (crate::discover::DiscoveryFile, FileStateRaw) {
    (
        make_discovery_file(path),
        FileStateRaw {
            content_hash: make_hash(0xAA),
            config_hash: make_hash(0xBB),
            analysis_hash: make_hash(0xCC),
            transform_hash: make_hash(0xDD),
            chunk_hash: make_hash(0xEE),
            last_processed_secs: 1_699_999_999,
            reserved: [0u8; 32],
        },
    )
}
