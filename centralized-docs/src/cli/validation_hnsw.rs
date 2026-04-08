//! HNSW graph parameter validators for the index command.
//!
//! Parse as i64 first to properly detect and report negative numbers,
//! then validate range before converting to usize.

use anyhow::Result;

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
