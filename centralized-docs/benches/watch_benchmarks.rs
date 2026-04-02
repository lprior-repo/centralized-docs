#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

//! Performance benchmarks for the watch/change-plan pipeline.
//!
//! Covers: hashing, snapshot building, diffing, formatting, JSON serialization,
//! and end-to-end compute_plan at various scales.

use chrono::Utc;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use doc_transformer::cache::content_hash;
use doc_transformer::scrape::validation::{PageFilterStatus, ScrapeResult, ScrapedPage};
use doc_transformer::watch::{
    compute_plan, format_plan_json, format_plan_markdown, snapshot_from_scrape, ChangeKind,
    ChangePlan, ChangeSummary, PageChange, Snapshot,
};
use std::collections::BTreeMap;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_page(url: &str, title: &str, content: &str) -> ScrapedPage {
    ScrapedPage {
        url: url.to_string(),
        markdown: content.to_string(),
        title: title.to_string(),
        links: vec![],
        headers: vec![],
        word_count: content.split_whitespace().count(),
        slug: url.replace('/', "_"),
        filter_status: PageFilterStatus::Unfiltered,
        elements_removed: 0,
        density_score: 1.0,
    }
}

fn make_scrape_result(base: &str, n: usize, content: &str) -> ScrapeResult {
    let pages: Vec<ScrapedPage> = (0..n)
        .map(|i| make_page(&format!("{base}/page-{i}"), &format!("Page {i}"), content))
        .collect();
    ScrapeResult {
        total_urls: pages.len(),
        success_count: pages.len(),
        error_count: 0,
        errors: vec![],
        base_url: base.to_string(),
        pages,
    }
}

fn make_snapshot_from_scrape(base: &str, n: usize, content: &str) -> Snapshot {
    let result = make_scrape_result(base, n, content);
    snapshot_from_scrape(base, &result)
}

fn make_plan_with_changes(n: usize) -> ChangePlan {
    let changes: Vec<PageChange> = (0..n)
        .map(|i| PageChange {
            url: format!("https://example.com/page-{i}"),
            kind: if i % 3 == 0 {
                ChangeKind::Added
            } else if i % 3 == 1 {
                ChangeKind::Modified
            } else {
                ChangeKind::Removed
            },
            old_hash: if i % 3 == 0 {
                None
            } else {
                Some([i as u8; 32])
            },
            new_hash: if i % 3 == 2 {
                None
            } else {
                Some([(i as u8).wrapping_add(1); 32])
            },
            title: format!("Page {i}"),
        })
        .collect();

    let added = changes
        .iter()
        .filter(|c| c.kind == ChangeKind::Added)
        .count();
    let removed = changes
        .iter()
        .filter(|c| c.kind == ChangeKind::Removed)
        .count();
    let modified = changes
        .iter()
        .filter(|c| c.kind == ChangeKind::Modified)
        .count();

    ChangePlan {
        target_url: "https://example.com".to_string(),
        timestamp: Utc::now(),
        changes,
        summary: ChangeSummary {
            added,
            removed,
            modified,
            unchanged: 0,
            total_current: added + modified,
            total_previous: removed + modified,
        },
        pending_snapshot: Snapshot {
            target_url: "https://example.com".to_string(),
            timestamp: Utc::now(),
            pages: BTreeMap::new(),
        },
    }
}

// ---------------------------------------------------------------------------
// Benchmark 1: Hashing speed
// ---------------------------------------------------------------------------

fn bench_hashing(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash");

    let kb1 = "A".repeat(1024);
    let mb1 = "B".repeat(1024 * 1024);

    group.bench_function("hash_1kb_page", |b| {
        b.iter(|| content_hash(black_box(kb1.as_bytes())))
    });

    group.bench_function("hash_1mb_page", |b| {
        b.iter(|| content_hash(black_box(mb1.as_bytes())))
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Benchmark 2: Snapshot building
// ---------------------------------------------------------------------------

fn bench_snapshot_from_scrape(c: &mut Criterion) {
    let mut group = c.benchmark_group("snapshot_from_scrape");

    for n in [100, 1_000, 10_000] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            let result = make_scrape_result(
                "https://example.com",
                n,
                "some page content for hashing benchmark test",
            );
            b.iter(|| snapshot_from_scrape(black_box("https://example.com"), black_box(&result)));
        });
    }

    group.finish();
}

