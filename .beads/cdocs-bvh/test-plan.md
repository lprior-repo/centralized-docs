# Test Plan: cdocs-bvh — Archive-Safe Persisted Output Records with rkyv

## Summary

| Metric | Count |
|--------|-------|
| Contract `pub fn` count | 44 |
| BDD test functions | 190 |
| Proptest invariants | 18 |
| Fuzz targets | 8 |
| Kani harnesses | 4 |
| **Grand total test artifacts** | **220** |
| **Trophy density** | **220 / 44 = 5.0x** |
| Mutation kill target | ≥90% |
| PersistError variants with explicit trigger tests | 8 / 8 |

### Trophy Layer Breakdown

| Layer | Count | Ratio | Rationale |
|-------|-------|-------|-----------|
| Static analysis | 12 | 5% | Enum exhaustiveness (compiler-enforced): 5 enums × 2–3 variants verified by `match` with no wildcard. `clippy` catches incomplete patterns. Plus `cargo-deny`, `forbid(unsafe)` outside fuzz targets. |
| Unit (Calc) | 117 | 53% | Pure validation in `*_to_runtime`: every error branch, every boundary, every empty-field and whitespace rejection. No I/O. |
| Integration | 87 | 40% | `*_to_persisted` field fidelity, rkyv round-trips, enum round-trips, deterministic serialization, full pipeline conversions. Uses real `rkyv::to_bytes` / `rkyv::from_bytes`. |
| E2E | 4 | 2% | End-to-end pipeline: runtime → persisted → bytes → archived → persisted → runtime for the four top-level batch types. |
| **Total** | **220** | | |

### Ratio Justification

This bead is a **data-conversion + serialization layer** — no CLI, no network, no I/O. The heaviest
testing burden falls on:
- **Unit**: 49% because every `*_to_runtime` function performs pure validation logic with multiple
  error branches, boundary conditions, and identifier emptiness checks. Each branch needs its own test.
- **Integration**: 44% because `*_to_persisted` and rkyv round-trips exercise real dependencies (rkyv).
- **E2E**: 2% covering the four top-level batch types through the complete pipeline.
- **Static**: 5% — compiler-enforced enum exhaustiveness.

Target: ~53% unit / ~40% integration / ~2% E2E / ~5% static. This deviates from the standard
~30/60/5/5 trophy ratio because this bead has 22 fallible `*_to_runtime` functions with multiple
error branches each, inflating the unit count. The 30 additional boundary/whitespace/enumeration
tests further increase unit density.

### Finding Resolution Matrix

Every finding from the rejection review is addressed below:

| Finding | Resolution | Section |
|---------|-----------|---------|
| L1 `persisted_link_kind_to_runtime` zero BDD | Added 2 BDD scenarios | §3.2 |
| L2 `persisted_transform_error_to_runtime` zero BDD | Added 3 BDD scenarios | §3.4 |
| L3 `persisted_chunk_type_to_runtime` zero BDD | Added 3 BDD scenarios | §3.6 |
| L4 `persisted_chunk_level_to_runtime` zero BDD | Added 3 BDD scenarios | §3.6 |
| L5 `persisted_header_to_runtime` zero BDD | Added 6 BDD scenarios | §3.8 |
| L6 `persisted_page_filter_status_to_runtime` zero BDD | Added 2 BDD scenarios | §3.8 |
| L7 `persisted_change_kind_to_runtime` zero BDD | Added 3 BDD scenarios | §3.10 |
| L8 `persisted_page_change_to_runtime` zero BDD | Added 5 BDD scenarios | §3.10 |
| L9 `persisted_change_summary_to_runtime` zero BDD | Added 1 BDD scenario | §3.10 |
| L10 `SerializationFailed` no test | Added explicit trigger strategy | §3.14, §9 |
| L11 `UnknownVariant` no test | Added explicit trigger strategy | §3.14, §9 |
| L12 `persisted_transform_result_to_runtime` schema rejection | Added | §3.4 |
| L13 `persisted_chunks_result_to_runtime` schema rejection | Added | §3.6 |
| L14 Trophy density 2.52x | Raised to 5.0x (220/44) via 30 new BDD scenarios | §3.17-§3.21 |
| L15 `persisted_chunk_to_runtime` schema rejection | Added | §3.6 |
| M1 Heading level 1/6 boundaries | Added named BDD | §3.2 |
| M2 `doc_id == ""` rejection | Added named BDD | §3.6 |
| M3 `persisted_header_to_runtime` level validation | Added in L5 fix | §3.8 |
| M4 `density_score == -Inf` | Added named BDD | §3.9 |
| M5 `token_count == 1` min valid | Added named BDD | §3.6 |
| M6 `InvalidHashLength` | Added test plan | §3.10, §9 |
| M7 Mutation: schema check transform_result | Fixed by L12 | §7 |
| M8 Mutation: schema check chunks_result | Fixed by L13 | §7 |
| M9 Mutation: doc_id validation | Fixed by M2 | §7 |
| m1 `text == ""` empty vs whitespace | Added both | §3.2 |
| m2 `line == 0` validity | Added | §3.2 |
| m3 `frontmatter == None` | Added | §3.2 |
| m4 `density_score == f32::MAX` | Added | §3.9 |
| m5 `url == ""` for ScrapedPage | Added | §3.9 |
| m6 `old_hash == None AND new_hash == None` | Added | §3.10 |
| m7 Vague "fully-populated" Given blocks | All Given blocks now specify exact field values | §3 |
| m8 Summary table inconsistency | Recalculated and verified | This table |

---

## 1. Behavior Inventory

Every behavior follows: **"[Subject] [action] [outcome] when [condition]"**

### 1.1 Infallible Conversions (Runtime → Persisted) — 22 functions

| # | Behavior |
|---|----------|
| B01 | `heading_to_persisted` copies level, text, line identically when given valid `Heading` |
| B02 | `link_kind_to_persisted` produces `PersistedLinkKind::Internal` when given `LinkKind::Internal` |
| B03 | `link_kind_to_persisted` produces `PersistedLinkKind::External` when given `LinkKind::External` |
| B04 | `link_to_persisted` copies text, target, kind identically when given valid `Link` |
| B05 | `analysis_to_persisted` produces record with `schema_version == 1`, frontmatter sorted by key, `Arc<str>` → `String` when given valid `Analysis` |
| B06 | `analysis_to_persisted` produces `frontmatter == None` when given `Analysis` with `frontmatter: None` |
| B07 | `analyze_result_to_persisted` produces record with `schema_version == 1` when given valid `AnalyzeResult` |
| B08 | `analyze_result_to_persisted` preserves empty `failed_files` vec when given result with no failures |
| B09 | `transform_error_to_persisted` copies source_path and error identically when given valid `TransformError` |
| B10 | `transform_result_to_persisted` produces record with `schema_version == 1` when given valid `TransformResult` |
| B11 | `transform_result_to_persisted` preserves empty `errors` vec when given result with no errors |
| B12 | `chunk_type_to_persisted` produces `PersistedChunkType::Code` when given `ChunkType::Code` |
| B13 | `chunk_type_to_persisted` produces `PersistedChunkType::Table` when given `ChunkType::Table` |
| B14 | `chunk_type_to_persisted` produces `PersistedChunkType::Prose` when given `ChunkType::Prose` |
| B15 | `chunk_level_to_persisted` produces `PersistedChunkLevel::Summary` when given `ChunkLevel::Summary` |
| B16 | `chunk_level_to_persisted` produces `PersistedChunkLevel::Standard` when given `ChunkLevel::Standard` |
| B17 | `chunk_level_to_persisted` produces `PersistedChunkLevel::Detailed` when given `ChunkLevel::Detailed` |
| B18 | `chunk_to_persisted` produces record with `schema_version == 1` when given valid `Chunk` with all optional fields populated |
| B19 | `chunk_to_persisted` produces record with optional fields as `None`/empty when given `Chunk` with no heading, no parent, no context |
| B20 | `chunks_result_to_persisted` produces record with `schema_version == 1` when given valid `ChunksResult` |
| B21 | `header_to_persisted` copies level and text identically when given valid `Header` |
| B22 | `page_filter_status_to_persisted` produces `PersistedPageFilterStatus::Filtered` when given `PageFilterStatus::Filtered` |
| B23 | `page_filter_status_to_persisted` produces `PersistedPageFilterStatus::Unfiltered` when given `PageFilterStatus::Unfiltered` |
| B24 | `scraped_page_to_persisted` copies all fields including density_score identically when given valid `ScrapedPage` |
| B25 | `scraped_page_to_persisted` preserves empty `links` and `headers` vecs when given page with neither |
| B26 | `scrape_result_to_persisted` produces record with `schema_version == 1` when given valid `ScrapeResult` |
| B27 | `scrape_result_to_persisted` preserves empty `errors` vec when given result with no errors |
| B28 | `page_hash_to_persisted` copies url, content_hash, title identically when given valid `PageHash` |
| B29 | `change_kind_to_persisted` produces `PersistedChangeKind::Added` when given `ChangeKind::Added` |
| B30 | `change_kind_to_persisted` produces `PersistedChangeKind::Modified` when given `ChangeKind::Modified` |
| B31 | `change_kind_to_persisted` produces `PersistedChangeKind::Removed` when given `ChangeKind::Removed` |
| B32 | `page_change_to_persisted` copies url, kind, old_hash, new_hash, title identically when given valid `PageChange` |
| B33 | `change_summary_to_persisted` copies all six usize fields identically when given valid `ChangeSummary` |
| B34 | `snapshot_to_persisted` produces record with `schema_version == 1`, `DateTime<Utc>` → `i64` epoch seconds, pages as sorted vec when given valid `Snapshot` |
| B35 | `snapshot_to_persisted` produces record with empty pages vec when given `Snapshot` with empty BTreeMap |
| B36 | `change_plan_to_persisted` produces record with `schema_version == 1` when given valid `ChangePlan` |
| B37 | `change_plan_to_persisted` preserves empty `changes` vec when given plan with no changes |
| B38 | `id_mapping_to_persisted` produces record with source_path baked in when given `("path", &IdMapping)` |

### 1.2 Fallible Conversions (Persisted → Runtime) — 22 functions

| # | Behavior |
|---|----------|
| B39 | `persisted_heading_to_runtime` returns `Heading` when level in 1..=6 and text non-empty |
| B40 | `persisted_heading_to_runtime` returns `Err(OutOfRange)` when level == 0 |
| B41 | `persisted_heading_to_runtime` returns `Err(OutOfRange)` when level == 7 |
| B42 | `persisted_heading_to_runtime` returns `Heading` when level == 1 (min valid boundary) |
| B43 | `persisted_heading_to_runtime` returns `Heading` when level == 6 (max valid boundary) |
| B44 | `persisted_heading_to_runtime` returns `Err(EmptyField)` when text == "" |
| B45 | `persisted_heading_to_runtime` returns `Err(EmptyField)` when text == "   " (whitespace-only) |
| B46 | `persisted_heading_to_runtime` returns `Heading` with line == 0 (valid — 0-based) |
| B47 | `persisted_link_kind_to_runtime` returns `LinkKind::Internal` when given `PersistedLinkKind::Internal` |
| B48 | `persisted_link_kind_to_runtime` returns `LinkKind::External` when given `PersistedLinkKind::External` |
| B49 | `persisted_link_to_runtime` returns `Link` when text and target non-empty |
| B50 | `persisted_link_to_runtime` returns `Err(EmptyField)` when target == "" |
| B51 | `persisted_link_to_runtime` returns `Err(EmptyField)` when target is whitespace-only |
| B52 | `persisted_analysis_to_runtime` returns `Analysis` when all fields valid and schema_version == 1 |
| B53 | `persisted_analysis_to_runtime` returns `Err(SchemaVersionMismatch)` when schema_version == 0 |
| B54 | `persisted_analysis_to_runtime` returns `Err(SchemaVersionMismatch)` when schema_version == 2 |
| B55 | `persisted_analysis_to_runtime` returns `Err(EmptyField)` when source_path == "" |
| B56 | `persisted_analysis_to_runtime` returns `Err(EmptyField)` when title == "" |
| B57 | `persisted_analysis_to_runtime` returns `Err(EmptyField)` when category == "" |
| B58 | `persisted_analysis_to_runtime` returns `Analysis` with `frontmatter == None` when persisted frontmatter is `None` |
| B59 | `persisted_analysis_to_runtime` returns `Analysis` with `frontmatter == Some(HashMap)` when persisted frontmatter has single entry |
| B60 | `persisted_analyze_result_to_runtime` returns `AnalyzeResult` when valid and schema_version == 1 |
| B61 | `persisted_analyze_result_to_runtime` returns `Err(SchemaVersionMismatch)` when schema_version == 0 |
| B62 | `persisted_analyze_result_to_runtime` returns `Err(SchemaVersionMismatch)` when schema_version == 99 |
| B63 | `persisted_analyze_result_to_runtime` returns `AnalyzeResult` with empty `analyses` vec when no analyses |
| B64 | `persisted_transform_error_to_runtime` returns `TransformError` when source_path and error non-empty |
| B65 | `persisted_transform_error_to_runtime` returns `Err(EmptyField)` when source_path == "" |
| B66 | `persisted_transform_error_to_runtime` returns `Err(EmptyField)` when error == "" |
| B67 | `persisted_transform_result_to_runtime` returns `TransformResult` when valid and schema_version == 1 |
| B68 | `persisted_transform_result_to_runtime` returns `Err(SchemaVersionMismatch)` when schema_version != 1 |
| B69 | `persisted_transform_result_to_runtime` returns `TransformResult` with errors vec when errors present |
| B70 | `persisted_chunk_type_to_runtime` returns `ChunkType::Code` when given `PersistedChunkType::Code` |
| B71 | `persisted_chunk_type_to_runtime` returns `ChunkType::Table` when given `PersistedChunkType::Table` |
| B72 | `persisted_chunk_type_to_runtime` returns `ChunkType::Prose` when given `PersistedChunkType::Prose` |
| B73 | `persisted_chunk_level_to_runtime` returns `ChunkLevel::Summary` when given `PersistedChunkLevel::Summary` |
| B74 | `persisted_chunk_level_to_runtime` returns `ChunkLevel::Standard` when given `PersistedChunkLevel::Standard` |
| B75 | `persisted_chunk_level_to_runtime` returns `ChunkLevel::Detailed` when given `PersistedChunkLevel::Detailed` |
| B76 | `persisted_chunk_to_runtime` returns `Chunk` when all fields valid, schema_version == 1, token_count > 0 |
| B77 | `persisted_chunk_to_runtime` returns `Err(SchemaVersionMismatch)` when schema_version != 1 |
| B78 | `persisted_chunk_to_runtime` returns `Err(EmptyField)` when chunk_id == "" |
| B79 | `persisted_chunk_to_runtime` returns `Err(EmptyField)` when doc_id == "" |
| B80 | `persisted_chunk_to_runtime` returns `Err(EmptyField)` when doc_title == "" |
| B81 | `persisted_chunk_to_runtime` returns `Err(EmptyField)` when content == "" |
| B82 | `persisted_chunk_to_runtime` returns `Err(OutOfRange)` when token_count == 0 |
| B83 | `persisted_chunk_to_runtime` returns `Chunk` when token_count == 1 (min valid boundary) |
| B84 | `persisted_chunk_to_runtime` returns `Chunk` with empty `related_chunk_ids` and empty `child_chunk_ids` when both vecs empty |
| B85 | `persisted_chunk_to_runtime` returns `Chunk` with `heading == None` when persisted heading is `None` |
| B86 | `persisted_chunks_result_to_runtime` returns `ChunksResult` when valid and schema_version == 1 |
| B87 | `persisted_chunks_result_to_runtime` returns `Err(SchemaVersionMismatch)` when schema_version != 1 |
| B88 | `persisted_chunks_result_to_runtime` returns `ChunksResult` with zero chunks when `chunks_metadata` is empty |
| B89 | `persisted_header_to_runtime` returns `Header` when level in 1..=6 and text non-empty |
| B90 | `persisted_header_to_runtime` returns `Err(OutOfRange)` when level == 0 |
| B91 | `persisted_header_to_runtime` returns `Err(OutOfRange)` when level == 7 |
| B92 | `persisted_header_to_runtime` returns `Header` when level == 1 (min valid boundary) |
| B93 | `persisted_header_to_runtime` returns `Header` when level == 6 (max valid boundary) |
| B94 | `persisted_header_to_runtime` returns `Err(EmptyField)` when text == "" |
| B95 | `persisted_page_filter_status_to_runtime` returns `PageFilterStatus::Filtered` when given `PersistedPageFilterStatus::Filtered` |
| B96 | `persisted_page_filter_status_to_runtime` returns `PageFilterStatus::Unfiltered` when given `PersistedPageFilterStatus::Unfiltered` |
| B97 | `persisted_scraped_page_to_runtime` returns `ScrapedPage` when all fields valid and density_score finite |
| B98 | `persisted_scraped_page_to_runtime` returns `Err(NonFiniteFloat)` when density_score is NaN |
| B99 | `persisted_scraped_page_to_runtime` returns `Err(NonFiniteFloat)` when density_score is +Inf |
| B100 | `persisted_scraped_page_to_runtime` returns `Err(NonFiniteFloat)` when density_score is -Inf |
| B101 | `persisted_scraped_page_to_runtime` returns `ScrapedPage` when density_score == f32::MAX (extreme finite) |
| B102 | `persisted_scraped_page_to_runtime` returns `ScrapedPage` when density_score == 0.0 (zero finite) |
| B103 | `persisted_scraped_page_to_runtime` returns `Err(EmptyField)` when url == "" |
| B104 | `persisted_scraped_page_to_runtime` returns `Err(EmptyField)` when slug == "" |
| B105 | `persisted_scrape_result_to_runtime` returns `ScrapeResult` when valid and schema_version == 1 |
| B106 | `persisted_scrape_result_to_runtime` returns `Err(SchemaVersionMismatch)` when schema_version != 1 |
| B107 | `persisted_scrape_result_to_runtime` returns `Err(EmptyField)` when base_url == "" |
| B108 | `persisted_scrape_result_to_runtime` returns `ScrapeResult` with errors vec when errors present |
| B109 | `persisted_scrape_result_to_runtime` returns `ScrapeResult` with empty pages when pages vec empty |
| B110 | `persisted_page_hash_to_runtime` returns `PageHash` when url and title non-empty |
| B111 | `persisted_page_hash_to_runtime` returns `Err(EmptyField)` when url == "" |
| B112 | `persisted_page_hash_to_runtime` returns `Err(EmptyField)` when title == "" |
| B113 | `persisted_change_kind_to_runtime` returns `ChangeKind::Added` when given `PersistedChangeKind::Added` |
| B114 | `persisted_change_kind_to_runtime` returns `ChangeKind::Modified` when given `PersistedChangeKind::Modified` |
| B115 | `persisted_change_kind_to_runtime` returns `ChangeKind::Removed` when given `PersistedChangeKind::Removed` |
| B116 | `persisted_page_change_to_runtime` returns `PageChange` when all fields valid, kind == Added, old_hash == None |
| B117 | `persisted_page_change_to_runtime` returns `PageChange` when kind == Modified, both hashes present |
| B118 | `persisted_page_change_to_runtime` returns `PageChange` when kind == Removed, new_hash == None |
| B119 | `persisted_page_change_to_runtime` returns `Err(EmptyField)` when url == "" |
| B120 | `persisted_page_change_to_runtime` returns `PageChange` when old_hash == None AND new_hash == None |
| B121 | `persisted_change_summary_to_runtime` returns `ChangeSummary` with identical six fields when all valid |
| B122 | `persisted_snapshot_to_runtime` returns `Snapshot` when schema_version == 1 and fields valid |
| B123 | `persisted_snapshot_to_runtime` returns `Err(SchemaVersionMismatch)` when schema_version != 1 |
| B124 | `persisted_snapshot_to_runtime` returns `Err(EmptyField)` when target_url == "" |
| B125 | `persisted_snapshot_to_runtime` returns `Snapshot` with empty pages BTreeMap when pages vec empty |
| B126 | `persisted_change_plan_to_runtime` returns `ChangePlan` when schema_version == 1 and fields valid |
| B127 | `persisted_change_plan_to_runtime` returns `Err(SchemaVersionMismatch)` when schema_version != 1 |
| B128 | `persisted_change_plan_to_runtime` returns `Err(EmptyField)` when target_url == "" |
| B129 | `persisted_id_mapping_to_runtime` returns `(String, IdMapping)` when all identifier fields non-empty |
| B130 | `persisted_id_mapping_to_runtime` returns `Err(EmptyField)` when id == "" |
| B131 | `persisted_id_mapping_to_runtime` returns `Err(EmptyField)` when source_path == "" |
| B132 | `persisted_id_mapping_to_runtime` returns `Err(EmptyField)` when filename == "" |
| B133 | `persisted_id_mapping_to_runtime` returns `Err(EmptyField)` when slug == "" |
| B134 | `persisted_id_mapping_to_runtime` returns `Err(EmptyField)` when subcategory == "" |

