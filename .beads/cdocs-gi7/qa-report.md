# QA Report: cdocs-gi7 — cli: honor json mode on watch and diff failures

## Status: PASS

---

## Execution Evidence

### Binary Verification
```
$ which ctd && ctd --version
/home/lewis/.local/bin/ctd
ctd 0.7.1
```

### Phase 1 — Discovery

**[PASS]** `ctd --help` works and shows JSON mode documentation
**[PASS]** `ctd diff --help` shows `--json` flag
**[PASS]** `ctd watch --help` shows `--json` flag

### Phase 2 — Happy Path

#### `ctd diff /nonexistent /other --json`
```
{
  "command": "diff",
  "error": "No manifest.json found in '/nonexistent' or '/nonexistent/.scrape'. Searched:\n  - /nonexistent/manifest.json\n  - /nonexistent/.scrape/manifest.json\nTip: Run 'ctd scrape --output <DIR>' first, then pass '<DIR>' to this command.",
  "status": "error"
}
EXIT_CODE: 1
```

#### `ctd diff /nonexistent /other` (non-JSON)
```
Error: No manifest.json found in '/nonexistent' or '/nonexistent/.scrape'. Searched:
  - /nonexistent/manifest.json
  - /nonexistent/.scrape/manifest.json
Tip: Run 'ctd scrape --output <DIR>' first, then pass '<DIR>' to this command.
EXIT_CODE: 1
```

#### `ctd watch --output /tmp/watch_test http://127.0.0.1:9 --json`
```
{
  "command": "watch",
  "error": "Execution failed: TCP connect failed: Connection refused (os error 111)",
  "status": "error"
}
EXIT_CODE: 2
```

#### `ctd watch --output /tmp/watch_test http://127.0.0.1:9` (non-JSON)
```
Error: Execution failed: TCP connect failed: Connection refused (os error 111)
EXIT_CODE: 2
```

### Phase 3 — Hostile Interrogation

#### Exit Code Parity (JSON vs non-JSON)
| Command | Error Type | --json exit | non-json exit | Parity |
|---------|------------|-------------|---------------|--------|
| `diff /nonexistent /other` | manifest.json not found | 1 | 1 | ✅ |
| `watch --output /tmp x http://127.0.0.1:9` | connection refused | 2 | 2 | ✅ |
| `watch --output /tmp x not-url` | invalid URL | 1 | 1 | ✅ |
| `diff --json=` (empty) | clap parse error | 2 | N/A | ✅ (clap error) |

#### JSON Schema Validation
```
$ python3 -c "import sys,json; d=json.load(sys.stdin); assert 'status' in d and 'error' in d and 'command' in d"
ctd diff /nonexistent /other --json 2>/dev/null | python3 -c ...
JSON SCHEMA VALID (diff)

ctd watch --output /tmp/watch_test http://127.0.0.1:9 --json 2>/dev/null | python3 -c ...
JSON SCHEMA VALID (watch)
```

#### Edge Cases
| Test | Command | Expected | Actual | Pass |
|------|---------|----------|--------|------|
| Invalid URL | `watch --output /tmp x not-url --json` | exit 1 | exit 1 | ✅ |
| Empty manifest dir | `diff /tmp/no_manifest_test /other --json` | exit 1 | exit 1 | ✅ |
| --json= (empty) | `diff --json= /nonexistent /other` | exit 2 (clap) | exit 2 | ✅ |

#### Security Checks
```
$ ctd diff /nonexistent /other --json 2>/dev/null | grep -iE "password|token|secret|api_key"
No secrets found ✅

$ ctd diff /nonexistent /other --json 2>&1 | grep -iE "panic|unwrap|thread.*main"
No panics found ✅
```

---

## Test Suite Verification

```
$ /home/lewis/src/cdocs-gi7/target/release/deps/watch_diff_json_mode_tests-87eb165f79c29980 --test-threads=1

running 11 tests
test diff_json_and_no_json_exit_code_parity_manifest_not_found ... ok
test diff_json_error_manifest_not_found_emits_json_error_payload ... ok
test diff_json_error_payload_schema_is_exact_no_extra_fields ... ok
test diff_json_pipeline_error_emits_json_error_payload_and_exit_2 ... ok
test diff_no_json_still_emits_plain_text_error ... ok
test search_json_error_still_works_reference ... ok
test watch_json_and_no_json_exit_code_parity_network_error ... ok
test watch_json_error_payload_schema_is_exact_no_extra_fields ... ok
test watch_json_network_error_emits_json_error_payload_and_exit_2 ... ok
test watch_json_user_input_error_emits_json_error_payload_and_exit_1 ... ok
test watch_no_json_still_emits_plain_text_error ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## Findings

### None — All Checks Pass

#### Implementation Verification
- `manifest.json` added to `user_input_patterns` in `src/sys/error.rs:88`
- `map_error_to_exit_code()` correctly returns 1 for manifest errors
- JSON output goes to stdout (not stderr)
- Error text goes to stderr in non-JSON mode
- Exit code parity maintained between JSON and non-JSON modes

#### Exit Code Contract
| Exit Code | Meaning | Used For |
|-----------|---------|----------|
| 0 | Success | - |
| 1 | User input error | Invalid URL, manifest.json not found, validation failures |
| 2 | Pipeline/internal error | Network errors, connection refused, corrupt data |

---

## VERDICT: PASS

All verification checks passed:
- ✅ JSON mode emits valid JSON to stdout on watch/diff failures
- ✅ Exit code 1 for user input errors (manifest not found, invalid URL)
- ✅ Exit code 2 for pipeline errors (connection refused)
- ✅ Exit code parity between --json and non-json modes
- ✅ JSON schema: `{"status": "error", "error": "...", "command": "..."}`
- ✅ No panics, no secret leaks
- ✅ All 11 watch_diff_json_mode_tests pass
