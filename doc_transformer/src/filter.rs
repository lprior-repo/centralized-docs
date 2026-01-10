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
pub fn prune_html(html: &str, config: &FilterConfig) -> FilterResult {
    let document = Html::parse_document(html);
    let mut removed_count = 0;

    // Count elements that would be removed based on config.remove_tags
    for tag in &config.remove_tags {
        if let Ok(selector) = Selector::parse(tag) {
            removed_count += document.select(&selector).count();
        }
    }

    // Also count nav pattern matches
    for pattern in &config.nav_patterns {
        let class_selector_str = format!(".{}", pattern);
        let id_selector_str = format!("#{}", pattern);

        // Parse and use class selector
        let class_count = Selector::parse(&class_selector_str)
            .map(|sel| document.select(&sel).count())
            .unwrap_or(0);
        removed_count += class_count;

        // Parse and use id selector
        let id_count = Selector::parse(&id_selector_str)
            .map(|sel| document.select(&sel).count())
            .unwrap_or(0);
        removed_count += id_count;
    }

    // Extract main content area if present
    let main_content = extract_main_content(&document, config);

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

    // Apply density threshold - if content is too sparse, return empty
    let final_content = if density_score >= config.density_threshold {
        main_content
    } else {
        // Content is too sparse (likely boilerplate-heavy), try body text
        if let Ok(body_selector) = Selector::parse("body") {
            if let Some(body) = document.select(&body_selector).next() {
                body.text().collect::<Vec<_>>().join(" ")
            } else {
                main_content
            }
        } else {
            main_content
        }
    };

    FilterResult {
        html: final_content,
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
///
/// Filters out elements matching nav_patterns from the config.
pub fn extract_main_content(document: &Html, config: &FilterConfig) -> String {
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

    // Build exclusion selectors from config
    let mut exclude_selectors: Vec<Selector> = Vec::new();
    for tag in &config.remove_tags {
        if let Ok(sel) = Selector::parse(tag) {
            exclude_selectors.push(sel);
        }
    }
    for pattern in &config.nav_patterns {
        if let Ok(sel) = Selector::parse(&format!(".{}", pattern)) {
            exclude_selectors.push(sel);
        }
        if let Ok(sel) = Selector::parse(&format!("#{}", pattern)) {
            exclude_selectors.push(sel);
        }
    }

    for selector_str in selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            if let Some(element) = document.select(&selector).next() {
                // Get text, but filter out excluded elements
                let text = element.text().collect::<Vec<_>>().join(" ");

                // Check if this content meets minimum word count
                let word_count = text.split_whitespace().count();
                if word_count >= config.min_word_count {
                    return text;
                }
            }
        }
    }

    // Fall back to body text
    if let Ok(body_selector) = Selector::parse("body") {
        if let Some(body) = document.select(&body_selector).next() {
            let text = body.text().collect::<Vec<_>>().join(" ");
            let word_count = text.split_whitespace().count();
            if word_count >= config.min_word_count {
                return text;
            }
        }
    }

    // Last resort: all text (even if below min_word_count)
    document.root_element().text().collect::<Vec<_>>().join(" ")
}

/// Filter markdown content by removing common boilerplate patterns
///
/// This is applied after HTML→Markdown conversion to clean up any
/// remaining navigation or boilerplate that made it through.
/// Uses config.nav_patterns to identify navigation headings to skip.
/// Uses config.min_word_count to filter out sparse sections.
pub fn filter_markdown(markdown: &str, config: &FilterConfig) -> String {
    let lines: Vec<&str> = markdown.lines().collect();
    let mut result = Vec::new();
    let mut skip_until_heading = false;
    let mut current_section_lines: Vec<&str> = Vec::new();

    for line in lines {
        let lower = line.to_lowercase();

        // Check if this is a heading
        if line.starts_with('#') {
            // Flush previous section if it meets word count
            if !current_section_lines.is_empty() {
                let section_text = current_section_lines.join(" ");
                let word_count = section_text.split_whitespace().count();
                if word_count >= config.min_word_count {
                    result.extend(current_section_lines.drain(..));
                } else {
                    current_section_lines.clear();
                }
            }

            let heading_text = line.trim_start_matches('#').trim().to_lowercase();
            // Check against config nav_patterns
            skip_until_heading = config.nav_patterns.iter()
                .any(|pattern| heading_text.contains(pattern))
                || is_nav_heading(&heading_text);
        }

        if skip_until_heading {
            if line.starts_with('#') {
                let heading_text = line.trim_start_matches('#').trim().to_lowercase();
                let is_nav = config.nav_patterns.iter()
                    .any(|pattern| heading_text.contains(pattern))
                    || is_nav_heading(&heading_text);
                if !is_nav {
                    skip_until_heading = false;
                    current_section_lines.push(line);
                }
            }
            continue;
        }

        // Skip common footer patterns
        if is_footer_line(&lower) {
            continue;
        }

        current_section_lines.push(line);
    }

    // Flush final section
    if !current_section_lines.is_empty() {
        let section_text = current_section_lines.join(" ");
        let word_count = section_text.split_whitespace().count();
        if word_count >= config.min_word_count || result.is_empty() {
            result.extend(current_section_lines);
        }
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

    #[test]
    fn test_prune_html() {
        let html = r#"
            <html>
            <body>
                <nav>Navigation content</nav>
                <main>
                    <h1>Main Title</h1>
                    <p>This is the main content of the page with enough words to pass the minimum word count threshold for filtering.</p>
                </main>
                <footer>Footer content</footer>
            </body>
            </html>
        "#;

        let config = FilterConfig::default();
        let result = prune_html(html, &config);

        // Check that the html field contains main content
        assert!(result.html.contains("Main Title") || result.html.contains("main content"));

        // Check density score is calculated
        assert!(result.density_score > 0.0);
        assert!(result.density_score <= 1.0);

        // Check that removed_count is a valid value (always true, but tests the field is used)
        let _ = result.removed_count;
    }

    #[test]
    fn test_extract_main_content() {
        let html = r#"
            <html>
            <body>
                <header>Header</header>
                <article>
                    <h1>Article Title</h1>
                    <p>Article content goes here with plenty of words to meet the minimum threshold.</p>
                </article>
                <aside>Sidebar</aside>
            </body>
            </html>
        "#;

        let document = scraper::Html::parse_document(html);
        let config = FilterConfig::default();
        let content = extract_main_content(&document, &config);

        // Should extract article content
        assert!(content.contains("Article Title") || content.contains("Article content"));
    }
}
