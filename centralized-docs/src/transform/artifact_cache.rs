//! Transform artifact cache: fingerprinting, loading, storing, and cached orchestration.

use super::pipeline::transform_to_content;
use super::types::{
    TransformArtifact, TransformArtifactError, TransformArtifactKey, TransformResult,
};
use crate::analyze::Analysis;
use crate::assign::IdMapping;
use crate::cache::{ContentHash, DocCache};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Compute a deterministic fingerprint of the `link_map` for cache invalidation.
pub fn compute_link_map_fingerprint(
    link_map: &HashMap<String, IdMapping>,
) -> std::result::Result<ContentHash, TransformArtifactError> {
    let mut sorted_entries: Vec<(&String, &IdMapping)> = link_map.iter().collect();
    sorted_entries.sort_by_key(|(k, _)| *k);
    let serialized = serde_json::to_string(&sorted_entries).map_err(|e| {
        TransformArtifactError::LinkMapFingerprintFailed {
            message: e.to_string(),
        }
    })?;
    Ok(ContentHash::compute(serialized.as_bytes()))
}

/// Attempt to load a cached transform artifact for a single source path.
pub fn load_cached_artifact(
    cache: &DocCache,
    source_path: &str,
    content_hash: &ContentHash,
    link_map_fingerprint: &ContentHash,
) -> std::result::Result<Option<TransformArtifact>, TransformArtifactError> {
    let key = TransformArtifactKey::compute(source_path, content_hash, link_map_fingerprint);
    let cache_result: anyhow::Result<Option<TransformArtifact>> =
        cache.get_transform::<TransformArtifact>(key.as_bytes());
    match cache_result {
        Ok(Some(artifact)) => Ok(Some(artifact)),
        Ok(None) => Ok(None),
        Err(e) => {
            let msg = format!("{e}");
            let is_deser_err = msg.contains("expected")
                || msg.contains("invalid type")
                || msg.contains("missing field")
                || msg.contains("invalid value")
                || msg.contains("data did not match");
            if is_deser_err {
                Err(TransformArtifactError::DeserializationFailed {
                    source_path: source_path.to_string(),
                    message: msg,
                })
            } else {
                Err(TransformArtifactError::CacheReadFailed {
                    source_path: source_path.to_string(),
                    message: msg,
                })
            }
        }
    }
}

/// Persist a transform artifact to cache.
pub fn store_artifact(
    cache: &DocCache,
    artifact: &TransformArtifact,
    link_map_fingerprint: &ContentHash,
) -> std::result::Result<(), TransformArtifactError> {
    let key = TransformArtifactKey::compute(
        &artifact.source_path,
        &artifact.content_hash,
        link_map_fingerprint,
    );
    cache
        .put_transform(key.as_bytes(), artifact)
        .map_err(
            |e: anyhow::Error| TransformArtifactError::CacheWriteFailed {
                source_path: artifact.source_path.clone(),
                message: e.to_string(),
            },
        )
}

/// Write a cached artifact's markdown to the output directory.
pub fn write_artifact_to_output(
    artifact: &TransformArtifact,
    link_map: &HashMap<String, IdMapping>,
    docs_dir: &Path,
) -> std::result::Result<(), TransformArtifactError> {
    if artifact.transformed_markdown.is_empty() {
        return Err(TransformArtifactError::OutputWriteFailed {
            source_path: artifact.source_path.clone(),
            message: "precondition violated: transformed_markdown must be non-empty".to_string(),
        });
    }
    let mapping = link_map.get(&artifact.source_path).ok_or_else(|| {
        TransformArtifactError::MissingIdMapping {
            source_path: artifact.source_path.clone(),
        }
    })?;
    fs::create_dir_all(docs_dir).map_err(|e| TransformArtifactError::OutputWriteFailed {
        source_path: artifact.source_path.clone(),
        message: format!("failed to create docs directory: {e}"),
    })?;
    let output_file = docs_dir.join(&mapping.filename);
    fs::write(&output_file, &artifact.transformed_markdown).map_err(|e| {
        TransformArtifactError::OutputWriteFailed {
            source_path: artifact.source_path.clone(),
            message: e.to_string(),
        }
    })
}

/// Transform all analyses with caching support.
pub fn transform_all_cached(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping>,
    output_dir: &Path,
    cache: &DocCache,
) -> std::result::Result<TransformResult, TransformArtifactError> {
    if analyses.is_empty() {
        return Ok(TransformResult {
            success_count: 0,
            total_count: 0,
            error_count: 0,
            errors: vec![],
        });
    }

    let first_invalid = analyses.iter().find(|a| a.source_path.is_empty());
    if first_invalid.is_some() {
        return Err(TransformArtifactError::EmptySourcePath);
    }

    let first_missing = analyses
        .iter()
        .find(|a| !link_map.contains_key(&a.source_path));
    if let Some(missing) = first_missing {
        return Err(TransformArtifactError::MissingIdMapping {
            source_path: missing.source_path.clone(),
        });
    }

    let link_map_fp = compute_link_map_fingerprint(link_map)?;
    let docs_dir = output_dir.join("docs");
    fs::create_dir_all(&docs_dir).map_err(|e| TransformArtifactError::OutputWriteFailed {
        source_path: String::new(),
        message: format!("failed to create docs directory: {e}"),
    })?;

    let filename_map: HashMap<String, &IdMapping> = link_map
        .iter()
        .filter_map(|(src_path, mapping)| {
            Path::new(src_path)
                .file_name()
                .and_then(|n| n.to_str())
                .map(|name| (name.to_string(), mapping))
        })
        .collect();

    let results: std::result::Result<Vec<()>, TransformArtifactError> = analyses
        .iter()
        .map(|analysis| {
            process_single_cached(
                analysis,
                link_map,
                &filename_map,
                &docs_dir,
                cache,
                &link_map_fp,
            )
        })
        .collect();

    results.map(|oks| TransformResult {
        success_count: oks.len(),
        total_count: analyses.len(),
        error_count: 0,
        errors: vec![],
    })
}

/// Process a single analysis through the cached transform pipeline.
pub fn process_single_cached(
    analysis: &Analysis,
    link_map: &HashMap<String, IdMapping>,
    filename_map: &HashMap<String, &IdMapping>,
    docs_dir: &Path,
    cache: &DocCache,
    link_map_fp: &ContentHash,
) -> std::result::Result<(), TransformArtifactError> {
    let content_hash = ContentHash::compute(analysis.content.as_bytes());
    let cached = load_cached_artifact(cache, &analysis.source_path, &content_hash, link_map_fp)?;

    if let Some(artifact) = cached {
        write_artifact_to_output(&artifact, link_map, docs_dir)
    } else {
        let mapping = link_map.get(&analysis.source_path).ok_or_else(|| {
            TransformArtifactError::MissingIdMapping {
                source_path: analysis.source_path.clone(),
            }
        })?;
        let transformed = transform_to_content(analysis, mapping, link_map, filename_map);
        let artifact = TransformArtifact {
            source_path: analysis.source_path.clone(),
            content_hash,
            link_map_fingerprint: *link_map_fp,
            transformed_markdown: transformed,
        };
        store_artifact(cache, &artifact, link_map_fp)?;
        write_artifact_to_output(&artifact, link_map, docs_dir)
    }
}
