---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#5-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 528
summary: This highly abstract definition determines almost everything about CUE. For instance, lazy binding was not a design decision,
---


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
In other words, an instance may never violate the constraints of its parent.
This property makes it very hard to inadvertently make false conclusions in CUE.
