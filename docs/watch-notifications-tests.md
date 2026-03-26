# Martin Fowler Test Plan: Watch Notifications

## Happy Path Tests

### `test_parse_notification_uri_http_scheme`
Given: A valid HTTP URL string `"http://example.com/webhook"`
When: `parse_notification_uri` is called
Then: Returns `Ok(NotificationUri)` with `scheme() == "http"`

### `test_parse_notification_uri_https_scheme`
Given: A valid HTTPS URL string `"https://example.com/webhook"`
When: `parse_notification_uri` is called
Then: Returns `Ok(NotificationUri)` with `scheme() == "https"`

### `test_parse_notification_uri_webhook_scheme`
Given: A webhook URI `"webhook://hooks.slack.com/services/T.../B.../xxx"`
When: `parse_notification_uri` is called
Then: Returns `Ok(NotificationUri)` with `scheme() == "webhook"`

### `test_parse_notification_uri_github_scheme`
Given: A GitHub URI `"github://acme/docs-tracker"`
When: `parse_notification_uri` is called
Then: Returns `Ok(NotificationUri)` with `scheme() == "github"`

### `test_resolve_config_http_backend`
Given: A valid `NotificationUri` with HTTP scheme
When: `resolve_notification_config` is called
Then: Returns config with `backend == NotificationBackend::Http { url, headers }`

### `test_resolve_config_slack_backend`
Given: A valid `NotificationUri` with `webhook://` scheme
When: `resolve_notification_config` is called
Then: Returns config with `backend == NotificationBackend::Slack { webhook_url }`

### `test_resolve_config_github_backend`
Given: A valid `NotificationUri` with `github://owner/repo` format
When: `resolve_notification_config` is called
Then: Returns config with `backend == NotificationBackend::GitHub { owner, repo }`

### `test_build_payload_from_plan_with_changes`
Given: A `ChangePlan` with 2 added, 1 modified, 0 removed
When: `build_notification_payload` is called
Then: Returns payload with `summary.added == 2`, `summary.modified == 1`, `summary.removed == 0`, `changes.len() == 3`

### `test_build_payload_from_empty_plan`
Given: A `ChangePlan` with zero changes
When: `build_notification_payload` is called
Then: Returns payload with all summary counts at 0 and `changes.is_empty()`

### `test_notify_if_changed_sends_when_changes_exist`
Given: A valid `NotificationConfig` and a `ChangePlan` with changes
When: `notify_if_changed` is called
Then: Returns `Ok(Some(()))` and the backend receives the payload

### `test_notify_if_changed_skips_when_no_changes`
Given: A valid `NotificationConfig` and a `ChangePlan` with zero changes
When: `notify_if_changed` is called
Then: Returns `Ok(None)` and no HTTP request is made

## Error Path Tests

### `test_parse_notification_uri_empty_string`
Given: An empty string `""`
When: `parse_notification_uri` is called
Then: Returns `Err(NotificationError::EmptyUri)`

### `test_parse_notification_uri_whitespace_only`
Given: A whitespace-only string `"   "`
When: `parse_notification_uri` is called
Then: Returns `Err(NotificationError::EmptyUri)`

### `test_parse_notification_uri_unsupported_scheme`
Given: A URI with unsupported scheme `"ftp://example.com/path"`
When: `parse_notification_uri` is called
Then: Returns `Err(NotificationError::UnsupportedScheme { scheme: "ftp" })`

### `test_parse_notification_uri_malformed`
Given: A string that looks like a URI but is malformed `"http://"`
When: `parse_notification_uri` is called
Then: Returns `Err(NotificationError::MalformedUri { .. })`

### `test_resolve_config_github_invalid_repo_path`
Given: A `NotificationUri` with `github://just-a-name` (no slash)
When: `resolve_notification_config` is called
Then: Returns `Err(NotificationError::InvalidGitHubRepo { path: "just-a-name" })`

