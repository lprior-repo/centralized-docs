//! Web scraping module using spider-rs
//!
//! Provides sequential scraping of documentation sites with HTML-to-Markdown conversion.
//! Designed for AI agent consumption - no complex concurrency, predictable output.

use crate::filter::{filter_markdown, prune_html, FilterConfig, FilterResult};
use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use spider::website::Website;
use spider_transformations::transformation::content::{self, ReturnFormat, SelectorConfiguration, TransformConfig};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Configuration for scraping a documentation site
#[derive(Debug, Clone)]
pub struct ScrapeConfig {
    /// Base URL to scrape (e.g., "https://docs.example.com")
    pub base_url: String,
    /// Use sitemap.xml to discover pages (default: true)
    pub use_sitemap: bool,
    /// Regex pattern to filter URLs (e.g., "^/docs/")
    pub path_filter: Option<String>,
    /// Delay between requests in milliseconds (default: 250)
    pub delay_ms: u64,
    /// User agent string
    pub user_agent: String,
    /// Respect robots.txt (default: true)
    pub respect_robots: bool,
    /// Enable content filtering to remove nav/footer/boilerplate (default: true)
    pub enable_filtering: bool,
}

impl Default for ScrapeConfig {
    fn default() -> Self {
        Self {
            base_url: String::new(),
            use_sitemap: true,
            path_filter: None,
            delay_ms: 250,
            user_agent: "DocTransformer/5.0 (AI Documentation Indexer)".to_string(),
            respect_robots: true,
            enable_filtering: true,
        }
    }
}

/// A scraped page with extracted content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedPage {
    /// Original URL
    pub url: String,
    /// Markdown content (converted from HTML)
    pub markdown: String,
    /// Page title (extracted from H1 or <title>)
    pub title: String,
    /// Internal links found on this page
    pub links: Vec<String>,
    /// Headers extracted (level, text)
    pub headers: Vec<Header>,
    /// Word count of markdown content
    pub word_count: usize,
    /// URL slug for filename
    pub slug: String,
    /// Whether content filtering was applied
    pub filtered: bool,
    /// Number of HTML elements removed by pruning
    pub elements_removed: usize,
    /// Content density score (0.0 - 1.0)
    pub density_score: f32,
}

/// A header extracted from the page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub level: u8,
    pub text: String,
}

/// Result of scraping a site
#[derive(Debug, Serialize, Deserialize)]
pub struct ScrapeResult {
    /// Successfully scraped pages
    pub pages: Vec<ScrapedPage>,
    /// Total URLs discovered
    pub total_urls: usize,
    /// Number of successful scrapes
    pub success_count: usize,
    /// Number of failed scrapes
    pub error_count: usize,
    /// Errors encountered (url, error message)
    pub errors: Vec<(String, String)>,
    /// Base URL that was scraped
    pub base_url: String,
}

/// Scrape a documentation site and return structured results
///
/// This function uses spider-rs internally but presents a sequential interface.
/// Spider handles concurrency; we process results one at a time.
///
/// When `use_sitemap` is true, scrapes using the sitemap for URL discovery.
/// Otherwise uses standard crawling.
pub async fn scrape_site(config: &ScrapeConfig) -> Result<ScrapeResult> {
    let mut website = Website::new(&config.base_url);

    // Configure spider via the website's configuration
    website.configuration.delay = config.delay_ms;
    website.configuration.respect_robots_txt = config.respect_robots;
    website.configuration.user_agent = Some(Box::new(config.user_agent.clone().into()));

    // Perform the scrape - use sitemap scraping if enabled
    if config.use_sitemap {
        // Scrape using sitemap for URL discovery
        website.scrape_sitemap().await;
    } else {
        // Standard crawling
        website.scrape().await;
    }

    // Compile path filter regex if provided
    let path_regex = config
        .path_filter
        .as_ref()
        .map(|p| Regex::new(p))
        .transpose()
        .context("Invalid path filter regex")?;

    // Process results sequentially
    let mut pages = Vec::new();
    let mut errors = Vec::new();
    let mut seen_urls = HashSet::new();

    let binding = website.get_pages();
    let scraped_pages = binding.as_ref();

    let total_urls = scraped_pages.map(|p| p.len()).unwrap_or(0);

    if let Some(spider_pages) = scraped_pages {
        for page in spider_pages.iter() {
            let url = page.get_url();

            // Skip duplicates
            if seen_urls.contains(url) {
                continue;
            }
            seen_urls.insert(url.to_string());

            // Apply path filter
            if let Some(ref regex) = path_regex {
                let path = url::Url::parse(url)
                    .map(|u| u.path().to_string())
                    .unwrap_or_default();
                if !regex.is_match(&path) {
                    continue;
                }
            }

            // Transform HTML to Markdown (with optional filtering)
            match transform_page(page, &config.base_url, config.enable_filtering) {
                Ok(scraped) => pages.push(scraped),
                Err(e) => errors.push((url.to_string(), e.to_string())),
            }
        }
    }

    let success_count = pages.len();
    let error_count = errors.len();

    Ok(ScrapeResult {
        pages,
        total_urls,
        success_count,
        error_count,
        errors,
        base_url: config.base_url.clone(),
    })
}

