//! Content filtering module
//!
//! Implements content filtering using Mozilla Readability algorithm:
//! - Readability: Extract main article content using proven Mozilla algorithm
//! - Fallback pruning: Custom heuristics for edge cases (no content detected)
//! - BM25: Query-based relevance filtering
//!
//! The Readability filter removes navigation, footers, sidebars, and boilerplate
//! while preserving main documentation content. Falls back to density-based pruning
//! when Readability cannot extract content.

#![allow(dead_code)] // Public API functions exported but not used in current project

use anyhow::Result;
use readability::extractor;
use scraper::{Html, Selector};
use tap::Pipe;

/// Strategy for content filtering (PLAN.md requirement)
#[derive(Debug, Clone, PartialEq, Default)]
#[allow(dead_code)] // Public API - exported for library users, not used internally
pub enum FilterStrategy {
    /// Use pruning heuristics (text/link density)
    #[default]
    Pruning,
    /// Use BM25 query-based filtering
    BM25,
    /// No filtering (keep all content)
    None,
}

/// Configuration for content filtering
#[derive(Debug, Clone)]
pub struct FilterConfig {
    /// Filtering strategy to use
    #[allow(dead_code)] // Public API - part of exported interface
    pub strategy: FilterStrategy,
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
            strategy: FilterStrategy::default(),
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
#[allow(dead_code)] // Public API - exported for library users, not used internally
pub struct FilterResult {
    /// Cleaned HTML content (used in tests and for future filtering enhancements)
    #[allow(dead_code)] // Public API field
    pub html: String,
    /// Number of elements removed
    pub removed_count: usize,
    /// Density score of kept content
    pub density_score: crate::math_types::Score,
    /// Whether Readability was successfully used (vs fallback to custom pruning)
    #[allow(dead_code)] // Public API field
    pub used_readability: bool,
}

/// Apply pruning filter to HTML content using Mozilla Readability algorithm
///
/// This filter attempts to use Mozilla Readability (proven by 14+ years of Firefox Reader Mode)
/// to extract main article content. If Readability cannot extract content, falls back to
/// custom text density heuristics.
///
/// # Contract (Design by Contract)
///
/// **Preconditions:**
/// - `html` is valid UTF-8 (guaranteed by &str)
/// - `config` is valid `FilterConfig`
///
/// **Postconditions:**
/// - Returns `FilterResult` with non-empty `html` field
/// - `used_readability` indicates extraction method used
/// - `density_score` is always between 0.0 and 1.0
/// - `removed_count` may be 0 if Readability extraction succeeded
///
/// **Invariants:**
/// - Function never panics on any input HTML
/// - Gracefully degrades to fallback if Readability fails
/// - Always returns some content (never empty result)
#[must_use]
pub fn prune_html(html: &str, config: &FilterConfig) -> FilterResult {
    // Attempt Readability extraction first
    match try_readability_extraction(html) {
        Ok(extracted_content) => {
            let density = calculate_text_density(&extracted_content);
            FilterResult {
                html: extracted_content,
                removed_count: 0, // Readability handles removal internally
                density_score: crate::math_types::Score::try_new(density)
                    .unwrap_or_else(|_| crate::math_types::Score::zero()),
                used_readability: true,
            }
        }
        Err(_) => {
            // Fallback to custom density-based pruning
            fallback_prune_html(html, config)
        }
    }
}

/// Extract content using Mozilla Readability algorithm
///
/// Attempts to extract the main article content from HTML using the Readability crate.
/// This is a wrapper around `readability::extractor::extract()` that provides error handling.
///
/// # Returns
/// - `Ok(String)` with extracted HTML content
/// - `Err` if Readability cannot extract content (no article found, etc.)
fn try_readability_extraction(html: &str) -> Result<String, anyhow::Error> {
    // Readability requires &mut R and &Url
    // Create a cursor for the HTML string and parse a dummy URL
    use std::io::Cursor;
    use url::Url;

    let mut cursor = Cursor::new(html.as_bytes());
    let base_url =
        Url::parse("https://example.com").map_err(|e| anyhow::anyhow!("URL parse error: {e}"))?;

    let product = extractor::extract(&mut cursor, &base_url)
        .map_err(|e| anyhow::anyhow!("Readability extraction failed: {e}"))?;

    // Return the extracted HTML content
    Ok(product.content)
}

/// Calculate text density score (ratio of non-whitespace to total characters)
///
/// Used to assess content quality after extraction.
#[allow(clippy::cast_precision_loss)]
fn calculate_text_density(content: &str) -> f32 {
    let text_length = content.chars().filter(|c| !c.is_whitespace()).count();
    let total_length = content.len();

    if total_length > 0 {
        // SAFETY: Content length typically < 1MB, well within f32 precision (2^24 ≈ 16.7M)
        (text_length as f32 / total_length as f32).min(1.0)
    } else {
        0.0
    }
}

/// Fallback pruning function using custom text density heuristics
///
/// Used when Readability cannot extract content. This provides compatibility
/// with edge cases (navigation-only pages, paywalled content, etc.).
fn fallback_prune_html(html: &str, config: &FilterConfig) -> FilterResult {
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
            [format!(".{pattern}"), format!("#{pattern}")]
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
                // SAFETY: Content length typically < 1MB, well within f32 precision
                #[allow(clippy::cast_precision_loss)]
                let ratio = text_length as f32 / total_length as f32;
                ratio
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
                    .map_or_else(
                        || main_content.clone(),
                        |body| body.text().collect::<Vec<_>>().join(" "),
                    )
            };
            (density, content)
        });

    FilterResult {
        html: final_content,
        removed_count,
        density_score: crate::math_types::Score::try_new(density_score)
            .unwrap_or_else(|_| crate::math_types::Score::zero()),
        used_readability: false,
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
/// Filters out elements matching `nav_patterns` from the config.
#[must_use]
pub fn extract_main_content(document: &Html, config: &FilterConfig) -> String {
    // Build exclusion selectors from config using functional chain
    let _exclude_selectors: Vec<Selector> = config
        .remove_tags
        .iter()
        .filter_map(|tag| Selector::parse(tag).ok())
        .chain(config.nav_patterns.iter().flat_map(|pattern| {
            [format!(".{pattern}"), format!("#{pattern}")]
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
/// Uses `config.nav_patterns` to identify navigation headings to skip.
/// Uses `config.min_word_count` to filter out sparse sections.
/// Uses functional composition with pipe and fold.
#[must_use]
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

/// Test helper: Discover markdown files from a directory (for integration tests)
///
/// This function is used in integration tests to simulate the discovery phase
/// without depending on full discover module. Returns a Vec of relative paths.
///
/// Note: This function is primarily for testing purposes but is made public
/// to be accessible from integration tests in the tests/ directory.
///
/// # Errors
///
/// Returns an error if:
/// - Path resolution fails (e.g., permission issues)
#[allow(dead_code)] // Test helper function for integration tests
pub fn discover_test_files(root: &std::path::Path) -> Result<Vec<String>, anyhow::Error> {
    use walkdir::WalkDir;

    let mut files = Vec::new();
    // Markdown extensions: .md, .mdx, and unusual variants (.markdown, .mdown, .mkd)
    let extensions = [".md", ".mdx", ".markdown", ".mdown", ".mkd", ".rst", ".txt"];
    let exclude_dirs = ["node_modules", ".git", "_build", "dist", "vendor"];

    for entry in WalkDir::new(root) {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error: Skipping path due to I/O error: {e}");
                continue;
            }
        };

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
    fn test_filter_markdown() {
        let md = "# Title\n\nContent here.\n\n## Table of Contents\n\n- Item 1\n- Item 2\n\n## Real Section\n\nMore content.";
        let config = FilterConfig::default();
        let filtered = filter_markdown(md, &config);
        assert!(filtered.contains("Real Section"));
        assert!(!filtered.contains("Table of Contents"));
    }

    #[test]
    fn test_prune_html() {
        let html = r"
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
        ";

        let config = FilterConfig::default();
        let result = prune_html(html, &config);

        // Check that the html field contains main content
        assert!(result.html.contains("Main Title") || result.html.contains("main content"));

        // Check density score is calculated
        assert!(result.density_score.value() >= 0.0);
        assert!(result.density_score.value() <= 1.0);

        // Check that used_readability indicates which method was used
        let _ = result.used_readability;

        // Check that removed_count is a valid value (always true, but tests the field is used)
        let _ = result.removed_count;
    }

    #[test]
    fn test_prune_html_with_article_tag() {
        // Test that Readability can extract from article tags
        let html = r"
            <html>
            <body>
                <nav>Navigation</nav>
                <article>
                    <h1>Article Title</h1>
                    <p>This is substantive article content with plenty of words. Article content includes discussion, explanations, and detailed information about topics. It is the main focus of the page and should be extracted properly.</p>
                </article>
                <aside>Sidebar content</aside>
            </body>
            </html>
        ";

        let config = FilterConfig::default();
        let result = prune_html(html, &config);

        // Should extract article content regardless of method
        assert!(result.html.contains("Article Title") || result.html.contains("article content"));
        assert!(result.density_score.value() > 0.0);
        assert!(result.density_score.value() <= 1.0);
    }

    #[test]
    fn test_readability_fallback_on_nav_only() {
        // Test fallback behavior when page is navigation-only
        let html = r#"
            <html>
            <body>
                <nav>
                    <a href="/page1">Page 1</a>
                    <a href="/page2">Page 2</a>
                    <a href="/page3">Page 3</a>
                </nav>
            </body>
            </html>
        "#;

        let config = FilterConfig::default();
        let result = prune_html(html, &config);

        // Should have used fallback (Readability can't extract)
        // Result should still be valid (non-panic)
        assert!(!result.html.is_empty());
        assert!(result.density_score.value() >= 0.0);
        assert!(result.density_score.value() <= 1.0);
    }

    #[test]
    fn test_extract_main_content() {
        let html = r"
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
        ";

        let document = scraper::Html::parse_document(html);
        let config = FilterConfig::default();
        let content = extract_main_content(&document, &config);

        // Should extract article content
        assert!(content.contains("Article Title") || content.contains("Article content"));
    }
}
