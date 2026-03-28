use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Value>,
}

pub fn run_mcp_serve(index_dir: &Path) -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let index_dir = index_dir.to_path_buf();

    for line_result in stdin.lock().lines() {
        let line = line_result?;
        if line.trim().is_empty() {
            continue;
        }

        match serde_json::from_str::<JsonRpcRequest>(&line) {
            Ok(req) => {
                let id = req.id.unwrap_or(Value::Null);

                if id.is_null() && req.method == "notifications/initialized" {
                    continue;
                }

                let response =
                    handle_request(&req.method, req.params.unwrap_or(Value::Null), &index_dir);

                let rpc_res = match response {
                    Ok(res) => JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id,
                        result: Some(res),
                        error: None,
                    },
                    Err(e) => JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id,
                        result: None,
                        error: Some(serde_json::json!({
                            "code": -32603,
                            "message": e.to_string()
                        })),
                    },
                };

                let res_str = serde_json::to_string(&rpc_res)?;
                writeln!(stdout, "{}", res_str)?;
                stdout.flush()?;
            }
            Err(e) => {
                let rpc_res = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: Value::Null,
                    result: None,
                    error: Some(serde_json::json!({
                        "code": -32700,
                        "message": format!("Parse error: {}", e)
                    })),
                };
                let res_str = serde_json::to_string(&rpc_res)?;
                writeln!(stdout, "{}", res_str)?;
                stdout.flush()?;
            }
        }
    }

    Ok(())
}

fn handle_request(method: &str, params: Value, index_dir: &PathBuf) -> Result<Value> {
    match method {
        "initialize" => Ok(serde_json::json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "ctd-mcp",
                "version": env!("CARGO_PKG_VERSION")
            }
        })),
        "mcp.tools/list" => Ok(serde_json::json!({
            "tools": [
                {
                    "name": "search_docs",
                    "description": "Search indexed documentation using BM25",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "query": {"type": "string", "description": "Search query"},
                            "limit": {"type": "integer", "description": "Max results (default: 10)"}
                        },
                        "required": ["query"]
                    }
                },
                {
                    "name": "read_chunk",
                    "description": "Read the exact content of a document or chunk",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "id": {"type": "string", "description": "The doc_id or chunk_id to read"}
                        },
                        "required": ["id"]
                    }
                },
                {
                    "name": "get_related_concepts",
                    "description": "Get related concepts/chunks based on the Knowledge Graph (DAG)",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "id": {"type": "string", "description": "The doc_id or chunk_id"}
                        },
                        "required": ["id"]
                    }
                }
            ]
        })),
        "mcp.tools/call" => {
            let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let args = params.get("arguments").unwrap_or(&Value::Null);

            match name {
                "search_docs" => tool_search_docs(args, index_dir),
                "read_chunk" => tool_read_chunk(args, index_dir),
                "get_related_concepts" => tool_get_related_concepts(args, index_dir),
                _ => Err(anyhow::anyhow!("Unknown tool: {}", name)),
            }
        }
        _ => Err(anyhow::anyhow!("Method not found: {}", method)),
    }
}

fn tool_search_docs(args: &Value, index_dir: &PathBuf) -> Result<Value> {
    let query = args
        .get("query")
        .and_then(|v| v.as_str())
        .context("Missing 'query' argument")?;
    let limit = args.get("limit").and_then(|v| v.as_i64()).unwrap_or(10) as usize;

    let index_path = index_dir.join("INDEX.json");
    if !index_path.exists() {
        anyhow::bail!("INDEX.json not found in {}", index_dir.display());
    }

    let index = match crate::search::open_existing_index(index_dir)? {
        Some(idx) => idx,
        None => crate::search::rebuild_index_from_json(index_dir)?,
    };

    let search_results = crate::search::search_index(&index, query, limit)?;

    let output = if search_results.is_empty() {
        "No results found.".to_string()
    } else {
        let mut out = String::new();
        for (i, result) in search_results.iter().enumerate() {
            out.push_str(&format!(
                "{}. [{}] Score: {:.4}\n",
                i + 1,
                result.category,
                result.score.value()
            ));
            out.push_str(&format!("Title: {}\n", result.title));
            out.push_str(&format!("Path: {}\n", result.path));
            out.push_str(&format!("Summary: {}\n", result.summary));
            out.push_str("---\n");
        }
        out
    };

    Ok(serde_json::json!({
        "content": [
            {
                "type": "text",
                "text": output
            }
        ]
    }))
}

