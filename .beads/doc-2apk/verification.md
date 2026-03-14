# Verification: doc-2apk

Date: 2026-03-01
Verifier: qa-enforcer

## Scope

Contract under test: `.beads/doc-2apk/contract.md`

Required behaviors:
1. `--max-related-chunks` outside `1-100` exits with code `2`
2. valid values `1-100` are accepted
3. help text shows valid range `1-100`

## Environment and Build Evidence

Command:
```bash
cargo build --release
```

Workdir: `ctd`

Observed output (stdout/stderr excerpt):
```text
Blocking waiting for file lock on artifact directory
Finished `release` profile [optimized] target(s) in 1m 12s
```

Exit code: `0`

## Acceptance Test Evidence

### Test 1: Invalid value `101`

Command:
```bash
./target/release/ctd index /tmp/testdir --output /tmp/doc-2apk-out-101 --max-related-chunks 101
```

Observed output:
```text
error: invalid value '101' for '--max-related-chunks <N>': max_related_chunks must be at most 100, got '101'

For more information, try '--help'.
```

Exit code: `2`

Expected vs actual:
- Expected: invalid out-of-range input exits `2`
- Actual: exits `2`
- Result: PASS

Reproduction steps:
1. Build release binary (`cargo build --release`)
2. Run command above
3. Check `$?` immediately after execution

### Test 2: Invalid value `0`

Command:
```bash
./target/release/ctd index /tmp/testdir --output /tmp/doc-2apk-out-0 --max-related-chunks 0
```

Observed output:
```text
error: invalid value '0' for '--max-related-chunks <N>': max_related_chunks must be at least 1, got '0'

For more information, try '--help'.
```

Exit code: `2`

Expected vs actual:
- Expected: invalid out-of-range input exits `2`
- Actual: exits `2`
- Result: PASS

Reproduction steps:
1. Build release binary (`cargo build --release`)
2. Run command above
3. Check `$?` immediately after execution

### Test 3: Valid value `50`

Command:
```bash
./target/release/ctd index /tmp/testdir --output /tmp/doc-2apk-out-50 --max-related-chunks 50
```

Observed output (stdout excerpt):
```text
[CONFIG] Graph Parameters:
  max_related_chunks: 50 (default: 20)
...
COMPLETE
...
Validation: 1/1 passed
```

Exit code: `0`

Expected vs actual:
- Expected: valid in-range input exits `0`
- Actual: exits `0`
- Result: PASS

Reproduction steps:
1. Build release binary (`cargo build --release`)
2. Run command above
3. Check `$?` immediately after execution

### Test 4: Help text range

Command:
```bash
./target/release/ctd index --help
```

Observed output excerpt:
```text
--max-related-chunks <N>
    Maximum number of related chunks per document (1-100, default: 20)
```

Exit code: `0`

Expected vs actual:
- Expected: help text documents `1-100`
- Actual: help text documents `1-100`
- Result: PASS

Reproduction steps:
1. Run command above
2. Inspect `--max-related-chunks` description

## Answers

1. Is the contract satisfied?
- Yes. All acceptance criteria in `.beads/doc-2apk/contract.md` pass in this environment.

2. Is this a contract drift issue (implementation differs from contract)?
- No contract drift is currently observed. Runtime behavior matches the contract (`2` for invalid range, `0` for valid values).

3. What is the recommended fix?
- No code fix is needed for this contract in the tested binary.
- If another environment still reports exit code `1`, verify the exact executable and invocation path first (for example, wrapper scripts or stale binaries), then re-run the same commands against `./target/release/ctd`.

## Notes

- Additional cross-check performed:
  - `cargo run --release --bin ctd -- index /tmp/testdir --output /tmp/doc-2apk-out-cargo --max-related-chunks 101`
  - Observed exit code: `2`
  - Confirms behavior is consistent via Cargo-run invocation when targeting the correct binary.
