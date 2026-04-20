---
bead_id: cdocs-5d8
bead_title: "QA: ctd diff/apply require hidden .scrape dir"
phase: p1-contract
updated_at: 2026-04-20T01:59:15Z
---

# Contract Specification: `resolve_manifest_dir`

## Context

### Feature
Introduce a `resolve_manifest_dir` helper that resolves a user-supplied path to
the directory containing `manifest.json`, transparently handling two directory
layouts produced by `ctd scrape`:

1. **Scrape output root** — `DIR/.scrape/manifest.json` (what `ctd scrape --output DIR` produces)
2. **Direct manifest directory** — `DIR/manifest.json` (what tests, manual users, and some scripts produce)

This helper will be consumed by three call sites:
- `diff_directories(dir_a, dir_b)` in `watch/diff.rs` (line 133)
- `read_manifest(scrape_dir)` in `cmd/watch.rs` (line 192)
- Transitively by `run_diff`, `run_apply` via those functions

### Domain Terms
- **Scrape output root**: The directory passed to `ctd scrape --output`. Contains
  `.scrape/` subdirectory with `manifest.json` and page `.md` files.
- **Manifest directory**: A directory that directly contains `manifest.json`.
  May be the `.scrape/` subdirectory itself, or any directory the user points at.
- **User-supplied path**: The CLI argument the user provides. Could be either
  layout; the function must resolve which one.

### Current Bug
`ctd scrape` writes to `output_dir/.scrape/manifest.json`, but `ctd diff` and
`ctd apply` look for `manifest.json` directly at the provided path root,
ignoring the `.scrape/` subdirectory. This makes `ctd diff` and `ctd apply`
fail when given the same directory that `ctd scrape` wrote to.

### Assumptions
- The codebase uses `anyhow::Error` for watch/diff/apply error handling (no
  domain error enum yet). The contract specifies a new typed error but
  acknowledges the current convention.
- `ingest.rs` already does `output.join(".scrape")` correctly and does NOT need
  the new helper — it constructs the path itself.
