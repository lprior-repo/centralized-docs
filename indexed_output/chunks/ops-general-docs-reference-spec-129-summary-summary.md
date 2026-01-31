---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#129-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 134
summary: value: 5 & matchN(>=1, [int, >10])  // true: int matches. // Exactly 0 schemas must match (none should match)
---

value: 5 & matchN(>=1, [int, >10])  // true: int matches

// Exactly 0 schemas must match (none should match)
value: "test" & matchN(0, [int, >100])  // true: neither matches

If the numeric constraint cannot be satisfied even with incomplete information,
the error is marked as incomplete and will be reevaluated as more information
becomes available.

MATCHIF

The matchIf builtin is a conditional validator that applies different schema
constraints based on whether an initial condition is satisfied.

matchIf takes three arguments:
