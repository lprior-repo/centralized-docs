#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Red Queen Adversarial Tests for the MCP server module.
//!
//! These tests probe for edge-case bugs, design gaps, and behavioral quirks
//! in the MCP domain layer, type validators, and server operations.
//! They document CURRENT behavior — including potential issues — rather than
//! prescribing fixes. Tests that reveal questionable behavior are annotated
//! with a `⚠️ POTENTIAL ISSUE` comment.

use doc_transformer::math_types::Score;
use doc_transformer::mcp::domain::IndexData;
use doc_transformer::mcp::types::{
    GetRelatedConceptsParams, ReadChunkParams, SearchDocsParams, ToolResult, ValidId, ValidLimit,
    ValidQuery,
};
use doc_transformer::mcp::{CtdMcpError, CtdMcpServer};
use doc_transformer::search::SearchResult;
use serde_json::json;
use tempfile::TempDir;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

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

// ===========================================================================
// Gap 1: find_doc_summary only matches doc_id, NOT the id field
// ===========================================================================

mod gap1_doc_id_vs_id {
    use super::*;

    /// ✅ FIXED (GEN-4-4): `find_doc_summary` now uses `get_id()` which checks
    /// both `id` and `doc_id`. A document with only the `id` field set is now
    /// correctly found by `find_doc_summary`.
    #[test]
    fn find_doc_summary_returns_some_when_only_id_field_set() {
        let index_data: IndexData = serde_json::from_value(json!({
            "documents": [{"id": "x", "title": "Doc X", "summary": "Summary X"}],
            "chunks": []
        }))
        .unwrap();

        // Document has `id: "x"` but no `doc_id`. find_doc_summary now finds it.
        let result = index_data.find_doc_summary("x");
        assert_eq!(
            result,
            Some("Summary X".to_string()),
            "find_doc_summary should return Some when 'id' is set, via get_id()."
        );
    }

    /// Confirms that `find_doc_summary` DOES work when `doc_id` is set.
    #[test]
    fn find_doc_summary_returns_some_when_doc_id_field_set() {
        let index_data: IndexData = serde_json::from_value(json!({
            "documents": [{"doc_id": "x", "title": "Doc X", "summary": "Summary X"}],
            "chunks": []
        }))
        .unwrap();

        let result = index_data.find_doc_summary("x");
        assert_eq!(result, Some("Summary X".to_string()));
    }

    /// ✅ FIXED (GEN-4-4): `find_doc_summary` now uses `get_id()` which prefers
    /// `id` over `doc_id`. When both are present, looking up by `id` works,
    /// but looking up by `doc_id` does not (since get_id() returns the `id` value).
    #[test]
    fn find_doc_summary_uses_get_id_when_both_present() {
        let index_data: IndexData = serde_json::from_value(json!({
            "documents": [{"id": "by-id", "doc_id": "by-doc-id", "title": "T", "summary": "S"}],
            "chunks": []
        }))
        .unwrap();

        // get_id() prefers id over doc_id, so looking up by-id succeeds
        assert_eq!(index_data.find_doc_summary("by-id"), Some("S".to_string()));
        // Looking up by-doc-id fails because get_id() returns "by-id", not "by-doc-id"
        assert_eq!(index_data.find_doc_summary("by-doc-id"), None);
    }

    /// ✅ FIXED (GEN-4-4): `read_chunk` now finds documents with only the `id`
    /// field set, because `find_doc_summary` uses `get_id()`.
    #[tokio::test]
    async fn read_chunk_finds_doc_with_only_id_field() {
        let dir = TempDir::new().unwrap();
        let docs = vec![json!({
            "id": "my-doc",
            "title": "My Doc",
            "summary": "Now visible to find_doc_summary"
        })];
        write_index_json(
            &dir,
            &make_index_json_with_chunks_and_docs(vec![], docs, vec![]),
        );
        let server = make_server(&dir);

        let result = server
            .read_chunk(ReadChunkParams {
                id: "my-doc".to_string(),
            })
            .await
            .expect("read_chunk should succeed");

        let text = result.text_content().unwrap_or("");
        assert!(
            text.contains("Document my-doc:"),
            "Expected doc summary for doc with only 'id' field, got: {text}"
        );
        assert!(
            text.contains("Now visible"),
            "Expected summary content, got: {text}"
        );
    }
}

// ===========================================================================
// Gap 2: Self-referencing graph edge produces duplicate output
// ===========================================================================

mod gap2_self_referencing_edge {
    use super::*;

