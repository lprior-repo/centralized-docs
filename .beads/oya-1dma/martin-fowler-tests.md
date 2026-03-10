# Martin Fowler Test Plan

## Happy Path Tests
- test_chunk_markdown_splits_text_within_capacity
- test_open_or_create_index_successfully_returns_tantivy_index
- test_score_bm25_returns_correctly_sorted_documents

## Error Path Tests
- test_open_or_create_index_returns_error_when_directory_inaccessible
- test_score_bm25_returns_error_on_invalid_query_syntax

## Edge Case Tests
- test_chunk_markdown_handles_single_word_larger_than_capacity
- test_score_bm25_handles_query_with_no_matches
- test_score_bm25_returns_fewer_than_top_k_if_not_enough_matches

## Contract Verification Tests
- test_precondition_chunk_capacity_greater_than_zero
- test_precondition_query_not_empty
- test_postcondition_chunks_do_not_exceed_capacity
- test_postcondition_scores_are_sorted_descending
- test_invariant_bm25_scores_are_non_negative

## Contract Violation Tests
- `test_p1_violation_returns_invalid_chunk_capacity`
  Given: `ChunkCapacity::new(0)`
  When: function is called with violating input
  Then: returns `Err(ChunkerError::InvalidChunkCapacity)`

- `test_p2_violation_returns_directory_access_failed`
  Given: `open_or_create_index(Path::new("/root/secret/index"))`
  When: function is called with violating input
  Then: returns `Err(IndexerError::DirectoryAccessFailed)`

- `test_p3_violation_returns_invalid_document`
  Given: `Document::new("")`
  When: function is called with violating input
  Then: returns `Err(IndexerError::InvalidDocument)`

- `test_p4_violation_returns_empty_query`
  Given: `QueryStr::new("")`
  When: function is called with violating input
  Then: returns `Err(SearchError::EmptyQuery)`

- `test_q1_violation_returns_postcondition_violated`
  Given: `chunk_markdown` returns a chunk of size 1500 when capacity is 1000
  When: function output is generated and validated
  Then: returns `Err(ChunkerError::PostconditionViolated)`

- `test_q2_violation_returns_postcondition_violated`
  Given: `score_bm25` returns documents out of order (e.g., score 1.2 then score 1.5)
  When: function output is generated and validated
  Then: returns `Err(SearchError::PostconditionViolated)`

- `test_q3_violation_returns_uncommitted_changes`
  Given: `score_bm25` is called before `index_documents` commits, resulting in missing documents
  When: function output is generated and validated
  Then: returns `Err(IndexerError::UncommittedChanges)`

## Given-When-Then Scenarios
### Scenario 1: Chunking and Indexing End-to-End
Given: A markdown document and a valid tantivy index path
When: The document is chunked into pieces, and the pieces are indexed via `index_documents`
Then:
- `chunk_markdown` returns multiple valid chunks.
- `index_documents` successfully writes to the index path.
- Searching the index for a keyword from the document returns the correct chunk with a positive BM25 score.
