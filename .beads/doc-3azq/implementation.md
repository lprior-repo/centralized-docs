# Implementation: doc-3azq - scrape-identity slug collisions

**bead_id**: doc-3azq  
**bead_title**: scrape-identity: slug collisions overwrite pages with query or fragment variants  
**phase**: p2  
**updated_at**: 2026-03-01T01:50:00Z

---

## Problem

When the crawler discovers URLs like `/docs?page=1` and `/docs?page=2`, these should be treated as distinct pages with unique identities. However, the `url_to_slug` function was ignoring query parameters and fragments, causing all such URLs to generate the same slug and overwrite each other.

## Solution

Modified the `url_to_slug` function in `doc_transformer/src/scrape/transformers.rs` to include a hash of query parameters and fragments in the generated slug. This ensures:

1. `/docs?page=1` and `/docs?page=2` produce different slugs
2. `/docs#section1` and `/docs#section2` produce different slugs
3. URLs without query/fragment parameters produce slugs without the `-q` suffix (backward compatible)

## Changes Made

### File: `doc_transformer/src/scrape/transformers.rs`

1. **Added import**: `use std::hash::Hasher;`

2. **Modified `url_to_slug` function** (lines 33-110):
   - Added logic to detect query parameters and fragments
   - When query or fragment exists, compute a hash using `DefaultHasher`
   - Append `-q{hash}` to the slug (e.g., `docs-q1234`)
   - Hash is limited to 4 digits (0-9999) to keep slugs short

3. **Added 5 new tests**:
   - `test_url_to_slug_with_query_params` - verifies query params produce unique slugs
   - `test_url_to_slug_with_fragment` - verifies fragments produce unique slugs
   - `test_url_to_slug_query_and_fragment_together` - verifies combined handling
   - `test_url_to_slug_no_query_no_suffix` - verifies backward compatibility
   - `test_url_to_slug_different_paths_different_slugs` - verifies path differences preserved

## Test Results

All 278 library tests pass:
```
test result: ok. 278 passed; 0 failed; 0 ignored; 0 measured
```

## Backward Compatibility

- URLs without query parameters or fragments produce the same slugs as before
- The collision handling in `write_scraped_pages` still works as a safety net
- No changes to public API

## Verification

```bash
cd doc_transformer
cargo test --lib -- scrape::transformers::tests::test_url_to_slug
# All 6 tests pass
```
