#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::complexity)]
#![warn(clippy::cognitive_complexity)]
#![forbid(unsafe_code)]
#![allow(clippy::print_stdout)]

use axum::{extract::Path, response::Html, routing::get, Router};
use std::fmt::Write;
use std::sync::LazyLock;
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

// Load the heavy template at compile time or startup
static HEAVY_TEMPLATE: LazyLock<String> = LazyLock::new(|| {
    let content = std::fs::read_to_string("heavy_template.html")
        .unwrap_or_else(|_| "<html><body><h1>Missing template</h1></body></html>".to_string());

    // Remove existing links so the spider doesn't crawl out to the real internet
    content
        .replace("href=\"", "data-href=\"")
        .replace("href='", "data-href='")
});

// Split point where we inject our dynamic navigation
static TEMPLATE_PARTS: LazyLock<(String, String)> = LazyLock::new(|| {
    if let Some(idx) = HEAVY_TEMPLATE.find("<body") {
        if let Some(end_idx) = HEAVY_TEMPLATE.get(idx..).and_then(|s| s.find('>')) {
            let split_point = idx + end_idx + 1;
            return (
                HEAVY_TEMPLATE
                    .get(..split_point)
                    .map_or_else(String::new, ToString::to_string),
                HEAVY_TEMPLATE
                    .get(split_point..)
                    .map_or_else(String::new, ToString::to_string),
            );
        }
    }
    (String::new(), (*HEAVY_TEMPLATE).clone())
});

async fn handle_root() -> Html<String> {
    generate_page(0).await
}

async fn handle_page(Path(path): Path<String>) -> Html<String> {
    // path will be something like "1.html"
    let page_num = path
        .strip_suffix(".html")
        .unwrap_or(&path)
        .parse::<usize>()
        .unwrap_or(0);
    generate_page(page_num).await
}

const MAX_PAGES: usize = 10_000;

async fn generate_page(page_num: usize) -> Html<String> {
    // 1. Simulate real-world network latency / DB lookup (Time To First Byte)
    // 50ms is a very realistic average for a fast web server under load
    sleep(Duration::from_millis(50)).await;

    // 2. Generate the dynamic navigation
    // Each page links to the next 50 pages to ensure massive discovery tree
    // We cap the site at 10,000 pages (approx 6 GB of raw HTML data to scrape)
    let mut links = String::with_capacity(4096);
    links.push_str("<div id=\"mock-navigation\">\n<h2>Site Navigation</h2>\n<ul>\n");

    if page_num < MAX_PAGES {
        for i in 1..=50 {
            let next_page = page_num + i;
            if next_page <= MAX_PAGES {
                let _ = writeln!(
                    links,
                    "<li><a href=\"/page_{next_page}.html\">Heavy Page {next_page}</a></li>"
                );
            }
        }
    }
    links.push_str("</ul>\n</div>\n");

    // 3. Assemble the massive HTML string (approx 625 KB per page)
    let header = format!("<h1>Realistic Load Page {page_num}</h1>\n");

    let full_html = if TEMPLATE_PARTS.0.is_empty() {
        format!(
            "<html><body>{header}{links}{body}</body></html>",
            body = TEMPLATE_PARTS.1
        )
    } else {
        format!("{}{header}{links}{}", TEMPLATE_PARTS.0, TEMPLATE_PARTS.1)
    };

    Html(full_html)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ensure template is loaded
    LazyLock::force(&TEMPLATE_PARTS);
    info!("Loaded template ({} bytes)", HEAVY_TEMPLATE.len());

    // Build our application with a route
    let app = Router::new()
        .route("/", get(handle_root))
        .route("/page_:path", get(handle_page));

    let port = 8081;
    let addr = format!("0.0.0.0:{port}");
    info!("Starting heavy simulated Rust server on {addr}");
    info!("- 50ms simulated latency per request");
    info!("- ~625 KB payload per request");
    info!("- 10_000 total pages");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
