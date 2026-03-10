# BEAD-016: God Object - scrape.rs at 1,248 Lines

**Epic**: Code Quality
**Severity**: Medium
**Status**: Open

---

## CONTEXT BLOCK

- **File/Function**: `doc_transformer/src/scrape.rs` (entire module, 1,248 lines)
- **The Smell**: Single file contains HTTP client, HTML parsing, markdown conversion, content filtering, URL validation, sitemap processing, and error handling. It's a god object that mixes multiple concerns and is impossible to navigate or test effectively.

**Evidence**:
```bash
$ wc -l doc_transformer/src/scrape.rs
1248 doc_transformer/src/scrape.rs  # Single file!

$ grep -n "^pub fn\|^pub async fn\|^fn" doc_transformer/src/scrape.rs | head -20
4:pub async fn scrape_site(config: &ScrapeConfig) -> Result<()>
18:fn get_sitemap(...)
47:fn crawl_site(...)
98:fn fetch_page(...)
123:fn parse_html(...)
156:fn convert_to_markdown(...)
203:fn filter_by_bm25(...)
245:fn extract_content(...)
289:fn validate_url(...)
... (continues for 1,248 lines)
```

**Functionality Mixed in One File**:
- HTTP client (`fetch_page`, `crawl_site`)
- HTML parsing (`parse_html`, `extract_content`)
- Markdown conversion (`convert_to_markdown`)
- Content filtering (`filter_by_bm25`)
- URL validation (`validate_url`)
- Sitemap processing (`get_sitemap`)
- Data transformation (multiple small helpers)
- Error handling throughout

**User Impact**:
- Impossible to find where a bug is located (1,248 lines to search)
- Hard to test individual concerns in isolation
- Refactoring one feature risks breaking unrelated code
- New contributors overwhelmed by file size
- Git changes affect unrelated functionality
- No clear separation between layers

---

## SPECIFICATION BLOCK

### 1. EARS (Easy Approach to Requirements Syntax)

| Trigger | System | Response |
|---------|--------|----------|
| File exceeds 500 lines | Code review | Refactor into smaller modules |
| Multiple concerns in one file | Architecture | Split by responsibility |
| Adding HTTP-related feature | Module structure | Add to `http.rs` module |
| Adding HTML parsing feature | Module structure | Add to `html_parser.rs` module |

### 2. DbC (Design by Contract)

**Preconditions**:
- Current functionality must be preserved
- Public API unchanged
- All tests pass after refactoring

**Postconditions**:
- Each module < 500 lines
- Single responsibility per module
- Clear module boundaries
- Easy to locate and test specific functionality

**Invariants**:
- No module exceeds 500 lines
- Each module has single responsibility
- Public API remains backward compatible

### 3. Schema & Edge Cases

**Proposed Module Structure**:
```
doc_transformer/src/scrape/
├── mod.rs              # Public API, orchestration (50 lines)
├── http.rs             # HTTP client, sitemap fetching (150 lines)
├── html_parser.rs      # HTML parsing, content extraction (200 lines)
├── markdown.rs         # HTML → Markdown conversion (180 lines)
├── filtering.rs        # BM25 filtering, content pruning (180 lines)
├── validation.rs       # URL validation, scheme checking (120 lines)
└── transformers.rs     # Data transformation helpers (150 lines)
```

**Module Responsibilities**:

| Module | Responsibility | Public API |
|--------|---------------|------------|
| `http.rs` | HTTP requests, sitemap fetching | `fetch_page()`, `get_sitemap()` |
| `html_parser.rs` | Parse HTML, extract main content | `parse_html()`, `extract_content()` |
| `markdown.rs` | HTML to Markdown conversion | `convert_to_markdown()` |
| `filtering.rs` | BM25 scoring, relevance filtering | `filter_by_relevance()` |
| `validation.rs` | URL validation, security checks | `validate_url()`, `is_safe_url()` |
| `transformers.rs` | Text transformations, cleanup | `clean_text()`, `normalize_whitespace()` |

**Example Refactoring - `http.rs`**:
```rust
// Before: buried in scrape.rs at line 98
async fn fetch_page(url: &str, delay: u64) -> Result<String> { /* 80 lines */ }

// After: dedicated module
// scrape/http.rs
pub async fn fetch_page(url: &Url, config: &HttpClientConfig) -> Result<Response> {
    // HTTP client logic, timeout, retries
}

pub async fn get_sitemap(base_url: &Url) -> Result<Vec<Url>> {
    // Sitemap fetching logic
}
```

