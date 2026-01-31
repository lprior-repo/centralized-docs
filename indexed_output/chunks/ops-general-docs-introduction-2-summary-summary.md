---
doc_id: ops/general/docs-introduction
chunk_id: ops/general/docs-introduction#2-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 139
summary: Whereas in most languages types and values are strictly distinct,. CUE orders them in a single hierarchy (a lattice, to be precise)
---


Whereas in most languages types and values are strictly distinct,
CUE orders them in a single hierarchy (a lattice, to be precise).
This is a very powerful concept that allows CUE to do
many fancy things.
It also simplifies matters.
For instance, there is no need for generics, and enums, sum types
and null coalescing are all the same thing.

APPLICATIONS

CUE’s design ensures that combining CUE values in any
order always gives the same result
(it is associative, commutative and idempotent).
This makes CUE particularly well-suited for cases where CUE
