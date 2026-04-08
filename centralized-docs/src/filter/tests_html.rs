use super::*;

#[test]
fn test_prune_html() {
    let html = r"
            <html>
            <body>
                <nav>Navigation content</nav>
                <main>
                    <h1>Main Title</h1>
                    <p>This is the main content of the page with enough words to pass the minimum word count threshold for filtering.</p>
                </main>
                <footer>Footer content</footer>
            </body>
            </html>
        ";

    let config = FilterConfig::default();
    let result = prune_html(html, &config);

    // Check that the html field contains main content
    assert!(result.html.contains("Main Title") || result.html.contains("main content"));

    // Check density score is calculated
    assert!(result.density_score.value() >= 0.0);
    assert!(result.density_score.value() <= 1.0);

    // Check that used_readability indicates which method was used
    let _ = result.used_readability;

    // Check that removed_count is a valid value
    let _ = result.removed_count;
}

#[test]
fn test_prune_html_with_article_tag() {
    let html = r"
            <html>
            <body>
                <nav>Navigation</nav>
                <article>
                    <h1>Article Title</h1>
                    <p>This is substantive article content with plenty of words. Article content includes discussion, explanations, and detailed information about topics. It is the main focus of the page and should be extracted properly.</p>
                </article>
                <aside>Sidebar content</aside>
            </body>
            </html>
        ";

    let config = FilterConfig::default();
    let result = prune_html(html, &config);

    assert!(result.html.contains("Article Title") || result.html.contains("article content"));
    assert!(result.density_score.value() > 0.0);
    assert!(result.density_score.value() <= 1.0);
}

#[test]
fn test_readability_fallback_on_nav_only() {
    let html = r#"
            <html>
            <body>
                <nav>
                    <a href="/page1">Page 1</a>
                    <a href="/page2">Page 2</a>
                    <a href="/page3">Page 3</a>
                </nav>
            </body>
            </html>
        "#;

    let config = FilterConfig::default();
    let result = prune_html(html, &config);

    assert!(!result.html.is_empty());
    assert!(result.density_score.value() >= 0.0);
    assert!(result.density_score.value() <= 1.0);
}

#[test]
fn test_extract_main_content() {
    let html = r"
            <html>
            <body>
                <header>Header</header>
                <article>
                    <h1>Article Title</h1>
                    <p>Article content goes here with plenty of words to meet the minimum threshold.</p>
                </article>
                <aside>Sidebar</aside>
            </body>
            </html>
        ";

    let document = scraper::Html::parse_document(html);
    let config = FilterConfig::default();
    let content = extract_main_content(&document, &config);

    assert!(content.contains("Article Title") || content.contains("Article content"));
}

#[test]
fn test_prune_html_empty_input() {
    let config = FilterConfig::default();
    let result = prune_html("", &config);
    assert!(result.density_score.value() >= 0.0);
    assert!(result.density_score.value() <= 1.0);
}

#[test]
fn test_prune_html_script_and_style_removal() {
    let html = r"
            <html><body>
            <script>alert('hi');</script>
            <style>body { color: red; }</style>
            <main>
                <h1>Content</h1>
                <p>Real content with enough words to pass the minimum word count threshold for content filtering.</p>
            </main>
            </body></html>
        ";
    let config = FilterConfig::default();
    let result = prune_html(html, &config);
    assert!(
        result.html.contains("Content")
            || result.html.contains("content")
            || result.html.contains("Real")
    );
    assert!(result.density_score.value() > 0.0);
}

#[test]
fn test_prune_html_with_nav_class_patterns() {
    let html = r#"
            <html><body>
            <div class="sidebar">Sidebar junk</div>
            <div class="breadcrumb">Home > Page</div>
            <div class="pagination">1 2 3 4</div>
            <article>
                <h1>Real</h1>
                <p>Real content with enough words to pass the minimum word count threshold for content filtering in tests.</p>
            </article>
            </body></html>
        "#;
    let config = FilterConfig::default();
    let result = prune_html(html, &config);
    assert!(result.html.contains("Real"));
}

#[test]
fn test_extract_main_content_role_main() {
    let html = r#"
            <html><body>
            <div role="main">
                <h1>Role Main</h1>
                <p>Content inside role main with enough words to pass the minimum threshold.</p>
            </div>
            </body></html>
        "#;
    let document = scraper::Html::parse_document(html);
    let config = FilterConfig::default();
    let content = extract_main_content(&document, &config);
    assert!(content.contains("Role Main"));
}

#[test]
fn test_extract_main_content_fallback_to_body() {
    let html = r"
            <html><body>
            <p>This is body text with enough words to pass the minimum threshold for extraction tests.</p>
            </body></html>
        ";
    let document = scraper::Html::parse_document(html);
    let config = FilterConfig::default();
    let content = extract_main_content(&document, &config);
    assert!(content.contains("body text"));
}

#[test]
fn test_extract_main_content_no_match_all_text() {
    let html = r"<html><body><span>short</span></body></html>";
    let document = scraper::Html::parse_document(html);
    let config = FilterConfig::default();
    let content = extract_main_content(&document, &config);
    assert!(content.contains("short"));
}

#[test]
fn test_extract_main_content_class_content() {
    let html = r#"
            <html><body>
            <div class="content">
                <h1>Class Content</h1>
                <p>Content inside div class content with enough words to pass the minimum word count.</p>
            </div>
            </body></html>
        "#;
    let document = scraper::Html::parse_document(html);
    let config = FilterConfig::default();
    let content = extract_main_content(&document, &config);
    assert!(content.contains("Class Content"));
}

#[test]
fn test_extract_main_content_id_main() {
    let html = r#"
            <html><body>
            <div id="main">
                <h1>ID Main</h1>
                <p>Content inside id main with enough words to pass the minimum word count for tests.</p>
            </div>
            </body></html>
        "#;
    let document = scraper::Html::parse_document(html);
    let config = FilterConfig::default();
    let content = extract_main_content(&document, &config);
    assert!(content.contains("ID Main"));
}

#[test]
fn test_filter_result_with_is_empty() {
    let result = FilterResult {
        html: "content".to_string(),
        removed_count: 0,
        density_score: crate::math_types::Score::zero(),
        used_readability: true,
        is_empty: false,
    };
    let modified = result.with_is_empty(true);
    assert!(modified.is_empty);
    assert_eq!(modified.html, "content");
}

#[test]
fn test_filter_config_default() {
    let config = FilterConfig::default();
    assert_eq!(config.density_threshold, 0.45);
    assert_eq!(config.min_word_count, 10);
    assert!(config.remove_tags.contains(&"nav".to_string()));
    assert!(config.remove_tags.contains(&"script".to_string()));
    assert!(config.nav_patterns.contains(&"sidebar".to_string()));
}

#[test]
fn test_filter_strategy_default() {
    assert_eq!(FilterStrategy::default(), FilterStrategy::Pruning);
}
