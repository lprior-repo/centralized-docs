# Test Plan: MCP Server with Official rmcp SDK

```
bead_id: cdocs-drj
bead_title: mcp: Implement official rmcp SDK and expose semantic tools
phase: state-1.5-retry-1
updated_at: 2026-03-28T13:25:00Z
```

## Summary

- **Behaviors identified**: 47
- **Trophy allocation**: 36 unit / 22 integration / 4 e2e / 2 static
- **Proptest invariants**: 6
- **Fuzz targets**: 3
- **Kani harnesses**: 2
- **Mutation kill threshold**: ≥90%

### Trophy Ratio Justification

| Layer     | Count | Ratio  | Rationale                                                                                    |
|-----------|-------|--------|----------------------------------------------------------------------------------------------|
| Static    | 2     | ~5%    | clippy zero-panic lint + cargo-deny; compile-time guarantees from type system                |
| Unit      | 36    | ~55%   | Pure Calc functions + param validation + error mapping + boundary tests + mutation killers  |
| Integration | 22  | ~52%   | Tool handlers calling real INDEX.json + real Tantivy index + real rmcp ServerHandler wiring  |
| E2E       | 4     | ~10%   | Full stdio transport cycle: initialize → list tools → call tool → shutdown via pipe          |

This is slightly more unit-heavy than the 60/30/5/5 ideal because `CtdMcpServer` has four pure internal helpers that are combinatorially rich and deserve exhaustive Calc-layer coverage. Integration remains the largest layer by scenario count.

---

## 1. Behavior Inventory

Every behavior the system guarantees, expressed as "[Subject] [action] [outcome] when [condition]".

### Construction & Lifecycle

1. **CtdMcpServer::new** returns `Ok(CtdMcpServer)` when `index_dir` exists and is a directory
2. **CtdMcpServer::new** returns `Err(CtdMcpError::IoError)` when `index_dir` does not exist
3. **CtdMcpServer::new** returns `Err(CtdMcpError::IoError)` when `index_dir` is a file, not a directory
4. **CtdMcpServer::new** canonicalizes the path so `.` resolves to absolute
4a. **CtdMcpServer::new** returns `Err(CtdMcpError::IoError)` when `index_dir` is an empty string path
4b. **CtdMcpServer::new** returns `Err(CtdMcpError::IoError)` when `index_dir` has no read permissions
4c. **CtdMcpServer::new** returns `Ok(CtdMcpServer)` with canonicalized target when `index_dir` is a symlink to a real directory
5. **CtdMcpServer::info** returns server name "ctd-mcp" and version from `CARGO_PKG_VERSION`
6. **run** drives stdio transport to completion and returns `Ok(())` when stdin reaches EOF
7. **run** returns `Err` when index_dir does not exist

### Tool: search_docs

8. **search_docs** returns ranked BM25 results with rank/category/score/title/path/summary when query matches documents
9. **search_docs** returns `"No results found."` when query matches no documents
10. **search_docs** returns `Err(CtdMcpError::InvalidInput)` when query is empty string
11. **search_docs** returns `Err(CtdMcpError::InvalidInput)` when query is whitespace-only
12. **search_docs** returns `Err(CtdMcpError::InvalidInput)` when limit is 0
13. **search_docs** returns `Err(CtdMcpError::InvalidInput)` when limit exceeds 100
14. **search_docs** returns exactly 10 results when limit not provided and 15+ docs match (asserts EXACTLY 10, not "10 or fewer")
14a. **search_docs** accepts limit=100 as valid maximum and returns up to 100 results
14b. **search_docs** does not block the async runtime — concurrent `tokio::time::timeout` completes during search
15. **search_docs** accepts limit=100 as valid maximum and returns exactly 100 results when ≥100 docs match
16. **search_docs** returns `Err(CtdMcpError::IndexNotFound)` when INDEX.json is missing
16. **search_docs** returns `Err(CtdMcpError::IndexCorrupted)` when INDEX.json is malformed JSON
17. **search_docs** returns `Err(CtdMcpError::SearchIndexError)` when Tantivy index cannot be opened or rebuilt
18. **search_docs** returns `Err(CtdMcpError::QueryError)` when Tantivy cannot parse the query
19. **search_docs** results are sorted by BM25 score descending
20. **search_docs** limits results to at most `limit` entries

### Tool: read_chunk

21. **read_chunk** returns chunk content verbatim when `id` matches a chunk_id in INDEX.json
22. **read_chunk** returns document summary when `id` matches a doc_id (not a chunk_id)
23. **read_chunk** returns not-found message when `id` matches neither chunk nor document
24. **read_chunk** returns `Err(CtdMcpError::InvalidInput)` when `id` is empty string
25. **read_chunk** returns `Err(CtdMcpError::InvalidInput)` when `id` is whitespace-only
26. **read_chunk** returns `Err(CtdMcpError::IndexNotFound)` when INDEX.json is missing
27. **read_chunk** returns `Err(CtdMcpError::IndexCorrupted)` when INDEX.json is malformed JSON
28. **read_chunk** returns `Err(CtdMcpError::IoError)` when INDEX.json file read fails

### Tool: get_related_concepts

29. **get_related_concepts** returns edges where `id` appears as `from` field
30. **get_related_concepts** returns edges where `id` appears as `to` field (labeled inbound)
31. **get_related_concepts** returns `"No related concepts found for ID '{id}'"` when no edges reference `id`
32. **get_related_concepts** returns `Err(CtdMcpError::InvalidInput)` when `id` is empty string
33. **get_related_concepts** returns `Err(CtdMcpError::InvalidInput)` when `id` is whitespace-only
34. **get_related_concepts** returns `Err(CtdMcpError::IndexNotFound)` when INDEX.json is missing
35. **get_related_concepts** returns `Err(CtdMcpError::IndexCorrupted)` when INDEX.json is malformed JSON
36. **get_related_concepts** returns `Err(CtdMcpError::IoError)` when INDEX.json file read fails

