//! Implementation for bead cdocs-90e: Load archived scrape outputs for unchanged pages.
//!
//! Data flow:
//! ```text
//! Input:  fresh_result (ScrapeResult), session (StateReadSession)
//!             |
//!             v
//!     [compute_page_content_hash for each page]
//!             |
//!             v
//!     [[u8; 32]] fresh_hashes
//!             |
//!             v
//!     [session.load_url_states()]
//!             |
//!             v
//!     HashMap<String, UrlStateRaw>
//!             |
//!             v
//!     [classify_scraped_pages]
//!             |
//!             v
//!     ScrapePageDiff { unchanged, changed_or_new }
//!             |
//!             v
//!     [load_archived_scrape_pages]
//!             |
//!        _____|_____
//!       |           |
//!       v           v
//!  archived       changed+new
//!  pages          pages
//!       |           |
//!       v           v
//!   [merge_scrape_pages_in_order]
//!             |
//!             v
//!   (ScrapeResult, ScrapeReuseStats)
//! ```

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![forbid(unsafe_code)]
#![allow(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::uninlined_format_args)]

mod archive;
mod classify;
mod merge;
mod types;

#[cfg(test)]
mod tests_classify;
#[cfg(test)]
mod tests_errors;
#[cfg(test)]
mod tests_hash;
#[cfg(test)]
mod tests_merge;

pub use archive::load_archived_scrape_pages;
pub use classify::{classify_scraped_pages, compute_page_content_hash};
pub use merge::{merge_scrape_pages_in_order, scrape_with_reuse};
pub use types::{ScrapePageDiff, ScrapeReuseError, ScrapeReuseStats};