    /// ✅ FIXED (GEN-4-3): When an edge has `from == to == id`, the code now
    /// uses an if/else-if chain so only one entry is produced (with "self" label).
    #[test]
    fn find_related_edges_with_self_referencing_edge_returns_one_entry() {
        let index_data: IndexData = serde_json::from_value(json!({
            "graph": {
                "edges": [{"from": "a", "to": "a", "relationship_type": "Self"}]
            }
        }))
        .unwrap();

        let result = index_data.find_related_edges("a");
        assert_eq!(
            result.len(),
            1,
            "Self-referencing edge should produce exactly 1 entry: {:?}",
            result
        );

        // Single entry should be the self-referencing entry
        assert!(
            result[0].contains("- a (Relationship: Self - self)"),
            "Self-referencing entry should have 'self' label: {:?}",
            result[0]
        );
    }

    /// Confirms normal (non-self) edges produce exactly one entry per direction.
    #[test]
    fn find_related_edges_normal_edge_returns_one_entry() {
        let index_data: IndexData = serde_json::from_value(json!({
            "graph": {
                "edges": [{"from": "a", "to": "b", "relationship_type": "Parent"}]
            }
        }))
        .unwrap();

        let result_a = index_data.find_related_edges("a");
        assert_eq!(result_a.len(), 1);

        let result_b = index_data.find_related_edges("b");
        assert_eq!(result_b.len(), 1);
    }
}

// ===========================================================================
// Gap 3: INDEX.json with explicitly null fields (not absent)
// ===========================================================================

mod gap3_null_fields {
    use super::*;

    /// ✅ FIXED (GEN-4-5): `serde(default, deserialize_with = "null_to_default")`
    /// now handles both missing and explicitly `null` fields, converting them
    /// to empty Vecs.
    #[test]
    fn index_data_accepts_explicit_null_documents_and_chunks() {
        let index_data: Result<IndexData, _> = serde_json::from_value(json!({
            "documents": null,
            "chunks": null
        }));

        assert!(
            index_data.is_ok(),
            "Explicit null fields should deserialize to empty vecs, not error."
        );
        let data = index_data.unwrap();
        assert!(data.documents.is_empty());
        assert!(data.chunks.is_empty());
    }

    /// Confirms that *missing* documents/chunks fields work (serde default).
    #[test]
    fn index_data_deserializes_with_missing_documents_and_chunks() {
        let index_data: Result<IndexData, _> = serde_json::from_value(json!({}));

        assert!(
            index_data.is_ok(),
            "Missing fields should use serde defaults"
        );
        let data = index_data.unwrap();
        assert!(data.documents.is_empty());
        assert!(data.chunks.is_empty());
    }

    #[test]
    fn index_data_deserializes_with_null_graph() {
        let index_data: Result<IndexData, _> = serde_json::from_value(json!({
            "documents": [],
            "chunks": [],
            "graph": null
        }));

        assert!(index_data.is_ok());
        let data = index_data.unwrap();
        assert!(data.graph.is_none());
    }

    /// ✅ FIXED (GEN-4-5): `serde(default, deserialize_with = "null_to_default")`
    /// on graph.edges now handles explicitly `null` edges.
    #[test]
    fn index_data_accepts_explicit_null_edges() {
        let index_data: Result<IndexData, _> = serde_json::from_value(json!({
            "documents": [],
            "chunks": [],
            "graph": { "edges": null }
        }));

        assert!(
            index_data.is_ok(),
            "Explicit null edges should deserialize to empty vec, not error."
        );
        let data = index_data.unwrap();
        assert!(data.graph.is_some());
        assert!(data.graph.as_ref().unwrap().edges.is_empty());
    }

    /// Confirms that *missing* edges field works (serde default → empty vec).
    #[test]
    fn index_data_deserializes_with_missing_edges() {
        let index_data: Result<IndexData, _> = serde_json::from_value(json!({
            "documents": [],
            "chunks": [],
            "graph": {}
        }));

        assert!(index_data.is_ok());
        let data = index_data.unwrap();
        assert!(data.graph.is_some());
        assert!(data.graph.as_ref().unwrap().edges.is_empty());
    }

    /// Edge with all null fields — `from`/`to`/`relationship_type` are all
    /// Option<String>, so null is valid.
    #[test]
    fn graph_edge_with_all_null_fields_deserializes() {
        let index_data: IndexData = serde_json::from_value(json!({
            "documents": [],
            "chunks": [],
            "graph": { "edges": [{"from": null, "to": null, "relationship_type": null}] }
        }))
        .unwrap();

        let edges = index_data.find_related_edges("anything");
        assert!(
            edges.is_empty(),
            "Edge with null from/to should not match any ID"
        );
    }
}

// ===========================================================================
// Gap 4: INDEX.json with extra unknown fields
// ===========================================================================

mod gap4_unknown_fields {
    use super::*;

