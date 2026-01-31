---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#132-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 135
summary: A reference cycle occurs if a field references itself, either directly or. // x references itself
---


A reference cycle occurs if a field references itself, either directly or
indirectly.


Copy code
Copied!

// x references itself
x: x

// indirect cycles
b: c
c: d
d: b

Implementations should treat these as _.
Two particular cases are discussed below.


EXPRESSIONS THAT UNIFY AN ATOM WITH AN EXPRESSION

An expression of the form a & e, where a is an atom
and e is an expression, always evaluates to a or bottom.
As it does not matter how we fail, we can assume the result to be a
and postpone validating a == e until after all references
