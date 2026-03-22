use std::time::Duration;

use doc_transformer::scrape::{build_website_base, ScrapeConfig};
use spider::configuration::RedirectPolicy;

#[test]
fn request_timeout_is_applied_to_website_configuration() {
    let config = ScrapeConfig {
        base_url: "http://127.0.0.1:7878".to_string(),
        request_timeout_secs: 1,
        ..Default::default()
    };

    let url = doc_transformer::scrape::http::ValidatedUrl::try_new(&config.base_url).unwrap();
    let website = build_website_base(&url, &config).unwrap();

    assert_eq!(
        website.configuration.request_timeout.as_deref(),
        Some(&Duration::from_secs(1))
    );
}

#[test]
fn redirect_policy_is_applied_to_website_configuration() {
    let blocked_config = ScrapeConfig {
        base_url: "http://127.0.0.1:7878".to_string(),
        redirect_policy: RedirectPolicy::None,
        ..Default::default()
    };

    let url1 =
        doc_transformer::scrape::http::ValidatedUrl::try_new(&blocked_config.base_url).unwrap();
    let blocked_website = build_website_base(&url1, &blocked_config).unwrap();
    assert_eq!(
        blocked_website.configuration.redirect_policy,
        RedirectPolicy::None
    );

    let allowed_config = ScrapeConfig {
        base_url: "http://127.0.0.1:7878".to_string(),
        redirect_policy: RedirectPolicy::Loose,
        ..Default::default()
    };

    let url2 =
        doc_transformer::scrape::http::ValidatedUrl::try_new(&allowed_config.base_url).unwrap();
    let allowed_website = build_website_base(&url2, &allowed_config).unwrap();
    assert_eq!(
        allowed_website.configuration.redirect_policy,
        RedirectPolicy::Loose
    );
}

#[test]
fn spider_byte_caps_are_applied_to_website_configuration() {
    let config = ScrapeConfig {
        base_url: "http://127.0.0.1:7878".to_string(),
        spider_max_page_bytes: Some(64),
        spider_max_total_bytes: Some(1024),
        ..Default::default()
    };

    let url = doc_transformer::scrape::http::ValidatedUrl::try_new(&config.base_url).unwrap();
    let website = build_website_base(&url, &config).unwrap();

    assert_eq!(website.configuration.max_page_bytes, Some(64.0));
    assert_eq!(website.configuration.max_bytes_allowed, Some(1024));
}
