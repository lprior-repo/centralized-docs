//! Adapter layer between `ctd` and `contextual-chunker`
//!
//! This module provides conversion functions between `ctd`'s types
//! and `contextual-chunker`'s types, enabling clean separation of concerns while
//! maintaining all `ctd`-specific functionality.

use crate::analyze::Analysis;
use crate::assign::IdMapping;
use crate::cache::{composite_hash, CacheType, ContentHash, DocCache};
use crate::types::Slug;
use anyhow::Result;
use contextual_chunker::{self, ChunkType, Document};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tap::Pipe;

/// Create directory with improved error context for permission issues
fn create_dir_with_context(path: &Path, context: &str) -> Result<()> {
    fs::create_dir_all(path).map_err(|e| {
        if e.kind() == io::ErrorKind::PermissionDenied {
            anyhow::anyhow!(
                "Permission denied: cannot create {} directory '{}'\n  \
                 Hint: Check directory permissions or run with appropriate access",
                context,
                path.display()
            )
        } else {
            anyhow::anyhow!(
                "Failed to create {} directory '{}': {}",
                context,
                path.display(),
                e
            )
        }
    })
}

/// Extended chunk type for `ctd` with knowledge graph relationships
///
/// This extends `contextual_chunker::Chunk` with `ctd`-specific fields
/// like `related_chunk_ids` for the knowledge graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub chunk_id: String,
    pub doc_id: String,
    pub doc_title: String,
    pub chunk_index: usize,
    pub content: String,
    pub token_count: usize,
    pub heading: Option<String>,
    pub heading_path: Vec<String>,
    pub chunk_type: ChunkType,
    pub previous_chunk_id: Option<String>,
    pub next_chunk_id: Option<String>,
    pub related_chunk_ids: Vec<String>,
    pub summary: String,
    pub chunk_level: contextual_chunker::ChunkLevel,
    pub parent_chunk_id: Option<String>,
    pub child_chunk_ids: Vec<String>,
    /// Context preserved from previous chunk for continuity
    pub context_prefix: Option<String>,
}

/// Extended chunking result for `ctd`
#[derive(Debug)]
pub struct ChunksResult {
    pub total_chunks: usize,
    pub document_count: usize,
    pub chunks_metadata: Vec<Chunk>,
    pub summary_chunks: usize,
    pub standard_chunks: usize,
    pub detailed_chunks: usize,
}

/// Convert Analysis to `contextual_chunker::Document`
///
/// Maps `ctd`'s Analysis type to the simpler Document type
/// used by `contextual-chunker`. Uses `link_map` to get the assigned doc ID,
/// falling back to a deterministic slugified ID if missing.
fn analysis_to_document(analysis: &Analysis, link_map: &HashMap<String, IdMapping>) -> Document {
    let doc_id = link_map
        .get(&analysis.source_path)
        .map_or_else(|| fallback_doc_id(analysis), |m| m.id.clone());

    let title = if analysis.title.is_empty() {
        "Untitled".to_string()
    } else {
        analysis.title.clone()
    };

    Document::new(doc_id, title, analysis.content.to_string())
}

fn fallback_doc_id(analysis: &Analysis) -> String {
    let parts: Vec<&str> = analysis.source_path.split('/').collect();
    let subcategory = parts
        .get(parts.len().saturating_sub(2))
        .map_or_else(|| "general".to_string(), |s| s.to_lowercase());
    let filename_stem = Path::new(&analysis.source_path)
        .file_stem()
        .filter(|s| !s.is_empty())
        .map_or_else(
            || "untitled".to_string(),
            |s| s.to_string_lossy().to_string(),
        );
    let slug = slugify(&filename_stem);

    format!("{}/{}/{}", analysis.category, subcategory, slug)
}

fn slugify(text: &str) -> String {
    let slug = text
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == ' ' {
                c
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-");
    Slug::from_text(&slug).into_string()
}

/// Convert `contextual_chunker::Chunk` to `doc_transformer::Chunk`
///
/// Creates extended chunk with empty `related_chunk_ids` (filled later by graph analysis)
fn convert_chunk(chunk: contextual_chunker::Chunk) -> Chunk {
    Chunk {
        chunk_id: chunk.chunk_id,
        doc_id: chunk.doc_id,
        doc_title: chunk.doc_title,
        chunk_index: chunk.chunk_index,
        content: chunk.content,
        token_count: chunk.token_count,
        heading: chunk.heading,
        heading_path: chunk.heading_path,
        chunk_type: chunk.chunk_type,
        previous_chunk_id: chunk.previous_chunk_id,
        next_chunk_id: chunk.next_chunk_id,
        related_chunk_ids: Vec::new(), // Populated later by knowledge graph
        summary: chunk.summary,
        chunk_level: chunk.chunk_level,
        parent_chunk_id: chunk.parent_chunk_id,
        child_chunk_ids: chunk.child_chunk_ids,
        context_prefix: chunk.context_prefix, // Preserve context from previous chunk
    }
}

/// Convert `contextual_chunker::ChunkingResult` to `doc_transformer::ChunksResult`
fn convert_chunking_result(
    result: contextual_chunker::ChunkingResult,
    document_count: usize,
) -> ChunksResult {
    let chunks_metadata = result.chunks.into_iter().map(convert_chunk).collect();

    ChunksResult {
        total_chunks: result
            .summary_count
            .saturating_add(result.standard_count)
            .saturating_add(result.detailed_count),
        document_count,
        chunks_metadata,
        summary_chunks: result.summary_count,
        standard_chunks: result.standard_count,
        detailed_chunks: result.detailed_count,
    }
}

/// Escape frontmatter values
fn escape_frontmatter(s: &str) -> String {
    s.replace('\n', " ").replace('\"', "\\\"")
}

// ---------------------------------------------------------------------------
// Chunk Reuse Error Taxonomy (cdocs-c34)
// ---------------------------------------------------------------------------

/// Errors specific to the chunk-reuse (cached chunking) pathway.
///
/// Fatal variants are returned via `anyhow::Error`.
/// Non-fatal variants are logged and result in re-chunking.
#[non_exhaustive]
#[derive(Debug)]
pub enum ChunkReuseError {
    /// The chunks output directory could not be created.
    ChunksDirCreationFailed { path: PathBuf, source: io::Error },

    /// A document exceeds the configured byte limit.
    DocumentExceedsSizeLimit {
        source_path: String,
        content_size: u64,
        max_bytes: u64,
    },

    /// The `contextual_chunker` returned an error for a changed file.
    ChunkerFailed { source_path: String, reason: String },

    /// A cached chunk entry could not be deserialized (non-fatal, logged).
    CacheDeserializationFailed { key: ContentHash, reason: String },

    /// Writing a chunk `.md` file to disk failed.
    ChunkWriteFailed { path: PathBuf, source: io::Error },

    /// Cache read I/O error (non-fatal, logged, results in re-chunking).
    CacheReadFailed { key: ContentHash, reason: String },

    /// Cache write failed after fresh chunking (non-fatal, logged).
    CacheWriteFailed { key: ContentHash, reason: String },
}

impl fmt::Display for ChunkReuseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ChunksDirCreationFailed { path, source } => {
                write!(
                    f,
                    "chunks dir creation failed for '{}': {source}",
                    path.display()
                )
            }
            Self::DocumentExceedsSizeLimit {
                source_path,
                content_size,
                max_bytes,
            } => {
                write!(
                    f,
                    "document '{source_path}' ({content_size} bytes) exceeds maximum document size limit ({max_bytes} bytes)"
                )
            }
            Self::ChunkerFailed {
                source_path,
                reason,
            } => {
                write!(f, "chunker failed for '{source_path}': {reason}")
            }
            Self::CacheDeserializationFailed { key, reason } => {
                write!(f, "cache deserialization failed for key {key}: {reason}")
            }
            Self::ChunkWriteFailed { path, source } => {
                write!(f, "chunk write failed for '{}': {source}", path.display())
            }
            Self::CacheReadFailed { key, reason } => {
                write!(f, "cache read failed for key {key}: {reason}")
            }
            Self::CacheWriteFailed { key, reason } => {
                write!(f, "cache write failed for key {key}: {reason}")
            }
        }
    }
}

