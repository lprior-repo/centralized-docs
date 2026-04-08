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

mod content;
mod discover_helper;
mod prune;
mod types;

// Re-exports — preserve all public APIs
pub use content::{extract_main_content, filter_markdown};
pub use discover_helper::discover_test_files;
pub use prune::prune_html;
pub use types::{FilterConfig, FilterResult, FilterStrategy};

// TODO: Re-enable when test files are created in split
// #[cfg(test)]
// mod tests_html;
// #[cfg(test)]
// mod tests_markdown;
