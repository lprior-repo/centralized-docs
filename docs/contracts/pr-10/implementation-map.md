# Implementation Map - PR #10 Contract Resolution

## Goal
Provide exact implementation guidance for aligning PR #10 with contract-first behavior and QA verification.

## Files to Modify (Implementation)
- `doc_transformer/src/validate.rs`
  - Keep `ValidationError::NullBytesNotAllowed`
  - Ensure `validate_query` remains single boundary parser for search query safety

- `doc_transformer/src/main.rs`
  - Keep `run_search` partial-failure signaling behavior
  - Introduce explicit internal outcome enum if not present (`SearchOutcome`)
  - Ensure process exit mapping treats partial failure as non-zero

- `doc_transformer/src/scrape/validation.rs`
  - Keep URL pre-parse checks but preserve IPv6 literal behavior in authority
  - Keep helper functions (`find_unencoded_special_char`, authority bounds parsing)
  - Prefer typed error conversion over raw `anyhow::bail!` strings over time

- `doc_transformer/src/discover.rs`
  - Keep extension list parity with contract (`.md`, `.mdx`, `.markdown`, `.mdown`, `.mkd`, `.rst`, `.txt`)
  - Emit warning on `.txt` processing

- `doc_transformer/src/filter.rs`
  - Keep integration-test helper extension list in sync with production discovery

- `contextual-chunker/src/chunk.rs`
  - Keep tokenizer caching (`OnceLock`) path and fallback estimator
  - Add/retain tests proving bounded execution under repeated calls

## Files to Add or Update (Specs and Tests)
- Added now:
  - `docs/contracts/pr-10/type-contract.md`
  - `docs/contracts/pr-10/failure-analysis.md`
  - `docs/contracts/pr-10/bdd-test-plan.md`
  - `docs/contracts/pr-10/implementation-map.md`

- Test files to update/create for parity:
  - `doc_transformer/src/scrape/validation.rs` unit tests for IPv6 and special-char boundary behavior
  - `doc_transformer/src/validate.rs` null-byte rejection tests (start/middle/end/multiple)
  - `doc_transformer/tests/search_adversarial.rs` and/or CLI integration tests for partial-failure exit semantics
  - `doc_transformer/src/discover.rs` tests for markdown extension variants and `.txt` warning behavior
  - `doc_transformer/src/filter.rs` test-helper parity tests
  - `contextual-chunker/src/chunk.rs` stress/non-hang tests for `estimate_tokens`

## Exact Signatures to Preserve or Introduce
- Preserve:
  - `pub fn validate_query(query: &str) -> Result<&str, ValidationError>`
  - `fn run_search(query: &str, index_dir: &Path, limit: usize, _use_color: bool) -> Result<()>`
  - `pub fn validate_url(url: &str) -> Result<url::Url>`
  - `pub fn discover_files(source_dir: &Path) -> Result<(Vec<DiscoveryFile>, DiscoverManifest)>`

- Introduce incrementally (non-breaking adapter pattern):
  - `SearchQuery::parse(raw: &str) -> Result<SearchQuery, SearchContractError>`
  - `HttpUrl::parse(raw: &str) -> Result<HttpUrl, UrlValidationError>`
  - `SearchOutcome` -> `ExitCode` mapping function for explicit partial failure semantics

## Ordered Implementation Sequence
1. Enforce and test query boundary contracts (`validate_query`) for null-byte behavior.
2. Enforce and test partial-failure semantics in `run_search` (issue `doc-1vtz`).
3. Finalize URL special-char contract with IPv6 exception coverage (issue `doc-13x9`, review `discussion_r2866841648`).
4. Finalize extension/warning contract in discovery paths (`doc-2mzo`, `doc-sgzo`) and ensure helper parity.
5. Lock in tokenizer non-hang behavior with stress-oriented tests (`doc-3e4v`).
6. Run full test matrix and verify each error variant has a named scenario in `bdd-test-plan.md`.

## Done Criteria
- Every function in scope has explicit pre/postconditions covered by tests.
- Every listed failure mode maps to a named error variant.
- IPv6 host literal regression test exists and passes.
- Partial-failure search path returns non-zero process exit.
- Discovery behavior for `.markdown/.mdown/.mkd/.txt` is tested and documented.
