---
doc_id: ops/general/docs-reference-spec
chunk_id: ops/general/docs-reference-spec#18-summary
chunk_level: summary
chunk_type: prose
heading: Introduction
token_count: 131
summary: When multiplying a fraction by a multiplier, the result is truncated. towards zero if it is not an integer
---

When multiplying a fraction by a multiplier, the result is truncated
towards zero if it is not an integer.


Copy code
Copied!

42
1.5G    // 1_500_000_000
1.3Ki   // 1.3 * 1024 = trunc(1331.2) = 1331
170_141_183_460_469_231_731_687_303_715_884_105_727
0xBad_Face
0o755
0b0101_0001

A decimal floating-point literal is a representation of
a decimal floating-point value (a float).
It has an integer part, a decimal point, a fractional part, and an
exponent part.
The integer and fractional part comprise decimal digits; the