### Invariant Behaviors

37. **All public functions** never panic under any input (INV1 zero-panic)
38. **All tool handlers** produce identical output given identical input and INDEX.json (INV4 idempotent)
39. **Parameter structs** own all data with no borrowed lifetimes (INV5 owned params)
40. **Blocking search calls** are isolated via `tokio::task::spawn_blocking` (INV6 blocking isolation)
41. **Error mapping** converts every `CtdMcpError` variant to `rmcp::model::Error` with code -32603
42. **read_chunk** prefers chunk match over document match when both exist for same id

---

## 2. Trophy Allocation

| # | Behavior | Layer | Rationale |
|---|----------|-------|-----------|
| 1 | new succeeds with valid dir | integration | touches filesystem |
| 2 | new fails when dir missing | integration | touches filesystem |
| 3 | new fails when dir is file | integration | touches filesystem |
| 4 | new canonicalizes path | integration | filesystem canonicalization |
| 5 | info returns name/version | unit | pure data return from struct |
| 6 | run returns on EOF | e2e | full stdio transport lifecycle |
| 7 | run fails with bad dir | integration | filesystem + async runtime |
| 8 | search_docs returns ranked results | integration | real INDEX.json + real Tantivy |
| 9 | search_docs returns "No results found." | integration | real search with no matches |
| 10 | search_docs rejects empty query | unit | param validation, pure |
| 11 | search_docs rejects whitespace query | unit | param validation, pure |
| 12 | search_docs rejects limit 0 | unit | param validation, pure |
| 13 | search_docs rejects limit > 100 | unit | param validation, pure |
| 14 | search_docs defaults limit to 10 | integration | real search with default limit |
| 15 | search_docs IndexNotFound | integration | filesystem precond failure |
| 16 | search_docs IndexCorrupted | integration | filesystem + parse failure |
| 17 | search_docs SearchIndexError | integration | real Tantivy failure path |
| 18 | search_docs QueryError | integration | real Tantivy query failure |
| 19 | search_docs results sorted by score | integration | real Tantivy scoring |
| 20 | search_docs respects limit | integration | real search with limit |
| 21 | read_chunk returns chunk content | integration | real INDEX.json lookup |
| 22 | read_chunk returns doc summary | integration | real INDEX.json lookup |
| 23 | read_chunk returns not-found | integration | real INDEX.json lookup |
| 24 | read_chunk rejects empty id | unit | param validation, pure |
| 25 | read_chunk rejects whitespace id | unit | param validation, pure |
| 26 | read_chunk IndexNotFound | integration | filesystem precond failure |
| 27 | read_chunk IndexCorrupted | integration | filesystem + parse failure |
| 28 | read_chunk IoError | integration | filesystem failure |
| 29 | get_related_concepts returns from-edges | integration | real INDEX.json graph |
| 30 | get_related_concepts returns to-edges | integration | real INDEX.json graph |
| 31 | get_related_concepts returns empty message | integration | real INDEX.json graph |
| 32 | get_related_concepts rejects empty id | unit | param validation, pure |
| 33 | get_related_concepts rejects whitespace id | unit | param validation, pure |
| 34 | get_related_concepts IndexNotFound | integration | filesystem precond failure |
| 35 | get_related_concepts IndexCorrupted | integration | filesystem + parse failure |
| 36 | get_related_concepts IoError | integration | filesystem failure |
| 37 | zero-panic invariant | static | clippy lint deny + code review |
| 38 | idempotent tool calls | proptest | property: same input → same output |
| 39 | owned params | static | compile-time check (no lifetimes in structs) |
| 40 | blocking isolation | integration | verify spawn_blocking via timing or runtime check |
| 41 | error mapping to rmcp | unit | pure conversion function |
| 42 | read_chunk chunk preferred over doc | integration | real INDEX.json with ambiguous id |

**Total: 14 unit / 22 integration / 4 e2e / 2 static** (some behaviors split across layers)

---

## 3. BDD Scenarios

### 3.1 Construction: CtdMcpServer::new

#### Behavior: new succeeds with valid dir
```
Given: a temporary directory exists on disk
When: CtdMcpServer::new(temp_dir) is called
Then: Ok(CtdMcpServer) is returned
And:  server.index_dir equals the canonicalized absolute path of temp_dir
```
Test: `fn new_returns_ok_when_dir_exists()`

#### Behavior: new fails when dir missing
```
Given: "/nonexistent/path/xyz" does not exist on disk
When: CtdMcpServer::new(PathBuf::from("/nonexistent/path/xyz")) is called
Then: Err(CtdMcpError::IoError { reason }) is returned
And:  reason contains "directory" or "not found" or "No such file"
```
Test: `fn new_returns_io_error_when_dir_missing()`

#### Behavior: new fails when dir is file
```
Given: a file (not a directory) exists at the given path
When: CtdMcpServer::new(file_path) is called
Then: Err(CtdMcpError::IoError { reason }) is returned
```
Test: `fn new_returns_io_error_when_path_is_file()`

#### Behavior: new canonicalizes path
```
Given: a temporary directory exists and current_dir is its parent
When: CtdMcpServer::new(PathBuf::from(".")) is called
Then: Ok(server) is returned
And:  server.index_dir is an absolute path (starts with '/')
```
Test: `fn new_canonicalizes_relative_path()`

#### Behavior: new rejects empty string path
```
Given: PathBuf::from("") as index_dir
When:  CtdMcpServer::new(index_dir) is called
Then:  Err(CtdMcpError::IoError) with message containing "directory" or "path"
```
Test: `fn new_returns_error_for_empty_string_path()`

#### Behavior: new handles permission-denied directory
```
Given: a TempDir with mode 0o000 (no permissions) as index_dir
When:  CtdMcpServer::new(index_dir) is called
Then:  Err(CtdMcpError::IoError) with message containing "permission" or "denied"
```
Test: `fn new_handles_permission_denied_dir()`

