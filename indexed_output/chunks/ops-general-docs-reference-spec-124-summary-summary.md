---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#124-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 136
summary: and([a, b])          a & b. and([a])             a
---

and([a, b])          a & b
and([a])             a
and([])              _

OR

The builtin function or takes a list and returns the result of applying
the | operator to all elements in the list.
It returns bottom for the empty list.


Copy code
Copied!

Expression:          Result
or([a, b])           a | b
or([a])              a
or([])               _|_

DIV, MOD, QUO AND REM

For two integer values x and y,
the integer quotient q = div(x, y) and remainder r = mod(x, y)
implement Euclidean division and
satisfy the following relationship:
