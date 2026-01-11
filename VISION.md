# Project Vision: Centralized Documentation Transformer

## Purpose

The Centralized Documentation Transformer is a Rust-based CLI tool that transforms raw documentation into AI-optimized knowledge structures. It addresses the fundamental problem that modern documentation systems face: **documentation is written for humans to read linearly, but AI systems and search engines need structured, interconnected knowledge graphs to provide meaningful assistance.**

This project bridges that gap by analyzing, chunking, and indexing documentation in a way that preserves semantic meaning, creates explicit relationships, and enables both human and AI consumers to efficiently navigate and understand complex documentation landscapes.

## Problems We Solve

### 1. **AI Systems Can't Navigate Unstructured Docs**
- **Problem**: Large language models and AI assistants struggle with raw markdown files. They lack context about document relationships, hierarchies, and cross-references.
- **Solution**: We build a knowledge graph (directed acyclic graph) that explicitly maps relationships between documents and chunks, enabling AI systems to understand "what relates to what" and "what should be read first."

### 2. **Documentation Lacks Discoverability**
- **Problem**: Users don't know what documentation exists, where to start, or how pieces connect.
- **Solution**: We generate a COMPASS.md file that provides a high-level navigation map, categorizing documents by type (tutorial, reference, concept, operations) and showing entry points.

### 3. **Context Windows Are Limited**
- **Problem**: AI systems have token limits. Sending entire documentation sets is inefficient and often impossible.
- **Solution**: We chunk documents semantically (targeting ~170 tokens per chunk) with contextual prefixes, allowing precise retrieval of relevant sections rather than entire documents.

### 4. **Documentation Quality Is Inconsistent**
- **Problem**: Documentation accumulates over time with varying quality, broken links, missing metadata, and structural inconsistencies.
- **Solution**: We validate every document against strict rules (proper headings, link integrity, metadata completeness) and report actionable feedback.

### 5. **Search Is Token-Based, Not Semantic**
- **Problem**: Traditional grep-style search matches strings, not meaning. Searching for "authentication" won't find "login" or "user verification."
- **Solution**: We provide BM25-based search across both documents and chunks, enabling ranking by relevance rather than simple pattern matching.

### 6. **Updates Are All-or-Nothing**
- **Problem**: Re-indexing large documentation sets is slow. Most changes only affect a few files.
- **Solution**: We implement incremental indexing that tracks file hashes and only reprocesses changed documents, dramatically reducing iteration time.

## Target Users

### 1. **AI Coding Assistants (Primary)**
- **Use Case**: Claude Code, Cursor, Aider, and similar tools need structured context to answer questions about frameworks, libraries, or internal systems.
- **Value**: Instead of sending raw markdown, these tools can query our INDEX.json for relevant chunks, explore relationships via the knowledge graph, and provide precise, context-aware answers.

### 2. **Documentation Engineers**
- **Use Case**: Teams maintaining large documentation sets (framework docs, API references, internal wikis) need quality assurance and structural insights.
- **Value**: Automated validation catches errors, the knowledge graph reveals gaps in coverage, and the categorical organization exposes structural weaknesses.

### 3. **Developer Experience Teams**
- **Use Case**: Teams building onboarding systems, internal developer portals, or knowledge bases need optimized content for AI-powered Q&A systems.
- **Value**: Output is ready for embedding into vector databases, semantic search engines, or RAG (Retrieval-Augmented Generation) systems.

### 4. **Personal Knowledge Management**
- **Use Case**: Individuals maintaining personal notes, research collections, or technical journals want better organization and retrieval.
- **Value**: Transform scattered markdown files into a queryable, navigable knowledge base with minimal manual curation.

## Design Philosophy

### Core Principles

#### 1. **Single Source of Truth**
- Documentation lives in version control as markdown.
- Transformation is deterministic and repeatable.
- The INDEX.json becomes the canonical representation of documentation structure.

#### 2. **AI-First, Human-Compatible**
- Primary consumer is AI systems (structured JSON, explicit relationships).
- Secondary consumer is humans (COMPASS.md, readable chunks, validated links).
- Both must be equally well-served.