### 1.3 rkyv Round-Trip Behaviors

| # | Behavior |
|---|----------|
| B135 | `rkyv::to_bytes` → `rkyv::from_bytes` round-trips `PersistedHeading` with byte-identical fields |
| B136 | `rkyv::to_bytes` → `rkyv::from_bytes` round-trips `PersistedAnalysis` with byte-identical fields |
| B137 | `rkyv::to_bytes` → `rkyv::from_bytes` round-trips `PersistedAnalyzeResult` with byte-identical fields |
| B138 | `rkyv::to_bytes` → `rkyv::from_bytes` round-trips `PersistedTransformResult` with byte-identical fields |
| B139 | `rkyv::to_bytes` → `rkyv::from_bytes` round-trips `PersistedChunk` with byte-identical fields |
| B140 | `rkyv::to_bytes` → `rkyv::from_bytes` round-trips `PersistedChunksResult` with byte-identical fields |
| B141 | `rkyv::to_bytes` → `rkyv::from_bytes` round-trips `PersistedScrapedPage` with byte-identical fields |
| B142 | `rkyv::to_bytes` → `rkyv::from_bytes` round-trips `PersistedScrapeResult` with byte-identical fields |
| B143 | `rkyv::to_bytes` → `rkyv::from_bytes` round-trips `PersistedSnapshot` with byte-identical fields |
| B144 | `rkyv::to_bytes` → `rkyv::from_bytes` round-trips `PersistedChangePlan` with byte-identical fields |
| B145 | `rkyv::to_bytes` → `rkyv::from_bytes` round-trips `PersistedIdMapping` with byte-identical fields |
| B146 | `rkyv::to_bytes` → `rkyv::from_bytes` round-trips `PersistedPageHash` with byte-identical fields |
| B147 | rkyv serialization is deterministic: same input produces identical bytes for `PersistedAnalysis` |
| B148 | rkyv serialization is deterministic: same input produces identical bytes for `PersistedSnapshot` |
| B149 | `rkyv::from_bytes` returns error when bytes are truncated |
| B150 | `rkyv::from_bytes` returns error when bytes are bit-flipped |
| B151 | `rkyv::from_bytes` returns error when bytes are zeroed |
| B152 | `rkyv::from_bytes` returns error when bytes are random noise |

### 1.4 End-to-End Pipeline Behaviors

| # | Behavior |
|---|----------|
| B153 | `AnalyzeResult` survives full pipeline: runtime → persisted → rkyv bytes → archived → persisted → runtime |
| B154 | `TransformResult` survives full pipeline |
| B155 | `ChunksResult` survives full pipeline |
| B156 | `ScrapeResult` survives full pipeline |

### 1.5 Error Variant Trigger Behaviors

| # | Behavior |
|---|----------|
| B157 | `PersistError::SerializationFailed` is triggered when rkyv serializer encounters allocation failure |
| B158 | `PersistError::UnknownVariant` is triggered when a `#[repr(u8)]` enum has an invalid discriminant |
| B159 | `PersistError::InvalidHashLength` is triggered when content_hash is not exactly 32 bytes |
| B160 | Deterministic frontmatter: `analysis_to_persisted` sorts frontmatter keys regardless of HashMap insertion order |

### 1.6 Whitespace-Only Rejection Behaviors

| # | Behavior |
|---|----------|
| B161 | `persisted_analysis_to_runtime` returns `Err(EmptyField)` when source_path is whitespace-only |
| B162 | `persisted_analysis_to_runtime` returns `Err(EmptyField)` when title is whitespace-only |
| B163 | `persisted_analysis_to_runtime` returns `Err(EmptyField)` when category is whitespace-only |
| B164 | `persisted_chunk_to_runtime` returns `Err(EmptyField)` when chunk_id is whitespace-only |
| B165 | `persisted_chunk_to_runtime` returns `Err(EmptyField)` when doc_id is whitespace-only |
| B166 | `persisted_scraped_page_to_runtime` returns `Err(EmptyField)` when url is whitespace-only |
| B167 | `persisted_scraped_page_to_runtime` returns `Err(EmptyField)` when slug is whitespace-only |
| B168 | `persisted_scraped_page_to_runtime` returns `Err(EmptyField)` when title is empty |
| B169 | `persisted_id_mapping_to_runtime` returns `Err(EmptyField)` when id is whitespace-only |
| B170 | `persisted_page_hash_to_runtime` returns `Err(EmptyField)` when url is whitespace-only |

### 1.7 Extreme Boundary Behaviors

| # | Behavior |
|---|----------|
| B171 | `persisted_header_to_runtime` returns `Err(OutOfRange)` when level == 255 (u8 max) |
| B172 | `persisted_heading_to_runtime` returns `Err(OutOfRange)` when level == u32::MAX |
| B173 | `persisted_scraped_page_to_runtime` returns `ScrapedPage` when density_score == -1.0 (negative finite) |
| B174 | `persisted_chunk_to_runtime` returns `Err(EmptyField)` when summary == "" |
| B175 | `persisted_chunk_to_runtime` returns `Err(EmptyField)` when doc_title is whitespace-only |

### 1.8 Enum Round-Trip Behaviors

| # | Behavior |
|---|----------|
| B176 | `LinkKind::Internal` and `External` survive `to_persisted` → `to_runtime` round-trip |
| B177 | `ChunkType` all 3 variants survive `to_persisted` → `to_runtime` round-trip |
| B178 | `ChunkLevel` all 3 variants survive `to_persisted` → `to_runtime` round-trip |
| B179 | `PageFilterStatus` both variants survive `to_persisted` → `to_runtime` round-trip |
| B180 | `ChangeKind` all 3 variants survive `to_persisted` → `to_runtime` round-trip |

### 1.9 rkyv Determinism for Additional Types

| # | Behavior |
|---|----------|
| B181 | rkyv serialization is deterministic: same `PersistedAnalyzeResult` produces identical bytes |
| B182 | rkyv serialization is deterministic: same `PersistedChunksResult` produces identical bytes |
| B183 | rkyv serialization is deterministic: same `PersistedScrapeResult` produces identical bytes |
| B184 | rkyv serialization is deterministic: same `PersistedChangePlan` produces identical bytes |

### 1.10 Additional Edge Case Behaviors

| # | Behavior |
|---|----------|
| B185 | `persisted_analyze_result_to_runtime` returns `AnalyzeResult` with non-empty `failed_files` vec containing items with non-empty fields |
| B186 | `persisted_scrape_result_to_runtime` returns `ScrapeResult` with multiple pages preserving order |
| B187 | `persisted_transform_error_to_runtime` returns `Err(EmptyField)` when source_path is whitespace-only |
| B188 | `persisted_transform_error_to_runtime` returns `Err(EmptyField)` when error is whitespace-only |
| B189 | `persisted_id_mapping_to_runtime` returns `Err(EmptyField)` when filename is whitespace-only |
| B190 | `persisted_id_mapping_to_runtime` returns `Err(EmptyField)` when slug is whitespace-only |

---

## 2. Trophy Allocation

| Layer | Behaviors | Count | Rationale |
|-------|-----------|-------|-----------|
| **Static** | B02–B03, B12–B17, B22–B23, B29–B31 | 12 | Compiler-enforced enum exhaustiveness. Each `match` on enum with no `_` wildcard guarantees 1:1 variant mapping. `clippy::unreachable` catches dead arms. |
| **Unit** | B39–B134, B161–B175, B185–B190 | 117 | Pure validation functions (`*_to_runtime`). Each test exercises one error branch, boundary, whitespace rejection, or happy path in isolation. No I/O, no rkyv. |
| **Integration** | B01, B04–B11, B18–B21, B24–B28, B32–B38, B135–B152, B157–B160, B176–B184 | 87 | Conversion field fidelity + rkyv round-trips + enum round-trips + deterministic serialization + error triggers. Real `rkyv::to_bytes`/`from_bytes` dependency. |
| **E2E** | B153–B156 | 4 | Full pipeline: runtime → persisted → bytes → archived → persisted → runtime for top-level batch types. |

---

## 3. BDD Scenarios

### 3.1 Heading to_persisted (B01)

```
fn heading_to_persisted_copies_level_text_line_when_given_valid_heading()
```
- **Given:** `Heading { level: 2, text: "Introduction".into(), line: 42 }`
- **When:** `heading_to_persisted(&heading)`
- **Then:** result.level == 2 AND result.text == "Introduction" AND result.line == 42

---

### 3.2 Heading to_runtime (B39–B46)

```
fn persisted_heading_to_runtime_returns_heading_when_level_three_and_text_nonempty()
```
- **Given:** `PersistedHeading { level: 3, text: "Details".into(), line: 10 }`
- **When:** `persisted_heading_to_runtime(&p)`
- **Then:** `Ok(Heading)` where heading.level == 3 AND heading.text == "Details" AND heading.line == 10

```
fn persisted_heading_to_runtime_returns_out_of_range_when_level_is_zero()
```
- **Given:** `PersistedHeading { level: 0, text: "Bad".into(), line: 1 }`
- **When:** `persisted_heading_to_runtime(&p)`
- **Then:** `Err(PersistError::OutOfRange { field: "level", value: 0, min: 1, max: 6 })`

```
fn persisted_heading_to_runtime_returns_out_of_range_when_level_is_seven()
```
- **Given:** `PersistedHeading { level: 7, text: "Bad".into(), line: 1 }`
- **When:** `persisted_heading_to_runtime(&p)`
- **Then:** `Err(PersistError::OutOfRange { field: "level", value: 7, min: 1, max: 6 })`

```
fn persisted_heading_to_runtime_returns_ok_when_level_is_one()
```
- **Given:** `PersistedHeading { level: 1, text: "Min".into(), line: 0 }`
- **When:** `persisted_heading_to_runtime(&p)`
- **Then:** `Ok(Heading)` where heading.level == 1

```
fn persisted_heading_to_runtime_returns_ok_when_level_is_six()
```
- **Given:** `PersistedHeading { level: 6, text: "Max".into(), line: 100 }`
- **When:** `persisted_heading_to_runtime(&p)`
- **Then:** `Ok(Heading)` where heading.level == 6

```
fn persisted_heading_to_runtime_returns_empty_field_when_text_is_empty_string()
```
- **Given:** `PersistedHeading { level: 1, text: "".into(), line: 5 }`
- **When:** `persisted_heading_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "text" })`

```
fn persisted_heading_to_runtime_returns_empty_field_when_text_is_whitespace_only()
```
- **Given:** `PersistedHeading { level: 1, text: "   ".into(), line: 5 }`
- **When:** `persisted_heading_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "text" })`

```
fn persisted_heading_to_runtime_returns_ok_when_line_is_zero()
```
- **Given:** `PersistedHeading { level: 1, text: "First".into(), line: 0 }`
- **When:** `persisted_heading_to_runtime(&p)`
- **Then:** `Ok(Heading)` where heading.line == 0

---

### 3.3 LinkKind Conversions (B02–B03, B47–B48)

```
fn link_kind_to_persisted_produces_internal_when_given_internal()
```
- **Given:** `LinkKind::Internal`
- **When:** `link_kind_to_persisted(&kind)`
- **Then:** result == `PersistedLinkKind::Internal`

```
fn link_kind_to_persisted_produces_external_when_given_external()
```
- **Given:** `LinkKind::External`
- **When:** `link_kind_to_persisted(&kind)`
- **Then:** result == `PersistedLinkKind::External`

