#![allow(clippy::unwrap_used, clippy::expect_used)]

use doc_transformer::math_types::Score;
use doc_transformer::mcp::domain::IndexData;
use doc_transformer::mcp::types::{
    GetRelatedConceptsParams, ReadChunkParams, SearchDocsParams, ToolResult,
};
use doc_transformer::mcp::{CtdMcpError, CtdMcpServer};
use doc_transformer::search::SearchResult;
use serde_json::json;
use std::path::PathBuf;
use tempfile::TempDir;

fn make_search_result(
    title: &str,
    category: &str,
    score: f32,
    path: &str,
    summary: &str,
) -> SearchResult {
    SearchResult {
        id: format!("id-{title}"),
        title: title.to_string(),
        summary: summary.to_string(),
        category: category.to_string(),
        score: Score::try_new(score).expect("valid score"),
        path: path.to_string(),
    }
}

fn make_index_json_with_chunks_and_docs(
    chunks: Vec<serde_json::Value>,
    docs: Vec<serde_json::Value>,
    edges: Vec<serde_json::Value>,
) -> serde_json::Value {
    json!({
        "documents": docs,
        "chunks": chunks,
        "graph": { "edges": edges }
    })
}

fn write_index_json(dir: &TempDir, data: &serde_json::Value) {
    let path = dir.path().join("INDEX.json");
    std::fs::write(&path, serde_json::to_string(data).unwrap()).unwrap();
}

fn make_server(dir: &TempDir) -> CtdMcpServer {
    CtdMcpServer::new(dir.path().to_path_buf()).expect("server creation should succeed")
}

mod construction {
    use super::*;

    #[test]
    fn new_returns_ok_when_dir_exists() {
        let dir = TempDir::new().unwrap();
        let result = CtdMcpServer::new(dir.path().to_path_buf());
        assert!(
            result.is_ok(),
            "CtdMcpServer::new should succeed for existing directory"
        );
        let server = result.unwrap();
        assert!(
            server.index_dir.is_absolute(),
            "index_dir should be canonicalized to absolute path"
        );
    }

    #[test]
    fn new_returns_io_error_when_dir_missing() {
        let result = CtdMcpServer::new(PathBuf::from("/nonexistent/path/xyz_abc_123"));
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, CtdMcpError::IoError { .. }),
            "Expected IoError, got {:?}",
            err
        );
    }

    #[test]
    fn new_returns_io_error_when_path_is_file() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("a_file.txt");
        std::fs::write(&file_path, "not a dir").unwrap();
        let result = CtdMcpServer::new(file_path);
        assert!(result.is_err());
        assert!(
            matches!(result.unwrap_err(), CtdMcpError::IoError { .. }),
            "Expected IoError when path is a file"
        );
    }

    #[test]
    fn new_canonicalizes_relative_path() {
        let dir = TempDir::new().unwrap();
        let result = CtdMcpServer::new(dir.path().to_path_buf());
        assert!(result.is_ok());
        assert!(result.unwrap().index_dir.is_absolute());
    }
}

mod search_docs_validation {
    use super::*;

