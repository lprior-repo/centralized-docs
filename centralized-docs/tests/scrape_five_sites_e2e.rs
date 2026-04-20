#![allow(clippy::unwrap_used, clippy::expect_used)]

use serde_json::Value;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, sleep, JoinHandle};
use std::time::Duration;
use tempfile::TempDir;

struct SiteCase {
    name: &'static str,
    query: &'static str,
    use_no_sitemap: bool,
    expected_pages: usize,
    expected_total_urls: usize,
}

struct ResponseSpec {
    status: u16,
    body: String,
    content_type: &'static str,
    location: Option<String>,
}

struct SiteServer {
    addr: String,
    join: Option<JoinHandle<()>>,
    stop: Arc<AtomicBool>,
}

impl SiteServer {
    fn base_url(&self) -> String {
        format!("http://{}", self.addr)
    }
}

impl Drop for SiteServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        let _ = TcpStream::connect(&self.addr);
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}

fn binary_path() -> PathBuf {
    Path::new(env!("CARGO_BIN_EXE_ctd")).to_path_buf()
}

fn run_cli(args: &[&str]) -> std::process::Output {
    Command::new(binary_path()).args(args).output().unwrap()
}

fn sitemap(urls: &[String]) -> String {
    let body = urls
        .iter()
        .map(|url| format!("<url><loc>{url}</loc></url>"))
        .collect::<Vec<_>>()
        .join("");
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">{body}</urlset>"#
    )
}

fn html(title: &str, body: &str) -> String {
    format!("<html><head><title>{title}</title></head><body>{body}</body></html>")
}

fn parse_path(stream: &mut TcpStream) -> String {
    let mut buffer = [0_u8; 4096];
    let bytes = stream.read(&mut buffer).unwrap_or(0);
    String::from_utf8_lossy(&buffer[..bytes])
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("/")
        .to_string()
}

fn write_response(stream: &mut TcpStream, response: &ResponseSpec) {
    let reason = if response.status == 200 {
        "OK"
    } else if response.status == 302 {
        "Found"
    } else {
        "Not Found"
    };
    let location = response
        .location
        .as_ref()
        .map_or(String::new(), |value| format!("Location: {value}\r\n"));
    let message = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n{}Connection: close\r\n\r\n{}",
        response.status,
        reason,
        response.content_type,
        response.body.len(),
        location,
        response.body
    );
    let _ = stream.write_all(message.as_bytes());
    let _ = stream.flush();
}

fn spawn_site(case: &SiteCase) -> SiteServer {
    let port = portpicker::pick_unused_port().expect("portpicker should return a port");
    let addr = format!("127.0.0.1:{port}");
    let base_url = format!("http://{addr}");
    let routes = build_routes(case, &base_url);
    let stop = Arc::new(AtomicBool::new(false));
    let listener = TcpListener::bind(&addr).expect("listener should bind");
    listener
        .set_nonblocking(true)
        .expect("listener should be nonblocking");
    let join_stop = Arc::clone(&stop);
    let join = thread::spawn(move || loop {
        if join_stop.load(Ordering::SeqCst) {
            break;
        }
        match listener.accept() {
            Ok((mut stream, _)) => {
                let path = parse_path(&mut stream);
                let missing = ResponseSpec {
                    status: 404,
                    body: "missing".to_string(),
                    content_type: "text/plain; charset=utf-8",
                    location: None,
                };
                let response = routes.get(&path).unwrap_or(&missing);
                write_response(&mut stream, response);
            }
            Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                sleep(Duration::from_millis(10));
            }
            Err(_) => break,
        }
    });
    sleep(Duration::from_millis(100));
    SiteServer {
        addr,
        join: Some(join),
        stop,
    }
}

