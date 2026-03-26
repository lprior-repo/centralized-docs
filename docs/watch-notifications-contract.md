# Contract Specification: Watch Notifications

## Context

- **Feature**: Webhook notification system for `ctd watch` change tracking
- **Domain terms**: `NotificationUri`, `NotificationConfig`, `NotificationPayload`, `NotificationBackend`, `ChangePlan`, `ChangeSummary`
- **Assumptions**:
  - The existing `watch.rs` module (`ChangePlan`, `ChangeSummary`, `PageChange`) is the source of truth for change data
  - Notifications are fire-and-forget: failures are logged but never block the watch command
  - Idempotency is enforced at the payload level: same `ChangePlan` produces same `NotificationPayload` (same `plan_hash`)
  - The `gh` CLI is available and authenticated when `github://` backend is used
  - HTTP timeouts default to 30 seconds; Slack and generic webhooks use `reqwest`
- **Open questions**:
  - Should notification config be persisted (e.g., `.ctd/notify.toml`) or CLI-only?
  - Should retry logic exist for transient HTTP failures, or is single-attempt sufficient for MVP?
  - Should `--notify` accept multiple URIs (comma-separated) for fan-out?

## Preconditions

| ID | Precondition |
|----|-------------|
| P1 | The notification URI string is non-empty and parses into a valid `NotificationUri` |
| P2 | The URI scheme is one of: `http`, `https`, `webhook`, `github` |
| P3 | For `webhook://` and `http(s)://` schemes, the host is a valid DNS name or IP |
| P4 | For `github://` scheme, the path matches `owner/repo` format |
| P5 | For `github://` scheme, `gh` CLI is installed and authenticated |
| P6 | A `ChangePlan` exists (produced by `compute_plan`) |
| P7 | Network is available for outbound HTTP requests (best-effort; failure is non-fatal) |

## Postconditions

| ID | Postcondition |
|----|--------------|
| Q1 | If `plan.summary.is_empty()` == true, NO notification is sent |
| Q2 | If `plan.summary.is_empty()` == false, a notification IS sent to the configured backend |
| Q3 | The `NotificationPayload` contains: `target_url`, `timestamp`, `summary`, `changes`, `plan_hash` |
| Q4 | The `plan_hash` is deterministic: same `ChangePlan` bytes always produce same hash |
| Q5 | After notification dispatch, the watch command continues to completion regardless of notification success/failure |
| Q6 | Notification failures produce a warning log line, NOT a non-zero exit code |

## Invariants

| ID | Invariant |
|----|-----------|
| I1 | **Idempotency**: Sending the same `NotificationPayload` twice produces the same external effect (no duplicate Slack messages for identical plans) |
| I2 | **Non-blocking**: Notification dispatch never changes the exit code of `ctd watch` |
| I3 | **Deterministic payload**: `NotificationPayload::from_plan(plan_a) == NotificationPayload::from_plan(plan_b)` when `plan_a` and `plan_b` have identical content |
| I4 | **Type safety**: The `NotificationUri` newtype prevents passing raw strings where a validated URI is required |
| I5 | **Backend isolation**: Each `NotificationBackend` variant has exactly the fields it needs; no optional fields shared across variants |

## Error Taxonomy

