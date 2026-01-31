---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#11-standard
chunk_level: standard
chunk_type: table
heading: Introduction
token_count: 527
summary: is only relevant when both an outer and nested disjunction are marked. Intuitively, when an expression needs to be resolved for an operation other
---

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
(*1|2|3) | (1|*2|3)&2    ⟨1|2|3, 1|2⟩            M1, D1, U1, D2

(*1|2) & (1|*2)          ⟨1|2, _|_⟩              M1, D1, U2

The rules of subsumption for defaults can be derived from the above definitions
