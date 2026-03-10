# Implementation: doc-2nt1

## bead_id: doc-2nt1
## bead_title: cli: Limit too large validation error returns exit code 0
## phase: p3
## updated_at: 2026-03-01T20:55:00Z

---

## Summary

The bug has already been fixed in the codebase.

## Verification

Test 1: `--limit 1000000000`
```
$ doc_transformer search test --index-dir /tmp -n 1000000000
error: invalid value '1000000000' for '--limit <LIMIT>': limit must be at most 1000 results, got 1000000000
Exit: 1 ✅
```

Test 2: `--max-related-chunks 10000`
```
$ doc_transformer index /tmp --output /tmp/out --max-related-chunks 10000
error: invalid value '10000' for '--max-related-chunks <N>': max_related_chunks must be at most 100, got '10000'
Exit: 1 ✅
```

## Root Cause

The clap error handler at main.rs lines 737-748 correctly maps ValueValidation, InvalidValue, and MissingRequiredArgument errors to exit code 1.

## Code Location

File: `doc_transformer/src/main.rs`, lines 722-749
