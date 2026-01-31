---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#57-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 131
summary: {a: 1} ⊑ {}. {a: 1, b: 1} ⊑ {a: 1}
---


{a: 1} ⊑ {}
{a: 1, b: 1} ⊑ {a: 1}
{a: 1} ⊑ {a: int}
{a: 1, b: 1.0} ⊑ {a: int, b: number}

{} ⋢ {a: 1}
{a: 2} ⋢ {a: 1}
{a: 1} ⋢ {b: 1}

The successful unification of structs a and b is a new struct c which
has all fields of both a and b, where
the value of a field f in c is a.f & b.f if f is defined in both a and b,
or just a.f or b.f if f is in just a or b, respectively.
Any references [/docs/reference/spec/#references] to a or b
in their respective field values need to be replaced with references to c.
