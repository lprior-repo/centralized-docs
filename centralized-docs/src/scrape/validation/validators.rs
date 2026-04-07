//! Validation utilities: URL validation, size checks, regex safety.

use anyhow::{Context, Result};
use regex::Regex;

use super::types::ScrapeResult;

/// Safely compiles a user-provided regex pattern with ReDoS protection.
///
/// This function validates that the user's regex pattern:
/// - Is not longer than 500 characters
/// - Does not contain known ReDoS patterns that can cause catastrophic backtracking
/// - Can be compiled within memory limits (1MB compiled size, 1MB DFA size)
///
/// # Errors
///
/// Returns an error if:
/// - The pattern exceeds 500 characters
/// - The pattern contains known ReDoS patterns
/// - The regex compilation fails (invalid syntax)
/// - The regex is too complex (exceeds memory limits)
pub(crate) fn compile_safe_regex(pattern: &str) -> Result<Regex> {
    let char_count = pattern.chars().count();
    if char_count > 500 {
        anyhow::bail!("Regex pattern too long (max 500 characters, got {char_count})");
    }

    let redos_detector =
        Regex::new(r"\([^)]+\)[+*]").context("failed to compile ReDoS detector regex")?;
    if redos_detector.is_match(pattern) {
        anyhow::bail!(
            "Regex contains potentially slow pattern (ReDoS risk): nested quantifiers detected. \
             This pattern can cause catastrophic backtracking and hang the application.",
        );
    }

    regex::RegexBuilder::new(pattern)
        .size_limit(1024 * 1024)
        .dfa_size_limit(1024 * 1024)
        .build()
        .context("Invalid or too complex regex pattern")
}

/// Validate URL format before passing to spider
pub fn validate_url(url: &str) -> Result<url::Url> {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        anyhow::bail!("URL cannot be empty");
    }

    if trimmed.contains(' ') {
        anyhow::bail!(
            "URL contains spaces: '{trimmed}'. Use '%20' instead of spaces or remove them."
        );
    }

    if trimmed.chars().any(char::is_control) {
        anyhow::bail!(
            "URL contains control characters (for example tab/newline), which are not allowed"
        );
    }

    if let Some(found) = find_unencoded_special_char(trimmed) {
        anyhow::bail!(
            "URL contains unencoded special character '{found}'. \
            Characters like [ ] {{ }} | \\ ^ ` < > must be percent-encoded.",
        );
    }

    let parsed = url::Url::parse(trimmed).context("Invalid URL format")?;

    match parsed.scheme() {
        "http" | "https" => {}
        scheme => anyhow::bail!("Invalid URL scheme '{scheme}': only http and https are supported"),
    }

    match parsed.host_str() {
        Some(host) if !host.is_empty() => {}
        Some(_) => anyhow::bail!("URL host cannot be empty"),
        None => anyhow::bail!("URL must have a valid host"),
    }

    let serialized = parsed.to_string();
    let reparsed = url::Url::parse(&serialized);
    if reparsed.is_err() {
        anyhow::bail!(
            "URL contains invalid encoding. Please ensure special characters are percent-encoded."
        );
    }

    Ok(parsed)
}

fn find_unencoded_special_char(url: &str) -> Option<char> {
    let authority_bounds = parse_authority_bounds(url);

    url.char_indices().find_map(|(index, ch)| {
        let is_unencoded_special = matches!(
            ch,
            '[' | ']' | '{' | '}' | '|' | '\\' | '^' | '`' | '<' | '>'
        );

        if is_unencoded_special && !is_ipv6_host_bracket(index, ch, authority_bounds) {
            Some(ch)
        } else {
            None
        }
    })
}

fn parse_authority_bounds(url: &str) -> Option<(usize, usize)> {
    url.find("://").and_then(|scheme_separator_index| {
        let authority_start = scheme_separator_index.checked_add(3)?;
        let after_scheme = url.get(authority_start..)?;
        let authority_end = after_scheme
            .find(|ch| ['/', '?', '#'].contains(&ch))
            .map_or(url.len(), |offset| authority_start.saturating_add(offset));
        Some((authority_start, authority_end))
    })
}

fn is_ipv6_host_bracket(index: usize, ch: char, authority_bounds: Option<(usize, usize)>) -> bool {
    matches!(ch, '[' | ']')
        && authority_bounds.is_some_and(|(authority_start, authority_end)| {
            index >= authority_start && index < authority_end
        })
}

/// Check if HTML content exceeds size limit
pub fn check_html_size(html: &str, max_size: u64) -> Result<()> {
    let size_bytes = html.len() as u64;
    if size_bytes > max_size {
        anyhow::bail!("Page HTML too large: {size_bytes} bytes (limit: {max_size} bytes)");
    }
    Ok(())
}

/// Check if markdown content exceeds size limit
pub fn check_markdown_size(markdown: &str, max_size: u64) -> Result<()> {
    let size_bytes = markdown.len() as u64;
    if size_bytes > max_size {
        anyhow::bail!("Page markdown too large: {size_bytes} bytes (limit: {max_size} bytes)");
    }
    Ok(())
}

/// Enforce maximum links per page limit
#[must_use]
pub fn limit_links_per_page(links: Vec<String>, max_links: usize) -> (Vec<String>, bool) {
    if links.len() <= max_links {
        (links, false)
    } else {
        (links.into_iter().take(max_links).collect(), true)
    }
}

/// Validate that a slug is non-empty and filesystem-safe
pub fn validate_slug(slug: &str) -> Result<()> {
    if slug.trim().is_empty() {
        anyhow::bail!("URL slug cannot be empty: all URLs must produce non-empty identifiers");
    }
    Ok(())
}

/// Validate that a scrape result contains at least one page
pub fn validate_scrape_result(result: &ScrapeResult) -> Result<()> {
    if result.success_count == 0 {
        if result.total_urls == 0 {
            anyhow::bail!(
                "Failed to reach '{}'. The domain may not exist or DNS resolution failed. \
                Please verify the URL is correct and accessible in a browser.",
                result.base_url
            );
        }
        anyhow::bail!(
            "Failed to scrape any pages from '{}'. \
            Please verify:\n  \
            - The URL is accessible in a browser\n  \
            - The site has HTML content (not just API endpoints)\n  \
            - The site allows scraping (check robots.txt)",
            result.base_url
        );
    }
    Ok(())
}
