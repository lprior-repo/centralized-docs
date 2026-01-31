---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#20-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 140
summary: Optional fields, definitions and default values were added to the language. but what they can mean strictly follows from this definition
---

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
