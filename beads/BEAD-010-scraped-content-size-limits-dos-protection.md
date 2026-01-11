# BEAD-010: Add Size Limits to Scraped Content (DoS Protection)

**Status**: CLOSED - IMPLEMENTED & TESTED
**Priority**: P1 (Security/DoS)
**Component**: `doc_transformer/src/scrape.rs`
**Date Created**: 2026-01-11
**Date Closed**: 2026-01-11

---

## Summary

Implemented comprehensive DoS protection for the web scraping module by adding configurable size limits that prevent:
- Memory exhaustion from huge individual pages (10GB+ HTML files)
- Streaming attacks (thousands of medium-sized pages)
- Markdown conversion bloat
- Link extraction memory overload
- Page flood attacks

---

## Problem Statement

The scraping module had no size limits on scraped content, making it vulnerable to:

1. **Huge File Attacks**: A malicious or broken server could serve a 10GB HTML file, causing memory exhaustion
2. **Streaming Attacks**: A site could serve 1000+ pages of 10MB each, hitting 10GB+ total
3. **Markdown Bloat**: HTML-to-Markdown conversion can expand content significantly (1MB HTML → 5MB Markdown)
4. **Link Extraction DOS**: A page with 100,000+ links would cause memory issues and slow processing
5. **Page Flood**: Crawling a site with millions of pages would never complete

## Contract (Specification)

### Size Limit Configuration

```rust
pub struct ScrapeConfig {
    pub max_page_size_bytes: u64,       // Default: 10MB (prevents huge file attacks)
    pub max_total_size_bytes: u64,      // Default: 500MB (prevents streaming attacks)
    pub max_markdown_size_bytes: u64,   // Default: 5MB (memory exhaustion protection)
    pub max_pages: usize,               // Default: 10,000 pages (page flood prevention)
    pub max_links_per_page: usize,      // Default: 1,000 links (memory protection)
}
```

### Helper Functions

```rust
fn check_html_size(html: &str, max_size: u64) -> Result<()>
fn check_markdown_size(markdown: &str, max_size: u64) -> Result<()>
fn limit_links_per_page(links: Vec<String>, max_links: usize) -> (Vec<String>, bool)
```

### Integration Points

**In scrape_site():**
- Track `total_content_size` across all pages using `saturating_add()`
- Stop scrape gracefully when `max_total_size_bytes` is exceeded
- Stop scrape when `max_pages` is reached
- All errors are collected and returned

**In transform_page():**
- Check HTML size immediately after fetching (line 279)
- Check markdown size after conversion (line 330)
- Enforce link limit with warning on truncation (line 339-345)

---

## Implementation

### Configuration Defaults

```rust
impl Default for ScrapeConfig {
    fn default() -> Self {
        Self {
            // ... existing fields ...
            max_page_size_bytes: 10 * 1024 * 1024,        // 10MB
            max_total_size_bytes: 500 * 1024 * 1024,      // 500MB
            max_markdown_size_bytes: 5 * 1024 * 1024,     // 5MB
            max_pages: 10_000,
            max_links_per_page: 1_000,
        }
    }
}
```

### Attack Vectors Mitigated

| Attack | Mechanism | Limit | Protection |
|--------|-----------|-------|------------|
| Single huge file | check_html_size() | 10MB | Prevents 10GB+ pages |
| Streaming attack | total_content_size tracking | 500MB | ~100 × 5MB pages max |
| Markdown bloat | check_markdown_size() | 5MB | Limits conversion bloat |
| Link DOS | limit_links_per_page() | 1,000 | Prevents 100K link pages |
| Page flood | max_pages limit | 10,000 | Prevents infinite crawls |

### Error Handling (Graceful Degradation)

- Individual page failures: Collected in errors, scrape continues
- Total size exceeded: Graceful stop with error message
- Page count exceeded: Graceful stop with error message
- Link truncation: Logged as warning, non-fatal

---

## Tests Added

