#![allow(clippy::print_stdout, clippy::unwrap_used)]

use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use doc_transformer::scrape::{scrape_site, ScrapeConfig, SitemapStrategy};
use portpicker::pick_unused_port;
use spider::configuration::RedirectPolicy;
use tiny_http::{Response, Server, StatusCode};

fn start_server(
    handler: Arc<dyn Fn(tiny_http::Request) + Send + Sync + 'static>,
) -> (String, thread::JoinHandle<()>) {
    let port = pick_unused_port().expect("port available");
    let addr = format!("127.0.0.1:{port}");
    let server = Server::http(&addr).expect("start server");
    let handle = thread::spawn(move || {
        for request in server.incoming_requests() {
            (handler)(request);
        }
    });

    (format!("http://{addr}"), handle)
}

#[tokio::test(flavor = "multi_thread")]
async fn timeout_is_enforced_against_slow_server() {
    let handler = Arc::new(|req: tiny_http::Request| {
        // sleep longer than timeout
        std::thread::sleep(Duration::from_secs(2));
        let _ = req.respond(Response::from_string("ok"));
    });

    let (base_url, handle) = start_server(handler);

    let config = ScrapeConfig {
        base_url: base_url.clone(),
        sitemap_strategy: SitemapStrategy::CrawlOnly,
        path_filter: None,
        delay_ms: 0,
        request_timeout_secs: 1,
        max_retries: 0,
        ..Default::default()
    };

    let start = Instant::now();
    let result = scrape_site(&config).await;
    let elapsed = start.elapsed();

    // Should error and return quickly (well under the server sleep)
    assert!(result.is_err(), "expected timeout error");
    assert!(
        elapsed < Duration::from_secs(2),
        "timeout should short-circuit slow server"
    );

    // Shutdown server thread
    drop(handle);
}

#[tokio::test(flavor = "multi_thread")]
async fn redirect_policy_none_blocks_redirect() {
    let handler = Arc::new(|req: tiny_http::Request| {
        if req.url() == "/" {
            let resp = Response::empty(StatusCode(302)).with_header(
                tiny_http::Header::from_bytes(&b"Location"[..], &b"/final"[..]).unwrap(),
            );
            let _ = req.respond(resp);
        } else {
            let _ = req.respond(Response::from_string("final"));
        }
    });

    let (base_url, handle) = start_server(handler);

    // Policy None should block redirects
    let blocked = ScrapeConfig {
        base_url: base_url.clone(),
        sitemap_strategy: SitemapStrategy::CrawlOnly,
        path_filter: None,
        delay_ms: 0,
        request_timeout_secs: 5,
        max_retries: 0,
        redirect_policy: RedirectPolicy::None,
        ..Default::default()
    };
    let blocked_result = scrape_site(&blocked).await;
    assert!(
        blocked_result.is_err(),
        "redirects should be blocked with policy None"
    );

    // Policy Loose should allow redirects
    let allowed = ScrapeConfig {
        base_url: base_url.clone(),
        sitemap_strategy: SitemapStrategy::CrawlOnly,
        path_filter: None,
        delay_ms: 0,
        request_timeout_secs: 5,
        max_retries: 0,
        redirect_policy: RedirectPolicy::Loose,
        ..Default::default()
    };
    let allowed_result = scrape_site(&allowed).await;
    assert!(
        allowed_result.is_ok(),
        "redirects should be allowed with policy Loose"
    );

    drop(handle);
}

#[tokio::test(flavor = "multi_thread")]
async fn spider_max_page_bytes_limits_download() {
    let handler = Arc::new(|req: tiny_http::Request| {
        // Return a 2KB body
        let mut body = vec![b'a'; 2048];
        let _ = req.respond(Response::from_data(body.split_off(0)));
    });

    let (base_url, handle) = start_server(handler);

    let config = ScrapeConfig {
        base_url: base_url.clone(),
        sitemap_strategy: SitemapStrategy::CrawlOnly,
        path_filter: None,
        delay_ms: 0,
        request_timeout_secs: 5,
        max_retries: 0,
        spider_max_page_bytes: Some(64),
        spider_max_total_bytes: None,
        ..Default::default()
    };

    let result = scrape_site(&config).await;

    // Spider should reject or record an error due to size cap
    match result {
        Ok(r) => {
            assert!(r.pages.is_empty(), "expected no pages under byte cap");
        }
        Err(e) => {
            let msg = e.to_string();
            assert!(msg.contains("bytes") || msg.contains("timeout") || msg.contains("payload"));
        }
    }

    drop(handle);
}
