use std::path::Path;

use anyhow::{Context, Result};
use rayon::prelude::*;

use crate::cache::{composite_hash, CacheType, DocCache};
use crate::config::CategoryConfig;
use crate::discover::DiscoveryFile;

use super::types::{Analysis, AnalyzeResult, FailedFile};

pub fn analyze_files(
    files: &[DiscoveryFile],
    source_dir: &Path,
    category_config_path: Option<&Path>,
) -> Result<AnalyzeResult> {
    let config = load_category_config(category_config_path)?;
    let input_count = files.len();

    let (analyses, failed_files): (Vec<_>, Vec<_>) = files
        .par_iter()
        .map(|file| {
            let file_path = source_dir.join(&file.source_path);
            super::category::analyze_single_file(&file.source_path, &file_path, config.as_ref())
                .map_err(|e| FailedFile {
                    source_path: file.source_path.clone(),
                    error: e.to_string(),
                })
        })
        .partition(Result::is_ok);

    let analyses: Vec<_> = analyses.into_iter().filter_map(Result::ok).collect();
    let failed_files: Vec<_> = failed_files.into_iter().filter_map(Result::err).collect();

    if input_count > 0 && analyses.is_empty() {
        let error_summary = failed_files
            .iter()
            .map(|f| format!("{}: {}", f.source_path, f.error))
            .collect::<Vec<_>>()
            .join("; ");
        anyhow::bail!(
            "Failed to analyze any of the {input_count} discovered file(s). \
            Check file permissions, encoding (files must be valid UTF-8), \
            and that files are not corrupted. Errors: {error_summary}"
        );
    }

    Ok(AnalyzeResult {
        analyses,
        failed_files,
        total_discovered: input_count,
    })
}

/// Cached version of `analyze_files`.
///
/// For each file, computes `SHA-256(source_path + file_bytes + config_hash)`
/// and checks the cache. Cache hits skip re-parsing entirely.
///
/// Returns `(AnalyzeResult, u64)` where the u64 is the cache hit count.
pub fn analyze_files_cached(
    files: &[DiscoveryFile],
    source_dir: &Path,
    category_config_path: Option<&Path>,
    cache: &DocCache,
) -> Result<(AnalyzeResult, u64)> {
    let config = load_category_config(category_config_path)?;
    let config_hash = compute_config_hash(category_config_path);
    let input_count = files.len();

    let (analyses, failed_files, hits): (Vec<_>, Vec<_>, u64) = files
        .par_iter()
        .map(|file| {
            let file_path = source_dir.join(&file.source_path);

            // Read file bytes for cache key + cache miss path
            let file_bytes = std::fs::read(&file_path).map_err(|e| FailedFile {
                source_path: file.source_path.clone(),
                error: e.to_string(),
            })?;

            let cache_key = composite_hash(&[
                file.source_path.as_bytes(),
                &file_bytes,
                config_hash.as_bytes(),
            ]);

            // Check cache first
            if let Some(cached) = cache
                .get::<Analysis>(CacheType::Analysis, cache_key.as_bytes())
                .map_err(|e| FailedFile {
                    source_path: file.source_path.clone(),
                    error: e.to_string(),
                })?
            {
                return Ok((cached, true));
            }

            // Cache miss — run analysis
            let analysis = super::category::analyze_single_file(
                &file.source_path,
                &file_path,
                config.as_ref(),
            )
            .map_err(|e| FailedFile {
                source_path: file.source_path.clone(),
                error: e.to_string(),
            })?;

            // Store in cache (best-effort — don't fail the pipeline on cache write errors)
            let _ = cache.put(CacheType::Analysis, cache_key.as_bytes(), &analysis);

            Ok((analysis, false))
        })
        .fold(
            || (Vec::new(), Vec::new(), 0u64),
            |(mut ok, mut err, mut hits), res: Result<(Analysis, bool), FailedFile>| {
                match res {
                    Ok((analysis, was_cached)) => {
                        if was_cached {
                            hits = hits.saturating_add(1);
                        }
                        ok.push(analysis);
                    }
                    Err(e) => err.push(e),
                }
                (ok, err, hits)
            },
        )
        .reduce(
            || (Vec::new(), Vec::new(), 0u64),
            |(mut ok1, mut err1, mut hits1), (ok2, err2, hits2)| {
                ok1.extend(ok2);
                err1.extend(err2);
                hits1 = hits1.saturating_add(hits2);
                (ok1, err1, hits1)
            },
        );

    if input_count > 0 && analyses.is_empty() {
        let error_summary = failed_files
            .iter()
            .map(|f| format!("{}: {}", f.source_path, f.error))
            .collect::<Vec<_>>()
            .join("; ");
        anyhow::bail!(
            "Failed to analyze any of the {input_count} discovered file(s). \
            Check file permissions, encoding (files must be valid UTF-8), \
            and that files are not corrupted. Errors: {error_summary}"
        );
    }

    Ok((
        AnalyzeResult {
            analyses,
            failed_files,
            total_discovered: input_count,
        },
        hits,
    ))
}

/// Compute a deterministic hash of the category config file contents (or empty if none).
///
/// Delegates to [`crate::diff::compute_config_hash`] (promoted to public API in cdocs-2rt).
fn compute_config_hash(category_config_path: Option<&Path>) -> crate::cache::ContentHash {
    crate::diff::compute_config_hash(category_config_path)
}

fn load_category_config(category_config_path: Option<&Path>) -> Result<Option<CategoryConfig>> {
    if let Some(path) = category_config_path {
        Some(
            CategoryConfig::load_from_file(path).with_context(|| {
                format!("Failed to load category config from '{}'", path.display())
            })?,
        )
    } else {
        None
    }
    .map_or(Ok(None), |c| Ok(Some(c)))
}