#### 3. **Semantic Over Syntactic**
- We chunk by meaning (headings, sections) not by character count.
- We detect categories by analyzing content, not just filenames.
- We rank by relevance (BM25) not alphabetically.

#### 4. **Incremental Over Full**
- Most operations should be incremental by default.
- Full re-indexing should be explicit (--force flag).
- Changelogs and deltas should be visible to users.

#### 5. **Validation Is Non-Negotiable**
- Every document must pass structural validation.
- Every internal link must resolve.
- Warnings are informational; errors block production use.

#### 6. **Composition Over Complexity**
- Pipeline is linear: Discover → Analyze → Assign → Transform → Chunk → Index → Validate.
- Each stage has a single responsibility.
- Errors in one stage don't cascade to others.

### Architectural Commitments

#### 1. **Pure Functional Pipeline**
- Each stage takes immutable input and produces new output.
- No shared mutable state between stages.
- Parallelization is trivial because stages are independent.

#### 2. **Strong Typing**
- Rust's type system enforces contracts at compile time.
- Serialization/deserialization is explicit (serde).
- Invalid states are unrepresentable (e.g., a chunk without a parent document).

#### 3. **Error Handling Without Panics**
- All operations return `Result<T, Error>`.
- Regex patterns are validated at compile time via `once_cell::sync::Lazy`.
- File I/O failures are recoverable (skip file, log error, continue).

#### 4. **Graph Theory for Relationships**
- Documentation is a directed acyclic graph (DAG).
- Edges have types (SemanticLink, ParentChild, Prerequisite).
- Cycles are detected and rejected.

#### 5. **Observability Through Output**
- Every stage prints progress (DISCOVER, ANALYZE, etc.).
- Verbose mode shows validation issues per file.
- Final summary provides complete operation statistics.

## Core Features

### 1. **Document Discovery**
- Walks directory trees to find markdown files.
- Filters by extension (configurable).
- Produces a manifest of all discovered files.

### 2. **Content Analysis**
- Extracts title (H1 or filename fallback).
- Parses frontmatter (YAML between `---` delimiters).
- Identifies headings, links, code blocks, tables.
- Detects category (tutorial, reference, concept, operations, meta).

### 3. **ID Assignment**
- Generates stable IDs from category, audience, and slugified title.
- Example: `tutorial-general-getting-started`
- Creates link map for resolving cross-references.

### 4. **Transformation**
- Rewrites documents with frontmatter (id, title, tags).
- Resolves internal links to stable IDs.
- Preserves original content structure.
- Outputs to `docs/` directory.

### 5. **Semantic Chunking**
- Splits documents by headings (not character count).
- Each chunk gets contextual prefix (document title, breadcrumb).
- Targets ~170 tokens per chunk.
- Outputs to `chunks/` directory.
- Example: `tutorial-general-getting-started-0.md`

### 6. **Knowledge Graph Construction**
- Nodes represent documents and chunks.
- Edges represent relationships:
  - **SemanticLink**: Explicit markdown links between documents.
  - **ParentChild**: Document-to-chunk containment.
  - **Prerequisite**: Implied reading order (tutorial → reference).
- Edges have weights (link distance, structural importance).

### 7. **Index Generation**
- Produces `INDEX.json` with:
  - Documents array (metadata, tags, word count).
  - Chunks array (content, parent document, heading context).
  - Graph (nodes, edges with types and weights).
- Produces `COMPASS.md` with categorized document list.

### 8. **Validation**
- Structural: proper H1, headings hierarchy, frontmatter.
- Content: no empty sections, reasonable word count.
- Links: internal links resolve, no empty targets.
- Outputs warnings (suggestions) and errors (blockers).

### 9. **Search**
- BM25 ranking across documents or chunks.
- Query against title, summary, content, tags.
- Returns top N results with scores.
- Snippet extraction for preview.

### 10. **Incremental Indexing**
- Tracks file hashes in `.incremental_state.json`.
- Compares current files against previous state.
- Reports added/modified/unchanged/deleted counts.
- Only reprocesses changed files.

