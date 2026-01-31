---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#54-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 133
summary: The number of bytes is called the length of the byte sequence. and is never negative
---

The number of bytes is called the length of the byte sequence
and is never negative.
The predeclared byte sequence type is bytes; it is a defined type.

BOUNDS

A bound, syntactically a unary expression [/docs/reference/spec/#operands], defines
a logically infinite disjunction of concrete values represented as a single comparison.
For example, >= 2 represents the infinite disjunction 2|3|4|5|6|7|….

For any comparison operator [/docs/reference/spec/#comparison-operators] op,
op a is the disjunction of every x such that x op a.
