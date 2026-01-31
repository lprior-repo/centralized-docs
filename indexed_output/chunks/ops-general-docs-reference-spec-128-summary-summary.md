---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#128-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 134
summary:  * a numeric constraint specifying how many schemas must match,.  * a list of schemas to test against the value
---


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
