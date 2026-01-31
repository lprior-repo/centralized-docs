---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#86-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 136
summary:  * Each file has a file block containing all CUE source text in that file.  * Each for and let clause in a comprehension [/docs/reference/spec/#comprehensions]
---

 * Each file has a file block containing all CUE source text in that file.
 * Each for and let clause in a comprehension [/docs/reference/spec/#comprehensions]
   is considered to be its own implicit block.

Blocks nest and influence scoping.

DECLARATIONS AND SCOPE

A declaration may bind an identifier to a field, alias, or package.
Every identifier in a program must be declared.
Other than for fields,
no identifier may be declared twice within the same block.
For fields, an identifier may be declared more than once within the same block,
