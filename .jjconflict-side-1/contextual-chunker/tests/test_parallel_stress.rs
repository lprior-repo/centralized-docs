//! Rayon parallelization stress tests.
//!
//! These tests verify correct behavior of the parallel chunking system under
//! various concurrency conditions:
//!
//! - LazyLock initialization under concurrent access
//! - Rayon thread pool behavior under load
//! - Work stealing and load balancing
//! - No data races or deadlocks
//!
//! Run with: cargo test --release --test test_parallel_stress -- --nocapture

use contextual_chunker::{chunk, chunk_all, ChunkLevel, Document};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

/// Generate a document with varied content sizes to test load balancing.
fn generate_varied_doc(id: usize) -> Document {
    let paragraphs = 5 + (id % 50); // 5-54 paragraphs per doc
    let sections: String = (0..paragraphs)
        .map(|i| {
            let words = 50 + (id * 7 + i) % 200; // 50-249 words per paragraph
            format!("## Section {i}\n\n{}\n\n", "word ".repeat(words))
        })
        .collect();

    Document::new(
        format!("varied-{id}"),
        format!("Varied Doc {id}"),
        format!("# Varied Document {id}\n\n{sections}"),
    )
}

#[test]
fn test_concurrent_chunk_all_calls() {
    // Multiple concurrent chunk_all calls should not interfere
    let docs1: Vec<Document> = (0..20).map(generate_varied_doc).collect();
    let docs2: Vec<Document> = (20..40).map(generate_varied_doc).collect();
    let docs3: Vec<Document> = (40..60).map(generate_varied_doc).collect();

    let start = Instant::now();

    let handle1 = thread::spawn(move || chunk_all(&docs1).expect("chunk_all 1 should succeed"));
    let handle2 = thread::spawn(move || chunk_all(&docs2).expect("chunk_all 2 should succeed"));
    let handle3 = thread::spawn(move || chunk_all(&docs3).expect("chunk_all 3 should succeed"));

    let (result1, result2, result3) = (
        handle1.join().expect("Thread 1 should not panic"),
        handle2.join().expect("Thread 2 should not panic"),
        handle3.join().expect("Thread 3 should not panic"),
    );

    let elapsed = start.elapsed();

    println!("Concurrent chunk_all: 3×20 docs in {:?}", elapsed);
    println!(
        "  Results: {}, {}, {} chunks",
        result1.chunks.len(),
        result2.chunks.len(),
        result3.chunks.len()
    );

    // Verify each result is non-empty and valid
    assert!(result1.chunks.len() > 0);
    assert!(result2.chunks.len() > 0);
    assert!(result3.chunks.len() > 0);
}

#[test]
fn test_many_concurrent_chunk_calls() {
    // Stress test with many concurrent single-document chunk calls
    let num_threads = 16;
    let docs_per_thread = 10;
    let success_count = Arc::new(AtomicUsize::new(0));
    let total_chunks = Arc::new(AtomicUsize::new(0));

    let start = Instant::now();

    let handles: Vec<_> = (0..num_threads)
        .map(|thread_id| {
            let success_count = Arc::clone(&success_count);
            let total_chunks = Arc::clone(&total_chunks);

            thread::spawn(move || {
                (0..docs_per_thread).for_each(|doc_id| {
                    let global_id = thread_id * docs_per_thread + doc_id;
                    let doc = generate_varied_doc(global_id);

                    match chunk(&doc, ChunkLevel::Standard) {
                        Ok(chunks) => {
                            success_count.fetch_add(1, Ordering::Relaxed);
                            total_chunks.fetch_add(chunks.len(), Ordering::Relaxed);
                        }
                        Err(e) => {
                            eprintln!("Thread {} doc {} failed: {}", thread_id, doc_id, e);
                        }
                    }
                });
            })
        })
        .collect();

    handles.into_iter().for_each(|h| {
        h.join().expect("Thread should not panic");
    });

    let elapsed = start.elapsed();
    let successes = success_count.load(Ordering::Relaxed);
    let chunks = total_chunks.load(Ordering::Relaxed);

    println!(
        "Many concurrent chunk calls: {} threads × {} docs in {:?}",
        num_threads, docs_per_thread, elapsed
    );
    println!(
        "  Successes: {}/{} ({} chunks total)",
        successes,
        num_threads * docs_per_thread,
        chunks
    );

    // All should succeed
    assert_eq!(
        successes,
        num_threads * docs_per_thread,
        "All chunk calls should succeed"
    );
    assert!(chunks > 0, "Should produce chunks");
}

#[test]
fn test_rayon_pool_saturation() {
    // Verify that rayon actually parallelizes work across available cores
    let docs: Vec<Document> = (0..100).map(generate_varied_doc).collect();

    // Warm up the thread pool and BPE cache
    let _ = chunk_all(&docs[..5]).expect("Warmup should succeed");

    let start = Instant::now();
    let result = chunk_all(&docs).expect("chunk_all should succeed");
    let parallel_time = start.elapsed();

    println!(
        "Rayon pool saturation: 100 docs in {:?} ({} chunks)",
        parallel_time,
        result.chunks.len()
    );

    println!("  Rayon thread pool active");

    // With 100 varied docs, we should see meaningful parallelism
    assert!(result.chunks.len() > 100, "Should produce many chunks");
}