#### Behavior: new handles symlink directory
```
Given: a TempDir containing INDEX.json,And:  a symlink pointing to that TempDir
When:  CtdMcpServer::new(symlink_path) is called
Then:  Ok(CtdMcpServer) with index_dir canonicalized to the real target path
```
Test: `fn new_handles_symlink_directory()`

---

### 3.2 Server Info

#### Behavior: info returns name/version
```
Given: a CtdMcpServer constructed with a valid directory
When: server.info() is called via the ServerHandler trait
Then: ServerInfo.server_name equals "ctd-mcp"
And:  ServerInfo.version equals CARGO_PKG_VERSION
```
Test: `fn info_returns_ctd_mcp_name_and_version()`

---

### 3.3 Tool: search_docs

#### Behavior: search_docs returns ranked results
```
Given: INDEX.json contains 3 documents with titles containing "kubernetes pods tutorial", "kubernetes services", and "python flask"
And:   Tantivy index is built from those documents
When:  search_docs(SearchDocsParams { query: "kubernetes", limit: 10 }) is called
Then:  Ok(CallToolResult) with content type "text"
And:   text contains "1." and "2." (at least 2 results)
And:   first result contains "kubernetes" (case-insensitive) in the text
And:   each result entry contains "Score:" followed by a float
```
Test: `fn search_docs_returns_ranked_results_when_query_matches()`

#### Behavior: search_docs returns "No results found."
```
Given: INDEX.json contains 1 document about "rust programming"
And:   Tantivy index is built from that document
When:  search_docs(SearchDocsParams { query: "xyzzyplughnothing", limit: 10 }) is called
Then:  Ok(CallToolResult) with content type "text"
And:   text equals "No results found."
```
Test: `fn search_docs_returns_no_results_message_when_no_match()`

#### Behavior: search_docs rejects empty query
```
Given: a valid CtdMcpServer with INDEX.json present
When:  search_docs(SearchDocsParams { query: "", limit: 10 }) is called
Then:  Err(McpError) where message contains "Invalid input"
And:   the underlying CtdMcpError is InvalidInput { detail: "query must be non-empty" } (or equivalent)
```
Test: `fn search_docs_returns_invalid_input_when_query_empty()`

#### Behavior: search_docs rejects whitespace query
```
Given: a valid CtdMcpServer with INDEX.json present
When:  search_docs(SearchDocsParams { query: "   \t  ", limit: 10 }) is called
Then:  Err(McpError) where message contains "Invalid input"
```
Test: `fn search_docs_returns_invalid_input_when_query_whitespace()`

#### Behavior: search_docs rejects limit 0
```
Given: a valid CtdMcpServer with INDEX.json present
When:  search_docs(SearchDocsParams { query: "rust", limit: 0 }) is called
Then:  Err(McpError) where message contains "Invalid input" and "limit"
```
Test: `fn search_docs_returns_invalid_input_when_limit_zero()`

#### Behavior: search_docs rejects limit > 100
```
Given: a valid CtdMcpServer with INDEX.json present
When:  search_docs(SearchDocsParams { query: "rust", limit: 101 }) is called
Then:  Err(McpError) where message contains "Invalid input" and "limit"
```
Test: `fn search_docs_returns_invalid_input_when_limit_exceeds_100()`

#### Behavior: search_docs defaults limit to 10
```
Given: INDEX.json contains 15 documents all containing the word "testdoc"
And:   Tantivy index is built from those documents
When:  search_docs(SearchDocsParams { query: "testdoc", limit: 10 }) is called  (default)
Then:  Ok(CallToolResult) with exactly 10 result entries (Given 15 docs, must return exactly 10)
```
Test: `fn search_docs_defaults_limit_to_10()`

#### Behavior: search_docs accepts limit=100 as valid maximum
```
Given: INDEX.json contains 120 documents all containing "bulkword"
And:   Tantivy index is built from those documents
When:  search_docs(SearchDocsParams { query: "bulkword", limit: 100 }) is called
Then:  Ok(CallToolResult) with exactly 100 result entries
And:   each entry contains "bulkword" in its text
```
Test: `fn search_docs_accepts_limit_100_as_valid_max()`

#### Behavior: search_docs does not block the async runtime
```
Given: INDEX.json contains 50 documents all containing "slowquery"
And:   Tantivy index is built from those documents
When:  search_docs is called concurrently with tokio::time::timeout(Duration::from_secs(5))
Then:  The timeout does NOT fire (search completes within 5 seconds)
And:   Ok(CallToolResult) is returned with matching results
```
Test: `fn search_docs_does_not_block_async_runtime()`

#### Behavior: search_docs IndexNotFound
```
Given: a CtdMcpServer whose index_dir contains no INDEX.json file
When:  search_docs(SearchDocsParams { query: "rust", limit: 10 }) is called
Then:  Err(McpError) where message matches "INDEX.json not found"
And:   error code equals -32603
```
Test: `fn search_docs_returns_index_not_found_when_json_missing()`

#### Behavior: search_docs IndexCorrupted
```
Given: a CtdMcpServer whose index_dir contains INDEX.json with content "not valid json{{{"
When:  search_docs(SearchDocsParams { query: "rust", limit: 10 }) is called
Then:  Err(McpError) where message matches "Failed to parse INDEX.json"
And:   error code equals -32603
```
Test: `fn search_docs_returns_index_corrupted_when_json_malformed()`

#### Behavior: search_docs results sorted by score descending
```
Given: INDEX.json contains 3 documents with varying relevance to the query "kubernetes deployment"
When:  search_docs(SearchDocsParams { query: "kubernetes deployment", limit: 10 }) is called
Then:  Ok(CallToolResult)
And:   result text lists scores in descending order
And:   score at position N is >= score at position N+1
```
Test: `fn search_docs_results_sorted_by_score_descending()`

