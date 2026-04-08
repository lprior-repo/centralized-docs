//! Link extraction, header parsing, TOC generation, and page relationship indexing.

use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Write;

use super::header_regex;
use crate::scrape::transformers::link_regex;

/// Extract headers from markdown
#[must_use]
pub fn extract_headers(markdown: &str) -> Vec<super::super::validation::Header> {
    let Some(re) = header_regex() else {
        return vec![];
    };
    markdown
        .lines()
        .filter_map(|line| re.captures(line.trim()))
        .filter_map(|caps| {
            let level_match = caps.get(1)?;
            let text_match = caps.get(2)?;
            let level = u8::try_from(level_match.as_str().len()).map_or(1, |v| v);
            let text = text_match.as_str().to_string();
            Some(super::super::validation::Header { level, text })
        })
        .collect::<Vec<_>>()
}

/// Extract internal links from markdown
#[must_use]
pub fn extract_internal_links(markdown: &str, base_url: &str) -> Vec<String> {
    let base = url::Url::parse(base_url).ok();
    let Some(re) = link_regex() else {
        return vec![];
    };

    re.captures_iter(markdown)
        .filter_map(|caps| caps.get(2))
        .flat_map(|href_match| {
            let href = href_match.as_str();

            let resolved = base
                .as_ref()
                .and_then(|b| b.join(href).ok())
                .filter(|resolved| resolved.host() == base.as_ref().and_then(reqwest::Url::host));

            let is_relative = href.starts_with('/') || href.starts_with("./");

            resolved
                .map(|r| vec![r.to_string()])
                .map_or_else(Vec::new, std::convert::identity)
                .into_iter()
                .chain(is_relative.then(|| href.to_string()))
                .collect::<Vec<String>>()
        })
        .collect::<Vec<String>>()
        .into_iter()
        .sorted()
        .dedup()
        .collect()
}

/// Generate table of contents from headers
#[must_use]
pub fn generate_toc(headers: &[super::super::validation::Header]) -> String {
    if headers.is_empty() {
        return String::new();
    }

    headers
        .iter()
        .fold(String::from("## Table of Contents\n\n"), |acc, header| {
            let indent = "  ".repeat(header.level.saturating_sub(1) as usize);
            let anchor = header
                .text
                .to_lowercase()
                .replace(' ', "-")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-')
                .collect::<String>();
            #[allow(unused_mut)] // writeln! macro requires &mut Write — no functional alternative
            let mut acc = acc;
            let _ = writeln!(acc, "{}- [{}](#{})", indent, header.text, anchor);
            acc
        })
        + "\n---\n\n"
}

/// Build inverted indexes from all pages — O(P×L) once.
/// Returns (link→urls, url→page) pair for reuse across multiple lookups.
#[must_use]
pub fn build_link_indexes(
    all_pages: &[super::super::validation::ScrapedPage],
) -> (
    HashMap<&String, Vec<&str>>,
    HashMap<&str, &super::super::validation::ScrapedPage>,
) {
    let link_to_urls: HashMap<&String, Vec<&str>> = all_pages
        .iter()
        .flat_map(|page| page.links.iter().zip(std::iter::repeat(page.url.as_str())))
        .into_group_map();

    let url_to_page: HashMap<&str, &super::super::validation::ScrapedPage> = all_pages
        .iter()
        .map(|page| (page.url.as_str(), page))
        .collect();

    (link_to_urls, url_to_page)
}

/// Find related pages using pre-built indexes — O(L) per call instead of O(P×L).
#[must_use]
#[allow(clippy::implicit_hasher)]
pub fn find_related_pages_with_index<'a>(
    current_page: &super::super::validation::ScrapedPage,
    #[allow(clippy::implicit_hasher)] link_to_urls: &HashMap<&String, Vec<&str>>,
    #[allow(clippy::implicit_hasher)] url_to_page: &HashMap<
        &str,
        &'a super::super::validation::ScrapedPage,
    >,
) -> Vec<&'a super::super::validation::ScrapedPage> {
    current_page
        .links
        .iter()
        .filter_map(|link| link_to_urls.get(link))
        .flatten()
        .filter(|url| **url != current_page.url)
        .map(|url| (*url, ()))
        .into_group_map()
        .into_iter()
        .sorted_by_key(|(_, occurrences)| std::cmp::Reverse(occurrences.len()))
        .take(5)
        .filter_map(|(url, _)| url_to_page.get(url).copied())
        .collect()
}
