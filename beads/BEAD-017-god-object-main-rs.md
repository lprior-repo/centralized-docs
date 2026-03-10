# BEAD-017: God Object - main.rs at 1,071 Lines

**Epic**: Code Quality
**Severity**: Medium
**Status**: Open

---

## CONTEXT BLOCK

- **File/Function**: `doc_transformer/src/main.rs` (1,071 lines, 29 functions)
- **The Smell**: Single file mixes CLI argument parsing, pipeline orchestration, business logic, and configuration. No separation between presentation layer (CLI) and application layer (pipeline). All commands (`scrape`, `index`, `ingest`, `search`) are defined in one massive file.

**Evidence**:
```bash
$ wc -l doc_transformer/src/main.rs
1071 doc_transformer/src/main.rs  # CLI + business logic mixed

$ grep -n "^fn\|^async fn\|^pub fn" doc_transformer/src/main.rs
36:fn main()
42:fn run_scrape_command(...)
89:fn run_index_command(...)
156:fn run_ingest_command(...)
213:fn run_search_command(...)
245:fn run_legacy_mode(...)
278:fn run_pipeline(...)
345:fn scrape_and_index(...)
... (continues for 29 functions)
```

**What's in main.rs**:
- CLI argument parsing with `clap::Parser`
- Subcommand handlers (`run_scrape_command`, `run_index_command`, etc.)
- Pipeline orchestration (`run_pipeline`)
- Business logic for each command
- Configuration struct definitions
- Error handling for all commands
- 8-step pipeline logic

**User Impact**:
- Impossible to understand pipeline flow at a glance
- Business logic coupled to CLI details
- Hard to unit test business logic (CLI parsing in the way)
- Adding a new subcommand requires editing 1,000+ line file
- Can't reuse business logic without CLI (e.g., as a library)
- Changes to one command risk breaking others

---

## SPECIFICATION BLOCK

### 1. EARS (Easy Approach to Requirements Syntax)

| Trigger | System | Response |
|---------|--------|----------|
| main.rs exceeds 500 lines | Code review | Split into CLI and pipeline modules |
| Adding new subcommand | CLI structure | Add to `cli.rs` module |
| Testing business logic | Module structure | Add to `pipeline.rs` module (no CLI) |
| Reusing pipeline as library | Architecture | Business logic independent of CLI |

### 2. DbC (Design by Contract)

**Preconditions**:
- Current functionality must be preserved
- CLI interface unchanged
- All tests pass after refactoring

**Postconditions**:
- `cli.rs` contains CLI parsing and command dispatch (< 400 lines)
- `pipeline.rs` contains business logic (< 400 lines)
- `main.rs` is minimal (< 100 lines)
- Business logic testable without CLI

**Invariants**:
- CLI parsing separated from business logic
- Each module < 500 lines
- Public API unchanged
- Library users can import `pipeline` module directly

### 3. Schema & Edge Cases

**Proposed Module Structure**:
```
doc_transformer/src/
├── main.rs              # Entry point only (~50 lines)
├── cli.rs               # CLI parsing, subcommands (~350 lines)
├── pipeline.rs          # Business logic, orchestration (~400 lines)
├── config.rs            # Configuration structures (~200 lines)
└── ... (existing modules: scrape, index, chunk, etc.)
```

**Module Responsibilities**:

| Module | Responsibility | Public API |
|--------|---------------|------------|
| `main.rs` | Entry point, CLI dispatch | `main()` only |
| `cli.rs` | CLI argument parsing, subcommand definitions | `Cli`, `Commands` enums |
| `pipeline.rs` | Business logic, 8-step pipeline | `run_scrape()`, `run_index()`, `run_pipeline()` |
| `config.rs` | Configuration structures | `IndexConfig`, `ScrapeConfig` |

**Example Refactoring - `main.rs`**:
```rust
// Before: 1,071 lines of mixed concerns
fn main() {
    // CLI parsing
    let args = Args::parse();
    match args.command {
        Commands::Scrape { ... } => run_scrape_command(...),  // Business logic here!
        Commands::Index { ... } => run_index_command(...),  // Business logic here!
        // ... 29 functions in one file
    }
}

// After: clean separation
// main.rs (~50 lines)
fn main() -> Result<()> {
    let cli = Cli::parse();
    cli::run(cli)
}
```

**Example Refactoring - `cli.rs`** (~350 lines):
```rust
// cli.rs
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Scrape {
        url: String,
        #[arg(short, long)]
        output: PathBuf,
        // ... CLI args only
    },
    Index {
        source: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
        // ... CLI args only
    },
    // ... other subcommands
}

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Scrape(args) => pipeline::run_scrape(args),
        Commands::Index(args) => pipeline::run_index(args),
        Commands::Ingest(args) => pipeline::run_ingest(args),
        Commands::Search(args) => pipeline::run_search(args),
        Commands::Legacy(source, output) => pipeline::run_legacy(source, output),
    }
}
```

