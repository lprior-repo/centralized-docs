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

#[test]
fn test_map_error_to_exit_code_url_mismatch_does_not_match() {
    let err = anyhow::anyhow!(
        "Scrape manifest base_url ('https://kubernetes.io/docs/home/') does not match \
         apply target URL ('https://example.com'). \
         This scrape was produced for a different site."
    );
    let code = map_error_to_exit_code(&err);

    assert_eq!(
        code, 1,
        "URL mismatch error containing 'does not match' should be exit code 1 (user input error), \
         not {}. FM-4: Add 'does not match' or 'mismatch' to user_input_patterns.",
        code
    );
}

#[test]
fn test_map_error_to_exit_code_url_mismatch_keyword() {
    let cases = vec![
        "URL mismatch: scrape is for example.com but apply targets other.com",
        "Scrape mismatch: base_url does not match target",
        "Error: URL mismatch detected between scrape and apply target",
    ];

    for msg in cases {
        let err = anyhow::anyhow!(msg);
        let code = map_error_to_exit_code(&err);

        assert_eq!(
            code, 1,
            "Error containing 'mismatch' should be exit code 1, got {} for: {}",
            code, msg
        );
    }
}

#[test]
fn test_map_error_to_exit_code_url_mismatch_not_pipeline_error() {
    let err = anyhow::anyhow!(
        "Scrape for https://docs.example.com does not match apply target https://example.com"
    );
    let code = map_error_to_exit_code(&err);

    assert_ne!(
        code, 2,
        "URL mismatch should NEVER be classified as pipeline error (exit 2). \
         Got exit code {}. FM-4 fix required.",
        code
    );
}
