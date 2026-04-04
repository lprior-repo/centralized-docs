use crate::mcp::domain::IndexData;
use crate::mcp::error::CtdMcpError;
use crate::mcp::types::{GetRelatedConceptsParams, ReadChunkParams, SearchDocsParams, ToolResult};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_handler, tool_router};
use std::path::PathBuf;
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct CtdMcpServer {
    pub index_dir: PathBuf,
    pub tool_router: rmcp::handler::server::router::tool::ToolRouter<Self>,
    pub state: std::sync::Arc<tokio::sync::OnceCell<ServerState>>,
}

#[derive(Debug)]
pub struct ServerState {
    pub index_data: IndexData,
    pub search_index: tantivy::Index,
}

#[tool_handler(router = self.tool_router)]
impl rmcp::ServerHandler for CtdMcpServer {}

#[tool_router(router = tool_router)]
impl CtdMcpServer {
    #[tool(name = "search_docs", description = "Search documentation")]
    #[instrument(skip(self), fields(query = %params.query))]
    async fn tool_search_docs(
        &self,
        Parameters(params): Parameters<SearchDocsParams>,
    ) -> Result<String, rmcp::ErrorData> {
        match self.search_docs(params).await {
            Ok(res) => Ok(res
                .content
                .into_iter()
                .map(|c| c.text)
                .collect::<Vec<_>>()
                .join("\n")),
            Err(e) => Err(map_error(e)),
        }
    }

    #[tool(name = "read_chunk", description = "Read a specific chunk")]
    #[instrument(skip(self), fields(id = %params.id))]
    async fn tool_read_chunk(
        &self,
        Parameters(params): Parameters<ReadChunkParams>,
    ) -> Result<String, rmcp::ErrorData> {
        match self.read_chunk(params).await {
            Ok(res) => Ok(res
                .content
                .into_iter()
                .map(|c| c.text)
                .collect::<Vec<_>>()
                .join("\n")),
            Err(e) => Err(map_error(e)),
        }
    }

    #[tool(name = "get_related_concepts", description = "Get related concepts")]
    #[instrument(skip(self), fields(id = %params.id))]
    async fn tool_get_related_concepts(
        &self,
        Parameters(params): Parameters<GetRelatedConceptsParams>,
    ) -> Result<String, rmcp::ErrorData> {
        match self.get_related_concepts(params).await {
            Ok(res) => Ok(res
                .content
                .into_iter()
                .map(|c| c.text)
                .collect::<Vec<_>>()
                .join("\n")),
            Err(e) => Err(map_error(e)),
        }
    }
}

