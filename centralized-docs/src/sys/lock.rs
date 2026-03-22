use anyhow::Result;
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct OutputLock {
    lock_path: PathBuf,
    file: std::fs::File,
}

#[derive(Debug, Serialize, Deserialize)]
struct OutputLockMetadata {
    pid: u32,
    start_time: u64,
    created_at_unix_secs: u64,
}

const OUTPUT_LOCK_STALE_AFTER_SECS: u64 = 60 * 30;

impl Drop for OutputLock {
    fn drop(&mut self) {
        if let Err(err) = self.file.unlock() {
            eprintln!("Warning: cleanup failed: {err}");
        }
        if let Err(err) = std::fs::remove_file(&self.lock_path) {
            eprintln!("Warning: cleanup failed: {err}");
        }
    }
}

pub fn acquire_output_lock(output: &Path) -> Result<OutputLock> {
    std::fs::create_dir_all(output)?;
    let lock_path = output.join(".ctd.lock");

    fn try_acquire(
        lock_path: &Path,
        output: &Path,
        retries: usize,
        max_retries: usize,
    ) -> Result<OutputLock> {
        match OpenOptions::new()
            .create_new(true)
            .write(true)
            .read(true)
            .open(lock_path)
        {
            #[allow(unused_mut)]
            Ok(mut file) => {
                // Write lock metadata
                let metadata = OutputLockMetadata {
                    pid: process::id(),
                    start_time: get_process_start_time(process::id()).map_or(0, |v| v),
                    created_at_unix_secs: now_unix_secs(),
                };

                if let Err(error) = serde_json::to_writer(&mut file, &metadata) {
                    if let Err(err) = std::fs::remove_file(lock_path) {
                        eprintln!("Warning: cleanup failed: {err}");
                    }
                    return Err(anyhow::anyhow!("Failed to write lock metadata: {error}"));
                }

                // Flush to ensure metadata is written before acquiring lock
                if let Err(error) = file.flush() {
                    if let Err(err) = std::fs::remove_file(lock_path) {
                        eprintln!("Warning: cleanup failed: {err}");
                    }
                    return Err(anyhow::anyhow!("Failed to flush lock file: {error}"));
                }

                // Acquire exclusive file lock - this is the key to preventing race conditions
                // The lock is automatically released when the file is closed (in Drop)
                if let Err(error) = file.lock_exclusive() {
                    if let Err(err) = std::fs::remove_file(lock_path) {
                        eprintln!("Warning: cleanup failed: {err}");
                    }
                    return Err(anyhow::anyhow!("Failed to acquire file lock: {error}"));
                }

                Ok(OutputLock {
                    lock_path: lock_path.to_path_buf(),
                    file,
                })
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                // Lock file exists - check if it's stale (process died or lock too old)
                // Only reclaim AFTER atomic creation fails, not before
                if lock_path.exists() && should_reclaim_stale_lock(lock_path) {
                    eprintln!("[WARN] Reclaiming stale lock at {}", lock_path.display());
                    // Try to remove stale lock - may fail if other process took it
                    if std::fs::remove_file(lock_path).is_ok() {
                        // Successfully removed stale lock, retry creation
                        return try_acquire(lock_path, output, retries, max_retries);
                    }
                }

                // Lock exists and is not stale (or failed to remove) - retry with backoff
                if retries < max_retries {
                    // Brief sleep to allow other process to release lock
                    std::thread::sleep(std::time::Duration::from_millis(50));
                    try_acquire(lock_path, output, retries.saturating_add(1), max_retries)
                } else {
                    // Max retries exceeded - report lock conflict
                    Err(anyhow::anyhow!(
                        "Another index operation appears to be running for '{}'. Remove '{}' if stale.",
                        output.display(),
                        lock_path.display()
                    ))
                }
            }
            Err(e) => Err(anyhow::anyhow!(
                "Failed to acquire output lock '{}': {e}",
                lock_path.display()
            )),
        }
    }

    try_acquire(&lock_path, output, 0, 3)
}

fn now_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs())
}

fn extract_last_path_segment(url_str: &str) -> Option<String> {
    url::Url::parse(url_str).ok().and_then(|u| {
        u.path_segments()
            .map(|segments| segments.collect::<Vec<_>>())
            .and_then(|vec| vec.into_iter().next_back().map(String::from))
    })
}

/// Get process start time in clock ticks since system boot.
/// Reads from /proc/<pid>/stat, field 22 (starttime).
fn get_process_start_time(pid: u32) -> Option<u64> {
    let stat_path = PathBuf::from("/proc").join(pid.to_string()).join("stat");
    std::fs::read_to_string(&stat_path)
        .ok()
        .and_then(|content| {
            content.split(')').nth(1).and_then(|rest| {
                let fields: Vec<&str> = rest.split_whitespace().collect();
                fields.get(19).and_then(|s| s.parse::<u64>().ok())
            })
        })
}

fn process_is_alive(pid: u32, start_time: u64) -> bool {
    let current_start_time = get_process_start_time(process::id()).map_or(0, |v| v);

    if pid == process::id() {
        return current_start_time == start_time;
    }

    get_process_start_time(pid)
        .map(|actual_start_time| actual_start_time == start_time)
        .map_or(false, |v| v)
}

fn read_lock_metadata(lock_path: &Path) -> Option<OutputLockMetadata> {
    std::fs::File::open(lock_path)
        .ok()
        .and_then(|file| serde_json::from_reader(file).ok())
}

fn should_reclaim_stale_lock(lock_path: &Path) -> bool {
    if let Some(metadata) = read_lock_metadata(lock_path) {
        let age_secs = now_unix_secs().saturating_sub(metadata.created_at_unix_secs);
        return !process_is_alive(metadata.pid, metadata.start_time)
            || age_secs > OUTPUT_LOCK_STALE_AFTER_SECS;
    }

    // FIX: If we can't read the lock metadata (empty/malformed),
    // treat it as stale immediately - likely from crashed process
    true
}

include!("lock_tests.rs");
