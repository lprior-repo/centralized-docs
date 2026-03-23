use crate::validate;
use anyhow::Result;
use spider::configuration::RedirectPolicy;

// Validation functions for HNSW graph parameters
//
// Parse as i64 first to properly detect and report negative numbers,
// then validate range before converting to usize.

pub(crate) fn validate_max_related_chunks(s: &str) -> Result<usize, String> {
    let value = s
        .parse::<i64>()
        .map_err(|_| format!("max_related_chunks must be an integer, got '{s}'"))?;

    if value < 1 {
        return Err(format!("max_related_chunks must be at least 1, got '{s}'"));
    }
    if value > 100 {
        return Err(format!("max_related_chunks must be at most 100, got '{s}'"));
    }

    value
        .try_into()
        .map_err(|_| format!("max_related_chunks value too large: {value}"))
}

pub(crate) fn validate_max_chunk_keywords(s: &str) -> Result<usize, String> {
    let value = s
        .parse::<i64>()
        .map_err(|_| format!("max_chunk_keywords must be an integer, got '{s}'"))?;

    if value < 0 {
        return Err(format!("max_chunk_keywords must be at least 0, got '{s}'"));
    }
    if value > 50 {
        return Err(format!("max_chunk_keywords must be at most 50, got '{s}'"));
    }

    value
        .try_into()
        .map_err(|_| format!("max_chunk_keywords value too large: {value}"))
}

pub(crate) fn validate_hnsw_m(s: &str) -> Result<usize, String> {
    let value = s
        .parse::<i64>()
        .map_err(|_| format!("hnsw_m must be an integer, got '{s}'"))?;

    if value < 4 {
        return Err(format!(
            "hnsw_m must be at least 4 for proper connectivity, got '{s}'"
        ));
    }
    if value > 64 {
        return Err(format!(
            "hnsw_m must be at most 64 for reasonable performance, got '{s}'"
        ));
    }

    value
        .try_into()
        .map_err(|_| format!("hnsw_m value too large: {value}"))
}

pub(crate) fn validate_hnsw_ef_construction(s: &str) -> Result<usize, String> {
    let value = s
        .parse::<i64>()
        .map_err(|_| format!("hnsw_ef_construction must be an integer, got '{s}'"))?;

    if value < 50 {
        return Err(format!(
            "hnsw_ef_construction must be at least 50 for acceptable build quality, got '{s}'"
        ));
    }
    if value > 1000 {
        return Err(format!(
            "hnsw_ef_construction must be at most 1000 for reasonable build times, got '{s}'"
        ));
    }

    value
        .try_into()
        .map_err(|_| format!("hnsw_ef_construction value too large: {value}"))
}

/// Validate threshold value for BM25 filtering
///
/// BM25 scores range from 0.0 (no relevance) to positive values.
/// Negative thresholds are meaningless for BM25 and indicate user error.
/// Upper bound is set to 10.0 to allow for flexible filtering while preventing obvious errors.
pub fn validate_threshold(s: &str) -> Result<f32, String> {
    let value = s
        .parse::<f32>()
        .map_err(|_| format!("threshold must be a number, got '{s}'"))?;

    if !value.is_finite() {
        return Err(format!(
            "threshold must be a finite number between 0.0 and 10.0, got {value}"
        ));
    }

    if value < 0.0 {
        return Err(format!(
            "threshold must be non-negative (BM25 scores are >= 0.0), got {value}"
        ));
    }

    if value > 10.0 {
        return Err(format!(
            "threshold must be at most 10.0 for practical filtering, got {value}"
        ));
    }

    Ok(value)
}

/// Validate retry count (0-255 inclusive)
pub fn validate_retry_count(s: &str) -> Result<u32, String> {
    let value = s
        .parse::<i64>()
        .map_err(|_| format!("retry must be an integer, got '{s}'"))?;

    if value < 0 {
        return Err(format!(
            "retry must be non-negative (0 disables spider retry), got {value}"
        ));
    }

    if value > u8::MAX as i64 {
        return Err(format!("retry must be at most {}, got {value}", u8::MAX));
    }

    value
        .try_into()
        .map_err(|_| format!("retry value too large: {value}"))
}

/// Validate timeout seconds (1-600)
pub fn validate_timeout_secs(s: &str) -> Result<u64, String> {
    let value = s
        .parse::<i64>()
        .map_err(|_| format!("timeout must be an integer, got '{s}'"))?;

    if value < 1 {
        return Err("timeout must be at least 1 second".to_string());
    }

    if value > 600 {
        return Err(format!(
            "timeout must be at most 600 seconds (10 minutes), got {value}"
        ));
    }

    value
        .try_into()
        .map_err(|_| format!("timeout value too large: {value}"))
}

