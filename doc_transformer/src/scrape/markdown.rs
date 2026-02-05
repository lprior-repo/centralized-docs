#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

//! HTML to Markdown conversion utilities
//!
//! Provides functions for extracting headers and converting HTML content.

/// Extract headers from markdown content
///
/// This function is re-exported from transformers module.
pub use super::transformers::extract_headers;

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_exists() {
        // This module provides re-exports from transformers
        // Full tests are in transformers module
        assert!(true);
    }
}
