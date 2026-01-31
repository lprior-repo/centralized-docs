---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#119-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 141
summary: LetClause           = \"let\" identifier \"=\" Expression . a: [1, 2, 3, 4]
---

LetClause           = "let" identifier "=" Expression .


Copy code
Copied!

a: [1, 2, 3, 4]
b: [for x in a if x > 1 { x+1 }]  // [3, 4, 5]

c: {
    for x in a
    if x < 4
    let y = 1 {
        "\(x)": x + y
    }
}
d: { "1": 2, "2": 3, "3": 4 }

STRING INTERPOLATION

String interpolation allows constructing strings by replacing placeholder
expressions with their string representation.
String interpolation may be used in single- and double-quoted strings, as well
as their multiline equivalent.

A placeholder consists of \( followed by an expression and ).
