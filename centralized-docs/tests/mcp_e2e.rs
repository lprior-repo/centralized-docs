#![allow(clippy::unwrap_used, clippy::expect_used)]

//! End-to-end MCP protocol tests using in-memory duplex transport.
//!
//! These tests exercise the full JSON-RPC protocol stack:
//!   1. Create `CtdMcpServer` backed by a temp directory with `INDEX.json`
//!   2. Wire client <-> server over `tokio::io::duplex` (no subprocess)
//!   3. Call `initialize`, `tools/list`, `tools/call` through the rmcp client
//!   4. Assert on wire-level responses

use doc_transformer::mcp::CtdMcpServer;
use rmcp::handler::server::router::Router;
use rmcp::model::CallToolRequestParams;
use rmcp::{serve_client, serve_server, RoleClient};
use serde_json::json;
use tempfile::TempDir;
use tracing::Instrument;

// ---------------------------------------------------------------------------
// Test fixtures
// ---------------------------------------------------------------------------

fn write_index_json(dir: &TempDir) {
    let index = json!({
        "documents": [
            {"doc_id": "doc-k8s-pods", "title": "Kubernetes Pods", "summary": "Learn about Kubernetes pods and container management", "category": "tutorial", "path": "pods.md"},
            {"doc_id": "doc-k8s-svc", "title": "Kubernetes Services", "summary": "Service types in Kubernetes ClusterIP NodePort LoadBalancer", "category": "ref", "path": "services.md"},
            {"doc_id": "doc-k8s-deploy", "title": "Kubernetes Deployments", "summary": "Deploy applications to Kubernetes clusters", "category": "tutorial", "path": "deployment.md"}
        ],
        "chunks": [
            {"chunk_id": "chunk-pods-1", "content": "Kubernetes pods are the fundamental units of deployment. A pod encapsulates one or more containers."},
            {"chunk_id": "chunk-pods-2", "content": "Pod lifecycle includes phases: Pending, Running, Succeeded, Failed."},
            {"chunk_id": "chunk-svc-1", "content": "Kubernetes Services expose an abstract networking layer for pod access."},
            {"chunk_id": "chunk-svc-2", "content": "Service types include ClusterIP, NodePort, LoadBalancer, and ExternalName."}
        ],
        "graph": {
            "edges": [
                {"from": "doc-k8s-pods", "to": "doc-k8s-svc", "relationship_type": "Parent"},
                {"from": "doc-k8s-pods", "to": "doc-k8s-deploy", "relationship_type": "Parent"},
                {"from": "doc-k8s-svc", "to": "doc-k8s-deploy", "relationship_type": "Related"},
                {"from": "chunk-pods-1", "to": "chunk-pods-2", "relationship_type": "Sequential"},
                {"from": "chunk-svc-1", "to": "chunk-svc-2", "relationship_type": "Sequential"}
            ]
        }
    });
    let path = dir.path().join("INDEX.json");
    std::fs::write(path, serde_json::to_string(&index).unwrap()).unwrap();
}

fn make_server(dir: &TempDir) -> CtdMcpServer {
    CtdMcpServer::new(dir.path().to_path_buf()).expect("CtdMcpServer::new should succeed")
}

fn make_router(server: &CtdMcpServer) -> Router<CtdMcpServer> {
    let mut router = Router::new(server.clone());
    router.tool_router = server.tool_router.clone();
    router
}

/// Extract all text from a `CallToolResult`'s content blocks.
fn extract_text(result: &rmcp::model::CallToolResult) -> String {
    result
        .content
        .iter()
        .filter_map(|c| c.raw.as_text().map(|t| t.text.as_str()))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Spawn server + client on a duplex transport pair. Returns the client handle.
async fn setup_client(dir: &TempDir) -> rmcp::service::RunningService<RoleClient, ()> {
    let server = make_server(dir);
    let router = make_router(&server);

    let (client_side, server_side) = tokio::io::duplex(4096);

    // Server runs in background; errors will surface as client-side transport failures.
    tokio::spawn(
        async move {
            let running = match serve_server(router, server_side).await {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("server init error (may be expected): {e}");
                    return;
                }
            };
            let _ = running.waiting().await;
        }
        .instrument(tracing::info_span!("mcp_e2e_server")),
    );

    serve_client((), client_side)
        .await
        .expect("client should connect to server")
}

// ---------------------------------------------------------------------------
// Test 1: Server initializes and client connects
// ---------------------------------------------------------------------------

#[tokio::test]
async fn server_initializes_and_client_connects() {
    let dir = TempDir::new().unwrap();
    write_index_json(&dir);
    let client = setup_client(&dir).await;

    let peer_info = client.peer().peer_info();
    assert!(
        peer_info.is_some(),
        "client should have received server info during initialize"
    );
}

// ---------------------------------------------------------------------------
// Test 2: Client lists available tools
// ---------------------------------------------------------------------------

