# Implementation: release-gate --help flag fix (bead: doc-3nzf)

## Problem
The release-gate binary does not handle --help or --version flags properly. Running `./target/release/release-gate --help` or `./target/release/release-gate -h` executes the full release gate instead of displaying help.

## Root Cause
The `main` function in `release-gate/src/main.rs` directly calls `run_gate()` without first checking for `--help`, `-h`, or `--version` flags. The gate execution requires preconditions (br and moon commands) to be available, which causes the help flag to fail before it can be processed.

## Solution
Added argument parsing at the start of `main()` to check for `--help`, `-h`, and `--version` flags before running the gate. If any of these flags are present, print the appropriate message and exit with code 0.

## Implementation Details

### Changes to main.rs
1. Added `handle_help_version_flags()` function to parse command-line arguments
2. Added `print_help()` function to display usage information
3. Added `print_version()` function to display version information
4. Modified `main()` to check for help/version flags before calling `run_gate()`

### Code Changes
File: `release-gate/src/main.rs`

Added constants and functions:
```rust
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn handle_help_version_flags() -> Option<i32> {
    let args: Vec<String> = std::env::args().collect();
    for arg in &args[1..] {
        match arg.as_str() {
            "-h" | "--help" => { print_help(); return Some(EXIT_GATE_PASSED); }
            "--version" => { print_version(); return Some(EXIT_GATE_PASSED); }
            _ => {}
        }
    }
    None
}
```

Modified main():
```rust
fn main() {
    // Check for help/version flags before running the gate
    if let Some(code) = handle_help_version_flags() {
        std::process::exit(code);
    }
    // ... rest of main
}
```

## Testing
```bash
# Test --help
$ ./target/release/release-gate --help
release-gate - Production Release Gate
...
Exit code: 0

# Test -h
$ ./target/release/release-gate -h
...
Exit code: 0

# Test --version
$ ./target/release/release-gate --version
release-gate 0.1.0
Exit code: 0
```

All tests pass with exit code 0 as expected.
