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

    let website = build_website_base(&config.base_url, &config);

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

    let blocked_website = build_website_base(&blocked_config.base_url, &blocked_config);
    assert_eq!(
        blocked_website.configuration.redirect_policy,
        RedirectPolicy::None
    );

    let allowed_config = ScrapeConfig {
        base_url: "http://127.0.0.1:7878".to_string(),
        redirect_policy: RedirectPolicy::Loose,
        ..Default::default()
    };

    let allowed_website = build_website_base(&allowed_config.base_url, &allowed_config);
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

    let website = build_website_base(&config.base_url, &config);

    assert_eq!(website.configuration.max_page_bytes, Some(64.0));
    assert_eq!(website.configuration.max_bytes_allowed, Some(1024));
}
