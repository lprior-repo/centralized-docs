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
    use super::*;

    #[test]
    fn test_extract_headers_reexport() {
        let md = "## Heading Two\n\nSome text.";
        let headers = extract_headers(md);
        assert_eq!(headers.len(), 1);
        assert_eq!(headers[0].text, "Heading Two");
    }
}
