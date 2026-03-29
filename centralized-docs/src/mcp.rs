#![allow(clippy::unused_async)]

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CtdMcpError {
    #[error("INDEX.json not found in {path}")]
    IndexNotFound { path: String },

    #[error("Failed to parse INDEX.json: {reason}")]
    IndexCorrupted { reason: String },

    #[error("Invalid input: {detail}")]
    InvalidInput { detail: String },

    #[error("Search index error: {reason}")]
    SearchIndexError { reason: String },

    #[error("Query error: {reason}")]
    QueryError { reason: String },

    #[error("I/O error: {reason}")]
    IoError { reason: String },

    #[error("Internal error: {reason}")]
    Internal { reason: String },
}

#[derive(Debug, Clone)]
pub struct ToolContent {
    pub content_type: String,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct ToolResult {
    pub content: Vec<ToolContent>,
    pub is_error: bool,
}

impl ToolResult {
    pub fn text(text: impl Into<String>) -> Self {
        ToolResult {
            content: vec![ToolContent {
                content_type: "text".to_string(),
                text: text.into(),
            }],
            is_error: false,
        }
    }

    pub fn error(text: impl Into<String>) -> Self {
        ToolResult {
            content: vec![ToolContent {
                content_type: "text".to_string(),
                text: text.into(),
            }],
            is_error: true,
        }
    }

