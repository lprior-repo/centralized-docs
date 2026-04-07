use std::collections::HashMap;

use tantivy::doc;

use super::{create_schema, IndexerError};

/// Index a batch of documents into Tantivy (used by tests)
#[allow(dead_code)]
pub fn index_documents(
    writer: &mut tantivy::IndexWriter,
    documents: &[crate::index::IndexDocument],
) -> std::result::Result<(), IndexerError> {
    let (_schema, fields) = create_schema();

    // Add each document
    documents
        .iter()
        .try_for_each(|doc| -> std::result::Result<(), IndexerError> {
            let tags_str = doc.tags.join(" ");
            let headings_str = doc.headings.join(" ");
            let searchable_content = format!(
                "{} {} {} {} {} {}",
                doc.title, doc.summary, doc.path, tags_str, headings_str, doc.content
            );

            // Use tantivy::doc! macro to build document
            let tantivy_doc = doc!(
                fields.id => doc.id.as_str(),
                fields.title => doc.title.as_str(),
                fields.summary => doc.summary.as_str(),
                fields.content => searchable_content.as_str(),
                fields.category => doc.category.as_str(),
                fields.word_count => doc.word_count as u64,
                fields.path => doc.path.as_str(),
            );

            writer
                .add_document(tantivy_doc)
                .map_err(|e| IndexerError::IndexCommitFailed(e.to_string()))?;
            Ok(())
        })?;

    Ok(())
}

/// Index a batch of chunks into Tantivy
///
/// ## Behavior
///
/// - Adds all chunks
/// - Does NOT commit transaction (caller is responsible)
///
/// ## Error Handling
///
/// Returns error if write fails.
///
/// # Arguments
///
/// * `writer` - Mutable reference to Tantivy `IndexWriter`
/// * `documents` - Original documents to resolve categories/paths
/// * `chunks` - Chunks to index
///
/// # Returns
///
/// Success, error if any operation fails
pub fn index_chunks(
    writer: &mut tantivy::IndexWriter,
    documents: &[crate::index::IndexDocument],
    chunks: &[crate::chunking_adapter::Chunk],
) -> std::result::Result<(), IndexerError> {
    let (_schema, fields) = create_schema();

    // Map doc_id to doc for fast lookup of category and path
    let doc_map: HashMap<_, _> = documents.iter().map(|d| (d.id.as_str(), d)).collect();

    // Add each chunk
    chunks
        .iter()
        .try_for_each(|chunk| -> std::result::Result<(), IndexerError> {
            let doc = doc_map.get(chunk.doc_id.as_str());
            let category = doc.map_or("uncategorized", |d| d.category.as_str());

            // Build the path based on how chunks are saved: "chunks/xxx-summary.md"
            let level_suffix = chunk.chunk_level.as_str();
            let chunk_filename = format!(
                "chunks/{}-{}.md",
                chunk.chunk_id.replace(['/', '#'], "-"),
                level_suffix
            );

            let title = if let Some(h) = &chunk.heading {
                format!("{} - {}", chunk.doc_title, h)
            } else {
                chunk.doc_title.clone()
            };

            // Use tantivy::doc! macro to build document
            let tantivy_doc = doc!(
                fields.id => chunk.chunk_id.as_str(),
                fields.title => title.as_str(),
                fields.summary => chunk.summary.as_str(),
                fields.content => chunk.content.as_str(),
                fields.category => category,
                fields.word_count => chunk.token_count as u64,
                fields.path => chunk_filename.as_str(),
            );

            writer
                .add_document(tantivy_doc)
                .map_err(|e| IndexerError::IndexCommitFailed(e.to_string()))?;
            Ok(())
        })?;

    Ok(())
}