    #[tokio::test]
    async fn search_docs_returns_invalid_input_when_query_empty() {
        let dir = TempDir::new().unwrap();
        write_index_json(
            &dir,
            &json!({"documents": [], "chunks": [], "graph": {"edges": []}}),
        );
        let server = make_server(&dir);
        let result = server
            .search_docs(SearchDocsParams {
                query: "".to_string(),
                limit: 10,
            })
            .await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, CtdMcpError::InvalidInput { .. }),
            "Expected InvalidInput for empty query, got {:?}",
            err
        );
    }

    #[tokio::test]
    async fn search_docs_returns_invalid_input_when_query_whitespace() {
        let dir = TempDir::new().unwrap();
        write_index_json(
            &dir,
            &json!({"documents": [], "chunks": [], "graph": {"edges": []}}),
        );
        let server = make_server(&dir);
        let result = server
            .search_docs(SearchDocsParams {
                query: "   \t  ".to_string(),
                limit: 10,
            })
            .await;
        assert!(result.is_err());
        assert!(
            matches!(result.unwrap_err(), CtdMcpError::InvalidInput { .. }),
            "Expected InvalidInput for whitespace query"
        );
    }

    #[tokio::test]
    async fn search_docs_returns_invalid_input_when_limit_zero() {
        let dir = TempDir::new().unwrap();
        write_index_json(
            &dir,
            &json!({"documents": [], "chunks": [], "graph": {"edges": []}}),
        );
        let server = make_server(&dir);
        let result = server
            .search_docs(SearchDocsParams {
                query: "test".to_string(),
                limit: 0,
            })
            .await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, CtdMcpError::InvalidInput { .. }),
            "Expected InvalidInput for limit 0, got {:?}",
            err
        );
    }

    #[tokio::test]
    async fn search_docs_returns_invalid_input_when_limit_exceeds_1000() {
        let dir = TempDir::new().unwrap();
        write_index_json(
            &dir,
            &json!({"documents": [], "chunks": [], "graph": {"edges": []}}),
        );
        let server = make_server(&dir);
        let result = server
            .search_docs(SearchDocsParams {
                query: "test".to_string(),
                limit: 1001,
            })
            .await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, CtdMcpError::InvalidInput { .. }),
            "Expected InvalidInput for limit 1001, got {:?}",
            err
        );
    }
}

mod search_docs_integration {
    use super::*;

    #[tokio::test]
    async fn search_docs_returns_index_not_found_when_json_missing() {
        let dir = TempDir::new().unwrap();
        let server = make_server(&dir);
        let result = server
            .search_docs(SearchDocsParams {
                query: "test".to_string(),
                limit: 10,
            })
            .await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, CtdMcpError::IndexNotFound { .. }),
            "Expected IndexNotFound, got {:?}",
            err
        );
    }

    #[tokio::test]
    async fn search_docs_returns_index_corrupted_when_json_malformed() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("INDEX.json"), "not valid json{{{").unwrap();
        let server = make_server(&dir);
        let result = server
            .search_docs(SearchDocsParams {
                query: "test".to_string(),
                limit: 10,
            })
            .await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, CtdMcpError::IndexCorrupted { .. }),
            "Expected IndexCorrupted, got {:?}",
            err
        );
    }

    #[tokio::test]
    async fn search_docs_returns_no_results_message_when_no_match() {
        let dir = TempDir::new().unwrap();
        let docs = vec![
            json!({"doc_id": "d1", "title": "rust programming", "summary": "about rust", "category": "tutorial"}),
        ];
        write_index_json(
            &dir,
            &make_index_json_with_chunks_and_docs(vec![], docs, vec![]),
        );
        let server = make_server(&dir);
        let result = server
            .search_docs(SearchDocsParams {
                query: "xyzzyplughnothing".to_string(),
                limit: 10,
            })
            .await;
        assert!(
            result.is_ok(),
            "search should succeed even with no matches: {:?}",
            result
        );
        let tool_result = result.unwrap();
        assert_eq!(tool_result.text_content(), Some("No results found."));
    }

    #[tokio::test]
    async fn search_docs_returns_ranked_results_when_query_matches() {
        let dir = TempDir::new().unwrap();
        let docs = vec![
            json!({"doc_id": "d1", "title": "kubernetes pods tutorial", "summary": "learn about pods", "category": "tutorial", "path": "pods.md"}),
            json!({"doc_id": "d2", "title": "kubernetes services", "summary": "service types", "category": "ref", "path": "services.md"}),
            json!({"doc_id": "d3", "title": "python flask", "summary": "flask web framework", "category": "tutorial", "path": "flask.md"}),
        ];
        write_index_json(
            &dir,
            &make_index_json_with_chunks_and_docs(vec![], docs, vec![]),
        );
        let server = make_server(&dir);
        let result = server
            .search_docs(SearchDocsParams {
                query: "kubernetes".to_string(),
                limit: 10,
            })
            .await;
        assert!(result.is_ok(), "search should succeed: {:?}", result);
        let text = result.unwrap().text_content().unwrap_or("").to_string();
        assert!(
            text.contains("1."),
            "Expected ranked results, got: {}",
            text
        );
        assert!(
            text.to_lowercase().contains("kubernetes"),
            "Expected 'kubernetes' in results, got: {}",
            text
        );
    }

    #[tokio::test]
    async fn search_docs_respects_limit_parameter() {
        let dir = TempDir::new().unwrap();
        let docs: Vec<serde_json::Value> = (0..5)
            .map(|i| {
                json!({"doc_id": format!("d{i}"), "title": format!("test doc {i}"), "summary": "test", "category": "tutorial", "path": format!("d{i}.md")})
            })
            .collect();
        write_index_json(
            &dir,
            &make_index_json_with_chunks_and_docs(vec![], docs, vec![]),
        );
        let server = make_server(&dir);
        let result = server
            .search_docs(SearchDocsParams {
                query: "test".to_string(),
                limit: 3,
            })
            .await;
        assert!(result.is_ok());
        let text = result.unwrap().text_content().unwrap_or("").to_string();
        let count = text.matches("Score:").count();
        assert!(count <= 3, "Expected at most 3 results, got {}", count);
    }
}

