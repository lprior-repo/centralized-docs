#![allow(clippy::unused_async)]

pub mod domain;
pub mod error;
pub mod server;
pub mod types;

use std::path::PathBuf;

pub use error::CtdMcpError;
pub use server::CtdMcpServer;

pub async fn run(index_dir: PathBuf) -> Result<(), CtdMcpError> {
    run_mcp_serve(&index_dir).await.map_err(|e| {
        if let Some(err) = e.downcast_ref::<CtdMcpError>() {
            return CtdMcpError::IoError {
                reason: err.to_string(),
            };
        }
        CtdMcpError::Internal {
            reason: e.to_string(),
        }
    })
}

pub async fn run_mcp_serve(index_dir: &std::path::Path) -> anyhow::Result<()> {
    let server = CtdMcpServer::new(index_dir.to_path_buf()).map_err(|e| anyhow::anyhow!(e))?;
    let mut router = rmcp::handler::server::router::Router::new(server.clone());
    router.tool_router = server.tool_router.clone();

    let transport = rmcp::transport::IntoTransport::into_transport(
        rmcp::transport::async_rw::AsyncRwTransport::new(tokio::io::stdin(), tokio::io::stdout()),
    );

    let handle = match rmcp::service::serve_server(router, transport).await {
        Ok(h) => h,
        Err(e) => {
            if is_eof_error(&e) {
                return Ok(());
            }
            return Err(e.into());
        }
    };

    if let Err(e) = handle.waiting().await {
        if !is_eof_error(&e) {
            return Err(e.into());
        }
    }
    Ok(())
}

fn is_eof_error(e: &(dyn std::error::Error + 'static)) -> bool {
    if e.to_string()
        .contains("connection closed: initialize request")
    {
        return true;
    }
    let mut current: Option<&(dyn std::error::Error + 'static)> = Some(e);
    while let Some(err) = current {
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return matches!(
                io_err.kind(),
                std::io::ErrorKind::UnexpectedEof
                    | std::io::ErrorKind::BrokenPipe
                    | std::io::ErrorKind::ConnectionAborted
                    | std::io::ErrorKind::ConnectionReset
                    | std::io::ErrorKind::NotConnected
            );
        }
        current = err.source();
    }
    false
}
