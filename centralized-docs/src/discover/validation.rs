use anyhow::Result;

use super::DiscoveryFile;

/// Validate discovery results and emit appropriate warnings/errors.
///
/// Extracted from the walker to keep file sizes under 300 lines.
/// Returns `Ok(())` if results are acceptable, `Err` otherwise.
pub(super) fn validate_discovery_results(
    files: &[DiscoveryFile],
    skipped_large: usize,
    skipped_empty: usize,
    skipped_broken_symlink: usize,
    skipped_io_error: usize,
    permission_denied_files: &[String],
    max_file_bytes: u64,
) -> Result<()> {
    // Error if no files found but there were I/O errors
    if files.is_empty() && skipped_io_error > 0 {
        anyhow::bail!(
            "No indexable files found after filtering (skipped {skipped_io_error} file(s) due to I/O errors). \
             Check file permissions: ensure source files are readable."
        );
    }

    if files.is_empty() && skipped_large > 0 {
        anyhow::bail!(
            "No indexable files found after filtering (skipped {skipped_large} oversized, {skipped_empty} empty, and {skipped_broken_symlink} broken symlink files). \
             Maximum supported file size is {max_file_bytes} bytes."
        );
    }

    // Broken symlinks cause non-zero exit code
    if skipped_broken_symlink > 0 {
        anyhow::bail!(
            "Found {skipped_broken_symlink} broken symlink(s) in source directory. \
             Please fix or remove broken symlinks before indexing."
        );
    }

    // Emit warning summary for I/O errors
    if skipped_io_error > 0 {
        eprintln!(
            "Warning: Skipped {skipped_io_error} path(s) due to I/O errors (e.g., permission denied). \
            Some files may not have been processed."
        );
    }

    // Handle permission denied files
    if !permission_denied_files.is_empty() {
        let file_list = permission_denied_files.join(", ");
        if files.is_empty() {
            anyhow::bail!(
                "Error: Cannot read {} file(s) due to permission denied: {}. \
                 Please check file permissions with 'chmod +r' or remove unreadable files.",
                permission_denied_files.len(),
                file_list
            );
        }
        eprintln!(
            "Warning: Cannot read {} file(s) due to permission denied: {}. \
             These files will be skipped. To include them, run 'chmod +r' on these files.",
            permission_denied_files.len(),
            file_list
        );
    }

    Ok(())
}
