//! Main `build_file_state_changes` function and internal helpers.

use std::collections::HashSet;

use crate::analyze::Analysis;
use crate::chunking_adapter::Chunk;

use super::error::BatchBuildError;
use super::pure::{build_file_state_raw, hash_payload};
use super::types::{FileDiff, FileStateRaw, PipelineOutputs, StateChanges};

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Collect all source paths from all diff categories into a single vector.
fn collect_all_paths<'a>(diff: &'a FileDiff) -> Vec<&'a str> {
    diff.unchanged
        .iter()
        .map(|(f, _)| f.source_path.as_str())
        .chain(diff.changed.iter().map(|f| f.source_path.as_str()))
        .chain(diff.new_files.iter().map(|f| f.source_path.as_str()))
        .chain(diff.deleted.iter().map(String::as_str))
        .collect()
}

/// Check that no source path appears in more than one diff category.
fn check_no_duplicates(diff: &FileDiff) -> Result<(), BatchBuildError> {
    let all_paths = collect_all_paths(diff);
    let unique: HashSet<&str> = all_paths.iter().copied().collect();

    if unique.len() == all_paths.len() {
        return Ok(());
    }

    let duplicate = all_paths
        .iter()
        .find(|&&path| all_paths.iter().filter(|&&p| p == path).count() > 1)
        .copied();

    match duplicate {
        Some(path) => Err(BatchBuildError::DuplicateSourcePath {
            path: path.to_string(),
        }),
        None => Ok(()),
    }
}

/// Validate that all required artifacts exist for a given source path.
fn validate_artifacts<'a>(
    path: &str,
    outputs: &'a PipelineOutputs,
) -> Result<(&'a Analysis, &'a String, &'a Vec<Chunk>, &'a [u8; 32]), BatchBuildError> {
    let analysis = outputs
        .analyses
        .get(path)
        .ok_or_else(|| BatchBuildError::MissingAnalysis {
            path: path.to_string(),
        })?;
    let transform =
        outputs
            .transforms
            .get(path)
            .ok_or_else(|| BatchBuildError::MissingTransform {
                path: path.to_string(),
            })?;
    let chunks = outputs
        .chunks
        .get(path)
        .ok_or_else(|| BatchBuildError::MissingChunk {
            path: path.to_string(),
        })?;
    let content_hash =
        outputs
            .content_hashes
            .get(path)
            .ok_or_else(|| BatchBuildError::MissingContentHash {
                path: path.to_string(),
            })?;
    Ok((analysis, transform, chunks, content_hash))
}

/// Serialize analysis, transform, and chunks for a single file.
fn serialize_file_artifacts(
    analysis: &Analysis,
    transform: &String,
    chunks: &Vec<Chunk>,
    path: &str,
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), BatchBuildError> {
    let analysis_bytes =
        serde_json::to_vec(analysis).map_err(|e| BatchBuildError::AnalysisSerializationFailed {
            path: path.to_string(),
            reason: e.to_string(),
        })?;
    let transform_bytes = serde_json::to_vec(transform).map_err(|e| {
        BatchBuildError::TransformSerializationFailed {
            path: path.to_string(),
            reason: e.to_string(),
        }
    })?;
    let chunk_bytes =
        serde_json::to_vec(chunks).map_err(|e| BatchBuildError::ChunkSerializationFailed {
            path: path.to_string(),
            reason: e.to_string(),
        })?;
    Ok((analysis_bytes, transform_bytes, chunk_bytes))
}

/// Processed artifacts for a single changed/new file.
struct FileArtifacts {
    path: String,
    state: FileStateRaw,
    analysis_blob: ([u8; 32], Vec<u8>),
    transform_blob: ([u8; 32], Vec<u8>),
    chunk_blob: ([u8; 32], Vec<u8>),
}

/// Process a single changed/new file: validate artifacts, serialize, build state.
fn process_single_file(
    file: &crate::discover::DiscoveryFile,
    outputs: &PipelineOutputs,
) -> Result<FileArtifacts, BatchBuildError> {
    let path = &file.source_path;

    let (analysis, transform, chunks, content_hash) = validate_artifacts(path, outputs)?;
    let (a_bytes, t_bytes, c_bytes) = serialize_file_artifacts(analysis, transform, chunks, path)?;

    let a_hash = hash_payload(&a_bytes);
    let t_hash = hash_payload(&t_bytes);
    let c_hash = hash_payload(&c_bytes);

    let state = build_file_state_raw(
        *content_hash,
        outputs.config_hash,
        a_hash,
        t_hash,
        c_hash,
        outputs.now_secs,
    );

    Ok(FileArtifacts {
        path: path.clone(),
        state,
        analysis_blob: (a_hash, a_bytes),
        transform_blob: (t_hash, t_bytes),
        chunk_blob: (c_hash, c_bytes),
    })
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Build a deterministic file-state change batch from diff results and pipeline outputs.
///
/// Changed and new files produce updated rows and payload blobs.
/// Deleted files produce only delete entries. Unchanged files are not rewritten.
///
/// # Errors
///
/// Returns `Err(BatchBuildError)` if preconditions are violated.
pub fn build_file_state_changes(
    diff: &FileDiff,
    outputs: &PipelineOutputs,
) -> Result<StateChanges, BatchBuildError> {
    if diff.unchanged.is_empty()
        && diff.changed.is_empty()
        && diff.new_files.is_empty()
        && diff.deleted.is_empty()
    {
        return Err(BatchBuildError::EmptyDiff);
    }

    check_no_duplicates(diff)?;

    let processed: Vec<FileArtifacts> = diff
        .changed
        .iter()
        .chain(diff.new_files.iter())
        .map(|file| process_single_file(file, outputs))
        .collect::<Result<Vec<_>, _>>()?;

    let updated_files = processed
        .iter()
        .map(|f| (f.path.clone(), f.state))
        .collect();
    let new_analyses = processed.iter().map(|f| f.analysis_blob.clone()).collect();
    let new_transforms = processed.iter().map(|f| f.transform_blob.clone()).collect();
    let new_chunks = processed.iter().map(|f| f.chunk_blob.clone()).collect();

    Ok(StateChanges {
        updated_files,
        deleted_files: diff.deleted.clone(),
        new_analyses,
        new_transforms,
        new_chunks,
        updated_urls: vec![],
        deleted_urls: vec![],
        new_scrapes: vec![],
        new_snapshots: vec![],
        deleted_snapshots: vec![],
    })
}
