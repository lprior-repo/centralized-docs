---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#51-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 130
summary: number, int and float; they are defined types. A decimal floating-point literal always has type float;
---

number, int and float; they are defined types.

A decimal floating-point literal always has type float;
it is not an instance of int even if it is an integral number.

Integer literals are always of type int and don’t match type float.

Numeric literals are exact values of arbitrary precision.
If the operation permits it, numbers should be kept in arbitrary precision.

Implementation restriction: although numeric values have arbitrary precision
in the language, implementations may implement them using an internal