mod read_chunk_validation {
    use super::*;

    #[tokio::test]
    async fn read_chunk_returns_invalid_input_when_id_empty() {
        let dir = TempDir::new().unwrap();
        write_index_json(
            &dir,
            &json!({"documents": [], "chunks": [], "graph": {"edges": []}}),
        );
        let server = make_server(&dir);
        let result = server
            .read_chunk(ReadChunkParams { id: "".to_string() })
            .await;
        assert!(result.is_err());
        assert!(
            matches!(result.unwrap_err(), CtdMcpError::InvalidInput { .. }),
            "Expected InvalidInput for empty id"
        );
    }

    #[tokio::test]
    async fn read_chunk_returns_invalid_input_when_id_whitespace() {
        let dir = TempDir::new().unwrap();
        write_index_json(
            &dir,
            &json!({"documents": [], "chunks": [], "graph": {"edges": []}}),
        );
        let server = make_server(&dir);
        let result = server
            .read_chunk(ReadChunkParams {
                id: "  \t ".to_string(),
            })
            .await;
        assert!(result.is_err());
        assert!(
            matches!(result.unwrap_err(), CtdMcpError::InvalidInput { .. }),
            "Expected InvalidInput for whitespace id"
        );
    }
}

mod read_chunk_integration {
    use super::*;

    #[tokio::test]
    async fn read_chunk_returns_chunk_content_when_id_matches_chunk() {
        let dir = TempDir::new().unwrap();
        let chunks =
            vec![json!({"chunk_id": "chunk-abc", "content": "This is chunk ABC verbatim."})];
        write_index_json(
            &dir,
            &make_index_json_with_chunks_and_docs(chunks, vec![], vec![]),
        );
        let server = make_server(&dir);
        let result = server
            .read_chunk(ReadChunkParams {
                id: "chunk-abc".to_string(),
            })
            .await;
        assert!(result.is_ok(), "read_chunk should succeed: {:?}", result);
        assert_eq!(
            result.unwrap().text_content(),
            Some("This is chunk ABC verbatim.")
        );
    }

    #[tokio::test]
    async fn read_chunk_returns_doc_summary_when_id_matches_doc() {
        let dir = TempDir::new().unwrap();
        let docs = vec![json!({"doc_id": "doc-123", "summary": "Summary of doc 123."})];
        write_index_json(
            &dir,
            &make_index_json_with_chunks_and_docs(vec![], docs, vec![]),
        );
        let server = make_server(&dir);
        let result = server
            .read_chunk(ReadChunkParams {
                id: "doc-123".to_string(),
            })
            .await;
        assert!(result.is_ok(), "read_chunk should succeed: {:?}", result);
        let text = result.unwrap().text_content().unwrap_or("").to_string();
        assert!(
            text.contains("Summary of doc 123."),
            "Expected doc summary in output, got: {}",
            text
        );
    }

