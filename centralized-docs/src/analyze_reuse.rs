//! Implementation for bead cdocs-b5h: Reuse archived analyses for unchanged files.
//!
//! Data flow:
//! ```text
//! Input:  files, source_dir, config_path, session
//!             |
//!             v
//!     [load_file_states(session)]
//!             |
//!             v
//!     HashMap<String, FileStateRaw>
//!             |
//!             v
//!     [build_stored_hashes]
//!             |
//!             v
//!     HashMap<String, StoredHashes>
//!             |
//!             v
//!     [compute_file_diff(files, source_dir, config_path, stored_hashes)]
//!             |
//!             v
//!     FileDiff { unchanged, changed, new, deleted }
//!             |
//!             v
//!     [partition_for_reuse(files, diff)]
//!             |
//!        _____|_____
//!       |           |
//!       v           v
//!  unchanged     changed+new
//!  paths         files
//!       |           |
//!       v           |
//!   [load_archived  |
//!    _analyses]     |
//!       |           |
//!    (Vec<Analysis>,|
//!     fallback)     |
//!       |           |
//!       v           v
//!   fallback + changed + new files
//!       |
//!       v
//!   [analyze_files(subset, source_dir, config_path)]
//!       |
//!       v
//!   Vec<Analysis> (fresh)
//!       |
//!       v
//!   [merge_analyses_in_order]
//!       |
//!       v
//!   (AnalyzeResult, AnalyzeReuseStats)
//! ```

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![forbid(unsafe_code)]
#![allow(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::uninlined_format_args)]

use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::analyze::{analyze_files, Analysis, AnalyzeResult, FailedFile};
use crate::diff::{compute_file_diff, FileDiff, StoredHashes};
use crate::discover::DiscoveryFile;
use crate::persisted::{persisted_analysis_to_runtime, PersistedAnalyzeResult};
use crate::state::bulk_load::{BulkLoadError, OwnedArchive, StateReadSession};
use crate::state::{FileStateRaw, StateLoadError};

// ---------------------------------------------------------------------------
// Error taxonomy
// ---------------------------------------------------------------------------