/// Transform a spider page into our ScrapedPage format
fn transform_page(page: &spider::page::Page, base_url: &str, enable_filtering: bool) -> Result<ScrapedPage> {
    let url = page.get_url().to_string();
    let filter_config = FilterConfig::default();

    // Apply HTML-level pruning to analyze content quality
    let raw_html = page.get_html();
    let prune_result: FilterResult = if enable_filtering {
        prune_html(&raw_html, &filter_config)
    } else {
        FilterResult {
            html: raw_html.clone(),
            removed_count: 0,
            density_score: 1.0,
        }
    };

    // Configure transformation for markdown output
    let mut transform_config = TransformConfig::default();
    transform_config.return_format = ReturnFormat::Markdown;

    // Build selector configuration for HTML filtering (nav, footer, aside, etc.)
    // Uses the same patterns from FilterConfig for consistency
    let selector_config = if enable_filtering {
        let mut exclude_tags: Vec<String> = filter_config.remove_tags.clone();
        for pattern in &filter_config.nav_patterns {
            exclude_tags.push(format!(".{}", pattern));
            exclude_tags.push(format!("#{}", pattern));
        }
        Some(SelectorConfiguration {
            root_selector: None,
            exclude_selector: Some(exclude_tags.join(", ")),
        })
    } else {
        None
    };

    // Transform HTML to Markdown using spider_transformations
    // Args: page, config, url_selector, selector_config, clean_selectors
    let mut markdown = content::transform_content(
        page,
        &transform_config,
        &None,
        &selector_config,
        &None,
    );

    // Apply additional markdown-level content filtering
    let filtered = if enable_filtering {
        markdown = filter_markdown(&markdown, &filter_config);
        true
    } else {
        false
    };

    // Extract title from markdown (first H1) or fall back to URL
    let title = extract_title(&markdown, &url);

    // Extract headers from markdown
    let headers = extract_headers(&markdown);

    // Extract internal links
    let links = extract_internal_links(&markdown, base_url);

    // Count words
    let word_count = markdown.split_whitespace().count();

    // Generate slug from URL
    let slug = url_to_slug(&url);

    Ok(ScrapedPage {
        url,
        markdown,
        title,
        links,
        headers,
        word_count,
        slug,
        filtered,
        elements_removed: prune_result.removed_count,
        density_score: prune_result.density_score,
    })
}

/// Extract title from markdown content
fn extract_title(markdown: &str, url: &str) -> String {
    // Look for first H1
    let h1_regex = Regex::new(r"^#\s+(.+)$").unwrap();
    for line in markdown.lines() {
        if let Some(caps) = h1_regex.captures(line.trim()) {
            return caps.get(1).unwrap().as_str().to_string();
        }
    }

    // Fall back to URL path
    url::Url::parse(url)
        .map(|u| {
            u.path()
                .trim_matches('/')
                .split('/')
                .last()
                .unwrap_or("Untitled")
                .replace('-', " ")
                .replace('_', " ")
        })
        .unwrap_or_else(|_| "Untitled".to_string())
}

