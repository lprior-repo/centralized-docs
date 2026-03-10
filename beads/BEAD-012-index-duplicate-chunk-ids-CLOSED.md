# BEAD-012: INDEX.json Contains 372 Duplicate Chunk IDs

**Epic**: Data Integrity
**Severity**: Critical
**Status**: Open

---

## CONTEXT BLOCK

- **File/Function**: `doc_transformer/src/index.rs` (INDEX.json generation)
- **The Smell**: The `index` command reports "COMPLETE" with a nice ASCII banner, but the generated INDEX.json contains 372 duplicate chunk entries. Every chunk appears 2-3 times in the chunks array. The validator catches this but the tool itself treats the output as valid.

**Evidence**:
```bash
$ ./target/release/doc_transformer index ./cue_docs ./output
# [STEP 6] INDEX + GRAPH: Created INDEX.json and COMPASS.md
# [STEP 7] LLMS.TXT + AGENTS.MD: Created llms.txt, llms-full.txt, and AGENTS.md
# [STEP 8] VALIDATE: 33/36 files passed (3 errors, 2 warnings)
# ======================================================================
# COMPLETE
# ======================================================================

$ ./target/release/llms_txt_validator --index ./output/INDEX.json
# 📊 Found 372 errors, 0 warnings, 0 info
# ❌ [ERROR] chunks: Duplicate chunk ID: ops/general/community#0
# ❌ [ERROR] chunks: Duplicate chunk ID: ops/general/community#1
# (repeated 372 times)
# ============================================================
# ❌ Validation failed
```

**User Impact**:
- AI agents consuming this index see duplicate results
- Wasted tokens on redundant content
- Bad decisions based on corrupted data
- Search quality is severely degraded
- Knowledge graph relationships point to duplicate chunks

---

## SPECIFICATION BLOCK

### 1. EARS (Easy Approach to Requirements Syntax)

| Trigger | System | Response |
|---------|--------|----------|
| Generating INDEX.json | `index` command | Ensure each chunk ID appears exactly once in chunks array |
| Adding a chunk to INDEX.json | `index_chunks` function | Check if chunk_id already exists, skip or update if found |
| Validation runs | `llms_txt_validator` | Fail hard on duplicates, return non-zero exit code |

### 2. DbC (Design by Contract)

**Preconditions**:
- Input documents have unique doc_ids
- Each document generates chunks with sequential IDs (doc_id#0, doc_id#1, etc.)

**Postconditions**:
- INDEX.json chunks array contains no duplicate chunk_ids
- Each chunk_id maps to exactly one chunk entry
- Validation passes with zero duplicate errors
- If duplicates are detected during generation, fail early with error message

**Invariants**:
- Total chunks count = sum of chunks per document
- No two chunks share the same chunk_id
- Chunk IDs follow pattern `{doc_id}#{index}`

### 3. Schema & Edge Cases

**Root Cause Analysis**:
The issue is likely in how chunks are accumulated and written to INDEX.json. Possible scenarios:

1. **Double-loop bug**: Chunks are being added to the index array twice
2. **Document reprocessing**: Same documents are being processed multiple times
3. **Chunk deduplication missing**: No check for existing chunk_id before insertion

**Expected INDEX.json Structure**:
```json
{
  "chunks": [
    {
      "chunk_id": "docs-introduction#0",
      "doc_id": "docs-introduction",
      // ... appears exactly once
    }
  ],
  // ... no duplicates
}
```

**Edge Cases to Handle**:
| Scenario | Expected Behavior |
|----------|-------------------|
| Document generates 5 chunks | Index contains 5 unique entries |
| Same doc_id processed twice | Detect and error, don't create duplicates |
| Chunk collision (same ID) | Fail with error "Duplicate chunk_id: {id}" |
| Empty chunks array | Skip or warn, don't create empty entries |

---

## FIX LOCATIONS

1. **`doc_transformer/src/index.rs`** - Investigate `index_chunks()` or similar function
   - Find where chunks are added to the index structure
   - Add deduplication logic before insertion
   - Use a `HashSet<ChunkId>` to track seen chunks

2. **`doc_transformer/src/main.rs`** - Add validation step
   - After INDEX.json is written, run internal duplicate check
   - Fail fast if duplicates detected
   - Don't report "COMPLETE" if data is corrupted

3. **`llms_txt_validator.rs`** - Make validator strict
   - Change exit code to non-zero on duplicate errors
   - Remove soft-fail behavior on duplicates

---

## TEST CASES

```rust
#[test]
fn test_no_duplicate_chunk_ids() {
    // Generate index from sample docs
    let index = generate_index("./test_docs");

    // Collect all chunk_ids
    let mut chunk_ids = std::collections::HashSet::new();
    for chunk in &index.chunks {
        assert!(
            chunk_ids.insert(&chunk.chunk_id),
            "Duplicate chunk_id found: {}",
            chunk.chunk_id
        );
    }

    // Verify total count matches unique count
    assert_eq!(index.chunks.len(), chunk_ids.len());
}

#[test]
fn test_duplicate_detection_fails_fast() {
    // Test data with duplicate doc_id processing
    let result = run_index_command("./duplicate_test_docs");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Duplicate chunk_id"));
}

#[test]
fn test_validator_returns_nonzero_on_duplicates() {
    // Generate index with duplicates (artificially corrupted)
    create_corrupted_index_with_duplicates();

    let output = Command::new("./target/release/llms_txt_validator")
        .args(["--index", "./corrupted/INDEX.json"])
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Duplicate chunk ID"));
}
```

---

## VERIFICATION

After fix:
```bash
$ ./target/release/doc_transformer index ./cue_docs ./output
# [STEP 6] INDEX + GRAPH: Created INDEX.json with 1202 unique chunks
# [STEP 8] VALIDATE: 36/36 files passed
# ======================================================================
# COMPLETE
# ======================================================================

$ ./target/release/llms_txt_validator --index ./output/INDEX.json
# 📊 Found 0 errors, 0 warnings, 0 info
# ============================================================
# ✅ Validation passed

$ ./target/release/doc_transformer search "installation" --index-dir ./output
# Results: 1 unique result (not duplicated)
```
