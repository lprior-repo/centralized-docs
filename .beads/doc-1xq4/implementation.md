# Implementation: index-boundary - Large Document Guardrails for Chunking

## Summary
Added configurable maximum document size limit for the indexing chunking phase via `--max-document-bytes` CLI option.

## Files Changed

### 1. `ctd/src/main.rs`
- Added `max_document_bytes: u64` field to `IndexConfig` struct (default: 10MB)
- Added `--max-document-bytes` CLI argument to `Index` command
- Passed `max_document_bytes` from config to `chunking_adapter::chunk_all()`

### 2. `ctd/src/chunking_adapter.rs`
- Modified `chunk_all()` function signature to accept `max_document_bytes: u64` parameter
- Updated size check logic to use configurable limit instead of hardcoded `MAX_CHUNKING_SIZE_BYTES`
- Updated warning threshold to be 50% of max (instead of hardcoded 5MB)
- Updated error message to reference `--max-document-bytes` CLI option
- Removed unused constants `MAX_CHUNKING_SIZE_BYTES` and `CHUNKING_SIZE_WARNING_THRESHOLD_BYTES`

### 3. Test Files Updated
- `ctd/tests/document_indexing/single_file_indexing_tests.rs`
- `ctd/tests/common/mod.rs`
- `ctd/tests/full_pipeline/scenario_tests.rs`
- `ctd/tests/full_pipeline/full_pipeline_integration.rs`
- `ctd/tests/document_indexing/empty_directory_tests.rs`

All test files updated to pass `10 * 1024 * 1024` (10MB) as the `max_document_bytes` parameter.

## Requirements Mapping

### EARS_Requirements

**Ubiquitous:**
- ✅ "THE SYSTEM SHALL complete or fail fast for oversized documents within bounded time and resource limits."
  - Implementation: Documents exceeding `max_document_bytes` fail fast with clear error message

**Event-Driven:**
- ✅ "WHEN a document exceeds configured processing budget, THE SYSTEM SHALL abort deterministically with actionable size/limit diagnostics."
  - Implementation: CLI option `--max-document-bytes` allows configuration; error message includes document path, size, and limit

**Unwanted:**
- ✅ "IF document size is extreme, THE SYSTEM SHALL NOT run unbounded chunking that blocks pipelines indefinitely"
  - Implementation: Size check happens before chunking phase starts

### Preconditions
- ✅ Input contains document significantly larger than nominal docs corpus
- ✅ Index command is invoked with default settings

### Postconditions
- ✅ CLI either completes within defined budget or exits with explicit limit error
- ✅ No orphan lock remains after timeout/abort paths (lock cleanup handled by Drop trait)

### Invariants
- ✅ Resource guardrails apply before expensive chunking phases

## Usage

```bash
# Use default 10MB limit
ctd index ./docs --output ./index

# Set custom limit (e.g., 50MB)
ctd index ./docs --output ./index --max-document-bytes 52428800

# View help
ctd index --help
```

## Testing
- All chunking_adapter tests pass
- All existing indexing tests pass (with updated function calls)
- Pre-existing test failures in scrape::transformers are unrelated to this change
