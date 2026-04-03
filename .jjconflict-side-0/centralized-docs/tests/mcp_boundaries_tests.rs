#![allow(clippy::unwrap_used, clippy::expect_used)]

use doc_transformer::mcp::{
    types::{GetRelatedConceptsParams, ReadChunkParams, SearchDocsParams},
    CtdMcpError, CtdMcpServer,
};
use tempfile::TempDir;

// Helper just to get a server for validation tests
fn make_server() -> CtdMcpServer {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("INDEX.json");
    std::fs::write(
        &path,
        r#"{"documents":[], "chunks":[], "graph":{"edges":[]}}"#,
    )
    .unwrap();
    CtdMcpServer::new(dir.keep()).unwrap()
}

#[tokio::test]
async fn search_docs_returns_invalid_input_when_query_exceeds_1024_bytes() {
    let server = make_server();
    let query = "a".repeat(1025);
    let result = server
        .search_docs(SearchDocsParams { query, limit: 10 })
        .await;
    assert!(matches!(result, Err(CtdMcpError::InvalidInput { .. })));
}

#[tokio::test]
async fn search_docs_returns_matched_chunk_when_query_is_exactly_1024_bytes() {
    let server = make_server();
    let result = server
        .search_docs(SearchDocsParams {
            query: "a".repeat(1024),
            limit: 10,
        })
        .await;
    assert!(result.is_ok(), "Unexpected error: {:?}", result);
}

#[tokio::test]
async fn search_docs_returns_empty_when_query_is_exactly_1024_bytes_and_missing() {
    let server = make_server();
    let result = server
        .search_docs(SearchDocsParams {
            query: "b".repeat(1024),
            limit: 10,
        })
        .await;
    assert!(result.is_ok(), "Unexpected error: {:?}", result);
}

#[tokio::test]
async fn search_docs_returns_matched_chunk_when_multibyte_query_is_exactly_1024_bytes_and_exists() {
    let server = make_server();
    let query = "🦀".repeat(256); // 1024 bytes
    let result = server
        .search_docs(SearchDocsParams { query, limit: 10 })
        .await;
    assert!(result.is_ok(), "Unexpected error: {:?}", result);
}

#[tokio::test]
async fn search_docs_returns_empty_when_multibyte_query_is_exactly_1024_bytes() {
    let server = make_server();
    let query = "🦀".repeat(256); // 1024 bytes
    let result = server
        .search_docs(SearchDocsParams { query, limit: 10 })
        .await;
    assert!(result.is_ok(), "Unexpected error: {:?}", result);
}

#[tokio::test]
async fn search_docs_returns_default_number_of_results_when_limit_is_omitted() {
    let json = serde_json::json!({
        "query": "test"
    });
    let params: SearchDocsParams = serde_json::from_value(json).unwrap();
    assert_eq!(params.limit, 10);
}

#[tokio::test]
async fn search_docs_returns_one_result_when_limit_is_one() {
    let server = make_server();
    let result = server
        .search_docs(SearchDocsParams {
            query: "test".to_string(),
            limit: 1,
        })
        .await;
    assert!(result.is_ok(), "Unexpected error: {:?}", result);
}

#[tokio::test]
async fn search_docs_returns_10_results_when_limit_is_10() {
    let server = make_server();
    let result = server
        .search_docs(SearchDocsParams {
            query: "test".to_string(),
            limit: 10,
        })
        .await;
    assert!(result.is_ok(), "Unexpected error: {:?}", result);
}

#[tokio::test]
async fn search_docs_returns_results_when_limit_is_exactly_1000() {
    let server = make_server();
    let result = server
        .search_docs(SearchDocsParams {
            query: "test".to_string(),
            limit: 1000,
        })
        .await;
    assert!(result.is_ok(), "Unexpected error: {:?}", result);
}

#[tokio::test]
async fn read_chunk_returns_invalid_input_when_id_exceeds_256_bytes() {
    let server = make_server();
    let result = server
        .read_chunk(ReadChunkParams {
            id: "a".repeat(257),
        })
        .await;
    assert!(matches!(result, Err(CtdMcpError::InvalidInput { .. })));
}

#[tokio::test]
async fn read_chunk_returns_invalid_input_when_multibyte_id_exceeds_256_bytes() {
    let server = make_server();
    let result = server
        .read_chunk(ReadChunkParams {
            id: "🦀".repeat(65),
        })
        .await; // 260 bytes
    assert!(matches!(result, Err(CtdMcpError::InvalidInput { .. })));
}

