# Release Checklist

## Quality Gates

- Run `moon run :quick` and confirm it completes.
- Run `moon run :test` and confirm all tests pass.
- Run `moon run :ci` before tagging a release.

## CLI Contract Gates

- `search --json` emits valid JSON only for all outcomes (`ok`, `no_results`, `partial`, `error`).
- `search --json` returns non-zero exit code for `no_results`, `partial`, and `error` outcomes.
- Non-JSON modes return clear human-readable errors and non-zero exit on failures.

## Scrape/Ingest Gates

- `scrape --query/--threshold` actually filters pages and fails when all pages are filtered out.
- `--max-page-bytes` and `--max-total-bytes` are enforced during scraping.
- Slug collisions never overwrite pages; emitted markdown filenames are unique.
- Empty markdown pages are skipped and reported as scrape errors.

## Indexing and Locking Gates

- Indexing acquires `.doc_transformer.lock` and blocks concurrent writers.
- Stale lock reclaim works (PID dead or lock older than TTL).
- Validation errors in index output cause non-zero exit.

## End-to-End Functional Gates

- Static site flow passes: `scrape -> index -> search`.
- One-shot flow passes: `ingest -> search`.
- Search fallback works when Tantivy index is unavailable/corrupt.

## Operational Gates

- No manual cleanup required after normal runs.
- Crash/interruption recovery path is documented and tested.
- Release notes include any known limitations and mitigations.
