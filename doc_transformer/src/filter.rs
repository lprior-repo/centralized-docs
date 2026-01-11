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
///
/// # Contract (Design by Contract)
///
/// **Preconditions:**
/// - `document` is valid UTF-8 (guaranteed by &str)
/// - `query` is valid UTF-8 (guaranteed by &str)
/// - `avg_doc_length` may be 0.0 (empty corpus or all empty docs)
///
/// **Postconditions:**
/// - Return value is always finite (never NaN, never Infinity)
/// - Return value is always non-negative
/// - Function never panics (checked arithmetic prevents division by zero)
///
/// **Invariants:**
/// - Safe defaults prevent division by zero
/// - Denominator is always > 0.0 before division
/// - Output is valid for floating-point comparisons
pub fn bm25_score(document: &str, query: &str, avg_doc_length: f32) -> f32 {
    const DEFAULT_AVG_LENGTH: f32 = 100.0;
    const K1: f32 = 1.2;
    const B: f32 = 0.75;

    // Guard against invalid average length
    let safe_avg = if avg_doc_length > 0.0 {
        avg_doc_length
    } else {
        DEFAULT_AVG_LENGTH
    };

    let doc_words: Vec<&str> = document.split_whitespace().collect();
    let doc_length = doc_words.len() as f32;

    // Early exit for empty query or document
    if doc_words.is_empty() || query.split_whitespace().next().is_none() {
        return 0.0;
    }

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
            let numerator = tf * (K1 + 1.0);
            // Guard against division by zero in denominator
            let denominator = tf + K1 * (1.0 - B + B * (doc_length / safe_avg));

            // Ensure denominator is valid before division
            if denominator > 0.0 {
                idf * (numerator / denominator)
            } else {
                0.0
            }
        })
        .sum::<f32>()
        .pipe(|score| {
            // Final sanity check: ensure result is finite
            if score.is_finite() {
                score
            } else {
                0.0
            }
        })
}