/// Validate positive byte limits (>=1)
pub fn validate_positive_bytes(s: &str) -> Result<u64, String> {
    let value = s
        .parse::<i64>()
        .map_err(|_| format!("bytes must be an integer, got '{s}'"))?;

    if value < 1 {
        return Err(format!("bytes must be at least 1, got {value}"));
    }

    value
        .try_into()
        .map_err(|_| format!("bytes value too large: {value}"))
}

/// Validate concurrency (1-2 inclusive)
pub fn validate_concurrency_limit(s: &str) -> Result<usize, String> {
    let value = s
        .parse::<i64>()
        .map_err(|_| format!("concurrency must be an integer, got '{s}'"))?;

    if value < 1 {
        return Err(format!("concurrency must be at least 1, got {value}"));
    }

    if value > 128 {
        return Err(format!(
            "concurrency must be at most 128 for safety, got {value}"
        ));
    }

    value
        .try_into()
        .map_err(|_| format!("concurrency value too large: {value}"))
}

/// Parse redirect policy (loose|strict|none)
pub fn parse_redirect_policy(s: &str) -> Result<RedirectPolicy, String> {
    match s.to_ascii_lowercase().as_str() {
        "loose" => Ok(RedirectPolicy::Loose),
        "strict" => Ok(RedirectPolicy::Strict),
        "none" => Ok(RedirectPolicy::None),
        other => Err(format!(
            "redirect policy must be one of: loose, strict, none (got '{other}')"
        )),
    }
}

/// Delay between HTTP requests in milliseconds.
/// Negative delays are meaningless and indicate user error.
/// Upper bound prevents impractically long delays.
pub fn validate_delay(s: &str) -> Result<u64, String> {
    let value = s
        .parse::<i64>()
        .map_err(|_| format!("delay must be an integer, got '{s}'"))?;

    if value < 0 {
        return Err(format!(
            "delay must be non-negative (milliseconds), got {value}"
        ));
    }

    if value > 60_000 {
        return Err(format!(
            "delay must be at most 60000 milliseconds (60 seconds), got {value}"
        ));
    }

    value
        .try_into()
        .map_err(|_| format!("delay value too large: {value}"))
}

/// CLI wrapper for validate_limit that returns String error for clap compatibility.
pub fn validate_limit_cli(s: &str) -> Result<usize, String> {
    validate::validate_limit(s).map_err(|e| e.to_string())
}

/// Validate and compile regex pattern with ReDoS protection (BEAD-004).
///
/// Implements safety measures against ReDoS attacks:
/// - Maximum pattern length (500 characters)
/// - Detection of known ReDoS patterns (nested quantifiers)
/// - Compilation size limits via RegexBuilder
///
/// Returns Ok(()) if pattern is safe and compiles successfully.
pub fn validate_filter_regex(pattern: &str) -> Result<(), String> {
    // BEAD-004: Reject patterns that are too long (use char count, not byte count)
    let char_count = pattern.chars().count();
    if char_count > 500 {
        return Err(format!(
            "Regex pattern too long: {} chars (max 500)",
            char_count
        ));
    }

    // BEAD-004: Check for known ReDoS patterns
    // Catches: (.*)* (.+)+ (a+)+ (\w+)+ ([a-z]+)+ (a|a)+ etc.
    let redos_patterns = [(r"\([^)]+\)[+*]", "nested quantifiers on groups")];

    let redos_match = redos_patterns.iter().find_map(|(pattern_re, description)| {
        regex::Regex::new(pattern_re).ok().and_then(|re| {
            re.is_match(pattern).then(|| {
                format!("Regex contains potentially slow pattern (ReDoS risk): {description}")
            })
        })
    });

    if let Some(error) = redos_match {
        return Err(error);
    }

    // BEAD-004: Compile with size limits to prevent excessive memory usage
    regex::RegexBuilder::new(pattern)
        .size_limit(1024 * 1024) // 1MB compiled size limit
        .dfa_size_limit(1024 * 1024) // 1MB DFA size limit
        .build()
        .map(|_| ())
        .map_err(|e| format!("Invalid or too complex regex pattern '{pattern}': {e}"))
}

#[cfg(test)]
#[path = "validation_tests_delay.rs"]
mod validation_tests_delay;

#[cfg(test)]
#[path = "validation_tests_threshold.rs"]
mod validation_tests_threshold;

#[cfg(test)]
#[path = "validation_tests_limit.rs"]
mod validation_tests_limit;

#[cfg(test)]
#[path = "validation_tests_regex.rs"]
mod validation_tests_regex;
