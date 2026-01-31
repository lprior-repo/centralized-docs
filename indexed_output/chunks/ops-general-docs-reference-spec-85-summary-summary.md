---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#85-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 139
summary: be derived from those of structs. Implementations are not required to implement lists as structs
---

be derived from those of structs.
Implementations are not required to implement lists as structs.
The Elem and Tail fields are not special and len will not work as
expected in these cases.

DECLARATIONS AND SCOPES

BLOCKS

A block is a possibly empty sequence of declarations.
The braces of a struct literal { ... } form a block, but there are
others as well:

 * The universe block encompasses all CUE source text.
 * Each package [/docs/reference/spec/#modules-instances-and-packages] has a package block
   containing all CUE source text in that package.