10 comprehensive test cases:

1. `test_check_html_size_valid` - Small HTML passes
2. `test_check_html_size_exceeds_limit` - Oversized HTML rejected
3. `test_check_markdown_size_valid` - Small markdown passes
4. `test_check_markdown_size_exceeds_limit` - Oversized markdown rejected
5. `test_limit_links_per_page_within_limit` - Links under limit kept
6. `test_limit_links_per_page_exceeds_limit` - Links truncated correctly
7. `test_limit_links_per_page_exactly_at_limit` - Edge case: exact limit
8. `test_limit_links_per_page_empty` - Edge case: empty list
9. `test_scrape_config_default_has_size_limits` - Verify default values
10. `test_scrape_config_limits_are_reasonable` - Verify limit constraints
11. `test_huge_content_detection` - 100MB file rejected
12. `test_streaming_attack_protection` - ~100 × 5MB pages hit 500MB limit

### Test Coverage

- ✓ HTML size validation (valid/exceeds)
- ✓ Markdown size validation (valid/exceeds)
- ✓ Link limiting (within/exceeds/exact/empty)
- ✓ Config defaults and reasonableness
- ✓ Huge content detection (100MB)
- ✓ Streaming attack prevention

---

## Backward Compatibility

- ✓ All new fields have sensible defaults in `ScrapeConfig::default()`
- ✓ Existing code continues to work without changes
- ✓ Limits are configurable for advanced use cases
- ✓ No breaking changes to public API

---

## Safety & Correctness

- ✓ Uses `saturating_add()` for overflow prevention
- ✓ All size checks use `>` comparison (no off-by-one)
- ✓ Proper error types (`Result<T>`) for all size violations
- ✓ No unwraps or panics in size checking code
- ✓ Complies with project's `#![deny(clippy::unwrap_used)]`

---

## Related Beads

- BEAD-004: Unbounded regex input (complementary)
- BEAD-008: Search query length limits (similar approach)

---

## Future Enhancements

1. Make limits configurable via CLI `--max-page-size` argument
2. Per-domain limits (e.g., reddit.com ≠ docs.rs)
3. Adaptive limits based on available memory
4. Time-based limits (abort if scrape > 30 minutes)
5. Bandwidth rate limiting (bytes/second cap)
6. Progress reporting (current size / max size)

---

## Verification Checklist

- ✓ Size limits added to ScrapeConfig
- ✓ Helper functions implemented (3)
- ✓ Integration in scrape_site() - tracks total size
- ✓ Integration in transform_page() - checks HTML and markdown
- ✓ Link limiting enforced with warnings
- ✓ 12 tests added and passing
- ✓ Graceful error handling implemented
- ✓ Safe arithmetic (saturating_add)
- ✓ Backward compatible (all defaults set)
- ✓ No breaking changes to API

---

## Code Locations

### Configuration
- Lines 65-74: Size limit fields added to ScrapeConfig
- Lines 89-93: Default values in Default impl

### Helper Functions
- Lines 491-500: `check_html_size()`
- Lines 506-520: `check_markdown_size()`
- Lines 521-531: `limit_links_per_page()`

### Integration
- Lines 186: `total_content_size` tracking variable
- Lines 198-205: Max pages check
- Lines 229-241: Total size tracking and limit check
- Lines 279: HTML size check
- Lines 330: Markdown size check
- Lines 339-345: Link limiting with warning

### Tests
- Lines 949-1075: All 12 test functions

---

## Deployment Notes

- No configuration migration needed (all defaults provided)
- Limits can be overridden in code if needed
- Future work: expose via CLI arguments
- Future work: expose via config file

---

## Summary

Successfully implemented comprehensive DoS protection for the scraping module with 5 configurable size limits, 3 helper functions, 12 tests, and graceful error handling. All limits have sensible defaults that protect against known attack vectors while allowing legitimate documentation site scraping (typical doc site: 100-500 pages, 50-500MB total).

