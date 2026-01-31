---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#39-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 129
summary: It is commutative, associative, and idempotent. The unification of a disjunction with another value is equal to the disjunction
---

It is commutative, associative, and idempotent.

The unification of a disjunction with another value is equal to the disjunction
composed of the unification of this value with all of the original elements
of the disjunction.
In other words, unification distributes over disjunction.


Copy code
Copied!

(a_0 | ... |a_n) & b ==> a_0&b | ... | a_n&b.


Copy code
Copied!

Expression                Result
({a:1} | {b:2}) & {c:3}   {a:1, c:3} | {b:2, c:3}
(int | string) & "foo"    "foo"
("a" | "b") & "c"         _|_