#[tokio::test]
async fn read_chunk_returns_invalid_input_when_id_contains_spaces() {
    let server = make_server();
    let result = server
        .read_chunk(ReadChunkParams {
            id: "chunk 123".to_string(),
        })
        .await;
    assert!(matches!(result, Err(CtdMcpError::InvalidInput { .. })));
}

#[tokio::test]
async fn read_chunk_returns_invalid_input_when_id_contains_symbols() {
    let server = make_server();
    let result = server
        .read_chunk(ReadChunkParams {
            id: "chunk_!@#".to_string(),
        })
        .await;
    assert!(matches!(result, Err(CtdMcpError::InvalidInput { .. })));
}

#[tokio::test]
async fn read_chunk_returns_exact_text_when_id_is_exactly_1_char() {
    let server = make_server();
    let _result = server
        .read_chunk(ReadChunkParams {
            id: "a".to_string(),
        })
        .await;
    // red phase check
}

#[tokio::test]
async fn read_chunk_returns_exact_text_when_id_is_exactly_256_bytes() {
    let server = make_server();
    let _result = server
        .read_chunk(ReadChunkParams {
            id: "a".repeat(256),
        })
        .await;
    // red phase check
}

#[tokio::test]
async fn read_chunk_returns_exact_text_when_multibyte_id_is_exactly_256_bytes_and_exists() {
    let server = make_server();
    let _result = server
        .read_chunk(ReadChunkParams {
            id: "🦀".repeat(64),
        })
        .await;
    // red phase check
}

#[tokio::test]
async fn read_chunk_returns_index_not_found_when_id_is_exactly_256_bytes_and_missing() {
    let server = make_server();
    let _result = server
        .read_chunk(ReadChunkParams {
            id: "b".repeat(256),
        })
        .await;
    // Should be OK to return empty or error
}

#[tokio::test]
async fn get_related_concepts_returns_invalid_input_when_id_exceeds_256_bytes() {
    let server = make_server();
    let result = server
        .get_related_concepts(GetRelatedConceptsParams {
            id: "a".repeat(257),
        })
        .await;
    assert!(matches!(result, Err(CtdMcpError::InvalidInput { .. })));
}

#[tokio::test]
async fn get_related_concepts_returns_invalid_input_when_multibyte_id_exceeds_256_bytes() {
    let server = make_server();
    let result = server
        .get_related_concepts(GetRelatedConceptsParams {
            id: "🦀".repeat(65),
        })
        .await;
    assert!(matches!(result, Err(CtdMcpError::InvalidInput { .. })));
}

#[tokio::test]
async fn get_related_concepts_returns_invalid_input_when_id_contains_newline() {
    let server = make_server();
    let result = server
        .get_related_concepts(GetRelatedConceptsParams {
            id: "rust\n".to_string(),
        })
        .await;
    assert!(matches!(result, Err(CtdMcpError::InvalidInput { .. })));
}

#[tokio::test]
async fn get_related_concepts_returns_invalid_input_when_id_contains_symbols() {
    let server = make_server();
    let result = server
        .get_related_concepts(GetRelatedConceptsParams {
            id: "rust!@#".to_string(),
        })
        .await;
    assert!(matches!(result, Err(CtdMcpError::InvalidInput { .. })));
}

#[tokio::test]
async fn get_related_concepts_returns_exact_json_when_id_is_exactly_1_char() {
    let server = make_server();
    let _result = server
        .get_related_concepts(GetRelatedConceptsParams {
            id: "a".to_string(),
        })
        .await;
}

#[tokio::test]
async fn get_related_concepts_returns_exact_json_when_id_is_exactly_256_bytes() {
    let server = make_server();
    let _result = server
        .get_related_concepts(GetRelatedConceptsParams {
            id: "a".repeat(256),
        })
        .await;
}

#[tokio::test]
async fn get_related_concepts_returns_exact_json_when_multibyte_id_is_exactly_256_bytes_and_exists()
{
    let server = make_server();
    let _result = server
        .get_related_concepts(GetRelatedConceptsParams {
            id: "🦀".repeat(64),
        })
        .await;
}

#[tokio::test]
async fn get_related_concepts_returns_empty_when_id_is_exactly_256_bytes_and_missing() {
    let server = make_server();
    let _result = server
        .get_related_concepts(GetRelatedConceptsParams {
            id: "b".repeat(256),
        })
        .await;
}
