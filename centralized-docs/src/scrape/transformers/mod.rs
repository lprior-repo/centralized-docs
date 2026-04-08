#![cfg_attr(
    not(test),
    warn(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::pedantic
    )
)]
#![forbid(unsafe_code)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! Data transformation utilities
//!
//! Provides URL-to-slug conversion, exponential backoff calculation,
//! table of contents generation, and related page finding.

mod links;
mod write;

pub use links::{
    build_link_indexes, extract_headers, extract_internal_links, find_related_pages_with_index,
    generate_toc,
};
pub use write::write_scraped_pages;

use crate::filter::filter_markdown;
use crate::filter::{prune_html, FilterConfig, FilterResult};
use anyhow::{Context, Result};
use regex::Regex;
use std::hash::Hasher;

use super::validation::{FilteringMode, PageFilterStatus};

/// Calculate exponential backoff delay with overflow protection
#[must_use]
pub fn calculate_backoff_delay(base_delay_ms: u64, attempt: u32) -> u64 {
    let exponent = attempt.saturating_sub(1).min(62);
    let multiplier = 2_u64.pow(exponent);
    base_delay_ms.saturating_mul(multiplier)
}

/// Convert URL to a filesystem-safe slug
///
/// This function generates a unique slug for each distinct URL, including
/// query parameters and fragments. This prevents slug collisions between
/// URLs like `/docs?page=1` and `/docs?page=2`.
pub fn url_to_slug(url: &str) -> Result<String> {
    let parsed = url::Url::parse(url).context("Failed to parse URL for slug generation")?;

    let path = parsed.path().trim_matches('/');

    let path = path.strip_suffix(".html").map_or(path, |s| s);
    let path = path.strip_suffix(".htm").map_or(path, |s| s);

    let query_suffix = parsed
        .query()
        .filter(|q| !q.is_empty())
        .map(|q| {
            let slug = q
                .replace(['=', '&'], "-")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-')
                .collect::<String>();
            if slug.is_empty() {
                String::new()
            } else {
                format!("--q-{slug}")
            }
        })
        .map_or_else(String::new, std::convert::identity);

    let frag_suffix = parsed
        .fragment()
        .filter(|f| !f.is_empty())
        .map(|f| {
            let slug = f
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-')
                .collect::<String>();
            if slug.is_empty() {
                String::new()
            } else {
                format!("--f-{slug}")
            }
        })
        .map_or_else(String::new, std::convert::identity);

    let raw_slug = format!("{path}{query_suffix}{frag_suffix}").replace(['/', '.'], "-");

    let slug = raw_slug
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>()
        .to_lowercase();

    let slug = slug
        .strip_suffix("-html")
        .map_or(slug.as_str(), |s| s)
        .strip_suffix("-htm")
        .map_or(slug.as_str(), |s| s)
        .to_string();

    let query = parsed.query();
    let fragment = parsed.fragment();

    let slug = if query.is_some() || fragment.is_some() {
        #[allow(unused_mut)]
        // std::hash::Hash::hash requires &mut Hasher — no functional alternative
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        if let Some(q) = query {
            std::hash::Hash::hash(q, &mut hasher);
        }
        if let Some(f) = fragment {
            std::hash::Hash::hash(f, &mut hasher);
        }
        let hash = (hasher.finish() % 10000).to_string();
        format!("{slug}-q{hash}")
    } else {
        slug
    };

    let slug = if slug.len() > 200 {
        let boundary = slug
            .char_indices()
            .take(200)
            .last()
            .map_or(slug.len(), |(i, c)| i.saturating_add(c.len_utf8()));
        slug[..boundary].to_string()
    } else {
        slug
    };

    let slug = if slug.trim().is_empty() {
        "index".to_string()
    } else {
        slug
    };

    super::validation::validate_slug(&slug)?;

    Ok(slug)
}

/// Detect if a page is a rate limit response
#[must_use]
pub fn detect_rate_limit_page(html: &str) -> bool {
    html.contains("429") || {
        let lower = html.to_ascii_lowercase();
        lower.contains("rate limit exceeded") || lower.contains("too many requests")
    }
}

