# Contract Specification

## Context
- Feature: Replace custom O(N^2) tokenizing and chunking in `contextual-chunker` with `text-splitter::MarkdownSplitter`. Delete `pseudo_bm25` and use a persistent `tantivy` index to calculate true BM25 scores in `doc_transformer`.
- Domain terms: Chunking, MarkdownSplitter, BM25, Tantivy Index, Persistent Index.
- Assumptions: The `tantivy` index is persisted on disk. The document text is valid Markdown.
- Open questions: Where is the default path for the persistent tantivy index? What is the maximum chunk size limit configured for the MarkdownSplitter?

## Preconditions
- [P1] `MarkdownSplitter` must be initialized with a valid `chunk_capacity` (> 0).
- [P2] The `tantivy` index directory must be accessible and writable if building, or readable if searching.
- [P3] Documents passed to the BM25 indexer must have valid, non-empty text.
- [P4] The search query for BM25 must be a valid, non-empty string.

## Postconditions
- [Q1] `contextual-chunker` outputs a list of chunks where no chunk exceeds the maximum chunk capacity (unless a single token/word is strictly larger than capacity).
- [Q2] `doc_transformer` returns documents sorted by their true BM25 score in strictly descending order.
- [Q3] `tantivy` index writer must successfully commit the added documents before they can be searched.

## Invariants
- [I1] The number of chunks generated from a document is >= 1 if the input document text is not empty.
- [I2] BM25 scores returned from the search are always non-negative (>= 0.0).

## Error Taxonomy
- `ChunkerError::InvalidChunkCapacity` - when chunk capacity is 0.
- `IndexerError::DirectoryAccessFailed` - when the tantivy index directory cannot be read or written.
- `IndexerError::IndexCommitFailed` - when tantivy fails to commit added documents to disk.
- `SearchError::EmptyQuery` - when the provided search query is empty.
- `SearchError::QueryParseError` - when the query cannot be parsed by tantivy's query parser.

## Contract Signatures
- `fn chunk_markdown(text: &str, capacity: ChunkCapacity) -> Result<Vec<String>, ChunkerError>`
- `fn open_or_create_index(index_path: &Path) -> Result<Index, IndexerError>`
- `fn index_documents(index: &mut IndexWriter, docs: &[Document]) -> Result<(), IndexerError>`
- `fn score_bm25(index: &Index, query: &QueryStr, top_k: usize) -> Result<Vec<ScoredDocument>, SearchError>`

## Type Encoding
For each precondition, specify the strongest possible type enforcement:
| Precondition | Enforcement Level | Type / Pattern |
|---|---|---|
| chunk_capacity > 0 | Compile-time | `NonZeroUsize` |
| valid index path | Runtime-checked | `Result<Index, IndexerError>` |
| non-empty query | Compile-time | `NonEmptyString` |
| non-empty document | Compile-time | `NonEmptyString` |

## Violation Examples (REQUIRED -- one per precondition and postcondition)
- VIOLATES P1: `ChunkCapacity::new(0)` -- should produce `Err(ChunkerError::InvalidChunkCapacity)`.
- VIOLATES P2: `open_or_create_index(Path::new("/root/secret/index"))` -- should produce `Err(IndexerError::DirectoryAccessFailed)`.
- VIOLATES P3: `Document::new("")` -- should produce `Err(IndexerError::InvalidDocument)` (or fail via `NonEmptyString` instantiation).
- VIOLATES P4: `QueryStr::new("")` -- should produce `Err(SearchError::EmptyQuery)`.
- VIOLATES Q1: `chunk_markdown` returns a chunk of size 1500 when capacity is 1000 -- should produce `Err(ChunkerError::PostconditionViolated)`.
- VIOLATES Q2: `score_bm25` returns documents out of order (e.g., score 1.2 then score 1.5) -- should produce `Err(SearchError::PostconditionViolated)`.
- VIOLATES Q3: `score_bm25` is called before `index_documents` commits, resulting in missing documents -- should produce `Err(IndexerError::UncommittedChanges)`.

## Ownership Contracts (Rust-specific)
- `fn chunk_markdown(text: &str, capacity: ChunkCapacity)` -- borrows text read-only. Returns owned `Vec<String>`.
- `fn open_or_create_index(index_path: &Path)` -- borrows index path read-only. Returns an owned `Index`.
- `fn index_documents(writer: &mut IndexWriter, docs: &[Document])` -- Exclusive borrow on `IndexWriter`. Mutates writer state by adding documents.
- `fn score_bm25(index: &Index, query: &QueryStr, top_k: usize)` -- Shared borrow on `Index` and `QueryStr`. Read-only operation.

## Non-goals
- Implementing a custom query parser (relying entirely on tantivy's query parser).
- Supporting text splitting for languages other than Markdown at this stage.
