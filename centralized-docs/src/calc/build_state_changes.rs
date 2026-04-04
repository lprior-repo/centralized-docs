//! Calc: Build deterministic file-state commit batches from index outputs.
//!
//! Pure functions that consume `FileDiff` and pipeline artifacts to produce
//! a `StateChanges` batch ready for atomic commit.

use crate::analyze::Analysis;
use crate::chunking_adapter::Chunk;
use crate::discover::DiscoveryFile;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

// ---------------------------------------------------------------------------
// Domain Types
// ---------------------------------------------------------------------------

/// Partition of discovered files into unchanged, changed, new, and deleted buckets.
#[derive(Debug, Clone)]
pub struct FileDiff {
    pub unchanged: Vec<(DiscoveryFile, FileStateRaw)>,
    pub changed: Vec<DiscoveryFile>,
    pub new_files: Vec<DiscoveryFile>,
    pub deleted: Vec<String>,
}

/// Fixed 200-byte Pod struct holding content/config/analysis/transform/chunk
/// hashes plus timestamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct FileStateRaw {
    pub content_hash: [u8; 32],
    pub config_hash: [u8; 32],
    pub analysis_hash: [u8; 32],
    pub transform_hash: [u8; 32],
    pub chunk_hash: [u8; 32],
    pub last_processed_secs: u64,
    pub reserved: [u8; 32],
}

// Static assertion: FileStateRaw is exactly 200 bytes.
const _: () = assert!(std::mem::size_of::<FileStateRaw>() == 200);

/// Placeholder type for URL state rows (populated by a separate bead).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UrlStateRaw {
    pub placeholder: [u8; 0],
}

/// Batch of updated file rows, deleted file keys, and new payload blobs.
#[derive(Debug, Clone)]
pub struct StateChanges {
    pub updated_files: Vec<(String, FileStateRaw)>,
    pub deleted_files: Vec<String>,
    pub new_analyses: Vec<([u8; 32], Vec<u8>)>,
    pub new_transforms: Vec<([u8; 32], Vec<u8>)>,
    pub new_chunks: Vec<([u8; 32], Vec<u8>)>,
    // URL state fields — populated by a separate calc bead (cdocs-drj).
    pub updated_urls: Vec<(String, UrlStateRaw)>,
    pub deleted_urls: Vec<String>,
    pub new_scrapes: Vec<([u8; 32], Vec<u8>)>,
    pub new_snapshots: Vec<([u8; 32], Vec<u8>)>,
    pub deleted_snapshots: Vec<[u8; 32]>,
}

/// Input bundle for `build_file_state_changes`.
/// Groups all pipeline outputs needed to derive the commit batch.
#[derive(Debug, Clone)]
pub struct PipelineOutputs {
    /// Analysis results keyed by `source_path`.
    pub analyses: HashMap<String, Analysis>,
    /// Transformed markdown content keyed by `source_path`.
    pub transforms: HashMap<String, String>,
    /// Chunked output keyed by `source_path`.
    pub chunks: HashMap<String, Vec<Chunk>>,
    /// SHA-256 of each file's current bytes, keyed by `source_path`.
    pub content_hashes: HashMap<String, [u8; 32]>,
    /// SHA-256 of the category config used for this run.
    pub config_hash: [u8; 32],
    /// Unix timestamp (seconds) for `last_processed_secs`.
    pub now_secs: u64,
}

// ---------------------------------------------------------------------------
// Error Taxonomy
// ---------------------------------------------------------------------------

/// Exhaustive error taxonomy for `build_file_state_changes`.
#[derive(Debug, Error)]
pub enum BatchBuildError {
    /// A changed or new file has no analysis artifact.
    #[error("missing analysis artifact for source path: {path}")]
    MissingAnalysis { path: String },

    /// A changed or new file has no transform artifact.
    #[error("missing transform artifact for source path: {path}")]
    MissingTransform { path: String },

    /// A changed or new file has no chunk artifact.
    #[error("missing chunk artifact for source path: {path}")]
    MissingChunk { path: String },

    /// A changed or new file has no content hash.
    #[error("missing content hash for source path: {path}")]
    MissingContentHash { path: String },

    /// Serialization of an Analysis value failed.
    #[error("rkyv serialization failed for analysis of {path}: {reason}")]
    AnalysisSerializationFailed { path: String, reason: String },

    /// Serialization of a transform value failed.
    #[error("rkyv serialization failed for transform of {path}: {reason}")]
    TransformSerializationFailed { path: String, reason: String },

    /// Serialization of a chunk value failed.
    #[error("rkyv serialization failed for chunks of {path}: {reason}")]
    ChunkSerializationFailed { path: String, reason: String },

    /// A duplicate `source_path` was detected across diff categories.
    #[error("duplicate source_path in diff: {path} appears in multiple categories")]
    DuplicateSourcePath { path: String },

    /// The input `FileDiff` was empty (no files in any category).
    #[error("file diff is empty: no unchanged, changed, new, or deleted files")]
    EmptyDiff,
}

// ---------------------------------------------------------------------------
// Pure Functions
// ---------------------------------------------------------------------------

/// Compute the SHA-256 hash of arbitrary bytes.
///
/// Returns a non-zero `[u8; 32]` for any non-empty input.
/// Deterministic: same input always produces same output.
#[must_use]
pub fn hash_payload(rkyv_bytes: &[u8]) -> [u8; 32] {
    let digest = Sha256::digest(rkyv_bytes);
    let mut array = [0u8; 32];
    array.copy_from_slice(&digest);
    array
}

/// Serialize an artifact and return both the bytes and their content hash.
///
/// # Errors
///
/// Returns `BatchBuildError::AnalysisSerializationFailed` if serialization fails.
pub fn serialize_and_hash<T: Serialize + ?Sized>(
    value: &T,
    path: &str,
) -> Result<([u8; 32], Vec<u8>), BatchBuildError> {
    let bytes =
        serde_json::to_vec(value).map_err(|e| BatchBuildError::AnalysisSerializationFailed {
            path: path.to_string(),
            reason: e.to_string(),
        })?;
    Ok((hash_payload(&bytes), bytes))
}

/// Construct a `FileStateRaw` from individual hash components.
///
/// All hash fields are set to the provided values, `last_processed_secs` to `now_secs`,
/// and `reserved` is zeroed. Total struct size is exactly 200 bytes.
#[allow(clippy::too_many_arguments)]
#[must_use]
pub fn build_file_state_raw(
    content_hash: [u8; 32],
    config_hash: [u8; 32],
    analysis_hash: [u8; 32],
    transform_hash: [u8; 32],
    chunk_hash: [u8; 32],
    now_secs: u64,
) -> FileStateRaw {
    FileStateRaw {
        content_hash,
        config_hash,
        analysis_hash,
        transform_hash,
        chunk_hash,
        last_processed_secs: now_secs,
        reserved: [0u8; 32],
    }
}

// ---------------------------------------------------------------------------
// Internal Helpers
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

    // Find first path that appears more than once (O(n²) but n is small)
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

/// Serialize analysis, transform, and chunks for a single file,
/// mapping serialization errors to the correct `BatchBuildError` variant.
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
    file: &DiscoveryFile,
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

