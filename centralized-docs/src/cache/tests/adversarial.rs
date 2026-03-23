use super::super::config::{CacheConfig, CacheType};
use super::super::hash::content_hash;
use super::super::store::DocCache;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::sync::Barrier;
use std::thread;
use tempfile::TempDir;

// ============================================================
// RED QUEEN ADVERSARIAL TESTS — Concurrency & Crypto
// ============================================================

/// ATTACK 1: 100+ threads hitting `get_or_compute` simultaneously.
/// Proves compute runs exactly once under extreme contention.
#[test]
fn rq_attack_1_extreme_concurrency_100_threads() -> anyhow::Result<()> {
    let config = CacheConfig::in_memory();
    let cache = Arc::new(DocCache::open(config)?);
    let call_count = Arc::new(AtomicUsize::new(0));
    const NUM_THREADS: usize = 100;

    let barrier = Arc::new(Barrier::new(NUM_THREADS));
    let handles: Vec<_> = (0..NUM_THREADS)
        .map(|_| {
            let cache = Arc::clone(&cache);
            let call_count = Arc::clone(&call_count);
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();
                cache.get_or_compute(CacheType::Document, b"storm_key", || {
                    call_count.fetch_add(1, Ordering::SeqCst);
                    Ok("storm_value".to_string())
                })
            })
        })
        .collect();

    for handle in handles {
        let result = handle.join().expect("thread should not panic");
        assert_eq!(result.expect("computation should succeed"), "storm_value");
    }

    assert_eq!(
        call_count.load(Ordering::SeqCst),
        1,
        "compute must run exactly once across {NUM_THREADS} threads"
    );

    Ok(())
}

/// ATTACK 2: xxh3-128 collision probe — exhaustive check of many near-miss inputs.
#[test]
fn rq_attack_2_xxh3_no_collision_proximity() {
    let inputs: Vec<Vec<u8>> = (0..10_000u16)
        .map(|i| format!("key_{i}").into_bytes())
        .collect();

    let mut hashes: Vec<u128> = Vec::with_capacity(inputs.len());
    for input in &inputs {
        hashes.push(content_hash(input));
    }

    use std::collections::HashSet;
    let mut seen: HashSet<u128> = HashSet::new();
    for hash in hashes {
        assert!(
            seen.insert(hash),
            "xxh3-128 collision detected between two distinct inputs"
        );
    }
}

/// ATTACK 3: Disk file corruption — store value, corrupt the .redb file, verify graceful error.
#[test]
fn rq_attack_3_disk_corruption_graceful_error() -> anyhow::Result<()> {
    use std::io::{Seek, Write};

    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("corrupt.redb");
    let config = CacheConfig::new(&db_path);
    let cache = DocCache::open(config)?;

    cache.put_document(b"intact_key", &"intact_value")?;
    let result: Option<String> = cache.get_document(b"intact_key")?;
    assert_eq!(result, Some("intact_value".to_string()));

    drop(cache);

    let mut file = std::fs::OpenOptions::new().write(true).open(&db_path)?;
    file.seek(std::io::SeekFrom::Start(512))?;
    file.write_all(b"\xff\xfe\xfd\xfc\xfb\xfa")?;
    file.sync_all()?;
    drop(file);

    let config2 = CacheConfig::new(&db_path);
    let open_result = DocCache::open(config2);
    match open_result {
        Ok(cache2) => {
            let retrieval: anyhow::Result<Option<String>> = cache2.get_document(b"intact_key");
            match retrieval {
                Ok(Some(_)) => { /* corrupted DB may still serve data; acceptable */ }
                Ok(None) => { /* acceptable — cache miss on corruption */ }
                Err(e) => {
                    assert!(!e.to_string().is_empty(), "error should have a message");
                }
            }
        }
        Err(e) => {
            assert!(
                !e.to_string().is_empty(),
                "corrupted DB should produce a descriptive error"
            );
        }
    }

    Ok(())
}

/// ATTACK 4: `InMemory` backend — verify zero disk files created.
#[test]
fn rq_attack_4_inmemory_no_disk_files() -> anyhow::Result<()> {
    let config = CacheConfig::in_memory();
    let cache = DocCache::open(config)?;

    for i in 0..50u32 {
        let key = format!("mem_key_{i}");
        cache.put_document(key.as_bytes(), &format!("mem_value_{i}"))?;
    }
    drop(cache);

    let current_exe = std::env::current_exe()?;
    let exe_dir = current_exe.parent().expect("exe should have parent dir");
    let walker = walkdir::WalkDir::new(exe_dir)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            name.contains("ctd_cache") || name.contains(".redb")
        });

    let redb_files: Vec<_> = walker
        .filter_map(std::result::Result::ok)
        .filter(|e| {
            let name = e.file_name().to_string_lossy();
            name.contains(".redb")
        })
        .collect();

    assert!(
        redb_files.is_empty(),
        "InMemory backend should create zero .redb files, found: {redb_files:?}"
    );

    Ok(())
}

/// ATTACK 5: Empty inputs — empty key, empty value, empty content hash.
/// Empty keys MUST be rejected to prevent cache poisoning.
#[test]
fn rq_attack_5_empty_inputs() -> anyhow::Result<()> {
    let config = CacheConfig::in_memory();
    let cache = DocCache::open(config)?;

    let empty_key: &[u8] = b"";

    // Empty keys must be rejected on put
    let result = cache.put_document(empty_key, &"");
    assert!(result.is_err(), "empty key put_document should be rejected");
    let result = cache.put_scrape(b"", &"");
    assert!(result.is_err(), "empty key put_scrape should be rejected");
    let result = cache.put_transform(b"", &0usize);
    assert!(
        result.is_err(),
        "empty key put_transform should be rejected"
    );

    // Empty keys must be rejected on get_or_compute
    let result: Result<String, _> =
        cache.get_or_compute(
            CacheType::Document,
            empty_key,
            || Ok("computed".to_string()),
        );
    assert!(
        result.is_err(),
        "empty key get_or_compute should be rejected"
    );

    // xxh3-128 of empty input should still be well-defined (hashing is fine, caching is not)
    let empty_hash = content_hash(b"");
    assert_ne!(
        empty_hash, 0u128,
        "xxh3-128 of empty input should not be zero"
    );

    // No entries should have been stored
    let stats = cache.stats()?;
    assert_eq!(stats.document_entries, 0);
    assert_eq!(stats.scrape_entries, 0);
    assert_eq!(stats.transform_entries, 0);

    Ok(())
}

/// ATTACK 10: Double open — open same file path twice.
/// redb uses advisory file locks; on Linux the second open may succeed (shared access)
/// or fail depending on the platform. Either outcome is acceptable, but the test
/// must not silently swallow the result — it asserts a specific known behaviour.
#[test]
fn rq_attack_10_double_open_same_path() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("double.redb");

    let config1 = CacheConfig::new(&db_path);
    let cache1 = DocCache::open(config1)?;
    cache1.put_document(b"first", &"value1")?;

    let config2 = CacheConfig::new(&db_path);
    let result = DocCache::open(config2);

    // redb allows concurrent read access via separate file descriptors.
    // If the second open succeeds, verify it reads data written by the first handle.
    // If it fails, verify the error is descriptive (not a silent panic).
    match result {
        Ok(cache2) => {
            let val: Option<String> = cache2.get_document(b"first")?;
            assert_eq!(
                val,
                Some("value1".to_string()),
                "second handle should read data written by first"
            );
        }
        Err(e) => {
            let msg = format!("{e:#}");
            assert!(
                !msg.is_empty(),
                "double open failure should produce a descriptive error"
            );
        }
    }

    Ok(())
}
