---
doc_id: ops/general/docs-concept-the-logic-of-cue
chunk_id: ops/general/docs-concept-the-logic-of-cue#2-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 524
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
