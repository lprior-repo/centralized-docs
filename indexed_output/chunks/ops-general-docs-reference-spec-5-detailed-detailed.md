---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#5-detailed
chunk_level: detailed
chunk_type: table
heading: Introduction
token_count: 1024
summary: and for any other value e for which a ⊑ e and b ⊑ e,. it holds that d ⊑ e
---

and for any other value e for which a ⊑ e and b ⊑ e,
it holds that d ⊑ e.)
This style of disjunctions is sometimes also referred to as sum types.
Since CUE values form a lattice, the disjunction of two CUE values is always unique.

These all follow from the definition of disjunction:

 * The disjunction of a with itself is always a.
 * The disjunction of a value a and b where a ⊑ b is always b.
 * The disjunction of a value a with bottom is always a.
 * The disjunction of two bottom values is bottom.

Disjunction in CUE is a binary expression [/docs/reference/spec/#operands], written a | b.
It is commutative, associative, and idempotent.

The unification of a disjunction with another value is equal to the disjunction
composed of the unification of this value with all of the original elements
of the disjunction.
In other words, unification distributes over disjunction.


Copy code
Copied!

(a_0 | ... |a_n) & b ==> a_0&b | ... | a_n&b.


Copy code
Copied!

Expression                Result
({a:1} | {b:2}) & {c:3}   {a:1, c:3} | {b:2, c:3}
(int | string) & "foo"    "foo"
("a" | "b") & "c"         _|_

A disjunction is normalized if there is no element
a for which there is an element b such that a ⊑ b.


DEFAULT VALUES

Any value v may be associated with a default value d,
where d must be in instance of v (d ⊑ v).

Default values are introduced by means of disjunctions.
Any element of a disjunction can be marked as a default
by prefixing it with an asterisk * (a unary expression [/docs/reference/spec/#operators]).
Syntactically consecutive disjunctions are considered to be
part of a single disjunction,
whereby multiple disjuncts can be marked as default.
A marked disjunction is one where any of its terms are marked.
So a | b | *c | d is a single marked disjunction of four terms,
whereas a | (b | *c | d) is an unmarked disjunction of two terms,
one of which is a marked disjunction of three terms.
During unification, if all the marked disjuncts of a marked disjunction are
eliminated, then the remaining unmarked disjuncts are considered as if they
originated from an unmarked disjunction

As explained below, distinguishing the nesting of disjunctions like this
is only relevant when both an outer and nested disjunction are marked.

Intuitively, when an expression needs to be resolved for an operation other
than unification or disjunction,
non-starred elements are dropped in favor of starred ones if the starred ones
do not resolve to bottom.

To define the unification and disjunction operation we use the notation
⟨v⟩ to denote a CUE value v that is not associated with a default
and the notation ⟨v, d⟩ to denote a value v associated with a default
value d.

The rewrite rules for unifying such values are as follows:


Copy code
Copied!

U0: ⟨v1⟩ & ⟨v2⟩         => ⟨v1&v2⟩
U1: ⟨v1, d1⟩ & ⟨v2⟩     => ⟨v1&v2, d1&v2⟩
U2: ⟨v1, d1⟩ & ⟨v2, d2⟩ => ⟨v1&v2, d1&d2⟩

The rewrite rules for disjoining terms of unmarked disjunctions are


Copy code
Copied!

D0: ⟨v1⟩ | ⟨v2⟩         => ⟨v1|v2⟩
D1: ⟨v1, d1⟩ | ⟨v2⟩     => ⟨v1|v2, d1⟩
D2: ⟨v1, d1⟩ | ⟨v2, d2⟩ => ⟨v1|v2, d1|d2⟩

Terms of marked disjunctions are first rewritten according to the following
rules:


Copy code
Copied!

M0:  ⟨v⟩    => ⟨v⟩        don't introduce defaults for unmarked term
M1: *⟨v⟩    => ⟨v, v⟩     introduce identical default for marked term
M2: *⟨v, d⟩ => ⟨v, d⟩     keep existing defaults for marked term
M3:  ⟨v, d⟩ => ⟨v⟩        strip existing defaults from unmarked term

Note that for any marked disjunction a,
the expressions a|a, *a|a and *a|*a all resolve to a.


Copy code
Copied!

Expression               Value-default pair     Rules applied
*"tcp" | "udp"           ⟨"tcp"|"udp", "tcp"⟩    M1, D1
string | *"foo"          ⟨string, "foo"⟩         M1, D1

*1 | 2 | 3               ⟨1|2|3, 1⟩              M1, D1

(*1|2|3) | (1|*2|3)      ⟨1|2|3, 1|2⟩            M1, D1, D2
(*1|2|3) | *(1|*2|3)     ⟨1|2|3, 2⟩              M1, M2, M3, D1, D2
