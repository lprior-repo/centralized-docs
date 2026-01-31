---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#33-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 133
summary: A value a is an instance of a value b,. denoted a ⊑ b, if b == a or b is more general than a,
---

A value a is an instance of a value b,
denoted a ⊑ b, if b == a or b is more general than a,
that is if a orders before b in the partial order
(⊑ is not a CUE operator).
We also say that b subsumes a in this case.
In graphical terms, b is “above” a in the lattice.

At the top of the lattice is the single ancestor of all values, called
top [/docs/reference/spec/#top], denoted _ in CUE.
Every value is an instance of top.

At the bottom of the lattice is the value called bottom [/docs/reference/spec/#bottom], denoted _|_.