    #[must_use]
    pub fn text_content(&self) -> Option<&str> {
        self.content.first().map(|c| c.text.as_str())
    }
}

fn default_limit() -> u32 {
    10
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SearchDocsParams {
    pub query: String,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ReadChunkParams {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetRelatedConceptsParams {
    pub id: String,
}

#[derive(Debug)]
pub struct CtdMcpServer {
    pub index_dir: PathBuf,
}

impl CtdMcpServer {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(index_dir: PathBuf) -> Result<Self, CtdMcpError> {
        let metadata = std::fs::metadata(&index_dir).map_err(|e| CtdMcpError::IoError {
            reason: format!("{}: {e}", index_dir.display()),
        })?;

        if !metadata.is_dir() {
            return Err(CtdMcpError::IoError {
                reason: format!("{} is not a directory", index_dir.display()),
            });
        }

        let canonicalized =
            std::fs::canonicalize(&index_dir).map_err(|e| CtdMcpError::IoError {
                reason: format!("Failed to canonicalize {}: {e}", index_dir.display()),
            })?;

        Ok(CtdMcpServer {
            index_dir: canonicalized,
        })
    }

    pub fn load_index_json(&self) -> Result<serde_json::Value, CtdMcpError> {
        let path = self.index_dir.join("INDEX.json");

        if !path.exists() {
            return Err(CtdMcpError::IndexNotFound {
                path: path.display().to_string(),
            });
        }

        let content = std::fs::read_to_string(&path).map_err(|e| CtdMcpError::IoError {
            reason: format!("{}: {e}", path.display()),
        })?;

        serde_json::from_str(&content).map_err(|e| CtdMcpError::IndexCorrupted {
            reason: e.to_string(),
        })
    }

    pub fn open_or_rebuild_search_index(&self) -> Result<tantivy::Index, CtdMcpError> {
        match crate::search::open_existing_index(&self.index_dir) {
            Ok(Some(idx)) => return Ok(idx),
            Ok(None) => {}
            Err(e) => {
                return Err(CtdMcpError::SearchIndexError {
                    reason: e.to_string(),
                })
            }
        }

        let index = crate::search::rebuild_index_from_json(&self.index_dir).map_err(|e| {
            CtdMcpError::SearchIndexError {
                reason: e.to_string(),
            }
        })?;

        let has_docs = index
            .reader()
            .ok()
            .is_some_and(|r| r.searcher().num_docs() > 0);

        if has_docs {
            return Ok(index);
        }

        self.index_documents_from_json(&index)?;

        Ok(index)
    }

    fn index_documents_from_json(&self, index: &tantivy::Index) -> Result<(), CtdMcpError> {
        let index_data = self.load_index_json()?;

        let documents: Vec<crate::index::IndexDocument> = index_data
            .get("documents")
            .and_then(serde_json::Value::as_array)
            .map_or_else(Vec::new, |arr| {
                arr.iter()
                    .filter_map(|doc| {
                        let id = doc
                            .get("id")
                            .or_else(|| doc.get("doc_id"))
                            .and_then(serde_json::Value::as_str)?;

                        let title = doc
                            .get("title")
                            .and_then(serde_json::Value::as_str)
                            .map_or_else(String::new, String::from);

                        let summary = doc
                            .get("summary")
                            .and_then(serde_json::Value::as_str)
                            .map_or_else(String::new, String::from);

                        let path = doc
                            .get("path")
                            .and_then(serde_json::Value::as_str)
                            .map_or_else(String::new, String::from);

                        let category = doc
                            .get("category")
                            .and_then(serde_json::Value::as_str)
                            .map_or_else(String::new, String::from);

                        let content: std::sync::Arc<str> =
                            std::sync::Arc::<str>::from(format!("{title} {summary}"));

                        Some(crate::index::IndexDocument {
                            id: id.to_string(),
                            title,
                            path,
                            category,
                            tags: vec![],
                            summary,
                            word_count: 0,
                            chunk_ids: vec![],
                            headings: vec![],
                            content,
                        })
                    })
                    .collect()
            });

        if documents.is_empty() {
            return Ok(());
        }

        let mut writer = index
            .writer(50_000_000)
            .map_err(|e| CtdMcpError::SearchIndexError {
                reason: e.to_string(),
            })?;

        crate::search::index_documents(&mut writer, &documents).map_err(|e| {
            CtdMcpError::SearchIndexError {
                reason: e.to_string(),
            }
        })?;

        writer.commit().map_err(|e| CtdMcpError::SearchIndexError {
            reason: e.to_string(),
        })?;

        Ok(())
    }

    pub fn validate_search_params(&self, params: &SearchDocsParams) -> Result<(), CtdMcpError> {
        if params.query.trim().is_empty() {
            return Err(CtdMcpError::InvalidInput {
                detail: "query must be non-empty".to_string(),
            });
        }
        if params.limit == 0 {
            return Err(CtdMcpError::InvalidInput {
                detail: "limit must be > 0".to_string(),
            });
        }
        if params.limit > 100 {
            return Err(CtdMcpError::InvalidInput {
                detail: "limit must be <= 100".to_string(),
            });
        }
        Ok(())
    }

    pub fn validate_id_param(&self, id: &str) -> Result<(), CtdMcpError> {
        if id.trim().is_empty() {
            return Err(CtdMcpError::InvalidInput {
                detail: "id must be non-empty".to_string(),
            });
        }
        Ok(())
    }

    #[must_use]
    pub fn format_search_results(results: &[crate::search::SearchResult]) -> String {
        if results.is_empty() {
            return "No results found.".to_string();
        }

        let mut output = String::new();
        for (i, r) in results.iter().enumerate() {
            output.push_str(&format!(
                "{rank}. [{category}] Score: {score:.4}\nTitle: {title}\nPath: {path}\nSummary: {summary}\n---\n",
                rank = i + 1,
                category = r.category,
                score = r.score.value(),
                title = r.title,
                path = r.path,
                summary = r.summary,
            ));
        }
        output
    }

    pub fn find_chunk_content(index_data: &serde_json::Value, id: &str) -> Option<String> {
        index_data
            .get("chunks")
            .and_then(serde_json::Value::as_array)
            .and_then(|chunks| {
                chunks.iter().find_map(|chunk| {
                    chunk
                        .get("chunk_id")
                        .and_then(serde_json::Value::as_str)
                        .filter(|&chunk_id| chunk_id == id)
                        .map(|_| {
                            chunk
                                .get("content")
                                .and_then(serde_json::Value::as_str)
                                .map_or_else(String::new, String::from)
                        })
                })
            })
    }

    pub fn find_doc_summary(index_data: &serde_json::Value, id: &str) -> Option<String> {
        index_data
            .get("documents")
            .and_then(serde_json::Value::as_array)
            .and_then(|docs| {
                docs.iter().find_map(|doc| {
                    doc.get("doc_id")
                        .and_then(serde_json::Value::as_str)
                        .filter(|&doc_id| doc_id == id)
                        .and_then(|_| {
                            doc.get("summary")
                                .and_then(serde_json::Value::as_str)
                                .map(String::from)
                        })
                })
            })
    }

    pub fn find_related_edges(index_data: &serde_json::Value, id: &str) -> Vec<String> {
        let edges = match index_data
            .get("graph")
            .and_then(|g| g.get("edges"))
            .and_then(serde_json::Value::as_array)
        {
            Some(e) => e,
            None => return Vec::new(),
        };

        edges
            .iter()
            .flat_map(|edge| {
                let from = edge
                    .get("from")
                    .and_then(serde_json::Value::as_str)
                    .map_or_else(String::new, String::from);
                let to = edge
                    .get("to")
                    .and_then(serde_json::Value::as_str)
                    .map_or_else(String::new, String::from);
                let rel_type = edge
                    .get("relationship_type")
                    .and_then(serde_json::Value::as_str)
                    .map_or_else(|| "related".to_string(), String::from);

                let from_match = (from == id).then(|| format!("- {to} (Relationship: {rel_type})"));
                let to_match =
                    (to == id).then(|| format!("- {from} (Relationship: {rel_type} - inbound)"));

                from_match.into_iter().chain(to_match)
            })
            .collect()
    }

    pub async fn search_docs(&self, params: SearchDocsParams) -> Result<ToolResult, CtdMcpError> {
        self.validate_search_params(&params)?;
        self.load_index_json()?;
        let index = self.open_or_rebuild_search_index()?;

        let results = crate::search::search_index(&index, &params.query, params.limit as usize)
            .map_err(|e| match e {
                crate::search::SearchError::EmptyQuery => CtdMcpError::InvalidInput {
                    detail: "query must be non-empty".to_string(),
                },
                crate::search::SearchError::QueryParseError(reason) => {
                    CtdMcpError::QueryError { reason }
                }
                crate::search::SearchError::PostconditionViolated => {
                    CtdMcpError::SearchIndexError {
                        reason: "Postcondition violated".to_string(),
                    }
                }
                crate::search::SearchError::Other(err) => CtdMcpError::SearchIndexError {
                    reason: err.to_string(),
                },
            })?;

        let formatted = Self::format_search_results(&results);
        Ok(ToolResult::text(formatted))
    }

    pub async fn read_chunk(&self, params: ReadChunkParams) -> Result<ToolResult, CtdMcpError> {
        self.validate_id_param(&params.id)?;
        let index_data = self.load_index_json()?;

        if let Some(content) = Self::find_chunk_content(&index_data, &params.id) {
            return Ok(ToolResult::text(content));
        }

        if let Some(summary) = Self::find_doc_summary(&index_data, &params.id) {
            return Ok(ToolResult::text(format!(
                "Document {}:\nSummary:\n{}",
                params.id, summary
            )));
        }

        Ok(ToolResult::text(format!(
            "ID '{}' not found in chunks or documents",
            params.id
        )))
    }

    pub async fn get_related_concepts(
        &self,
        params: GetRelatedConceptsParams,
    ) -> Result<ToolResult, CtdMcpError> {
        self.validate_id_param(&params.id)?;
        let index_data = self.load_index_json()?;
        let edges = Self::find_related_edges(&index_data, &params.id);

        if edges.is_empty() {
            return Ok(ToolResult::text(format!(
                "No related concepts found for ID '{}'",
                params.id
            )));
        }

        Ok(ToolResult::text(format!(
            "Related concepts for '{}':\n{}",
            params.id,
            edges.join("\n")
        )))
    }
}

pub async fn run(index_dir: PathBuf) -> Result<(), CtdMcpError> {
    let _ = index_dir;
    Err(CtdMcpError::Internal {
        reason: "not implemented".to_string(),
    })
}