    /// Serde by default ignores unknown fields. This confirms the INDEX.json
    /// deserializer is not using `#[serde(deny_unknown_fields)]`.
    #[test]
    fn index_data_ignores_extra_top_level_fields() {
        let index_data: Result<IndexData, _> = serde_json::from_value(json!({
            "documents": [],
            "chunks": [],
            "unknown_future_field": "some value",
            "metadata": {"version": 2}
        }));

        assert!(
            index_data.is_ok(),
            "Extra fields should be silently ignored"
        );
    }

    #[test]
    fn index_data_ignores_extra_fields_in_documents() {
        let index_data: Result<IndexData, _> = serde_json::from_value(json!({
            "documents": [{
                "doc_id": "d1",
                "title": "T",
                "summary": "S",
                "future_field": "ignored"
            }],
            "chunks": []
        }));

        assert!(index_data.is_ok());
        let data = index_data.unwrap();
        assert_eq!(data.documents.len(), 1);
    }

    #[test]
    fn index_data_ignores_extra_fields_in_chunks() {
        let index_data: Result<IndexData, _> = serde_json::from_value(json!({
            "documents": [],
            "chunks": [{
                "chunk_id": "c1",
                "content": "hello",
                "extra_field": true
            }]
        }));

        assert!(index_data.is_ok());
    }
}

// ===========================================================================
// Gap 5: OnceCell retry after initial failure
// ===========================================================================

mod gap5_oncecell_retry {
    use super::*;

    /// OnceCell caches the result (success or error) of `get_state()`.
    /// If the first load fails because INDEX.json is missing, the error is
    /// cached and subsequent calls will fail even if the file is created.
    ///
    /// This test documents the OnceCell caching behavior: once an error
    /// occurs, the cell is NOT populated (OnceCell::get_or_try_init retries
    /// on Err), so subsequent calls SHOULD retry.
    #[tokio::test]
    async fn oncecell_retries_after_index_not_found_error() {
        let dir = TempDir::new().unwrap();
        let server = make_server(&dir);

        // First call: INDEX.json doesn't exist → IndexNotFound
        let result1 = server
            .search_docs(SearchDocsParams {
                query: "test".to_string(),
                limit: 10,
            })
            .await;
        assert!(
            matches!(result1, Err(CtdMcpError::IndexNotFound { .. })),
            "First call should fail with IndexNotFound"
        );

        // Now write a valid INDEX.json
        write_index_json(
            &dir,
            &make_index_json_with_chunks_and_docs(
                vec![],
                vec![
                    json!({"doc_id": "d1", "title": "test doc", "summary": "test", "category": "tutorial"}),
                ],
                vec![],
            ),
        );

        // Second call: OnceCell should retry since the error was NOT cached
        // (OnceCell::get_or_try_init only caches Ok values)
        let result2 = server
            .search_docs(SearchDocsParams {
                query: "test".to_string(),
                limit: 10,
            })
            .await;
        assert!(
            result2.is_ok(),
            "Second call should succeed after INDEX.json is created: {:?}",
            result2
        );
    }

    /// If the first load succeeds, OnceCell caches it. Even if INDEX.json
    /// is later modified, the cached state is used.
    #[tokio::test]
    async fn oncecell_caches_successful_load() {
        let dir = TempDir::new().unwrap();
        write_index_json(
            &dir,
            &make_index_json_with_chunks_and_docs(
                vec![],
                vec![
                    json!({"doc_id": "d1", "title": "alpha", "summary": "alpha content", "category": "ref"}),
                ],
                vec![],
            ),
        );
        let server = make_server(&dir);

        // First call: loads and caches
        let result1 = server
            .search_docs(SearchDocsParams {
                query: "alpha".to_string(),
                limit: 10,
            })
            .await;
        assert!(result1.is_ok());

        // Overwrite INDEX.json with different data
        write_index_json(
            &dir,
            &make_index_json_with_chunks_and_docs(
                vec![],
                vec![
                    json!({"doc_id": "d2", "title": "bravo", "summary": "bravo content", "category": "ref"}),
                ],
                vec![],
            ),
        );

        // Second call: should still use cached data (finds "alpha" not "bravo")
        let result2 = server
            .search_docs(SearchDocsParams {
                query: "alpha".to_string(),
                limit: 10,
            })
            .await;
        assert!(result2.is_ok(), "Should succeed with cached state");
    }
}

// ===========================================================================
// Gap 6: SearchDocsParams with missing query field
// ===========================================================================

mod gap6_search_params_missing_query {
    use super::*;

    #[test]
    fn search_docs_params_fails_without_query_field() {
        let result = serde_json::from_value::<SearchDocsParams>(json!({"limit": 5}));
        assert!(result.is_err(), "Should fail without 'query' field");
    }

    #[test]
    fn search_docs_params_fails_with_null_query() {
        let result = serde_json::from_value::<SearchDocsParams>(json!({"query": null, "limit": 5}));
        assert!(result.is_err(), "Should fail with null query");
    }

