bead_id: cdocs-2ey
bead_title: action: wire scrape command to one shared read session and one shutdown commit
phase: state-1-contract
updated_at: 2026-04-03T00:00:00Z

# Contract Specification

## Context

- **Feature**: Wire the `ctd scrape` command (`run_scrape`) into the existing two-transaction state architecture: one shared `StateReadSession` for all reads, one `StateDb::commit_changes` for all writes at shutdown.
- **Domain terms**:
  - `StateDb` -- persistent redb database at `<output>/state.redb`.
  - `StateReadSession` -- RAII guard holding one `redb::ReadTransaction` for the entire command run (from `state::bulk_load`).
  - `StateChanges` -- batch of mutations consumed by `commit_changes` (from `state::commit`).
  - `UrlStateRaw` -- 120-byte pod storing `content_hash`, `url_hash`, `last_fetched_secs`, `status_code`.
  - `PersistedScrapeResult` -- rkyv-archivable scrape output stored in the `scrape_outputs` table.
  - `ScrapeResult` / `ScrapedPage` -- runtime types from `scrape::validation`.
  - `FileDiff` / `DiffStatus` -- classification pattern (unchanged/changed/new/deleted) already used by `run_index`.
- **Assumptions**:
  - `StateDb::open`, `StateReadSession::new`, `StateReadSession::load_url_states`, `StateReadSession::load_scrapes`, `StateDb::commit_changes`, and `StateChanges::empty` are the existing building blocks.
  - The `run_index` pattern (open StateDb -> begin_read -> load -> compute diff -> ... -> commit at end) is the canonical reference implementation.
  - Scrape startup load means loading `url_state` rows from the state database at the beginning of `run_scrape`.
  - Classification means comparing freshly scraped page content hashes against stored `UrlStateRaw.content_hash` values to classify URLs as unchanged/changed/new/deleted (analogous to `compute_file_diff`).
  - Unchanged-page reuse means skipping re-scrape for URLs whose content hash matches the stored value.
  - Batch building means constructing a `StateChanges` containing `updated_urls` and `new_scrapes` during the run, then committing it once at shutdown.
- **Open questions**: None. All research_requirements satisfied.

## Preconditions

- PRE-1: `run_scrape` receives a valid `url: &str`, `output: &Path`, and `config: &ScrapeCommandConfig`.
- PRE-2: The `output` directory either exists or is creatable (for both scrape output files and `state.redb`).
- PRE-3: `StateDb::open` succeeds at `output.join("state.redb")` (creates if absent).
- PRE-4: `StateReadSession::new` succeeds, yielding a single shared read transaction.
- PRE-5: `session.load_url_states()` returns `Ok(HashMap<String, UrlStateRaw>)` (may be empty on first run).
- PRE-6: The URL validation and query/filter validation already present in `run_scrape` pass before any state operations begin.
- PRE-7: No concurrent writers hold a write transaction on `state.redb` during the scrape run.

## Postconditions

- POST-1 (success): Exactly one `StateReadSession` was created and used for all state reads during the command run.
- POST-2 (success): Exactly one call to `StateDb::commit_changes` occurred, and it returned `Ok(())`.
- POST-3 (success): The committed `StateChanges` contains:
  - `updated_urls`: every URL that was scraped (new or changed), with correct `content_hash`, `url_hash`, `last_fetched_secs`, and `status_code`.
  - `new_scrapes`: serialized `PersistedScrapeResult` keyed by `SHA-256(persisted_bytes)`.
  - All other `StateChanges` fields empty.
- POST-4 (success): Unchanged pages (content hash match) are reused from the persisted `scrape_outputs` table via `session.load_scrapes()` instead of being re-fetched.
- POST-5 (success): The `ScrapeResult` produced by `run_scrape` reflects the combined output (reused pages + freshly scraped pages).
- POST-6 (failure before commit): No writes were applied to `state.redb`. Previously committed state is intact.
- POST-7 (commit failure): `run_scrape` returns `Err` propagating the commit error. The scrape output files on disk may exist, but state is not updated.

## Invariants

- INV-1: The command performs zero per-page writes to `state.redb`. All writes are batched into a single `StateChanges` and committed atomically at shutdown.
- INV-2: A failed scrape run (any error before the final `commit_changes` call) leaves the `state.redb` database in its prior state. No partial writes.
- INV-3: The `StateReadSession` is dropped before `commit_changes` is called (redb constraint: cannot hold a read transaction while opening a write transaction).
- INV-4: The transaction model is identical to `run_index`: one shared read session, one atomic write commit. No divergence in architectural pattern.
- INV-5: `updated_urls` keys are unique (no duplicate URL strings).
- INV-6: Every `UrlStateRaw.url_hash` that is non-zero has a corresponding entry in `new_scrapes` (reference integrity enforced by `commit_changes`).
- INV-7: `new_scrapes` hash keys are non-zero (enforced by `commit_changes` precondition validation).
- INV-8: The function signature remains `pub async fn run_scrape(url: &str, output: &Path, config: &ScrapeCommandConfig) -> Result<()>` -- the state wiring is an internal implementation detail with no signature change.

