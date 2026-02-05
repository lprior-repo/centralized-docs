#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

//! HTTP client and website configuration
//!
//! Provides spider-rs website building and HTTP request configuration.

use super::validation::ScrapeConfig;

/// Build a spider Website with shared base configuration
pub fn build_website_base(url: &str, config: &ScrapeConfig) -> spider::website::Website {
    let mut website = spider::website::Website::new(url);

    website.configuration.delay = config.delay_ms;
    website.configuration.respect_robots_txt = config.respect_robots;
    website.configuration.user_agent = Some(Box::new(config.user_agent.clone().into()));

    let capped_concurrency = config.concurrency_limit.clamp(1, 2);
    website.configuration.concurrency_limit = Some(capped_concurrency);

    website.configuration.modify_headers = config.stealth_mode;

    website.configuration.retry = config.max_retries.min(u8::MAX as u32) as u8;

    use std::time::Duration;
    website.configuration.request_timeout =
        Some(Box::new(Duration::from_secs(config.request_timeout_secs)));

    website.configuration.redirect_policy = config.redirect_policy.clone();

    website.configuration.max_page_bytes = config.spider_max_page_bytes.map(|v| v as f64);
    website.configuration.max_bytes_allowed = config.spider_max_total_bytes;

    let _ = website.configuration.with_limit(config.max_pages as u32);

    website.configuration.normalize = true;

    website
}

/// Execute scrape operation with retry logic
pub async fn execute_scrape_with_website(
    website: &mut spider::website::Website,
    config: &ScrapeConfig,
) -> anyhow::Result<()> {
    if config.use_sitemap {
        website.scrape_sitemap().await;
    } else {
        website.scrape().await;
    }

    Ok(())
}

/// Extract pages from website
pub fn extract_pages_from_website(
    website: spider::website::Website,
    config: &ScrapeConfig,
) -> super::validation::ScrapeResult {
    use std::collections::HashSet;

    let mut pages = Vec::new();
    let mut errors = Vec::new();
    let mut seen_urls = HashSet::new();
    let mut total_content_size: u64 = 0;

    let binding = website.get_pages();
    let scraped_pages = binding.as_ref();

    let total_urls = match scraped_pages {
        Some(pages) => pages.len(),
        None => 0,
    };

    if let Some(spider_pages) = scraped_pages {
        for page in spider_pages.iter() {
            let url = page.get_url();

            if seen_urls.contains(url) {
                continue;
            }
            seen_urls.insert(url.to_string());

            if pages.len() >= config.max_pages {
                let error_msg = format!(
                    "Reached page limit ({}), stopping scrape. {} URLs remain.",
                    config.max_pages,
                    spider_pages.len().saturating_sub(pages.len())
                );
                errors.push((url.to_string(), error_msg));
                break;
            }

            match super::transformers::transform_page(
                page,
                &config.base_url,
                config.enable_filtering,
            ) {
                Ok(scraped) => {
                    let page_size = scraped.markdown.len() as u64;
                    total_content_size = match total_content_size.checked_add(page_size) {
                        Some(size) => size,
                        None => {
                            let error_msg =
                                "Integer overflow: total content size would exceed u64::MAX"
                                    .to_string();
                            errors.push((url.to_string(), error_msg));
                            break;
                        }
                    };

                    if total_content_size > config.max_total_size_bytes {
                        let error_msg = format!(
                            "Total content size ({} bytes) exceeds limit ({} bytes), stopping scrape",
                            total_content_size, config.max_total_size_bytes
                        );
                        errors.push((url.to_string(), error_msg));
                        break;
                    }

                    pages.push(scraped);
                }
                Err(e) => {
                    let error_msg = format!("Failed to transform page: {e}");
                    errors.push((url.to_string(), error_msg));
                }
            }
        }
    }

    let success_count = pages.len();
    let error_count = errors.len();

    super::validation::ScrapeResult {
        pages,
        total_urls,
        success_count,
        error_count,
        errors,
        base_url: config.base_url.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_website_base() {
        let config = ScrapeConfig::default();
        let website = build_website_base("https://example.com", &config);

        assert_eq!(website.configuration.delay, 1000);
        assert_eq!(website.configuration.respect_robots_txt, true);
    }

    #[test]
    fn test_extract_pages_from_website_empty() {
        let config = ScrapeConfig::default();
        let website = build_website_base("https://example.com", &config);
        let result = extract_pages_from_website(website, &config);

        assert_eq!(result.pages.len(), 0);
        assert_eq!(result.total_urls, 0);
    }
}
