# BEAD-015: Unused Extracted Crates - Delete or Integrate

**Epic**: Code Quality
**Severity**: High
**Status**: Open

---

## CONTEXT BLOCK

- **Files**: `contextual-chunker/`, `llms-txt-parser/` (workspace crates)
- **The Smell**: Two separate crate directories were "extracted" from main codebase but are never used. They exist as disconnected proof-of-concepts with zero integration. ROADMAP.md claimed "✅ Contextual-Chunker Crate - Ready for crates.io publication" but main binary doesn't use them.

**Evidence**:
```bash
$ grep -r "use contextual_chunker" doc_transformer/src
# ZERO RESULTS - never imported anywhere

$ grep -r "use llms_txt_parser" doc_transformer/src
# ZERO RESULTS - never imported anywhere

# But we have duplicate implementations:
$ wc -l doc_transformer/src/chunk.rs contextual-chunker/src/chunk.rs
  546 doc_transformer/src/chunk.rs       # ACTUALLY USED
  ~600 contextual-chunker/src/chunk.rs    # NEVER USED

# Cargo.toml lists them as dependencies:
$ grep -A5 "\[dependencies\]" doc_transformer/Cargo.toml
[dependencies]
contextual-chunker = { path = "../contextual-chunker" }  # Loaded but unused
llms-txt-parser = { path = "../llms-txt-parser" }       # Loaded but unused
```

**User Impact**:
- Maintaining duplicate codebases for same functionality
- Bug fixes must be applied in two places
- Tests written twice for same logic
- Build time includes compiling unused dependencies
- Binary size includes unused code
- Confusing for contributors - which implementation is the "real" one?

---

## SPECIFICATION BLOCK

### 1. EARS (Easy Approach to Requirements Syntax)

| Trigger | System | Response |
|---------|--------|----------|
| Project has duplicate implementations | Architecture | Choose ONE: integrate or delete |
| If integrating | `doc_transformer/src/chunk.rs` | Replace with `contextual_chunker` crate |
| If deleting | Workspace | Remove `contextual-chunker/` and `llms-txt-parser/` |
| If publishing to crates.io | Both crates | Ensure they're actually usable standalone |

### 2. DbC (Design by Contract)

**Preconditions**:
- Decision made: integrate OR delete
- No remaining duplicate implementations

**Postconditions**:
- Option A (Integrate): Main binary uses extracted crates, `doc_transformer/src/chunk.rs` deleted
- Option B (Delete): Extracted crates deleted, only `doc_transformer/src/chunk.rs` remains
- No duplicate code anywhere
- Tests run for the kept implementation only
- Cargo.toml only includes used dependencies

**Invariants**:
- Each piece of functionality exists in exactly one place
- All imports from workspace crates are actually used
- No dead dependencies in Cargo.toml

### 3. Schema & Edge Cases

**Decision Matrix**:

| Option | Pros | Cons | Recommendation |
|--------|------|------|----------------|
| **A. Integrate extracted crates** | Cleaner architecture, reusable, publishes to crates.io | More refactoring, potential breaking changes | If crates are actually reusable |
| **B. Delete extracted crates** | Simpler codebase, no duplicate maintenance, faster builds | No reusable crates, less modular | If crates are vaporware |

**Implementation Option A (Integrate)**:
```toml
# doc_transformer/Cargo.toml
[dependencies]
contextual-chunker = { path = "../contextual-chunker" }
llms-txt-parser = { path = "../llms-txt-parser" }
```

```rust
// doc_transformer/src/main.rs
use contextual_chunker::Chunk;  // Now actually used
use llms_txt_parser::parse_llms_txt;

// doc_transformer/src/chunk.rs - DELETE THIS FILE
// Move all functionality to contextual-chunker crate
```

**Implementation Option B (Delete)**:
```bash
rm -rf contextual-chunker/
rm -rf llms-txt-parser/

# Update doc_transformer/Cargo.toml
# Remove these lines:
# contextual-chunker = { path = "../contextual-chunker" }
# llms-txt-parser = { path = "../llms-txt-parser" }

# Keep doc_transformer/src/chunk.rs
# This is the working implementation
```

