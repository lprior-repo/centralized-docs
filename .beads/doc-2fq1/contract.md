# Contract: doc-2fq1 - scrape-filter: query threshold is ignored in scrape and ingest

## Metadata
- bead_id: doc-2fq1
- bead_title: scrape-filter: query threshold is ignored in scrape and ingest
- phase: p0
- updated_at: 2026-03-01T01:20:00Z

## Problem Statement
When --query with --threshold is provided to scrape or ingest, the threshold is ignored and all pages are kept.

## EARS Requirements
- **Ubiquitous**: THE SYSTEM SHALL apply query filtering semantics consistently for scrape and ingest.
- **Event-driven**: WHEN a user passes --query with --threshold, THE SYSTEM SHALL drop pages scoring below threshold.
- **Unwanted**: IF pages do not match the query, THE SYSTEM SHALL NOT report them as kept matches.

## Preconditions
- auth_required: false
- system_state: Query and threshold inputs are validated.

## Postconditions
- state_changes: Result pages satisfy score >= threshold when query is provided.

## Invariants
- Reported kept count equals number of retained pages after filter logic.

## Research Requirements
- Read: doc_transformer/src/main.rs, doc_transformer/src/filter.rs

## Anti-Hallucination
- READ files before modifying them
- Use functional patterns
- Return Result<T, Error> throughout
- NEVER use .unwrap() or .expect()
