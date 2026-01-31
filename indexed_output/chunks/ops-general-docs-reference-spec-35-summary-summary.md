---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#35-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 129
summary: Each CUE “type” subsumes the concrete values that one would normally think. of as part of that type
---

Each CUE “type” subsumes the concrete values that one would normally think
of as part of that type.
For example, "hello" is an instance of string, and 42.0 is an instance of
float.
In addition to string and float, CUE has null, int, bool, and bytes.
We informally call these CUE’s “basic types”.


Copy code
Copied!

false ⊑ bool
true  ⊑ bool
true  ⊑ true
5.0   ⊑ float
bool  ⊑ _
_|_   ⊑ _
_|_   ⊑ _|_

_     ⋢ _|_
_     ⋢ bool
int   ⋢ bool
bool  ⋢ int
false ⋢ true
true  ⋢ false
