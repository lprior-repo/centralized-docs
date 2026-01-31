---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#46-summary
chunk_level: summary
chunk_type: table
heading: Introduction
token_count: 134
summary: ⟨v2, d2⟩ ⊑ ⟨v1, d1⟩  if v2 ⊑ v1 and d2 ⊑ d1. ⟨v1, d1⟩ ⊑ ⟨v⟩       if v1 ⊑ v
---


⟨v2, d2⟩ ⊑ ⟨v1, d1⟩  if v2 ⊑ v1 and d2 ⊑ d1
⟨v1, d1⟩ ⊑ ⟨v⟩       if v1 ⊑ v
⟨v⟩      ⊑ ⟨v1, d1⟩  if v ⊑ d1


Copy code
Copied!

Expression                       Resolves to
"tcp" | "udp"                    "tcp" | "udp"
*"tcp" | "udp"                   "tcp"
float | *1                       1
*string | 1.0                    string
(*1|2) + (2|*3)                  4

(*1|2|3) | (1|*2|3)              1|2
(*1|2|3) & (1|*2|3)              1|2|3 // default is _|_

(* >=5 | int) & (* <=5 | int)    5