    #[tokio::test]
    async fn read_chunk_returns_not_found_when_id_matches_nothing() {
        let dir = TempDir::new().unwrap();
        write_index_json(
            &dir,
            &make_index_json_with_chunks_and_docs(
                vec![json!({"chunk_id": "c1", "content": "hello"})],
                vec![json!({"doc_id": "d1", "summary": "world"})],
                vec![],
            ),
        );
        let server = make_server(&dir);
        let result = server
            .read_chunk(ReadChunkParams {
                id: "nonexistent-xyz".to_string(),
            })
            .await;
        assert!(result.is_ok());
        let text = result.unwrap().text_content().unwrap_or("").to_string();
        assert!(
            text.contains("nonexistent-xyz") && text.contains("not found"),
            "Expected not-found message, got: {}",
            text
        );
    }

    #[tokio::test]
    async fn read_chunk_returns_index_not_found_when_json_missing() {
        let dir = TempDir::new().unwrap();
        let server = make_server(&dir);
        let result = server
            .read_chunk(ReadChunkParams {
                id: "anything".to_string(),
            })
            .await;
        assert!(result.is_err());
        assert!(
            matches!(result.unwrap_err(), CtdMcpError::IndexNotFound { .. }),
            "Expected IndexNotFound"
        );
    }

    #[tokio::test]
    async fn read_chunk_returns_index_corrupted_when_json_malformed() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("INDEX.json"), "}invalid{").unwrap();
        let server = make_server(&dir);
        let result = server
            .read_chunk(ReadChunkParams {
                id: "anything".to_string(),
            })
            .await;
        assert!(result.is_err());
        assert!(
            matches!(result.unwrap_err(), CtdMcpError::IndexCorrupted { .. }),
            "Expected IndexCorrupted"
        );
    }

    #[tokio::test]
    async fn read_chunk_prefers_chunk_match_over_doc_match() {
        let dir = TempDir::new().unwrap();
        let chunks = vec![json!({"chunk_id": "shared-id", "content": "chunk content"})];
        let docs = vec![json!({"doc_id": "shared-id", "summary": "doc summary"})];
        write_index_json(
            &dir,
            &make_index_json_with_chunks_and_docs(chunks, docs, vec![]),
        );
        let server = make_server(&dir);
        let result = server
            .read_chunk(ReadChunkParams {
                id: "shared-id".to_string(),
            })
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().text_content(), Some("chunk content"));
    }
}

mod get_related_validation {
    use super::*;

    #[tokio::test]
    async fn get_related_returns_invalid_input_when_id_empty() {
        let dir = TempDir::new().unwrap();
        write_index_json(
            &dir,
            &json!({"documents": [], "chunks": [], "graph": {"edges": []}}),
        );
        let server = make_server(&dir);
        let result = server
            .get_related_concepts(GetRelatedConceptsParams { id: "".to_string() })
            .await;
        assert!(result.is_err());
        assert!(
            matches!(result.unwrap_err(), CtdMcpError::InvalidInput { .. }),
            "Expected InvalidInput for empty id"
        );
    }

    #[tokio::test]
    async fn get_related_returns_invalid_input_when_id_whitespace() {
        let dir = TempDir::new().unwrap();
        write_index_json(
            &dir,
            &json!({"documents": [], "chunks": [], "graph": {"edges": []}}),
        );
        let server = make_server(&dir);
        let result = server
            .get_related_concepts(GetRelatedConceptsParams {
                id: "  \n ".to_string(),
            })
            .await;
        assert!(result.is_err());
        assert!(
            matches!(result.unwrap_err(), CtdMcpError::InvalidInput { .. }),
            "Expected InvalidInput for whitespace id"
        );
    }
}

mod get_related_integration {
    use super::*;

