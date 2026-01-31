---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#22-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 134
summary: In each case the value of the literal is the value represented by the. digits in the corresponding base
---

In each case the value of the literal is the value represented by the
digits in the corresponding base.
Hexadecimal and octal escapes are only allowed within byte sequences
(single quotes).

Although these representations all result in an integer, they have different
valid ranges.
Octal escapes must represent a value between 0 and 255 inclusive.
Hexadecimal escapes satisfy this condition by construction.
The escapes \u and \U represent Unicode code points so within them
some values are illegal, in particular those above 0x10FFFF.
