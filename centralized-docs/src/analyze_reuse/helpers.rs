use std::collections::{HashMap, HashSet};

use crate::analyze::{Analysis, AnalyzeResult, FailedFile};
use crate::diff::{FileDiff, StoredHashes};
use crate::discover::DiscoveryFile;
use crate::state::FileStateRaw;

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
