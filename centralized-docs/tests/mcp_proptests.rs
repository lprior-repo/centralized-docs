use doc_transformer::mcp::types::{GetRelatedConceptsParams, ReadChunkParams, SearchDocsParams};
use proptest::prelude::*;

proptest! {
    #[test]
    fn valid_search_docs_params_parse(
        query in ".*{1,1024}", // UTF-8 string up to 1024 bytes
        limit in 1u32..=1000u32
    ) {
        if query.len() <= 1024 && !query.is_empty() {
            let json = serde_json::json!({
                "query": query,
                "limit": limit
            });
            let result: Result<SearchDocsParams, _> = serde_json::from_value(json);
            prop_assert!(result.is_ok());
        }
    }

    #[test]
    fn invalid_search_docs_params_fail_validation(
        limit in 1001u32..=u32::MAX
    ) {
        let json = serde_json::json!({
            "query": "valid",
            "limit": limit
        });
        let result: Result<SearchDocsParams, _> = serde_json::from_value(json);
        prop_assert!(result.is_err());
    }

    #[test]
    fn valid_read_chunk_params_parse(
        id in "[a-zA-Z0-9_-]{1,256}"
    ) {
        let json = serde_json::json!({
            "id": id
        });
        let result: Result<ReadChunkParams, _> = serde_json::from_value(json);
        prop_assert!(result.is_ok());
    }

    #[test]
    fn valid_get_related_concepts_params_parse(
        id in "[a-zA-Z0-9_-]{1,256}"
    ) {
        let json = serde_json::json!({
            "id": id
        });
        let result: Result<GetRelatedConceptsParams, _> = serde_json::from_value(json);
        prop_assert!(result.is_ok());
    }
}