```rust
/// All notification errors. None of these should propagate to the watch command's
/// exit code — they are logged and swallowed.
#[derive(Debug, thiserror::Error)]
pub enum NotificationError {
    /// The notification URI string is empty or whitespace-only.
    #[error("Notification URI cannot be empty")]
    EmptyUri,

    /// The URI scheme is not one of: http, https, webhook, github.
    #[error("Unsupported notification scheme: '{scheme}'. Expected: http, https, webhook, github")]
    UnsupportedScheme { scheme: String },

    /// The URI is malformed and cannot be parsed.
    #[error("Malformed notification URI: {reason}")]
    MalformedUri { reason: String },

    /// For github:// URIs, the path doesn't match owner/repo.
    #[error("Invalid GitHub repository path: '{path}'. Expected format: owner/repo")]
    InvalidGitHubRepo { path: String },

    /// The HTTP request to the webhook URL failed (network error, DNS failure, timeout).
    #[error("Webhook delivery failed for {url}: {source}")]
    DeliveryFailed {
        url: String,
        source: reqwest::Error,
    },

    /// The webhook endpoint returned a non-2xx status code.
    #[error("Webhook returned HTTP {status} for {url}: {body}")]
    HttpError {
        url: String,
        status: u16,
        body: String,
    },

    /// The `gh` CLI is not installed or not authenticated.
    #[error("GitHub CLI (gh) not available: {reason}")]
    GhCliUnavailable { reason: String },

    /// The `gh issue create` command failed.
    #[error("GitHub issue creation failed: {stderr}")]
    GhCommandFailed { stderr: String },

    /// JSON serialization of the notification payload failed.
    #[error("Failed to serialize notification payload: {source}")]
    SerializationFailed {
        source: serde_json::Error,
    },
}
```

## Data Types

### `NotificationUri` — Validated URI newtype

```rust
/// A validated notification target URI. Prevents passing raw strings
/// where a validated URI is required (type safety invariant I4).
///
/// # Contract
/// - **Preconditions**: input is non-empty, scheme is supported
/// - **Postconditions**: inner value is a valid URI string
/// - **Invariant**: parsing is infallible after construction
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NotificationUri(String);

impl NotificationUri {
    /// Parse a raw URI string into a validated `NotificationUri`.
    ///
    /// # Errors
    /// Returns `NotificationError::EmptyUri` if input is empty/whitespace.
    /// Returns `NotificationError::UnsupportedScheme` if scheme is not recognized.
    /// Returns `NotificationError::MalformedUri` if the URI cannot be parsed.
    pub fn parse(raw: &str) -> Result<Self, NotificationError>;

    /// Get the scheme component (e.g., "http", "webhook", "github").
    #[must_use]
    pub fn scheme(&self) -> &str;

    /// Get the underlying URI string.
    #[must_use]
    pub fn as_str(&self) -> &str;
}
```

### `NotificationBackend` — Discriminated backend enum

```rust
/// The notification delivery backend. Each variant carries exactly
/// the configuration its backend needs (invariant I5: no shared optional fields).
///
/// # Contract
/// - **Postconditions**: constructed only via `NotificationConfig::from_uri()`
/// - **Invariant**: each variant is self-contained
#[derive(Debug, Clone)]
pub enum NotificationBackend {
    /// POST JSON to an arbitrary HTTP(S) endpoint.
    Http {
        url: String,
        /// Optional headers to include (e.g., Authorization).
        headers: BTreeMap<String, String>,
    },
    /// POST to a Slack Incoming Webhook with Block Kit format.
    Slack {
        webhook_url: String,
    },
    /// Create a GitHub Issue via `gh` CLI.
    GitHub {
        owner: String,
        repo: String,
    },
    // Email is future — not MVP.
}
```

### `NotificationConfig` — Parsed configuration

```rust
/// Fully resolved notification configuration, ready for dispatch.
///
/// # Contract
/// - **Preconditions**: `uri` is a validated `NotificationUri`
/// - **Postconditions**: `backend` matches the URI scheme
/// - **Invariant**: `backend` variant is determined solely by `uri.scheme()`
#[derive(Debug, Clone)]
pub struct NotificationConfig {
    /// The validated source URI.
    pub uri: NotificationUri,
    /// The resolved backend to dispatch to.
    pub backend: NotificationBackend,
    /// Optional custom headers for HTTP backends.
    pub custom_headers: BTreeMap<String, String>,
    /// Dispatch timeout in seconds (default: 30).
    pub timeout_secs: u64,
}

impl NotificationConfig {
    /// Parse a `NotificationUri` into a fully resolved config.
    ///
    /// # Errors
    /// Returns `NotificationError::InvalidGitHubRepo` for github:// URIs with bad paths.
    pub fn from_uri(uri: NotificationUri) -> Result<Self, NotificationError>;

    /// Parse a raw URI string into a config (convenience wrapper).
    ///
    /// # Errors
    /// Returns any `NotificationError` from URI parsing or config resolution.
    pub fn parse(raw: &str) -> Result<Self, NotificationError>;
}
```

