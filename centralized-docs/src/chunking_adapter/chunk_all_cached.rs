//! Cached chunking orchestration — partition, compute, merge, persist.

use super::cache_ops::{chunk_cache_key, partition_by_cache_status, write_chunk_file};
use super::error::ChunkReuseError;
use super::types::{
    analysis_to_document, convert_chunk, create_dir_with_context, fallback_doc_id, Chunk,
    ChunksResult,
};
use crate::analyze::Analysis;
use crate::assign::IdMapping;
use crate::cache::{CacheType, DocCache};
use anyhow::Result;
use std::collections::HashMap;
use std::io;
use std::path::Path;

/// Compute fresh chunks for changed analyses, group by analysis, and cache results.
///
/// Returns `Vec<(Analysis, Vec<Chunk>)>` in the same order as `changed`.
/// Cache write failures are non-fatal (logged).
fn compute_and_cache_changed_chunks(
    changed: Vec<Analysis>,
    link_map: &HashMap<String, IdMapping>,
    cache: &DocCache,
    chunker_config_hash: &crate::cache::ContentHash,
) -> Result<Vec<(Analysis, Vec<Chunk>)>> {
    let documents: Vec<_> = changed
        .iter()
        .map(|a| analysis_to_document(a, link_map))
        .collect();

    let result = contextual_chunker::chunk_all(&documents).map_err(|e| {
        let source_path = changed
            .first()
            .map_or_else(|| "unknown".to_string(), |a| a.source_path.clone());
        anyhow::anyhow!(ChunkReuseError::ChunkerFailed {
            source_path,
            reason: e.to_string(),
        })
    })?;

    let converted: Vec<Chunk> = result.chunks.into_iter().map(convert_chunk).collect();

    let doc_id_to_idx: Vec<(String, usize)> = changed
        .iter()
        .enumerate()
        .map(|(idx, a)| {
            let doc_id = link_map
                .get(&a.source_path)
                .map_or_else(|| fallback_doc_id(a), |m| m.id.clone());
            (doc_id, idx)
        })
        .collect();

    let mut grouped: Vec<(Analysis, Vec<Chunk>)> =
        changed.into_iter().map(|a| (a, Vec::new())).collect();

    for chunk in converted {
        if let Some(&(_, idx)) = doc_id_to_idx
            .iter()
            .find(|(doc_id, _)| *doc_id == chunk.doc_id)
        {
            grouped[idx].1.push(chunk);
        }
    }

    for (analysis, chunks) in &grouped {
        if chunks.is_empty() {
            continue;
        }
        let key = chunk_cache_key(
            &analysis.source_path,
            &analysis.content,
            chunker_config_hash,
        );
        if let Err(e) = cache.put(CacheType::Chunk, key.as_bytes(), chunks) {
            eprintln!("Warning: cache write failed for key {key} (non-fatal): {e}");
        }
    }

    Ok(grouped)
}

/// Count chunks by level.
fn count_chunks_by_level(chunks: &[Chunk]) -> (usize, usize, usize) {
    let summary = chunks
        .iter()
        .filter(|c| matches!(c.chunk_level, contextual_chunker::ChunkLevel::Summary))
        .count();
    let standard = chunks
        .iter()
        .filter(|c| matches!(c.chunk_level, contextual_chunker::ChunkLevel::Standard))
        .count();
    let detailed = chunks
        .iter()
        .filter(|c| matches!(c.chunk_level, contextual_chunker::ChunkLevel::Detailed))
        .count();
    (summary, standard, detailed)
}

/// Merge unchanged and changed chunk groups in analysis order.
fn merge_chunks_in_order(
    analyses: &[Analysis],
    unchanged: &[(Analysis, Vec<Chunk>)],
    changed_chunks: &[(Analysis, Vec<Chunk>)],
) -> Vec<Chunk> {
    analyses
        .iter()
        .flat_map(|analysis| {
            let cached = unchanged
                .iter()
                .find(|(a, _)| a.source_path == analysis.source_path)
                .map(|(_, chunks)| chunks.clone());

            let fresh = changed_chunks
                .iter()
                .find(|(a, _)| a.source_path == analysis.source_path)
                .map(|(_, chunks)| chunks.clone());

            cached.or(fresh).into_iter().flatten()
        })
        .collect()
}

/// Cached variant of `chunk_all`.
///
/// Partitions analyses into "unchanged" (cache hit) and "changed" (cache miss)
/// groups, loads archived chunks for unchanged files, computes fresh chunks
/// for changed files, stores them in cache, merges, and writes all to disk.
///
/// # Errors
///
/// Returns `anyhow::Error` for fatal conditions (see `ChunkReuseError`).
/// Cache I/O errors are non-fatal and result in re-chunking.
#[allow(clippy::implicit_hasher)]
pub fn chunk_all_cached(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping>,
    output_dir: &Path,
    max_document_bytes: u64,
    cache: &DocCache,
    chunker_config_hash: crate::cache::ContentHash,
) -> Result<ChunksResult> {
    let chunks_dir = output_dir.join("chunks");
    create_dir_with_context(&chunks_dir, "chunks").map_err(|e| {
        if let Some(io_err) = e.downcast_ref::<io::Error>() {
            anyhow::anyhow!(ChunkReuseError::ChunksDirCreationFailed {
                path: chunks_dir.clone(),
                source: io::Error::new(io_err.kind(), io_err.to_string()),
            })
        } else if e.to_string().contains("Permission denied") || e.to_string().contains("chunks") {
            anyhow::anyhow!(ChunkReuseError::ChunksDirCreationFailed {
                path: chunks_dir.clone(),
                source: io::Error::new(io::ErrorKind::PermissionDenied, e.to_string()),
            })
        } else {
            e
        }
    })?;

    if analyses.is_empty() {
        return Ok(ChunksResult {
            total_chunks: 0,
            document_count: 0,
            chunks_metadata: Vec::new(),
            summary_chunks: 0,
            standard_chunks: 0,
            detailed_chunks: 0,
        });
    }

    analyses.iter().try_for_each(|analysis| {
        let content_size = u64::try_from(analysis.content.len()).map_or(u64::MAX, |v| v);
        if content_size > max_document_bytes {
            return Err(anyhow::anyhow!(ChunkReuseError::DocumentExceedsSizeLimit {
                source_path: analysis.source_path.clone(),
                content_size,
                max_bytes: max_document_bytes,
            }));
        }
        Ok(())
    })?;

    let (unchanged, changed) = partition_by_cache_status(analyses, cache, &chunker_config_hash)?;

    let changed_chunks = if changed.is_empty() {
        Vec::new()
    } else {
        compute_and_cache_changed_chunks(changed, link_map, cache, &chunker_config_hash)?
    };

    let all_chunks = merge_chunks_in_order(analyses, &unchanged, &changed_chunks);

    all_chunks
        .iter()
        .try_for_each(|chunk| write_chunk_file(chunk, &chunks_dir))?;

    let (summary_chunks, standard_chunks, detailed_chunks) = count_chunks_by_level(&all_chunks);

    Ok(ChunksResult {
        total_chunks: summary_chunks
            .saturating_add(standard_chunks)
            .saturating_add(detailed_chunks),
        document_count: analyses.len(),
        chunks_metadata: all_chunks,
        summary_chunks,
        standard_chunks,
        detailed_chunks,
    })
}
