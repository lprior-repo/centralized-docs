# Contract: index-boundary - Large Document Guardrails for Chunking

## Overview
Add guardrails to prevent unbounded chunking operations that can block pipelines indefinitely when processing oversized documents.

## Problem Statement
The `index` command can encounter documents significantly larger than the nominal corpus. Without proper guardrails:
- Very large documents can cause unbounded chunking that blocks pipelines
- No configurable budget for processing large documents
- Operators receive unclear errors when limits are exceeded

## Requirements

### EARS_Requirements

**Ubiquitous:**
- THE SYSTEM SHALL complete or fail fast for oversized documents within bounded time and resource limits.

**Event-Driven:**
- WHEN a document exceeds configured processing budget, THE SYSTEM SHALL abort deterministically with actionable size/limit diagnostics.

**Unwanted:**
- IF document size is extreme, THE SYSTEM SHALL NOT run unbounded chunking that blocks pipelines indefinitely, because: long-running unbounded operations cause deadlocks and operator timeouts.

### Preconditions
- Input contains document significantly larger than nominal docs corpus
- Index command is invoked with default settings

### Postconditions
- CLI either completes within defined budget or exits with explicit limit error
- No orphan lock remains after timeout/abort paths

### Invariants
- Resource guardrails apply before expensive chunking phases

## Implementation Details

### Existing Patterns
1. `scrape` command already has `--max-page-bytes` and `--max-total-bytes` CLI options
2. `chunking_adapter.rs` has `MAX_CHUNKING_SIZE_BYTES` (10MB) and `CHUNKING_SIZE_WARNING_THRESHOLD_BYTES` (5MB) constants
3. `discover.rs` has `MAX_SOURCE_FILE_BYTES` (50MB)

### Missing Functionality
1. No CLI option to configure max document size for indexing
2. No timeout/budget mechanism for chunking phase
3. Hard-coded limits not exposed for configuration

### Solution
Add CLI options `--max-document-bytes` to the `index` command with sensible defaults:
- Default: 10MB (matching existing MAX_CHUNKING_SIZE_BYTES)
- Warning threshold: 5MB
- Ensure proper cleanup on abort (lock file removal)

### Error Messages
Provide actionable diagnostics when limits are exceeded:
- Document path and size
- Current limit value
- Suggestion to split document or increase limit

## Acceptance Tests

### Happy Path
- Normal documents index successfully
- Documents under default limit succeed
- Warning is shown for large documents (5-10MB) but processing continues

### Error Paths  
- Document > 10MB: Error with clear message about size limit
- Lock cleanup: No orphan locks after any abort path

## Research Notes
- Read `doc_transformer/src/main.rs` for CLI patterns
- Read `doc_transformer/src/chunking_adapter.rs` for existing size checks
- Read `doc_transformer/src/discover.rs` for file size handling