## Error Taxonomy

All errors propagate through `anyhow::Error` (matching the existing `run_scrape` return type). The underlying error sources are:

| Error source | Variant | When |
|---|---|---|
| `CommitError::DatabaseOpen` | state db open failure | `StateDb::open` cannot create/open `state.redb` |
| `BulkLoadError::StorageError` | read session creation failure | `StateReadSession::new` cannot begin a read transaction |
| `StateLoadError` | url state load failure | `session.load_url_states()` encounters malformed rows or backend errors |
| `BulkLoadError` | scrape output load failure | `session.load_scrapes()` fails to load archived scrape results |
| `CommitError::*` | precondition or write failure | `StateDb::commit_changes` rejects the batch (zero hash, empty key, payload too large, missing reference, write/commit failure) |
| `anyhow::Error` | existing scrape failures | URL validation, network errors, domain unreachable, query filter, write errors (all pre-existing, unchanged) |

### Error propagation contract

- State-database errors are wrapped with context: `"failed to open state database: {e}"`, `"failed to load URL states: {e}"`, `"failed to commit scrape state: {e}"`.
- Commit errors are NEVER swallowed. A commit failure at shutdown is returned as `Err` from `run_scrape`.
- Pre-commit errors (scrape failures, validation failures) abort the pipeline before reaching `commit_changes`, satisfying INV-2.

## Contract Signatures

No new public signatures are required. The wiring uses existing types:

```
// Existing -- used by run_scrape
fn StateDb::open(path: &Path) -> Result<Self, CommitError>
fn StateReadSession::new(db: &Database) -> Result<Self, BulkLoadError>
fn StateReadSession::load_url_states(&self) -> Result<HashMap<String, UrlStateRaw>, StateLoadError>
fn StateReadSession::load_scrapes(&self, hashes: &[[u8; 32]]) -> Result<HashMap<[u8; 32], OwnedArchive<PersistedScrapeResult>>, BulkLoadError>
fn StateDb::commit_changes(&self, changes: StateChanges) -> Result<(), CommitError>
fn StateChanges::empty() -> Self
fn scrape_result_to_persisted(r: &ScrapeResult) -> PersistedScrapeResult

// Existing -- unchanged entry point
pub async fn run_scrape(url: &str, output: &Path, config: &ScrapeCommandConfig) -> Result<()>
```

### Internal wiring flow (within `run_scrape`)

```
1. Validate URL, query, filter (existing, unchanged)
2. StateDb::open(output.join("state.redb"))
3. StateReadSession::new(state_db.database())
4. session.load_url_states() -> stored_url_states: HashMap<String, UrlStateRaw>
5. Scrape site (existing scrape_site call)
6. Classify scraped pages:
   - For each scraped page, compute SHA-256 of markdown content
   - Compare against stored_url_states[url].content_hash
   - Classify: Unchanged | Changed | New
   - For unchanged pages, load full PersistedScrapeResult via session.load_scrapes()
7. Build ScrapeResult from (reused unchanged pages + freshly scraped changed/new pages)
8. Apply query filter (existing apply_query_filter)
9. Validate scrape result (existing validate_scrape_result)
10. Write scraped pages to disk (existing write_scraped_pages)
11. Build StateChanges:
    - updated_urls: (url, UrlStateRaw { content_hash, url_hash, last_fetched_secs, status_code })
    - new_scrapes: (hash, rkyv_bytes) for the full PersistedScrapeResult
12. Drop StateReadSession (explicit block scope)
13. state_db.commit_changes(changes)
14. Print summary, return Ok(())
```

## Non-goals

- Changing the `run_scrape` function signature or return type.
- Implementing incremental sitemap discovery (only pages returned by the current scrape run are considered).
- Adding new public API surface beyond the internal wiring.
- Implementing scrape diff classification for deleted URLs (pages that were in a prior run but not in the current crawl are outside scope).
- Modifying `StateDb`, `StateReadSession`, `StateChanges`, or `commit_changes` -- these are consumed as-is.
- Changing the file output format or scrape pipeline stages (query filter, SPA detection, validation).
