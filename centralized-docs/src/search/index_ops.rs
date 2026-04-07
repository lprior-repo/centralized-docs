use std::fs;
use std::path::Path;

use anyhow::{anyhow, Result};
use tantivy::Index;

use super::create_schema;

/// Rebuild the Tantivy index from INDEX.json data.
///
/// This is used for recovery when the Tantivy index is corrupted.
/// It reads document data from INDEX.json and re-indexes all documents.
#[allow(clippy::too_many_lines)]
pub fn rebuild_index_from_json(index_path: &Path) -> Result<Index> {
    let index_json_path = index_path.join("INDEX.json");

    let index_content = fs::read_to_string(&index_json_path)
        .map_err(|e| anyhow!("Failed to read INDEX.json: {e}"))?;

    let index_value: serde_json::Value = serde_json::from_str(&index_content)
        .map_err(|e| anyhow!("Failed to parse INDEX.json: {e}"))?;

    let documents = index_value["documents"]
        .as_array()
        .ok_or_else(|| anyhow!("Invalid INDEX.json: missing documents array"))?;

    let docs: Vec<crate::index::IndexDocument> = documents
        .iter()
        .filter_map(|doc| {
            let id = doc["id"].as_str()?;
            let title = doc["title"].as_str().map_or("", |s| s);
            let summary = doc["summary"].as_str().map_or("", |s| s);
            let path = doc["path"].as_str().map_or("", |s| s);
            let category = doc["category"].as_str().map_or("", |s| s);
            let word_count = doc["word_count"].as_u64().map_or(0, |v| v) as usize;
            let tags: Vec<String> = doc["tags"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .map_or_else(Vec::new, std::convert::identity);
            let chunk_ids: Vec<String> = doc["chunk_ids"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .map_or_else(Vec::new, std::convert::identity);
            let headings: Vec<String> = doc["headings"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .map_or_else(Vec::new, std::convert::identity);
            let content = doc["content"].as_str().map_or("", |s| s).into();

            Some(crate::index::IndexDocument {
                id: id.to_string(),
                title: title.to_string(),
                path: path.to_string(),
                category: category.to_string(),
                tags,
                summary: summary.to_string(),
                word_count,
                chunk_ids,
                headings,
                content,
            })
        })
        .collect();

    let chunks_val = index_value["chunks"]
        .as_array()
        .ok_or_else(|| anyhow!("Invalid INDEX.json: missing chunks array"))?;

    let chunks: Vec<crate::chunking_adapter::Chunk> = chunks_val
        .iter()
        .filter_map(|chunk| {
            let chunk_id = chunk["chunk_id"].as_str()?;
            let doc_id = chunk["doc_id"].as_str().map_or("", |s| s);
            let doc_title = chunk["doc_title"].as_str().map_or("", |s| s);
            let summary = chunk["summary"].as_str().map_or("", |s| s);
            let token_count = chunk["token_count"].as_u64().map_or(0, |v| v) as usize;
            let heading = chunk["heading"].as_str().map(String::from);

            let level_str = chunk["chunk_level"].as_str().map_or("standard", |s| s);
            let chunk_level = match level_str {
                "summary" => contextual_chunker::ChunkLevel::Summary,
                "detailed" => contextual_chunker::ChunkLevel::Detailed,
                _ => contextual_chunker::ChunkLevel::Standard,
            };

            let chunk_filename = format!("{}-{}.md", chunk_id.replace(['/', '#'], "-"), level_str);
            let chunk_path = index_path.join("chunks").join(&chunk_filename);

            let raw_content = fs::read_to_string(&chunk_path)
                .map_err(|e| {
                    anyhow::anyhow!("Failed to read chunk file {}: {}", chunk_path.display(), e)
                })
                .ok()?;

            let content = if raw_content.starts_with("---\n") || raw_content.starts_with("---\r\n")
            {
                let remaining = raw_content
                    .lines()
                    .skip(1)
                    .skip_while(|line| line.trim_end() != "---")
                    .skip(1)
                    .collect::<Vec<_>>()
                    .join("\n");
                if remaining.is_empty() {
                    raw_content
                } else {
                    remaining
                }
            } else {
                raw_content
            };

            Some(crate::chunking_adapter::Chunk {
                chunk_id: chunk_id.to_string(),
                doc_id: doc_id.to_string(),
                doc_title: doc_title.to_string(),
                chunk_index: 0,
                content,
                token_count,
                heading,
                heading_path: vec![],
                chunk_type: contextual_chunker::ChunkType::Prose,
                previous_chunk_id: None,
                next_chunk_id: None,
                related_chunk_ids: vec![],
                summary: summary.to_string(),
                chunk_level,
                parent_chunk_id: None,
                child_chunk_ids: vec![],
                context_prefix: None,
            })
        })
        .collect();

    let index_dir = index_path.join(".tantivy_index");
    fs::create_dir_all(&index_dir)?;
    let (schema, _fields) = create_schema();
    let index = Index::create_in_dir(&index_dir, schema)
        .map_err(|e| anyhow!("Failed to create index: {e}"))?;

    if !chunks.is_empty() {
        #[allow(unused_mut)] // tantivy IndexWriter API requires &mut self
        let mut writer = index
            .writer(50_000_000)
            .map_err(|e| anyhow!("Failed to create writer: {e}"))?;
        super::indexer::index_chunks(&mut writer, &docs, &chunks)?;
        writer
            .commit()
            .map_err(|e| anyhow!("Failed to commit: {e}"))?;
    }

    Ok(index)
}

pub fn open_or_create_index(index_path: &Path) -> Result<Index> {
    let index_dir = index_path.join(".tantivy_index");

    // Try to open existing index
    if index_dir.exists() {
        if index_dir.is_file() {
            fs::remove_file(&index_dir)?;
        } else {
            match Index::open_in_dir(&index_dir) {
                Ok(index) => return Ok(index),
                Err(e) => {
                    // Index is corrupted, try to rebuild from INDEX.json
                    eprintln!("Warning: Failed to open index: {e}");
                    if index_path.join("INDEX.json").exists() {
                        eprintln!("Attempting to rebuild index from INDEX.json...");
                        match rebuild_index_from_json(index_path) {
                            Ok(index) => {
                                eprintln!("Successfully rebuilt index from INDEX.json");
                                return Ok(index);
                            }
                            Err(rebuild_err) => {
                                eprintln!(
                                    "Warning: Failed to rebuild index from INDEX.json: {rebuild_err}"
                                );
                            }
                        }
                    }
                    // Fall back to creating empty index
                    fs::remove_dir_all(&index_dir).ok();
                }
            }
        }
    }

    // Create new index
    fs::create_dir_all(&index_dir)?;
    let (schema, _fields) = create_schema();
    Index::create_in_dir(&index_dir, schema).map_err(|e| anyhow!("Failed to create index: {e}"))
}

/// Open Tantivy index if it already exists.
///
/// Returns Ok(None) when:
/// - No index directory is present
/// - Index directory is corrupted (recovers by removing)
///
/// This allows the search to fall back to INDEX.json when the Tantivy index
/// is unavailable or corrupted.
#[allow(dead_code)]
pub fn open_existing_index(index_path: &Path) -> Result<Option<Index>> {
    let index_dir = index_path.join(".tantivy_index");

    if !index_dir.exists() {
        return Ok(None);
    }

    // Handle case where path exists but is a file (not a directory)
    if index_dir.is_file() {
        fs::remove_file(&index_dir).ok();
        return Ok(None);
    }

    // Try to open existing index
    if let Ok(index) = Index::open_in_dir(&index_dir) {
        Ok(Some(index))
    } else {
        // Index is corrupted, remove it and return None to trigger fallback
        // This allows search to use INDEX.json instead
        fs::remove_dir_all(&index_dir).ok();
        Ok(None)
    }
}
