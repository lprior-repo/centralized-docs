#![allow(clippy::print_stdout)]
#![allow(clippy::dbg_macro)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::print_stderr)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use futures::stream::{self, StreamExt};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("========================================================");
    info!("Starting Comprehensive 500 Site Benchmark Evaluator...");
    info!("========================================================");

    // 1. Get 500 sites dynamically from crates.io
    let mut sites = Vec::new();
    let client = reqwest::Client::builder()
        .user_agent("ctd_benchmark/1.0")
        .build()?;

    info!("Fetching top crates from crates.io...");
    for page in 1..=6 {
        // 6 pages of 100 = 600, then truncate
        let url = format!(
            "https://crates.io/api/v1/crates?page={}&per_page=100&sort=downloads",
            page
        );
        let resp = client
            .get(&url)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        if let Some(crates) = resp.get("crates").and_then(|c| c.as_array()) {
            for c in crates {
                if let Some(id) = c.get("id").and_then(|i| i.as_str()) {
                    sites.push(format!("https://docs.rs/{}/latest/", id));
                }
            }
        }
    }

    if sites.is_empty() {
        info!("Failed to fetch from crates.io. Using fallback list.");
        sites = vec![
            "https://doc.rust-lang.org/book/".to_string(),
            "https://docs.python.org/3/tutorial/".to_string(),
        ];
    }

    sites.truncate(500);
    println!("Found {} sites to benchmark.", sites.len());

    let cli_path = if PathBuf::from("target/release/ctd").exists() {
        PathBuf::from("target/release/ctd")
    } else if PathBuf::from("../target/release/ctd").exists() {
        PathBuf::from("../target/release/ctd")
    } else {
        info!("Error: Could not find target/release/ctd. Please run 'moon run :build' from the root workspace first.");
        return Ok(());
    };

    let out_dir = PathBuf::from("benchmark_results_500");
    let _ = fs::remove_dir_all(&out_dir);
    fs::create_dir_all(&out_dir)?;

    let success_count = Arc::new(AtomicUsize::new(0));
    let partial_count = Arc::new(AtomicUsize::new(0));
    let fail_count = Arc::new(AtomicUsize::new(0));
    let completed_count = Arc::new(AtomicUsize::new(0));

    let total_start = Instant::now();
    let total_sites = sites.len();

    info!("\nStarting parallel scrape (Concurrency: 10)...\n");

    let fetches = stream::iter(sites.into_iter().map(|site| {
        let cli_path = cli_path.clone();
        let out_dir = out_dir.clone();
        let success_count = success_count.clone();
        let partial_count = partial_count.clone();
        let fail_count = fail_count.clone();
        let completed_count = completed_count.clone();

        async move {
            let safe_name = site
                .replace("https://", "")
                .replace("http://", "")
                .replace(['/', '.'], "_");
            let site_out = out_dir.join(&safe_name);

            let start = Instant::now();

            let site_clone = site.clone();
            // Run synchronous Command inside tokio blocking task to not block the executor
            let output_res = tokio::task::spawn_blocking(move || {
                Command::new("timeout")
                    .arg("36000") // 10 hours timeout limit for an absolute maximum
                    .arg(&cli_path)
                    .arg("scrape")
                    .arg(&site_clone)
                    .arg("--output")
                    .arg(&site_out)
                    .output()
            })
            .await;

            let duration = start.elapsed();
            let completed = completed_count.fetch_add(1, Ordering::SeqCst) + 1;

            if let Ok(Ok(out)) = output_res {
                match out.status.code() {
                    Some(0) => {
                        println!(
                            "[{}/{}] ✅ Perfect Success in {:.2}s: {}",
                            completed,
                            total_sites,
                            duration.as_secs_f64(),
                            site
                        );
                        success_count.fetch_add(1, Ordering::SeqCst);
                    }
                    Some(1 | 2) => {
                        // doc_transformer uses code 2 for partial failure
                        println!(
                            "[{}/{}] ⚠️ Partial Success in {:.2}s: {}",
                            completed,
                            total_sites,
                            duration.as_secs_f64(),
                            site
                        );
                        partial_count.fetch_add(1, Ordering::SeqCst);
                    }
                    Some(124) => {
                        println!(
                            "[{}/{}] ❌ Timed Out in {:.2}s: {}",
                            completed,
                            total_sites,
                            duration.as_secs_f64(),
                            site
                        );
                        fail_count.fetch_add(1, Ordering::SeqCst);
                    }
                    _ => {
                        println!(
                            "[{}/{}] Failed in {:.2}s: {}",
                            completed,
                            total_sites,
                            duration.as_secs_f64(),
                            site
                        );
                        fail_count.fetch_add(1, Ordering::SeqCst);
                    }
                }
            } else {
                println!(
                    "[{}/{}] ❌ OS Execution Error: {}",
                    completed, total_sites, site
                );
                fail_count.fetch_add(1, Ordering::SeqCst);
            }
        }
    }))
    .buffer_unordered(10); // Process 10 sites concurrently

    fetches.collect::<Vec<()>>().await;

    let total_duration = total_start.elapsed();
    let s = success_count.load(Ordering::SeqCst);
    let p = partial_count.load(Ordering::SeqCst);
    let f = fail_count.load(Ordering::SeqCst);

    info!("\n========================================================");
    info!("BENCHMARK COMPLETE");
    info!("========================================================");
    println!(
        "Total Wall-Clock Time: {:.2}s",
        total_duration.as_secs_f64()
    );
    println!(
        "Average Time/Site:   {:.2}s",
        total_duration.as_secs_f64() / total_sites as f64
    );
    println!("Perfect Successes:   {}", s);
    println!("Partial Successes:   {}", p);
    println!("Failures/Timeouts:   {}", f);
    println!(
        "Success Rate:        {:.1}%",
        ((s + p) as f64 / total_sites as f64) * 100.0
    );
    println!("Results Output Dir:  {}", out_dir.display());
    info!("========================================================");

    Ok(())
}
