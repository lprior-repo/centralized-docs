//! Pure cache operations for chunking: config hashing, cache keys, partitioning, and file writing.

use super::error::ChunkReuseError;
use super::types::{escape_frontmatter, Chunk};
use crate::analyze::Analysis;
use crate::cache::{composite_hash, CacheType, ContentHash, DocCache};
use crate::types::bounded_chunk_name;
use anyhow::Result;
use std::fmt::Write;
use std::fs;
use std::path::Path;
use tap::Pipe;

/// Compute the chunker configuration hash.
///
/// Pure function. Returns `ContentHash` that captures all parameters
/// affecting chunk output. If any parameter changes, the hash changes
/// and all chunk cache entries are invalidated.
#[must_use]
pub fn compute_chunker_config_hash(max_document_bytes: u64) -> ContentHash {
    composite_hash(&[max_document_bytes.to_string().as_bytes()])
}

/// Compute the cache key for a single file's chunk output.
///
/// Pure function: `SHA-256(source_path_bytes || content_bytes || config_hash_bytes)`.
#[must_use]
pub fn chunk_cache_key(source_path: &str, content: &str, config_hash: &ContentHash) -> ContentHash {
    composite_hash(&[
        source_path.as_bytes(),
        content.as_bytes(),
        config_hash.as_bytes(),
    ])
}

/// Partition analyses into (unchanged, changed) groups by checking cache.
///
/// For each analysis, computes the chunk cache key and probes the cache.
/// Cache read errors are treated as cache misses (file goes to `changed`).
#[allow(clippy::type_complexity)]
pub fn partition_by_cache_status(
    analyses: &[Analysis],
    cache: &DocCache,
    config_hash: &ContentHash,
) -> Result<(Vec<(Analysis, Vec<Chunk>)>, Vec<Analysis>)> {
    let initial: (Vec<(Analysis, Vec<Chunk>)>, Vec<Analysis>) = (Vec::new(), Vec::new());

    analyses
        .iter()
        .fold(initial, |(mut unchanged, mut changed), analysis| {
            let key = chunk_cache_key(&analysis.source_path, &analysis.content, config_hash);

            match cache.get::<Vec<Chunk>>(CacheType::Chunk, key.as_bytes()) {
                Ok(Some(cached_chunks)) => {
                    unchanged.push((analysis.clone(), cached_chunks));
                }
                Ok(None) => {
                    changed.push(analysis.clone());
                }
                Err(e) => {
                    // Non-fatal: cache read failure -> treat as changed
                    eprintln!(
                        "Warning: cache read failed for key {key}, downgrading to re-chunk: {e}"
                    );
                    changed.push(analysis.clone());
                }
            }
            (unchanged, changed)
        })
        .pipe(Result::Ok)
}

/// Write a single chunk's `.md` file to the chunks directory.
///
/// Creates `output_dir/chunks/<chunk_filename>.md` with YAML frontmatter
/// and chunk content.
///
/// # Errors
///
/// Returns `ChunkWriteFailed` if disk write fails.
pub fn write_chunk_file(chunk: &Chunk, chunks_dir: &Path) -> Result<()> {
    let level_suffix = match chunk.chunk_level {
        contextual_chunker::ChunkLevel::Summary => "summary",
        contextual_chunker::ChunkLevel::Standard => "standard",
        contextual_chunker::ChunkLevel::Detailed => "detailed",
    };

    let chunk_filename = bounded_chunk_name(&chunk.chunk_id.replace(['/', '#'], "-"), level_suffix);
    let chunk_file = chunks_dir.join(&chunk_filename);

    let mut frontmatter = format!(
        "---\ndoc_id: {}\nchunk_id: {}\nchunk_level: {}\nchunk_type: {}\n",
        chunk.doc_id, chunk.chunk_id, level_suffix, chunk.chunk_type,
    );

    if let Some(ref heading) = chunk.heading {
        let _ = writeln!(frontmatter, "heading: {}", escape_frontmatter(heading));
    }

    let _ = writeln!(
        frontmatter,
        "token_count: {}\nsummary: {}",
        chunk.token_count,
        escape_frontmatter(&chunk.summary)
    );
    let _ = writeln!(frontmatter, "---");

    let content = format!("{}\n{}", frontmatter, chunk.content);

    fs::write(&chunk_file, content).map_err(|e| {
        anyhow::anyhow!(ChunkReuseError::ChunkWriteFailed {
            path: chunk_file.clone(),
            source: e,
        })
    })
}
