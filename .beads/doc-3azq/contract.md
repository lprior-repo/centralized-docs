# Contract: doc-3azq - scrape-identity slug collisions

**bead_id**: doc-3azq  
**bead_title**: scrape-identity: slug collisions overwrite pages with query or fragment variants  
**phase**: p0  
**updated_at**: 2026-03-01T01:40:00Z

---

## Problem Statement

When the crawler discovers URLs like `/docs?page=1` and `/docs?page=2`, these should be treated as distinct pages with unique identities. However, the current implementation is creating slugs that lose the query/fragment information, causing page content to be overwritten silently.

## Contract Requirements

### Preconditions
- URL normalization policy must be defined for query and fragment handling
- System must have access to original URL with query parameters intact

### Postconditions
- Written markdown file count matches retained unique canonical URLs
- Each unique URL (including distinct query/fragment) produces a unique output file
- No silent overwrites occur

### Invariants
- Manifest pages and on-disk page artifacts remain bijective after deduplication policy
- One-to-one mapping between distinct canonical page identities and output markdown files

### Unwanted Behavior Prevention
- IF distinct pages map to same slug: THE SYSTEM SHALL NOT overwrite prior page content silently
- Because: silent overwrite causes data loss and broken links

## Acceptance Criteria

1. `/docs?page=1` and `/docs?page=2` produce two separate markdown files
2. `/docs#section1` and `/docs#section2` produce two separate markdown files OR are properly deduplicated with clear policy
3. Manifest reflects the correct count of unique URLs
4. No silent overwrites - if collision detected, error or explicit dedup with warning

## Research Required

- Read `doc_transformer/src/scrape/transformers.rs` for existing slug generation patterns
- Read `doc_transformer/src/scrape/http.rs` for URL handling patterns
