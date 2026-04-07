//! SPA detection and title extraction.

use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

use super::types::ScrapeResult;

/// Statically compiled H1 regex for extract_title
static H1_REGEX: LazyLock<Option<regex::Regex>> =
    LazyLock::new(|| regex::Regex::new(r"^#\s+(.+)$").ok());

fn h1_regex() -> Option<&'static regex::Regex> {
    H1_REGEX.as_ref()
}

/// Minimum page count threshold for SPA detection
const SPA_DETECTION_PAGE_THRESHOLD: usize = 5;

/// Potential SPA detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaDetectionResult {
    pub is_potential_spa: bool,
    pub pages_scraped: usize,
    pub total_urls_discovered: usize,
    pub warning_message: Option<String>,
}

/// Detect if a site may be a Single-Page Application (SPA) requiring JavaScript rendering.
///
/// Analyzes the scrape result to identify patterns suggesting the site uses client-side
/// rendering that static HTML scraping cannot fully capture.
#[allow(clippy::cast_precision_loss)]
pub fn detect_potential_spa(result: &ScrapeResult) -> SpaDetectionResult {
    let pages_scraped = result.success_count;
    let total_urls = result.total_urls;

    let scrape_ratio = if total_urls > 0 {
        pages_scraped as f64 / total_urls as f64
    } else {
        0.0
    };

    let is_potential_spa = pages_scraped < SPA_DETECTION_PAGE_THRESHOLD
        && total_urls > SPA_DETECTION_PAGE_THRESHOLD
        && scrape_ratio < 0.5;

    let warning_message = if is_potential_spa {
        Some(format!(
            "\u{26a0}\u{fe0f}  POTENTIAL SPA DETECTED\n\
            Only {pages_scraped} pages scraped from {total_urls} discovered URLs.\n\
            This site may require JavaScript rendering to capture all content.\n\
            \n\
            Suggestions:\n\
            - Use a headless browser (Playwright, Puppeteer) for JavaScript rendering\n\
            - Check if the site provides a sitemap.xml with all content URLs\n\
            - Verify the site doesn't use client-side routing (React, Vue, Angular)\n\
            - Consider using --no-sitemap flag if sitemap URLs are empty"
        ))
    } else {
        None
    };

    SpaDetectionResult {
        is_potential_spa,
        pages_scraped,
        total_urls_discovered: total_urls,
        warning_message,
    }
}

/// Extract title from markdown content
/// Uses statically compiled H1 regex for performance
pub fn extract_title(markdown: &str, url: &str) -> String {
    let fallback_title = || {
        url::Url::parse(url).map_or_else(
            |_| "Untitled".to_string(),
            |u| {
                u.path()
                    .trim_matches('/')
                    .split('/')
                    .next_back()
                    .map_or_else(
                        || "Untitled".to_string(),
                        |s| {
                            let decoded: String = url::form_urlencoded::parse(s.as_bytes())
                                .map(|(key, _)| key.into_owned())
                                .collect();
                            decoded.replace(['-', '_'], " ")
                        },
                    )
            },
        )
    };

    let Some(re) = h1_regex() else {
        return fallback_title();
    };

    markdown
        .lines()
        .find_map(|line| {
            re.captures(line.trim())
                .and_then(|caps| caps.get(1))
                .map(|m| m.as_str().to_string())
        })
        .unwrap_or_else(fallback_title)
}