### `test_resolve_config_github_empty_owner`
Given: A `NotificationUri` with `github:///repo` (empty owner)
When: `resolve_notification_config` is called
Then: Returns `Err(NotificationError::InvalidGitHubRepo { .. })`

### `test_dispatch_http_returns_delivery_failed_on_network_error`
Given: A config pointing to a non-existent host
When: `dispatch_http` is called
Then: Returns `Err(NotificationError::DeliveryFailed { .. })`

### `test_dispatch_http_returns_http_error_on_4xx`
Given: A config pointing to a mock server returning HTTP 403
When: `dispatch_http` is called
Then: Returns `Err(NotificationError::HttpError { status: 403, .. })`

### `test_dispatch_http_returns_http_error_on_5xx`
Given: A config pointing to a mock server returning HTTP 500
When: `dispatch_http` is called
Then: Returns `Err(NotificationError::HttpError { status: 500, .. })`

### `test_dispatch_github_returns_unavailable_when_gh_missing`
Given: A GitHub config and `gh` CLI not on PATH
When: `dispatch_github` is called
Then: Returns `Err(NotificationError::GhCliUnavailable { .. })`

### `test_dispatch_github_returns_command_failed_on_error`
Given: A GitHub config, `gh` installed, but `gh issue create` exits non-zero
When: `dispatch_github` is called
Then: Returns `Err(NotificationError::GhCommandFailed { stderr: .. })`

## Edge Case Tests

### `test_parse_notification_uri_with_custom_headers`
Given: A URI with query params for headers `"https://example.com/hook?auth=Bearer+xxx"`
When: Parsed and resolved
Then: Custom headers are extracted and included in the config

### `test_build_payload_preserves_change_ordering`
Given: A `ChangePlan` with changes in mixed order (modified, added, removed)
When: `build_notification_payload` is called
Then: `changes` in payload are sorted by kind then URL (same as `ChangePlan.changes`)

### `test_build_payload_deterministic_hash`
Given: Two `ChangePlan` instances with identical content
When: `build_notification_payload` is called on each
Then: Both payloads have identical `plan_hash` values

### `test_payload_serialization_roundtrip`
Given: A `NotificationPayload` built from a `ChangePlan`
When: Serialized to JSON and deserialized back
Then: The deserialized payload equals the original

### `test_dispatch_with_empty_custom_headers`
Given: A config with no custom headers
When: `dispatch_http` is called
Then: Request is sent with only Content-Type header (no Authorization)

### `test_github_uri_with_hyphenated_owner_and_repo`
Given: `"github://my-org/my-repo.name"`
When: Resolved
Then: `owner == "my-org"`, `repo == "my-repo.name"`

### `test_slack_payload_exceeds_block_limit`
Given: A `ChangePlan` with 200+ changes
When: `dispatch_slack` formats the payload
Then: Changes are capped at 50 entries with a "... and N more" suffix

## Contract Verification Tests

### `test_postcondition_q1_no_notification_when_no_changes`
Given: A valid config and an empty `ChangePlan`
When: `notify_if_changed` is called
Then: Returns `Ok(None)` — postcondition Q1 verified (no notification sent)

### `test_postcondition_q2_notification_sent_when_changes`
Given: A valid config and a `ChangePlan` with changes
When: `notify_if_changed` is called
Then: Returns `Ok(Some(()))` — postcondition Q2 verified (notification dispatched)

### `test_postcondition_q3_payload_fields`
Given: A `ChangePlan` with known values
When: `build_notification_payload` is called
Then: Payload contains `target_url`, `timestamp`, `summary`, `changes`, `plan_hash` — Q3 verified

### `test_postcondition_q4_deterministic_plan_hash`
Given: Two identical `ChangePlan` instances
When: `build_notification_payload` is called on each
Then: `plan_hash` values are identical — Q4 verified

