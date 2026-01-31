---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#8-standard
chunk_level: standard
chunk_type: prose
heading: Introduction
token_count: 524
summary: immediately followed by a newline, which is discarded from the string contents. The string is closed by a matching triple quote, which must be by itself
---

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
