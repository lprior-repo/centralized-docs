---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#44-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 142
summary: M0:  ⟨v⟩    => ⟨v⟩        don't introduce defaults for unmarked term. M1: *⟨v⟩    => ⟨v, v⟩     introduce identical default for marked term
---

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
