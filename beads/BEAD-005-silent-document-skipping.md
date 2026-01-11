# BEAD-005: Silent Document Skipping When link_map Entry Missing

**Epic**: Data Integrity
**Severity**: Medium
**Status**: Open

---

## CONTEXT BLOCK

- **File/Function**: Multiple locations silently skip documents:
  - `doc_transformer/src/llms.rs:70` (`generate_llms_txt`)
  - `doc_transformer/src/index.rs:66` (`build_and_write_index`)
  - `doc_transformer/src/index.rs:211` (`build_and_write_compass`)
  - `doc_transformer/src/transform.rs:41` (`transform_all`)
- **The Smell**: When a document's `source_path` is not found in `link_map`, it is silently skipped with `if let Some(mapping) = link_map.get(...)`. This can cause data loss without any warning to the user. The final report shows "1 document processed" but the user doesn't know which documents were silently ignored.

**Example**:
```rust
for analysis in analyses {
    if let Some(mapping) = link_map.get(&analysis.source_path) {
        // Process document
    }
    // Documents without mappings are silently dropped!
}
```

---

## SPECIFICATION BLOCK

### 1. EARS (Easy Approach to Requirements Syntax)

| Trigger | System | Response |
|---------|--------|----------|
| Document has no link_map entry | All index/transform functions | Log warning with document path |
| Multiple documents skipped | Final report | Include count of skipped documents |
| Document processing fails | Transform step | Continue processing, log error, track failure |

### 2. DbC (Design by Contract)

**Preconditions**:
- `analyses` contains all discovered documents
- `link_map` is populated from `assign_ids`

**Postconditions**:
- Every document in `analyses` is either processed OR explicitly logged as skipped
- User sees warning if any documents were skipped
- Final summary includes "Documents: X processed, Y skipped"

**Invariants**:
- Number of processed + skipped + errored = total documents
- No silent data loss

### 3. Schema & Edge Cases

**Verbose Processing Pattern**:
```rust
pub fn transform_all(
    analyses: &[Analysis],
    link_map: &HashMap<String, IdMapping>,
    output_dir: &Path,
) -> Result<TransformResult> {
    let mut success_count: usize = 0;
    let mut error_count: usize = 0;
    let mut skipped_count: usize = 0;
    let mut skipped_paths: Vec<String> = Vec::new();

    for analysis in analyses {
        match link_map.get(&analysis.source_path) {
            Some(mapping) => {
                match transform_file(analysis, mapping, link_map, &docs_dir) {
                    Ok(_) => success_count = success_count.saturating_add(1),
                    Err(e) => {
                        eprintln!("TRANSFORM ERROR: {}: {}", analysis.source_path, e);
                        error_count = error_count.saturating_add(1);
                    }
                }
            }
            None => {
                skipped_count = skipped_count.saturating_add(1);
                skipped_paths.push(analysis.source_path.clone());
                eprintln!("WARNING: No ID mapping for {}", analysis.source_path);
            }
        }
    }

    if !skipped_paths.is_empty() {
        eprintln!("WARNING: {} documents skipped (no ID mapping)", skipped_count);
    }

    Ok(TransformResult {
        success_count,
        total_count: analyses.len(),
        error_count,
        skipped_count,  // Add this field
    })
}
```

**Edge Cases**:
| Scenario | Expected Behavior |
|----------|-------------------|
| All documents have mappings | No warnings, process all |
| One document missing mapping | Warning logged, continue processing others |
| All documents missing mappings | Warning for each, final summary shows 0 processed |
| link_map is empty | All documents skipped with warnings |

---

## FIX LOCATIONS

1. `src/transform.rs:40-49` - Add skipped tracking and logging
2. `src/index.rs:65-101` - Add skipped tracking for index building
3. `src/llms.rs:69-76` - Add skipped tracking for llms.txt generation
4. All `TransformResult`, `IndexResult` structs - Add `skipped_count` field
5. `src/main.rs` final summary - Include skipped count

---

## TEST CASES

```rust
#[test]
fn test_transform_logs_skipped_documents() {
    let analyses = vec![
        Analysis { source_path: "exists.md".to_string(), ..Default::default() },
        Analysis { source_path: "missing.md".to_string(), ..Default::default() },
    ];
    let mut link_map = HashMap::new();
    link_map.insert("exists.md".to_string(), IdMapping::default());
    // "missing.md" has no entry

    let result = transform_all(&analyses, &link_map, Path::new("/tmp/out")).unwrap();
    assert_eq!(result.success_count, 1);
    assert_eq!(result.skipped_count, 1);  // New field
}
```