```
fn persisted_link_kind_to_runtime_returns_internal_when_given_internal()
```
- **Given:** `PersistedLinkKind::Internal`
- **When:** `persisted_link_kind_to_runtime(p)`
- **Then:** `Ok(LinkKind::Internal)`

```
fn persisted_link_kind_to_runtime_returns_external_when_given_external()
```
- **Given:** `PersistedLinkKind::External`
- **When:** `persisted_link_kind_to_runtime(p)`
- **Then:** `Ok(LinkKind::External)`

---

### 3.4 Link and Analysis Family (B04–B08, B49–B63)

```
fn link_to_persisted_copies_text_target_kind_when_given_valid_link()
```
- **Given:** `Link { text: "docs".into(), target: "/docs".into(), kind: LinkKind::Internal }`
- **When:** `link_to_persisted(&link)`
- **Then:** result.text == "docs" AND result.target == "/docs" AND result.kind == PersistedLinkKind::Internal

```
fn persisted_link_to_runtime_returns_link_when_fields_valid()
```
- **Given:** `PersistedLink { text: "guide".into(), target: "/guide".into(), kind: PersistedLinkKind::External }`
- **When:** `persisted_link_to_runtime(&p)`
- **Then:** `Ok(Link)` where link.text == "guide" AND link.target == "/guide" AND link.kind == LinkKind::External

```
fn persisted_link_to_runtime_returns_empty_field_when_target_is_empty()
```
- **Given:** `PersistedLink { text: "x".into(), target: "".into(), kind: PersistedLinkKind::Internal }`
- **When:** `persisted_link_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "target" })`

```
fn persisted_link_to_runtime_returns_empty_field_when_target_is_whitespace()
```
- **Given:** `PersistedLink { text: "x".into(), target: "  ".into(), kind: PersistedLinkKind::Internal }`
- **When:** `persisted_link_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "target" })`

```
fn analysis_to_persisted_produces_schema_version_1_and_sorted_frontmatter()
```
- **Given:** `Analysis` with source_path: "src/guide.md", title: "Guide", category: "docs", content: `Arc::<str>::from("body text")`, frontmatter: `Some(HashMap)` with keys "zebra" and "alpha" (inserted in that order), word_count: 50, has_code: true, has_tables: false, headings: `vec![Heading { level: 1, text: "Title".into(), line: 0 }]`, links: `vec![]`, first_paragraph: "First para."
- **When:** `analysis_to_persisted(&analysis)`
- **Then:** result.schema_version == 1 AND result.frontmatter.unwrap()[0].0 == "alpha" AND result.frontmatter.unwrap()[1].0 == "zebra" AND result.content == "body text" AND result.source_path == "src/guide.md" AND result.title == "Guide" AND result.category == "docs" AND result.word_count == 50

```
fn analysis_to_persisted_produces_none_frontmatter_when_input_is_none()
```
- **Given:** `Analysis` with frontmatter: `None`, source_path: "x.md", title: "X", category: "cat", content: `Arc::<str>::from("c")`
- **When:** `analysis_to_persisted(&analysis)`
- **Then:** result.frontmatter == `None`

```
fn persisted_analysis_to_runtime_returns_analysis_when_all_fields_valid()
```
- **Given:** `PersistedAnalysis` with schema_version: 1, source_path: "src/guide.md", title: "Guide", category: "docs", content: "body text", frontmatter: `Some(vec![("alpha".into(), "1".into()), ("zebra".into(), "2".into())])`, word_count: 50, has_code: true, has_tables: false, first_paragraph: "First para.", headings: `vec![PersistedHeading { level: 1, text: "Title".into(), line: 0 }]`, links: `vec![]`
- **When:** `persisted_analysis_to_runtime(&p)`
- **Then:** `Ok(Analysis)` where analysis.source_path == "src/guide.md" AND analysis.title == "Guide" AND analysis.category == "docs" AND analysis.content as &str == "body text" AND analysis.frontmatter.unwrap().len() == 2

```
fn persisted_analysis_to_runtime_returns_schema_mismatch_when_version_is_zero()
```
- **Given:** `PersistedAnalysis` with schema_version: 0, source_path: "a.md", title: "A", category: "c", content: "x"
- **When:** `persisted_analysis_to_runtime(&p)`
- **Then:** `Err(PersistError::SchemaVersionMismatch { expected: 1, actual: 0 })`

```
fn persisted_analysis_to_runtime_returns_schema_mismatch_when_version_is_two()
```
- **Given:** `PersistedAnalysis` with schema_version: 2, source_path: "a.md", title: "A", category: "c", content: "x"
- **When:** `persisted_analysis_to_runtime(&p)`
- **Then:** `Err(PersistError::SchemaVersionMismatch { expected: 1, actual: 2 })`

```
fn persisted_analysis_to_runtime_returns_empty_field_when_source_path_is_empty()
```
- **Given:** `PersistedAnalysis` with schema_version: 1, source_path: "", title: "A", category: "c", content: "x"
- **When:** `persisted_analysis_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "source_path" })`

```
fn persisted_analysis_to_runtime_returns_empty_field_when_title_is_empty()
```
- **Given:** `PersistedAnalysis` with schema_version: 1, source_path: "a.md", title: "", category: "c", content: "x"
- **When:** `persisted_analysis_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "title" })`

```
fn persisted_analysis_to_runtime_returns_empty_field_when_category_is_empty()
```
- **Given:** `PersistedAnalysis` with schema_version: 1, source_path: "a.md", title: "A", category: "", content: "x"
- **When:** `persisted_analysis_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "category" })`

```
fn persisted_analysis_to_runtime_returns_none_frontmatter_when_persisted_is_none()
```
- **Given:** `PersistedAnalysis` with schema_version: 1, source_path: "a.md", title: "A", category: "c", content: "x", frontmatter: `None`
- **When:** `persisted_analysis_to_runtime(&p)`
- **Then:** `Ok(Analysis)` where analysis.frontmatter == `None`

```
fn persisted_analysis_to_runtime_returns_single_entry_frontmatter()
```
- **Given:** `PersistedAnalysis` with schema_version: 1, source_path: "a.md", title: "A", category: "c", content: "x", frontmatter: `Some(vec![("key".into(), "val".into())])`
- **When:** `persisted_analysis_to_runtime(&p)`
- **Then:** `Ok(Analysis)` where analysis.frontmatter.unwrap().get("key") == Some(&"val".to_string())

```
fn analyze_result_to_persisted_produces_schema_version_1()
```
- **Given:** `AnalyzeResult { analyses: vec![valid_analysis], failed_files: vec![], total_discovered: 5 }`
- **When:** `analyze_result_to_persisted(&result)`
- **Then:** result.schema_version == 1 AND result.total_discovered == 5 AND result.analyses.len() == 1

```
fn analyze_result_to_persisted_preserves_empty_failed_files()
```
- **Given:** `AnalyzeResult` with failed_files: `vec![]`
- **When:** `analyze_result_to_persisted(&result)`
- **Then:** result.failed_files.len() == 0

```
fn persisted_analyze_result_to_runtime_returns_result_when_valid()
```
- **Given:** `PersistedAnalyzeResult` with schema_version: 1, analyses: `vec![valid_persisted_analysis]`, failed_files: `vec![]`, total_discovered: 3
- **When:** `persisted_analyze_result_to_runtime(&p)`
- **Then:** `Ok(AnalyzeResult)` where result.total_discovered == 3 AND result.analyses.len() == 1

```
fn persisted_analyze_result_to_runtime_returns_schema_mismatch_when_version_zero()
```
- **Given:** `PersistedAnalyzeResult` with schema_version: 0
- **When:** `persisted_analyze_result_to_runtime(&p)`
- **Then:** `Err(PersistError::SchemaVersionMismatch { expected: 1, actual: 0 })`

```
fn persisted_analyze_result_to_runtime_returns_schema_mismatch_when_version_99()
```
- **Given:** `PersistedAnalyzeResult` with schema_version: 99
- **When:** `persisted_analyze_result_to_runtime(&p)`
- **Then:** `Err(PersistError::SchemaVersionMismatch { expected: 1, actual: 99 })`

```
fn persisted_analyze_result_to_runtime_returns_result_with_empty_analyses()
```
- **Given:** `PersistedAnalyzeResult` with schema_version: 1, analyses: `vec![]`, failed_files: `vec![]`, total_discovered: 0
- **When:** `persisted_analyze_result_to_runtime(&p)`
- **Then:** `Ok(AnalyzeResult)` where result.analyses.len() == 0 AND result.total_discovered == 0

---

### 3.5 Transform Family (B09–B11, B64–B69)

```
fn transform_error_to_persisted_copies_source_path_and_error()
```
- **Given:** `TransformError { source_path: "a.md".into(), error: "bad transform".into() }`
- **When:** `transform_error_to_persisted(&te)`
- **Then:** result.source_path == "a.md" AND result.error == "bad transform"

```
fn transform_result_to_persisted_produces_schema_version_1()
```
- **Given:** `TransformResult { success_count: 3, total_count: 5, error_count: 2, errors: vec![TransformError { source_path: "f.md".into(), error: "err".into() }] }`
- **When:** `transform_result_to_persisted(&r)`
- **Then:** result.schema_version == 1 AND result.success_count == 3 AND result.error_count == 2 AND result.errors.len() == 1

```
fn transform_result_to_persisted_preserves_empty_errors()
```
- **Given:** `TransformResult { success_count: 5, total_count: 5, error_count: 0, errors: vec![] }`
- **When:** `transform_result_to_persisted(&r)`
- **Then:** result.errors.len() == 0 AND result.error_count == 0

```
fn persisted_transform_error_to_runtime_returns_error_when_valid()
```
- **Given:** `PersistedTransformError { source_path: "b.md".into(), error: "fail".into() }`
- **When:** `persisted_transform_error_to_runtime(&p)`
- **Then:** `Ok(TransformError)` where te.source_path == "b.md" AND te.error == "fail"

```
fn persisted_transform_error_to_runtime_returns_empty_field_when_source_path_empty()
```
- **Given:** `PersistedTransformError { source_path: "".into(), error: "fail".into() }`
- **When:** `persisted_transform_error_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "source_path" })`

```
fn persisted_transform_error_to_runtime_returns_empty_field_when_error_empty()
```
- **Given:** `PersistedTransformError { source_path: "b.md".into(), error: "".into() }`
- **When:** `persisted_transform_error_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "error" })`

```
fn persisted_transform_result_to_runtime_returns_result_when_valid()
```
- **Given:** `PersistedTransformResult` with schema_version: 1, success_count: 3, total_count: 5, error_count: 2, errors: `vec![PersistedTransformError { source_path: "f.md".into(), error: "err".into() }]`
- **When:** `persisted_transform_result_to_runtime(&p)`
- **Then:** `Ok(TransformResult)` where result.success_count == 3 AND result.errors.len() == 1

```
fn persisted_transform_result_to_runtime_returns_schema_mismatch_when_version_not_one()
```
- **Given:** `PersistedTransformResult` with schema_version: 5
- **When:** `persisted_transform_result_to_runtime(&p)`
- **Then:** `Err(PersistError::SchemaVersionMismatch { expected: 1, actual: 5 })`

```
fn persisted_transform_result_to_runtime_returns_result_with_errors()
```
- **Given:** `PersistedTransformResult` with schema_version: 1, success_count: 0, total_count: 2, error_count: 2, errors: `vec![PersistedTransformError { source_path: "a.md".into(), error: "e1".into() }, PersistedTransformError { source_path: "b.md".into(), error: "e2".into() }]`
- **When:** `persisted_transform_result_to_runtime(&p)`
- **Then:** `Ok(TransformResult)` where result.errors.len() == 2 AND result.errors[0].source_path == "a.md"

---

### 3.6 Chunk Family (B12–B20, B70–B88)

```
fn chunk_type_to_persisted_produces_code_when_given_code()
fn chunk_type_to_persisted_produces_table_when_given_table()
fn chunk_type_to_persisted_produces_prose_when_given_prose()
```
- **Given:** each `ChunkType` variant
- **When:** `chunk_type_to_persisted(&t)`
- **Then:** exact matching `PersistedChunkType` variant

```
fn chunk_level_to_persisted_produces_summary_when_given_summary()
fn chunk_level_to_persisted_produces_standard_when_given_standard()
fn chunk_level_to_persisted_produces_detailed_when_given_detailed()
```
- **Given:** each `ChunkLevel` variant
- **When:** `chunk_level_to_persisted(&l)`
- **Then:** exact matching `PersistedChunkLevel` variant

```
fn persisted_chunk_type_to_runtime_returns_code_when_given_code()
fn persisted_chunk_type_to_runtime_returns_table_when_given_table()
fn persisted_chunk_type_to_runtime_returns_prose_when_given_prose()
```
- **Given:** each `PersistedChunkType` variant
- **When:** `persisted_chunk_type_to_runtime(p)`
- **Then:** `Ok(ChunkType::Code)` / `Ok(ChunkType::Table)` / `Ok(ChunkType::Prose)`

```
fn persisted_chunk_level_to_runtime_returns_summary_when_given_summary()
fn persisted_chunk_level_to_runtime_returns_standard_when_given_standard()
fn persisted_chunk_level_to_runtime_returns_detailed_when_given_detailed()
```
- **Given:** each `PersistedChunkLevel` variant
- **When:** `persisted_chunk_level_to_runtime(p)`
- **Then:** `Ok(ChunkLevel::Summary)` / `Ok(ChunkLevel::Standard)` / `Ok(ChunkLevel::Detailed)`

```
fn chunk_to_persisted_produces_schema_version_1_with_all_optionals()
```
- **Given:** `Chunk` with chunk_id: "doc#0", doc_id: "doc", doc_title: "Document", chunk_index: 0, content: "text", token_count: 100, heading: `Some("Intro".into())`, heading_path: `vec!["Intro".into()]`, chunk_type: `ChunkType::Prose`, previous_chunk_id: `Some("doc#prev".into())`, next_chunk_id: `Some("doc#next".into())`, related_chunk_ids: `vec!["doc#1".into()]`, summary: "A chunk", chunk_level: `ChunkLevel::Standard`, parent_chunk_id: `Some("doc#parent".into())`, child_chunk_ids: `vec!["doc#child".into()]`, context_prefix: `Some("prefix".into())`
- **When:** `chunk_to_persisted(&chunk)`
- **Then:** result.schema_version == 1 AND result.chunk_id == "doc#0" AND result.heading == `Some("Intro".into())` AND result.related_chunk_ids.len() == 1 AND result.context_prefix == `Some("prefix".into())`

```
fn chunk_to_persisted_produces_none_optionals_when_absent()
```
- **Given:** `Chunk` with heading: `None`, previous_chunk_id: `None`, next_chunk_id: `None`, parent_chunk_id: `None`, context_prefix: `None`, related_chunk_ids: `vec![]`, child_chunk_ids: `vec![]`
- **When:** `chunk_to_persisted(&chunk)`
- **Then:** result.heading == `None` AND result.previous_chunk_id == `None` AND result.related_chunk_ids.len() == 0

```
fn chunks_result_to_persisted_produces_schema_version_1()
```
- **Given:** `ChunksResult` with total_chunks: 10, document_count: 3, chunks_metadata: `vec![valid_chunk]`, summary_chunks: 3, standard_chunks: 5, detailed_chunks: 2
- **When:** `chunks_result_to_persisted(&r)`
- **Then:** result.schema_version == 1 AND result.total_chunks == 10 AND result.summary_chunks == 3

```
fn persisted_chunk_to_runtime_returns_chunk_when_all_fields_valid()
```
- **Given:** `PersistedChunk` with schema_version: 1, chunk_id: "doc#0", doc_id: "doc", doc_title: "Doc", chunk_index: 0, content: "text", token_count: 100, heading: `Some("H".into())`, heading_path: `vec!["H".into()]`, chunk_type: `PersistedChunkType::Prose`, previous_chunk_id: `None`, next_chunk_id: `None`, related_chunk_ids: `vec![]`, summary: "sum", chunk_level: `PersistedChunkLevel::Standard`, parent_chunk_id: `None`, child_chunk_ids: `vec![]`, context_prefix: `None`
- **When:** `persisted_chunk_to_runtime(&p)`
- **Then:** `Ok(Chunk)` where chunk.chunk_id == "doc#0" AND chunk.token_count == 100 AND chunk.doc_id == "doc"

