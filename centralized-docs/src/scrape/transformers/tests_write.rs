//! Integration tests for write_scraped_pages and slug collision handling.

use super::*;
use crate::scrape::validation;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir(prefix: &str) -> std::path::PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    std::env::temp_dir().join(format!("{prefix}-{nanos}"))
}

#[test]
fn test_write_scraped_pages_handles_slug_collisions() {
    let output_dir = unique_temp_dir("doc-transformer-slug-collision");

    let page_a = validation::ScrapedPage {
        url: "https://example.com/a.html".to_string(),
        markdown: "# A".to_string(),
        title: "A".to_string(),
        links: Vec::new(),
        headers: Vec::new(),
        word_count: 1,
        slug: "a".to_string(),
        filter_status: PageFilterStatus::Unfiltered,
        elements_removed: 0,
        density_score: 1.0,
    };

    let page_b = validation::ScrapedPage {
        url: "https://example.com/a.htm".to_string(),
        markdown: "# B".to_string(),
        title: "B".to_string(),
        links: Vec::new(),
        headers: Vec::new(),
        word_count: 1,
        slug: "a".to_string(),
        filter_status: PageFilterStatus::Unfiltered,
        elements_removed: 0,
        density_score: 1.0,
    };

    let scrape_result = validation::ScrapeResult {
        pages: vec![page_a, page_b],
        total_urls: 2,
        success_count: 2,
        error_count: 0,
        errors: Vec::new(),
        base_url: "https://example.com".to_string(),
    };

    let write_result = write_scraped_pages(&scrape_result, &output_dir);
    assert!(write_result.is_ok());

    let scrape_dir = output_dir.join(".scrape");
    assert!(scrape_dir.join("a.md").exists());
    assert!(scrape_dir.join("a-2.md").exists());

    let _ = std::fs::remove_dir_all(output_dir);
}

#[test]
fn test_write_scraped_pages_related_links_use_disambiguated_filenames() {
    let output_dir = unique_temp_dir("doc-transformer-related-collision");

    let page_a = validation::ScrapedPage {
        url: "https://example.com/a.html".to_string(),
        markdown: "# A\n\ncontent".to_string(),
        title: "A".to_string(),
        links: vec!["https://example.com/shared".to_string()],
        headers: Vec::new(),
        word_count: 2,
        slug: "a".to_string(),
        filter_status: PageFilterStatus::Unfiltered,
        elements_removed: 0,
        density_score: 1.0,
    };

    let page_b = validation::ScrapedPage {
        url: "https://example.com/a.htm".to_string(),
        markdown: "# B\n\ncontent".to_string(),
        title: "B".to_string(),
        links: vec!["https://example.com/shared".to_string()],
        headers: Vec::new(),
        word_count: 2,
        slug: "a".to_string(),
        filter_status: PageFilterStatus::Unfiltered,
        elements_removed: 0,
        density_score: 1.0,
    };

    let scrape_result = validation::ScrapeResult {
        pages: vec![page_a, page_b],
        total_urls: 2,
        success_count: 2,
        error_count: 0,
        errors: Vec::new(),
        base_url: "https://example.com".to_string(),
    };

    let write_result = write_scraped_pages(&scrape_result, &output_dir);
    assert!(write_result.is_ok());

    let scrape_dir = output_dir.join(".scrape");
    let content_a = std::fs::read_to_string(scrape_dir.join("a.md"));
    let content_b = std::fs::read_to_string(scrape_dir.join("a-2.md"));

    assert!(content_a.is_ok());
    assert!(content_b.is_ok());

    if let Ok(text) = content_a {
        assert!(text.contains("[B](a-2.md)"));
    }

    if let Ok(text) = content_b {
        assert!(text.contains("[A](a.md)"));
    }

    let _ = std::fs::remove_dir_all(output_dir);
}