/// Build a deterministic file-state change batch from diff results and pipeline outputs.
///
/// Changed and new files produce updated rows and payload blobs.
/// Deleted files produce only delete entries. Unchanged files are not rewritten.
///
/// # Errors
///
/// Returns `Err(BatchBuildError)` if preconditions are violated:
/// - `EmptyDiff` when all four categories are empty
/// - `DuplicateSourcePath` when a path appears in multiple categories
/// - `MissingAnalysis` / `MissingTransform` / `MissingChunk` / `MissingContentHash`
///   when pipeline outputs don't cover a changed or new file
/// - `*SerializationFailed` when artifact serialization fails
pub fn build_file_state_changes(
    diff: &FileDiff,
    outputs: &PipelineOutputs,
) -> Result<StateChanges, BatchBuildError> {
    // Check for completely empty diff (all four categories empty)
    if diff.unchanged.is_empty()
        && diff.changed.is_empty()
        && diff.new_files.is_empty()
        && diff.deleted.is_empty()
    {
        return Err(BatchBuildError::EmptyDiff);
    }

    // Validate no duplicate source paths across categories
    check_no_duplicates(diff)?;

    // Process all changed + new files through the pipeline
    let processed: Vec<FileArtifacts> = diff
        .changed
        .iter()
        .chain(diff.new_files.iter())
        .map(|file| process_single_file(file, outputs))
        .collect::<Result<Vec<_>, _>>()?;

    // Build output collections from processed results
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

// ===========================================================================
// Unit Tests + Proptests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::sync::Arc;

    // -----------------------------------------------------------------------
    // Test Helpers
    // -----------------------------------------------------------------------

    fn make_discovery_file(path: &str) -> DiscoveryFile {
        DiscoveryFile {
            source_path: path.to_string(),
            size_bytes: 100,
        }
    }

    fn make_analysis(path: &str) -> Analysis {
        Analysis {
            source_path: path.to_string(),
            title: format!("Title for {}", path),
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

    fn make_chunk(doc_id: &str, index: usize) -> Chunk {
        use contextual_chunker::ChunkType;
        Chunk {
            chunk_id: format!("{}#{}", doc_id, index),
            doc_id: doc_id.to_string(),
            doc_title: format!("Title for {}", doc_id),
            chunk_index: index,
            content: format!("Chunk {} content", index),
            token_count: 50,
            heading: None,
            heading_path: vec![],
            chunk_type: ChunkType::Prose,
            previous_chunk_id: None,
            next_chunk_id: None,
            related_chunk_ids: vec![],
            summary: format!("Summary for chunk {}", index),
            chunk_level: contextual_chunker::ChunkLevel::Standard,
            parent_chunk_id: None,
            child_chunk_ids: vec![],
            context_prefix: None,
        }
    }

    fn make_hash(value: u8) -> [u8; 32] {
        [value; 32]
    }

    fn make_pipeline_outputs_for(paths: &[&str]) -> PipelineOutputs {
        let mut analyses = HashMap::new();
        let mut transforms = HashMap::new();
        let mut chunks = HashMap::new();
        let mut content_hashes = HashMap::new();

        for &path in paths {
            analyses.insert(path.to_string(), make_analysis(path));
            transforms.insert(path.to_string(), format!("transformed {}", path));
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

    fn make_diff_with_changed(paths: &[&str]) -> FileDiff {
        FileDiff {
            unchanged: vec![],
            changed: paths.iter().map(|p| make_discovery_file(p)).collect(),
            new_files: vec![],
            deleted: vec![],
        }
    }

    fn make_diff_with_new(paths: &[&str]) -> FileDiff {
        FileDiff {
            unchanged: vec![],
            changed: vec![],
            new_files: paths.iter().map(|p| make_discovery_file(p)).collect(),
            deleted: vec![],
        }
    }

    fn make_diff_with_deleted(paths: &[&str]) -> FileDiff {
        FileDiff {
            unchanged: vec![],
            changed: vec![],
            new_files: vec![],
            deleted: paths.iter().map(|p| p.to_string()).collect(),
        }
    }

    fn make_unchanged_entry(path: &str) -> (DiscoveryFile, FileStateRaw) {
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

    // ===================================================================
    // B01: Changed files produce updated rows
    // ===================================================================

    #[test]
    fn build_changes_produces_updated_rows_for_changed_files() {
        // Given
        let paths = ["docs/a.md", "docs/b.md"];
        let diff = make_diff_with_changed(&paths);
        let outputs = make_pipeline_outputs_for(&paths);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        let changes = result.expect("should succeed for valid changed files");
        assert_eq!(
            changes.updated_files.len(),
            2,
            "updated_files must have exactly 2 entries"
        );
        assert_eq!(changes.updated_files[0].0, "docs/a.md");
        assert_eq!(changes.updated_files[1].0, "docs/b.md");
        assert!(
            changes.deleted_files.is_empty(),
            "deleted_files must be empty"
        );
    }

    // ===================================================================
    // B02: New files produce updated rows
    // ===================================================================

    #[test]
    fn build_changes_produces_updated_rows_for_new_files() {
        // Given
        let paths = ["docs/new1.md", "docs/new2.md"];
        let diff = make_diff_with_new(&paths);
        let outputs = make_pipeline_outputs_for(&paths);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        let changes = result.expect("should succeed for valid new files");
        assert_eq!(
            changes.updated_files.len(),
            2,
            "updated_files must have exactly 2 entries"
        );
        assert_eq!(changes.updated_files[0].0, "docs/new1.md");
        assert_eq!(changes.updated_files[1].0, "docs/new2.md");
    }

    // ===================================================================
    // B03: Changed files produce payload blobs
    // ===================================================================

    #[test]
    fn build_changes_produces_payload_blobs_for_changed_files() {
        // Given
        let paths = ["docs/a.md", "docs/b.md"];
        let diff = make_diff_with_changed(&paths);
        let outputs = make_pipeline_outputs_for(&paths);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        let changes = result.expect("should succeed for valid changed files");
        assert_eq!(
            changes.new_analyses.len(),
            2,
            "new_analyses must have 2 entries"
        );
        assert_eq!(
            changes.new_transforms.len(),
            2,
            "new_transforms must have 2 entries"
        );
        assert_eq!(
            changes.new_chunks.len(),
            2,
            "new_chunks must have 2 entries"
        );
    }

    // ===================================================================
    // B04: New files produce payload blobs
    // ===================================================================

    #[test]
    fn build_changes_produces_payload_blobs_for_new_files() {
        // Given
        let paths = ["docs/new1.md", "docs/new2.md"];
        let diff = make_diff_with_new(&paths);
        let outputs = make_pipeline_outputs_for(&paths);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        let changes = result.expect("should succeed for valid new files");
        assert_eq!(
            changes.new_analyses.len(),
            2,
            "new_analyses must have 2 entries"
        );
        assert_eq!(
            changes.new_transforms.len(),
            2,
            "new_transforms must have 2 entries"
        );
        assert_eq!(
            changes.new_chunks.len(),
            2,
            "new_chunks must have 2 entries"
        );
    }

    // ===================================================================
    // B05: Deleted files produce only delete entries
    // ===================================================================

    #[test]
    fn build_changes_produces_only_delete_entries_for_deleted_files() {
        // Given
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

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        let changes = result.expect("should succeed for deleted-only diff");
        assert_eq!(
            changes.deleted_files.len(),
            3,
            "deleted_files must have 3 entries"
        );
        assert!(
            changes.deleted_files.contains(&"docs/old1.md".to_string()),
            "must contain docs/old1.md"
        );
        assert!(
            changes.deleted_files.contains(&"docs/old2.md".to_string()),
            "must contain docs/old2.md"
        );
        assert!(
            changes.deleted_files.contains(&"docs/old3.md".to_string()),
            "must contain docs/old3.md"
        );
        assert!(
            changes.updated_files.is_empty(),
            "updated_files must be empty"
        );
        assert!(
            changes.new_analyses.is_empty(),
            "new_analyses must be empty"
        );
        assert!(
            changes.new_transforms.is_empty(),
            "new_transforms must be empty"
        );
        assert!(changes.new_chunks.is_empty(), "new_chunks must be empty");
    }

    // ===================================================================
    // B06: Unchanged files are absent from all outputs
    // ===================================================================

    #[test]
    fn build_changes_excludes_unchanged_files_from_all_outputs() {
        // Given: 5 unchanged + 1 changed
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

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        let changes = result.expect("should succeed");
        assert_eq!(
            changes.updated_files.len(),
            1,
            "only changed file should appear"
        );
        let updated_paths: Vec<&str> = changes
            .updated_files
            .iter()
            .map(|(p, _)| p.as_str())
            .collect();
        for uc_path in &unchanged_paths {
            assert!(
                !updated_paths.contains(uc_path),
                "unchanged file {} must not appear in updated_files",
                uc_path
            );
        }
    }

    // ===================================================================
    // B07: content_hash set from PipelineOutputs
    // ===================================================================

    #[test]
    fn build_changes_sets_content_hash_from_pipeline_outputs() {
        // Given
        let specific_content_hash: [u8; 32] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ];
        let path = "docs/a.md";
        let diff = make_diff_with_changed(&[path]);
        let mut outputs = make_pipeline_outputs_for(&[path]);
        outputs
            .content_hashes
            .insert(path.to_string(), specific_content_hash);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        let changes = result.expect("should succeed");
        let state = &changes.updated_files[0].1;
        assert_eq!(
            state.content_hash, specific_content_hash,
            "content_hash must match PipelineOutputs::content_hashes"
        );
    }

    // ===================================================================
    // B08: config_hash set from PipelineOutputs
    // ===================================================================

    #[test]
    fn build_changes_sets_config_hash_from_pipeline_outputs() {
        // Given
        let specific_config_hash: [u8; 32] = [
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3,
        ];
        let path = "docs/a.md";
        let diff = make_diff_with_changed(&[path]);
        let mut outputs = make_pipeline_outputs_for(&[path]);
        outputs.config_hash = specific_config_hash;

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        let changes = result.expect("should succeed");
        assert_eq!(
            changes.updated_files[0].1.config_hash, specific_config_hash,
            "config_hash must match PipelineOutputs::config_hash"
        );
    }

    // ===================================================================
    // B09: last_processed_secs set from PipelineOutputs
    // ===================================================================

    #[test]
    fn build_changes_sets_last_processed_secs_from_pipeline_outputs() {
        // Given
        let path = "docs/a.md";
        let diff = make_diff_with_changed(&[path]);
        let mut outputs = make_pipeline_outputs_for(&[path]);
        outputs.now_secs = 1_700_000_000;

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        let changes = result.expect("should succeed");
        assert_eq!(
            changes.updated_files[0].1.last_processed_secs, 1_700_000_000,
            "last_processed_secs must match PipelineOutputs::now_secs"
        );
    }

    // ===================================================================
    // B10: analysis_hash matches new_analyses key
    // ===================================================================

    #[test]
    fn build_changes_analysis_hash_matches_new_analyses_key() {
        // Given
        let path = "docs/a.md";
        let diff = make_diff_with_changed(&[path]);
        let outputs = make_pipeline_outputs_for(&[path]);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        let changes = result.expect("should succeed");
        let state = &changes.updated_files[0].1;
        let analysis_keys: Vec<[u8; 32]> = changes.new_analyses.iter().map(|(k, _)| *k).collect();
        assert!(
            analysis_keys.contains(&state.analysis_hash),
            "analysis_hash {:?} must appear as a key in new_analyses",
            state.analysis_hash
        );
    }

    // ===================================================================
    // B11: transform_hash matches new_transforms key
    // ===================================================================

    #[test]
    fn build_changes_transform_hash_matches_new_transforms_key() {
        // Given
        let path = "docs/a.md";
        let diff = make_diff_with_changed(&[path]);
        let outputs = make_pipeline_outputs_for(&[path]);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        let changes = result.expect("should succeed");
        let state = &changes.updated_files[0].1;
        let transform_keys: Vec<[u8; 32]> =
            changes.new_transforms.iter().map(|(k, _)| *k).collect();
        assert!(
            transform_keys.contains(&state.transform_hash),
            "transform_hash {:?} must appear as a key in new_transforms",
            state.transform_hash
        );
    }

    // ===================================================================
    // B12: chunk_hash matches new_chunks key
    // ===================================================================

    #[test]
    fn build_changes_chunk_hash_matches_new_chunks_key() {
        // Given
        let path = "docs/a.md";
        let diff = make_diff_with_changed(&[path]);
        let outputs = make_pipeline_outputs_for(&[path]);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        let changes = result.expect("should succeed");
        let state = &changes.updated_files[0].1;
        let chunk_keys: Vec<[u8; 32]> = changes.new_chunks.iter().map(|(k, _)| *k).collect();
        assert!(
            chunk_keys.contains(&state.chunk_hash),
            "chunk_hash {:?} must appear as a key in new_chunks",
            state.chunk_hash
        );
    }

    // ===================================================================
    // B13: reserved is zeroed
    // ===================================================================

    #[test]
    fn build_changes_zeroesreserved_field_in_file_state_raw() {
        // Given
        let path = "docs/a.md";
        let diff = make_diff_with_changed(&[path]);
        let outputs = make_pipeline_outputs_for(&[path]);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        let changes = result.expect("should succeed");
        assert_eq!(
            changes.updated_files[0].1.reserved, [0u8; 32],
            "reserved must be all zeros"
        );
    }

    // ===================================================================
    // B14: URL state fields are empty
    // ===================================================================

    #[test]
    fn build_changes_produces_empty_url_state_fields() {
        // Given
        let path = "docs/a.md";
        let diff = make_diff_with_changed(&[path]);
        let outputs = make_pipeline_outputs_for(&[path]);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        let changes = result.expect("should succeed");
        assert!(
            changes.updated_urls.is_empty(),
            "updated_urls must be empty"
        );
        assert!(
            changes.deleted_urls.is_empty(),
            "deleted_urls must be empty"
        );
        assert!(changes.new_scrapes.is_empty(), "new_scrapes must be empty");
        assert!(
            changes.new_snapshots.is_empty(),
            "new_snapshots must be empty"
        );
        assert!(
            changes.deleted_snapshots.is_empty(),
            "deleted_snapshots must be empty"
        );
    }

    // ===================================================================
    // B25: Determinism — identical inputs produce identical outputs
    // ===================================================================

    #[test]
    fn build_changes_produces_identical_output_for_identical_inputs() {
        // Given
        let paths = ["docs/a.md", "docs/b.md", "docs/c.md"];
        let diff1 = make_diff_with_changed(&paths);
        let outputs1 = make_pipeline_outputs_for(&paths);

        // Clone for second call
        let diff2 = diff1.clone();
        let outputs2 = outputs1.clone();

        // When
        let result1 = build_file_state_changes(&diff1, &outputs1);
        let result2 = build_file_state_changes(&diff2, &outputs2);

        // Then
        let changes1 = result1.expect("first call should succeed");
        let changes2 = result2.expect("second call should succeed");
        assert_eq!(
            changes1.updated_files, changes2.updated_files,
            "updated_files must be byte-identical across calls"
        );
        assert_eq!(
            changes1.new_analyses, changes2.new_analyses,
            "new_analyses must be identical"
        );
        assert_eq!(
            changes1.new_transforms, changes2.new_transforms,
            "new_transforms must be identical"
        );
        assert_eq!(
            changes1.new_chunks, changes2.new_chunks,
            "new_chunks must be identical"
        );
    }

    // ===================================================================
    // B26: Non-zero hashes for all payloads
    // ===================================================================

    #[test]
    fn build_changes_produces_non_zero_hashes_for_all_payloads() {
        // Given
        let path = "docs/a.md";
        let diff = make_diff_with_changed(&[path]);
        let outputs = make_pipeline_outputs_for(&[path]);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        let changes = result.expect("should succeed");
        let state = &changes.updated_files[0].1;
        assert_ne!(
            state.analysis_hash, [0u8; 32],
            "analysis_hash must be non-zero"
        );
        assert_ne!(
            state.transform_hash, [0u8; 32],
            "transform_hash must be non-zero"
        );
        assert_ne!(state.chunk_hash, [0u8; 32], "chunk_hash must be non-zero");
    }

    // ===================================================================
    // Mixed scenario: changed + new + deleted + unchanged together
    // ===================================================================

    #[test]
    fn build_changes_handles_mixed_diff_categories_correctly() {
        // Given: 2 unchanged, 3 changed, 1 new, 2 deleted
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

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        let changes = result.expect("should succeed for mixed diff");
        assert_eq!(
            changes.updated_files.len(),
            4,
            "3 changed + 1 new = 4 updated"
        );
        assert_eq!(changes.deleted_files.len(), 2, "2 deleted");
        assert_eq!(changes.new_analyses.len(), 4, "4 analysis payloads");
        assert_eq!(changes.new_transforms.len(), 4, "4 transform payloads");
        assert_eq!(changes.new_chunks.len(), 4, "4 chunk payloads");

        // No unchanged source_path in updated_files
        let updated_paths: Vec<&str> = changes
            .updated_files
            .iter()
            .map(|(p, _)| p.as_str())
            .collect();
        assert!(
            !updated_paths.contains(&"docs/u1.md"),
            "unchanged u1 must not appear"
        );
        assert!(
            !updated_paths.contains(&"docs/u2.md"),
            "unchanged u2 must not appear"
        );
    }

    // ===================================================================
    // ERROR PATHS
    // ===================================================================

    // B15: Missing analysis for changed file

    #[test]
    fn build_changes_returns_missing_analysis_when_changed_file_has_no_analysis() {
        // Given
        let path = "docs/missing_analysis.md";
        let diff = make_diff_with_changed(&[path]);
        let mut outputs = make_pipeline_outputs_for(&[path]);
        outputs.analyses.remove(path);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        let err = result.expect_err("missing analysis should return error");
        assert_eq!(
            err.to_string(),
            BatchBuildError::MissingAnalysis {
                path: path.to_string(),
            }
            .to_string(),
            "error must be MissingAnalysis with exact path"
        );
        match err {
            BatchBuildError::MissingAnalysis { path: p } => {
                assert_eq!(p, "docs/missing_analysis.md")
            }
            other => panic!("expected MissingAnalysis, got {:?}", other),
        }
    }

    // B16: Missing transform for changed file

    #[test]
    fn build_changes_returns_missing_transform_when_changed_file_has_no_transform() {
        // Given
        let path = "docs/no_transform.md";
        let diff = make_diff_with_changed(&[path]);
        let mut outputs = make_pipeline_outputs_for(&[path]);
        outputs.transforms.remove(path);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        match result {
            Err(BatchBuildError::MissingTransform { path: p }) => {
                assert_eq!(p, "docs/no_transform.md");
            }
            other => panic!("expected MissingTransform, got {:?}", other),
        }
    }

    // B17: Missing chunk for changed file

    #[test]
    fn build_changes_returns_missing_chunk_when_changed_file_has_no_chunk() {
        // Given
        let path = "docs/no_chunk.md";
        let diff = make_diff_with_changed(&[path]);
        let mut outputs = make_pipeline_outputs_for(&[path]);
        outputs.chunks.remove(path);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        match result {
            Err(BatchBuildError::MissingChunk { path: p }) => {
                assert_eq!(p, "docs/no_chunk.md");
            }
            other => panic!("expected MissingChunk, got {:?}", other),
        }
    }

    // B18: Missing content hash for changed file

    #[test]
    fn build_changes_returns_missing_content_hash_when_changed_file_has_no_hash() {
        // Given
        let path = "docs/no_hash.md";
        let diff = make_diff_with_changed(&[path]);
        let mut outputs = make_pipeline_outputs_for(&[path]);
        outputs.content_hashes.remove(path);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        match result {
            Err(BatchBuildError::MissingContentHash { path: p }) => {
                assert_eq!(p, "docs/no_hash.md");
            }
            other => panic!("expected MissingContentHash, got {:?}", other),
        }
    }

    // B19: Missing analysis for new file

    #[test]
    fn build_changes_returns_missing_analysis_when_new_file_has_no_analysis() {
        // Given
        let path = "brand_new.md";
        let diff = make_diff_with_new(&[path]);
        let mut outputs = make_pipeline_outputs_for(&[path]);
        outputs.analyses.remove(path);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        match result {
            Err(BatchBuildError::MissingAnalysis { path: p }) => {
                assert_eq!(p, "brand_new.md");
            }
            other => panic!("expected MissingAnalysis, got {:?}", other),
        }
    }

    // B20: Missing transform for new file

    #[test]
    fn build_changes_returns_missing_transform_when_new_file_has_no_transform() {
        // Given
        let path = "brand_new.md";
        let diff = make_diff_with_new(&[path]);
        let mut outputs = make_pipeline_outputs_for(&[path]);
        outputs.transforms.remove(path);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        match result {
            Err(BatchBuildError::MissingTransform { path: p }) => {
                assert_eq!(p, "brand_new.md");
            }
            other => panic!("expected MissingTransform, got {:?}", other),
        }
    }

    // B21: Missing chunk for new file

    #[test]
    fn build_changes_returns_missing_chunk_when_new_file_has_no_chunk() {
        // Given
        let path = "brand_new.md";
        let diff = make_diff_with_new(&[path]);
        let mut outputs = make_pipeline_outputs_for(&[path]);
        outputs.chunks.remove(path);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        match result {
            Err(BatchBuildError::MissingChunk { path: p }) => {
                assert_eq!(p, "brand_new.md");
            }
            other => panic!("expected MissingChunk, got {:?}", other),
        }
    }

    // B22: Missing content hash for new file

    #[test]
    fn build_changes_returns_missing_content_hash_when_new_file_has_no_hash() {
        // Given
        let path = "brand_new.md";
        let diff = make_diff_with_new(&[path]);
        let mut outputs = make_pipeline_outputs_for(&[path]);
        outputs.content_hashes.remove(path);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        match result {
            Err(BatchBuildError::MissingContentHash { path: p }) => {
                assert_eq!(p, "brand_new.md");
            }
            other => panic!("expected MissingContentHash, got {:?}", other),
        }
    }

    // B23: Duplicate source path — changed and new_files

    #[test]
    fn build_changes_returns_duplicate_when_path_in_changed_and_new() {
        // Given
        let dup = "docs/dup.md";
        let diff = FileDiff {
            unchanged: vec![],
            changed: vec![make_discovery_file(dup)],
            new_files: vec![make_discovery_file(dup)],
            deleted: vec![],
        };
        let outputs = make_pipeline_outputs_for(&[dup]);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        match result {
            Err(BatchBuildError::DuplicateSourcePath { path: p }) => {
                assert_eq!(p, "docs/dup.md");
            }
            other => panic!("expected DuplicateSourcePath, got {:?}", other),
        }
    }

    // B23b: Duplicate — unchanged and changed

    #[test]
    fn build_changes_returns_duplicate_when_path_in_unchanged_and_changed() {
        // Given
        let dup = "docs/stale.md";
        let diff = FileDiff {
            unchanged: vec![make_unchanged_entry(dup)],
            changed: vec![make_discovery_file(dup)],
            new_files: vec![],
            deleted: vec![],
        };
        let outputs = make_pipeline_outputs_for(&[dup]);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        match result {
            Err(BatchBuildError::DuplicateSourcePath { path: p }) => {
                assert_eq!(p, "docs/stale.md");
            }
            other => panic!("expected DuplicateSourcePath, got {:?}", other),
        }
    }

    // B23c: Duplicate — unchanged and new_files

    #[test]
    fn build_changes_returns_duplicate_when_path_in_unchanged_and_new() {
        // Given
        let dup = "docs/existing.md";
        let diff = FileDiff {
            unchanged: vec![make_unchanged_entry(dup)],
            changed: vec![],
            new_files: vec![make_discovery_file(dup)],
            deleted: vec![],
        };
        let outputs = make_pipeline_outputs_for(&[dup]);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        match result {
            Err(BatchBuildError::DuplicateSourcePath { path: p }) => {
                assert_eq!(p, "docs/existing.md");
            }
            other => panic!("expected DuplicateSourcePath, got {:?}", other),
        }
    }

    // B23d: Duplicate — unchanged and deleted

    #[test]
    fn build_changes_returns_duplicate_when_path_in_unchanged_and_deleted() {
        // Given
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

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        match result {
            Err(BatchBuildError::DuplicateSourcePath { path: p }) => {
                assert_eq!(p, "docs/ghost.md");
            }
            other => panic!("expected DuplicateSourcePath, got {:?}", other),
        }
    }

    // B23e: Duplicate — changed and deleted

    #[test]
    fn build_changes_returns_duplicate_when_path_in_changed_and_deleted() {
        // Given
        let dup = "docs/contradiction.md";
        let diff = FileDiff {
            unchanged: vec![],
            changed: vec![make_discovery_file(dup)],
            new_files: vec![],
            deleted: vec![dup.to_string()],
        };
        let outputs = make_pipeline_outputs_for(&[dup]);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        match result {
            Err(BatchBuildError::DuplicateSourcePath { path: p }) => {
                assert_eq!(p, "docs/contradiction.md");
            }
            other => panic!("expected DuplicateSourcePath, got {:?}", other),
        }
    }

    // B23f: Duplicate — new_files and deleted

    #[test]
    fn build_changes_returns_duplicate_when_path_in_new_and_deleted() {
        // Given
        let dup = "docs/impossible.md";
        let diff = FileDiff {
            unchanged: vec![],
            changed: vec![],
            new_files: vec![make_discovery_file(dup)],
            deleted: vec![dup.to_string()],
        };
        let outputs = make_pipeline_outputs_for(&[dup]);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        match result {
            Err(BatchBuildError::DuplicateSourcePath { path: p }) => {
                assert_eq!(p, "docs/impossible.md");
            }
            other => panic!("expected DuplicateSourcePath, got {:?}", other),
        }
    }

    // B24: Empty diff — all categories empty returns Err(EmptyDiff)

    #[test]
    fn build_changes_returns_empty_diff_error_when_all_categories_empty() {
        // Given
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

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        match result {
            Err(BatchBuildError::EmptyDiff) => {}
            other => panic!("expected EmptyDiff, got {:?}", other),
        }
    }

    // B24b: Diff with only unchanged files returns Ok(empty StateChanges)

    #[test]
    fn build_changes_returns_empty_ok_when_diff_has_only_unchanged_files() {
        // Given
        let diff = FileDiff {
            unchanged: vec![
                make_unchanged_entry("docs/u1.md"),
                make_unchanged_entry("docs/u2.md"),
                make_unchanged_entry("docs/u3.md"),
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

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        let changes = result.expect("unchanged-only diff should return Ok");
        assert!(
            changes.updated_files.is_empty(),
            "updated_files must be empty"
        );
        assert!(
            changes.deleted_files.is_empty(),
            "deleted_files must be empty"
        );
        assert!(
            changes.new_analyses.is_empty(),
            "new_analyses must be empty"
        );
        assert!(
            changes.new_transforms.is_empty(),
            "new_transforms must be empty"
        );
        assert!(changes.new_chunks.is_empty(), "new_chunks must be empty");
        assert!(
            changes.updated_urls.is_empty(),
            "updated_urls must be empty"
        );
        assert!(
            changes.deleted_urls.is_empty(),
            "deleted_urls must be empty"
        );
    }

    // B27: Analysis serialization failure
    // NOTE: The implementation uses serde_json which cannot fail for standard Serialize types.
    // This error variant exists for the planned rkyv migration. Once rkyv serialization is
    // integrated, this test can be re-enabled with a mechanism to inject serialization failures.
    // For now, verify the error variant is constructible and produces the correct display string.
    #[test]
    #[ignore = "requires rkyv serialization to trigger failure; serde_json cannot fail for Analysis"]
    fn build_changes_returns_analysis_serialization_failed_on_rkyv_error() {
        let path = "docs/fail.md";
        let diff = make_diff_with_changed(&[path]);
        let outputs = make_pipeline_outputs_for(&[path]);

        let result = build_file_state_changes(&diff, &outputs);

        match result {
            Err(BatchBuildError::AnalysisSerializationFailed { path: p, reason }) => {
                assert_eq!(p, "docs/fail.md");
                assert!(!reason.is_empty(), "reason must be non-empty");
            }
            other => panic!("expected AnalysisSerializationFailed, got {:?}", other),
        }
    }

    // B28: Transform serialization failure
    // NOTE: serde_json cannot fail for String serialization. This test is preserved for the
    // planned rkyv migration where serialization can fail for non-byte-compatible types.
    #[test]
    #[ignore = "requires rkyv serialization to trigger failure; serde_json cannot fail for String"]
    fn build_changes_returns_transform_serialization_failed_on_rkyv_error() {
        let path = "docs/fail_transform.md";
        let diff = make_diff_with_changed(&[path]);
        let outputs = make_pipeline_outputs_for(&[path]);

        let result = build_file_state_changes(&diff, &outputs);

        match result {
            Err(BatchBuildError::TransformSerializationFailed { path: p, reason }) => {
                assert_eq!(p, "docs/fail_transform.md");
                assert!(!reason.is_empty(), "reason must be non-empty");
            }
            other => panic!("expected TransformSerializationFailed, got {:?}", other),
        }
    }

    // B29: Chunk serialization failure
    // NOTE: serde_json cannot fail for Vec<Chunk> serialization. This test is preserved for the
    // planned rkyv migration where serialization can fail for non-byte-compatible types.
    #[test]
    #[ignore = "requires rkyv serialization to trigger failure; serde_json cannot fail for Vec<Chunk>"]
    fn build_changes_returns_chunk_serialization_failed_on_rkyv_error() {
        let path = "docs/fail_chunks.md";
        let diff = make_diff_with_changed(&[path]);
        let outputs = make_pipeline_outputs_for(&[path]);

        let result = build_file_state_changes(&diff, &outputs);

        match result {
            Err(BatchBuildError::ChunkSerializationFailed { path: p, reason }) => {
                assert_eq!(p, "docs/fail_chunks.md");
                assert!(!reason.is_empty(), "reason must be non-empty");
            }
            other => panic!("expected ChunkSerializationFailed, got {:?}", other),
        }
    }

    // B15b: Multiple artifacts missing — first detected wins

    #[test]
    fn build_changes_reports_first_missing_artifact_when_multiple_missing() {
        // Given: file missing both analysis AND transform
        let path = "docs/multi_missing.md";
        let diff = make_diff_with_changed(&[path]);
        let mut outputs = make_pipeline_outputs_for(&[path]);
        outputs.analyses.remove(path);
        outputs.transforms.remove(path);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then: first check (analysis) should win
        match result {
            Err(BatchBuildError::MissingAnalysis { path: p }) => {
                assert_eq!(p, "docs/multi_missing.md");
            }
            other => panic!(
                "expected MissingAnalysis (first missing wins), got {:?}",
                other
            ),
        }
    }

    // B15c: Empty-string source_path handled without panic

    #[test]
    fn build_changes_handles_empty_source_path_without_panic() {
        // Given
        let path = "";
        let diff = make_diff_with_changed(&[path]);
        let outputs = make_pipeline_outputs_for(&[path]);

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then: should succeed with empty-string as valid key
        let changes = result.expect("empty-string path should succeed");
        assert_eq!(changes.updated_files[0].0, "");
    }

    // B27b: Empty transform content produces valid output

    #[test]
    fn build_changes_handles_empty_transform_content_without_error() {
        // Given
        let path = "docs/empty_content.md";
        let diff = make_diff_with_changed(&[path]);
        let mut outputs = make_pipeline_outputs_for(&[path]);
        outputs.transforms.insert(path.to_string(), String::new());

        // When
        let result = build_file_state_changes(&diff, &outputs);

        // Then
        let changes = result.expect("empty transform content should succeed");
        assert_eq!(changes.updated_files.len(), 1, "should have 1 updated file");
        assert_eq!(
            changes.new_transforms.len(),
            1,
            "should have 1 transform payload"
        );
        assert!(
            !changes.new_transforms[0].1.is_empty(),
            "rkyv bytes for empty string must be non-empty"
        );
        assert_ne!(
            changes.updated_files[0].1.transform_hash, [0u8; 32],
            "transform_hash must be non-zero even for empty content"
        );
    }

    // ===================================================================
    // HELPER: hash_payload (B30, B31, B32)
    // ===================================================================

    // B30: SHA-256 of "hello world" known test vector

    #[test]
    fn hash_payload_returns_sha256_of_input_bytes() {
        // Given: the known SHA-256 test vector for "hello world"
        let input = b"hello world";
        let expected: [u8; 32] = [
            0xb9, 0x4d, 0x27, 0xb9, 0x93, 0x4d, 0x3e, 0x08, 0xa5, 0x2e, 0x52, 0xd7, 0xda, 0x7d,
            0xab, 0xfa, 0xc4, 0x84, 0xef, 0xe3, 0x7a, 0x53, 0x80, 0xee, 0x90, 0x88, 0xf7, 0xac,
            0xe2, 0xef, 0xcd, 0xe9,
        ];

        // When
        let result = hash_payload(input);

        // Then
        assert_eq!(
            result, expected,
            "hash_payload must return SHA-256 of input"
        );
    }

    // B31: Determinism

    #[test]
    fn hash_payload_produces_identical_output_for_identical_input() {
        // Given
        let input = b"determinism test data";

        // When
        let hash1 = hash_payload(input);
        let hash2 = hash_payload(input);

        // Then
        assert_eq!(
            hash1, hash2,
            "identical inputs must produce identical hashes"
        );
    }

    // B32: Non-zero for non-empty input

    #[test]
    fn hash_payload_returns_non_zero_hash_for_non_empty_input() {
        // Given
        let input = b"any non-empty data";

        // When
        let result = hash_payload(input);

        // Then
        assert_ne!(
            result, [0u8; 32],
            "hash of non-empty input must be non-zero"
        );
    }

    // ===================================================================
    // HELPER: serialize_and_hash (B33, B33b, B33c, B33d, B34)
    // ===================================================================

    // B33: Returns hash and serialized bytes for valid input

    #[test]
    fn serialize_and_hash_returns_hash_and_bytes_for_valid_input() {
        // Given
        let value = "test transform content";

        // When
        let result = serialize_and_hash(value, "test/path.md");

        // Then
        let (hash, bytes) = result.expect("should succeed for valid string");
        assert!(!bytes.is_empty(), "serialized bytes must be non-empty");
        // hash must be SHA-256 of the serialized bytes
        assert_eq!(
            hash,
            hash_payload(&bytes),
            "hash must be SHA-256 of serialized bytes"
        );
    }

    // B34: Error includes path context with non-empty reason
    // NOTE: serialize_and_hash uses serde_json which cannot fail for &str serialization.
    // This test verifies the error variant structure for when rkyv serialization is integrated.
    // Once rkyv is adopted, a non-serializable type or fault injection can trigger this path.
    #[test]
    #[ignore = "requires rkyv serialization to trigger failure; serde_json cannot fail for &str"]
    fn serialize_and_hash_includes_path_and_reason_in_error_when_serialization_fails() {
        let result = serialize_and_hash("value", "docs/fail.md");

        match result {
            Err(BatchBuildError::AnalysisSerializationFailed { path, reason }) => {
                assert_eq!(path, "docs/fail.md");
                assert!(!reason.is_empty(), "reason must never be empty");
            }
            Err(BatchBuildError::TransformSerializationFailed { path, reason }) => {
                assert_eq!(path, "docs/fail.md");
                assert!(!reason.is_empty(), "reason must never be empty");
            }
            Err(BatchBuildError::ChunkSerializationFailed { path, reason }) => {
                assert_eq!(path, "docs/fail.md");
                assert!(!reason.is_empty(), "reason must never be empty");
            }
            Ok((hash, bytes)) => {
                assert_eq!(
                    hash,
                    hash_payload(&bytes),
                    "hash must match SHA-256 of bytes"
                );
                panic!(
                    "serialize_and_hash should fail for this test case, \
                     but it succeeded with {} bytes",
                    bytes.len()
                );
            }
            Err(other) => {
                panic!("wrong error variant: {:?}", other);
            }
        }
    }

    // B33b: Minimum serializable value (empty String)

    #[test]
    fn serialize_and_hash_handles_empty_string_without_panic() {
        // Given
        let value = "";

        // When
        let result = serialize_and_hash(value, "test.md");

        // Then
        let (hash, bytes) = result.expect("empty string should serialize successfully");
        assert!(
            !bytes.is_empty(),
            "rkyv bytes for empty string must be non-empty"
        );
        assert_eq!(
            hash,
            hash_payload(&bytes),
            "hash must match SHA-256 of bytes"
        );
    }

    // B33c: Large serializable value (64KB)

    #[test]
    fn serialize_and_hash_handles_large_value_without_panic() {
        // Given: 64KB string
        let large_value = "a".repeat(65536);

        // When
        let result = serialize_and_hash(&large_value, "large.md");

        // Then
        let (hash, bytes) = result.expect("large value should serialize successfully");
        assert!(
            bytes.len() >= 65536,
            "serialized bytes should be at least as large as input"
        );
        assert_eq!(
            hash,
            hash_payload(&bytes),
            "hash must match SHA-256 of bytes"
        );
    }

    // B33d: Empty path string

    #[test]
    fn serialize_and_hash_handles_empty_path_string_without_panic() {
        // Given
        let value = "some content";

        // When
        let result = serialize_and_hash(value, "");

        // Then
        let (hash, bytes) = result.expect("empty path should succeed");
        assert_eq!(
            hash,
            hash_payload(&bytes),
            "hash must match SHA-256 of bytes"
        );
    }

    // ===================================================================
    // HELPER: build_file_state_raw (B35, B36, B37, B38)
    // ===================================================================

    // B35: Sets all hash fields to provided values

    #[test]
    fn build_file_state_raw_sets_all_hash_fields_to_provided_values() {
        // Given
        let content = make_hash(1);
        let config = make_hash(2);
        let analysis = make_hash(3);
        let transform = make_hash(4);
        let chunk = make_hash(5);

        // When
        let raw = build_file_state_raw(content, config, analysis, transform, chunk, 1_700_000_000);

        // Then
        assert_eq!(raw.content_hash, content, "content_hash must match");
        assert_eq!(raw.config_hash, config, "config_hash must match");
        assert_eq!(raw.analysis_hash, analysis, "analysis_hash must match");
        assert_eq!(raw.transform_hash, transform, "transform_hash must match");
        assert_eq!(raw.chunk_hash, chunk, "chunk_hash must match");
    }

    // B36: Sets last_processed_secs

    #[test]
    fn build_file_state_raw_sets_last_processed_secs() {
        // Given
        let now_secs = 1_700_000_000;

        // When
        let raw = build_file_state_raw(
            make_hash(0),
            make_hash(0),
            make_hash(0),
            make_hash(0),
            make_hash(0),
            now_secs,
        );

        // Then
        assert_eq!(
            raw.last_processed_secs, now_secs,
            "last_processed_secs must match"
        );
    }

    // B37: Zeroes reserved field

    #[test]
    fn build_file_state_raw_zeroesreserved_field() {
        // Given: any inputs
        let raw = build_file_state_raw(
            make_hash(0xFF),
            make_hash(0xFF),
            make_hash(0xFF),
            make_hash(0xFF),
            make_hash(0xFF),
            999,
        );

        // Then
        assert_eq!(raw.reserved, [0u8; 32], "reserved must be all zeros");
    }

    // B38: Struct size is exactly 200 bytes (verified through the function)

    #[test]
    fn build_file_state_raw_produces_200_byte_struct() {
        // Call the function so this test depends on implementation (RED phase).
        // The returned struct must be exactly 200 bytes.
        let raw = build_file_state_raw(
            make_hash(0),
            make_hash(0),
            make_hash(0),
            make_hash(0),
            make_hash(0),
            0,
        );
        assert_eq!(
            std::mem::size_of_val(&raw),
            200,
            "FileStateRaw must be exactly 200 bytes"
        );
    }

    // ===================================================================
    // Boundary tests for build_file_state_raw
    // ===================================================================

    #[test]
    fn build_file_state_raw_handles_all_zero_hashes() {
        let raw = build_file_state_raw([0u8; 32], [0u8; 32], [0u8; 32], [0u8; 32], [0u8; 32], 0);
        assert_eq!(raw.content_hash, [0u8; 32]);
        assert_eq!(raw.config_hash, [0u8; 32]);
        assert_eq!(raw.analysis_hash, [0u8; 32]);
        assert_eq!(raw.transform_hash, [0u8; 32]);
        assert_eq!(raw.chunk_hash, [0u8; 32]);
        assert_eq!(raw.last_processed_secs, 0);
        assert_eq!(raw.reserved, [0u8; 32]);
    }

    #[test]
    fn build_file_state_raw_handles_max_values() {
        let max_hash = [0xFFu8; 32];
        let raw = build_file_state_raw(max_hash, max_hash, max_hash, max_hash, max_hash, u64::MAX);
        assert_eq!(raw.content_hash, max_hash);
        assert_eq!(raw.config_hash, max_hash);
        assert_eq!(raw.last_processed_secs, u64::MAX);
        assert_eq!(raw.reserved, [0u8; 32]);
    }

    // ===================================================================
    // Boundary tests for hash_payload
    // ===================================================================

    #[test]
    fn hash_payload_handles_empty_bytes() {
        let result = hash_payload(b"");
        // SHA-256 of empty input is a known value
        let expected: [u8; 32] = [
            0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f,
            0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b,
            0x78, 0x52, 0xb8, 0x55,
        ];
        assert_eq!(
            result, expected,
            "SHA-256 of empty input must match known value"
        );
    }

    #[test]
    fn hash_payload_handles_single_byte() {
        let result = hash_payload(b"a");
        // SHA-256 of "a" — known value
        let expected: [u8; 32] = [
            0xca, 0x97, 0x81, 0x12, 0xca, 0x1b, 0xbd, 0xca, 0xfa, 0xc2, 0x31, 0xb3, 0x9a, 0x23,
            0xdc, 0x4d, 0xa7, 0x86, 0xef, 0xf8, 0x14, 0x7c, 0x4e, 0x72, 0xb9, 0x80, 0x77, 0x85,
            0xaf, 0xee, 0x48, 0xbb,
        ];
        assert_eq!(result, expected, "SHA-256 of 'a' must match known value");
    }

    #[test]
    fn hash_payload_handles_large_input() {
        let large_input = vec![0u8; 1_048_576]; // 1MB of zeros
        let result = hash_payload(&large_input);
        assert_ne!(result, [0u8; 32], "hash of large input must be non-zero");
    }

    // ===================================================================
    // PROPTESTS
    // ===================================================================

    // Proptest 1: hash_payload determinism

    proptest! {
        #[test]
        fn proptest_hash_payload_is_deterministic(input in prop::collection::vec(any::<u8>(), 0..1024)) {
            let hash1 = hash_payload(&input);
            let hash2 = hash_payload(&input);
            prop_assert_eq!(hash1, hash2, "hash_payload must be deterministic");
        }
    }

    // Proptest 2: hash_payload injectivity

    proptest! {
        #[test]
        fn proptest_hash_payload_is_injective_for_distinct_inputs(
            a in prop::collection::vec(any::<u8>(), 0..1024),
            b in prop::collection::vec(any::<u8>(), 0..1024),
        ) {
            prop_assume!(a != b);
            let hash_a = hash_payload(&a);
            let hash_b = hash_payload(&b);
            prop_assert_ne!(hash_a, hash_b, "distinct inputs must produce distinct hashes");
        }
    }

    // Proptest 3: build_file_state_raw field preservation

    proptest! {
        #[test]
        fn proptest_build_file_state_raw_preserves_all_fields(
            content in any::<[u8; 32]>(),
            config in any::<[u8; 32]>(),
            analysis in any::<[u8; 32]>(),
            transform in any::<[u8; 32]>(),
            chunk in any::<[u8; 32]>(),
            now_secs in any::<u64>(),
        ) {
            let raw = build_file_state_raw(content, config, analysis, transform, chunk, now_secs);
            prop_assert_eq!(raw.content_hash, content);
            prop_assert_eq!(raw.config_hash, config);
            prop_assert_eq!(raw.analysis_hash, analysis);
            prop_assert_eq!(raw.transform_hash, transform);
            prop_assert_eq!(raw.chunk_hash, chunk);
            prop_assert_eq!(raw.last_processed_secs, now_secs);
            prop_assert_eq!(raw.reserved, [0u8; 32]);
        }
    }

    // Proptest 4: Count invariants (INV-01, INV-02, INV-03)

    proptest! {
        #[test]
        fn proptest_count_invariants_hold_for_valid_inputs(
            changed_count in 0usize..5,
            new_count in 0usize..5,
            deleted_count in 0usize..5,
        ) {
            // Skip the all-empty case (returns Err(EmptyDiff))
            prop_assume!(changed_count + new_count + deleted_count > 0);

            let changed_paths: Vec<String> = (0..changed_count)
                .map(|i| format!("changed/{}.md", i))
                .collect();
            let new_paths: Vec<String> = (0..new_count)
                .map(|i| format!("new/{}.md", i))
                .collect();
            let deleted_paths: Vec<String> = (0..deleted_count)
                .map(|i| format!("deleted/{}.md", i))
                .collect();

            let all_active: Vec<&str> = changed_paths.iter()
                .chain(new_paths.iter())
                .map(|s| s.as_str())
                .collect();

            let diff = FileDiff {
                unchanged: vec![],
                changed: changed_paths.iter().map(|p| make_discovery_file(p)).collect(),
                new_files: new_paths.iter().map(|p| make_discovery_file(p)).collect(),
                deleted: deleted_paths.clone(),
            };
            let outputs = make_pipeline_outputs_for(&all_active);

            let result = build_file_state_changes(&diff, &outputs);
            let changes = result.expect("should succeed for valid inputs");

            // INV-01: updated_files.len() == changed + new
            prop_assert_eq!(
                changes.updated_files.len(),
                changed_count + new_count,
                "INV-01: updated_files count must equal changed + new"
            );

            // INV-02: deleted_files.len() == deleted
            prop_assert_eq!(
                changes.deleted_files.len(),
                deleted_count,
                "INV-02: deleted_files count must equal deleted count"
            );

            // INV-03: payload counts match file count
            let expected = changed_count + new_count;
            prop_assert_eq!(changes.new_analyses.len(), expected, "INV-03: analyses count");
            prop_assert_eq!(changes.new_transforms.len(), expected, "INV-03: transforms count");
            prop_assert_eq!(changes.new_chunks.len(), expected, "INV-03: chunks count");
        }
    }

    // Proptest 5: Hash consistency (INV-04, INV-05)

    proptest! {
        #[test]
        fn proptest_hash_consistency_invariants_hold(
            changed_count in 1usize..5,
            new_count in 0usize..3,
        ) {
            let changed_paths: Vec<String> = (0..changed_count)
                .map(|i| format!("c/{}.md", i))
                .collect();
            let new_paths: Vec<String> = (0..new_count)
                .map(|i| format!("n/{}.md", i))
                .collect();

            let all_active: Vec<&str> = changed_paths.iter()
                .chain(new_paths.iter())
                .map(|s| s.as_str())
                .collect();

            let diff = FileDiff {
                unchanged: vec![],
                changed: changed_paths.iter().map(|p| make_discovery_file(p)).collect(),
                new_files: new_paths.iter().map(|p| make_discovery_file(p)).collect(),
                deleted: vec![],
            };
            let outputs = make_pipeline_outputs_for(&all_active);

            let result = build_file_state_changes(&diff, &outputs);
            let changes = result.expect("should succeed");

            // INV-04: every hash in FileStateRaw appears in corresponding payload
            for (_path, state) in &changes.updated_files {
                let analysis_keys: Vec<[u8; 32]> = changes.new_analyses.iter().map(|(k, _)| *k).collect();
                let transform_keys: Vec<[u8; 32]> = changes.new_transforms.iter().map(|(k, _)| *k).collect();
                let chunk_keys: Vec<[u8; 32]> = changes.new_chunks.iter().map(|(k, _)| *k).collect();

                prop_assert!(analysis_keys.contains(&state.analysis_hash),
                    "INV-04: analysis_hash must appear in new_analyses keys");
                prop_assert!(transform_keys.contains(&state.transform_hash),
                    "INV-04: transform_hash must appear in new_transforms keys");
                prop_assert!(chunk_keys.contains(&state.chunk_hash),
                    "INV-04: chunk_hash must appear in new_chunks keys");
            }

            // INV-05: every payload key is referenced by exactly one FileStateRaw
            let analysis_refs: Vec<[u8; 32]> = changes.updated_files.iter()
                .map(|(_, s)| s.analysis_hash).collect();
            let transform_refs: Vec<[u8; 32]> = changes.updated_files.iter()
                .map(|(_, s)| s.transform_hash).collect();
            let chunk_refs: Vec<[u8; 32]> = changes.updated_files.iter()
                .map(|(_, s)| s.chunk_hash).collect();

            for (key, _) in &changes.new_analyses {
                let count = analysis_refs.iter().filter(|h| *h == key).count();
                prop_assert_eq!(count, 1, "INV-05: each analysis key referenced exactly once");
            }
            for (key, _) in &changes.new_transforms {
                let count = transform_refs.iter().filter(|h| *h == key).count();
                prop_assert_eq!(count, 1, "INV-05: each transform key referenced exactly once");
            }
            for (key, _) in &changes.new_chunks {
                let count = chunk_refs.iter().filter(|h| *h == key).count();
                prop_assert_eq!(count, 1, "INV-05: each chunk key referenced exactly once");
            }
        }
    }

    // Proptest 6: Determinism

    proptest! {
        #[test]
        fn proptest_build_changes_is_deterministic(
            file_count in 1usize..5,
        ) {
            let paths: Vec<String> = (0..file_count)
                .map(|i| format!("det/{}.md", i))
                .collect();
            let path_refs: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();

            let diff = make_diff_with_changed(&path_refs);
            let outputs = make_pipeline_outputs_for(&path_refs);

            let result1 = build_file_state_changes(&diff, &outputs);
            let result2 = build_file_state_changes(&diff, &outputs);

            let changes1 = result1.expect("first call should succeed");
            let changes2 = result2.expect("second call should succeed");

            prop_assert_eq!(changes1.updated_files, changes2.updated_files);
            prop_assert_eq!(changes1.new_analyses, changes2.new_analyses);
            prop_assert_eq!(changes1.new_transforms, changes2.new_transforms);
            prop_assert_eq!(changes1.new_chunks, changes2.new_chunks);
        }
    }

    // Proptest 7: Unchanged exclusion (INV-06)

    proptest! {
        #[test]
        fn proptest_unchanged_files_never_appear_in_output(
            unchanged_count in 0usize..5,
            changed_count in 1usize..5,
        ) {
            let unchanged_paths: Vec<String> = (0..unchanged_count)
                .map(|i| format!("unchanged/{}.md", i))
                .collect();
            let changed_paths: Vec<String> = (0..changed_count)
                .map(|i| format!("changed/{}.md", i))
                .collect();

            let active_refs: Vec<&str> = changed_paths.iter().map(|s| s.as_str()).collect();

            let diff = FileDiff {
                unchanged: unchanged_paths.iter().map(|p| make_unchanged_entry(p)).collect(),
                changed: changed_paths.iter().map(|p| make_discovery_file(p)).collect(),
                new_files: vec![],
                deleted: vec![],
            };
            let outputs = make_pipeline_outputs_for(&active_refs);

            let result = build_file_state_changes(&diff, &outputs);
            let changes = result.expect("should succeed");

            let updated_paths: Vec<&str> = changes.updated_files.iter()
                .map(|(p, _)| p.as_str()).collect();

            for uc_path in &unchanged_paths {
                prop_assert!(
                    !updated_paths.contains(&uc_path.as_str()),
                    "INV-06: unchanged file {} must not appear in updated_files",
                    uc_path
                );
            }
        }
    }

    // Proptest 8: serialize_and_hash — hash integrity

    proptest! {
        #[test]
        fn proptest_serialize_and_hash_hash_matches_sha256_of_bytes(
            input in ".*",
        ) {
            let result = serialize_and_hash(&input, "proptest.md");
            let (hash, bytes) = result.expect("string serialization should succeed");
            let expected_hash = hash_payload(&bytes);
            prop_assert_eq!(hash, expected_hash, "hash must equal SHA-256 of serialized bytes");
        }
    }
}

// ===========================================================================
// Kani Harnesses
// ===========================================================================

#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn kani_file_state_raw_size() {
        let content: [u8; 32] = kani::any();
        let config: [u8; 32] = kani::any();
        let analysis: [u8; 32] = kani::any();
        let transform: [u8; 32] = kani::any();
        let chunk: [u8; 32] = kani::any();
        let now: u64 = kani::any();
        let raw = build_file_state_raw(content, config, analysis, transform, chunk, now);
        assert!(std::mem::size_of_val(&raw) == 200);
        assert!(raw.reserved == [0u8; 32]);
    }

    #[kani::proof]
    fn kani_hash_payload_nonzero() {
        let len: usize = kani::any();
        kani::assume(len > 0 && len <= 32);
        let mut bytes = vec![0u8; len];
        for b in bytes.iter_mut() {
            *b = kani::any();
        }
        let hash = hash_payload(&bytes);
        assert!(hash != [0u8; 32]);
    }

    #[kani::proof]
    fn kani_file_state_raw_preserves_fields() {
        let content: [u8; 32] = kani::any();
        let config: [u8; 32] = kani::any();
        let analysis: [u8; 32] = kani::any();
        let transform: [u8; 32] = kani::any();
        let chunk: [u8; 32] = kani::any();
        let now: u64 = kani::any();
        let raw = build_file_state_raw(content, config, analysis, transform, chunk, now);
        assert!(raw.content_hash == content);
        assert!(raw.config_hash == config);
        assert!(raw.analysis_hash == analysis);
        assert!(raw.transform_hash == transform);
        assert!(raw.chunk_hash == chunk);
        assert!(raw.last_processed_secs == now);
        assert!(raw.reserved == [0u8; 32]);
    }
}