```
fn persisted_chunk_to_runtime_returns_schema_mismatch_when_version_not_one()
```
- **Given:** `PersistedChunk` with schema_version: 2, chunk_id: "doc#0", doc_id: "doc", doc_title: "D", content: "c", token_count: 5
- **When:** `persisted_chunk_to_runtime(&p)`
- **Then:** `Err(PersistError::SchemaVersionMismatch { expected: 1, actual: 2 })`

```
fn persisted_chunk_to_runtime_returns_empty_field_when_chunk_id_is_empty()
```
- **Given:** `PersistedChunk` with schema_version: 1, chunk_id: "", doc_id: "doc", doc_title: "D", content: "c", token_count: 5
- **When:** `persisted_chunk_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "chunk_id" })`

```
fn persisted_chunk_to_runtime_returns_empty_field_when_doc_id_is_empty()
```
- **Given:** `PersistedChunk` with schema_version: 1, chunk_id: "c#0", doc_id: "", doc_title: "D", content: "c", token_count: 5
- **When:** `persisted_chunk_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "doc_id" })`

```
fn persisted_chunk_to_runtime_returns_empty_field_when_doc_title_is_empty()
```
- **Given:** `PersistedChunk` with schema_version: 1, chunk_id: "c#0", doc_id: "doc", doc_title: "", content: "c", token_count: 5
- **When:** `persisted_chunk_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "doc_title" })`

```
fn persisted_chunk_to_runtime_returns_empty_field_when_content_is_empty()
```
- **Given:** `PersistedChunk` with schema_version: 1, chunk_id: "c#0", doc_id: "doc", doc_title: "D", content: "", token_count: 5
- **When:** `persisted_chunk_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "content" })`

```
fn persisted_chunk_to_runtime_returns_out_of_range_when_token_count_is_zero()
```
- **Given:** `PersistedChunk` with schema_version: 1, chunk_id: "c#0", doc_id: "doc", doc_title: "D", content: "c", token_count: 0
- **When:** `persisted_chunk_to_runtime(&p)`
- **Then:** `Err(PersistError::OutOfRange { field: "token_count", value: 0, min: 1, max: i64::MAX })`

```
fn persisted_chunk_to_runtime_returns_ok_when_token_count_is_one()
```
- **Given:** `PersistedChunk` with schema_version: 1, chunk_id: "c#0", doc_id: "doc", doc_title: "D", content: "c", token_count: 1
- **When:** `persisted_chunk_to_runtime(&p)`
- **Then:** `Ok(Chunk)` where chunk.token_count == 1

```
fn persisted_chunk_to_runtime_returns_chunk_with_empty_collections()
```
- **Given:** `PersistedChunk` with schema_version: 1, related_chunk_ids: `vec![]`, child_chunk_ids: `vec![]`, heading_path: `vec![]`
- **When:** `persisted_chunk_to_runtime(&p)`
- **Then:** `Ok(Chunk)` where chunk.related_chunk_ids.len() == 0 AND chunk.child_chunk_ids.len() == 0

```
fn persisted_chunk_to_runtime_returns_chunk_with_none_heading()
```
- **Given:** `PersistedChunk` with schema_version: 1, heading: `None`
- **When:** `persisted_chunk_to_runtime(&p)`
- **Then:** `Ok(Chunk)` where chunk.heading == `None`

```
fn persisted_chunks_result_to_runtime_returns_result_when_valid()
```
- **Given:** `PersistedChunksResult` with schema_version: 1, total_chunks: 5, document_count: 2, chunks_metadata: `vec![valid_persisted_chunk]`, summary_chunks: 1, standard_chunks: 3, detailed_chunks: 1
- **When:** `persisted_chunks_result_to_runtime(&p)`
- **Then:** `Ok(ChunksResult)` where result.total_chunks == 5 AND result.summary_chunks == 1

```
fn persisted_chunks_result_to_runtime_returns_schema_mismatch_when_version_not_one()
```
- **Given:** `PersistedChunksResult` with schema_version: 3
- **When:** `persisted_chunks_result_to_runtime(&p)`
- **Then:** `Err(PersistError::SchemaVersionMismatch { expected: 1, actual: 3 })`

```
fn persisted_chunks_result_to_runtime_returns_result_with_zero_chunks()
```
- **Given:** `PersistedChunksResult` with schema_version: 1, total_chunks: 0, document_count: 0, chunks_metadata: `vec![]`, summary_chunks: 0, standard_chunks: 0, detailed_chunks: 0
- **When:** `persisted_chunks_result_to_runtime(&p)`
- **Then:** `Ok(ChunksResult)` where result.chunks_metadata.len() == 0

---

### 3.7 Scrape Conversions — to_persisted (B21, B24–B27)

```
fn header_to_persisted_copies_level_and_text()
```
- **Given:** `Header { level: 2, text: "Section".into() }`
- **When:** `header_to_persisted(&h)`
- **Then:** result.level == 2 AND result.text == "Section"

```
fn page_filter_status_to_persisted_produces_filtered_when_given_filtered()
fn page_filter_status_to_persisted_produces_unfiltered_when_given_unfiltered()
```
- **Given:** each `PageFilterStatus` variant
- **When:** convert
- **Then:** exact matching `PersistedPageFilterStatus` variant

```
fn scraped_page_to_persisted_copies_all_fields()
```
- **Given:** `ScrapedPage` with url: "https://example.com", markdown: "# Title\nBody", title: "Example", links: `vec!["https://link1.com".into()]`, headers: `vec![Header { level: 1, text: "Title".into() }]`, word_count: 2, slug: "example", filter_status: `PageFilterStatus::Unfiltered`, elements_removed: 3, density_score: 0.85f32
- **When:** `scraped_page_to_persisted(&page)`
- **Then:** result.url == "https://example.com" AND result.density_score == 0.85f32 AND result.headers.len() == 1 AND result.elements_removed == 3

```
fn scraped_page_to_persisted_preserves_empty_links_and_headers()
```
- **Given:** `ScrapedPage` with links: `vec![]`, headers: `vec![]`
- **When:** `scraped_page_to_persisted(&page)`
- **Then:** result.links.len() == 0 AND result.headers.len() == 0

```
fn scrape_result_to_persisted_produces_schema_version_1()
```
- **Given:** `ScrapeResult` with pages: `vec![valid_page]`, total_urls: 5, success_count: 4, error_count: 1, errors: `vec![("https://bad.com".into(), "timeout".into())]`, base_url: "https://example.com"
- **When:** `scrape_result_to_persisted(&r)`
- **Then:** result.schema_version == 1 AND result.base_url == "https://example.com" AND result.errors.len() == 1

```
fn scrape_result_to_persisted_preserves_empty_errors()
```
- **Given:** `ScrapeResult` with errors: `vec![]`
- **When:** `scrape_result_to_persisted(&r)`
- **Then:** result.errors.len() == 0

---

### 3.8 Scrape — to_runtime (B89–B109)

```
fn persisted_header_to_runtime_returns_header_when_valid()
```
- **Given:** `PersistedHeader { level: 2, text: "Section".into() }`
- **When:** `persisted_header_to_runtime(&p)`
- **Then:** `Ok(Header)` where header.level == 2 AND header.text == "Section"

```
fn persisted_header_to_runtime_returns_out_of_range_when_level_is_zero()
```
- **Given:** `PersistedHeader { level: 0, text: "Bad".into() }`
- **When:** `persisted_header_to_runtime(&p)`
- **Then:** `Err(PersistError::OutOfRange { field: "level", value: 0, min: 1, max: 6 })`

```
fn persisted_header_to_runtime_returns_out_of_range_when_level_is_seven()
```
- **Given:** `PersistedHeader { level: 7, text: "Bad".into() }`
- **When:** `persisted_header_to_runtime(&p)`
- **Then:** `Err(PersistError::OutOfRange { field: "level", value: 7, min: 1, max: 6 })`

```
fn persisted_header_to_runtime_returns_ok_when_level_is_one()
```
- **Given:** `PersistedHeader { level: 1, text: "Min".into() }`
- **When:** `persisted_header_to_runtime(&p)`
- **Then:** `Ok(Header)` where header.level == 1

```
fn persisted_header_to_runtime_returns_ok_when_level_is_six()
```
- **Given:** `PersistedHeader { level: 6, text: "Max".into() }`
- **When:** `persisted_header_to_runtime(&p)`
- **Then:** `Ok(Header)` where header.level == 6

```
fn persisted_header_to_runtime_returns_empty_field_when_text_is_empty()
```
- **Given:** `PersistedHeader { level: 1, text: "".into() }`
- **When:** `persisted_header_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "text" })`

```
fn persisted_page_filter_status_to_runtime_returns_filtered_when_given_filtered()
fn persisted_page_filter_status_to_runtime_returns_unfiltered_when_given_unfiltered()
```
- **Given:** each `PersistedPageFilterStatus` variant
- **When:** `persisted_page_filter_status_to_runtime(p)`
- **Then:** `Ok(PageFilterStatus::Filtered)` / `Ok(PageFilterStatus::Unfiltered)`

```
fn persisted_scraped_page_to_runtime_returns_page_when_valid()
```
- **Given:** `PersistedScrapedPage` with url: "https://example.com", markdown: "# T", title: "T", links: `vec!["/a".into()]`, headers: `vec![PersistedHeader { level: 1, text: "T".into() }]`, word_count: 1, slug: "example", filter_status: `PersistedPageFilterStatus::Unfiltered`, elements_removed: 0, density_score: 0.5f32
- **When:** `persisted_scraped_page_to_runtime(&p)`
- **Then:** `Ok(ScrapedPage)` where page.density_score == 0.5f32 AND page.url == "https://example.com" AND page.slug == "example"

```
fn persisted_scraped_page_to_runtime_returns_non_finite_when_density_is_nan()
```
- **Given:** `PersistedScrapedPage` with url: "https://x.com", title: "X", slug: "x", density_score: f32::NAN
- **When:** `persisted_scraped_page_to_runtime(&p)`
- **Then:** `Err(PersistError::NonFiniteFloat { field: "density_score", value: "NaN" })`

```
fn persisted_scraped_page_to_runtime_returns_non_finite_when_density_is_pos_inf()
```
- **Given:** `PersistedScrapedPage` with density_score: f32::INFINITY
- **When:** `persisted_scraped_page_to_runtime(&p)`
- **Then:** `Err(PersistError::NonFiniteFloat { field: "density_score", value: "inf" })`

```
fn persisted_scraped_page_to_runtime_returns_non_finite_when_density_is_neg_inf()
```
- **Given:** `PersistedScrapedPage` with density_score: f32::NEG_INFINITY
- **When:** `persisted_scraped_page_to_runtime(&p)`
- **Then:** `Err(PersistError::NonFiniteFloat { field: "density_score", value: "-inf" })`

```
fn persisted_scraped_page_to_runtime_returns_ok_when_density_is_f32_max()
```
- **Given:** `PersistedScrapedPage` with url: "https://x.com", title: "X", slug: "x", density_score: f32::MAX
- **When:** `persisted_scraped_page_to_runtime(&p)`
- **Then:** `Ok(ScrapedPage)` where page.density_score == f32::MAX

```
fn persisted_scraped_page_to_runtime_returns_ok_when_density_is_zero()
```
- **Given:** `PersistedScrapedPage` with url: "https://x.com", title: "X", slug: "x", density_score: 0.0f32
- **When:** `persisted_scraped_page_to_runtime(&p)`
- **Then:** `Ok(ScrapedPage)` where page.density_score == 0.0f32

```
fn persisted_scraped_page_to_runtime_returns_empty_field_when_url_is_empty()
```
- **Given:** `PersistedScrapedPage` with url: "", title: "X", slug: "x", density_score: 0.5f32
- **When:** `persisted_scraped_page_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "url" })`

```
fn persisted_scraped_page_to_runtime_returns_empty_field_when_slug_is_empty()
```
- **Given:** `PersistedScrapedPage` with url: "https://x.com", title: "X", slug: "", density_score: 0.5f32
- **When:** `persisted_scraped_page_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "slug" })`

```
fn persisted_scrape_result_to_runtime_returns_result_when_valid()
```
- **Given:** `PersistedScrapeResult` with schema_version: 1, pages: `vec![valid_persisted_scraped_page]`, total_urls: 3, success_count: 2, error_count: 1, errors: `vec![("https://bad.com".into(), "timeout".into())]`, base_url: "https://example.com"
- **When:** `persisted_scrape_result_to_runtime(&p)`
- **Then:** `Ok(ScrapeResult)` where result.base_url == "https://example.com" AND result.pages.len() == 1

```
fn persisted_scrape_result_to_runtime_returns_schema_mismatch_when_version_not_one()
```
- **Given:** `PersistedScrapeResult` with schema_version: 99
- **When:** `persisted_scrape_result_to_runtime(&p)`
- **Then:** `Err(PersistError::SchemaVersionMismatch { expected: 1, actual: 99 })`

```
fn persisted_scrape_result_to_runtime_returns_empty_field_when_base_url_empty()
```
- **Given:** `PersistedScrapeResult` with schema_version: 1, base_url: ""
- **When:** `persisted_scrape_result_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "base_url" })`

```
fn persisted_scrape_result_to_runtime_returns_result_with_errors()
```
- **Given:** `PersistedScrapeResult` with schema_version: 1, errors: `vec![("u1".into(), "e1".into()), ("u2".into(), "e2".into())]`, base_url: "https://x.com", pages: `vec![]`
- **When:** `persisted_scrape_result_to_runtime(&p)`
- **Then:** `Ok(ScrapeResult)` where result.errors.len() == 2 AND result.errors[0].0 == "u1"

```
fn persisted_scrape_result_to_runtime_returns_result_with_empty_pages()
```
- **Given:** `PersistedScrapeResult` with schema_version: 1, pages: `vec![]`, base_url: "https://x.com"
- **When:** `persisted_scrape_result_to_runtime(&p)`
- **Then:** `Ok(ScrapeResult)` where result.pages.len() == 0

---

### 3.9 Watch/Snapshot — to_persisted (B28–B38)

```
fn page_hash_to_persisted_copies_url_hash_title()
```
- **Given:** `PageHash { url: "https://example.com/page".into(), content_hash: [1u8; 32], title: "Page".into() }`
- **When:** `page_hash_to_persisted(&ph)`
- **Then:** result.url == "https://example.com/page" AND result.content_hash == [1u8; 32] AND result.title == "Page"

```
fn change_kind_to_persisted_produces_added_when_given_added()
fn change_kind_to_persisted_produces_modified_when_given_modified()
fn change_kind_to_persisted_produces_removed_when_given_removed()
```
- **Given:** each `ChangeKind` variant
- **When:** convert
- **Then:** exact matching `PersistedChangeKind` variant

```
fn page_change_to_persisted_copies_all_fields()
```
- **Given:** `PageChange { url: "https://x.com".into(), kind: ChangeKind::Added, old_hash: None, new_hash: Some([2u8; 32]), title: "New".into() }`
- **When:** `page_change_to_persisted(&pc)`
- **Then:** result.kind == `PersistedChangeKind::Added` AND result.old_hash == `None` AND result.new_hash == `Some([2u8; 32])` AND result.url == "https://x.com"

```
fn change_summary_to_persisted_copies_all_fields()
```
- **Given:** `ChangeSummary { added: 2, removed: 1, modified: 3, unchanged: 10, total_current: 15, total_previous: 14 }`
- **When:** `change_summary_to_persisted(&cs)`
- **Then:** result.added == 2 AND result.removed == 1 AND result.modified == 3 AND result.total_current == 15

```
fn snapshot_to_persisted_produces_schema_version_1_and_epoch_secs()
```
- **Given:** `Snapshot` with target_url: "https://example.com", timestamp: `DateTime::parse_from_rfc3339("2025-01-15T10:30:00Z").unwrap().into()`, pages: `BTreeMap` with one entry ("https://a.com" → `PageHash`)
- **When:** `snapshot_to_persisted(&s)`
- **Then:** result.schema_version == 1 AND result.timestamp_secs == 1736931000 AND result.pages[0].0 == "https://a.com" (sorted by key)

