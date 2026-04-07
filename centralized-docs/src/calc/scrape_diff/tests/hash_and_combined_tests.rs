use super::*;

#[test]
fn hash_content_returns_deterministic_sha256() {
    let h1 = hash_content(b"hello world");
    let h2 = hash_content(b"hello world");
    assert_eq!(h1, h2);
}

#[test]
fn hash_content_returns_non_zero_for_non_empty_input() {
    let h = hash_content(b"test");
    assert_ne!(h, [0u8; 32]);
}

#[test]
fn hash_content_sha256_empty_bytes_is_known_value() {
    let h = hash_content(b"");
    assert_eq!(
        h,
        [
            0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f,
            0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b,
            0x78, 0x52, 0xb8, 0x55,
        ]
    );
}

#[test]
fn build_combined_scrape_result_merges_reused_and_fresh_pages() {
    let reused = vec![make_scraped_page("https://a.com/p1", "reused content")];
    let fresh = vec![make_scraped_page("https://a.com/p2", "fresh content")];
    let result = build_combined_scrape_result(reused, fresh, "https://a.com");
    assert_eq!(result.pages.len(), 2);
    assert_eq!(result.success_count, 2);
    assert_eq!(result.base_url, "https://a.com");
}

#[test]
fn build_combined_scrape_result_with_empty_inputs() {
    let result = build_combined_scrape_result(vec![], vec![], "https://a.com");
    assert_eq!(result.pages.len(), 0);
    assert_eq!(result.success_count, 0);
    assert_eq!(result.error_count, 0);
}
