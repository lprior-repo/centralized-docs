use super::*;
use crate::persisted::PersistedScrapeResult;
use crate::state::UrlStateRaw;

fn make_stored(url: &str, content_hash: [u8; 32]) -> (String, UrlStateRaw) {
    (
        url.to_string(),
        UrlStateRaw {
            content_hash,
            url_hash: [0u8; 32],
            last_fetched_secs: 0,
            status_code: 200,
            reserved: [0u8; 46],
        },
    )
}

fn make_scraped_page(url: &str, markdown: &str) -> ScrapedPage {
    ScrapedPage {
        url: url.to_string(),
        markdown: markdown.to_string(),
        title: format!("Title for {url}"),
        links: vec![],
        headers: vec![],
        word_count: markdown.split_whitespace().count(),
        slug: url.trim_start_matches("https://").replace('/', "-"),
        filter_status: crate::scrape::validation::PageFilterStatus::Unfiltered,
        elements_removed: 0,
        density_score: 0.5,
    }
}

mod build_changes_tests;
mod classify_tests;
mod hash_and_combined_tests;
mod proptests;
