# Contract: `cdocs-mgf` — cli: bound derived filenames during index chunk emission

**STATUS: READY FOR TEST PLANNING**

---

## EARS Requirements

### Ubiquitous (always active)

| # | Requirement |
|---|------------|
| U1 | THE SYSTEM SHALL derive document and chunk artifact names that fit supported filesystem filename limits. |
| U2 | THE SYSTEM SHALL produce deterministic output names for the same input corpus. |
| U3 | THE SYSTEM SHALL NOT write a file to a path that exceeds filesystem filename limits. |
| U4 | THE SYSTEM SHALL preserve distinguishability between distinct source artifacts in their bounded output names. |

### Event-Driven

| # | Trigger | Shall |
|---|---------|-------|
| E1 | WHEN `ctd index` processes source documents whose derived artifact stems would exceed filesystem filename limits | THE SYSTEM SHALL emit bounded deterministic names instead of failing mid-pipeline with an OS filename error. |
| E2 | WHEN `ctd index` writes a chunk file | THE SYSTEM SHALL derive the chunk filename from the bounded doc artifact name, not the raw source path. |

### Unwanted Behaviors

| # | Condition | Shall NOT |
|---|-----------|-----------|
| W1 | IF a derived document or chunk artifact name would exceed a supported filesystem filename limit | THE SYSTEM SHALL NOT attempt to write the overlong path, BECAUSE: mid-pipeline OS error 36 breaks indexing of valid corpora and leaves partial output behind. |
| W2 | IF two distinct source documents produce truncated stems that would collide | THE SYSTEM SHALL NOT write them to the same path, BECAUSE: data loss from overwrite is unacceptable. |

---

## Derived Filename Limits

| Parameter | Value | Source |
|-----------|-------|--------|
| ext4 filename limit | 255 bytes | POSIX standard |
| Conservative working limit | 200 bytes | Allows prefix/suffix additions (e.g. `-standard.md` = 13 bytes) |
| Maximum stem budget | 180 bytes | 200 − `-standard.md` suffix |
| Document filename budget | 187 bytes | 200 − `.md` extension |

---

## Domain Terms

| Term | Definition |
|------|------------|
| **Artifact** | A written file on disk: either a derived document (`.md` in `docs/`) or a chunk (`.md` in `chunks/`). |
| **Derived document filename** | The `.md` filename produced by `assign_ids` via `format!("{category}-{subcategory}-{slug}.md")`. Currently unbounded. |
| **Chunk filename** | The `.md` filename produced by `chunk_all`/`write_chunk_file` via `format!("{chunk_id}-{level_suffix}.md")`. Currently unbounded. |
| **Bounded name** | A name that has been deterministically shortened to fit within the filename budget while preserving uniqueness. |
| **Stem collision** | Two distinct source inputs that, after truncation, produce identical bounded stems. Must be prevented. |
| **Filename explosion** | The observed corpus where source paths already contain repeated path segments (e.g., `ref-docs-...-docs-...`), causing derived filenames to exceed 255 bytes. |

---

## Filename Derivation Points (Current Implementation)

### 1. `assign_ids` → `assign.rs:45`

```
source_path  →  filename = format!("{category}-{subcategory}-{slug}.md")
                         └─ unbounded slug from Path::file_stem()
```

- **Input**: `analysis.source_path`, `analysis.category`
- **Derives**: `IdMapping.filename` (written to `docs/*.md`)
- **Problem**: No length check; long `source_path` → long `slug` → overlong `filename`

### 2. `chunk_all` → `chunk_all.rs:84–89`

```rust
let chunk_filename = format!(
    "{}-{}.md",
    chunk.chunk_id.replace(['/', '#'], "-"),
    level_suffix
);
```

- **Input**: `chunk.chunk_id` (contains doc_id + position info)
- **Derives**: chunk file on disk
- **Problem**: `chunk.chunk_id` can be very long if the doc_id/stem is long

### 3. `write_chunk_file` → `cache_ops.rs:87–92`

```rust
let chunk_filename = format!(
    "{}-{}.md",
    chunk.chunk_id.replace(['/', '#'], "-"),
    level_suffix
);
let chunk_file = chunks_dir.join(&chunk_filename);
```

- Same pattern as `chunk_all`; used by the cached chunking pathway
- Same problem

### 4. `build_chunk_metadata` → `build_index.rs:204–208`

```rust
path: format!(
    "chunks/{}-{}.md",
    chunk.chunk_id.replace(['/', '#'], "-"),
    chunk.chunk_level.as_str()
)
```

- Sets `ChunkMetadata.path` (in INDEX.json), not a filesystem write
- Must stay in sync with actual chunk filename produced by steps 2 and 3

---

## Preconditions

| # | Condition | Notes |
|---|-----------|-------|
| P1 | The input markdown corpus is valid and readable | No change needed |
| P2 | The output directory is writable | No change needed |
| P3 | `assign_ids` has computed `IdMapping` for all analyses | Upstream of chunking |
| P4 | The chunking pipeline has produced `ChunksResult` | Downstream of `chunk_all` |

---

## Postconditions