impl std::error::Error for ChunkReuseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ChunksDirCreationFailed { source, .. }
            | Self::ChunkWriteFailed { source, .. } => Some(source),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Pure Functions (Calc Layer) — cdocs-c34
// ---------------------------------------------------------------------------

/// Compute the chunker configuration hash.
///
/// Pure function. Returns `ContentHash` that captures all parameters
/// affecting chunk output. If any parameter changes, the hash changes
/// and all chunk cache entries are invalidated.
///
/// Currently includes `max_document_bytes` as a string representation.
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

// ---------------------------------------------------------------------------
// Partition Logic — cdocs-c34
// ---------------------------------------------------------------------------

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
                    // Non-fatal: cache read failure → treat as changed
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

// ---------------------------------------------------------------------------
// Chunk File Writing — cdocs-c34
// ---------------------------------------------------------------------------

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

    let chunk_filename = format!(
        "{}-{}.md",
        chunk.chunk_id.replace(['/', '#'], "-"),
        level_suffix
    );
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

// ---------------------------------------------------------------------------
// Cached Chunking Orchestration — cdocs-c34
// ---------------------------------------------------------------------------

/// Compute fresh chunks for changed analyses, group by analysis, and cache results.
///
/// Returns `Vec<(Analysis, Vec<Chunk>)>` in the same order as `changed`.
/// Cache write failures are non-fatal (logged).
fn compute_and_cache_changed_chunks(
    changed: Vec<Analysis>,
    link_map: &HashMap<String, IdMapping>,
    cache: &DocCache,
    chunker_config_hash: &ContentHash,
) -> Result<Vec<(Analysis, Vec<Chunk>)>> {
    let documents: Vec<Document> = changed
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

    // Build doc_id → index mapping for changed analyses
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

    // Initialize grouped with empty chunk vectors for each changed analysis
    let mut grouped: Vec<(Analysis, Vec<Chunk>)> =
        changed.into_iter().map(|a| (a, Vec::new())).collect();

    // Distribute chunks to their corresponding analysis by doc_id
    for chunk in converted {
        if let Some(&(_, idx)) = doc_id_to_idx
            .iter()
            .find(|(doc_id, _)| *doc_id == chunk.doc_id)
        {
            grouped[idx].1.push(chunk);
        }
    }

    // Store freshly computed chunks in cache (B17, B23 — non-fatal on failure)
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
    chunker_config_hash: ContentHash,
) -> Result<ChunksResult> {
    // Create chunks directory (also handles empty analyses case for B32)
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

    // Early return for empty analyses (B32)
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

    // Check document sizes (B14, B31 — strict > not >=)
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

    // Partition into cached and changed
    let (unchanged, changed) = partition_by_cache_status(analyses, cache, &chunker_config_hash)?;

    // Compute fresh chunks for changed files (B15, B16)
    let changed_chunks = if changed.is_empty() {
        Vec::new()
    } else {
        compute_and_cache_changed_chunks(changed, link_map, cache, &chunker_config_hash)?
    };

    // Merge in analysis order (B20, POST-07)
    let all_chunks = merge_chunks_in_order(analyses, &unchanged, &changed_chunks);

    // Write ALL chunks to disk (B18, POST-04)
    all_chunks
        .iter()
        .try_for_each(|chunk| write_chunk_file(chunk, &chunks_dir))?;

    // Compute counters (B19, POST-05)
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

/// Chunk all analyses using `contextual-chunker`
///
/// This is the main entry point for chunking in `ctd`.
/// It converts Analysis types to Documents, calls `contextual-chunker`,
/// converts the results back to `ctd` types, and writes
/// chunk files to disk.
///
/// # Errors
///
/// Returns an error if:
/// - The chunks directory cannot be created
/// - Chunking fails in contextual-chunker
/// - Writing chunk files fails
/// - A document exceeds `max_document_bytes`
#[allow(clippy::implicit_hasher)]
pub fn chunk_all(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping>,
    output_dir: &Path,
    max_document_bytes: u64,
) -> Result<ChunksResult> {
    // Create chunks directory
    let chunks_dir = output_dir.join("chunks");
    create_dir_with_context(&chunks_dir, "chunks")?;

    // Calculate warning threshold (50% of max)
    let warning_threshold = max_document_bytes / 2;

    // Check document sizes and warn/fail for oversized content
    analyses.iter().try_for_each(|analysis| {
        let content_size = analysis.content.len() as u64;
        if content_size > max_document_bytes {
            anyhow::bail!(
                "Document '{}' ({} bytes) exceeds maximum document size limit ({} bytes). \
                 Please split the document or increase --max-document-bytes.",
                analysis.source_path,
                content_size,
                max_document_bytes
            );
        }
        if content_size > warning_threshold {
            eprintln!(
                "Warning: Large document '{}' ({} bytes) may take significant time to chunk. \
                 Consider splitting documents larger than {} bytes for better performance.",
                analysis.source_path, content_size, warning_threshold
            );
        }
        Ok(())
    })?;

    // Convert analyses to documents
    let documents: Vec<Document> = analyses
        .iter()
        .map(|a| analysis_to_document(a, link_map))
        .collect();

    // Call contextual-chunker
    let result = contextual_chunker::chunk_all(&documents)?;

    // Convert result back to ctd types
    let chunks_result = convert_chunking_result(result, analyses.len());

    // Write chunks to disk
    chunks_result.chunks_metadata.iter().try_for_each(|chunk| {
        let level_suffix = match chunk.chunk_level {
            contextual_chunker::ChunkLevel::Summary => "summary",
            contextual_chunker::ChunkLevel::Standard => "standard",
            contextual_chunker::ChunkLevel::Detailed => "detailed",
        };

        let chunk_filename = format!(
            "{}-{}.md",
            chunk.chunk_id.replace(['/', '#'], "-"),
            level_suffix
        );
        let chunk_file = chunks_dir.join(&chunk_filename);

        let frontmatter = format!(
            "---\ndoc_id: {}\nchunk_id: {}\nchunk_level: {}\nchunk_type: {}\nheading: {}\ntoken_count: {}\nsummary: {}\n---\n",
            chunk.doc_id,
            chunk.chunk_id,
            level_suffix,
            chunk.chunk_type,
            chunk.heading.as_ref().map_or(&"Introduction".to_string(), |v| v),
            chunk.token_count,
            escape_frontmatter(&chunk.summary)
        );

        let content = format!("{}\n{}", frontmatter, chunk.content);
        fs::write(chunk_file, content)
    })?;

    Ok(chunks_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analysis_to_document_with_link_map() {
        let analysis = Analysis {
            source_path: "concept/general/test.md".to_string(),
            title: "Test Document".to_string(),
            content: "## Section\nContent here".into(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: "Content here".to_string(),
            word_count: 2,
            has_code: false,
            has_tables: false,
            category: "concept".to_string(),
        };

        let mut link_map = HashMap::new();
        link_map.insert(
            "concept/general/test.md".to_string(),
            IdMapping {
                id: "concept/general/test".to_string(),
                filename: "concept-general-test.md".to_string(),
                subcategory: "general".to_string(),
                slug: "test".to_string(),
            },
        );

        let doc = analysis_to_document(&analysis, &link_map);

        // Should use link_map entry, not slugified path
        assert_eq!(doc.id, "concept/general/test");
        assert_eq!(doc.title, "Test Document");
        assert_eq!(doc.content, "## Section\nContent here");
    }

    #[test]
    fn test_analysis_to_document_missing_link_map_fallbacks() {
        let analysis = Analysis {
            source_path: "concept/general/test.md".to_string(),
            title: "Test Document".to_string(),
            content: "## Section\nContent here".into(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: "Content here".to_string(),
            word_count: 2,
            has_code: false,
            has_tables: false,
            category: "concept".to_string(),
        };

        let link_map = HashMap::new();

        let doc = analysis_to_document(&analysis, &link_map);

        assert_eq!(doc.id, "concept/general/test");
    }

    #[test]
    fn test_chunk_conversion() {
        let cc_chunk = contextual_chunker::Chunk {
            chunk_id: "test#0".to_string(),
            doc_id: "test".to_string(),
            doc_title: "Test".to_string(),
            chunk_index: 0,
            content: "Content".to_string(),
            context_prefix: Some("Context from previous".to_string()),
            token_count: 10,
            heading: Some("Section".to_string()),
            heading_path: vec!["Test".to_string(), "Section".to_string()],
            chunk_type: contextual_chunker::ChunkType::Prose,
            previous_chunk_id: None,
            next_chunk_id: None,
            summary: "Summary".to_string(),
            chunk_level: contextual_chunker::ChunkLevel::Standard,
            parent_chunk_id: None,
            child_chunk_ids: vec![],
        };

        let chunk = convert_chunk(cc_chunk);

        assert_eq!(chunk.chunk_id, "test#0");
        assert_eq!(chunk.chunk_level, contextual_chunker::ChunkLevel::Standard);
        assert!(chunk.related_chunk_ids.is_empty()); // Populated later by graph
    }

    #[test]
    fn test_convert_chunk_with_navigation() {
        let cc_chunk = contextual_chunker::Chunk {
            chunk_id: "nav#0".to_string(),
            doc_id: "nav-doc".to_string(),
            doc_title: "Nav Doc".to_string(),
            chunk_index: 0,
            content: "Nav content".to_string(),
            context_prefix: None,
            token_count: 5,
            heading: Some("Table of Contents".to_string()),
            heading_path: vec!["Nav Doc".to_string(), "Table of Contents".to_string()],
            chunk_type: contextual_chunker::ChunkType::Code,
            previous_chunk_id: None,
            next_chunk_id: Some("nav#1".to_string()),
            summary: "TOC summary".to_string(),
            chunk_level: contextual_chunker::ChunkLevel::Summary,
            parent_chunk_id: Some("nav-doc".to_string()),
            child_chunk_ids: vec!["nav#1".to_string()],
        };

        let chunk = convert_chunk(cc_chunk);
        assert_eq!(chunk.chunk_type, contextual_chunker::ChunkType::Code);
        assert_eq!(chunk.previous_chunk_id, None);
        assert_eq!(chunk.next_chunk_id.as_deref(), Some("nav#1"));
        assert_eq!(chunk.parent_chunk_id.as_deref(), Some("nav-doc"));
        assert_eq!(chunk.child_chunk_ids.len(), 1);
        assert_eq!(chunk.context_prefix, None);
    }

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("Test Document"), "test-document");
        assert_eq!(slugify("hello-world"), "hello-world");
    }

    #[test]
    fn test_slugify_special_chars() {
        let result = slugify("test@#$%^&*()doc");
        assert!(result.contains("test"));
        assert!(result.contains("doc"));
        assert!(!result.contains('@'));
    }

    #[test]
    fn test_fallback_doc_id() {
        let analysis = Analysis {
            source_path: "concept/general/my-doc.md".to_string(),
            title: "My Doc".to_string(),
            content: "content".into(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: "content".to_string(),
            word_count: 1,
            has_code: false,
            has_tables: false,
            category: "concept".to_string(),
        };

        let link_map = HashMap::new();
        let doc = analysis_to_document(&analysis, &link_map);
        assert_eq!(doc.id, "concept/general/my-doc");
    }

    #[test]
    fn test_fallback_doc_id_shallow_path() {
        let analysis = Analysis {
            source_path: "file.md".to_string(),
            title: "File".to_string(),
            content: "content".into(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: "content".to_string(),
            word_count: 1,
            has_code: false,
            has_tables: false,
            category: "concept".to_string(),
        };

        let link_map = HashMap::new();
        let doc = analysis_to_document(&analysis, &link_map);
        assert!(doc.id.starts_with("concept/"));
    }

    #[test]
    fn test_analysis_to_document_empty_title() {
        let analysis = Analysis {
            source_path: "concept/general/test.md".to_string(),
            title: String::new(),
            content: "content".into(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: "content".to_string(),
            word_count: 1,
            has_code: false,
            has_tables: false,
            category: "concept".to_string(),
        };

        let link_map = HashMap::new();
        let doc = analysis_to_document(&analysis, &link_map);
        assert_eq!(doc.title, "Untitled");
    }

    #[test]
    fn test_escape_frontmatter() {
        assert_eq!(escape_frontmatter("hello"), "hello");
        assert_eq!(escape_frontmatter("line1\nline2"), "line1 line2");
        assert_eq!(escape_frontmatter("say \"hi\""), "say \\\"hi\\\"");
    }

    #[test]
    fn test_convert_chunking_result() {
        let cc_result = contextual_chunker::ChunkingResult {
            chunks: vec![contextual_chunker::Chunk {
                chunk_id: "doc#0".to_string(),
                doc_id: "doc".to_string(),
                doc_title: "Doc".to_string(),
                chunk_index: 0,
                content: "Summary content".to_string(),
                context_prefix: None,
                token_count: 20,
                heading: Some("Intro".to_string()),
                heading_path: vec!["Doc".to_string()],
                chunk_type: contextual_chunker::ChunkType::Prose,
                previous_chunk_id: None,
                next_chunk_id: None,
                summary: "A summary".to_string(),
                chunk_level: contextual_chunker::ChunkLevel::Summary,
                parent_chunk_id: None,
                child_chunk_ids: vec![],
            }],
            summary_count: 1,
            standard_count: 0,
            detailed_count: 0,
        };

        let result = convert_chunking_result(cc_result, 1);
        assert_eq!(result.total_chunks, 1);
        assert_eq!(result.document_count, 1);
        assert_eq!(result.summary_chunks, 1);
        assert_eq!(result.standard_chunks, 0);
        assert_eq!(result.detailed_chunks, 0);
        assert_eq!(result.chunks_metadata.len(), 1);
    }

    // ===================================================================
    // cdocs-c34: Chunk Reuse Tests
    // ===================================================================

    /// Helper: create a minimal Analysis for testing.
    fn make_analysis(source_path: &str, title: &str, content: &str, category: &str) -> Analysis {
        Analysis {
            source_path: source_path.to_string(),
            title: title.to_string(),
            content: content.into(),
            frontmatter: None,
            headings: vec![],
            links: vec![],
            first_paragraph: content.to_string(),
            word_count: content.split_whitespace().count(),
            has_code: false,
            has_tables: false,
            category: category.to_string(),
        }
    }

    /// Helper: create a test link_map entry.
    fn make_link_map(analyses: &[Analysis]) -> HashMap<String, IdMapping> {
        analyses
            .iter()
            .map(|a| {
                let slug = a.source_path.split('/').last().map_or_else(
                    || "untitled".to_string(),
                    |s| s.trim_end_matches(".md").to_string(),
                );
                (
                    a.source_path.clone(),
                    IdMapping {
                        id: format!("{}/{}", a.category, slug),
                        filename: format!("{}-{}.md", a.category, slug),
                        subcategory: "general".to_string(),
                        slug,
                    },
                )
            })
            .collect()
    }

    /// Helper: create a test Chunk.
    fn make_test_chunk(
        chunk_id: &str,
        doc_id: &str,
        chunk_index: usize,
        chunk_level: contextual_chunker::ChunkLevel,
        content: &str,
        heading: Option<&str>,
    ) -> Chunk {
        Chunk {
            chunk_id: chunk_id.to_string(),
            doc_id: doc_id.to_string(),
            doc_title: "Test Doc".to_string(),
            chunk_index,
            content: content.to_string(),
            token_count: content.split_whitespace().count(),
            heading: heading.map(str::to_string),
            heading_path: vec![],
            chunk_type: contextual_chunker::ChunkType::Prose,
            previous_chunk_id: None,
            next_chunk_id: None,
            related_chunk_ids: vec![],
            summary: format!("Summary for {}", chunk_id),
            chunk_level,
            parent_chunk_id: None,
            child_chunk_ids: vec![],
            context_prefix: None,
        }
    }

    // -------------------------------------------------------------------
    // B01: compute_chunker_config_hash determinism
    // -------------------------------------------------------------------
    #[test]
    fn compute_chunker_config_hash_returns_identical_hash_for_same_max_bytes() {
        let hash1 = compute_chunker_config_hash(1_048_576);
        let hash2 = compute_chunker_config_hash(1_048_576);
        assert_eq!(hash1, hash2);
    }

    // -------------------------------------------------------------------
    // B02: compute_chunker_config_hash differentiation
    // -------------------------------------------------------------------
    #[test]
    fn compute_chunker_config_hash_returns_different_hash_when_max_bytes_differs() {
        let hash_a = compute_chunker_config_hash(1_048_576);
        let hash_b = compute_chunker_config_hash(2_097_152);
        assert_ne!(hash_a, hash_b);
    }

    // -------------------------------------------------------------------
    // B03: chunk_cache_key determinism
    // -------------------------------------------------------------------
    #[test]
    fn chunk_cache_key_returns_identical_hash_for_identical_triple() {
        let config_hash = compute_chunker_config_hash(1024);
        let key1 = chunk_cache_key("concept/general/test.md", "file body", &config_hash);
        let key2 = chunk_cache_key("concept/general/test.md", "file body", &config_hash);
        assert_eq!(key1, key2);
    }

    // -------------------------------------------------------------------
    // B04: chunk_cache_key differentiation by component
    // -------------------------------------------------------------------
    #[test]
    fn chunk_cache_key_returns_different_hash_when_source_path_differs() {
        let config_hash = compute_chunker_config_hash(1024);
        let key1 = chunk_cache_key("a.md", "content", &config_hash);
        let key2 = chunk_cache_key("b.md", "content", &config_hash);
        assert_ne!(key1, key2);
    }

    #[test]
    fn chunk_cache_key_returns_different_hash_when_content_differs() {
        let config_hash = compute_chunker_config_hash(1024);
        let key1 = chunk_cache_key("test.md", "body A", &config_hash);
        let key2 = chunk_cache_key("test.md", "body B", &config_hash);
        assert_ne!(key1, key2);
    }

    #[test]
    fn chunk_cache_key_returns_different_hash_when_config_hash_differs() {
        let config_a = compute_chunker_config_hash(1024);
        let config_b = compute_chunker_config_hash(2048);
        let key1 = chunk_cache_key("test.md", "content", &config_a);
        let key2 = chunk_cache_key("test.md", "content", &config_b);
        assert_ne!(key1, key2);
    }

    // -------------------------------------------------------------------
    // B05: chunk_cache_key path-sensitivity
    // -------------------------------------------------------------------
    #[test]
    fn chunk_cache_key_returns_different_hash_for_same_content_at_different_paths() {
        let config_hash = compute_chunker_config_hash(1024);
        let key1 = chunk_cache_key("dir/a.md", "same content", &config_hash);
        let key2 = chunk_cache_key("dir/b.md", "same content", &config_hash);
        assert_ne!(key1, key2);
    }

    // -------------------------------------------------------------------
    // B06: Empty cache → all changed
    // -------------------------------------------------------------------
    #[test]
    fn partition_returns_all_changed_when_cache_is_empty() {
        let config = crate::cache::CacheConfig::in_memory();
        let cache = DocCache::open(config).unwrap();
        let config_hash = compute_chunker_config_hash(1024);

        let analyses = vec![
            make_analysis("a.md", "A", "content a", "concept"),
            make_analysis("b.md", "B", "content b", "concept"),
            make_analysis("c.md", "C", "content c", "concept"),
        ];

        let (unchanged, changed) =
            partition_by_cache_status(&analyses, &cache, &config_hash).unwrap();

        assert!(unchanged.is_empty(), "unchanged should be empty");
        assert_eq!(changed.len(), 3, "all 3 should be changed");
    }

    // -------------------------------------------------------------------
    // B07: Cache hit → unchanged
    // -------------------------------------------------------------------
    #[test]
    fn partition_returns_unchanged_when_cache_key_matches_existing_entry() {
        let config = crate::cache::CacheConfig::in_memory();
        let cache = DocCache::open(config).unwrap();
        let config_hash = compute_chunker_config_hash(1024);

        let cached_chunks = vec![
            make_test_chunk(
                "doc#0",
                "doc",
                0,
                contextual_chunker::ChunkLevel::Standard,
                "body 0",
                Some("H0"),
            ),
            make_test_chunk(
                "doc#1",
                "doc",
                1,
                contextual_chunker::ChunkLevel::Standard,
                "body 1",
                Some("H1"),
            ),
        ];

        let key = chunk_cache_key("a.md", "content-a", &config_hash);
        cache
            .put(CacheType::Chunk, key.as_bytes(), &cached_chunks)
            .unwrap();

        let analyses = vec![make_analysis("a.md", "A", "content-a", "concept")];

        let (unchanged, changed) =
            partition_by_cache_status(&analyses, &cache, &config_hash).unwrap();

        assert_eq!(unchanged.len(), 1, "should have 1 unchanged");
        assert_eq!(unchanged[0].1.len(), 2, "should have 2 cached chunks");
        assert!(changed.is_empty(), "changed should be empty");
    }

    // -------------------------------------------------------------------
    // B09: Cache deserialization error → downgrade to changed
    // -------------------------------------------------------------------
    #[test]
    fn partition_downgrades_deserialization_failure_to_changed() {
        let config = crate::cache::CacheConfig::in_memory();
        let cache = DocCache::open(config).unwrap();
        let config_hash = compute_chunker_config_hash(1024);

        // Store invalid bytes under the key
        let key = chunk_cache_key("deser-fail.md", "some-content", &config_hash);
        cache
            .put(CacheType::Chunk, key.as_bytes(), b"NOT VALID JSON")
            .unwrap();

        let analyses = vec![make_analysis(
            "deser-fail.md",
            "F",
            "some-content",
            "concept",
        )];

        let (unchanged, changed) =
            partition_by_cache_status(&analyses, &cache, &config_hash).unwrap();

        assert!(unchanged.is_empty(), "unchanged should be empty");
        assert_eq!(
            changed.len(),
            1,
            "deser failure should downgrade to changed"
        );
    }

    // -------------------------------------------------------------------
    // B10: Order preservation in partition
    // -------------------------------------------------------------------
    #[test]
    fn partition_preserves_analysis_order_in_both_vectors() {
        let config = crate::cache::CacheConfig::in_memory();
        let cache = DocCache::open(config).unwrap();
        let config_hash = compute_chunker_config_hash(1024);

        // Pre-populate B and D
        let key_b = chunk_cache_key("b.md", "content-b", &config_hash);
        cache
            .put(
                CacheType::Chunk,
                key_b.as_bytes(),
                &vec![make_test_chunk(
                    "b#0",
                    "b",
                    0,
                    contextual_chunker::ChunkLevel::Standard,
                    "b0",
                    None,
                )],
            )
            .unwrap();

        let key_d = chunk_cache_key("d.md", "content-d", &config_hash);
        cache
            .put(
                CacheType::Chunk,
                key_d.as_bytes(),
                &vec![make_test_chunk(
                    "d#0",
                    "d",
                    0,
                    contextual_chunker::ChunkLevel::Standard,
                    "d0",
                    None,
                )],
            )
            .unwrap();

        let analyses = vec![
            make_analysis("a.md", "A", "content-a", "concept"),
            make_analysis("b.md", "B", "content-b", "concept"),
            make_analysis("c.md", "C", "content-c", "concept"),
            make_analysis("d.md", "D", "content-d", "concept"),
        ];

        let (unchanged, changed) =
            partition_by_cache_status(&analyses, &cache, &config_hash).unwrap();

        // Unchanged: B then D (order of input)
        assert_eq!(unchanged[0].0.source_path, "b.md");
        assert_eq!(unchanged[1].0.source_path, "d.md");

        // Changed: A then C (order of input)
        assert_eq!(changed[0].source_path, "a.md");
        assert_eq!(changed[1].source_path, "c.md");
    }

    // -------------------------------------------------------------------
    // B11: Chunk file written with correct format
    // -------------------------------------------------------------------
    #[test]
    fn write_chunk_file_creates_md_with_frontmatter_and_content() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let chunks_dir = temp_dir.path().join("chunks");
        fs::create_dir_all(&chunks_dir).unwrap();

        let chunk = make_test_chunk(
            "doc#0",
            "doc",
            0,
            contextual_chunker::ChunkLevel::Standard,
            "body text",
            Some("Intro"),
        );
        // Override token_count and summary for exact matching
        let chunk = Chunk {
            token_count: 42,
            summary: "A summary".to_string(),
            ..chunk
        };

        write_chunk_file(&chunk, &chunks_dir).unwrap();

        let file_path = chunks_dir.join("doc-0-standard.md");
        assert!(file_path.exists(), "chunk file should exist");

        let contents = fs::read_to_string(file_path).unwrap();
        assert!(contents.starts_with("---"), "should start with frontmatter");
        assert!(contents.contains("doc_id: doc"), "should contain doc_id");
        assert!(
            contents.contains("chunk_id: doc#0"),
            "should contain chunk_id"
        );
        assert!(
            contents.contains("chunk_level: standard"),
            "should contain chunk_level"
        );
        assert!(
            contents.contains("heading: Intro"),
            "should contain heading"
        );
        assert!(
            contents.contains("token_count: 42"),
            "should contain token_count"
        );
        assert!(
            contents.contains("summary: A summary"),
            "should contain summary"
        );
        assert!(contents.contains("body text"), "should contain body text");

        // Verify body appears after second ---
        let parts: Vec<&str> = contents.splitn(3, "---").collect();
        assert!(parts.len() >= 3, "should have frontmatter delimiters");
        assert!(
            parts[2].trim().contains("body text"),
            "body should be after frontmatter"
        );
    }

    // -------------------------------------------------------------------
    // B12: Write to unwritable dir fails
    // -------------------------------------------------------------------
    #[test]
    fn write_chunk_file_returns_chunk_write_failed_when_dir_unwritable() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let ro_dir = temp_dir.path().join("ro");
        let chunks_dir = ro_dir.join("chunks");
        fs::create_dir_all(&chunks_dir).unwrap();

        // Make parent read-only
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&ro_dir, fs::Permissions::from_mode(0o444)).unwrap();
        }

        let chunk = make_test_chunk(
            "doc#0",
            "doc",
            0,
            contextual_chunker::ChunkLevel::Standard,
            "body",
            Some("H"),
        );

        let result = write_chunk_file(&chunk, &chunks_dir);
        assert!(result.is_err(), "should fail on unwritable dir");

        let err = result.unwrap_err();
        let reuse_err = err.downcast_ref::<ChunkReuseError>();
        assert!(reuse_err.is_some(), "error should be ChunkReuseError");

        if let Some(ChunkReuseError::ChunkWriteFailed { path, .. }) = reuse_err {
            assert!(
                path.to_string_lossy().contains("chunks"),
                "path should contain 'chunks': {}",
                path.display()
            );
        } else {
            panic!("expected ChunkWriteFailed variant, got {:?}", reuse_err);
        }

        // Cleanup: restore permissions so tempdir can clean up
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&ro_dir, fs::Permissions::from_mode(0o755)).unwrap();
        }
    }

    // -------------------------------------------------------------------
    // B14: Document exceeds size limit
    // -------------------------------------------------------------------
    #[test]
    fn chunk_all_cached_returns_document_exceeds_size_limit_when_content_too_large() {
        let config = crate::cache::CacheConfig::in_memory();
        let cache = DocCache::open(config).unwrap();
        let config_hash = compute_chunker_config_hash(1000);

        let temp_dir = tempfile::TempDir::new().unwrap();
        let link_map = HashMap::new();

        let analyses = vec![make_analysis("big.md", "Big", &"x".repeat(2000), "concept")];

        let result = chunk_all_cached(
            &analyses,
            &link_map,
            temp_dir.path(),
            1000,
            &cache,
            config_hash,
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        let reuse_err = err.downcast_ref::<ChunkReuseError>();
        assert!(reuse_err.is_some());

        if let Some(ChunkReuseError::DocumentExceedsSizeLimit {
            source_path,
            content_size,
            max_bytes,
        }) = reuse_err
        {
            assert_eq!(*source_path, "big.md");
            assert_eq!(*content_size, 2000);
            assert_eq!(*max_bytes, 1000);
        } else {
            panic!("expected DocumentExceedsSizeLimit, got {:?}", reuse_err);
        }
    }

    // -------------------------------------------------------------------
    // B16: Unchanged files skip chunker
    // -------------------------------------------------------------------
    #[test]
    fn chunk_all_cached_skips_contextual_chunker_for_unchanged_files() {
        let config = crate::cache::CacheConfig::in_memory();
        let cache = DocCache::open(config).unwrap();
        let config_hash = compute_chunker_config_hash(1_048_576);

        let temp_dir = tempfile::TempDir::new().unwrap();

        let analyses = vec![
            make_analysis(
                "a.md",
                "A",
                "# Hello World\nSome content for doc A",
                "concept",
            ),
            make_analysis("b.md", "B", "# Another Doc\nContent for B", "concept"),
        ];
        let link_map = make_link_map(&analyses);

        // Pre-cache A with 3 chunks
        let cached_a = vec![
            make_test_chunk(
                "concept/a#0",
                "concept/a",
                0,
                contextual_chunker::ChunkLevel::Summary,
                "A summary",
                Some("A"),
            ),
            make_test_chunk(
                "concept/a#1",
                "concept/a",
                1,
                contextual_chunker::ChunkLevel::Standard,
                "A body 1",
                Some("A1"),
            ),
            make_test_chunk(
                "concept/a#2",
                "concept/a",
                2,
                contextual_chunker::ChunkLevel::Standard,
                "A body 2",
                Some("A2"),
            ),
        ];
        let key_a = chunk_cache_key("a.md", analyses[0].content.as_ref(), &config_hash);
        cache
            .put(CacheType::Chunk, key_a.as_bytes(), &cached_a)
            .unwrap();

        let result = chunk_all_cached(
            &analyses,
            &link_map,
            temp_dir.path(),
            1_048_576,
            &cache,
            config_hash,
        )
        .unwrap();

        // A's 3 cached chunks + B's freshly computed chunks
        assert!(
            result.chunks_metadata.len() >= 3,
            "should have at least 3 cached chunks from A + B's fresh chunks, got {}",
            result.chunks_metadata.len()
        );

        // A's chunks should come first (order preservation)
        assert_eq!(result.chunks_metadata[0].doc_id, "concept/a");
        assert_eq!(result.chunks_metadata[1].doc_id, "concept/a");
        assert_eq!(result.chunks_metadata[2].doc_id, "concept/a");

        // B's chunks follow
        let b_chunks: Vec<&Chunk> = result
            .chunks_metadata
            .iter()
            .filter(|c| c.doc_id.contains("b"))
            .collect();
        assert!(!b_chunks.is_empty(), "B should have fresh chunks");
    }

    // -------------------------------------------------------------------
    // B17: Fresh chunks stored in cache
    // -------------------------------------------------------------------
    #[test]
    fn chunk_all_cached_stores_fresh_chunks_in_cache_for_changed_files() {
        let config = crate::cache::CacheConfig::in_memory();
        let cache = DocCache::open(config).unwrap();
        let config_hash = compute_chunker_config_hash(1_048_576);

        let temp_dir = tempfile::TempDir::new().unwrap();

        let analyses = vec![make_analysis(
            "fresh.md",
            "Fresh Doc",
            "# Fresh\nSome content for fresh doc",
            "concept",
        )];
        let link_map = make_link_map(&analyses);

        let result = chunk_all_cached(
            &analyses,
            &link_map,
            temp_dir.path(),
            1_048_576,
            &cache,
            config_hash,
        )
        .unwrap();

        // Verify cached
        let key = chunk_cache_key("fresh.md", analyses[0].content.as_ref(), &config_hash);
        let cached: Option<Vec<Chunk>> = cache.get(CacheType::Chunk, key.as_bytes()).unwrap();
        assert!(cached.is_some(), "fresh chunks should be cached");
        let cached_chunks = cached.unwrap();
        assert_eq!(
            cached_chunks.len(),
            result.chunks_metadata.len(),
            "cached count should match result count"
        );
    }

    // -------------------------------------------------------------------
    // B18: All chunk files written to disk
    // -------------------------------------------------------------------
    #[test]
    fn chunk_all_cached_writes_all_chunk_files_to_disk_for_mixed_cache_hits() {
        let config = crate::cache::CacheConfig::in_memory();
        let cache = DocCache::open(config).unwrap();
        let config_hash = compute_chunker_config_hash(1_048_576);

        let temp_dir = tempfile::TempDir::new().unwrap();

        let analyses = vec![
            make_analysis("a.md", "A", "# Doc A\nContent A", "concept"),
            make_analysis("b.md", "B", "# Doc B\nContent B", "concept"),
            make_analysis("c.md", "C", "# Doc C\nContent C", "concept"),
        ];
        let link_map = make_link_map(&analyses);

        // Pre-cache A and B with 2 chunks each
        for (name, doc_id) in &[("a.md", "concept/a"), ("b.md", "concept/b")] {
            let cached = vec![
                make_test_chunk(
                    &format!("{}#0", doc_id),
                    *doc_id,
                    0,
                    contextual_chunker::ChunkLevel::Standard,
                    "body0",
                    Some("H0"),
                ),
                make_test_chunk(
                    &format!("{}#1", doc_id),
                    *doc_id,
                    1,
                    contextual_chunker::ChunkLevel::Standard,
                    "body1",
                    Some("H1"),
                ),
            ];
            let analysis = analyses.iter().find(|a| a.source_path == *name).unwrap();
            let key = chunk_cache_key(name, analysis.content.as_ref(), &config_hash);
            cache
                .put(CacheType::Chunk, key.as_bytes(), &cached)
                .unwrap();
        }

        let result = chunk_all_cached(
            &analyses,
            &link_map,
            temp_dir.path(),
            1_048_576,
            &cache,
            config_hash,
        )
        .unwrap();

        let chunks_dir = temp_dir.path().join("chunks");
        let md_files: Vec<_> = fs::read_dir(&chunks_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
            .collect();

        assert_eq!(
            md_files.len(),
            result.chunks_metadata.len(),
            "file count should match chunk count: {} files vs {} chunks",
            md_files.len(),
            result.chunks_metadata.len()
        );
    }

    // -------------------------------------------------------------------
    // B19: Counter accuracy
    // -------------------------------------------------------------------
    #[test]
    fn chunk_all_cached_returns_accurate_counters_for_mixed_results() {
        let config = crate::cache::CacheConfig::in_memory();
        let cache = DocCache::open(config).unwrap();
        let config_hash = compute_chunker_config_hash(1_048_576);

        let temp_dir = tempfile::TempDir::new().unwrap();

        let analyses = vec![
            make_analysis("a.md", "A", "# Doc A\nContent A", "concept"),
            make_analysis(
                "b.md",
                "B",
                "# Doc B\nContent B here with more text",
                "concept",
            ),
        ];
        let link_map = make_link_map(&analyses);

        // Pre-cache A with 2 standard chunks
        let cached_a = vec![
            make_test_chunk(
                "concept/a#0",
                "concept/a",
                0,
                contextual_chunker::ChunkLevel::Standard,
                "body0",
                Some("H0"),
            ),
            make_test_chunk(
                "concept/a#1",
                "concept/a",
                1,
                contextual_chunker::ChunkLevel::Standard,
                "body1",
                Some("H1"),
            ),
        ];
        let key_a = chunk_cache_key("a.md", analyses[0].content.as_ref(), &config_hash);
        cache
            .put(CacheType::Chunk, key_a.as_bytes(), &cached_a)
            .unwrap();

        let result = chunk_all_cached(
            &analyses,
            &link_map,
            temp_dir.path(),
            1_048_576,
            &cache,
            config_hash,
        )
        .unwrap();

        assert_eq!(result.document_count, 2, "document_count should be 2");

        let total = result.summary_chunks + result.standard_chunks + result.detailed_chunks;
        assert_eq!(
            total, result.total_chunks,
            "total_chunks should equal sum of level counts"
        );
        assert_eq!(
            result.total_chunks,
            result.chunks_metadata.len(),
            "total_chunks should equal chunks_metadata length"
        );

        // A contributed 2 standard chunks
        assert!(
            result.standard_chunks >= 2,
            "should have at least 2 standard chunks from A"
        );
    }

    // -------------------------------------------------------------------
    // B20: Order stability
    // -------------------------------------------------------------------
    #[test]
    fn chunk_all_cached_preserves_analysis_order_in_chunks_metadata() {
        let config = crate::cache::CacheConfig::in_memory();
        let cache = DocCache::open(config).unwrap();
        let config_hash = compute_chunker_config_hash(1_048_576);

        let temp_dir = tempfile::TempDir::new().unwrap();

        let analyses = vec![
            make_analysis("a.md", "A", "# Doc A\nContent A here", "concept"),
            make_analysis("b.md", "B", "# Doc B\nContent B here", "concept"),
            make_analysis("c.md", "C", "# Doc C\nContent C here", "concept"),
        ];
        let link_map = make_link_map(&analyses);

        // Pre-cache A (2 chunks) and C (1 chunk)
        let cached_a = vec![
            make_test_chunk(
                "concept/a#0",
                "concept/a",
                0,
                contextual_chunker::ChunkLevel::Standard,
                "A0",
                Some("H"),
            ),
            make_test_chunk(
                "concept/a#1",
                "concept/a",
                1,
                contextual_chunker::ChunkLevel::Standard,
                "A1",
                Some("H"),
            ),
        ];
        let key_a = chunk_cache_key("a.md", analyses[0].content.as_ref(), &config_hash);
        cache
            .put(CacheType::Chunk, key_a.as_bytes(), &cached_a)
            .unwrap();

        let cached_c = vec![make_test_chunk(
            "concept/c#0",
            "concept/c",
            0,
            contextual_chunker::ChunkLevel::Standard,
            "C0",
            Some("H"),
        )];
        let key_c = chunk_cache_key("c.md", analyses[2].content.as_ref(), &config_hash);
        cache
            .put(CacheType::Chunk, key_c.as_bytes(), &cached_c)
            .unwrap();

        let result = chunk_all_cached(
            &analyses,
            &link_map,
            temp_dir.path(),
            1_048_576,
            &cache,
            config_hash,
        )
        .unwrap();

        // A's chunks [0..2], B's chunks [2..?], C's chunks [?..]
        let a_count = result
            .chunks_metadata
            .iter()
            .filter(|c| c.doc_id == "concept/a")
            .count();
        let b_count = result
            .chunks_metadata
            .iter()
            .filter(|c| c.doc_id.contains("b"))
            .count();
        let c_count = result
            .chunks_metadata
            .iter()
            .filter(|c| c.doc_id == "concept/c")
            .count();

        assert_eq!(a_count, 2, "A should have 2 cached chunks");
        assert_eq!(c_count, 1, "C should have 1 cached chunk");
        assert!(b_count > 0, "B should have freshly computed chunks");

        // Order: A's chunks, then B's, then C's
        let a_end = a_count;
        let b_end = a_end + b_count;

        for i in 0..a_end {
            assert_eq!(
                result.chunks_metadata[i].doc_id, "concept/a",
                "chunk {} should belong to A",
                i
            );
        }
        for i in a_end..b_end {
            assert!(
                result.chunks_metadata[i].doc_id.contains("b"),
                "chunk {} should belong to B, got {}",
                i,
                result.chunks_metadata[i].doc_id
            );
        }
        for i in b_end..result.chunks_metadata.len() {
            assert_eq!(
                result.chunks_metadata[i].doc_id, "concept/c",
                "chunk {} should belong to C",
                i
            );
        }
    }

    // -------------------------------------------------------------------
    // B21: document_count equals analyses length
    // -------------------------------------------------------------------
    #[test]
    fn chunk_all_cached_sets_document_count_equal_to_analyses_len() {
        let config = crate::cache::CacheConfig::in_memory();
        let cache = DocCache::open(config).unwrap();
        let config_hash = compute_chunker_config_hash(1_048_576);

        let temp_dir = tempfile::TempDir::new().unwrap();

        let analyses: Vec<Analysis> = (0..5)
            .map(|i| {
                make_analysis(
                    &format!("doc{}.md", i),
                    &format!("Doc {}", i),
                    &format!("# Doc {}\nContent {}", i, i),
                    "concept",
                )
            })
            .collect();
        let link_map = make_link_map(&analyses);

        // Pre-cache all
        for analysis in &analyses {
            let key = chunk_cache_key(
                &analysis.source_path,
                analysis.content.as_ref(),
                &config_hash,
            );
            let chunks = vec![make_test_chunk(
                &format!(
                    "concept/doc{}#0",
                    analysis.source_path.chars().nth(3).unwrap()
                ),
                "concept/doc",
                0,
                contextual_chunker::ChunkLevel::Standard,
                "body",
                None,
            )];
            cache
                .put(CacheType::Chunk, key.as_bytes(), &chunks)
                .unwrap();
        }

        let result = chunk_all_cached(
            &analyses,
            &link_map,
            temp_dir.path(),
            1_048_576,
            &cache,
            config_hash,
        )
        .unwrap();

        assert_eq!(result.document_count, 5);
    }

    // -------------------------------------------------------------------
    // B22: Equivalence with chunk_all
    // -------------------------------------------------------------------
    #[test]
    fn chunk_all_cached_produces_identical_result_as_chunk_all_for_same_inputs() {
        let config = crate::cache::CacheConfig::in_memory();
        let cache = DocCache::open(config).unwrap();
        let config_hash = compute_chunker_config_hash(1_048_576);

        let temp_dir_cached = tempfile::TempDir::new().unwrap();
        let temp_dir_uncached = tempfile::TempDir::new().unwrap();

        let analyses = vec![
            make_analysis("a.md", "Doc A", "# Doc A\nContent A", "concept"),
            make_analysis("b.md", "Doc B", "# Doc B\nContent B", "concept"),
            make_analysis("c.md", "Doc C", "# Doc C\nContent C", "concept"),
        ];
        let link_map = make_link_map(&analyses);

        let uncached =
            chunk_all(&analyses, &link_map, temp_dir_uncached.path(), 1_048_576).unwrap();

        let cached = chunk_all_cached(
            &analyses,
            &link_map,
            temp_dir_cached.path(),
            1_048_576,
            &cache,
            config_hash,
        )
        .unwrap();

        assert_eq!(cached.total_chunks, uncached.total_chunks);
        assert_eq!(cached.document_count, uncached.document_count);
        assert_eq!(cached.summary_chunks, uncached.summary_chunks);
        assert_eq!(cached.standard_chunks, uncached.standard_chunks);
        assert_eq!(cached.detailed_chunks, uncached.detailed_chunks);

        // Verify each chunk matches
        for (c, u) in cached
            .chunks_metadata
            .iter()
            .zip(uncached.chunks_metadata.iter())
        {
            assert_eq!(c.chunk_id, u.chunk_id, "chunk_id mismatch");
            assert_eq!(c.doc_id, u.doc_id, "doc_id mismatch");
            assert_eq!(c.chunk_index, u.chunk_index, "chunk_index mismatch");
            assert_eq!(c.content, u.content, "content mismatch");
            assert_eq!(c.chunk_level, u.chunk_level, "chunk_level mismatch");
            assert_eq!(c.chunk_type, u.chunk_type, "chunk_type mismatch");
        }
    }

    // -------------------------------------------------------------------
    // B24: chunk_cache_key with empty source_path
    // -------------------------------------------------------------------
    #[test]
    fn chunk_cache_key_returns_valid_hash_for_empty_source_path() {
        let config_hash = compute_chunker_config_hash(1024);
        let key1 = chunk_cache_key("", "some content", &config_hash);
        let key2 = chunk_cache_key("", "some content", &config_hash);
        assert_eq!(key1, key2, "empty path should produce deterministic hash");
        assert_eq!(key1.as_bytes().len(), 32, "should be valid SHA-256");
    }

    // -------------------------------------------------------------------
    // B25: chunk_cache_key with empty content
    // -------------------------------------------------------------------
    #[test]
    fn chunk_cache_key_returns_valid_hash_for_empty_content() {
        let config_hash = compute_chunker_config_hash(1024);
        let key1 = chunk_cache_key("test.md", "", &config_hash);
        let key2 = chunk_cache_key("test.md", "", &config_hash);
        assert_eq!(
            key1, key2,
            "empty content should produce deterministic hash"
        );
        assert_eq!(key1.as_bytes().len(), 32);
    }

    // -------------------------------------------------------------------
    // B26: All analyses cached → all unchanged
    // -------------------------------------------------------------------
    #[test]
    fn partition_returns_all_unchanged_when_all_analyses_cached() {
        let config = crate::cache::CacheConfig::in_memory();
        let cache = DocCache::open(config).unwrap();
        let config_hash = compute_chunker_config_hash(1024);

        let analyses = vec![
            make_analysis("a.md", "A", "content-a", "concept"),
            make_analysis("b.md", "B", "content-b", "concept"),
            make_analysis("c.md", "C", "content-c", "concept"),
        ];

        // Pre-populate all
        for (i, analysis) in analyses.iter().enumerate() {
            let key = chunk_cache_key(
                &analysis.source_path,
                analysis.content.as_ref(),
                &config_hash,
            );
            let chunks = vec![make_test_chunk(
                &format!("d{}#0", i),
                &format!("d{}", i),
                0,
                contextual_chunker::ChunkLevel::Standard,
                "body",
                None,
            )];
            cache
                .put(CacheType::Chunk, key.as_bytes(), &chunks)
                .unwrap();
        }

        let (unchanged, changed) =
            partition_by_cache_status(&analyses, &cache, &config_hash).unwrap();

        assert_eq!(unchanged.len(), 3, "all 3 should be unchanged");
        assert!(changed.is_empty(), "changed should be empty");
        assert_eq!(unchanged[0].1.len(), 1, "A should have 1 chunk");
        assert_eq!(unchanged[1].1.len(), 1, "B should have 1 chunk");
        assert_eq!(unchanged[2].1.len(), 1, "C should have 1 chunk");
    }

    // -------------------------------------------------------------------
    // B27: Empty analyses slice → empty partitions
    // -------------------------------------------------------------------
    #[test]
    fn partition_returns_empty_vectors_when_analyses_slice_is_empty() {
        let config = crate::cache::CacheConfig::in_memory();
        let cache = DocCache::open(config).unwrap();
        let config_hash = compute_chunker_config_hash(1024);

        let (unchanged, changed) = partition_by_cache_status(&[], &cache, &config_hash).unwrap();

        assert!(unchanged.is_empty());
        assert!(changed.is_empty());
    }

    // -------------------------------------------------------------------
    // B28: Chunk file with no heading
    // -------------------------------------------------------------------
    #[test]
    fn write_chunk_file_omits_heading_field_when_heading_is_none() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let chunks_dir = temp_dir.path().join("chunks");
        fs::create_dir_all(&chunks_dir).unwrap();

        let chunk = Chunk {
            heading: None,
            summary: "sum".to_string(),
            content: "body".to_string(),
            ..make_test_chunk(
                "doc#0",
                "doc",
                0,
                contextual_chunker::ChunkLevel::Standard,
                "body",
                None,
            )
        };

        write_chunk_file(&chunk, &chunks_dir).unwrap();

        let file_path = chunks_dir.join("doc-0-standard.md");
        let contents = fs::read_to_string(file_path).unwrap();
        assert!(
            !contents.contains("heading:"),
            "frontmatter should NOT contain 'heading:' field. Contents: {}",
            contents
        );
        assert!(
            contents.contains("body"),
            "should contain body after frontmatter"
        );
    }

    // -------------------------------------------------------------------
    // B29: Chunk file with empty content
    // -------------------------------------------------------------------
    #[test]
    fn write_chunk_file_creates_md_with_empty_body_when_content_is_empty() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let chunks_dir = temp_dir.path().join("chunks");
        fs::create_dir_all(&chunks_dir).unwrap();

        let chunk = Chunk {
            content: String::new(),
            heading: Some("H".to_string()),
            summary: "sum".to_string(),
            ..make_test_chunk(
                "doc#0",
                "doc",
                0,
                contextual_chunker::ChunkLevel::Standard,
                "",
                Some("H"),
            )
        };

        write_chunk_file(&chunk, &chunks_dir).unwrap();

        let file_path = chunks_dir.join("doc-0-standard.md");
        let contents = fs::read_to_string(file_path).unwrap();
        assert!(contents.starts_with("---"), "should start with frontmatter");
        assert!(contents.contains("---\n"), "should have closing ---");
        assert!(contents.len() > 0, "file should have content (frontmatter)");
    }

    // -------------------------------------------------------------------
    // B30: Chunk file escapes YAML special characters
    // -------------------------------------------------------------------
    #[test]
    fn write_chunk_file_escapes_yaml_special_characters_in_summary_and_heading() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let chunks_dir = temp_dir.path().join("chunks");
        fs::create_dir_all(&chunks_dir).unwrap();

        let chunk = Chunk {
            summary: "line1\nline2".to_string(),
            heading: Some("He said \"hello\" and: goodbye".to_string()),
            content: "body".to_string(),
            ..make_test_chunk(
                "doc#0",
                "doc",
                0,
                contextual_chunker::ChunkLevel::Standard,
                "body",
                Some("H"),
            )
        };

        write_chunk_file(&chunk, &chunks_dir).unwrap();

        let file_path = chunks_dir.join("doc-0-standard.md");
        let contents = fs::read_to_string(file_path).unwrap();

        // Verify escaped values are present
        assert!(
            contents.contains("line1 line2"),
            "newlines in summary should be escaped"
        );
        assert!(
            contents.contains(r#"He said \"hello\" and: goodbye"#),
            "quotes in heading should be escaped"
        );
    }

    // -------------------------------------------------------------------
    // B31: Document at exact size limit accepted
    // -------------------------------------------------------------------
    #[test]
    fn chunk_all_cached_accepts_document_at_exact_size_limit() {
        let config = crate::cache::CacheConfig::in_memory();
        let cache = DocCache::open(config).unwrap();
        let config_hash = compute_chunker_config_hash(1_048_576);

        let temp_dir = tempfile::TempDir::new().unwrap();

        let content = "x".repeat(1000);
        let analyses = vec![make_analysis("exact.md", "Exact", &content, "concept")];
        let link_map = make_link_map(&analyses);

        let result = chunk_all_cached(
            &analyses,
            &link_map,
            temp_dir.path(),
            1000,
            &cache,
            config_hash,
        );

        assert!(
            result.is_ok(),
            "exact size should be accepted: {:?}",
            result.err()
        );
        let result = result.unwrap();
        assert!(result.total_chunks >= 1, "should produce at least 1 chunk");
        assert_eq!(result.document_count, 1);
    }

    // -------------------------------------------------------------------
    // B32: Empty analyses slice → zero-count result
    // -------------------------------------------------------------------
    #[test]
    fn chunk_all_cached_returns_zero_count_result_when_analyses_is_empty() {
        let config = crate::cache::CacheConfig::in_memory();
        let cache = DocCache::open(config).unwrap();
        let config_hash = compute_chunker_config_hash(1024);

        let temp_dir = tempfile::TempDir::new().unwrap();
        let link_map = HashMap::new();

        let result =
            chunk_all_cached(&[], &link_map, temp_dir.path(), 1024, &cache, config_hash).unwrap();

        assert_eq!(result.total_chunks, 0);
        assert_eq!(result.document_count, 0);
        assert!(result.chunks_metadata.is_empty());

        // chunks dir should exist but be empty
        let chunks_dir = temp_dir.path().join("chunks");
        assert!(chunks_dir.exists(), "chunks dir should exist");
        let entries: Vec<_> = fs::read_dir(&chunks_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        assert!(entries.is_empty(), "chunks dir should be empty");
    }

    // -------------------------------------------------------------------
    // B33: Full pipeline two-run cache reuse
    // -------------------------------------------------------------------
    #[test]
    fn chunk_all_cached_reuses_cached_chunks_on_second_run_for_same_files() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let db_path = temp_dir.path().join("cache.redb");
        let config = crate::cache::CacheConfig::new(&db_path);

        let output_dir1 = tempfile::TempDir::new().unwrap();
        let output_dir2 = tempfile::TempDir::new().unwrap();

        let analyses = vec![
            make_analysis("a.md", "Doc A", "# Doc A\nContent A", "concept"),
            make_analysis("b.md", "Doc B", "# Doc B\nContent B", "concept"),
            make_analysis("c.md", "Doc C", "# Doc C\nContent C", "concept"),
        ];
        let link_map = make_link_map(&analyses);
        let config_hash = compute_chunker_config_hash(1_048_576);

        // First run
        let cache1 = DocCache::open(config.clone()).unwrap();
        let result1 = chunk_all_cached(
            &analyses,
            &link_map,
            output_dir1.path(),
            1_048_576,
            &cache1,
            config_hash,
        )
        .unwrap();
        drop(cache1);

        // Second run with same cache
        let cache2 = DocCache::open(config).unwrap();
        let result2 = chunk_all_cached(
            &analyses,
            &link_map,
            output_dir2.path(),
            1_048_576,
            &cache2,
            config_hash,
        )
        .unwrap();

        assert_eq!(result2.total_chunks, result1.total_chunks);
        assert_eq!(result2.document_count, result1.document_count);
        assert_eq!(result2.chunks_metadata.len(), result1.chunks_metadata.len());

        for (c2, c1) in result2
            .chunks_metadata
            .iter()
            .zip(result1.chunks_metadata.iter())
        {
            assert_eq!(c2.chunk_id, c1.chunk_id);
            assert_eq!(c2.doc_id, c1.doc_id);
            assert_eq!(c2.chunk_index, c1.chunk_index);
            assert_eq!(c2.content, c1.content);
        }
    }

    // -------------------------------------------------------------------
    // Proptest: compute_chunker_config_hash idempotent
    // -------------------------------------------------------------------
    #[test]
    fn proptest_compute_chunker_config_hash_idempotent() {
        use proptest::prelude::*;
        proptest!(|(max_bytes in any::<u64>())| {
            let h1 = compute_chunker_config_hash(max_bytes);
            let h2 = compute_chunker_config_hash(max_bytes);
            assert_eq!(h1, h2, "idempotent for max_bytes={}", max_bytes);
        });
    }

    // -------------------------------------------------------------------
    // Proptest: compute_chunker_config_hash injective
    // -------------------------------------------------------------------
    #[test]
    fn proptest_compute_chunker_config_hash_injective() {
        use proptest::prelude::*;
        proptest!(|(a in any::<u64>(), b in any::<u64>())| {
            if a != b {
                assert_ne!(
                    compute_chunker_config_hash(a),
                    compute_chunker_config_hash(b),
                    "different inputs should produce different hashes: {} vs {}",
                    a, b
                );
            }
        });
    }

    // -------------------------------------------------------------------
    // Proptest: chunk_cache_key equivalence with composite_hash
    // -------------------------------------------------------------------
    #[test]
    fn proptest_chunk_cache_key_equivalence_with_composite_hash() {
        use proptest::prelude::*;
        proptest!(|(path in ".*", content in ".*")| {
            let config_hash = compute_chunker_config_hash(1024);
            let key = chunk_cache_key(&path, &content, &config_hash);
            let expected = composite_hash(&[
                path.as_bytes(),
                content.as_bytes(),
                config_hash.as_bytes(),
            ]);
            assert_eq!(key, expected, "chunk_cache_key should match composite_hash for path={}, content={}", path, content);
        });
    }
}
