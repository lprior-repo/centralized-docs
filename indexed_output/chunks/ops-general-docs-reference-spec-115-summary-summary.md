---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#115-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 134
summary: to the function and the called function begins execution. The return parameters
---

to the function and the called function begins execution.
The return parameters
of the function are passed by value back to the calling function when the
function returns.

COMPREHENSIONS

Lists and fields can be constructed using comprehensions.

Comprehensions define a clause sequence that consists of a sequence of
for, if, and let clauses, nesting from left to right.
The sequence must start with a for or if clause.
The for and let clauses each define a new scope in which new values are
bound to be available for the next clause.
