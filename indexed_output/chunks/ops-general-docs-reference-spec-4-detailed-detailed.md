---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#4-detailed
chunk_level: detailed
chunk_type: table
heading: Introduction
token_count: 1037
summary: but allow newline characters. Multiline strings and byte sequences respectively start with
---

but allow newline characters.

Multiline strings and byte sequences respectively start with
a triple double quote (""") or triple single quote ('''),
immediately followed by a newline, which is discarded from the string contents.
The string is closed by a matching triple quote, which must be by itself
on a new line, preceded by optional whitespace.
The newline preceding the closing quote is discarded from the string contents.
The whitespace before a closing triple quote must appear before any non-empty
line after the opening quote and will be removed from each of these
lines in the string literal.
A closing triple quote may not appear in the string.
To include it is suffices to escape one of the quotes.


Copy code
Copied!

"""
    lily:
    out of the water
    out of itself

    bass
    picking \
    bugs
    off the moon
        — Nick Virgilio, Selected Haiku, 1988
    """

This represents the same string as:


Copy code
Copied!

"lily:\nout of the water\nout of itself\n\n" +
"bass\npicking bugs\noff the moon\n" +
"    — Nick Virgilio, Selected Haiku, 1988"

VALUES

In addition to simple values like "hello" and 42.0, CUE has structs [/docs/reference/spec/#structs].
A struct is a map from labels to values, like {a: 42.0, b: "hello"}.
Structs are CUE’s only way of building up complex values;
lists, which we will see later,
are defined in terms of structs.

All possible values are ordered in a lattice,
a partial order where every two elements have a single greatest lower bound.
A value a is an instance of a value b,
denoted a ⊑ b, if b == a or b is more general than a,
that is if a orders before b in the partial order
(⊑ is not a CUE operator).
We also say that b subsumes a in this case.
In graphical terms, b is “above” a in the lattice.

At the top of the lattice is the single ancestor of all values, called
top [/docs/reference/spec/#top], denoted _ in CUE.
Every value is an instance of top.

At the bottom of the lattice is the value called bottom [/docs/reference/spec/#bottom], denoted _|_.
A bottom value usually indicates an error.
Bottom is an instance of every value.

An atom is any value whose only instances are itself and bottom.
Examples of atoms are 42.0, "hello", true, and null.

A value is concrete if it is either an atom, or a struct whose field values
of regular (non-hidden and non-definition fields) are all concrete, recursively.

CUE’s values also include what we normally think of as types, like string and
float.
It does not distinguish between types and values:
only the relationship of values in the lattice is important.
Each CUE “type” subsumes the concrete values that one would normally think
of as part of that type.
For example, "hello" is an instance of string, and 42.0 is an instance of
float.
In addition to string and float, CUE has null, int, bool, and bytes.
We informally call these CUE’s “basic types”.


Copy code
Copied!

false ⊑ bool
true  ⊑ bool
true  ⊑ true
5.0   ⊑ float
bool  ⊑ _
_|_   ⊑ _
_|_   ⊑ _|_

_     ⋢ _|_
_     ⋢ bool
int   ⋢ bool
bool  ⋢ int
false ⋢ true
true  ⋢ false
float ⋢ 5.0
5     ⋢ 6

UNIFICATION

The unification of values a and b
is defined as the greatest lower bound of a and b. (That is, the
value u such that u ⊑ a and u ⊑ b,
and for any other value v for which v ⊑ a and v ⊑ b
it holds that v ⊑ u.)
Since CUE values form a lattice, the unification of two CUE values is
always unique.

These all follow from the definition of unification:

 * The unification of a with itself is always a.
 * The unification of values a and b where a ⊑ b is always a.
 * The unification of a value with bottom is always bottom.

Unification in CUE is a binary expression [/docs/reference/spec/#operands], written a & b.
It is commutative, associative, and idempotent.
As a consequence, order of evaluation is irrelevant, a property that is key
to many of the constructs in the CUE language as well as the tooling layered
on top of it.

DISJUNCTION

The disjunction of values a and b
is defined as the least upper bound of a and b.
(That is, the value d such that a ⊑ d and b ⊑ d,