    #[test]
    fn search_docs_params_succeeds_with_query_only() {
        let result = serde_json::from_value::<SearchDocsParams>(json!({"query": "rust"}));
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.query, "rust");
        assert_eq!(params.limit, 10, "Default limit should be 10");
    }
}

// ===========================================================================
// Gap 7: ValidId rejects dots — design limitation
// ===========================================================================

mod gap7_valid_id_rejects_dots {
    use super::*;

    /// ✅ FIXED (GEN-4-2): ValidId now accepts dots, allowing chunk IDs like
    /// "chunk-1.2.3" or version-style IDs like "1.0.0".
    #[test]
    fn valid_id_accepts_dot_character() {
        let result = ValidId::parse("chunk-1.2.3", "Chunk ID");
        assert!(result.is_ok(), "Dots should now be accepted by ValidId");
        assert_eq!(result.unwrap().as_str(), "chunk-1.2.3");
    }

    #[test]
    fn valid_id_rejects_slash_character() {
        let result = ValidId::parse("path/to/resource", "ID");
        assert!(result.is_err(), "Slashes should be rejected by ValidId");
    }

    #[test]
    fn valid_id_accepts_hyphens_and_underscores() {
        assert!(ValidId::parse("my-chunk_id-123", "ID").is_ok());
    }

    #[test]
    fn valid_id_accepts_alphanumeric() {
        assert!(ValidId::parse("abc123XYZ", "ID").is_ok());
    }

    /// ValidId rejects the hash character, which is relevant because
    /// chunk IDs often use `doc#section` format in this codebase.
    #[test]
    fn valid_id_rejects_hash_character() {
        let result = ValidId::parse("doc1#section", "ID");
        assert!(result.is_err(), "Hash should be rejected");
    }
}

// ===========================================================================
// Gap 8: format_search_results with special characters
// ===========================================================================

mod gap8_format_search_special_chars {
    use super::*;

    #[test]
    fn format_search_results_with_newlines_in_title() {
        let results = vec![make_search_result(
            "Title\nWith\nNewlines",
            "tutorial",
            0.9,
            "a.md",
            "clean summary",
        )];
        let output = CtdMcpServer::format_search_results(&results);
        // Should not panic; newlines in title should pass through
        assert!(
            output.contains("Title\nWith\nNewlines"),
            "Newlines should pass through: {output:?}"
        );
    }

    #[test]
    fn format_search_results_with_format_string_like_placeholders() {
        let results = vec![make_search_result(
            "Title with {rank} placeholder",
            "ref",
            0.8,
            "b.md",
            "summary {score} test",
        )];
        let output = CtdMcpServer::format_search_results(&results);
        // The format string should NOT interpret {rank} or {score}
        assert!(
            output.contains("{rank}"),
            "Literal {{rank}} should appear in output: {output:?}"
        );
        assert!(
            output.contains("{score}"),
            "Literal {{score}} should appear in output: {output:?}"
        );
    }

    #[test]
    fn format_search_results_with_unicode_title() {
        let results = vec![make_search_result(
            "日本語タイトル 🦀",
            "tutorial",
            0.75,
            "c.md",
            "unicode summary",
        )];
        let output = CtdMcpServer::format_search_results(&results);
        assert!(
            output.contains("日本語タイトル 🦀"),
            "Unicode should pass through: {output:?}"
        );
    }

    #[test]
    fn format_search_results_with_empty_title() {
        let results = vec![make_search_result("", "ref", 0.5, "d.md", "summary")];
        let output = CtdMcpServer::format_search_results(&results);
        assert!(
            output.contains("Title: \n"),
            "Empty title should not crash: {output:?}"
        );
    }

    #[test]
    fn format_search_results_with_very_long_title() {
        let long_title = "X".repeat(10_000);
        let results = vec![make_search_result(
            &long_title,
            "ref",
            0.5,
            "e.md",
            "summary",
        )];
        let output = CtdMcpServer::format_search_results(&results);
        assert!(
            output.contains(&long_title),
            "Long title should appear in output"
        );
    }
}

// ===========================================================================
// Gap 9: Edge with null from/to fields doesn't panic
// ===========================================================================

mod gap9_null_edge_fields {
    use super::*;

    /// Tests that edges with `null` from/to don't panic when compared.
    /// `unwrap_or_default()` in `find_related_edges` converts null to "".
    #[test]
    fn find_related_edges_with_null_from_does_not_panic() {
        let index_data: IndexData = serde_json::from_value(json!({
            "documents": [],
            "chunks": [],
            "graph": { "edges": [{"from": null, "to": "b", "relationship_type": "Parent"}] }
        }))
        .unwrap();

        // Should not panic. from=null → "" which won't match "b"
        let result = index_data.find_related_edges("b");
        assert_eq!(result.len(), 1, "Should match on 'to' field");
        assert!(result[0].contains("inbound"));
    }