// ---------------------------------------------------------------------------
// Benchmark 3: Diff speed (via compute_plan since diff_snapshots is private)
// ---------------------------------------------------------------------------

fn bench_compute_plan(c: &mut Criterion) {
    let mut group = c.benchmark_group("compute_plan");
    let base = "https://example.com";
    let content = "some page content for hashing benchmark test";

    // All added: 0 prev, N curr
    for n in [100, 1_000, 10_000] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("all_added", n), &n, |b, &n| {
            let empty = Snapshot {
                target_url: base.to_string(),
                timestamp: Utc::now(),
                pages: BTreeMap::new(),
            };
            let scrape = make_scrape_result(base, n, content);
            b.iter(|| compute_plan(black_box(base), black_box(&empty), black_box(&scrape)));
        });
    }

    // All unchanged: N prev, N curr, same content
    for n in [100, 1_000, 10_000] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("all_unchanged", n), &n, |b, &n| {
            let prev = make_snapshot_from_scrape(base, n, content);
            let scrape = make_scrape_result(base, n, content);
            b.iter(|| compute_plan(black_box(base), black_box(&prev), black_box(&scrape)));
        });
    }

    // 500 modified out of 1000
    group.bench_function("500_modified_1000", |b| {
        let prev = make_snapshot_from_scrape(base, 1000, "old content");
        let scrape = make_scrape_result(base, 1000, "new content");
        b.iter(|| compute_plan(black_box(base), black_box(&prev), black_box(&scrape)));
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Benchmark 4: Format speed
// ---------------------------------------------------------------------------

fn bench_format_markdown(c: &mut Criterion) {
    let mut group = c.benchmark_group("format_markdown");

    for n in [0, 1_000, 10_000] {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            let plan = make_plan_with_changes(n);
            b.iter(|| format_plan_markdown(black_box(&plan)));
        });
    }

    group.finish();
}

// ---------------------------------------------------------------------------
// Benchmark 5: JSON serialization round-trip
// ---------------------------------------------------------------------------

fn bench_json_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("json");

    for n in [1_000, 10_000] {
        group.throughput(Throughput::Elements(n as u64));

        // ChangePlan serialize
        group.bench_with_input(BenchmarkId::new("serialize_plan", n), &n, |b, &n| {
            let plan = make_plan_with_changes(n);
            b.iter(|| format_plan_json(black_box(&plan)).unwrap());
        });

        // ChangePlan deserialize
        group.bench_with_input(BenchmarkId::new("deserialize_plan", n), &n, |b, &n| {
            let plan = make_plan_with_changes(n);
            let json = serde_json::to_string(&plan).unwrap();
            b.iter(|| {
                let _: ChangePlan = serde_json::from_str(black_box(&json)).unwrap();
            });
        });

        // Snapshot serialize
        group.bench_with_input(BenchmarkId::new("serialize_snapshot", n), &n, |b, &n| {
            let snapshot = make_snapshot_from_scrape("https://example.com", n, "page content here");
            b.iter(|| serde_json::to_string(black_box(&snapshot)).unwrap());
        });
    }

    group.finish();
}

// ---------------------------------------------------------------------------
// Benchmark 6: End-to-end compute_plan (1000 pages)
// ---------------------------------------------------------------------------

fn bench_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e");
    group.sample_size(20); // fewer samples for heavier benchmarks
    let base = "https://example.com";

    group.bench_function("compute_plan_e2e_1000", |b| {
        let prev = make_snapshot_from_scrape(base, 1000, "old content version");
        let scrape = make_scrape_result(base, 1000, "new content version");
        b.iter(|| {
            let plan = compute_plan(black_box(base), black_box(&prev), black_box(&scrape));
            let _md = format_plan_markdown(&plan);
            let _json = format_plan_json(&plan).unwrap();
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_hashing,
    bench_snapshot_from_scrape,
    bench_compute_plan,
    bench_format_markdown,
    bench_json_serialization,
    bench_e2e,
);
criterion_main!(benches);
