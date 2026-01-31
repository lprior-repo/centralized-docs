---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#18-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 128
summary: CUE’s language specification, and most literature,. take a less abstract and more comprehensible approach,
---

CUE’s language specification, and most literature,
take a less abstract and more comprehensible approach,
but in its most abstract form, it can loosely be defined as follows:

Subsumption: given a set F of all TFSs (graphs, CUE values, basically),
and F and F′ in F,
F subsumes F′, denoted F⊑F′, if and only if:
π≡Fπ′ implies π≡F′π′PF(π)=t implies PF′(π)=t′ and t′⊑t
where π≡Fπ′ means that
F∈F contains a path equivalence or reentrancy between
the paths π and π′