    #[test]
    fn find_related_edges_with_null_to_does_not_panic() {
        let index_data: IndexData = serde_json::from_value(json!({
            "documents": [],
            "chunks": [],
            "graph": { "edges": [{"from": "a", "to": null, "relationship_type": "Parent"}] }
        }))
        .unwrap();

        let result = index_data.find_related_edges("a");
        assert_eq!(result.len(), 1, "Should match on 'from' field");
    }

    #[test]
    fn find_related_edges_with_both_null_matches_empty_string() {
        let index_data: IndexData = serde_json::from_value(json!({
            "documents": [],
            "chunks": [],
            "graph": { "edges": [{"from": null, "to": null, "relationship_type": null}] }
        }))
        .unwrap();

        // from="" and to="" — looking up "" hits the self-referencing branch
        let result = index_data.find_related_edges("");
        // ✅ FIXED (GEN-4-3): Deduplication via if/else-if prevents double entry
        assert_eq!(
            result.len(),
            1,
            "Empty string matching null-from and null-to should produce 1 entry (deduped): {:?}",
            result
        );
    }

    #[test]
    fn find_related_edges_with_null_relationship_type_uses_default() {
        let index_data: IndexData = serde_json::from_value(json!({
            "documents": [],
            "chunks": [],
            "graph": { "edges": [{"from": "a", "to": "b", "relationship_type": null}] }
        }))
        .unwrap();

        let result = index_data.find_related_edges("a");
        assert_eq!(result.len(), 1);
        assert!(
            result[0].contains("related"),
            "Default relationship type should be 'related': {:?}",
            result[0]
        );
    }
}

// ===========================================================================
// Gap 10: read_chunk returns Ok("not found") instead of Err
// ===========================================================================

mod gap10_not_found_is_ok {
    use super::*;

    /// `read_chunk` returns `Ok(ToolResult::text(...))` with a "not found"
    /// message rather than `Err(...)`. This is a design choice — missing
    /// IDs are not errors but informational results.
    #[tokio::test]
    async fn read_chunk_returns_success_with_not_found_text() {
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
                id: "nonexistent-id".to_string(),
            })
            .await;

        assert!(result.is_ok(), "Should return Ok, not Err");
        let tool_result = result.unwrap();
        assert!(
            !tool_result.is_error,
            "is_error should be false for not-found"
        );
        let text = tool_result.text_content().unwrap_or("");
        assert!(
            text.contains("not found"),
            "Should contain 'not found': {text}"
        );
        assert!(
            text.contains("nonexistent-id"),
            "Should mention the ID: {text}"
        );
    }

    /// Similarly, get_related_concepts returns Ok with "No related concepts"
    /// rather than Err.
    #[tokio::test]
    async fn get_related_concepts_returns_success_with_no_related_text() {
        let dir = TempDir::new().unwrap();
        write_index_json(
            &dir,
            &make_index_json_with_chunks_and_docs(vec![], vec![], vec![]),
        );
        let server = make_server(&dir);

        let result = server
            .get_related_concepts(GetRelatedConceptsParams {
                id: "anything".to_string(),
            })
            .await;

        assert!(result.is_ok());
        assert!(!result.unwrap().is_error);
    }
}

// ===========================================================================
// Gap 11: ValidQuery rejects all-whitespace including unicode whitespace
// ===========================================================================

mod gap11_valid_query_whitespace {
    use super::*;

    #[test]
    fn valid_query_rejects_ascii_whitespace_only() {
        assert!(ValidQuery::parse("   ").is_err());
        assert!(ValidQuery::parse("\t\n\r").is_err());
    }

    #[test]
    fn valid_query_rejects_non_breaking_space() {
        // U+00A0 (non-breaking space)
        assert!(
            ValidQuery::parse("\u{00a0}").is_err(),
            "Non-breaking space should be rejected"
        );
    }

    #[test]
    fn valid_query_rejects_en_quad() {
        // U+2000 (en quad)
        assert!(
            ValidQuery::parse("\u{2000}").is_err(),
            "En quad should be rejected"
        );
    }

    #[test]
    fn valid_query_rejects_em_space() {
        // U+2003 (em space)
        assert!(
            ValidQuery::parse("\u{2003}").is_err(),
            "Em space should be rejected"
        );
    }

    #[test]
    fn valid_query_rejects_ideographic_space() {
        // U+3000 (ideographic space)
        assert!(
            ValidQuery::parse("\u{3000}").is_err(),
            "Ideographic space should be rejected"
        );
    }