    #[tokio::test]
    async fn get_related_returns_from_edges_when_id_is_source() {
        let dir = TempDir::new().unwrap();
        let edges = vec![json!({"from": "node-a", "to": "node-b", "relationship_type": "Parent"})];
        write_index_json(
            &dir,
            &make_index_json_with_chunks_and_docs(vec![], vec![], edges),
        );
        let server = make_server(&dir);
        let result = server
            .get_related_concepts(GetRelatedConceptsParams {
                id: "node-a".to_string(),
            })
            .await;
        assert!(result.is_ok(), "get_related should succeed: {:?}", result);
        let text = result.unwrap().text_content().unwrap_or("").to_string();
        assert!(
            text.contains("node-b"),
            "Expected 'node-b' in output, got: {}",
            text
        );
        assert!(
            text.contains("Parent"),
            "Expected 'Parent' relationship, got: {}",
            text
        );
    }

    #[tokio::test]
    async fn get_related_returns_inbound_edges_when_id_is_target() {
        let dir = TempDir::new().unwrap();
        let edges = vec![json!({"from": "node-b", "to": "node-a", "relationship_type": "Related"})];
        write_index_json(
            &dir,
            &make_index_json_with_chunks_and_docs(vec![], vec![], edges),
        );
        let server = make_server(&dir);
        let result = server
            .get_related_concepts(GetRelatedConceptsParams {
                id: "node-a".to_string(),
            })
            .await;
        assert!(result.is_ok());
        let text = result.unwrap().text_content().unwrap_or("").to_string();
        assert!(
            text.contains("node-b"),
            "Expected 'node-b' in output, got: {}",
            text
        );
        assert!(
            text.contains("inbound"),
            "Expected 'inbound' label, got: {}",
            text
        );
    }

    #[tokio::test]
    async fn get_related_returns_empty_message_when_no_edges() {
        let dir = TempDir::new().unwrap();
        write_index_json(
            &dir,
            &make_index_json_with_chunks_and_docs(vec![], vec![], vec![]),
        );
        let server = make_server(&dir);
        let result = server
            .get_related_concepts(GetRelatedConceptsParams {
                id: "orphan-node".to_string(),
            })
            .await;
        assert!(result.is_ok());
        let text = result.unwrap().text_content().unwrap_or("").to_string();
        assert!(
            text.contains("No related concepts found") && text.contains("orphan-node"),
            "Expected no-related message, got: {}",
            text
        );
    }

    #[tokio::test]
    async fn get_related_returns_index_not_found_when_json_missing() {
        let dir = TempDir::new().unwrap();
        let server = make_server(&dir);
        let result = server
            .get_related_concepts(GetRelatedConceptsParams {
                id: "anything".to_string(),
            })
            .await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CtdMcpError::IndexNotFound { .. }
        ));
    }

    #[tokio::test]
    async fn get_related_returns_index_corrupted_when_json_malformed() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("INDEX.json"), "{bad json").unwrap();
        let server = make_server(&dir);
        let result = server
            .get_related_concepts(GetRelatedConceptsParams {
                id: "anything".to_string(),
            })
            .await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CtdMcpError::IndexCorrupted { .. }
        ));
    }
}

mod pure_helpers {
    use super::*;

    #[test]
    fn format_search_results_returns_no_results_for_empty() {
        let output = CtdMcpServer::format_search_results(&[]);
        assert_eq!(output, "No results found.");
    }

    #[test]
    fn format_search_results_formats_ranked_entries() {
        let results = vec![
            make_search_result("Title A", "tutorial", 0.95, "a.md", "Summary A"),
            make_search_result("Title B", "ref", 0.80, "b.md", "Summary B"),
        ];
        let output = CtdMcpServer::format_search_results(&results);
        assert!(
            output.starts_with("1. ["),
            "Should start with rank 1, got: {}",
            output
        );
        assert!(
            output.contains("2. ["),
            "Should contain rank 2, got: {}",
            output
        );
        assert!(
            output.contains("Score: "),
            "Should contain Score, got: {}",
            output
        );
        assert!(
            output.contains("Title: Title A"),
            "Should contain title, got: {}",
            output
        );
        assert!(
            output.contains("Path: a.md"),
            "Should contain path, got: {}",
            output
        );
        assert!(
            output.contains("Summary: Summary A"),
            "Should contain summary, got: {}",
            output
        );
        assert!(
            output.contains("---\n"),
            "Entries should be separated by ---"
        );
    }

