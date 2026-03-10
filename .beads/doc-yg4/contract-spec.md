# Contract Specification

## Context
- **Feature**: transform_content result validation
- **Bead**: doc-yg4
- **Domain terms**: 
  - `transform_content` - external function from spider_transformations that extracts markdown from HTML
  - `markdown` - the extracted content string
- **File**: transformers.rs:287-293
- **Assumptions**:
  - transform_content can return an empty string when extraction fails
  - The caller should validate the result before use
- **Open questions**: None - issue is clearly defined

## Preconditions
- [P1] `page` parameter must be a valid Page reference (not checked here - assumed valid from caller)
- [P2] `transform_config` must be a valid TransformConfig (not checked here - caller provides valid config)

## Postconditions
- [Q1] `markdown` must not be empty after transform_content returns
- [Q2] If `markdown` is empty, an appropriate error must be returned to the caller

## Invariants
- [I1] transform_page function must return Err if content extraction yields empty result

## Error Taxonomy
- `Error::EmptyExtractionResult` - when transform_content returns an empty string, indicating extraction failed

## Contract Signatures
- Current: `fn transform_page(...) -> Result<ScrapedPage>`
- The function should validate the result of transform_content before proceeding

## Type Encoding
| Precondition/Postcondition | Enforcement Level | Type / Pattern |
|---|---|---|
| P1 | Caller responsibility | Valid Page reference |
| P2 | Caller responsibility | Valid TransformConfig |
| Q1 | Runtime check | Result validation after transform_content |
| Q2 | Error variant | Error::EmptyExtractionResult |

## Violation Examples
- VIOLATES Q1: `transform_content(page, &config, &None, &selector_config, &None)` returns `""` (empty string) -- should produce `Err(Error::EmptyExtractionResult)`
- VIOLATES Q1: `transform_content(page, &config, &None, &selector_config, &None)` returns `"   "` (whitespace only) -- should produce `Err(Error::EmptyExtractionResult)`

## Ownership Contracts
- This is an existing function modification - no change to ownership patterns
- The `page` borrow is unchanged (shared reference)
- No new mutable borrows introduced

## Non-goals
- [ ] Modifying transform_content function (external dependency)
- [ ] Changing the function signature of transform_page
- [ ] Adding new functionality beyond validation