#### Behavior: search_docs respects limit
```
Given: INDEX.json contains 5 documents matching "test"
When:  search_docs(SearchDocsParams { query: "test", limit: 3 }) is called
Then:  Ok(CallToolResult) with at most 3 result entries
```
Test: `fn search_docs_respects_limit_parameter()`

---

### 3.4 Tool: read_chunk

#### Behavior: read_chunk returns chunk content
```
Given: INDEX.json contains chunks array with chunk_id "chunk-abc" and content "This is chunk ABC verbatim."
When:  read_chunk(ReadChunkParams { id: "chunk-abc" }) is called
Then:  Ok(CallToolResult) with content type "text"
And:   text equals "This is chunk ABC verbatim."
```
Test: `fn read_chunk_returns_chunk_content_when_id_matches_chunk()`

#### Behavior: read_chunk returns document summary
```
Given: INDEX.json contains documents array with doc_id "doc-123" and summary "Summary of doc 123."
And:   no chunk with chunk_id "doc-123" exists
When:  read_chunk(ReadChunkParams { id: "doc-123" }) is called
Then:  Ok(CallToolResult) with content type "text"
And:   text equals "Document doc-123:\nSummary:\nSummary of doc 123."
```
Test: `fn read_chunk_returns_doc_summary_when_id_matches_doc()`

#### Behavior: read_chunk returns not-found message
```
Given: INDEX.json contains documents and chunks, none with id "nonexistent-xyz"
When:  read_chunk(ReadChunkParams { id: "nonexistent-xyz" }) is called
Then:  Ok(CallToolResult) with content type "text"
And:   text equals "ID 'nonexistent-xyz' not found in chunks or documents"
```
Test: `fn read_chunk_returns_not_found_when_id_matches_nothing()`

#### Behavior: read_chunk rejects empty id
```
Given: a valid CtdMcpServer with INDEX.json present
When:  read_chunk(ReadChunkParams { id: "" }) is called
Then:  Err(McpError) where message contains "Invalid input"
```
Test: `fn read_chunk_returns_invalid_input_when_id_empty()`

#### Behavior: read_chunk rejects whitespace id
```
Given: a valid CtdMcpServer with INDEX.json present
When:  read_chunk(ReadChunkParams { id: "  \t " }) is called
Then:  Err(McpError) where message contains "Invalid input"
```
Test: `fn read_chunk_returns_invalid_input_when_id_whitespace()`

#### Behavior: read_chunk IndexNotFound
```
Given: a CtdMcpServer whose index_dir contains no INDEX.json file
When:  read_chunk(ReadChunkParams { id: "anything" }) is called
Then:  Err(McpError) where message matches "INDEX.json not found"
```
Test: `fn read_chunk_returns_index_not_found_when_json_missing()`

#### Behavior: read_chunk IndexCorrupted
```
Given: a CtdMcpServer whose index_dir contains INDEX.json with content "}invalid{"
When:  read_chunk(ReadChunkParams { id: "anything" }) is called
Then:  Err(McpError) where message matches "Failed to parse INDEX.json"
```
Test: `fn read_chunk_returns_index_corrupted_when_json_malformed()`

#### Behavior: read_chunk chunk preferred over document
```
Given: INDEX.json contains a chunk with chunk_id "shared-id" and content "chunk content"
And:   INDEX.json contains a document with doc_id "shared-id" and summary "doc summary"
When:  read_chunk(ReadChunkParams { id: "shared-id" }) is called
Then:  Ok(CallToolResult) with content type "text"
And:   text equals "chunk content" (chunk match wins)
```
Test: `fn read_chunk_prefers_chunk_match_over_doc_match()`

---

### 3.5 Tool: get_related_concepts

#### Behavior: get_related_concepts returns from-edges
```
Given: INDEX.json graph.edges contains { "from": "node-a", "to": "node-b", "relationship_type": "Parent" }
When:  get_related_concepts(GetRelatedConceptsParams { id: "node-a" }) is called
Then:  Ok(CallToolResult) with content type "text"
And:   text contains "Related concepts for 'node-a':"
And:   text contains "- node-b (Relationship: Parent)"
```
Test: `fn get_related_returns_from_edges_when_id_is_source()`

#### Behavior: get_related_concepts returns to-edges
```
Given: INDEX.json graph.edges contains { "from": "node-b", "to": "node-a", "relationship_type": "Related" }
When:  get_related_concepts(GetRelatedConceptsParams { id: "node-a" }) is called
Then:  Ok(CallToolResult) with content type "text"
And:   text contains "Related concepts for 'node-a':"
And:   text contains "- node-b (Relationship: Related - inbound)"
```
Test: `fn get_related_returns_inbound_edges_when_id_is_target()`

#### Behavior: get_related_concepts returns empty message
```
Given: INDEX.json graph.edges contains no edges referencing "orphan-node"
When:  get_related_concepts(GetRelatedConceptsParams { id: "orphan-node" }) is called
Then:  Ok(CallToolResult) with content type "text"
And:   text equals "No related concepts found for ID 'orphan-node'"
```
Test: `fn get_related_returns_empty_message_when_no_edges()`

#### Behavior: get_related_concepts rejects empty id
```
Given: a valid CtdMcpServer with INDEX.json present
When:  get_related_concepts(GetRelatedConceptsParams { id: "" }) is called
Then:  Err(McpError) where message contains "Invalid input"
```
Test: `fn get_related_returns_invalid_input_when_id_empty()`

#### Behavior: get_related_concepts rejects whitespace id
```
Given: a valid CtdMcpServer with INDEX.json present
When:  get_related_concepts(GetRelatedConceptsParams { id: "  \n " }) is called
Then:  Err(McpError) where message contains "Invalid input"
```
Test: `fn get_related_returns_invalid_input_when_id_whitespace()`

