---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#116-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 130
summary: The for clause binds the defined identifiers, on each iteration, to the next. value of some iterable value in a new scope
---


The for clause binds the defined identifiers, on each iteration, to the next
value of some iterable value in a new scope.
A for clause may bind one or two identifiers.
If there is one identifier, it binds it to the value of
a list element or struct field value.
If there are two identifiers, the first value will be the key or index,
if available, and the second will be the value.

For lists, for iterates over all elements in the list after closing it.
For structs, for iterates over all non-optional regular fields.
