use super::*;
use proptest::prelude::*;

// -----------------------------------------------------------------------
// Test Helpers
// -----------------------------------------------------------------------

/// Create a valid `ScrapeArtifact` with non-default values.
fn make_artifact(url: &str, payload: &[u8]) -> ScrapeArtifact {
    ScrapeArtifact {
        content_hash: hash_payload(url.as_bytes()),
        status_code: 200,
        payload_bytes: payload.to_vec(),
    }
}

/// Create a `ScrapeArtifact` with a specific content hash.
fn make_artifact_with_content_hash(content_hash: [u8; 32], payload: &[u8]) -> ScrapeArtifact {
    ScrapeArtifact {
        content_hash,
        status_code: 200,
        payload_bytes: payload.to_vec(),
    }
}

/// Create a `ScrapeArtifact` with a specific status code.
fn make_artifact_with_status(status_code: u16, payload: &[u8]) -> ScrapeArtifact {
    ScrapeArtifact {
        content_hash: [0x42; 32],
        status_code,
        payload_bytes: payload.to_vec(),
    }
}

/// Create a `ScrapeOutputs` with artifacts for the given URLs.
fn make_scrape_outputs(urls: &[&str]) -> ScrapeOutputs {
    let mut artifacts = HashMap::new();
    for &url in urls {
        artifacts.insert(
            url.to_string(),
            make_artifact(url, format!("payload_for_{url}").as_bytes()),
        );
    }
    ScrapeOutputs { artifacts }
}

fn make_config(now_secs: u64) -> ScrapeBatchConfig {
    ScrapeBatchConfig { now_secs }
}

mod edge_cases;
mod errors;
mod field_fidelity;
mod happy_path;
mod proptests;
mod url_state_raw;