| # | Condition | Evidence |
|---|-----------|----------|
| POST1 | Indexing the long-name corpus completes without `File name too long` (OS error 36) | `ctd index ./centralized-docs/docs --output <dir> --project-name "QA Docs"` exits 0 |
| POST2 | All derived document filenames are ≤ 187 bytes | `IdMapping.filename.len() <= 187` |
| POST3 | All derived chunk filenames are ≤ 200 bytes | `chunk_filename.len() <= 200` |
| POST4 | Repeated runs derive the same bounded output names for the same inputs | Determinism: `hash(stem)[:8]` suffix is stable across invocations |
| POST5 | Distinct source documents with distinct identities produce distinct artifact filenames | No stem collision: if `source_path_a ≠ source_path_b`, then `filename_a ≠ filename_b` |
| POST6 | ChunkMetadata.path matches the actual chunk filename on disk | `INDEX.json` paths resolve to real files |

---

## Invariants

| # | Invariant |
|---|-----------|
| INV1 | Derived artifact names remain portable across ext4 filesystems (≤ 255 bytes per component). |
| INV2 | Distinct source/chunk identities remain distinguishable even when long stems are shortened. |
| INV3 | The bounded name for a given input is stable across process restarts (deterministic shortening). |
| INV4 | The `ChunkMetadata.path` stored in `INDEX.json` always matches the physical chunk file path. |

---

## Error Taxonomy

| Error | Code | Category | From |
|-------|------|----------|------|
| `File name too long` | OS error 36 | Fatal mid-pipeline failure | `fs::write` in `chunk_all.rs` or `cache_ops.rs` |
| `ChunksDirCreationFailed` | IO | Fatal pipeline failure | `create_dir_with_context` |
| `ChunkWriteFailed` | IO | Fatal pipeline failure | `fs::write` in `write_chunk_file` |
| `DocumentExceedsSizeLimit` | Validation | Fatal pre-chunk abort | `chunk_all` size check |
| Stem collision | Logic | Silent data loss (UNACCEPTABLE) | Two files → same path |

---

## Failure Modes

| # | Symptom | Likely Cause | Fix Pattern |
|---|---------|--------------|-------------|
| FM1 | `ctd index` fails with `Error: File name too long (os error 36)` at [STEP 5] CHUNK | Derived chunk filename exceeds 200 bytes | Truncate `chunk_id` stem to ≤ 180 bytes, append deterministic hash suffix `[:8]` |
| FM2 | `ctd index` fails with `Error: File name too long (os error 36)` at document write | Derived document filename from `assign_ids` exceeds 187 bytes | Truncate `slug` in `IdMapping` to ≤ budget, append deterministic hash suffix `[:8]` |
| FM3 | Two distinct long-source files produce the same output filename (overwrite) | Truncation without uniqueness preservation | Include a deterministic 8-char hash of the full original stem in the bounded name |
| FM4 | `INDEX.json` references `chunks/foo.md` but file is actually `chunks/foo-<hash>.md` | `ChunkMetadata.path` not updated after bounded naming change | Synchronize path derivation between write sites and `build_index.rs` |
| FM5 | Tests pass locally but fail in CI with a different filesystem | Assumptions about filename limits are environment-specific | Always use the 200-byte conservative limit |

---

## Key Implementation Boundaries

```
assign_ids (assign.rs)
  → IdMapping { filename: bounded_doc_filename, slug: bounded_slug, ... }
      │
      ▼
chunk_all / chunk_all_cached (chunk_all.rs / cache_ops.rs)
  → derives chunk_filename from chunk.chunk_id (which contains doc_id)
  → writes docs/*.md  (from IdMapping.filename)
  → writes chunks/*.md (from bounded chunk_filename)
      │
      ▼
build_index / build_chunk_metadata (build_index.rs)
  → sets ChunkMetadata.path (must mirror actual chunk filename)
```

---

## Research Findings

| File | Finding |
|------|---------|
| `assign.rs:45` | `new_filename = format!("{}-{}-{}.md", category, subcategory, final_slug)` — no length check |
| `assign.rs:33–42` | `slugify` uses `Slug::from_text` (max 200 chars enforced at Slug level, but Slug is not used to bound the overall filename length) |
| `chunk_all.rs:84–89` | `chunk_filename = format!("{}-{}.md", chunk.chunk_id.replace([...]), level_suffix)` — no length check |
| `cache_ops.rs:87–92` | Same pattern as `chunk_all`; no length check |
| `build_index.rs:204–208` | `ChunkMetadata.path` mirrors the unbounded chunk_filename format; must stay in sync |
| `path_types.rs` | `Slug::from_text` caps at 200 chars but is not applied to bound the overall derived filename; `Slug` max 200 is for URL safety, not filesystem safety |
| `write_tests.rs` | Tests assume short filenames; no long-name regression test exists |
| `docs/` corpus | Many files have names like `ref-docs-tasks-administer-cluster-manage-resources.md-docs-tasks-administer-cluster-manage-resources.md` (already >255 bytes) — these are the source of the os error 36 |

---

## Design Hint

The bounding strategy should be applied at the **narrowest possible boundary** — just before the `format!` that builds the final filename — so that:
1. `IdMapping.slug` gets a bounded form for document filenames
2. `chunk.chunk_id` gets a bounded form for chunk filenames
3. `ChunkMetadata.path` stays synchronized automatically because it derives from the same bounded stem

A deterministic 8-character suffix from a SHA-256 hash of the full original stem guarantees:
- **Determinism**: same input → same hash → same output across runs
- **Uniqueness preservation**: two different stems → different hashes with overwhelming probability (collision attack resistant)

Bounded name format: `{truncated_stem[:172]}-{hash_suffix[:8]}.md`

This leaves 187 − 172 − 1 = 14 bytes of margin for the extension separator and extension.
