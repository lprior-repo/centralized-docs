#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

//! Content filtering and relevance scoring
//!
//! Provides BM25-based relevance filtering for scraped pages.

use super::validation::ScrapedPage;

/// Filter scraped pages by BM25 relevance to query
pub fn filter_pages_by_relevance(
    pages: Vec<ScrapedPage>,
    query: &str,
    threshold: f32,
) -> (Vec<ScrapedPage>, usize) {
    if pages.is_empty() {
        return (pages, 0);
    }

    if threshold <= 0.0 {
        return (pages, 0);
    }

    let total_words: usize = pages.iter().map(|p| p.word_count).sum();

    let avg_doc_length = if pages.is_empty() {
        return (pages, 0);
    } else {
        (total_words as f32 / pages.len() as f32).max(1.0)
    };

    let (kept, filtered): (Vec<_>, Vec<_>) = pages.into_iter().partition(|page| {
        let score = bm25_score(&page.markdown, query, avg_doc_length);
        score >= threshold
    });

    let filtered_count = filtered.len();

    (kept, filtered_count)
}

/// Calculate BM25 score for document vs query
fn bm25_score(document: &str, query: &str, _avg_doc_length: f32) -> f32 {
    let doc_words: Vec<&str> = document.split_whitespace().collect();
    let query_words: Vec<&str> = query.split_whitespace().collect();

    if query_words.is_empty() {
        return 0.0;
    }

    let k1 = 1.2_f32;
    let k2 = 0.75_f32;
    let b = 0.75_f32;

    let doc_length = doc_words.len() as f32;
    let dl = (doc_length + k1 * b).min(f32::MAX);

    let mut score: f32 = 0.0;

    for term in &query_words {
        let tf = term_frequency(term, &doc_words);
        let df = document_frequency(term, &query_words);
        let idf = ((query_words.len() as f32 - df as f32 + 0.5) / (df as f32 + 0.5))
            .ln_1p()
            .max(0.0);

        let numerator = (tf * (k1 + 1.0)).min(f32::MAX);
        let denominator = (tf * k2 + dl).min(f32::MAX);

        score = (score + idf * (numerator / denominator)).min(f32::MAX);
    }

    score
}

/// Calculate term frequency in document
fn term_frequency(term: &str, document: &[&str]) -> f32 {
    let count = document.iter().filter(|&&w| w == term).count() as f32;
    let doc_len = document.len() as f32;

    if doc_len == 0.0 {
        0.0
    } else {
        count / doc_len
    }
}

/// Calculate document frequency (inverse)
fn document_frequency(term: &str, query_terms: &[&str]) -> usize {
    query_terms.iter().filter(|&&t| t == term).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_pages_by_relevance_empty() {
        let pages = vec![];
        let (kept, filtered) = filter_pages_by_relevance(pages, "test query", 0.5);
        assert_eq!(kept.len(), 0);
        assert_eq!(filtered, 0);
    }

    #[test]
    fn test_filter_pages_by_relevance_no_threshold() {
        let pages = vec![ScrapedPage {
            url: "https://example.com/1".to_string(),
            markdown: "test content with query".to_string(),
            title: "Test".to_string(),
            links: vec![],
            headers: vec![],
            word_count: 4,
            slug: "test".to_string(),
            filtered: false,
            elements_removed: 0,
            density_score: 1.0,
        }];

        let (kept, filtered) = filter_pages_by_relevance(pages.clone(), "query", 0.0);
        assert_eq!(kept.len(), 1);
        assert_eq!(filtered, 0);

        let (kept2, filtered2) = filter_pages_by_relevance(pages, "nonexistent", 0.5);
        assert_eq!(kept2.len(), 0);
        assert_eq!(filtered2, 1);
    }

    #[test]
    fn test_term_frequency() {
        let doc = vec!["test", "content", "test", "example"];
        assert_eq!(term_frequency("test", &doc), 0.5);
        assert_eq!(term_frequency("nonexistent", &doc), 0.0);
    }

    #[test]
    fn test_document_frequency() {
        let terms = vec!["test", "query", "terms"];
        assert_eq!(document_frequency("test", &terms), 1);
        assert_eq!(document_frequency("nonexistent", &terms), 0);
    }
}