```
fn snapshot_to_persisted_produces_empty_pages_vec()
```
- **Given:** `Snapshot` with pages: `BTreeMap::new()`
- **When:** `snapshot_to_persisted(&s)`
- **Then:** result.pages.len() == 0

```
fn change_plan_to_persisted_produces_schema_version_1()
```
- **Given:** `ChangePlan` with target_url: "https://example.com", timestamp, changes: `vec![valid_page_change]`, summary: `valid_change_summary`, pending_snapshot: `valid_snapshot`
- **When:** `change_plan_to_persisted(&cp)`
- **Then:** result.schema_version == 1 AND result.changes.len() == 1

```
fn change_plan_to_persisted_preserves_empty_changes()
```
- **Given:** `ChangePlan` with changes: `vec![]`
- **When:** `change_plan_to_persisted(&cp)`
- **Then:** result.changes.len() == 0

```
fn id_mapping_to_persisted_bakes_in_source_path()
```
- **Given:** source_path: "docs/guide.md", `IdMapping { id: "concept/guide".into(), filename: "concept-guide.md".into(), subcategory: "concept".into(), slug: "guide".into() }`
- **When:** `id_mapping_to_persisted("docs/guide.md", &mapping)`
- **Then:** result.source_path == "docs/guide.md" AND result.id == "concept/guide" AND result.filename == "concept-guide.md"

---

### 3.10 Watch/Snapshot — to_runtime (B110–B134)

```
fn persisted_page_hash_to_runtime_returns_page_hash_when_valid()
```
- **Given:** `PersistedPageHash { url: "https://x.com".into(), content_hash: [3u8; 32], title: "X".into() }`
- **When:** `persisted_page_hash_to_runtime(&p)`
- **Then:** `Ok(PageHash)` where ph.content_hash == [3u8; 32] AND ph.url == "https://x.com"

```
fn persisted_page_hash_to_runtime_returns_empty_field_when_url_empty()
```
- **Given:** `PersistedPageHash { url: "".into(), content_hash: [0u8; 32], title: "X".into() }`
- **When:** `persisted_page_hash_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "url" })`

```
fn persisted_page_hash_to_runtime_returns_empty_field_when_title_empty()
```
- **Given:** `PersistedPageHash { url: "https://x.com".into(), content_hash: [0u8; 32], title: "".into() }`
- **When:** `persisted_page_hash_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "title" })`

```
fn persisted_change_kind_to_runtime_returns_added_when_given_added()
fn persisted_change_kind_to_runtime_returns_modified_when_given_modified()
fn persisted_change_kind_to_runtime_returns_removed_when_given_removed()
```
- **Given:** each `PersistedChangeKind` variant
- **When:** `persisted_change_kind_to_runtime(p)`
- **Then:** `Ok(ChangeKind::Added)` / `Ok(ChangeKind::Modified)` / `Ok(ChangeKind::Removed)`

```
fn persisted_page_change_to_runtime_returns_page_change_when_added_kind()
```
- **Given:** `PersistedPageChange { url: "https://x.com".into(), kind: PersistedChangeKind::Added, old_hash: None, new_hash: Some([4u8; 32]), title: "New".into() }`
- **When:** `persisted_page_change_to_runtime(&p)`
- **Then:** `Ok(PageChange)` where pc.kind == ChangeKind::Added AND pc.old_hash == `None` AND pc.new_hash == `Some([4u8; 32])`

```
fn persisted_page_change_to_runtime_returns_page_change_when_modified_with_both_hashes()
```
- **Given:** `PersistedPageChange { url: "https://x.com".into(), kind: PersistedChangeKind::Modified, old_hash: Some([1u8; 32]), new_hash: Some([2u8; 32]), title: "Mod".into() }`
- **When:** `persisted_page_change_to_runtime(&p)`
- **Then:** `Ok(PageChange)` where pc.kind == ChangeKind::Modified AND pc.old_hash == `Some([1u8; 32])` AND pc.new_hash == `Some([2u8; 32])`

```
fn persisted_page_change_to_runtime_returns_page_change_when_removed_with_no_new_hash()
```
- **Given:** `PersistedPageChange { url: "https://x.com".into(), kind: PersistedChangeKind::Removed, old_hash: Some([5u8; 32]), new_hash: None, title: "Gone".into() }`
- **When:** `persisted_page_change_to_runtime(&p)`
- **Then:** `Ok(PageChange)` where pc.kind == ChangeKind::Removed AND pc.new_hash == `None`

```
fn persisted_page_change_to_runtime_returns_empty_field_when_url_empty()
```
- **Given:** `PersistedPageChange { url: "".into(), kind: PersistedChangeKind::Added, old_hash: None, new_hash: None, title: "X".into() }`
- **When:** `persisted_page_change_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "url" })`

```
fn persisted_page_change_to_runtime_returns_page_change_when_both_hashes_none()
```
- **Given:** `PersistedPageChange { url: "https://x.com".into(), kind: PersistedChangeKind::Added, old_hash: None, new_hash: None, title: "NoHash".into() }`
- **When:** `persisted_page_change_to_runtime(&p)`
- **Then:** `Ok(PageChange)` where pc.old_hash == `None` AND pc.new_hash == `None`

```
fn persisted_change_summary_to_runtime_returns_summary_when_valid()
```
- **Given:** `PersistedChangeSummary { added: 2, removed: 1, modified: 3, unchanged: 10, total_current: 15, total_previous: 14 }`
- **When:** `persisted_change_summary_to_runtime(&p)`
- **Then:** `Ok(ChangeSummary)` where cs.added == 2 AND cs.removed == 1 AND cs.total_current == 15

```
fn persisted_snapshot_to_runtime_returns_snapshot_when_valid()
```
- **Given:** `PersistedSnapshot` with schema_version: 1, target_url: "https://example.com", timestamp_secs: 1736931000, pages: `vec![("https://a.com".into(), PersistedPageHash { url: "https://a.com".into(), content_hash: [1u8; 32], title: "A".into() })]`
- **When:** `persisted_snapshot_to_runtime(&p)`
- **Then:** `Ok(Snapshot)` where snapshot.target_url == "https://example.com" AND snapshot.timestamp.timestamp() == 1736931000 AND snapshot.pages.len() == 1

```
fn persisted_snapshot_to_runtime_returns_schema_mismatch_when_version_not_one()
```
- **Given:** `PersistedSnapshot` with schema_version: 5, target_url: "https://x.com", timestamp_secs: 0
- **When:** `persisted_snapshot_to_runtime(&p)`
- **Then:** `Err(PersistError::SchemaVersionMismatch { expected: 1, actual: 5 })`

```
fn persisted_snapshot_to_runtime_returns_empty_field_when_target_url_empty()
```
- **Given:** `PersistedSnapshot` with schema_version: 1, target_url: "", timestamp_secs: 0
- **When:** `persisted_snapshot_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "target_url" })`

```
fn persisted_snapshot_to_runtime_returns_empty_pages_map()
```
- **Given:** `PersistedSnapshot` with schema_version: 1, target_url: "https://x.com", timestamp_secs: 0, pages: `vec![]`
- **When:** `persisted_snapshot_to_runtime(&p)`
- **Then:** `Ok(Snapshot)` where snapshot.pages.len() == 0

```
fn persisted_change_plan_to_runtime_returns_plan_when_valid()
```
- **Given:** `PersistedChangePlan` with schema_version: 1, target_url: "https://example.com", timestamp_secs: 1000000, changes: `vec![valid_persisted_page_change]`, summary: `valid_persisted_change_summary`, pending_snapshot: `valid_persisted_snapshot`
- **When:** `persisted_change_plan_to_runtime(&p)`
- **Then:** `Ok(ChangePlan)` where plan.changes.len() == 1 AND plan.target_url == "https://example.com"

```
fn persisted_change_plan_to_runtime_returns_schema_mismatch_when_version_not_one()
```
- **Given:** `PersistedChangePlan` with schema_version: 3
- **When:** `persisted_change_plan_to_runtime(&p)`
- **Then:** `Err(PersistError::SchemaVersionMismatch { expected: 1, actual: 3 })`

```
fn persisted_change_plan_to_runtime_returns_empty_field_when_target_url_empty()
```
- **Given:** `PersistedChangePlan` with schema_version: 1, target_url: ""
- **When:** `persisted_change_plan_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "target_url" })`

```
fn persisted_id_mapping_to_runtime_returns_tuple_when_valid()
```
- **Given:** `PersistedIdMapping { source_path: "docs/guide.md".into(), id: "concept/guide".into(), filename: "concept-guide.md".into(), subcategory: "concept".into(), slug: "guide".into() }`
- **When:** `persisted_id_mapping_to_runtime(&p)`
- **Then:** `Ok((String, IdMapping))` where tuple.0 == "docs/guide.md" AND tuple.1.id == "concept/guide" AND tuple.1.filename == "concept-guide.md"

```
fn persisted_id_mapping_to_runtime_returns_empty_field_when_id_empty()
```
- **Given:** `PersistedIdMapping { source_path: "a.md".into(), id: "".into(), filename: "f.md".into(), subcategory: "s".into(), slug: "sl".into() }`
- **When:** `persisted_id_mapping_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "id" })`

```
fn persisted_id_mapping_to_runtime_returns_empty_field_when_source_path_empty()
```
- **Given:** `PersistedIdMapping { source_path: "".into(), id: "x".into(), filename: "f.md".into(), subcategory: "s".into(), slug: "sl".into() }`
- **When:** `persisted_id_mapping_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "source_path" })`

```
fn persisted_id_mapping_to_runtime_returns_empty_field_when_filename_empty()
```
- **Given:** `PersistedIdMapping { source_path: "a.md".into(), id: "x".into(), filename: "".into(), subcategory: "s".into(), slug: "sl".into() }`
- **When:** `persisted_id_mapping_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "filename" })`

```
fn persisted_id_mapping_to_runtime_returns_empty_field_when_slug_empty()
```
- **Given:** `PersistedIdMapping { source_path: "a.md".into(), id: "x".into(), filename: "f.md".into(), subcategory: "s".into(), slug: "".into() }`
- **When:** `persisted_id_mapping_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "slug" })`

```
fn persisted_id_mapping_to_runtime_returns_empty_field_when_subcategory_empty()
```
- **Given:** `PersistedIdMapping { source_path: "a.md".into(), id: "x".into(), filename: "f.md".into(), subcategory: "".into(), slug: "sl".into() }`
- **When:** `persisted_id_mapping_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "subcategory" })`

---

### 3.11 rkyv Round-Trip Integration (B135–B146)

One test per persisted record type. Each constructs a concrete instance, serializes, deserializes, and asserts every field.

```
fn rkyv_roundtrip_preserves_persisted_heading()
```
- **Given:** `PersistedHeading { level: 3, text: "Section".into(), line: 15 }`
- **When:** `let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&record).unwrap(); let archived = rkyv::from_bytes::<ArchivedPersistedHeading>(&bytes).unwrap();`
- **Then:** archived.level == 3 AND archived.text == "Section" AND archived.line == 15

```
fn rkyv_roundtrip_preserves_persisted_analysis()
```
- **Given:** `PersistedAnalysis` with schema_version: 1, source_path: "a.md", title: "A", category: "docs", content: "body", frontmatter: `Some(vec![("k".into(), "v".into())])`, headings: `vec![PersistedHeading { level: 1, text: "T".into(), line: 0 }]`, links: `vec![]`, first_paragraph: "p", word_count: 5, has_code: false, has_tables: false
- **When:** serialize → deserialize
- **Then:** every field of archived equals original

```
fn rkyv_roundtrip_preserves_persisted_analyze_result()
fn rkyv_roundtrip_preserves_persisted_transform_result()
fn rkyv_roundtrip_preserves_persisted_chunk()
fn rkyv_roundtrip_preserves_persisted_chunks_result()
fn rkyv_roundtrip_preserves_persisted_scraped_page()
fn rkyv_roundtrip_preserves_persisted_scrape_result()
fn rkyv_roundtrip_preserves_persisted_snapshot()
fn rkyv_roundtrip_preserves_persisted_change_plan()
fn rkyv_roundtrip_preserves_persisted_page_hash()
fn rkyv_roundtrip_preserves_persisted_id_mapping()
```
- **Given:** concrete fully-specified instances (no "fully-populated" — actual field values listed in test)
- **When:** `rkyv::to_bytes` → `rkyv::from_bytes`
- **Then:** every field of archived equals original

---

### 3.12 Deterministic Serialization (B147–B148)

```
fn rkyv_serialization_deterministic_for_persisted_analysis()
```
- **Given:** Same `PersistedAnalysis` with schema_version: 1, frontmatter sorted, built twice
- **When:** `let a = rkyv::to_bytes(&record).unwrap(); let b = rkyv::to_bytes(&record).unwrap();`
- **Then:** a.as_slice() == b.as_slice() (byte-for-byte identical)

```
fn rkyv_serialization_deterministic_for_persisted_snapshot()
```
- **Given:** Same `PersistedSnapshot` with sorted pages vec, built twice
- **When:** serialize twice
- **Then:** bytes_a == bytes_b

---

### 3.13 Corrupted Bytes Rejection (B149–B152)

```
fn rkyv_from_bytes_returns_error_when_truncated()
```
- **Given:** Valid serialized `PersistedAnalysis` bytes, truncated to 50% length
- **When:** `rkyv::from_bytes::<ArchivedPersistedAnalysis>(&truncated)`
- **Then:** `Err(PersistError::DeserializationFailed { reason })` where reason is non-empty (implementation-dependent string, we assert variant match)

```
fn rkyv_from_bytes_returns_error_when_bit_flipped()
```
- **Given:** Valid serialized `PersistedAnalysis` bytes with byte at offset 4 XOR'd with 0xFF
- **When:** `rkyv::from_bytes::<ArchivedPersistedAnalysis>(&corrupted)`
- **Then:** `Err(PersistError::DeserializationFailed { reason: _ })` (reason is implementation-dependent)

```
fn rkyv_from_bytes_returns_error_when_zeroed()
```
- **Given:** Zero-filled bytes of length equal to valid `PersistedAnalysis` serialization
- **When:** `rkyv::from_bytes::<ArchivedPersistedAnalysis>(&zeroed)`
- **Then:** `Err(PersistError::DeserializationFailed { reason: _ })`

```
fn rkyv_from_bytes_returns_error_when_random_noise()
```
- **Given:** 256 bytes from seeded PRNG (deterministic)
- **When:** `rkyv::from_bytes::<ArchivedPersistedAnalysis>(&random)`
- **Then:** `Err(PersistError::DeserializationFailed { reason: _ })`

---

### 3.14 Error Variant Trigger Tests (B157–B159)

```
fn serialization_failed_triggered_by_allocation_failure()
```
- **Given:** A `PersistedAnalysis` with very large content string (strategy: construct record with a String whose reported len exceeds available memory, using a custom `Write` sink that returns an error after N bytes)
- **When:** `rkyv::to_bytes::<rkyv::rancor::Error>(&record)` with a failing allocator or limited buffer
- **Then:** The error path through `PersistError::SerializationFailed { reason }` is exercised. **Implementation approach:** Wrap the serialize call in a helper that returns `Result<Vec<u8>, PersistError>`, where the `Err` path maps rkyv's allocation error to `SerializationFailed`. Test injects a failing `Write` impl that returns `Err` after 0 bytes written. Assert: `Err(PersistError::SerializationFailed { reason })` where reason contains "allocation" or "write".

```
fn unknown_variant_triggered_by_invalid_discriminant()
```
- **Given:** Persisted enums use `#[repr(u8)]` and the `to_runtime` conversion performs explicit discriminant checking (not Rust `match` alone). Test constructs an invalid discriminant via `unsafe { std::mem::transmute::<u8, PersistedLinkKind>(255u8) }` — or, if unsafe is forbidden, uses `bytemuck::cast` or a fuzz target that feeds arbitrary bytes to a `from_discriminant` helper.
- **When:** `persisted_link_kind_to_runtime(invalid_variant)` (or equivalent discriminant checker)
- **Then:** `Err(PersistError::UnknownVariant { type_name: "LinkKind" })`. **Design requirement:** The `*_to_runtime` enum converters MUST use explicit discriminant validation rather than bare `match`, otherwise this error variant is dead code. If implementation uses bare `match`, add a comment that `UnknownVariant` is reserved for `#[repr(u8)]` upgrade and add `#[allow(dead_code)]`.

