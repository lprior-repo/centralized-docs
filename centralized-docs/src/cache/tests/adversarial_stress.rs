use super::super::config::{CacheConfig, CacheType};
use super::super::store::DocCache;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::sync::Barrier;
use std::thread;

// ============================================================
// RED QUEEN ADVERSARIAL TESTS — Concurrency stress & edge cases
// ============================================================

/// ATTACK 8: Concurrent `clear_all` during read.
#[test]
fn rq_attack_8_concurrent_clear_during_read() -> anyhow::Result<()> {
    let config = CacheConfig::in_memory();
    let cache = Arc::new(DocCache::open(config)?);

    for i in 0..100u32 {
        cache.put_document(format!("key_{i}").as_bytes(), &format!("value_{i}"))?;
    }

    let stop = Arc::new(AtomicBool::new(false));
    let reader_running = Arc::new(AtomicBool::new(false));
    let errors = Arc::new(AtomicUsize::new(0));

    let reader_cache = Arc::clone(&cache);
    let reader_stop = Arc::clone(&stop);
    let reader_errors = Arc::clone(&errors);
    let reader_running_flag = Arc::clone(&reader_running);
    let reader = thread::spawn(move || {
        reader_running_flag.store(true, Ordering::SeqCst);
        let mut reads_ok = 0usize;
        while !reader_stop.load(Ordering::SeqCst) {
            match reader_cache.get_document::<String>(b"key_0") {
                Ok(_) => {
                    reads_ok += 1;
                }
                Err(_) => {
                    reader_errors.fetch_add(1, Ordering::SeqCst);
                }
            }
        }
        reads_ok
    });

    while !reader_running.load(Ordering::SeqCst) {
        thread::yield_now();
    }

    let clearer_cache = Arc::clone(&cache);
    let clearer_errors = Arc::clone(&errors);
    let clearer = thread::spawn(move || {
        for _ in 0..20 {
            if clearer_cache.clear_all().is_err() {
                clearer_errors.fetch_add(1, Ordering::SeqCst);
            }
            for i in 0..100u32 {
                let _ = clearer_cache
                    .put_document(format!("key_{i}").as_bytes(), &format!("value_{i}"));
            }
        }
    });

    clearer.join().expect("clearer thread should not panic");
    stop.store(true, Ordering::SeqCst);
    let reads_ok = reader.join().expect("reader thread should not panic");

    let err_count = errors.load(Ordering::SeqCst);
    assert_eq!(
        err_count, 0,
        "concurrent clear+read should never panic; got {err_count} errors across {reads_ok} reads"
    );

    Ok(())
}

/// ATTACK 9: Concurrent `clear_all` during `get_or_compute`.
#[test]
fn rq_attack_9_concurrent_clear_during_compute() -> anyhow::Result<()> {
    let config = CacheConfig::in_memory();
    let cache = Arc::new(DocCache::open(config)?);
    let barrier = Arc::new(Barrier::new(2));

    let owner_cache = Arc::clone(&cache);
    let owner_barrier = Arc::clone(&barrier);
    let owner = thread::spawn(move || -> anyhow::Result<String> {
        let result: anyhow::Result<String> =
            owner_cache.get_or_compute(CacheType::Document, b"compute_during_clear", || {
                owner_barrier.wait();
                Ok("survived_clear".to_string())
            });
        result
    });

    let clearer_cache = Arc::clone(&cache);
    let clearer_barrier = Arc::clone(&barrier);
    thread::spawn(move || {
        let _ = clearer_cache.clear_all();
        clearer_barrier.wait();
    })
    .join()
    .expect("clearer should not panic");

    let result = owner.join().expect("owner should not panic");
    assert!(
        result.is_ok(),
        "compute during clear should still succeed: {result:?}"
    );
    assert_eq!(result.unwrap(), "survived_clear");

    Ok(())
}
