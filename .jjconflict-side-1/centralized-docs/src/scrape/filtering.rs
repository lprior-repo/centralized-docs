#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

//! Content filtering and relevance scoring
//!
//! Reserved for future BM25-based relevance filtering of scraped pages.
//! Currently unused — filtering decisions are made upstream in `transformers`.