### `NotificationPayload` — The dispatch envelope

```rust
/// The deterministic payload sent to notification backends.
///
/// # Contract
/// - **Preconditions**: constructed only from a non-empty `ChangePlan`
/// - **Postconditions**: `plan_hash` is deterministic (same plan = same hash)
/// - **Invariant**: serializes to identical JSON for identical plans (I3)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NotificationPayload {
    /// The documentation URL being watched.
    pub target_url: String,
    /// When the change plan was generated (RFC 3339).
    pub timestamp: String,
    /// Summary statistics.
    pub summary: NotificationSummary,
    /// List of individual changes (capped at 50 for Slack; full list in JSON).
    pub changes: Vec<NotificationChangeEntry>,
    /// Deterministic hash of the full ChangePlan for idempotency checks (I1).
    pub plan_hash: String,
}

/// Slimmed-down summary for notification payloads.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NotificationSummary {
    pub added: usize,
    pub removed: usize,
    pub modified: usize,
    pub unchanged: usize,
    pub total: usize,
}

/// A single change entry in the notification payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NotificationChangeEntry {
    pub url: String,
    pub kind: String,  // "added" | "removed" | "modified"
    pub title: String,
}

impl NotificationPayload {
    /// Build a payload from a `ChangePlan`. Pure calculation.
    ///
    /// # Contract
    /// - **Preconditions**: `plan` is a valid `ChangePlan`
    /// - **Postconditions**: `plan_hash` is deterministic; `changes` are sorted
    /// - **Invariant**: `from_plan(p).plan_hash == from_plan(p).plan_hash` (I3)
    #[must_use]
    pub fn from_plan(plan: &ChangePlan) -> Self;
}
```

## Contract Signatures

### Pure Calculations (no I/O)

```rust
/// Parse a raw URI string into a validated `NotificationUri`.
///
/// # Contract
/// - **Precondition**: `raw` is non-empty
/// - **Postcondition**: returns `Ok(uri)` with valid scheme, or `Err` with semantic reason
/// - **Invariant**: deterministic — same input always produces same result
pub fn parse_notification_uri(raw: &str) -> Result<NotificationUri, NotificationError>;

/// Build a `NotificationConfig` from a validated URI.
///
/// # Contract
/// - **Precondition**: `uri` is a valid `NotificationUri`
/// - **Postcondition**: `config.backend` matches `uri.scheme()`
pub fn resolve_notification_config(uri: NotificationUri) -> Result<NotificationConfig, NotificationError>;

/// Build a `NotificationPayload` from a `ChangePlan`.
///
/// # Contract
/// - **Precondition**: `plan` is a valid `ChangePlan`
/// - **Postcondition**: payload contains deterministic `plan_hash`
/// - **Invariant**: idempotent — same plan produces same payload
#[must_use]
pub fn build_notification_payload(plan: &ChangePlan) -> NotificationPayload;
```

### I/O Actions (side effects at boundary)

