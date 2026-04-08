use anyhow::Result;
use walkdir::WalkDir;

/// Test helper: Discover markdown files from a directory (for integration tests)
///
/// This function is used in integration tests to simulate the discovery phase
/// without depending on full discover module. Returns a Vec of relative paths.
///
/// # Errors
///
/// Returns an error if:
/// - Path resolution fails (e.g., permission issues)
#[allow(dead_code)] // Test helper function for integration tests
pub fn discover_test_files(root: &std::path::Path) -> Result<Vec<String>, anyhow::Error> {
    let extensions = [".md", ".mdx", ".markdown", ".mdown", ".mkd", ".rst", ".txt"];
    let exclude_dirs = ["node_modules", ".git", "_build", "dist", "vendor"];

    let files: Vec<String> = WalkDir::new(root)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry
                .inspect_err(|e| eprintln!("Error: Skipping path due to I/O error: {e}"))
                .ok()?;
            let path = entry.path();

            if exclude_dirs.iter().any(|excl| {
                path.components()
                    .any(|c| c.as_os_str().to_string_lossy().contains(excl))
            }) {
                return None;
            }

            if !path.is_file() {
                return None;
            }

            let ext_str = path
                .extension()
                .map(|ext| format!(".{}", ext.to_string_lossy()))?;
            if !extensions.contains(&ext_str.as_str()) {
                return None;
            }

            path.strip_prefix(root)
                .ok()
                .map(|p| p.to_string_lossy().to_string())
        })
        .collect();

    Ok(files)
}
