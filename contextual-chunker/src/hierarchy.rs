//! Hierarchical chunk linking and parent-child assignment.
//!
//! Assigns parent-child relationships across chunk levels (Summary →
//! Standard → Detailed) and creates sequential prev/next navigation links.

// for_each is intentional: functional-rust doctrine forbids `for`/`while` loops.
#![allow(clippy::needless_for_each)]

use crate::chunk::{heading_regex, Chunk};
use std::collections::HashMap;

/// Key function: returns the heading string for grouping, defaulting to "intro".
pub(crate) fn heading_key(chunk: &Chunk) -> String {
    chunk
        .heading
        .clone()
        .unwrap_or_else(|| "intro".to_string())
}

/// Group chunk indices by their heading key.
pub(crate) fn group_by_heading(chunks: &[Chunk]) -> HashMap<String, Vec<usize>> {
    chunks
        .iter()
        .enumerate()
        .map(|(idx, chunk)| (heading_key(chunk), idx))
        .fold(HashMap::new(), |mut groups, (key, idx)| {
            groups.entry(key).or_default().push(idx);
            groups
        })
}

/// Link child chunks to parent chunks by heading group.
///
/// Returns a map of `parent_idx → Vec<child_chunk_id>` for setting
/// `child_chunk_ids` on parents after mutation.
fn link_parent_to_children(
    parents: &[Chunk],
    children: &mut [Chunk],
    parent_groups: &HashMap<String, Vec<usize>>,
    child_groups: &HashMap<String, Vec<usize>>,
    fallback_key: &str,
) -> HashMap<usize, Vec<String>> {
    let mut parent_children: HashMap<usize, Vec<String>> = HashMap::new();

    child_groups.iter().for_each(|(heading, child_indices)| {
        let parent_indices = parent_groups
            .get(heading)
            .or_else(|| parent_groups.get(fallback_key));

        if let Some(parent_indices) = parent_indices {
            let items_per_parent = child_indices.len().div_ceil(parent_indices.len());
            child_indices
                .iter()
                .enumerate()
                .for_each(|(pos, child_idx)| {
                    let parent_idx = parent_indices[pos / items_per_parent];
                    children[*child_idx].parent_chunk_id =
                        Some(parents[parent_idx].chunk_id.clone());
                    parent_children
                        .entry(parent_idx)
                        .or_default()
                        .push(children[*child_idx].chunk_id.clone());
                });
        }
    });

    parent_children
}

/// Assign parent-child relationships across all three hierarchical levels.
///
/// Summary chunks become parents of Standard chunks, and Standard chunks
/// become parents of Detailed chunks. Distribution uses `div_ceil` for
/// even child-to-parent mapping.
pub(crate) fn assign_hierarchy(
    summary: &mut [Chunk],
    standard: &mut [Chunk],
    detailed: &mut [Chunk],
) {
    if summary.is_empty() && standard.is_empty() && detailed.is_empty() {
        return;
    }

    let summary_groups = group_by_heading(summary);
    let standard_groups = group_by_heading(standard);
    let detailed_groups = group_by_heading(detailed);

    let summary_children = link_parent_to_children(
        summary,
        standard,
        &summary_groups,
        &standard_groups,
        "intro",
    );
    let standard_children = link_parent_to_children(
        standard,
        detailed,
        &standard_groups,
        &detailed_groups,
        "intro",
    );

    summary_children
        .into_iter()
        .for_each(|(idx, children)| summary[idx].child_chunk_ids = children);
    standard_children
        .into_iter()
        .for_each(|(idx, children)| standard[idx].child_chunk_ids = children);
}

/// Link chunks sequentially (prev/next) within same document and level.
pub(crate) fn link_chunks(chunks: &mut [Chunk]) {
    (0..chunks.len().saturating_sub(1)).for_each(|i| {
        if chunks[i].doc_id == chunks[i + 1].doc_id
            && chunks[i].chunk_level == chunks[i + 1].chunk_level
        {
            chunks[i].next_chunk_id = Some(chunks[i + 1].chunk_id.clone());
        }
    });
}

/// Parse a markdown heading line into `(level, text)`.
pub(crate) fn parse_heading(line: &str) -> Option<(usize, String)> {
    let regex = heading_regex().ok()?;
    let caps = regex.captures(line)?;
    let level = caps.get(1)?.as_str().len();
    let text = caps.get(2)?.as_str().trim().to_string();
    Some((level, text))
}

/// Update the heading stack for a new heading at the given level.
pub(crate) fn update_heading_stack(stack: &mut Vec<String>, level: usize, text: String) {
    if level == 0 {
        return;
    }
    let target_len = level.saturating_sub(1);
    stack.truncate(target_len);
    stack.resize_with(target_len, String::new);
    stack.push(text);
}

/// Normalize a heading stack into a path, filtering empty entries.
pub(crate) fn normalize_heading_path(stack: &[String]) -> Vec<String> {
    let path: Vec<String> = stack.iter().filter(|s| !s.is_empty()).cloned().collect();
    if path.is_empty() {
        vec!["Intro".to_string()]
    } else {
        path
    }
}
