#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

//! HTML parsing and content extraction
//!
//! This module re-exports page transformation from transformers module.

/// Transform a spider page into ScrapedPage format
///
/// This function is re-exported from transformers module.
pub use super::transformers::transform_page;

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_exists() {
        // This module provides re-exports from transformers
        // Full tests are in transformers module
        // Verify that the transform_page function is accessible
        let _: fn(
            &spider::page::Page,
            &str,
            bool,
        ) -> Result<super::super::validation::ScrapedPage, anyhow::Error> = super::transform_page;
    }
}
