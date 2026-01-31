---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#2-detailed
chunk_level: detailed
chunk_type: prose
heading: Introduction
token_count: 1030
summary: Or yet another way to say it: a new version may not forbid what was allowed. in the older version
---

Or yet another way to say it: a new version may not forbid what was allowed
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

This highly abstract definition determines almost everything about CUE.
For instance, lazy binding was not a design decision,
but a direct consequence of following this definition.
It determines the possible evaluation strategies and
what cycles mean, if allowed.
Optional fields, definitions and default values were added to the language
by choice,
but what they can mean strictly follows from this definition.


NULL

We conveniently left out the discussion of null before.
Not only does it make an uninspiring example to describe a lattice,
it is also actually surprisingly complicated to pin down what it means.
This is partly due to lack of guidance from the JSON
standard regarding its
meaning and the different interpretations it gets in practice.

TypeScript creates some order in the chaos by introducing the concepts
undefined and void in addition to null.
It is a necessary evil to give null some meaning
that is compatible with common practices,
within the context of its type system.

CUE got lucky.
CUE’s interpretation of null, optionality, and related concepts
is actually inspired by TypeScript.
But because types are values in CUE, TypeScript’s concepts of
undefined, void and null and optional fields, roughly collapse onto CUE’s
null, bottom (_|_), and optional fields,
resulting in a somewhat simpler model.

DEFAULT VALUES

Default values are CUE’s equivalent of inheritance,
specifically the kind that allows instances to override any value of its parent.
Without it, very little boilerplate removal would be possible.
That is fine if CUE is used just for validation,
but as it aims to be useful across the entire configuration continuum,
it seemed too restrictive to not have such a construct.


RELATION TO INHERITANCE

In CUE, if one sees a concrete value for a field,
it is guaranteed that this will be the final result.
If a value is not concrete (like string), it is clear the search
for a concrete value is not over.