    #[test]
    fn valid_query_rejects_mixed_unicode_whitespace() {
        // Mix of NBSP + en-quad + regular space
        assert!(
            ValidQuery::parse("\u{00a0}\u{2000} ").is_err(),
            "Mixed unicode whitespace should be rejected"
        );
    }

    #[test]
    fn valid_query_accepts_non_whitespace_unicode() {
        assert!(
            ValidQuery::parse("日本語クエリ").is_ok(),
            "Non-whitespace unicode should be accepted"
        );
    }

    #[test]
    fn valid_query_trims_and_checks_emptiness() {
        // "  hello  " should be accepted (trim doesn't make it empty)
        assert!(ValidQuery::parse("  hello  ").is_ok());
    }
}

// ===========================================================================
// Gap 12: find_chunk_content with null content field
// ===========================================================================

mod gap12_null_chunk_content {
    use super::*;

    /// When `content` is explicitly `null` (not absent), `unwrap_or_default()`
    /// in `find_chunk_content` should return an empty string.
    #[test]
    fn find_chunk_content_with_null_content_returns_empty_string() {
        let index_data: IndexData = serde_json::from_value(json!({
            "documents": [],
            "chunks": [{"chunk_id": "c1", "content": null}]
        }))
        .unwrap();

        let result = index_data.find_chunk_content("c1");
        assert_eq!(
            result,
            Some(String::new()),
            "null content should return Some(\"\")"
        );
    }

    #[test]
    fn find_chunk_content_with_missing_content_field_returns_empty_string() {
        let index_data: IndexData = serde_json::from_value(json!({
            "documents": [],
            "chunks": [{"chunk_id": "c1"}]
        }))
        .unwrap();

        let result = index_data.find_chunk_content("c1");
        assert_eq!(
            result,
            Some(String::new()),
            "Missing content field should return Some(\"\")"
        );
    }

    #[test]
    fn find_chunk_content_with_absent_chunk_id_returns_none() {
        let index_data: IndexData = serde_json::from_value(json!({
            "documents": [],
            "chunks": [{"content": "orphan content"}]
        }))
        .unwrap();

        // No chunk_id at all — find_map won't match anything
        let result = index_data.find_chunk_content("orphan");
        assert_eq!(result, None);
    }

    #[test]
    fn find_chunk_content_with_null_chunk_id_returns_none_for_non_empty_search() {
        let index_data: IndexData = serde_json::from_value(json!({
            "documents": [],
            "chunks": [{"chunk_id": null, "content": "orphaned"}]
        }))
        .unwrap();

        let result = index_data.find_chunk_content("anything");
        assert_eq!(result, None);
    }
}

// ===========================================================================
// Gap 13: Multiple edges between same two nodes
// ===========================================================================

mod gap13_multiple_edges_same_nodes {
    use super::*;

    /// Multiple edges from A to B with different relationship types should
    /// all appear in the output for node A.
    #[test]
    fn find_related_edges_with_multiple_edges_same_direction() {
        let index_data: IndexData = serde_json::from_value(json!({
            "documents": [],
            "chunks": [],
            "graph": {
                "edges": [
                    {"from": "a", "to": "b", "relationship_type": "Parent"},
                    {"from": "a", "to": "b", "relationship_type": "Related"},
                    {"from": "a", "to": "b", "relationship_type": "Sequential"}
                ]
            }
        }))
        .unwrap();

        let result = index_data.find_related_edges("a");
        assert_eq!(result.len(), 3, "All three edges should appear");

        assert!(result[0].contains("Parent"));
        assert!(result[1].contains("Related"));
        assert!(result[2].contains("Sequential"));
    }

    #[test]
    fn find_related_edges_with_bidirectional_edges() {
        let index_data: IndexData = serde_json::from_value(json!({
            "documents": [],
            "chunks": [],
            "graph": {
                "edges": [
                    {"from": "a", "to": "b", "relationship_type": "Parent"},
                    {"from": "b", "to": "a", "relationship_type": "Child"}
                ]
            }
        }))
        .unwrap();

        // For "a": one outbound (Parent→b) + one inbound (Child←b)
        let result_a = index_data.find_related_edges("a");
        assert_eq!(result_a.len(), 2);

        // For "b": one inbound (Parent from a) + one outbound (Child→a)
        let result_b = index_data.find_related_edges("b");
        assert_eq!(result_b.len(), 2);
    }
}

// ===========================================================================
// Gap 14: Very large INDEX.json (performance/stress)
// ===========================================================================

mod gap14_large_index {
    use super::*;

