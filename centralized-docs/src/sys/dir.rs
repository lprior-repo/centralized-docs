use anyhow::Result;
use std::path::Path;

/// Validate output path is a directory or can be created
pub fn validate_output_path(path: &Path) -> Result<()> {
    if path.exists() {
        if !path.is_dir() {
            anyhow::bail!(
                "Output path must be a directory, but got: {}",
                path.display()
            );
        }

        // Check write permission on existing directory
        check_write_permission(path)?;
    } else {
        let parent = path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid output path: {}", path.display()))?;

        if !parent.exists() {
            anyhow::bail!("Parent directory does not exist: {}", parent.display());
        }

        if !parent.is_dir() {
            anyhow::bail!("Parent path is not a directory: {}", parent.display());
        }

        // Check write permission on parent directory (where we'll create the new dir)
        check_write_permission(parent)?;
    }

    Ok(())
}

/// Check if we have write permission to a directory
/// Attempts to create a temporary file to verify write access
fn check_write_permission(dir: &Path) -> Result<()> {
    // Try to create a temporary file to verify write access
    // Using .permission_check.tmp as a unique name unlikely to conflict
    let test_file = dir.join(".permission_check.tmp");

    match std::fs::write(&test_file, b"") {
        Ok(_) => {
            // Clean up the test file
            if let Err(err) = std::fs::remove_file(&test_file) {
                eprintln!("Warning: cleanup failed: {err}");
            }
            Ok(())
        }
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            anyhow::bail!(
                "Permission denied: cannot write to output directory '{}'\n  \
                 Hint: Check directory permissions or run with appropriate access",
                dir.display()
            );
        }
        Err(e) => {
            // Other errors (e.g., read-only filesystem) - still report but with context
            anyhow::bail!(
                "Cannot write to output directory '{}': {}\n  \
                 Hint: Check if the directory exists and you have write access",
                dir.display(),
                e
            );
        }
    }
}
