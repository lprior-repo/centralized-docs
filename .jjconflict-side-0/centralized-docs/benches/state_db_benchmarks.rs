#![allow(clippy::pedantic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use doc_transformer::state::{StateBatch, StateDb};
use std::path::Path;

fn bench_state_db_construction(c: &mut Criterion) {
    let dir = tempfile::TempDir::new().expect("Failed to create temp dir");

    c.bench_function("state_db_new", |b| {
        b.iter(|| {
            let _ = StateDb::new(black_box(dir.path()));
        })
    });
}

fn bench_record_file_hash(c: &mut Criterion) {
    let dir = tempfile::TempDir::new().expect("Failed to create temp dir");

    c.bench_function("record_file_hash_single", |b| {
        b.iter(|| {
            let mut db = StateDb::new(dir.path()).expect("StateDb::new should succeed");
            let _ = db.record_file_hash(black_box("src/guide.md"), black_box("sha256:abc123"));
        })
    });
}

fn bench_record_file_hash_batch(c: &mut Criterion) {
    let dir = tempfile::TempDir::new().expect("Failed to create temp dir");

    let mut group = c.benchmark_group("record_file_hash_batch");
    for size in [10, 100, 1000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let mut db = StateDb::new(dir.path()).expect("StateDb::new should succeed");
                for i in 0..size {
                    let _ = db.record_file_hash(&format!("file_{i}.md"), &format!("hash_{i}"));
                }
            })
        });
    }
    group.finish();
}

fn bench_commit_changes(c: &mut Criterion) {
    let dir = tempfile::TempDir::new().expect("Failed to create temp dir");

    c.bench_function("commit_changes_with_100_files", |b| {
        b.iter(|| {
            let mut db = StateDb::new(dir.path()).expect("StateDb::new should succeed");
            db.set_document_count(100)
                .expect("set_document_count should succeed");
            db.set_chunk_count(500)
                .expect("set_chunk_count should succeed");
            for i in 0..100 {
                db.record_file_hash(&format!("file_{i}.md"), &format!("hash_{i}"))
                    .expect("record_file_hash should succeed");
            }
            let _ = db.commit_changes();
        })
    });
}

fn bench_state_batch_serialization(c: &mut Criterion) {
    let batch = StateBatch {
        run_id: doc_transformer::state::RunId("bench-run-001".to_string()),
        source_path: "/source".to_string(),
        output_path: "/output".to_string(),
        document_count: 100,
        chunk_count: 500,
        file_hashes: (0..100)
            .map(|i| doc_transformer::state::FileHashRecord {
                relative_path: format!("file_{i}.md"),
                content_hash: format!("sha256:{i:016x}"),
            })
            .collect(),
        created_at_unix_secs: 1_700_000_000,
    };

    c.bench_function("state_batch_serialize_json", |b| {
        b.iter(|| {
            let _ = serde_json::to_vec(black_box(&batch));
        })
    });

    c.bench_function("state_batch_deserialize_json", |b| {
        let serialized = serde_json::to_vec(&batch).expect("serialization should succeed");
        b.iter(|| {
            let _: StateBatch = serde_json::from_slice(black_box(&serialized))
                .expect("deserialization should succeed");
        })
    });
}

criterion_group!(
    benches,
    bench_state_db_construction,
    bench_record_file_hash,
    bench_record_file_hash_batch,
    bench_commit_changes,
    bench_state_batch_serialization,
);
criterion_main!(benches);