**Example Refactoring - `html_parser.rs`**:
```rust
// Before: buried in scrape.rs at line 123
fn parse_html(html: &str) -> Document { /* 40 lines */ }

// After: dedicated module
// scrape/html_parser.rs
pub use scraper::Html;

pub fn parse_html(html: &str) -> Html {
    scraper::Html::parse_document(html)
}

pub fn extract_main_content(document: &Html) -> Result<String> {
    // Content extraction logic
}
```

---

## FIX LOCATIONS

1. **Create `doc_transformer/src/scrape/` directory**
   - New module structure

2. **`scrape/mod.rs`** - Orchestration (50 lines)
   - Re-export public APIs from submodules
   - Main `scrape_site()` function using submodules
   - Integration testing entry point

3. **`scrape/http.rs`** - HTTP client (150 lines)
   - Move `fetch_page()` from line 98
   - Move `get_sitemap()` from line 18
   - Move `crawl_site()` from line 47
   - Add `HttpClientConfig` struct

4. **`scrape/html_parser.rs`** - HTML parsing (200 lines)
   - Move `parse_html()` from line 123
   - Move `extract_content()` from line 245
   - Add selectors for main content extraction

5. **`scrape/markdown.rs`** - Conversion (180 lines)
   - Move `convert_to_markdown()` from line 156
   - Add markdown cleanup logic

6. **`scrape/filtering.rs`** - Content filtering (180 lines)
   - Move `filter_by_bm25()` from line 203
   - Add relevance scoring helpers

7. **`scrape/validation.rs`** - URL validation (120 lines)
   - Move `validate_url()` from line 289
   - Add security checks (scheme validation)

8. **`scrape/transformers.rs`** - Data transformations (150 lines)
   - Move text cleanup helpers
   - Add normalization functions

9. **`doc_transformer/src/main.rs`** - Update imports
   - `use scrape::scrape_site;` (unchanged, re-exported)

---

## TEST CASES

```rust
// Test each module independently

#[test]
fn test_http_fetch_page() {
    use scrape::http::fetch_page;
    let url = Url::parse("https://example.com").unwrap();
    let config = HttpClientConfig::default();

    let result = tokio_test::block_on(fetch_page(&url, &config));
    assert!(result.is_ok());
}

#[test]
fn test_html_parser_extraction() {
    use scrape::html_parser::{parse_html, extract_main_content};

    let html = "<html><body><main>Content here</main></body></html>";
    let doc = parse_html(html);
    let content = extract_main_content(&doc);

    assert!(content.unwrap().contains("Content here"));
}

#[test]
fn test_markdown_conversion() {
    use scrape::markdown::convert_to_markdown;

    let html = "<h1>Title</h1><p>Paragraph</p>";
    let markdown = convert_to_markdown(html);

    assert_eq!(markdown, "# Title\n\nParagraph\n");
}

#[test]
fn test_url_validation() {
    use scrape::validation::{validate_url, is_safe_url};

    assert!(validate_url("https://example.com").is_ok());
    assert!(is_safe_url("https://example.com"));
    assert!(!is_safe_url("file:///etc/passwd"));
    assert!(!is_safe_url("javascript:alert(1)"));
}

#[test]
fn test_module_line_counts() {
    let modules = vec![
        "scrape/mod.rs",
        "scrape/http.rs",
        "scrape/html_parser.rs",
        "scrape/markdown.rs",
        "scrape/filtering.rs",
        "scrape/validation.rs",
        "scrape/transformers.rs",
    ];

    for module in modules {
        let path = format!("doc_transformer/src/{}", module);
        let content = std::fs::read_to_string(path).unwrap();
        let lines = content.lines().count();

        assert!(
            lines < 500,
            "{} has {} lines (max 500 allowed)",
            module,
            lines
        );
    }
}
```

---

## VERIFICATION

After refactoring:
```bash
$ find doc_transformer/src/scrape -name "*.rs" -exec wc -l {} +
50 scrape/mod.rs
150 scrape/http.rs
200 scrape/html_parser.rs
180 scrape/markdown.rs
180 scrape/filtering.rs
120 scrape/validation.rs
150 scrape/transformers.rs
# Total: 1,030 lines (vs 1,248 in one file)

$ cargo test scrape::
# test_http_fetch_page ... ok
# test_html_parser_extraction ... ok
# test_markdown_conversion ... ok
# test_url_validation ... ok
# test_result: ok. passed. 28/28 tests

# Public API unchanged
$ cargo build
# doc_transformer binary builds successfully

# Easy to find specific functionality
$ grep -r "fetch_page" doc_transformer/src/scrape/
# scrape/http.rs:pub async fn fetch_page() { ... }
```

---

## RECOMMENDATION

Split `scrape.rs` into the 7-module structure above. This will:
- Make the codebase navigable
- Enable independent testing of each concern
- Reduce merge conflicts
- Make onboarding easier for new contributors
- Maintain backward compatibility (public API unchanged)
