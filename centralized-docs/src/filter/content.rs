use scraper::{Html, Selector};
use tap::Pipe;

use super::types::FilterConfig;

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

/// Fast case-insensitive substring search without allocating
#[inline]
pub(super) fn contains_ignore_ascii_case(haystack: &str, needle: &str) -> bool {
    let n_bytes = needle.as_bytes();
    if n_bytes.is_empty() {
        return true;
    }
    haystack
        .as_bytes()
        .windows(n_bytes.len())
        .any(|w| w.eq_ignore_ascii_case(n_bytes))
}

/// Check if a heading indicates navigation content
pub(super) fn is_nav_heading(heading: &str) -> bool {
    NAV_HEADINGS
        .iter()
        .any(|&h| contains_ignore_ascii_case(heading, h))
}

/// Check if a line looks like footer content
pub(super) fn is_footer_line(line: &str) -> bool {
    FOOTER_PATTERNS
        .iter()
        .any(|&p| contains_ignore_ascii_case(line, p))
}

/// Extract main content from HTML document using functional composition
///
/// Tries to find the main content area using common selectors:
/// 1. <main> tag
/// 2. <article> tag
/// 3. Element with role="main"
/// 4. Common content class names
/// 5. Falls back to <body>
#[must_use]
pub fn extract_main_content(document: &Html, config: &FilterConfig) -> String {
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
        .map_or_else(
            || document.root_element().text().collect::<Vec<_>>().join(" "),
            std::convert::identity,
        )
}

/// Filter markdown content by removing common boilerplate patterns
///
/// This is applied after HTML→Markdown conversion to clean up any
/// remaining navigation or boilerplate that made it through.
/// Uses `config.nav_patterns` to identify navigation headings to skip.
/// Uses `config.min_word_count` to filter out sparse sections.
#[must_use]
pub fn filter_markdown(markdown: &str, config: &FilterConfig) -> String {
    /// State for markdown filtering fold operation
    struct FilterState<'a> {
        result: Vec<&'a str>,
        current_section: Vec<&'a str>,
        skip_until_heading: bool,
    }

    /// Check if heading indicates navigation content
    fn is_nav_section(heading_text: &str, cfg: &FilterConfig) -> bool {
        cfg.nav_patterns
            .iter()
            .any(|pattern| contains_ignore_ascii_case(heading_text, pattern))
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
            // Check if this is a heading
            if line.starts_with('#') {
                // Flush previous section if it meets word count
                if !state.current_section.is_empty() {
                    let word_count = state
                        .current_section
                        .iter()
                        .map(|s| s.split_whitespace().count())
                        .sum::<usize>();
                    if word_count >= config.min_word_count {
                        state.result.append(&mut state.current_section);
                    } else {
                        state.current_section.clear();
                    }
                }

                let heading_text = line.trim_start_matches('#').trim();
                state.skip_until_heading = is_nav_section(heading_text, config);
            }

            if state.skip_until_heading {
                if line.starts_with('#') {
                    let heading_text = line.trim_start_matches('#').trim();
                    if !is_nav_section(heading_text, config) {
                        state.skip_until_heading = false;
                        state.current_section.push(line);
                    }
                }
                return state;
            }

            // Skip common footer patterns
            if !is_footer_line(line) {
                state.current_section.push(line);
            }

            state
        })
        .pipe(|mut state| {
            // Flush final section
            if !state.current_section.is_empty() {
                let word_count = state
                    .current_section
                    .iter()
                    .map(|s| s.split_whitespace().count())
                    .sum::<usize>();
                if word_count >= config.min_word_count || state.result.is_empty() {
                    state.result.extend(state.current_section);
                }
            }
            state.result.join("\n")
        })
}
