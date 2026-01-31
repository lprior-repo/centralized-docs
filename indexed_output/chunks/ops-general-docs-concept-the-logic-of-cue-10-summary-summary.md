---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#10-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: We cannot show a complete lattice, of course, as the number of elements is. infinite (it actually is, CUE has arbitrary precision arithmetic)
---

We cannot show a complete lattice, of course, as the number of elements is
infinite (it actually is, CUE has arbitrary precision arithmetic).

number
int
>=0.5
<10
0
1
1.1
20.0
⊥ (bottom)

Here we see what is traditionally a type class (number and int)
and some concrete instances, that is, specific numbers.
They are ordered as expected: 0 and 1 are
integral numbers, whereas 20.0 (by definition) and 1.1 are numbers,
but not integers.
But we also see “constraints”, a category of values that falls between
