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
        // URL mismatch errors (cdocs-1gr - FM-4)
        "does not match",
        "mismatch",
        // User abort patterns
        "apply aborted",
        "user aborted",
        "aborted by user",
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
#[path = "error_tests.rs"]
mod tests;
