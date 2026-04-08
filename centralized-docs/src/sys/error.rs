/// Map errors to exit codes per contract requirements:
///
/// - Exit 0: Success
/// - Exit 1: User input errors (invalid arguments, bad format, missing files)
/// - Exit 2: Pipeline/internal errors (transform failures, corrupt data, network errors)
///
/// This ensures consistent exit codes across all validation layers:
/// - Parser-level validation (via clap `value_parser`) now exits with 1 (user error)
/// - Runtime validation also exits with 1 for user input errors
#[must_use]
pub fn map_error_to_exit_code(err: &anyhow::Error) -> i32 {
    let error_string = err.to_string();
    let error_string_lower = error_string.to_lowercase();

    // Pipeline error patterns (must check BEFORE user input patterns)
    // These are network/infrastructure errors that should exit with 2
    let pipeline_error_patterns = [
        "url protocol",
        "class=net",
        "connection refused",
        "connection timed out",
        "network error",
        "ssl error",
        "tls error",
        "certificate",
        "domain unreachable",
        "DNS",
        "failed to clone repository",
        "git clone failed",
        "no pages extracted",
        "failed to scrape",
        // Database corruption errors
        "i/o error",
        "invalid data",
        "corrupt",
        "failed to open state database",
        "failed to begin state read session",
        "failed to load file states",
        "failed to initialize tables",
    ];

    let is_pipeline_error = pipeline_error_patterns
        .iter()
        .any(|pattern| error_string_lower.contains(pattern));

    if is_pipeline_error {
        return 2;
    }

    // User input error patterns (explicit matches - high precision)
    // These are errors where the user provided invalid input
    // Network/infrastructure errors have been REMOVED from here
    let user_input_patterns = [
        "must be",
        "cannot be",
        "missing",
        "required",
        "no such file",
        "file not found",
        "must be at least",
        "must be at most",
        "must be positive",
        "too long",
        "too short",
        "out of range",
        "query cannot be empty",
        "query too long",
        "limit must be",
        "another index operation appears to be running",
        "invalid config",
        "invalid or too complex regex",
        "regex parse error",
        "slow pattern",
        "redos risk",
        "permission denied",
        "no markdown files found",
        "cannot index empty",
        "parent path is not a directory",
        "validation failed",
        "query parse error",
        "invalid query",
        "invalid url",
        "source not found",
        "not found in",
        "does not exist",
        "regex queries not allowed",
        "redos",
    ];

    let is_user_input = user_input_patterns
        .iter()
        .any(|pattern| error_string_lower.contains(pattern));

    if is_user_input {
        // User input error -> exit 1
        return 1;
    }

    // "no results found" is a user input error — exit 1 so pipelines can detect it
    if error_string_lower.contains("no results found") {
        return 1;
    }

    // Pipeline error -> exit 2
    // These include: IO errors, transform failures, network errors, corrupt data
    // Anything that isn't a user input error is a pipeline error
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_error_to_exit_code_pipeline_network() {
        let err = anyhow::anyhow!("network error while connecting");
        assert_eq!(map_error_to_exit_code(&err), 2);
    }

    #[test]
    fn test_map_error_to_exit_code_pipeline_dns() {
        let err = anyhow::anyhow!("DNS lookup failed");
        assert_eq!(map_error_to_exit_code(&err), 2);
    }

    #[test]
    fn test_map_error_to_exit_code_pipeline_ssl() {
        let err = anyhow::anyhow!("ssl error: certificate expired");
        assert_eq!(map_error_to_exit_code(&err), 2);
    }

    #[test]
    fn test_map_error_to_exit_code_pipeline_connection_refused() {
        let err = anyhow::anyhow!("connection refused");
        assert_eq!(map_error_to_exit_code(&err), 2);
    }

    #[test]
    fn test_map_error_to_exit_code_pipeline_connection_timeout() {
        let err = anyhow::anyhow!("connection timed out");
        assert_eq!(map_error_to_exit_code(&err), 2);
    }

    #[test]
    fn test_map_error_to_exit_code_user_input_validation() {
        let err = anyhow::anyhow!("query cannot be empty");
        assert_eq!(map_error_to_exit_code(&err), 1);
    }

    #[test]
    fn test_map_error_to_exit_code_user_input_missing() {
        let err = anyhow::anyhow!("no such file or directory");
        assert_eq!(map_error_to_exit_code(&err), 1);
    }

    #[test]
    fn test_map_error_to_exit_code_file_not_found() {
        let err = anyhow::anyhow!("file not found: /path/to/file.txt");
        assert_eq!(map_error_to_exit_code(&err), 1);
    }

    #[test]
    fn test_map_error_to_exit_code_query_parse() {
        let err = anyhow::anyhow!("query parse error: invalid syntax");
        assert_eq!(map_error_to_exit_code(&err), 1);
    }

    #[test]
    fn test_map_error_to_exit_code_no_results() {
        let err = anyhow::anyhow!("no results found for 'test'");
        assert_eq!(map_error_to_exit_code(&err), 1);
    }

    #[test]
    fn test_map_error_to_exit_code_default_pipeline() {
        let err = anyhow::anyhow!("unexpected io error");
        assert_eq!(map_error_to_exit_code(&err), 2);
    }

    #[test]
    fn test_map_error_to_exit_case_insensitive() {
        let err = anyhow::anyhow!("NETWORK ERROR while connecting");
        assert_eq!(map_error_to_exit_code(&err), 2);

        let err = anyhow::anyhow!("Query Parse Error: invalid");
        assert_eq!(map_error_to_exit_code(&err), 1);
    }

    #[test]
    fn test_map_error_to_exit_code_domain_unreachable() {
        let err = anyhow::anyhow!("Domain unreachable: https://example.com");
        assert_eq!(map_error_to_exit_code(&err), 2);
    }

    #[test]
    fn test_map_error_to_exit_code_git_clone_failed() {
        let err = anyhow::anyhow!("Failed to clone repository: https://github.com/repo");
        assert_eq!(map_error_to_exit_code(&err), 2);
    }

    #[test]
    fn test_map_error_to_exit_code_no_pages_extracted() {
        let err = anyhow::anyhow!("No pages extracted from https://example.com");
        assert_eq!(map_error_to_exit_code(&err), 2);
    }

    #[test]
    fn test_map_error_to_exit_code_corrupt_database() {
        let err = anyhow::anyhow!(
            "failed to open state database at /path/to/state.redb: I/O error: invalid data"
        );
        assert_eq!(map_error_to_exit_code(&err), 2);
    }

    #[test]
    fn test_map_error_to_exit_code_failed_scrape() {
        let err = anyhow::anyhow!("Failed to scrape https://example.com");
        assert_eq!(map_error_to_exit_code(&err), 2);
    }

    #[test]
    fn test_map_error_to_exit_code_failed_begin_state_read_session() {
        let err = anyhow::anyhow!("failed to begin state read session: transaction error");
        assert_eq!(map_error_to_exit_code(&err), 2);
    }

    #[test]
    fn test_map_error_to_exit_code_failed_load_file_states() {
        let err = anyhow::anyhow!("failed to load file states: malformed row data");
        assert_eq!(map_error_to_exit_code(&err), 2);
    }

    #[test]
    fn test_map_error_to_exit_code_failed_initialize_tables() {
        let err = anyhow::anyhow!("failed to initialize tables: table already exists");
        assert_eq!(map_error_to_exit_code(&err), 2);
    }

    #[test]
    fn test_map_error_to_exit_code_invalid_url() {
        let err = anyhow::anyhow!("Invalid URL format: not-a-url");
        assert_eq!(map_error_to_exit_code(&err), 1);
    }

    #[test]
    fn test_map_error_to_exit_code_source_not_found() {
        let err = anyhow::anyhow!("Source not found: /path/to/source");
        assert_eq!(map_error_to_exit_code(&err), 1);
    }

    #[test]
    fn test_map_error_to_exit_code_not_found_in() {
        let err = anyhow::anyhow!("INDEX.json not found in /path/to/dir");
        assert_eq!(map_error_to_exit_code(&err), 1);
    }

    #[test]
    fn test_map_error_to_exit_code_does_not_exist() {
        let err = anyhow::anyhow!("compaction failed: file does not exist");
        assert_eq!(map_error_to_exit_code(&err), 1);
    }

    #[test]
    fn test_map_error_to_exit_code_regex_not_allowed() {
        let err = anyhow::anyhow!("Regex queries not allowed (potential ReDoS attack)");
        assert_eq!(map_error_to_exit_code(&err), 1);
    }
}
