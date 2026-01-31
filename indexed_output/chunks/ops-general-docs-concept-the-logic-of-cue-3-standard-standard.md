---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#3-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 519
summary: Note that we say all numbers, even though 10 is an integer. This is because CUE allows implicit conversion between
---

Note that we say all numbers, even though 10 is an integer.
This is because CUE allows implicit conversion between
number types in comparisons.


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
and you can use expressions as well, like *pet.species | "cat".
The latter evaluates to the value of pet.species, or "cat" if
pet.species is null; this is called null coalescing in some languages.

These various uses of | are not the result of operator overloading: they are
all the same operation in CUE.


STRUCTS

Ordering of scalar types, like numbers and strings, is fairly straightforward
and will feel familiar to anyone that has worked with a typed programming
language.
But ordering structs might seem a bit unusual.

Below are two examples of an ordering defined on structs.

municipality
name: string
population: int
big city
name: string
population: >1M
London
name: 'London'
population: 8M
London is a big city, which is a municipality
⊤
a: int
b: int
a: 1
a: int
b: int
a: 1
b: 1
⊥
b: 1

Loosely speaking, a struct is an instance of another if it has at least
all the fields defined by the parent and if its constraints on these fields
are at least as strict as those defined by its parent.

The instance relation for structs has an analogy in
software engineering: backwards compatibility.
For a newer version of an API to be backwards compatible with the previous
version it must subsume it.
In other words, the old version must be an instance of the new one.
Or yet another way to say it: a new version may not forbid what was allowed
