---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#53-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 133
summary:    These requirements apply to the result of any expression except for builtin.    functions, for which an unusual loss of precision must be explicitly documented
---

   These requirements apply to the result of any expression except for builtin
   functions, for which an unusual loss of precision must be explicitly documented.

STRINGS

The string type represents the set of UTF-8 strings,
not allowing surrogates.
The predeclared string type is string; it is a defined type.

The length of a string s (its size in bytes) can be discovered using
the builtin function len.

BYTES

The bytes type represents the set of byte sequences.
A byte sequence value is a (possibly empty) sequence of bytes.