/// Extract headers from markdown
fn extract_headers(markdown: &str) -> Vec<Header> {
    let header_regex = Regex::new(r"^(#{1,6})\s+(.+)$").unwrap();
    let mut headers = Vec::new();

    for line in markdown.lines() {
        if let Some(caps) = header_regex.captures(line.trim()) {
            let level = caps.get(1).unwrap().as_str().len() as u8;
            let text = caps.get(2).unwrap().as_str().to_string();
            headers.push(Header { level, text });
        }
    }

    headers
}

/// Extract internal links from markdown
fn extract_internal_links(markdown: &str, base_url: &str) -> Vec<String> {
    let link_regex = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
    let base = url::Url::parse(base_url).ok();
    let mut links = Vec::new();

    for caps in link_regex.captures_iter(markdown) {
        let href = caps.get(2).unwrap().as_str();

        // Check if internal link
        if let Some(ref base) = base {
            if let Ok(resolved) = base.join(href) {
                if resolved.host() == base.host() {
                    links.push(resolved.to_string());
                }
            }
        } else if href.starts_with('/') || href.starts_with("./") {
            links.push(href.to_string());
        }
    }

    links.sort();
    links.dedup();
    links
}

/// Convert URL to a filesystem-safe slug
fn url_to_slug(url: &str) -> String {
    let path = url::Url::parse(url)
        .map(|u| u.path().to_string())
        .unwrap_or_else(|_| url.to_string());

    path.trim_matches('/')
        .replace('/', "-")
        .replace('.', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>()
        .to_lowercase()
}

/// Write scraped pages to output directory
pub fn write_scraped_pages(result: &ScrapeResult, output_dir: &Path) -> Result<()> {
    let scrape_dir = output_dir.join(".scrape");
    fs::create_dir_all(&scrape_dir)?;

    for page in &result.pages {
        let filename = format!("{}.md", page.slug);
        let filepath = scrape_dir.join(&filename);

        // Write markdown with metadata header
        let content = format!(
            "---\nurl: {}\ntitle: {}\nword_count: {}\nfiltered: {}\nelements_removed: {}\ndensity_score: {:.2}\n---\n\n{}",
            page.url, page.title, page.word_count, page.filtered, page.elements_removed, page.density_score, page.markdown
        );

        fs::write(&filepath, content)?;
    }

    // Write manifest
    let manifest = serde_json::to_string_pretty(result)?;
    fs::write(scrape_dir.join("manifest.json"), manifest)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_to_slug() {
        assert_eq!(url_to_slug("https://example.com/docs/getting-started"), "docs-getting-started");
        assert_eq!(url_to_slug("https://example.com/api/v1/users.html"), "api-v1-users-html");
        assert_eq!(url_to_slug("https://example.com/"), "");
    }

    #[test]
    fn test_extract_title() {
        let md = "# Getting Started\n\nThis is content.";
        assert_eq!(extract_title(md, "https://example.com/foo"), "Getting Started");

        let md_no_h1 = "Some content without header";
        assert_eq!(extract_title(md_no_h1, "https://example.com/getting-started"), "getting started");
    }

    #[test]
    fn test_extract_headers() {
        let md = "# Title\n## Section 1\n### Subsection\n## Section 2";
        let headers = extract_headers(md);
        assert_eq!(headers.len(), 4);
        assert_eq!(headers[0].level, 1);
        assert_eq!(headers[0].text, "Title");
        assert_eq!(headers[1].level, 2);
        assert_eq!(headers[2].level, 3);
    }

    #[test]
    fn test_extract_internal_links() {
        let md = "[Link 1](/docs/page1) and [Link 2](https://example.com/docs/page2) and [External](https://other.com/page)";
        let links = extract_internal_links(md, "https://example.com");
        assert_eq!(links.len(), 2);
        assert!(links.iter().any(|l| l.contains("page1")));
        assert!(links.iter().any(|l| l.contains("page2")));
    }
}
