---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#4-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 522
summary: in the older version. With optional fields it gets a bit more subtle, but basically,
---

in the older version.

With optional fields it gets a bit more subtle, but basically,
an instance may change an optional field to required, but not remove it.
The backwards compatibility metaphor applies here as well.

a?: int
a: int
a?: int & <10
a: int & <10
Required is more specific than optional
a?: 0
a?: 1
a?: ⊥
a: 0
a: 1
⊥
Conflicting values for optional fields result in disallowing that field, conflicting required fields result in a faulty struct

An important thing to note is that, unlike for required fields,
conflicting values for an optional field do not cause a struct to be faulty.
This definition was a result from fitting the notion of closed structs into
the value lattice.
But it can also be explained with some logic.
A common practice in interpretations of logic is to allow
infering ¬P from P→⊥.
If for an optional field we find the value ⊥, we can infer
"not that field", or, drop it.
If we derive ⊥ for a required field, we have a problem,
as a required field cannot be omitted.

THE ANSWER TO LIFE, THE UNIVERSE AND EVERYTHING

CUE has its own equivalent of 42, the answer to life, the universe and
everything, albeit more than 2 characters.
Graph unification of typed feature structures,
CUE’s theoretical foundation, can be described at many levels of abstraction.
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
(two references starting from the root of a config end up at the same node)
and PF(π)=t means the type
at path π is t (itself a graph in F).

Unification F⊓F′ of two TFSs F and F′ is then the greatest lower
bound of F and F′ in F ordered by subsumption.
