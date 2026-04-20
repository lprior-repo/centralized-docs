#![allow(clippy::unwrap_used, clippy::expect_used)]

use rmcp::model::CallToolRequestParams;
use rmcp::{serve_client, RoleClient};
use serde_json::json;
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;
use tempfile::TempDir;
use tokio::process::Command;
use tokio::time::timeout;

fn binary_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_BIN_EXE_ctd")).to_path_buf()
}

fn write_index_json(dir: &TempDir) {
    let index = json!({
        "documents": [
            {
                "doc_id": "doc-alpha",
                "title": "Alpha Guide",
                "summary": "alphaorchid setup instructions",
                "category": "tutorial",
                "path": "alpha.md"
            }
        ],
        "chunks": [
            {
                "chunk_id": "chunk-alpha-1",
                "content": "alphaorchid is the unique keyword for subprocess MCP validation"
            }
        ],
        "graph": {
            "edges": [
                {
                    "from": "doc-alpha",
                    "to": "chunk-alpha-1",
                    "relationship_type": "Parent"
                }
            ]
        }
    });

    let path = dir.path().join("INDEX.json");
    std::fs::write(path, serde_json::to_string(&index).unwrap()).unwrap();
}

fn extract_text(result: &rmcp::model::CallToolResult) -> String {
    result
        .content
        .iter()
        .filter_map(|content| content.raw.as_text().map(|text| text.text.as_str()))
        .collect::<Vec<_>>()
        .join("\n")
}

async fn spawn_client(
    dir: &TempDir,
) -> (
    rmcp::service::RunningService<RoleClient, ()>,
    tokio::process::Child,
) {
    let mut child = Command::new(binary_path())
        .args(["mcp", "serve", dir.path().to_str().unwrap()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("ctd mcp serve should spawn");

    let stdout = child.stdout.take().expect("child stdout should exist");
    let stdin = child.stdin.take().expect("child stdin should exist");
    let transport = rmcp::transport::IntoTransport::into_transport(
        rmcp::transport::async_rw::AsyncRwTransport::new(stdout, stdin),
    );
    let client = serve_client((), transport)
        .await
        .expect("client should initialize over stdio");

    (client, child)
}

async fn assert_clean_exit(mut child: tokio::process::Child) {
    let status = timeout(Duration::from_secs(3), child.wait())
        .await
        .expect("child should exit after EOF")
        .expect("child wait should succeed");
    assert!(status.success(), "child should exit successfully: {status}");
}

#[tokio::test]
async fn subprocess_stdio_session_lists_tools_and_serves_requests() {
    let dir = TempDir::new().unwrap();
    write_index_json(&dir);

    let (client, child) = spawn_client(&dir).await;
    let tools = client
        .list_tools(None)
        .await
        .expect("tools/list should work");
    let tool_names: Vec<&str> = tools.tools.iter().map(|tool| tool.name.as_ref()).collect();
    assert_eq!(tool_names.len(), 3, "unexpected tools: {tool_names:?}");
    assert!(tool_names.contains(&"search_docs"));
    assert!(tool_names.contains(&"read_chunk"));
    assert!(tool_names.contains(&"get_related_concepts"));

    let search = client
        .call_tool(
            CallToolRequestParams::new("search_docs").with_arguments(
                json!({"query": "alphaorchid", "limit": 5})
                    .as_object()
                    .unwrap()
                    .clone(),
            ),
        )
        .await
        .expect("search_docs should succeed");
    assert!(extract_text(&search).contains("alphaorchid"));

    let chunk = client
        .call_tool(
            CallToolRequestParams::new("read_chunk")
                .with_arguments(json!({"id": "chunk-alpha-1"}).as_object().unwrap().clone()),
        )
        .await
        .expect("read_chunk should succeed");
    assert!(extract_text(&chunk).contains("subprocess MCP validation"));

    drop(client);
    assert_clean_exit(child).await;
}

#[tokio::test]
async fn subprocess_stdio_session_survives_invalid_request_then_valid_request() {
    let dir = TempDir::new().unwrap();
    write_index_json(&dir);

    let (client, child) = spawn_client(&dir).await;
    let invalid = client
        .call_tool(
            CallToolRequestParams::new("search_docs").with_arguments(
                json!({"query": "alphaorchid", "limit": 0})
                    .as_object()
                    .unwrap()
                    .clone(),
            ),
        )
        .await;
    assert!(
        invalid.is_err(),
        "invalid limit should return protocol error"
    );

    let valid = client
        .call_tool(
            CallToolRequestParams::new("search_docs").with_arguments(
                json!({"query": "alphaorchid", "limit": 1})
                    .as_object()
                    .unwrap()
                    .clone(),
            ),
        )
        .await
        .expect("server should still answer after invalid request");
    assert!(extract_text(&valid).contains("Alpha Guide"));

    drop(client);
    assert_clean_exit(child).await;
}
