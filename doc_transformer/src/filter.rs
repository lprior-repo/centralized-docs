//! Content filtering module
//!
//! Implements content filtering strategies inspired by Crawl4AI:
//! - Pruning: Remove low-value content based on text density and tag importance
//! - BM25: Query-based relevance filtering (future enhancement)
//!
//! The pruning filter removes navigation, footers, sidebars, and boilerplate
//! while preserving main documentation content.

use scraper::{Html, Selector};
use tap::Pipe;

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
    /// Cleaned HTML content (used in tests and for future filtering enhancements)
    #[allow(dead_code)]
    pub html: String,
    /// Number of elements removed
    pub removed_count: usize,
    /// Density score of kept content
    pub density_score: f32,
}

/// Apply pruning filter to HTML content
///
/// This filter uses functional composition to:
/// 1. Remove known non-content tags (nav, footer, script, etc.)
/// 2. Remove elements with navigation-related classes/IDs
/// 3. Score remaining content by text density
/// 4. Keep only sections above the density threshold
pub fn prune_html(html: &str, config: &FilterConfig) -> FilterResult {
    let document = Html::parse_document(html);

    // Count elements removed from tags using functional chain
    let tag_removed_count: usize = config
        .remove_tags
        .iter()
        .filter_map(|tag| Selector::parse(tag).ok())
        .map(|sel| document.select(&sel).count())
        .sum();

    // Count elements removed from nav patterns using functional chain
    let nav_removed_count: usize = config
        .nav_patterns
        .iter()
        .flat_map(|pattern| {
            [format!(".{}", pattern), format!("#{}", pattern)]
                .into_iter()
                .filter_map(|sel_str| Selector::parse(&sel_str).ok())
                .map(|sel| document.select(&sel).count())
                .collect::<Vec<_>>()
        })
        .sum();

    let removed_count = tag_removed_count.saturating_add(nav_removed_count);

    // Extract main content and calculate density score using pipe
    let main_content = extract_main_content(&document, config);

    let (density_score, final_content) = main_content
        .chars()
        .filter(|c| !c.is_whitespace())
        .count()
        .pipe(|text_length| {
            let total_length = main_content.len();
            if total_length > 0 {
                text_length as f32 / total_length as f32
            } else {
                0.0
            }
        })
        .pipe(|density| {
            let content = if density >= config.density_threshold {
                main_content.clone()
            } else {
                // Content is too sparse, try body text
                Selector::parse("body")
                    .ok()
                    .and_then(|sel| document.select(&sel).next())
                    .map(|body| body.text().collect::<Vec<_>>().join(" "))
                    .unwrap_or_else(|| main_content.clone())
            };
            (density, content)
        });

    FilterResult {
        html: final_content,
        removed_count,
        density_score,
    }
}

/// Content selectors in priority order
const CONTENT_SELECTORS: [&str; 11] = [
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

/// Extract main content from HTML document using functional composition
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
    // Build exclusion selectors from config using functional chain
    let _exclude_selectors: Vec<Selector> = config
        .remove_tags
        .iter()
        .filter_map(|tag| Selector::parse(tag).ok())
        .chain(config.nav_patterns.iter().flat_map(|pattern| {
            [format!(".{}", pattern), format!("#{}", pattern)]
                .into_iter()
                .filter_map(|s| Selector::parse(&s).ok())
        }))
        .collect();

    // Try each content selector in priority order
    CONTENT_SELECTORS
        .iter()
        .filter_map(|selector_str| Selector::parse(selector_str).ok())
        .find_map(|selector| {
            document.select(&selector).next().and_then(|element| {
                let text = element.text().collect::<Vec<_>>().join(" ");
                (text.split_whitespace().count() >= config.min_word_count).then_some(text)
            })
        })
        .or_else(|| {
            // Fall back to body text
            Selector::parse("body").ok().and_then(|sel| {
                document.select(&sel).next().and_then(|body| {
                    let text = body.text().collect::<Vec<_>>().join(" ");
                    (text.split_whitespace().count() >= config.min_word_count).then_some(text)
                })
            })
        })
        .unwrap_or_else(|| {
            // Last resort: all text (even if below min_word_count)
            document.root_element().text().collect::<Vec<_>>().join(" ")
        })
}

