use super::*;
use tempfile::TempDir;

pub(crate) fn make_index_document(
    id: &str,
    title: &str,
    summary: &str,
    content: &str,
    category: &str,
) -> crate::index::IndexDocument {
    crate::index::IndexDocument {
        id: id.to_string(),
        title: title.to_string(),
        path: format!("docs/{id}.md"),
        category: category.to_string(),
        tags: vec![],
        summary: summary.to_string(),
        word_count: content.split_whitespace().count(),
        chunk_ids: vec![],
        headings: vec![],
        content: content.into(),
    }
}

pub(crate) fn make_chunk(
    chunk_id: &str,
    doc_id: &str,
    doc_title: &str,
    content: &str,
    heading: Option<&str>,
) -> crate::chunking_adapter::Chunk {
    crate::chunking_adapter::Chunk {
        chunk_id: chunk_id.to_string(),
        doc_id: doc_id.to_string(),
        doc_title: doc_title.to_string(),
        chunk_index: 0,
        content: content.to_string(),
        token_count: content.split_whitespace().count(),
        heading: heading.map(String::from),
        heading_path: vec![],
        chunk_type: contextual_chunker::ChunkType::Prose,
        previous_chunk_id: None,
        next_chunk_id: None,
        related_chunk_ids: vec![],
        summary: content.to_string(),
        chunk_level: contextual_chunker::ChunkLevel::Standard,
        parent_chunk_id: None,
        child_chunk_ids: vec![],
        context_prefix: None,
    }
}

pub(crate) fn create_test_index_with_docs(
    docs: &[crate::index::IndexDocument],
) -> (TempDir, tantivy::Index) {
    let dir = TempDir::new().unwrap();
    let index = open_or_create_index(dir.path()).unwrap();
    let mut writer = index.writer(50_000_000).unwrap();
    index_documents(&mut writer, docs).unwrap();
    writer.commit().unwrap();
    (dir, index)
}
