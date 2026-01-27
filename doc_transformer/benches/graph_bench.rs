use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use doc_transformer::chunk::{Chunk, ChunkLevel};
use doc_transformer::graph::KnowledgeDAG;
use doc_transformer::index::build_knowledge_dag;
use std::collections::HashMap;

/// Generate synthetic test chunks with realistic structure
/// Each chunk represents a section of a document
fn generate_test_chunks(n: usize) -> Vec<Chunk> {
    let docs_per_batch = (n as f64).sqrt().ceil() as usize;
    let chunks_per_doc = n.div_ceil(docs_per_batch);

    let mut chunks = Vec::with_capacity(n);

    for doc_idx in 0..docs_per_batch {
        let doc_id = format!("doc_{doc_idx:04}");
        let doc_title = format!("Document {doc_idx}");

        for chunk_idx in 0..chunks_per_doc {
            if chunks.len() >= n {
                break;
            }

            let chunk_id = format!("chunk_{doc_idx}_{chunk_idx:04}");
            let previous_chunk_id = if chunk_idx > 0 {
                Some(format!("chunk_{}_{:04}", doc_idx, chunk_idx - 1))
            } else {
                None
            };

            let next_chunk_id = if chunk_idx + 1 < chunks_per_doc {
                Some(format!("chunk_{}_{:04}", doc_idx, chunk_idx + 1))
            } else {
                None
            };

            let chunk = Chunk {
                chunk_id,
                doc_id: doc_id.clone(),
                doc_title: doc_title.clone(),
                chunk_index: chunk_idx,
                content: format!(
                    "Content for chunk {chunk_idx} in document {doc_idx}. This is sample documentation text."
                ),
                token_count: 256 + (chunk_idx % 256),
                heading: Some(format!("Section {chunk_idx}")),
                chunk_type: "standard".to_string(),
                previous_chunk_id,
                next_chunk_id,
                related_chunk_ids: Vec::new(),
                summary: format!("Summary of chunk {chunk_idx} in doc {doc_idx}"),
                chunk_level: ChunkLevel::Standard,
                parent_chunk_id: None,
                child_chunk_ids: Vec::new(),
            };

            chunks.push(chunk);
        }
    }

    chunks
}

/// Generate synthetic index documents
fn generate_test_documents(chunks: &[Chunk]) -> Vec<doc_transformer::index::IndexDocument> {
    // Group chunks by doc_id
    let mut docs_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut docs_titles: HashMap<String, String> = HashMap::new();

    for chunk in chunks {
        docs_map
            .entry(chunk.doc_id.clone())
            .or_default()
            .push(chunk.chunk_id.clone());
        docs_titles
            .entry(chunk.doc_id.clone())
            .or_insert_with(|| chunk.doc_title.clone());
    }

    docs_map
        .into_iter()
        .enumerate()
        .map(|(idx, (doc_id, chunk_ids))| {
            let title = docs_titles
                .get(&doc_id)
                .cloned()
                .unwrap_or_else(|| format!("Document {idx}"));

            doc_transformer::index::IndexDocument {
                id: doc_id.clone(),
                title,
                path: format!("/docs/doc_{idx}.md"),
                category: format!("Category {}", idx % 5),
                tags: vec![
                    format!("tag_{}", idx % 3),
                    format!("tag_{}", (idx + 1) % 3),
                    format!("tag_{}", (idx + 2) % 3),
                ],
                summary: format!("Summary for document {idx}"),
                word_count: 1000 + idx * 100,
                chunk_ids,
            }
        })
        .collect()
}

/// Generate document tags for relationship detection
fn generate_test_tags(chunks: &[Chunk]) -> Vec<(String, Vec<String>, String)> {
    // Group by doc_id and create tags
    let mut docs_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut docs_categories: HashMap<String, String> = HashMap::new();

    for chunk in chunks {
        docs_map
            .entry(chunk.doc_id.clone())
            .or_default()
            .push(chunk.chunk_id.clone());
        docs_categories
            .entry(chunk.doc_id.clone())
            .or_insert_with_key(|doc_id| {
                let doc_num: usize = doc_id
                    .strip_prefix("doc_")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0);
                format!("Category {}", doc_num % 5)
            });
    }

    docs_map
        .into_iter()
        .enumerate()
        .map(|(idx, (doc_id, _))| {
            let category = docs_categories
                .get(&doc_id)
                .cloned()
                .unwrap_or_else(|| format!("Category {}", idx % 5));

            // Create semantically meaningful tags
            let tags = vec![
                format!("tag_{}", idx % 3),
                format!("tag_{}", (idx + 1) % 3),
                format!("tag_{}", (idx + 2) % 3),
                "documentation".to_string(),
                format!("section_{}", (idx / 10) % 10),
            ];

            (doc_id, tags, category)
        })
        .collect()
}

/// Build a knowledge DAG with manually controlled parameters
/// This is the function being benchmarked
fn build_dag_for_benchmark(
    chunks: &[Chunk],
    documents: &[doc_transformer::index::IndexDocument],
    document_tags: &[(String, Vec<String>, String)],
) -> KnowledgeDAG {
    build_knowledge_dag(documents, chunks, document_tags)
}

fn benchmark_dag_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("dag_construction");

    // Configure group for longer-running benchmarks
    group.sample_size(10);
    group.measurement_time(std::time::Duration::from_secs(30));

    // Test various scales: 100, 1K, 5K, 10K chunks
    for n in [100, 1_000, 5_000, 10_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
            let chunks = black_box(generate_test_chunks(n));
            let documents = black_box(generate_test_documents(&chunks));
            let tags = black_box(generate_test_tags(&chunks));

            b.iter(|| {
                build_dag_for_benchmark(black_box(&chunks), black_box(&documents), black_box(&tags))
            });
        });
    }

    group.finish();
}

fn benchmark_dag_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("dag_scaling");

    group.sample_size(5);
    group.measurement_time(std::time::Duration::from_secs(60));

    // Test larger scales to demonstrate scaling characteristics
    for n in [5_000, 10_000, 20_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
            let chunks = black_box(generate_test_chunks(n));
            let documents = black_box(generate_test_documents(&chunks));
            let tags = black_box(generate_test_tags(&chunks));

            b.iter(|| {
                build_dag_for_benchmark(black_box(&chunks), black_box(&documents), black_box(&tags))
            });
        });
    }

    group.finish();
}

fn benchmark_chunk_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("chunk_generation");

    group.sample_size(10);

    for n in [100, 1_000, 5_000, 10_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
            b.iter(|| generate_test_chunks(black_box(n)));
        });
    }

    group.finish();
}

fn benchmark_tag_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("tag_generation");

    group.sample_size(10);

    for n in [100, 1_000, 5_000, 10_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
            let chunks = generate_test_chunks(n);

            b.iter(|| generate_test_tags(black_box(&chunks)));
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_chunk_generation,
    benchmark_tag_generation,
    benchmark_dag_construction,
    benchmark_dag_scaling
);
criterion_main!(benches);