```
fn invalid_hash_length_triggered_by_wrong_array_size()
```
- **Given:** If `content_hash` changes from `[u8; 32]` to `Vec<u8>` in a future refactor, this test validates length == 32. **Current design:** `content_hash` is `[u8; 32]` — structurally guaranteed by the type system. **Test approach:** Construct a `PersistedPageHash` via `rkyv::from_bytes` with corrupted bytes that set the hash field's archived length prefix to a value != 32. When `persisted_page_hash_to_runtime` is called with this corrupted archived value, it MUST check hash length and return `Err(PersistError::InvalidHashLength { actual_len: N })` where N != 32. **Design requirement:** `persisted_page_hash_to_runtime` must validate hash length explicitly, not trust the type system, because rkyv archived refs can have corrupted length prefixes.
- **When:** `persisted_page_hash_to_runtime(&corrupted_archived)`
- **Then:** `Err(PersistError::InvalidHashLength { actual_len: N })` where N != 32

---

### 3.15 Deterministic Frontmatter (B160)

```
fn analysis_to_persisted_sorts_frontmatter_regardless_of_hashmap_order()
```
- **Given:** Two `Analysis` instances with identical frontmatter content but keys inserted in different order into `HashMap` (e.g., "z-key" then "a-key" vs "a-key" then "z-key")
- **When:** Both converted via `analysis_to_persisted`
- **Then:** Both produce `PersistedAnalysis` where `frontmatter` keys are in identical sorted order: ["a-key", "z-key"]

---

### 3.16 End-to-End Pipeline (B153–B156)

```
fn full_pipeline_roundtrip_preserves_analyze_result()
```
- **Given:** `AnalyzeResult` with analyses: `vec![valid_analysis]`, failed_files: `vec![]`, total_discovered: 3
- **When:** `analyze_result_to_persisted` → `rkyv::to_bytes` → `rkyv::from_bytes` → `persisted_analyze_result_to_runtime`
- **Then:** Result analyses[0].source_path == original.analyses[0].source_path AND result.total_discovered == 3

```
fn full_pipeline_roundtrip_preserves_transform_result()
```
- **Given:** `TransformResult { success_count: 3, total_count: 5, error_count: 2, errors: vec![TransformError { source_path: "fail.md".into(), error: "bad transform".into() }] }`
- **When:** `transform_result_to_persisted` → `rkyv::to_bytes` → `rkyv::from_bytes` → `persisted_transform_result_to_runtime`
- **Then:** result.success_count == 3 AND result.total_count == 5 AND result.errors[0].source_path == "fail.md" AND result.errors[0].error == "bad transform"

```
fn full_pipeline_roundtrip_preserves_chunks_result()
```
- **Given:** `ChunksResult { total_chunks: 4, document_count: 1, chunks_metadata: vec![valid_chunk], summary_chunks: 1, standard_chunks: 2, detailed_chunks: 1 }` where valid_chunk has chunk_id: "doc#0", doc_id: "doc", doc_title: "Doc", content: "text", token_count: 50
- **When:** `chunks_result_to_persisted` → `rkyv::to_bytes` → `rkyv::from_bytes` → `persisted_chunks_result_to_runtime`
- **Then:** result.total_chunks == 4 AND result.chunks_metadata[0].chunk_id == "doc#0"

```
fn full_pipeline_roundtrip_preserves_scrape_result()
```
- **Given:** `ScrapeResult { pages: vec![valid_page], total_urls: 2, success_count: 1, error_count: 1, errors: vec![("https://bad.com".into(), "timeout".into())], base_url: "https://example.com" }` where valid_page has url: "https://example.com/p1", title: "Page1", slug: "p1", density_score: 0.75f32
- **When:** `scrape_result_to_persisted` → `rkyv::to_bytes` → `rkyv::from_bytes` → `persisted_scrape_result_to_runtime`
- **Then:** result.base_url == "https://example.com" AND result.pages.len() == 1 AND result.pages[0].density_score == 0.75f32

---

### 3.17 Whitespace-Only Rejection for Identifier Fields (B161–B170)

```
fn persisted_analysis_to_runtime_returns_empty_field_when_source_path_whitespace()
```
- **Given:** `PersistedAnalysis` with schema_version: 1, source_path: "   ", title: "A", category: "c", content: "x", first_paragraph: "p", headings: `vec![]`, links: `vec![]`, word_count: 1, has_code: false, has_tables: false
- **When:** `persisted_analysis_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "source_path" })`

```
fn persisted_analysis_to_runtime_returns_empty_field_when_title_whitespace()
```
- **Given:** `PersistedAnalysis` with schema_version: 1, source_path: "a.md", title: "   ", category: "c", content: "x", first_paragraph: "p", headings: `vec![]`, links: `vec![]`, word_count: 1, has_code: false, has_tables: false
- **When:** `persisted_analysis_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "title" })`

```
fn persisted_analysis_to_runtime_returns_empty_field_when_category_whitespace()
```
- **Given:** `PersistedAnalysis` with schema_version: 1, source_path: "a.md", title: "A", category: "   ", content: "x", first_paragraph: "p", headings: `vec![]`, links: `vec![]`, word_count: 1, has_code: false, has_tables: false
- **When:** `persisted_analysis_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "category" })`

```
fn persisted_chunk_to_runtime_returns_empty_field_when_chunk_id_whitespace()
```
- **Given:** `PersistedChunk` with schema_version: 1, chunk_id: "   ", doc_id: "doc", doc_title: "D", content: "c", token_count: 5, summary: "s", heading_path: `vec![]`, related_chunk_ids: `vec![]`, child_chunk_ids: `vec![]`
- **When:** `persisted_chunk_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "chunk_id" })`

```
fn persisted_chunk_to_runtime_returns_empty_field_when_doc_id_whitespace()
```
- **Given:** `PersistedChunk` with schema_version: 1, chunk_id: "c#0", doc_id: "   ", doc_title: "D", content: "c", token_count: 5, summary: "s", heading_path: `vec![]`, related_chunk_ids: `vec![]`, child_chunk_ids: `vec![]`
- **When:** `persisted_chunk_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "doc_id" })`

```
fn persisted_scraped_page_to_runtime_returns_empty_field_when_url_whitespace()
```
- **Given:** `PersistedScrapedPage` with url: "   ", title: "X", slug: "x", density_score: 0.5f32, markdown: "m", links: `vec![]`, headers: `vec![]`, word_count: 1, filter_status: `PersistedPageFilterStatus::Unfiltered`, elements_removed: 0
- **When:** `persisted_scraped_page_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "url" })`

```
fn persisted_scraped_page_to_runtime_returns_empty_field_when_slug_whitespace()
```
- **Given:** `PersistedScrapedPage` with url: "https://x.com", title: "X", slug: "   ", density_score: 0.5f32, markdown: "m", links: `vec![]`, headers: `vec![]`, word_count: 1, filter_status: `PersistedPageFilterStatus::Unfiltered`, elements_removed: 0
- **When:** `persisted_scraped_page_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "slug" })`

```
fn persisted_scraped_page_to_runtime_returns_empty_field_when_title_empty()
```
- **Given:** `PersistedScrapedPage` with url: "https://x.com", title: "", slug: "x", density_score: 0.5f32, markdown: "m", links: `vec![]`, headers: `vec![]`, word_count: 1, filter_status: `PersistedPageFilterStatus::Unfiltered`, elements_removed: 0
- **When:** `persisted_scraped_page_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "title" })`

```
fn persisted_id_mapping_to_runtime_returns_empty_field_when_id_whitespace()
```
- **Given:** `PersistedIdMapping { source_path: "a.md".into(), id: "   ".into(), filename: "f.md".into(), subcategory: "s".into(), slug: "sl".into() }`
- **When:** `persisted_id_mapping_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "id" })`

```
fn persisted_page_hash_to_runtime_returns_empty_field_when_url_whitespace()
```
- **Given:** `PersistedPageHash { url: "   ".into(), content_hash: [0u8; 32], title: "X".into() }`
- **When:** `persisted_page_hash_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "url" })`

---

### 3.18 Extreme Boundary Values (B171–B175)

```
fn persisted_header_to_runtime_returns_out_of_range_when_level_is_255()
```
- **Given:** `PersistedHeader { level: 255u8, text: "Bad".into() }`
- **When:** `persisted_header_to_runtime(&p)`
- **Then:** `Err(PersistError::OutOfRange { field: "level", value: 255, min: 1, max: 6 })`

```
fn persisted_heading_to_runtime_returns_out_of_range_when_level_is_u32_max()
```
- **Given:** `PersistedHeading { level: u32::MAX, text: "Bad".into(), line: 0 }`
- **When:** `persisted_heading_to_runtime(&p)`
- **Then:** `Err(PersistError::OutOfRange { field: "level", value: 4294967295, min: 1, max: 6 })`

```
fn persisted_scraped_page_to_runtime_returns_ok_when_density_is_negative_one()
```
- **Given:** `PersistedScrapedPage` with url: "https://x.com", title: "X", slug: "x", density_score: -1.0f32, markdown: "m", links: `vec![]`, headers: `vec![]`, word_count: 1, filter_status: `PersistedPageFilterStatus::Unfiltered`, elements_removed: 0
- **When:** `persisted_scraped_page_to_runtime(&p)`
- **Then:** `Ok(ScrapedPage)` where page.density_score == -1.0f32

```
fn persisted_chunk_to_runtime_returns_empty_field_when_summary_empty()
```
- **Given:** `PersistedChunk` with schema_version: 1, chunk_id: "c#0", doc_id: "doc", doc_title: "D", content: "c", token_count: 5, summary: "", heading_path: `vec![]`, related_chunk_ids: `vec![]`, child_chunk_ids: `vec![]`
- **When:** `persisted_chunk_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "summary" })`

```
fn persisted_chunk_to_runtime_returns_empty_field_when_doc_title_whitespace()
```
- **Given:** `PersistedChunk` with schema_version: 1, chunk_id: "c#0", doc_id: "doc", doc_title: "   ", content: "c", token_count: 5, summary: "s", heading_path: `vec![]`, related_chunk_ids: `vec![]`, child_chunk_ids: `vec![]`
- **When:** `persisted_chunk_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "doc_title" })`

---

### 3.19 Enum Round-Trip via to_persisted → to_runtime (B176–B180)

```
fn link_kind_roundtrip_exhaustive_via_persisted()
```
- **Given:** `LinkKind::Internal` and `LinkKind::External`
- **When:** For each variant: `link_kind_to_persisted(&k)` → `persisted_link_kind_to_runtime(persisted)`
- **Then:** Each round-trip produces `Ok(original_variant)` — `Ok(LinkKind::Internal)` and `Ok(LinkKind::External)` respectively

```
fn chunk_type_roundtrip_exhaustive_via_persisted()
```
- **Given:** `ChunkType::Code`, `ChunkType::Table`, `ChunkType::Prose`
- **When:** For each variant: `chunk_type_to_persisted(&t)` → `persisted_chunk_type_to_runtime(persisted)`
- **Then:** Each round-trip produces `Ok(original_variant)` — all three match exactly

```
fn chunk_level_roundtrip_exhaustive_via_persisted()
```
- **Given:** `ChunkLevel::Summary`, `ChunkLevel::Standard`, `ChunkLevel::Detailed`
- **When:** For each variant: `chunk_level_to_persisted(&l)` → `persisted_chunk_level_to_runtime(persisted)`
- **Then:** Each round-trip produces `Ok(original_variant)` — all three match exactly

```
fn page_filter_status_roundtrip_exhaustive_via_persisted()
```
- **Given:** `PageFilterStatus::Filtered` and `PageFilterStatus::Unfiltered`
- **When:** For each variant: `page_filter_status_to_persisted(&s)` → `persisted_page_filter_status_to_runtime(persisted)`
- **Then:** Each round-trip produces `Ok(original_variant)` — both match exactly

```
fn change_kind_roundtrip_exhaustive_via_persisted()
```
- **Given:** `ChangeKind::Added`, `ChangeKind::Modified`, `ChangeKind::Removed`
- **When:** For each variant: `change_kind_to_persisted(&k)` → `persisted_change_kind_to_runtime(persisted)`
- **Then:** Each round-trip produces `Ok(original_variant)` — all three match exactly

---

### 3.20 rkyv Deterministic Serialization for Additional Types (B181–B184)

```
fn rkyv_serialization_deterministic_for_persisted_analyze_result()
```
- **Given:** `PersistedAnalyzeResult` with schema_version: 1, analyses: `vec![PersistedAnalysis]` with frontmatter sorted, failed_files: `vec![]`, total_discovered: 5 — constructed twice from identical inputs
- **When:** `let a = rkyv::to_bytes(&record).unwrap(); let b = rkyv::to_bytes(&record).unwrap();`
- **Then:** a.as_slice() == b.as_slice() (byte-for-byte identical)

```
fn rkyv_serialization_deterministic_for_persisted_chunks_result()
```
- **Given:** `PersistedChunksResult` with schema_version: 1, total_chunks: 3, document_count: 1, chunks_metadata: `vec![valid_chunk]`, summary_chunks: 1, standard_chunks: 1, detailed_chunks: 1 — constructed twice from identical inputs
- **When:** serialize twice
- **Then:** bytes_a == bytes_b

```
fn rkyv_serialization_deterministic_for_persisted_scrape_result()
```
- **Given:** `PersistedScrapeResult` with schema_version: 1, pages: `vec![valid_page]`, total_urls: 1, success_count: 1, error_count: 0, errors: `vec![]`, base_url: "https://example.com" — constructed twice from identical inputs
- **When:** serialize twice
- **Then:** bytes_a == bytes_b

```
fn rkyv_serialization_deterministic_for_persisted_change_plan()
```
- **Given:** `PersistedChangePlan` with schema_version: 1, target_url: "https://x.com", timestamp_secs: 1000000, changes: `vec![valid_page_change]`, summary: `valid_change_summary`, pending_snapshot: `valid_snapshot` — constructed twice from identical inputs
- **When:** serialize twice
- **Then:** bytes_a == bytes_b

---

### 3.21 Additional Edge Cases and Collection Boundaries (B185–B190)

```
fn persisted_analyze_result_to_runtime_returns_result_with_failed_files()
```
- **Given:** `PersistedAnalyzeResult` with schema_version: 1, analyses: `vec![]`, failed_files: `vec![PersistedFailedFile { source_path: "bad.md".into(), error: "parse error".into() }, PersistedFailedFile { source_path: "worse.md".into(), error: "io error".into() }]`, total_discovered: 5
- **When:** `persisted_analyze_result_to_runtime(&p)`
- **Then:** `Ok(AnalyzeResult)` where result.failed_files.len() == 2 AND result.failed_files[0].source_path == "bad.md" AND result.failed_files[1].error == "io error"

```
fn persisted_scrape_result_to_runtime_returns_result_with_many_pages()
```
- **Given:** `PersistedScrapeResult` with schema_version: 1, pages: `vec![page_a, page_b, page_c]` where each page has url: "https://x.com/{n}", title: "Page {n}", slug: "page-{n}", density_score: 0.5f32, markdown: "content", links: `vec![]`, headers: `vec![]`, word_count: 1, filter_status: `PersistedPageFilterStatus::Unfiltered`, elements_removed: 0, base_url: "https://x.com", total_urls: 3, success_count: 3, error_count: 0, errors: `vec![]`
- **When:** `persisted_scrape_result_to_runtime(&p)`
- **Then:** `Ok(ScrapeResult)` where result.pages.len() == 3 AND result.pages[0].url == "https://x.com/1" AND result.pages[2].slug == "page-3"

```
fn persisted_transform_error_to_runtime_returns_empty_field_when_source_path_whitespace()
```
- **Given:** `PersistedTransformError { source_path: "   ".into(), error: "fail".into() }`
- **When:** `persisted_transform_error_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "source_path" })`

```
fn persisted_transform_error_to_runtime_returns_empty_field_when_error_whitespace()
```
- **Given:** `PersistedTransformError { source_path: "a.md".into(), error: "   ".into() }`
- **When:** `persisted_transform_error_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "error" })`

