---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#48-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: Any evaluation error in CUE results in a bottom value, represented by. the token _|_
---

Any evaluation error in CUE results in a bottom value, represented by
the token _|_.
Bottom is an instance of every other value.
Any evaluation error is represented as bottom.

Implementations may associate error strings with different instances of bottom;
logically they all remain the same value.


Copy code
Copied!

bottom_lit = "_|_" .

TOP

Top is represented by the underscore character _, lexically an identifier.
Unifying any value v with top results in v itself.


Copy code
Copied!

Expr        Result