    /// Stress test: 10,000 documents should be searchable without timeout.
    /// This verifies that IndexData::extract_documents and search can handle
    /// large datasets without pathological performance.
    #[tokio::test]
    async fn search_with_10000_documents_completes() {
        let dir = TempDir::new().unwrap();

        // Generate 10,000 documents with varying content
        let docs: Vec<serde_json::Value> = (0..10_000)
            .map(|i| {
                json!({
                    "doc_id": format!("doc-{i}"),
                    "title": format!("Document {i} about rust programming"),
                    "summary": format!("Summary of document {i} with rust content"),
                    "category": if i % 3 == 0 { "tutorial" } else if i % 3 == 1 { "ref" } else { "concept" },
                    "path": format!("docs/doc-{i}.md")
                })
            })
            .collect();

        write_index_json(
            &dir,
            &make_index_json_with_chunks_and_docs(vec![], docs, vec![]),
        );
        let server = make_server(&dir);

        let result = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            server.search_docs(SearchDocsParams {
                query: "rust".to_string(),
                limit: 10,
            }),
        )
        .await;

        assert!(result.is_ok(), "Search should complete within 30 seconds");
        let inner = result.unwrap();
        assert!(inner.is_ok(), "Search should succeed: {:?}", inner);
        let text = inner.unwrap().text_content().unwrap_or("").to_string();
        assert!(
            text.contains("Score:"),
            "Should have results for 'rust': {text}"
        );
    }

    /// Stress test: 1,000 chunks should be searchable by read_chunk.
    #[tokio::test]
    async fn read_chunk_finds_needle_in_1000_chunk_haystack() {
        let dir = TempDir::new().unwrap();

        let chunks: Vec<serde_json::Value> = (0..1_000)
            .map(|i| {
                json!({
                    "chunk_id": format!("chunk-{i}"),
                    "content": format!("Content of chunk number {i}")
                })
            })
            .collect();

        write_index_json(
            &dir,
            &make_index_json_with_chunks_and_docs(chunks, vec![], vec![]),
        );
        let server = make_server(&dir);

        // Read the last chunk
        let result = server
            .read_chunk(ReadChunkParams {
                id: "chunk-999".to_string(),
            })
            .await;

        assert!(result.is_ok());
        let text = result.unwrap().text_content().unwrap_or("").to_string();
        assert!(
            text.contains("Content of chunk number 999"),
            "Should find chunk-999: {text}"
        );
    }

    /// Stress test: 1,000 edges with related concepts lookup.
    #[tokio::test]
    async fn get_related_with_1000_edges_completes() {
        let dir = TempDir::new().unwrap();

        let edges: Vec<serde_json::Value> = (0..1_000)
            .map(|i| {
                json!({
                    "from": "hub",
                    "to": format!("spoke-{i}"),
                    "relationship_type": "Related"
                })
            })
            .collect();

        write_index_json(
            &dir,
            &make_index_json_with_chunks_and_docs(vec![], vec![], edges),
        );
        let server = make_server(&dir);

        let result = server
            .get_related_concepts(GetRelatedConceptsParams {
                id: "hub".to_string(),
            })
            .await;

        assert!(result.is_ok());
        let text = result.unwrap().text_content().unwrap_or("").to_string();
        assert!(
            text.contains("spoke-0"),
            "Should contain first spoke: {text}"
        );
        assert!(
            text.contains("spoke-999"),
            "Should contain last spoke: {text}"
        );
    }
}

// ===========================================================================
// Additional adversarial: ValidLimit edge cases
// ===========================================================================

mod valid_limit_edge_cases {
    use super::*;

    #[test]
    fn valid_limit_rejects_zero() {
        assert!(ValidLimit::parse(0).is_err());
    }

    #[test]
    fn valid_limit_accepts_one() {
        assert!(ValidLimit::parse(1).is_ok());
        assert_eq!(ValidLimit::parse(1).unwrap().as_u32(), 1);
    }

    #[test]
    fn valid_limit_accepts_1000() {
        assert!(ValidLimit::parse(1000).is_ok());
    }

    #[test]
    fn valid_limit_rejects_1001() {
        assert!(ValidLimit::parse(1001).is_err());
    }

    #[test]
    fn valid_limit_rejects_max_u32() {
        assert!(ValidLimit::parse(u32::MAX).is_err());
    }
}

// ===========================================================================
// Additional adversarial: ValidQuery length boundaries
// ===========================================================================

mod valid_query_length {
    use super::*;

    #[test]
    fn valid_query_accepts_exactly_1024_bytes() {
        let query = "a".repeat(1024);
        assert!(ValidQuery::parse(&query).is_ok());
    }

    #[test]
    fn valid_query_rejects_1025_bytes() {
        let query = "a".repeat(1025);
        assert!(ValidQuery::parse(&query).is_err());
    }

    #[test]
    fn valid_query_accepts_single_char() {
        assert!(ValidQuery::parse("x").is_ok());
    }

    #[test]
    fn valid_query_rejects_empty() {
        assert!(ValidQuery::parse("").is_err());
    }
}