/// Error type for the analysis-reuse pipeline.
#[derive(Debug, thiserror::Error)]
pub enum ReuseAnalysisError {
    /// Failed to load file states from the state database.
    #[error("failed to load file states: {0}")]
    StateLoad(#[from] StateLoadError),

    /// Failed to load archived analyses from the state database.
    #[error("failed to load archived analyses: {0}")]
    BulkLoad(#[from] BulkLoadError),

    /// Failed to compute file diff.
    #[error("failed to compute file diff: {0}")]
    DiffError(#[from] crate::diff::DiffError),

    /// All files failed analysis (no successful analyses).
    #[error("all {count} file(s) failed analysis. Errors: {error_summary}")]
    AllFilesFailed {
        /// Number of files that failed.
        count: usize,
        /// Summary of errors encountered.
        error_summary: String,
    },
}

// ---------------------------------------------------------------------------
// Statistics
// ---------------------------------------------------------------------------

/// Statistics about analysis reuse within a single `run_index` invocation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AnalyzeReuseStats {
    /// Number of analyses loaded from archived state (zero-cost reuse).
    pub reused: usize,
    /// Number of analyses computed fresh via `analyze_single_file`.
    pub analyzed: usize,
}

// ---------------------------------------------------------------------------
// Pure helper: build_stored_hashes
// ---------------------------------------------------------------------------

/// Build a `StoredHashes` map from loaded file states.
///
/// Pure calculation: converts `HashMap<String, FileStateRaw>` to
/// `HashMap<String, StoredHashes>` by extracting `content_hash` and
/// `config_hash` fields.
///
/// # Invariants
///
/// - INV-06: Deterministic output for identical input.
/// - Returns empty map when input is empty (first-run case).
#[must_use]
pub fn build_stored_hashes(
    file_states: &HashMap<String, FileStateRaw>,
) -> HashMap<String, StoredHashes> {
    file_states
        .iter()
        .map(|(path, raw)| {
            (
                path.clone(),
                StoredHashes {
                    content_hash: raw.content_hash.into(),
                    config_hash: raw.config_hash.into(),
                },
            )
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Pure helper: partition_for_reuse
// ---------------------------------------------------------------------------

/// Partition discovered files into (reusable_paths, needs_analysis_files)
/// based on FileDiff classification.
///
/// Pure calculation: no I/O, no errors.
///
/// # Arguments
///
/// * `files` -- All discovered files in discovery order.
/// * `diff` -- The computed FileDiff partition.
///
/// # Returns
///
/// * `reusable_paths` -- Set of source_paths that are Unchanged (sorted).
/// * `needs_analysis` -- Vec of DiscoveryFile for Changed + New files
///   (preserving input order).
///
/// # Invariants
///
/// - INV-01: `reusable_paths ∪ {source_paths in needs_analysis} ∪ deleted`
///   covers all input files (deleted files are not in input, so excluded).
/// - No file appears in both groups.
#[must_use]
pub fn partition_for_reuse<'a>(
    files: &'a [DiscoveryFile],
    diff: &FileDiff,
) -> (HashSet<String>, Vec<&'a DiscoveryFile>) {
    let reusable_paths = diff.unchanged.clone();

    let needs_analysis: Vec<&'a DiscoveryFile> = files
        .iter()
        .filter(|f| diff.changed.contains(&f.source_path) || diff.new.contains(&f.source_path))
        .collect();

    (reusable_paths, needs_analysis)
}

// ---------------------------------------------------------------------------
// I/O helper: load_archived_analyses
// ---------------------------------------------------------------------------

/// Load and deserialize archived analyses for unchanged files.
///
/// For each reusable path, looks up its `FileStateRaw.analysis_hash` and
/// attempts to load + deserialize from the state database. Files whose
/// archive entry is missing or corrupt are added to `fallback_paths`.
///
/// # Error handling
///
/// BulkLoadError from `load_analyses` is propagated (fatal).
/// Individual deserialization failures are non-fatal (file added to fallback).
///
/// # Arguments
///
/// * `reusable_paths` -- Unchanged file paths to load from archive.
/// * `file_states` -- Loaded file state entries (provides analysis_hash).
/// * `session` -- Shared read session for archive access.
///
/// # Returns
///
/// * `Vec<Analysis>` -- Successfully deserialized analyses.
/// * `HashSet<String>` -- Paths that need re-analysis (archive miss/corrupt).
pub fn load_archived_analyses(
    reusable_paths: &HashSet<String>,
    file_states: &HashMap<String, FileStateRaw>,
    session: &StateReadSession<'_>,
) -> Result<(Vec<Analysis>, HashSet<String>), ReuseAnalysisError> {
    // Early return for empty input
    if reusable_paths.is_empty() {
        return Ok((Vec::new(), HashSet::new()));
    }

    // Collect analysis hashes for paths that have a non-zero hash
    let mut paths_and_hashes: Vec<(&str, [u8; 32])> = Vec::new();
    for path in reusable_paths {
        if let Some(state) = file_states.get(path) {
            // Zero hash means "never analyzed" — treat as fallback
            if state.analysis_hash != [0u8; 32] {
                paths_and_hashes.push((path.as_str(), state.analysis_hash));
            }
        }
    }

    // If all paths have zero hash, return early with all as fallback
    if paths_and_hashes.is_empty() {
        return Ok((Vec::new(), reusable_paths.clone()));
    }

    // Bulk load all archived analyses
    let hashes: Vec<[u8; 32]> = paths_and_hashes.iter().map(|(_, h)| *h).collect();
    let archived_map = session.load_analyses(&hashes)?;

    // Deserialize each archived analysis
    let mut successful_analyses = Vec::with_capacity(archived_map.len());
    let mut fallback_paths = HashSet::new();

    for (path, analysis_hash) in &paths_and_hashes {
        let path = *path;
        let analysis_hash = *analysis_hash;

        match archived_map.get(&analysis_hash) {
            None => {
                // Archive entry missing — fallback to re-analysis
                fallback_paths.insert(path.to_string());
            }
            Some(archive) => {
                match deserialize_single_analysis(archive) {
                    Ok(analysis) => {
                        successful_analyses.push(analysis);
                    }
                    Err(_) => {
                        // Deserialization failed or empty analyses vec — fallback
                        fallback_paths.insert(path.to_string());
                    }
                }
            }
        }
    }

    Ok((successful_analyses, fallback_paths))
}

/// Deserialize a single `PersistedAnalyzeResult` from an `OwnedArchive`.
///
/// Returns `Ok(Analysis)` if the result contains exactly one analysis,
/// or `Err(())` if deserialization fails or the analyses vec is empty.
fn deserialize_single_analysis(
    archive: &OwnedArchive<PersistedAnalyzeResult>,
) -> Result<Analysis, ()> {
    let persisted = archive.deserialize().map_err(|_| ())?;

    // Each PersistedAnalyzeResult for a single file contains exactly one analysis
    if persisted.analyses.is_empty() {
        return Err(());
    }

    let first = persisted.analyses.first().ok_or(())?;

    persisted_analysis_to_runtime(first).map_err(|_| ())
}

// ---------------------------------------------------------------------------
// Pure helper: merge_analyses_in_order
// ---------------------------------------------------------------------------

/// Merge reused and freshly-analyzed analyses into a single vec in
/// discovery order, then construct the final AnalyzeResult.
///
/// Pure calculation: no I/O, no errors.
///
/// # Arguments
///
/// * `files` -- Original discovered files (defines output order).
/// * `reused_analyses` -- Analyses loaded from archive, keyed by source_path.
/// * `fresh_analyses` -- Analyses computed fresh, keyed by source_path.
/// * `failed_files` -- Files that failed both archive and fresh analysis.
/// * `total_discovered` -- Total input file count.
///
/// # Postconditions
///
/// - POST-01: One Analysis per non-failed file.
/// - POST-02: Output order matches input `files` order.
/// - POST-05: total_discovered == original files.len().
#[must_use]
pub fn merge_analyses_in_order(
    files: &[DiscoveryFile],
    mut reused_analyses: HashMap<String, Analysis>,
    mut fresh_analyses: HashMap<String, Analysis>,
    failed_files: Vec<FailedFile>,
    total_discovered: usize,
) -> AnalyzeResult {
    let mut merged_analyses = Vec::with_capacity(files.len());

    for file in files {
        // Check reused first, then fresh (order doesn't matter for lookup)
        if let Some(analysis) = reused_analyses.remove(&file.source_path) {
            merged_analyses.push(analysis);
        } else if let Some(analysis) = fresh_analyses.remove(&file.source_path) {
            merged_analyses.push(analysis);
        }
        // If neither found, file is in failed_files — don't add to analyses
    }

    AnalyzeResult {
        analyses: merged_analyses,
        failed_files,
        total_discovered,
    }
}

// ---------------------------------------------------------------------------
// Primary entry point
// ---------------------------------------------------------------------------

/// Analyze files with archived-result reuse for unchanged files.
///
/// Classifies discovered files via `compute_file_diff`, loads archived
/// analyses for unchanged files from the state database, and runs fresh
/// analysis only on changed/new files. Merges results in discovery order.
///
/// # Arguments
///
/// * `files` -- All discovered files (non-empty).
/// * `source_dir` -- Root source directory (must exist on disk).
/// * `category_config_path` -- Optional category config file path.
/// * `session` -- Shared read session for state database access.
///
/// # Errors
///
/// Returns `ReuseAnalysisError` for:
/// - State database read failures (`StateLoad`, `BulkLoad`)
/// - File diff computation failures (`DiffError`)
/// - All files failing analysis (`AllFilesFailed`)
///
/// # Guarantees
///
/// - POST-01: Every input file appears in the result.
/// - POST-02: Analysis order matches input order.
/// - POST-04: Unchanged files never touch the filesystem for analysis.
/// - INV-01: Every file classified into exactly one bucket.
/// - INV-05: No panics, unwraps, or expects.
pub fn analyze_with_reuse(
    files: &[DiscoveryFile],
    source_dir: &Path,
    category_config_path: Option<&Path>,
    session: &StateReadSession<'_>,
) -> Result<(AnalyzeResult, AnalyzeReuseStats), ReuseAnalysisError> {
    let total_discovered = files.len();

    // Step 1: Load all file states from the database
    let file_states = session.load_file_states()?;

    // Step 2: Build stored hashes for diff computation
    let stored_hashes = build_stored_hashes(&file_states);

    // Step 3: Compute file diff to classify files
    let diff = compute_file_diff(files, source_dir, category_config_path, &stored_hashes)?;

    // Step 4: Partition into reusable (unchanged) vs needs_analysis (changed+new)
    let (reusable_paths, needs_analysis_files) = partition_for_reuse(files, &diff);

    // Step 5: Load archived analyses for unchanged files
    let (reused_analyses, fallback_paths) =
        load_archived_analyses(&reusable_paths, &file_states, session)?;

    // Step 6: Build HashMap of reused analyses for merging
    let reused_map: HashMap<String, Analysis> = reused_analyses
        .into_iter()
        .map(|a| (a.source_path.clone(), a))
        .collect();

    // Step 7: Collect all files that need fresh analysis (fallback + changed + new)
    let needs_analysis_paths: HashSet<String> = needs_analysis_files
        .iter()
        .map(|f| f.source_path.clone())
        .collect();
    let all_needs_analysis: HashSet<String> = fallback_paths
        .union(&needs_analysis_paths)
        .cloned()
        .collect();

    // Step 8: Analyze files that need fresh analysis
    let (fresh_result, fresh_failed): (Vec<Analysis>, Vec<FailedFile>) =
        if all_needs_analysis.is_empty() {
            (Vec::new(), Vec::new())
        } else {
            // Build file list for analyze_files - include fallback paths as DiscoveryFile
            let needs_files: Vec<DiscoveryFile> = files
                .iter()
                .filter(|f| all_needs_analysis.contains(&f.source_path))
                .cloned()
                .collect();

            if needs_files.is_empty() {
                (Vec::new(), Vec::new())
            } else {
                match analyze_files(&needs_files, source_dir, category_config_path) {
                    Ok(result) => (result.analyses, result.failed_files),
                    Err(e) => {
                        // analyze_files returns anyhow::Error - treat all as failed
                        let error_msg = e.to_string();
                        let failed: Vec<FailedFile> = needs_files
                            .into_iter()
                            .map(|f| FailedFile {
                                source_path: f.source_path.clone(),
                                error: error_msg.clone(),
                            })
                            .collect();
                        (Vec::new(), failed)
                    }
                }
            }
        };

    // Step 9: Count stats
    let reused_count = reused_map.len();
    let analyzed_count = fresh_result.len();

    // Step 10: Build fresh analyses HashMap for merging
    let fresh_map: HashMap<String, Analysis> = fresh_result
        .into_iter()
        .map(|a| (a.source_path.clone(), a))
        .collect();

    // Step 11: Collect all failed files (from fresh analysis)
    let all_failed: Vec<FailedFile> = fresh_failed;

    // Step 12: Check for all-files-failed case
    if reused_count == 0 && analyzed_count == 0 && !all_failed.is_empty() {
        let error_summary = all_failed
            .iter()
            .map(|f| format!("{}: {}", f.source_path, f.error))
            .collect::<Vec<_>>()
            .join("; ");
        return Err(ReuseAnalysisError::AllFilesFailed {
            count: all_failed.len(),
            error_summary,
        });
    }

    // Step 13: Merge all analyses in discovery order
    let result =
        merge_analyses_in_order(files, reused_map, fresh_map, all_failed, total_discovered);

    let stats = AnalyzeReuseStats {
        reused: reused_count,
        analyzed: analyzed_count,
    };

    Ok((result, stats))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::ContentHash;

    #[test]
    fn build_stored_hashes_returns_empty_when_input_empty() {
        let input: HashMap<String, FileStateRaw> = HashMap::new();
        let result = build_stored_hashes(&input);
        assert!(result.is_empty());
    }

    #[test]
    fn build_stored_hashes_extracts_hashes() {
        let mut input = HashMap::new();
        input.insert(
            "a.md".to_string(),
            FileStateRaw {
                content_hash: [0xAA; 32],
                config_hash: [0xBB; 32],
                analysis_hash: [0xCC; 32],
                transform_hash: [0x00; 32],
                chunk_hash: [0x00; 32],
                last_processed_secs: 0,
                reserved: [0x00; 32],
            },
        );
        let result = build_stored_hashes(&input);
        assert_eq!(result.len(), 1);
        assert_eq!(result["a.md"].content_hash, ContentHash::from([0xAA; 32]));
        assert_eq!(result["a.md"].config_hash, ContentHash::from([0xBB; 32]));
    }

    #[test]
    fn partition_for_reuse_empty_diff() {
        let files = vec![
            DiscoveryFile {
                source_path: "a.md".to_string(),
                size_bytes: 10,
            },
            DiscoveryFile {
                source_path: "b.md".to_string(),
                size_bytes: 10,
            },
        ];
        let diff = FileDiff {
            unchanged: HashSet::new(),
            changed: HashSet::new(),
            new: HashSet::new(),
            deleted: HashSet::new(),
        };
        let (reusable, needs) = partition_for_reuse(&files, &diff);
        assert!(reusable.is_empty());
        // When diff is empty, no files are classified as changed or new,
        // so needs_analysis is also empty (files are neither unchanged nor changed/new)
        assert!(needs.is_empty());
    }

    #[test]
    fn partition_for_reuse_all_unchanged() {
        let files = vec![
            DiscoveryFile {
                source_path: "a.md".to_string(),
                size_bytes: 10,
            },
            DiscoveryFile {
                source_path: "b.md".to_string(),
                size_bytes: 10,
            },
        ];
        let diff = FileDiff {
            unchanged: ["a.md".to_string(), "b.md".to_string()]
                .into_iter()
                .collect(),
            changed: HashSet::new(),
            new: HashSet::new(),
            deleted: HashSet::new(),
        };
        let (reusable, needs) = partition_for_reuse(&files, &diff);
        assert_eq!(reusable.len(), 2);
        assert!(needs.is_empty());
    }

    #[test]
    fn merge_analyses_in_order_empty() {
        let files: Vec<DiscoveryFile> = vec![];
        let result = merge_analyses_in_order(&files, HashMap::new(), HashMap::new(), vec![], 0);
        assert!(result.analyses.is_empty());
        assert_eq!(result.total_discovered, 0);
    }

    #[test]
    fn merge_analyses_in_order_preserves_order() {
        let files = vec![
            DiscoveryFile {
                source_path: "c.md".to_string(),
                size_bytes: 10,
            },
            DiscoveryFile {
                source_path: "a.md".to_string(),
                size_bytes: 10,
            },
            DiscoveryFile {
                source_path: "b.md".to_string(),
                size_bytes: 10,
            },
        ];
        let mut reused = HashMap::new();
        reused.insert(
            "c.md".to_string(),
            Analysis {
                source_path: "c.md".to_string(),
                title: "C".to_string(),
                frontmatter: None,
                headings: vec![],
                links: vec![],
                first_paragraph: String::new(),
                word_count: 0,
                has_code: false,
                has_tables: false,
                category: "c".to_string(),
                content: std::sync::Arc::from(""),
            },
        );
        let mut fresh = HashMap::new();
        fresh.insert(
            "a.md".to_string(),
            Analysis {
                source_path: "a.md".to_string(),
                title: "A".to_string(),
                frontmatter: None,
                headings: vec![],
                links: vec![],
                first_paragraph: String::new(),
                word_count: 0,
                has_code: false,
                has_tables: false,
                category: "a".to_string(),
                content: std::sync::Arc::from(""),
            },
        );

        let result = merge_analyses_in_order(&files, reused, fresh, vec![], 3);

        assert_eq!(result.analyses.len(), 2);
        assert_eq!(result.analyses[0].source_path, "c.md");
        assert_eq!(result.analyses[1].source_path, "a.md");
    }
}