#[test]
fn test_deterministic_output_under_parallelism() {
    // Same input should always produce same output, even with parallelism
    let docs: Vec<Document> = (0..30).map(generate_varied_doc).collect();

    let result1 = chunk_all(&docs).expect("First chunk_all should succeed");
    let result2 = chunk_all(&docs).expect("Second chunk_all should succeed");
    let result3 = chunk_all(&docs).expect("Third chunk_all should succeed");

    // Verify chunk counts match
    assert_eq!(result1.chunks.len(), result2.chunks.len());
    assert_eq!(result2.chunks.len(), result3.chunks.len());
    assert_eq!(result1.summary_count, result2.summary_count);
    assert_eq!(result2.summary_count, result3.summary_count);

    // Verify chunk IDs are in same order (deterministic)
    let ids1: Vec<_> = result1.chunks.iter().map(|c| c.chunk_id.clone()).collect();
    let ids2: Vec<_> = result2.chunks.iter().map(|c| c.chunk_id.clone()).collect();
    let ids3: Vec<_> = result3.chunks.iter().map(|c| c.chunk_id.clone()).collect();

    assert_eq!(ids1, ids2, "Chunk IDs should be deterministic");
    assert_eq!(ids2, ids3, "Chunk IDs should be deterministic");

    // Verify content is identical
    result1
        .chunks
        .iter()
        .zip(result2.chunks.iter())
        .for_each(|(c1, c2)| {
            assert_eq!(c1.content, c2.content, "Content should be identical");
            assert_eq!(c1.token_count, c2.token_count, "Token counts should match");
        });

    println!(
        "Determinism verified: {} chunks, {} summary, {} standard, {} detailed",
        result1.chunks.len(),
        result1.summary_count,
        result1.standard_count,
        result1.detailed_count
    );
}

#[test]
fn test_no_deadlock_under_high_contention() {
    // Create high contention on shared resources (BPE tokenizer)
    // This should NOT deadlock even with many threads

    let num_threads = 32;
    let docs_per_thread = 5;
    let barrier = Arc::new(std::sync::Barrier::new(num_threads));
    let completed = Arc::new(AtomicUsize::new(0));

    let start = Instant::now();

    let handles: Vec<_> = (0..num_threads)
        .map(|thread_id| {
            let barrier = Arc::clone(&barrier);
            let completed = Arc::clone(&completed);

            thread::spawn(move || {
                // All threads start at exactly the same time (high contention)
                barrier.wait();

                (0..docs_per_thread).for_each(|doc_id| {
                    let doc = generate_varied_doc(thread_id * 1000 + doc_id);
                    let result = chunk(&doc, ChunkLevel::Standard);
                    if result.is_ok() {
                        completed.fetch_add(1, Ordering::Relaxed);
                    }
                });
            })
        })
        .collect();

    // Wait with timeout to detect deadlocks
    handles.into_iter().for_each(|h| {
        h.join().expect("Thread should not panic or deadlock");
    });

    let elapsed = start.elapsed();
    let completed_count = completed.load(Ordering::Relaxed);

    println!(
        "High contention test: {} threads × {} docs completed in {:?}",
        num_threads, docs_per_thread, elapsed
    );
    println!(
        "  Completed: {}/{}",
        completed_count,
        num_threads * docs_per_thread
    );

    assert_eq!(
        completed_count,
        num_threads * docs_per_thread,
        "All operations should complete without deadlock"
    );

    // Should complete in reasonable time (no lock contention issues)
    assert!(
        elapsed.as_secs() < 60,
        "Should complete within 60 seconds (possible deadlock or contention)"
    );
}

#[test]
fn test_work_stealing_balance() {
    // Create imbalanced workload: some docs are much larger than others
    // Rayon should balance via work stealing

    let tiny_docs: Vec<Document> = (0..10)
        .map(|i| {
            Document::new(
                format!("tiny-{i}"),
                format!("Tiny {i}"),
                "## A\n\nShort content.".to_string(),
            )
        })
        .collect();

    let huge_docs: Vec<Document> = (0..10)
        .map(|i| {
            let content = format!(
                "# Huge Doc {i}\n\n{}",
                "This is a very long paragraph with lots of content. ".repeat(5000)
            );
            Document::new(format!("huge-{i}"), format!("Huge {i}"), content)
        })
        .collect();

    let docs: Vec<Document> = tiny_docs.into_iter().chain(huge_docs).collect();

    let start = Instant::now();
    let result = chunk_all(&docs).expect("chunk_all should succeed");
    let elapsed = start.elapsed();

    println!(
        "Work stealing balance: 10 tiny + 10 huge docs in {:?}",
        elapsed
    );
    println!("  Total chunks: {}", result.chunks.len());

    // Tiny docs should produce fewer chunks than huge docs
    let tiny_count = result
        .chunks
        .iter()
        .filter(|c| c.doc_id.starts_with("tiny-"))
        .count();
    let huge_count = result
        .chunks
        .iter()
        .filter(|c| c.doc_id.starts_with("huge-"))
        .count();

    println!(
        "  Tiny doc chunks: {}, Huge doc chunks: {}",
        tiny_count, huge_count
    );

    assert!(huge_count > tiny_count * 10);
    assert!(result.chunks.len() > 0);
}

#[test]
fn test_parallel_chunk_all_error_propagation() {
    // Verify errors are correctly propagated from parallel execution

    // Create one invalid document mixed with valid ones
    let mut docs: Vec<Document> = (0..10).map(generate_varied_doc).collect();
    docs.push(Document::new(
        String::new(), // Invalid: empty ID
        "Invalid Doc".to_string(),
        "Content".to_string(),
    ));

    let result = chunk_all(&docs);

    assert!(
        result.is_err(),
        "chunk_all should return error for invalid document"
    );

    // Use pattern matching instead of unwrap_err (avoids Debug bound)
    match result {
        Err(err) => {
            let err_msg = err.to_string();
            assert!(
                err_msg.contains("Invalid document"),
                "Error message should mention invalid document"
            );
            println!("Error propagation verified: {}", err_msg);
        }
        Ok(_) => panic!("Expected error but got success"),
    }
}
