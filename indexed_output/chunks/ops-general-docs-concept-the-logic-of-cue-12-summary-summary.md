---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#12-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 144
summary: Let’s look at all types CUE supports. ⊥ (bottom)
---



CUE TYPES

Let’s look at all types CUE supports.

⊤ (top)
bool
null
bytes
number
int
struct
list
string
⊥ (bottom)

There are actually values between top and the basic types.
The | operator in CUE allows one to define “sum types” like int | string.
The same operator can also be used to describe what are called “enums”
in other languages, for instance, 1 | 2 | 3.
To CUE these two things—disjunctions of types and disjunctions of values—are the same thing.
You can also mix types and values in a disjunction, as in *1 | intto define defaults (marked by *),