## Success Criteria

### Measurable Checkpoints

1. **Zero Panics in Production**
   - All regex patterns validated at compile time.
   - All file I/O wrapped in Result types.
   - No `.expect()` calls on dynamic data.

2. **Incremental Mode Reduces Indexing Time by >80%**
   - For a 100-file documentation set with 5 changes, processing time should be <20% of full indexing.

3. **Validation Catches 100% of Broken Internal Links**
   - No false negatives (broken links reported as valid).
   - No false positives (valid links reported as broken).

4. **Knowledge Graph is Acyclic**
   - No document can be its own ancestor.
   - Graph construction fails fast on cycle detection.

5. **Chunks Are Semantically Coherent**
   - 95%+ of chunks should have a clear heading.
   - Chunks should not split mid-sentence.

6. **Search Ranks by Relevance, Not Alphabetically**
   - BM25 scoring should prioritize documents with higher term frequency.
   - Manual testing: "authentication" should rank auth docs above unrelated docs mentioning the word once.

7. **COMPASS.md Is Human-Navigable**
   - Grouped by category (TUTORIAL, REFERENCE, etc.).
   - Shows document titles and key tags.
   - No more than 50 documents per category before introducing subcategories.

8. **INDEX.json Is Machine-Parseable**
   - Valid JSON schema.
   - All fields present (no optional fields that break parsers).
   - Deserializable by serde without errors.

9. **Pipeline Completes in <10 Seconds for 100 Documents**
   - Full indexing on modest hardware (4-core CPU, SSD).
   - No network I/O in critical path.

10. **Documentation Coverage Reaches 90%+**
    - Every public module has doc comments.
    - Every CLI subcommand has help text.
    - Every data structure in INDEX.json is documented.

## Implementation Examples

### Use Case 1: AI Assistant Integration

**Scenario**: A developer asks Claude Code, "How do I authenticate users?"

**Without This Tool**:
- AI receives entire documentation set (100+ files, 50,000+ tokens).
- Exceeds context window or wastes tokens on irrelevant content.
- Response is generic or incomplete.

**With This Tool**:
1. AI queries INDEX.json: `search_documents("authentication")`
2. Returns ranked results: `tutorial-auth-jwt`, `ref-auth-api`, `ops-auth-troubleshooting`
3. AI retrieves top 3 chunks related to authentication.
4. AI provides precise answer with correct references.

### Use Case 2: Documentation Quality Audit

**Scenario**: A team merges 10 PRs, each adding documentation. CI runs validation.

**Without This Tool**:
- Manual review catches some issues.
- Broken links go unnoticed until users complain.
- Inconsistent structure (some use H2 for subtitles, some use H3).

**With This Tool**:
1. CI runs: `doc-transformer transform docs/ output/ --verbose`
2. Validation fails with actionable errors:
   - `tutorial-setup.md: Missing H1 heading (V001)`
   - `reference-api.md: Broken link to ./endpoints.md (L001)`
3. PR blocked until issues resolved.

### Use Case 3: Personal Knowledge Base

**Scenario**: A researcher maintains 500+ markdown notes on machine learning.

**Without This Tool**:
- Files scattered across directories.
- No way to find related notes.
- Search is grep (string matching, not relevance).

**With This Tool**:
1. Run: `doc-transformer transform notes/ output/`
2. Open `output/COMPASS.md` to see categorized overview.
3. Query: `doc-transformer search "gradient descent" -i output/ -n 5`
4. Returns 5 most relevant notes, ranked by BM25.
5. Explore relationships: `doc-transformer graph tutorial-gradient-descent-0 -i output/ --reachable`
6. See all notes reachable from this chunk (prerequisites, related concepts).

### Use Case 4: Team Onboarding

**Scenario**: A new engineer joins and needs to learn the system.

**Without This Tool**:
- Given a link to a docs folder.
- No clear starting point.
- Reads documents in random order, misses prerequisites.

