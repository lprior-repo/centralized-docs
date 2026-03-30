use anyhow::Result;
use std::path::Path;

pub async fn run_mcp_serve(index_dir: &Path) -> Result<()> {
    doc_transformer::mcp::run_mcp_serve(index_dir).await
}
