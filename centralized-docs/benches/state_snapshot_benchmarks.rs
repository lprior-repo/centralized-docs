#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

//! Performance benchmarks for the state snapshot layer (cdocs-0tv).
//!
//! Covers: serialize_snapshot, load_snapshots, and commit_changes
//! at various scales.

use chrono::{TimeZone, Utc};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use doc_transformer::state::{serialize_snapshot, StateChanges, StateDb};
use doc_transformer::watch::{PageHash, Snapshot};
use std::collections::BTreeMap;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_page_hash(url: &str, title: &str, hash_bytes: [u8; 32]) -> PageHash {
    PageHash {
        url: url.to_string(),
        content_hash: hash_bytes,
        title: title.to_string(),
    }
}

fn make_snapshot(target: &str, num_pages: usize) -> Snapshot {
    let pages: BTreeMap<String, PageHash> = (0..num_pages)
        .map(|i| {
            let url = format!("{target}/page-{i}");
            let mut h = [0u8; 32];
            h[0] = (i % 256) as u8;
            let page = make_page_hash(&url, &format!("Page {i}"), h);
            (url, page)
        })
        .collect();

    Snapshot {
        target_url: target.to_string(),
        timestamp: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        pages,
    }
}

fn make_key(i: usize) -> [u8; 32] {
    let mut k = [0u8; 32];
    let i_bytes = (i as u64).to_le_bytes();
    k[..8].copy_from_slice(&i_bytes);
    k
}

// ---------------------------------------------------------------------------
// Benchmark: serialize_snapshot
// ---------------------------------------------------------------------------

fn bench_serialize_snapshot(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialize_snapshot");

    for num_pages in [0, 1, 10, 50, 100] {
        let snapshot = make_snapshot("https://example.com", num_pages);
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(
            BenchmarkId::new("pages", num_pages),
            &snapshot,
            |b, snap| {
                b.iter(|| {
                    let result = serialize_snapshot(black_box(snap));
                    let _ = black_box(result);
                });
            },
        );
    }

    group.finish();
}

// ---------------------------------------------------------------------------
// Benchmark: commit_changes (write path)
// ---------------------------------------------------------------------------

fn bench_commit_changes(c: &mut Criterion) {
    let mut group = c.benchmark_group("commit_changes");

    for num_entries in [1, 10, 100, 1000] {
        group.throughput(Throughput::Elements(num_entries as u64));

        group.bench_with_input(
            BenchmarkId::new("new_snapshots", num_entries),
            &num_entries,
            |b, &n| {
                b.iter_with_setup(
                    || {
                        // Setup: create temp db + changes
                        let dir = tempfile::tempdir().unwrap();
                        let db_path = dir.path().join("bench.redb");
                        let db = StateDb::open(&db_path).unwrap();

                        let new_snapshots: Vec<([u8; 32], Vec<u8>)> = (0..n)
                            .map(|i| {
                                let snap = make_snapshot(&format!("https://bench-{i}.com"), 1);
                                let bytes = serialize_snapshot(&snap).unwrap();
                                (make_key(i), bytes)
                            })
                            .collect();

                        let changes = StateChanges {
                            updated_files: vec![],
                            deleted_files: vec![],
                            new_analyses: vec![],
                            new_transforms: vec![],
                            new_chunks: vec![],
                            updated_urls: vec![],
                            deleted_urls: vec![],
                            new_scrapes: vec![],
                            new_snapshots,
                            deleted_snapshots: vec![],
                        };

                        (dir, db, changes)
                    },
                    |(_dir, db, changes)| {
                        let _ = black_box(db.commit_changes(black_box(changes)));
                    },
                );
            },
        );
    }

    group.finish();
}

// ---------------------------------------------------------------------------
// Benchmark: load_snapshots (read path)
// ---------------------------------------------------------------------------

fn bench_load_snapshots(c: &mut Criterion) {
    let mut group = c.benchmark_group("load_snapshots");

    for num_entries in [1, 10, 100, 1000] {
        group.throughput(Throughput::Elements(num_entries as u64));

        group.bench_with_input(
            BenchmarkId::new("hashes", num_entries),
            &num_entries,
            |b, &n| {
                b.iter_with_setup(
                    || {
                        // Setup: create temp db, populate
                        let dir = tempfile::tempdir().unwrap();
                        let db_path = dir.path().join("bench.redb");
                        let db = StateDb::open(&db_path).unwrap();

                        let new_snapshots: Vec<([u8; 32], Vec<u8>)> = (0..n)
                            .map(|i| {
                                let snap = make_snapshot(&format!("https://bench-{i}.com"), 1);
                                let bytes = serialize_snapshot(&snap).unwrap();
                                (make_key(i), bytes)
                            })
                            .collect();

                        let changes = StateChanges {
                            updated_files: vec![],
                            deleted_files: vec![],
                            new_analyses: vec![],
                            new_transforms: vec![],
                            new_chunks: vec![],
                            updated_urls: vec![],
                            deleted_urls: vec![],
                            new_scrapes: vec![],
                            new_snapshots,
                            deleted_snapshots: vec![],
                        };
                        db.commit_changes(changes).unwrap();

                        let keys: Vec<[u8; 32]> = (0..n).map(make_key).collect();

                        (dir, db, keys)
                    },
                    |(_dir, db, keys)| {
                        // Begin read + load in the hot loop (includes session overhead)
                        let session = db.begin_read().unwrap();
                        let _ = black_box(session.load_snapshots(black_box(&keys)));
                    },
                );
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_serialize_snapshot,
    bench_commit_changes,
    bench_load_snapshots
);
criterion_main!(benches);
