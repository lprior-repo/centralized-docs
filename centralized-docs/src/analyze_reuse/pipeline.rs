use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::analyze::{analyze_files, Analysis, FailedFile};
use crate::diff::compute_file_diff;
use crate::discover::DiscoveryFile;
use crate::persisted::{persisted_analysis_to_runtime, PersistedAnalyzeResult};
use crate::state::bulk_load::{OwnedArchive, StateReadSession};
use crate::state::FileStateRaw;

use super::helpers::{build_stored_hashes, merge_analyses_in_order, partition_for_reuse};
use super::types::{AnalyzeReuseStats, ReuseAnalysisError};

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
/// `BulkLoadError` from `load_analyses` is propagated (fatal).
/// Individual deserialization failures are non-fatal (file added to fallback).
///
/// # Arguments
///
/// * `reusable_paths` -- Unchanged file paths to load from archive.
/// * `file_states` -- Loaded file state entries (provides `analysis_hash`).
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
            Some(archive) => match deserialize_single_analysis(archive) {
                Ok(analysis) => {
                    successful_analyses.push(analysis);
                }
                Err(()) => {
                    // Deserialization failed or empty analyses vec — fallback
                    fallback_paths.insert(path.to_string());
                }
            },
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
#[allow(clippy::too_many_lines)]
pub fn analyze_with_reuse(
    files: &[DiscoveryFile],
    source_dir: &Path,
    category_config_path: Option<&Path>,
    session: &StateReadSession<'_>,
) -> Result<(crate::analyze::AnalyzeResult, AnalyzeReuseStats), ReuseAnalysisError> {
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