    #[test]
    fn find_chunk_content_returns_some_when_id_matches() {
        let json = json!({"chunks": [{"chunk_id": "c1", "content": "hello"}]});
        let index_data = serde_json::from_value::<IndexData>(json).unwrap();
        let result = index_data.find_chunk_content("c1");
        assert_eq!(result, Some("hello".to_string()));
    }

    #[test]
    fn find_chunk_content_returns_none_when_no_match() {
        let json = json!({"chunks": [{"chunk_id": "c1", "content": "hello"}]});
        let index_data = serde_json::from_value::<IndexData>(json).unwrap();
        let result = index_data.find_chunk_content("missing");
        assert_eq!(result, None);
    }

    #[test]
    fn find_chunk_content_returns_none_when_chunks_empty() {
        let json = json!({"chunks": []});
        let index_data = serde_json::from_value::<IndexData>(json).unwrap();
        let result = index_data.find_chunk_content("any");
        assert_eq!(result, None);
    }

    #[test]
    fn find_chunk_content_returns_first_match_for_duplicate_ids() {
        let json = json!({"chunks": [
            {"chunk_id": "dup-1", "content": "first"},
            {"chunk_id": "dup-1", "content": "second"}
        ]});
        let index_data = serde_json::from_value::<IndexData>(json).unwrap();
        let result = index_data.find_chunk_content("dup-1");
        assert_eq!(result, Some("first".to_string()));
    }

    #[test]
    fn find_chunk_content_returns_some_empty_string_for_empty_content() {
        let json = json!({"chunks": [{"chunk_id": "e1", "content": ""}]});
        let index_data = serde_json::from_value::<IndexData>(json).unwrap();
        let result = index_data.find_chunk_content("e1");
        assert_eq!(result, Some("".to_string()));
    }

    #[test]
    fn find_doc_summary_returns_some_when_id_matches() {
        let json = json!({"documents": [{"doc_id": "d1", "summary": "my summary"}]});
        let index_data = serde_json::from_value::<IndexData>(json).unwrap();
        let result = index_data.find_doc_summary("d1");
        assert_eq!(result, Some("my summary".to_string()));
    }

    #[test]
    fn find_doc_summary_returns_none_when_no_match() {
        let json = json!({"documents": [{"doc_id": "d1", "summary": "my summary"}]});
        let index_data = serde_json::from_value::<IndexData>(json).unwrap();
        let result = index_data.find_doc_summary("missing");
        assert_eq!(result, None);
    }

    #[test]
    fn find_doc_summary_returns_none_when_docs_empty() {
        let json = json!({"documents": []});
        let index_data = serde_json::from_value::<IndexData>(json).unwrap();
        let result = index_data.find_doc_summary("any");
        assert_eq!(result, None);
    }

    #[test]
    fn find_doc_summary_returns_first_match_for_duplicate_ids() {
        let json = json!({"documents": [
            {"doc_id": "dup-d1", "summary": "first"},
            {"doc_id": "dup-d1", "summary": "second"}
        ]});
        let index_data = serde_json::from_value::<IndexData>(json).unwrap();
        let result = index_data.find_doc_summary("dup-d1");
        assert_eq!(result, Some("first".to_string()));
    }

    #[test]
    fn find_doc_summary_returns_some_empty_string_for_empty_summary() {
        let json = json!({"documents": [{"doc_id": "e2", "summary": ""}]});
        let index_data = serde_json::from_value::<IndexData>(json).unwrap();
        let result = index_data.find_doc_summary("e2");
        assert_eq!(result, Some("".to_string()));
    }

