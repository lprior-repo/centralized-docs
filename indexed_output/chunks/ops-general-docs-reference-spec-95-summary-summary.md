---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#95-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 139
summary: as described in default values [/docs/reference/spec/#default-values], the field binds to this. value-default pair
---

as described in default values [/docs/reference/spec/#default-values], the field binds to this
value-default pair.

LET DECLARATIONS

Within a struct, a let clause binds an identifier to the given expression.

Within the scope of the identifier, the identifier refers to the
locally declared expression.
The expression is evaluated in the scope it was declared.

EXPRESSIONS

An expression specifies the computation of a value by applying operators and
builtin functions to operands.

Expressions that require concrete values are called incomplete if any of
