//! Content filtering module
//!
//! Implements content filtering strategies inspired by Crawl4AI:
//! - Pruning: Remove low-value content based on text density and tag importance
//! - BM25: Query-based relevance filtering (future enhancement)
//!
//! The pruning filter removes navigation, footers, sidebars, and boilerplate
//! while preserving main documentation content.

use scraper::{Html, Selector};

/// Configuration for content filtering
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FilterConfig {
    /// Minimum text density threshold (0.0 - 1.0)
    pub density_threshold: f32,
    /// Minimum word count to keep a section
    pub min_word_count: usize,
    /// Tags to always remove
    pub remove_tags: Vec<String>,
    /// CSS classes/IDs that indicate navigation (to remove)
    pub nav_patterns: Vec<String>,
}

impl Default for FilterConfig {
    fn default() -> Self {
        Self {
            density_threshold: 0.45,
            min_word_count: 10,
            remove_tags: vec![
                "nav".to_string(),
                "footer".to_string(),
                "aside".to_string(),
                "script".to_string(),
                "style".to_string(),
                "noscript".to_string(),
                "iframe".to_string(),
            ],
            nav_patterns: vec![
                "nav".to_string(),
                "sidebar".to_string(),
                "footer".to_string(),
                "header".to_string(),
                "menu".to_string(),
                "breadcrumb".to_string(),
                "pagination".to_string(),
                "toc".to_string(),
                "table-of-contents".to_string(),
            ],
        }
    }
}

/// Result of content filtering
#[derive(Debug)]
#[allow(dead_code)]
pub struct FilterResult {
    /// Cleaned HTML content
    pub html: String,
    /// Number of elements removed
    pub removed_count: usize,
    /// Density score of kept content
    pub density_score: f32,
}

/// Apply pruning filter to HTML content
///
/// This filter:
/// 1. Removes known non-content tags (nav, footer, script, etc.)
/// 2. Removes elements with navigation-related classes/IDs
/// 3. Scores remaining content by text density
/// 4. Keeps only sections above the density threshold
#[allow(dead_code)]
pub fn prune_html(html: &str, _config: &FilterConfig) -> FilterResult {
    let document = Html::parse_document(html);
    let removed_count = 0;

    // Extract main content area if present
    let main_content = extract_main_content(&document);

    // Calculate density score
    let text_length = main_content
        .chars()
        .filter(|c| !c.is_whitespace())
        .count();
    let total_length = main_content.len();
    let density_score = if total_length > 0 {
        text_length as f32 / total_length as f32
    } else {
        0.0
    };

    // For now, return the main content extraction
    // Full pruning would require DOM manipulation which is complex in scraper
    FilterResult {
        html: main_content,
        removed_count,
        density_score,
    }
}

/// Extract main content from HTML document
///
/// Tries to find the main content area using common selectors:
/// 1. <main> tag
/// 2. <article> tag
/// 3. Element with role="main"
/// 4. Common content class names
/// 5. Falls back to <body>
#[allow(dead_code)]
fn extract_main_content(document: &Html) -> String {
    // Priority list of content selectors
    let selectors = [
        "main",
        "article",
        "[role='main']",
        ".content",
        ".main-content",
        ".doc-content",
        ".markdown-body",
        ".post-content",
        "#content",
        "#main",
        ".documentation",
    ];

    for selector_str in selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            if let Some(element) = document.select(&selector).next() {
                return element.text().collect::<Vec<_>>().join(" ");
            }
        }
    }

    // Fall back to body text
    if let Ok(body_selector) = Selector::parse("body") {
        if let Some(body) = document.select(&body_selector).next() {
            return body.text().collect::<Vec<_>>().join(" ");
        }
    }

    // Last resort: all text
    document.root_element().text().collect::<Vec<_>>().join(" ")
}

/// Filter markdown content by removing common boilerplate patterns
///
/// This is applied after HTML→Markdown conversion to clean up any
/// remaining navigation or boilerplate that made it through.
pub fn filter_markdown(markdown: &str, _config: &FilterConfig) -> String {
    let lines: Vec<&str> = markdown.lines().collect();
    let mut result = Vec::new();
    let mut skip_until_heading = false;

    for line in lines {
        let lower = line.to_lowercase();

        // Skip navigation-like sections
        if line.starts_with('#') {
            let heading_text = line.trim_start_matches('#').trim().to_lowercase();
            skip_until_heading = is_nav_heading(&heading_text);
        }

        if skip_until_heading {
            if line.starts_with('#') && !is_nav_heading(&line.to_lowercase()) {
                skip_until_heading = false;
                result.push(line);
            }
            continue;
        }

        // Skip common footer patterns
        if is_footer_line(&lower) {
            continue;
        }

        result.push(line);
    }

    result.join("\n")
}

/// Check if a heading indicates navigation content
fn is_nav_heading(heading: &str) -> bool {
    let nav_headings = [
        "navigation",
        "menu",
        "table of contents",
        "toc",
        "on this page",
        "in this article",
        "related articles",
        "see also",
        "footer",
        "breadcrumb",
    ];
    nav_headings.iter().any(|&h| heading.contains(h))
}

/// Check if a line looks like footer content
fn is_footer_line(line: &str) -> bool {
    let footer_patterns = [
        "copyright",
        "all rights reserved",
        "privacy policy",
        "terms of service",
        "cookie policy",
        "powered by",
        "built with",
        "last updated:",
        "© 20",
    ];
    footer_patterns.iter().any(|&p| line.contains(p))
}

/// Calculate BM25 score for a document against a query
///
/// This is a simplified BM25 implementation for filtering
/// documents by query relevance.
pub fn bm25_score(document: &str, query: &str, avg_doc_length: f32) -> f32 {
    let k1 = 1.2;
    let b = 0.75;

    let doc_words: Vec<&str> = document.split_whitespace().collect();
    let query_words: Vec<&str> = query.split_whitespace().collect();
    let doc_length = doc_words.len() as f32;

    let mut score = 0.0;

    for term in &query_words {
        let term_lower = term.to_lowercase();
        let tf = doc_words
            .iter()
            .filter(|w| w.to_lowercase() == term_lower)
            .count() as f32;

        if tf > 0.0 {
            // Simplified IDF (assume term appears in ~10% of docs)
            let idf = (10.0_f32).ln();

            let numerator = tf * (k1 + 1.0);
            let denominator = tf + k1 * (1.0 - b + b * (doc_length / avg_doc_length));

            score += idf * (numerator / denominator);
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nav_heading() {
        assert!(is_nav_heading("table of contents"));
        assert!(is_nav_heading("navigation"));
        assert!(!is_nav_heading("getting started"));
        assert!(!is_nav_heading("api reference"));
    }

    #[test]
    fn test_is_footer_line() {
        assert!(is_footer_line("copyright 2024 example corp"));
        assert!(is_footer_line("powered by docusaurus"));
        assert!(!is_footer_line("this is regular content"));
    }

    #[test]
    fn test_bm25_score() {
        let doc = "rust programming language systems programming";
        let query = "rust programming";
        let score = bm25_score(doc, query, 100.0);
        assert!(score > 0.0);

        let unrelated = "python web development django";
        let score2 = bm25_score(unrelated, query, 100.0);
        assert!(score > score2);
    }

    #[test]
    fn test_filter_markdown() {
        let md = "# Title\n\nContent here.\n\n## Table of Contents\n\n- Item 1\n- Item 2\n\n## Real Section\n\nMore content.";
        let config = FilterConfig::default();
        let filtered = filter_markdown(md, &config);
        assert!(filtered.contains("Real Section"));
        assert!(!filtered.contains("Table of Contents"));
    }
}
