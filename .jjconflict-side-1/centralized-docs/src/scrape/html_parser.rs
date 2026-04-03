#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

//! HTML parsing and content extraction
//!
//! This module re-exports page transformation from transformers module.
//!
//! # Re-exports
//!
//! - `transform_page` - Function from transformers module
#[allow(unused_imports)]
pub use super::transformers::transform_page;

#[cfg(test)]
mod tests {
    use crate::scrape::transformers::transform_page;

    #[test]
    fn test_module_exists() {
        // Verify that the transform_page function is accessible
        use crate::scrape::validation::FilteringMode;
        let _: fn(
            &spider::page::Page,
            &str,
            &super::super::validation::ScrapeConfig,
            FilteringMode,
        ) -> Result<super::super::validation::ScrapedPage, anyhow::Error> = transform_page;
    }
}
