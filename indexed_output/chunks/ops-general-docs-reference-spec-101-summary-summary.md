---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#101-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 129
summary: Otherwise, if x is not a struct,. or if f does not exist in x,
---


Otherwise, if x is not a struct,
or if f does not exist in x,
the result of the expression is bottom (an error).
In the latter case the expression is incomplete.
The operand of a selector may be associated with a default.


Copy code
Copied!

T: {
    x:     int
    y:     3
    "x-y": 4
}

a: T.x     // int
b: T.y     // 3
c: T.z     // _|_ // field 'z' not found in T
d: T."x-y" // 4

e: {a: 1|*2} | *{a: 3|*4}
f: e.a  // 4 (default value)

INDEX EXPRESSIONS

A primary expression of the form


Copy code
Copied!
