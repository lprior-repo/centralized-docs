---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#34-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 139
summary: A bottom value usually indicates an error. Bottom is an instance of every value
---

A bottom value usually indicates an error.
Bottom is an instance of every value.

An atom is any value whose only instances are itself and bottom.
Examples of atoms are 42.0, "hello", true, and null.

A value is concrete if it is either an atom, or a struct whose field values
of regular (non-hidden and non-definition fields) are all concrete, recursively.

CUE’s values also include what we normally think of as types, like string and
float.
It does not distinguish between types and values:
only the relationship of values in the lattice is important.
