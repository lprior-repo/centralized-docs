use super::super::config::{CacheConfig, CacheType};
use super::super::store::DocCache;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

#[test]
fn test_get_or_compute_caches_result() -> anyhow::Result<()> {
    let temp_dir = tempfile::TempDir::new()?;
    let db_path = temp_dir.path().join("test.redb");
    let config = CacheConfig::new(&db_path);
    let cache = DocCache::open(config)?;

    let key = b"compute_key";
    let mut call_count = 0;

    let result1 = cache.get_or_compute(CacheType::Document, key, || {
        call_count += 1;
        Ok("computed".to_string())
    })?;

    let result2 = cache.get_or_compute(CacheType::Document, key, || {
        call_count += 1;
        Ok("should_not_compute".to_string())
    })?;

    assert_eq!(result1, "computed");
    assert_eq!(result2, "computed");
    assert_eq!(call_count, 1, "compute function should only be called once");

    Ok(())
}

/// Proves the TOCTOU fix: N threads hitting `get_or_compute` with the same key
/// concurrently invoke the compute closure exactly ONCE.
#[test]
fn test_get_or_compute_deduplicates_concurrent_computation() -> anyhow::Result<()> {
    let config = CacheConfig::in_memory();
    let cache = Arc::new(DocCache::open(config)?);
    let call_count = Arc::new(AtomicUsize::new(0));
    const NUM_THREADS: usize = 8;

    let handles: Vec<_> = (0..NUM_THREADS)
        .map(|_| {
            let cache = Arc::clone(&cache);
            let call_count = Arc::clone(&call_count);
            thread::spawn(move || {
                cache.get_or_compute(CacheType::Document, b"dedup_key", || {
                    call_count.fetch_add(1, Ordering::SeqCst);
                    // Hold the thread to maximise the window for concurrent arrivals
                    thread::sleep(std::time::Duration::from_millis(50));
                    Ok("deduplicated_value".to_string())
                })
            })
        })
        .collect();

    for handle in handles {
        let result = handle.join().expect("thread should not panic");
        assert_eq!(
            result.expect("computation should succeed"),
            "deduplicated_value"
        );
    }

    assert_eq!(
        call_count.load(Ordering::SeqCst),
        1,
        "compute closure must be invoked exactly once across {NUM_THREADS} concurrent threads"
    );

    Ok(())
}

/// ATTACK 11: Error propagation race — compute fails while 50 waiters are parked.
#[test]
fn rq_attack_11_error_propagation_50_waiters() -> anyhow::Result<()> {
    let config = CacheConfig::in_memory();
    let cache = Arc::new(DocCache::open(config)?);
    let call_count = Arc::new(AtomicUsize::new(0));
    let owner_go = Arc::new(std::sync::atomic::AtomicBool::new(false));

    let owner_cache = Arc::clone(&cache);
    let owner_count = Arc::clone(&call_count);
    let owner_go_flag = Arc::clone(&owner_go);
    let owner = thread::spawn(move || {
        owner_go_flag.store(true, Ordering::SeqCst);
        let result: anyhow::Result<String> =
            owner_cache.get_or_compute(CacheType::Document, b"race_fail_key", || {
                owner_count.fetch_add(1, Ordering::SeqCst);
                thread::sleep(std::time::Duration::from_millis(100));
                Err(anyhow::anyhow!("boom: intentional compute failure"))
            });
        assert!(result.is_err(), "owner should get error");
        result
    });

    while !owner_go.load(Ordering::SeqCst) {
        thread::yield_now();
    }
    thread::sleep(std::time::Duration::from_millis(5));

    const NUM_WAITERS: usize = 50;
    let waiter_handles: Vec<_> = (0..NUM_WAITERS)
        .map(|_| {
            let cache = Arc::clone(&cache);
            let count = Arc::clone(&call_count);
            thread::spawn(move || {
                let result: anyhow::Result<String> =
                    cache.get_or_compute(CacheType::Document, b"race_fail_key", || {
                        count.fetch_add(1, Ordering::SeqCst);
                        Ok("waiter should never compute".to_string())
                    });
                assert!(result.is_err(), "waiter should receive error from owner");
                result
            })
        })
        .collect();

    let _ = owner.join().expect("owner should not panic");
    for w in waiter_handles {
        let _ = w.join().expect("waiter should not panic");
    }

    assert_eq!(
        call_count.load(Ordering::SeqCst),
        1,
        "only owner should compute; {NUM_WAITERS} waiters should never execute closure"
    );

    Ok(())
}
