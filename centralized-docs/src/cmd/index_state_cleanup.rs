//! Cleanup logic for deleted source files' outputs.
//!
//! Loads old analyses to recover categories, then removes matching docs/
//! and chunks/ entries for deleted source paths.

use crate::persisted::PersistedAnalyzeResult;
use crate::state::bulk_load::StateReadSession;
use crate::state::FileStateRaw;
use crate::types::Slug;
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Compute the output filename prefix for a source path given its category.
///
/// Returns `{category}-{subcategory}-{slug}` which matches the naming convention
/// in [`assign::assign_ids`] (without the `.md` extension).
pub(crate) fn output_prefix(source_path: &str, category: &str) -> String {
    let parts: Vec<&str> = source_path.split('/').collect();
    let subcategory = parts
        .get(parts.len().saturating_sub(2))
        .map_or_else(|| "general".to_string(), |s| s.to_ascii_lowercase());
    let filename_stem = Path::new(source_path)
        .file_stem()
        .filter(|s| !s.is_empty())
        .map_or_else(
            || "untitled".to_string(),
            |s| s.to_string_lossy().to_string(),
        );
    let slug = Slug::from_text(&filename_stem).into_string();
    format!("{category}-{subcategory}-{slug}")
}

/// Remove stale output files for deleted source paths.
///
/// Loads old analyses from the state database to recover the `category` for
/// each deleted file, then removes matching `docs/` and `chunks/` entries.
/// Missing files are silently ignored (already cleaned up or never generated).
pub fn cleanup_deleted_outputs(
    session: &StateReadSession,
    deleted_paths: &HashSet<String>,
    file_states: &HashMap<String, FileStateRaw>,
    output_dir: &Path,
) -> Result<usize> {
    if deleted_paths.is_empty() {
        return Ok(0);
    }

    // Load old analyses for deleted files to recover their categories.
    let analysis_hashes: Vec<[u8; 32]> = deleted_paths
        .iter()
        .filter_map(|p| {
            file_states
                .get(p)
                .filter(|s| s.analysis_hash != [0u8; 32])
                .map(|s| s.analysis_hash)
        })
        .collect();

    let cached = session
        .load_analyses(&analysis_hashes)
        .map_err(|e| anyhow::anyhow!("load deleted file analyses: {e}"))?;

    // Build source_path → category map from loaded analyses.
    let categories: HashMap<String, String> = deleted_paths
        .iter()
        .filter_map(|p| {
            file_states.get(p).and_then(|s| {
                cached.get(&s.analysis_hash).and_then(|arc| {
                    arc.deserialize()
                        .map_err(|e| {
                            tracing::warn!(
                                path = %p,
                                error = %e,
                                "Failed to deserialize analysis for deleted file — skipping cleanup"
                            );
                            e
                        })
                        .ok()
                        .and_then(|r: PersistedAnalyzeResult| {
                            r.analyses
                                .iter()
                                .find(|a| a.source_path == *p)
                                .map(|a| (p.clone(), a.category.clone()))
                        })
                })
            })
        })
        .collect();

    // Compute output prefixes for each deletable file.
    let prefixes: Vec<String> = deleted_paths
        .iter()
        .filter_map(|p| categories.get(p).map(|cat| output_prefix(p, cat)))
        .collect();

    let docs_dir = output_dir.join("docs");
    let chunks_dir = output_dir.join("chunks");

    // Collect files to remove from docs/ matching any prefix.
    let doc_removals = collect_matching_files(&docs_dir, &prefixes);
    // Collect files to remove from chunks/ matching any prefix.
    let chunk_removals = collect_matching_files(&chunks_dir, &prefixes);

    // Remove all collected files.
    let all_removals: Vec<PathBuf> = doc_removals.into_iter().chain(chunk_removals).collect();

    let removed_count = all_removals
        .iter()
        .filter(|p| std::fs::remove_file(p).is_ok())
        .count();

    if removed_count > 0 {
        eprintln!(
            "[CLEANUP] Removed {removed_count} stale output files for {} deleted sources",
            deleted_paths.len()
        );
    }

    Ok(removed_count)
}

/// Collect file paths from `dir` whose stem starts with any of the given `prefixes`.
pub(crate) fn collect_matching_files(dir: &Path, prefixes: &[String]) -> Vec<PathBuf> {
    std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .filter(|entry| {
                    entry.file_name().to_str().map_or(false, |name| {
                        prefixes.iter().any(|prefix| {
                            name.starts_with(prefix.as_str()) && name.ends_with(".md")
                        })
                    })
                })
                .map(|entry| entry.path())
                .collect()
        })
        .unwrap_or_default()
}