#### Behavior: get_related_concepts IndexNotFound
```
Given: a CtdMcpServer whose index_dir contains no INDEX.json file
When:  get_related_concepts(GetRelatedConceptsParams { id: "anything" }) is called
Then:  Err(McpError) where message matches "INDEX.json not found"
```
Test: `fn get_related_returns_index_not_found_when_json_missing()`

#### Behavior: get_related_concepts IndexCorrupted
```
Given: a CtdMcpServer whose index_dir contains INDEX.json with content "{bad json"
When:  get_related_concepts(GetRelatedConceptsParams { id: "anything" }) is called
Then:  Err(McpError) where message matches "Failed to parse INDEX.json"
```
Test: `fn get_related_returns_index_corrupted_when_json_malformed()`

---

### 3.6 Error Mapping

#### Behavior: CtdMcpError maps to rmcp Error with code -32603
```
Given: any CtdMcpError variant (each tested individually)
When:  CtdMcpError is converted to rmcp::model::Error via From trait
Then:  error.code equals -32603
And:   error.message contains the error display string
```
Test variants:
- `fn error_map_index_not_found_has_code_neg32603()`
- `fn error_map_index_corrupted_has_code_neg32603()`
- `fn error_map_invalid_input_has_code_neg32603()`
- `fn error_map_search_index_error_has_code_neg32603()`
- `fn error_map_query_error_has_code_neg32603()`
- `fn error_map_io_error_has_code_neg32603()`
- `fn error_map_internal_has_code_neg32603()`

---

### 3.7 Entrypoint: run

#### Behavior: run drives stdio to completion on EOF
```
Given: a valid index_dir with INDEX.json
And:   stdin is a pipe that sends an initialize request then closes
When:  run(index_dir) is called
Then:  Ok(()) is returned
And:   stdout received a valid JSON-RPC initialize response
```
Test: `fn run_completes_on_stdin_eof()`

#### Behavior: run fails with bad dir
```
Given: index_dir path does not exist
When:  run(index_dir) is called
Then:  Err(CtdMcpError::IoError { .. }) is returned
```
Test: `fn run_returns_error_when_dir_missing()`

---

### 3.8 Internal Helpers (tested via public API, but pure functions tested directly)

#### Behavior: format_search_results formats non-empty results
```
Given: a Vec<SearchResult> with 2 entries
When:  format_search_results(&results) is called
Then:  returned string starts with "1. ["
And:   contains "2. ["
And:   contains "Score: "
And:   contains "Title: "
And:   contains "Path: "
And:   contains "Summary: "
And:   entries are separated by "---\n"
```
Test: `fn format_search_results_formats_ranked_entries()`

#### Behavior: format_search_results returns empty message
```
Given: an empty Vec<SearchResult>
When:  format_search_results(&[]) is called
Then:  returned string equals "No results found."
```
Test: `fn format_search_results_returns_no_results_for_empty()`

#### Behavior: find_chunk_content returns content when found
```
Given: parsed INDEX.json Value with chunks array containing chunk_id "c1" with content "hello"
When:  find_chunk_content(&json, "c1") is called
Then:  Some("hello")
```
Test: `fn find_chunk_content_returns_some_when_id_matches()`

#### Behavior: find_chunk_content returns None when not found
```
Given: parsed INDEX.json Value with chunks array not containing "missing"
When:  find_chunk_content(&json, "missing") is called
Then:  None
```
Test: `fn find_chunk_content_returns_none_when_no_match()`

#### Behavior: find_chunk_content handles empty chunks array
```
Given: parsed INDEX.json Value with chunks: []
When:  find_chunk_content(&json, "any") is called
Then:  None
```
Test: `fn find_chunk_content_returns_none_when_chunks_empty()`

#### Behavior: find_chunk_content returns first match for duplicate chunk_ids
```
Given: parsed INDEX.json Value with two chunks having chunk_id "dup-1" but different content
When:  find_chunk_content(&json, "dup-1") is called
Then:  Some(first_content) — first match wins
```
Test: `fn find_chunk_content_returns_first_match_for_duplicate_ids()`

#### Behavior: find_chunk_content returns Some("") for chunk with empty content
```
Given: parsed INDEX.json Value with chunk "e1" having content ""
When:  find_chunk_content(&json, "e1") is called
Then:  Some("")
```
Test: `fn find_chunk_content_returns_some_empty_string_for_empty_content()`

#### Behavior: find_doc_summary returns summary when found
```
Given: parsed INDEX.json Value with documents array containing doc_id "d1" with summary "my summary"
When:  find_doc_summary(&json, "d1") is called
Then:  Some("my summary")
```
Test: `fn find_doc_summary_returns_some_when_id_matches()`

#### Behavior: find_doc_summary returns None when not found
```
Given: parsed INDEX.json Value with documents array not containing "missing"
When:  find_doc_summary(&json, "missing") is called
Then:  None
```
Test: `fn find_doc_summary_returns_none_when_no_match()`

#### Behavior: find_doc_summary handles empty documents array
```
Given: parsed INDEX.json Value with documents: []
When:  find_doc_summary(&json, "any") is called
Then:  None
```
Test: `fn find_doc_summary_returns_none_when_docs_empty()`

#### Behavior: find_doc_summary returns first match for duplicate doc_ids
```
Given: parsed INDEX.json Value with two documents having doc_id "dup-d1" but different summaries
When:  find_doc_summary(&json, "dup-d1") is called
Then:  Some(first_summary) — first match wins
```
Test: `fn find_doc_summary_returns_first_match_for_duplicate_ids()`

#### Behavior: find_doc_summary returns Some("") for doc with empty summary
```
Given: parsed INDEX.json Value with doc "e2" having summary ""
When:  find_doc_summary(&json, "e2") is called
Then:  Some("")
```
Test: `fn find_doc_summary_returns_some_empty_string_for_empty_summary()`

