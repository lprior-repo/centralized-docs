---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#117-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 131
summary: An if clause, or guard, specifies an expression that terminates the current. iteration if it evaluates to false
---


An if clause, or guard, specifies an expression that terminates the current
iteration if it evaluates to false.

The let clause binds the result of an expression to the defined identifier
in a new scope.

A current iteration is said to complete if the innermost block of the clause
sequence is reached.
Syntactically, the comprehension value is a struct.
A comprehension can generate non-struct values by embedding such values within
this struct.

Within lists, the values yielded by a comprehension are inserted in the list
