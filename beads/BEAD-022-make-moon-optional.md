# BEAD-022: Make Moon Build System Optional

**Epic**: Developer Experience
**Severity**: Low
**Status**: Open

---

## CONTEXT BLOCK

- **Files**: `.moon/`, `CLAUDE.md`, `AGENTS.md`, build scripts
- **The Smell**: Project requires Moon build system exclusively. `cargo` commands are forbidden in documentation. This creates a high barrier for contributors who don't know Moon and locks the project into Moon-specific tooling. If Moon dies or becomes unmaintained, the project becomes difficult to build.

**Evidence**:
```bash
$ which moon
# moon not found - must install first

$ cargo build
# Builds fine, but docs say "NEVER DO THIS"

$ cat CLAUDE.md | grep -A10 "ALWAYS Use Moon"
# **CRITICAL: NEVER use raw cargo commands. ALWAYS use Moon.**
#
# # ❌ WRONG - NEVER do this
# cargo build        # NO - no caching, slow
# cargo test         # NO - no workspace awareness

$ cat AGENTS.md
# **CRITICAL: NEVER use raw cargo commands. ALWAYS use Moon.**
```

**Moon vs Cargo Reality**:

| Command | Moon | Cargo |
|---------|-------|-------|
| Build | `moon run :build` (cached) | `cargo build --release` (slower) |
| Test | `moon run :test` | `cargo test` |
| Check | `moon run :check` | `cargo check` |
| Install | `moon run :install` | `cargo install --path .` |
| Barrier | Must install Moon first | Rust users already have Cargo |

**User Impact**:
- High barrier for new contributors (learn Moon vs just use Cargo)
- Locked into Moon ecosystem (vendor lock-in)
- If Moon unmaintained, project stuck
- Confusing for experienced Rust developers
- Can't use standard Rust tooling
- CI must have Moon installed

---

## SPECIFICATION BLOCK

### 1. EARS (Easy Approach to Requirements Syntax)

| Trigger | System | Response |
|---------|--------|----------|
| New contributor joins | Onboarding | Document both Moon AND Cargo workflows |
| Moon unavailable | Fallback | Provide Cargo alternative commands |
| Adding build instructions | Docs | Show both methods side-by-side |

### 2. DbC (Design by Contract)

**Preconditions**:
- Both Moon and Cargo workflows documented
- No functionality lost without Moon
- CI supports both (or Cargo-only)

**Postconditions**:
- Contributors can use EITHER Moon OR Cargo
- Cargo commands work for all operations
- Moon remains available for caching benefits
- No "NEVER use Cargo" language

**Invariants**:
- Cargo always works (Rust standard)
- Moon is optional (performance optimization)
- Documentation shows both methods
- CI doesn't require Moon

### 3. Schema & Edge Cases

**Dual Workflow Documentation**:

```markdown
# CLAUDE.md - UPDATED

## Build System: Moon (Recommended) or Cargo (Standard)

### Moon (Recommended - Faster, Cached)

Moon provides persistent caching and parallel execution. Use if available.

```bash
moon run :build      # Release build (cached)
moon run :test       # Run tests (workspace-wide)
moon run :check      # Type check
moon run :ci         # Full pipeline
moon run :install    # Install to ~/.local/bin
```

### Cargo (Standard - Always Works)

Standard Rust tooling that every developer has. Slower but universal.

```bash
cargo build --release     # Release build
cargo test               # Run tests
cargo check              # Type check
cargo clippy             # Lint
cargo fmt --check        # Format check
cargo install --path .   # Install binary
```

### Comparison

| Feature | Moon | Cargo |
|---------|-------|-------|
| **Caching** | ✅ Fast (ms when cached) | ❌ Slower (rebuilds) |
| **Parallel** | ✅ Runs tasks in parallel | ⚠️ Limited |
| **Required** | ❌ Optional | ✅ Always available |
| **Barrier** | ⚠️ Must install | ✅ Pre-installed with Rust |
| **Standard** | ❌ Non-standard | ✅ Rust ecosystem standard |

### Recommendation

Use **Moon** if you want faster builds and have it installed. Use **Cargo** if you don't have Moon or want standard Rust tooling.

Both work. Both are supported. No lock-in.
```

**Cargo Makefile (Optional)**:

```makefile
# Makefile - Cargo shortcuts (for those who prefer make)
.PHONY: build test check fmt lint install clean

build:
	cargo build --release

test:
	cargo test

check:
	cargo check

fmt:
	cargo fmt --check

lint:
	cargo clippy -- -D warnings

install:
	cargo install --path .

clean:
	cargo clean

ci: fmt lint test
```