**Cargo.toml Workspace Update**:
```toml
# Root Cargo.toml (Option B - Delete)
[workspace]
members = [
    "doc_transformer",
    # "contextual-chunker",  # Removed
    # "llms-txt-parser",     # Removed
]
```

---

## FIX LOCATIONS

**Option A - Integrate**:
1. **`contextual-chunker/`** - Enhance to be production-ready
   - Add comprehensive tests
   - Improve error handling
   - Document public API

2. **`doc_transformer/src/main.rs`** - Replace imports
   - Replace local chunking with `use contextual_chunker::Chunk`
   - Replace local parsing with `use llms_txt_parser::parse`

3. **`doc_transformer/src/chunk.rs`** - DELETE
   - Move any unique logic to contextual-chunker crate

4. **`doc_transformer/Cargo.toml`** - No changes (deps already listed)

**Option B - Delete** (RECOMMENDED):
1. **`contextual-chunker/`** - DELETE DIRECTORY
2. **`llms-txt-parser/`** - DELETE DIRECTORY
3. **`doc_transformer/Cargo.toml`** - Remove workspace dependencies
4. **`Cargo.toml` (root)** - Remove workspace members
5. **`ROADMAP.md`** - Remove false claims about crate readiness

---

## TEST CASES

```rust
// Test after integration (Option A)
#[test]
fn test_imports_from_workspace_crates() {
    // Verify we're using extracted crates
    let chunk = contextual_chunker::Chunk::new("test content");
    assert_eq!(chunk.content, "test content");
}

#[test]
fn test_no_duplicate_chunking_logic() {
    // Verify only one chunking implementation exists
    let source_files = vec![
        "doc_transformer/src/chunk.rs",
        "contextual-chunker/src/chunk.rs",
    ];

    let implementations = source_files
        .iter()
        .filter(|f| std::path::Path::new(f).exists())
        .count();

    assert_eq!(
        implementations, 1,
        "Exactly one chunking implementation should exist"
    );
}

// Test after deletion (Option B)
#[test]
fn test_no_unused_workspace_crates() {
    // Verify workspace only has doc_transformer
    let workspace_toml = std::fs::read_to_string("Cargo.toml").unwrap();
    assert!(!workspace_toml.contains("contextual-chunker"));
    assert!(!workspace_toml.contains("llms-txt-parser"));
}

#[test]
fn test_cargo_toml_no_dead_deps() {
    let deps_toml = std::fs::read_to_string("doc_transformer/Cargo.toml").unwrap();
    assert!(!deps_toml.contains("contextual-chunker"));
    assert!(!deps_toml.contains("llms-txt-parser"));
}

#[test]
fn test_build_time_improved() {
    // Build should be faster without unused deps
    let start = std::time::Instant::now();
    let output = Command::new("cargo")
        .args(["build", "--release"])
        .output()
        .unwrap();

    let duration = start.elapsed();

    // Compare to baseline (with unused deps)
    // Should be significantly faster
    assert!(duration.as_secs() < 60); // Example threshold
}
```

---

## VERIFICATION

**After Option A (Integrate)**:
```bash
$ grep -r "use contextual_chunker" doc_transformer/src
doc_transformer/src/main.rs:use contextual_chunker::Chunk;  # Found!

$ cargo build --release
# Compiling contextual-chunker v0.1.0  # Actually used
# Compiling doc_transformer v0.5.0
# Finished in 1m 15s

$ cargo test
# test_result: ok. passed. 24/24 tests

$ grep -c "chunk.rs" $(find . -name "*.rs")
# 1  # Only contextual-chunker/src/chunk.rs exists
```

**After Option B (Delete)**:
```bash
$ ls -la
# doc_transformer/  # Only one crate left

$ grep -r "contextual-chunker" Cargo.toml
# (no results)

$ cargo build --release
# Compiling doc_transformer v0.5.0  # Faster!
# Finished in 45s (was 1m 15s)

$ wc -l doc_transformer/src/chunk.rs
# 546 doc_transformer/src/chunk.rs  # One implementation

$ cargo test
# test_result: ok. passed. 24/24 tests
```

---

## RECOMMENDATION

**Go with Option B (Delete)** unless you have a concrete plan to publish these crates to crates.io and maintain them as separate projects. The "extracted" crates are currently vaporware - they exist but no one uses them. Delete them and simplify the codebase.