fn build_routes(case: &SiteCase, base_url: &str) -> HashMap<String, ResponseSpec> {
    let ok = |body: String, content_type| ResponseSpec {
        status: 200,
        body,
        content_type,
        location: None,
    };
    let mut routes = HashMap::from([(
        "/robots.txt".to_string(),
        ok(
            "User-agent: *\nAllow: /\n".to_string(),
            "text/plain; charset=utf-8",
        ),
    )]);

    match case.name {
        "alphaorchid" => {
            let urls = [
                format!("{base_url}/"),
                format!("{base_url}/guide.html"),
                format!("{base_url}/api.html"),
                format!("{base_url}/nested/setup.html"),
            ];
            routes.extend(HashMap::from([
                ("/".to_string(), ok(html("alpha home", "<h1>alphaorchid</h1><a href=\"/guide.html\">Guide</a><a href=\"/api.html\">API</a>"), "text/html; charset=utf-8")),
                ("/guide.html".to_string(), ok(html("alpha guide", "<h1>Guide</h1><p>alphaorchid guide keyword</p><a href=\"/nested/setup.html\">Setup</a>"), "text/html; charset=utf-8")),
                ("/api.html".to_string(), ok(html("alpha api", "<h1>API</h1><p>alphaorchid api keyword</p>"), "text/html; charset=utf-8")),
                ("/nested/setup.html".to_string(), ok(html("alpha setup", "<h1>Setup</h1><p>alphaorchid setup keyword</p>"), "text/html; charset=utf-8")),
                ("/sitemap.xml".to_string(), ok(sitemap(&urls), "application/xml")),
            ]));
        }
        "bravopine" => {
            routes.extend(HashMap::from([
                ("/".to_string(), ok(html("bravo home", "<h1>bravopine</h1><a href=\"/docs/start.html\">Start</a>"), "text/html; charset=utf-8")),
                ("/docs/start.html".to_string(), ok(html("bravo start", "<h1>Start</h1><p>bravopine start keyword</p><a href=\"/docs/advanced.html\">Advanced</a>"), "text/html; charset=utf-8")),
                ("/docs/advanced.html".to_string(), ok(html("bravo advanced", "<h1>Advanced</h1><p>bravopine advanced keyword</p>"), "text/html; charset=utf-8")),
            ]));
        }
        "charliequartz" => {
            routes.extend(HashMap::from([
                (
                    "/".to_string(),
                    ResponseSpec {
                        status: 302,
                        body: String::new(),
                        content_type: "text/plain; charset=utf-8",
                        location: Some("/docs/welcome.html".to_string()),
                    },
                ),
                (
                    "/docs/welcome.html".to_string(),
                    ok(
                        html(
                            "charlie welcome",
                            "<h1>charliequartz</h1><a href=\"/docs/reference.html\">Reference</a>",
                        ),
                        "text/html; charset=utf-8",
                    ),
                ),
                (
                    "/docs/reference.html".to_string(),
                    ok(
                        html(
                            "charlie reference",
                            "<h1>Reference</h1><p>charliequartz reference keyword</p>",
                        ),
                        "text/html; charset=utf-8",
                    ),
                ),
            ]));
        }
        "deltamesa" => {
            let urls = [
                format!("{base_url}/"),
                format!("{base_url}/deep/level/one.html"),
                format!("{base_url}/deep/level/two.html"),
            ];
            routes.extend(HashMap::from([
                ("/".to_string(), ok("<html><head><title>delta home</title></head><body><h1>deltamesa<div><a href=\"/deep/level/one.html\">One</a>".to_string(), "text/html; charset=utf-8")),
                ("/deep/level/one.html".to_string(), ok(html("delta one", "<section><h1>One</h1><a href=\"/deep/level/two.html\">Two</a></section>"), "text/html; charset=utf-8")),
                ("/deep/level/two.html".to_string(), ok(html("delta two", "<article><h1>Two</h1><p>deltamesa unique keyword</p></article>"), "text/html; charset=utf-8")),
                ("/sitemap.xml".to_string(), ok(sitemap(&urls), "application/xml")),
            ]));
        }
        _ => {
            let urls = [
                format!("{base_url}/"),
                format!("{base_url}/ok.html"),
                format!("{base_url}/missing.html"),
            ];
            routes.extend(HashMap::from([
                ("/".to_string(), ok(html("echo home", "<h1>echofjord</h1><a href=\"/ok.html\">OK</a><a href=\"/missing.html\">Missing</a>"), "text/html; charset=utf-8")),
                ("/ok.html".to_string(), ok(html("echo ok", "<h1>OK</h1><p>echofjord success keyword</p>"), "text/html; charset=utf-8")),
                ("/sitemap.xml".to_string(), ok(sitemap(&urls), "application/xml")),
            ]));
        }
    }

    routes
}

