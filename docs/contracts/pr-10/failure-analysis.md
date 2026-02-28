# Failure Analysis - PR #10

## Purpose
Failure-mode inventory for contracted functions, mapped to Red Queen issues and PR review feedback.

## Function-by-Function Failure Matrix

### 1) `validate_query` (`doc-nnly`)
- Invalid inputs -> error variant:
  - Empty or whitespace-only -> `ValidationError::EmptyQuery`
  - Null byte present anywhere -> `ValidationError::NullBytesNotAllowed`
  - Byte length > 1000 -> `ValidationError::QueryTooLong`
  - Regex-like forbidden pattern -> `ValidationError::RegexNotAllowed`
- External failures:
  - None (pure validation)
- Invariant breaks:
  - Any accepted query must be exactly trimmed user query, never silently mutated
- Edge cases:
  - Null at start, middle, end, repeated nulls
  - Multibyte queries close to byte limit

### 2) `run_search` (`doc-1vtz`)
- Invalid inputs -> error variant:
  - Too many words (>100) -> `SearchContractError::QueryTooManyTerms`
  - Missing `INDEX.json` -> `SearchContractError::IndexMissing`
- External failures:
  - Tantivy parse/search failure -> `SearchContractError::AdvancedSearchFailed`
  - JSON read/parse failure -> `SearchContractError::InvalidIndexJson`
- Invariant breaks:
  - If advanced search fails, command must not report full success
- Edge cases:
  - Advanced fails + fallback returns no results
  - Advanced fails + fallback returns some results
  - Advanced unavailable (no Tantivy index) should not be treated as advanced failure

### 3) `validate_url` (`doc-13x9`, review `discussion_r2866841648`)
- Invalid inputs -> error variant:
  - Empty/whitespace -> `UrlValidationError::EmptyUrl`
  - Contains space char -> `UrlValidationError::SpaceInUrl`
  - Contains unencoded `{ } | \\ ^ ` < >` or brackets outside IPv6 authority -> `UrlValidationError::UnencodedSpecialCharacter`
  - Parse error -> `UrlValidationError::InvalidUrlFormat`
  - Scheme not http/https -> `UrlValidationError::UnsupportedScheme`
  - Missing host -> `UrlValidationError::MissingHost`
  - Reparse of serialized URL fails -> `UrlValidationError::InvalidUrlEncoding`
- External failures:
  - URL parser internals (surface as parse/encoding variants)
- Invariant breaks:
  - Valid IPv6 literal host URLs must remain accepted
- Edge cases:
  - `https://[::1]:3000/docs` valid
  - `https://example.com/foo[bar]` invalid
  - Percent-encoded reserved chars valid

### 4) `discover_files` / `discover_single_file` (`doc-2mzo`, `doc-sgzo`)
- Invalid inputs -> error variant:
  - Missing source path -> `DiscoveryError::SourceNotFound`
  - Bad single-file path semantics -> `DiscoveryError::InvalidFilePath`
- External failures:
  - Canonicalization I/O error -> `DiscoveryError::CanonicalizationFailed`
  - Metadata read failure -> `DiscoveryError::MetadataReadFailed` or skip-with-warning policy
- Invariant breaks:
  - Markdown variant extensions must be discoverable
  - Non-markdown `.txt` must emit warning when processed
- Edge cases:
  - Mixed-case extension handling policy (documented behavior)
  - Empty directory returns empty manifest, not error

### 5) `discover_test_files` (test parity for `doc-sgzo`)
- Invalid inputs -> error variant:
  - Root path not strip-prefix-compatible -> propagated path error
- External failures:
  - Walkdir iteration I/O issues (skip with warning)
- Invariant breaks:
  - Test helper extension list must match production discovery extension list
- Edge cases:
  - Directories in exclude list embedded in deeper paths

### 6) `estimate_tokens` (`doc-3e4v`)
- Invalid inputs -> error variant:
  - None at API boundary (string accepted)
- External failures:
  - Tokenizer dict unavailable -> fallback estimator path
  - CoreBPE construction failure -> fallback estimator path
- Invariant breaks:
  - Token estimation must complete quickly and deterministically
- Edge cases:
  - Very long text
  - Concurrent calls from multiple threads (OnceLock correctness)

## DomainError Coverage Requirements
- Every variant listed in `type-contract.md` must have at least one explicit test.
- Every partial-success branch must assert non-zero process exit.
- URL bracket behavior must include both positive and negative tests to prevent IPv6 regression.

## Failure-to-Test Traceability
- `doc-nnly` -> null-byte rejection tests
- `doc-1vtz` -> partial failure exit-code tests
- `doc-13x9` -> malformed URL message and variant tests
- `discussion_r2866841648` -> IPv6 host-literal acceptance test
- `doc-2mzo` -> `.txt` warning tests
- `doc-sgzo` -> markdown-variant discovery tests
- `doc-3e4v` -> tokenizer initialization caching and non-hang tests
