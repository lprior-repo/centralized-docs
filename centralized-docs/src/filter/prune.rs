use anyhow::Result;
use readability::extractor;
use scraper::{Html, Selector};
use tap::Pipe;

use super::types::{FilterConfig, FilterResult};

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
/// - Returns `FilterResult` with non-empty `html` field OR `is_empty: true`
/// - `used_readability` indicates extraction method used
/// - `density_score` is always between 0.0 and 1.0
/// - `removed_count` may be 0 if Readability extraction succeeded
///
/// **Invariants:**
/// - Function never panics on any input HTML
/// - Gracefully degrades to fallback if Readability fails
/// - Returns `FilterResult` with explicit `is_empty: true` when no content extracted
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
                    .map_or_else(|_| crate::math_types::Score::zero(), std::convert::identity),
                used_readability: true,
                is_empty: false,
            }
        }
        Err(e) => {
            // Log that we're falling back to custom pruning
            eprintln!(
                "[filter] Content extraction: Readability failed ({e:?}), using fallback pruning"
            );
            // Fallback to custom density-based pruning
            let fallback_result = fallback_prune_html(html, config);

            // Explicit empty content handling: check if both Readability and fallback failed
            let is_content_empty = fallback_result.html.trim().is_empty();

            if is_content_empty {
                eprintln!("[filter] Content extraction: Both Readability and fallback failed - empty content");
                FilterResult {
                    html: String::new(),
                    removed_count: fallback_result.removed_count,
                    density_score: crate::math_types::Score::zero(),
                    used_readability: false,
                    is_empty: true, // Explicit empty state
                }
            } else {
                fallback_result.with_is_empty(false)
            }
        }
    }
}

/// Extract content using Mozilla Readability algorithm
///
/// Attempts to extract the main article content from HTML using the Readability crate.
fn try_readability_extraction(html: &str) -> Result<String, anyhow::Error> {
    use std::io::Cursor;
    use url::Url;

    #[allow(unused_mut)] // I/O boundary: Readability extractor requires &mut Read
    let mut cursor = Cursor::new(html.as_bytes());
    let base_url =
        Url::parse("https://example.com").map_err(|e| anyhow::anyhow!("URL parse error: {e}"))?;

    let product = extractor::extract(&mut cursor, &base_url)
        .map_err(|e| anyhow::anyhow!("Readability extraction failed: {e}"))?;

    Ok(product.content)
}

/// Calculate text density score (ratio of non-whitespace to total characters)
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
    let main_content = super::content::extract_main_content(&document, config);

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
        html: final_content.clone(),
        removed_count,
        density_score: crate::math_types::Score::try_new(density_score)
            .map_or_else(|_| crate::math_types::Score::zero(), std::convert::identity),
        used_readability: false,
        is_empty: final_content.trim().is_empty(),
    }
}