**With This Tool**:
1. Open `output/COMPASS.md`.
2. See categorized list: TUTORIAL section has "Getting Started" as first entry.
3. Read tutorial, which has prerequisite links to concepts.
4. Knowledge graph ensures linear progression (TUTORIAL → CONCEPT → REFERENCE → OPS).

## Future Directions

### Roadmap (Not Committed)

1. **Advanced Search**
   - Vector embeddings for semantic search (not just BM25).
   - Query expansion (synonyms, related terms).
   - Faceted filtering (by category, tags, word count).

2. **Web UI**
   - Local server mode: `doc-transformer serve --port 8080`
   - Interactive graph visualization (D3.js or similar).
   - Real-time search with highlighting.

3. **Plugin System**
   - Custom analyzers (detect code smells in examples).
   - Custom validators (team-specific rules).
   - Custom chunking strategies (e.g., by function definitions).

4. **Multi-Format Support**
   - Input: AsciiDoc, reStructuredText, Org-mode.
   - Output: HTML, PDF, EPUB (via pandoc).

5. **Collaboration Features**
   - Diff mode: compare two indexes, show what changed.
   - Merge mode: combine multiple documentation sources.
   - Conflict resolution for duplicate IDs.

6. **Performance Optimizations**
   - Parallel processing (rayon for CPU-bound work).
   - Streaming JSON parsing (avoid loading entire index into memory).
   - Persistent caching (Redis or similar for repeated queries).

7. **AI-Specific Enhancements**
   - Pre-computed embeddings stored in INDEX.json.
   - Tokenization metadata (exact token counts per chunk).
   - Prompt templates for common queries.

## Relationship to Other Documents

- **ARCHITECTURE.md** (to be created): Details the Rust module structure, data flow, and component interactions. Explains hexagonal architecture if applicable, or clarifies the current linear pipeline approach.

- **CLAUDE.md** (exists): Defines TCR (Test-Commit-Revert) rules for development workflow. Specifies that tests must pass before changes are committed, and enforces Go patterns (though this project is Rust, the TCR philosophy applies).

- **AGENTS.md** (to be created): AI agent coding standards, constraints, and implementation patterns specific to this project. Covers Rust-specific idioms, error handling conventions, and testing requirements.

## Constraints and Non-Goals

### What We Won't Do

1. **Host Documentation**: We transform and index; hosting is external (GitHub Pages, Netlify, etc.).
2. **Edit Documentation**: We don't rewrite content for clarity or correctness; we only restructure.
3. **Replace Git**: Documentation still lives in version control; we augment, not replace.
4. **Guarantee AI Accuracy**: We provide structure; AI systems are responsible for correct interpretation.
5. **Support Real-Time Collaboration**: This is a batch CLI tool, not a live editing environment.

### Technical Constraints

1. **Rust Only**: No polyglot codebases. Dependencies must be Rust crates.
2. **No Network I/O in Core Pipeline**: Validation may check URL schemes, but no HTTP requests during indexing.
3. **No Database**: State is file-based (JSON, markdown). No SQLite, Postgres, etc.
4. **CLI-First**: No mandatory GUI. Web UI (if added) is optional.
5. **Markdown-Centric**: Primary input format is markdown. Other formats require conversion first.

## Versioning and Stability

### Semantic Versioning
- **v0.x.x**: Experimental. INDEX.json schema may change.
- **v1.x.x**: Stable. INDEX.json schema is frozen; breaking changes require v2.
- **v2.x.x**: Major evolution. New graph types, new chunking strategies, etc.

### Backwards Compatibility
- INDEX.json v1.0 consumers must work with v1.x outputs indefinitely.
- Command-line interface is stable after v1.0.
- Internal module structure may change; public API (CLI, INDEX.json) won't.

---

## Conclusion

This project exists to make documentation **computable**. We transform static text into a dynamic knowledge graph that AI systems can traverse, query, and reason about. We don't just index documents; we reveal their structure, validate their integrity, and optimize their consumability.

If documentation is knowledge, this tool is the librarian—categorizing, cross-referencing, and ensuring that every piece is exactly where it needs to be when it's needed.

**Start here. Read the ARCHITECTURE.md next to understand how we implement these principles. Then dive into the code.**