**CI Configuration**:

```yaml
# .github/workflows/ci.yml - Support both
name: CI

jobs:
  test:
    strategy:
      matrix:
        build_system: [moon, cargo]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - if: matrix.build_system == 'moon'
        run: |
          curl -fsSL https://moonrepo.dev/install.sh | bash
          moon run :ci
      - if: matrix.build_system == 'cargo'
        run: |
          cargo fmt --check
          cargo clippy -- -D warnings
          cargo test
```

**Command Reference Table**:

| Task | Moon (Fast) | Cargo (Standard) | Make (Shortcut) |
|------|--------------|------------------|----------------|
| Build | `moon run :build` | `cargo build --release` | `make build` |
| Test | `moon run :test` | `cargo test` | `make test` |
| Check | `moon run :check` | `cargo check` | `make check` |
| Format | `moon run :fmt` | `cargo fmt --check` | `make fmt` |
| Lint | `moon run :clippy` | `cargo clippy` | `make lint` |
| Install | `moon run :install` | `cargo install --path .` | `make install` |
| Clean | `moon run :clean` | `cargo clean` | `make clean` |

---

## FIX LOCATIONS

1. **`CLAUDE.md`** - Add Cargo alternative documentation
   - Remove "NEVER use cargo" language
   - Add "Moon (Recommended) or Cargo (Standard)" section
   - Document both workflows side-by-side
   - Add comparison table

2. **`AGENTS.md`** - Update build instructions
   - Remove "ALWAYS use Moon" language
   - Document Cargo alternatives
   - Keep Moon as "recommended" not "required"

3. **Create `Makefile`** (OPTIONAL, ~50 lines)
   - Provide make shortcuts for Cargo commands
   - Convenient for contributors

4. **`.github/workflows/ci.yml`** - Support both systems
   - Add matrix build (Moon and Cargo)
   - Both should pass in CI

5. **`README.md`** - Update Quick Start
   - Show both Moon and Cargo examples
   - Add note about Moon being optional

---

## TEST CASES

```rust
// Not really applicable, but verify CI works with both

#[test]
fn test_cargo_build_works() {
    // Verify standard cargo commands work
    let output = Command::new("cargo")
        .args(["build", "--release"])
        .output()
        .unwrap();

    assert!(output.status.success());
}

#[test]
fn test_moon_build_works() {
    // Verify moon commands work (if moon is installed)
    let output = Command::new("moon")
        .args(["run", ":build"])
        .output()
        .unwrap_or_else(|_| {
            // Moon not installed - skip test
            Command::new("true").output().unwrap()
        });

    assert!(output.status.success());
}

#[test]
fn test_makefile_targets_work() {
    // Verify Makefile shortcuts work
    let targets = vec!["build", "test", "check", "clean"];

    for target in targets {
        let output = Command::new("make")
            .args([target])
            .output()
            .unwrap();

        assert!(
            output.status.success(),
            "make {} failed",
            target
        );
    }
}
```

---

## VERIFICATION

After updates:
```bash
# Cargo works for everything
$ cargo build --release
# Compiling... Finished in 1m 15s  (slower but works)

$ cargo test
# test result: ok. passed. 24/24 tests

# Moon still available and fast
$ moon run :build
# doc_transformer:build (cached, cb157da1)  Runs in 5ms!

$ moon run :test
# doc_transformer:test (cached)  Runs in 10ms!

# Makefile shortcuts work
$ make test
# cargo test
# test result: ok. passed.

# Documentation shows both
$ cat CLAUDE.md | grep -A5 "Build System"
## Build System: Moon (Recommended) or Cargo (Standard)
# Both work. Both are supported. No lock-in.

# No "NEVER use Cargo" language
$ grep -r "NEVER use cargo" CLAUDE.md AGENTS.md
# (no results)

# CI supports both
$ git push  # CI runs both Moon and Cargo builds
# ✅ Both build systems pass
```

---

## RECOMMENDATION

Make **Cargo the primary (standard) method** and **Moon an optional (fast) enhancement**. Document both, don't forbid Cargo.

**Why this matters**:
- Lower barrier for new contributors
- No vendor lock-in
- Future-proof (if Moon dies, project survives)
- Standard Rust tooling works
- Moon still available for performance
- More inclusive to all Rust developers
