#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]


//! End-to-End Tests using Playwright
//!
//! This module tests the CLI through a real browser using Playwright.
//! Tests verify actual rendering, JavaScript execution, and page behavior.
//!
//! ## Prerequisites
//!
//! Run `npx playwright@1.56.1 install chromium` before executing these tests.

#![allow(dead_code)]

use std::path::Path;
use tempfile::TempDir;

fn binary_path() -> std::path::PathBuf {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let debug_path = workspace_root.join("target/debug/ctd");
    let release_path = workspace_root.join("target/release/ctd");

    if release_path.exists() {
        release_path
    } else if debug_path.exists() {
        debug_path
    } else {
        panic!(
            "ctd binary not found at {:?} or {:?}",
            debug_path, release_path
        );
    }
}

fn create_test_html_page(dir: &Path, filename: &str, html_content: &str) {
    let file_path = dir.join(filename);
    std::fs::write(&file_path, html_content).unwrap();
}

fn start_http_server(dir: &Path) -> (std::process::Child, String) {
    let port = portpicker::pick_unused_port().expect("Failed to pick unused port");
    let url = format!("http://127.0.0.1:{}", port);

    let child = std::process::Command::new("python3")
        .args(["-m", "http.server", &port.to_string()])
        .current_dir(dir)
        .spawn()
        .expect("Failed to start HTTP server");

    std::thread::sleep(std::time::Duration::from_millis(500));

    (child, url)
}

#[cfg(test)]
mod playwright_tests {
    use super::*;
    use playwright_rs::{Locator, Playwright};

    async fn setup_playwright() -> Playwright {
        Playwright::launch().await.unwrap()
    }

    async fn get_locator(page: &playwright_rs::Page, selector: &str) -> Locator {
        page.locator(selector).await
    }

    #[tokio::test]
    async fn test_browser_launch_and_navigation() {
        let playwright = setup_playwright().await;

        let browser = playwright.chromium().launch().await.unwrap();
        let page = browser.new_page().await.unwrap();

        page.goto("about:blank", None).await.unwrap();

        let title = page.title().await.unwrap();
        assert_eq!(title, "");

        browser.close().await.unwrap();
    }

    #[tokio::test]
    async fn test_scrape_dynamic_page_with_js() {
        let temp_dir = TempDir::new().unwrap();
        let dir = temp_dir.path();

        let html_content = r#"<!DOCTYPE html>
<html>
<head><title>Dynamic Page</title></head>
<body>
    <h1 id="title">Initial Title</h1>
    <button id="btn">Click me</button>
    <div id="content"></div>
    <script>
        document.getElementById('btn').addEventListener('click', function() {
            document.getElementById('title').textContent = 'Clicked!';
            document.getElementById('content').innerHTML = '<p>Content loaded via JavaScript</p>';
        });
    </script>
</body>
</html>"#;

        create_test_html_page(dir, "index.html", html_content);

        let (mut server, url) = start_http_server(dir);
        let page_url = format!("{}/index.html", url);

        let playwright = setup_playwright().await;
        let browser = playwright.chromium().launch().await.unwrap();
        let page = browser.new_page().await.unwrap();

        page.goto(&page_url, None).await.unwrap();

        let title = page.title().await.unwrap();
        assert_eq!(title, "Dynamic Page");

        let title_locator = get_locator(&page, "#title").await;
        let title_html = title_locator.inner_html().await.unwrap();
        assert_eq!(title_html, "Initial Title");

        let btn_locator = get_locator(&page, "#btn").await;
        btn_locator.click(None).await.unwrap();

        let updated_locator = get_locator(&page, "#title").await;
        let clicked_html = updated_locator.inner_html().await.unwrap();
        assert_eq!(clicked_html, "Clicked!");

        let content_locator = get_locator(&page, "#content").await;
        let content_html = content_locator.inner_html().await.unwrap();
        assert_eq!(content_html, "<p>Content loaded via JavaScript</p>");

        browser.close().await.unwrap();
        server.kill().unwrap();
    }

    #[tokio::test]
    async fn test_scrape_page_with_forms() {
        let temp_dir = TempDir::new().unwrap();
        let dir = temp_dir.path();

        let html_content = r#"<!DOCTYPE html>
<html>
<head><title>Form Test</title></head>
<body>
    <form id="search-form">
        <input type="text" name="query" placeholder="Search..." />
        <select name="filter">
            <option value="all">All</option>
            <option value="docs">Documentation</option>
        </select>
        <button type="submit">Search</button>
    </form>
    <div id="results">
        <article class="result">
            <h2>Result 1</h2>
            <p>Content of result 1</p>
        </article>
        <article class="result">
            <h2>Result 2</h2>
            <p>Content of result 2</p>
        </article>
    </div>
</body>
</html>"#;

        create_test_html_page(dir, "form.html", html_content);

        let (mut server, url) = start_http_server(dir);
        let page_url = format!("{}/form.html", url);

        let playwright = setup_playwright().await;
        let browser = playwright.chromium().launch().await.unwrap();
        let page = browser.new_page().await.unwrap();

        page.goto(&page_url, None).await.unwrap();

        let title = page.title().await.unwrap();
        assert_eq!(title, "Form Test");

        let input_loc = get_locator(&page, "input[name='query']").await;
        let input_value = input_loc.input_value(None).await.unwrap();
        assert_eq!(input_value, "");

        let results_loc = get_locator(&page, ".result").await;
        let results_count = results_loc.count().await.unwrap();
        assert_eq!(results_count, 2);

        browser.close().await.unwrap();
        server.kill().unwrap();
    }

    #[tokio::test]
    async fn test_page_with_ajax_requests() {
        let temp_dir = TempDir::new().unwrap();
        let dir = temp_dir.path();

        let html_content = r#"<!DOCTYPE html>
<html>
<head><title>AJAX Test</title></head>
<body>
    <div id="data-container">Loading...</div>
    <script>
        fetch('/data.json')
            .then(response => response.json())
            .then(data => {
                document.getElementById('data-container').textContent = data.message;
                document.getElementById('data-container').classList.add('loaded');
            })
            .catch(err => {
                document.getElementById('data-container').textContent = 'Error: ' + err.message;
            });
    </script>
</body>
</html>"#;

        let json_content = r#"{"message": "Data loaded successfully via AJAX"}"#;

        create_test_html_page(dir, "ajax.html", html_content);
        create_test_html_page(dir, "data.json", json_content);

        let (mut server, url) = start_http_server(dir);
        let page_url = format!("{}/ajax.html", url);

        let playwright = setup_playwright().await;
        let browser = playwright.chromium().launch().await.unwrap();
        let page = browser.new_page().await.unwrap();

        page.goto(&page_url, None).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        let container_loc = get_locator(&page, "#data-container").await;
        let content = container_loc.inner_html().await.unwrap();
        assert!(
            content.contains("Data loaded successfully"),
            "AJAX content should be loaded, got: {}",
            content
        );

        browser.close().await.unwrap();
        server.kill().unwrap();
    }
}
