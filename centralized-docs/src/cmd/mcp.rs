use anyhow::Result;
use std::path::Path;
use tracing::instrument;

#[instrument(skip_all, fields(index_dir = %index_dir.display()))]
pub async fn run_mcp_serve(index_dir: &Path) -> Result<()> {
    if !index_dir.exists() {
        anyhow::bail!("INDEX_DIR does not exist: {}", index_dir.display());
    }
    if !index_dir.join("INDEX.json").exists() {
        anyhow::bail!(
            "INDEX.json not found in {} — is this a valid ctd output directory?",
            index_dir.display()
        );
    }
    doc_transformer::mcp::run_mcp_serve(index_dir).await
}