fn tool_read_chunk(args: &Value, index_dir: &PathBuf) -> Result<Value> {
    let id = args
        .get("id")
        .and_then(|v| v.as_str())
        .context("Missing 'id' argument")?;

    let index_path = index_dir.join("INDEX.json");
    if !index_path.exists() {
        anyhow::bail!("INDEX.json not found in {}", index_dir.display());
    }

    let file_content = std::fs::read_to_string(&index_path)?;
    let index_data: serde_json::Value = serde_json::from_str(&file_content)?;

    if let Some(chunks) = index_data.get("chunks").and_then(|c| c.as_array()) {
        for chunk in chunks {
            if chunk.get("chunk_id").and_then(|i| i.as_str()) == Some(id) {
                let content = chunk.get("content").and_then(|c| c.as_str()).unwrap_or("");
                return Ok(serde_json::json!({
                    "content": [
                        {
                            "type": "text",
                            "text": content
                        }
                    ]
                }));
            }
        }
    }

    if let Some(docs) = index_data.get("documents").and_then(|d| d.as_array()) {
        for doc in docs {
            if doc.get("doc_id").and_then(|i| i.as_str()) == Some(id) {
                let content = doc.get("summary").and_then(|c| c.as_str()).unwrap_or("");
                return Ok(serde_json::json!({
                    "content": [
                        {
                            "type": "text",
                            "text": format!("Document {}:\nSummary:\n{}", id, content)
                        }
                    ]
                }));
            }
        }
    }

    Ok(serde_json::json!({
        "content": [
            {
                "type": "text",
                "text": format!("ID '{}' not found in chunks or documents", id)
            }
        ]
    }))
}

fn tool_get_related_concepts(args: &Value, index_dir: &PathBuf) -> Result<Value> {
    let id = args
        .get("id")
        .and_then(|v| v.as_str())
        .context("Missing 'id' argument")?;

    let index_path = index_dir.join("INDEX.json");
    if !index_path.exists() {
        anyhow::bail!("INDEX.json not found in {}", index_dir.display());
    }

    let file_content = std::fs::read_to_string(&index_path)?;
    let index_data: serde_json::Value = serde_json::from_str(&file_content)?;

    let mut related = Vec::new();

    if let Some(edges) = index_data
        .get("graph")
        .and_then(|g| g.get("edges"))
        .and_then(|e| e.as_array())
    {
        for edge in edges {
            let from = edge.get("from").and_then(|v| v.as_str());
            let to = edge.get("to").and_then(|v| v.as_str());
            let rel_type = edge
                .get("relationship_type")
                .and_then(|v| v.as_str())
                .unwrap_or("related");

            if from == Some(id) {
                if let Some(t) = to {
                    related.push(format!("- {} (Relationship: {})", t, rel_type));
                }
            } else if to == Some(id) {
                if let Some(f) = from {
                    related.push(format!("- {} (Relationship: {} - inbound)", f, rel_type));
                }
            }
        }
    }

    let output = if related.is_empty() {
        format!("No related concepts found for ID '{}'", id)
    } else {
        format!("Related concepts for '{}':\n{}", id, related.join("\n"))
    };

    Ok(serde_json::json!({
        "content": [
            {
                "type": "text",
                "text": output
            }
        ]
    }))
}
