---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#27-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 129
summary: (>=3 & <=10 and >=5 & <=20 become >=5 & <=10, >=1 & <=1 becomes 1),. GCL and Jsonnet do not (it would be quite complex),
---

(>=3 & <=10 and >=5 & <=20 become >=5 & <=10, >=1 & <=1 becomes 1),
GCL and Jsonnet do not (it would be quite complex),
causing an ever-growing pile of assertions.


SEMANTICS

CUE defaults, which are values marked with a * in disjunctions,
preserve the beneficial properties of the lattice.
In order to do so,
CUE must ensure that the order of picking defaults does not influence the outcome.
Suppose we define two fields, each with the same default value.
We also define that these fields are equal to each other.