#### Behavior: find_related_edges returns formatted edges for matching id
```
Given: parsed INDEX.json Value with graph.edges = [
  { "from": "a", "to": "b", "relationship_type": "Parent" },
  { "from": "c", "to": "a", "relationship_type": "Related" }
]
When:  find_related_edges(&json, "a") is called
Then:  vec with 2 strings:
  "- b (Relationship: Parent)"
  "- c (Relationship: Related - inbound)"
```
Test: `fn find_related_edges_returns_formatted_edges_for_matching_id()`

#### Behavior: find_related_edges returns empty vec when no matches
```
Given: parsed INDEX.json Value with graph.edges = [
  { "from": "x", "to": "y", "relationship_type": "Parent" }
]
When:  find_related_edges(&json, "z") is called
Then:  empty Vec
```
Test: `fn find_related_edges_returns_empty_vec_when_no_matches()`

#### Behavior: find_related_edges handles missing graph key
```
Given: parsed INDEX.json Value with no "graph" key
When:  find_related_edges(&json, "any") is called
Then:  empty Vec (not panic)
```
Test: `fn find_related_edges_returns_empty_when_graph_missing()`

---

## 4. Proptest Invariants

### Proptest: find_chunk_content — lookup consistency
```
Invariant: For any valid INDEX.json Value with chunks array,
           find_chunk_content returns Some(content) for every chunk_id
           that appears in the chunks array, where content matches the
           chunk's "content" field. For any string NOT in the chunks,
           returns None.

Strategy: Generate a Vec<(String, String)> of (chunk_id, content) pairs,
          construct the JSON Value, then for each pair assert Some(content)
          and for a random non-member string assert None.

Anti-invariant: Empty string as chunk_id → still matches if the JSON
                contains an entry with chunk_id "" (this is valid behavior).
```

### Proptest: find_doc_summary — lookup consistency
```
Invariant: For any valid INDEX.json Value with documents array,
           find_doc_summary returns Some(summary) for every doc_id
           present in the documents array. For any string not in the array,
           returns None.

Strategy: Generate a Vec<(String, String)> of (doc_id, summary) pairs,
          construct the JSON Value, verify round-trip for each pair.

Anti-invariant: doc_id containing special JSON characters (quotes, backslashes)
                must still match correctly (no escaping bugs).
```

### Proptest: find_related_edges — bidirectional completeness
```
Invariant: For any set of graph edges, the union of find_related_edges
           called with any edge participant produces output containing all
           edge partners. No edge is lost.

Strategy: Generate a Vec<(String, String, String)> of (from, to, rel_type) triples.
          Build JSON. For each unique node, verify that all connected edges appear.

Anti-invariant: Edge with empty "from" or "to" → should still be findable
                if the id matches the empty string.
```

### Proptest: format_search_results — output structure
```
Invariant: For any non-empty Vec<SearchResult>, the output string:
           - Starts with "1. ["
           - Contains N occurrences of "Score: " where N = results.len()
           - Ends with "---\n"
           For empty Vec, output is exactly "No results found."

Strategy: Generate Vec<SearchResult> of length 0..20 with arbitrary strings
          and valid Score values (0.0..=100.0).

Anti-invariant: Results with empty strings for title/summary → should still
                produce valid formatted output without panic.
```

### Proptest: idempotent tool calls
```
Invariant: For any valid (INDEX.json, query, limit) triple,
           calling search_docs twice produces byte-identical CallToolResult text.
           Same for read_chunk and get_related_concepts.

Strategy: Generate fixed INDEX.json corpus with known content.
          For arbitrary valid queries and ids, call tool twice, assert equality.

Anti-invariant: If INDEX.json is modified between calls, results may differ
                (but this is outside the guarantee — no filesystem mutation during calls).
```

### Proptest: error mapping preserves message
```
Invariant: For any CtdMcpError variant, converting to rmcp::model::Error
           produces error.message containing the original .to_string() output.

Strategy: Generate each CtdMcpError variant with arbitrary String payloads.
          Assert the mapping preserves the message text.

Anti-invariant: Empty string as reason → message should still contain the
                variant prefix (e.g., "I/O error: ").
```

---

## 5. Fuzz Targets

### Fuzz Target: load_index_json — JSON parser
```
Input type: arbitrary bytes (treated as file content)
Risk: panic in serde_json::from_str, OOM on huge JSON, logic error in
      graph traversal with malformed structure
Corpus seeds:
  - valid INDEX.json with documents, chunks, and graph
  - empty string
  - single null byte
  - deeply nested JSON (100 levels)
  - JSON with huge string values (1MB)
  - valid JSON but missing "documents" key
  - valid JSON with "chunks" as object instead of array
  - valid JSON with "graph.edges" containing null entries
```

### Fuzz Target: search_docs query — user-provided query string
```
Input type: arbitrary &str
Risk: panic in Tantivy query parser, regex DoS in validation,
      unhandled Tantivy error variant
Corpus seeds:
  - normal English text: "kubernetes pods"
  - empty string ""
  - whitespace only "  \t\n "
  - Tantivy special chars: "test*", "test?", "(a OR b)"
  - unicode: "日本語テスト", "emoji 🚀"
  - very long string (64KB)
  - null bytes "test\0query"
  - backslash-heavy: "\\\\\\\\"
```

### Fuzz Target: find_related_edges — graph edge traversal
```
Input type: arbitrary (serde_json::Value, &str) pair
Risk: panic on malformed graph structure (missing fields, wrong types),
      infinite loop if edge structure is unexpected
Corpus seeds:
  - valid graph with 10 edges
  - graph with edges missing "from", "to", or "relationship_type"
  - graph where "edges" is a string instead of array
  - graph where edges contain numbers instead of strings
  - graph with self-referencing edges (from == to == id)
  - id containing special JSON characters
```

---

## 6. Kani Harnesses

