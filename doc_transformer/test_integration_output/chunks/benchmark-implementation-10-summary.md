---
doc_id: benchmark-implementation
chunk_id: benchmark-implementation#10
chunk_level: summary
chunk_type: prose
heading: 4. Test Data Generators
token_count: 135
summary: - Word counts scale with document index. ### `generate_test_tags(chunks: &[Chunk]) -> Vec<(String, V
---

- Word counts scale with document index

### `generate_test_tags(chunks: &[Chunk]) -> Vec<(String, Vec<String>, String)>`

Creates tag metadata for relationship detection.

**Features:**
- Cyclic tag distribution (tag_0, tag_1, tag_2)
- Global tags: "documentation", "section_X"
- Categories: "Category 0" through "Category 4"
- Realistic for semantic clustering

### Data Properties

All generators produce:
- **Deterministic output** (same N → same data every run)
- **Reproducible relationships** (enables benchmarking same comparisons)
