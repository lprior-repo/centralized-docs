---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#45-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 128
summary: string | *\"foo\"          ⟨string, \"foo\"⟩         M1, D1. *1 | 2 | 3               ⟨1|2|3, 1⟩              M1, D1
---

string | *"foo"          ⟨string, "foo"⟩         M1, D1

*1 | 2 | 3               ⟨1|2|3, 1⟩              M1, D1

(*1|2|3) | (1|*2|3)      ⟨1|2|3, 1|2⟩            M1, D1, D2
(*1|2|3) | *(1|*2|3)     ⟨1|2|3, 2⟩              M1, M2, M3, D1, D2
(*1|2|3) | (1|*2|3)&2    ⟨1|2|3, 1|2⟩            M1, D1, U1, D2

(*1|2) & (1|*2)          ⟨1|2, _|_⟩              M1, D1, U2

The rules of subsumption for defaults can be derived from the above definitions
and are as follows.


Copy code
Copied!