fn map_error(e: CtdMcpError) -> rmcp::ErrorData {
    match e {
        CtdMcpError::InvalidInput { detail } => {
            rmcp::ErrorData::new(rmcp::model::ErrorCode::INVALID_PARAMS, detail, None)
        }
        CtdMcpError::IndexNotFound { .. }
        | CtdMcpError::IndexCorrupted { .. }
        | CtdMcpError::SearchIndexError { .. }
        | CtdMcpError::QueryError { .. }
        | CtdMcpError::IoError { .. }
        | CtdMcpError::Internal { .. } => {
            rmcp::ErrorData::new(rmcp::model::ErrorCode::INTERNAL_ERROR, e.to_string(), None)
        }
    }
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
            tool_router: Self::tool_router(),
            state: std::sync::Arc::new(tokio::sync::OnceCell::new()),
        })
    }

    #[instrument(skip(self))]
    async fn get_state(&self) -> Result<&ServerState, CtdMcpError> {
        self.state
            .get_or_try_init(|| async {
                let index_data = Self::load_index_data(&self.index_dir).await?;
                let search_index = Self::ensure_search_index(&self.index_dir, &index_data)?;
                Ok(ServerState {
                    index_data,
                    search_index,
                })
            })
            .await
    }

    #[instrument(skip_all)]
    async fn load_index_data(index_dir: &std::path::Path) -> Result<IndexData, CtdMcpError> {
        let path = index_dir.join("INDEX.json");
        if !path.exists() {
            return Err(CtdMcpError::IndexNotFound { path: path.clone() });
        }
        let content = tokio::fs::read_to_string(&path)
            .await
            .map_err(|e| CtdMcpError::IoError {
                reason: format!("{}: {e}", path.display()),
            })?;
        serde_json::from_str(&content).map_err(|e| CtdMcpError::IndexCorrupted {
            reason: e.to_string(),
        })
    }

    fn ensure_search_index(
        index_dir: &std::path::Path,
        index_data: &IndexData,
    ) -> Result<tantivy::Index, CtdMcpError> {
        let search_index = match crate::search::open_existing_index(index_dir) {
            Ok(Some(idx)) => idx,
            Ok(None) => Self::build_new_index(index_dir, index_data)?,
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
            Self::populate_index(&search_index, index_data)?;
        }
        Ok(search_index)
    }

    fn build_new_index(
        dir: &std::path::Path,
        index_data: &IndexData,
    ) -> Result<tantivy::Index, CtdMcpError> {
        let index = crate::search::rebuild_index_from_json(dir).map_err(|e| {
            CtdMcpError::SearchIndexError {
                reason: e.to_string(),
            }
        })?;
        Self::populate_index(&index, index_data)?;
        Ok(index)
    }

    fn populate_index(index: &tantivy::Index, index_data: &IndexData) -> Result<(), CtdMcpError> {
        let documents = index_data.extract_documents();
        if documents.is_empty() {
            return Ok(());
        }
        Self::write_documents_to_index(index, &documents)
    }

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

    #[must_use]
    pub fn format_search_results(results: &[crate::search::SearchResult]) -> String {
        if results.is_empty() {
            return "No results found.".to_string();
        }
        results.iter().enumerate().fold(String::new(), |mut acc, (i, r)| {
            use std::fmt::Write;
            let _ = write!(
                acc,
                "{rank}. [{category}] Score: {score:.4}\nTitle: {title}\nPath: {path}\nSummary: {summary}\n---\n",
                rank = i + 1,
                category = r.category,
                score = r.score.value(),
                title = r.title,
                path = r.path,
                summary = r.summary,
            );
            acc
        })
    }

    #[instrument(skip(self), fields(query = %params.query))]
    pub async fn search_docs(&self, params: SearchDocsParams) -> Result<ToolResult, CtdMcpError> {
        let valid_query = crate::mcp::types::ValidQuery::parse(&params.query)
            .map_err(|e| CtdMcpError::InvalidInput { detail: e })?;
        let valid_limit = crate::mcp::types::ValidLimit::parse(params.limit)
            .map_err(|e| CtdMcpError::InvalidInput { detail: e })?;

        let state = self.get_state().await?;
        let results = crate::search::search_index(
            &state.search_index,
            valid_query.as_str(),
            valid_limit.as_u32() as usize,
        )
        .map_err(|e| match e {
            crate::search::SearchError::EmptyQuery => CtdMcpError::InvalidInput {
                detail: "query must be non-empty".to_string(),
            },
            crate::search::SearchError::QueryParseError(reason) => {
                CtdMcpError::QueryError { reason }
            }
            crate::search::SearchError::PostconditionViolated => CtdMcpError::SearchIndexError {
                reason: "Postcondition violated".to_string(),
            },
            crate::search::SearchError::Other(err) => CtdMcpError::SearchIndexError {
                reason: err.to_string(),
            },
        })?;

        let formatted = Self::format_search_results(&results);
        Ok(ToolResult::text(formatted))
    }

    #[instrument(skip(self), fields(id = %params.id))]
    pub async fn read_chunk(&self, params: ReadChunkParams) -> Result<ToolResult, CtdMcpError> {
        let valid_id = crate::mcp::types::ValidId::parse(&params.id, "ID")
            .map_err(|e| CtdMcpError::InvalidInput { detail: e })?;

        let state = self.get_state().await?;

        if let Some(content) = state.index_data.find_chunk_content(valid_id.as_str()) {
            return Ok(ToolResult::text(content));
        }

        if let Some(summary) = state.index_data.find_doc_summary(valid_id.as_str()) {
            return Ok(ToolResult::text(format!(
                "Document {}:\nSummary:\n{}",
                valid_id.as_str(),
                summary
            )));
        }

        Ok(ToolResult::text(format!(
            "ID '{}' not found in chunks or documents",
            valid_id.as_str()
        )))
    }

    #[instrument(skip(self), fields(id = %params.id))]
    pub async fn get_related_concepts(
        &self,
        params: GetRelatedConceptsParams,
    ) -> Result<ToolResult, CtdMcpError> {
        let valid_id = crate::mcp::types::ValidId::parse(&params.id, "ID")
            .map_err(|e| CtdMcpError::InvalidInput { detail: e })?;

        let state = self.get_state().await?;
        let edges = state.index_data.find_related_edges(valid_id.as_str());

        if edges.is_empty() {
            return Ok(ToolResult::text(format!(
                "No related concepts found for ID '{}'",
                valid_id.as_str()
            )));
        }

        Ok(ToolResult::text(format!(
            "Related concepts for '{}':\n{}",
            valid_id.as_str(),
            edges.join("\n")
        )))
    }
}