- The helper is a pure resolution function; it does NOT read or parse
  `manifest.json` (that remains the caller's responsibility).
- Symlinks are followed by `std::fs::exists` — no special symlink handling.

### Open Questions
1. Should `resolve_manifest_dir` be `pub(crate)` or `pub`?  **Assumption:
   `pub(crate)`** — only used within the crate by watch/diff/cmd modules.
2. Should the error type be a dedicated enum or remain `anyhow`?
   **Recommendation: dedicated `ManifestResolveError` enum for contract
   clarity, wrapped in `anyhow::Error` at call sites to match existing convention.**
3. If BOTH `path/manifest.json` AND `path/.scrape/manifest.json` exist, which
   wins? **Decision: `path/manifest.json` wins** — direct match takes precedence,
   matching the principle of least surprise (user gave a path, we found what
   they wanted at that path).

## Preconditions

1. **P1**: `path` argument is a non-empty, valid `&Path` (may or may not exist on disk).
2. **P2**: The caller has filesystem read access to `path` and its `.scrape/` subdirectory (if present).
3. **P3**: At most one call to `resolve_manifest_dir` is needed per directory argument — the function is stateless and idempotent.

## Postconditions

1. **Post1 (Direct match)**: If `path/manifest.json` exists, returns `Ok(path.as_ref().to_path_buf())` — the input path unchanged.
2. **Post2 (Subdirectory match)**: If `path/manifest.json` does NOT exist but `path/.scrape/manifest.json` DOES exist, returns `Ok(path.join(".scrape"))`.
3. **Post3 (Neither match)**: If neither `path/manifest.json` nor `path/.scrape/manifest.json` exists, returns `Err(ManifestResolveError::NotFound { ... })` with both attempted paths in the error message.
4. **Post4 (Returned path is absolute or preserves input form)**: The returned `PathBuf` is constructed via `Path::join`, preserving the input path's form (absolute stays absolute, relative stays relative).
5. **Post5 (No side effects)**: The function performs only `std::fs::exists` checks — no file creation, deletion, or mutation.

## Invariants

1. **INV1 (Termination)**: The function checks at most 2 paths and always returns (no loops, no blocking I/O).
2. **INV2 (Determinism)**: Given the same filesystem state, calling `resolve_manifest_dir` twice yields identical results.
3. **INV3 (Path identity)**: The returned `PathBuf` always satisfies
   `result.join("manifest.json").exists()` at the moment of return (barring
   concurrent filesystem modification).
4. **INV4 (No partial resolution)**: The function never returns a path where
   `manifest.json` does not exist — it either resolves fully or errors.
5. **INV5 (Error message completeness)**: Every error variant includes both
   candidate paths so the user can diagnose the issue without re-running.

## Error Taxonomy

```rust
/// Errors from resolving a manifest directory from a user-supplied path.
#[derive(Debug, thiserror::Error)]
pub enum ManifestResolveError {
    /// Neither `path/manifest.json` nor `path/.scrape/manifest.json` exists.
    #[error(
        "No manifest.json found in '{path}' or '{scrape_subdir}'. \
         Searched:\n  - {direct}\n  - {nested}\n\
         Tip: Run 'ctd scrape --output <DIR>' first, then pass '<DIR>' to this command."
    )]
    NotFound {
        /// The user-supplied path.
        path: PathBuf,
        /// `path/.scrape`
        scrape_subdir: PathBuf,
        /// `path/manifest.json`
        direct: PathBuf,
        /// `path/.scrape/manifest.json`
        nested: PathBuf,
    },
}
```

### Error mapping at call sites

The existing functions use `anyhow::Error`. The integration pattern is:

```rust
// In diff_directories:
let resolved_a = resolve_manifest_dir(dir_a)
    .map_err(|e| anyhow::anyhow!("{e}"))?;

// In read_manifest:
let resolved = resolve_manifest_dir(scrape_dir)
    .map_err(|e| anyhow::anyhow!("{e}"))?;
```

## Contract Signatures

### Primary: `resolve_manifest_dir`

```rust
/// Resolve a user-supplied path to the directory containing `manifest.json`.
///
/// Checks two candidate locations in order:
/// 1. `path/manifest.json` — direct match (takes precedence)
/// 2. `path/.scrape/manifest.json` — nested match (what `ctd scrape` produces)
///
/// # Errors
///
/// Returns `ManifestResolveError::NotFound` if neither candidate exists.
///
/// # Examples
///
/// ```no_run
/// // Given: /tmp/output/.scrape/manifest.json exists
/// let dir = resolve_manifest_dir(Path::new("/tmp/output"))?;
/// assert_eq!(dir, PathBuf::from("/tmp/output/.scrape"));
///
/// // Given: /tmp/manual/manifest.json exists
/// let dir = resolve_manifest_dir(Path::new("/tmp/manual"))?;
/// assert_eq!(dir, PathBuf::from("/tmp/manual"));
/// ```
pub fn resolve_manifest_dir(path: &Path) -> Result<PathBuf, ManifestResolveError>
```

### Modified: `diff_directories` (watch/diff.rs)

```rust
/// Compute a plan by comparing two scrape directories.
///
/// Each directory is resolved via `resolve_manifest_dir`, supporting both
/// scrape output roots (with `.scrape/` subdirectory) and direct manifest
/// directories.
///
/// # Errors
///
/// Returns an error if either directory cannot be resolved to a manifest,
/// or if either manifest.json is missing or invalid.
pub fn diff_directories(dir_a: &Path, dir_b: &Path) -> Result<ChangePlan, anyhow::Error>
```

**Change**: Lines 134-135 change from:
```rust
let manifest_a = dir_a.join("manifest.json");
let manifest_b = dir_b.join("manifest.json");
```
To:
```rust
let resolved_a = resolve_manifest_dir(dir_a).map_err(|e| anyhow::anyhow!("{e}"))?;
let resolved_b = resolve_manifest_dir(dir_b).map_err(|e| anyhow::anyhow!("{e}"))?;
let manifest_a = resolved_a.join("manifest.json");
let manifest_b = resolved_b.join("manifest.json");
```

### Modified: `read_manifest` (cmd/watch.rs)

```rust
/// Read a manifest.json from the given scrape directory.
///
/// The directory is resolved via `resolve_manifest_dir`, supporting both
/// scrape output roots and direct manifest directories.
///
/// # Errors
///
/// Returns an error if the directory cannot be resolved to a manifest,
/// or if the manifest is missing or contains invalid JSON.
fn read_manifest(scrape_dir: &Path) -> Result<ScrapeResult>
```

**Change**: Line 193 changes from:
```rust
let manifest_path = scrape_dir.join("manifest.json");
```
To:
```rust
let resolved = resolve_manifest_dir(scrape_dir).map_err(|e| anyhow::anyhow!("{e}"))?;
let manifest_path = resolved.join("manifest.json");
```

### Unchanged call sites (confirmed correct, no modification needed)

- `write_scraped_pages` in `scrape/transformers/write.rs` — already writes to `output_dir.join(".scrape")`.
- `run_ingest` in `cmd/ingest.rs` — already does `output.join(".scrape")`.
- `run_watch` in `cmd/watch.rs` — calls `write_scraped_pages` which handles `.scrape/` internally.
- CLI arg descriptions in `cli/commands.rs` — should be updated to say "scrape output directory (may contain .scrape/manifest.json or manifest.json at root)".

## Call Graph (post-fix)

```
ctd diff DIR_A DIR_B
  -> run_diff(dir_a, dir_b, ...)
    -> diff_directories(dir_a, dir_b)
      -> resolve_manifest_dir(dir_a)   // NEW
      -> resolve_manifest_dir(dir_b)   // NEW
      -> read manifest from resolved path
      -> compute diff

ctd apply --scrape-dir DIR
  -> run_apply(url, cache_path, scrape_dir, ...)
    -> read_manifest(scrape_dir)
      -> resolve_manifest_dir(scrape_dir)  // NEW
      -> read manifest from resolved path
```

## Non-goals

1. **No filesystem mutation** — `resolve_manifest_dir` is read-only.
2. **No manifest parsing** — validation and deserialization remain the caller's job.
3. **No symlink loop detection** — relies on OS-level `exists()` behavior.
4. **No directory creation** — does not create `.scrape/` if missing.
5. **No changes to `ctd scrape` output layout** — the bug is in the consumers, not the producer.
6. **No changes to `ctd ingest`** — it already resolves correctly.
7. **No changes to `ctd watch`** — it writes plans, doesn't read manifests from directories.
