use anyhow::Result;
use std::path::Path;

pub fn run_mcp_serve(index_dir: &Path) -> Result<()> {
    let _server = doc_transformer::mcp::CtdMcpServer::new(index_dir.to_path_buf())?;
    Err(anyhow::anyhow!("rmcp SDK transport not yet integrated"))
}
