//! Search-index lifecycle management for the MCP server.
//!
//! Provides functions for ensuring a Tantivy search index exists and is
//! populated, factored out of [`CtdMcpServer`] for file-length compliance.

use crate::mcp::domain::IndexData;
use crate::mcp::error::CtdMcpError;

/// Ensure a search index exists and is populated for the given index directory.
///
/// Opens an existing index if present, otherwise builds a new one from
/// `INDEX.json`. If the index is empty, populates it from the index data.
pub fn ensure_search_index(
    index_dir: &std::path::Path,
    index_data: &IndexData,
) -> Result<tantivy::Index, CtdMcpError> {
    let search_index = match crate::search::open_existing_index(index_dir) {
        Ok(Some(idx)) => idx,
        Ok(None) => build_new_index(index_dir, index_data)?,
        Err(e) => {
            return Err(CtdMcpError::SearchIndexError {
                reason: e.to_string(),
            })
        }
    };

    if !search_index
        .reader()
        .ok()
        .is_some_and(|r| r.searcher().num_docs() > 0)
    {
        populate_index(&search_index, index_data)?;
    }
    Ok(search_index)
}

/// Build a new Tantivy search index from INDEX.json data.
fn build_new_index(
    dir: &std::path::Path,
    index_data: &IndexData,
) -> Result<tantivy::Index, CtdMcpError> {
    let index =
        crate::search::rebuild_index_from_json(dir).map_err(|e| CtdMcpError::SearchIndexError {
            reason: e.to_string(),
        })?;
    populate_index(&index, index_data)?;
    Ok(index)
}

/// Populate a Tantivy index with documents extracted from index data.
fn populate_index(index: &tantivy::Index, index_data: &IndexData) -> Result<(), CtdMcpError> {
    let documents = index_data.extract_documents();
    if documents.is_empty() {
        return Ok(());
    }
    write_documents_to_index(index, &documents)
}

/// Write a batch of documents to a Tantivy index and commit.
fn write_documents_to_index(
    index: &tantivy::Index,
    documents: &[crate::index::IndexDocument],
) -> Result<(), CtdMcpError> {
    let mut writer = index
        .writer(50_000_000)
        .map_err(|e| CtdMcpError::SearchIndexError {
            reason: e.to_string(),
        })?;
    crate::search::index_documents(&mut writer, documents).map_err(|e| {
        CtdMcpError::SearchIndexError {
            reason: e.to_string(),
        }
    })?;
    writer.commit().map_err(|e| CtdMcpError::SearchIndexError {
        reason: e.to_string(),
    })?;
    Ok(())
}