    #[test]
    fn find_related_edges_returns_formatted_edges_for_matching_id() {
        let json = json!({"graph": {"edges": [
            {"from": "a", "to": "b", "relationship_type": "Parent"},
            {"from": "c", "to": "a", "relationship_type": "Related"}
        ]}});
        let index_data = serde_json::from_value::<IndexData>(json).unwrap();
        let result = index_data.find_related_edges("a");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "- b (Relationship: Parent)");
        assert_eq!(result[1], "- c (Relationship: Related - inbound)");
    }

    #[test]
    fn find_related_edges_returns_empty_vec_when_no_matches() {
        let json = json!({"graph": {"edges": [
            {"from": "x", "to": "y", "relationship_type": "Parent"}
        ]}});
        let index_data = serde_json::from_value::<IndexData>(json).unwrap();
        let result = index_data.find_related_edges("z");
        assert!(result.is_empty());
    }

    #[test]
    fn find_related_edges_returns_empty_when_graph_missing() {
        let json = json!({});
        let index_data = serde_json::from_value::<IndexData>(json).unwrap();
        let result = index_data.find_related_edges("any");
        assert!(result.is_empty());
    }
}

mod error_mapping {
    use super::*;

    #[test]
    fn ctd_mcp_error_display_index_not_found() {
        let err = CtdMcpError::IndexNotFound {
            path: PathBuf::from("/foo"),
        };
        let msg = err.to_string();
        assert!(
            msg.contains("/foo"),
            "Error message should contain path, got: {}",
            msg
        );
        assert!(
            msg.contains("INDEX.json"),
            "Error message should mention INDEX.json, got: {}",
            msg
        );
    }

    #[test]
    fn ctd_mcp_error_display_index_corrupted() {
        let err = CtdMcpError::IndexCorrupted {
            reason: "bad json".to_string(),
        };
        let msg = err.to_string();
        assert!(
            msg.contains("bad json"),
            "Error message should contain reason, got: {}",
            msg
        );
    }

    #[test]
    fn ctd_mcp_error_display_invalid_input() {
        let err = CtdMcpError::InvalidInput {
            detail: "query empty".to_string(),
        };
        let msg = err.to_string();
        assert!(
            msg.contains("query empty"),
            "Error message should contain detail, got: {}",
            msg
        );
    }

    #[test]
    fn ctd_mcp_error_display_search_index_error() {
        let err = CtdMcpError::SearchIndexError {
            reason: "open failed".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("open failed"), "got: {}", msg);
    }

    #[test]
    fn ctd_mcp_error_display_query_error() {
        let err = CtdMcpError::QueryError {
            reason: "parse fail".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("parse fail"), "got: {}", msg);
    }

    #[test]
    fn ctd_mcp_error_display_io_error() {
        let err = CtdMcpError::IoError {
            reason: "permission denied".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("permission denied"), "got: {}", msg);
    }

    #[test]
    fn ctd_mcp_error_display_internal() {
        let err = CtdMcpError::Internal {
            reason: "not implemented".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("not implemented"), "got: {}", msg);
    }
}

mod tool_result {
    use super::*;

    #[test]
    fn tool_result_text_creates_non_error_result() {
        let result = ToolResult::text("hello");
        assert_eq!(result.text_content(), Some("hello"));
        assert!(!result.is_error);
    }

    #[test]
    fn tool_result_error_creates_error_result() {
        let result = ToolResult::error("bad thing");
        assert_eq!(result.text_content(), Some("bad thing"));
        assert!(result.is_error);
    }

    #[test]
    fn tool_result_text_content_returns_none_for_empty_content() {
        let result = ToolResult {
            content: vec![],
            is_error: false,
        };
        assert_eq!(result.text_content(), None);
    }
}

mod run_entrypoint {
    use super::*;

    #[tokio::test]
    async fn run_returns_error_when_dir_missing() {
        let result = doc_transformer::mcp::run(PathBuf::from("/nonexistent/path_xyz_abc")).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CtdMcpError::IoError { .. }));
    }
}