fn scrape_site(case: &SiteCase, output_dir: &Path, base_url: &str) {
    let mut args = vec![
        "scrape",
        base_url,
        "--output",
        output_dir.to_str().unwrap(),
        "--request-timeout-secs",
        "5",
        "--connect-timeout-secs",
        "3",
    ];
    if case.use_no_sitemap {
        args.push("--no-sitemap");
    }
    let result = run_cli(&args);
    assert!(
        result.status.success(),
        "scrape failed for {}: {}",
        case.name,
        String::from_utf8_lossy(&result.stderr)
    );
}

fn index_site(scrape_dir: &Path, index_dir: &Path, name: &str) {
    let result = run_cli(&[
        "index",
        scrape_dir.to_str().unwrap(),
        "--output",
        index_dir.to_str().unwrap(),
        "--project-name",
        name,
    ]);
    assert!(
        result.status.success(),
        "index failed for {name}: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

fn assert_search(index_dir: &Path, query: &str, name: &str) {
    let result = run_cli(&[
        "search",
        query,
        "--index-dir",
        index_dir.to_str().unwrap(),
        "--json",
        "--limit",
        "3",
    ]);
    assert!(
        result.status.success(),
        "search failed for {name}: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(String::from_utf8_lossy(&result.stdout).contains(query));
}

fn assert_artifacts(case: &SiteCase, output_dir: &Path, index_dir: &Path) {
    let manifest: Value = serde_json::from_str(
        &std::fs::read_to_string(output_dir.join(".scrape/manifest.json")).unwrap(),
    )
    .unwrap();
    let success_count = manifest["success_count"].as_u64().unwrap() as usize;
    let total_urls = manifest["total_urls"].as_u64().unwrap() as usize;
    assert!(
        success_count >= case.expected_pages,
        "{} scraped too few pages",
        case.name
    );
    assert!(
        total_urls >= case.expected_total_urls,
        "{} discovered too few URLs",
        case.name
    );
    assert!(index_dir.join("INDEX.json").exists());
    assert!(index_dir.join("llms.txt").exists());
}

#[test]
fn scrape_index_and_search_work_for_five_distinct_local_sites() {
    let cases = [
        SiteCase {
            name: "alphaorchid",
            query: "alphaorchid",
            use_no_sitemap: false,
            expected_pages: 4,
            expected_total_urls: 4,
        },
        SiteCase {
            name: "bravopine",
            query: "bravopine",
            use_no_sitemap: true,
            expected_pages: 3,
            expected_total_urls: 3,
        },
        SiteCase {
            name: "charliequartz",
            query: "charliequartz",
            use_no_sitemap: true,
            expected_pages: 2,
            expected_total_urls: 2,
        },
        SiteCase {
            name: "deltamesa",
            query: "deltamesa",
            use_no_sitemap: false,
            expected_pages: 3,
            expected_total_urls: 3,
        },
        SiteCase {
            name: "echofjord",
            query: "echofjord",
            use_no_sitemap: false,
            expected_pages: 2,
            expected_total_urls: 3,
        },
    ];

    for case in cases {
        let server = spawn_site(&case);
        let temp = TempDir::new().unwrap();
        let scrape_output = temp.path().join("scrape-output");
        let index_output = temp.path().join("index-output");
        scrape_site(&case, &scrape_output, &server.base_url());
        index_site(&scrape_output.join(".scrape"), &index_output, case.name);
        assert_artifacts(&case, &scrape_output, &index_output);
        assert_search(&index_output, case.query, case.name);
    }
}