### `test_invariant_i1_idempotent_payload`
Given: A `NotificationPayload`
When: Serialized to JSON twice
Then: Both serializations are byte-identical — I1 verified (idempotency)

### `test_invariant_i2_non_blocking_error_handling`
Given: A valid config pointing to an unreachable host
When: `notify_if_changed` is called from within `run_watch`
Then: The function returns `Err` but `run_watch` logs a warning and exits 0 — I2 verified

### `test_invariant_i3_deterministic_payload`
Given: `plan_a` and `plan_b` with identical content but different timestamps
When: `build_notification_payload` is called on each
Then: `payload_a.plan_hash == payload_b.plan_hash` — I3 verified

### `test_invariant_i4_type_safety_prevents_raw_strings`
Given: The type system
When: Attempting to pass a raw `&str` to `dispatch_notification`
Then: Compilation fails — `NotificationConfig` is required, not `&str` — I4 verified (compile-time)

### `test_invariant_i5_backend_isolation`
Given: A `NotificationBackend::Slack` variant
When: Accessing fields
Then: Only `webhook_url` is available; no `owner`/`repo`/`headers` fields exist — I5 verified (compile-time)

## Given-When-Then Scenarios

### Scenario 1: Full HTTP webhook notification flow
Given:
- `ctd watch https://docs.example.com --notify https://hooks.example.com/ctd`
- The documentation has 3 new pages since last snapshot
- The webhook endpoint returns HTTP 200

When: `run_watch` executes through scrape, diff, and notification

Then:
- `compute_plan` produces a plan with `summary.added == 3`
- `notify_if_changed` dispatches the payload
- HTTP POST is sent to `https://hooks.example.com/ctd`
- Request body contains JSON with `target_url`, `timestamp`, `summary`, `changes`, `plan_hash`
- `run_watch` exits with code 1 (changes detected) regardless of notification success

### Scenario 2: Slack notification with no changes
Given:
- `ctd watch https://docs.example.com --notify webhook://hooks.slack.com/services/T.../B.../xxx`
- The documentation has NOT changed since last snapshot

When: `run_watch` executes

Then:
- `compute_plan` produces an empty plan
- `notify_if_changed` returns `Ok(None)`
- No HTTP request is made to Slack
- `run_watch` exits with code 0 (no changes)

### Scenario 3: GitHub issue creation with network failure
Given:
- `ctd watch https://docs.example.com --notify github://acme/docs-tracker`
- The documentation has 1 modified page
- `gh` CLI is installed and authenticated
- `gh issue create` succeeds

When: `run_watch` executes

Then:
- `notify_if_changed` calls `dispatch_github`
- `gh issue create` is invoked with title and body derived from the change plan
- `run_watch` exits with code 1 (changes detected)

### Scenario 4: Notification failure is non-fatal
Given:
- `ctd watch https://docs.example.com --notify https://unreachable.invalid/webhook`
- The documentation has changes

When: `run_watch` executes and the webhook fails

Then:
- `dispatch_notification` returns `Err(DeliveryFailed)`
- A warning is printed to stderr: `[NOTIFY] Warning: Webhook delivery failed for https://unreachable.invalid/webhook: ...`
- `run_watch` continues and exits with code 1 (changes detected, NOT notification failure)

### Scenario 5: Invalid notification URI is caught early
Given:
- `ctd watch https://docs.example.com --notify "not a uri"`

When: `run_watch` starts

Then:
- `NotificationConfig::parse` returns `Err(EmptyUri | MalformedUri | UnsupportedScheme)`
- A warning is printed to stderr
- Watch continues without notification
- Exit code is determined solely by whether changes exist

### Scenario 6: Idempotent re-notification
Given:
- `ctd watch` is run twice against unchanged documentation
- Both runs produce the same `ChangePlan`

When: `build_notification_payload` is called in both runs

Then:
- Both payloads have identical `plan_hash`
- If using a backend that checks `plan_hash`, the second notification is suppressed
