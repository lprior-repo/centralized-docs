---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#72-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 136
summary: An embedded value of type struct is unified with the struct in which it is. embedded, but disregarding the restrictions imposed by closed structs
---

An embedded value of type struct is unified with the struct in which it is
embedded, but disregarding the restrictions imposed by closed structs.
So if an embedding resolves to a closed struct, the corresponding enclosing
struct will also be closed, but may have fields that are not allowed if
normal rules for closed structs were observed.

If an embedded value is not of type struct, the struct may only have
definitions or hidden fields. Regular fields are not allowed in such case.

The result of { A } is A for any A (including definitions).