**Example Refactoring - `pipeline.rs`** (~400 lines):
```rust
// pipeline.rs - Business logic, testable without CLI
use crate::config::{ScrapeArgs, IndexArgs};

pub fn run_scrape(args: ScrapeArgs) -> Result<()> {
    let config = ScrapeConfig::from(args);
    scrape::scrape_site(&config)?;
    Ok(())
}

pub fn run_index(args: IndexArgs) -> Result<()> {
    let config = IndexConfig::from(args);
    pipeline::run_pipeline(config)?;
    Ok(())
}

pub fn run_pipeline(config: PipelineConfig) -> Result<()> {
    // 8-step pipeline: DISCOVER, ANALYZE, ASSIGN, TRANSFORM, CHUNK, INDEX, VALIDATE
    // ... business logic only, no CLI concerns
}
```

---

## FIX LOCATIONS

1. **Create `doc_transformer/src/cli.rs`** (~350 lines)
   - Move `#[derive(Parser)]` structs from main.rs
   - Move all `Commands` enum variants
   - Create `run()` function for command dispatch
   - Move CLI-specific validation

2. **Create `doc_transformer/src/pipeline.rs`** (~400 lines)
   - Move `run_pipeline()` function
   - Move `run_scrape_command()` → `run_scrape()`
   - Move `run_index_command()` → `run_index()`
   - Move `run_ingest_command()` → `run_ingest()`
   - Move `run_search_command()` → `run_search()`
   - Move `run_legacy_mode()` → `run_legacy()`

3. **Simplify `doc_transformer/src/main.rs`** (~50 lines)
   - Keep only entry point
   - Delegate to `cli::run()`
   - Add mod declarations

4. **Update `doc_transformer/src/lib.rs`** (if exists)
   - Re-export `pipeline` module for library users

---

## TEST CASES

```rust
// Test business logic without CLI

#[test]
fn test_pipeline_index_flow() {
    use crate::pipeline::{IndexConfig, run_index};

    let config = IndexConfig {
        source: "./test_docs".into(),
        output: "./test_output".into(),
        generate_llms: true,
        ..Default::default()
    };

    let result = run_index(config);
    assert!(result.is_ok());

    // Verify output files exist
    assert!(PathBuf::from("./test_output/INDEX.json").exists());
}

#[test]
fn test_pipeline_scrape_flow() {
    use crate::pipeline::{ScrapeConfig, run_scrape};

    let config = ScrapeConfig {
        base_url: "https://example.com".into(),
        output_dir: "./test_scraped".into(),
        ..Default::default()
    };

    let result = run_scrape(config);
    assert!(result.is_ok());
}

#[test]
fn test_cli_parsing() {
    use crate::cli::{Cli, Commands};

    let cli = Cli::try_parse_from(["doc_transformer", "scrape", "https://example.com", "-o", "./out"]).unwrap();

    match cli.command {
        Commands::Scrape(args) => {
            assert_eq!(args.url, "https://example.com");
            assert_eq!(args.output, PathBuf::from("./out"));
        }
        _ => panic!("Expected Scrape command"),
    }
}

#[test]
fn test_main_is_minimal() {
    // main.rs should be < 100 lines
    let content = std::fs::read_to_string("doc_transformer/src/main.rs").unwrap();
    let lines = content.lines().count();

    assert!(
        lines < 100,
        "main.rs has {} lines (max 100 allowed)",
        lines
    );
}

#[test]
fn test_modules_below_limit() {
    let modules = vec![
        "cli.rs",
        "pipeline.rs",
        "main.rs",
    ];

    for module in modules {
        let path = format!("doc_transformer/src/{}", module);
        let content = std::fs::read_to_string(path).unwrap();
        let lines = content.lines().count();

        assert!(
            lines < 500,
            "{} has {} lines (max 500 allowed)",
            module,
            lines
        );
    }
}
```

---

## VERIFICATION

After refactoring:
```bash
$ wc -l doc_transformer/src/main.rs doc_transformer/src/cli.rs doc_transformer/src/pipeline.rs
50 doc_transformer/src/main.rs     # Entry point only
350 doc_transformer/src/cli.rs    # CLI parsing
400 doc_transformer/src/pipeline.rs  # Business logic

# Total: 800 lines (vs 1,071 in one file)

$ cargo test pipeline::
# test_pipeline_index_flow ... ok
# test_pipeline_scrape_flow ... ok
# test_result: ok. passed. 30/30 tests

$ cargo test cli::
# test_cli_parsing ... ok
# test_result: ok. passed. 6/6 tests

# CLI interface unchanged
$ ./target/release/doc_transformer scrape https://example.com -o ./out
# Works exactly as before

# Business logic reusable as library
$ cat example_lib_usage.rs
use doc_transformer::pipeline::IndexConfig;
use doc_transformer::pipeline::run_index;

let config = IndexConfig { ... };
run_index(config)?;  // No CLI needed!
```

---

## RECOMMENDATION

Split `main.rs` into `cli.rs`, `pipeline.rs`, and minimal `main.rs`. This will:
- Enable unit testing of business logic without CLI overhead
- Make codebase navigable (find CLI vs business logic easily)
- Allow reuse as a library (import `pipeline` module)
- Reduce merge conflicts (CLI changes don't touch business logic)
- Maintain backward compatibility (CLI interface unchanged)
