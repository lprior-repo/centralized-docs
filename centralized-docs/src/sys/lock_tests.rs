#[cfg(test)]
mod tests {
    use super::*;
    use crate::sys::lock::{
        get_process_start_time, now_unix_secs, should_reclaim_stale_lock, OutputLockMetadata,
        OUTPUT_LOCK_STALE_AFTER_SECS,
    };
    use std::path::{Path, PathBuf};
    use std::process;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |duration| duration.as_nanos());
        std::env::temp_dir().join(format!("{prefix}-{nanos}"))
    }

    fn write_lock_metadata(lock_path: &Path, metadata: &OutputLockMetadata) {
        let file_result = std::fs::File::create(lock_path);
        assert!(file_result.is_ok());

        if let Ok(file) = file_result {
            let write_result = serde_json::to_writer(file, metadata);
            assert!(write_result.is_ok());
        }
    }

    #[test]
    fn test_should_reclaim_stale_lock_when_pid_not_alive() {
        let temp_dir = unique_temp_dir("lock-reclaim-dead-pid");
        let create_dir_result = std::fs::create_dir_all(&temp_dir);
        assert!(create_dir_result.is_ok());

        let lock_path = temp_dir.join(".ctd.lock");
        let metadata = OutputLockMetadata {
            pid: u32::MAX,
            start_time: 0,
            created_at_unix_secs: now_unix_secs(),
        };

        write_lock_metadata(&lock_path, &metadata);
        assert!(should_reclaim_stale_lock(&lock_path));
        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_should_reclaim_stale_lock_when_too_old() {
        let temp_dir = unique_temp_dir("lock-reclaim-old");
        let create_dir_result = std::fs::create_dir_all(&temp_dir);
        assert!(create_dir_result.is_ok());

        let lock_path = temp_dir.join(".ctd.lock");
        let current_start_time = get_process_start_time(process::id()).unwrap_or(0);
        let metadata = OutputLockMetadata {
            pid: process::id(),
            start_time: current_start_time,
            created_at_unix_secs: now_unix_secs().saturating_sub(OUTPUT_LOCK_STALE_AFTER_SECS + 5),
        };

        write_lock_metadata(&lock_path, &metadata);
        assert!(should_reclaim_stale_lock(&lock_path));
        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_should_not_reclaim_fresh_live_lock() {
        let temp_dir = unique_temp_dir("lock-reclaim-live");
        let create_dir_result = std::fs::create_dir_all(&temp_dir);
        assert!(create_dir_result.is_ok());

        let lock_path = temp_dir.join(".ctd.lock");
        let current_start_time = get_process_start_time(process::id()).unwrap_or(0);
        let metadata = OutputLockMetadata {
            pid: process::id(),
            start_time: current_start_time,
            created_at_unix_secs: now_unix_secs(),
        };

        write_lock_metadata(&lock_path, &metadata);
        assert!(!should_reclaim_stale_lock(&lock_path));
        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_should_reclaim_stale_lock_when_pid_recycled() {
        let temp_dir = unique_temp_dir("lock-reclaim-recycled");
        let create_dir_result = std::fs::create_dir_all(&temp_dir);
        assert!(create_dir_result.is_ok());

        let lock_path = temp_dir.join(".ctd.lock");

        let current_start_time = get_process_start_time(process::id()).unwrap_or(0);
        let wrong_start_time = current_start_time.wrapping_add(1000);

        let metadata = OutputLockMetadata {
            pid: process::id(),
            start_time: wrong_start_time,
            created_at_unix_secs: now_unix_secs(),
        };

        write_lock_metadata(&lock_path, &metadata);
        assert!(should_reclaim_stale_lock(&lock_path));
        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_should_reclaim_stale_lock_when_empty_file() {
        let temp_dir = unique_temp_dir("lock-reclaim-empty");
        let create_dir_result = std::fs::create_dir_all(&temp_dir);
        assert!(create_dir_result.is_ok());

        let lock_path = temp_dir.join(".ctd.lock");
        let file_result = std::fs::File::create(&lock_path);
        assert!(file_result.is_ok());

        assert!(should_reclaim_stale_lock(&lock_path));
        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_should_reclaim_stale_lock_when_malformed_json() {
        let temp_dir = unique_temp_dir("lock-reclaim-malformed");
        let create_dir_result = std::fs::create_dir_all(&temp_dir);
        assert!(create_dir_result.is_ok());

        let lock_path = temp_dir.join(".ctd.lock");
        let write_result = std::fs::write(&lock_path, "{not valid json");
        assert!(write_result.is_ok());

        assert!(should_reclaim_stale_lock(&lock_path));
        let _ = std::fs::remove_dir_all(temp_dir);
    }
}
