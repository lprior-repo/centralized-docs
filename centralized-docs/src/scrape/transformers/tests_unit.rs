//! Unit tests for data transformation pure functions.

use super::*;

#[test]
fn test_calculate_backoff_delay() {
    assert_eq!(calculate_backoff_delay(2000, 1), 2000);
    assert_eq!(calculate_backoff_delay(2000, 2), 4000);
    assert_eq!(calculate_backoff_delay(2000, 3), 8000);
}

#[test]
fn test_extract_headers() {
    let md = "# H1\n\n## H2\n\n### H3";
    let headers = extract_headers(md);
    assert_eq!(headers.len(), 3);
    assert_eq!(headers[0].level, 1);
    assert_eq!(headers[0].text, "H1");
}

#[test]
fn test_url_to_slug_basic() {
    let result = url_to_slug("https://example.com/docs/getting-started");
    assert!(result.is_ok());
    if let Ok(slug) = result {
        assert_eq!(slug, "docs-getting-started");
    }
}

#[test]
fn test_url_to_slug_with_query_params() {
    let slug1 = url_to_slug("https://example.com/docs?page=1").unwrap();
    let slug2 = url_to_slug("https://example.com/docs?page=2").unwrap();
    assert_ne!(slug1, slug2);
    assert!(slug1.contains("-q"), "slug1: {slug1}");
    assert!(slug2.contains("-q"), "slug2: {slug2}");
}

#[test]
fn test_url_to_slug_with_fragment() {
    let slug1 = url_to_slug("https://example.com/docs#section1").unwrap();
    let slug2 = url_to_slug("https://example.com/docs#section2").unwrap();
    assert_ne!(slug1, slug2);
    assert!(
        slug1.contains("-q")
            && slug1
                .chars()
                .skip_while(|c| *c != 'q')
                .nth(1)
                .is_some_and(|c| c.is_ascii_digit())
    );
    assert!(
        slug2.contains("-q")
            && slug2
                .chars()
                .skip_while(|c| *c != 'q')
                .nth(1)
                .is_some_and(|c| c.is_ascii_digit())
    );
}

#[test]
fn test_url_to_slug_no_query_no_suffix() {
    let slug = url_to_slug("https://example.com/docs").unwrap();
    assert_eq!(slug, "docs");
    assert!(!slug.contains("-q"));
}

#[test]
fn test_url_to_slug_different_paths_different_slugs() {
    let slug1 = url_to_slug("https://example.com/docs?page=1").unwrap();
    let slug2 = url_to_slug("https://example.com/api?page=1").unwrap();
    assert_ne!(slug1, slug2);
}

#[test]
fn test_detect_rate_limit_page() {
    assert!(detect_rate_limit_page("Rate limit exceeded"));
    assert!(detect_rate_limit_page("Too many requests"));
    assert!(detect_rate_limit_page("Error 429"));
    assert!(!detect_rate_limit_page("Normal content"));
}
