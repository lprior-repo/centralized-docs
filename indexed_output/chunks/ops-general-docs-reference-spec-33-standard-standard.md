---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#33-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 521
summary: A successful validation yields the original value;. a failed validation yields an error
---

A successful validation yields the original value;
a failed validation yields an error.

Bounds (<10) are a type of validator.

Functions that return a boolean value can be used as validators by omitting
their first argument.

The remainder of this section defines builtin validators. These can only be
used as validators, so we will not refer to their function equivalents.

These builtins refer to finalized values, which means that the value being
validated is fully resolved, and defaults taken, before it is unified with the
schema.

MATCHN

The matchN builtin is a validator that checks if a specified number of schemas
from a given list unify successfully with the finalized value being validated.

matchN takes two arguments:

 * a numeric constraint specifying how many schemas must match,
 * a list of schemas to test against the value.

The validator evaluates each schema in the list by unifying it with the value.
It counts how many schemas unify successfully (without producing an error).
The validator succeeds if the count satisfies the numeric constraint provided
as the first argument.


Copy code
Copied!

// Exactly 2 schemas must match
value: "foo" & matchN(2, [string, !="bar", <4])  // true: string and !="bar" match

// At least 1 schema must match
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

 * a condition schema (the “if” clause),
 * the schema to apply if the condition matches (the “then” clause),
 * the schema to apply if the condition does not match (the “else” clause).

The validator first attempts to unify the finalized value with the condition
