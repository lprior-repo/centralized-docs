# Type Contract - PR #10

## Scope
- PR: `#10` (`fix/red-queen-bugs`)
- Rust components: `doc_transformer` and `contextual-chunker`
- Issues covered: `doc-nnly`, `doc-1vtz`, `doc-13x9`, `doc-2mzo`, `doc-sgzo`, `doc-3e4v`
- Review feedback covered: `discussion_r2866841648` (IPv6 host literal regression risk)

## Contracted Components and Functions
1. `doc_transformer/src/validate.rs`
   - `validate_query(query: &str) -> Result<&str, ValidationError>`
2. `doc_transformer/src/main.rs`
   - `run_search(query: &str, index_dir: &Path, limit: usize, _use_color: bool) -> Result<()>`
3. `doc_transformer/src/scrape/validation.rs`
   - `validate_url(url: &str) -> Result<url::Url>`
   - `find_unencoded_special_char(url: &str) -> Option<char>`
4. `doc_transformer/src/discover.rs`
   - `discover_files(source_dir: &Path) -> Result<(Vec<DiscoveryFile>, DiscoverManifest)>`
   - `discover_single_file(file_path: &Path, extensions: &[&str]) -> Result<(Vec<DiscoveryFile>, DiscoverManifest)>`
5. `doc_transformer/src/filter.rs`
   - `discover_test_files(root: &Path) -> Result<Vec<String>, anyhow::Error>`
6. `contextual-chunker/src/chunk.rs`
   - `estimate_tokens(text: &str) -> usize`

## Domain Types (Make Illegal States Unrepresentable)

### Search Query Domain
```rust
pub struct SearchQuery(String);

impl SearchQuery {
    pub fn parse(raw: &str) -> Result<Self, SearchContractError>;
    pub fn as_str(&self) -> &str;
}

pub struct QueryWordCount(u16); // invariant: <= 100
```

Compile-time/rich-type guidance:
- Do not pass raw `&str` after boundary parsing.
- Parse once at CLI boundary into `SearchQuery`; all internal search APIs accept `&SearchQuery`.

Invariants:
- No null byte.
- Trimmed content non-empty.
- Byte length <= configured max.
- Word count <= 100.

### URL Validation Domain
```rust
pub struct RawUrlInput(String);
pub struct HttpUrl(url::Url); // invariant: scheme is http/https and host present

pub enum HostKind {
    Domain,
    Localhost,
    Ipv4,
    Ipv6Literal,
}
```

Guidance:
- Parse `RawUrlInput` into `HttpUrl` at boundary.
- Special-character policy is path/query/fragment aware.
- `[` and `]` are only legal unencoded in authority as IPv6 host delimiters.

Invariants:
- Scheme in `{http, https}`.
- Host is present and non-empty.
- Serialization roundtrip remains parseable.

### Discovery Domain
```rust
pub enum SourceDocKind {
    Markdown,
    MarkdownX,
    MarkdownLong,
    MarkdownDown,
    MarkdownK,
    ReStructuredText,
    PlainText,
}

pub struct SupportedExtension(SourceDocKind);

pub enum DiscoveryNotice {
    NonMarkdownParsedAsMarkdown { path: String, extension: String },
}
```

Guidance:
- Convert extension string to `SourceDocKind` before branching logic.
- Unsupported extension should not enter discovery output collection.

Invariants:
- Only supported extensions appear in manifest file list.
- `.txt` always emits a warning notice.

### Search Exit Semantics Domain
```rust
pub enum SearchOutcome {
    AdvancedSuccess,
    FallbackSuccessAfterAdvancedFailure,
    CompleteFailure,
}

pub struct ExitCode(u8);
```

Guidance:
- Represent partial success explicitly (`FallbackSuccessAfterAdvancedFailure`).
- Exit-code mapping is a pure function from `SearchOutcome`.

Invariants:
- Partial success never maps to zero exit status.

### Chunking Tokenizer Domain
```rust
pub enum TokenEstimator {
    TikTokenCl100k,
    CharApproximation,
}
```

Invariants:
- Estimation must terminate in bounded time.
- Failure to initialize tokenizer falls back without panic.

## Error Taxonomy (Exhaustive)

