//! Index construction: document index, chunk metadata, knowledge graph, search index.

pub mod build_index;
pub(crate) mod index_assembly;
pub mod knowledge_dag;
pub mod navigation;
pub mod types;

// Re-exports for backward compatibility
pub use build_index::{build_and_write_index, build_chunk_metadata, build_document_index};
pub use knowledge_dag::build_knowledge_dag;
pub use navigation::{build_and_write_navigation, extract_tags};
pub use types::{ChunkMetadata, IndexDocument, RelatedChunk};

#[cfg(test)]
mod tests;
