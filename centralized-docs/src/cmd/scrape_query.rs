//! Query filtering logic for scrape results using BM25 ranking.
//!
//! Design by Contract:
//! - **Preconditions:**
//!   - pages may be empty (returns empty with count 0)
//!   - query may be None (returns pages unchanged)
//!   - threshold and pages are valid
//! - **Postconditions:**
//!   - Returns filtered pages and count of removed pages
//!   - All returned pages scored >= threshold (if query provided)
//!   - Logs filtering statistics

use crate::scrape;
use anyhow::Result;

/// Validate query length to prevent `DoS` attacks and resource exhaustion
///
/// Constraints:
/// - Maximum 1000 bytes (prevents regex compilation timeouts)
/// - None/empty queries allowed (no filtering)
pub fn validate_query_length(query: &Option<&str>) -> Result<()> {
    const MAX_QUERY_LENGTH: usize = 1000;

    if let Some(q) = query {
        let byte_count = q.len();
        if byte_count > MAX_QUERY_LENGTH {
            anyhow::bail!("Query too long ({byte_count} bytes, maximum {MAX_QUERY_LENGTH})");
        }
    }

    Ok(())
}

/// Apply BM25 query filtering to scraped pages
///
/// Design by Contract:
/// - **Preconditions:**
///   - pages may be empty (returns empty with count 0)
///   - query may be None (returns pages unchanged)
///   - threshold and pages are valid
/// - **Postconditions:**
///   - Returns filtered pages and count of removed pages
///   - All returned pages scored >= threshold (if query provided)
///   - Logs filtering statistics
///
/// Edge Cases Handled:
/// - Query is None → returns all pages unchanged
/// - Query is empty string → returns all pages (empty query scores all = 0)
/// - threshold <= 0.0 → no filtering applied
/// - threshold = 1.0 → very strict (only highly relevant pages)
/// - All pages filtered out → logs warning and returns empty
/// - Pages with identical content → same score, all kept or all removed together
pub fn apply_query_filter(
    pages: Vec<scrape::ScrapedPage>,
    query: Option<&str>,
    threshold: f32,
) -> Result<Vec<scrape::ScrapedPage>> {
    let Some(raw_query) = query else {
        return Ok(pages);
    };

    let query = raw_query.trim();
    if query.is_empty() || threshold <= 0.0 || pages.is_empty() {
        return Ok(pages);
    }

    let original_len = pages.len();
    let (index, id_field) = build_tantivy_index(&pages)?;
    let valid_ids = score_pages(&index, id_field, query, threshold, pages.len())?;
    let kept_pages = filter_pages(pages, &valid_ids);

    let removed_count = original_len.saturating_sub(kept_pages.len());
    tracing::info!(
        kept = kept_pages.len(),
        query = %query,
        removed = removed_count,
        "Filtered pages by query"
    );

    if kept_pages.is_empty() {
        tracing::warn!("All pages filtered out by query");
        tracing::warn!("Consider lowering the --threshold value");
        anyhow::bail!("All pages filtered out by query '{query}' (threshold: {threshold})");
    }

    Ok(kept_pages)
}

/// Build a RAM-based tantivy index from scraped pages.
///
/// Returns the index reader and the id field for later retrieval.
/// This is a pure function - no I/O beyond memory allocation.
fn build_tantivy_index(
    pages: &[scrape::ScrapedPage],
) -> Result<(tantivy::Index, tantivy::schema::Field)> {
    use tantivy::schema::{Schema, STORED, TEXT};
    use tantivy::Index;

    #[allow(unused_mut)]
    let mut schema_builder = Schema::builder();
    let title_field = schema_builder.add_text_field("title", TEXT);
    let content_field = schema_builder.add_text_field("content", TEXT);
    let id_field = schema_builder.add_u64_field("id", STORED);
    let schema = schema_builder.build();

    let index = Index::create_in_ram(schema);
    #[allow(unused_mut)]
    let mut writer = index.writer(15_000_000)?;

    pages
        .iter()
        .enumerate()
        .try_for_each(|(id, page)| -> Result<()> {
            let doc = tantivy::doc!(
                title_field => page.title.as_str(),
                content_field => page.markdown.as_str(),
                id_field => id as u64
            );
            writer.add_document(doc)?;
            Ok(())
        })?;
    writer.commit()?;

    Ok((index, id_field))
}

/// Score pages using BM25 and return IDs that meet the threshold.
fn score_pages(
    index: &tantivy::Index,
    id_field: tantivy::schema::Field,
    query: &str,
    threshold: f32,
    page_count: usize,
) -> Result<std::collections::HashSet<usize>> {
    use tantivy::collector::TopDocs;
    use tantivy::query::QueryParser;
    use tantivy::schema::Value;

    let reader = index.reader()?;
    let searcher = reader.searcher();
    let title_field = searcher
        .schema()
        .get_field("title")
        .map_err(|e| anyhow::anyhow!("title field not found: {e}"))?;
    let content_field = searcher
        .schema()
        .get_field("content")
        .map_err(|e| anyhow::anyhow!("content field not found: {e}"))?;
    let query_parser = QueryParser::for_index(index, vec![title_field, content_field]);
    let parsed_query = query_parser.parse_query(query)?;

    let top_docs = searcher.search(&parsed_query, &TopDocs::with_limit(page_count))?;

    let valid_ids: std::collections::HashSet<usize> = top_docs
        .iter()
        .filter(|(score, _)| *score >= threshold)
        .filter_map(|(_, doc_address)| {
            let fetched = searcher
                .doc::<tantivy::TantivyDocument>(*doc_address)
                .ok()?;
            let val = fetched.get_first(id_field)?;
            val.as_u64().map(|id_val| id_val as usize)
        })
        .collect();

    Ok(valid_ids)
}

/// Filter pages to only those with IDs in the valid set.
fn filter_pages(
    pages: Vec<scrape::ScrapedPage>,
    valid_ids: &std::collections::HashSet<usize>,
) -> Vec<scrape::ScrapedPage> {
    pages
        .into_iter()
        .enumerate()
        .filter(|(i, _)| valid_ids.contains(i))
        .map(|(_, page)| page)
        .collect()
}

#[cfg(test)]
#[path = "scrape_query_tests.rs"]
mod tests;
