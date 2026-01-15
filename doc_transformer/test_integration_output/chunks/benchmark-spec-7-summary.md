---
doc_id: benchmark-spec
chunk_id: benchmark-spec#7
chunk_level: summary
chunk_type: prose
heading: 2. Test Data Generators
token_count: 134
summary: Groups chunks into documents with tags and metadata. **Properties:**
---

```


Groups chunks into documents with tags and metadata.

**Properties:**
- One document per unique doc_id in chunks
- Assigns 3-5 tags per document
- Categories distributed across 5 categories
- Word counts scale with document index

#### C. `generate_test_tags(chunks: &[Chunk]) -> Vec<(String, Vec<String>, String)>`

Creates tag sets for relationship detection.

**Properties:**
- Tags follow pattern: tag_0, tag_1, tag_2 (cyclical)
- All chunks share category prefixes (Category 0-4)
- Includes "documentation" and "section_X" tags