// ===========================================================================
// Additional adversarial: ValidId length and character boundaries
// ===========================================================================

mod valid_id_boundaries {
    use super::*;

    #[test]
    fn valid_id_accepts_exactly_256_bytes() {
        let id = "a".repeat(256);
        assert!(ValidId::parse(&id, "ID").is_ok());
    }

    #[test]
    fn valid_id_rejects_257_bytes() {
        let id = "a".repeat(257);
        assert!(ValidId::parse(&id, "ID").is_err());
    }

    #[test]
    fn valid_id_rejects_empty() {
        assert!(ValidId::parse("", "ID").is_err());
    }

    #[test]
    fn valid_id_rejects_tab() {
        assert!(ValidId::parse("id\tvalue", "ID").is_err());
    }

    #[test]
    fn valid_id_rejects_colon() {
        assert!(ValidId::parse("ns:id", "ID").is_err());
    }

    #[test]
    fn valid_id_rejects_at_sign() {
        assert!(ValidId::parse("user@domain", "ID").is_err());
    }
}

// ===========================================================================
// Additional adversarial: ToolResult edge cases
// ===========================================================================

mod tool_result_edge_cases {
    use super::*;

    #[test]
    fn tool_result_text_content_returns_first_only() {
        let result = ToolResult {
            content: vec![
                doc_transformer::mcp::types::ToolContent {
                    content_type: "text".to_string(),
                    text: "first".to_string(),
                },
                doc_transformer::mcp::types::ToolContent {
                    content_type: "text".to_string(),
                    text: "second".to_string(),
                },
            ],
            is_error: false,
        };
        assert_eq!(result.text_content(), Some("first"));
    }

    #[test]
    fn tool_result_with_empty_text() {
        let result = ToolResult::text("");
        assert_eq!(result.text_content(), Some(""));
        assert!(!result.is_error);
    }
}

// ===========================================================================
// Additional adversarial: Domain deserialization robustness
// ===========================================================================

mod domain_deserialization_robustness {
    use super::*;

    #[test]
    fn index_data_handles_empty_json_object() {
        let result = serde_json::from_value::<IndexData>(json!({}));
        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(data.documents.is_empty());
        assert!(data.chunks.is_empty());
        assert!(data.graph.is_none());
    }

    #[test]
    fn document_with_only_required_defaults() {
        let result = serde_json::from_value::<IndexData>(json!({
            "documents": [{}],
            "chunks": []
        }));
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.documents.len(), 1);
        assert_eq!(data.documents[0].title, "");
        assert_eq!(data.documents[0].summary, "");
        assert_eq!(data.documents[0].path, "");
        assert_eq!(data.documents[0].category, "");
        assert!(data.documents[0].id.is_none());
        assert!(data.documents[0].doc_id.is_none());
    }

    #[test]
    fn chunk_with_only_chunk_id() {
        let result = serde_json::from_value::<IndexData>(json!({
            "documents": [],
            "chunks": [{"chunk_id": "c1"}]
        }));
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.chunks.len(), 1);
        assert_eq!(data.chunks[0].chunk_id, Some("c1".to_string()));
        assert_eq!(data.chunks[0].content, None);
    }

    #[test]
    fn get_id_prefers_id_over_doc_id() {
        let data: IndexData = serde_json::from_value(json!({
            "documents": [{"id": "from-id", "doc_id": "from-doc-id"}],
            "chunks": []
        }))
        .unwrap();

        // get_id() returns id first, then doc_id as fallback
        assert_eq!(data.documents[0].get_id(), Some("from-id"));
    }

    #[test]
    fn get_id_returns_doc_id_when_id_is_none() {
        let data: IndexData = serde_json::from_value(json!({
            "documents": [{"doc_id": "from-doc-id"}],
            "chunks": []
        }))
        .unwrap();

        assert_eq!(data.documents[0].get_id(), Some("from-doc-id"));
    }

    #[test]
    fn get_id_returns_none_when_both_absent() {
        let data: IndexData = serde_json::from_value(json!({
            "documents": [{}],
            "chunks": []
        }))
        .unwrap();

        assert_eq!(data.documents[0].get_id(), None);
    }

    /// extract_documents skips entries with no identifiable id
    #[test]
    fn extract_documents_skips_entries_without_id() {
        let data: IndexData = serde_json::from_value(json!({
            "documents": [
                {"title": "No ID doc"},
                {"id": "has-id", "title": "Has ID"},
                {"doc_id": "has-doc-id", "title": "Has doc_id"}
            ],
            "chunks": []
        }))
        .unwrap();

        let docs = data.extract_documents();
        assert_eq!(docs.len(), 2, "Should skip entry with no id/doc_id");
        assert_eq!(docs[0].id, "has-id");
        assert_eq!(docs[1].id, "has-doc-id");
    }
}
