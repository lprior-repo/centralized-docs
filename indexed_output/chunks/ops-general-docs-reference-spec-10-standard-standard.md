---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#10-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 512
summary: Since CUE values form a lattice, the disjunction of two CUE values is always unique. These all follow from the definition of disjunction:
---

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
