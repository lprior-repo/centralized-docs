use anyhow::anyhow;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{OwnedValue, Value};
use tantivy::Index;

use super::{create_schema, SearchError, SearchResult};

/// Escape wildcard characters that would create unintended wildcard queries.
///
/// Only escapes `*` and `?` which would match arbitrary characters.
/// Other special characters (quotes, parentheses, etc.) are left unescaped
/// so that invalid queries still produce helpful error messages.
pub(super) fn escape_tantivy_query(query: &str) -> String {
    query.chars().fold(
        String::with_capacity(query.len().saturating_mul(2)),
        |mut escaped, ch| {
            if matches!(ch, '*' | '?') {
                escaped.push('\\');
            }
            escaped.push(ch);
            escaped
        },
    )
}

/// Search the Tantivy index
///
/// ## Query Syntax
///
/// - Simple: `rust programming` → Any document with both terms
/// - Phrase: `"rust programming"` → Exact phrase match
/// - Boolean: `rust AND systems` → Both terms required
/// - Negation: `rust NOT python` → rust without python
/// - Operators: `(rust OR systems) AND NOT python`
///
/// ## Behavior
///
/// - Parses query using Tantivy's default `QueryParser`
/// - Executes against content field (searchable combination)
/// - Returns top N results sorted by BM25 score (highest first)
/// - Returns empty Vec if no matches
///
/// ## Error Handling
///
/// Returns error if query is invalid (syntax error).
/// Empty query returns error.
///
/// # Arguments
///
/// * `index` - Tantivy index to search
/// * `query_str` - Query string (supports phrase and boolean operators)
/// * `limit` - Maximum number of results to return
///
/// # Returns
///
/// Vector of `SearchResult` sorted by relevance (highest score first)
#[allow(dead_code)] // Exported for library users - not used internally
pub fn search_index(
    index: &Index,
    query_str: &str,
    limit: usize,
) -> std::result::Result<Vec<SearchResult>, SearchError> {
    let (_schema, fields) = create_schema();

    // Validate query using centralized validation
    let query_str = crate::validate::validate_query(query_str)
        .map_err(|e| SearchError::QueryParseError(e.to_string()))?;

    // Validate limit to prevent Tantivy panic (must be > 0)
    let limit = crate::validate::validate_limit(&limit.to_string())
        .map_err(|e| SearchError::QueryParseError(e.to_string()))?;

    // Escape special characters that have meaning in Tantivy query syntax
    // This prevents wildcard queries and other unintended query parsing
    let escaped_query = escape_tantivy_query(query_str);

    // Get reader for searching
    let reader = index.reader().map_err(|e| SearchError::Other(anyhow!(e)))?;
    let searcher = reader.searcher();

    // Parse query
    // Search across title and content. We could add boosts, but simply including title
    // helps find relevant structural matches.
    #[allow(unused_mut)] // tantivy QueryParser::set_field_boost requires &mut self
    let mut query_parser = QueryParser::for_index(index, vec![fields.title, fields.content]);
    query_parser.set_field_boost(fields.title, 3.0); // Boost title matches significantly

    let query = query_parser.parse_query(&escaped_query).map_err(|_e| {
        SearchError::QueryParseError("Search query contains unsupported syntax.".to_string())
    })?;

    // Execute search and get top results
    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(limit))
        .map_err(|e| SearchError::Other(anyhow!(e)))?;

    // Extract stored fields from results
    #[allow(unused_mut)] // Vec::sort_by_key requires &mut self — no functional alternative in std
    let mut results: Vec<SearchResult> = top_docs
        .into_iter()
        .map(
            |(tantivy_score, doc_address)| -> std::result::Result<Option<SearchResult>, SearchError> {
                let retrieved_doc: tantivy::TantivyDocument = searcher.doc(doc_address).map_err(|e| SearchError::Other(anyhow!(e)))?;

                // Extract fields (safely with defaults)
                // Tantivy 0.25: Convert CompactDocValue -> OwnedValue -> extract
                let id = retrieved_doc
                    .get_first(fields.id)
                    .map(OwnedValue::from)
                    .and_then(|v| v.as_ref().as_str().map(std::string::ToString::to_string))
                    .map_or_else(|| "unknown".to_string(), std::convert::identity);

                let title = retrieved_doc
                    .get_first(fields.title)
                    .map(OwnedValue::from)
                    .and_then(|v| v.as_ref().as_str().map(std::string::ToString::to_string))
                    .map_or_else(|| "Untitled".to_string(), std::convert::identity);

                let summary = retrieved_doc
                    .get_first(fields.summary)
                    .map(OwnedValue::from)
                    .and_then(|v| v.as_ref().as_str().map(std::string::ToString::to_string))
                    .map_or_else(|| "No summary available".to_string(), std::convert::identity);

                let category = retrieved_doc
                    .get_first(fields.category)
                    .map(OwnedValue::from)
                    .and_then(|v| v.as_ref().as_str().map(std::string::ToString::to_string))
                    .map_or_else(|| "uncategorized".to_string(), std::convert::identity);

                let _word_count = retrieved_doc
                    .get_first(fields.word_count)
                    .map(OwnedValue::from)
                    .and_then(|v| v.as_ref().as_u64())
                    .map_or(0, |v| v);

                let path = retrieved_doc
                    .get_first(fields.path)
                    .map(OwnedValue::from)
                    .and_then(|v| v.as_ref().as_str().map(std::string::ToString::to_string))
                    .map_or_else(|| format!("docs/{}.md", id.replace('/', "-")), std::convert::identity);

                let score = crate::math_types::Score::try_new(tantivy_score)
                    .map_or_else(|_| crate::math_types::Score::zero(), std::convert::identity);

                Ok(Some(SearchResult {
                    id,
                    title,
                    summary,
                    category,
                    score,
                    path,
                }))
            },
        )
        .filter_map(std::result::Result::transpose)
        .collect::<std::result::Result<Vec<_>, SearchError>>()?;

    results.sort_by_key(|b| std::cmp::Reverse(b.score));

    Ok(results)
}
