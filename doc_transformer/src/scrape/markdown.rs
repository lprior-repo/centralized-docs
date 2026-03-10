#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

//! HTML to Markdown conversion utilities
//!
//! Provides functions for extracting headers and converting HTML content.

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use crate::scrape::transformers::extract_headers;

    #[test]
    fn test_extract_headers_reexport() {
        let md = "## Heading Two\n\nSome text.";
        let headers = extract_headers(md);
        assert_eq!(headers.len(), 1);
        assert_eq!(headers[0].text, "Heading Two");
    }
}