```rust
/// Dispatch a notification to the configured backend.
///
/// # Contract
/// - **Precondition**: `config` is valid, `payload` is non-empty
/// - **Postcondition**: notification delivered (best-effort), or error logged
/// - **Invariant**: this function never panics; all errors are returned (I2)
///
/// # Error Handling
/// Network failures return `Err(NotificationError::DeliveryFailed)` but callers
/// MUST treat this as a warning, not a fatal error.
pub async fn dispatch_notification(
    config: &NotificationConfig,
    payload: &NotificationPayload,
) -> Result<(), NotificationError>;

/// Send notification if the change plan has changes.
///
/// # Contract
/// - **Precondition**: `config` is valid, `plan` is a valid `ChangePlan`
/// - **Postcondition**: if `plan.summary.is_empty()`, returns `Ok(None)` (Q1)
/// - **Postcondition**: if changes exist, dispatches and returns `Ok(Some(()))` (Q2)
/// - **Invariant**: never changes the watch command's exit code (I2)
pub async fn notify_if_changed(
    config: &NotificationConfig,
    plan: &ChangePlan,
) -> Result<Option<()>, NotificationError>;
```

### Backend-Specific Dispatchers

```rust
/// POST JSON payload to an HTTP(S) webhook URL.
///
/// # Contract
/// - **Precondition**: `url` is a valid HTTP(S) URL
/// - **Postcondition**: HTTP 2xx response, or `Err(DeliveryFailed | HttpError)`
async fn dispatch_http(
    url: &str,
    headers: &BTreeMap<String, String>,
    payload: &NotificationPayload,
    timeout_secs: u64,
) -> Result<(), NotificationError>;

/// POST Slack Block Kit payload to an Incoming Webhook URL.
///
/// # Contract
/// - **Precondition**: `webhook_url` is a valid Slack webhook URL
/// - **Postcondition**: Slack API returns 200, or `Err(DeliveryFailed | HttpError)`
async fn dispatch_slack(
    webhook_url: &str,
    payload: &NotificationPayload,
    timeout_secs: u64,
) -> Result<(), NotificationError>;

/// Create a GitHub Issue via `gh` CLI.
///
/// # Contract
/// - **Precondition**: `gh` is installed and authenticated (P5)
/// - **Postcondition**: issue created in `owner/repo`, or `Err(GhCliUnavailable | GhCommandFailed)`
async fn dispatch_github(
    owner: &str,
    repo: &str,
    payload: &NotificationPayload,
) -> Result<(), NotificationError>;
```

## CLI Integration

### New flag on `ctd watch`

```
ctd watch <URL> --notify <URI>
```

The `--notify` flag accepts:

| URI Pattern | Backend | Example |
|------------|---------|---------|
| `http://...` or `https://...` | HTTP webhook (POST JSON) | `--notify http://my-server.com/webhook` |
| `webhook://hooks.slack.com/...` | Slack Incoming Webhook | `--notify webhook://hooks.slack.com/services/T.../B.../xxx` |
| `github://owner/repo` | GitHub Issue via `gh` CLI | `--notify github://acme/docs-tracker` |

### Wiring in `cmd/watch.rs`

```rust
pub async fn run_watch(
    url: &str,
    output: &Path,
    cache_path: &Path,
    // ... existing params ...
    notify: Option<&str>,  // NEW: raw notification URI string
) -> Result<()> {
    // ... existing scrape + diff logic ...

    // ── Notification: best-effort, non-blocking ──────────────────
    if let Some(notify_uri) = notify {
        match NotificationConfig::parse(notify_uri) {
            Ok(config) => {
                match notify_if_changed(&config, &plan).await {
                    Ok(Some(())) => eprintln!("[NOTIFY] Notification sent"),
                    Ok(None) => {} // No changes, no notification
                    Err(e) => eprintln!("[NOTIFY] Warning: {e}"),
                }
            }
            Err(e) => eprintln!("[NOTIFY] Warning: invalid notification URI: {e}"),
        }
    }

    // ... existing exit logic ...
}
```

## Non-goals

- Email notifications (future; not MVP)
- Authentication tokens stored on disk (use environment variables or `gh` CLI auth)
- Retry logic for transient failures (single attempt, log and continue)
- Batching multiple watch cycles into one notification
- Webhook signature verification (HMAC) for inbound payloads
- Rate limiting or backoff for notification endpoints