#[tokio::test]
async fn client_lists_available_tools() {
    let dir = TempDir::new().unwrap();
    write_index_json(&dir);
    let client = setup_client(&dir).await;

    let result = client
        .list_tools(None)
        .await
        .expect("list_tools should succeed");

    let tool_names: Vec<&str> = result.tools.iter().map(|t| t.name.as_ref()).collect();

    assert_eq!(
        tool_names.len(),
        3,
        "expected exactly 3 tools, got {tool_names:?}"
    );
    assert!(tool_names.contains(&"search_docs"), "missing search_docs");
    assert!(tool_names.contains(&"read_chunk"), "missing read_chunk");
    assert!(
        tool_names.contains(&"get_related_concepts"),
        "missing get_related_concepts"
    );
}

// ---------------------------------------------------------------------------
// Test 3: search_docs returns results for valid query
// ---------------------------------------------------------------------------

#[tokio::test]
async fn search_docs_returns_results_for_valid_query() {
    let dir = TempDir::new().unwrap();
    write_index_json(&dir);
    let client = setup_client(&dir).await;

    let params = CallToolRequestParams::new("search_docs").with_arguments(
        json!({"query": "kubernetes", "limit": 10})
            .as_object()
            .unwrap()
            .clone(),
    );

    let result = client
        .call_tool(params)
        .await
        .expect("call_tool search_docs should succeed");

    let text = extract_text(&result);
    assert!(
        text.to_lowercase().contains("kubernetes"),
        "search result should contain 'kubernetes', got: {text}"
    );
    assert!(
        text.contains("Score:"),
        "search result should contain score, got: {text}"
    );
}

// ---------------------------------------------------------------------------
// Test 4: search_docs returns no results for nonsense query
// ---------------------------------------------------------------------------

#[tokio::test]
async fn search_docs_returns_no_results_for_nonsense_query() {
    let dir = TempDir::new().unwrap();
    write_index_json(&dir);
    let client = setup_client(&dir).await;

    let params = CallToolRequestParams::new("search_docs").with_arguments(
        json!({"query": "xyzzyplughnothingexist42", "limit": 10})
            .as_object()
            .unwrap()
            .clone(),
    );

    let result = client
        .call_tool(params)
        .await
        .expect("call_tool search_docs should succeed");

    let text = extract_text(&result);
    assert!(
        text.contains("No results found"),
        "expected 'No results found' for nonsense query, got: {text}"
    );
}

// ---------------------------------------------------------------------------
// Test 5: read_chunk returns content for valid chunk ID
// ---------------------------------------------------------------------------

#[tokio::test]
async fn read_chunk_returns_content_for_valid_id() {
    let dir = TempDir::new().unwrap();
    write_index_json(&dir);
    let client = setup_client(&dir).await;

    let params = CallToolRequestParams::new("read_chunk")
        .with_arguments(json!({"id": "chunk-pods-1"}).as_object().unwrap().clone());

    let result = client
        .call_tool(params)
        .await
        .expect("call_tool read_chunk should succeed");

    let text = extract_text(&result);
    assert!(
        text.contains("Kubernetes pods are the fundamental units"),
        "expected chunk content, got: {text}"
    );
}

// ---------------------------------------------------------------------------
// Test 6: read_chunk returns not-found for invalid ID
// ---------------------------------------------------------------------------

#[tokio::test]
async fn read_chunk_returns_not_found_for_invalid_id() {
    let dir = TempDir::new().unwrap();
    write_index_json(&dir);
    let client = setup_client(&dir).await;

    let params = CallToolRequestParams::new("read_chunk").with_arguments(
        json!({"id": "nonexistent-chunk-999"})
            .as_object()
            .unwrap()
            .clone(),
    );

    let result = client
        .call_tool(params)
        .await
        .expect("call_tool read_chunk should succeed");

    let text = extract_text(&result);
    assert!(
        text.contains("not found"),
        "expected 'not found' message for invalid chunk ID, got: {text}"
    );
}

// ---------------------------------------------------------------------------
// Test 7: get_related_concepts returns edges for valid ID
// ---------------------------------------------------------------------------

#[tokio::test]
async fn get_related_concepts_returns_edges_for_valid_id() {
    let dir = TempDir::new().unwrap();
    write_index_json(&dir);
    let client = setup_client(&dir).await;

    let params = CallToolRequestParams::new("get_related_concepts")
        .with_arguments(json!({"id": "doc-k8s-pods"}).as_object().unwrap().clone());

    let result = client
        .call_tool(params)
        .await
        .expect("call_tool get_related_concepts should succeed");

    let text = extract_text(&result);
    assert!(
        text.contains("doc-k8s-svc"),
        "expected 'doc-k8s-svc' in related concepts, got: {text}"
    );
    assert!(
        text.contains("doc-k8s-deploy"),
        "expected 'doc-k8s-deploy' in related concepts, got: {text}"
    );
}
