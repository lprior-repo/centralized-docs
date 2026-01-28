use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
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

    for entry in WalkDir::new(source_dir).into_iter() {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Warning: Skipping path due to I/O error: {e}");
                continue;
            }
        };

        let path = entry.path();

        // Skip excluded directories
        if exclude_dirs.iter().any(|excl| {
            path.components()
                .any(|c| c.as_os_str().to_string_lossy().contains(excl))
        }) {
            continue;
        }

        if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext_str = format!(".{}", ext.to_string_lossy());
                if extensions.contains(&ext_str.as_str()) {
                    let rel_path = path.strip_prefix(source_dir)?.to_string_lossy().to_string();
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
