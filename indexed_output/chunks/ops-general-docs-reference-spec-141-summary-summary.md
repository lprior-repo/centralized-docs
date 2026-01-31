---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#141-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 139
summary: followed by a possibly empty set of import declarations that declare. packages whose contents it wishes to use, followed by a possibly empty set of
---

followed by a possibly empty set of import declarations that declare
packages whose contents it wishes to use, followed by a possibly empty set of
declarations.

Like with a struct, a source file may contain embeddings.
Unlike with a struct, the embedded expressions may be any value.
If the result of the unification of all embedded values is not a struct,
it will be output instead of its enclosing file when exporting CUE
to a data format


Copy code
Copied!

SourceFile = { attribute "," } [ PackageClause "," ] { ImportDecl "," } { Declaration "," } .