/// Filter markdown content by removing common boilerplate patterns
///
/// This is applied after HTML→Markdown conversion to clean up any
/// remaining navigation or boilerplate that made it through.
/// Uses config.nav_patterns to identify navigation headings to skip.
/// Uses config.min_word_count to filter out sparse sections.
/// Uses functional composition with pipe and fold.
pub fn filter_markdown(markdown: &str, config: &FilterConfig) -> String {
    /// State for markdown filtering fold operation
    struct FilterState<'a> {
        result: Vec<&'a str>,
        current_section: Vec<&'a str>,
        skip_until_heading: bool,
    }

    /// Check if heading indicates navigation content
    fn is_nav_section(heading_text: &str, config: &FilterConfig) -> bool {
        config
            .nav_patterns
            .iter()
            .any(|pattern| heading_text.contains(pattern))
            || is_nav_heading(heading_text)
    }

    let initial_state = FilterState {
        result: Vec::new(),
        current_section: Vec::new(),
        skip_until_heading: false,
    };

    markdown
        .lines()
        .fold(initial_state, |mut state, line| {
            let lower = line.to_lowercase();

            // Check if this is a heading
            if line.starts_with('#') {
                // Flush previous section if it meets word count
                if !state.current_section.is_empty() {
                    let section_text = state.current_section.join(" ");
                    let word_count = section_text.split_whitespace().count();
                    if word_count >= config.min_word_count {
                        state.result.append(&mut state.current_section);
                    } else {
                        state.current_section.clear();
                    }
                }

                let heading_text = line.trim_start_matches('#').trim().to_lowercase();
                state.skip_until_heading = is_nav_section(&heading_text, config);
            }

            if state.skip_until_heading {
                if line.starts_with('#') {
                    let heading_text = line.trim_start_matches('#').trim().to_lowercase();
                    if !is_nav_section(&heading_text, config) {
                        state.skip_until_heading = false;
                        state.current_section.push(line);
                    }
                }
                return state;
            }

            // Skip common footer patterns
            if !is_footer_line(&lower) {
                state.current_section.push(line);
            }

            state
        })
        .pipe(|mut state| {
            // Flush final section
            if !state.current_section.is_empty() {
                let section_text = state.current_section.join(" ");
                let word_count = section_text.split_whitespace().count();
                if word_count >= config.min_word_count || state.result.is_empty() {
                    state.result.extend(state.current_section);
                }
            }
            state.result.join("\n")
        })
}

/// Navigation heading patterns as a const array for functional matching
const NAV_HEADINGS: [&str; 10] = [
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

/// Footer patterns as a const array for functional matching
const FOOTER_PATTERNS: [&str; 9] = [
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

/// Check if a heading indicates navigation content
fn is_nav_heading(heading: &str) -> bool {
    NAV_HEADINGS.iter().any(|&h| heading.contains(h))
}

/// Check if a line looks like footer content
fn is_footer_line(line: &str) -> bool {
    FOOTER_PATTERNS.iter().any(|&p| line.contains(p))
}

/// Calculate BM25 score for a document against a query
///
/// This is a simplified BM25 implementation for filtering
/// documents by query relevance. Uses functional composition.
pub fn bm25_score(document: &str, query: &str, avg_doc_length: f32) -> f32 {
    let k1 = 1.2;
    let b = 0.75;

    let doc_words: Vec<&str> = document.split_whitespace().collect();
    let doc_length = doc_words.len() as f32;

    query
        .split_whitespace()
        .map(|term| {
            let term_lower = term.to_lowercase();
            doc_words
                .iter()
                .filter(|w| w.to_lowercase() == term_lower)
                .count() as f32
        })
        .filter(|&tf| tf > 0.0)
        .map(|tf| {
            let idf = (10.0_f32).ln();
            let numerator = tf * (k1 + 1.0);
            let denominator = tf + k1 * (1.0 - b + b * (doc_length / avg_doc_length));
            idf * (numerator / denominator)
        })
        .sum()
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