/// Statically compiled header regex for extract_headers
static HEADER_REGEX: std::sync::LazyLock<Option<Regex>> =
    std::sync::LazyLock::new(|| Regex::new(r"^(#{1,6})\s+(.+)$").ok());

pub(crate) fn header_regex() -> Option<&'static Regex> {
    HEADER_REGEX.as_ref()
}

/// Statically compiled link regex for extract_internal_links
static LINK_REGEX: std::sync::LazyLock<Option<Regex>> =
    std::sync::LazyLock::new(|| Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").ok());

pub(crate) fn link_regex() -> Option<&'static Regex> {
    LINK_REGEX.as_ref()
}

/// Transform a spider page into [`ScrapedPage`] format
pub fn transform_page(
    page: &spider::page::Page,
    base_url: &str,
    config: &super::validation::ScrapeConfig,
    filtering_mode: FilteringMode,
) -> Result<super::validation::ScrapedPage> {
    let url = page.get_url().to_string();
    let filter_config = FilterConfig::default();

    let raw_html = page.get_html();

    if detect_rate_limit_page(&raw_html) {
        anyhow::bail!("Rate limit page detected for {url} - skipping");
    }

    super::validation::check_html_size(&raw_html, config.max_page_size_bytes)?;

    let filtering_enabled = filtering_mode == FilteringMode::Enabled;

    let prune_result: FilterResult = if filtering_enabled {
        prune_html(&raw_html, &filter_config)
    } else {
        FilterResult {
            html: raw_html,
            removed_count: 0,
            density_score: crate::math_types::Score::try_new(1.0)
                .map_or_else(|_| crate::math_types::Score::zero(), std::convert::identity),
            used_readability: false,
            is_empty: false,
        }
    };

    let transform_config = spider_transformations::transformation::content::TransformConfig {
        return_format: spider_transformations::transformation::content::ReturnFormat::Markdown,
        ..Default::default()
    };

    let selector_config = if filtering_enabled {
        let exclude_tags: Vec<String> = filter_config
            .remove_tags
            .clone()
            .into_iter()
            .chain(
                filter_config
                    .nav_patterns
                    .iter()
                    .flat_map(|pattern| [format!(".{pattern}"), format!("#{pattern}")]),
            )
            .collect();
        Some(
            spider_transformations::transformation::content::SelectorConfiguration {
                root_selector: None,
                exclude_selector: Some(exclude_tags.join(", ")),
            },
        )
    } else {
        None
    };

    let markdown = spider_transformations::transformation::content::transform_content(
        page,
        &transform_config,
        &None,
        &selector_config,
        &None,
    );

    // Validate that content extraction succeeded - must not be empty or whitespace-only
    if markdown.trim().is_empty() {
        anyhow::bail!(
            "transform_content returned empty result for {url} - content extraction failed"
        );
    }

    let (markdown, filter_status) = if filtering_enabled {
        (
            filter_markdown(&markdown, &filter_config),
            PageFilterStatus::Filtered,
        )
    } else {
        (markdown, PageFilterStatus::Unfiltered)
    };

    super::validation::check_markdown_size(&markdown, config.max_markdown_size_bytes)?;

    let title = super::validation::extract_title(&markdown, &url);

    let headers = extract_headers(&markdown);

    let links = extract_internal_links(&markdown, base_url);
    let (links, was_truncated) =
        super::validation::limit_links_per_page(links, config.max_links_per_page);
    if was_truncated {
        eprintln!(
            "[WARN] Page {} had too many links, truncated to {}",
            url, config.max_links_per_page
        );
    }

    let word_count = markdown.split_whitespace().count();

    let slug = url_to_slug(&url).context(format!(
        "Failed to generate slug for URL {url}: ensure URL has a valid path or hostname"
    ))?;

    Ok(super::validation::ScrapedPage {
        url,
        markdown,
        title,
        links,
        headers,
        word_count,
        slug,
        filter_status,
        elements_removed: prune_result.removed_count,
        density_score: prune_result.density_score.value(),
    })
}

#[cfg(test)]
mod tests_unit;
#[cfg(test)]
mod tests_write;
