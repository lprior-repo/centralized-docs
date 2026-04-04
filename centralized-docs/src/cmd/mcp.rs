use anyhow::Result;
use std::path::Path;
use tracing::instrument;

#[instrument(skip_all, fields(index_dir = %index_dir.display()))]
pub async fn run_mcp_serve(index_dir: &Path) -> Result<()> {
    doc_transformer::mcp::run_mcp_serve(index_dir).await
}
