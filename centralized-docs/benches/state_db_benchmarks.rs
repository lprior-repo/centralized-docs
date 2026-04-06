//! Benchmarks for centralized-docs state database operations.
//!
//! # Running
//!
//! ```bash
//! cargo bench -p centralized-docs --bench state_db_benchmarks
//! ```

#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use doc_transformer::state::FileStateRaw;
use doc_transformer::state::{StateChanges, StateDb};

fn bench_state_db_open(c: &mut Criterion) {
    let dir = tempfile::TempDir::new().expect("Failed to create temp dir");

    c.bench_function("state_db_open", |b| {
        b.iter(|| {
            let _ = StateDb::open(black_box(dir.path()));
        })
    });
}

fn bench_state_changes_empty(c: &mut Criterion) {
    c.bench_function("state_changes_empty", |b| {
        b.iter(|| {
            let _ = StateChanges::empty();
        })
    });
}

fn bench_file_state_raw_zeroed(c: &mut Criterion) {
    c.bench_function("file_state_raw_zeroed", |b| {
        b.iter(|| {
            let _ = FileStateRaw::zeroed();
        })
    });
}

fn bench_file_state_raw_serialization(c: &mut Criterion) {
    let file_state = FileStateRaw::zeroed();

    c.bench_function("file_state_raw_to_bytes", |b| {
        b.iter(|| {
            let _ = black_box(&file_state).to_bytes();
        })
    });

    c.bench_function("file_state_raw_from_bytes", |b| {
        let bytes = file_state.to_bytes();
        b.iter(|| {
            let _ = FileStateRaw::from_bytes(black_box(&bytes));
        })
    });
}

fn bench_commit_empty_changes(c: &mut Criterion) {
    let dir = tempfile::TempDir::new().expect("Failed to create temp dir");
    let db = StateDb::open(dir.path()).expect("StateDb::open should succeed");

    c.bench_function("commit_empty_changes", |b| {
        b.iter(|| {
            let changes = StateChanges::empty();
            let _ = db.commit_changes(changes);
        })
    });
}

fn bench_commit_changes_with_files(c: &mut Criterion) {
    let file_counts = [10, 100];

    let mut group = c.benchmark_group("commit_changes_with_files");
    for count in file_counts {
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            let dir = tempfile::TempDir::new().expect("Failed to create temp dir");
            let db = StateDb::open(dir.path()).expect("StateDb::open should succeed");

            b.iter(|| {
                let mut changes = StateChanges::empty();
                for i in 0..count {
                    let path = format!("docs/file_{i}.md");
                    let mut file_state = FileStateRaw::zeroed();
                    // Set content_hash to a pseudo-random value based on i
                    file_state.content_hash = [
                        (i & 0xff) as u8,
                        ((i >> 8) & 0xff) as u8,
                        ((i >> 16) & 0xff) as u8,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ];
                    file_state.last_processed_secs = 1_700_000_000 + i as u64;
                    changes.updated_files.push((path, file_state));
                }
                let _ = db.commit_changes(changes);
            })
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_state_db_open,
    bench_state_changes_empty,
    bench_file_state_raw_zeroed,
    bench_file_state_raw_serialization,
    bench_commit_empty_changes,
    bench_commit_changes_with_files,
);
criterion_main!(benches);
