#![allow(clippy::print_stdout)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use axum::{extract::Path, response::Html, routing::get, Router};
use lazy_static::lazy_static;
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

// Load the heavy template at compile time or startup
lazy_static! {
    static ref HEAVY_TEMPLATE: String = {
        let content = std::fs::read_to_string("heavy_template.html")
            .unwrap_or_else(|_| "<html><body><h1>Missing template</h1></body></html>".to_string());

        // Remove existing links so the spider doesn't crawl out to the real internet
        content.replace("href=\"", "data-href=\"").replace("href='", "data-href='")
    };

    // Split point where we inject our dynamic navigation
    static ref TEMPLATE_PARTS: (String, String) = {
        if let Some(idx) = HEAVY_TEMPLATE.find("<body") {
            if let Some(end_idx) = HEAVY_TEMPLATE[idx..].find('>') {
                let split_point = idx + end_idx + 1;
                return (
                    HEAVY_TEMPLATE[..split_point].to_string(),
                    HEAVY_TEMPLATE[split_point..].to_string()
                );
            }
        }
        (String::new(), (*HEAVY_TEMPLATE).clone())
    };
}

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

async fn generate_page(page_num: usize) -> Html<String> {
    // 1. Simulate real-world network latency / DB lookup (Time To First Byte)
    // 50ms is a very realistic average for a fast web server under load
    sleep(Duration::from_millis(50)).await;

    // 2. Generate the dynamic navigation
    // Each page links to the next 50 pages to ensure massive discovery tree
    // We cap the site at 10,000 pages (approx 6 GB of raw HTML data to scrape)
    const MAX_PAGES: usize = 10_000;

    let mut links = String::with_capacity(4096);
    links.push_str("<div id=\"mock-navigation\">\n<h2>Site Navigation</h2>\n<ul>\n");

    if page_num < MAX_PAGES {
        for i in 1..=50 {
            let next_page = page_num + i;
            if next_page <= MAX_PAGES {
                links.push_str(&format!(
                    "<li><a href=\"/page_{next_page}.html\">Heavy Page {}</a></li>\n",
                    next_page
                ));
            }
        }
    }
    links.push_str("</ul>\n</div>\n");

    // 3. Assemble the massive HTML string (approx 625 KB per page)
    let header = format!("<h1>Realistic Load Page {}</h1>\n", page_num);

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
async fn main() {
    // Ensure template is loaded
    lazy_static::initialize(&TEMPLATE_PARTS);
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
    info!("- 10,000 total pages");

    let listener = tokio::net::TcpListener::bind(&addr).await
        .unwrap();
    axum::serve(listener, app).await
        .unwrap();
}