```rust
#[derive(Debug, thiserror::Error)]
pub enum SearchContractError {
    #[error("query cannot be empty")]
    EmptyQuery,
    #[error("query contains null byte")]
    NullByteInQuery,
    #[error("query too long: {length} bytes (max {max})")]
    QueryTooLong { length: usize, max: usize },
    #[error("query has too many terms: {count} (max {max})")]
    QueryTooManyTerms { count: usize, max: usize },
    #[error("regex syntax not allowed in advanced mode")]
    RegexNotAllowed,
    #[error("advanced search failed: {reason}")]
    AdvancedSearchFailed { reason: String },
    #[error("advanced search failed; fallback succeeded")]
    PartialSearchFailure,
    #[error("index metadata missing at {path}")]
    IndexMissing { path: String },
    #[error("index document malformed: {reason}")]
    InvalidIndexJson { reason: String },
}

#[derive(Debug, thiserror::Error)]
pub enum UrlValidationError {
    #[error("url cannot be empty")]
    EmptyUrl,
    #[error("url contains spaces")]
    SpaceInUrl,
    #[error("url contains unencoded special character: {ch}")]
    UnencodedSpecialCharacter { ch: char },
    #[error("invalid url format")]
    InvalidUrlFormat,
    #[error("invalid url scheme: {scheme}")]
    UnsupportedScheme { scheme: String },
    #[error("url host is missing or empty")]
    MissingHost,
    #[error("url serialization roundtrip failed")]
    InvalidUrlEncoding,
}

#[derive(Debug, thiserror::Error)]
pub enum DiscoveryError {
    #[error("source path not found: {path}")]
    SourceNotFound { path: String },
    #[error("canonicalization failed: {path}")]
    CanonicalizationFailed { path: String },
    #[error("invalid file path: {path}")]
    InvalidFilePath { path: String },
    #[error("metadata read failed: {path}")]
    MetadataReadFailed { path: String },
}

#[derive(Debug, thiserror::Error)]
pub enum ChunkingError {
    #[error("tokenizer initialization failed")]
    TokenizerInitializationFailed,
}
```

## Preconditions and Postconditions

### `validate_query`
- Preconditions:
  - Raw query is UTF-8 string from CLI boundary.
- Postconditions:
  - `Ok(trimmed)` only when query satisfies all invariants.
  - `Err(NullBytesNotAllowed)` for any `\0` byte occurrence.

### `run_search`
- Preconditions:
  - Query already validated.
  - `INDEX.json` exists and is parseable JSON.
- Postconditions:
  - If advanced mode fails and fallback succeeds, function returns error (non-zero process exit).
  - Output always discloses fallback when used.

### `validate_url`
- Preconditions:
  - Input string provided by caller.
- Postconditions:
  - `Ok(url)` only for `http|https` with non-empty host.
  - IPv6 bracket host literals remain valid (`https://[::1]:3000/docs`).
  - Unencoded special chars outside IPv6 authority context are rejected.

### `discover_files` / `discover_single_file`
- Preconditions:
  - Source path exists.
- Postconditions:
  - Manifest includes only supported extensions.
  - `.markdown`, `.mdown`, `.mkd` are discoverable.
  - `.txt` files are included with warning.

### `estimate_tokens`
- Preconditions:
  - Input string may be any UTF-8 text.
- Postconditions:
  - Returns deterministic token estimate.
  - Never hangs due to per-call tokenizer reconstruction.

## Violation Examples (Required)
- `validate_query("test\0query")` -> `Err(NullBytesNotAllowed)`
- `run_search("<script>alert(1)</script>", idx, 10, false)` with advanced parse failure + fallback hits -> `Err(PartialSearchFailure)`
- `validate_url("https://example.com/foo bar")` -> `Err(SpaceInUrl)`
- `validate_url("https://example.com/foo[bar]")` -> `Err(UnencodedSpecialCharacter { ch: '[' })`
- `validate_url("https://[::1]:3000/docs")` -> `Ok(HttpUrl)`
- `discover_files(path_with_only_markdown_variants)` finds `.markdown|.mdown|.mkd` entries
- `discover_files(path_with_txt)` emits `DiscoveryNotice::NonMarkdownParsedAsMarkdown`