### Kani Harness: limit bounds verification
```
Property: For any u32 value `limit`, the validation logic
          (limit > 0 && limit <= 100) correctly classifies all inputs
          into Valid or InvalidInput. No u32 value causes the check
          to produce the wrong classification.
Bound: full u32 range (exhaustive via Kani's non-deterministic input)
Rationale: Integer boundary bugs are subtle and easy to miss with
           random testing alone. Kani proves exhaustiveness.
```

### Kani Harness: find_chunk_content index safety
```
Property: For any valid JSON array of chunks, find_chunk_content never
          panics on index out of bounds. All array accesses are guarded.
Bound: chunks array length 0..10, chunk_id length 0..20
Rationale: JSON traversal involves .as_array() and .get() chains that
           could panic if assumptions about structure are violated.
```

---

## 7. Mutation Testing Checkpoints

**Target: ≥90% mutation kill rate**

### Critical Mutations to Catch

| Mutation | Caught by test |
|----------|---------------|
| Remove `query.trim().is_empty()` check in search_docs | `search_docs_returns_invalid_input_when_query_whitespace` |
| Remove `limit > 0` check | `search_docs_returns_invalid_input_when_limit_zero` |
| Change `limit <= 100` to `limit < 100` | `search_docs_returns_invalid_input_when_limit_exceeds_100` — specifically test limit=100 passes |
| Remove INDEX.json existence check in search_docs | `search_docs_returns_index_not_found_when_json_missing` |
| Swap chunk vs document lookup order in read_chunk | `read_chunk_prefers_chunk_match_over_doc_match` |
| Remove edge matching for `to` field in get_related | `get_related_returns_inbound_edges_when_id_is_target` |
| Remove edge matching for `from` field in get_related | `get_related_returns_from_edges_when_id_is_source` |
| Change error code from -32603 to -32600 | `error_map_*_has_code_neg32603` (all 7 variants) |
| Remove "No results found." branch | `search_docs_returns_no_results_message_when_no_match` |
| Remove "not found" branch in read_chunk | `read_chunk_returns_not_found_when_id_matches_nothing` |
| Remove "No related concepts" branch | `get_related_returns_empty_message_when_no_edges` |
| Change default limit from 10 to 5 | `search_docs_defaults_limit_to_10` |
| Remove canonicalization in `new` | `new_canonicalizes_relative_path` |
| Skip empty id validation in read_chunk | `read_chunk_returns_invalid_input_when_id_empty` |
| Skip empty id validation in get_related | `get_related_returns_invalid_input_when_id_empty` |
| Remove "inbound" label for to-edges | `get_related_returns_inbound_edges_when_id_is_target` |
| Change score sort order (ascending) | `search_docs_results_sorted_by_score_descending` |
| Remove spawn_blocking wrapper | integration test verifying non-blocking behavior |
| Remove graph.edges handling for null entries | `find_related_edges_returns_empty_when_graph_missing` |
| Flip `new` directory existence check | `new_returns_io_error_when_dir_missing` |

---

## 8. Combinatorial Coverage Matrix

### 8.1 CtdMcpServer::new

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| valid directory | existing dir path | Ok(CtdMcpServer { canonicalized path }) | integration |
| missing directory | nonexistent path | Err(CtdMcpError::IoError { reason: contains "directory" }) | integration |
| path is file | path to existing file | Err(CtdMcpError::IoError { .. }) | integration |
| relative path | "." | Ok(CtdMcpServer { absolute path }) | integration |

### 8.2 search_docs

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| happy path: matching query | query="kubernetes", limit=10 | Ok(text containing "1. [" and "Score:") | integration |
| no matches | query="xyznonexistent" | Ok(text = "No results found.") | integration |
| empty query | query="" | Err(McpError { code: -32603, message contains "Invalid input" }) | unit |
| whitespace query | query="   \t" | Err(McpError { code: -32603, message contains "Invalid input" }) | unit |
| limit = 0 | limit=0 | Err(McpError { message contains "limit" }) | unit |
| limit = 101 | limit=101 | Err(McpError { message contains "limit" }) | unit |
| limit = 1 | limit=1 | Ok(text with at most 1 result) | integration |
| limit = 100 | limit=100 | Ok(text with at most 100 results) | integration |
| INDEX.json missing | empty dir | Err(McpError { message contains "INDEX.json not found" }) | integration |
| INDEX.json corrupted | invalid JSON content | Err(McpError { message contains "Failed to parse" }) | integration |
| boundary: query length 1 | query="a" (single char) | Ok(text) — may be empty or have matches | integration |
| boundary: default limit | limit=10 (default) | Ok(text with ≤10 results) | integration |
| sorted by score | multiple matching docs | Ok(text with scores in descending order) | integration |

### 8.3 read_chunk

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| chunk_id match | id matching a chunk | Ok(text = chunk content verbatim) | integration |
| doc_id match (no chunk match) | id matching a doc | Ok(text = "Document {id}:\nSummary:\n{summary}") | integration |
| no match | id not in chunks or docs | Ok(text = "ID '{id}' not found in chunks or documents") | integration |
| chunk + doc with same id | id in both chunks and docs | Ok(text = chunk content, not doc summary) | integration |
| empty id | id="" | Err(McpError { message contains "Invalid input" }) | unit |
| whitespace id | id="  \t " | Err(McpError { message contains "Invalid input" }) | unit |
| INDEX.json missing | empty dir | Err(McpError { message contains "INDEX.json not found" }) | integration |
| INDEX.json corrupted | invalid JSON | Err(McpError { message contains "Failed to parse" }) | integration |

### 8.4 get_related_concepts

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| id is source (from) | id in from field | Ok(text contains "- {to} (Relationship: {type})") | integration |
| id is target (to) | id in to field | Ok(text contains "- {from} (Relationship: {type} - inbound)") | integration |
| no edges | id not in any edge | Ok(text = "No related concepts found for ID '{id}'") | integration |
| multiple edges | id in 3+ edges | Ok(text contains 3+ lines with "- ") | integration |
| empty id | id="" | Err(McpError { message contains "Invalid input" }) | unit |
| whitespace id | id=" \n" | Err(McpError { message contains "Invalid input" }) | unit |
| INDEX.json missing | empty dir | Err(McpError { message contains "INDEX.json not found" }) | integration |
| INDEX.json corrupted | invalid JSON | Err(McpError { message contains "Failed to parse" }) | integration |

