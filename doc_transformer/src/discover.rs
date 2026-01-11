use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryFile {
    pub source_path: String,
    pub size_bytes: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoverManifest {
    pub source_dir: String,
    pub discovered_at: String,
    pub total_files: usize,
    pub files: Vec<DiscoveryFile>,
}

pub fn discover_files(source_dir: &Path) -> Result<(Vec<DiscoveryFile>, DiscoverManifest)> {
    if !source_dir.exists() {
        anyhow::bail!("Source directory not found: {}", source_dir.display());
    }

    let mut files = Vec::new();
    let extensions = [".md", ".mdx", ".rst", ".txt"];
    let exclude_dirs = ["node_modules", ".git", "_build", "dist", "vendor"];

    for entry in WalkDir::new(source_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Skip excluded directories
        if exclude_dirs.iter().any(|excl| path.components().any(|c| {
            c.as_os_str()
                .to_string_lossy()
                .contains(excl)
        })) {
            continue;
        }

        if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext_str = format!(".{}", ext.to_string_lossy());
                if extensions.contains(&ext_str.as_str()) {
                    let rel_path = path.strip_prefix(source_dir)?
                        .to_string_lossy()
                        .to_string();
                    let size = path.metadata()?.len();

                    files.push(DiscoveryFile {
                        source_path: rel_path,
                        size_bytes: size,
                    });
                }
            }
        }
    }

    let manifest = DiscoverManifest {
        source_dir: source_dir.to_string_lossy().to_string(),
        discovered_at: chrono::Utc::now().to_rfc3339(),
        total_files: files.len(),
        files: files.clone(),
    };

    Ok((files, manifest))
}

/// Filter files by BM25 relevance to query
/// Returns (kept_files, filtered_count)
pub fn filter_files_by_relevance(
    files: Vec<DiscoveryFile>,
    query: &str,
    threshold: f32,
    source_dir: &Path,
) -> Result<(Vec<DiscoveryFile>, usize)> {
    if files.is_empty() {
        return Ok((files, 0));
    }

    // Guard: if threshold is 0.0, keep all files (no filtering)
    if threshold <= 0.0 {
        return Ok((files, 0));
    }

    // Read file contents and calculate word counts
    let mut files_with_content: Vec<(DiscoveryFile, String, usize)> = Vec::new();

    for file in files {
        let full_path = source_dir.join(&file.source_path);
        match fs::read_to_string(&full_path) {
            Ok(content) => {
                let word_count = content.split_whitespace().count();
                files_with_content.push((file, content, word_count));
            }
            Err(_) => {
                // Skip files that can't be read
                continue;
            }
        }
    }

    if files_with_content.is_empty() {
        return Ok((Vec::new(), 0));
    }

    // Calculate average document length
    let total_words: usize = files_with_content.iter().map(|(_, _, wc)| wc).sum();
    let avg_doc_length = (total_words as f32 / files_with_content.len() as f32).max(1.0);

    // Import bm25_score from search module
    use crate::search::bm25_score;

    // Filter files by BM25 score
    let (kept, filtered): (Vec<_>, Vec<_>) = files_with_content
        .into_iter()
        .partition(|(_, content, _)| {
            let score = bm25_score(query, content, avg_doc_length, 1.5, 0.75);
            score >= threshold
        });

    let kept_files: Vec<DiscoveryFile> = kept.into_iter().map(|(file, _, _)| file).collect();
    let filtered_count = filtered.len();

    Ok((kept_files, filtered_count))
}
