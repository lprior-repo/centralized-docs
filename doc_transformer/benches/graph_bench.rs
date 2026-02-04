use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use doc_transformer::chunk::{Chunk, ChunkLevel};
use doc_transformer::graph::KnowledgeDAG;
use doc_transformer::index::build_knowledge_dag;
use std::collections::HashMap;
use std::time::Duration;

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
                heading_path: vec!["Document".to_string(), format!("Section {chunk_idx}")],
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

fn generate_test_documents(chunks: &[Chunk]) -> Vec<doc_transformer::index::IndexDocument> {
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
                headings: vec![
                    "Introduction".to_string(),
                    "Content".to_string(),
                    "Conclusion".to_string(),
                ],
            }
        })
        .collect()
}

fn generate_test_tags(chunks: &[Chunk]) -> Vec<(String, Vec<String>, String)> {
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

fn build_dag_for_benchmark(
    documents: &[doc_transformer::index::IndexDocument],
    chunks: &[Chunk],
    document_tags: &[(String, Vec<String>, String)],
) -> KnowledgeDAG {
    build_knowledge_dag(documents, chunks, document_tags, None, None, None).unwrap()
}

fn benchmark_dag_with_metrics(c: &mut Criterion) {
    let mut group = c.benchmark_group("dag_with_metrics");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(30));

    let test_sizes = [100, 1_000, 5_000, 10_000];
    let mut times: Vec<Duration> = Vec::new();

    for &n in &test_sizes {
        let chunks = generate_test_chunks(n);
        let documents = generate_test_documents(&chunks);
        let tags = generate_test_tags(&chunks);

        let edge_count = estimate_edge_count(n);

        group.throughput(Throughput::Elements(edge_count as u64));

        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &_n| {
            b.iter(|| {
                let dag = build_dag_for_benchmark(
                    black_box(&documents),
                    black_box(&chunks),
                    black_box(&tags),
                );
                dag.edges().len()
            });
        });

        let start = std::time::Instant::now();
        for _ in 0..3 {
            let _ = build_dag_for_benchmark(&documents, &chunks, &tags);
        }
        let elapsed = start.elapsed() / 3;
        times.push(elapsed);

        eprintln!(
            "N={:>5}: {} avg time, ~{} edges, {:.0} edges/sec",
            n,
            format_duration(elapsed),
            edge_count,
            edge_count as f64 / elapsed.as_secs_f64()
        );
    }

    group.finish();

    eprintln!("\n=== Scaling Validation ===");
    for i in 0..test_sizes.len() - 1 {
        let n1 = test_sizes[i];
        let n2 = test_sizes[i + 1];
        let ratio = if n2 >= n1 * 2 && times[i + 1] > Duration::ZERO {
            times[i + 1].as_secs_f64() / times[i].as_secs_f64()
        } else {
            0.0
        };

        if ratio > 0.0 {
            let passes = ratio < 2.5;
            eprintln!(
                "{}x ({}→{}): {:.2}x time increase - {} (threshold: <2.5x)",
                n2 / n1,
                n1,
                n2,
                ratio,
                if passes { "✓ PASS" } else { "✗ FAIL" }
            );
        }
    }
}

fn estimate_edge_count(n: usize) -> usize {
    let sequential_edges = n;
    let parent_edges = n;
    let related_edges = n;
    sequential_edges + parent_edges + related_edges
}

fn format_duration(d: Duration) -> String {
    if d.as_secs() > 0 {
        format!("{}.{:03}s", d.as_secs(), d.subsec_millis())
    } else if d.as_millis() > 0 {
        format!("{}ms", d.as_millis())
    } else {
        format!("{}μs", d.as_micros())
    }
}

criterion_group!(benches, benchmark_dag_with_metrics);
criterion_main!(benches);