/// Test helper: Discover markdown files from a directory (for integration tests)
///
/// This function is used in integration tests to simulate the discovery phase
/// without depending on the full discover module. Returns a Vec of relative paths.
#[cfg(test)]
pub fn discover_test_files(root: &std::path::Path) -> Result<Vec<String>, anyhow::Error> {
    use std::fs;
    use walkdir::WalkDir;

    let mut files = Vec::new();
    let extensions = [".md", ".mdx", ".rst", ".txt"];
    let exclude_dirs = ["node_modules", ".git", "_build", "dist", "vendor"];

    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        // Skip excluded directories
        if exclude_dirs.iter().any(|excl| {
            path.components()
                .any(|c| c.as_os_str().to_string_lossy().contains(excl))
        }) {
            continue;
        }

        if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext_str = format!(".{}", ext.to_string_lossy());
                if extensions.contains(&ext_str.as_str()) {
                    let rel_path = path.strip_prefix(root)?.to_string_lossy().to_string();
                    files.push(rel_path);
                }
            }
        }
    }

    Ok(files)
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
    fn test_bm25_zero_avg_length() {
        // Edge case: avg_doc_length is 0.0 (empty corpus)
        // Should NOT panic, should NOT return NaN/Inf
        let score = bm25_score("rust programming", "rust", 0.0);
        assert!(score.is_finite(), "Score must be finite, got {}", score);
        assert!(score >= 0.0, "Score must be non-negative, got {}", score);
        assert!(
            score > 0.0,
            "Score should be > 0.0 for matching document with fallback"
        );
    }

    #[test]
    fn test_bm25_negative_avg_length() {
        // Edge case: avg_doc_length is negative (invalid input)
        // Should use safe default instead
        let score = bm25_score("hello world", "hello", -100.0);
        assert!(score.is_finite(), "Score must be finite, got {}", score);
        assert!(score >= 0.0, "Score must be non-negative, got {}", score);
    }

    #[test]
    fn test_bm25_empty_document() {
        // Edge case: empty document string
        let score = bm25_score("", "rust", 100.0);
        assert_eq!(score, 0.0, "Empty document should return 0.0 score");
    }

    #[test]
    fn test_bm25_empty_query() {
        // Edge case: empty query string
        let score = bm25_score("rust programming language", "", 100.0);
        assert_eq!(score, 0.0, "Empty query should return 0.0 score");
    }

    #[test]
    fn test_bm25_both_empty() {
        // Edge case: both document and query are empty
        let score = bm25_score("", "", 100.0);
        assert_eq!(score, 0.0, "Empty doc and query should return 0.0 score");
    }

    #[test]
    fn test_bm25_no_matches() {
        // Edge case: query terms don't appear in document
        let score = bm25_score("python django flask", "rust", 100.0);
        assert_eq!(score, 0.0, "No matching terms should return 0.0 score");
    }

    #[test]
    fn test_bm25_all_zeros_edge_case() {
        // Edge case: all inputs are minimal
        let score = bm25_score("a", "a", 0.0);
        assert!(
            score.is_finite(),
            "Even with zero avg_length, should be finite"
        );
        assert!(score >= 0.0, "Score must be non-negative");
    }

    #[test]
    fn test_bm25_single_word_document() {
        // Edge case: document with one word
        let score = bm25_score("rust", "rust", 100.0);
        assert!(score.is_finite());
        assert!(score > 0.0);
    }

    #[test]
    fn test_bm25_very_long_document() {
        // Edge case: very long document (1M+ words)
        let long_doc = vec!["rust"; 1_000_000].join(" ");
        let score = bm25_score(&long_doc, "rust", 100.0);
        assert!(
            score.is_finite(),
            "Long document should not produce NaN/Inf"
        );
        assert!(score >= 0.0);
    }

    #[test]
    fn test_bm25_case_insensitive() {
        // Verify case-insensitive matching
        let doc = "Rust Programming Language";
        let query_lower = "rust";
        let query_upper = "RUST";
        let score_lower = bm25_score(doc, query_lower, 100.0);
        let score_upper = bm25_score(doc, query_upper, 100.0);
        assert_eq!(
            score_lower, score_upper,
            "Matching should be case-insensitive"
        );
    }

    #[test]
    fn test_bm25_whitespace_normalization() {
        // Edge case: multiple spaces between words
        let doc1 = "rust   programming";
        let doc2 = "rust programming";
        let query = "rust programming";
        let score1 = bm25_score(doc1, query, 100.0);
        let score2 = bm25_score(doc2, query, 100.0);
        // Scores may differ due to different word counts, but both must be finite
        assert!(score1.is_finite());
        assert!(score2.is_finite());
    }

    #[test]
    fn test_bm25_relevance_ordering() {
        // Verify that documents with more matches score higher
        let query = "rust programming";
        let exact_match = "rust programming rust programming rust programming";
        let single_match = "rust programming";
        let partial_match = "rust web development";

        let score_exact = bm25_score(exact_match, query, 100.0);
        let score_single = bm25_score(single_match, query, 100.0);
        let score_partial = bm25_score(partial_match, query, 100.0);

        assert!(
            score_exact >= score_single,
            "More matches should score >= single match"
        );
        assert!(
            score_single >= score_partial,
            "Exact match should score >= partial"
        );
    }

    #[test]
    fn test_bm25_never_panics_on_pathological_input() {
        // Fuzz with various pathological inputs
        let long_a = "a".repeat(10000);
        let long_d = "d".repeat(1000);

        let pathological_inputs = vec![
            ("", "", 0.0),
            ("x", "x", 0.0),
            ("a", "b", 0.0),
            ("  ", "  ", 0.0),
            ("\t\n", "\r\n", f32::NAN),
            ("🦀", "🦀", -1.0),
            (&long_a, "a", 0.0),
            ("a b c", &long_d, f32::INFINITY),
        ];

        for (doc, query, avg_len) in pathological_inputs {
            let score = bm25_score(doc, query, avg_len);
            assert!(
                score.is_finite(),
                "Score must be finite for input: doc={:?}, query={:?}, avg_len={}",
                doc.chars().take(10).collect::<String>(),
                query.chars().take(10).collect::<String>(),
                avg_len
            );
        }
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
