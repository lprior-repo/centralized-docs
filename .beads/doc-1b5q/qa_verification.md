# QA Verification - doc-1b5q

Date: 2026-03-01
Verifier: qa-enforcer
Status: FAIL

## Implementation Verification

Verified requested implementation exists in source:

1. `doc_transformer/src/config.rs`
   - `CategoryConfig::load_from_file` wraps YAML parse errors with sanitized message:
     - `invalid config: failed to parse YAML at '<path>'`
   - Location: `doc_transformer/src/config.rs:147`

2. `doc_transformer/src/main.rs`
   - `map_error_to_exit_code` user input patterns include `"invalid config"`.
   - Location: `doc_transformer/src/main.rs:1034`

## Acceptance Test Execution

### Test 1 - Exact acceptance command

Command:

```bash
doc_transformer index qa-fixtures/basic --output /tmp/test --category-config /etc/passwd
```

Actual output:

```text
======================================================================
DOC_TRANSFORMER v5.0 (Knowledge DAG + llms.txt)
======================================================================

[CONFIG] Graph Parameters:
  max_related_chunks: 20 (default: 20)
  max_chunk_keywords: 12 (default: 12)
  hnsw_m: 16 (default: 16)
  hnsw_ef_construction: 200 (default: 200)

[STEP 1] DISCOVER
Error: Source not found: qa-fixtures/basic
```

Exit code: `1`

Expected vs actual:
- Expected: command reaches category config parsing and emits `invalid config` without leaking file content.
- Actual: command fails earlier because `qa-fixtures/basic` is missing in this workspace.

### Test 2 - Equivalent repro to reach category-config parsing

Command:

```bash
doc_transformer index docs --output /tmp/test-doc-1b5q --category-config /etc/passwd
```

Actual output:

```text
======================================================================
DOC_TRANSFORMER v5.0 (Knowledge DAG + llms.txt)
======================================================================

[CONFIG] Graph Parameters:
  max_related_chunks: 20 (default: 20)
  max_chunk_keywords: 12 (default: 12)
  hnsw_m: 16 (default: 16)
  hnsw_ef_construction: 200 (default: 200)

[STEP 1] DISCOVER
  Found 8 files

[STEP 2] ANALYZE
Error: invalid type: string "root:x:0:0::/root:/usr/bin/bash ... aiops:x:1001:1001::/home/aiops:/bin/bash", expected struct CategoryConfig
```

Exit code: `2`

Expected vs actual:
- Expected: generic `invalid config` message, no leaked `/etc/passwd` content, non-zero user-error exit code.
- Actual:
  - Generic `invalid config` message is **not** shown.
  - `/etc/passwd` content is **leaked** in the error message.
  - Exit code is non-zero, but is `2` (pipeline) instead of expected user error `1`.

## Required Evidence Outcome

- Error includes `invalid config`: **NO** (failed)
- Exit code non-zero: **YES**
- Sensitive `/etc/passwd` content not shown: **NO** (failed; content leaked)

## Reproduction Steps

1. Run exact acceptance command:
   - `doc_transformer index qa-fixtures/basic --output /tmp/test --category-config /etc/passwd`
2. In this workspace, fixture path is missing, so run equivalent command with valid source:
   - `doc_transformer index docs --output /tmp/test-doc-1b5q --category-config /etc/passwd`
3. Observe leaked passwd-style line content in the error text and exit code `2`.

## Severity Assessment

- Severity: CRITICAL
- Rationale: Sensitive file contents are still exposed in CLI error output.
