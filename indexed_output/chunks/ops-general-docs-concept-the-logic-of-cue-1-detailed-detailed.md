---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#1-detailed
chunk_level: detailed
chunk_type: table
heading: Introduction
token_count: 1024
summary: as true and false. For instance, when we say that a value is both a bool and true,
---

as true and false.
For instance, when we say that a value is both a bool and true,
or in lattice terms,
if we find the greatest lower bound of these values,
the answer is true.
Again maybe no surprise, except that in CUE this is actually
an operation, denoted bool & true.

This also explains the odd fourth element in the graph labeled bottom.
Bottom, in this example, is the result of computing true & false.
A value cannot be both true and false, so this an error.
Bottom is analogous to an error in many other languages.
Bottom is an instance of every value and type, in fact.
More on errors later.

One more detail:
besides the meet operator (&), CUE also has a join operator (|),
which computes the least upper bound.
The result of true | false is indeed bool in CUE.


NUMBERS

With numbers things get a bit more interesting.
CUE has gone through various iterations of the number type system
to find the mix of being practical and strict, while still being simple.
CUE recognizes number, and the instances int and float as classes
of numbers.
For now it suffices to only consider number and int, the latter being
an instance of the former.

Let’s consider a lattice with some example numeric values.
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
the traditional concepts of value and type.

CUE defines the constraints we see here in terms of its binary operators
>= and <.
It allows all binary operators that result in a boolean, except ==,
to be used as a constraint by leaving off the left value,
where op B defines the set of all values A for which A op B is true.
The constraint <10 means all numbers less than 10.
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