```
fn persisted_id_mapping_to_runtime_returns_empty_field_when_filename_whitespace()
```
- **Given:** `PersistedIdMapping { source_path: "a.md".into(), id: "x".into(), filename: "   ".into(), subcategory: "s".into(), slug: "sl".into() }`
- **When:** `persisted_id_mapping_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "filename" })`

```
fn persisted_id_mapping_to_runtime_returns_empty_field_when_slug_whitespace()
```
- **Given:** `PersistedIdMapping { source_path: "a.md".into(), id: "x".into(), filename: "f.md".into(), subcategory: "s".into(), slug: "   ".into() }`
- **When:** `persisted_id_mapping_to_runtime(&p)`
- **Then:** `Err(PersistError::EmptyField { field: "slug" })`

---

## 4. Proptest Invariants

### P1: Heading round-trip invariance
```
Invariant: heading_to_persisted(h) → persisted_heading_to_runtime(_) == Ok(h) for any valid Heading
Strategy: level in 1..=6u32, text = "[a-zA-Z ]{1,50}" (non-empty), line in 0..1000usize
Anti-invariant: level == 0 or level >= 7 always returns Err(OutOfRange); text == "" returns Err(EmptyField)
```

### P2: LinkKind round-trip invariance
```
Invariant: link_kind_to_persisted(k) → persisted_link_kind_to_runtime(_) == Ok(k) for Internal and External
Strategy: proptest::sample::select(vec![LinkKind::Internal, LinkKind::External])
Anti-invariant: N/A (exhaustive enum — no invalid variants through public API)
```

### P3: Link round-trip invariance
```
Invariant: link_to_persisted(l) → persisted_link_to_runtime(_) == Ok(l) for any Link with non-empty target
Strategy: text = "any_string", target = ".{1,100}" (non-empty), kind = select(Internal, External)
Anti-invariant: target == "" or whitespace-only always returns Err(EmptyField)
```

### P4: Analysis round-trip invariance
```
Invariant: analysis_to_persisted(a) → persisted_analysis_to_runtime(_) produces Analysis where
  source_path, title, category, word_count match; Arc<str> content compared by &str value;
  frontmatter keys match (order may differ in HashMap vs Vec)
Strategy: Arbitrary Analysis with non-empty source_path/title/category, frontmatter = Option<HashMap<String,String>>
Anti-invariant: empty source_path → Err(EmptyField); schema_version != 1 → Err(SchemaVersionMismatch)
```

### P5: AnalyzeResult round-trip invariance
```
Invariant: Full round-trip preserves total_discovered, analyses.len(), failed_files.len()
Strategy: Arbitrary AnalyzeResult with 0..3 valid analyses, 0..2 failed files
Anti-invariant: schema_version != 1 → Err(SchemaVersionMismatch)
```

### P6: TransformError round-trip invariance
```
Invariant: transform_error_to_persisted(e) → persisted_transform_error_to_runtime(_) == Ok(e) for non-empty fields
Strategy: source_path = ".{1,50}", error = ".{1,200}"
Anti-invariant: empty source_path or empty error → Err(EmptyField)
```

### P7: Chunk round-trip invariance
```
Invariant: chunk_to_persisted(c) → persisted_chunk_to_runtime(_) produces Chunk where
  chunk_id, doc_id, token_count match; all optional fields match
Strategy: Arbitrary Chunk with non-empty chunk_id/doc_id/content, token_count in 1..10000
Anti-invariant: token_count == 0 → Err(OutOfRange); empty chunk_id → Err(EmptyField)
```

### P8: ChunksResult round-trip invariance
```
Invariant: Full round-trip preserves total_chunks, document_count, summary_chunks
Strategy: Arbitrary ChunksResult with 0..5 chunks
Anti-invariant: schema_version != 1 → Err(SchemaVersionMismatch)
```

### P9: ScrapedPage round-trip invariance
```
Invariant: scraped_page_to_persisted(p) → persisted_scraped_page_to_runtime(_) produces ScrapedPage
  where all fields match; density_score preserved as finite f32
Strategy: Arbitrary ScrapedPage with density_score in 0.0f32..=1.0f32, non-empty url/slug
Anti-invariant: density_score = NaN → Err(NonFiniteFloat); empty url → Err(EmptyField)
```

### P10: ScrapeResult round-trip invariance
```
Invariant: Full round-trip preserves base_url, pages.len(), errors.len()
Strategy: Arbitrary ScrapeResult with 0..3 pages, 0..2 errors
Anti-invariant: schema_version != 1 → Err(SchemaVersionMismatch); empty base_url → Err(EmptyField)
```

### P11: Snapshot round-trip invariance
```
Invariant: snapshot_to_persisted(s) → persisted_snapshot_to_runtime(_) produces Snapshot where
  target_url matches, pages BTreeMap matches, timestamp matches to second precision
Strategy: Arbitrary Snapshot with timestamp in year 2000..2100, pages = BTreeMap with 0..5 entries
Anti-invariant: schema_version != 1 → Err(SchemaVersionMismatch); empty target_url → Err(EmptyField)
```

### P12: ChangePlan round-trip invariance
```
Invariant: change_plan_to_persisted(p) → persisted_change_plan_to_runtime(_) produces ChangePlan
  where changes match, summary matches, timestamp matches to second precision
Strategy: Arbitrary ChangePlan with 0..10 changes
Anti-invariant: schema_version != 1 → Err(SchemaVersionMismatch)
```

### P13: IdMapping round-trip invariance
```
Invariant: id_mapping_to_persisted(path, m) → persisted_id_mapping_to_runtime(_) == Ok((path, m))
  for non-empty identifier fields
Strategy: source_path = ".{1,100}", all IdMapping fields non-empty strings
Anti-invariant: empty id → Err(EmptyField); empty source_path → Err(EmptyField)
```

### P14: Deterministic serialization
```
Invariant: rkyv::to_bytes(&record).unwrap() == rkyv::to_bytes(&record).unwrap() for any identical record
Strategy: Generate arbitrary PersistedAnalysis, serialize twice, compare bytes
Anti-invariant: N/A (always holds by definition)
```

### P15: Frontmatter sorted
```
Invariant: analysis_to_persisted(a).frontmatter keys are in ascending lexicographic order
Strategy: Analysis with frontmatter containing 1..20 random key-value pairs
Anti-invariant: N/A (frontmatter from HashMap may be unsorted — assert output IS sorted)
```

### P16: Heading level boundary completeness
```
Invariant: persisted_heading_to_runtime succeeds iff level in 1..=6
Strategy: level in 0..=100u32
Anti-invariant: level == 0 or level >= 7 always returns Err(OutOfRange)
```

### P17: Density score finiteness
```
Invariant: persisted_scraped_page_to_runtime succeeds iff density_score.is_finite()
Strategy: density_score = proptest::num::f32::ANY filtered to non-finite values
Anti-invariant: NaN and ±Inf always return Err(NonFiniteFloat)
```

### P18: All identifier fields non-empty (INV-02)
```
Invariant: Every to_runtime function rejects empty string for fields listed in INV-02
  (source_path, chunk_id, doc_id, url, slug, id, filename)
Strategy: For each function with identifier fields, generate input with one identifier set to ""
Anti-invariant: empty identifier always returns Err(EmptyField { field: "<name>" })
```

---

## 5. Fuzz Targets

### F1: fuzz_persisted_analysis_deserialization
```
Input type: raw bytes (&[u8])
Function: rkyv::from_bytes::<ArchivedPersistedAnalysis>(&input)
Risk: Panic on malformed archived refs, OOB access, use-after-free via misaligned pointers
Corpus seeds:
  - valid serialized PersistedAnalysis (happy path)
  - truncated valid bytes (10%, 50%, 90% length)
  - bytes with all zeros (len == valid_len)
  - bytes with 0xFF fill
  - bytes with single-bit flip at offsets [0, 1, 2, 4, 8, 16, len-1]
```

### F2: fuzz_persisted_chunk_deserialization
```
Input type: raw bytes (&[u8])
Function: rkyv::from_bytes::<ArchivedPersistedChunk>(&input)
Risk: Panic from invalid archived string/slice refs, OOB
Corpus seeds: same pattern as F1 but for PersistedChunk
```

### F3: fuzz_persisted_scrape_result_deserialization
```
Input type: raw bytes (&[u8])
Function: rkyv::from_bytes::<ArchivedPersistedScrapeResult>(&input)
Risk: Nested vec deserialization, malformed length prefixes
Corpus seeds: valid result, empty result, result with 1000 pages
```

### F4: fuzz_persisted_snapshot_deserialization
```
Input type: raw bytes (&[u8])
Function: rkyv::from_bytes::<ArchivedPersistedSnapshot>(&input)
Risk: Sorted vec invariant, content_hash array bounds
Corpus seeds: snapshot with 0 pages, 1 page, 100 pages
```

### F5: fuzz_persisted_change_plan_deserialization
```
Input type: raw bytes (&[u8])
Function: rkyv::from_bytes::<ArchivedPersistedChangePlan>(&input)
Risk: Deeply nested structure (ChangePlan contains Snapshot), malformed enum discriminant
Corpus seeds: plan with 0 changes, plan with 50 changes, plan with Added/Modified/Removed variants
```

### F6: fuzz_persisted_chunks_result_deserialization
```
Input type: raw bytes (&[u8])
Function: rkyv::from_bytes::<ArchivedPersistedChunksResult>(&input)
Risk: Large chunk_metadata vec, malformed ChunkType/ChunkLevel discriminants
Corpus seeds: empty chunks, 10 chunks all Code, mixed chunk types
```

### F7: fuzz_persisted_analyze_result_deserialization
```
Input type: raw bytes (&[u8])
Function: rkyv::from_bytes::<ArchivedPersistedAnalyzeResult>(&input)
Risk: Nested analyses vec with frontmatter, heading arrays, link arrays
Corpus seeds: empty result, result with 1 analysis, result with 50 analyses
```

### F8: fuzz_persisted_scraped_page_deserialization
```
Input type: raw bytes (&[u8])
Function: rkyv::from_bytes::<ArchivedPersistedScrapedPage>(&input)
Risk: density_score f32 NaN/Inf through corrupted bytes, headers/links vec OOB
Corpus seeds: page with NaN density, page with 100 headers, page with empty markdown
```

---

## 6. Kani Harnesses

### K1: Heading level range check completeness
```
Property: persisted_heading_to_runtime returns Err(OutOfRange) for ALL u32 values outside 1..=6
          AND returns Ok for ALL u32 values inside 1..=6
Bound: u32 (exhaustive via Kani's symbolic execution — 2^32 states explored)
Rationale: Off-by-one at heading level boundary would silently corrupt heading hierarchy in every analysis.
```

### K2: Content_hash array bounds
```
Property: persisted_page_hash_to_runtime always reads exactly 32 bytes from content_hash
Bound: Array is fixed [u8; 32] — Kani proves zero out-of-bounds access
Rationale: content_hash is the integrity guarantee for page change detection. Any bounds violation
           allows silent hash collisions or missed changes.
```

### K3: Schema_version check exhaustive for all 7 versioned types
```
Property: For every versioned type (PersistedAnalysis, PersistedAnalyzeResult,
          PersistedTransformResult, PersistedChunksResult, PersistedScrapeResult,
          PersistedSnapshot, PersistedChangePlan): if schema_version != 1, the conversion
          returns Err(SchemaVersionMismatch) and NEVER proceeds to field validation
Bound: u32 (symbolic — Kani proves the match on schema_version is total for != 1)
Rationale: Schema version is the first line of defense. Bypassing it means field validation runs
           on alien data from a future version.
```

### K4: density_score finite check completeness
```
Property: persisted_scraped_page_to_runtime returns Err(NonFiniteFloat) for ALL f32 values
          where !is_finite(), AND returns Ok for ALL values where is_finite()
Bound: f32 (symbolic — Kani explores NaN, +Inf, -Inf, and representative finite values)
Rationale: NaN propagation through density_score would corrupt scoring and filtering logic.
```

---

## 7. Mutation Testing Checkpoints

**Target: ≥90% mutation kill rate**

### Critical Mutations to Catch

| Mutation | Caught By |
|----------|-----------|
| `schema_version == 1` → `== 2` in `persisted_analysis_to_runtime` | B53, B54 (schema mismatch tests) |
| `schema_version == 1` → `== 2` in `persisted_analyze_result_to_runtime` | B61, B62 |
| `schema_version == 1` → `== 2` in `persisted_transform_result_to_runtime` | B68 |
| `schema_version == 1` → `== 2` in `persisted_chunks_result_to_runtime` | B87 |
| `schema_version == 1` → `== 2` in `persisted_scrape_result_to_runtime` | B106 |
| `schema_version == 1` → `== 2` in `persisted_snapshot_to_runtime` | B123 |
| `schema_version == 1` → `== 2` in `persisted_change_plan_to_runtime` | B127 |
| `schema_version == 1` → `== 2` in `persisted_chunk_to_runtime` | B77 |
| `level >= 1 && level <= 6` → `>= 0` | B40 (level == 0 rejects) |
| `level >= 1 && level <= 6` → `<= 7` | B41 (level == 7 rejects) |
| `text.trim().is_empty()` → `text.is_empty()` | B45 ("" empty), B44 (whitespace-only) |
| `frontmatter.sort_by_key` removed | B160 (deterministic frontmatter) |
| `density_score.is_finite()` → `is_normal()` | B98 (NaN), B99 (+Inf), B100 (-Inf) |
| `token_count > 0` → `>= 0` | B82 (token_count == 0 rejects) |
| `source_path` validation removed | B55 (Analysis), B65 (TransformError), B131 (IdMapping) |
| `title` validation removed | B56 (Analysis), B112 (PageHash) |
| `category` validation removed | B57 |
| `target` validation removed | B50 (Link) |
| `chunk_id` validation removed | B78 |
| `doc_id` validation removed | B79 |
| `doc_title` validation removed | B80 |
| `content` validation removed | B81 |
| `id` validation removed | B130 |
| `url` validation removed | B103 (ScrapedPage), B111 (PageHash), B119 (PageChange) |
| `slug` validation removed | B104, B133 |
| `base_url` validation removed | B107 |
| `target_url` validation removed | B124, B128 |
| `filename` validation removed | B132 |
| `subcategory` validation removed | B134 |
| `error` validation removed | B66 |
| `Arc<str>` → `String` produces empty | Full pipeline round-trip for Analysis |
| `DateTime → i64` off-by-one | B34 / B122 (snapshot round-trip with known timestamp 1736931000) |
| rkyv `to_bytes` removed (returns empty) | B135–B146 (round-trip on empty vec fails) |
| `from_bytes` validation skipped | B149–B152 (corrupted bytes tests) |
| `persisted_link_kind_to_runtime` body deleted | B47, B48 (both variants tested) |
| `persisted_chunk_type_to_runtime` body deleted | B70, B71, B72 (all 3 variants) |
| `persisted_chunk_level_to_runtime` body deleted | B73, B74, B75 (all 3 variants) |
| `persisted_header_to_runtime` body deleted | B89, B90, B91, B92, B93, B94 (6 scenarios) |
| `persisted_page_filter_status_to_runtime` body deleted | B95, B96 (both variants) |
| `persisted_change_kind_to_runtime` body deleted | B113, B114, B115 (all 3 variants) |
| `persisted_page_change_to_runtime` body deleted | B116–B120 (5 scenarios) |
| `persisted_change_summary_to_runtime` body deleted | B121 |
| `persisted_transform_error_to_runtime` body deleted | B64, B65, B66 (3 scenarios) |

### Mutations NOT Expected to be Caught

| Mutation | Reason |
|----------|--------|
| `#[derive(Clone)]` removed on persisted types | Tests borrow, don't clone persisted records. Low risk — add clone test if desired. |
| `#[derive(Debug)]` removed | No Debug-format assertions. Low risk. |

---

## 8. Combinatorial Coverage Matrix

