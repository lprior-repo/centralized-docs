---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#87-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 129
summary: resulting in a field with a value that is the result of unifying the values. of all fields with the same identifier
---

resulting in a field with a value that is the result of unifying the values
of all fields with the same identifier.
String labels do not bind an identifier to the respective field.

The scope of a declared identifier is the extent of source text in which the
identifier denotes the specified field, alias, or package.

CUE is lexically scoped using blocks:

 1. The scope of a predeclared identifier [/docs/reference/spec/#predeclared-identifiers] is the universe block.
 2. The scope of an identifier denoting a field
