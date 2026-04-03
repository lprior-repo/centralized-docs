# Contract Specification

```
bead_id: cdocs-drj
bead_title: mcp: Implement official rmcp SDK and expose semantic tools
phase: state-1
updated_at: 2026-03-28T12:50:00Z
```

## Context

- **Feature**: Replace the hand-rolled JSON-RPC loop in `centralized-docs/src/cmd/mcp.rs` with the
  official `rmcp` Rust SDK (v1.3.0). Expose the same 3 semantic tools via rmcp's `#[tool]` macro
  attribute system and `ServerHandler` trait, using `schemars` for JSON Schema generation.
- **Domain terms**:
  - **CtdMcpServer** -- the MCP server struct, holding an `index_dir: PathBuf` at construction time.
  - **Tool** -- an MCP-callable function annotated with `#[tool(boxed)]`, exposed via rmcp's
    `ServerHandler` trait.
  - **INDEX.json** -- the central metadata file containing `documents`, `chunks`, and `graph` arrays.
  - **Tantivy index** -- the BM25 full-text search index at `{index_dir}/.tantivy_index/`.
  - **Knowledge DAG** -- the directed acyclic graph stored in `INDEX.json.graph.edges` with
    `from`, `to`, and `relationship_type` fields.
- **Assumptions**:
  - `rmcp` v1.3.0 and `rmcp-macros` v1.3.0 are published on crates.io.
  - `schemars` 0.8.x is used for deriving `JsonSchema` on parameter structs.
  - The existing `search` module (`open_existing_index`, `rebuild_index_from_json`, `search_index`)
    and its types (`SearchResult`, `SearchError`, `SchemaFields`) remain unchanged and are called
    by the new MCP tool handlers.
  - `tokio` runtime is already a dependency (required by rmcp's async transport).
  - The binary entrypoint (`ctd mcp serve <dir>`) will be updated to call the new async server
    instead of the old synchronous `run_mcp_serve`.
  - Protocol version "2024-11-05" is negotiated by rmcp internally.
- **Open questions**:
  - Whether rmcp v1.3.0 `#[tool(boxed)]` requires the handler method to return
    `Result<CallToolResult, McpError>` or a custom error type. (Assumption: `McpError` from rmcp.)
  - Whether the existing `search_index` function signature will be wrapped in a `tokio::task::spawn_blocking` call since it is synchronous. (Assumption: yes, to avoid blocking the async runtime.)

## Preconditions

1. **P1 -- INDEX.json exists**: Before any tool handler is invoked, `index_dir` must contain a
   valid `INDEX.json` file. If it does not, tool handlers return a structured MCP error (not panic).
2. **P2 -- index_dir is a directory**: `CtdMcpServer::new(index_dir)` requires `index_dir` to be
   an existing directory on the filesystem. Construction fails with a descriptive error otherwise.
3. **P3 -- valid UTF-8 query**: `search_docs` requires `query` to be a non-empty UTF-8 string.
   Empty or whitespace-only queries are rejected with `CtdMcpError::InvalidInput`.
4. **P4 -- valid limit**: `search_docs` requires `limit` to be > 0 and <= 100. Out-of-range values
   are rejected with `CtdMcpError::InvalidInput`.
5. **P5 -- non-empty id**: `read_chunk` and `get_related_concepts` require `id` to be a non-empty
   string. Empty strings are rejected with `CtdMcpError::InvalidInput`.
6. **P6 -- tokio runtime**: The `run` entrypoint must be called within a tokio runtime context.
   This is satisfied by `#[tokio::main]` in the binary.

## Postconditions

1. **Post1 -- search_docs returns ranked results**: On success, `search_docs` returns a
   `CallToolResult` containing text content with results ranked by BM25 score (descending).
   Each result includes rank number, category, score, title, path, and summary.
2. **Post2 -- search_docs returns empty message on no matches**: When no documents match the query,
   the result text is `"No results found."` (not an error).
3. **Post3 -- read_chunk returns chunk content**: When `id` matches a chunk_id in INDEX.json,
   the result contains the chunk's `content` field verbatim.
4. **Post4 -- read_chunk returns document summary**: When `id` matches a doc_id (not a chunk_id),
   the result contains `"Document {id}:\nSummary:\n{summary}"`.
5. **Post5 -- read_chunk returns not-found message**: When `id` matches neither a chunk nor a
   document, the result contains `"ID '{id}' not found in chunks or documents"` (not an error).
6. **Post6 -- get_related_concepts returns graph edges**: On success, returns all edges in the
   knowledge DAG where the given `id` appears as either `from` or `to`, including relationship type.
7. **Post7 -- get_related_concepts returns empty message**: When no edges reference the `id`,
   the result contains `"No related concepts found for ID '{id}'"` (not an error).
8. **Post8 -- no panics**: No function in this module shall panic under any input. All failures
   are expressed as `Result::Err`.
9. **Post9 -- clean stdio transport**: The `run` function drives rmcp's stdio transport to
   completion and returns cleanly when stdin is closed (EOF).

## Invariants

1. **INV1 -- Zero-panic**: No `.unwrap()`, `.expect()`, or array indexing without bounds check
   anywhere in the new `mcp.rs`. All fallible operations use `Result` propagation or explicit
   error mapping.
2. **INV2 -- Railway-oriented error handling**: Every fallible function returns `Result<T, E>`
   where `E` is `CtdMcpError` or `McpError`. No early returns with `anyhow::bail!` in tool
   handlers; errors are mapped to `CtdMcpError` variants.
3. **INV3 -- No global mutable state**: `CtdMcpServer` owns its `index_dir: PathBuf`. No static
   variables, no `lazy_static!`, no interior mutability beyond what rmcp requires internally.
4. **INV4 -- Idempotent tool calls**: Calling any tool with the same arguments and the same
   underlying INDEX.json produces the same result. Tool handlers have no side effects.
5. **INV5 -- Owned parameters**: All tool parameter structs own their data (String, not &str).
   No borrowed lifetimes in parameter structs.
6. **INV6 -- Blocking isolation**: Calls to the synchronous `search` module functions are wrapped
   in `tokio::task::spawn_blocking` to avoid blocking the async tokio runtime.

## Error Taxonomy

```rust
/// CtdMcpError enumerates all failure modes for the MCP tool handlers.
/// Each variant maps to a meaningful MCP error response.
#[derive(Debug, thiserror::Error)]
pub enum CtdMcpError {
    /// INDEX.json is missing or cannot be read from disk.
    #[error("INDEX.json not found in {path}")]
    IndexNotFound {
        path: String,
    },

    /// INDEX.json exists but cannot be parsed as valid JSON.
    #[error("Failed to parse INDEX.json: {reason}")]
    IndexCorrupted {
        reason: String,
    },

    /// A required tool parameter is missing, empty, or out of range.
    #[error("Invalid input: {detail}")]
    InvalidInput {
        detail: String,
    },

    /// The Tantivy search index could not be opened or rebuilt.
    #[error("Search index error: {reason}")]
    SearchIndexError {
        reason: String,
    },

    /// A Tantivy query could not be parsed or executed.
    #[error("Query error: {reason}")]
    QueryError {
        reason: String,
    },

    /// An I/O error occurred reading a file from the index directory.
    #[error("I/O error: {reason}")]
    IoError {
        reason: String,
    },

    /// Catch-all for unexpected errors not covered above.
    #[error("Internal error: {reason}")]
    Internal {
        reason: String,
    },
}

impl From<CtdMcpError> for rmcp::model::Error {
    fn from(err: CtdMcpError) -> Self {
        // Map to rmcp's error type with code -32603 and the error message.
        rmcp::model::Error {
            code: -32603,
            message: err.to_string().into(),
            data: None,
        }
    }
}
```

### Error Mapping Rules

| Source | CtdMcpError variant | MCP behavior |
|---|---|---|
| `std::fs::read_to_string` fails on INDEX.json | `IndexNotFound` | Tool returns error result |
| `serde_json::from_str` fails on INDEX.json | `IndexCorrupted` | Tool returns error result |
| Empty/whitespace `query`, limit <= 0 or > 100 | `InvalidInput` | Tool returns error result |
| Empty `id` string | `InvalidInput` | Tool returns error result |
| `open_existing_index` / `rebuild_index_from_json` fail | `SearchIndexError` | Tool returns error result |
| `search_index` returns `SearchError::EmptyQuery` | `InvalidInput` | Tool returns error result |
| `search_index` returns `SearchError::QueryParseError(e)` | `QueryError` | Tool returns error result |
| `search_index` returns `SearchError::Other(e)` | `SearchIndexError` | Tool returns error result |
| `std::io::Error` during file reads | `IoError` | Tool returns error result |

## Public API Surface

### Types

```rust
// === Parameter Structs (owned, no lifetimes) ===

/// Parameters for the `search_docs` tool.
#[derive(Debug, Clone, serde::Deserialize, schemars::JsonSchema)]
pub struct SearchDocsParams {
    /// The search query string (BM25 full-text search).
    pub query: String,
    /// Maximum number of results to return. Defaults to 10 if not provided.
    /// Must be > 0 and <= 100.
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_limit() -> u32 {
    10
}

/// Parameters for the `read_chunk` tool.
#[derive(Debug, Clone, serde::Deserialize, schemars::JsonSchema)]
pub struct ReadChunkParams {
    /// The doc_id or chunk_id to read.
    pub id: String,
}

/// Parameters for the `get_related_concepts` tool.
#[derive(Debug, Clone, serde::Deserialize, schemars::JsonSchema)]
pub struct GetRelatedConceptsParams {
    /// The doc_id or chunk_id to look up in the knowledge graph.
    pub id: String,
}

// === Server Struct ===

/// The MCP server for centralized-docs.
///
/// Owns the index directory path and implements rmcp's `ServerHandler` trait.
/// Created via `CtdMcpServer::new(index_dir)`.
pub struct CtdMcpServer {
    /// Absolute path to the directory containing INDEX.json and .tantivy_index/.
    index_dir: PathBuf,
}
```

### Trait Implementation

```rust
// CtdMcpServer implements rmcp's ServerHandler trait.
// The #[tool(boxed)] macro on each method registers it as an MCP tool
// with automatic JSON Schema derived from the parameter struct's schemars derive.

impl rmcp::handler::server::ServerHandler for CtdMcpServer {
    fn info(&self) -> rmcp::model::ServerInfo {
        // Returns server name "ctd-mcp" and version from CARGO_PKG_VERSION.
        // Capabilities: tools only.
    }
}
```

### Tool Methods

```rust
impl CtdMcpServer {
    // --- Tool: search_docs ---

    /// Search indexed documentation using BM25.
    ///
    /// # Preconditions
    /// - P1: INDEX.json exists in self.index_dir
    /// - P3: query is non-empty after trimming
    /// - P4: limit > 0 and limit <= 100
    ///
    /// # Postconditions
    /// - Post1: Returns ranked results on match
    /// - Post2: Returns "No results found." on no match
    /// - Post8: Never panics
    #[tool(boxed)]
    pub async fn search_docs(
        &self,
        params: SearchDocsParams,
    ) -> Result<CallToolResult, McpError>;

    // --- Tool: read_chunk ---

    /// Read the exact content of a document or chunk by ID.
    ///
    /// # Preconditions
    /// - P1: INDEX.json exists in self.index_dir
    /// - P5: id is non-empty after trimming
    ///
    /// # Postconditions
    /// - Post3: Returns chunk content when id matches a chunk_id
    /// - Post4: Returns document summary when id matches a doc_id
    /// - Post5: Returns not-found message when id matches neither
    /// - Post8: Never panics
    #[tool(boxed)]
    pub async fn read_chunk(
        &self,
        params: ReadChunkParams,
    ) -> Result<CallToolResult, McpError>;

    // --- Tool: get_related_concepts ---

    /// Get related concepts from the knowledge graph DAG.
    ///
    /// # Preconditions
    /// - P1: INDEX.json exists in self.index_dir
    /// - P5: id is non-empty after trimming
    ///
    /// # Postconditions
    /// - Post6: Returns all edges referencing id (both directions)
    /// - Post7: Returns empty message when no edges found
    /// - Post8: Never panics
    #[tool(boxed)]
    pub async fn get_related_concepts(
        &self,
        params: GetRelatedConceptsParams,
    ) -> Result<CallToolResult, McpError>;
}
```

### Construction & Entrypoint

```rust
impl CtdMcpServer {
    /// Create a new MCP server pointing at the given index directory.
    ///
    /// # Preconditions
    /// - P2: index_dir must be an existing directory
    ///
    /// # Postconditions
    /// - Returns Ok(CtdMcpServer) with the canonicalized index_dir path
    /// - Returns Err(CtdMcpError::IoError) if the directory does not exist
    /// - INV1: Never panics
    pub fn new(index_dir: PathBuf) -> Result<Self, CtdMcpError>;
}

/// Run the MCP server on stdio transport until EOF.
///
/// This is the top-level entrypoint called from the CLI `ctd mcp serve <dir>`.
///
/// # Preconditions
/// - P2: index_dir is an existing directory
/// - P6: Called within a tokio runtime
///
/// # Postconditions
/// - Post9: Returns Ok(()) when stdin reaches EOF
/// - Returns Err if transport fails
/// - INV1: Never panics
pub async fn run(index_dir: PathBuf) -> Result<(), CtdMcpError>;
```

### Internal Helper Functions

```rust
impl CtdMcpServer {
    /// Read and parse INDEX.json from self.index_dir.
    ///
    /// # Errors
    /// - CtdMcpError::IndexNotFound if file missing
    /// - CtdMcpError::IndexCorrupted if JSON invalid
    fn load_index_json(&self) -> Result<serde_json::Value, CtdMcpError>;

    /// Open or rebuild the Tantivy search index.
    ///
    /// Tries open_existing_index first; falls back to rebuild_index_from_json.
    ///
    /// # Errors
    /// - CtdMcpError::SearchIndexError if both open and rebuild fail
    fn open_or_rebuild_search_index(&self) -> Result<tantivy::Index, CtdMcpError>;

    /// Format search results into the text output expected by MCP clients.
    ///
    /// # Postconditions
    /// - Post1: Results are formatted with rank, category, score, title, path, summary
    /// - Post2: Returns "No results found." for empty input
    fn format_search_results(results: &[SearchResult]) -> String;

    /// Find a chunk by ID in the parsed INDEX.json data.
    ///
    /// Returns the chunk's content string, or None if not found.
    fn find_chunk_content(index_data: &serde_json::Value, id: &str) -> Option<String>;

    /// Find a document by ID in the parsed INDEX.json data.
    ///
    /// Returns the document's summary string, or None if not found.
    fn find_doc_summary(index_data: &serde_json::Value, id: &str) -> Option<String>;

    /// Extract all graph edges referencing the given ID.
    ///
    /// Returns a vector of formatted strings like "- {target} (Relationship: {type})"
    /// or "- {source} (Relationship: {type} - inbound)".
    fn find_related_edges(index_data: &serde_json::Value, id: &str) -> Vec<String>;
}
```

## Non-goals

1. **No streaming transport**: Only stdio transport is supported (no HTTP SSE, no WebSocket).
2. **No authentication**: MCP protocol-level auth is out of scope.
3. **No tool registration changes at runtime**: Tools are statically defined via `#[tool]` macros.
4. **No caching layer**: Each tool call reads INDEX.json and/or Tantivy index fresh.
5. **No modification to the search module**: `search.rs` remains untouched.
6. **No support for MCP resources or prompts**: Only tools are exposed.
7. **No backward compatibility with the hand-rolled JSON-RPC**: The old `run_mcp_serve` function
   is fully replaced, not kept as a fallback.
