//! Async performance benchmarks for centralized-docs hot paths.
//!
//! Benchmarks:
//! - `embed_texts`: Batch embedding throughput for varying batch sizes
//! - `search_index`: Search latency for varying index sizes
//!
//! # Running
//!
//! ```bash
//! cargo bench -p centralized-docs --bench async_perf_benchmarks
//! ```

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use doc_transformer::embeddings::{Embedding, EmbeddingProvider, EmbeddingProviderError};
use doc_transformer::index::IndexDocument;
use doc_transformer::search::{open_or_create_index, search_index};
use std::sync::Arc;
use tempfile::TempDir;
use tokio::runtime::Runtime;

/// Mock embedding provider for deterministic benchmarking.
///
/// Generates synthetic embeddings of the correct dimension without
/// requiring external API calls.
#[derive(Debug, Clone)]
struct MockEmbeddingProvider {
    dimension: usize,
    model_name: String,
}

impl MockEmbeddingProvider {
    #[must_use]
    fn new(dimension: usize) -> Self {
        Self {
            dimension,
            model_name: "mock-embedding-model".to_string(),
        }
    }
}

impl Default for MockEmbeddingProvider {
    fn default() -> Self {
        Self::new(1536) // OpenAI text-embedding-3-small dimension
    }
}

#[doc(hidden)]
#[async_trait::async_trait]
impl EmbeddingProvider for MockEmbeddingProvider {
    fn name(&self) -> &'static str {
        "mock"
    }

    fn model(&self) -> &str {
        &self.model_name
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    async fn embed_texts(&self, texts: &[&str]) -> Result<Vec<Embedding>, EmbeddingProviderError> {
        if texts.is_empty() {
            return Err(EmbeddingProviderError::EmptyInput);
        }

        // Generate deterministic embeddings based on text content
        // This simulates realistic embedding generation without API calls
        let embeddings: Vec<Embedding> = texts
            .iter()
            .map(|text| {
                // Create a pseudo-random but deterministic vector based on text
                let seed = text.bytes().fold(0u64, |acc, b| {
                    acc.wrapping_mul(31).wrapping_add(u64::from(b))
                });
                let mut vector = vec![0.0_f32; self.dimension];
                for (i, v) in vector.iter_mut().enumerate() {
                    // Simple hash-based pseudo-random
                    let hash = seed
                        .wrapping_add(u64::try_from(i).unwrap_or(0))
                        .wrapping_mul(0x517cc1b727220a95);
                    *v = ((hash & 0xFFFF) as f32 / 0xFFFF as f32) * 2.0 - 1.0;
                }
                // Normalize the vector
                let magnitude: f32 = vector.iter().map(|x| x * x).sum::<f32>().sqrt();
                if magnitude > 0.0 {
                    for v in &mut vector {
                        *v /= magnitude;
                    }
                }
                Embedding {
                    vector,
                    text: Some(text.to_string()),
                    model: self.model_name.clone(),
                }
            })
            .collect();

        Ok(embeddings)
    }

    fn estimate_tokens(&self, text: &str) -> usize {
        text.len() / 4
    }
}

/// Benchmark embed_texts with varying batch sizes.
fn bench_embed_texts(c: &mut Criterion) {
    let batch_sizes = [1, 10, 100];

    let mut group = c.benchmark_group("embed_texts");

    // Pre-create runtime to avoid measuring runtime creation
    let rt = Runtime::new().expect("Failed to create tokio runtime");

    for batch_size in batch_sizes {
        // Create test texts
        let texts: Vec<String> = (0..batch_size)
            .map(|i| {
                format!(
                    "This is test document number {} with some content for embedding.",
                    i
                )
            })
            .collect();
        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();

        // Create provider
        let provider = MockEmbeddingProvider::default();

        group.bench_with_input(
            BenchmarkId::from_parameter(batch_size),
            &batch_size,
            |b, _bs| {
                b.iter(|| {
                    let _result = rt.block_on(provider.embed_texts(black_box(&text_refs)));
                });
            },
        );
    }

    group.finish();
}

/// Create a search index with the specified number of documents.
fn create_search_index(
    doc_count: usize,
) -> std::result::Result<(TempDir, tantivy::Index), anyhow::Error> {
    let dir = TempDir::new()?;
    let index = open_or_create_index(dir.path())?;

    // Create test documents
    let documents: Vec<IndexDocument> = (0..doc_count)
        .map(|i| {
            let category = match i % 4 {
                0 => "tutorial",
                1 => "concept",
                2 => "reference",
                _ => "ops",
            };
            IndexDocument {
                id: format!("doc-{}", i),
                title: format!("Document {} Title", i),
                path: format!("docs/doc-{}.md", i),
                category: category.to_string(),
                tags: vec!["tag1".to_string(), "tag2".to_string()],
                summary: format!(
                    "This is the summary for document {} which contains important information.",
                    i
                ),
                word_count: 100 + (i % 500),
                chunk_ids: vec![],
                headings: vec!["Introduction".to_string(), "Details".to_string()],
                content: Arc::from(format!(
                    "Document {} content. This file contains detailed documentation about topic {}. \
                     It includes multiple sections covering various aspects of the subject matter. \
                     The content is designed to be searchable and relevant for testing purposes.",
                    i,
                    i % 50
                )),
            }
        })
        .collect();

    // Index the documents
    let mut writer = index.writer(50_000_000)?;
    doc_transformer::search::index_documents(&mut writer, &documents)?;
    writer.commit()?;

    Ok((dir, index))
}

/// Benchmark search_index with varying index sizes.
fn bench_search_docs(c: &mut Criterion) {
    let index_sizes = [100, 1000, 10000];
    let queries = ["tutorial", "document information", "topic details"];

    let mut group = c.benchmark_group("search_index");

    for index_size in index_sizes {
        // Build the index once per size
        let build_result = create_search_index(index_size);
        let (dir, index) = match build_result {
            Ok((d, i)) => (d, i),
            Err(e) => {
                eprintln!(
                    "Warning: Failed to create index with {} docs: {}",
                    index_size, e
                );
                continue;
            }
        };

        // Benchmark with different queries
        for query in &queries {
            let query_str = *query;
            let index_clone = &index;

            group.bench_with_input(
                BenchmarkId::new(index_size.to_string(), query_str),
                &(index_size, query_str),
                |b, _| {
                    b.iter(|| {
                        let _results = search_index(
                            black_box(index_clone),
                            black_box(query_str),
                            black_box(10),
                        );
                    });
                },
            );
        }

        // Explicit drop to ensure tempdir lives through benchmark
        drop(dir);
    }

    group.finish();
}

criterion_group!(
    name = async_benches;
    config = Criterion::default().sample_size(10).nresamples(20);
    targets = bench_embed_texts, bench_search_docs
);
criterion_main!(async_benches);
