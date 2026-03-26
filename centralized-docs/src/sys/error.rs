/// Map errors to exit codes per contract requirements:
///
/// - Exit 0: Success
/// - Exit 1: User input errors (invalid arguments, bad format, missing files)
/// - Exit 2: Pipeline/internal errors (transform failures, corrupt data, network errors)
///
/// This ensures consistent exit codes across all validation layers:
/// - Parser-level validation (via clap value_parser) now exits with 1 (user error)
/// - Runtime validation also exits with 1 for user input errors
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
    ];

    let is_pipeline_error = pipeline_error_patterns
        .iter()
        .any(|pattern| error_string_lower.contains(pattern));

    if is_pipeline_error {
        return 2;
    }

    // User input error patterns (explicit matches - high precision)
    // These are errors where the user provided invalid input
    let user_input_patterns = [
        "must be",
        "cannot be",
        "missing",
        "required",
        "not found",
        "no such file",
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
        "invalid url",
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
        "failed to clone repository",
        "git clone failed",
        "domain unreachable",
        "no pages extracted",
        "failed to scrape",
    ];

    let is_user_input = user_input_patterns
        .iter()
        .any(|pattern| error_string_lower.contains(pattern));

    if is_user_input {
        // User input error -> exit 1
        return 1;
    }

    // "no results found" is NOT an error - it's a valid result state
    // Exit code 0 means success (even with empty results)
    // Exit code 1 is for actual errors (invalid index, missing args, etc.)
    if error_string_lower.contains("no results found") {
        // No results is a valid result -> exit 0 (success)
        return 0;
    }

    // Pipeline error -> exit 2
    // These include: IO errors, transform failures, network errors, corrupt data
    // Anything that isn't a user input error is a pipeline error
    2
}