#[test]
fn test_slug_collision_prevents_data_loss() {
    let output_dir = unique_temp_dir("doc-transformer-collision-test");

    let page1 = validation::ScrapedPage {
        url: "https://example.com/docs/page".to_string(),
        markdown: "# Page 1\n\nFirst page content".to_string(),
        title: "Page 1".to_string(),
        links: Vec::new(),
        headers: vec![validation::Header {
            level: 1,
            text: "Page 1".to_string(),
        }],
        word_count: 4,
        slug: "docs-page".to_string(),
        filter_status: PageFilterStatus::Unfiltered,
        elements_removed: 0,
        density_score: 1.0,
    };

    let page2 = validation::ScrapedPage {
        url: "https://example.com/docs/page.html".to_string(),
        markdown: "# Page 2\n\nSecond page content".to_string(),
        title: "Page 2".to_string(),
        links: Vec::new(),
        headers: vec![validation::Header {
            level: 1,
            text: "Page 2".to_string(),
        }],
        word_count: 4,
        slug: "docs-page".to_string(),
        filter_status: PageFilterStatus::Unfiltered,
        elements_removed: 0,
        density_score: 1.0,
    };

    let page3 = validation::ScrapedPage {
        url: "https://example.com/docs/page.htm".to_string(),
        markdown: "# Page 3\n\nThird page content".to_string(),
        title: "Page 3".to_string(),
        links: Vec::new(),
        headers: vec![validation::Header {
            level: 1,
            text: "Page 3".to_string(),
        }],
        word_count: 4,
        slug: "docs-page".to_string(),
        filter_status: PageFilterStatus::Unfiltered,
        elements_removed: 0,
        density_score: 1.0,
    };

    let scrape_result = validation::ScrapeResult {
        pages: vec![page1, page2, page3],
        total_urls: 3,
        success_count: 3,
        error_count: 0,
        errors: Vec::new(),
        base_url: "https://example.com".to_string(),
    };

    let write_result = write_scraped_pages(&scrape_result, &output_dir);
    assert!(
        write_result.is_ok(),
        "Should handle slug collisions without error"
    );

    let scrape_dir = output_dir.join(".scrape");

    assert!(scrape_dir.join("docs-page.md").exists());
    assert!(scrape_dir.join("docs-page-2.md").exists());
    assert!(scrape_dir.join("docs-page-3.md").exists());

    let content1 = std::fs::read_to_string(scrape_dir.join("docs-page.md"));
    let content2 = std::fs::read_to_string(scrape_dir.join("docs-page-2.md"));
    let content3 = std::fs::read_to_string(scrape_dir.join("docs-page-3.md"));

    assert!(content1.is_ok());
    assert!(content2.is_ok());
    assert!(content3.is_ok());

    assert!(content1.as_ref().is_ok_and(|c| c.contains("Page 1")));
    assert!(content2.as_ref().is_ok_and(|c| c.contains("Page 2")));
    assert!(content3.as_ref().is_ok_and(|c| c.contains("Page 3")));

    let _ = std::fs::remove_dir_all(output_dir);
}

#[test]
fn test_query_param_collision_handling() {
    let output_dir = unique_temp_dir("doc-transformer-query-collision");

    let page1 = validation::ScrapedPage {
        url: "https://example.com/api/users?id=1".to_string(),
        markdown: "# User 1\n\nFirst user".to_string(),
        title: "User 1".to_string(),
        links: Vec::new(),
        headers: Vec::new(),
        word_count: 3,
        slug: "api-users".to_string(),
        filter_status: PageFilterStatus::Unfiltered,
        elements_removed: 0,
        density_score: 1.0,
    };

    let page2 = validation::ScrapedPage {
        url: "https://example.com/api/users?id=2".to_string(),
        markdown: "# User 2\n\nSecond user".to_string(),
        title: "User 2".to_string(),
        links: Vec::new(),
        headers: Vec::new(),
        word_count: 3,
        slug: "api-users".to_string(),
        filter_status: PageFilterStatus::Unfiltered,
        elements_removed: 0,
        density_score: 1.0,
    };

    let scrape_result = validation::ScrapeResult {
        pages: vec![page1, page2],
        total_urls: 2,
        success_count: 2,
        error_count: 0,
        errors: Vec::new(),
        base_url: "https://example.com".to_string(),
    };

    let write_result = write_scraped_pages(&scrape_result, &output_dir);
    assert!(write_result.is_ok());

    let scrape_dir = output_dir.join(".scrape");
    assert!(scrape_dir.join("api-users.md").exists());
    assert!(scrape_dir.join("api-users-2.md").exists());

    let _ = std::fs::remove_dir_all(output_dir);
}
