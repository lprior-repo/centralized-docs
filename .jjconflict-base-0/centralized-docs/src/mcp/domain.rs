use crate::index::IndexDocument;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct IndexData {
    #[serde(default, deserialize_with = "null_to_default")]
    pub documents: Vec<IndexDocumentData>,
    #[serde(default, deserialize_with = "null_to_default")]
    pub chunks: Vec<ChunkData>,
    pub graph: Option<GraphData>,
}

fn null_to_default<'de, D, T>(de: D) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    use serde::Deserialize;
    let opt = Option::<Vec<T>>::deserialize(de)?;
    Ok(opt.unwrap_or_default())
}

#[derive(Debug, Deserialize)]
pub struct IndexDocumentData {
    pub id: Option<String>,
    pub doc_id: Option<String>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub category: String,
}

impl IndexDocumentData {
    #[must_use]
    pub fn get_id(&self) -> Option<&str> {
        self.id.as_deref().or(self.doc_id.as_deref())
    }

    #[must_use]
    pub fn into_index_document(self) -> Option<IndexDocument> {
        let id = self.get_id()?.to_string();
        let content = Arc::<str>::from(format!("{} {}", self.title, self.summary));
        Some(IndexDocument {
            id,
            content,
            path: self.path,
            category: self.category,
            title: self.title,
            summary: self.summary,
            tags: vec![],
            word_count: 0,
            chunk_ids: vec![],
            headings: vec![],
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct ChunkData {
    pub chunk_id: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GraphData {
    #[serde(default, deserialize_with = "null_to_default")]
    pub edges: Vec<GraphEdgeData>,
}

#[derive(Debug, Deserialize)]
pub struct GraphEdgeData {
    pub from: Option<String>,
    pub to: Option<String>,
    pub relationship_type: Option<String>,
}

impl IndexData {
    #[must_use]
    pub fn extract_documents(&self) -> Vec<IndexDocument> {
        // Clone the document data to create IndexDocuments for Tantivy indexing
        self.documents
            .iter()
            .filter_map(|d| {
                let id = d.get_id()?.to_string();
                let content = Arc::<str>::from(format!("{} {}", d.title, d.summary));
                Some(IndexDocument {
                    id,
                    content,
                    path: d.path.clone(),
                    category: d.category.clone(),
                    title: d.title.clone(),
                    summary: d.summary.clone(),
                    tags: vec![],
                    word_count: 0,
                    chunk_ids: vec![],
                    headings: vec![],
                })
            })
            .collect()
    }

    #[must_use]
    pub fn find_chunk_content(&self, id: &str) -> Option<String> {
        self.chunks.iter().find_map(|chunk| {
            if chunk.chunk_id.as_deref() == Some(id) {
                Some(chunk.content.clone().unwrap_or_default())
            } else {
                None
            }
        })
    }

    #[must_use]
    pub fn find_doc_summary(&self, id: &str) -> Option<String> {
        self.documents.iter().find_map(|doc| {
            if doc.get_id() == Some(id) {
                Some(doc.summary.clone())
            } else {
                None
            }
        })
    }

    #[must_use]
    pub fn find_related_edges(&self, id: &str) -> Vec<String> {
        let edges = match &self.graph {
            Some(g) => &g.edges,
            None => return Vec::new(),
        };

        edges
            .iter()
            .flat_map(|edge| {
                let from = edge.from.as_deref().unwrap_or_default();
                let to = edge.to.as_deref().unwrap_or_default();
                let rel_type = edge.relationship_type.as_deref().unwrap_or("related");

                let mut matches = Vec::new();
                if from == id && to == id {
                    // Self-referencing edge: emit single deduplicated entry
                    matches.push(format!("- {to} (Relationship: {rel_type} - self)"));
                } else if from == id {
                    matches.push(format!("- {to} (Relationship: {rel_type})"));
                } else if to == id {
                    matches.push(format!("- {from} (Relationship: {rel_type} - inbound)"));
                }
                matches
            })
            .collect()
    }
}