### 8.5 Error Mapping (From<CtdMcpError> for rmcp::model::Error)

| Scenario | CtdMcpError Variant | Expected Output | Layer |
|----------|---------------------|-----------------|-------|
| IndexNotFound | { path: "/tmp/x" } | Error { code: -32603, message: contains "/tmp/x" } | unit |
| IndexCorrupted | { reason: "parse err" } | Error { code: -32603, message: contains "parse err" } | unit |
| InvalidInput | { detail: "bad" } | Error { code: -32603, message: contains "bad" } | unit |
| SearchIndexError | { reason: "tantivy" } | Error { code: -32603, message: contains "tantivy" } | unit |
| QueryError | { reason: "syntax" } | Error { code: -32603, message: contains "syntax" } | unit |
| IoError | { reason: "perm" } | Error { code: -32603, message: contains "perm" } | unit |
| Internal | { reason: "unexpected" } | Error { code: -32603, message: contains "unexpected" } | unit |

### 8.6 Internal Pure Functions

| Scenario | Input Class | Expected Output | Layer |
|----------|-------------|-----------------|-------|
| format_search_results: 0 results | empty vec | "No results found." | unit |
| format_search_results: 1 result | vec with 1 entry | string starting with "1. [" | unit |
| format_search_results: 3 results | vec with 3 entries | string with "1.", "2.", "3." | unit |
| find_chunk_content: found | valid json + existing id | Some(content string) | unit |
| find_chunk_content: not found | valid json + missing id | None | unit |
| find_chunk_content: empty chunks | json with chunks:[] | None | unit |
| find_doc_summary: found | valid json + existing id | Some(summary string) | unit |
| find_doc_summary: not found | valid json + missing id | None | unit |
| find_doc_summary: empty docs | json with documents:[] | None | unit |
| find_related_edges: matching id | json with edges | Vec with formatted strings | unit |
| find_related_edges: no match | json with edges | empty Vec | unit |
| find_related_edges: no graph key | json without "graph" | empty Vec (no panic) | unit |

---

## 9. Static Analysis Checks

### clippy: zero-panic enforcement
```
Configuration: clippy.toml or .clippy.toml with:
  disallowed-methods = [
    { path = "std::option::Option::unwrap", reason = "INV1: zero-panic" },
    { path = "std::result::Result::unwrap", reason = "INV1: zero-panic" },
    { path = "std::option::Option::expect", reason = "INV1: zero-panic" },
    { path = "std::result::Result::expect", reason = "INV1: zero-panic" },
  ]
Applied to: the new mcp.rs module only (not the entire crate)

Rationale: INV1 mandates no .unwrap()/.expect() anywhere in the new module.
           The clippy lint makes this a compile-time guarantee.
```

### cargo-deny: dependency licensing
```
Verify that rmcp v1.3.0 and rmcp-macros v1.3.0 are under acceptable licenses.
No new dependency introduces a license incompatible with the existing crate license.
```

---

## 10. Test Infrastructure Requirements

### Fixtures

Each integration test needs a temporary directory containing a valid INDEX.json. Create a helper:

```
struct TestIndex {
    dir: TempDir,
    // Provides methods to add documents, chunks, and graph edges
    // Writes INDEX.json and optionally builds Tantivy index
}
```

The `TestIndex` builder should support:
- `add_doc(id, title, summary, category, path)`
- `add_chunk(chunk_id, doc_id, content, heading)`
- `add_edge(from, to, relationship_type)`
- `build_on_disk() -> (TempDir, PathBuf)` — writes INDEX.json to filesystem (I/O side effect)
- `build_with_tantivy() -> (TempDir, PathBuf)` — writes INDEX.json + builds Tantivy index

### Async Test Runtime

All tool handler tests require `#[tokio::test]` since the handlers are async.
Use `tokio::test` macro with `flavor = "current_thread"` for deterministic tests.

### E2E Test Harness

The `run` function test requires spawning the server process with piped stdin/stdout:
- Create a pipe pair
- Write JSON-RPC messages to stdin
- Read responses from stdout
- Close stdin to trigger EOF and clean shutdown

---

## Open Questions

1. **rmcp v1.3.0 API stability**: The contract assumes `#[tool(boxed)]` returns `Result<CallToolResult, McpError>`. If rmcp's actual API differs (e.g., different error type), the error mapping tests will need adjustment. **Resolution**: Confirm on first compilation.

2. **`doc_id` vs `id` in INDEX.json documents**: The existing `mcp.rs` line 235 checks for `doc.get("doc_id")` but the contract spec (Post4) refers to `doc_id`. Need to verify which field name is actually used in INDEX.json documents. The current implementation checks `doc_id` in documents but documents likely use `id`. **Resolution**: Check actual INDEX.json structure.

3. **Tantivy index rebuild in tests**: Some integration tests need a Tantivy index. Should we use `rebuild_index_from_json` or manually construct via `index_documents`/`index_chunks`? The former is more realistic but slower. **Recommendation**: Use `rebuild_index_from_json` for search_docs tests (matches production path), use manual construction for targeted edge cases.

4. **`spawn_blocking` verification**: How to prove that `search_index` is called via `spawn_blocking`? Options: (a) timing-based test, (b) verify it doesn't block the runtime via concurrent tasks, (c) code review. **Recommendation**: Integration test that runs a long search concurrently with a tokio timer — if the timer fires, spawn_blocking is working.

5. **rmcp ServerHandler `info` method**: The contract spec says `info()` returns `ServerInfo`. Need to confirm rmcp's exact return type and field names. **Resolution**: Confirm on first compilation.
