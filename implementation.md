# Implementation: Regex pattern too long returns exit code 0 instead of 1 (bead: doc-29o8)

## Problem
When a regex filter pattern is too long (>500 chars) or contains potentially slow patterns (ReDoS risk), the CLI returns exit code 2 (pipeline error) instead of exit code 1 (user error).

## Root Cause
The `map_error_to_exit_code` function in `doc_transformer/src/main.rs` didn't include patterns to match:
- "Regex pattern too long" - already had "too long" pattern (working)
- "Regex contains potentially slow pattern (ReDoS risk)" - missing from user input patterns

## Solution
Added "slow pattern" and "redos risk" to the user input error patterns in `map_error_to_exit_code()`.

## Implementation Details

### Changes to main.rs
File: `doc_transformer/src/main.rs`

Added two new patterns to the `user_input_patterns` array in `map_error_to_exit_code()`:
```rust
let user_input_patterns = [
    // ... existing patterns ...
    "slow pattern",    // Added: catches ReDoS pattern errors
    "redos risk",      // Added: catches ReDoS risk errors
    // ... rest of patterns ...
];
```

### Testing
```bash
# Test ReDoS pattern - should return exit code 1 (user error)
$ doc_transformer scrape https://example.com --output /tmp/out --filter "(.*)*"
Error: Regex contains potentially slow pattern (ReDoS risk): nested .* quantifiers: (.*)
Exit code: 1

# Test too long pattern - should return exit code 1 (user error)
$ doc_transformer scrape https://example.com --output /tmp/out --filter "aaaaa... (501+ chars)"
Error: Regex pattern too long: N chars (max 500)
Exit code: 1

# Test valid pattern - should proceed normally
$ doc_transformer scrape https://example.com --output /tmp/out --filter "^/docs/"
Exit code: 0
```

All tests pass with correct exit codes.

---

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
