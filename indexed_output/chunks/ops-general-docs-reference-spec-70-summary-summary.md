---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#70-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 129
summary: or the pattern of a pattern constraint defined in c. Hidden fields are excluded from this limitation
---

or the pattern of a pattern constraint defined in c.
Hidden fields are excluded from this limitation.
A struct that is the result of unifying any struct with a ... [/docs/reference/spec/#structs]
declaration is defined for all regular fields.
Closing a struct is equivalent to adding ..._|_ to it.

Syntactically, structs are closed explicitly with the close builtin or
implicitly and recursively by definitions [/docs/reference/spec/#definitions-and-hidden-fields].


Copy code
Copied!

A: close({
    field1: string