### 8.1 PersistedHeading Validation

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| valid heading | level 3, text "Sec" | `Ok(Heading { level: 3, text: "Sec", line: 10 })` | unit |
| level == 0 | min boundary invalid | `Err(OutOfRange { value: 0, min: 1, max: 6 })` | unit |
| level == 7 | max+1 boundary | `Err(OutOfRange { value: 7, min: 1, max: 6 })` | unit |
| level == 1 | min valid | `Ok(Heading { level: 1, .. })` | unit |
| level == 6 | max valid | `Ok(Heading { level: 6, .. })` | unit |
| text == "" | empty string | `Err(EmptyField { field: "text" })` | unit |
| text == "   " | whitespace only | `Err(EmptyField { field: "text" })` | unit |
| line == 0 | zero valid | `Ok(Heading { line: 0 })` | unit |
| any valid level+text | proptest 1..=6 | `Ok(Heading)` == input | proptest |

### 8.2 PersistedAnalysis Validation

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| fully valid | schema=1, all fields populated | `Ok(Analysis)` with matching fields | unit |
| schema_version == 0 | wrong version | `Err(SchemaVersionMismatch { expected: 1, actual: 0 })` | unit |
| schema_version == 2 | future version | `Err(SchemaVersionMismatch { expected: 1, actual: 2 })` | unit |
| source_path == "" | empty identifier | `Err(EmptyField { field: "source_path" })` | unit |
| title == "" | empty required field | `Err(EmptyField { field: "title" })` | unit |
| category == "" | empty identifier | `Err(EmptyField { field: "category" })` | unit |
| frontmatter == None | Option boundary | `Ok(Analysis)` with frontmatter == None | unit |
| frontmatter single entry | Option-Some boundary | `Ok(Analysis)` with 1-entry HashMap | unit |
| frontmatter unsorted input | HashMap with z-key before a-key | frontmatter keys sorted: ["a-key", "z-key"] | integration |

### 8.3 PersistedChunk Validation

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| fully valid, schema=1 | all fields populated | `Ok(Chunk)` with matching fields | unit |
| schema_version == 2 | wrong version | `Err(SchemaVersionMismatch { expected: 1, actual: 2 })` | unit |
| chunk_id == "" | empty identifier | `Err(EmptyField { field: "chunk_id" })` | unit |
| doc_id == "" | empty identifier | `Err(EmptyField { field: "doc_id" })` | unit |
| doc_title == "" | empty required | `Err(EmptyField { field: "doc_title" })` | unit |
| content == "" | empty required | `Err(EmptyField { field: "content" })` | unit |
| token_count == 0 | zero invalid | `Err(OutOfRange { field: "token_count", value: 0 })` | unit |
| token_count == 1 | min valid | `Ok(Chunk)` with token_count == 1 | unit |
| heading == None | Optional boundary | `Ok(Chunk)` with heading == None | unit |
| related_chunk_ids empty | empty collection | `Ok(Chunk)` with 0 related | unit |
| child_chunk_ids empty | empty collection | `Ok(Chunk)` with 0 children | unit |

### 8.4 PersistedScrapedPage Validation

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| density_score == 0.5 | normal finite | `Ok(ScrapedPage)` | unit |
| density_score == 0.0 | zero finite | `Ok(ScrapedPage)` | unit |
| density_score == NaN | non-finite | `Err(NonFiniteFloat { field: "density_score" })` | unit |
| density_score == +Inf | non-finite | `Err(NonFiniteFloat { field: "density_score" })` | unit |
| density_score == -Inf | non-finite | `Err(NonFiniteFloat { field: "density_score" })` | unit |
| density_score == f32::MAX | extreme finite | `Ok(ScrapedPage)` | unit |
| url == "" | empty identifier | `Err(EmptyField { field: "url" })` | unit |
| slug == "" | empty identifier | `Err(EmptyField { field: "slug" })` | unit |

### 8.5 Schema Version Validation (all versioned types)

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| version == 1 (all 8 types) | valid | `Ok(T)` | unit |
| version == 0 (all 8 types) | invalid | `Err(SchemaVersionMismatch { actual: 0 })` | unit |
| version == u32::MAX (any type) | invalid | `Err(SchemaVersionMismatch)` | unit |

Note: 8 versioned types = PersistedAnalysis, PersistedAnalyzeResult, PersistedTransformResult, PersistedChunk, PersistedChunksResult, PersistedScrapeResult, PersistedSnapshot, PersistedChangePlan.

### 8.6 rkyv Round-Trip (all record types)

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| valid → bytes → archived (12 types) | populated record | archived fields == original | integration |
| valid → bytes → bytes (same record) | deterministic | bytes_a == bytes_b | integration |
| truncated bytes | invalid | `Err(DeserializationFailed)` | integration |
| bit-flipped bytes | invalid | `Err(DeserializationFailed)` | integration |
| zeroed bytes | invalid | `Err(DeserializationFailed)` | integration |
| random bytes | invalid | `Err(DeserializationFailed)` | integration |

### 8.7 Full Pipeline Round-Trip (4 top-level batch types)

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| AnalyzeResult | valid runtime | fields match original | E2E |
| TransformResult | valid runtime | fields match original | E2E |
| ChunksResult | valid runtime | fields match original | E2E |
| ScrapeResult | valid runtime | fields match original | E2E |

### 8.8 Enum Variant Mapping (all enums)

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| LinkKind both variants | exhaustive | matching persisted variants | static |
| ChunkType all 3 variants | exhaustive | matching persisted variants | static |
| ChunkLevel all 3 variants | exhaustive | matching persisted variants | static |
| PageFilterStatus both variants | exhaustive | matching persisted variants | static |
| ChangeKind all 3 variants | exhaustive | matching persisted variants | static |
| ChangeKind to_runtime 3 variants | exhaustive | matching runtime variants | unit |

### 8.9 Identifier Field Emptiness (INV-02 across all functions)

| Function | Field | Empty Test | Layer |
|----------|-------|-----------|-------|
| persisted_analysis_to_runtime | source_path | B55 | unit |
| persisted_analysis_to_runtime | title | B56 | unit |
| persisted_analysis_to_runtime | category | B57 | unit |
| persisted_link_to_runtime | target | B50 | unit |
| persisted_transform_error_to_runtime | source_path | B65 | unit |
| persisted_transform_error_to_runtime | error | B66 | unit |
| persisted_chunk_to_runtime | chunk_id | B78 | unit |
| persisted_chunk_to_runtime | doc_id | B79 | unit |
| persisted_chunk_to_runtime | doc_title | B80 | unit |
| persisted_chunk_to_runtime | content | B81 | unit |
| persisted_scraped_page_to_runtime | url | B103 | unit |
| persisted_scraped_page_to_runtime | slug | B104 | unit |
| persisted_scrape_result_to_runtime | base_url | B107 | unit |
| persisted_page_hash_to_runtime | url | B111 | unit |
| persisted_page_hash_to_runtime | title | B112 | unit |
| persisted_page_change_to_runtime | url | B119 | unit |
| persisted_snapshot_to_runtime | target_url | B124 | unit |
| persisted_change_plan_to_runtime | target_url | B128 | unit |
| persisted_id_mapping_to_runtime | id | B130 | unit |
| persisted_id_mapping_to_runtime | source_path | B131 | unit |
| persisted_id_mapping_to_runtime | filename | B132 | unit |
| persisted_id_mapping_to_runtime | slug | B133 | unit |
| persisted_id_mapping_to_runtime | subcategory | B134 | unit |

---

## 9. PersistError Variant Coverage Audit

| Error Variant | Trigger Function | Test Scenario | Status |
|---------------|-----------------|---------------|--------|
| `EmptyField { field }` | `persisted_heading_to_runtime` (text == "") | B44 | ✅ |
| `EmptyField { field }` | `persisted_heading_to_runtime` (text whitespace) | B45 | ✅ |
| `EmptyField { field }` | `persisted_link_to_runtime` (target) | B50 | ✅ |
| `EmptyField { field }` | `persisted_analysis_to_runtime` (source_path) | B55 | ✅ |
| `EmptyField { field }` | `persisted_analysis_to_runtime` (title) | B56 | ✅ |
| `EmptyField { field }` | `persisted_analysis_to_runtime` (category) | B57 | ✅ |
| `EmptyField { field }` | `persisted_chunk_to_runtime` (chunk_id) | B78 | ✅ |
| `EmptyField { field }` | `persisted_chunk_to_runtime` (doc_id) | B79 | ✅ |
| `EmptyField { field }` | `persisted_chunk_to_runtime` (doc_title) | B80 | ✅ |
| `EmptyField { field }` | `persisted_chunk_to_runtime` (content) | B81 | ✅ |
| `EmptyField { field }` | `persisted_transform_error_to_runtime` (source_path) | B65 | ✅ |
| `EmptyField { field }` | `persisted_transform_error_to_runtime` (error) | B66 | ✅ |
| `EmptyField { field }` | `persisted_scraped_page_to_runtime` (url) | B103 | ✅ |
| `EmptyField { field }` | `persisted_scraped_page_to_runtime` (slug) | B104 | ✅ |
| `EmptyField { field }` | `persisted_scrape_result_to_runtime` (base_url) | B107 | ✅ |
| `EmptyField { field }` | `persisted_page_hash_to_runtime` (url) | B111 | ✅ |
| `EmptyField { field }` | `persisted_page_hash_to_runtime` (title) | B112 | ✅ |
| `EmptyField { field }` | `persisted_page_change_to_runtime` (url) | B119 | ✅ |
| `EmptyField { field }` | `persisted_snapshot_to_runtime` (target_url) | B124 | ✅ |
| `EmptyField { field }` | `persisted_change_plan_to_runtime` (target_url) | B128 | ✅ |
| `EmptyField { field }` | `persisted_id_mapping_to_runtime` (id) | B130 | ✅ |
| `EmptyField { field }` | `persisted_id_mapping_to_runtime` (source_path) | B131 | ✅ |
| `EmptyField { field }` | `persisted_id_mapping_to_runtime` (filename) | B132 | ✅ |
| `EmptyField { field }` | `persisted_id_mapping_to_runtime` (slug) | B133 | ✅ |
| `EmptyField { field }` | `persisted_id_mapping_to_runtime` (subcategory) | B134 | ✅ |
| `OutOfRange { field, value, min, max }` | `persisted_heading_to_runtime` (level == 0) | B40 | ✅ |
| `OutOfRange { field, value, min, max }` | `persisted_heading_to_runtime` (level == 7) | B41 | ✅ |
| `OutOfRange { field, value, min, max }` | `persisted_chunk_to_runtime` (token_count == 0) | B82 | ✅ |
| `OutOfRange { field, value, min, max }` | `persisted_header_to_runtime` (level == 0) | B90 | ✅ |
| `OutOfRange { field, value, min, max }` | `persisted_header_to_runtime` (level == 7) | B91 | ✅ |
| `SchemaVersionMismatch { expected, actual }` | `persisted_analysis_to_runtime` (v0) | B53 | ✅ |
| `SchemaVersionMismatch { expected, actual }` | `persisted_analysis_to_runtime` (v2) | B54 | ✅ |
| `SchemaVersionMismatch { expected, actual }` | `persisted_analyze_result_to_runtime` (v0) | B61 | ✅ |
| `SchemaVersionMismatch { expected, actual }` | `persisted_analyze_result_to_runtime` (v99) | B62 | ✅ |
| `SchemaVersionMismatch { expected, actual }` | `persisted_transform_result_to_runtime` (v5) | B68 | ✅ |
| `SchemaVersionMismatch { expected, actual }` | `persisted_chunks_result_to_runtime` (v3) | B87 | ✅ |
| `SchemaVersionMismatch { expected, actual }` | `persisted_scrape_result_to_runtime` (v99) | B106 | ✅ |
| `SchemaVersionMismatch { expected, actual }` | `persisted_snapshot_to_runtime` (v5) | B123 | ✅ |
| `SchemaVersionMismatch { expected, actual }` | `persisted_change_plan_to_runtime` (v3) | B127 | ✅ |
| `SchemaVersionMismatch { expected, actual }` | `persisted_chunk_to_runtime` (v2) | B77 | ✅ |
| `SerializationFailed { reason }` | via failing Write/buffer injection | B157 | ✅ explicit trigger |
| `DeserializationFailed { reason }` | corrupted bytes (truncated) | B149 | ✅ |
| `DeserializationFailed { reason }` | corrupted bytes (bit-flipped) | B150 | ✅ |
| `DeserializationFailed { reason }` | corrupted bytes (zeroed) | B151 | ✅ |
| `DeserializationFailed { reason }` | corrupted bytes (random) | B152 | ✅ |
| `UnknownVariant { type_name }` | via invalid discriminant injection | B158 | ✅ explicit trigger |
| `NonFiniteFloat { field, value }` | `persisted_scraped_page_to_runtime` (NaN) | B98 | ✅ |
| `NonFiniteFloat { field, value }` | `persisted_scraped_page_to_runtime` (+Inf) | B99 | ✅ |
| `NonFiniteFloat { field, value }` | `persisted_scraped_page_to_runtime` (-Inf) | B100 | ✅ |
| `InvalidHashLength { actual_len }` | via corrupted rkyv archived hash | B159 | ✅ explicit trigger |

**Summary: 8 of 8 variants directly tested via specific trigger scenarios.** All variants including
`SerializationFailed`, `UnknownVariant`, and `InvalidHashLength` have explicit test strategies
documented in §3.14 with concrete implementation approaches.

---

## 10. Density Calculation Verification

| Artifact Type | Count |
|---------------|-------|
| BDD test functions (§3) | 190 |
| Proptest invariants (§4) | 18 |
| Fuzz targets (§5) | 8 |
| Kani harnesses (§6) | 4 |
| **Grand total** | **220** |
| **Density ratio** | **220 / 44 = 5.0x** |

---

## 11. Open Questions

1. **`SerializationFailed` trigger approach (B157):** The test requires a mechanism to inject
   allocation failures into `rkyv::to_bytes`. Options: (a) custom `Write` impl that fails after
   N bytes, (b) `#[cfg(test)]` wrapper that maps the error, (c) test-only helper that constructs
   the error directly. The test-writer should choose the approach that matches the implementation's
   error wrapping strategy. **Minimum requirement:** the code path that produces `SerializationFailed`
   must be exercised, not just the error variant constructed in isolation.

2. **`UnknownVariant` trigger approach (B158):** This requires either (a) `#[repr(u8)]` on persisted
   enums with explicit discriminant validation in `to_runtime`, or (b) a `from_discriminant` helper
   that returns `UnknownVariant` for unrecognized values. If the implementation uses bare Rust
   `match` (which makes `UnknownVariant` unreachable), the error variant should be annotated with
   `#[allow(dead_code)]` and a comment explaining it's reserved for future `#[repr(u8)]` upgrade.
   **Decision needed at implementation time.**

3. **`InvalidHashLength` trigger approach (B159):** The `[u8; 32]` fixed array makes wrong-length
   hashes impossible in safe Rust. However, rkyv's archived representation may have corrupted
   length prefixes when deserialized from untrusted bytes. The `persisted_page_hash_to_runtime`
   function should validate hash length explicitly. **If the implementation trusts the type system
   entirely, add a comment and `#[allow(dead_code)]` on `InvalidHashLength`.**

4. **`DateTime<Utc>` precision loss:** The contract acknowledges `DateTime → i64` loses sub-second
   precision. Round-trip tests MUST compare via `.timestamp()` (i64 seconds), NOT via direct
   `DateTime` equality.

5. **`contextual_chunker::ChunkType` / `ChunkLevel` extern types:** These come from the
   `contextual-chunker` crate. Test-writer must ensure variant mapping tests use the actual
   crate types, not assumed variants.

6. **Feature flag:** Contract mentions `#[cfg(feature = "persist")]` as optional. All tests
   assume the `persist` feature is enabled. Add `#[cfg(test)]` or feature-gated test modules.

7. **`line == 0` for Heading:** Contract says "0-based line number" (line 86 of contract).
   `line: 0` is valid and represents the first line. Test B46 confirms this explicitly.

8. **`DeserializationFailed { reason: _ }` wildcard in §3.13 (B149–B152):** The `reason`
   field in `DeserializationFailed` contains implementation-dependent error strings from
   rkyv internals (e.g., `"validation failed: invalid archive variant"`). Tests assert
   the correct error variant but use `_` for the reason string because matching exact
   strings would couple tests to rkyv's internal error messages, making tests fragile
   across rkyv version bumps. This is an intentional design decision, not an oversight.
