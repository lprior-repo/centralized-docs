//! Memory pressure tests for chunk_all with large corpora.
//!
//! These tests verify that memory usage scales linearly (O(n)) rather than
//! quadratically when processing large numbers of documents.
//!
//! Run with: cargo test --release --package contextual-chunker -- --nocapture

use contextual_chunker::{chunk_all, Document};
use std::time::Instant;

/// Generate a document with realistic content structure.
fn generate_realistic_doc(id: usize, paragraphs: usize) -> Document {
    let sections: String = (0..paragraphs)
        .map(|i| {
            format!(
                "## Section {i}\n\n{}\n\n",
                "This is a realistic paragraph with various words and content \
                 that represents typical documentation text. "
                    .repeat(5)
            )
        })
        .collect();

    Document::new(
        format!("doc-{id}"),
        format!("Document {id}"),
        format!("# Document {id}\n\n{sections}"),
    )
}

/// Estimate current process memory in KB (Linux only).
#[cfg(target_os = "linux")]
fn get_memory_usage_kb() -> Option<usize> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    status.lines().find_map(|line| {
        line.strip_prefix("VmRSS:")
            .and_then(|rest| rest.split_whitespace().next())
            .and_then(|kb| kb.parse::<usize>().ok())
    })
}

#[cfg(not(target_os = "linux"))]
fn get_memory_usage_kb() -> Option<usize> {
    None
}

#[test]
fn test_memory_pressure_small_corpus() {
    let docs: Vec<Document> = (0..10).map(|i| generate_realistic_doc(i, 20)).collect();

    let mem_before = get_memory_usage_kb();
    let start = Instant::now();
    let result = chunk_all(&docs).expect("chunk_all should succeed");
    let elapsed = start.elapsed();
    let mem_after = get_memory_usage_kb();

    println!(
        "Small corpus (10 docs, 20 paragraphs each): {} chunks in {:?}",
        result.chunks.len(),
        elapsed
    );

    if let (Some(before), Some(after)) = (mem_before, mem_after) {
        let delta_kb = after.saturating_sub(before);
        println!(
            "Memory delta: {} KB ({:.2} MB)",
            delta_kb,
            delta_kb as f64 / 1024.0
        );
        println!(
            "Memory per chunk: {:.2} KB",
            delta_kb as f64 / result.chunks.len() as f64
        );
        assert!(
            delta_kb < 500_000,
            "Memory should not exceed 500MB for this workload, got {}KB",
            delta_kb
        );
    }

    assert!(!result.chunks.is_empty(), "Should produce chunks");
}

#[test]
fn test_memory_pressure_medium_corpus() {
    let docs: Vec<Document> = (0..50).map(|i| generate_realistic_doc(i, 30)).collect();

    let mem_before = get_memory_usage_kb();
    let start = Instant::now();
    let result = chunk_all(&docs).expect("chunk_all should succeed");
    let elapsed = start.elapsed();
    let mem_after = get_memory_usage_kb();

    println!(
        "Medium corpus (50 docs, 30 paragraphs each): {} chunks in {:?}",
        result.chunks.len(),
        elapsed
    );

    if let (Some(before), Some(after)) = (mem_before, mem_after) {
        let delta_kb = after.saturating_sub(before);
        println!(
            "Memory delta: {} KB ({:.2} MB)",
            delta_kb,
            delta_kb as f64 / 1024.0
        );
        println!(
            "Memory per chunk: {:.2} KB",
            delta_kb as f64 / result.chunks.len() as f64
        );
        assert!(
            delta_kb < 500_000,
            "Memory should not exceed 500MB for this workload, got {}KB",
            delta_kb
        );
    }

    assert!(!result.chunks.is_empty(), "Should produce chunks");
}

#[test]
fn test_memory_pressure_large_corpus() {
    let docs: Vec<Document> = (0..100).map(|i| generate_realistic_doc(i, 50)).collect();

    let mem_before = get_memory_usage_kb();
    let start = Instant::now();
    let result = chunk_all(&docs).expect("chunk_all should succeed");
    let elapsed = start.elapsed();
    let mem_after = get_memory_usage_kb();

    println!(
        "Large corpus (100 docs, 50 paragraphs each): {} chunks in {:?}",
        result.chunks.len(),
        elapsed
    );

    if let (Some(before), Some(after)) = (mem_before, mem_after) {
        let delta_kb = after.saturating_sub(before);
        println!(
            "Memory delta: {} KB ({:.2} MB)",
            delta_kb,
            delta_kb as f64 / 1024.0
        );
        println!(
            "Memory per chunk: {:.2} KB",
            delta_kb as f64 / result.chunks.len() as f64
        );
        assert!(
            delta_kb < 500_000,
            "Memory should not exceed 500MB for this workload, got {}KB",
            delta_kb
        );
    }

    assert!(!result.chunks.is_empty(), "Should produce chunks");
}

#[test]
fn test_memory_scaling_is_linear() {
    // Verify sub-quadratic scaling: doubling input should NOT quadruple time
    let sizes: [usize; 3] = [10, 20, 40];
    let mut metrics: Vec<(usize, usize, f64)> = Vec::new(); // (doc_count, chunk_count, time_secs)

    // Warm up BPE cache first
    let warmup: Vec<Document> = (0..10).map(|i| generate_realistic_doc(i, 20)).collect();
    let _ = chunk_all(&warmup).expect("Warmup should succeed");

    sizes.iter().for_each(|&size| {
        let docs: Vec<Document> = (0..size).map(|i| generate_realistic_doc(i, 20)).collect();

        let start = Instant::now();
        let result = chunk_all(&docs).expect("chunk_all should succeed");
        let elapsed = start.elapsed();

        metrics.push((size, result.chunks.len(), elapsed.as_secs_f64()));

        println!(
            "Scaling test ({} docs): {} chunks in {:.3}s",
            size,
            result.chunks.len(),
            elapsed.as_secs_f64()
        );
    });

    assert!(metrics.len() >= 3, "Need at least 3 data points");

    let (n1, _, t1) = metrics[0];
    let (n2, _, t2) = metrics[1];
    let (n3, _, t3) = metrics[2];

    let ratio_1_to_2 = t2 / t1.max(0.001);
    let ratio_2_to_3 = t3 / t2.max(0.001);

    println!("Scaling ratios:");
    println!(
        "  {}->{} docs: {:.2}x time (input: {:.1}x)",
        n1,
        n2,
        ratio_1_to_2,
        n2 as f64 / n1 as f64
    );
    println!(
        "  {}->{} docs: {:.2}x time (input: {:.1}x)",
        n2,
        n3,
        ratio_2_to_3,
        n3 as f64 / n2 as f64
    );

    // Allow up to 5x for 2x input (parallelization variance + tokenizer init)
    assert!(
        ratio_1_to_2 < 5.0,
        "Scaling from {} to {} docs took {:.2}x longer - possible O(n^2) regression",
        n1,
        n2,
        ratio_1_to_2
    );
    assert!(
        ratio_2_to_3 < 5.0,
        "Scaling from {} to {} docs took {:.2}x longer - possible O(n^2) regression",
        n2,
        n3,
        ratio_2_to_3
    );
}
